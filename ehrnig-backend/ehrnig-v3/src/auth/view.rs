use askama::Template;
use axum::Json;
use axum::extract::Query;
use axum::extract::{Form, State};
use axum::{
    self,
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tower_sessions::Session;
use tower_sessions_redis_store::fred::clients::RedisClient;
use tower_sessions_redis_store::fred::interfaces::KeysInterface;
use tower_sessions_redis_store::fred::types::Expiration;
use uuid::Uuid;

use super::authutils::{create_jwt, generate_auth_code, verify_user_password};
use super::repository::find_oauth_client;
use crate::auth::repository::OAuthClient;
use crate::routes::AppState;
use crate::users::find_by_email;

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {
    state: String, // This helps link the login back to the OIDC request
}

#[derive(Deserialize)]
pub struct AuthRequestParams {
    pub state: Option<String>,
}

#[derive(Deserialize)]
pub struct LoginDto {
    email: String,
    password: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct AuthorizeRequest {
    pub client_id: String,
    pub redirect_uri: String,
    pub response_type: String,
    pub state: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct OAuthSession {
    pub user_id: uuid::Uuid,
    pub client_id: String,
}

#[derive(Deserialize)]
pub struct TokenRequest {
    pub grant_type: String,
    pub code: Option<String>,         // needed only for auth code
    pub redirect_uri: Option<String>, // needed only for auth code
    pub client_id: String,
    pub client_secret: Option<String>,
    pub refresh_token: Option<String>, // needed only for refresh token
}

#[derive(Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub refresh_token: Option<String>,
    pub scope: Vec<String>,
}

// GET "/auth/login"
pub async fn show_login(Query(params): Query<AuthRequestParams>) -> impl IntoResponse {
    let template = LoginTemplate {
        state: params.state.unwrap_or_default(),
    };
    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to render template",
        )
            .into_response(),
    }
}

// POST "/auth/login"
pub async fn handle_login(
    session: Session,
    State(state): State<AppState>,
    Form(payload): Form<LoginDto>,
) -> impl IntoResponse {
    let user = match find_by_email(&state.db, &payload.email).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return Redirect::to("/auth/login?error=Invalid+email+or+password").into_response();
        }
        Err(_) => {
            return Redirect::to("/auth/login?error=Invalid+email+or+password").into_response();
        }
    };
    // 2. verify_password in spawn_blocking due to argon to small delay
    let password_hash = user.password_hash.clone();
    let password = payload.password.clone();
    let is_valid_password =
        tokio::task::spawn_blocking(move || verify_user_password(&password, &password_hash))
            .await
            .unwrap_or(false);
    if !is_valid_password {
        return Redirect::to("/auth/login?error=Invalid+email+or+password").into_response();
    }
    // 3. Set a Session Cookie (using tower-sessions)
    if session.insert("user_id", user.id).await.is_err() {
        return Redirect::to("/auth/login?error=server").into_response();
    }
    // 4. Redirect back to the /authorize flow
    let pending: Option<AuthorizeRequest> = session.get("pending_authorize").await.unwrap_or(None);
    if let Some(params) = pending {
        // delete the key so its not used twice
        let _ = session
            .remove::<AuthorizeRequest>("pending_authorize")
            .await;
        // build the loop back url
        let next_url = format!(
            "/auth/authorize?client_id={}&redirect_uri={}&response_type={}&state={}",
            params.client_id,
            params.redirect_uri,
            params.response_type,
            params.state.unwrap_or_default()
        );
        Redirect::to(&next_url).into_response()
    } else {
        "Login Successful".into_response()
    }
}

// GET "/auth/authorize"
// Method to get the authorization code -> needed to get the auth token
pub async fn handle_authorize(
    session: Session,
    State(state): State<AppState>,
    Query(params): Query<AuthorizeRequest>,
) -> impl IntoResponse {
    // 1. check if the user is logged in
    let user_id: Option<uuid::Uuid> = session.get("user_id").await.unwrap_or(None);

    if user_id.is_none() {
        // Not logged in: Send them to the login page.
        // Save the existing OIDC request in the session here to redirect them BACK to this authorize page after they log in.
        let _ = session.insert("pending_authorize", &params).await;
        return Redirect::to("/auth/login").into_response();
    }

    // 2. check if the client is valid from the db
    let client_result = find_oauth_client(&state.db, &params.client_id).await;

    match client_result {
        Ok(Some(client)) => {
            // Validate is the redirect_uri they sent in the allowed list?
            if !client.redirect_uris.contains(&params.redirect_uri) {
                return (
                    StatusCode::BAD_REQUEST,
                    "Invalid redirect URI for this client",
                )
                    .into_response();
            }

            // 3. Generate the actual 'code' "auth code" -> needed for authorization flow grant type to get the token
            let auth_code = generate_auth_code();
            // create the data package to be stored in redis for later confirmation
            let pending_data = OAuthSession {
                user_id: user_id.unwrap(),
                client_id: params.client_id,
            };
            // Prepare the Redis key
            let redis_key = format!("auth_code:{}", auth_code);

            // Serialize the data to JSON
            let data_json = match serde_json::to_string(&pending_data) {
                Ok(json) => json,
                Err(_) => {
                    return (StatusCode::INTERNAL_SERVER_ERROR, "Serialization error")
                        .into_response();
                }
            };
            // 4. SAVE TO REDIS (with a 5-minute TTL)
            match state
                .redis
                .set::<(), _, _>(redis_key, data_json, Some(Expiration::EX(300)), None, false)
                .await
            {
                Ok(_) => {
                    // 5. Success! Redirect the code and the state back to the client server callback so as to generate the auth token
                    let redirect_url = format!(
                        "{}?code={}&state={}",
                        params.redirect_uri,
                        auth_code,
                        params.state.unwrap_or_default()
                    );
                    Redirect::to(&redirect_url).into_response()
                }
                Err(e) => {
                    tracing::error!("Redis Error: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Failed to save auth code",
                    )
                        .into_response()
                }
            }
        }
        Ok(None) => (StatusCode::NOT_FOUND, "Error: Client not found").into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error: failed to query client",
        )
            .into_response(),
    }
}

// POST "/auth/token"
// generate auth token (this will be called in background by the client server) 2nd handshake
pub async fn handle_generate_token(
    State(state): State<AppState>,
    Form(payload): Form<TokenRequest>,
) -> impl IntoResponse {
    match payload.grant_type.as_str() {
        "authorization_code" => handle_auth_code_grant(state, payload).await,
        "refresh_token" => handle_refresh_token_grant(state, payload).await,
        _ => (StatusCode::UNPROCESSABLE_ENTITY, "Unsupported grant type").into_response(),
    }
}

async fn handle_auth_code_grant(
    state: AppState,
    payload: TokenRequest,
) -> axum::response::Response {
    // validate client and secret
    let client = match validate_client(
        &state.db,
        &payload.client_id,
        payload.client_secret.as_deref().unwrap_or(""),
    )
    .await
    {
        Ok(c) => c,
        Err(e) => return e,
    };

    let code = match payload.code.as_deref() {
        Some(c) => c.to_string(),
        None => return (StatusCode::BAD_REQUEST, "Missing code").into_response(),
    };

    let redirect_uri = match payload.redirect_uri.as_deref() {
        Some(r) => r.to_string(),
        None => return (StatusCode::BAD_REQUEST, "Missing redirect_uri").into_response(),
    };

    // validate redirect url
    if !client.redirect_uris.contains(&redirect_uri) {
        return (StatusCode::UNAUTHORIZED, "Invalid redirect URI").into_response();
    }

    // fetch and delete auth code from Redis (Atomic Get + Del)
    let redis_key = format!("auth_code:{}", code);
    let redis_data: Option<String> = match state.redis.getdel(&redis_key).await {
        Ok(data) => data,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Redis error").into_response(),
    };

    // check if the authcode key exist in redis or expired
    let pending_data: OAuthSession = match redis_data {
        Some(json) => serde_json::from_str(&json).unwrap(),
        None => return (StatusCode::UNAUTHORIZED, "Invalid or Expired Code").into_response(),
    };

    // create access token
    let user_permission = vec!["read:patients".to_string(), "write:patients".to_string()];
    let access_token = create_jwt(&pending_data.user_id, &payload.client_id, user_permission);

    // store refresh token in Redis
    let refresh_token =
        match store_refresh_token(&state.redis, pending_data.user_id, payload.client_id).await {
            Ok(token) => token,
            Err(e) => return e,
        };

    // return both tokens
    let response = TokenResponse {
        access_token,
        token_type: "Bearer".to_string(),
        expires_in: 3600,
        refresh_token: Some(refresh_token),
        scope: client.scopes,
    };

    Json(response).into_response()
}

async fn handle_refresh_token_grant(
    state: AppState,
    payload: TokenRequest,
) -> axum::response::Response {
    // get refresh token from payload
    let refresh_token = match payload.refresh_token.as_deref() {
        Some(t) => t.to_string(),
        None => return (StatusCode::BAD_REQUEST, "Missing refresh token").into_response(),
    };

    // validate client and secret
    let client = match validate_client(
        &state.db,
        &payload.client_id,
        payload.client_secret.as_deref().unwrap_or(""),
    )
    .await
    {
        Ok(c) => c,
        Err(e) => return e,
    };

    // fetch and delete refresh token from Redis (single use)
    let redis_key = format!("refresh_token:{}", refresh_token);
    let redis_data: Option<String> = match state.redis.getdel(&redis_key).await {
        Ok(data) => data,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Redis error").into_response(),
    };

    // check if refresh token exists and not expired
    let session: OAuthSession = match redis_data {
        Some(json) => serde_json::from_str(&json).unwrap(),
        None => {
            return (StatusCode::UNAUTHORIZED, "Invalid or expired refresh token").into_response();
        }
    };

    // verify refresh token belongs to this client
    if session.client_id != payload.client_id {
        return (StatusCode::UNAUTHORIZED, "Client mismatch").into_response();
    }

    // issue new access token
    let user_permissions = vec!["read:patients".to_string(), "write:patients".to_string()];
    let new_access_token = create_jwt(&session.user_id, &payload.client_id, user_permissions);

    // store new refresh token in Redis (rotation)
    let new_refresh_token =
        match store_refresh_token(&state.redis, session.user_id, payload.client_id).await {
            Ok(token) => token,
            Err(e) => return e,
        };

    // return new tokens
    let response = TokenResponse {
        access_token: new_access_token,
        token_type: "Bearer".to_string(),
        expires_in: 3600,
        refresh_token: Some(new_refresh_token),
        scope: client.scopes,
    };

    Json(response).into_response()
}

async fn validate_client(
    db: &PgPool,
    client_id: &str,
    client_secret: &str,
) -> Result<OAuthClient, axum::response::Response> {
    let client = match find_oauth_client(db, client_id).await {
        Ok(Some(client)) => client,
        _ => return Err((StatusCode::UNAUTHORIZED, "Invalid Client").into_response()),
    };

    let db_secret = client.client_secret_hash.as_deref().unwrap_or("");
    if db_secret != client_secret {
        return Err((StatusCode::UNAUTHORIZED, "Invalid Secret").into_response());
    }

    Ok(client)
}

async fn store_refresh_token(
    redis: &RedisClient,
    user_id: Uuid,
    client_id: String,
) -> Result<String, axum::response::Response> {
    let refresh_token = generate_auth_code();
    let redis_key = format!("refresh_token:{}", refresh_token);

    let session = OAuthSession { user_id, client_id };
    let json = match serde_json::to_string(&session) {
        Ok(j) => j,
        Err(_) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, "Serialization error").into_response());
        }
    };
    let result: Result<(), _> = redis
        .set(
            &redis_key,
            json,
            Some(Expiration::EX(60 * 60 * 24 * 30)), // expire 30days
            None,
            false,
        )
        .await;

    if result.is_err() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to store refresh token",
        )
            .into_response());
    }

    Ok(refresh_token)
}

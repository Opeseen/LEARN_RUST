use anyhow::Ok;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};

use super::config::EmailConfig;

#[derive(Clone)]
pub struct EmailService {
    mailer: AsyncSmtpTransport<Tokio1Executor>,
    from_email: String,
}

impl EmailService {
    pub fn new(config: EmailConfig) -> Self {
        let creds = Credentials::new(config.user, config.pass);

        let mailer = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&config.host)
            .unwrap()
            .port(config.port)
            .credentials(creds)
            .build();

        tracing::info!("Email server initialized!");

        Self {
            mailer,
            from_email: config.from,
        }
    }

    pub async fn send_welcome_email(&self, to: &str, name: &str) -> anyhow::Result<()> {
        let email = Message::builder()
            .from(self.from_email.parse()?)
            .to(to.parse()?)
            .subject("Welcome to the EHR Platform")
            .body(format!("Hello {}, welcome aboard!", name))?;

        self.mailer.send(email).await?;
        Ok(())
    }
}

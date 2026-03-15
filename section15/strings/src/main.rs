use std::io;
fn main() {
    let pirate = "Bloodshed";
    let first_initial = &pirate[0..1];
    println!("{first_initial}");
    println!("");

    let mut full_name = String::from("Sylvester");
    let last_name = "Stallone";
    full_name.push(' ');
    full_name.push_str(last_name);
    println!("{full_name}");
    println!("{last_name}");

    let first_name = String::from("Sylvester");
    let last_name = String::from("Stallone");
    let full_name = first_name + &last_name;
    println!("{full_name}");
    println!("");

    let first_name = String::from("Sylvester");
    let last_name = String::from("Stallone");
    let icon = format!("{first_name} {last_name}");
    println!("{icon}");
    println!("{first_name}");
    println!("{last_name}");
    let result = format!("hello {}", "world!");
    println!("{}", result);
    println!("");

    let mut music_genres = "          Rock, Metal, Country, Rap      ";
    println!("{}", music_genres.trim());
    music_genres = music_genres.trim();
    println!("{}", music_genres);
    println!("{}", music_genres.replace("a", "@"));
    let genres: Vec<&str> = music_genres.split(", ").collect();
    println!("{:?}", genres);
    println!("");

    // collecting user input / output

    let mut name = String::new();
    println!("What is your name?");
    // io::stdin()
    //     .read_line(&mut name)
    //     .expect("Failed to collect input from the user");
    // println!("Hello, {name}");

    match io::stdin().read_line(&mut name) {
        Ok(_) => println!("Hello, {}", name.trim()),
        Err(message) => println!("There was an error: {message}"),
    };
}

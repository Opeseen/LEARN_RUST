use std::{
    fs::File,
    io::{Read, stdin},
    process,
};
fn main() {
    // eprintln macro
    println!("Some status update");
    eprintln!("Some error message");

    // Opening a file
    let file = File::open("story.txt");
    println!("{:#?}", file);

    let file = match File::open("story.txt") {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Something went wrong reading the file. The error is {error:?}");
            process::exit(1)
        }
    };
    println!("{file:?}");
    println!("");

    // asking a user for input
    println!("Please enter the name of the file you like to read");
    let mut input = String::new();
    let user_requested_file = stdin().read_line(&mut input);
    // checks if there was an error during the read_line
    if let Err(error) = user_requested_file {
        eprint!("Something went wrong reading the user input. The error was {error}");
        process::exit(1)
    }

    let mut file = match File::open(input.trim()) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Something went wrong opening the file. The error is {error}");
            process::exit(1)
        }
    };
    println!("{file:?}");
    println!("");

    // reading the file
    let mut file_content = String::new();
    let read_operation = file.read_to_string(&mut file_content);
    if let Err(error) = read_operation {
        eprint!("Something went wrong reading the file as a string. The error was {error}");
        process::exit(1)
    }
    println!("{file_content:?}");
    println!("");
}

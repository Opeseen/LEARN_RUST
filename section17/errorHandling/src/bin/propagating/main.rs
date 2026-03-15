use std::{
    fs::File,
    io::{self, Read, stdin},
};
fn main() {
    // Propagating Errors
    let file_result = read_file();
    match file_result {
        Ok(content) => println!("{content}"),
        Err(error) => {
            eprintln!("There was an error: {error:?}")
        }
    }
}

// Propagating Errors
fn read_file() -> Result<String, io::Error> {
    // asking a user for input
    println!("Please enter the name of the file you like to read");
    let mut input = String::new();
    let user_requested_file = stdin().read_line(&mut input);
    // checks if there was an error during the read_line
    if let Err(error) = user_requested_file {
        return Err(error);
    }

    let mut file = match File::open(input.trim()) {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    // reading the file
    let mut file_content = String::new();
    let read_operation = file.read_to_string(&mut file_content);
    if let Err(error) = read_operation {
        return Err(error);
    }

    Ok(file_content)
}

use std::fs;
use std::{
    fs::File,
    io::{self, Read, stdin},
};
fn main() {
    // Propagating Errors
    let file_result = read_file2();
    match file_result {
        Ok(content) => println!("{content}"),
        Err(error) => {
            eprintln!("There was an error: {error:?}")
        }
    }

    // using ? with options
    let mut animals = vec!["Giraffe", "Monkey", "Zebra"];
    println!("{:?}", length_of_last_element(&mut animals));
    println!("{:?}", length_of_last_element(&mut animals));
    println!("{:?}", length_of_last_element(&mut animals));
    println!("{:?}", length_of_last_element(&mut animals));
}

fn length_of_last_element(input: &mut Vec<&str>) -> Option<usize> {
    let last_element = input.pop()?;
    Some(last_element.len())
}

// Propagating Errors
fn read_file() -> Result<String, io::Error> {
    // asking a user for input
    println!("Please enter the name of the file you like to read");
    let mut input = String::new();
    stdin().read_line(&mut input)?;

    let mut file_content = String::new();
    File::open(input.trim())?.read_to_string(&mut file_content)?;

    Ok(file_content)
}

// read_to_string a& Associated function

fn read_file2() -> Result<String, io::Error> {
    // asking a user for input
    println!("Please enter the name of the file you like to read2");
    let mut input = String::new();
    stdin().read_line(&mut input)?;

    fs::read_to_string(input.trim())
}

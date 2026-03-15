use std::{
    fs::write,
    io::{self, stdin},
};

fn main() {
    let file_result = write_to_file();
    match file_result {
        Ok(file) => println!("Successfully wrote to file: {file}"),
        Err(error) => {
            eprintln!("There was error: {error}");
            std::process::exit(1);
        }
    }
}

fn write_to_file() -> Result<String, io::Error> {
    println!("What file would you like write to");
    let mut input = String::new();
    // read the user input
    stdin().read_line(&mut input)?;
    println!("");
    println!("What would you like to write to the file?");
    let mut file_content = String::new();
    // read the user file content
    stdin().read_line(&mut file_content)?;
    // write to the file here
    write(input.trim(), file_content.trim())?;

    // return the user file name
    Ok(input)
}

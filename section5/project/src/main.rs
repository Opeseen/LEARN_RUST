fn main() {
    let result = color_to_number("blue");
    println!("{result}");

    let color = "green";
    let result = match color {
        "red" => 1,
        "green" => 2,
        "blue" => 3,
        _ => 0,
    };
    println!("Result is: {result}");
    println!("");
    println!("{}", factorial_iterative_number(3));
    println!("");
    println!("{}", factorial_recursive(4));
}

fn color_to_number(color: &str) -> i32 {
    if color == "red" {
        return 1;
    } else if color == "green" {
        return 2;
    } else if color == "blue" {
        return 3;
    } else {
        return 0;
    }
}

fn factorial_iterative_number(number: i32) -> i32 {
    let mut product = 1;
    let mut count = number;

    while count > 0 {
        product *= count;
        count -= 1;
    }

    product
}

fn factorial_recursive(number: i32) -> i32 {
    if number == 1 {
        return 1;
    }
    number * factorial_recursive(number - 1) // implicit return
}

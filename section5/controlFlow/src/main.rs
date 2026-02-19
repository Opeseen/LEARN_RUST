fn main() {
    // if statement
    let some_condition = true;
    if some_condition {
        println!("This line will be output")
    }
    // else if & else statement
    let season = "winter!";
    if season == "summer" {
        println!("School out")
    } else if season == "winter" {
        println!("Brr, so cold")
    } else {
        println!("Lots of rain")
    }

    // assigning results of if-statement
    even_or_odd(18);

    // match statement
    let evaluation = true;
    match evaluation {
        true => {
            println!("The value is true")
        }
        false => {
            println!("The value is false")
        }
    }
    let value = match evaluation {
        true => 20,
        false => 40,
    };
    println!("{value}");

    // underscore in match
    match season {
        "summer" => println!("School out"),
        "winter" => println!("Brr, so cold"),
        _ => println!("Lots of rain"),
    }

    // match statement with multiple values abd conditions
    let number = 5;
    match number {
        2 | 4 | 6 | 8 => println!("{number} is even"),
        1 | 3 | 5 | 7 => println!("{number} is odd"),
        _ => println!("Unknown number!"),
    }
    match number {
        value if value % 2 == 0 => println!("{value} is an even number"),
        x if x % 2 != 0 => println!("{x} is an odd number"),
        _ => unreachable!(),
    }

    println!("");

    // loop & break & continue keyword
    let mut seconds = 21;
    loop {
        if seconds == 0 {
            println!("Blastoff!");
            break;
        }

        if seconds % 2 == 0 {
            println!("{seconds} seconds (even number), skipping 3 seconds..");
            seconds -= 3;
            continue;
        }
        println!("{seconds} seconds to blastoff!");
        seconds -= 1
    }

    println!("");

    // while loop
    let mut seconds = 21;
    while seconds > 0 {
        if seconds % 2 == 0 {
            println!("{seconds} seconds (even number), skipping 3 seconds..");
            seconds -= 3;
            continue;
        }
        println!("{seconds} seconds to blastoff!");
        seconds -= 1
    }
    println!("Blastoff!");

    // recursion
    println!("");
    count_down(5);
}

fn even_or_odd(number: i32) {
    let result = if number % 2 == 0 { "Even" } else { "Odd" };
    println!("The result is: {result}")
}

fn count_down(second: i32) {
    if second == 0 {
        println!("Blastoff!")
    } else {
        println!("{second} seconds to blastoff..");
        count_down(second - 1);
    }
}

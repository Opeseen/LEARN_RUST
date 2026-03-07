fn main() {
    // option enum
    let a = Option::Some(5);
    let p = Option::Some("hello");
    let a = Option::<i16>::Some(5);
    let d: Option<&str> = Option::None;

    // rea; example of option enum
    let musical_instruments = [
        String::from("Guitar"),
        String::from("Drums"),
        String::from("Bass"),
    ];
    let bass = musical_instruments.get(2);
    println!("{:?}", bass);
    let invalid_arguments = musical_instruments.get(4);
    println!("{:?}", invalid_arguments);
    println!("");

    // unwrap & expect keyword
    let bass = musical_instruments.get(2);
    println!("{:?}", bass.unwrap());
    println!("{:?}", bass.expect("Unable to retrieve element"));
    let invalid_instrument = musical_instruments.get(100);
    println!("{:?}", invalid_instrument);
    // invalid_arguments.expect("Unable to retrieve musical instrument");
    println!("");

    // match keyword with options
    let bass = musical_instruments.get(2);
    match bass {
        Option::Some(instrument) => println!("Playing the {instrument}"),
        Option::None => println!("Singing with my voice"),
    }
    play(bass);
    println!("{:?}", bass);

    let invalid_instrument = musical_instruments.get(100);
    match invalid_instrument {
        Option::Some(instrument) => println!("Playing the {instrument}"),
        Option::None => println!("Singing with my voice"),
    }
    play(invalid_instrument);
    println!("");

    // returning option from function body
    let availability = is_item_in_stock(true, false);
    println!("{availability:?}");
    let availability = is_item_in_stock(true, true);
    println!("{availability:?}");

    match availability {
        Option::Some(value) => println!("item is available, {value}"),
        Option::None => println!("Your item doesn't exists in out system"),
    }

    match availability {
        Option::Some(true) => println!("item is available"),
        Option::Some(false) => println!("No item is available"),
        Option::None => println!("Your item doesn't exists in out system"),
    }
    println!("");

    // top level option variant
    match availability {
        Some(true) => println!("item is available"),
        Some(false) => println!("No item is available"),
        None => println!("Your item doesn't exists in out system"),
    }
    println!("");

    // unwrap_or method
    let present_value = Some(13);
    let missing_value: Option<i32> = None;
    println!("{}", present_value.unwrap_or(0));
    println!("{}", missing_value.unwrap_or(0));
    println!("");

    // building option from scratch
    let some_option = MyOption::Some(100);
    println!("{}", some_option.unwraps());
    println!("{}", some_option.unwraps_or(13));
    let none_option = MyOption::None;
    // println!("{}", none_option.unwraps()); // will panic here
    println!("{}", none_option.unwraps_or(13)); // will fallback here
    println!("");

    // result enum
    let ok: Result<i8, &str> = Result::Ok(5);
    let disaster: Result<i32, &str> = Result::Err("Something went wrong");
    let ok: Result<i8, &str> = Ok(5);
    println!("{:?}", ok);
    let disaster: Result<i32, &str> = Err("Something went wrong");
    println!("{:?}", disaster);
    println!("");

    // result enum -> parse method on a string
    let text = "50";
    let text_as_number = text.parse::<i32>();
    println!("{:?}", text_as_number);

    let text = "alabama";
    let text_as_number = text.parse::<i32>();
    println!("{:?}", text_as_number);
    println!("");

    // returning a result enum from a function
    let result = divide(10.0, 0.0);
    match &result {
        // borrow here so ownership doesn't move
        Ok(calculation) => println!("Result: {}", calculation),
        Err(message) => println!("Error: {}", message),
    }
    println!("{}", result.unwrap_or(4.0));
    println!("");

    // result method
    let result = divide(10.0, 0.0);
    println!("{}", result.is_ok());
    println!("{}", result.is_err());
    println!("{}", result.unwrap_or(4.0));
    println!("");

    // nuances of unwrap method
    let my_result = operation2(true);
    let content = match my_result {
        Ok(message) => message,
        Err(error) => error,
    };
    println!("{}", my_result.unwrap());
    println!("");

    // the while let construct
    let mut sauces = vec!["Mayonaise", "Ketchup", "Ranch"];
    while let Some(sauce) = sauces.pop() {
        println!("The next sauce is {sauce}");
    }
}

fn operation2(great_success: bool) -> Result<&'static str, &'static str> {
    if great_success {
        Ok("Success")
    } else {
        Err("Error")
    }
}

fn operation(great_success: bool) -> Result<String, String> {
    if great_success {
        Ok("Success".to_string())
    } else {
        Err("Error".to_string())
    }
}

fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(numerator / denominator)
    }
}

#[derive(Debug, Copy, Clone)]
enum MyOption {
    Some(i32),
    None,
}

impl MyOption {
    fn unwraps(self) -> i32 {
        match self {
            MyOption::Some(value) => value,
            MyOption::None => panic!("Uh oh"),
        }
    }

    fn unwraps_or(self, fallback_value: i32) -> i32 {
        match self {
            MyOption::Some(value) => value,
            MyOption::None => fallback_value,
        }
    }
}

fn is_item_in_stock(item_is_in_system: bool, item_is_in_stock: bool) -> Option<bool> {
    if item_is_in_system && item_is_in_stock {
        Option::Some(true)
    } else if item_is_in_system {
        Option::Some(false)
    } else {
        Option::None
    }
}

fn play(instrument_option: Option<&String>) {
    match instrument_option {
        Option::Some(instrument) => println!("Playing the {instrument}"),
        Option::None => println!("Singing with my voice"),
    }
}

fn main() {
    let sixteen_bit_signed: i16 = -32500;
    let sixteen_bit_unsigned: u16 = 64000;
    let thirty_two_bit_signed: i32 = -2147483648;
    let some_value = 20;

    let sixteen_bit_signed = 6_132_500;
    // usize & isize
    let days: usize = 55;
    let years: isize = -15_000;

    println!("Dear entity, \nHow have you been?");
    println!("\tOnce upon a time");
    println!("Juliet said \"I love you Romeo\" ");

    let filepath = "C:\\My Document\\new\\videos";
    let filepath2 = r"C:\My Document\new\videos";
    println!("{filepath}");
    println!("{filepath2}");

    // Methods
    let value: i32 = -15;
    println!("{}", value.abs());
    let empty_space = "        my content         ";
    println!("{}", empty_space.trim());
    println!("{}", value.pow(2));
    println!("{}", value.pow(3));

    // float
    let pi: f64 = 3.1415926536897984;
    println!("the current value of pi is: {pi}");
    println!("{}", pi.floor());
    println!("{}", pi.ceil());
    println!("{}", pi.round());
    println!("the current value of pi is: {pi:.4}");
    println!("the current value of pi is: {:.4}", pi);

    // casting
    let miles_away = 50;
    let miles_away_i8 = miles_away as i8;
    let miles_away_u8 = miles_away as u8;

    let miles_away = 100.329032;
    let miles_away_f32 = miles_away as f32;
    let miles_away_int = miles_away as i32;
    println!("{miles_away_int}");

    // math operations
    let addition = 5 + 4;
    let subtraction = 10 - 6;
    let multiplication = 3 * 4;
    println!("Addition: {addition}, Subtraction: {subtraction} Multiplication: {multiplication}");
    let floor_division = 5 / 3;
    println!("{floor_division}");
    let decimal_division = 5.0 / 3.0;
    println!("{decimal_division}");
    let remainder = 7 % 2;
    println!("{remainder}");

    // augmented assignment
    let mut year = 2025;
    year = year + 1;
    println!("The new year is {year}");
    year += 1;
    println!("The new year is {year}");

    // boolean
    let is_handsome = true;
    let is_silly = false;
    println!("Handsome: {is_handsome}, Silly: {is_silly}");
    let age: i32 = 40;
    let is_young = age < 35;
    println!("{is_young}");
    println!("{} {}", age.is_positive(), age.is_negative());
    println!("{}", true);
    println!("{}", !true);

    let age = 3;
    let can_see_rated_r_movie = age >= 17;
    let cannot_see_rated_r_movie = !can_see_rated_r_movie;
    println!("I am {age} years old. Can i see this scary movie? {can_see_rated_r_movie}");
    println!("I am {age} years old. Can i not see this scary movie? {cannot_see_rated_r_movie}");

    // equality/inequality
    println!("{}", "Coke" == "Pepsi");
    println!("{}", "Coke" != "Pepsi");
    println!("{}", "Coke" == "coke");
    println!("{}", "Coke" == "Coke ");
    println!("{}", 13 == 13.0 as i32);

    // AND logic (&&)
    let purchased_ticket = false;
    let plane_on_time = false;
    let making_event = purchased_ticket && plane_on_time;
    println!("It is {} that i will arrive as expected", making_event);

    // OR logic (||)
    let user_has_paid_for_subscription = false;
    let user_is_admin = true;
    let user_can_see_premium_experience = user_has_paid_for_subscription || user_is_admin;
    println!("Can the user see my site? {user_can_see_premium_experience}");

    // characters type
    let first_initial = 'B';
    let emoji = '😊';
    println!(
        "{} {}",
        first_initial.is_alphabetic(),
        emoji.is_alphabetic()
    );
    println!("{} {}", first_initial.is_uppercase(), emoji.is_uppercase());

    // array types
    let numbers = [4, 8, 15, 16, 23, 22];
    let apples = ["banana", "macintosh", "red delicious"];
    println!("Length: {}", apples.len());
    let currency_rates: [f64; 0] = [];

    // read & write array element
    let seasons = ["spring", "summer", "fall", "winter"];
    let first = seasons[0];
    let second = seasons[1];
    println!("The first season is {first} and the second is {second}");

    let mut seasons2 = ["spring", "summer", "fall", "winter"];
    seasons2[2] = "Autumn";
    println!("{}", seasons2[2]);

    // display trait
    println!("{}", 5);
    println!("{}", 3.14);
    println!("{}", true);
    println!("{:?}", seasons);
    println!("{seasons:?}");
    println!("{:#?}", seasons);

    // dbg! type
    dbg!(2 + 2);
    dbg!(seasons);

    // tuple types
    let employee = ("Molly", 32, "Marketing");
    let name = employee.0;
    let age = employee.1;
    let department = employee.2;
    println!("Name: {name}, age: {age}, department: {department}");
    println!("{employee:#?}");

    let (name, age, department) = employee;
    println!("Name: {name}, age: {age}, department: {department}");

    // ranges & range iteration
    let month_days = 1..31;
    println!("{month_days:?}");
    let month_days = 1..=31;
    println!("{month_days:?}");

    for number in month_days {
        println!("{number}");
    }

    let letters = 'b'..'f';
    for letter in letters {
        println!("{letter}");
    }

    let colors = ["Red", "Green", "Yellow"];
    for color in colors {
        println!("{color} is a great color!")
    }

    // Generics
    let month_days: std::ops::Range<i32> = 1..31;
    let letters: std::ops::Range<char> = 'b'..'f';
}

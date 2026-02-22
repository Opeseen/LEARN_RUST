fn main() {
    //immutable & mutable ref params
    let mut current_meal = String::new();
    add_flour(&mut current_meal);
    show_my_meal(&current_meal);

    // multiple immutable ref
    let car = String::from("Red");
    let ref1 = &car;
    let ref2 = &car;
    println!("{ref1} and {ref2} and {}", &car);

    // mutable ref restrictions
    let mut car = String::from("Red");
    let ref1 = &mut car; // borrowed car
    ref1.push_str(" and silver");
    let ref2 = &car;
    println!("{ref2}");

    // ownership with mutable & immutable ref
    let mut coffee = String::from("Mocha");
    let a = &mut coffee;
    println!("{a}");
    let b = a;
    // println!("{a} and {b}"); // not valid here because "a" is no more available.
    println!("{b}");
    println!("");

    // dangling references
    let city = create_city();
    println!("{city}");

    // ownership with arrays and tuples
    let registrations = [true, false, true];
    let first = registrations[0];
    println!("{first} and {registrations:?}");

    let languages = [String::from("Rust"), String::from("JavaScript")];
    let first = &languages[0];
    println!("{first} and {registrations:?}");
    println!("");

    let registrations = (true, false, true);
    let first = registrations.0;
    println!("{first} and {registrations:?}");

    let languages = (String::from("Rust"), String::from("JavaScript"));
    let first = &languages.0;
    println!("{first} and {registrations:?}");
}

fn add_flour(meal: &mut String) {
    meal.push_str("Add flour");
}

fn show_my_meal(meal: &String) {
    println!("Meal steps: {meal}");
}

fn create_city() -> String {
    let city = String::from("New York");
    city
}

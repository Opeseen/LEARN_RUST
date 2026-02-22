fn main() {
    // copy trait
    let time = 2025;
    let year = time;

    // string type
    let text = String::new();
    let candy = String::from("Hello");

    // push_str_method
    let mut name = String::from("Boris");
    name.push_str(" Parker");
    println!("{name}");

    // moves & ownership
    let person = String::from("Boris");
    let genius = person;

    // drop func
    let person = String::from("Boris");
    drop(person);

    // clone func
    let person = String::from("Boris");
    let genius = person.clone();
    println!("{person}");

    // reference & borrowing
    let my_stack_value = 2;
    let my_integer_reference = &my_stack_value;
    let my_heap_value = String::from("Toyota");
    let my_heap_reference = &my_heap_value;

    // deference
    println!("{}", *my_integer_reference);
    println!("{}", *my_heap_reference);
    println!("{}", my_heap_reference);
    println!("");

    // string, &string, str, &str
    let ice_cream = "Cookies and Cream";
    println!("{}", ice_cream);

    // copy trait references
    let ice_cream = "Cookies and Cream";
    let desert = ice_cream;
    println!("{ice_cream}, {desert}");

    // ownership & func parameters
    let apples = 6;
    print_my_value(apples);
    println!("{apples} is still valid");
    let oranges = String::from("Oranges");
    my_value(oranges);
    // println!("{oranges} is still valid"); // -> ownership has been moved

    // mutable parameters
    let burger = String::from("Burger");
    add_fruits(burger); // let meal = burger

    // return values
    let cake = bake_cake();
    println!("I now have a {cake} cake");
    let mut current_meal = String::new();
    current_meal = add_flour(current_meal);
    println!("{current_meal}");
}

fn print_my_value(value: i32) {
    println!("Your value is {value}");
}

fn my_value(value: String) {
    println!("Your value is {value}");
}

fn add_fruits(mut meal: String) {
    meal.push_str(" and fruits");
    println!("{meal}");
}

fn bake_cake() -> String {
    String::from("Chocolate Mouse")
}

fn add_flour(mut meal: String) -> String {
    meal.push_str("Add flour");
    meal
}

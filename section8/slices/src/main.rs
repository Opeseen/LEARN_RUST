fn main() {
    // create slice from string
    let action_hero = String::from("Arnold Schwarzenegger");
    let first_name = &action_hero[0..6];
    println!("{first_name}");
    let last_name = &action_hero[7..21];
    println!("{last_name}");

    // string slices & literals
    let first_name = {
        let action_hero = "Arnold Schwarzenegger";
        &action_hero[0..6]
    };
    println!("{first_name}");

    // string slice length
    let food = "🍕";
    println!("{}", food.len());

    // syntactic shortcut
    let action_hero = String::from("Arnold Schwarzenegger");
    let first_name = &action_hero[..6];
    println!("The firstName name is {}", first_name);
    let last_name = &action_hero[7..];
    println!("The lastName name is {}", last_name);

    let full_name = &action_hero[..];
    println!("The fullName is: {}", full_name);

    // string slices in function params
    let action_hero: String = String::from("Arnold Schwarzenegger");
    do_hero_stuff(&action_hero);
    let another_action_hero = "Sylvester Stallone";
    do_hero_stuff(another_action_hero);

    // array slices
    let values = [4, 8, 15, 16, 23, 42];
    let my_slice = &values[0..4];
    println!("{my_slice:?}");
    let my_slice = &values[2..4];
    println!("{my_slice:?}");
    let my_slice = &values;
    println!("{my_slice:?}");

    // deref coercion with array slices
    let values = [4, 8, 15, 16, 23, 42];
    let regular_references = &values;
    print_length(regular_references);
    let slice_of_three = &values[..3];
    print_length(slice_of_three);

    // mutable array slices
    let mut my_array = [10, 15, 20, 25, 30];
    let my_slice = &mut my_array[2..4];
    println!("My slice: {:?}", my_slice);
    my_slice[0] = 100;
    println!("My slice: {:?}", my_slice);
    println!("My array: {:?}", my_array);
}

fn print_length(reference: &[i32]) {
    println!("{}", reference.len());
}

fn do_hero_stuff(hero_name: &str) {
    println!("{hero_name} saves the day");
}

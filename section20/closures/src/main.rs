use std::io::stdin;

fn main() {
    let multiplier = 5;

    let multiply_by = |value: i32| -> i32 {
        return value * multiplier;
    };

    println!("{}", multiply_by(5));

    let product = |a: i32, b: i32| -> i32 { return a * b };
    println!("{}", product(3, 10));
    println!("{}", product(5, 8));
    println!("");

    // closure shortcut
    let multiply_by = |value| value * multiplier;
    println!("{}", multiply_by(3));

    let mirror = |value| value;
    println!("{}", mirror("why"));
    println!("");

    // closures trait that captures immutable reference
    let numbers = vec![4, 8, 15, 16, 23, 42];
    println!("{:?}", numbers);

    let print_numbers = || println!("{:?}", numbers);
    print_numbers();
    println!("{:?}", numbers);
    println!("");

    // closures trait that captures mutable reference
    let mut numbers = vec![4, 8, 15, 16, 23, 42];
    let mut add_number = || numbers.push(100);
    add_number();
    println!("{:?}", numbers);
    println!("");

    // closure with ownership
    let numbers = 13;
    let capture_number = || numbers;
    let a = capture_number();
    let b = capture_number();
    println!("{a} {b} {numbers}");

    let first_name = String::from("Alice");
    let capture_string = || first_name;
    let first_name = String::from("Alice");
    let capture_string = || {
        let person = first_name;
        println!("{person}");
    };
    capture_string();
    // capture_string(); // cannot run twice as its impl FnOnce

    println!("");
    // the move keyword
    let first_name = String::from("Alice");
    let last_name = String::from("Wonder");
    let capture_string = move || {
        println!("{first_name} {last_name}");
    };
    // println!("{first_name} {last_name}");
    capture_string();
    capture_string();
    println!("");

    // the unwrap or_else method
    let options = Some("Salami");
    let food = options.unwrap_or_else(|| "Pizza");
    println!("{food}");

    let options: Option<&str> = None;
    let pizza_fan = false;
    let closure = || if pizza_fan { "Pizza" } else { "Hot Packets" };
    let food = options.unwrap_or_else(closure);
    println!("{food}");
    println!("");

    let vault = Vault {
        password: String::from("topsecret"),
        treasure: String::from("Gold"),
    };

    let hack = || {
        let mut user_input = String::new();
        println!("Please provide a password to crack the vault");
        stdin().read_line(&mut user_input);
        user_input.trim().to_string()
    };
    // let extraction = vault.unlock(hack);
    // println!("{:?}", extraction);
    println!("");

    // the string.retain method
    let mut game_console = String::from("PlayStation");
    let mut deleted_characters = String::new();
    // let closure = |character| character != 'a';
    let closure = |character| {
        let is_not_a = character != 'a';
        if is_not_a {
            true
        } else {
            deleted_characters.push(character);
            false
        }
    };
    game_console.retain(closure);
    println!("{game_console}");
    println!("{deleted_characters}");
    println!("");

    // defining a method that accept a closure
    let locations = [
        Location {
            name: String::from("Enchanted Forest"),
            treasures: 5,
        },
        Location {
            name: String::from("Mystic Mountains"),
            treasures: 10,
        },
    ];
    let map = Map {
        locations: &locations,
    };
    let mut total_treasures = 0;

    map.explore(|location| {
        total_treasures += location.treasures;
    });
    println!("Total treasures collected: {total_treasures}");

    let mut location_names: Vec<String> = Vec::new();
    map.explore(|location| {
        location_names.push(location.name.clone());
    });
    println!("{location_names:?}");
    println!("");

    // the fn trait
    let closure = || println!("I'm the boss");
    execute_thrice(closure);

    let mut bosses = vec!["Boris"];
    let closure = || {
        bosses.push("Alexander");
    };
    execute_thrice2(closure);
    println!("{bosses:?}");
    println!("");

    // passing in a function to fn trait parameter
    execute_thrice2(bake_cake);
    let options: Option<Vec<String>> = None;
    let collections = options.unwrap_or_else(Vec::new);
    println!("{:?}", collections);
}

fn bake_cake() {
    println!("Hello chocolate");
}

fn execute_thrice<F>(procedure: F)
where
    F: Fn(),
{
    procedure();
    procedure();
    procedure();
}

fn execute_thrice2<F>(mut procedure: F)
where
    F: FnMut(),
{
    procedure();
    procedure();
    procedure();
}

#[derive(Debug)]
struct Location {
    name: String,
    treasures: u32,
}

struct Map<'a> {
    locations: &'a [Location],
}

impl<'a> Map<'a> {
    fn explore<F>(&self, mut action: F)
    where
        F: FnMut(&Location),
    {
        let final_index = self.locations.len() - 1;
        let mut current_index = 0;
        while current_index <= final_index {
            let current_location = &self.locations[current_index];
            action(current_location);
            current_index += 1;
        }
    }
}

#[derive(Debug)]
struct Vault {
    password: String,
    treasure: String,
}

impl Vault {
    fn unlock<F>(self, procedure: F) -> Option<String>
    where
        F: FnOnce() -> String,
    {
        let user_password = procedure();
        if user_password == self.password {
            Some(self.treasure)
        } else {
            None
        }
    }
}

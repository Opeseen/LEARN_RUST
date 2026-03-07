fn main() {
    println!("{}", identity(5));
    println!("{}", identity(13.14));
    println!("{}", identity(String::from("Hello")));
    println!("{}", identity("World"));
    println!("{}", identity(true));
    println!("{:?}", identity(DeliSandwich {}));
    println!("");

    // turbofish operator
    // Used to provide our own custom type apart from the default
    println!("{}", identity::<f64>(5.3));
    println!("");

    // multiple generics
    make_tuple("Hello", 5);
    make_tuple2(5, "Hello");
    make_tuple2(5, 13);
    make_tuple2(true, 3.14);
    println!("");

    // generics in struct
    let gold_chest = TreasureChest {
        captain: String::from("Forebear"),
        treasure: "Gold",
    };
    println!("{:?}", gold_chest);
    let silver_chest = TreasureChest {
        captain: String::from("BloodSail"),
        treasure: String::from("Silver"),
    };
    println!("{:?}", silver_chest);
    let special_chest = TreasureChest {
        captain: String::from("BootyPlunder"),
        treasure: ["Gold", "Silver", "Platinum"],
    };
    println!("{:?}", special_chest);
    println!("");

    // generics and impl block
    let mut silver_chest = TreasureChest {
        captain: String::from("BloodSail"),
        treasure: String::from("       Silver         "),
    };
    silver_chest.clean_treasure();
    println!("{:?}", silver_chest);

    let special_chest = TreasureChest {
        captain: String::from("BootyPlunder"),
        treasure: ["Gold", "Silver", "Platinum"],
    };
    println!("{}", special_chest.amount_of_treasure());
    println!("{:?}", special_chest);

    println!("");
    println!("{}", gold_chest.capital_captain());
    println!("{}", silver_chest.capital_captain());
    println!("{}", special_chest.capital_captain());
    println!("");

    // generics in enum
    let mushroom = Cheesesteak::Topping("mushroom");
    let onions = Cheesesteak::Topping("onions".to_string());
    let topping = "bacon".to_string();
    let bacon = Cheesesteak::Topping(&topping);

    let mut plain: Cheesesteak<String> = Cheesesteak::Plain;
    plain = Cheesesteak::Topping("sausage".to_string());
    println!("{plain:?}");
}

#[derive(Debug)]
enum Cheesesteak<T> {
    Plain,
    Topping(T),
}

impl<T> TreasureChest<T> {
    fn capital_captain(&self) -> String {
        self.captain.to_uppercase()
    }
}

impl TreasureChest<String> {
    fn clean_treasure(&mut self) {
        self.treasure = self.treasure.trim().to_string();
    }
}

impl TreasureChest<[&str; 3]> {
    fn amount_of_treasure(&self) -> usize {
        self.treasure.len()
    }
}

#[derive(Debug)]
struct TreasureChest<T> {
    captain: String,
    treasure: T,
}

fn make_tuple<T>(first: T, second: i32) -> (T, i32) {
    (first, second)
}

fn make_tuple2<T, U>(first: T, second: U) -> (T, U) {
    (first, second)
}

#[derive(Debug)]
struct DeliSandwich {}

fn identity<T>(value: T) -> T {
    value
}

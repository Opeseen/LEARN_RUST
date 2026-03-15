use std::collections::{HashMap, HashSet};
fn main() {
    let mut menu: HashMap<String, f64> = HashMap::new();
    menu.insert(String::from("Steak"), 29.99);
    menu.insert(String::from("Tuna"), 29.99);
    menu.insert(String::from("Burger"), 14.99);
    println!("{menu:?}");

    let mut country_capitals = HashMap::<&str, &str>::new();
    country_capitals.insert("France", "Paris");
    country_capitals.insert("Nigeria", "Abuja");
    println!("{:?}", country_capitals);
    println!("");

    let data = [("Bobby", 7), ("Grant", 7), ("Ben", 7)];
    let mut years_at_company = HashMap::from(data);
    println!("{:?}", years_at_company);
    let ben = years_at_company.remove("Ben");
    println!("{:?}", ben.unwrap());
    println!("{:?}", years_at_company);
    println!("");

    let mut coffee_pairings: HashMap<&str, &str> = HashMap::new();
    let drink = String::from("Latte");
    let milk = String::from("Oat Milk");
    coffee_pairings.insert(&drink, &milk);
    coffee_pairings.insert("Flat White", &"Almond Milk");
    println!("{:?}", coffee_pairings.len());
    println!("{:?}", coffee_pairings);
    println!("{milk} {drink}");
    println!("");

    let value = coffee_pairings["Flat White"];
    let value = coffee_pairings
        .get("Cappuccino")
        .copied()
        .unwrap_or("Unknown Milk");
    println!("{:?}", value);
    println!("");

    println!("{:?}", coffee_pairings);
    coffee_pairings.insert("Latte", "Pistachio Milk");
    println!("{:?}", coffee_pairings);
    println!("");

    coffee_pairings.entry("Latte").or_insert("Pistachio2 Milk");
    coffee_pairings
        .entry("Cappuccino")
        .or_insert("Cappuccino Milk");
    println!("{:?}", coffee_pairings);
    println!("");

    let mut concert_queue = HashSet::<&str>::new();
    println!("{:?}", concert_queue);

    concert_queue.insert("Molly");
    concert_queue.insert("Megan");
    concert_queue.insert("Megan");
    println!("{:?}", concert_queue);
    println!("{}", concert_queue.remove("Megan"));
    println!("{}", concert_queue.contains("Megan"));
    println!("{:?}", concert_queue.get("Megan"));
    println!("{:?}", concert_queue.get("Molly"));
    println!("");

    let mut movie_queue: HashSet<&str> = HashSet::new();

    concert_queue.insert("Boris");
    concert_queue.insert("Melissa");

    movie_queue.insert("Boris");
    movie_queue.insert("Phil");

    println!("{:?}", concert_queue.union(&movie_queue));
    println!("{:?}", movie_queue.difference(&concert_queue));
    println!("{:?}", concert_queue.difference(&movie_queue));
    println!("{:?}", movie_queue.symmetric_difference(&concert_queue));
    println!("{:?}", concert_queue.is_disjoint(&movie_queue));
    println!("{:?}", movie_queue.is_disjoint(&concert_queue));
    let mut attendees = HashSet::new();
    attendees.insert("Boris");
    println!("{:?}", attendees.is_subset(&concert_queue));
    println!("{:?}", concert_queue.is_superset(&attendees));
}

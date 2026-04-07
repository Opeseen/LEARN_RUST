use std::{collections::HashMap, fmt::Display, ops::Add, str::FromStr};
fn main() {
    // multiple trait bound
    let mut hotel = Hotel::new("The Luxe");
    let mut airbnb = AirbnB::new("Peter");
    mix_and_match(&mut hotel, &mut airbnb, "Pier");
    println!("{hotel:#?} {airbnb:#?}");
    println!("");

    // trait as function as return value
    let mut hotel = choose_best_place_to_stay();
    let mut airbnb = AirbnB::new("Peter");
    mix_and_match2(&mut hotel, &mut airbnb, "Pier");
    println!("");

    let hotel1 = Hotel::new(String::from("The Luxe1"));
    println!("{}", hotel1.summarize());
    let hotel2 = Hotel::new("The Golden Standard");
    println!("{}", hotel2.summarize());
    let hotel3 = Hotel::new(vec!["The Sweet Escape", "Hilton Edition"]);
    // println!("{}", hotel3.summarize()) // this will not compile because it doesn't impl it
    println!("");

    // preview of trait object
    let hotel = Hotel::new(String::from("The Luxe"));
    let airbnb = AirbnB::new("Peter");

    let stays: Vec<&dyn Description> = vec![&hotel, &airbnb];
    println!("{}", stays[0].get_description());
    println!("{}", stays[1].get_description());
    println!("");
    let mut hotel = Hotel::new(String::from("The Luxe"));
    let mut airbnb = AirbnB::new("Peter");
    let mut stays: Vec<&mut dyn Accommodation> = vec![&mut hotel, &mut airbnb];
    stays[0].book("Piers", 2);
    stays[1].book("Amanda", 3);
    println!("{:?}", hotel);
    println!("{:?}", airbnb);
    println!("");

    // trait in scope to use its definition
    let a = 5;
    let b = 10;
    let sum = a.add(b);
    println!("{sum}");
    let numeric_count = u64::from_str("5");
    println!("{:?}", numeric_count.unwrap());
    println!("");
}

fn choose_best_place_to_stay() -> impl Accommodation + Description {
    Hotel::new("The Luxe")
}

trait Description {
    fn get_description(&self) -> String {
        String::from("A wonderful place to stay")
    }
}

fn mix_and_match(
    first: &mut (impl Accommodation + Description),
    second: &mut impl Accommodation,
    guest: &str,
) {
    first.book(guest, 1);
    first.get_description();
    second.book(guest, 1);
}

fn mix_and_match2<T, U>(first: &mut T, second: &mut U, guest: &str)
where
    T: Accommodation + Description,
    U: Accommodation,
{
    first.book(guest, 1);
    first.get_description();
    second.book(guest, 1);
}

fn book_for_one_night<T: Accommodation + Description>(entity: &mut T, guest: &str) {
    entity.book(guest, 1);
}

trait Accommodation {
    fn book(&mut self, name: &str, night: u32);
}

#[derive(Debug)]
struct Hotel<T> {
    name: T,
    reservation: HashMap<String, u32>,
}

impl<T> Hotel<T> {
    fn new(name: T) -> Self {
        Self {
            name,
            reservation: HashMap::new(),
        }
    }
}

impl<T: Display> Hotel<T> {
    fn summarize(&self) -> String {
        format!("{}: {}", self.name, self.get_description())
    }
}

impl<T> Accommodation for Hotel<T> {
    fn book(&mut self, name: &str, nights: u32) {
        self.reservation.insert(name.to_string(), nights);
    }
}

impl<T> Description for Hotel<T> {}

#[derive(Debug)]
struct AirbnB {
    host: String,
    guests: Vec<(String, u32)>,
}

impl AirbnB {
    fn new(host: &str) -> Self {
        Self {
            host: host.to_string(),
            guests: vec![],
        }
    }
}

impl Accommodation for AirbnB {
    fn book(&mut self, name: &str, night: u32) {
        self.guests.push((name.to_string(), night));
    }
}

impl Description for AirbnB {
    fn get_description(&self) -> String {
        format!("Please, enjoy {}'s apartment", self.host)
    }
}

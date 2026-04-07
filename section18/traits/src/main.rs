use std::collections::HashMap;
fn main() {
    let hotel = Hotel::new("The Luxe");
    println!("{}", hotel.get_description());
    let airbnb = AirbnB::new("Peter");
    println!("{}", airbnb.get_description());
    println!();

    let mut hotel = Hotel::new("The Luxe");
    hotel.book("Piers", 5);
    println!("{:#?}", hotel);
    let mut airbnb = AirbnB::new("Peter");
    airbnb.book("Piers", 3);
    println!("{:#?}", airbnb);
    println!("");

    let hotel = Hotel::new("The Luxe");
    println!("{}", hotel.summarize());
    println!("");

    // trait for function parameter constraints
    let mut hotel = Hotel::new("The Luxe");
    book_for_one_night(&mut hotel, "Piers");
    println!("{:#?}", hotel);
    let mut airbnb = AirbnB::new("Peter");
    book_for_one_night(&mut airbnb, "Amanda");
    println!("{:#?}", airbnb);
    println!("");

    // Trait bound syntax
    let mut hotel = Hotel::new("The Luxe");
    book_for_one_night2(&mut hotel, "Piers");
    println!("{:#?}", hotel);
    let mut airbnb = AirbnB::new("Peter");
    book_for_one_night2(&mut airbnb, "Amanda");
    println!("{:#?}", airbnb);
    println!("");
    let mut hotel = Hotel::new("The Luxe");
    let mut airbnb = AirbnB::new("Peter");
    mix_and_match(&mut hotel, &mut airbnb, "Pier");
    println!("{hotel:#?} {airbnb:#?}");
    println!("");

    // multiple trait bound
}

trait Description {
    fn get_description(&self) -> String {
        String::from("A wonderful place to stay") // fallback to this 
    }
}

fn mix_and_match<T: Accommodation, U: Accommodation>(first: &mut T, second: &mut U, guest: &str) {
    first.book(guest, 1);
    second.book(guest, 1);
}

fn book_for_one_night(entity: &mut impl Accommodation, guest: &str) {
    println!("{}", entity.get_description());
    entity.book(guest, 1);
}

fn book_for_one_night2<T: Accommodation>(entity: &mut T, guest: &str) {
    entity.book(guest, 1);
}

trait Accommodation {
    fn get_description(&self) -> String {
        String::from("A wonderful place to stay") // fallback to this 
    }
    fn book(&mut self, name: &str, night: u32);
}

#[derive(Debug)]
struct Hotel {
    name: String,
    reservation: HashMap<String, u32>,
}

impl Hotel {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            reservation: HashMap::new(),
        }
    }
    fn summarize(&self) -> String {
        format!("{}: {}", self.name, self.get_description())
    }
}

impl Accommodation for Hotel {
    // This will be override by the default it not present
    fn get_description(&self) -> String {
        format!("{} is the pinnacle of luxury", self.name)
    }

    fn book(&mut self, name: &str, nights: u32) {
        self.reservation.insert(name.to_string(), nights);
    }
}

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
    fn get_description(&self) -> String {
        format!("Please, enjoy {}'s apartment", self.host)
    }
    fn book(&mut self, name: &str, night: u32) {
        self.guests.push((name.to_string(), night));
    }
}

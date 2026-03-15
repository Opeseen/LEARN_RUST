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
}

trait Accommodation {
    fn get_description(&self) -> String;
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
}

impl Accommodation for Hotel {
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

use std::{collections::HashMap, fmt::Display};

pub trait Description {
    fn get_description(&self) -> String {
        String::from("A wonderful place to stay")
    }
}

pub trait Accommodation {
    fn book(&mut self, name: &str, night: u32);
}
#[derive(Debug)]
pub struct Hotel<T> {
    name: T,
    reservation: HashMap<String, u32>,
}

#[derive(Debug)]
pub struct AirbnB {
    host: String,
    guests: Vec<(String, u32)>,
}

impl<T> Hotel<T> {
    pub fn new(name: T) -> Self {
        Self {
            name,
            reservation: HashMap::new(),
        }
    }
}

impl<T: Display> Hotel<T> {
    pub fn summarize(&self) -> String {
        format!("{}: {}", self.name, self.get_description())
    }
}

impl<T> Accommodation for Hotel<T> {
    fn book(&mut self, name: &str, nights: u32) {
        self.reservation.insert(name.to_string(), nights);
    }
}

impl<T> Description for Hotel<T> {}

impl AirbnB {
    pub fn new(host: &str) -> Self {
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

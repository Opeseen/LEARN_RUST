use core::fmt;
use std::fmt::{Debug, Display, Formatter, Result};

fn main() {
    let mut latte = Coffee::new("Latte", Milk::Almond, 2);
    println!("{:?}", latte);
    latte.consume();
    println!("{:?}", latte);
    println!("");

    let cappuccino = Coffee::new(String::from("Cappuccino"), Milk::Whole, 3);
    println!("{}", cappuccino.get_data());
    println!();

    let pepsi = Soda::new(4, 67.00, String::from("Cherry"));
    println!("{}", pepsi);
    println!("");
    let mut coke = pepsi.clone();
    println!("{:?}", coke);
    println!("{}", pepsi == coke);
    coke.consume();
    println!("{:?}", coke);
}

trait Drinkable {
    fn consume(&mut self);
    fn get_data(&self) -> String;
    fn stats(&self) {
        println!("{}", self.get_data())
    }
}

#[derive(Debug)]
enum Milk {
    Whole,
    Oat,
    Almond,
}

struct Coffee<T> {
    kind: T,
    milk: Milk,
    ounces: u32,
}

#[derive(Debug)]
struct Soda {
    calories: u32,
    price: f64,
    flavor: String,
    percentage: u32,
}

impl<T> Coffee<T> {
    fn new(kind: T, milk: Milk, ounces: u32) -> Self {
        Self { kind, milk, ounces }
    }
}

impl<T: Debug> Debug for Coffee<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("** COFFEE **")
            .field("Kind", &self.kind)
            .field("Milk", &self.milk)
            .field("Ounces", &self.ounces)
            .finish()
    }
}

impl<T: Display> Drinkable for Coffee<T> {
    fn consume(&mut self) {
        self.ounces = 0
    }
    fn get_data(&self) -> String {
        format!("A Delicious {} ounce {}", self.ounces, self.kind)
    }
}

impl Soda {
    fn new(calories: u32, price: f64, flavor: String) -> Self {
        Self {
            calories,
            price,
            flavor,
            percentage: 100,
        }
    }
}

impl Drinkable for Soda {
    fn consume(&mut self) {
        self.percentage = 0
    }
    fn get_data(&self) -> String {
        format!("Flavor: {}, Calories: {}", self.flavor, self.calories)
    }
}
impl Display for Soda {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "** {} Soda **", self.flavor) // user facing string should look like this
    }
}

impl Clone for Soda {
    fn clone(&self) -> Self {
        Self {
            calories: self.calories,
            price: self.price,
            flavor: self.flavor.clone(),
            percentage: self.percentage,
        }
    }
}

impl PartialEq for Soda {
    fn eq(&self, other: &Self) -> bool {
        self.price == other.price
    }
}

impl Eq for Soda {}

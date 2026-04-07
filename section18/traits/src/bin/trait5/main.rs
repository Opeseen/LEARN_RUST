use core::fmt;
use std::fmt::{Debug, Display, Formatter, Result};
use std::ops::{Add, Drop};
use std::{clone::Clone, cmp::Ordering, fs};

fn main() {
    let mut income = Income { amount: 50000.00 };
    println!("Total tax owed: ${:.2}", income.tax_bill());
    income.double_amount();
    println!("Total tax owed: ${:.2}", income.tax_bill());
    println!("");

    let mut bonus = Bonus { value: 10000.23 };
    println!("Bonus tax owned: ${:.2}", bonus.tax_bill());
    bonus.double_amount();
    println!("Bonus tax owned: ${:.2}", bonus.tax_bill());
    println!("");

    let mut weekend = QualityTime { minutes: 120 };
    weekend.double_amount();
    println!("Relaxation time: {:.2}", weekend.amount());
    println!("");

    let lunch_snack = Apple {
        kind: AppleType::GrannySmith,
        price: 1.04,
    };
    println!("{}", lunch_snack);

    let dinner_snack = Apple {
        kind: AppleType::RedDelicious,
        price: 1.06,
    };
    println!("{}", dinner_snack);
    println!("");

    println!("{:?}", lunch_snack);
    println!("{:?}", dinner_snack);

    println!("");

    let morning_appt = Appointment::new("Dr. Andrews", "9:00AM", "10:00AM");
    let replacement_appt = morning_appt.clone();
    println!(
        "{} is seeing the patient from {} to {}",
        replacement_appt.doctor, replacement_appt.start_time, replacement_appt.end_time
    );
    println!("{morning_appt:?}");
    println!("");

    let one_hour = Duration::new(1, 0, 0);
    let another_hour = one_hour;
    println!("{:?}", one_hour);
    println!("");

    let a = Flight::new("New York", "London", "08:00");
    let b = Flight::new("New York", "London", "23:30");
    let c = Flight::new("New York", "Los Angeles", "08:00");
    println!("{}", a == b);
    println!("{}", a.eq(&b));
    println!("");

    let b = BusTrip::new("Los Angeles", "Tokyo", "08:00");
    println!("{}", a == b);
    println!("{}", b == a);
    println!("");

    let rustin_bieber = SingerSongWriter("Rustin".to_string());
    let rustin_timberlake = SingerSongWriter("Rustin".to_string());
    let holly = SingerSongWriter("Holly".to_string());

    let rust_no_one = Band(5);
    let untrustworthy = Band(4);
    let rust_for_vengeance = Band(5);

    println!("{}", rustin_bieber == holly);
    println!("{}", rustin_bieber == rustin_timberlake);
    println!("{}", rustin_bieber == rust_no_one);
    println!("{}", rust_no_one == untrustworthy);
    println!("{}", rust_no_one == rust_for_vengeance);
    println!("");

    let division = 0.0 / 0.0;
    println!("{}", division);

    let a = Flight::new("New York", "London", "08:00");
    let b = Flight::new("New York", "London", "08:00");
    let c = Flight::new("New York", "London", "08:00");

    println!("{}", a == a);
    println!("{}", a == b);
    println!("{}", b == a);
    println!("{}", b == c);

    println!("");
    let long_commute_job = Job {
        salary: 100000,
        commute_time: 2,
    };

    let short_commute_job = Job {
        salary: 73900,
        commute_time: 1,
    };

    println!("{}", long_commute_job > short_commute_job);
    println!("{}", long_commute_job < short_commute_job);
    println!("{}", long_commute_job == short_commute_job);
    println!("{}", long_commute_job.ge(&short_commute_job));
    println!("");

    let one = Launch { cost: 19.99 };
    let two = Launch { cost: 29.99 };
    println!("{:?}", one + two);
    println!("");

    let integer_sum = add_two_numbers(1, 2);
    let float_sum = add_two_numbers(1.5, 2.4);
    println!("{integer_sum} and {float_sum}");
}

fn add_two_numbers<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

impl Add for Launch {
    type Output = Launch;
    // type Output = f64;
    fn add(self, rhs: Self) -> Self::Output {
        // self.cost + rhs.cost
        Self {
            cost: self.cost + rhs.cost,
        }
    }
}

#[derive(Debug)]
struct Launch {
    cost: f64,
}

struct Job {
    salary: u32,
    commute_time: u32,
}

impl PartialEq for Job {
    fn eq(&self, other: &Self) -> bool {
        self.salary == other.salary
    }
}

impl Eq for Job {}

impl PartialOrd for Job {
    // works on u32 (Integer)
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.salary.partial_cmp(&other.salary)
    }

    // fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    //     if self.salary == other.salary {
    //         Some(Ordering::Equal)
    //     } else if self.salary < other.salary {
    //         Some(Ordering::Less)
    //     } else if self.salary > other.salary {
    //         Some(Ordering::Greater)
    //     } else {
    //         None
    //     }
    // }
}

impl PartialEq for Musician {
    fn eq(&self, other: &Self) -> bool {
        match self {
            SingerSongWriter(name) => match other {
                SingerSongWriter(other_name) => name == other_name,
                Band(_) => false,
            },
            Band(members) => match other {
                SingerSongWriter(_) => false,
                Band(other_members) => members == other_members,
            },
        }
    }
}
// #[derive(PartialEq)]
enum Musician {
    SingerSongWriter(String),
    Band(u32),
}

use Musician::{Band, SingerSongWriter};

struct BusTrip {
    origin: String,
    destination: String,
    time: String,
}

impl BusTrip {
    fn new(origin: &str, destination: &str, time: &str) -> Self {
        Self {
            origin: origin.to_string(),
            destination: destination.to_string(),
            time: time.to_string(),
        }
    }
}

// #[derive(PartialEq)]
struct Flight {
    origin: String,
    destination: String,
    time: String,
}

impl Flight {
    fn new(origin: &str, destination: &str, time: &str) -> Self {
        Self {
            origin: origin.to_string(),
            destination: destination.to_string(),
            time: time.to_string(),
        }
    }
}

// custom impl for partialEq
impl PartialEq for Flight {
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin && self.destination == other.destination
    }
}

impl PartialEq<BusTrip> for Flight {
    fn eq(&self, other: &BusTrip) -> bool {
        self.time == other.time
    }
}

impl PartialEq<Flight> for BusTrip {
    fn eq(&self, other: &Flight) -> bool {
        self.time == other.time
    }
}

impl Copy for Duration {}

#[derive(Debug, Clone)]
struct Duration {
    hours: u32,
    minutes: u32,
    seconds: u32,
}

impl Duration {
    fn new(hours: u32, minutes: u32, seconds: u32) -> Self {
        Self {
            hours,
            minutes,
            seconds,
        }
    }
}

impl Clone for Appointment {
    fn clone(&self) -> Self {
        Self {
            doctor: self.doctor.clone(),
            start_time: self.start_time.clone(),
            end_time: self.end_time.clone(),
        }
    }
}

#[derive(Debug)]
struct Appointment {
    doctor: String,
    start_time: String,
    end_time: String,
}

impl Appointment {
    fn new(doctor: &str, start_time: &str, end_time: &str) -> Self {
        Self {
            doctor: doctor.to_string(),
            start_time: start_time.to_string(),
            end_time: end_time.to_string(),
        }
    }
}

// impl Drop for Apple {
//     fn drop(&mut self) {
//         println!("Apple is been cleanup!");
//         match fs::remove_file("apple.txt") {
//             Ok(_) => println!("Goodbye my sweet apple"),
//             Err(error) => eprintln!("Error deleting file: {error}"),
//         }
//     }
// }

impl Debug for AppleType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            AppleType::RedDelicious => write!(f, "AppleType::RedDelicious"),
            AppleType::GrannySmith => write!(f, "AppleType::GrannySmith"),
        }
    }
}

impl Debug for Apple {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // write!(
        //     f,
        //     "Apple ::: [kind: {:?}, price: {} ]",
        //     self.kind, self.price
        // )
        f.debug_struct("** Apple **")
            .field("kind", &self.kind)
            .field("cost", &self.price)
            .finish()
    }
}

impl Display for Apple {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} for {}", self.kind, self.price)
    }
}

impl Display for AppleType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            AppleType::RedDelicious => write!(f, "🍎 Delicious 🍎"),
            AppleType::GrannySmith => write!(f, "🍏 Granny Smith 🍏"),
        }
    }
}

enum AppleType {
    RedDelicious,
    GrannySmith,
}
struct Apple {
    kind: AppleType,
    price: f64,
}

trait Investment<T> {
    fn amount(&self) -> T; // getter

    fn double_amount(&mut self);
}

// Investment is a parent to taxable
trait Taxable: Investment<f64> {
    const TAX_RATE: f64 = 0.25;

    fn tax_bill(&self) -> f64 {
        self.amount() * Self::TAX_RATE
    }
}

#[derive(Debug)]
struct Income {
    amount: f64,
}
#[derive(Debug)]
struct Bonus {
    value: f64,
}
#[derive(Debug)]
struct QualityTime {
    minutes: u32,
}

// This impl from parent but not child (Taxable)
impl Investment<u32> for QualityTime {
    fn amount(&self) -> u32 {
        self.minutes
    }

    fn double_amount(&mut self) {
        self.minutes *= 2;
    }
}

impl Investment<f64> for Income {
    fn amount(&self) -> f64 {
        self.amount
    }
    fn double_amount(&mut self) {
        self.amount *= 2.0
    }
}

impl Taxable for Income {}

impl Investment<f64> for Bonus {
    fn amount(&self) -> f64 {
        self.value
    }
    fn double_amount(&mut self) {
        self.value *= 2.0
    }
}

impl Taxable for Bonus {
    const TAX_RATE: f64 = 0.50;
}

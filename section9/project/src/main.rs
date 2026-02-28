#[derive(Debug)]
struct Flight {
    origin: String,
    destination: String,
    price: f64,
    passengers: u32,
}

impl Flight {
    fn new(origin: String, destination: String, price: f64, passengers: u32) -> Self {
        Self {
            origin,
            destination,
            price,
            passengers,
        }
    }

    fn change_destination(&mut self, new_destination: String) {
        self.destination = new_destination;
    }

    fn increase_price(&mut self) {
        self.price *= 1.2;
    }

    fn itinerary(&self) {
        println!("{} -> {}", self.origin, self.destination);
    }
}

fn main() {
    let flight_details = Flight::new(String::from("Nigeria"), String::from("USA"), 20.8, 12);
    println!("{:#?}", flight_details);
    println!("");
    let mut flight_details = Flight::new(String::from("Nigeria"), String::from("USA"), 20.8, 12);
    // flight_details
    //     .change_destination(String::from("GHANA"))
    //     .increase_price()
    //     .itinerary();
    flight_details.change_destination(String::from("GHANA"));
    flight_details.increase_price();
    flight_details.itinerary();
    println!("{:#?}", flight_details);
    println!("");

    let another_flight = Flight {
        origin: String::from("Paris"),
        destination: String::from("Rome"),
        ..flight_details
    };
    println!("{:#?}", another_flight);
}

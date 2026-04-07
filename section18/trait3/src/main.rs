use trait3::lodging::{Accommodation, AirbnB, Description, Hotel};
use trait3::utils;
fn main() {
    // multiple trait bound
    let mut hotel = Hotel::new("The Luxe");
    println!("{}", hotel.summarize());
    hotel.book("Dana", 5);

    let mut airbnb = AirbnB::new("Peter");
    println!("{}", airbnb.get_description());
    utils::book_for_one_night(&mut airbnb, "Dan");

    utils::mix_and_match(&mut hotel, &mut airbnb, "Pier");
}

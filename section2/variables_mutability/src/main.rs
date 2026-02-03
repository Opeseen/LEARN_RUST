#![allow(unused_assignments)] // allow at all files at the global level with the use of "!"

const TAX_RATE: f64 = 7.25; // constant can be declared at the global scope be the main global function

type Meters = i32;
#[allow(unused_variables)] // apply to the next line
fn main() {
    let apples = 50;
    let oranges = 14 + 6;
    let _fruits = apples + oranges;

    println!("This year, my garden has {} apples.", apples);
    println!("This year, may gardens has {apples} apples and {oranges} oranges.");
    println!(
        "This year, my garden has {0} apples and {1} oranges, I can't believe have {0} apples",
        apples, oranges
    );

    let mut gym_reps = 10;
    println!("I plan to do {gym_reps} reps");
    gym_reps = 15;
    println!("I now plan to do {gym_reps} reps");
    println!();

    let grams_of_protein = "100.345";
    let grams_of_protein = 100.345;
    let mut grams_of_protein = 100;
    grams_of_protein = 105;
    println!("grams is: {}", grams_of_protein);
    println!();

    let coffee_price = 5.99;
    {
        let cookie_price = 1.99;
        let coffee_price = 2.00;
        println!("The cookie price is {coffee_price}");
    }
    println!("The price is {coffee_price}");

    let income = 100000;
    println!("My income is {income} and my tax rate is {TAX_RATE}");

    let mile_race_length: Meters = 1600;
    let two_mile_race_length: Meters = 3200;
    println!(
        "A ome mile race is {mile_race_length} meters long and a two mile race is {two_mile_race_length} meters long"
    );

    #[allow(unused_variables)]
    let mile_race_length_1: Meters = 1600;
    let two_mile_race_length_2: Meters = 3200;
}

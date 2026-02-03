#![allow(unused_assignments)]
const TOUCHDOWN_POINTS: i32 = 6;
#[allow(unused_variables)]
fn main() {
    let season = "game of thrown";
    let mut pointer_scored = 28;
    pointer_scored = 35;
    let event_time = "06.00";
    let event_time = 6;

    println!(
        "i love watching {season} movies and the 
    recent pointer score is {pointer_scored} with an event time of {event_time} 
    and a touch down point of {TOUCHDOWN_POINTS}",
    );
    println!();
    println!(
        "i love watching {0} movies and the 
    recent pointer score is {1} with an event time of {2} 
    and a touch down point of {3}",
        season, pointer_scored, event_time, TOUCHDOWN_POINTS,
    );
}

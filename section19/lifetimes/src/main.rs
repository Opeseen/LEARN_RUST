const COUNT: i32 = 400;

fn main() {
    let cities = vec![
        String::from("London"),
        String::from("New York"),
        String::from("Barcelona"),
    ];
    let two_cities = select_first_two_elements(&cities);
    println!("{two_cities:?}");

    {
        let coffee = [String::from("Latte"), String::from("Mocha")];
        let two_coffee = select_first_two_elements(&coffee);
        println!("{two_coffee:?}");
    }
    println!("");
    let two_cities = {
        let cities_reference = &cities;
        select_first_two_elements(&cities_reference)
    };
    println!("{two_cities:?}");
    println!("");

    // let orlando = String::from("Orlando");
    // let san_francisco = String::from("San Francisco");
    // let result = longest(&orlando, &san_francisco);
    // println!("{result}");
    // println!("");

    // let orlando = String::from("Orlando");
    // let result = {
    //     let san_francisco = String::from("San Francisco");
    //     longest(&orlando, &san_francisco) // this will not make it compile as it will lead to dangling reference
    // };
    // println!("{orlando}");

    println!("");

    let orlando = String::from("Orlando");
    let result = {
        let san_francisco = String::from("San Francisco");
        longest2(&orlando, &san_francisco)
    };
    println!("{orlando}");
    println!("");

    let appt = DentistAppointment {
        doctor: String::from("David"),
    };
    let result = appt.book("03:00PM", "04:00");
    drop(appt);
    println!("{result}");
    println!("");

    // lifetimes in struct
    let name = String::from(" NJ Transit");
    let nj_transit = { TrainSystem { name: &name } };
    println!("{:?}", nj_transit.name);
    println!("");

    // multiple lifetimes
    // let from = String::from("Portland");
    // let plan = {
    //     let to = String::from("Bangor");
    //     let travel_plan = TravelPlan {
    //         from: &from,
    //         to: &to,
    //     };
    //     travel_plan.from
    // };
    // println!("{plan}");

    let from = String::from("Portland");
    let plan = figure_out_ending_point(&from);
    println!("{plan}");
    println!("");

    // static lifetime
    let greeting = say_hello();
    println!("{greeting}");

    let count = value();
    println!("{count}");
}

fn value() -> &'static i32 {
    &COUNT
}

fn say_hello() -> &'static str {
    "Hello"
}

fn figure_out_ending_point(from: &str) -> &str {
    let to = String::from("Bangor");
    let travel_plan = TravelPlan {
        from: &from,
        to: &to,
    };
    travel_plan.from
}

#[derive(Debug)]
struct TravelPlan<'b, 'a> {
    from: &'b str,
    to: &'a str,
}

#[derive(Debug)]
struct TrainSystem<'a> {
    name: &'a str,
}

struct DentistAppointment {
    doctor: String,
}

// impl DentistAppointment {
//     fn book(&self, check_in_time: &str, check_out_time: &str) -> &str {
//         println!(
//             "You are booked from {} to {} with doctor {}",
//             check_in_time, check_out_time, self.doctor
//         );
//         &self.doctor
//     }
// }

impl DentistAppointment {
    fn book<'a>(&self, check_in_time: &'a str, check_out_time: &str) -> &'a str {
        println!(
            "You are booked from {} to {} with doctor {}",
            check_in_time, check_out_time, self.doctor
        );
        check_in_time
    }
}

fn longest<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.len() > second.len() {
        first
    } else {
        second
    }
}

fn longest2<'a, 'b>(first: &'a str, second: &'b str) -> &'a str {
    println!("The second is {second}");
    first
}

fn choose_favorite<'a>(first: &'a str, second: &str) -> &'a str {
    println!("{second}");
    first
}

fn select_first_two_elements<'a>(items: &'a [String]) -> &'a [String] {
    &items[..2]
}

// this works because we pass in a ref and return a ref
// fn select_first_two_elements(items: &[String]) -> &[String] {
//     &items[..2]
// }

// fn select_first_two_elements(items: &[String]) {
//     let selected_items = &items[..2];
//     println!("{selected_items:?}");
// }

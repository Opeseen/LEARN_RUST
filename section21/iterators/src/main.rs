use std::collections::{HashMap, HashSet};
fn main() {
    // manual iteration
    let numbers = vec![4, 8, 15, 16, 23, 42];
    for num in numbers {
        println!("{num}");
    }
    println!("");
    let my_vector = vec![4, 8, 15, 16, 23, 42];
    let my_iterator = my_vector.into_iter();

    let my_vector = vec![false, true, false];
    let my_iterator = my_vector.into_iter();

    let mut my_hashmap = HashMap::new();
    my_hashmap.insert("CBS", 2);
    let my_iterator = my_hashmap.into_iter();
    println!("");

    // exhausting iterator
    let my_vector = vec![4, 8, 15, 16, 23, 42];
    let mut my_iterator = my_vector.into_iter();
    println!("{:?}", my_iterator.next());
    println!("{:?}", my_iterator.next());
    println!("{:?}", my_iterator.next());
    println!("{:?}", my_iterator.next());
    println!("{:?}", my_iterator.next());
    println!("{:?}", my_iterator.next());
    println!("{:?}", my_iterator.next());
    println!("{:?}", my_iterator.next());

    println!("{:?}", my_iterator);
    println!("");

    // for loop iterator
    let my_vector = vec![4, 8, 15, 16, 23, 42];
    let my_iterator = my_vector.into_iter();
    for number in my_iterator {
        println!("{number}");
    }
    println!("");
    // why iterator can be mutable
    let my_vector = vec![4, 8, 15, 16, 23, 42];
    let my_iterator = my_vector.into_iter();
    for number in my_iterator {
        println!("{number}");
    }
    println!("");
    // the iter method
    let my_vector = vec![4, 8, 15, 16, 23, 42];
    let my_iterator = my_vector.iter();
    for number in my_iterator {
        println!("{number}");
    }
    println!("{my_vector:?}");
    println!("");
    let my_vector = vec![4, 8, 15, 16, 23, 42];
    let my_iterator = &my_vector;
    for number in my_iterator {
        println!("{number}");
    }
    println!("{my_vector:?}");

    let cities = vec![String::from("Phoenix"), String::from("Dallas")];
    for city in cities.iter() {
        println!("{city}");
    }
    println!("{cities:?}");
    println!("");

    // the iter_mut method
    let mut flavours = [
        String::from("Chocolate"),
        String::from("Vanilla"),
        String::from("Strawberry"),
    ];
    let iterator = flavours.iter_mut();
    for flavor in iterator {
        flavor.push_str(" Ice Cream");
    }
    println!("{flavours:?}");
    for flavor in &mut flavours {
        flavor.push_str(" Ice Cream");
    }
    println!("{flavours:?}");
    println!("");

    let mut school_grades = [85, 90, 72, 92];
    for grade in &mut school_grades {
        *grade -= 2;
    }
    println!("{school_grades:?}");
    println!("");

    // hashmap iteration
    let mut todos = HashMap::new();
    todos.insert("Pick up groceries", false);
    todos.insert("Study Rust", true);
    todos.insert("Sleep", false);

    for (todo, completion_status) in &mut todos {
        *completion_status = true;
        println!("Task: {todo}. Complete: {completion_status}");
    }
    println!("{todos:?}");
    println!("");

    // string iteration
    let seafood = String::from("Oyster 🦪");
    for byte in seafood.bytes() {
        print!("{byte}/")
    }
    println!("");

    for character in seafood.chars() {
        print!("{character}/")
    }
    println!("{seafood}");
    println!("{}", seafood.bytes().len());
    println!("{}", seafood.chars().count());
    println!("");

    // solving problem with iteration
    println!(
        "{:?}",
        count_characters("Sally sells sea shells by the sea shore")
    );
    println!("");

    // for each method
    println!(
        "{:?}",
        count_characters2("Sally sells sea shells by the sea shore")
    );
    println!("");

    // the map method 1
    let numbers = vec![4, 8, 15, 16, 23, 42];

    let my_iterator = numbers.iter();
    let squares = my_iterator.map(|num: &i32| num.pow(2));
    println!("{squares:?}");

    for num in squares {
        println!("Square: {num}");
    }

    let my_iterator = numbers.into_iter();
    let squares = my_iterator.map(|num: i32| num.pow(2));
    println!("{squares:?}");

    for num in squares {
        println!("Square: {num}");
    }
    println!("");

    // the collect method
    let numbers = vec![4, 8, 15, 16, 23, 42];
    let squares: Vec<i32> = numbers.iter().map(|num: &i32| num.pow(2)).collect();
    println!("{squares:?}");
    println!("{numbers:?}");
    let squares: HashSet<i32> = numbers.iter().map(|num: &i32| num.pow(2)).collect();
    println!("{squares:?}");
    println!("");

    // the map method 2
    let names = [
        String::from("Jimmy"),
        String::from("Cleveland"),
        String::from("Boris"),
    ];
    let name_length: Vec<usize> = names
        .iter()
        .map(|name| name.to_string())
        .map(|name| name.replace("i", "@@"))
        .map(|name| name.len())
        .collect();
    println!("{name_length:?}");
    println!("");

    // the filter method
    let numbers = [10, 13, 23, 2, 8, 9, 6];
    let event: Vec<i32> = numbers.into_iter().filter(|num| num % 2 == 0).collect();
    println!("{event:?}");
    println!("{numbers:?}");
    let event: Vec<i32> = numbers
        .iter()
        .filter(|num| *num % 2 == 0)
        .copied()
        .collect();
    println!("{event:?}");
    println!("{numbers:?}");
    println!("");
    let first_even = numbers.into_iter().find(|num| num % 2 == 0);
    println!("{first_even:?}");
    let first_odd = numbers.into_iter().find(|num| num % 2 != 0);
    println!("{:?}", first_odd);

    let noting = numbers.into_iter().find(|num| *num > 100);
    println!("{noting:?}");

    let last_even: Option<i32> = numbers.into_iter().rfind(|num| num % 2 == 0);
    println!("{last_even:?}");
    println!("");

    // filter and find method 2
    let channels = [
        TVChannel {
            name: String::from("CBS"),
            channel_type: ChannelType::Comedy,
        },
        TVChannel {
            name: String::from("RustLive"),
            channel_type: ChannelType::ProgrammingTutorials,
        },
        TVChannel {
            name: String::from("NBC"),
            channel_type: ChannelType::News,
        },
        TVChannel {
            name: String::from("RustTV"),
            channel_type: ChannelType::ProgrammingTutorials,
        },
    ];
    let good_channels = channels
        .iter()
        .filter(|channel| channel.channel_type == ChannelType::ProgrammingTutorials)
        .collect::<Vec<&TVChannel>>();
    println!("{good_channels:?}");
    print!("");
    let good_channels = channels
        .iter()
        .filter(|channel| channel.channel_type == ChannelType::ProgrammingTutorials)
        .map(|channel| channel.name.clone())
        .collect::<Vec<String>>();
    println!("{good_channels:?}");

    let good_channel = channels
        .iter()
        .find(|channel| channel.channel_type == ChannelType::ProgrammingTutorials);
    match good_channel {
        Some(channel) => println!("Great choice to watch {channel:?}"),
        None => println!("There was no rust programming on the TV"),
    }
    println!("");
    // the any and all methods
    println!("{}", good_channel.is_some());
    let all_are_rust = channels
        .iter()
        .all(|channel| channel.channel_type == ChannelType::ProgrammingTutorials);
    println!("{all_are_rust}");
    let any_are_rust = channels
        .iter()
        .any(|channel| channel.channel_type == ChannelType::ProgrammingTutorials);
    println!("{any_are_rust}");
    println!("");

    // the cloned method
    let teas = [
        String::from("Hot Earl Grey"),
        String::from("Iced Cream"),
        String::from("Hot Matcha"),
    ];

    let more_teas: Vec<String> = teas.iter().cloned().collect();
    println!("{more_teas:?}");
    let more_teas: Vec<String> = teas.iter().map(|tea| tea.clone()).collect();
    // more efficient because it filter before it calls clone
    let more_teas: Vec<String> = teas
        .iter()
        .filter(|tea| tea.contains("Hot"))
        .cloned()
        .collect();
    println!("{more_teas:?}");
    // less efficient because it calls clone first
    let more_teas: Vec<String> = teas
        .iter()
        .cloned()
        .filter(|tea| tea.contains("Hot"))
        .collect();
    println!("{more_teas:?}");
    println!("");

    // the filter map method
    let stocks = ["nvda", "aapl", "msft", "goog"];
    let capitalized_stocks: Vec<String> = stocks
        .iter()
        .filter(|stock| !stock.is_empty())
        .map(|stock| stock.to_uppercase())
        .collect();
    println!("{:?}", capitalized_stocks);
    let capitalized_stocks: Vec<String> = stocks
        .iter()
        .filter_map(|stock| {
            if stock.is_empty() {
                None
            } else {
                Some(stock.to_uppercase())
            }
        })
        .collect();
    println!("{:?}", capitalized_stocks);
    println!("");

    // the flatten method
    let spreadsheet = vec![[100, 200, 300], [123, 456, 789], [987, 654, 321]];
    let values: Vec<i32> = spreadsheet.into_iter().flatten().collect();
    println!("{values:?}");
    println!("");

    // the flat map method
    let attendance = [
        "Bob, Mary, Kevin",
        "Mike, Robbie, Matt, Austin",
        "Piers, Liam",
    ];
    let attendees: Vec<&str> = attendance
        .iter()
        .map(|group| group.split(", "))
        .flatten()
        .collect();
    println!("{attendees:?}");
    let attendees: Vec<&str> = attendance
        .iter()
        .flat_map(|group| group.split(", "))
        .collect();
    println!("{attendees:?}");
    println!("");

    // the enumerate method
    let applicants = vec![
        "Bob", "Mary", "Kevin", "Mike", "Robbie", "Matt", "Austin", "Piers", "Liam",
    ];
    let winners = applicants
        .into_iter()
        .enumerate()
        .filter(|(index, _)| index % 3 == 0)
        .map(|(_, applicant)| applicant)
        .collect::<Vec<&str>>();
    println!("{winners:?}");
    let applicants = vec![
        "Bob", "Mary", "Kevin", "Mike", "Robbie", "Matt", "Austin", "Piers", "Liam",
    ];
    let winners = applicants
        .into_iter()
        .enumerate()
        .filter_map(|(index, applicant)| {
            if index % 3 == 0 {
                Some(applicant)
            } else {
                None
            }
        })
        .collect::<Vec<&str>>();
    println!("{winners:?}");
    println!("");
    // the partition method
    let numbers = [10, 13, 23, 2, 8, 9, 6];
    let (even, odds): (Vec<i32>, Vec<i32>) = numbers.into_iter().partition(|num| num % 2 == 0);
    println!("{even:?}");
    println!("{odds:?}");
    println!("");

    // the zip method
    let first_name = ["Casey", "Robert", "Cargo", "Dan"];
    let last_name = ["Johnson", "Smith", "Rustman"];

    for (first_name, last_name) in first_name.iter().zip(last_name) {
        println!("{first_name} {last_name}");
    }
    let complete_name = first_name
        .iter()
        .zip(last_name)
        .map(|(fne, lne)| format!("{fne} {lne}"))
        .collect::<Vec<String>>();
    println!("{complete_name:?}");
}

#[derive(Debug)]
struct TVChannel {
    name: String,
    channel_type: ChannelType,
}

#[derive(Debug, PartialEq, Eq)]
enum ChannelType {
    Comedy,
    News,
    ProgrammingTutorials,
}

fn count_characters(text: &str) -> HashMap<char, u32> {
    let words = text.split_whitespace();
    let mut counts = HashMap::new();

    for word in words {
        for character in word.chars() {
            let count = counts.entry(character).or_insert(0);
            *count += 1;
        }
    }
    counts
}

fn count_characters2(text: &str) -> HashMap<char, u32> {
    let words = text.split_whitespace();
    let mut counts = HashMap::new();

    words.for_each(|word| {
        word.chars().for_each(|character| {
            let count = counts.entry(character).or_insert(0);
            *count += 1;
        });
    });
    counts
}

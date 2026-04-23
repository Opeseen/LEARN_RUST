use std::collections::{HashMap, HashSet};
use std::{env, fs, io, process};
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
    println!("");

    // the fold method
    let earnings = [4, 7, 9, 13];
    let sum = earnings.into_iter().fold(0, |total, current| {
        println!("Total: {total}, Current: {current}");
        total + current
    });
    println!("{sum}");
    println!("");

    let week = [
        SupportStaff {
            day: String::from("Monday"),
            employee: String::from("Brian"),
        },
        SupportStaff {
            day: String::from("Tuesday"),
            employee: String::from("Can"),
        },
        SupportStaff {
            day: String::from("Wednesday"),
            employee: String::from("Walter"),
        },
    ];

    let map = week.into_iter().fold(HashMap::new(), |mut data, entry| {
        data.insert(entry.day, entry.employee);
        data
    });
    println!("{map:?}");
    println!("");

    let earnings: [i32; 0] = [];

    let sum = earnings
        .into_iter()
        .reduce(|total, current| total + current);
    println!("{sum:?}");

    let address_portions = [
        String::from("123 Eln Street"),
        String::from("Suburbia"),
        String::from("New Jersey"),
    ];
    let address = address_portions
        .into_iter()
        .reduce(|mut accumulator, portion| {
            accumulator.push_str(", ");
            accumulator.push_str(&portion);
            accumulator
        });
    println!("{address:?}");
    println!("");

    let numbers = vec![4, 8, 15, 16, 23, 42];
    let total: i32 = numbers.iter().sum();
    println!("{total}");
    let total: i32 = numbers.iter().product();
    println!("{total}");
    let max = numbers.iter().max().unwrap();
    println!("{max}");

    let count = numbers.iter().count();
    println!("{count}");

    let numbers = vec![4.6, 8.8, 0.0 / 0.0, 6.2];
    let total: f64 = numbers.iter().sum();
    println!("{total}");

    // let max = numbers.iter().max().unwrap(); will not work for f64 as trait bound not implemented
    let total = numbers
        .iter()
        .filter(|number| !number.is_nan())
        .copied()
        .fold(0.0, |total, current| total + current);
    println!("{total}");

    let max = numbers
        .iter()
        .filter(|number| !number.is_nan())
        .copied()
        .reduce(|accumulator, current| accumulator.max(current));
    println!("{max:?}");
    println!("");

    let performance = ["Rustful five", "Rust in peace", "Rustin Bieber"];
    let last = performance.into_iter().last().unwrap();
    println!("{last}");
    let second = performance.into_iter().nth(1).unwrap();
    println!("{second}");
    let second_to_last = performance.into_iter().nth_back(1).unwrap();
    println!("{second_to_last}");

    let target_index = performance
        .into_iter()
        .position(|element| element == "Rustin Bieber");
    println!("{target_index:?}");
    println!("");

    let fifty_number = 1..=50;
    for number in fifty_number {
        print!("{number}/");
    }
    println!("");
    let fifty_number = 1..=50;
    for number in fifty_number.take(15) {
        print!("{number}/");
    }
    println!("");
    let fifty_number = 1..=50;
    for number in fifty_number.rev().take(15) {
        print!("{number}/");
    }
    println!("");
    let fifty_number = 1..=50;
    for number in fifty_number.skip(5).take(15) {
        print!("{number}/");
    }
    println!("");
    let fifty_number = 1..=50;
    for number in fifty_number.take(15).skip(5) {
        print!("{number}/");
    }
    println!("");
    let fifty_number = 1..=50;
    for number in fifty_number.take(15).skip(5).step_by(2) {
        print!("{number}/");
    }
    println!("");
    let fifty_number = 1..=50;
    for number in fifty_number.clone().take(15).skip(5).step_by(2) {
        print!("{number}/");
    }
    println!("");

    let mut points = [3, 8, 1, 11, 5];
    println!("{}", points.is_sorted());
    points.sort();
    println!("{points:?}");
    println!("{}", points.is_sorted());
    points.reverse();
    println!("{points:?}");
    println!("{}", points.is_sorted());

    let mut exercises = ["squat", "bench", "Deadlift"];
    exercises.sort();
    println!("{exercises:?}");
    println!("");

    let mobile = GasStation {
        snack_count: 100,
        manager: String::from("Meg Mobil"),
        employee_count: 3,
    };

    let exon = GasStation {
        snack_count: 130,
        manager: String::from("Eric Exxon"),
        employee_count: 4,
    };

    let shell = GasStation {
        snack_count: 50,
        manager: String::from("Shane Shell"),
        employee_count: 2,
    };

    let mut steps = [mobile, exon, shell];
    // steps.sort_by_key(|station| station.snack_count);
    println!("{steps:?}");
    // steps.sort_by_key(|station| station.employee_count);
    println!("{steps:?}");
    steps.sort_by_key(|station| -(station.employee_count as i32));
    println!("{steps:?}");
    println!("");

    println!("{:?}", read_file());
    println!("");

    let args = env::args();
    for arg in args {
        println!("{arg}");
    }
    println!("");

    let settings = collect_settings();
    println!("{settings:?}");
    println!("");

    // reading a directory
    let directory = fs::read_dir("./").unwrap_or_else(|error| {
        eprintln!("Could not read directory: {error}");
        process::exit(1);
    });

    for entry_result in directory {
        match entry_result {
            Ok(entry) => println!("{:?}", entry.path()),
            Err(error) => {
                eprintln!("Could not read entry: {error}");
            }
        }
    }

    println!("");
    println!("{:?}", list_files2());
    println!("");

    let fifty_numbers = 1..=50;
    let result = Vec::from_iter(fifty_number.clone());
    println!("{result:?}");

    let results = fifty_number.clone().collect::<Vec<i32>>();
    println!("{results:?}");
    println!("");
    let unique_set: HashSet<_> = HashSet::from_iter(fifty_number.clone());
    println!("{unique_set:?}");

    let chars = ['H', 'e', 'l', 'l', 'o'];
    let greeting = String::from_iter(chars);
    println!("{greeting}");
    println!("");

    let songs = [
        (String::from("I Rust Go On"), String::from("Bob")),
        (String::from("A Rut of Wind"), String::from("Bob")),
        (String::from("A Rustworthy man"), String::from("Sheila")),
    ];

    let playlist = Playlist::from_iter(songs);
    println!("{playlist:?}");
    println!("");

    let songs = [
        (String::from("I Rust Go On"), String::from("Bob")),
        (String::from("A Rut of Wind"), String::from("Bob")),
        (String::from("A Rustworthy man"), String::from("Sheila")),
    ];

    let playlist: Playlist = songs.into_iter().collect();
    println!("{playlist:?}");
}

#[derive(Debug)]
struct Playlist {
    songs: Vec<String>,
    users: HashSet<String>,
}

impl FromIterator<(String, String)> for Playlist {
    fn from_iter<T: IntoIterator<Item = (String, String)>>(iter: T) -> Self {
        let mut songs = Vec::new();
        let mut users = HashSet::new();
        for (song, user) in iter {
            songs.push(song);
            users.insert(user);
        }
        Self { songs, users }
    }
}

fn list_files2() -> io::Result<()> {
    for entry_result in fs::read_dir("./")? {
        let entry = entry_result?;
        let entry_name = entry.path();
        let metadata = fs::metadata(&entry_name)?;
        if metadata.is_file() {
            println!("{entry_name:?}\n-----------------");
            let contents = fs::read_to_string(&entry_name)?;
            println!("{contents}");
        }
    }
    Ok(())
}

fn list_files() -> io::Result<()> {
    for entry_result in fs::read_dir("./")? {
        // Method 1
        // if let Ok(entry) = entry_result {
        //     println!("{:?}", entry.path())
        // }

        // Method 2
        let entry = entry_result?;
        println!("{:?}", entry.path());
    }
    Ok(())
}

fn read_file() -> io::Result<()> {
    let contents = fs::read_to_string("story.txt")?;

    for line in contents.lines() {
        println!("{line}");
    }

    Ok(())
}

fn collect_settings() -> Settings {
    let mut args = env::args().skip(1).take(3);
    let video_file = args.next().unwrap_or_else(|| {
        eprintln!("No video file specified");
        process::exit(1);
    });

    let mut settings = args.map(|settings| settings.parse::<bool>().unwrap_or(false));

    let subtitles = settings.next().unwrap_or(false);
    let high_definitions = settings.next().unwrap_or(false);

    Settings {
        video_file,
        subtitles,
        high_definitions,
    }
}

#[derive(Debug)]
struct Settings {
    video_file: String,
    subtitles: bool,
    high_definitions: bool,
}

#[derive(Debug)]
struct GasStation {
    snack_count: u32,
    manager: String,
    employee_count: u32,
}

#[derive(Debug)]
struct SupportStaff {
    day: String,
    employee: String,
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

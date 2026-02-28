fn main() {
    // define struct
    struct Coffee {
        price: f64,
        name: String,
        is_hot: bool,
    }

    // create a struct instance
    let mocha = Coffee {
        name: String::from("Mocha"),
        price: 4.09,
        is_hot: false,
    };

    // access struct field
    println!(
        "My {} this morning cost {}. It is {} that it was hot",
        mocha.name, mocha.price, mocha.is_hot
    );
    let favorite_coffee = mocha.name;
    println!("{favorite_coffee}");
    // println!("{}", mocha.name); // will failed b3cause ownership has moved because "String doesn't implement copy trait"

    // overwrite struct fields
    let mut beverage = Coffee {
        name: String::from("Mocha"),
        price: 4.09,
        is_hot: false,
    };
    beverage.name = String::from("Caramel Macchiato");
    beverage.price = 6.99;
    beverage.is_hot = true;
    println!(
        "My {} this morning cost {}. It is {} that it was hot",
        beverage.name, beverage.price, beverage.is_hot
    );

    // struct in a function
    let name = String::from("Latte");
    let coffee2 = make_coffee(name, 4.99, true);
    println!(
        "My {} this morning cost {}. It is {} that it was hot",
        coffee2.name, coffee2.price, coffee2.is_hot
    );

    // shorthand struct initialization
    let name = String::from("Latte");
    let price = 4.99;
    let is_hot = false;
    let latte = Coffee {
        name,
        price,
        is_hot,
    };

    // struct update syntax
    let mocha = make_coffee(String::from("Mocha"), 4.99, true);
    let caramel_macchiato = Coffee2 {
        name: mocha.name.clone(),
        ..mocha
    };
    println!("{}", caramel_macchiato.name);
    println!("{}", mocha.name);
    println!("");

    // passing struct into a function
    drink_coffee(&mocha); // without "&" pointer -> will not run the below code
    println!("{}", mocha.name); // will not work as ownership has moved
    println!("");
    let mut mocha = make_coffee(String::from("Mocha"), 4.99, true);
    drink_coffee2(&mut mocha);
    println!("{}", mocha.price);
    println!("");

    // derive debug trait in struct
    println!("{:?}", mocha);
    println!("{:#?}", mocha); // pretty print
    println!("");

    // define struct method
    let song = TaylorSwiftSong {
        title: String::from("Blank Space"),
        release_year: 2012,
        duration_secs: 231,
    };
    // song.display_song_info(); @INFO: commented this out to run the next line because ownership has moved
    // song.double_length();
    song.display_song_info();
    println!("");

    let mut song = TaylorSwiftSong {
        title: String::from("Blank Space"),
        release_year: 2012,
        duration_secs: 231,
    };

    song.display_song_info();
    song.double_length();

    song.display_song_info();
    println!("");

    // method with parameters
    let blank_space = TaylorSwiftSong {
        title: String::from("Blank Space"),
        release_year: 2014,
        duration_secs: 231,
    };
    let all_too_well = TaylorSwiftSong {
        title: String::from("Blank Space"),
        release_year: 2014,
        duration_secs: 231,
    };

    if blank_space.is_longer_than(&all_too_well) {
        println!(
            "{} is longer than {}",
            blank_space.title, all_too_well.title
        )
    } else {
        println!(
            "{} is shorter than or equal to {}",
            blank_space.title, all_too_well.title
        )
    }

    // calling method on other methods
    blank_space.display_song_info();
    println!("");
    // associated function
    let blank_space = TaylorSwiftSong::new(String::from("Blank Space"), 2015, 231);
    blank_space.display_song_info();
    println!("");
    // multiple impl block
    let blank_space = TaylorSwiftSong::new2(String::from("Blank Space"), 2016, 231);
    blank_space.display_song_info();
    println!("");

    // builder pattern
    let mut computers = Computer::new(String::from("M3 Max"), 64, 2);
    println!("Stats: {computers:#?}");
    computers
        .upgrade_cpu(String::from("M4 Max"))
        .upgrade_memory(128)
        .upgrade_hard_drive_capacity(4);
    println!("Stats: {computers:#?}");
    println!("");

    // tuple struct
    let works_shift = ShortDuration(8, 0);
    println!("{} hours {} minutes", works_shift.0, works_shift.1);
    let era = LongDuration(5, 3);
    println!("{} years {} months", era.0, era.1);

    go_to_work(works_shift);
    println!("");

    // unit type struct
    let my_empty_struct = Empty;
}

struct Empty;

fn go_to_work(length: ShortDuration) {
    println!("passing time {} hours {} minutes", length.0, length.1);
}
// hours and minutes
struct ShortDuration(u32, u32);
// years and month
struct LongDuration(u32, u32);

#[derive(Debug)]
struct Computer {
    cpu: String,
    memory: u32,
    hard_drive_capacity: u32,
}

impl Computer {
    fn new(cpu: String, memory: u32, hard_drive_capacity: u32) -> Self {
        Self {
            cpu,
            memory,
            hard_drive_capacity,
        }
    }

    fn upgrade_cpu(&mut self, new_cpu: String) -> &mut Self {
        self.cpu = new_cpu;
        self
    }

    fn upgrade_memory(&mut self, new_memory: u32) -> &mut Self {
        self.memory = new_memory;
        self
    }

    fn upgrade_hard_drive_capacity(&mut self, new_capacity: u32) -> &mut Self {
        self.hard_drive_capacity = new_capacity;
        self
    }
}

#[derive(Debug)]
struct TaylorSwiftSong {
    title: String,
    release_year: u32,
    duration_secs: u32,
}

// self
// mut self
// &self
// &mut self

impl TaylorSwiftSong {
    fn new2(title: String, release_year: u32, duration_secs: u32) -> Self {
        Self {
            title,
            release_year,
            duration_secs,
        }
    }
}

impl TaylorSwiftSong {
    fn new(title: String, release_year: u32, duration_secs: u32) -> Self {
        Self {
            title,
            release_year,
            duration_secs,
        }
    }

    // immutable struct value (self parameter takes ownership)
    // Immutable reference the struct instance (no ownership moved)
    fn display_song_info(&self) {
        println!("Title {}", self.title);
        println!("Release Year {}", self.release_year);
        println!("Duration {} seconds", self.duration_secs);
        println!("Year since release: {}", self.year_since_release()); // calling method in methods
    }

    // mutable struct value (self parameter takes ownership, has permission to mutate)
    // mutable reference the struct instance (no ownership moved, has permission to mutate)
    fn double_length(&mut self) {
        self.duration_secs = self.duration_secs * 2;
        // println!("{:#?}", self);
    }

    fn is_longer_than(&self, other: &TaylorSwiftSong) -> bool {
        self.duration_secs > other.duration_secs
    }

    fn year_since_release(&self) -> u32 {
        2024 - self.release_year
    }
}

#[derive(Debug)]
struct Coffee2 {
    price: f64,
    name: String,
    is_hot: bool,
}

fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee2 {
    Coffee2 {
        price,
        name,
        is_hot,
    }
}

fn drink_coffee(coffee: &Coffee2) {
    println!("Drinking my delicious {}", coffee.name);
}

fn drink_coffee2(coffee: &mut Coffee2) {
    println!("Drinking my delicious {}", coffee.name);
    coffee.price = 10.99;
}

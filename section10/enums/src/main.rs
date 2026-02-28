use std::mem;

fn main() {
    let first_card = CardSuit::Hearts;
    let mut second_card = CardSuit::Spades;
    second_card = CardSuit::Clubs;
    println!("{:?}", second_card);
    let card_suits = [CardSuit::Hearts, CardSuit::Clubs];
    let card_suits = (CardSuit::Hearts, CardSuit::Spades);
    println!("");

    // enums with associated value
    let visa = PaymentMethodType::CreditCard(String::from("4242-4242-4242-4242"));
    let mastercard = PaymentMethodType::DebitCard(String::from("1234-1234-1234"));
    println!("{:?}", visa);
    println!("{:?}", mastercard);
    let mut my_payment_method = PaymentMethodType::CreditCard(String::from("4242-4242-4242-4242"));
    my_payment_method =
        PaymentMethodType::PayPal(String::from("bob@example.com"), String::from("password"));
    println!("{:?}", my_payment_method);
    println!("");
    // struct variant
    let paystalk_credentials = Credentials {
        username: String::from("bob@gmail.com"),
        password: String::from("password"),
    };
    let paystalk = PaymentMethodType::PayStalk(paystalk_credentials);
    println!("{:?}", paystalk);
    let paystalk = PaymentMethodType::MoMO {
        username: String::from("bob@gmail.com"),
        password: String::from("password"),
    };
    println!("{:?}", paystalk);
    println!("");

    // nesting enums to enums
    let launch = RestaurantItem::Burrito {
        meat: Meat::Chicken,
        beans: Beans::Black,
    };
    let dinner = RestaurantItem::Bowl {
        meat: Meat::Steak,
        beans: Beans::Pinto,
    };
    let supper = RestaurantItem::Bowls(Meat::Chicken);
    let abandoned_meal = RestaurantItem::VeganPlate;
    println!("launch was {launch:?} and dinner was {dinner:?}");
    println!("Nobody ate {abandoned_meal:?}");
    println!("");

    // match keywords
    let my_computer = OperatingSystem::MacOS;
    let age = years_since_release(my_computer);
    println!("My computer is {age} years old");

    let dads_computer = OperatingSystem::Windows;
    let age = years_since_release(dads_computer);
    println!("Dad's computer is {age} years old");
    println!("");

    wash_laundry(LaundryCycle::Cold);
    wash_laundry(LaundryCycle::Hot { temperature: 100 });
    wash_laundry(LaundryCycle::Delicate(String::from("Silk")));
    println!("");

    // methods on enum
    LaundryCycle::Cold.wash_laundry();
    LaundryCycle::Hot { temperature: 100 }.wash_laundry();
    LaundryCycle::Delicate(String::from("Silk")).wash_laundry();
    println!("");

    // match keywords
    OnlineOrderStatus::Delivered.check();
    Milk::LowFat(1).drink();
    Milk::LowFat(2).drink();
    Milk::Whole.drink();
    println!("");

    // if let construct
    let my_beverage = Milk::LowFat(3);
    if let Milk::Whole = my_beverage {
        println!("You have a whole milk");
    };
    if let Milk::LowFat(percent) = my_beverage {
        println!("Your beverage is {percent}% milk");
    };

    let my_beverage = Milk::NonDiary {
        kind: String::from("Oat"),
    };
    if let Milk::NonDiary { kind } = my_beverage {
        println!("Your beverage is {kind} milk")
    } else {
        println!("You have some other milk variant");
    }
    println!("");

    // let else construct
    let my_beverage = Milk::LowFat(2);

    let Milk::LowFat(percent) = my_beverage else {
        println!("You do not have the low fat milk");
        return;
    };
    println!("{percent}% milk is available here");
}
enum Milk {
    LowFat(i32),
    Whole,
    NonDiary { kind: String },
}

#[derive(Debug)]
enum OnlineOrderStatus {
    Ordered,
    Packed,
    Shipped,
    Delivered,
}

impl Milk {
    fn drink(self) {
        match self {
            Milk::LowFat(2) => {
                println!("Delicious, 2% is my favorite");
            }
            Milk::LowFat(percent) => {
                println!("You got the lowFat {percent} percent version")
            }
            Milk::Whole => {
                println!("You have got the whole")
            }
            Milk::NonDiary { kind } => {
                println!("You have got the {kind} kind type");
            }
        }
    }
}

impl OnlineOrderStatus {
    fn check(&self) {
        match self {
            OnlineOrderStatus::Ordered | OnlineOrderStatus::Packed => {
                println!("Your item is been prepped for shipment");
            }
            OnlineOrderStatus::Delivered => {
                println!("Your item has been delivered!");
            }
            order_status => {
                println!("Your item is {order_status:?}");
            }
        }
    }
}

impl LaundryCycle {
    fn wash_laundry(&self) {
        match self {
            LaundryCycle::Cold => {
                println!("Running the laundry with cold temperature");
            }
            LaundryCycle::Hot { temperature } => {
                println!("Running a laundry with a temperature of {temperature}");
            }
            LaundryCycle::Delicate(fabric_type) => {
                println!("Running a laundry with a delicate cycle for the {fabric_type}");
            }
        }
    }
}

fn years_since_release(os: OperatingSystem) -> u32 {
    match os {
        OperatingSystem::Windows => {
            println!("Quite an old operating system");
            39
        }
        OperatingSystem::MacOS => 23,
        OperatingSystem::Linux => 34,
    }
}

fn wash_laundry(cycle: LaundryCycle) {
    match cycle {
        LaundryCycle::Cold => {
            println!("Running the laundry with cold temperature");
        }
        LaundryCycle::Hot { temperature } => {
            println!("Running a laundry with a temperature of {temperature}");
        }
        LaundryCycle::Delicate(fabric_type) => {
            println!("Running a laundry with a delicate cycle for the {fabric_type}");
        }
    }
}
#[derive(Debug)]
enum LaundryCycle {
    Cold,
    Hot { temperature: u32 },
    Delicate(String),
}
enum OperatingSystem {
    Windows,
    MacOS,
    Linux,
}

#[derive(Debug)]
enum Beans {
    Pinto,
    Black,
}
#[derive(Debug)]
enum Meat {
    Chicken,
    Steak,
}
#[derive(Debug)]
enum RestaurantItem {
    Burrito { meat: Meat, beans: Beans },
    Bowl { meat: Meat, beans: Beans },
    Bowls(Meat),
    VeganPlate,
}

#[derive(Debug)]
struct Credentials {
    username: String,
    password: String,
}

#[derive(Debug)]
enum PaymentMethodType {
    CreditCard(String),
    DebitCard(String),
    PayPal(String, String),
    PayStalk(Credentials),
    MoMO { username: String, password: String },
}

#[derive(Debug)]
enum CardSuit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

struct Card {
    rank: String,
    suit: CardSuit,
}

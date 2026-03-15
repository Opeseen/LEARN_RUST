// mod inventory;
// mod orders;

use inventory::MANAGER;
use inventory::products::{Item, ProductCategory};
// use inventory::{FLOOR_SPACE, MANAGER as INVENTORY_MANAGER, talk_to_manager};
// use orders::MANAGER as ORDERS_MANAGER;

// use inventory::products;
use inventory::products::{self};

use fake::{Fake, Faker};

use std::{
    fmt,
    io::{self, stdin, stdout},
};

// the glob operator
use std::collections::*;

// library crates import
use warehouse::{FLOOR_SPACE, INVENTORY_MANAGER, ORDERS_MANAGER, talk_to_manager};
use warehouse::{inventory, orders};

fn main() {
    println!("The manager of our inventory is {}", inventory::MANAGER);
    println!("The manager of our orders is {}", orders::MANAGER);
    println!("");

    println!(
        "Our managers are {} amd {}, We have {} square feet of floor space",
        inventory::MANAGER,
        orders::MANAGER,
        inventory::FLOOR_SPACE
    );

    inventory::talk_to_manager();
    let favorite_category = inventory::ProductCategory::Hammer;
    println!("My favorite category of item is {favorite_category:?}");

    let tall_ladder = inventory::Item {
        name: String::from("Ladder-o-matic 2000"),
        category: favorite_category,
        quantity: 100,
    };
    println!("{:#?}", tall_ladder);
    println!("");

    // sub-module
    let favorite_category = inventory::products::ProductCategory::Hammer;
    println!("My favorite category of item is {favorite_category:?}");

    let tall_ladder = inventory::products::Item {
        name: String::from("Ladder-o-matic 2000"),
        category: favorite_category,
        quantity: 100,
    };
    println!("{:#?}", tall_ladder);
    println!("");

    // crates
    println!(
        "Our managers are {} amd {}, We have {} square feet of floor space",
        crate::inventory::MANAGER,
        crate::orders::MANAGER,
        crate::inventory::FLOOR_SPACE
    );

    inventory::talk_to_manager2();
    println!("");

    // use keyword
    println!(
        "Our managers are {} amd {}, We have {} square feet of floor space",
        MANAGER,
        orders::MANAGER,
        FLOOR_SPACE
    );
    let favorite_category = ProductCategory::Ladder;
    println!("My favorite category of item is {favorite_category:?}");
    let tall_ladder = Item {
        name: String::from("Ladder-o-matic 2000"),
        category: favorite_category,
        quantity: 100,
    };
    println!("{:#?}", tall_ladder);
    talk_to_manager();
    println!("");

    // self keyword
    let favorite_category = ProductCategory::Ladder;
    println!("My favorite category of item is {favorite_category:?}");
    let tall_ladder = products::Item {
        name: String::from("Ladder-o-matic 2002"),
        category: favorite_category,
        quantity: 100,
    };
    println!("{:#?}", tall_ladder);
    println!("");

    // super keyword
    let favorite_category = ProductCategory::Ladder;
    println!("My favorite category of item is {favorite_category:?}");
    let tall_ladder = Item::new(String::from("Ladder-o-matic 2003"), favorite_category, 100);
    println!("{:#?}", tall_ladder);
    println!("");

    // create alias with the as keyword
    println!(
        "Our managers are {} amd {}, We have {} square feet of floor space",
        INVENTORY_MANAGER, ORDERS_MANAGER, FLOOR_SPACE
    );
    let favorite_category = ProductCategory::Ladder;
    println!("My favorite category of item is {favorite_category:?}");
    println!("");

    // using pub use to export names from submodules
    let fake_item: Item = Faker.fake();
    println!("{:?}", fake_item);

    let random_category: ProductCategory = Faker.fake();
    println!("{:?}", random_category);
    println!("");
}

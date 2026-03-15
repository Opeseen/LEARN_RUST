pub mod products;
pub const FLOOR_SPACE: i32 = 10000;
pub const MANAGER: &str = "Ivan Inventory";

pub use products::{Item as item, ProductCategory as prodCat};

#[derive(Debug)]
pub enum ProductCategory {
    Ladder,
    Hammer,
}

#[derive(Debug)]
pub struct Item {
    pub name: String,
    pub category: ProductCategory,
    pub quantity: u32,
}

/*
 * sub-modules option
 * inline
 * inventory/product.ts
 * inventory/product/mod.ts
*/

pub fn talk_to_manager() {
    println!("Hey, {}, how's your coffee", MANAGER);
}

pub fn talk_to_manager2() {
    println!("Hey, {}, how's your coffee2", crate::inventory::MANAGER);
}

fn talk_to_manager3() {
    println!("Hey, {}, how's your coffee3", crate::inventory::MANAGER);
}

fn talk_to_manager4() {
    println!(
        "Hey, {}, how's your coffee? What do you think of {:?}",
        MANAGER,
        prodCat::Ladder
    )
}

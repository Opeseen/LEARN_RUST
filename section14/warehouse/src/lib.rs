// library crates
/// Tools for inventory management
pub mod inventory;
/// Tools for order management
pub mod orders;

pub use inventory::{
    FLOOR_SPACE, Item, MANAGER as INVENTORY_MANAGER, ProductCategory, talk_to_manager,
};
pub use orders::MANAGER as ORDERS_MANAGER;

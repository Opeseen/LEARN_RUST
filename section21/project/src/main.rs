use std::{collections::HashMap, env};

fn main() {
    let mut orders = vec![
        CustomerOrder {
            product: Product::Blender,
            quantity: 2,
            shipped: true,
        },
        CustomerOrder {
            product: Product::Microwave,
            quantity: 3,
            shipped: true,
        },
        CustomerOrder {
            product: Product::Toaster,
            quantity: 2,
            shipped: false,
        },
        CustomerOrder {
            product: Product::Toaster,
            quantity: 1,
            shipped: false,
        },
        CustomerOrder {
            product: Product::Microwave,
            quantity: 2,
            shipped: false,
        },
        CustomerOrder {
            product: Product::Fridge,
            quantity: 1,
            shipped: true,
        },
    ];

    let customer_ids_by_order = [2, 1, 2, 3, 4, 1];

    let blender_orders = orders
        .iter()
        .filter(|order| order.product == Product::Blender)
        .collect::<Vec<&CustomerOrder>>();
    println!("{:?}", blender_orders);
    println!();

    let microwave_count = orders
        .iter()
        .filter(|order| order.product == Product::Microwave)
        .map(|order| order.quantity)
        .sum::<u32>();
    println!("Total quantity of microwave is: {microwave_count}");
    println!();

    let microwave_count = orders
        .iter()
        .filter_map(|order| {
            if order.product == Product::Microwave {
                Some(order.quantity)
            } else {
                None
            }
        })
        .sum::<u32>();
    println!("Total quantity of microwave is: {microwave_count}");
    println!();

    let user_quantity = env::args()
        .skip(1)
        .take(1)
        .map(|quantity| quantity.parse::<u32>().unwrap_or(2))
        .next()
        .unwrap_or(2);

    let orders_by_quantity = orders
        .iter()
        .filter(|order| order.quantity >= user_quantity)
        .collect::<Vec<&CustomerOrder>>();
    println!("{orders_by_quantity:?}");
    println!();

    let product_quantities = orders.iter().filter(|order| order.shipped == false).fold(
        HashMap::new(),
        |mut data, order| {
            let entry = data.entry(&order.product).or_insert(0);
            *entry += order.quantity;
            data
        },
    );
    println!("{:?}", product_quantities);
    println!();

    if let Some(order) = orders.iter_mut().find(|order| order.shipped == false) {
        order.shipped = true;
    }
    println!("{orders:#?}");
    println!();
    println!();

    let mut customers = orders
        .into_iter()
        .zip(customer_ids_by_order)
        .fold(HashMap::new(), |mut ids_to_orders, (order, customer_id)| {
            let orders = ids_to_orders.entry(customer_id).or_insert(vec![]);
            orders.push(order);
            ids_to_orders
        })
        .into_iter()
        .map(|(id, orders)| Customer { id, orders })
        .collect::<Vec<Customer>>();
    customers.sort_by_key(|customer| customer.id);
    println!("{customers:#?}");
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Product {
    Blender,
    Microwave,
    Toaster,
    Fridge,
}

#[derive(Debug)]
struct CustomerOrder {
    product: Product,
    quantity: u32,
    shipped: bool,
}

#[derive(Debug)]
struct Customer {
    id: u32,
    orders: Vec<CustomerOrder>,
}

impl CustomerOrder {
    fn new(product: Product, quantity: u32, shipped: bool) -> Self {
        Self {
            product,
            quantity,
            shipped,
        }
    }
}

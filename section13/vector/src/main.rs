fn main() {
    // create a vector
    let pizza: Vec<i32> = Vec::new();
    let pizza = Vec::<i32>::new();
    println!("{pizza:?}");

    let pastas: Vec<&str> = Vec::new();
    let pastas = Vec::<&str>::new();
    println!("{pastas:?}");

    let pizza = vec![8, 10, 12, 14];
    println!("{pizza:?}");
    println!("");

    // adding and removing from a vector
    let mut pizza = vec![8, 10, 12, 14];
    pizza.push(18);
    pizza.push(16);
    println!("{pizza:?}");
    pizza.insert(0, 4);
    println!("{pizza:?}");
    let last_pizza = pizza.pop();
    println!("{last_pizza:?}");
    let third_pizza = pizza.remove(2);
    println!("{third_pizza}");
    println!("");

    // reading vector element
    let pizza_diameters = vec![8, 10, 12, 14];
    let pepperoni = String::from("Pepperoni");
    let mushroom = String::from("Mushroom");
    let sausage = String::from("Sausage");
    let pizza_toppings = vec![pepperoni, mushroom, sausage];
    let reference = &pizza_toppings[2];
    println!("{reference}");
    let pizza_slice = &pizza_toppings[1..3];
    println!("{pizza_slice:?}");
    println!("");

    // the get method
    let option = pizza_toppings.get(20);
    match option {
        Some(topping) => println!("The topping is {topping}"),
        None => println!("No value at that index position"),
    }
    println!("");

    // ownership with vectors
    let mut delicious_toppings = pizza_toppings;
    let topping_reference = &delicious_toppings[1];
    delicious_toppings.push(String::from("Olives"));
    // println!("{topping_reference}") // immutable borrow later used here
    println!("");

    // writing vector element
    let pepperoni = String::from("Pepperoni");
    let mushroom = String::from("Mushroom");
    let sausage = String::from("Sausage");
    let mut pizza_toppings = vec![pepperoni, mushroom, sausage];
    pizza_toppings[1] = String::from("Olives");
    println!("{pizza_toppings:?}");
    let target_toppings = &mut pizza_toppings[2];
    target_toppings.push_str(" and Meatballs");
    println!("{target_toppings:?}");
    println!("");

    // vector capacity
    let mut seasons: Vec<&str> = Vec::with_capacity(4);
    println!("Length: {}. Capacity {}", seasons.len(), seasons.capacity());
    seasons.push("Summer");
    seasons.push("Fall");
    seasons.push("Winter");
    seasons.push("Spring");
    println!("Length: {}. Capacity {}", seasons.len(), seasons.capacity());
    seasons.push("Summer");
    println!("Length: {}. Capacity {}", seasons.len(), seasons.capacity());
}

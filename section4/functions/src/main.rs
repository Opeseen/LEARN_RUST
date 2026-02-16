fn main() {
    open_store("Brooklyn");
    bake_pizza(20, "pepperoni");
    swim_in_profit();
    open_store("Queens");
    bake_pizza(50, "mushroom");

    let result = square(13);
    println!("The square of 13 is: {result}");

    let result = square2(15);
    println!("The square of 15 is: {result}");

    let result: () = mystery();
    println!("{result:?}");

    let multiplier = 3;

    // blocks in function
    {
        let calculation = {
            let value = 5 + 4;
            value * multiplier
        };

        println!("{calculation}")
    }
}

// intro to func | parameters & args
fn open_store(neighborhood: &str) {
    println!("Opening my pizza store in {neighborhood}");
}
fn bake_pizza(number: i32, topping: &str) {
    println!("Baking {number} {topping} pizza");
}
fn swim_in_profit() {
    println!("So much $$$, so little time!");
}

// explicit return value
fn square(number: i32) -> i32 {
    return number * number;
}

// implicit return value
fn square2(number: i32) -> i32 {
    number * number
}

// unit as return type
fn mystery() {
    println!("Hello there printed!");
}

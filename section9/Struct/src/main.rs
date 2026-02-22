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
}

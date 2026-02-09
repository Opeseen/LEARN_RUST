fn main() {
    let number: i32 = 1_337;
    println!("{number}");
    let number_int = number as i16;
    println!("{number_int}");

    let float_value: f64 = 3.1566539;
    println!("{float_value:.4}");
    println!("{}", float_value.round());

    let with_milk = true;
    let with_sugar = false;
    let is_my_type_of_coffee = with_milk && with_sugar;
    println!("{is_my_type_of_coffee}");
    let is_an_acceptable_coffee = with_milk || with_sugar;
    println!("{is_an_acceptable_coffee}");

    let arrays: [i8; 4] = [1, 2, 3, 4];
    dbg!(arrays);

    let tuple = (1, 2.3, true, arrays);
    dbg!(tuple);
}

use std::collections::HashMap;

fn main() {
    let mut sauce_to_meals: HashMap<&str, Vec<&str>> = HashMap::from([
        ("Ketchup", vec!["French Fries", "Burgers", "Hot Dogs"]),
        ("Mayonnaise", vec!["Sandwiches", "Burgers", "Coleslaw"]),
    ]);
    sauce_to_meals.insert("Mustard", vec!["Hot dog", "Burgers", "pretzels"]);
    println!("{:?}", sauce_to_meals);
    println!("{:?}", sauce_to_meals.remove("Mayonnaise"));
    let mustard_meals = sauce_to_meals.get("Mustard");
    println!("{:?}", mustard_meals);
    match mustard_meals {
        Some(meals) => println!("The meals were {meals:?}"),
        None => println!("There were no meals for that sauce! Oh no"),
    }

    sauce_to_meals
        .entry("Soy Sauce")
        .or_insert(vec!["Sushi", "Dumpings"]);
    println!("{:?}", sauce_to_meals);
}

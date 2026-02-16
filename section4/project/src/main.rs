fn main() {
    apply_to_jobs(2, "DevOps");

    let result = is_even(10);
    println!("result is: {}", result)
}

fn apply_to_jobs(number: i32, title: &str) {
    println!("I am applying to {number} {title} jobs");
}

fn is_even(number: i32) -> bool {
    return number % 2 == 0;
}

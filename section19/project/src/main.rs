fn main() {
    println!("{}", double_the_length(&vec![1, 2, 3]));
    println!("{:?}", last_two(&vec![1, 2, 3, 4, 5, 6]));
    println!("{}", first_five("refrigerator", "Hello"));
    println!(
        "{}",
        find_string_that_has_content("programming", "dining", "gram")
    );
}

fn double_the_length<T>(collection: &Vec<T>) -> usize {
    collection.len() * 2
}

fn last_two<T>(collection: &[T]) -> &[T] {
    let two_from_end = collection.len() - 2;
    &collection[two_from_end..]
}

fn first_five<'a>(text: &'a str, announcement: &str) -> &'a str {
    println!("The value of announcement is: {announcement}");
    &text[..5]
}

fn find_string_that_has_content<'a>(first: &'a str, second: &'a str, target: &str) -> &'a str {
    if first.contains(target) {
        first
    } else {
        second
    }
}

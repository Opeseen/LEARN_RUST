fn main() {
    let mut cereals = [
        String::from("Cookie Crisp"),
        String::from("Cinnamon Toast"),
        String::from("Frosted Flakes"),
        String::from("Cocoa Puffs"),
        String::from("Captain Crunch"),
    ];
    println!("{:?}", cereals);
    let first_two = &cereals[0..2];
    println!("{:?}", first_two);
    let mid_three = &cereals[1..4];
    println!("{:?}", mid_three);
    let mut last_three = &mut cereals[2..];
    println!("{:?}", last_three);
    last_three[2] = String::from("Lucky Charms");
    println!("The new last three is: {:?}", last_three);

    let cookie_crisp = &cereals[0];
    let cookie = &cookie_crisp[..6];
    println!("{cookie}");

    let cocoa_puffs = &cereals[3];
    let puffs = &cocoa_puffs[6..];
    println!("{puffs}");
}

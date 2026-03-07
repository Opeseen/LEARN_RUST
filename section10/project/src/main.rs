fn main() {
    Subscription::Free.summarize();
    Subscription::Basic(6.99, 2).summarize();
    Subscription::Premium { tier: Tier::Gold }.summarize();
    Subscription::Premium { tier: Tier::Silver }.summarize();
    Subscription::Premium {
        tier: Tier::Platinum,
    }
    .summarize();
}

#[derive(Debug)]
enum Tier {
    Gold,
    Silver,
    Platinum,
}

#[derive(Debug)]
enum Subscription {
    Free,
    Basic(f64, u32),
    Premium { tier: Tier },
}

impl Subscription {
    fn summarize(&self) {
        match self {
            Subscription::Free => {
                println!("You have limited access to the sites");
            }
            Subscription::Basic(price, months) => {
                println!(
                    "You have limited access to the site's premium features for {price} for {months} months"
                )
            }
            Subscription::Premium { tier } => {
                println!("You have full access to the site premium features. Your tier is {tier:?}")
            }
        }
    }
}

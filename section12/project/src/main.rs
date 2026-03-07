fn main() {
    let mario = Restaurant {
        // create a restaurant instance
        reservation: 11,
        has_mice_infestation: true,
    };
    println!("{:?}", mario.chef_special());
    println!("{:?}", mario.deliver_burger("123 Enm Street"));
    println!("");

    let angelo = Restaurant {
        reservation: 15,
        has_mice_infestation: false,
    };
    println!("{:?}", angelo.chef_special());
    println!("{:?}", angelo.deliver_burger(""));
    println!("");
}

#[derive(Debug)]
struct Food {
    name: String,
}
#[derive(Debug)]
struct Restaurant {
    reservation: u32,
    has_mice_infestation: bool,
}

impl Restaurant {
    fn chef_special(&self) -> Option<Food> {
        if self.has_mice_infestation {
            return None;
        }

        if self.reservation < 12 {
            Some(Food {
                name: String::from("Uni Sashimi"),
            })
        } else {
            Some(Food {
                name: String::from("Strip Steak"),
            })
        }
    }

    fn deliver_burger(&self, address: &str) -> Result<Food, String> {
        if self.has_mice_infestation {
            return Err("Sorry, we have a mice problem".to_string());
        }

        if address.is_empty() {
            return Err("No delivery address specified".to_string());
        }

        Ok(Food {
            name: String::from("Burger"),
        })
    }
}

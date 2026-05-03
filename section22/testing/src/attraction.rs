pub trait TicketSeller {
    fn sell_ticket(&mut self);
}

#[derive(Debug, PartialEq, Eq)]
pub struct Museum {
    pub paintings: Vec<String>,
    pub revenue: u32,
}

impl Museum {
    const MAXIMUM_CAPACITY: usize = 3;

    /// Creates a new Museum instance.
    ///
    /// # Examples
    /// ```
    /// use testing::attraction::Museum;
    /// let museum = Museum::new();
    /// let empty_vec: Vec<String> = Vec::new();
    /// assert_eq!(museum.paintings, empty_vec);
    /// assert_eq!(museum.revenue, 0)
    /// ```
    pub fn new() -> Self {
        Self {
            paintings: vec![],
            revenue: 0,
        }
    }

    /// Buys a painting for the museum
    ///
    /// # Examples
    /// ```
    /// use testing::attraction::Museum;
    ///
    /// let mut museum = Museum::new();
    /// museum.buy_painting("Mona Lisa");
    /// assert_eq!(museum.paintings, vec!["Mona Lisa".to_string()]);
    ///
    /// ```
    ///
    pub fn buy_painting(&mut self, painting: &str) {
        if self.paintings.len() >= Self::MAXIMUM_CAPACITY {
            panic!("Museum does not have storage space for another paintings");
        }
        self.paintings.push(painting.to_string());
    }

    // fn sell_ticket(&mut self) {
    //     self.revenue += 25;
    // }

    fn has_impressive_collection(&self) -> bool {
        self.paintings.len() > 2
    }
}

impl TicketSeller for Museum {
    fn sell_ticket(&mut self) {
        if self.has_impressive_collection() {
            self.revenue += 35
        } else {
            self.revenue += 25;
        }
    }
}

#[derive(Debug)]
pub struct MovieTheater {
    pub movies: Vec<String>,
    pub sales: u32,
}

impl MovieTheater {
    pub fn new() -> Self {
        Self {
            movies: vec![],
            sales: 0,
        }
    }

    pub fn add_movie(&mut self, movie: &str) {
        self.movies.push(movie.to_string());
    }
}

impl TicketSeller for MovieTheater {
    fn sell_ticket(&mut self) {
        self.sales += 15;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::collections::HashMap;
    #[test]
    fn museum_sells_ticket_to_increase_revenue() {
        let mut museum = Museum::new();
        museum.sell_ticket();
        assert_eq!(museum.revenue, 25);
    }

    #[test]
    fn museum_can_sell_multiple_tickets() {
        let mut museum = Museum::new();
        museum.sell_ticket();
        museum.sell_ticket();
        assert_eq!(museum.revenue, 50);
    }

    #[test]
    fn museum_can_have_impressive_art_collection() {
        let mut museum = Museum::new();
        museum.buy_painting("Mona Lisa");
        museum.buy_painting("The Starry Night");
        museum.buy_painting("Girl with a Pearl Earring");
        assert!(museum.has_impressive_collection());
    }

    // assert not equals
    #[test]
    fn museum_sells_ticket_to_increase_revenue2() {
        let mut museum = Museum::new();
        museum.sell_ticket();
        assert_ne!(museum.revenue, 0);
    }

    #[test]
    fn hashmaps() {
        let mut one = HashMap::new();
        one.insert("A", 2);
        one.insert("B", 3);

        let mut two = HashMap::new();
        two.insert("B", 3);
        two.insert("C", 4);

        // assert_eq!(one, two);
    }

    #[test]
    fn new_museums_are_equals() {
        let museum1 = Museum::new();
        let museum2 = Museum::new();
        assert_eq!(museum1, museum2);

        let mut museum1 = Museum::new();
        museum1.sell_ticket();
        let museum2 = Museum::new();
        // assert_eq!(museum1, museum2);
    }

    #[test]
    fn museum_sells_ticket_to_increase_revenue3() {
        let mut museum = Museum::new();
        museum.sell_ticket();
        assert_eq!(
            museum.revenue, 25,
            "The revenue from selling 1 ticket did not match expectations"
        );
    }

    #[test]
    fn museum_can_have_impressive_art_collection2() {
        let mut museum = Museum::new();
        museum.buy_painting("Mona Lisa");
        museum.buy_painting("The Starry Night");
        museum.buy_painting("Girl with a Pearl Earring");
        assert!(
            museum.has_impressive_collection(),
            "The museum did not have an impressive collections despite having 2 paintings"
        );
    }

    #[test]
    fn new_museums_are_equals2() {
        let museum1 = Museum::new();
        let museum2 = Museum::new();
        assert_eq!(
            museum1, museum2,
            "Two new museum instances were not found to be equal: {museum1:?} // {museum2:?}"
        );
    }

    #[test]
    #[should_panic(expected = "storage space")] // the should panic attribute
    fn museum_prohibits_adding_paintings_when_capacity_has_been_reached() {
        let mut museum = Museum::new();
        museum.buy_painting("Mona Lisa");
        museum.buy_painting("The Starry Night");
        museum.buy_painting("Girl with a Pearl Earring");
        museum.buy_painting("Water Lilies");
    }

    #[test]
    fn result_example() -> Result<(), String> {
        Ok(())
        // Err(String::from("Failure"))
    }

    #[test]
    fn museum_sells_ticket_to_increase_revenue4() -> Result<(), String> {
        let mut museum = Museum::new();
        museum.sell_ticket();
        if museum.revenue == 25 {
            Ok(())
        } else {
            Err(String::from(
                "The revenue from selling 1 ticket did not match the expectations",
            ))
        }
    }

    #[test]
    fn museum_with_impressive_art_collection_charges_more_for_admission() {
        let mut museum = Museum::new();
        museum.buy_painting("Mona Lisa");
        museum.buy_painting("The Starry Night");
        museum.buy_painting("Girl with a Pearl Earring");

        museum.sell_ticket();

        assert_eq!(museum.revenue, 35);
    }

    // using result enum in test
    #[test]
    fn museum_can_have_impressive_art_collection3() -> Result<(), String> {
        let mut museum = Museum::new();
        museum.buy_painting("Mona Lisa");
        museum.buy_painting("The Starry Night");
        museum.buy_painting("Girl with a Pearl Earring");

        if museum.has_impressive_collection() {
            Ok(())
        } else {
            Err(String::from(
                "The museum did not have an impressive collections despite having 2 paintings",
            ))
        }
    }

    // ignoring test
    #[test]
    #[ignore]
    fn new_museums_are_equals3() {
        let museum1 = Museum::new();
        let museum2 = Museum::new();
        assert_eq!(
            museum1, museum2,
            "Two new museum instances were not found to be equal: {museum1:?} // {museum2:?}"
        );
    }

    #[test]
    fn print_success() {
        println!("Success inside the function");
        assert!(true);
    }

    #[test]
    fn print_failure() {
        println!("Failure inside the function");
        // assert!(false);
    }
}

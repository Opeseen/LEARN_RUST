use crate::attraction::TicketSeller;

#[derive(Debug)]
pub struct VenueManagement<T: TicketSeller> {
    pub venue: T,
    pub manager: Option<String>,
}

impl<T: TicketSeller> VenueManagement<T> {
    pub fn new(venue: T) -> Self {
        Self {
            venue,
            manager: None,
        }
    }

    fn hire_manager(&mut self, manager: &str) {
        self.manager = Some(manager.to_string());
    }

    pub fn make_money(&mut self) {
        self.venue.sell_ticket();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::attraction::MovieTheater;
    use pretty_assertions::assert_eq;

    #[test]
    fn venue_management_can_hire_manager() {
        let movie_theater = MovieTheater::new();
        let mut venue_mgt = VenueManagement::new(movie_theater);
        venue_mgt.hire_manager("Mario");
        assert_eq!(venue_mgt.manager.unwrap(), String::from("Mario"));
    }
}

#[cfg(test)]
mod test2 {
    use super::*;
    use pretty_assertions::assert_eq;

    struct DummyVenue {}

    impl TicketSeller for DummyVenue {
        fn sell_ticket(&mut self) {}
    }

    #[test]
    fn venue_management_can_hire_manager() {
        let dummy_venue = DummyVenue {};
        let mut venue_mgt = VenueManagement::new(dummy_venue);
        venue_mgt.hire_manager("Mario");
        assert_eq!(venue_mgt.manager.unwrap(), String::from("Mario"));
    }
}

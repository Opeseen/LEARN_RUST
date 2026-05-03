use crate::attraction::{MovieTheater, TicketSeller};

#[derive(Debug)]
struct VenueManagement {
    venue: MovieTheater,
    manager: Option<String>,
}

impl VenueManagement {
    fn new() -> Self {
        Self {
            venue: MovieTheater::new(),
            manager: None,
        }
    }

    fn hire_manager(&mut self, manager: &str) {
        self.manager = Some(manager.to_string());
    }

    fn make_money(&mut self) {
        self.venue.sell_ticket();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn venue_management_can_hire_manager() {
        let mut venue_mgt = VenueManagement::new();
        venue_mgt.hire_manager("Mario");
        assert_eq!(venue_mgt.manager.unwrap(), String::from("Mario"));
    }
}

use pretty_assertions::assert_eq;
use rstest::{fixture, rstest};

use testing::attraction::{MovieTheater, Museum};
use testing::management2::VenueManagement;

#[fixture]
fn museum_with_three_paintings() -> Museum {
    let mut museum = Museum::new();
    museum.buy_painting("Mona Lisa");
    museum.buy_painting("The Starry Night");
    museum.buy_painting("Girl with a Pearl Earring");
    museum
}

#[fixture]
fn museum_management(museum_with_three_paintings: Museum) -> VenueManagement<Museum> {
    VenueManagement::new(museum_with_three_paintings)
}

#[fixture]
fn movie_management_with_one_movie() -> MovieTheater {
    let mut movie_theater = MovieTheater::new();
    movie_theater.add_movie("Titanic");
    movie_theater
}

#[fixture]
fn movie_theater_management(
    movie_management_with_one_movie: MovieTheater,
) -> VenueManagement<MovieTheater> {
    VenueManagement::new(movie_management_with_one_movie)
}

#[test]
fn venue_management_interacts_with_venue() {
    let mut museum = Museum::new();
    museum.buy_painting("Mona Lisa");

    let mut venue_mgt = VenueManagement::new(museum);
    venue_mgt.make_money();
    assert_eq!(venue_mgt.venue.revenue, 25);
}

#[rstest]
fn venue_management_interacts_with_venue2(museum_with_three_paintings: Museum) {
    let mut venue_mgt = VenueManagement::new(museum_with_three_paintings);
    venue_mgt.make_money();
    assert_eq!(venue_mgt.venue.paintings.len(), 3);
    assert_eq!(venue_mgt.venue.revenue, 35);
}

#[rstest]
fn venue_management_interacts_with_movie_theater_venue(
    mut movie_theater_management: VenueManagement<MovieTheater>,
) {
    movie_theater_management.make_money();
    assert_eq!(movie_theater_management.venue.sales, 15);
}

// extern crate geodesy;
use geodesy::{Coordinate, Location};

pub fn main() {
    let start = Coordinate{
        utc: 0.0,
        latitude: 51.55814,
        longitude: -0.02955,
        altitude: 0.0
    };
    let end = Coordinate{
        utc: 0.0,
        latitude: 51.55795,
        longitude: -0.03014,
        altitude: 0.0
    };
    let locations = vec![start, end];
    let d = locations.distance_between_points();

    println!("{:?}", d);
}
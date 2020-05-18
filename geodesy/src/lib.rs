use std::fs::File;
use std::io::prelude::*;

/// # Geodesy library for all that extra fun post data gathering analysis.
///
/// One to work out the distance between two points.
/// Make a structure of a series of points and implement.
/// At any point you are doing stuff to a series of points.


/// This is the basic coordinate data for a single point in space.

mod functions;

#[derive(Default, PartialEq, Debug)]
pub struct Coordinate {
    pub utc: f64,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
}

pub trait Location {
    fn distance_between_points(&self) -> Vec<f64>;
    fn accuracy_between_points(&self) -> Coordinate;
}

impl Location for Vec<Coordinate> {
    fn distance_between_points(&self) -> Vec<f64> {
        let mut return_vec: Vec<f64> = Vec::new();
        for t in 0..self.len() - 1 {
            let d = functions::inverse_vincenty(self.get(t).unwrap(), self.get(t + 1).unwrap());
            return_vec.push(d)
        }
        return return_vec;
    }

    fn accuracy_between_points(&self) -> Coordinate {
        functions::average_long_lat(self)
    }
}


fn load_points(file_path: &str) -> Vec<Coordinate> {
    // Format: UTC,Lat,Long,Sats,Geoid,MSL_alt
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let contents = contents.trim();
    let data: Vec<&str> = contents.split("\n").collect();

    let mut locations_vec: Vec<Coordinate> = Vec::new();
    for line in data {
        let line: Vec<&str> = line.split(",").collect();
        let location = Coordinate {
            utc: line.get(0).unwrap().parse().unwrap(),
            latitude: line.get(1).unwrap().parse().unwrap(),
            longitude: line.get(2).unwrap().parse().unwrap(),
            altitude: line.get(5).unwrap().parse().unwrap(),
        };
        locations_vec.push(location);
    }
    return locations_vec;
}

use geodesy::kinematics::{DeltaCoordinates, Kinematics};
use geodesy::Coordinate;

use std::fs::File;
use std::io::Read;


fn load_points(file_path: &str) -> Vec<Coordinate> {
    // Format: Header,UTC,Lat,Long,Sats,MSL_alt
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let contents = contents.trim();
    let data: Vec<&str> = contents.split("\n").collect();

    let mut locations_vec: Vec<Coordinate> = Vec::new();
    for line in data {
        let line: Vec<&str> = line.split(",").collect();
        if line.get(0).unwrap() == & "GGA"{
            let location = Coordinate {
                utc: line.get(1).unwrap().parse().unwrap(),
                latitude: line.get(2).unwrap().parse().unwrap(),
                longitude: line.get(3).unwrap().parse().unwrap(),
                altitude: line.get(5).unwrap().parse().unwrap(),
            };
            if location.latitude == 0.0 {
            } else {
                locations_vec.push(location);
            }
        }
    }
    return locations_vec;
}


pub fn main() {
    let locations: Vec<Coordinate> = load_points("feldspar5-4_gps.txt");

    let d1 = locations.haversine().speed();

    println!("{:?}", d1);
}
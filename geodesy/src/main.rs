// use geodesy::kinematics::{DeltaCoordinates, Kinematics};
// use geodesy::Coordinate;

use adafruit_gps::GpsSentence;
use geodesy::position::{GpsSentenceConverter, Position};


fn main() {
    // This code converts gps data into coordinate data and then produces a klm file which can be
    // uploaded to google earth for visualisation.
    let flight_num = "3";
    let mut mod_vec = Vec::new();
    let vec = GpsSentence::read_from(format!("./feldspar5-{}_gps", flight_num).as_str());
    for item in vec.iter() {
        match item {
            GpsSentence::GGA(s) => {
                let mut new_item = s.clone();
                new_item.geoidal_sep = Some(47.0);
                mod_vec.push(GpsSentence::GGA(new_item))
            }
            _ => {}
        }
    }
    let coords = mod_vec.to_coords(true);
    let _ = coords.to_klm(format!("5-{}", flight_num).as_str(), format!("Feldspar 5-{} flight path", flight_num).as_str());
}
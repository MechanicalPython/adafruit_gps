// use geodesy::kinematics::{DeltaCoordinates, Kinematics};
// use geodesy::Coordinate;

use std::fs::File;
use std::io::Read;

use adafruit_gps::GpsSentence;
use adafruit_gps::gga::GgaData;
use adafruit_gps::gsa::GsaData;
use geodesy::position::{GpsSentenceConverter, Position};

#[allow(dead_code)]
fn convert_to_bytes(file_path: &str, bytes_file: &str) {
    // Format: Header,UTC,Lat,Long,Sats,MSL_alt
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let contents = contents.trim();
    let data: Vec<&str> = contents.split("\n").collect();

    for line in data {
        let line: Vec<&str> = line.split(",").collect();
        if line.get(0).unwrap() == &"GGA" {
            let gga = GpsSentence::GGA(GgaData {
                utc: line.get(1).unwrap().parse().unwrap(),
                lat: Some(line.get(2).unwrap().parse().unwrap()),
                long: Some(line.get(3).unwrap().parse().unwrap()),
                sat_fix: Default::default(),
                satellites_used: line.get(4).unwrap().parse().unwrap(),
                hdop: None,
                msl_alt: Some(line.get(5).unwrap().parse().unwrap()),
                geoidal_sep: None,
                age_diff_corr: None,
            });
            gga.append_to(&bytes_file)
        } else if line.get(0).unwrap() == &"GSA" {
            let gsa = GpsSentence::GSA(GsaData {
                mode: Default::default(),
                dimension_fix: Default::default(),
                sat1: None,
                sat2: None,
                sat3: None,
                sat4: None,
                sat5: None,
                sat6: None,
                sat7: None,
                sat8: None,
                sat9: None,
                sat10: None,
                sat11: None,
                sat12: None,
                pdop: Some(line.get(4).unwrap().parse().unwrap()),
                hdop: Some(line.get(3).unwrap().parse().unwrap()),
                vdop: Some(line.get(2).unwrap().parse().unwrap()),
            });
            gsa.append_to(&bytes_file)
        } else {}
    }
}


fn main() {
    let flight_num = "2";
    let vec = GpsSentence::read_from(format!("./feldspar5-{}_gps", flight_num).as_str());

    let coords = vec.to_coords();
    coords.to_klm(format!("5-{}", flight_num).as_str(), format!("Feldspar 5-{} flight path", flight_num).as_str());
}
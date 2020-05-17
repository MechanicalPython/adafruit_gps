extern crate adafruit_gps;

use adafruit_gps::save_gps_coordinates;
use adafruit_gps::geodesy::Location;
use adafruit_gps::gps::{Gps, open_port, GpsSentence};

fn main() {
    let port = open_port("/dev/serial0", 57600);
    let mut gps = Gps{port};


    for _ in 0..100 {
        let values = gps.update();
        return match values {
            GpsSentence::GGA(sentence) => {
                sentence.lat;
                sentence.long;

            },

            _ => {}
        }
    }


}
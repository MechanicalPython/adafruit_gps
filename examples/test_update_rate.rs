extern crate adafruit_gps;

pub use adafruit_gps::gps::{GetGpsData, Gps, open_port};
use adafruit_gps::PMTK::send_pmtk::{SendPmtk, set_baud_rate};

// For use in testing your gps modules update rate. type the update rate in miliseconds in the cmd line.


fn main() {

}

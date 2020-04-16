extern crate adafruit_gps;

pub use adafruit_gps::gps::{GetGpsData, Gps, open_port};
use adafruit_gps::PMTK::send_pmtk::SendPmtk;

fn main() {
    let port = open_port("/dev/serial0");
    let mut gps = Gps {port};
    gps.pmtk_314_api_set_nmea_output(1,1,1,1,1,1,1);
    for _i in 0..100 {
        let l = gps.read_line();
        dbg!(l);
    }
    // Call a method that gets you all the data from a pre_defined list.


}
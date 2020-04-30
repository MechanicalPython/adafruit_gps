extern crate adafruit_gps;

pub use adafruit_gps::gps::{GetGpsData, Gps, open_port};
use adafruit_gps::PMTK::send_pmtk::SendPmtk;

// use std::thread;
// use std::time::Duration;

fn main() {
    let port = open_port("/dev/serial0", 9600);
    let mut gps = Gps { port , satellite_data: true, naviagtion_data: true };

    gps.pmtk_314_api_set_nmea_output(0, 0, 1, 1, 1, 1, 1);

    loop {
        let values = gps.read_line();
        println!("{}", values);
        // thread::sleep(Duration::from_secs(1))
    }
}
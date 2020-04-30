extern crate adafruit_gps;

use std::env;

pub use adafruit_gps::gps::{GetGpsData, Gps, open_port};
use adafruit_gps::PMTK::send_pmtk::SendPmtk;

// For use in testing your gps modules update rate. type the update rate in miliseconds in the cmd line.

fn main() {
    let args: Vec<String> = env::args().collect();

    let baud_rate = args.get(2).unwrap();
    let baud_rate: u32 = baud_rate.parse().unwrap();
    let port = open_port("/dev/serial0", baud_rate);
    let mut gps = Gps { port , satellite_data: true, naviagtion_data: true };

    let update_r = gps.pmtk_220_set_nmea_updaterate(&args[1]);
    dbg!(update_r);

    for _ in 0..10 {
        let values = gps.update();
        println!("{}", values.utc);
    }
}
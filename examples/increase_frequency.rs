extern crate adafruit_gps;

pub use adafruit_gps::gps::{GetGpsData, Gps, open_port};
use adafruit_gps::PMTK::send_pmtk::{set_baud_rate};

use std::env;

fn main() {
    // These args are just for easy testing for what baud rate and what update rate you want work.
    let args: Vec<String> = env::args().collect();
    let baud_rate = args.get(1).unwrap();
    let update_rate = args.get(2).unwrap();

    // First, set the baud rate.
    set_baud_rate(baud_rate, "/dev/serial0");

    // Then open the port to the gps and you're good.
    let port = open_port("/dev/serial0", baud_rate.parse::<u32>().unwrap());
    // Initialise the Gps.
    let mut gps = Gps {port, satellite_data: true, naviagtion_data: true };
    gps.init(update_rate);

    for _ in 0..10 {
        let values = gps.update();
        println!("{}", values.utc);
    }

}

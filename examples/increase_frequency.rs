extern crate adafruit_gps;

pub use adafruit_gps::gps::{self, GetGpsData, Gps, open_port};
use adafruit_gps::PMTK::send_pmtk::{set_baud_rate, SendPmtk};

use std::env;
use std::str;

fn main() {
    // These args are just for easy testing for what baud rate and what update rate you want work.
    // let mut v = vec![154, 233, 202, 106, 82, 68, 58, 49, 29, 77, 89, 177, 197, 98, 138, 98, 130, 130, 82, 178, 170, 106, 82, 68, 58, 57, 89, 81, 29, 177, 193, 114, 130, 130, 98, 162, 177, 177, 106, 177, 193, 114, 130, 130, 98, 114, 177, 193, 114, 130, 130, 98, 90, 177, 57, 169, 201, 26, 53, 41, 254, 36, 71, 78, 71, 71, 65, 44, 48, 48, 49, 51, 51, 54, 46, 56, 53, 57, 44, 44, 44, 44, 44, 48, 44, 48, 44, 44, 44, 77, 44, 44, 77, 44, 44, 42, 53, 53, 13, 10];
    // v.retain(|&element| element < 128);
    // let string: String = str::from_utf8(&v).unwrap_or("Invalid bytes given").to_string();
    // println!("{}", string);
    let args: Vec<String> = env::args().collect();
    let baud_rate = args.get(1).unwrap();
    let update_rate = args.get(2).unwrap();

    // First, set the baud rate.
    let r = set_baud_rate(baud_rate, "/dev/serial0");
    println!("{:?}", r);

    // Then open the port to the gps and you're good.
    let port = open_port("/dev/serial0", baud_rate.parse::<u32>().unwrap());
    // Initialise the Gps.
    let mut gps = Gps {port, satellite_data: true, naviagtion_data: true };
    let r = gps.init(update_rate);
    println!("{:?}", r.get("Update rate").unwrap());

    for _ in 0..10 {
        let values = gps.update();
        println!("{}", values.utc);
    }
    gps.send_command(format!("PMTK251,9600").as_str());
    for _ in 0..10 {
        let values = gps.update();
        println!("{}", values.utc);
    }
    // Important note:
    // Valid baud rates are 4800,9600,14400,19200,38400,57600,115200.
    // However, not all baud rates will work, so some trial and error will be needed.
}

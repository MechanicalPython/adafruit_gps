extern crate adafruit_gps;

pub use adafruit_gps::gps::{self, GetGpsData, Gps, open_port};
use adafruit_gps::PMTK::send_pmtk::{set_baud_rate, SendPmtk};

use std::env;

fn main() {
    // These args are just for easy testing for what baud rate and what update rate you want work.
    let args: Vec<String> = env::args().collect();
    let baud_rate = args.get(1).unwrap();
    let update_rate = args.get(2).unwrap();

    // First, set the baud rate.
    let r = set_baud_rate(baud_rate, "/dev/serial0");
    println!("{:?}", r);

    // // Then open the port to the gps and you're good.
    // let port = open_port("/dev/serial0", baud_rate.parse::<u32>().unwrap());
    // println!("{:?}", port.baud_rate());
    // // Initialise the Gps.
    // let mut gps = Gps {port, satellite_data: true, naviagtion_data: true };
    // let r = gps.init(update_rate);
    // println!("{:?}", r.get("Update rate").unwrap());
    //
    // for _ in 0..10 {
    //     let values = gps.update();
    //     println!("{}", values.utc);
    // }
    // gps.send_command(format!("PMTK251,9600").as_str());
    // for _ in 0..10 {
    //     let values = gps.update();
    //     println!("{}", values.utc);
    // }
    // Important note:
    // Valid baud rates are 4800,9600,14400,19200,38400,57600,115200.
    // However, not all baud rates will work, so some trial and error will be needed.
}

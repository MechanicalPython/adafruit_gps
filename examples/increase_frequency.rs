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
    let r = gps.init(update_rate);
    println!("{:?}", r.get("Update rate").unwrap());

    for _ in 0..10 {
        let values = gps.update();
        println!("{}", values.utc);
    }

    // Important note:
    // If for some reason this doesn't work and you get invalid byte sentences, the port baud rate
    // and the gps baud rate are out of sync. So best thing to do is reboot the gps (physically unplug it, without battery)
    // or change the port baud setting using: stty -F /dev/port baudrate clocal cread cs8 -cstopb -parenb
    // until cat /dev/port gives sentences ($GGA,stuff kind of strings).
    // Valid baud rates are 4800,9600,14400,19200,38400,57600,115200.
}

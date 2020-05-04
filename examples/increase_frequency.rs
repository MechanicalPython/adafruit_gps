extern crate adafruit_gps;
use std::env;


use adafruit_gps::gps::{self, Gps, open_port, PortConnection};
use adafruit_gps::PMTK::send_pmtk::set_baud_rate;

fn main() {
    // These args are just for easy testing for what baud rate and what update rate you want work.

    let args: Vec<String> = env::args().collect();
    let baud_rate = args.get(1).unwrap();
    let update_rate = args.get(2).unwrap();

    // First, set the baud rate. If it returns an error, just try again.
    let r = set_baud_rate(baud_rate, "/dev/serial0");
    println!("{:?}", r);

    // Then open the port to the gps and you're good.
    let port = open_port("/dev/serial0", baud_rate.parse::<u32>().unwrap());
    // Initialise the Gps.
    let mut gps = Gps {port};
    // Sometimes the first line read is invalid.
    let mut valid_count: i32 = 0;
    for _ in 0..5 {
        let line = gps.read_line();
        if line.connection == PortConnection::Valid{
            valid_count += 1;
        }
    }
    if valid_count == 0 {
        panic!("No valid bytes returned from gps. Try and different baud rate and frequency combination.")
    }
    let r = gps.init(update_rate);
    println!("{:?}", r.get("Update rate").unwrap());

    for _ in 0..10 {
        let values = gps.update();
        println!("{}", values.utc);
    }

    // going from 57600 at 100 to 9600 at 100 does not work.
    // Important note:
    // Valid baud rates are 4800,9600,14400,19200,38400,57600,115200.
    // However, not all baud rates will work, so some trial and error will be needed.
    // For me, 9600 and 57600 are the only valid rates.
    // If you are at a high baud rate and high frequency and you try to go to a lower baud rate
    // but don't lower the frequency, it will fail.

    // Some useful commands for debugging:
    // cat /dev/port -> prints out what that port is getting
    // stty -F /dev/port baud_rate clocal cread cs8 -cstopb -parenb -> sets the port baud rate
    // stty -F /dev/port -> prints out the port's current baud rate.
}

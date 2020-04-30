extern crate adafruit_gps;

use std::env;

pub use adafruit_gps::gps::{GetGpsData, Gps, open_port};
use adafruit_gps::PMTK::send_pmtk::SendPmtk;
use std::io::Write;
use std::time::Duration;
use std::thread;

// For use in testing your gps modules update rate. type the update rate in miliseconds in the cmd line.

fn main() {
    let args: Vec<String> = env::args().collect();
    // args: hz, baudrate for port, baudrate for gps.
    let baud_rate = args.get(2).unwrap();
    {
        let port = open_port("/dev/serial0", 9600);
        let mut gps = Gps { port , satellite_data: true, naviagtion_data: true };
        gps.pmtk_251_set_nmea_baudrate(baud_rate);
        thread::sleep(Duration::from_secs(1))
    }
    let port = open_port("/dev/serial0", baud_rate.parse::<u32>().unwrap());
    println!("{:?}", port.baud_rate());
    let mut gps = Gps { port , satellite_data: true, naviagtion_data: true };


    let update_b = gps.pmtk_251_set_nmea_baudrate(baud_rate);
    dbg!(update_b);
    gps.pmtk_314_api_set_nmea_output(0, 1, 0, 0, 0, 0, 1);

    let update_r = gps.pmtk_220_set_nmea_updaterate(&args[1]);
    dbg!(update_r);


    for _ in 0..10 {
        let values = gps.update();
        println!("{}", values.utc);
    }
}
extern crate adafruit_gps;

use std::env;
use std::process::Command;
pub use adafruit_gps::gps::{GetGpsData, Gps, open_port};
use adafruit_gps::PMTK::send_pmtk::{self, SendPmtk};

// use std::time::Duration;
// use std::thread;

// For use in testing your gps modules update rate. type the update rate in miliseconds in the cmd line.

fn main() {
    // Send baudrate change to the gps -> echo \cmd > /dev/serial0
    // change the serial port baudrate -> stty -F /dev/serial0 raw 19200 cs8 clocal -cstopb
    // Open gps and update the hz.

    let port_name = "/dev/serial0";

    let cmd = send_pmtk::add_checksum("PMTK251,19200".to_string());

    Command::new("echo").arg(format!("\\{}", cmd).as_str()).arg(">").arg("/dev/serial0");

    // stty -F /dev/serial0 raw 19200 cs8 clocal -cstopb
    Command::new("stty")
        .arg("-F")
        .arg("/dev/serial0")
        .arg("raw")
        .arg("19200")
        .arg("cs8")
        .arg("clocal")
        .arg("-cstopb");


    let port = open_port("/dev/serial0", 19200);

    let mut gps = Gps { port , satellite_data: true, naviagtion_data: true };

    let _ = gps.pmtk_251_set_nmea_baudrate("19200");

    gps.pmtk_314_api_set_nmea_output(0, 1, 0, 0, 0, 0, 1);

    let update_r = gps.pmtk_220_set_nmea_updaterate(100);
    dbg!(update_r);

    for _ in 0..10 {
        let values = gps.update();
        println!("{}", values.utc);
    }
}
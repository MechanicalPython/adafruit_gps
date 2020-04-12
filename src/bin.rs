use std::str;
use std::thread;
use std::time::Duration;

use adafruit_gps::{Gps, GpsArgValues, open_port, SendPmtk};


fn main() {
    let mut port = open_port("/dev/serial0");
    let mut gps = Gps{port, gps_type: "MT3339" };
    thread::sleep(Duration::from_secs(1));
    gps.send_command("$PMTK220,1000", true);


}
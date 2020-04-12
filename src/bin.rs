use std::str;
use std::thread;
use std::time::Duration;

use adafruit_gps::{Gps, GpsArgValues, open_port, SendPmtk};


fn main() {
    // let mut port = open_port("/dev/serial0");
    // let mut gps = Gps{port, gps_type: "MT3339" };
    // gps.send_command("$PMTK301,1", true);

    let s = "$PMTK001,0*AB";
    let slice = &s[0..5];
    dbg!(slice);


}
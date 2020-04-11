use std::time::{SystemTime};

use adafruit_gps::{Gps, GpsArgValues, open_port, SendPmtk};


fn main() {
    let mut port = open_port("/dev/serial0");
    let mut gps = Gps{port, gps_type: "MT3339" };
    gps.pmtk_010_sys_msg()



}
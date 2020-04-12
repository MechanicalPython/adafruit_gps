use std::str;
use std::thread;
use std::time::Duration;

use std::env;

use adafruit_gps::{Gps, GpsArgValues, open_port, SendPmtk};


fn main() {
    let args:Vec<String> = env::args().collect();
    dbg!(&args);
    let cmd:&String = args.get(1).expect("No command given.");
    let mut port = open_port("/dev/serial0");
    let mut gps = Gps{port, gps_type: "MT3339" };
    thread::sleep(Duration::from_secs(1));
    gps.send_command(format!("${}", cmd).as_str(), true);


}
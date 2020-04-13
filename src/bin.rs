#![allow(warnings)]

use std::env;
use std::str;
use std::thread;
use std::time::Duration;

use adafruit_gps::{Gps, GpsArgValues, open_port, SendPmtk, GetData};

fn main() {
    let args:Vec<String> = env::args().collect();
    let mut port = open_port("/dev/serial0");
    let mut gps = Gps{port, gps_type: "MT3339" };

    if args.len() == 2 {
        let cmd:&String = args.get(1).expect("No command given.");
        gps.send_command(format!("{}", cmd).as_str(), true);
        for i in 0..5 {
            let l = gps.read_line();
            println!("{}", l)
        }
    } else {
        let l = gps.read_line();
        println!("{}", l)
    }
}
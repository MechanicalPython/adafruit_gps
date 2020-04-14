use std::env;

extern crate adafruit_gps;
pub use adafruit_gps::gps::{Gps, open_port, GetGpsData};


fn main() {
    let args:Vec<String> = env::args().collect();

    let port = open_port("/dev/serial0");
    let mut gps = Gps{port, gps_type: "MT3339" };

    if args.len() == 2 {
        let _cmd:&String = args.get(1).expect("No command given.");

        for _i in 0..50 {
            let l = gps.read_line();
            dbg!(l);
        }
    } else {
        let l = gps.read_line();
        println!("{}", l)
    }
}
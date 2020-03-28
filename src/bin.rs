// use std::fs::File;
// use std::io::Read;
use std::thread::sleep;
use std::time::{Duration};

use mylib::Gps;

#[allow(unused_imports)]
#[allow(unused_variables)]

fn main() {
    let mut gps = Gps{
        port: mylib::open_port("/dev/serial0"),
    };
    sleep(Duration::from_secs(60));
    &gps.parse_sentence();

}

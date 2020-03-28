
#[allow(unused_imports)]
#[allow(unused_variables)]

// use std::fs::File;
// use std::io::Read;
// use std::thread::sleep;
// use std::time::{Duration};
// use std::str;
use mylib::Gps;



fn main() {
    let mut gps = Gps{
        port: mylib::open_port("/dev/serial0"),
    };

    &gps.parse_sentence();


}

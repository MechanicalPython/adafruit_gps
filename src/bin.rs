
#[allow(unused_imports)]
#[allow(unused_variables)]

use std::thread::sleep;
use std::time::{Duration};

use mylib::Gps;



fn main() {

    let mut gps = Gps::default();
    gps.port = mylib::open_port("/dev/serial0");

    loop {
        &gps.update();
        &gps.timestamp;
        sleep(Duration::from_secs(1));
    }

}

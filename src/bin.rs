
#[allow(unused_imports)]
#[allow(unused_variables)]

use std::thread::sleep;
use std::time::{Duration};

use mylib::Gps;



fn main() {

    let mut gps = Gps{port: mylib::open_port("/dev/serial0"), ..Default::default()};

    loop {
        &gps.update();
        sleep(Duration::from_secs(1));
    }

}

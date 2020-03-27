// use std::fs::File;
// use std::io::Read;

use mylib;

#[allow(unused_imports)]
#[allow(unused_variables)]

fn main() {
    let gps = mylib::read_serial_port("/dev/serial0");
    // let string = mylib::port_vec_to_string(gps);

    println!("{:?}", gps);




}

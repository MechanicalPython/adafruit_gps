// use std::fs::File;
// use std::io::Read;

use mylib;
use std::env::var;
// use std::str::from_utf8;
// use std::i64;

#[allow(unused_imports)]
#[allow(unused_variables)]

fn main() {
    let gps = mylib::read_serial_port("/dev/serial0");
    let string = mylib::port_vec_to_string(gps);

    println!("{}", string);




}

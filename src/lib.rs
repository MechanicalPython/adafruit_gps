#![allow(dead_code)]
#![allow(unused_imports)]

extern crate serialport;

use std::io::{self, Write};
use std::time::Duration;

use serialport::prelude::*;


pub fn read_serial_port() {
    let port_name = "/dev/serial0";
    let baud_rate:u32 = 9600;

    let mut settings: SerialPortSettings = Default::default();
    settings.timeout = Duration::from_millis(1000);
    settings.baud_rate = baud_rate;

    let port = serialport::open_with_settings(&port_name, &settings);
    let mut serial_buf: Vec<u8> = vec![0; 1000];
    port.read(serial_buf.as_mut_slice());
    println!("{}", port);

}

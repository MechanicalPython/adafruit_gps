#![allow(dead_code)]
#![allow(unused_imports)]

extern crate serialport;

use std::io::{self, Write};
use std::time::Duration;

use serialport::prelude::*;


pub fn read_serial_port() {
    let port_name = "/dev/serial0";
    let baud_rate = "9600";

    let mut settings: SerialPortSettings = Default::default();
    settings.timeout = Duration::from_millis(1000);

    if let Ok(rate) = baud_rate.parse::<u32>() {
        settings.baud_rate = rate.into();
    } else {
        eprintln!("Error: Invalid baud rate '{}' specified", baud_rate);
        ::std::process::exit(1);
    }

    match serialport::open_with_settings(&port_name, &settings) {
        Ok(mut port) => {
            let mut serial_buf: Vec<u8> = vec![0; 1000];
            loop {
                match port.read(serial_buf.as_mut_slice()) {
                    Ok(t) => io::stdout().write_all(&serial_buf[..t]).unwrap(),
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("{:?}", e),
                }
            }
        }

        Err(e) => {
            eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
            ::std::process::exit(1);
        }
    }

}

#![allow(dead_code)]
#![allow(unused_imports)]

extern crate serialport;

use std::io::{self, Write};
use std::time::Duration;

use serialport::prelude::*;


pub fn read_serial_port() {
    let port_name = "/dev/serial0";

    let settings = SerialPortSettings {
        baud_rate: 9600,
        data_bits: DataBits::Eight,
        flow_control: FlowControl::None,
        parity: Parity::None,
        stop_bits: StopBits::One,
        timeout: Duration::from_millis(1000),
    };

    match serialport::open_with_settings(&port_name, &settings) {
        Ok(mut port) => { // Port is now open.
            let mut buffer: Vec<u8> = Vec::new();
            loop {
                match port.read(buffer.as_mut_slice()) {
                    Ok(t) =>
                        println!("{:?}", t),

                    Err(e) => (eprint!("{:?}", e)),
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
            ::std::process::exit(1);
        }
    }


}

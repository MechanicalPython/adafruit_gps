#![allow(dead_code)]
#![allow(unused_imports)]

extern crate serialport;

use std::str;
use std::io::{self, Write, Read};
use std::time::{SystemTime, Duration};

use serialport::prelude::*;
use std::thread::sleep;

fn vec_to_str(v:Vec<u8>) {
    let s = str::from_utf8(&v).unwrap();
    println!("{}", s);
}

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
    let mut port = serialport::open_with_settings(&port_name, &settings).unwrap();
    let mut buffer: Vec<u8> = vec![0;1000];
    let mut sentence:Vec<u8> = Vec::new();

    let s = SystemTime::now();
    while s.elapsed().unwrap() < Duration::from_secs(1) {
        match port.read(buffer.as_mut_slice()) {
            Ok(_t) => {
                sentence.push(buffer[_t]);
                println!("{:?} -- {:?}\n", buffer[_t], _t);
            },
            Err(e) => (eprint!("{:?}\n", e)),
        }
    }
    println!("{:?}", buffer);

    // match serialport::open_with_settings(&port_name, &settings) {
    //     Ok(mut port) => { // Port is now open.
    //         let mut buffer: Vec<u8> = vec![0;1000];
    //         loop {
    //             // So create a massive vector. The port.read() method then changes the values
    //             // in that vector and returns t, the number of valid bytes (non 0).
    //             match port.read(buffer.as_mut_slice()) {
    //                 Ok(_t) => {
    //                     println!("{:?} -- {:?}\n", &buffer[..t], _t);
    //
    //                 }
    //
    //                 ,
    //
    //                 Err(e) => (eprint!("{:?}\n", e)),
    //             }
    //         }
    //     }
    //     Err(e) => {
    //         eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
    //         ::std::process::exit(1);
    //     }
    // }
}

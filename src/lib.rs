#![allow(dead_code)]
#![allow(unused_imports)]
/// General structure
/// GPS enum has all the items that are needed.
///
///
///
///
extern crate serialport;

use std::i64;
use std::io::{self, Read, Write};
use std::str;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

use serialport::prelude::*;

pub fn read_serial_port(port_name: &str) -> Vec<u8> {
    let settings = SerialPortSettings {
        baud_rate: 9600,
        data_bits: DataBits::Eight,
        flow_control: FlowControl::None,
        parity: Parity::None,
        stop_bits: StopBits::One,
        timeout: Duration::from_millis(1000),
    };

    let mut port = serialport::open_with_settings(&port_name, &settings).unwrap();
    let mut buffer: Vec<u8> = vec![0; 1000];
    let mut output: Vec<u8> = Vec::new();

    // let s = SystemTime::now();
    // while s.elapsed().unwrap() < Duration::from_secs(1) {
    loop{
        match port.read(buffer.as_mut_slice()) {
            Ok(_t) => {
                output.extend_from_slice(&buffer[.._t]);
                println!("{:?}", buffer);
            }
            Err(_e) => (),
        }
    }
    return output;
}

pub fn port_vec_to_string(vector:Vec<u8>) -> String {
    let string = str::from_utf8(&vector).unwrap().to_string();
    return string;
}
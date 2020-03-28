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

pub fn open_port(port_name: &str) -> Box<dyn SerialPort> {
        let settings = SerialPortSettings {
            baud_rate: 9600,
            data_bits: DataBits::Eight,
            flow_control: FlowControl::None,
            parity: Parity::None,
            stop_bits: StopBits::One,
            timeout: Duration::from_millis(1000),
        };
        match serialport::open_with_settings(&port_name, &settings) {
            Ok(port) => return port,
            Err(_e) => panic!("Port not found."),
        }
    }

pub struct Gps {
    pub port: Box<dyn SerialPort>,
}

impl Gps {
    pub fn read_port(&mut self) -> Vec<u8> {
        // Maximum port buffer size is 4095, or 1000 numbers.
        // Returns whatever is in the port.
        // Start of a line is $ and end is \n. So
        let mut buffer: Vec<u8> = vec![0; 100];
        let mut output: Vec<u8> = Vec::new();
        let p = &mut self.port;

        match p.read(buffer.as_mut_slice()) {
            Ok(_t) => {
                println!("{:?}", buffer);
                output.extend_from_slice(&buffer[.._t]);
            }
            Err(_e) => (),
        }
        return output;
    }
    pub fn parse_sentence(&mut self) -> (){
        let port_reading = self.read_port();
        let string = str::from_utf8(&port_reading).unwrap();
        println!("{:?}", string);
    }
}

#![allow(dead_code)]
#![allow(unused_imports)]
/// General structure
/// GPS enum has all the items that are needed.
/// The way it works. Constantly call gps.update(). This will update the variables with the
/// most up to date items (each type of prefix indicates a different level of importance)
/// And then every second print the most up to date info.

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
    fn read_line(&mut self) -> Vec<u8> {
        // Maximum port buffer size is 4095.
        // Returns whatever is in the port.
        // Start of a line is $ (36) and end is \n (10). So if
        // The correct line length is 70 (probably).
        let mut buffer: Vec<u8> = vec![0; 4095];  // Reads what is in the buffer, be it nothing or max.
        let mut output: Vec<u8> = Vec::new();
        let p = &mut self.port;
        let mut cont = true;
        while cont {
            match p.read(buffer.as_mut_slice()) {
                Ok(buffer_size) => {
                    output.extend_from_slice(&buffer[..buffer_size]);
                    while output.get(0).unwrap() != &36u8 {  // Remove all characters before $
                        output.remove(0);
                    }
                    if buffer[..buffer_size].contains(&10u8) {
                        cont = false;
                        while output.get(output.len() - 1).unwrap() != &10u8 {
                            output.remove(output.len() - 1);
                        }
                    }
                }
                Err(_e) => (),
            }
        }
        return output;
    }

    pub fn update(self) {}

    pub fn parse_sentence(&mut self) -> () {
        // Return only the last valid line.
        let port_reading = self.read_line();
        let string = str::from_utf8(&port_reading).unwrap();
        println!("{:?}", string);
    }
}

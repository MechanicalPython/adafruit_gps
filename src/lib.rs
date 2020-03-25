#![allow(dead_code)]
#![allow(unused_imports)]

extern crate serial;

use std::env;
use std::io;
use std::time::Duration;

use std::io::prelude::*;
use serial::prelude::*;



fn read_serial_port() {
    let port_name = "/dev/serial0";
    let mut port = serial::open(port_name).unwrap();
    interact(&mut port).unwrap();


}

fn interact<T: SerialPort>(port: &mut T) -> io::Result<()> {
    (port.reconfigure(&|settings| {
        (settings.set_baud_rate(serial::Baud9600));
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    }));

    port.set_timeout(Duration::from_millis(1000));

    let mut buf: Vec<u8> = (0..255).collect();

    port.write(&buf[..]);
    port.read(&mut buf[..]);
    println!("{:?}", buf);
    Ok(())

}

fn main() {
    read_serial_port()
}
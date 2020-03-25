extern crate serial;

use std::{env, io, time};
use serial::prelude::*;



fn read_serial_port() {
    let port_name = "/dev/serial0";
    let port = serial::open(port_name);
    serial::
    port.read()
}

fn interact<T: SerialPort>(port: &mut T) -> io::Result<()> {

    settings.set_baud_rate(serial::Baud9600);
    settings.set_char_size(serial::Bits8);
    settings.set_parity(serial::ParityNone);
    settings.set_stop_bits(serial::Stop1);
    settings.set_flow_control(serial::FlowNone);

    port.set_timeout(Duration::from_millis(1000));

    let mut buf: Vec<u8> = (0..255).collect();

    port.write(&buf[..]);
    port.read(&mut buf[..]);

}
use std::str;
use std::env;

use adafruit_gps::{Gps, open_port};
use std::io::Read;

fn main() {
    let args: Vec <String> = env::args().collect();
    let cmd = &args[1];

    let mut serial_buf= String::new();
    let mut gps = Gps { port: open_port("/dev/serial0") };
    let bytes_to_read = gps.port.bytes_to_read();
    println!("{:?}", bytes_to_read);
    let r = gps.port.read_to_string(&mut serial_buf).unwrap();
    println!("{:?}", r);



    // // Turn on the basic GGA and RMC info (what you typically want)
    // gps.send_command(cmd);
    // let line:Vec <u8> = gps.read_line();
    // let line: Vec <&str> = str::from_utf8(&line).unwrap().split("\n").collect();
    // println!("{:?}", line);

}

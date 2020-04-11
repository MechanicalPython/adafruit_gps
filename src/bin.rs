use std::thread;
use std::str;
use std::env;

use adafruit_gps::{Gps, open_port};

fn main() {
    let args: Vec <String> = env::args().collect();
    let cmd = &args[1];

    let mut gps = Gps { port: open_port("/dev/serial0") };

    // Turn on the basic GGA and RMC info (what you typically want)
    gps.send_command(cmd);
    let line:Vec <u8> = gps.read_line();
    let line: Vec <&str> = str::from_utf8(&line).unwrap().split("\n").collect();
    println!("{:?}", line);

}

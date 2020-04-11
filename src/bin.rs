use std::str;
use std::thread;
use std::time::Duration;

use adafruit_gps::{Gps, open_port};

fn main() {

    // What cannot fit into the buffer is not read. Reads from the top down. Least recent to most recent.
    // Always read from the top down

    let mut gps = Gps { port: open_port("/dev/serial0") };


    // thread::sleep(Duration::from_secs(2));
    println!("{}", gps.port.bytes_to_read());
    gps.send_command("PMTK010,001");
    println!("Sent");
    for _i in 0..20 {

        let lines = gps.read_line();
        let string: Vec<&str> = str::from_utf8(&lines).unwrap().split("\n").collect();
        println!("{:?}", string);
    }


    // // Turn on the basic GGA and RMC info (what you typically want)
    // gps.send_command(cmd);
    // let line:Vec <u8> = gps.read_line();
    // let line: Vec <&str> = str::from_utf8(&line).unwrap().split("\n").collect();
    // println!("{:?}", line);
}

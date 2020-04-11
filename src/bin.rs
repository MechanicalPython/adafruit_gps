use std::io::Read;
use std::str;
// use std::env;
use std::thread;
use std::time::Duration;
use std::env;

use adafruit_gps::{Gps, open_port};

fn main() {
    let args: Vec <String> = env::args().collect();
    let cmd: &usize = &args[1].parse::<usize>().unwrap();

    // What cannot fit into the buffer is not read.
    let mut buffer: Vec<u8> = vec![0; *cmd];  // Reads what is in the buffer, be it nothing or max.
    let mut output: Vec<u8> = Vec::new();

    let mut gps = Gps { port: open_port("/dev/serial0") };
    thread::sleep(Duration::from_secs(1));
    gps.send_command("PMTK010,001");
    let bytes_to_read = gps.port.bytes_to_read();
    println!("{:?}", bytes_to_read);

    match gps.port.read(buffer.as_mut_slice()) {
        Ok(buffer_size) => {
            output.extend_from_slice(&buffer[..buffer_size]);
        }
        Err(_e) => (),
    }
    let string:Vec<&str> = str::from_utf8(&output).unwrap().split("\n").collect();
    println!("{:?}", string);

    let bytes_to_read = gps.port.bytes_to_read();
    println!("{:?}", bytes_to_read);


    // // Turn on the basic GGA and RMC info (what you typically want)
    // gps.send_command(cmd);
    // let line:Vec <u8> = gps.read_line();
    // let line: Vec <&str> = str::from_utf8(&line).unwrap().split("\n").collect();
    // println!("{:?}", line);
}

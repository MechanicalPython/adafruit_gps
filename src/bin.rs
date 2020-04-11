use std::env;
use std::io::Read;
use std::str;
// use std::env;
use std::thread;
use std::time::Duration;

use adafruit_gps::{Gps, open_port};

fn main() {
    let args: Vec<String> = env::args().collect();
    let cmd: &usize = &args[1].parse::<usize>().unwrap();

    // What cannot fit into the buffer is not read. Reads from the top down. Least recent to most recent.
    // Always read from the top down

    let mut gps = Gps { port: open_port("/dev/serial0") };
    gps.send_command("PMTK010,001");

    // thread::sleep(Duration::from_secs(10));

    for _i in 0..10 {
        let mut buffer: Vec<u8> = vec![0; *cmd];  // Reads what is in the buffer, be it nothing or max.
        let mut output: Vec<u8> = Vec::new();
        let bytes_to_read = gps.port.bytes_to_read();
        println!("{:?}", bytes_to_read);

        match gps.port.read(buffer.as_mut_slice()) {
            Ok(buffer_size) => {
                output.extend_from_slice(&buffer[..buffer_size]);
            }
            Err(_e) => (),
        }
        let string: Vec<&str> = str::from_utf8(&output).unwrap().split("\n").collect();
        println!("{:?}", string);
    }


    // // Turn on the basic GGA and RMC info (what you typically want)
    // gps.send_command(cmd);
    // let line:Vec <u8> = gps.read_line();
    // let line: Vec <&str> = str::from_utf8(&line).unwrap().split("\n").collect();
    // println!("{:?}", line);
}

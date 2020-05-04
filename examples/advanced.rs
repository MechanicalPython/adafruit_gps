extern crate adafruit_gps;

pub use adafruit_gps::gps::{Gps, open_port};
use adafruit_gps::gps::PortConnection;
use adafruit_gps::nmea;

fn main() {
    let port = open_port("/dev/serial0", 9600);

    // For advanced use, the satellite_data nd navigation_data flags are for gps.update()
    // so just put any values there.
    let mut gps = Gps { port };

    // Set what sentences you want to be outputted
    gps.pmtk_314_api_set_nmea_output(0, 0, 0, 0, 0, 1, 1);

    // Here you can read your own line and parse it how you like.
    // Note, that once a line is read it is gone. If you don't parse it and then drop the variable
    // it will be gone forever.
    loop {
        // Read the line
        let line = gps.read_line();

        match line {
            PortConnection::Valid(output) => {
                // Convert the String to a Vec<&str>: [$HEADER], [arg 1], etc.
                let line: Vec<&str> = nmea::nmea::parse_sentence(output.as_str()).unwrap();

                // Parse the Vec<&str> to parse_gsa and return the GsaData struct.

                if &line[0][0..3] != "$GSA" {
                    println!("Not a gsa line")
                } else {
                    // This line will panic if the sentence isn't a valid GSA.
                    let _gsa = nmea::gsa::parse_gsa(line);
                }
            },
            PortConnection::InvalidBytes => println!("Invalid bytes"),
            PortConnection::NoConnection => println!("No connection to gps"),
        }
    }
}
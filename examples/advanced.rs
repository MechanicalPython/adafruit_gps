

use adafruit_gps::gps::{Gps, open_port, PortConnection};
use adafruit_gps::parse_nmea::parse_sentence;
use adafruit_gps::{gsa};
use adafruit_gps::send_pmtk::NmeaOutput;

fn main() {
    let port = open_port("/dev/serial0", 9600);

    // For advanced use, the satellite_data nd navigation_data flags are for gps.update()
    // so just put any values there.
    let mut gps = Gps { port };

    // Set what sentences you want to be outputted
    gps.pmtk_314_api_set_nmea_output(NmeaOutput{gga: 1, gsa: 1, gsv: 0,  gll: 0, rmc: 0, vtg: 0, pmtkchn_interval: 0 });

    // Here you can read your own line and parse it how you like.
    // Note, that once a line is read it is gone. If you don't parse it and then drop the variable
    // it will be gone forever.
    loop {
        // Read the line
        let line = gps.read_line();

        match line {
            PortConnection::Valid(output) => {
                // Convert the String to a Vec<&str>: [$HEADER], [arg 1], etc.
                let line: Vec<&str> = parse_sentence(output.as_str()).unwrap();

                // Parse the Vec<&str> to parse_gsa and return the GsaData struct.

                if &line[0][0..3] != "$GSA" {
                    println!("Not a gsa line")
                } else {
                    // This line will panic if the sentence isn't a valid GSA.
                    let _gsa = gsa::parse_gsa(line);
                }
            },
            PortConnection::InvalidBytes(_) => println!("Invalid bytes"),
            PortConnection::NoConnection => println!("No connection to gps"),
        }
    }
}
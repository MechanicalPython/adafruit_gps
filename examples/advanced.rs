extern crate adafruit_gps;

pub use adafruit_gps::gps::{GetGpsData, Gps, open_port};
use adafruit_gps::PMTK::send_pmtk::SendPmtk;
use adafruit_gps::nmea;

fn main() {
    let port = open_port("/dev/serial0");
    let mut gps = Gps {port};

    gps.pmtk_314_api_set_nmea_output(0,0,0,0,0,1,1);
    // Recommended gps update rate 1000miliseconds, or 1Hz.
    gps.pmtk_220_set_nmea_updaterate("500");

    // Here you can read your own line and parse it how you like.
    // Note, that once a line is read it is gone. If you don't parse it and then drop the variable
    // it will be gone forever.
    loop {
        // Read the line
        let line = gps.read_line();
        // Convert the String to a Vec<&str>
        let line = nmea::nmea::parse_sentence(line.as_str()).unwrap();
        // Parse the Vec<&str> to parse_gsa and return the GsaData struct.
        let _gsa = nmea::gsa::parse_gsa(line);

    }
}
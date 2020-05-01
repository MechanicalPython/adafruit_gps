//! # Adafruit_gps
//! This is a port from the adafruit python code that reads the output from their GPS systems.
//! ## Links
//! Python code: https://github.com/adafruit/Adafruit_CircuitPython_GPS
//!
//! GPS module docs: https://learn.adafruit.com/adafruit-ultimate-gps/
//!
//! PMTK commands https://cdn-shop.adafruit.com/datasheets/PMTK_A11.pdf
//!
//! ## Modules
//! The PMTK module is a way of easily sending command to the GPS, changing it's settings.
//!
//! The nmea module reads the data given by the GPS. Use the gps.update() trait to get easy to use
//! data, but for specific use cases custom commands can be read.
//!
//! ## Hardware specs
//! Please read the docs for the specific GPS module you are using.
//! Update rate: 1Hz or 10Hz outputs.
//!
//! # Gps struct
//! port is the open_port option
//! satellite_data -> the individual satellite data from GSA and GSV
//! navigation_data -> the naviagtion data from GGA and VTG
//!
//! # Module Outputs
//! gps.update() gives the following outputs in the GpsData struct
//!
//! - UTC - The UTC time as a f64
//! - Latitude - As degrees
//! - Longitude - As degrees
//! - Altitude - Altitude above Mean Sea Level in metres.
//! - True Course - Measured heading, degrees
//! - Magnetic Course - Measured heading by magnatic north, degrees
//! - Speed (knots)
//! - Speed (kph)
//! - Geoidal Separation - Difference between WGS-84 earth ellipsoid and mean sea level, basically altitude.
//! - Age Diff Corr - Age in seconds since last update from reference station.
//! - PDOP - Position DOP
//! - HDOP - Horizontal DOP
//! - VDOP - Vertical DOP
//! - Satellites - As a Vec<Satellites>
//!     - ID - Satellite id number, 1-32 and 193-195 for QZSS.
//!     - Elevation - Elevation of the satellite in degrees
//!     - Azimuth - The degrees from north the satellite is, if it was on the ground.
//!     - SNR - Signal to Noise ratio: Signal / Noise , 0-99, null if not tracking.
//!
//! Note: DOP is dilution of precision, a measure of error based on the position of the satellites.
//!
//!
//! More info on the GPS module at
//!

extern crate serialport;

#[allow(non_snake_case)]
pub mod PMTK;
pub mod nmea;

pub mod gps {
    //! This is the main module around which all other modules interact.
    //! It contains the Gps structure, open port and GpsData that are central to using this module.
    use std::io::Read;
    use std::str;
    use std::time::Duration;

    use serialport::prelude::*;

    use crate::nmea;
    use crate::nmea::gsv::Satellites;
    use crate::PMTK::send_pmtk::{SendPmtk, Pmtk001Ack};
    use std::collections::HashMap;

    /// Opens the port to the GPS, probably /dev/serial0
        /// Default baudrate is 9600
    pub fn open_port(port_name: &str, baud_rate: u32) -> Box<dyn SerialPort> {
        let settings = SerialPortSettings {
            baud_rate,
            data_bits: DataBits::Eight,
            flow_control: FlowControl::None,
            parity: Parity::None,
            stop_bits: StopBits::One,
            timeout: Duration::from_millis(1000),
        };
        match serialport::open_with_settings(port_name, &settings) {
            Ok(port) => return port,
            Err(_e) => panic!("Port not found: {} - {}", port_name, _e),
        }
    }

    /// Checks if a sentence is a valid sentence by checksumming the sentence and comparing it to
    /// the given checksum. Returns true for valid sentence, false for invalid.
    /// The format of the sentence should be $sentence*checksum
    pub fn is_valid_checksum(s: &str) -> bool {
        let s = s.trim();
        // String should be: $..., *XY

        let star = &s[s.len() - 3..s.len() - 2];
        let checksum = &s[s.len() - 2..s.len()];
        let body = &s[0..s.len() - 3];

        if star != "*" {  // Check third last item is a *
            return false;
        }

        match u8::from_str_radix(&checksum, 16) {  // Convert to base 16.
            Ok(expected_checksum) => {
                let mut actual: u8 = 0;
                for i in body[1..].as_bytes() {  // Skip $ sign. bitwise xor for each i in body
                    actual ^= *i;
                }
                if actual == expected_checksum {
                    return true;
                } else {
                    return false;
                }
            }
            Err(_e) => return false,
        }
    }

    #[derive(Debug)]
    #[derive(Default)]
    /// GpsData is the easy to use, out of the box data set that the update trait will give you.
    pub struct GpsData {
        pub utc: f64,
        pub latitude: Option<f32>,
        pub longitude: Option<f32>,
        pub altitude: Option<f32>,
        pub true_course: Option<f32>,
        pub mag_course: Option<f32>,
        pub speed_knots: Option<f32>,
        pub speed_kph: Option<f32>,
        pub geoidal_spe: Option<f32>,
        pub age_diff_corr: Option<f32>,
        pub sats_used: i32,
        pub pdop: Option<f32>,
        pub hdop: Option<f32>,
        pub vdop: Option<f32>,
        pub fix_quality: nmea::gga::SatFix,
        pub satellites: Vec<Satellites>,
    }

    /// This is the main struct around which all commands are centered. It allows for communication
    /// with the GPS module via the open port.
    ///
    /// Satellite data: true if you want the individual satellite data
    /// Navigation data: true if you want the naviagion data (lat, long, etc)
    pub struct Gps {
        pub port: Box<dyn SerialPort>,
        pub satellite_data: bool,
        pub naviagtion_data: bool,
    }

    /// This trait contains the two most important commands: update and read_line.
    pub trait GetGpsData {
        /// Init the gps settings with the correct settings based on a few things
        fn init(&mut self, update_rate: &str) -> HashMap<String, Pmtk001Ack>;
        /// Returns the GpsData struct
        fn update(&mut self) -> GpsData;
        /// Reads a whole sentence given by the serial buffer
        fn read_line(&mut self) -> String;
    }

    impl GetGpsData for Gps {
        /// Return hashmap values:
        /// Update rate: pmtk001 enum
        /// Return type: pmtk001 enum
        fn init(&mut self, update_rate: &str) -> HashMap<String, Pmtk001Ack> {
            let (vtg, gga) = if self.naviagtion_data {
                (1, 1)
            } else {
                (0, 0)
            };

            let (gsa, gsv) = if self.satellite_data {
                (1, 1)
            } else {
                (0, 0)
            };

            let mut hash = HashMap::new();
            let return_types = self.pmtk_314_api_set_nmea_output(0, 0, vtg, gga, gsa, gsv, 1);
            hash.insert(String::from("Return types"), return_types);

            let update_rate = self.pmtk_220_set_nmea_updaterate(update_rate);
            hash.insert(String::from("Update rate"), update_rate);

            return hash
        }

        /// Keeps reading sentences until all the required sentences are read.
        ///
        /// Returns GpsData.
        fn update(&mut self) -> GpsData {
            let mut gga = self.naviagtion_data;
            let mut vtg = self.naviagtion_data;
            let mut gsa = self.satellite_data;
            let mut gsv = self.satellite_data;

            let mut values = GpsData::default();
            while (gga == true) || (vtg == true) || (gsa == true) || (gsv == true) {
                let line = self.read_line();
                let sentence = nmea::nmea::parse_sentence(line.as_str());
                if sentence.is_some() {
                    let sentence = sentence.unwrap();
                    if &sentence.get(0).unwrap()[3..5] == "GG" {
                        let gga_values = nmea::gga::parse_gga(sentence);
                        values.utc = gga_values.utc;
                        values.latitude = gga_values.lat;
                        values.longitude = gga_values.long;
                        values.sats_used = gga_values.satellites_used;
                        values.altitude = gga_values.msl_alt;
                        values.geoidal_spe = gga_values.geoidal_sep;
                        values.age_diff_corr = gga_values.age_diff_corr;
                        values.fix_quality = gga_values.sat_fix;
                        gga = false;
                    } else if &sentence.get(0).unwrap()[3..6] == "VTG" {
                        let vtg_values = nmea::vtg::parse_vtg(sentence);
                        values.true_course = vtg_values.true_course;
                        values.mag_course = vtg_values.magnetic_course;
                        values.speed_knots = vtg_values.speed_knots;
                        values.speed_kph = vtg_values.speed_kph;
                        vtg = false;
                    } else if &sentence.get(0).unwrap()[3..6] == "GSA" {
                        let gsa_values = nmea::gsa::parse_gsa(sentence);
                        values.hdop = gsa_values.hdop;
                        values.vdop = gsa_values.vdop;
                        values.pdop = gsa_values.pdop;
                        gsa = false;
                    } else if &sentence.get(0).unwrap()[3..6] == "GSV" {
                        let number_of_messages: i32 = sentence.get(1).unwrap().parse().unwrap();

                        let v = if number_of_messages == 1 {
                            nmea::gsv::parse_gsv(sentence)
                        } else {
                            let mut gsv_values: Vec<Satellites> = nmea::gsv::parse_gsv(sentence);  // First sentence
                            for _message in 1..number_of_messages {  // Read lines and add it for each message.
                                let line = self.read_line();
                                let sentence = nmea::nmea::parse_sentence(line.as_str());
                                let sentence = sentence.unwrap();
                                gsv_values.append(nmea::gsv::parse_gsv(sentence).as_mut())
                            }
                            gsv_values
                        };
                        values.satellites = v;
                        gsv = false;
                    }
                } else {
                    println!("Invalid byte string returned");
                }
            }
            values
        }
        /// Reads a full sentence from the serial buffer, returns a String.
        fn read_line(&mut self) -> String {
            // Maximum port buffer size is 4095.
            // Returns whatever is in the port.
            // Start of a line is $ (36) and end is \n (10).
            // The serial buffer reads from bottom to top. New data is added to the top. The amount read
            // from the serial buffer is the size of the buffer vec.

            // 127 is the maximum valid utf8 number.
            let mut buffer: Vec<u8> = vec![0; 1];  // Reads what is in the buffer, be it nothing or max.
            let mut output: Vec<u8> = Vec::new();
            let p = &mut self.port;
            let mut cont = true;
            while cont {
                match p.read(buffer.as_mut_slice()) {
                    Ok(buffer_size) => {
                        output.extend_from_slice(&buffer[..buffer_size]);

                        if output.get(output.len() - 1).unwrap() == &10u8 || output.len() > 255 {
                            cont = false;
                        }
                    }
                    Err(_e) => (),
                }
            }
            // Panics if there is a byte number that is too high.
            let string: String = str::from_utf8(&output).unwrap_or("Invalid bytes given").to_string();
            return string;
        }
    }
}


#[cfg(test)]
mod gps_test {
    use super::gps;

    #[test]
    fn is_valid_sentence() {
        assert_eq!(gps::is_valid_checksum("$PMTK220,100*2F"), true);
        assert_eq!(gps::is_valid_checksum("$GPGSV,4,3,14,12,12,100,,04,11,331,,16,06,282,,05,05,074,22*75"), true);
        assert_eq!(gps::is_valid_checksum("$GPGSV,4,4,14,32,01,215,,41,,,*4F"), true);
        assert_eq!(gps::is_valid_checksum("$GNGGA,131613.000,5132.7314,N,00005.9099,W,1,9,1.17,42.4,M,47.0,M,,*60\r\n"), true);
        assert_eq!(gps::is_valid_checksum("$GPGSA,A,3,29,02,26,25,31,14,,,,,,,1.42,1.17,0.80*07\r\n"), true);
        assert_eq!(gps::is_valid_checksum("$GPGSA,A,3,29,02,26,25,31,14,,,,,,,1.42,1.17,0.80*A7\r\n"), false);
    }
}


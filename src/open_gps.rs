/// # Gps
/// This module controls the opening and reading of the port to the gps.
///


pub mod gps {
    //! This is the main module around which all other modules interact.
    //! It contains the Gps structure, open port and GpsData that are central to using this module.
    use std::io::Read;
    use std::str;
    use std::time::{Duration, SystemTime};

    use serde::{Serialize, Deserialize};
    use serialport::prelude::*;

    use crate::nmea::gga::{GgaData, parse_gga};
    use crate::nmea::gll::{GllData, parse_gll};
    use crate::nmea::gsa::{GsaData, parse_gsa};
    use crate::nmea::gsv::{parse_gsv, Satellites};
    use crate::nmea::parse_nmea::parse_sentence;
    use crate::nmea::rmc::{parse_rmc, RmcData};
    use crate::nmea::vtg::{parse_vtg, VtgData};

    /// Opens the port to the GPS, probably /dev/serial0
    /// Default baud rate is 9600
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

        if star != "*" {
            // Check third last item is a *
            return false;
        }

        match u8::from_str_radix(&checksum, 16) {
            // Convert to base 16.
            Ok(expected_checksum) => {
                let mut actual: u8 = 0;
                for i in body[1..].as_bytes() {
                    // Skip $ sign. bitwise xor for each i in body
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

    /// Enum for if the port connection to the gps is valid, gave invalid bytes, or is not connected
    #[derive(PartialEq, Debug)]
    pub enum PortConnection {
        Valid(String),
        InvalidBytes(Vec<u8>),
        NoConnection,
    }

    /// Enum for the gps.update() method.
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    pub enum GpsSentence {
        GGA(GgaData),
        VTG(VtgData),
        GSA(GsaData),
        GSV(Vec<Satellites>),
        GLL(GllData),
        RMC(RmcData),
        NoConnection,
        InvalidBytes,
        InvalidSentence,
    }

    /// This is the main struct around which all commands are centered. It allows for communication
    /// with the GPS module via the open port.
    ///
    /// Satellite data: true if you want the individual satellite data
    /// Navigation data: true if you want the navigation data (lat, long, etc)
    pub struct Gps {
        pub port: Box<dyn SerialPort>,
    }

    impl Gps {
        pub fn new(port: &str, baud_rate: &str) -> Gps {
            Gps{port: open_port(port, baud_rate.parse().unwrap())}
        }

        /// Reads a full sentence from the serial buffer, returns a String.
        /// "Invalid bytes given" when there are no bytes given.
        pub fn read_line(&mut self) -> PortConnection {
            // Maximum port buffer size is 4095.
            // Returns whatever is in the port.
            // Start of a line is $ (36) and end is \n (10).
            // The serial buffer reads from bottom to top. New data is added to the top. The amount read
            // from the serial buffer is the size of the buffer vec.

            // 127 is the maximum valid utf8 number.
            let mut buffer: Vec<u8> = vec![0; 1]; // Reads what is in the buffer, be it nothing or max.
            let mut output: Vec<u8> = Vec::new();
            let p = &mut self.port;
            let mut cont = true;
            let start = SystemTime::now();
            while cont {
                // If there is no connection, this match statement is looped over with a 1 second time out
                // as given by the port open.
                if start.elapsed().unwrap() > Duration::from_secs(1) {
                    return PortConnection::NoConnection;
                }
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
            let string = str::from_utf8(&output);
            return if string.is_ok() {
                PortConnection::Valid(string.unwrap().to_string())
            } else {
                PortConnection::InvalidBytes(output)
            };
        }

        /// Keeps reading sentences until all the required sentences are read.
        /// Returns GpsData.
        pub fn update(&mut self) -> GpsSentence {
            let port_output = self.read_line();

            return match port_output {
                PortConnection::NoConnection => GpsSentence::NoConnection,
                PortConnection::InvalidBytes(_vector) => GpsSentence::InvalidBytes,
                PortConnection::Valid(string) => {
                    let sentence: Option<Vec<&str>> = parse_sentence(string.as_str());
                    if sentence.is_some() {
                        let sentence = sentence.unwrap();
                        let header = sentence.get(0).unwrap();
                        // At this point sentences needs to be is_valid str.
                        if &header[3..5] == "GG" {
                            return GpsSentence::GGA(parse_gga(sentence));
                        } else if &header[3..6] == "VTG" {
                            return GpsSentence::VTG(parse_vtg(sentence));
                        } else if &header[3..6] == "GSA" {
                            return GpsSentence::GSA(parse_gsa(sentence));
                        } else if &header[3..6] == "GLL" {
                            return GpsSentence::GLL(parse_gll(sentence));
                        } else if &header[3..6] == "RMC" {
                            return GpsSentence::RMC(parse_rmc(sentence));
                        } else if &header[3..6] == "GSV" {
                            // Assumes that each GSV sentence if given in exact sequence, and not out of order.
                            let number_of_messages: i32 = sentence.get(1).unwrap().parse().unwrap();

                            let mut gsv_values: Vec<Satellites> = parse_gsv(sentence); // First sentence
                            for _message in 1..number_of_messages { // If number of messages is 1, this is all skipped.
                                // Read lines and add it for each message.
                                let line = self.read_line();
                                match line {
                                    PortConnection::Valid(line) => {
                                        let sentence = parse_sentence(line.as_str());
                                        let sentence = sentence.unwrap();
                                        gsv_values.append(parse_gsv(sentence).as_mut())
                                    }
                                    _ => ()
                                };
                            }
                            return GpsSentence::GSV(gsv_values);
                        }
                    }
                    GpsSentence::InvalidSentence
                }
            };
        }

    }
}

#[cfg(test)]
mod gps_test {
    use super::gps;

    #[test]
    fn is_valid_sentence() {
        assert_eq!(gps::is_valid_checksum("$PMTK220,100*2F"), true);
        assert_eq!(
            gps::is_valid_checksum(
                "$GPGSV,4,3,14,12,12,100,,04,11,331,,16,06,282,,05,05,074,22*75"
            ),
            true
        );
        assert_eq!(
            gps::is_valid_checksum("$GPGSV,4,4,14,32,01,215,,41,,,*4F"),
            true
        );
        assert_eq!(
            gps::is_valid_checksum(
                "$GNGGA,131613.000,5132.7314,N,00005.9099,W,1,9,1.17,42.4,M,47.0,M,,*60\r\n"
            ),
            true
        );
        assert_eq!(
            gps::is_valid_checksum("$GPGSA,A,3,29,02,26,25,31,14,,,,,,,1.42,1.17,0.80*07\r\n"),
            true
        );
        assert_eq!(
            gps::is_valid_checksum("$GPGSA,A,3,29,02,26,25,31,14,,,,,,,1.42,1.17,0.80*A7\r\n"),
            false
        );
    }
}
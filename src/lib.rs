//! # Adafruit_gps
//! This is a port from the adafruit python code that reads the output from their GPS systems.
//! ## Links
//! Python code: https://github.com/adafruit/Adafruit_CircuitPython_GPS
//! GPS module docs: https://learn.adafruit.com/adafruit-ultimate-gps/
//!
//! ## Hardware specs
//! Please read the docs for the specific GPS module you are using.
//! Update rate: 1Hz or 10Hz outputs.
//!
//! ## Outputs
//! Outputs are split between two types: position data and satellite data.
//! UTC - The UTC time as a f64
//! Latitude - As degrees
//! Longitude - As degrees
//! Altitude - Altitude above Mean Sea Level in metres.
//! True Course - Measured heading, degrees
//! Magnetic Course - Measured heading by magnatic north, degrees
//! Speed (knots)
//! Speed (kph)
//! Geoidal Separation - Difference between WGS-84 earth ellipsoid and mean sea level, basically altitude.
//! Age Diff Corr - Age in seconds since last update from reference station.
//! PDOP - Position DOP
//! HDOP - Horizontal DOP
//! VDOP - Vertical DOP
//! Satellites - As a Vec<Satellites>
//!     ID - Satellite id number, 1-32 and 193-195 for QZSS.
//!     Elevation - Elevation of the satellite in degrees
//!     Azimuth - The degrees from north the satellite is, if it was on the ground.
//!     SNR - Signal to Noise ratio: Signal / Noise , 0-99, null if not tracking.
//!
//! Note: DOP is dilution of precision, a measure of error based on the position of the satellites.
//!
//!
//! More info on the GPS module at https://cdn-shop.adafruit.com/datasheets/PMTK_A11.pdf
//!
//! QZSS satellites are 4 japanese satellites used in Asia-Oceania regions.
// Would be cool to support all the types of GPS chips and what strings they give.
// So rather than just update what is given, look for the strings that you want and give that.

extern crate serialport;

pub use crate::nmea::nmea as Nmea;
pub use crate::PMTK::send_pmtk;

#[allow(non_snake_case)]
pub mod PMTK;
pub mod nmea;

pub mod gps {
    use std::io::Read;
    use std::str;
    use std::time::Duration;

    use serialport::prelude::*;

    use crate::nmea;

    pub fn open_port(port_name: &str) -> Box<dyn SerialPort> {
        let settings = SerialPortSettings {
            baud_rate: 9600,
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
    pub struct Satellites {
        pub id: Option<i32>,
        pub elevation: Option<f32>,
        pub azimuth: Option<f32>,
        pub snr: Option<f32>,
    }

    #[derive(Debug)]
    #[derive(Default)]
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
        pub satellites: Vec<Satellites>,
    }

    pub struct Gps {
        pub port: Box<dyn SerialPort>,
    }

    pub trait GetGpsData {
        fn update(&mut self) -> GpsData;
        fn read_line(&mut self) -> String;
    }

    impl GetGpsData for Gps {
        fn update(&mut self) -> GpsData {
            // Read a certain satellites data.
            // So I can read a sentence, parse that sentence and get data out of it.
            // So a function could return some data.
            // But how to pass that data to the user?
            // Order of lines is GGA, GLL, GSA, GSV, RMC, VTG.

            // The data bools are a bit confusing as gps.update(true, true) means you want
            // that data, so while true, find the data.
            let mut gga = true;
            let mut vtg = true;
            let mut gsa = true;
            let mut gsv = true;

            let mut values = GpsData::default();

            while gga && vtg && gsa && gsv {
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
                            for _message in 1..number_of_messages + 1 {  // Read lines and add it for each message.
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
                }
            }
            values
        }

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

                        if output.get(output.len() - 1).unwrap() == &10u8 {
                            cont = false;
                        }
                    }
                    Err(_e) => (),
                }
            }
            let string: String = str::from_utf8(&output).expect("No line read").to_string();
            return string;
        }
    }
}


// #[cfg(test)]
// mod gps_test {
//     use super::gps;
//
//     use std::str;
//
//     #[test]
//     fn _parse_hhmmss() {
//         assert_eq!(gps::_format_hhmmss("205530").as_str(), "20:55:30");
//     }
//
//     #[test]
//     fn _parse_ddmmyy() {
//         assert_eq!(gps::_format_ddmmyy("300320"), "2020-03-30".to_string());
//     }
//
//     #[test]
//     fn _parse_degrees() {
//         assert_eq!(gps::_parse_degrees("3218.0489".to_string()).unwrap(), 32.300815);
//         assert_eq!(gps::_parse_degrees("6447.5086".to_string()).unwrap(), 64.79181);
//     }
//
//     // fn spoof_update(test_reading: Vec<u8>) -> gps::GpsArgValues {
//     //     let port_reading = test_reading;
//     //
//     //     let string: Vec<&str> = str::from_utf8(&port_reading).unwrap().split("\n").collect();
//     //     for sentence in string {
//     //         match gps::parse_sentence(sentence) {
//     //             Some((data_type, args)) => {
//     //                 println!("{:?}", sentence);
//     //                 return if (data_type == "GPGLL".to_string()) | (data_type == "GNGGL".to_string()) {
//     //                     let values = gps::parse_gpgll(args);
//     //                     values
//     //                 } else if (data_type == "GPRMC".to_string()) | (data_type == "GNRMC".to_string()) {
//     //                     let values = gps::parse_gprmc(args);
//     //                     values
//     //                 } else if (data_type == "GPGGA".to_string()) | (data_type == "GNGGA".to_string()) {
//     //                     let values = gps::parse_gpgga(args);
//     //                     values
//     //                 } else {  // If all else fails, return default values.
//     //                     gps::GpsArgValues::default()
//     //                 };
//     //             }
//     //             None => (),
//     //         }
//     //     }
//     //     return gps::GpsArgValues::default();
//     // }
//
//     fn _parse_gpgll() {}
//
//     #[cfg(test)]
//     mod rmc_tests {
//         use super::*;
//
//         fn test_rmc_string(s: &str) -> (Option<String>, Option<i32>, Option<f32>, Option<f32>, Option<f32>, Option<f32>) {
//             let s = s.as_bytes().to_vec();  // Process str to what read_lines gives.
//             let r = spoof_update(s);
//             return (r.timestamp, r.fix_quality, r.latitude, r.longitude, r.speed_knots, r.track_angle_deg);
//         }
//
//         #[test]
//         fn test_parse_gprmc_1() {
//             let s = "$GNRMC,110942.000,A,5132.7394,N,00005.9165,W,0.36,193.42,020420,,,A*63\r\n";
//             assert_eq!(test_rmc_string(&s), (Some("2020-04-02 11:09:42".to_string()), Some(1),
//                                              Some(51.54566), Some(-0.098608), Some(0.36), Some(193.42)));
//         }
//     }
//
//
//     // GGA tests
//     #[cfg(test)]
//     mod gga_tests {
//         use super::*;
//
//         fn test_gpgga_string(s: &str) -> (Option<String>, Option<f32>, Option<f32>, Option<i32>,
//                                           Option<i32>, Option<f32>, Option<f32>, Option<f32>) {
//             let s = s.as_bytes().to_vec();  // Process str to what read_lines gives.
//             let r = spoof_update(s);
//             return (r.timestamp, r.latitude, r.longitude, r.fix_quality, r.satellites, r.horizontal_dilution, r.altitude_m, r.height_geoid);
//         }
//
//         #[test]
//         fn test_parse_gpgga_1() {
//             let s1 = "$GNGGA,110942.000,5132.7394,N,00005.9165,W,1,8,1.38,50.9,M,47.0,M,,*60\r\n";
//             assert_eq!(test_gpgga_string(&s1),
//                        (Some("11:09:42".to_string()), Some(51.54566), Some(-0.098608),
//                         Some(1), Some(8), Some(1.38), Some(50.9), Some(47.0)));
//         }
//
//         #[test]
//         fn test_parse_gpgga_2() {
//             let s2 = "$GNGGA,131714.000,5132.7319,N,00005.9117,W,1,12,0.85,35.9,M,47.0,M,,*51\r\n";
//             assert_eq!(test_gpgga_string(&s2),
//                        (Some("13:17:14".to_string()), Some(51.545532), Some(-0.098528),
//                         Some(1), Some(12), Some(0.85), Some(35.9), Some(47.0)));
//         }
//
//         #[test]
//         fn test_parse_gpgga_3() {
//             let s3 = "$HFJHS,,,,,,";
//             assert_eq!(test_gpgga_string(&s3), (None, None, None, None, None, None, None, None));
//         }
//
//         #[test]
//         fn test_parse_gpgga_4() {
//             let s4 = "";
//             assert_eq!(test_gpgga_string(&s4), (None, None, None, None, None, None, None, None));
//         }
//
//         #[test]
//         fn test_parse_gpgga_5() {
//             let s5 = "$GNGGA,000400.100,,,,,0,0,,,M,,M,,*53\r";
//             assert_eq!(test_gpgga_string(&s5), (Some("00:04:00".to_string()), None, None, Some(0), Some(0), None, None, None));
//         }
//     }
// }


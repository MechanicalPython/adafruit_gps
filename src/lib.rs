//! # Adafruit_gps
//! This is a port from the adafruit python code that reads the output from their GPS systems.
//! ## Links
//! Python code: https://github.com/adafruit/Adafruit_CircuitPython_GPS
//! GPS module docs: https://learn.adafruit.com/adafruit-ultimate-gps/
//!
//! According the the GPS specs, it can give 1Hz or 10Hz outputs.
//!
//! # Examples
//! This example will open the port and print
//! ```
//! use std::thread::sleep;
//! use std::time::Duration;
//! use mylib::{Gps, open_port};
//! let mut gps = Gps{port: open_port("/dev/serial0")};
//!
//! loop {
//!     &gps.update();
//!     sleep(Duration::from_secs(1));
//! }
//! ```
//!
//! GPS enum has all the items that are needed.
//! The way it works. Constantly call gps.update(). This will update the variables with the
//! most up to date items (each type of prefix indicates a different level of importance)
//! And then every second print the most up to date info.

extern crate serialport;

use std::io::{Read, Write};
use std::str;
use std::time::{Duration};

use serialport::prelude::*;

pub fn open_port(port_name: &str) -> Box<dyn SerialPort> {
    let settings = SerialPortSettings {
        baud_rate: 9600,
        data_bits: DataBits::Eight,
        flow_control: FlowControl::None,
        parity: Parity::None,
        stop_bits: StopBits::One,
        timeout: Duration::from_millis(1000),
    };
    match serialport::open_with_settings(&port_name, &settings) {
        Ok(port) => return port,
        Err(_e) => panic!("Port not found: {} - {}", port_name, _e),
    }
}

#[derive(Debug)]
#[derive(Default)]
pub struct GpsArgValues {
    pub timestamp: Option<String>,
    pub latitude: Option<f32>,
    pub longitude: Option<f32>,
    pub fix_quality: Option<i32>,
    // if A, fix quality is 1.
    pub fix_quality_3d: Option<i32>,
    pub satellites: Option<i32>,
    pub horizontal_dilution: Option<f32>,
    pub altitude_m: Option<f32>,
    pub height_geoid: Option<f32>,
    pub speed_knots: Option<f32>,
    pub track_angle_deg: Option<f32>,
    pub sats: Option<i32>,
    pub isactivedata: Option<String>,
    pub sat_prns: Option<i32>,
    pub sel_mode: Option<i32>,
    // Selection mode. data[0] for parse gpgsa.
    pub pdop: Option<f32>,
    // PODP, dilution of precision
    pub hdop: Option<f32>,
    // HDOP, hosizontal of precision
    pub vdop: Option<f32>,
    // vertical dilution of precision
    pub total_mess_num: Option<i32>,
    // total number of messages. _parse_gpgsv
    pub mess_num: Option<i32>, // message number. _parse_gpgsv
    pub has_fix: Option<i8>, // 0 is no fix, 1 is fix.
}

pub struct Gps {
    pub port: Box<dyn SerialPort>,
}

impl Gps {
    // todo - regex to ensure that the strings passed to each _parse_xyz is correct.

    fn read_line(&mut self) -> Vec<u8> {
        // Maximum port buffer size is 4095.
        // Returns whatever is in the port.
        // Start of a line is $ (36) and end is \n (10). So if
        // The correct line length is 70 (probably).
        let mut buffer: Vec<u8> = vec![0; 4095];  // Reads what is in the buffer, be it nothing or max.
        let mut output: Vec<u8> = Vec::new();
        let p = &mut self.port;
        let mut cont = true;
        while cont {
            match p.read(buffer.as_mut_slice()) {
                Ok(buffer_size) => {
                    output.extend_from_slice(&buffer[..buffer_size]);
                    while output.get(0).unwrap() != &36u8 {  // Remove all characters before $
                        output.remove(0);
                    }
                    if buffer[..buffer_size].contains(&10u8) {
                        cont = false;
                        while output.get(output.len() - 1).unwrap() != &10u8 {
                            output.remove(output.len() - 1);
                        }
                    }
                }
                Err(_e) => (),
            }
        }
        return output;
    }

    #[allow(unused_must_use)]  // self.port.write is not used at the end.
    pub fn send_command(&mut self, cmd: &str) {
        // Sends byte commands to the gps.
        // Auto add the leading $ and the trailing *
        let mut checksum = 0;
        for char in cmd.as_bytes() {
            checksum ^= *char;
        }
        let checksum = format!("{:X}", checksum);
        let byte_cmd = format!("${}*{}\r\n", cmd, checksum).as_str().to_ascii_uppercase();
        let byte_cmd = byte_cmd.as_bytes();
        self.port.write(byte_cmd);
    }

    pub fn update(&mut self) -> GpsArgValues {
        let port_reading = self.read_line();

        let string: Vec<&str> = str::from_utf8(&port_reading).unwrap().split("\n").collect();
        for sentence in string {
            match Gps::parse_sentence(sentence) {
                Some((data_type, args)) => {
                    println!("{:?} {:?}", data_type, args);
                    return if (data_type == "GPGLL".to_string()) | (data_type == "GNGGL".to_string()) {
                        let values = Gps::_parse_gpgll(args);
                        values
                    } else if (data_type == "GPRMC".to_string()) |  (data_type == "GNRMC".to_string()) {
                        let values = Gps::_parse_gprmc(args);
                        values
                    } else if (data_type == "GPGGA".to_string()) |  (data_type == "GNGGA".to_string()) {
                        let values = Gps::_parse_gpgga(args);
                        values
                    } else {  // If all else fails, return default values.
                        GpsArgValues::default()
                    }
                }
                None => (),
            }
        }
        return GpsArgValues::default();
    }

    fn parse_sentence(sentence: &str) -> Option<(String, String)> {
        // Split sentence into data type (what kind of data there is) and args (the actual data)
        let sentence: String = sentence.split_whitespace().collect();
        let sentence: &str = sentence.chars().as_str();

        if Gps::checksum(sentence) == false {
            return None;
        }
        let sentence: &str = &sentence[0..sentence.len() - 3]; // Remove checksum.
        match sentence.find(",") {
            Some(delimiter) => {
                let datatype: String = sentence[1..delimiter].to_string();
                let args: String = sentence[delimiter + 1..].to_string();

                return Some((datatype, args));
            }
            None => return None,
        }
    }

    fn checksum(s: &str) -> bool {
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

    fn _parse_gpgll(args: String) -> GpsArgValues {
        // Format for the gpgll data string:
        // [0] Latitude(as hhmm.mmm),
        // [1] Latitude North or South,
        // [2] Longitude(as hhmm.mmm),
        // [3] Longitude North or South,
        // [4] Time as hhmmss,
        // [5] isactivedata(no idea what it does or is)

        // Assumes to have $GPGLL and *AB removed.
        // Untested with data.
        let mut gps_values = GpsArgValues::default();
        let data: Vec<&str> = args.split(",").collect();

        // Parse Latitude.
        match Gps::_parse_degrees(data[0].to_string()) {
            Some(mut latitude) => {
                if data[1].to_ascii_lowercase() == "s".to_ascii_lowercase() {
                    latitude *= -1 as f32;
                }
                gps_values.latitude = Some(latitude);
            }
            None => gps_values.latitude = None,
        }

        // Parse Longitude.
        match Gps::_parse_degrees(data[2].to_string()) {
            Some(mut longitude) => {
                if data[3].to_ascii_lowercase() == "w".to_ascii_lowercase() {
                    longitude *= -1 as f32
                }
                gps_values.longitude = Some(longitude);
            }
            None => gps_values.longitude = None,
        }

        // Parse time
        gps_values.timestamp = Some(Gps::_format_hhmmss(data[4]));

        // No idea what the point of this data point is.
        gps_values.isactivedata = Some(data[5].to_string());

        return gps_values
    }

    fn _parse_gprmc(args: String) -> GpsArgValues {
        //Data string format:
        // [0] Time (as hhmmss) -> parse to hh:mm:ss,
        // [1] fix_quality (a = good fix),
        // [2] latitude(as hhmm.mmm),
        // [3] atitude north or south,
        // [4] logitude (as hhmm.mmm),
        // [5] longitude north or south,
        // [6] speed in knots,
        // [7] track angle degrees,
        // [8] time (as ddmmyy) -> parse to yy-mm-dd

        let mut gps_values = GpsArgValues::default();
        let data:Vec<&str> = args.split(",").collect();
        if data.len() < 11 {
            return gps_values  // Unexpected number of params
        }

        // Parse date and time.

        gps_values.timestamp = Some(format!("{} {}", Gps::_format_ddmmyy(data[8]), Gps::_format_hhmmss(data[0])));

        // get fix quality.
        if data[1].to_ascii_lowercase() == "a".to_ascii_lowercase() {
            gps_values.fix_quality = Some(1)
        } else {
            gps_values.fix_quality = Some(0)
        }

        match Gps::_parse_degrees(data[2].to_string()) {
            Some(mut latitude) => {
                if data[3].to_ascii_lowercase() == "s".to_ascii_lowercase() {
                    latitude *= -1 as f32;
                }
                gps_values.latitude = Some(latitude);
            }
            None => gps_values.latitude = None,
        }
        match Gps::_parse_degrees(data[4].to_string()) {
            Some(mut latitude) => {
                if data[5].to_ascii_lowercase() == "w".to_ascii_lowercase() {
                    latitude *= -1 as f32;
                }
                gps_values.latitude = Some(latitude);
            }
            None => gps_values.latitude = None,
        }
        match data[6].parse::<f32>() {
            Ok(speed_knots) => gps_values.speed_knots = Some(speed_knots),
            Err(_e) => gps_values.speed_knots = None,
        }
        match data[7].parse::<f32>() {
            Ok(track_angle_deg) => gps_values.track_angle_deg = Some(track_angle_deg),
            Err(_e) => gps_values.track_angle_deg = None,
        }

        return gps_values

    }

    fn _parse_gpgga(args:String) -> GpsArgValues {
        // Format for data:
        // [0] time (as hhmmss),
        // [1] latitude (as hhmm.mmm),
        // [2] latitude north or south,
        // [3] longitude (as hhmm.mmm),
        // [4] longitude north or south,
        // [5] fix quality,
        // [6] satellites being tracked,
        // [7] horizontal dilution,
        // [8] altitude in metres,
        // [9] Unknown,
        // [10] height geoid,

        let mut gps_values = GpsArgValues::default();
        let data:Vec<&str> = args.split(",").collect();
        if data.len() != 14 {
            return gps_values  // Unexpected number of params.
        }

        // Parse time
        gps_values.timestamp = Some(Gps::_format_hhmmss(data[0]));

        // Parse lat
        match Gps::_parse_degrees(data[1].to_string()) {
            Some(mut latitude) => {
                if data[2].to_ascii_lowercase() == "s".to_ascii_lowercase() {
                    latitude *= -1 as f32;
                }
                gps_values.latitude = Some(latitude);
            }
            None => gps_values.latitude = None,
        }

        // Parse long
        match Gps::_parse_degrees(data[3].to_string()) {
            Some(mut longitude) => {
                if data[4].to_ascii_lowercase() == "w".to_ascii_lowercase() {
                    longitude *= -1 as f32;
                }
                gps_values.longitude = Some(longitude);
            }
            None => gps_values.longitude = None,
        }

        match data[5].parse::<i32>() {
            Ok(fix_quality) => gps_values.fix_quality = Some(fix_quality),
            Err(_e) => gps_values.fix_quality = None,
        }

        match data[6].parse::<i32>() {
            Ok(satellites) => gps_values.satellites = Some(satellites),
            Err(_e) => gps_values.satellites = None,
        }

        match data[7].parse::<f32>() {
            Ok(horizontal_dilution) => gps_values.horizontal_dilution = Some(horizontal_dilution),
            Err(_e) => gps_values.horizontal_dilution = None,
        }

        match data[8].parse::<f32>() {
            Ok(altitude_m) => gps_values.altitude_m = Some(altitude_m),
            Err(_e) => gps_values.altitude_m = None,
        }

        match data[10].parse::<f32>() {
            Ok(height_geoid) => gps_values.height_geoid = Some(height_geoid),
            Err(_e) => gps_values.height_geoid = None,
        }

        return gps_values
    }

    fn _parse_degrees(nmea_data: String) -> Option<f32> {
        // Parse NMEA lat/long data pair ddmm.mmmm into pure degrees value.
        // dd is degrees, mm.mmmm is minutes
        let nmea_data = nmea_data.as_str();

        let deg:f32 = (&nmea_data[0..2]).parse::<f32>().unwrap();

        let minutes: f32 = ((&nmea_data[2..]).parse::<f32>().unwrap()) / 60.0;

        let r: f32 = deg + minutes;
        let r:f32 = format!("{:.6}", r).parse().unwrap();  // Round to 6 decimal places.
        Some(r)

    }

    fn _format_hhmmss(time:&str) ->  String {
        // Take in a string of hhmmss and return it as a formatted hh-mm-ss
        let hours = &time[0..2];
        let mins = &time[2..4];
        let secs = &time[4..6];
        return format!("{}:{}:{}", hours, mins, secs);
    }

    fn _format_ddmmyy(time:&str) -> String {
        let days = &time[0..2];
        let months = &time[2..4];
        let years = format!("20{}", &time[4..6]);  // Only works till year 3000.
        return format!("{}-{}-{}", years, months, days);
    }
}



#[cfg(test)]
mod gps_test {
    use super::*;
    #[test]
    fn _parse_hhmmss() {
        assert_eq!(Gps::_format_hhmmss("205530"), "20:55:30".to_string());
    }
    #[test]
    fn _parse_ddmmyy() {
        assert_eq!(Gps::_format_ddmmyy("300320"), "2020-03-30".to_string());
    }
    #[test]
    fn _parse_degrees() {
        assert_eq!(Gps::_parse_degrees("3218.0489".to_string()).unwrap(), 32.300815);
        assert_eq!(Gps::_parse_degrees("6447.5086".to_string()).unwrap(), 64.79181);
    }
    #[test]
    fn checksum() {
        assert_eq!(Gps::checksum("$GNGGA,165419.000,5132.7378,N,00005.9192,W,1,7,1.93,34.4,M,47.0,M,,*6A"), true);
    }

    fn _parse_gpgll() {}

    fn _parse_gprmc() {}

    fn _parse_gpgga() {}
}


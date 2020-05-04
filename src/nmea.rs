//! NMEA is the sentence format for receiving data from a GPS.
//!
//! # GPS outputs
//! ## GPS sentence outputs.
//! - GGA -> UTC, Latitude, longitude, Position fix (GPS, DGPS, No fix), sats used, HDOP, altitude, Geoidal Seperation, Age of diff corr
//! - VTG -> Course (true), Course (magnetic), speed knots, speed kph, mode.
//! - GSA -> Manual or Automatic mode, 2D or 3D fix, List of satellites used, PDOP, HDOP, VDOP.
//! - GSV -> Satellites in view data: sat id, elevation, azimuth and SNR for each sat seen.
//!
//! These two are pointless.
//! - RMC -> UTC, lat, long, speed, course, date, magnetic variation.
//! - GLL -> Lat, Long. Pretty pointless sentence.
//!
//! On each iteration, all the data is the same. So why not output all the data and gather it all?
//!
//! ### Sentence prefix: ${GP, GL, GA, GN}{GGA, GSA, GSV, RMC, VTG}
//! GP is short for GPS (American)
//!
//! GL is short for GLONASS (Russian)
//!
//! GA is short for Galileo (EU)
//!
//! GN is multi-system.
//!
//! ### Prefixes table ({} means heading of GP/GL/GA is added.
//! |           |GGA     |GSA     |GSV    |RMC     |VTG  |
//! |-----------|:------:|-------:|------:|-------:|-----|
//! |GPS        |GPGGA   |GPGSA   |GPGSV  |GPRMC   |GPVTG|
//! |GP+GL      |GNGGA   |{}GAS   |{}GSV  |GNRMC   |GNVTG|
//! |GP+GL+GA   |GNGG    |{}GSA   |{}GSV  |GNRMC   |GNVTG|
//!
//! In the GP+GL and GP+GL+GA modes, all satellites from those systems are used for the best fix.
//!

pub mod nmea {
    //! Main moduel for parsing any NMEA sentence and exporting NMEA parsing to lib.rs

    use crate::gps;

    pub fn _parse_degrees(degrees: &str, compass_direction: &str) -> Option<f32> {
        // Parse NMEA lat/long data pair dddmm.mmmm into pure degrees value.
        // ddd is degrees, mm.mmmm is minutes
        // Formula is->
        if degrees.is_empty() {
            return None;
        }
        let deg: f32 = (&degrees[0..2]).parse::<f32>().unwrap();

        let minutes: f32 = ((&degrees[2..]).parse::<f32>().unwrap()) / 60.0;

        let r: f32 = deg + minutes;
        let r: f32 = format!("{:.6}", r).parse().unwrap(); // Round to 6 decimal places.

        if (compass_direction == "N") | (compass_direction == "E") {
            return Some(r);
        } else if (compass_direction == "S") | (compass_direction == "W") {
            return Some(r * -1.0);
        } else {
            panic!("Compass direction not north or south")
        }
    }

    pub fn _format_hhmmss(time: &str) -> String {
        // Take in a string of hhmmss and return it as a formatted hh-mm-ss
        if time.len() < 6 {
            return "".to_string();
        }
        let hours = &time[0..2];
        let mins = &time[2..4];
        let secs = &time[4..6];
        return format!("{}:{}:{}", hours, mins, secs);
    }

    pub fn parse_sentence(sentence: &str) -> Option<Vec<&str>> {
        // Assumes that a valid sentence is always given.
        // Convert sentence into a split vec along ','.

        let sentence = sentence.trim(); // Remove whitespace.
        if sentence.len() < 6 {
            return None;
        }
        return if gps::is_valid_checksum(sentence) {
            let sentence: &str = &sentence[0..sentence.len() - 3]; // Remove checksum.
            Some(sentence.split(",").collect())
        } else {
            None
        };
    }
}

pub mod gga {
    //! Parse GGA sentences
    //!
    //! UTC, Latitude, Longitude, Fix quality, Satellites used, HDOP, MSL altitude, Geoidal separation
    //! , Age of difference correction.

    use super::nmea::*;

    #[derive(Debug, PartialEq)]
    pub enum SatFix {
        NoFix,
        GpsFix,
        DgpsFix,
    }

    impl Default for SatFix {
        fn default() -> SatFix {
            SatFix::NoFix
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct GgaData {
        pub utc: f64,
        pub lat: Option<f32>,
        pub long: Option<f32>,
        pub sat_fix: SatFix,
        pub satellites_used: i32,
        pub hdop: Option<f32>,
        pub msl_alt: Option<f32>,
        pub geoidal_sep: Option<f32>,
        pub age_diff_corr: Option<f32>,
    }

    /// Take a parse_sentence vec<&str> and output GgaData.
    pub fn parse_gga(args: Vec<&str>) -> GgaData {
        //! ${GP,GL,GA,GN}GGA, UTC, lat, N/S, long, E/S, Fix quality, Sats used, HDOP, Alt, Alt Units,
        //! Geoidal separation, Geo units, Age of diff corr, * checksum
        //!
        //! Time, sat fix and sats used always given.
        let header = args.get(0).unwrap();
        if &header[3..5] != "GG" {
            panic!(format!(
                "Sentence is not a GGA format, it's {} format",
                header
            ))
        }

        // Parse time
        let utc: f64 = args.get(1).unwrap().parse().unwrap();

        // Parse lat
        let lat: Option<f32> = _parse_degrees(args.get(2).unwrap(), args.get(3).unwrap());
        let long: Option<f32> = _parse_degrees(args.get(4).unwrap(), args.get(5).unwrap());

        let sat_fix = match args.get(6).unwrap() {
            &"0" => SatFix::NoFix,
            &"1" => SatFix::GpsFix,
            &"2" => SatFix::DgpsFix,
            _ => SatFix::NoFix,
        };
        let satellites_used: i32 = args.get(7).unwrap().parse().unwrap();
        let hdop = args.get(8).unwrap().parse::<f32>().ok();
        let msl_alt: Option<f32> = args.get(9).unwrap().parse::<f32>().ok();
        let geoidal_sep: Option<f32> = args.get(11).unwrap().parse::<f32>().ok();
        let age_diff_corr: Option<f32> = args.get(13).unwrap().parse::<f32>().ok();
        return GgaData {
            utc,
            lat,
            long,
            sat_fix,
            satellites_used,
            hdop,
            msl_alt,
            geoidal_sep,
            age_diff_corr,
        };
    }
}

pub mod gsa {
    //! Parse GSA sentences.
    //!
    //! Gives All the satellites that are being tracked and the HDOP, VDOP, PDOP.

    #[derive(PartialEq, Debug)]
    pub enum Mode {
        Manual,
        Automatic,
    }

    #[derive(PartialEq, Debug)]
    pub enum DimentionFix {
        NotAvaliable,
        Dimention2d,
        Dimention3d,
    }

    #[derive(PartialEq, Debug)]
    pub struct GsaData {
        pub mode: Mode,
        pub dimention_fix: DimentionFix,
        pub sat1: Option<i32>,
        pub sat2: Option<i32>,
        pub sat3: Option<i32>,
        pub sat4: Option<i32>,
        pub sat5: Option<i32>,
        pub sat6: Option<i32>,
        pub sat7: Option<i32>,
        pub sat8: Option<i32>,
        pub sat9: Option<i32>,
        pub sat10: Option<i32>,
        pub sat11: Option<i32>,
        pub sat12: Option<i32>,
        pub pdop: Option<f32>,
        pub hdop: Option<f32>,
        pub vdop: Option<f32>,
    }

    pub fn parse_gsa(args: Vec<&str>) -> GsaData {
        //! Format
        //! $G{}GSA, Mode, dimention_fix, Sat1, Sat2, Sat3, Sat4, Sat5, Sat6, Sat7, Sat8, Sat9, Sat10,
        //! Sat11, Sat12, PDOP, HDOP, VDOP  *checksum
        //!
        //! Mode1 (Mode)
        //! - M (Manual - forced to operate in 2D or 3D mode),
        //! - A (2D automatic - can switch between 2D and 3D automatically)
        //!
        //! Mode2 (DimentionFix) :
        //! - 1 - Fix not avaliable
        //! - 2 - 2D (< 4 SVs used)
        //! - 3- 3D (>= 4 SVs used)
        //!
        //! Mode and DimentionFix should always be given. The other values don't have to be.

        let header = args.get(0).unwrap();
        if &header[3..6] != "GSA" {
            panic!(format!(
                "Incorrect sentence header. Should be GSA, it is {}",
                header
            ))
        }

        let mode = match args.get(1).unwrap() {
            &"M" => Mode::Manual,
            &"A" => Mode::Automatic,
            _ => Mode::Manual, // Default.
        };
        let dimention_fix = match args.get(2).unwrap() {
            &"1" => DimentionFix::NotAvaliable,
            &"2" => DimentionFix::Dimention2d,
            &"3" => DimentionFix::Dimention3d,
            _ => DimentionFix::NotAvaliable,
        };
        let sat1: Option<i32> = args.get(3).unwrap().parse::<i32>().ok();
        let sat2: Option<i32> = args.get(4).unwrap().parse::<i32>().ok();
        let sat3: Option<i32> = args.get(5).unwrap().parse::<i32>().ok();
        let sat4: Option<i32> = args.get(6).unwrap().parse::<i32>().ok();
        let sat5: Option<i32> = args.get(7).unwrap().parse::<i32>().ok();
        let sat6: Option<i32> = args.get(8).unwrap().parse::<i32>().ok();
        let sat7: Option<i32> = args.get(9).unwrap().parse::<i32>().ok();
        let sat8: Option<i32> = args.get(10).unwrap().parse::<i32>().ok();
        let sat9: Option<i32> = args.get(11).unwrap().parse::<i32>().ok();
        let sat10: Option<i32> = args.get(12).unwrap().parse::<i32>().ok();
        let sat11: Option<i32> = args.get(13).unwrap().parse::<i32>().ok();
        let sat12: Option<i32> = args.get(14).unwrap().parse::<i32>().ok();

        let pdop: Option<f32> = args.get(15).unwrap().parse::<f32>().ok();
        let hdop: Option<f32> = args.get(16).unwrap().parse::<f32>().ok();
        let vdop: Option<f32> = args.get(17).unwrap().parse::<f32>().ok();

        return GsaData {
            mode,
            dimention_fix,
            sat1,
            sat2,
            sat3,
            sat4,
            sat5,
            sat6,
            sat7,
            sat8,
            sat9,
            sat10,
            sat11,
            sat12,
            pdop,
            hdop,
            vdop,
        };
    }
}

pub mod gsv {
    //! Parse GSV sentence
    //!
    //! GSV gives satellites in view. If there are many satellites in view it will require
    //! multiple sentences.
    //!

    /// This is the individual satellite data given by the GSV sentence. It is used in the
    /// main GpsData struct, as a Vec<Satellites>.
    #[derive(PartialEq, Debug)]
    pub struct Satellites {
        pub id: Option<i32>,
        pub elevation: Option<f32>,
        pub azimuth: Option<f32>,
        pub snr: Option<f32>,
    }

    pub fn parse_gsv(args: Vec<&str>) -> Vec<Satellites> {
        //! Format $GPGSV, Number of messages, Message number, Sats in view,
        //!      sat ID, Sat elevation, Sat Azimuth, Sat SNE, Repeat 4 times, *checksum
        //!
        //! Sentences can vary in length.
        //!
        //! A single GSV string can hold 4 satellites worth of data.
        //!
        //! It is given for each set of satellites it could track (GP, GL, etc).
        //!
        //! $GPGSV,1,1,00*79 if no satellites are in view.
        //!
        //! Max of 4 messages so 16 total satellites.
        //!
        //! Assumes that the sentences will always come one after another, I can just read the next sentences.

        let header = args.get(0).unwrap();
        if &header[3..6] != "GSV" {
            panic!(format!(
                "Incorrect sentence header. Should be GSV, it is {}",
                header
            ))
        }
        let mut values = Vec::new();
        let _meta = &args.get(0..4);
        let sat1 = &args.get(4..8);
        let sat2 = &args.get(8..12);
        let sat3 = &args.get(12..16);
        let sat4 = &args.get(16..20);
        for sat in &[sat1, sat2, sat3, sat4] {
            if sat.is_some() {
                values.push(parse_sat(sat.unwrap()))
            }
        }
        values
    }

    fn parse_sat(args: &[&str]) -> Satellites {
        Satellites {
            id: args.get(0).unwrap().parse().ok(),
            elevation: args.get(1).unwrap().parse().ok(),
            azimuth: args.get(2).unwrap().parse().ok(),
            snr: args.get(3).unwrap().parse().ok(),
        }
    }
}

pub mod rmc {
    //! Parse RMC sentences.
    //!
    //! Gives UTC, latitude, longitude, Speed, True course, Magnetic course, Date, Magnatic variation
    use super::nmea::*;

    #[derive(PartialEq, Debug)]
    pub struct RmcData {
        pub utc: f64,
        pub fix_status: bool,
        pub latitude: Option<f32>,
        pub longitude: Option<f32>,
        pub speed: Option<f32>,
        pub course: Option<f32>,
        pub date: String,
        pub mag_var: Option<f32>,
    }

    pub fn parse_rmc(args: Vec<&str>) -> RmcData {
        //! Magnetic variation, positive is east, negative is west.
        //! Data string format:
        //!   0     1         2       3           4       5       6           7       8           9
        //! $GPRMC,UTC, Fix status, Lat, NS indicator, Long, EW indicator, Speed, Course (true), date,
        //!         10                           11                  12
        //! magnetic variation (degrees), magnetic variation (E/W), Mode * checksum

        let utc = args.get(1).unwrap().parse().unwrap_or(0.0);
        let fix_status = match args.get(2).unwrap_or(&"V") {
            &"A" => true,
            &"V" => false,
            _ => false,
        };
        let latitude: Option<f32> = _parse_degrees(args.get(3).unwrap(), args.get(4).unwrap());
        let longitude: Option<f32> = _parse_degrees(args.get(5).unwrap(), args.get(6).unwrap());
        let speed: Option<f32> = args.get(7).unwrap().parse::<f32>().ok();
        let course: Option<f32> = args.get(8).unwrap().parse::<f32>().ok();
        let date: String = args.get(9).unwrap_or(&"").to_string();
        let mag_var: Option<f32> = match args.get(12).unwrap_or(&"") {
            &"E" => args.get(11).unwrap().parse::<f32>().ok(),
            &"W" => Some(args.get(11).unwrap().parse::<f32>().unwrap() * -1.0),
            _ => None,
        };
        return RmcData {
            utc,
            fix_status,
            latitude,
            longitude,
            speed,
            course,
            date,
            mag_var,
        };
    }
}

pub mod vtg {
    //! Parse VTG sentences
    //!
    //! Gives course headings and speed data.

    #[derive(PartialEq, Debug)]
    pub enum Mode {
        Autonomous,
        Differential,
        Estimated,
        Unknown,
    }

    #[derive(PartialEq, Debug)]
    pub struct VtgData {
        pub true_course: Option<f32>,
        pub magnetic_course: Option<f32>,
        pub speed_knots: Option<f32>,
        pub speed_kph: Option<f32>,
        pub mode: Mode,
    }

    pub fn parse_vtg(args: Vec<&str>) -> VtgData {
        //! Sentence format
        //!
        //! $GPVTG,  course, reference (True), course, reference (magnetic), Speed, knots,
        //! speed, kph, mode.
        let true_course: Option<f32> = args.get(1).unwrap().parse::<f32>().ok();
        let magnetic_course: Option<f32> = args.get(3).unwrap().parse::<f32>().ok();
        let speed_knots: Option<f32> = args.get(5).unwrap().parse::<f32>().ok();
        let speed_kph: Option<f32> = args.get(7).unwrap().parse::<f32>().ok();

        let mode = match args.get(9).unwrap_or(&"N") {
            &"A" => Mode::Autonomous,
            &"D" => Mode::Differential,
            &"E" => Mode::Estimated,
            _ => Mode::Unknown,
        };
        return VtgData {
            true_course,
            magnetic_course,
            speed_knots,
            speed_kph,
            mode,
        };
    }
}

pub mod gll {
    /// This module is basically pointless as all gll data is in the gga data.
    use super::nmea::*;

    #[derive(PartialEq, Debug)]
    pub struct GllData {
        pub latitude: Option<f32>,
        pub longitude: Option<f32>,
        pub utc: Option<f64>,
        pub is_valid: bool,
    }

    pub fn parse_gll(args: Vec<&str>) -> GllData {
        // Format for the gpgll data string:
        // [1] Latitude(as hhmm.mmm),
        // [2] Latitude North or South,
        // [3] Longitude(as hhmm.mmm),
        // [4] Longitude North or South,
        // [5] Time as hhmmss.ss,
        // [6] A
        // [7] A

        // Parse Latitude.

        let latitude: Option<f32> = _parse_degrees(args.get(1).unwrap(), args.get(2).unwrap());
        let longitude: Option<f32> = _parse_degrees(args.get(3).unwrap(), args.get(4).unwrap());
        // Parse time
        let utc = args.get(5).unwrap_or(&"0").parse::<f64>().ok();
        let is_valid = match args.get(6).unwrap_or(&"") {
            &"A" => true,
            &"V" => false,
            _ => false,
        };
        return GllData {
            latitude,
            longitude,
            utc,
            is_valid,
        };
    }
}

#[cfg(test)]
mod nmea_tests {
    mod gga {
        use crate::nmea::gga;

        #[test]
        fn gga_normal() {
            //${GP,GL,GA,GN}GGA, UTC, lat, N/S, long, E/S, Fix quality, Sats used, HDOP, Alt, Alt Units,
            // Geoidal separation, Geo units, Age of diff corr, * checksum
            assert_eq!(
                gga::parse_gga(vec![
                    "$GPGGA",
                    "19294.00",
                    "29343.543",
                    "N",
                    "29343.543",
                    "E",
                    "1",
                    "10",
                    "1.01",
                    "47.7",
                    "M",
                    "10.0",
                    "M",
                    "0.1"
                ]),
                gga::GgaData {
                    utc: 19294.00,
                    lat: Some(34.725716),
                    long: Some(34.725716),
                    sat_fix: gga::SatFix::GpsFix,
                    satellites_used: 10,
                    hdop: Some(1.01),
                    msl_alt: Some(47.7),
                    geoidal_sep: Some(10.0),
                    age_diff_corr: Some(0.1),
                }
            );
        }

        #[test]
        #[should_panic]
        fn gga_incorrect_header() {
            gga::parse_gga(vec![
                "$GPGSV",
                "19294.00",
                "29343.543",
                "N",
                "29343.543",
                "E",
                "1",
                "10",
                "1.01",
                "47.7",
                "M",
                "10.0",
                "M",
                "0.1",
            ]);
        }
    }
    mod gsa {
        use crate::nmea::gsa;

        #[test]
        fn gsa_normal() {
            assert_eq!(
                gsa::parse_gsa(vec![
                    "$GPGSA", "M", "2", "01", "02", "03", "04", "05", "06", "07", "08", "09", "10",
                    "11", "12", "1.0", "2.04", "32.04"
                ]),
                gsa::GsaData {
                    mode: gsa::Mode::Manual,
                    dimention_fix: gsa::DimentionFix::Dimention2d,
                    sat1: Some(1),
                    sat2: Some(2),
                    sat3: Some(3),
                    sat4: Some(4),
                    sat5: Some(5),
                    sat6: Some(6),
                    sat7: Some(7),
                    sat8: Some(8),
                    sat9: Some(9),
                    sat10: Some(10),
                    sat11: Some(11),
                    sat12: Some(12),
                    pdop: Some(1.0),
                    hdop: Some(2.04),
                    vdop: Some(32.04)
                }
            )
        }
        #[test]
        #[should_panic]
        fn gsa_incorrect_header() {
            gsa::parse_gsa(vec![
                "$GPGGA", "M", "2", "01", "02", "03", "04", "05", "06", "07", "08", "09", "10",
                "11", "12", "1.0", "2.04", "32.04",
            ]);
        }
    }
    mod gsv {}
    mod rmc {}
    mod vtg {}
}

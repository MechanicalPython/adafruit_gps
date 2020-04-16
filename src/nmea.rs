//! NMEA is the sentence format for receiving data from a GPS.
//! There are 5 output formats:
//! GGA -> Time, position and fix type
//! GSA -> GNSS receiver operating mode: active satellite numbers, PDOP, VDOP, HDOP
//! GSV -> GNSS satellites in view, elevation, azimuth, SNR values
//! RMC -> Time, date, position, course, speed
//! VTG -> Course and speed info relative to the ground.
//!
//! ## Sentence prefix: ${GP, GL, GA, GN}{GGA, GSA, GSV, RMC, VTG}
//! GP is short for GPS (American)
//! GL is short for GLONASS (Russian)
//! GA is short for Galileo (EU)
//! GN is multi-system.
//!
//! ## Prefixes table ({} means heading of GP/GL/GA is added.
//! |           |GGA     |GSA     |GSV    |RMC     |VTG  |
//! |-----------|:------:|-------:|------:|-------:|-----|
//! |GPS        |GPGGA   |GPGSA   |GPGSV  |GPRMC   |GPVTG|
//! |GP+GL      |GNGGA   |{}GAS   |{}GSV  |GNRMC   |GNVTG|
//! |GP+GL+GA   |GNGG    |{}GSA   |{}GSV  |GNRMC   |GNVTG|
//! In the GP+GL and GP+GL+GA modes, all satellites from those systems are used for the best fix.
//!
//! ## Data formats
//! ### GGA
//! UTC, Latitude, longitude, Position fix (GPS, DGPS, No fix), sats used, HDOP, altitude, Geoidal Seperation, Age of diff corr
//! ### GSA
//! Manual or Automatic mode, 2D or 3D fix, List of satellites used, PDOP, HDOP, VDOP.
//! ### GSV
//! Satellites in view data: sat id, elevation, azimuth and SNR for each sat seen.
//! ### RMC
//! UTC, lat, long, speed, course, date, magnetic variation.
//! ### VTG
//! Course (true), Course (magnetic), speed knots, speed kph, mode.
//!
//! Combine GSA and GSV to give SatelliteMetaData:
//! For each satellite seen, give the data from GSV plus the DOP data from GSA.
//!
//!
//! |Position fix indicator|1||Value of satellite fix, 0:no fix, 1:GPS fix, 2:DGPS fix|
//! |Satellites used|14||Number of satellites that can be seen.|
//! |HDOP|1.26||Horizontal Dilution of Precision. It's a measure of error based on the satellites error bounds and position|
//! |PDOP|1.26||Position Dilution of Precision. It's a measure of error based on the satellites error bounds and position|
//! |VDOP|1.26||Vertical Dilution of Precision. It's a measure of error based on the satellites error bounds and position|
//! |MLS Altitude|22.6|metres|Altitude above Mean Sea Level|
//! |MLS Units|M|metres|Units for MLS|
//! |Geoidal Separation|18.5|metres|Unknown what this is|
//! |Geoidal units|M| metres||
//! |Age of Diff. Corr.||second|Null when no DGPS|
//! |SNR|39|dBHz|0 to 99, Null when not tracking.|
//! |Azimuth||degrees|The number of degrees (0-359) from north the satellite is. https://en.wikipedia.org/wiki/Azimuth
//!

#![allow(warnings)]
pub mod nmea {
    // Remember, read_line() always gives a sentence.
    struct PositionData {

    }
    pub fn satellite_data() {
    }

    pub fn position_data() {

    }

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
        let r: f32 = format!("{:.6}", r).parse().unwrap();  // Round to 6 decimal places.

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

    fn convert_sentence(sentence: &str) -> Vec<&str> {
        // Assumes that a valid sentence is always given.
        // Convert sentence into a split vec along ','.

        let sentence = sentence.trim();  // Remove whitespace.
        let sentence: &str = &sentence[0..sentence.len() - 3]; // Remove checksum.

        return sentence.split(",").collect();
    }
}

mod gga {
    use super::nmea::*;

    enum SatFix {
        NoFix,
        GpsFix,
        DgpsFix,
    }

    pub struct GgaData {
        utc: f64,
        lat: Option<f32>,
        long: Option<f32>,
        sat_fix: SatFix,
        satellites_used: i32,
        hdop: Option<f32>,
        msl_alt: Option<f32>,
        geoidal_sep: Option<f32>,
        age_diff_corr: Option<f32>,
    }

    pub fn parse_gga(args: Vec<&str>) -> GgaData {
        // Format for data:
        //      0             1     2   3     4     5        6           7       8     9       10
        // ${GP,GL,GA,GN}GGA, UTC, lat, N/S, long, E/S, Fix quality, Sats used, HDOP, Alt, Alt Units,
        //
        //        11              12             13             14
        // Geoidal separation, Geo units, Age of diff corr, * checksum
        // Time, sat fix and sats used always given.

        // Parse time
        let utc: f64 = _format_hhmmss(args.get(1).unwrap()).parse().unwrap();

        // Parse lat
        let lat: Option<f32> = _parse_degrees(args.get(2).unwrap(), args.get(3).unwrap());
        let long: Option<f32> = _parse_degrees(args.get(4).unwrap(), args.get(5).unwrap());

        let sat_fix = match args.get(6).unwrap() {
            &"0" => SatFix::NoFix,
            &"1" => SatFix::GpsFix,
            &"2" => SatFix::DgpsFix,
            _ => SatFix::NoFix,
        };
        use std::str::FromStr;
        let satellites_used: i32 = args.get(7).unwrap().parse().unwrap();
        let hdop  = args.get(8).unwrap().parse::<f32>().ok();
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

mod gsa {
    //! Format for GSA sentence:
    //! $GPGSA,Mode1, Mode2, Sat1,Sat2,Sat3,Sat4,Sat5,Sat6,Sat7,Sat8,Sat9,Sat10,Sat11,Sat12 ,PDOP,HDOP,VDOP*Checksum
    //! Mode1 (Mode) : M (Manual - forced to operate in 2D or 3D mode),
    //!                A (2D automatic - can switch between 2D and 3D automatically)
    //! Mode2 (DimentionFix) : 1 - Fix not avaliable
    //!                        2 - 2D (< 4 SVs used)
    //!                        3- 3D (>= 4 SVs used)
    //! Mode and DimentionFix should always be given. The other values don't have to be.

    enum Mode {
        Manual,
        Automatic,
    }

    enum DimentionFix {
        NotAvaliable,
        Dimention2d,
        Dimention3d,
    }

    pub struct GsaData {
        mode: Mode,
        dimention_fix: DimentionFix,
        sat1: Option<i32>,
        sat2: Option<i32>,
        sat3: Option<i32>,
        sat4: Option<i32>,
        sat5: Option<i32>,
        sat6: Option<i32>,
        sat7: Option<i32>,
        sat8: Option<i32>,
        sat9: Option<i32>,
        sat10: Option<i32>,
        sat11: Option<i32>,
        sat12: Option<i32>,
        pdop: Option<f32>,
        hdop: Option<f32>,
        vdop: Option<f32>,
    }

    pub fn parse_gsa(args: Vec<&str>) -> GsaData {
        // Format
        //    0      1          2         3     4     5     6     7     8     9     10    11    12
        // $G{}GSA, Mode, dimention_fix, Sat1, Sat2, Sat3, Sat4, Sat5, Sat6, Sat7, Sat8, Sat9, Sat10,
        //    13    14    15    16    17
        // Sat11, Sat12, PDOP, HDOP, VDOP  *checksum

        let mode = match args.get(1).unwrap() {
            &"M" => Mode::Manual,
            &"A" => Mode::Automatic,
            _ => Mode::Manual,  // Default.
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

// todo
mod gsv {
    // GSV gives satellites in view. If there are many satellites in view it will require
    // multiple sentences.
    // A single GSV string can hold 4 satellites worth of data.
    // It is given for each set of satellites it could track (GP, GL, etc).

    // $GPGSV,1,1,00*79 if no satellites are in view.
    // Format is $GPSGV,number of messages, message number, satellites in view, sat id, elevation, azimuth, SNR,
    // SNR can be null (,,)
    // Max of 4 messages so 16 total satellites.
    // If I assume that the sentences will always come one after another, I can just read the next sentences.

    pub struct GsvData {
        sat1: Option<SatData>,
        sat2: Option<SatData>,
        sat3: Option<SatData>,
        sat4: Option<SatData>,
        sat5: Option<SatData>,
        sat6: Option<SatData>,
        sat7: Option<SatData>,
        sat8: Option<SatData>,
        sat9: Option<SatData>,
        sat10: Option<SatData>,
        sat11: Option<SatData>,
        sat12: Option<SatData>,
        sat13: Option<SatData>,
        sat14: Option<SatData>,
        sat15: Option<SatData>,
        sat16: Option<SatData>,
    }

    pub struct SatData {
        id: i32,
        elevation: i32,
        azimuth: i32,
        snr: i32,
    }

    pub fn parse_gsv(args: Vec<&str>) -> GsvData {
        // Format $GPGSV, Number of messages, Message number, Sats in view,
        //      sat ID, Sat elevation, Sat Azimuth, Sat SNE, Repeat 4 times, *checksum
        if args.len() == 4 {
            return GsvData {
                sat1: None,
                sat2: None,
                sat3: None,
                sat4: None,
                sat5: None,
                sat6: None,
                sat7: None,
                sat8: None,
                sat9: None,
                sat10: None,
                sat11: None,
                sat12: None,
                sat13: None,
                sat14: None,
                sat15: None,
                sat16: None
            }
        }
        let number_of_messages:i32 = args.get(1).unwrap().parse().unwrap();
        for message in 1..number_of_messages + 1 {


        }
        return GsvData {
                sat1: None,
                sat2: None,
                sat3: None,
                sat4: None,
                sat5: None,
                sat6: None,
                sat7: None,
                sat8: None,
                sat9: None,
                sat10: None,
                sat11: None,
                sat12: None,
                sat13: None,
                sat14: None,
                sat15: None,
                sat16: None
            }

    }
}

mod rmc {
    //! Fix status is bool, true for it has a fix.
    //! Magnetic variation, positive is east, negative is west.

    use super::nmea::*;

    pub struct RmcData {
        utc: f64,
        fix_status: bool,
        latitude: Option<f32>,
        longitude: Option<f32>,
        speed: Option<f32>,
        course: Option<f32>,
        date: String,
        mag_var: Option<f32>,
    }

    pub fn parse_rmc(args: Vec<&str>) -> RmcData {
        // Data string format:
        //   0     1         2       3           4       5       6           7       8      9
        // $GPRMC,UTC, Fix status, Lat, NS indicator, Long, EW indicator, Speed, Course, date,
        //         10                           11                  12
        // magnetic variation (degrees), magnetic variation (E/W), Mode * checksum

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
        return RmcData{
            utc,
            fix_status,
            latitude,
            longitude,
            speed,
            course,
            date,
            mag_var
        }
    }
}

mod vtg {
    enum Mode {
        Autonomous,
        Differential,
        Estimated,
        Unknown,
    }
    pub struct VtgData {
        true_course: Option<f32>,
        magnetic_course: Option<f32>,
        speed_knots: Option<f32>,
        speed_kph: Option<f32>,
        mode: Mode,
    }
    pub fn parse_vtg(args: Vec<&str>) -> VtgData {
        // Format
        //    0       1             2             3             4             5      6
        // $GPVTG,  course, reference (True), course, reference (magnatic), Speed, knots,
        //   7     8    9
        // speed, kph, mode.
        let true_course: Option<f32> = args.get(1).unwrap().parse::<f32>().ok();
        let magnetic_course: Option<f32> = args.get(3).unwrap().parse::<f32>().ok();
        let speed_knots: Option<f32> = args.get(5).unwrap().parse::<f32>().ok();
        let speed_kph: Option<f32> = args.get(7).unwrap().parse::<f32>().ok();

        let mode = match args.get(9).unwrap_or(&"N") {
            &"A" => Mode::Autonomous,
            &"D" => Mode::Differential,
            &"E" => Mode::Estimated,
            _ => Mode::Unknown
        };
        return VtgData {
            true_course,
            magnetic_course,
            speed_knots,
            speed_kph,
            mode,
        }

    }
}

mod gll {
    use super::nmea::*;

    pub struct GllData {
        latitude: Option<f32>,
        longitude: Option<f32>,
        utc: Option<f64>,
        is_valid: bool,
    }

    pub fn parse_gll(args: Vec<&str>) -> GllData {
        // Format for the gpgll data string:
        // [1] Latitude(as hhmm.mmm),
        // [2] Latitude North or South,
        // [3] Longitude(as hhmm.mmm),
        // [4] Longitude North or South,
        // [5] Time as hhmmss.ss,
        // [6] isactivedata(no idea what it does or is)

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
            is_valid
        }
    }
}














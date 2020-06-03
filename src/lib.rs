//! # Adafruit_gps
//! This is a port from the adafruit python code that reads the output from their GPS systems.
//! This crate has been tested on a MTK3339 chip on a raspberry pi zero.
//!
//! ## Links
//! - Python code: https://github.com/adafruit/Adafruit_CircuitPython_GPS
//! - GPS module docs: https://learn.adafruit.com/adafruit-ultimate-gps/
//! - PMTK commands https://cdn-shop.adafruit.com/datasheets/PMTK_A11.pdf
//!
//! ## Modules
//! The PMTK module is a way of easily sending command to the GPS, changing it's settings.
//!
//! The nmea module reads the data given by the GPS. Use the gps.update() trait to get easy to use
//! data, but for specific use cases custom commands can be read.
//!
//! ## Hardware specs
//! Please read the docs for the specific GPS module you are using.
//!
//! Update rate is likely 1Hz to 10Hz.
//! If increasing the update rate, the baud rate may also need to be increased.
//! A rule of thumb is, one sentence is 256 bytes -> at 9600 baud rate, 37.5 sentences per second.
//!
//! # Module Outputs
//! gps.update() outputs a GpsSentence enum which mostly gives other structs for different sentence types
//!
//! - GGA(GgaData) -> [GgaData](nmea/gga/struct.GgaData.html): Latitude, Longitude, Position fix, Satellites seen, HDOP, altitude, Geoidal Seperation, Age of difference correction.
//! - VTG(VtgData) -> [VtgData](nmea/vtg/struct.VtgData.html): Course (true), Course (magnetic), speed knots, speed kph.
//! - GSA(GsaData) -> [GsaData](nmea/gsa/struct.GsaData.html): List of satellites used, PDOP, HDOP, VDOP.
//! - GSV(Vec<Satellites>) -> [Satellites](nmea/gsv/struct.Satellites.html): Satellites in view data: sat id, elevation, azimuth and SNR for each sat seen.
//! - GLL(GllData) -> [GllData](nmea/gll/struct.GllData.html): Latitude, Longitude only.
//! - RMC(RmcData) -> [RmcData](nmea/rmc/struct.RmcData.html): UTC, Latitude, Longitude, speed, course, date, magnetic variation.
//! - NoConnection -> The gps is not connected, no bytes are being received
//! - InvalidBytes -> Bytes being received are not valid, probably port baud rate and gps baud rate mismatch
//! - InvalidSentence -> Sentence outputted has incorrect checksum, the sentence was probably incomplete.
//!
//! # Some technical information
//! ## Dilution of precision
//! DOP is dilution of precision, a measure of error based on the position of the satellites.
//! The smaller the number the better (1 is excellent).
//!
//! The DOP is determined by the arrangement of the satellites being tracked.
//! The DOP can be either vertical or horizontal as different satellite arrangements affect the
//! vertical and horizontal DOP differently.
//!
//! See [This wikipeia page](https://en.wikipedia.org/wiki/Dilution_of_precision_(navigation)) for details
//!
//! ## Geoid and Mean sea level
//! Measuring height is difficult because where 0m is exactly is hard to establish.
//!
//! The geoid is the shape that the ocean would take under the influence of gravity and the earth's
//! rotation alone, ignoring tides and wind.
//!
//! The WGS84 ellipsoid is the ideal smooth surface shape of the earth, with no mountains or trenches.
//!
//! The height of the geoid given by GgaData geoidal_sep is the difference between the geoid and the
//! WGS84 ellipsoid. It ranges from +85 m to -106 m.
//!
//! A reading of +47 for geoidal_sep means the geoid is 47 metres above the WGS84 ellipsoid.
//!
//! Mean sea level is locally defined and changes depending on location. Therefore, altitude given
//! by the gps is, in my opinion, not overly useful for precise elevation, but rather is useful in
//! measuring the difference in height between objects.
//!
//!
//!

//todo https://en.wikipedia.org/wiki/List_of_GPS_satellites
//todo find the order of all sentences as they are produced.


use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Read, Write};

use bincode::{serialize, serialize_into};

// todo - re export this in a better way with fewer imports.
pub use crate::nmea::{gga, gll, gsa, gsv, rmc, vtg};
pub use crate::nmea::parse_nmea;
pub use crate::open_gps::gps::{Gps, GpsSentence, is_valid_checksum, open_port, PortConnection};
pub use crate::pmtk::send_pmtk;

mod nmea;
mod pmtk;
mod open_gps;

pub trait GpsIO {
    fn read_from(file: &str) -> Vec<GpsSentence>;
    fn write_to(&self, file: &str);
    fn append_to(self, file: &str);
}

impl GpsIO for Vec<GpsSentence> {
    fn read_from(file: &str) -> Vec<GpsSentence> {
        let f = File::open(file).unwrap();
        let decode: Vec<GpsSentence> = bincode::deserialize_from(f).unwrap();
        return decode;
    }

    fn write_to(&self, file: &str) {
        let mut f = BufWriter::new(File::create(file).unwrap());
        serialize_into(&mut f, self).unwrap();
    }

    fn append_to(self, _file: &str) { unimplemented!() }
}

impl GpsSentence {
    pub fn read_from(file: &str) -> Vec<GpsSentence> {
        let mut f = File::open(file).unwrap();
        let mut buffer = Vec::new();
        let _ = f.read_to_end(&mut buffer);
        let split = buffer.split(|num| num == &10);
        let mut struct_vec: Vec<GpsSentence> = Vec::new();
        for item in split {
            match bincode::deserialize(item) {
                Ok(T) => {
                    struct_vec.push(T)
                }
                _ => {}
            }
        }

        // let decode: Vec<GpsSentence> = bincode::deserialize_from(f).unwrap();
        // return decode;
        return struct_vec;
    }

    /// Write as a vector.
    pub fn write_to(&self, file: &str) {
        let value: Vec<&GpsSentence> = vec![self];
        let mut f = BufWriter::new(File::create(file).unwrap());
        serialize_into(&mut f, &value).unwrap();
    }

    /// Append a GpsSentence struct to a Vec<GpsSentence> in a file
    ///
    /// Append with a \n (10) byte for it to be read back into a vector.
    pub fn append_to(self, file: &str) {
        let mut f = OpenOptions::new().append(true).create(true).open(file).unwrap();
        // has to open a file if none exist.

        let _ = f.write(serialize(&self).unwrap().as_ref());
        let breakline: [u8; 1] = [10];
        let _ = f.write(&breakline);
    }
}

#[cfg(test)]
mod test_read_write {
    use std::fs::remove_file;

    use crate::GpsIO;
    use crate::nmea::gga::{GgaData, SatFix};

    use super::GpsSentence;

    const SENTENCE: GpsSentence = GpsSentence::GGA(GgaData {
        utc: 100.0,
        lat: Some(51.55465),
        long: Some(-0.05632),
        sat_fix: SatFix::DgpsFix,
        satellites_used: 4,
        hdop: Some(1.453),
        msl_alt: Some(42.53),
        geoidal_sep: Some(47.0),
        age_diff_corr: None,
    });

    #[test]
    fn read_write_single() {
        SENTENCE.write_to("test");
        let read = GpsSentence::read_from("test");
        let _ = remove_file("test");
        assert_eq!(read, vec![SENTENCE]);
    }

    #[test]
    fn read_write_vec() {
        let s: Vec<GpsSentence> = vec![SENTENCE];
        s.write_to("vec_test");
        let read: Vec<GpsSentence> = GpsSentence::read_from("vec_test");
        assert_eq!(s, read);
        let _ = remove_file("vec_test");
    }

    #[test]
    fn read_and_write_loop() {
        let mut check_vec = Vec::new();
        for _ in 0..3 {
            SENTENCE.append_to("loop_test");
            check_vec.push(SENTENCE)
        }

        let read: Vec<GpsSentence> = GpsSentence::read_from("loop_test");
        assert_eq!(read, check_vec);
        // let _ = remove_file("loop_test");
    }
}

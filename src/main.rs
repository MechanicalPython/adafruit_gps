use adafruit_gps::{Gps, GpsIO, GpsSentence};
use adafruit_gps::gga::{GgaData, SatFix};
use adafruit_gps::send_pmtk::NmeaOutput;

use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write, Read};

use bincode::{serialize_into, serialize};

fn main() {

    const a: GpsSentence = GpsSentence::GGA(GgaData {
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

    for _ in 0..3 {
        a.append_to("test");
    }

    println!("{:?}", GpsSentence::read_from("test"));


    // 256*256+256+5 -> [5, 1, 1, 0, 0, 0, 0, 0]
    //                [number of items, +256, +256^2, +256^3
    // Only 8 bytes at the front of the file -> max vec length -> 18,519,084,246,547,628,544
    // the vector has [1, 0, 0, 0, 0, 0, 0, 0] at the front of the file. which is the number of
    // items are in the vector.
    // [1, 0, 0, 0, 0, 0, 0, 0]

}


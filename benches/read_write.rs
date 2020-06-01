#![feature(test)]

extern crate test;

use test::Bencher;

use std::fs::remove_file;

use adafruit_gps::{GpsIO, GpsSentence};
use adafruit_gps::gga::{GgaData, SatFix};

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

#[bench]
fn bench_write_gps_sentence_at_once(b: &mut Bencher) {
    let mut v = Vec::new();
    for _ in 0..10 {
        v.push(SENTENCE)
    }

    b.iter(|| {
        v.write_to("bench_test1");
        remove_file("bench_test1")
    })
}

#[bench]
fn bench_write_gps_sentence(b: &mut Bencher) {
    b.iter(|| {
        for _ in 0..10 {
            SENTENCE.append_to("bench_test2")
        }
        remove_file("bench_test2")
    })
}

fn predone_setup() -> Vec<GpsSentence> {
    let mut v = Vec::new();
    for _ in 0..10 {
        v.push(SENTENCE)
    }
    return v;
}

#[bench]
fn predone_iter(b: &mut Bencher) {
    let v = predone_setup();
    b.iter(|| {
        for i in v.iter() {
            let i = i.clone();
            i.append_to("predone1")
        }
        remove_file("predone1")
    })
}

#[bench]
fn predone_straight(b: &mut Bencher) {
    let v = predone_setup();
    b.iter(|| {
        v.write_to("predone2");
        remove_file("predone2")
    })
}

//running 4 tests
// test bench_write_gps_sentence         ... bench: 7,226,068,625 ns/iter (+/- 62,822,335)
// test bench_write_gps_sentence_at_once ... bench:     627,288 ns/iter (+/- 77,663)
// test predone_iter                     ... bench: 7,171,678,115 ns/iter (+/- 163,164,323)
// test predone_straight                 ... bench:     613,634 ns/iter (+/- 52,295)
#![feature(test)]

extern crate test;

use test::Bencher;

use std::fs::{remove_file, OpenOptions};

use adafruit_gps::{GpsSentence};
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

const VECTOR_SIZE: i32 = 1_000;

#[bench]
fn bench_write_vector(b: &mut Bencher) {
    let mut v = Vec::new();
    for _ in 0..VECTOR_SIZE {
        v.push(SENTENCE)
    }

    b.iter(|| {
        for s in v.iter() {
            s.clone().append_to("bench_test1")
        }
        let _ = remove_file("bench_test1");
    });

}

#[bench]
fn bench_append(b: &mut Bencher) {
    b.iter(|| {
        for _ in 0..VECTOR_SIZE {
            SENTENCE.append_to("bench_test2")
        }
        let _ = remove_file("bench_test2");
    });

}

#[bench]
fn bench_append_single_struct(b: &mut Bencher) {
    b.iter(|| {
        for _ in 0..1 {
            SENTENCE.append_to("bench_test2")
        }
        let _ = remove_file("bench_test2");
    });
}

#[bench]
fn bench_open_remove_file(b: &mut Bencher) {
    b.iter(|| {
        let _f = OpenOptions::new().append(true).create(true).open("test_remove_file").unwrap();
        let _ = remove_file("test_remove_file");
    })
}

#[bench]
fn bench_read(b: &mut Bencher) {
    for _ in 0..VECTOR_SIZE {
            SENTENCE.append_to("bench_read")
    }

    b.iter(|| {
        GpsSentence::read_from("bench_read")
    });
    let _ = remove_file("bench_read");
}



// 10 iter in loop.
//running 4 tests
// open,append,write,open,append         ... bench: 7,226,068,625 ns/iter (+/- 62,822,335)
// write a vector at once                ... bench:     627,288 ns/iter (+/- 77,663)

// 10 iter loops
//test bench_append ... bench:     524,309 ns/iter (+/- 699,760)
// test bench_read   ... bench:      38,062 ns/iter (+/- 64,274)
// test bench_vector ... bench:     628,855 ns/iter (+/- 1,559,046)
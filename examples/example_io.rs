use adafruit_gps::{Gps, GpsSentence};
use adafruit_gps::NmeaOutput;

fn main() {
    let mut gps = Gps::new("/dev/serial0", "9600");
    gps.pmtk_220_set_nmea_updaterate("1000");
    gps.pmtk_314_api_set_nmea_output(NmeaOutput{ gll: 1, rmc: 0, vtg: 0, gga: 0, gsa: 1, gsv: 0, pmtkchn_interval: 0 });

    for _ in 0..100 {
        let values = gps.update();
        values.append_to("main_test");
    }

    // Read a file of structs. Always gives it as a vector.
    let gps: Vec<GpsSentence> = GpsSentence::read_from("main_test");
    println!("{:?}", gps);

    // If you have a Vec<GpsSentence> and you wish to save it, do the following:
    // v is the Vec<GpsSentence>. The reason you have to do this is I don't want to implement a
    // a trait on Vec<GpsSentence>.
    let v: Vec<GpsSentence> = Vec::new();
    for s in v.iter() {
        s.clone().append_to("bench_test1")
    }

}
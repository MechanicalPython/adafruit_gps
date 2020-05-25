
use adafruit_gps::gps::{Gps, GpsSentence};
use adafruit_gps::send_pmtk::NmeaOutput;

fn main() {
    let mut gps = Gps::new("/dev/serial0", "9600");
    gps.pmtk_220_set_nmea_updaterate("1000");
    gps.pmtk_314_api_set_nmea_output(NmeaOutput{ gll: 1, rmc: 0, vtg: 0, gga: 0, gsa: 1, gsv: 0, pmtkchn_interval: 0 });

    for _ in 0..100 {
        let values = gps.update();
        values.save("test");
    }
    let gps = GpsSentence::read("test");
    println!("{:?}", gps);

}


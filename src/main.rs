use adafruit_gps::{Gps, GpsSentence};
use adafruit_gps::NmeaOutput;

fn main() {
    let mut gps = Gps::new("/dev/serial0", "9600");
    gps.pmtk_314_api_set_nmea_output(NmeaOutput { gga: 1, gsa: 1, gsv: 1, gll: 1, rmc: 1, vtg: 1, pmtkchn_interval: 1 });
    let _r = gps.pmtk_220_set_nmea_updaterate("1000");
    loop {
        let values = gps.update();

        match values.clone() {
            GpsSentence::InvalidSentence => println!("Invalid sentence, try again"),
            GpsSentence::InvalidBytes => println!("Invalid bytes given, try again"),
            GpsSentence::NoConnection => println!("No connection with gps"),
            GpsSentence::GGA(sentence) => {
                println!("UTC: {}\nLat:{}, Long:{}, Sats:{}, MSL Alt:{}",
                         sentence.utc, sentence.lat.unwrap_or(0.0), sentence.long.unwrap_or(0.0), sentence.satellites_used,
                         sentence.msl_alt.unwrap_or(0.0) + sentence.geoidal_sep.unwrap_or(0.0));
            }
            GpsSentence::GSA(sentence) => {
                println!("PDOP:{}, VDOP:{}, HDOP:{}",
                         sentence.pdop.unwrap_or(0.0), sentence.vdop.unwrap_or(0.0), sentence.hdop.unwrap_or(0.0))
            }
            _ => {
                ()
            }
        }
        values.append_to("main_file");
    }
}


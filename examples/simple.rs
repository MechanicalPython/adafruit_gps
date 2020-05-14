extern crate adafruit_gps;

use std::env;

use adafruit_gps::gps::{Gps, open_port, GpsSentence};

fn main() {
    // Args are baud_rate, port.
    let args: Vec<String> = env::args().collect();
    let baud_rate = args.get(1).unwrap();
    let port = args.get(2).unwrap();

    // Open the port that is connected to the GPS module.
    let port = open_port(port.as_str(), baud_rate.parse().unwrap());
    // Initialise the Gps.
    let mut gps = Gps {port};

    // gps.init() requires the update rate for the gps (1000 miliseconds (1Hz) is default)
    // It returns a hash map to tell you if setting the update rate was successful and if the
    // return type setting was successful.
    // If the return type setting was not successful, the gps.update() method may hang forever and
    // you will need to try gps.init() again until it is successful.
    // If the update_rate is not successful, the gps will run but at whatever the previous setting was.
    // If setting the update_rate consistently fails for faster updates, see exmaples/increase_frequency.rs

    // Give settings here.
    gps.pmtk_314_api_set_nmea_output(1, 1, 1, 1, 1, 1, 1);
    let r = gps.pmtk_220_set_nmea_updaterate("1000");
    println!("{:?}", r);

    // In a loop, constantly update the gps. The update trait will give you all the data you
    // want from the gps module.
    loop {
        let values = gps.update();

        // Depending on what values you are interested in you can adjust what sentences you
        // wish to get and ignore all other sentences.
        match values {
            GpsSentence::InvalidSentence => println!("Invalid sentence, try again"),
            GpsSentence::InvalidBytes => println!("Invalid bytes given, try again"),
            GpsSentence::NoConnection => println!("No connection with gps"),
            GpsSentence::GGA(sentence) => {
                println!("UTC: {}\nLat:{}, Long:{}, Sats:{}, MSL Alt:{}",
                         sentence.utc, sentence.lat.unwrap_or(0.0), sentence.long.unwrap_or(0.0), sentence.satellites_used,
                sentence.msl_alt.unwrap_or(0.0));
            }
            GpsSentence::GSA(sentence) => {
                println!("PDOP:{}, VDOP:{}, HDOP:{}",
                         sentence.pdop.unwrap_or(0.0), sentence.vdop.unwrap_or(0.0), sentence.hdop.unwrap_or(0.0))
            }
            _ => {
                ()
            }
        }
    }
}
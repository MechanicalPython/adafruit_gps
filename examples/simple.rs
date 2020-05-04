extern crate adafruit_gps;

pub use adafruit_gps::gps::{GetGpsData, Gps, open_port};

fn main() {
    // Open the port that is connected to the GPS module.
    let port = open_port("/dev/serial0", 9600);
    // Initialise the Gps.
    let mut gps = Gps {port, satellite_data: true, naviagtion_data: true };

    // gps.init() requires the update rate for the gps (1000 miliseconds (1Hz) is default)
    // It returns a hash map to tell you if setting the update rate was successful and if the
    // return type setting was successful.
    // If the return type setting was not successful, the gps.update() method may hang forever and
    // you will need to try gps.init() again until it is successful.
    // If the update_rate is not successful, the gps will run but at whatever the previous setting was.
    // If setting the update_rate consistently fails for faster updates, see exmaples/increase_frequency.rs
    let values = gps.init("1000");

    // In a loop, constantly update the gps. The update trait will give you all the data you
    // want from the gps module.
    loop {
        let values = gps.update();
        let pretty_print = format!("utc:{},lat:{:?},long:{:?}, alt:{:?}, course true:{:?}, course mag:{:?}, knots:{:?}, kph:{:?}, geo:{:?}, age:{:?}, sats:{:?}, hdop:{:?}, vdop:{:?}, pdop:{:?}, satellites:{:?}\n", values.utc, values.latitude, values.longitude, values.altitude, values.true_course,
        values.mag_course, values.speed_knots, values.speed_kph, values.geoidal_spe, values.age_diff_corr,
        values.sats_used, values.hdop, values.vdop, values.pdop, values.satellites);
        println!("{}", pretty_print);
    }
}
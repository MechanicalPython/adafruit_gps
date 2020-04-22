extern crate adafruit_gps;

pub use adafruit_gps::gps::{GetGpsData, Gps, open_port};
use adafruit_gps::PMTK::send_pmtk::SendPmtk;


fn main() {
    // Open the port that is connected to the GPS module.
    let port = open_port("/dev/serial0");
    // Initialise the Gps.
    let mut gps = Gps {port};
    // Send the gps a PMTK command telling it to give you no rmc or gll data
    // but to give gga, gsa, vtg and gsv data once per loop. Read the docs for advanced usage.

    gps.pmtk_314_api_set_nmea_output(0,0,1,1,1,1,1);
    // Recommended gps update rate 1000miliseconds, or 1Hz.
    gps.pmtk_220_set_nmea_updaterate("1000");
    gps.pmtk_220_set_nmea_updaterate("100");
    gps.pmtk_220_set_nmea_updaterate("10000");

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
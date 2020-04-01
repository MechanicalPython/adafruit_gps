use std::time::{SystemTime};

use mylib::{Gps, GpsArgValues, open_port};

fn main() {
    let mut gps = Gps { port: open_port("/dev/serial0") };
    let mut gps_values = GpsArgValues::default();

    // Turn on the basic GGA and RMC info (what you typically want)
    gps.send_command("PMTK314,0,1,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0");

    // Turn on just minimum info (RMC only, location):
    // gps.send_command("PMTK314,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0");
    // Turn off everything:
    // gps.send_command("PMTK314,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0");
    //Then on everything (not all of it is parsed!)
    // gps.send_command("PMTK314,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0");

    // Set update rate to once a second (1hz) which is what you typically want.
    gps.send_command("PMTK220,1000");

    // Or decrease to once every two seconds by doubling the millisecond value.
    // Be sure to also increase your UART timeout above!
    // gps.send_command("PMTK220,2000");

    // You can also speed up the rate, but don't go too fast or else you can lose
    // data during parsing.  This would be twice a second (2hz, 500ms delay):
    // gps.send_command("PMTK220,500");
    let mut last_print = SystemTime::now();
    loop {
        gps_values = gps.update(gps_values);

        if last_print.elapsed().unwrap().as_secs() >= 1 {
            last_print = SystemTime::now();
            if (gps_values.fix_quality < Some(1)) | (gps_values.fix_quality == None) {
                println!("Waiting for fix...");
                continue;
            } else {
                println!("=========================================");
                println!("{:?}", gps_values.timestamp);
                println!("Latitude ----{:?} degrees", gps_values.latitude);
                println!("Longitude ---{:?} degrees", gps_values.longitude);
                println!("Fix quality -{:?}", gps_values.fix_quality);
                println!("Satellites --{:?}", gps_values.satellites);
                println!("Altitude (m) {:?}", gps_values.altitude_m);
                println!("Speed (knots){:?}", gps_values.speed_knots);
                println!("Track angle  {:?}", gps_values.track_angle_deg);
                println!("HODP --------{:?}", gps_values.horizontal_dilution);
                println!("Geod height -{:?}", gps_values.height_geoid);
            }
        }
    }
}

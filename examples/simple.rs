extern crate adafruit_gps;

pub use adafruit_gps::gps::{GetGpsData, Gps, open_port};
use adafruit_gps::send_pmtk::SendPmtk;

fn main() {
    let port = open_port("/dev/serial0");
    let mut gps = Gps { port, gps_type: "MT3339" };

    for _i in 0..100 {
        let l = gps.read_line();
        dbg!(l);
    }
}
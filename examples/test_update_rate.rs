extern crate adafruit_gps;

pub use adafruit_gps::gps::{GetGpsData, Gps, open_port};
use adafruit_gps::PMTK::send_pmtk::{SendPmtk, set_baud_rate};

// For use in testing your gps modules update rate. type the update rate in miliseconds in the cmd line.


// stty -F /dev/serial0 raw 9600 cs8 clocal -cstopb
// echo -e "\$PMTK251,57600*2C\r\n" > /dev/serial0
// stty -F /dev/serial0 57600 clocal cread cs8 -cstopb -parenb


fn main() {
    // Send baudrate change to the gps -> echo \cmd > /dev/serial0
    // change the serial port baudrate -> stty -F /dev/serial0 raw 19200 cs8 clocal -cstopb
    // Open gps and update the hz.

    set_baud_rate("57600",  "/dev/serial0");

    let port = open_port("/dev/serial0", 57600);

    let mut gps = Gps { port , satellite_data: false, naviagtion_data: true };

    let update_r = gps.pmtk_220_set_nmea_updaterate("100");
    dbg!(update_r);

    for _ in 0..10 {
        let values = gps.update();
        println!("{}", values.utc);
    }
}
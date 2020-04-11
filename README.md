# Introduction
This is a port of the adafruit python module (https://github.com/adafruit/Adafruit_CircuitPython_GPS) that reads and
parses the NMEA data sentences from their Ultimate GPS and Adafruit Mini GPS PA1010D. 
Most of this readme is also pulled directly from the adafruit library. 

The GPS module provides a serial byte signal providing longitude, latitude and metadata. 

This module has only been tested using an Adafruit Mini GPS PA1010D on a raspberry pi zero. 

# Dependencies
Serialport: https://docs.rs/serialport/3.2.0/serialport/. Serialport is used for reading the byte information provided
by the GPS as well as sending it commands. 

# Usage Example
```
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
```

Note: Sending multiple PMTK314 Packets with gps.send_command() will not work unless there is a substantial amount of 
time in-between each time gps.send_command() is called. A time.sleep() of 1 second or more should fix this.

# NMEA data
The GPS uses NMEA 0183 protocol. 

The data is therefore formatted by the GPS in two ways: GGA and RMC.

## GGA (Direct from adafruit README)
                                                        11
           1         2       3 4        5 6 7  8   9  10 |  12 13  14   15
           |         |       | |        | | |  |   |   | |   | |   |    |
    $--GGA,hhmmss.ss,llll.ll,a,yyyyy.yy,a,x,xx,x.x,x.x,M,x.x,M,x.x,xxxx*hh


1. Time (UTC)
2. Latitude
3. N or S (North or South)
4. Longitude
5. E or W (East or West)
6. GPS Quality Indicator,

   * 0 - fix not available,
   * 1 - GPS fix,
   * 2 - Differential GPS fix
7. Number of satellites in view, 00 - 12
8. Horizontal Dilution of precision. (A measure of error propagation due to the spatial arrangement of satellites and 
the error on those satellites. Meaning: 1 is ideal, 20 is very poor. [See here for details](https://en.wikipedia.org/wiki/Dilution_of_precision_(navigation)#Meaning_of_DOP_Values[citation_needed]))   
9. Antenna Altitude above/below mean-sea-level (geoid)
10. Units of antenna altitude, meters
11. Geoidal separation, the difference between the WGS-84 earth ellipsoid and mean-sea-level (geoid),
        "-" means mean-sea-level below ellipsoid
12. Units of geoidal separation, meters
13. Age of differential GPS data, time in seconds since last SC104 type 1 or 9 update, null field when DGPS is not used
14. Differential reference station ID, 0000-1023
15. Checksum

## RMC (Direct from adafruit README)
                                                               12
           1         2 3       4 5        6 7   8   9   10   11|
           |         | |       | |        | |   |   |    |   | |
    $--RMC,hhmmss.ss,A,llll.ll,a,yyyyy.yy,a,x.x,x.x,xxxx,x.x,a*hh

1. Time (UTC)
2. Status, V = Navigation receiver warning
3. Latitude
4. N or S
5. Longitude
6. E or W
7. Speed over ground, knots
8. Track made good, degrees true
9. Date, ddmmyy
10. Magnetic Variation, degrees
11. E or W
12. Checksum

[Info about NMEA taken from here](https://www.tronico.fi/OH6NT/docs/NMEA0183.pdf)

[Information on the PMTK commands you can send to the GPS](https://cdn-shop.adafruit.com/datasheets/PMTK_A11.pdf)
# Notes on the code and contributing
This crate library has mostly been made as a personal challenge and to fill a narrow gap so all contributions are welcome.
That said, this code is likely to need improvement, all of which is welcome. 

RMC and GGA are both documented but the source code includes GPGLL and GNGGL, neither of which have been tested but have 
been included in the lib. 
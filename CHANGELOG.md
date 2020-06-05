# Change log

##From version 3.5 to 4.0
- Added Geodesy
- New NmeaOutput for pmtk_314_api_set_nmea_output
- Stricter imports
- Gps struct no longer needs open_port: let mut gps = Gps::new(port, baud_rate);

//! NMEA is the sentence format for receiving data from a GPS.
//! There are 5 output formats:
//! GGA -> Time, position and fix type
//! GSA -> GNSS receiver operating mode, active satellites, DOP
//! GSV -> GNSS satellites in view, elevation, azimuth, SNR values
//! RMC -> Time, date, position, course, speed
//! VTG -> Course and speed info relative to the ground.
//!
//! ## Sentence prefix: ${GP, GL, GA, GN}{GGA, GSA, GSV, RMC, VTG}
//! GP is short for GPS (American)
//! GL is short for GLONASS (Russian)
//! GA is short for Galileo (EU)
//! GN is multi-system.
//!
//! ## Prefixes table ({} means heading of GP/GL/GA is added.
//! |           |GGA     |GSA     |GSV    |RMC     |VTG  |
//! |-----------|:------:|-------:|------:|-------:|-----|
//! |GPS        |GPGGA   |GPGSA   |GPGSV  |GPRMC   |GPVTG|
//! |GP+GL      |GNGGA   |{}GAS   |{}GSV  |GNRMC   |GNVTG|
//! |GP+GL+GA   |GNGG    |{}GSA   |{}GSV  |GNRMC   |GNVTG|
//!
//! ## Data formats
//! |Name       |Example        |Units          |Description|
//! |-----------|:-------------:|--------------:|-----------|
//! |Header     |$GNGGA | | Protocol header for each sentence|
//! |UTC time | 165006.000||hhmmss.sss|
//! |Latitude| 2241.9107||ddmm.mmmm|
//! |Longitude|17483.2848||ddmm.mmmm|
//! |N/S/E/W indicator|N||North, south, east, west for lat/long|
//! |Position fix indicator|1||Value of satellite fix, 0:no fix, 1:GPS fix, 2:DGPS fix|
//! |Satellites used|14||Number of satellites that can be seen.|
//! |HDOP|1.26||Horizontal Dilution of Precision. It's a measure of error based on the satellites error bounds and position|
//! |PDOP|1.26||Position Dilution of Precision. It's a measure of error based on the satellites error bounds and position|
//! |VDOP|1.26||Vertical Dilution of Precision. It's a measure of error based on the satellites error bounds and position|
//! |MLS Altitude|22.6|metres|Altitude above Mean Sea Level|
//! |MLS Units|M|metres|Units for MLS|
//! |Geoidal Separation|18.5|metres|Unknown what this is|
//! |Geoidal units|M| metres||
//! |Age of Diff. Corr.||second|Null when no DGPS|
//! |SNR|39|dBHz|0 to 99, Null when not tracking.|
//! |Azimuth||degrees|The number of degrees (0-359) from north the satellite is. https://en.wikipedia.org/wiki/Azimuth
//!


pub mod nmea {}
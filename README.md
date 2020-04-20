# Introduction
This is a port of the adafruit python module (https://github.com/adafruit/Adafruit_CircuitPython_GPS) that reads and
parses the NMEA data sentences from their Ultimate GPS and Adafruit Mini GPS PA1010D. 
Most of this readme is also pulled directly from the adafruit library. 

The GPS module provides a serial byte signal providing longitude, latitude and metadata. 

This module has only been tested using an Adafruit Mini GPS PA1010D on a raspberry pi zero. 

# Dependencies
Serialport: https://docs.rs/serialport/3.2.0/serialport/. Serialport is used for reading the byte information provided by the GPS as well as sending it commands. 

Serial dependencies are: 

For GNU Linux `pkg-config` and `libudev` headers are required:

- Ubuntu: `sudo apt install pkg-config libudev-dev`
- Fedora: `sudo dnf install pkgconf-pkg-config systemd-devel`
- Other: Some Linux distros are providing pkgconf.org's `pkgconf` package instead of freedesktop.org's `pkg-config`.

## Usage

See example/simple.rs for examples on basic usage.

For more advanced usage, read the specs.md for the commands that can be read and commands that can be sent.  

# Notes on the code and contributing
This crate library has mostly been made as a personal challenge and to fill a narrow gap so all contributions are welcome.
That said, this code is likely to need improvement, all of which is welcome. 

RMC and GGA are both documented but the source code includes GPGLL and GNGGL, neither of which have been tested but have 
been included in the lib. 
# Introduction
This is a port of the adafruit python module (https://github.com/adafruit/Adafruit_CircuitPython_GPS) that reads and
parses the NMEA data sentences from their Ultimate GPS and Adafruit Mini GPS PA1010D. 
Most of this readme is also pulled directly from the adafruit library. 

The GPS module provides a serial byte signal providing longitude, latitude and metadata. 

This module has only been tested using an Adafruit Mini GPS PA1010D on a raspberry pi zero. 


## Note
3.0 update introduced breaking changes to Gps struct and open port in an attempt to increase gps Hz. 

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

## Notes the baud rate and gps output frequency
For the 3.0 update, I've tried to get the gps to increase it's update frequency to 10Hz. 

To increase the frequency, the baud rate of the gps AND the port need to be changed to allow for more data to be outputted. 
To do this, at the beginning of the file, before opening the port, the baud rate must be set (unless the default 9600 is 
what you want). See increase_frequency.rs for an example.

set_baud_rate takes a while and is error prone, so ideally set the baud rate and frequency and use a battery/keep power
so that the settings are retained. If power is lost (and there is no battery), settings are reset to 9600 baud rate and 1000 mili frequency. 

1000 baud = 1000 symbols per second. 
baud_rate calculations. Gps max sentence length is 255 (or close enough). 9600 -> allows a minimum of 37.64 sentences per second or 3.7 sentences per second at 10Hz. 
GGA -> 1/iter
VTG -> 1/iter
GSA -> 1/iter
RMC -> 1/iter
GLL -> 1/iter
GSV -> up to 4/iter

# Notes on the code and contributing
This crate library has mostly been made as a personal challenge and to fill a narrow gap so all contributions are welcome.
That said, this code is likely to need improvement, all of which is welcome. 

RMC and GGA are both documented but the source code includes GPGLL and GNGGL, neither of which have been tested but have 
been included in the lib. 

# In case of emergency
If your gps isn't behaving for some reason the following commands, entered into the terminal, may help. /dev/serial0 
is the port being used here so change that depending on your situation. 

- echo -e "\$PMTK104*37\r\n" > /dev/serial0 -> cold restart the gps
- stty -F /dev/serial0 raw 9600 cs8 clocal -cstopb -> change the port baud rate (not the gps baud rate)
- stty -F /dev/serial0 -> gives the baud rate of the port
- cat /dev/serial0 -> prints output of the gps. 

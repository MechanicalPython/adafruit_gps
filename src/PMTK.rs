//! PMTK commands give the gps module settings for output.
//!
//! To give a PMTK command send it via the Gps struct: gps.pmtk....(args). See simple.rs for example.
//!
//! Note:
//! All commands will be destructive for all sentences in the buffer. You will not be able to read data given by
//! the gps after sending it a command.
//!
//! ## Important commands
//! pmtk_220_set_nmea_updaterate -> Hz for the gps update rate.
//!
//! //! pmtk_314_api_set_nmea_output  -> Sets 6 modes, GLL is not included in any other docs.
//!
//! ## Changing the baudrate
//! Given it's a special function, it's a stand alone methon in the send_pmtk module.
//!
//! The port baud rate needs to be changed and the gps baudrate needs to be changed. To do so,
//! a specific set of commands need to be given (changing to 57600, for example):
//! - stty -F /dev/serial0 raw 9600 cs8 clocal -cstopb
//! - echo -e "\$PMTK251,57600*2C\r\n" > /dev/serial0
//! - stty -F /dev/serial0 57600 clocal cread cs8 -cstopb -parenb
//!
//! Then open the port at the new baudrate and you're good.
//!
//! To remove sources of error, changing the baud rate here will do a full restart, so all saved
//! options will be removed.
//!
//!
//! ## PMTK return formats
//! ### PMTK001
//! The PMTK001 command is the response given when there is a valid command given.
//!
//! Commands are Invalid, Unsupported, Failed, Successful, or No packet received.
//!
//! ### PMTK500
//! 5** pmtk standards are all reply formats. Each is individual and unique.
//!
//! ### Reboot commands
//! Upon any kind of restart the following is printed:
//! $CDACK,7,0*49\r\n  -> Unknown what this is.
//! This is what is given upon wake up.
//! $PMTK011,MTKGPS*08\r\n -> Output sys message.
//! $PMTK010,001*2E\r\n -> Sys message, 001 Startup.
//! $PMTK011,MTKGPS*08\r\n -> 001 txt message, output system message.
//! $PMTK010,002*2D\r\n -> Sys msg, 002 = Notification, aiding EPO.
//!
// Tests for setting baudrate and getting EPO data still fail, unsure why.

pub mod send_pmtk {
    use std::process::Command;
    use std::str;
    use std::thread::sleep;
    use std::time::Duration;

    use crate::gps::{GetGpsData, Gps, is_valid_checksum, open_port};

    #[derive(Debug)]
    #[derive(PartialEq)]
    /// PMTK001 return types:
    ///
    /// - Invalid (No such command)
    /// - Unsupported (Chip type does not support this command)
    /// - Falied (Chip failed to do the command for some reason)
    /// - Success (Command implimented)
    /// - NoPacket (After 10 read lines, no command found)
    pub enum Pmtk001Ack {
        // format: $PMTK001,cmd,flag*checksum\r\n
        //flag: 0
        Invalid,

        //flag: 1
        Unsupported,

        //flag: 2
        Failed,

        //flag: 3
        Success,

        NoPacket,
    }

    #[derive(Debug)]
    #[derive(PartialEq)]
    /// Dgps (Differential GPS) mode is the usage of ground stations to aid in the accuracy of position.
    /// - NoDGPS: Default
    /// - RTCM
    /// - WAAS: Wide area augmentation system. Only avaliable in North America
    pub enum DgpsMode {
        NoDgps,
        RTCM,
        WAAS,
        Unknown,
    }

    #[derive(Debug)]
    #[derive(PartialEq)]
    /// SBAS (Satellite-based augmentation systems) uses ground stations broadcasting
    /// satellite messages to aid in navigation and accuracy.
    pub enum Sbas {
        Enabled,
        Disabled,
        Unknown,
    }

    #[derive(Debug)]
    #[derive(PartialEq)]
    pub enum SbasMode {
        Testing,
        Integrity,
        Unknown,
    }

    #[derive(Debug)]
    #[derive(PartialEq)]
    pub struct NmeaOutput {
        pub gll: i8,
        pub rmc: i8,
        pub vtg: i8,
        pub gga: i8,
        pub gsa: i8,
        pub gsv: i8,
        pub pmtkchn_interval: i8,
    }

    #[derive(Debug)]
    #[derive(PartialEq)]
    pub struct EpoData {
        pub set: i8,
        pub fwn_ftow_week_number: i8,
        pub fwn_ftow_tow: i8,
        pub lwn_ltow_week_number: i8,
        pub lwn_ltow_tow: i8,
        pub fcwn_fctow_week_number: i8,
        pub fcwn_fctow_tow: i8,
        pub lcwn_lctow_week_number: i8,
        pub lcwn_lctow_tow: i8,
    }

    pub fn add_checksum(sentence: String) -> String {
        let mut checksum = 0;
        for char in sentence.as_bytes() {
            checksum ^= *char;
        }
        let checksum = format!("{:X}", checksum);  //Format as hexidecimal.
        let checksumed_sentence = format!("${}*{}\r\n", sentence, checksum).as_str().to_ascii_uppercase();
        return checksumed_sentence;
    }

    pub fn set_baud_rate(baud_rate: &str, port_name: &str) {
        // stty -F /dev/serial0 9600 clocal cread cs8 -cstopb -parenb
        // echo -e "\$PMTK251,57600*2C\r\n" > /dev/serial0
        // stty -F /dev/serial0 57600 clocal cread cs8 -cstopb -parenb

        let baud_rates:[u32;7] = [4800,9600,14400,19200,38400,57600,115200];
        for rate in baud_rates.iter() {
            let port = open_port(port_name, rate.to_owned());
            let mut gps = Gps{port, naviagtion_data: false, satellite_data: false};
            if gps.read_line() != "Invalid bytes given".to_string() {
                gps.send_command(format!("PMTK251,{}", baud_rate).as_str());
                println!("Current rate: {}", rate);
                break
            }
        }

        sleep(Duration::from_secs(1));

        // stty -F /dev/serial0 57600 clocal cread cs8 -cstopb -parenb

        Command::new("stty")
            .arg("-F")
            .arg(port_name)
            .arg(baud_rate)
            .arg("clocal")
            .arg("cread")
            .arg("-cstopb")
            .arg("-parenb")
            .output().unwrap();
        sleep(Duration::from_secs(1));
    }

    pub trait SendPmtk {
        /// Send the PMTK command.
        fn send_command(&mut self, cmd: &str);

        /// Check for a PMTK001 return.
        fn pmtk_001(&mut self, search_depth: i32) -> Pmtk001Ack;

        /// Check for PMTK500 style return.
        fn pmtk_500(&mut self) -> Option<String>;

        /// Checks if the GPS rebooted.
        fn pmtk_startup(&mut self) -> bool;

        /// Restart with all data intact.
        fn pmtk_101_cmd_hot_start(&mut self) -> bool;

        /// Hot Restart without using Ephemeris data.
        fn pmtk_102_cmd_warm_start(&mut self) -> bool;

        /// Restart with current settings, but no navigation data.
        fn pmtk_103_cmd_cold_start(&mut self) -> bool;

        /// Full cold start resets all setting to default.
        fn pmtk_104_cmd_full_cold_start(&mut self) -> bool;

        /// Set the update rate, as miliseconds from 100 (100Hz) to 10_000 (0.1Hz). 1000 is default.
        fn pmtk_220_set_nmea_updaterate(&mut self, update_rate: &str) -> Pmtk001Ack;

        /// Set Differental Gps mode
        fn pmtk_301_api_set_dgps_mode(&mut self, dgps_mode: DgpsMode) -> Pmtk001Ack;

        /// Check what the current Differential Gps mode is.
        fn pmtk_401_api_q_dgps_mode(&mut self) -> DgpsMode;

        /// Set SBAS (Satellite-based augmentation systems) enabled or disabled.
        fn pmtk_313_api_set_sbas_enabled(&mut self, sbas: Sbas) -> Pmtk001Ack;

        /// Check if SBAS is enabled
        fn pmtk_413_api_q_sbas_enabled(&mut self) -> Sbas;

        /// Set what NMEA sentences are to be outputted as frequency.
        /// The range is 0-5.
        ///
        /// - 0 -> Never
        /// - 1 -> Every output
        /// - 2 -> Every second output
        /// ...
        /// - 5 -> Every 5th output
        fn pmtk_314_api_set_nmea_output(&mut self, gll: i8, rmc: i8, vtg: i8, gga: i8, gsa: i8, gsv: i8, pmtkchn_interval: i8) -> Pmtk001Ack;

        /// Gets current NMEA output frequency.
        fn pmtk_414_api_q_nmea_output(&mut self) -> NmeaOutput;

        /// Set SBAS mode
        fn pmtk_319_api_set_sbas_mode(&mut self, sbas_mode: SbasMode) -> bool;

        /// Check SBAS mode
        fn pmtk_419_api_q_sbas_mode(&mut self) -> SbasMode;

        /// Gives GPS firmware release info.
        fn pmtk_605_q_release(&mut self) -> String;

        /// Get EPO data: Extended Prediction Orbit tries to predict where satellites will be in the future.
        fn pmtk_607_q_epo_info(&mut self) -> EpoData;

        /// Clear EPO data.
        fn pmtk_127_cmd_clear_epo(&mut self) -> Pmtk001Ack;

        /// For MT3318 and MT3329 chips.
        ///
        /// Set the minimum number for which navigation speed is just set to 0
        ///
        /// Speed thresholds: 0/ 0.2/ 0.4/ 0.6/ 0.8/ 1.0/1.5/2.0 (m/s)
        fn pmtk_397_set_nav_speed_threshold(&mut self, nav_threshold: f32) -> Pmtk001Ack;

        /// For MT3339 chips.
        ///
        /// Set the minimum number for which navigation speed is just set to 0
        ///
        /// Speed thresholds: 0/ 0.2/ 0.4/ 0.6/ 0.8/ 1.0/1.5/2.0 (m/s)
        fn pmtk_386_set_nav_speed_threshold(&mut self, nav_threshold: f32) -> Pmtk001Ack;

        /// Gets current nav speed threshold.
        fn pmtk_447_q_nav_threshold(&mut self) -> f32;

        /// Puts gps on standby mode for power saving. Send any command to wake it up again.
        fn pmtk_161_cmd_standby_mode(&mut self) -> Pmtk001Ack;

        /// Used with pmtk_225_cmd_periodic_mode to set periodic mode.
        fn pmtk_223_set_al_dee_cfg(&mut self, sv: i8, snr: i8, ext_threshold: i32, ext_gap: i32) -> Pmtk001Ack;

        /// Sets periodic mode settings
        fn pmtk_225_cmd_periodic_mode(&mut self, run_type: u8, run_time: u32, sleep_time: u32,
                                      second_run_time: u32, second_sleep_time: u32) -> Pmtk001Ack;

        /// Active Interference Calcellation to counter jamming an enterfearance.
        ///
        /// True: enable, false: disabled.
        fn pmtk_286_cmd_aic_mode(&mut self, aic: bool) -> Pmtk001Ack;

        /// Set EASY status. True: enable, False: disable.
        fn pmtk_869_cmd_easy_enable(&mut self, enable_easy: bool) -> Pmtk001Ack;

        /// Get current EASY status
        fn pmtk_869_cmd_easy_query(&mut self) -> bool;

        /// Configure LOCUS interval, seconds.
        fn pmtk_187_locus_config(&mut self, locus_interval: i8) -> Pmtk001Ack;

        /// Get DATUM, 0 = WGS84 (default).
        fn pmtk_330_api_set_datum(&mut self, datum: u16) -> Pmtk001Ack;

        /// Query current DATUM
        fn pmtk_430_api_q_datum(&mut self) -> u16;

        /// The receiver support new NMEA format for QZSS. The command allow user enable or disable QZSS
        /// NMEA format. Default is disable QZSS NMEA format. (use NMEA 0183 v3.1)
        fn pmtk_351_api_set_support_qzss_nmea(&mut self, enable_qzss: bool) -> Pmtk001Ack;

        /// Since QZSS is regional positioning service. The command allow user enable or disable QZSS function.
        /// Default is enable QZSS function
        fn pmtk_352_api_set_stop_qzss(&mut self, enable: bool) -> Pmtk001Ack;
    }

    impl SendPmtk for Gps {
        #[allow(unused_must_use)]  // self.port.write is not used
        fn send_command(&mut self, cmd: &str) {
            //! Input: no $ and no *checksum.
            let cmd = add_checksum(cmd.to_string());
            let byte_cmd = cmd.as_bytes();
            self.port.clear(serialport::ClearBuffer::Input);
            self.port.write(byte_cmd);
        }

        fn pmtk_001(&mut self, search_depth: i32) -> Pmtk001Ack {
            //! Format: $pmtk{cmd},{flag},{value}*{checksum}
            for _i in 0..search_depth {   // Check 10 lines before giving up.
                let line = self.read_line();
                if is_valid_checksum(&line) {  // Sometimes the line is incomplete so check here.
                    if &line[0..8] == "$PMTK001" {
                        let line = line.trim();
                        // Remove checksum.
                        let line: Vec<&str> = line.split("*").collect();
                        let line: &str = line.get(0).unwrap();

                        let args: Vec<&str> = line.split(",").collect();
                        // args: $PMTK001, cmd, flag,
                        // let cmd: &str = args.get(1).expect("pmtk001 format not correct");
                        let flag: &str = args.get(2).expect("pmtk001 format not correct");
                        // let value: &str = args.get(3).unwrap_or(&"");

                        return if flag == "0" {
                            Pmtk001Ack::Invalid
                        } else if flag == "1" {
                            Pmtk001Ack::Unsupported
                        } else if flag == "2" {
                            Pmtk001Ack::Failed
                        } else if flag == "3" {
                            Pmtk001Ack::Success
                        } else {
                            Pmtk001Ack::NoPacket
                        };
                    } else {
                        continue;
                    }
                } else {
                    continue;
                }
            }
            return Pmtk001Ack::NoPacket;
        }

        fn pmtk_500(&mut self) -> Option<String> {
            //! Return the string without checksum.
            for _i in 0..10 {   // Check 10 lines before giving up.
                let line = self.read_line();
                if (&line[0..5] == "$PMTK") && (is_valid_checksum(&line)) {
                    let line = line.trim();
                    // Remove checksum.
                    let line: Vec<&str> = line.split("*").collect();
                    let line: &str = line.get(0).unwrap();
                    return Some(line.to_string());
                }
            }
            return None;
        }

        fn pmtk_startup(&mut self) -> bool {
            for _i in 0..10 {
                let line = self.read_line();
                if (&line[0..8] == "$PMTK011") && (is_valid_checksum(&line)) {
                    return true;
                }
            }
            false
        }

        fn pmtk_101_cmd_hot_start(&mut self) -> bool {
            self.send_command("PMTK101");
            self.pmtk_startup()
        }

        fn pmtk_102_cmd_warm_start(&mut self) -> bool {
            self.send_command("PMTK102");
            self.pmtk_startup()
        }

        fn pmtk_103_cmd_cold_start(&mut self) -> bool {
            self.send_command("PMTK103");
            self.pmtk_startup()
        }

        fn pmtk_104_cmd_full_cold_start(&mut self) -> bool {
            self.send_command("PMTK104");
            self.pmtk_startup()
        }

        fn pmtk_220_set_nmea_updaterate(&mut self, update_rate: &str) -> Pmtk001Ack {
            self.send_command(format!("PMTK220,{}", update_rate).as_str());
            self.pmtk_001(10)
        }

        fn pmtk_301_api_set_dgps_mode(&mut self, dgps_mode: DgpsMode) -> Pmtk001Ack {
            match dgps_mode {
                DgpsMode::NoDgps => self.send_command("PMTK301,0"),
                DgpsMode::RTCM => self.send_command("PMTK301,1"),
                DgpsMode::WAAS => self.send_command("PMTK301,2"),
                DgpsMode::Unknown => (),
            }

            self.pmtk_001(10)
        }

        fn pmtk_401_api_q_dgps_mode(&mut self) -> DgpsMode {
            self.send_command("PMTK401");

            // Should be just one arg.
            return match self.pmtk_500() {
                Some(args) => {
                    if args.len() != 10 {  // $PM TK5 01, {0,1,2}
                        return DgpsMode::Unknown;
                    }
                    let mode: String = args.chars().nth_back(0).unwrap().to_string();
                    let mode: &str = mode.as_str();
                    if mode == "0" {
                        return DgpsMode::NoDgps;
                    } else if mode == "1" {
                        DgpsMode::RTCM
                    } else if mode == "2" {
                        DgpsMode::WAAS
                    } else {
                        DgpsMode::Unknown
                    }
                }
                None => DgpsMode::Unknown
            };
        }

        fn pmtk_313_api_set_sbas_enabled(&mut self, sbas: Sbas) -> Pmtk001Ack {
            //! Enable = 1 -> Default.
            //!
            //! Disabled = 0
            match sbas {
                Sbas::Enabled => self.send_command("PMTK313,1"),
                Sbas::Disabled => self.send_command("PMTK313,0"),
                Sbas::Unknown => ()
            }
            self.pmtk_001(10)
        }

        fn pmtk_413_api_q_sbas_enabled(&mut self) -> Sbas {
            self.send_command("PMTK413");
            return match self.pmtk_500() {
                Some(args) => {
                    if args.len() != 10 {
                        return Sbas::Unknown;
                    }
                    let mode = args.chars().nth_back(0).unwrap().to_string();
                    let mode = mode.as_str();
                    if mode == "0" {
                        Sbas::Disabled
                    } else if mode == "1" {
                        Sbas::Enabled
                    } else {
                        Sbas::Unknown
                    }
                }
                None => Sbas::Unknown
            };
        }

        fn pmtk_314_api_set_nmea_output(&mut self, gll: i8, rmc: i8, vtg: i8, gga: i8, gsa: i8, gsv: i8, pmtkchn_interval: i8) -> Pmtk001Ack {
            //! 19 fields can be parsed to this one.
            //!
            //! $PMTK314,{GPGLL},{GPRMC},{GPTVG},{GPGGA},{GPGAS},{GPGSV},{R}..6-17,{PMTKCHN interval}
            //!
            //! For each field, frequency setting is given: 0-5, 0-> Disabled,
            //! 1-> Output once everty one position fix, 2-> every second... every 5th.
            //!
            //! Default is PMTK314,-1* (Default: 0,1,1,1,1,5,0..0)

            self.send_command(format!("PMTK314,{},{},{},{},{},{},0,0,0,0,0,0,0,{}",
                                      gll, rmc, vtg, gga, gsa, gsv, pmtkchn_interval).as_str());
            self.pmtk_001(10)
        }

        fn pmtk_414_api_q_nmea_output(&mut self) -> NmeaOutput {
            //! Return 514: PMTK514, the nmea outputs that are valid (see pmtk_314_api_set_nmea_output
            //! for the fields).
            self.send_command("PMTK414");
            return match self.pmtk_500() {
                Some(args) => {
                    let args: Vec<&str> = args.split(",").collect();
                    let gll: &str = args.get(1).unwrap_or(&"-1");
                    let rmc: &str = args.get(2).unwrap_or(&"-1");
                    let vtg: &str = args.get(3).unwrap_or(&"-1");
                    let gga: &str = args.get(4).unwrap_or(&"-1");
                    let gsa: &str = args.get(5).unwrap_or(&"-1");
                    let gsv: &str = args.get(6).unwrap_or(&"-1");
                    let pmtkchn_interval: &str = args.get(18).unwrap_or(&"-1");

                    NmeaOutput {
                        gll: gll.parse::<i8>().unwrap(),
                        rmc: rmc.parse::<i8>().unwrap(),
                        vtg: vtg.parse::<i8>().unwrap(),
                        gga: gga.parse::<i8>().unwrap(),
                        gsa: gsa.parse::<i8>().unwrap(),
                        gsv: gsv.parse::<i8>().unwrap(),
                        pmtkchn_interval: pmtkchn_interval.parse::<i8>().unwrap(),
                    }
                }
                None => NmeaOutput {
                    gll: -1,
                    rmc: -1,
                    vtg: -1,
                    gga: -1,
                    gsa: -1,
                    gsv: -1,
                    pmtkchn_interval: -1,
                }
            };
        }

        fn pmtk_319_api_set_sbas_mode(&mut self, sbas_mode: SbasMode) -> bool {
            //! Set sbas mode. 0=testing mode and 1=integrity mode.
            //! Integrity mode is default.
            //!
            //! Get's reboot code.
            //!
            match sbas_mode {
                SbasMode::Integrity => self.send_command("PMTK391,1"),
                SbasMode::Testing => self.send_command("PMTK391,0"),
                SbasMode::Unknown => (),
            }
            self.pmtk_startup()
        }

        fn pmtk_419_api_q_sbas_mode(&mut self) -> SbasMode {
            //! 519 response, PMTK519,{0,1} for {testing mode, integrity mode}, set by 319.
            //! false: testing mode, true: integrity mode.
            //!
            self.send_command("PMTK419");
            return match self.pmtk_500() {
                Some(args) => {
                    let arg = args.chars().nth_back(0).unwrap().to_string();
                    let arg = arg.as_str();
                    if arg == "0" {
                        SbasMode::Testing
                    } else if arg == "1" {
                        SbasMode::Integrity
                    } else {
                        SbasMode::Unknown
                    }
                }
                None => SbasMode::Unknown
            };
        }

        fn pmtk_605_q_release(&mut self) -> String {
            //! Return example: $PMTK705,AXN_5.1.7_3333_19020118,0027,PA1010D,1.0*76
            //!
            //! Return blank string if no info found.
            self.send_command("PMTK605");
            return match self.pmtk_500() {
                Some(args) => {
                    args[9..args.len()].to_string()
                }
                None => "".to_string()
            };
        }

        fn pmtk_607_q_epo_info(&mut self) -> EpoData {
            //! Example sentence: $PMTK707,0,0,0,0,0,0,0,0,0*2E
            //!
            //! Return -1 if it failed to get data in someway.
            //!
            //! Get EPO data status
            //! - 0 Set: Total number sets of EPO data stored in the GPS chip
            //! - 1 FWN & FTOW : GPS week number
            //! - 2 FWN & FTOW : TOW of the first set of EPO data stored in chip respectively
            //! - 3 LWN & LTOW : GPS week number
            //! - 4 LWN & LTOW : TOW of the last set of EPO data stored in chip respectively
            //! - 5 FCWN & FCTOW : GPS week number
            //! - 6 FCWN & FCTOW : TOW of the first set of EPO data that are currently used respectively
            //! - 7 LCWN & LCTOW : GPS week number
            //! - 8 LCWN & LCTOW : TOW of the last set of EPO data that are currently used respectively

            let args = self.pmtk_500().unwrap_or("PMTK,-1,-1,-1,-1,-1,-1,-1,-1,-1".to_string());
            let args: Vec<&str> = args.split(",").collect();
            return EpoData {
                set: args.get(1).unwrap_or(&"-1").parse::<i8>().unwrap(),
                fwn_ftow_week_number: args.get(2).unwrap_or(&"-1").parse::<i8>().unwrap(),
                fwn_ftow_tow: args.get(3).unwrap_or(&"-1").parse::<i8>().unwrap(),
                lwn_ltow_week_number: args.get(4).unwrap_or(&"-1").parse::<i8>().unwrap(),
                lwn_ltow_tow: args.get(5).unwrap_or(&"-1").parse::<i8>().unwrap(),
                fcwn_fctow_week_number: args.get(6).unwrap_or(&"-1").parse::<i8>().unwrap(),
                fcwn_fctow_tow: args.get(7).unwrap_or(&"-1").parse::<i8>().unwrap(),
                lcwn_lctow_week_number: args.get(8).unwrap_or(&"-1").parse::<i8>().unwrap(),
                lcwn_lctow_tow: args.get(9).unwrap_or(&"-1").parse::<i8>().unwrap(),
            };
        }

        fn pmtk_127_cmd_clear_epo(&mut self) -> Pmtk001Ack {
            //! Multiple $CLR,EPO,{000a8000}*5E lines, ending with a 001 response.
            self.send_command("PMTK127");
            self.pmtk_001(50) // 50 should be plenty. Probably.
        }

        fn pmtk_397_set_nav_speed_threshold(&mut self, nav_threshold: f32) -> Pmtk001Ack {
            //! For MT3318 and MT3329 chips.
            self.send_command(format!("PMTK397,{:.1}", nav_threshold).as_str());
            self.pmtk_001(10)
        }

        fn pmtk_386_set_nav_speed_threshold(&mut self, nav_threshold: f32) -> Pmtk001Ack {
            //! For MT3339 chips.
            self.send_command(format!("PMTK397,{:.1}", nav_threshold).as_str());
            self.pmtk_001(10)
        }

        fn pmtk_447_q_nav_threshold(&mut self) -> f32 {
            //! $PMTK527,{0.40}*04
            self.send_command("PMTK447");
            return match self.pmtk_500() {
                Some(args) => {
                    let args: Vec<&str> = args.split(",").collect();
                    let nav_threshold: f32 = args.get(1).unwrap().parse::<f32>().unwrap();
                    nav_threshold
                }
                None => return -1.0
            };
        }

        fn pmtk_161_cmd_standby_mode(&mut self) -> Pmtk001Ack {
            self.send_command("PMTK161,0");
            self.pmtk_001(10)
        }

        fn pmtk_223_set_al_dee_cfg(&mut self, sv: i8, snr: i8, ext_threshold: i32, ext_gap: i32) -> Pmtk001Ack {
            //! Should be used with the PMTK225 command to set periodic mode.
            //!
            //! SV: Default 1, range 1-4. Increases the time to receive more ephemeris data while the
            //! number of satellites without ephemeris data is less than the SV value.
            //!
            //! SNR: Fedault 30, range 25-30. Enable receiving ephemeris data while the SNR of satellite
            //! is more than the value.
            //!
            //! Extention threshold (millisecond): default 180_000, range 40_000-180_000. The extension time
            //! for ephemeris data receiving.
            //!
            //! Extention gap: Default 60000, range 0-3_600_000
            //!
            //! Standard 001 response.
            self.send_command(format!("PMTK223,{},{},{},{}", sv, snr, ext_threshold, ext_gap).as_str());
            self.pmtk_001(10)
        }

        fn pmtk_225_cmd_periodic_mode(&mut self, run_type: u8, run_time: u32, sleep_time: u32,
                                      second_run_time: u32, second_sleep_time: u32) -> Pmtk001Ack {
            //! Enter standby or backup mode for power saving.
            //!
            //! PMTK225,Type,Run time,Sleep time, Second run time,Second sleep time
            //!
            //! run_type: operation mode
            //!     - ‘0’ = go back to normal mode
            //!     - ‘1’ = Periodic backup mode
            //!     - ‘2’ = Periodic standby mode
            //!     - ‘4’ = Perpetual backup mode
            //!     - ‘8’ = AlwaysLocateTM standby mode
            //!     - ‘9’ = AlwaysLocateTM backup mode
            //!
            //! Run time (millisecond): Duration to fix for (or attempt to fix for) before switching
            //! from running modeback to a minimum power sleep mode.
            //!     - '0’: disable
            //!     - >=’1,000’: enable Range: 1,000~518400000
            //!
            //! Sleep time (millisecond):Interval to come out of a minimum power sleep mode and start
            //! running in order to get a new position fix.
            //!     - ‘0’: disable
            //!     - >=’1,000’: enable Range: 1,000~518400000
            //!
            //! Second run time (millisecond): Duration to fix for (or attempt to fix for) before
            //! switching from running mode back to a minimum power sleep mode.
            //!     - ‘0’: disable
            //!     - >=’1,000’: enable Range: 1,000~518400000
            //!
            //! Second sleep time (millisecond): Interval to come out of a minimum power sleep mode and
            //! start running in order to get a new position fix.
            //!     - ‘0’: disable
            //!     - >=’1,000’: enable Range: 1,000~518400000
            //!
            //! Note：
            //! - 1.The second run time should larger than first run time when non-zero value.
            //! - 2.The purpose of second run time and sleep time can let module to catch more satellite
            //!     ephemeris data in cold boot condition. The value of them can be null. Then it will
            //!     use the first run time and sleep time for ephemeris data receiving.
            //! - 3.AlwaysLocateTM is an intelligent controller of MT3339 power saving mode. Depending on
            //!     the environment and motion conditions, MT3339 can adaptive adjust the on/off time
            //!     to achieve balance of positioning accuracy and power consumption.
            //! - 4.This command needs to work normal with some hardware circuits.
            //!
            self.send_command(format!("PMTK223,{},{},{},{},{}",
                                      run_type, run_time, sleep_time, second_run_time, second_sleep_time).as_str());
            self.pmtk_001(10)
        }

        fn pmtk_286_cmd_aic_mode(&mut self, aic: bool) -> Pmtk001Ack {
            //! true is enable, false is disable.
            if aic {
                self.send_command("PMTK286,1")
            } else {
                self.send_command("PMTK286,0")
            }
            self.pmtk_001(10)
        }

        fn pmtk_869_cmd_easy_enable(&mut self, enable_easy: bool) -> Pmtk001Ack {
            //! Enable or disable EASY function.
            //!
            //! Enabled by default.
            //!
            //! Requires VBACKUP pin to be connected to battery.
            //!
            //! Only valid for 1Hz update rate
            //!
            //! true is enable easy, false is disable.
            //!
            //! If you wish to query the EASY function, use pmtk_869_cmd_easy_query
            //! Response
            //!
            //! - pmtk,0 -> gives $PMTK869,2,1,3*29
            //! - pmtk,1,0 -> Gives 001 reply.
            //! - pmtk,2,{0,1} -> Gives 001 reply.
            if enable_easy {
                self.send_command("PMTK869,1,1")
            } else {
                self.send_command("PMTK869,1,0")
            }
            self.pmtk_001(10)
        }

        fn pmtk_869_cmd_easy_query(&mut self) -> bool {
            //! Query the EASY command status. Return true or false, true is enabled, false it disabled.
            self.send_command("PMTK869,0");
            return match self.pmtk_500() {
                Some(args) => {
                    let args: Vec<&str> = args.split(",").collect();
                    if args.get(2).unwrap() == &"0" {
                        false
                    } else {
                        true
                    }
                }
                None => true
            };
        }

        fn pmtk_187_locus_config(&mut self, locus_interval: i8) -> Pmtk001Ack {
            //! Locus mode (1 for interval mode) is always on.
            //! Interval, in seconds, is how often to log a data.
            self.send_command(format!("PMTK187,1,{}", locus_interval).as_str());
            self.pmtk_001(10)
        }

        fn pmtk_330_api_set_datum(&mut self, datum: u16) -> Pmtk001Ack {
            //! Configure Datum. 222 datum options.
            //!
            //! ‘0’ = WGS84
            //!
            //! ‘1’ = TOKYO-M
            //!
            //! ‘2’ = TOKYO-A
            //!
            //! A full list is on the GTOP Datum list, but I can't find it.
            self.send_command(format!("PMTK330,{}", datum).as_str());
            self.pmtk_001(10)
        }

        fn pmtk_430_api_q_datum(&mut self) -> u16 {
            //! Query current datum. Gives PMTK530,datum
            //! See pmtk_330_api_set_datum for more details on datum.
            //!
            //! 0 is return value if there is an error.
            self.send_command("PMTK430");
            return match self.pmtk_500() {
                Some(args) => {
                    let args: Vec<&str> = args.split(",").collect();
                    let datum = args.get(1).unwrap_or(&"0").parse::<u16>().unwrap();
                    datum
                }
                None => 0
            };
        }

        fn pmtk_351_api_set_support_qzss_nmea(&mut self, enable_qzss: bool) -> Pmtk001Ack {
            //! Sets the output to be the QZSS NMEA format.
            //!
            //! True is enable, false is disable. Default is disable.
            if enable_qzss {
                self.send_command("PMTK351,1")
            } else {
                self.send_command("PMTK351,0")
            }
            self.pmtk_001(10)
        }

        fn pmtk_352_api_set_stop_qzss(&mut self, enable: bool) -> Pmtk001Ack {
            //! Since QZSS is regional positioning service. The command allow user enable or disable QZSS function.
            //!
            //! Default is enable QZSS function
            //!
            //! Enable is true, disable is false. Default is enable.
            if enable {
                self.send_command("PMTK352,0")
            } else {
                self.send_command("PMTK352,1")
            }
            self.pmtk_001(10)
        }
    }
}

#[cfg(test)]
mod pmtktests {
    use std::thread::sleep;
    use std::time::Duration;

    use super::send_pmtk::{DgpsMode, EpoData, NmeaOutput, Pmtk001Ack, Sbas, SbasMode};
    use super::send_pmtk::{add_checksum, set_baud_rate};
    use super::send_pmtk::SendPmtk;
    use super::super::gps::{Gps, open_port};

    #[test]
    fn checksum() {
        assert_eq!(add_checksum("GNGGA,165419.000,5132.7378,N,00005.9192,W,1,7,1.93,34.4,M,47.0,M,,".to_string()), "$GNGGA,165419.000,5132.7378,N,00005.9192,W,1,7,1.93,34.4,M,47.0,M,,*6A\r\n".to_string());
        assert_eq!(add_checksum("PMTK103".to_string()), "$PMTK103*30\r\n")
    }

    fn port_setup() -> Gps {
        let port = open_port("/dev/serial0", 9600);
        let gps = Gps { port, satellite_data: true, naviagtion_data: true };
        sleep(Duration::from_secs(1));
        return gps;
    }
    #[test]
    fn reset_baud_rate() {
        set_baud_rate("9600", "/dev/serial0")
    }

    #[test]
    fn test_pmtk_101_cmd_hot_start() { assert_eq!(port_setup().pmtk_101_cmd_hot_start(), true); }

    #[test]
    fn test_pmtk_102_cmd_warm_start() { assert_eq!(port_setup().pmtk_102_cmd_warm_start(), true); }

    #[test]
    fn test_pmtk_103_cmd_cold_start() { assert_eq!(port_setup().pmtk_103_cmd_cold_start(), true); }

    #[test]
    fn test_pmtk_104_cmd_full_cold_start() { assert_eq!(port_setup().pmtk_104_cmd_full_cold_start(), true); }

    #[test]
    fn test_pmtk_220_set_nmea_updaterate() {
        assert_eq!(port_setup().pmtk_220_set_nmea_updaterate("1000"), Pmtk001Ack::Success);
        assert_eq!(port_setup().pmtk_220_set_nmea_updaterate("200"), Pmtk001Ack::Failed);
    }

    #[test]
    fn test_pmtk_301_api_set_dgps_mode() { assert_eq!(port_setup().pmtk_301_api_set_dgps_mode(DgpsMode::NoDgps), Pmtk001Ack::Success); }

    #[test]
    fn test_pmtk_401_api_q_dgps_mode() { assert_eq!(port_setup().pmtk_401_api_q_dgps_mode(), DgpsMode::WAAS); }

    #[test]
    fn test_pmtk_313_api_set_sbas_enabled() { assert_eq!(port_setup().pmtk_313_api_set_sbas_enabled(Sbas::Enabled), Pmtk001Ack::Success); }

    #[test]
    fn test_pmtk_413_api_q_sbas_enabled() { assert_eq!(port_setup().pmtk_413_api_q_sbas_enabled(), Sbas::Enabled); }

    #[test]
    // fn test_ () {assert_eq!(port_setup().pmtk_314_api_set_nm(gll: i8, rmc: i8, vtg: i8, gga: i8, gsa: i8, gsv: i8, pmtkchn_interval: i8), Pmtk001Ack::Success);}
    #[test]
    fn test_pmtk_414_api_q_nmea_output() {
        assert_eq!(port_setup().pmtk_414_api_q_nmea_output(), NmeaOutput {
            gll: 0,
            rmc: 1,
            vtg: 1,
            gga: 1,
            gsa: 1,
            gsv: 5,
            pmtkchn_interval: 0,
        });
    }

    #[test]
    fn test_pmtk_319_api_set_sbas_mode() { assert_eq!(port_setup().pmtk_319_api_set_sbas_mode(SbasMode::Integrity), true); }

    #[test]
    fn test_pmtk_419_api_q_sbas_mode() { assert_eq!(port_setup().pmtk_419_api_q_sbas_mode(), SbasMode::Integrity); }

    #[test]
    fn test_pmtk_605_q_release() { assert_eq!(port_setup().pmtk_605_q_release(), "AXN_5.1.7_3333_19020118,0027,PA1010D,1.0".to_string()); }

    #[test]
    fn test_pmtk_127_cmd_clear_epo() { assert_eq!(port_setup().pmtk_127_cmd_clear_epo(), Pmtk001Ack::Success); }

    #[test]
    fn test_pmtk_607_q_epo_info() {
        assert_eq!(port_setup().pmtk_607_q_epo_info(), EpoData {
            set: 0,
            fwn_ftow_week_number: 0,
            fwn_ftow_tow: 0,
            lwn_ltow_week_number: 0,
            lwn_ltow_tow: 0,
            fcwn_fctow_week_number: 0,
            fcwn_fctow_tow: 0,
            lcwn_lctow_week_number: 0,
            lcwn_lctow_tow: 0,
        });
    }

    #[test]
    fn test_pmtk_397_set_nav_speed_threshold() {
        assert_eq!(port_setup().pmtk_397_set_nav_speed_threshold(0.2), Pmtk001Ack::Success);
        assert_eq!(port_setup().pmtk_397_set_nav_speed_threshold(0.4), Pmtk001Ack::Success);
        assert_eq!(port_setup().pmtk_397_set_nav_speed_threshold(0.8), Pmtk001Ack::Success);
    }

    #[test]
    fn test_pmtk_386_set_nav_speed_threshold() { assert_eq!(port_setup().pmtk_386_set_nav_speed_threshold(0.2), Pmtk001Ack::Success); }

    #[test]
    fn test_pmtk_447_q_nav_threshold() { assert_eq!(port_setup().pmtk_447_q_nav_threshold(), 0.0); }

    // fn test_ () {assert_eq!(port_setup().pmtk_161_cmd_standby_mode(), Pmtk001Ack::Success);}
    #[test]
    fn test_pmtk_223_set_al_dee_cfg() { assert_eq!(port_setup().pmtk_223_set_al_dee_cfg(1, 30, 180000, 60000), Pmtk001Ack::Success); }

    // fn test_ () {assert_eq!(port_setup().pmtk_225_cmd_periodic_mode(run_type: u8, run_time: u32, sleep_time: u32,}
    //                                  second_run_time: u32, second_sleep_time: u32), Pmtk001Ack::Success);
    #[test]
    fn test_pmtk_286_cmd_aic_mode() { assert_eq!(port_setup().pmtk_286_cmd_aic_mode(true), Pmtk001Ack::Success); }

    #[test]
    fn test_pmtk_869_cmd_easy_enable() { assert_eq!(port_setup().pmtk_869_cmd_easy_enable(true), Pmtk001Ack::Success); }

    #[test]
    fn test_pmtk_869_cmd_easy_query() { assert_eq!(port_setup().pmtk_869_cmd_easy_query(), true); }

    // fn test_ () {assert_eq!(port_setup().pmtk_187_locus_config(locus_interval: i8), Pmtk001Ack::Success);}
    #[test]
    fn test_pmtk_330_api_set_datum() { assert_eq!(port_setup().pmtk_330_api_set_datum(0), Pmtk001Ack::Success); }

    #[test]
    fn test_pmtk_430_api_q_datum() { assert_eq!(port_setup().pmtk_430_api_q_datum(), 0); }

    #[test]
    fn test_pmtk_351_api_set_support_qzss_nmea() { assert_eq!(port_setup().pmtk_351_api_set_support_qzss_nmea(false), Pmtk001Ack::Success); }

    #[test]
    fn test_pmtk_352_api_set_stop_qzss() { assert_eq!(port_setup().pmtk_352_api_set_stop_qzss(true), Pmtk001Ack::Success); }
}

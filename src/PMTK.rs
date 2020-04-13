//! PMTK commands are ways of setting the gps.
//!
//! The PMTK001 command is the response given when there is a valid command given.
//!     It's format is $PMTK001,Command it was given, Flag response (0-3), value passed to it*checksum

#![allow(warnings)]

use std::fmt::Error;
use std::str;
use std::thread::sleep;
use std::time::Duration;

use serialport::SerialPort;

use crate::GetData;

use super::Gps;

pub trait SendPmtk {
    fn send_command(&mut self, cmd: &str, acknowledge: bool) -> PmtkAck;
    // Just send it. bool for ack. true if not wanting ack.
    fn pmtk_010_sys_msg(&mut self, msg: &str) -> PmtkAck;
    fn pmtk_011_txt_msg(&mut self, acknowledge: bool) -> PmtkAck;
    fn pmtk_101_cmd_hot_start(&mut self, acknowledge: bool) -> PmtkAck;
    fn pmtk_102_cmd_warm_start(&mut self, acknowledge: bool) -> PmtkAck;
    fn pmtk_103_cmd_cold_start(&mut self, acknowledge: bool) -> PmtkAck;
    fn pmtk_104_cmd_full_cold_start(&mut self, acknowledge: bool) -> PmtkAck;
    fn pmtk_220_set_nmea_updaterate(&mut self, update_rate: i32, acknowledge: bool) -> PmtkAck;
    fn pmtk_251_set_nmea_baudrate(&mut self, baud_rate: u32, acknowledge: bool) -> PmtkAck;
    fn pmtk_301_api_set_dgps_mode(&mut self, mode: u8, acknowledge: bool) -> PmtkAck;
    fn pmtk_401_api_q_dgps_mode(&mut self, acknowledge: bool) -> PmtkAck;
    fn pmtk_501_api_dt_dgps_mode(&mut self, acknowledge: bool) -> PmtkAck;
    fn pmtk_313_api_set_sbas_enabled(&mut self, acknowledge: bool) -> PmtkAck;
    fn pmtk_413_api_q_sbas_enabled(&mut self, acknowledge: bool) -> PmtkAck;
    fn pmtk_513_dt_sbas_enabled(&mut self, acknowledge: bool) -> PmtkAck;
    fn pmtk_314_api_set_nmea_output(&mut self, acknowledge: bool) -> PmtkAck;
    fn pmtk_414_api_q_nmea_output(&mut self, acknowledge: bool) -> PmtkAck;
    fn pmtk_514_api_dt_nmea_output(&mut self, acknowledge: bool) -> PmtkAck;
    fn pmtk_319_api_set_sbas_mode(&mut self, acknowledge: bool) -> PmtkAck;
    fn pmtk_419_api_q_sbas_mode(&mut self, acknowledge: bool) -> PmtkAck;
    fn pmtk_519_api_dt_sbas_mode(&mut self, acknowledge: bool) -> PmtkAck;
    fn pmtk_605_q_release(&mut self, acknowledge: bool) -> PmtkAck;
    fn pmtk_705_dt_release(&mut self, acknowledge: bool) -> PmtkAck;
    fn pmtk_607_q_epo_info(&mut self, acknowledge: bool) -> PmtkAck;
    fn pmtk_707_dt_epo_info(&mut self, acknowledge: bool) -> PmtkAck;
    fn pmtk_127_cmd_clear_epo(&mut self, acknowledge: bool) -> PmtkAck;
    fn pmtk_397_set_nav_speed_threshold(&mut self, acknowledge: bool) -> PmtkAck;
    fn pmtk_386_set_nav_speed_threshold(&mut self, acknowledge: bool) -> PmtkAck;
    fn pmtk_447_q_nav_threshold(&mut self, acknowledge: bool) -> PmtkAck;
    fn pmtk_527_dt_nav_threshold(&mut self, acknowledge: bool) -> PmtkAck;
    fn pmtk_161_cmd_standby_mode(&mut self, acknowledge: bool) -> PmtkAck;
    fn pmtk_223_set_al_dee_cfg(&mut self, acknowledge: bool) -> PmtkAck;
    fn pmtk_225_cmd_periodic_mode(&mut self, acknowledge: bool) -> PmtkAck;
    fn pmtk_286_cmd_aic_mode(&mut self, acknowledge: bool) -> PmtkAck;
    fn pmtk_869_cmd_easy_enable(&mut self, acknowledge: bool) -> PmtkAck;
    fn pmtk_187_locus_config(&mut self, acknowledge: bool) -> PmtkAck;
    fn pmtk_330_api_set_datum(&mut self, acknowledge: bool) -> PmtkAck;
    fn pmtk_430_api_q_datum(&mut self, acknowledge: bool) -> PmtkAck;
    fn pmtk_530_api_dt_datum(&mut self, acknowledge: bool) -> PmtkAck;
    fn pmtk_351_api_set_support_qzss_nmea(&mut self, acknowledge: bool) -> PmtkAck;
    fn pmtk_352_api_set_stop_qzss(&mut self, acknowledge: bool) -> PmtkAck;
}

pub enum PmtkAck {
    // format: $PMTK001,cmd,flag*checksum\r\n
    Invalid,
    //flag: 0
    Unsupported,
    //flag: 1
    Failed,
    //flag: 2
    Success,  //flag: 3
}

fn add_checksum(sentence: String) -> String {
    let mut checksum = 0;
    for char in sentence.as_bytes() {
        checksum ^= *char;
    }
    let checksum = format!("{:X}", checksum);  //Format as hexidecimal.
    let checksumed_sentence = format!("${}*{}\r\n", sentence, checksum).as_str().to_ascii_uppercase();
    return checksumed_sentence;
}


impl SendPmtk for Gps {
    #[allow(unused_must_use)]  // self.port.write is not used
    fn send_command(&mut self, cmd: &str, acknowledge: bool) -> PmtkAck {
        let cmd = add_checksum(cmd.to_string());
        let byte_cmd = cmd.as_bytes();

        if acknowledge {  // Clear buffer, write and then read.
            self.port.clear(serialport::ClearBuffer::Input);
            self.port.write(byte_cmd);
            loop {
                let line = self.read_line();
                dbg!(&line);
                if line.len() < 5 {
                    return PmtkAck::Invalid;
                }
                if &line[0..5] == "$PMTK" {
                    let args: Vec<&str> = line.split(",").collect();
                    let flag: &str = args.get(2).unwrap_or(&"0");
                    if flag == "0" {
                        return PmtkAck::Invalid;
                    } else if flag == "1" {
                        return PmtkAck::Unsupported;
                    } else if flag == "2" {
                        return PmtkAck::Failed;
                    } else if flag == "3" {
                        return PmtkAck::Success;
                    } else {
                        panic!("No valid flag output")
                    }
                } else {
                    continue;
                }
            }
        } else {
            self.port.write(byte_cmd);
            return PmtkAck::Success;
        }
    }

    fn pmtk_010_sys_msg(&mut self, msg: &str) -> PmtkAck {
        unimplemented!()
    }

    fn pmtk_011_txt_msg(&mut self, acknowledge: bool) -> PmtkAck {
        unimplemented!()
    }

    fn pmtk_101_cmd_hot_start(&mut self, acknowledge: bool) -> PmtkAck {
        //! Hot restart gps: use all data in NV store
        //!
        //! $PMTK011,MTKGPS*08\r\n" -> response.
        //! "$CDACK,7,0*49\r\n"
        self.send_command("PMTK101", acknowledge)
    }

    fn pmtk_102_cmd_warm_start(&mut self, acknowledge: bool) -> PmtkAck {
        //! Warm restart gps: Dont use Ephemeris at re-start.
        self.send_command("PMTK102", acknowledge)
    }

    fn pmtk_103_cmd_cold_start(&mut self, acknowledge: bool) -> PmtkAck {
        //! Cold restart gps: Don't use time, position, almanacs or Ephemeris data to restart.
        self.send_command("PMTK103", acknowledge)
    }

    fn pmtk_104_cmd_full_cold_start(&mut self, acknowledge: bool) -> PmtkAck {
        //! Full restart gps: All systems, configs are reset. Basically factory reset.
        self.send_command("PMTK104", acknowledge)
    }

    fn pmtk_220_set_nmea_updaterate(&mut self, update_rate: i32, acknowledge: bool) -> PmtkAck {
        //! Set NMEA port update rate. Range is 100 to 10_000 miliseconds.
        if (update_rate <= 100) | (update_rate >= 10000) {
            eprintln!("update rate outside of range 100-10000. Setting to 1000 default");
            self.send_command("PMTK220,1000", acknowledge)
        } else {
            let update_rate = update_rate.to_string();
            self.send_command(format!("PMTK220,{}", update_rate).as_str(), acknowledge)
        }
    }

    fn pmtk_251_set_nmea_baudrate(&mut self, baud_rate: u32, acknowledge: bool) -> PmtkAck {
        //! Set NMEA port baud rate: Setting are: 4800,9600,14400,19200,38400,57600,115200
        if (baud_rate != 4800) | (baud_rate != 9600) | (baud_rate != 14400) | (baud_rate != 19200) |
            (baud_rate != 38400) | (baud_rate != 57600) | (baud_rate != 115200) {
            eprint!("Invalid baudrate given. Setting to default.");
            self.send_command("PMTK251,0", acknowledge)  // 0 makes it defualt.
        } else {
            let baud_rate = baud_rate.to_string();
            self.send_command(format!("PMTK251,{}", baud_rate).as_str(), acknowledge)
        }
    }

    fn pmtk_301_api_set_dgps_mode(&mut self, mode: u8, acknowledge: bool) -> PmtkAck {
        //! Set DGPS correction data source mode.
        //! Note: If you wish to set DGPS mode to RTCM, please use PMTK250 first to
        //! set RTCM baud rate before using this command.
        if (mode > 2) | (mode < 0) {
            self.send_command("PMTK301,0", acknowledge)
        } else {
            let mode = mode.to_string();
            self.send_command(format!("PMTK301,{}", mode).as_str(), acknowledge)
        }
    }

    fn pmtk_401_api_q_dgps_mode(&mut self, acknowledge: bool) -> PmtkAck {
        unimplemented!()
    }

    fn pmtk_501_api_dt_dgps_mode(&mut self, acknowledge: bool) -> PmtkAck {
        unimplemented!()
    }

    fn pmtk_313_api_set_sbas_enabled(&mut self, acknowledge: bool) -> PmtkAck {
        unimplemented!()
    }

    fn pmtk_413_api_q_sbas_enabled(&mut self, acknowledge: bool) -> PmtkAck {
        unimplemented!()
    }

    fn pmtk_513_dt_sbas_enabled(&mut self, acknowledge: bool) -> PmtkAck {
        unimplemented!()
    }

    fn pmtk_314_api_set_nmea_output(&mut self, acknowledge: bool) -> PmtkAck {
        unimplemented!()
    }

    fn pmtk_414_api_q_nmea_output(&mut self, acknowledge: bool) -> PmtkAck {
        unimplemented!()
    }

    fn pmtk_514_api_dt_nmea_output(&mut self, acknowledge: bool) -> PmtkAck {
        unimplemented!()
    }

    fn pmtk_319_api_set_sbas_mode(&mut self, acknowledge: bool) -> PmtkAck {
        unimplemented!()
    }

    fn pmtk_419_api_q_sbas_mode(&mut self, acknowledge: bool) -> PmtkAck {
        unimplemented!()
    }

    fn pmtk_519_api_dt_sbas_mode(&mut self, acknowledge: bool) -> PmtkAck {
        unimplemented!()
    }

    fn pmtk_605_q_release(&mut self, acknowledge: bool) -> PmtkAck {
        unimplemented!()
    }

    fn pmtk_705_dt_release(&mut self, acknowledge: bool) -> PmtkAck {
        unimplemented!()
    }

    fn pmtk_607_q_epo_info(&mut self, acknowledge: bool) -> PmtkAck {
        unimplemented!()
    }

    fn pmtk_707_dt_epo_info(&mut self, acknowledge: bool) -> PmtkAck {
        unimplemented!()
    }

    fn pmtk_127_cmd_clear_epo(&mut self, acknowledge: bool) -> PmtkAck {
        unimplemented!()
    }

    fn pmtk_397_set_nav_speed_threshold(&mut self, acknowledge: bool) -> PmtkAck {
        unimplemented!()
    }

    fn pmtk_386_set_nav_speed_threshold(&mut self, acknowledge: bool) -> PmtkAck {
        unimplemented!()
    }

    fn pmtk_447_q_nav_threshold(&mut self, acknowledge: bool) -> PmtkAck {
        unimplemented!()
    }

    fn pmtk_527_dt_nav_threshold(&mut self, acknowledge: bool) -> PmtkAck {
        unimplemented!()
    }

    fn pmtk_161_cmd_standby_mode(&mut self, acknowledge: bool) -> PmtkAck {
        unimplemented!()
    }

    fn pmtk_223_set_al_dee_cfg(&mut self, acknowledge: bool) -> PmtkAck {
        unimplemented!()
    }

    fn pmtk_225_cmd_periodic_mode(&mut self, acknowledge: bool) -> PmtkAck {
        unimplemented!()
    }

    fn pmtk_286_cmd_aic_mode(&mut self, acknowledge: bool) -> PmtkAck {
        unimplemented!()
    }

    fn pmtk_869_cmd_easy_enable(&mut self, acknowledge: bool) -> PmtkAck {
        unimplemented!()
    }

    fn pmtk_187_locus_config(&mut self, acknowledge: bool) -> PmtkAck {
        unimplemented!()
    }

    fn pmtk_330_api_set_datum(&mut self, acknowledge: bool) -> PmtkAck {
        unimplemented!()
    }

    fn pmtk_430_api_q_datum(&mut self, acknowledge: bool) -> PmtkAck {
        unimplemented!()
    }

    fn pmtk_530_api_dt_datum(&mut self, acknowledge: bool) -> PmtkAck {
        unimplemented!()
    }

    fn pmtk_351_api_set_support_qzss_nmea(&mut self, acknowledge: bool) -> PmtkAck {
        unimplemented!()
    }

    fn pmtk_352_api_set_stop_qzss(&mut self, acknowledge: bool) -> PmtkAck {
        unimplemented!()
    }
}

#[cfg(test)]
mod pmtktests {
    use super::add_checksum;
    #[test]
    fn checksum() {
        assert_eq!(add_checksum("GNGGA,165419.000,5132.7378,N,00005.9192,W,1,7,1.93,34.4,M,47.0,M,,".to_string()), "$GNGGA,165419.000,5132.7378,N,00005.9192,W,1,7,1.93,34.4,M,47.0,M,,*6A\r\n".to_string());
        assert_eq!(add_checksum("PMTK103".to_string()), "$PMTK103*30\r\n")
    }
}
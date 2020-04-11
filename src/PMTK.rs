
use super::Gps;
use serialport::SerialPort;
use std::fmt::Error;
use crate::GetData;

enum Pmtk {
    Invalid {f: u8},
    Unsupported {f: u8},
    Failed {f: u8},
    Success {f: u8},
}

pub trait SendPmtk {
    fn send_command(&self, cmd:&str);  // Just send it
    fn ack_command(&self) -> bool;
    fn add_checksum(&self, sentence:String) -> String;
    fn pmtk_010_sys_msg(&self, msg:&str);
    fn pmtk_011_txt_msg(&self, );
    fn pmtk_101_cmd_hot_start(&self, );
    fn pmtk_102_cmd_warm_start(&self, );
    fn pmtk_103_cmd_cold_start(&self, );
    fn pmtk_104_cmd_full_cold_start(&self, );
    fn pmtk_220_set_nmea_updaterate(&self, );
    fn pmtk_251_set_nmea_baudrate(&self, );
    fn pmtk_301_api_set_dgps_mode(&self, );
    fn pmtk_401_api_q_dgps_mode(&self, );
    fn pmtk_501_api_dt_dgps_mode(&self, );
    fn pmtk_313_api_set_sbas_enabled(&self, );
    fn pmtk_413_api_q_sbas_enabled(&self, );
    fn pmtk_513_dt_sbas_enabled(&self, );
    fn pmtk_314_api_set_nmea_output(&self, );
    fn pmtk_414_api_q_nmea_output(&self, );
    fn pmtk_514_api_dt_nmea_output(&self, );
    fn pmtk_319_api_set_sbas_mode(&self, );
    fn pmtk_419_api_q_sbas_mode(&self, );
    fn pmtk_519_api_dt_sbas_mode(&self, );
    fn pmtk_605_q_release(&self, );
    fn pmtk_705_dt_release(&self, );
    fn pmtk_607_q_epo_info(&self, );
    fn pmtk_707_dt_epo_info(&self, );
    fn pmtk_127_cmd_clear_epo(&self, );
    fn pmtk_397_set_nav_speed_threshold(&self, );
    fn pmtk_386_set_nav_speed_threshold(&self, );
    fn pmtk_447_q_nav_threshold(&self, );
    fn pmtk_527_dt_nav_threshold(&self, );
    fn pmtk_161_cmd_standby_mode(&self, );
    fn pmtk_223_set_al_dee_cfg(&self, );
    fn pmtk_225_cmd_periodic_mode(&self, );
    fn pmtk_286_cmd_aic_mode(&self, );
    fn pmtk_869_cmd_easy_enable(&self, );
    fn pmtk_187_locus_config(&self, );
    fn pmtk_330_api_set_datum(&self, );
    fn pmtk_430_api_q_datum(&self, );
    fn pmtk_530_api_dt_datum(&self, );
    fn pmtk_351_api_set_support_qzss_nmea(&self, );
    fn pmtk_352_api_set_stop_qzss(&self, );
}

impl SendPmtk for Gps {
    #[allow(unused_must_use)]  // self.port.write is not used at the end.
    fn send_command(&mut self, cmd: &str) {  // Take the full sentence, convert to
        let byte_cmd = cmd.as_bytes();
        self.port.write(byte_cmd);
        self.read_line()
    }

    fn add_checksum(&self, sentence: String) -> String {
        let mut checksum = 0;
        for char in sentence.as_bytes() {
            checksum ^= *char;
        }
        let checksum = format!("{:X}", checksum);  //Format as hexidecimal.
        let checksumed_sentence = format!("{}*{}\r\n", sentence, checksum).as_str().to_ascii_uppercase();
        return checksumed_sentence;
    }

    fn pmtk_010_sys_msg(&self, msg:&str) {
        struct SysMessages {
            Unknown

        }
        format!("$PMTK010")
    }
    fn pmtk_011_txt_msg(&self, ) {
        let mut sentence = format!("$PMTK001,{},{}", cmd, flag);
        sentence = self.add_checksum(sentence);
        self.send_command(sentence.as_str());
    }

    fn pmtk_101_cmd_hot_start(&self) {
        unimplemented!()
    }

    fn pmtk_102_cmd_warm_start(&self) {
        unimplemented!()
    }

    fn pmtk_103_cmd_cold_start(&self) {
        unimplemented!()
    }

    fn pmtk_104_cmd_full_cold_start(&self) {
        unimplemented!()
    }

    fn pmtk_220_set_nmea_updaterate(&self) {
        unimplemented!()
    }

    fn pmtk_251_set_nmea_baudrate(&self) {
        unimplemented!()
    }

    fn pmtk_301_api_set_dgps_mode(&self) {
        unimplemented!()
    }

    fn pmtk_401_api_q_dgps_mode(&self) {
        unimplemented!()
    }

    fn pmtk_501_api_dt_dgps_mode(&self) {
        unimplemented!()
    }

    fn pmtk_313_api_set_sbas_enabled(&self) {
        unimplemented!()
    }

    fn pmtk_413_api_q_sbas_enabled(&self) {
        unimplemented!()
    }

    fn pmtk_513_dt_sbas_enabled(&self) {
        unimplemented!()
    }

    fn pmtk_314_api_set_nmea_output(&self) {
        unimplemented!()
    }

    fn pmtk_414_api_q_nmea_output(&self) {
        unimplemented!()
    }

    fn pmtk_514_api_dt_nmea_output(&self) {
        unimplemented!()
    }

    fn pmtk_319_api_set_sbas_mode(&self) {
        unimplemented!()
    }

    fn pmtk_419_api_q_sbas_mode(&self) {
        unimplemented!()
    }

    fn pmtk_519_api_dt_sbas_mode(&self) {
        unimplemented!()
    }

    fn pmtk_605_q_release(&self) {
        unimplemented!()
    }

    fn pmtk_705_dt_release(&self) {
        unimplemented!()
    }

    fn pmtk_607_q_epo_info(&self) {
        unimplemented!()
    }

    fn pmtk_707_dt_epo_info(&self) {
        unimplemented!()
    }

    fn pmtk_127_cmd_clear_epo(&self) {
        unimplemented!()
    }

    fn pmtk_397_set_nav_speed_threshold(&self) {
        unimplemented!()
    }

    fn pmtk_386_set_nav_speed_threshold(&self) {
        unimplemented!()
    }

    fn pmtk_447_q_nav_threshold(&self) {
        unimplemented!()
    }

    fn pmtk_527_dt_nav_threshold(&self) {
        unimplemented!()
    }

    fn pmtk_161_cmd_standby_mode(&self) {
        unimplemented!()
    }

    fn pmtk_223_set_al_dee_cfg(&self) {
        unimplemented!()
    }

    fn pmtk_225_cmd_periodic_mode(&self) {
        unimplemented!()
    }

    fn pmtk_286_cmd_aic_mode(&self) {
        unimplemented!()
    }

    fn pmtk_869_cmd_easy_enable(&self) {
        unimplemented!()
    }

    fn pmtk_187_locus_config(&self) {
        unimplemented!()
    }

    fn pmtk_330_api_set_datum(&self) {
        unimplemented!()
    }

    fn pmtk_430_api_q_datum(&self) {
        unimplemented!()
    }

    fn pmtk_530_api_dt_datum(&self) {
        unimplemented!()
    }

    fn pmtk_351_api_set_support_qzss_nmea(&self) {
        unimplemented!()
    }

    fn pmtk_352_api_set_stop_qzss(&self) {
        unimplemented!()
    }
}



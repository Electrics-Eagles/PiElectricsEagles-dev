extern crate ini;

use ini::Ini;
use std::num::ParseIntError;

static file_path: &str = "/etc/pielectricseagles/config.ini";
//Converts &str to i32

pub struct config_parser {
    config_parser: ini::Ini ,
}
pub struct SbusConfig {
    pub baudrate: u32,
    pub parity: u32,
    pub data_bits: u8,
    pub stop_bit: u8,
    pub port: String,
}

pub struct Mpu6050Conifg {
    pub port: String,
    pub sample_amount: u8,
}

pub struct EscMotors {
    pub driver: String,
    pub port: String,
}

pub struct PID {
    pub p: f64,
    pub i: f64,
    pub d: f64,
    pub max: f64,
}

pub struct PIDS {
    pub roll: PID,
    pub pitch: PID,
    pub yaw: PID,
}

impl config_parser {
    pub fn new() -> config_parser {
        let conf = Ini::load_from_file(file_path).unwrap();
        config_parser {config_parser:conf}
    }

    pub fn parse(&mut self,first_number_str: &str) -> Result<i32, ParseIntError> {
        let value = first_number_str.parse::<i32>()?;
        Ok(value)
    }

    pub fn parse_u8(&mut self,first_number_str: &str) -> Result<u8, ParseIntError> {
        let value = first_number_str.parse::<u8>()?;
        Ok(value)
    }

    pub fn parse_u32(&mut self,first_number_str: &str) -> Result<u32, ParseIntError> {
        let value = first_number_str.parse::<u32>()?;
        Ok(value)
    }

    pub fn parse_f64(&mut self,first_number_str: &str) -> Result<f64, ParseIntError> {
        let value = first_number_str.parse::<f64>();
        Ok(value.unwrap())
    }

    pub fn config_parser_version(&mut self) -> &'static str {
        return "CONFIG PARSER VERSION V2.0.3 Alpha 22/12/2020";
    }

    pub fn esc_config_parser(&mut self) -> EscMotors {
        let esc_config =self.config_parser.section(Some("esc-config")).unwrap();
        let driver = (esc_config.get("driver")).unwrap();
        let port = (esc_config.get("port")).unwrap();
        let esc_motors_val = EscMotors {
            driver: driver.parse().unwrap(),
            port: port.parse().unwrap(),
          
        };
        return esc_motors_val;
    }
    pub fn mpu_config_parser(&mut self) -> Mpu6050Conifg {
        let conf = Ini::load_from_file(file_path).unwrap();
        let mpu_config = conf.section(Some("mpu6050")).unwrap();
        let sample = self.parse_u8((mpu_config.get("sample")).unwrap()).unwrap();
        let port = (mpu_config.get("port")).unwrap();
        let mpu6050_config = Mpu6050Conifg {
            port: port.parse().unwrap(),
            sample_amount: sample,
        };
        return mpu6050_config;
    }
    pub fn ibus_receiver_conifg(&mut self) -> SbusConfig {
        let conf = Ini::load_from_file(file_path).unwrap();
        let sbus_config = conf.section(Some("ibus_config")).unwrap();
        let baudrate = self.parse_u32((sbus_config.get("baudrate")).unwrap()).unwrap();
        let parity = self.parse_u32((sbus_config.get("parity")).unwrap()).unwrap();
        let data_bits = self.parse_u8((sbus_config.get("data_bits")).unwrap()).unwrap();
        let stop_bit = self.parse_u8((sbus_config.get("stop_bit")).unwrap()).unwrap();
        let port = sbus_config.get("port").unwrap();
        let sbus = SbusConfig {
            baudrate: baudrate,
            parity: parity,
            data_bits: data_bits,
            stop_bit: stop_bit,
            port: port.to_string(),
        };
        return sbus;
    }

    pub fn pid_parse(
        &mut self,
        section: String,
        p_str: String,
        i_str: String,
        d_str: String,
        max: String,
        
    ) -> PID {
        let conf = Ini::load_from_file(file_path).unwrap();
        let pid_configuration = conf.section(Some(section)).unwrap();
        let p_pid = (pid_configuration.get(p_str)).unwrap();
        let i_pid = pid_configuration.get(i_str).unwrap();
        let d_pid = pid_configuration.get(d_str).unwrap();
        let max = pid_configuration.get(max).unwrap();
        let f64_p_pid = self.parse_f64(p_pid).unwrap();
        let f64_i_pid = self.parse_f64(i_pid).unwrap();
        let f64_d_pid = self.parse_f64(d_pid).unwrap();
        let f64_max = self.parse_f64(max).unwrap();

        let pid_config: PID = PID {
            p: f64_p_pid,
            i: f64_i_pid,
            d: f64_d_pid,
            max: f64_max,
        };
        return pid_config;
    }

    pub fn get_pids(&mut self) -> PIDS {
        let roll = self.pid_parse(
            "pid-config".parse().unwrap(),
            "p_pid_roll".parse().unwrap(),
            "i_pid_roll".parse().unwrap(),
            "d_pid_roll".parse().unwrap(),
            "max_roll".parse().unwrap(),
        );
        let yaw = self.pid_parse(
            "pid-config".parse().unwrap(),
            "p_pid_yaw".parse().unwrap(),
            "i_pid_yaw".parse().unwrap(),
            "d_pid_yaw".parse().unwrap(),
            "max_yaw".parse().unwrap(),
        );
        let pitch = self.pid_parse(
            "pid-config".parse().unwrap(),
            "p_pid_pitch".parse().unwrap(),
            "i_pid_pitch".parse().unwrap(),
            "d_pid_pitch".parse().unwrap(),
            "max_pitch".parse().unwrap(),
        );

        let pid_config: PIDS = PIDS {
            roll: roll,
            yaw: yaw,
            pitch: pitch,
        };
        return pid_config;
    }

    pub fn auto_level_config(&mut self) -> i32 {
        let autoconfig =self.config_parser.section(Some("auto-level")).unwrap();
        let amount = (autoconfig.get("autolevel")).unwrap().to_string();
        if amount.eq("on") {
            return 1;
        } else {
            return 0;
        }
    }
}

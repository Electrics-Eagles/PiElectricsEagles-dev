extern crate ini;

use ini::Ini;
use std::num::ParseIntError;

//Converts &str to i32
fn parse(first_number_str: &str) -> Result<i32, ParseIntError> {
    let value = first_number_str.parse::<i32>()?;
    Ok(value)
}

fn parse_u8(first_number_str: &str) -> Result<u8, ParseIntError> {
    let value = first_number_str.parse::<u8>()?;
    Ok(value)
}

fn parse_u32(first_number_str: &str) -> Result<u32, ParseIntError> {
    let value = first_number_str.parse::<u32>()?;
    Ok(value)
}

pub fn config_parser_version() -> &'static str {
    return "CONFIG PARSER VERSION V2.0.2 Alpha 11/10/2020";
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
    pub amount: u8,
}

pub struct PID {
    pub p: i32,
    pub i: i32,
    pub d: i32,
    pub max: i32,
}

pub struct PIDS {
    pub roll: PID,
    pub pitch: PID,
    pub yaw: PID,
}

pub fn esc_config_parser() -> EscMotors {
    let conf = Ini::load_from_file("./src/config/core.ini").unwrap();

    let esc_config = conf.section(Some("esc-config")).unwrap();
    let amount = parse_u8((esc_config.get("amount")).unwrap()).unwrap();
    let driver = (esc_config.get("driver")).unwrap();
    let port = (esc_config.get("port")).unwrap();
    let esc_motors_val = EscMotors {
        driver: driver.parse().unwrap(),
        port: port.parse().unwrap(),
        amount: amount,
    };
    return esc_motors_val;
}
pub fn mpu_config_parser() -> Mpu6050Conifg {
    let conf = Ini::load_from_file("./src/config/core.ini").unwrap();
    let mpu_config = conf.section(Some("mpu6050")).unwrap();
    let sample = parse_u8((mpu_config.get("sample")).unwrap()).unwrap();
    let port = (mpu_config.get("port")).unwrap();
    let mpu6050_config = Mpu6050Conifg {
        port: port.parse().unwrap(),
        sample_amount: sample,
    };
    return mpu6050_config;
}
pub fn sbus_receiver_conifg() -> SbusConfig {
    let conf = Ini::load_from_file("./src/config/core.ini").unwrap();
    let sbus_config = conf.section(Some("sbus_config")).unwrap();
    let baudrate = parse_u32((sbus_config.get("baudrate")).unwrap()).unwrap();
    let parity = parse_u32((sbus_config.get("parity")).unwrap()).unwrap();
    let data_bits = parse_u8((sbus_config.get("data_bits")).unwrap()).unwrap();
    let stop_bit = parse_u8((sbus_config.get("stop_bit")).unwrap()).unwrap();
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

pub fn pid_parse(section: String, p_str: String, i_str: String, d_str: String, max: String) -> PID {
    let conf = Ini::load_from_file("./src/config/core.ini").unwrap();
    let pid_configuration = conf.section(Some(section)).unwrap();
    let p_pid = (pid_configuration.get(p_str)).unwrap();
    let i_pid = pid_configuration.get(i_str).unwrap();
    let d_pid = pid_configuration.get(d_str).unwrap();
    let max = pid_configuration.get(max).unwrap();
    let i32_p_pid = parse(p_pid).unwrap();
    let i32_i_pid = parse(i_pid).unwrap();
    let i32_d_pid = parse(d_pid).unwrap();
    let i32_max = parse(max).unwrap();

    let pid_config: PID = PID {
        p: i32_p_pid,
        i: i32_i_pid,
        d: i32_d_pid,
        max: i32_max,
    };
    return pid_config;
}

pub fn get_pids() -> PIDS {
    let roll = pid_parse(
        "pid-config".parse().unwrap(),
        "p_pid_roll".parse().unwrap(),
        "i_pid_roll".parse().unwrap(),
        "d_pid_roll".parse().unwrap(),
        "max_roll".parse().unwrap(),
    );
    let yaw = pid_parse(
        "pid-config".parse().unwrap(),
        "p_pid_yaw".parse().unwrap(),
        "i_pid_yaw".parse().unwrap(),
        "d_pid_yaw".parse().unwrap(),
        "max_yaw".parse().unwrap(),
    );
    let pitch = pid_parse(
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

pub fn auto_level_config() -> i32 {
    let conf = Ini::load_from_file("./src/config/core.ini").unwrap();
    let autoconfig = conf.section(Some("auto-level")).unwrap();
    let amount = (autoconfig.get("autolevel")).unwrap().to_string();
    if amount.eq("on") {
        return 1;
    } else {
        return 0;
    }
}

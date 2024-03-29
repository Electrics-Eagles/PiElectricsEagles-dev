#[allow(non_camel_case_types)]
//
//
// Alex Zaslavkis (Electrics Eagles) 2021
//
// ---------------- Configuration parse --------------------
//
// Simple software level that parse own configuration for drone setting that retrieve information about PID-setting, interface and others
//
// typical usage :
//
//
//
//
//Enjoy
extern crate ini;
use ini::Ini;
use std::num::ParseIntError;
/// #Static
/// This static string (std::str) is file path to storage of own configiration log.
/// This filename is config.ini
/// You can change configaruation of drone.
static file_path: &str = "/etc/pielectricseagles/config.ini";

/// It is a config_parser object
/// config_parser class (crate)
///
/// Made by: Alex Zaslavskis (Electrics Eagles) 2021
///
/// Simple software level that parse own configuration for drone setting that retrieve information about PID-setting, interface and others
/// # Examples
/// **** Already added to loggics file. Be careful. Editing code can break stability of devices. *****
///
/// ```
/// use crate::config_parse::*;
/// let mut config = config_parser::new(); // create object of config_parse
/// let autolevel = config.auto_level_config(); // getting value of auto level
/// let pid_settings = config.get_pids(); // getting object value of pid-controller of all section
/// let mpu6050_conifg = config.mpu_config_parser(); // getting object value of configuartion for MPU6050 sensor
/// let esc_config = config.esc_config_parser(); // getting object value of configuration for esc driver
/// let version_config: &'static str = config.config_parser_version(); // getting text about version of config parser
/// ```
///
pub struct config_parser {
    config_parser: ini::Ini,
}

pub struct ArmSwitchConfig {
    pub arm_switch_1: usize,
    pub arm_switch_2: usize,
}
/// It is a UART/Sbus/Ibus configuration struct
pub struct IbusConfig {
    /// baudrate (u32 value)
    pub baudrate: u32,
    /// Port name to connect UART/Sbus/Ibus (String value)
    pub port: String,
}
/// It is MPU6050 gyroscope configuration struct
pub struct ImuConfig {
    /// Name port of connecting MPU6050 via I2C interface (String value)
    pub port: String,
    pub axis_assignment_gyro: String,
    pub axis_assignment_acc: String,
    pub reversed_axis_gyro: String,
    pub reversed_axis_acc: String,
}
/// It is esc driver configuration struct
pub struct EscMotors {
    /// Name port of connecting ESC driver via I2C inferface (String value)
    pub port: String,
}
/// It is PID - configuration struct for each type
#[derive(Debug, Copy, Clone)]
pub struct PID {
    /// Gain of proportional
    pub p: f64,
    /// Gain of integral
    pub i: f64,
    /// Gain of  derivative
    pub d: f64,
    /// Maximun output of PID-controller
    pub max: f64,
}
/// It is PID - configuration struct for all type of controls
#[derive(Debug, Copy, Clone)]
pub struct PIDS {
    /// PID of roll control
    pub roll: PID,
    /// PID of pitch control
    pub pitch: PID,
    /// PID of yaw control
    pub yaw: PID,
}

pub struct Filter_Val {
    pub conf_a: f64,
    pub conf_b: f64,
}

pub struct Reciver {
    pub uart_port: String,
    pub baudrate: u32,
}
/// It is LIPO battery controller configuration struct
pub struct BatteryController {
    /// i2c selected port (example /dev/i2c-4)
    pub port: String,
    /// Maximum range voltage to measure common cell of battery (from 1st to 3rd cell of LIPO battery)
    pub max_vol_ch0: f32,
    /// Maximum range voltage to measure common 2nd cell of battery (from 1st to 2nd cell of LIPO battery)
    pub max_vol_ch1: f32,
    /// Maximum range voltage to measure 1st cell of battery
    pub max_vol_ch2: f32,
}

impl config_parser {
    /// Returns config_parser object
    ///
    /// # Arguments
    ///
    /// No arguments required
    ///
    /// # Return
    /// ```config_parser```
    ///
    /// # Example
    /// ```
    /// use crate::config_parse::*;
    /// let mut config = config_parser::new(); // create object of config_parse
    /// ```
    /// **** Already added to loggics file. Be careful. Editing code can break stability of devices. *****
    ///
    pub fn new() -> config_parser {
        let conf = Ini::load_from_file(file_path).unwrap();
        config_parser {
            config_parser: conf,
        }
    }
    /// Parse i32 value from &str value
    ///
    /// # Arguments
    ///
    /// ```first_number_str``` is value with type &str (std:str)
    ///
    /// # Return
    /// ```Result<i32, ParseIntError>```
    ///
    /// # Example
    /// **** Already added to loggics file. Be careful. Editing code can break stability of devices. *****
    ///
    ///

    /// Parse u8 value from &str value
    ///
    /// # Arguments
    ///
    /// ```first_number_str``` is value with type &str (std:str)
    ///
    /// # Return
    /// ```Result<u8, ParseIntError>```
    ///
    /// # Example
    /// **** Already added to loggics file. Be careful. Editing code can break stability of devices. *****
    /// ```
    /// let mut config = config_parser::new(); // create object of config_parse
    /// let val_u8: u8 = config.parse("55").unwrap(); // getting value u8 from string with numberic value
    /// ```
    ///

    /// Parse u32 value from &str value
    ///
    /// # Arguments
    ///
    /// ```first_number_str``` is value with type &str (std:str)
    ///
    /// # Return
    /// ```Result<u32, ParseIntError>```
    ///
    /// # Example
    /// **** Already added to loggics file. Be careful. Editing code can break stability of devices. *****
    /// ```
    /// let mut config = config_parser::new(); // create object of config_parse
    /// let val_u32: u32 = config.parse("55").unwrap(); // getting value u32 from string with numberic value
    /// ```
    ///

    /// Parse f64 value from &str value
    ///
    /// # Arguments
    ///
    /// ```first_number_str``` is value with type &str (std:str)
    ///
    /// # Return
    /// ```Result<f64, ParseIntError>```
    ///
    /// # Example
    /// ```
    /// let mut config = config_parser::new(); // create object of config_parse
    /// let val_f64: f64 = config.parse("5.51").unwrap(); // getting value f64 from string with numberic value
    /// ```
    /// **** Already added to loggics file. Be careful. Editing code can break stability of devices. *****
    ///
    /// Show version of configuration parser
    ///
    /// # Arguments
    ///
    /// No arguments required
    ///
    /// # Return
    /// ```&'static str```
    ///
    /// # Example
    /// ```
    /// use crate::config_parse::*;
    /// let mut config = config_parser::new(); // create object of config_parse
    /// let version_config: &'static str = config.config_parser_version();
    /// println!("{}", version_config); // print that shows version into console
    /// ```
    /// **** Already added to loggics file. Be careful. Editing code can break stability of devices. *****
    ///

    /// Return data of esc motor configuration from pasring ini-file
    ///
    /// # Arguments
    ///
    /// No arguments required
    ///
    /// # Return
    /// ```EscMotors```

    ///
    /*
    pub fn parse(&mut self, first_number_str: &str) -> Result<i32, ParseIntError> {
        let value = first_number_str.parse::<i32>()?;
        Ok(value)
    }
     */
    /// Parse u8 value from &str value
    ///
    /// # Arguments
    ///
    /// ```first_number_str``` is value with type &str (std:str)
    ///
    /// # Return
    /// ```Result<u8, ParseIntError>```
    ///
    /// # Example
    /// **** Already added to loggics file. Be careful. Editing code can break stability of devices. *****
    /// ```
    /// let mut config = config_parser::new(); // create object of config_parse
    /// let val_u8: u8 = config.parse("55").unwrap(); // getting value u8 from string with numberic value
    /// ```
    ///

    pub fn parse_u8(&mut self, first_number_str: &str) -> Result<u8, ParseIntError> {
        let value = first_number_str.parse::<u8>()?;
        Ok(value)
    }

    /// Parse u32 value from &str value
    ///
    /// # Arguments
    ///
    /// ```first_number_str``` is value with type &str (std:str)
    ///
    /// # Return
    /// ```Result<u32, ParseIntError>```
    ///
    /// # Example
    /// **** Already added to loggics file. Be careful. Editing code can break stability of devices. *****
    /// ```
    /// let mut config = config_parser::new(); // create object of config_parse
    /// let val_u32: u32 = config.parse("55").unwrap(); // getting value u32 from string with numberic value
    /// ```
    ///

    pub fn parse_u32(&mut self, first_number_str: &str) -> Result<u32, ParseIntError> {
        let value = first_number_str.parse::<u32>()?;
        Ok(value)
    }

    /// Parse f64 value from &str value
    ///
    /// # Arguments
    ///
    /// ```first_number_str``` is value with type &str (std:str)
    ///
    /// # Return
    /// ```Result<f64, ParseIntError>```
    ///
    /// # Example
    /// ```
    /// let mut config = config_parser::new(); // create object of config_parse
    /// let val_f64: f64 = config.parse("5.51").unwrap(); // getting value f64 from string with numberic value
    /// ```
    /// **** Already added to loggics file. Be careful. Editing code can break stability of devices. *****
    ///
    pub fn parse_f64(&mut self, first_number_str: &str) -> Result<f64, ParseIntError> {
        let value = first_number_str.parse::<f64>();
        Ok(value.unwrap())
    }
    /// Show version of configuration parser
    ///
    /// # Arguments
    ///
    /// No arguments required
    ///
    /// # Return
    /// ```&'static str```
    ///
    /// # Example
    /// ```
    /// use crate::config_parse::*;
    /// let mut config = config_parser::new(); // create object of config_parse
    /// let version_config: &'static str = config.config_parser_version();
    /// println!("{}", version_config); // print that shows version into console
    /// ```
    /// **** Already added to loggics file. Be careful. Editing code can break stability of devices. *****
    ///
    pub fn config_parser_version(&mut self) -> &'static str {
        return "CONFIG PARSER VERSION V2.0.3 Alpha 22/12/2020";
    }
    /// Return data of esc motor configuration from pasring ini-file
    ///
    /// # Arguments
    ///
    /// No arguments required
    ///
    /// # Return
    /// ```EscMotors```
    ///
    /// # Example
    /// ```
    /// use crate::config_parse::*;
    /// let mut config = config_parser::new(); // create object of config_parse
    /// let esc_config = config.esc_config_parser(); // getting object value of configuration for esc driver
    /// ```
    /// **** Already added to loggics file. Be careful. Editing code can break stability of devices. *****
    ///
    ///
    pub fn filter_config(&mut self) -> Filter_Val {
        let filter_config = self.config_parser.section(Some("filter")).unwrap();
        let a = filter_config.get("conf_a").unwrap();
        let b = filter_config.get("conf_b").unwrap();
        let data = Filter_Val {
            conf_a: a.parse().unwrap(),
            conf_b: b.parse().unwrap(),
        };
        return data;
    }

    pub fn reciver_config(&mut self) -> Reciver {
        let esc_config = self.config_parser.section(Some("reciver")).unwrap();
        let port = (esc_config.get("port")).unwrap();
        let baudrate = (esc_config.get("baudrate")).unwrap();
        let reciver = Reciver {
            uart_port: port.parse().unwrap(),
            baudrate: baudrate.parse().unwrap(),
        };
        return reciver;
    }

    pub fn arm_switch_parse(&mut self) -> ArmSwitchConfig {
        let arm_config = self.config_parser.section(Some("arm_switch")).unwrap();
        let arm_switch_1 = (arm_config.get("arm_switch_1")).unwrap();
        let arm_switch_2 = (arm_config.get("arm_switch_2")).unwrap();
        let arm_switch = ArmSwitchConfig {
            arm_switch_1: arm_switch_1.parse::<usize>().unwrap(),
            arm_switch_2: arm_switch_2.parse::<usize>().unwrap(),
        };

        return arm_switch;
    }

    pub fn esc_config_parser(&mut self) -> EscMotors {
        let esc_config = self.config_parser.section(Some("esc")).unwrap();
        let port = (esc_config.get("port")).unwrap();
        let esc_motors_val = EscMotors {
            port: port.parse().unwrap(),
        };
        return esc_motors_val;
    }
    /// Return data of MPU6050 gyroscope sensor configuration from pasring ini-file
    ///
    /// # Arguments
    ///
    /// No arguments required
    ///
    /// # Return
    /// ```Mpu6050Conifg```
    /// # Example
    /// **** Already added to loggics file. Be careful. Editing code can break stability of devices. *****
    /// ```
    /// use crate::config_parse::*;
    /// let mut config = config_parser::new(); // create object of config_parse
    /// let mpu6050_conifg = config.mpu_config_parser(); // getting object value of configuartion for MPU6050 sensor
    /// ```
    ///
    pub fn imu_config_parser(&mut self) -> ImuConfig {
        let conf = Ini::load_from_file(file_path).unwrap();
        let imu_config_ = conf.section(Some("imu")).unwrap();
        let port = (imu_config_.get("port")).unwrap();
        let axis_assignment_gyro = (imu_config_.get("axis_assignment_gyro")).unwrap();
        let axis_assignment_acc = (imu_config_.get("axis_assignment_acc")).unwrap();
        let reversed_axis_acc = (imu_config_.get("reversed_axis_acc")).unwrap();
        let reversed_axis_gyro = (imu_config_.get("reversed_axis_gyro")).unwrap();
        let imu_config = ImuConfig {
            port: port.parse().unwrap(),
            axis_assignment_gyro: axis_assignment_gyro.parse().unwrap(),
            axis_assignment_acc: axis_assignment_acc.parse().unwrap(),
            reversed_axis_gyro: reversed_axis_gyro.parse().unwrap(),
            reversed_axis_acc: reversed_axis_acc.parse().unwrap(),
        };
        return imu_config;
    }
    /// Return data of Sbus/Ibus configuration from pasring ini-file
    ///
    /// # Arguments
    ///
    /// No arguments required
    ///
    /// # Return
    /// ```SbusConfig```
    ///
    ///  # Example
    /// **** Already added to loggics file. Be careful. Editing code can break stability of devices. *****
    /// ```
    /// use crate::config_parse::*;
    /// let mut config = config_parser::new(); // create object of config_parse
    /// let Sbus = config.ibus_receiver_conifg(); // get object of values S-BUS configuration
    /// ```
    ///
    pub fn ibus_receiver_conifg(&mut self) -> IbusConfig {
        let conf = Ini::load_from_file(file_path).unwrap();
        let sbus_config = conf.section(Some("ibus")).unwrap();
        let baudrate = self
            .parse_u32((sbus_config.get("baudrate")).unwrap())
            .unwrap();
        let port = sbus_config.get("port").unwrap();
        let Ibus = IbusConfig {
            baudrate: baudrate,
            port: port.to_string(),
        };
        return Ibus;
    }

    /// Return data of LIPO battery controller configuration from pasring ini-file
    ///
    /// # Arguments
    ///
    /// No arguments required
    ///
    /// # Return
    /// ```BatteryController```
    ///
    /// # Example
    /// ```
    /// use crate::config_parse::*;
    /// let battery_config = config.battery_controller_parser();
    /// ```
    /// **** Already added to loggics file. Be careful. Editing code can break stability of devices. *****
    ///
    ///
    pub fn battery_controller_parser(&mut self) -> BatteryController {
        let battery_controller = self
            .config_parser
            .section(Some("batterycontroller"))
            .unwrap();
        let port = (battery_controller.get("port")).unwrap();
        let max_vol_ch0 = (battery_controller.get("maximum_voltage_ch0")).unwrap();
        let max_vol_ch1 = (battery_controller.get("maximum_voltage_ch1")).unwrap();
        let max_vol_ch2 = (battery_controller.get("maximum_voltage_ch2")).unwrap();

        BatteryController {
            port: port.parse().unwrap(),
            max_vol_ch0: max_vol_ch0.parse::<f32>().unwrap(),
            max_vol_ch1: max_vol_ch1.parse::<f32>().unwrap(),
            max_vol_ch2: max_vol_ch2.parse::<f32>().unwrap(),
        }
    }

    /// PID parse for selected section from ini file
    ///
    /// # Arguments
    /// ```section``` - choose section of PID-configarution (String value)
    /// ```p_str``` - proportional
    /// ```i_str``` - integral
    /// ```d_str``` - derivative
    /// ```max``` - maximun output of PID
    ///
    /// # Return
    /// ```PID```
    ///
    /// # Example
    /// **** Already added to loggics file. Be careful. Editing code can break stability of devices. *****
    /// ```
    /// use crate::config_parse::*;
    /// let mut config = config_parser::new(); // create object of config_parse
    /// let pid_value_yaw = config.pid_parse(
    ///        "pid-config".parse().unwrap(),
    ///    "p_pid_yaw".parse().unwrap(),
    ///    "i_pid_yaw".parse().unwrap(),
    ///    "d_pid_yaw".parse().unwrap(),
    ///    "max_yaw".parse().unwrap(),); // get object values of pid of single section from parsing ini log file
    /// ```
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
    /// Return data of PID-configuration for all type of controls
    ///
    /// # Arguments
    ///
    /// No arguments required
    ///
    /// # Return
    /// ```PIDS```
    ///
    ///  # Example
    /// **** Already added to loggics file. Be careful. Editing code can break stability of devices. *****
    /// ```
    /// use crate::config_parse::*;
    /// let mut config = config_parser::new(); // create object of config_parse
    /// let pid_settings = config.get_pids(); // getting object values of pid-controller of all section
    /// ```
    ///
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
    /// Return status of auto-level mode
    ///
    /// # Arguments
    ///
    /// No arguments required
    ///
    /// # Return
    /// ```i32```
    /// 0 - fasle
    /// 1 - true
    ///
    /// # Example
    /// **** Already added to loggics file. Be careful. Editing code can break stability of devices. *****
    /// ```
    /// use crate::config_parse::*;
    /// let mut config = config_parser::new(); // create object of config_parse
    /// let autolevel = config.auto_level_config(); // getting value of auto level
    /// ```
    pub fn auto_level_config(&mut self) -> i32 {
        let autoconfig = self.config_parser.section(Some("auto-level")).unwrap();
        let amount = (autoconfig.get("autolevel")).unwrap().to_string();
        if amount.eq("on") {
            return 1;
        } else {
            return 0;
        }
    }
}

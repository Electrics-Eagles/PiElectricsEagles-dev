#[allow(non_camel_case_types)]
use std::fs::File;
use std::io::prelude::*;
const LOG_PATH: &str = "/var/pi_drone.log";

pub struct Logger {
    file_: File,
    stream_buf: String,
}
pub struct LoggingStruct {
    pub acc_z: i32,
    pub acc_y: i32,
    pub acc_x: i32,
    pub gyro_x: i32,
    pub gyro_y: i32,
    pub gyro_z: i32,
    pub reciver_ch1: u16,
    pub reciver_ch2: u16,
    pub reciver_ch3: u16,
    pub reciver_ch4: u16,
    pub reciver_ch5: u16,
    pub reciver_ch6: u16,
    pub pitch_level_correction: f32,
    pub roll_level_correction: f32,
    pub angle_pitch_acc: f32,
    pub angle_roll_acc: f32,
    pub pid_roll_setpoint: f32,
    pub pid_pitch_setpoint: f32,
    pub pid_yaw_setpoint: f32,
    pub pid_output_roll: f32,
    pub pid_output_pitch: f32,
    pub esc_1: f32,
    pub esc_2: f32,
    pub esc_3: f32,
    pub esc_4: f32,
    pub temp: f32,
    pub time_spent: u128,
}
impl Logger {
    pub fn new() -> Logger {
        let mut file = File::create(LOG_PATH).unwrap();
        file.write_all("PiElectricsEagles Log\n".as_ref()).unwrap();
        Logger {
            file_: file,
            stream_buf: String::new(),
        }
    }

    pub fn write_to_log(&mut self, mode: u8, log: &LoggingStruct) {
        // let mut  data_to_write:String=String::new();
        self.stream_buf.push_str(",");
        self.stream_buf.push_str(&log.acc_x.to_string());
        self.stream_buf.push_str(",");
        self.stream_buf.push_str(&log.acc_y.to_string());
        self.stream_buf.push_str(",");
        self.stream_buf.push_str(&log.acc_z.to_string());
        self.stream_buf.push_str(",");

        self.stream_buf.push_str(&log.gyro_x.to_string());
        self.stream_buf.push_str(",");
        self.stream_buf.push_str(&log.gyro_y.to_string());
        self.stream_buf.push_str(",");
        self.stream_buf.push_str(&log.gyro_z.to_string());
        self.stream_buf.push_str(",");

        self.stream_buf.push_str(&log.angle_pitch_acc.to_string());
        self.stream_buf.push_str(",");
        self.stream_buf.push_str(&log.angle_roll_acc.to_string());
        self.stream_buf.push_str(",");

        self.stream_buf.push_str(&log.esc_1.to_string());
        self.stream_buf.push_str(",");
        self.stream_buf.push_str(&log.esc_2.to_string());
        self.stream_buf.push_str(",");
        self.stream_buf.push_str(&log.esc_3.to_string());
        self.stream_buf.push_str(",");
        self.stream_buf.push_str(&log.esc_4.to_string());
        self.stream_buf.push_str(",");

        self.stream_buf.push_str(&log.reciver_ch1.to_string());
        self.stream_buf.push_str(",");
        self.stream_buf.push_str(&log.reciver_ch2.to_string());
        self.stream_buf.push_str(",");
        self.stream_buf.push_str(&log.reciver_ch3.to_string());
        self.stream_buf.push_str(",");
        self.stream_buf.push_str(&log.reciver_ch4.to_string());
        self.stream_buf.push_str(",");
        self.stream_buf.push_str(&log.reciver_ch5.to_string());
        self.stream_buf.push_str(",");
        self.stream_buf.push_str(&log.reciver_ch6.to_string());
        self.stream_buf.push_str(",");

        self.stream_buf.push_str(&log.pid_output_pitch.to_string());
        self.stream_buf.push_str(",");
        self.stream_buf.push_str(&log.pid_output_roll.to_string());
        self.stream_buf.push_str(",");
        self.stream_buf
            .push_str(&log.pid_pitch_setpoint.to_string());
        self.stream_buf.push_str(",");
        self.stream_buf.push_str(&log.pid_roll_setpoint.to_string());
        self.stream_buf.push_str(",");
        self.stream_buf.push_str(&log.pid_yaw_setpoint.to_string());
        self.stream_buf.push_str(",");

        self.stream_buf
            .push_str(&log.pitch_level_correction.to_string());
        self.stream_buf.push_str(",");
        self.stream_buf
            .push_str(&log.roll_level_correction.to_string());
        self.stream_buf.push_str(",");

        self.stream_buf.push_str(&log.time_spent.to_string());
        self.stream_buf.push_str(",");
        self.stream_buf.push_str(&log.temp.to_string());
        self.stream_buf.push_str("\n");
    }
    pub fn print_telemetry(&mut self)
    {
        println!("{}", self.stream_buf.to_string());
    }
    pub fn save_file(&mut self) {
        self.file_
            .write_all(self.stream_buf.as_bytes())
            .expect("Log write error");
        self.stream_buf.clear();
    }
    pub fn clear_buffer(&mut self)
    {
        self.stream_buf.clear();
    }
}

/*
encoding=utf-8

[pid-config]
# roll PID settings
p_pid_roll= 1
i_pid_roll=2
d_pid_roll=3
max_roll= 400

# Yaw PID settings
p_pid_yaw=1
i_pid_yaw=2
d_pid_yaw=3
max_yaw=400
# Yaw PID settings

p_pid_pitch=1
i_pid_pitch=2
d_pid_pitch = 3
# Pitch PID settings




 */


extern crate ini;
use ini::Ini;
use std::num::ParseIntError;

//Converts &str to i32
fn parse(first_number_str: &str) -> Result<i32, ParseIntError> {
    let value = first_number_str.parse::<i32>()?;
    Ok(value)
}

//Takes from config data parser it and return in i32 format
pub fn pid_parse(section:String, p_str:String, i_str:String, d_str:String, max:String) -> [i32; 4] {
    let conf = Ini::load_from_file("./src/config/core.ini").unwrap();
    let pid_configuration = conf.section(Some(section)).unwrap();
    let  p_pid = (pid_configuration.get(p_str)).unwrap();
    let  i_pid=pid_configuration.get(i_str).unwrap();
    let  d_pid= pid_configuration.get(d_str).unwrap();
    let  max= pid_configuration.get(max).unwrap();
    let i32_p_pid= parse(p_pid).unwrap();
    let i32_i_pid= parse(i_pid).unwrap();
    let i32_d_pid= parse(d_pid).unwrap();
    let i32_max= parse(max).unwrap();
    let mut pid: [i32; 4] = [0; 4];
    pid[0]=i32_p_pid;
    pid[1]=i32_i_pid;
    pid[2]=i32_d_pid;
    pid[3]=i32_max;
    return pid;




}

 pub fn get_pids() -> Vec<[i32; 4]> {
    let roll = pid_parse("pid-config".parse().unwrap(), "p_pid_roll".parse().unwrap(), "i_pid_roll".parse().unwrap(), "d_pid_roll".parse().unwrap(), "max_roll".parse().unwrap());
    let yaw = pid_parse("pid-config".parse().unwrap(), "p_pid_yaw".parse().unwrap(), "i_pid_yaw".parse().unwrap(), "d_pid_yaw".parse().unwrap(), "max_yaw".parse().unwrap());
    let pitch = pid_parse("pid-config".parse().unwrap(), "p_pid_pitch".parse().unwrap(), "i_pid_pitch".parse().unwrap(), "d_pid_pitch".parse().unwrap(), "max_pitch".parse().unwrap());
    let mut values = Vec::new();
    values.push(roll);
    values.push(yaw);
    values.push(pitch);
    return values;
}




























extern crate ini;
use ini::Ini;
use std::str::FromStr;






pub fn read_config_pid() -> Vec<[i32; 4]> {
    let conf = Ini::load_from_file("./src/config/core.ini").unwrap();
    let pids_config = conf.section(Some("pid-config")).unwrap();
    let roll = pids_config.get("p_pid_roll");
    let yaw = pids_config.get("p_pid_yaw");
    let pitch = pids_config.get("p_pid_pitch");

    let mut pid_roll: [i32; 4] = [0; 4];
    pid_roll[0]=i32::from_str(roll.as_deref().unwrap_or("p_pid_roll")).unwrap();
    pid_roll[1]=i32::from_str(roll.as_deref().unwrap_or("i_pid_roll")).unwrap();
    pid_roll[2]=i32::from_str(roll.as_deref().unwrap_or("d_pid_roll")).unwrap();
    pid_roll[3]=i32::from_str(roll.as_deref().unwrap_or("max_roll")).unwrap();
    let mut pid_yaw: [i32; 4] = [0; 4];
    pid_yaw[0]=i32::from_str(yaw.as_deref().unwrap_or("p_pid_yaw")).unwrap();
    pid_yaw[1]=i32::from_str(yaw.as_deref().unwrap_or("i_pid_yaw")).unwrap();
    pid_yaw[2]=i32::from_str(yaw.as_deref().unwrap_or("d_pid_yaw")).unwrap();
    pid_yaw[3]=i32::from_str(yaw.as_deref().unwrap_or("max_yaw")).unwrap();
    let mut pid_pitch: [i32; 4] = [0; 4];
    pid_pitch[0]=i32::from_str(pitch.as_deref().unwrap_or("p_pid_pitch")).unwrap();
    pid_pitch[1]=i32::from_str(pitch.as_deref().unwrap_or("i_pid_pitch")).unwrap();
    pid_pitch[2]=i32::from_str(pitch.as_deref().unwrap_or("d_pid_pitch")).unwrap();
    pid_pitch[3]=i32::from_str(pitch.as_deref().unwrap_or("max_pitch")).unwrap();

    let mut values = Vec::new();
    values.push(pid_roll);
    values.push(pid_yaw);
    values.push(pid_pitch);
    return values;








}










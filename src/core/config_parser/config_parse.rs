


extern crate ini;
use ini::Ini;
use std::str::FromStr;






pub fn read_config_pid() -> [i32; 3] {
    let conf = Ini::load_from_file("./src/config/core.ini").unwrap();
    let pids = conf.section(Some("pid")).unwrap();
    let option = pids.get("p");
    let value = option.as_deref().unwrap_or("p");
    let p=i32::from_str(value).unwrap();
    let i=i32::from_str(value).unwrap();
    let d=i32::from_str(value).unwrap();
    let mut pids_value: [i32; 3] = [0; 3];
    pids_value[0]=p;
    pids_value[1]=i;
    pids_value[2]=d;
    return pids_value





}










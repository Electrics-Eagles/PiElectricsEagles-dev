mod client_setting_pid;

use crate::client_setting_pid::{remote_connect, pid_values};
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::thread::sleep;
use std::time::*;
fn main() {
    /* Create class of remote_connect for setting pid in server */
    let mut connect_remote: remote_connect = client_setting_pid::remote_connect::connect_to_http_server("192.168.1.104", "7878");
    /* Read ini - file from server connected */
    println!("{}", connect_remote.read_config_of_pid_to_String().as_str());
    /* Show version of library of requesting pid configuration client */
    println!("{}", connect_remote.showversion().as_str());
    /* Value of pid for setting pid */
    let mut my_value_of_pid = pid_values
    {
        p_pid_roll: 5.9,
        i_pid_roll: 2.5,
        d_pid_roll: 3.0,
        max_roll: 20.0,
        p_pid_yaw: 1.2,
        i_pid_yaw: 2.8,
        d_pid_yaw: 3.6,
        max_yaw: 45.0,
        p_pid_pitch: 6.1,
        i_pid_pitch: 5.1,
        d_pid_pitch: 4.8,
        max_pitch: 54.3,
    };
    /* Setting value of pid values */
    connect_remote.send_config_pid(my_value_of_pid);
    thread::sleep(Duration::from_millis(2000));
    /* Get value of pid_value struct from reading configuration */
    let rd_conf = connect_remote.read_config_of_pid_to_pid_values();
    println!("{}", rd_conf.p_pid_pitch);
}

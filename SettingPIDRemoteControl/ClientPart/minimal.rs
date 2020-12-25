#![allow(dead_code)]

mod client_setting_pid;

use crate::client_setting_pid::{remote_connect, pid_values};
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::thread::sleep;
use std::time::*;
fn main() {
    let mut connect_remote: remote_connect =
        client_setting_pid::remote_connect::connect_to_http_server("192.168.1.101", "7878");
    //println!("{}", connect_remote.read_config_of_pid_to_String().as_str());
    //println!("{}", connect_remote.showversion().as_str());
    //println!("{}", connect_remote.read_config_of_pid_to_String());

    let rd_conf = connect_remote.read_config_of_pid_to_pid_values();
    println!("{}", rd_conf.p_pid_pitch);
}

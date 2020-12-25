use std::io::prelude::*;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::net::TcpStream;

pub struct pid_values {
    // roll
    pub p_pid_roll: f32,
    pub i_pid_roll: f32,
    pub d_pid_roll: f32,
    pub max_roll: f32,
    // yaw
    pub p_pid_yaw: f32,
    pub i_pid_yaw: f32,
    pub d_pid_yaw: f32,
    pub max_yaw: f32,
    // pitch
    pub p_pid_pitch: f32,
    pub i_pid_pitch: f32,
    pub d_pid_pitch: f32,
    pub max_pitch: f32,
}
impl pid_values {
    pub fn new() -> Self
    {
        pid_values {
            p_pid_roll: 0.0,
            i_pid_roll: 0.0,
            d_pid_roll: 0.0,
            max_roll: 0.0,
            p_pid_yaw: 0.0,
            i_pid_yaw: 0.0,
            d_pid_yaw: 0.0,
            max_yaw: 0.0,
            p_pid_pitch: 0.0,
            i_pid_pitch: 0.0,
            d_pid_pitch: 0.0,
            max_pitch: 0.0,
        }
    }
}
pub struct remote_connect {
    Host: String,
    Port: String,
}
impl remote_connect {
    pub fn connect_to_http_server(Host: &str, Port: &str) -> Self {
        remote_connect {
            Host: Host.to_string(),
            Port: Port.to_string(),
        }
    }
    pub fn read_config_of_pid_to_String(&mut self) -> String {
        let mut str_host = String::from(&self.Host);
        str_host.push(':');
        str_host.push_str(&self.Port);
        let mut stream = std::net::TcpStream::connect(str_host.as_str()).unwrap();
        let mut wrtx = String::from("GET / HTTP/1.1\r\n");
        wrtx.push_str("Host:");
        wrtx.push_str(self.Host.as_str());
        wrtx.push_str("\r\n\r\n");
        stream.write(&mut wrtx.as_bytes());
        // Getting data from host (Core.ini reading file)
        let mut str: Vec<u8> = Vec::new();
        stream.read_to_end(&mut str).unwrap();
        let mut read_orgin_str = String::from_utf8_lossy(&mut str);
        stream.flush();
        return String::from(read_orgin_str.trim());
    }
    pub fn showversion(&mut self) -> String {
        let mut str_host = String::from(&self.Host);
        str_host.push(':');
        str_host.push_str(&self.Port);
        let mut stream = std::net::TcpStream::connect(str_host.as_str()).unwrap();
        let mut wrtx = String::from("GET /showver HTTP/1.1\r\n");
        wrtx.push_str("Host:");
        wrtx.push_str(self.Host.as_str());
        wrtx.push_str("\r\n\r\n");
        stream.write(&mut wrtx.as_bytes());
        // Getting data from host (Core.ini reading file)
        let mut str: Vec<u8> = Vec::new();
        stream.read_to_end(&mut str).unwrap();
        let mut read_orgin_str = String::from_utf8_lossy(&mut str);
        stream.flush();
        let mut str_ver_full = String::from(read_orgin_str.trim());
        return String::from(String::from(&str_ver_full[15..str_ver_full.len()]).trim());
    }
    pub fn read_config_of_pid_to_pid_values(&mut self) -> pid_values
    {
        let mut str_host = String::from(&self.Host);
        str_host.push(':');
        str_host.push_str(&self.Port);
        let mut stream = std::net::TcpStream::connect(str_host.as_str()).unwrap();
        let mut wrtx = String::from("GET / HTTP/1.1\r\n");
        wrtx.push_str("Host:");
        wrtx.push_str(self.Host.as_str());
        wrtx.push_str("\r\n\r\n");
        stream.write(&mut wrtx.as_bytes());
        // Getting data from host (Core.ini reading file)
        let mut str: Vec<u8> = Vec::new();
        stream.read_to_end(&mut str).unwrap();
        let mut read_orgin_str = String::from_utf8_lossy(&mut str);
        stream.flush();

        pid_values {
            p_pid_roll: *&read_orgin_str[read_orgin_str.find("p_pid_roll=").unwrap() + 11..read_orgin_str.find("i_pid_roll").unwrap()].trim().parse::<f32>().unwrap(),
            i_pid_roll: *&read_orgin_str[read_orgin_str.find("i_pid_roll=").unwrap() + 11..read_orgin_str.find("d_pid_roll").unwrap()].trim().parse::<f32>().unwrap(),
            d_pid_roll: *&read_orgin_str[read_orgin_str.find("d_pid_roll=").unwrap() + 11..read_orgin_str.find("max_roll").unwrap()].trim().parse::<f32>().unwrap(),
            max_roll: *&read_orgin_str[read_orgin_str.find("max_roll=").unwrap() + 9..read_orgin_str.find("p_pid_yaw").unwrap()].trim().parse::<f32>().unwrap(),
            p_pid_yaw: *&read_orgin_str[read_orgin_str.find("p_pid_yaw=").unwrap() + 10..read_orgin_str.find("i_pid_yaw").unwrap()].trim().parse::<f32>().unwrap(),
            i_pid_yaw: *&read_orgin_str[read_orgin_str.find("i_pid_yaw=").unwrap() + 10..read_orgin_str.find("d_pid_yaw").unwrap()].trim().parse::<f32>().unwrap(),
            d_pid_yaw: *&read_orgin_str[read_orgin_str.find("d_pid_yaw=").unwrap() + 10..read_orgin_str.find("max_yaw").unwrap()].trim().parse::<f32>().unwrap(),
            max_yaw: *&read_orgin_str[read_orgin_str.find("max_yaw=").unwrap() + 8..read_orgin_str.find("p_pid_pitch").unwrap()].trim().parse::<f32>().unwrap(),
            p_pid_pitch: *&read_orgin_str[read_orgin_str.find("p_pid_pitch=").unwrap() + 12..read_orgin_str.find("i_pid_pitch").unwrap()].trim().parse::<f32>().unwrap(),
            i_pid_pitch: *&read_orgin_str[read_orgin_str.find("i_pid_pitch=").unwrap() + 12..read_orgin_str.find("d_pid_pitch").unwrap()].trim().parse::<f32>().unwrap(),
            d_pid_pitch: *&read_orgin_str[read_orgin_str.find("d_pid_pitch=").unwrap() + 12..read_orgin_str.find("max_pitch").unwrap()].trim().parse::<f32>().unwrap(),
            max_pitch: *&read_orgin_str[read_orgin_str.find("max_pitch=").unwrap() + 10..read_orgin_str.find("[esc-config]").unwrap()].trim().parse::<f32>().unwrap(),
        }
    }
    pub fn send_config_pid(&mut self, pid_val: pid_values) -> String {
        // Concate string for building host...
        let mut pid_sub_host = String::from("/pid_config_sent=");
        // Pid config
        pid_sub_host.push_str("p_pid_roll=");
        pid_sub_host.push_str(&pid_val.p_pid_roll.to_string());
        pid_sub_host.push_str(":i_pid_roll=");
        pid_sub_host.push_str(&pid_val.i_pid_roll.to_string());
        pid_sub_host.push_str(":d_pid_roll=");
        pid_sub_host.push_str(&pid_val.d_pid_roll.to_string());
        pid_sub_host.push_str(":max_roll=");
        pid_sub_host.push_str(&pid_val.max_roll.to_string());
        // Yaw config
        pid_sub_host.push_str(":p_pid_yaw=");
        pid_sub_host.push_str(&pid_val.p_pid_yaw.to_string());
        pid_sub_host.push_str(":i_pid_yaw=");
        pid_sub_host.push_str(&pid_val.i_pid_yaw.to_string());
        pid_sub_host.push_str(":d_pid_yaw=");
        pid_sub_host.push_str(&pid_val.d_pid_yaw.to_string());
        pid_sub_host.push_str(":max_yaw=");
        pid_sub_host.push_str(&pid_val.max_yaw.to_string());
        // Pitch config
        pid_sub_host.push_str(":p_pid_pitch=");
        pid_sub_host.push_str(&pid_val.p_pid_pitch.to_string());
        pid_sub_host.push_str(":i_pid_pitch=");
        pid_sub_host.push_str(&pid_val.i_pid_pitch.to_string());
        pid_sub_host.push_str(":d_pid_pitch=");
        pid_sub_host.push_str(&pid_val.d_pid_pitch.to_string());
        pid_sub_host.push_str(":max_pitch=");
        pid_sub_host.push_str(&pid_val.max_pitch.to_string());
        // testing correct entering a host...
        println!("Send host to server ... {}", pid_sub_host.as_str());
        // Retrive host str before conncet to server...
        let mut str_host = String::from(&self.Host);
        str_host.push(':');
        str_host.push_str(&self.Port);
        // Connect to server
        let mut stream = std::net::TcpStream::connect(str_host.as_str()).unwrap();
        // Send host adress with sub config of pid
        let mut wrtx = String::from("GET ");
        wrtx.push_str(pid_sub_host.as_str());
        wrtx.push_str(" HTTP/1.1\r\n");
        wrtx.push_str("Host:");
        wrtx.push_str(self.Host.as_str());
        wrtx.push_str("\r\n\r\n");
        stream.write(&mut wrtx.as_bytes());
        // Getting data from host (Core.ini reading file and log, status of server)
        let mut str: Vec<u8> = Vec::new(); // values for reading info from server
        stream.read_to_end(&mut str).unwrap();
        let mut read_orgin_str = String::from_utf8_lossy(&mut str);
        // stream flush after using...
        stream.flush();
       return  String::from(read_orgin_str.trim());
    }
}

/* Server code source for remote setting PID configuration 
Created by M.Zaslavskis
25th December 2020 */
use std::fs::File;
use std::net::*;
use std::io::prelude::*;

fn main() {
    /* Path of configuartion file */
    let mut path_pid_config = String::from("/etc/pielectricseagles/config.ini");
    /* Print path of config-file via console */
    println!("Path of core config: {}", path_pid_config);
    /* Array for find related values of pid when check */
    let finding_related_values: [&str; 12] =
    ["p_pid_roll=", "i_pid_roll=", "d_pid_roll=", "max_roll=",
    "p_pid_yaw=", "i_pid_yaw=", "d_pid_yaw=", "max_yaw=",
    "p_pid_pitch=", "i_pid_pitch=", "d_pid_pitch=", "max_pitch="];
    // info for server
    let mut info_server = String::new();
    // send to computer from raspberry
    // need http server:
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    // Block forever, handling each request that arrives at this IP address
    for stream in listener.incoming()
    {
        let mut stream = stream.unwrap();

        // Read the first 1024 bytes of data from the stream
        let mut buffer = [0; 1024];
        // reading request
        stream.read(&mut buffer).unwrap();
        // string main request for send log to client
        let get = b"GET / HTTP/1.1\r\n";
        // if get requset from client
        if (buffer.starts_with(get)) {
            // reading config-file
            info_server =  reading_core_ini(&mut path_pid_config);
            // Binding http server string
            let response = format!("{}{}", "HTTP/1.1 200 OK\r\n\r\n", info_server);
            // Send to certain client
            stream.write(response.as_bytes()).unwrap();
        }
        else if (buffer.starts_with(b"GET /showver HTTP/1.1\r\n")) {
            // String from retreiving version of server
            let response = format!("{}{}", "HTTP/1.1 200 OK\r\n\r\n", "1.0.0");
            // Send to certain client
            stream.write(response.as_bytes()).unwrap();
        }
        // if get other sub-host request
        else {
            //string for check contain value of pid
            let mut essential_value_str = String::from_utf8_lossy(&buffer[..]);
            //string for retriving info for client
            let mut response = String::new();
            // if get host of pid configaration
            if (essential_value_str.contains("GET /pid_config_sent=") == true && essential_value_str.contains("HTTP/1.1\r\n"))
            {
                let mut addr_end :Option<usize>= Some(essential_value_str.find("HTTP/1.1")).unwrap();
                let mut newer_config_of_pid = String::from(&essential_value_str[21..addr_end.unwrap()]).replace(":", "\n");
                // checking strings
                let mut error_sytanixic_of_config = 0;
                let mut vec_str: Vec<&str>= newer_config_of_pid.split("\n").collect();
                /* Checking correct enter a host, if got error entering won't set new pid value and just message error to client  */
                for check_config in &finding_related_values
                {
                    if (!newer_config_of_pid.contains(check_config)) {
                        error_sytanixic_of_config += 1;
                    }

                }
                /* if error got, send message to client */
                if (error_sytanixic_of_config > 0)
                {
                    response = format!("{}{}", "HTTP/1.1 200 OK\r\n\r\n",
                    "This configuaration has error naming of subconfigation data... Please recheck spelling in config data before sending request http server");
                }
                /* if send host correctly of pid setting then file of configuartion will rewrite into newer  */
                else {
                    // rewrite config data
                    let mut updated_config_file = File::create(&mut path_pid_config).unwrap();
                    updated_config_file.
                        write(String::from(&info_server[0..info_server.find("[pid-config]\n").unwrap() + 13]).as_bytes())
                        .unwrap(); // rewrite fil not - related configuration from reading file
                    updated_config_file.
                        write(newer_config_of_pid.as_bytes())
                        .unwrap(); // rewrite new pid configuration
                    updated_config_file.
                        write(String::from(&info_server[info_server.find("\n[esc-config]").unwrap()..info_server.len()]).as_bytes())
                        .unwrap(); // rewrite fil not - related configuration from reading file
                    /* Reading new configuration file and send to client */
                    let mut log_of_new_file = reading_core_ini(&mut path_pid_config);
                    response = format!("{}{}{}", "HTTP/1.1 200 OK\r\n\r\n", "File wrote sucessfully! \n Log: \n", log_of_new_file);
                }


            }
            /* If send any name of host, but not related with essential task ...
             Just message about errorness to client */
            else {
                response = format!("{}{}", "HTTP/1.1 404 NOT FOUND\r\n\r\n", "Error!");
            }
            /* Write info to client */
            stream.write(response.as_bytes()).unwrap();
        }
        /* Clear buffer in tcp stream */
        stream.flush().unwrap();
    }
}
/* Getting configuration pid to String value (for reading configuration) */
fn reading_core_ini(path_of_config: &mut String) -> String
{
    let mut read_conf = File::open( path_of_config).expect("Error reading file!");
    let mut config_text = String::new();
    read_conf.read_to_string(&mut config_text);
    config_text
}

/*


encoding=utf-8

[pid-config]
# roll PID settings
p_pid_roll= 1.3
i_pid_roll= 0.04
d_pid_roll= 18.0
max_roll= 400

# Yaw PID settings
p_pid_yaw=4.0
i_pid_yaw=0.02
d_pid_yaw=0.0
max_yaw=400
# Pitch PID settings

p_pid_pitch=1.3
i_pid_pitch=0.04
d_pid_pitch = 18.0
max_pitch=400

[esc-config]
port="/dev/i2c-1"
amount =4
driver="pca9685"



[auto-level]
autolevel="off"

use configparser::ini::Ini;

let mut config = Ini::new();
config.read(String::from(
  "[somesection]
  someintvalue = 5"));
let my_value = config.get("somesection", "someintvalue").unwrap(),;
assert_eq!(my_value, 5); // value accessible!

//You can ofcourse just choose to parse the values yourself:
let my_string = String::from("1984");
let my_int = my_string.parse::<i32>().unwrap();
 */




use ini::Ini;
use std::borrow::{Borrow, BorrowMut};
use configparser::ini::Ini as OtherIni;
#[derive(Clone)]

pub struct Data {
    p_pid_roll:String,
    i_pid_roll:String,
    d_pid_roll:String,
    max_roll:String,
    p_pid_yaw:String,
    i_pid_yaw:String,
    d_pid_yaw:String,
    p_pid_pitch:String,
    max_yaw:String,
    i_pid_pitch:String,
    d_pid_pitch:String,
    max_pitch:String,
    port:String,
    amount:String,
    driver:String,
    autolevel:String,



}

pub struct  Params {
    param_1:String,
    param_2:String,
    param_3:String,
    param_4:String,
}


fn main() {
let mut input=detect_config();
    save_config( edit_config(input));


}
fn save_config(data:Data) {

    let mut conf = Ini::new();
    conf.with_section(Some("pid-config")).set("p_pid_roll", data.p_pid_roll);
    conf.with_section(Some("pid-config")).set("i_pid_roll", data.i_pid_roll);
    conf.with_section(Some("pid-config")).set("d_pid_roll", data.d_pid_roll);
    conf.with_section(Some("pid-config")).set("max_roll", data.max_roll);
    conf.with_section(Some("pid-config")).set("p_pid_yaw", data.p_pid_yaw);
    conf.with_section(Some("pid-config")).set("i_pid_yaw", data.i_pid_yaw);
    conf.with_section(Some("pid-config")).set("d_pid_yaw", data.d_pid_yaw);
    conf.with_section(Some("pid-config")).set("p_pid_pitch", data.p_pid_pitch);
    conf.with_section(Some("pid-config")).set("i_pid_pitch", data.i_pid_pitch);
    conf.with_section(Some("pid-config")).set("d_pid_pitch", data.d_pid_pitch);
    conf.with_section(Some("pid-config")).set("max_pitch", data.max_pitch);
    conf.with_section(Some("esc-config")).set("port", data.port);
    conf.with_section(Some("esc-config")).set("amount", data.amount);
    conf.with_section(Some("esc-config")).set("driver", data.driver);
    conf.with_section(Some("auto-level")).set("autolevel", data.autolevel);
    conf.write_to_file("/etc/pielectricseagles/config.ini").unwrap();


}


fn fetch_set_params() -> Params{
    let param_1 = std::env::args().nth(2).expect("no pattern given");
    let param_2 = std::env::args().nth(3).expect("no pattern given");
    let param_3 = std::env::args().nth(4).expect("no pattern given");
    let param_4 = std::env::args().nth(5).expect("no pattern given");
    let params=Params {
        param_1,
        param_2,
        param_3,
        param_4,
    };
    params
}


fn edit_config(data:Data) -> Data {
    let mut result =data;
    let mode = std::env::args().nth(1).expect("no pattern given");


   if mode=="roll" {
       result.p_pid_roll=fetch_set_params().param_1;
       result.i_pid_roll=fetch_set_params().param_2;
       result.d_pid_roll=fetch_set_params().param_3;
       result.max_roll=fetch_set_params().param_4;

   }
    if mode=="pitch" {
        result.p_pid_pitch=fetch_set_params().param_1;
        result.i_pid_pitch=fetch_set_params().param_2;
        result.d_pid_pitch=fetch_set_params().param_3;
        result.max_pitch=fetch_set_params().param_4;

    }
    if mode=="yaw" {
        result.p_pid_yaw=fetch_set_params().param_1;
        result.i_pid_yaw=fetch_set_params().param_2;
        result.d_pid_yaw=fetch_set_params().param_3;
        result.max_yaw=fetch_set_params().param_4;

    }

    if mode=="port" {
        result.port= std::env::args().nth(2).expect("no pattern given");
    }

    if mode=="driver" {
        result.driver= std::env::args().nth(2).expect("no pattern given");
    }

    if mode=="autolevel" {
        result.autolevel= std::env::args().nth(2).expect("no pattern given");
    }

    if mode=="amount" {
        result.amount= std::env::args().nth(2).expect("no pattern given");
    }
   return result;
}
fn detect_config() -> Data {
    let mut config = OtherIni::new();

    // You can easily load a file to get a clone of the map:
     config.load("/etc/pielectricseagles/config.ini");
       let data = Data{
           p_pid_roll: config.get("pid-config", "p_pid_roll").unwrap(),
           i_pid_roll: config.get("pid-config", "i_pid_roll").unwrap(),
           d_pid_roll: config.get("pid-config", "d_pid_roll").unwrap(),
           max_roll: config.get("pid-config", "max_roll").unwrap(),
           p_pid_yaw: config.get("pid-config", "p_pid_yaw").unwrap(),
           i_pid_yaw: config.get("pid-config", "i_pid_yaw").unwrap(),
           d_pid_yaw: config.get("pid-config", "d_pid_yaw").unwrap(),
           max_yaw: config.get("pid-config", "d_pid_yaw").unwrap(),
           p_pid_pitch: config.get("pid-config", "p_pid_pitch").unwrap(),
           i_pid_pitch: config.get("pid-config", "i_pid_pitch").unwrap(),
           d_pid_pitch: config.get("pid-config", "d_pid_pitch").unwrap(),
           max_pitch: config.get("pid-config", "max_pitch").unwrap(),
           port:  config.get("esc-config", "port").unwrap(),
           amount: config.get("esc-config", "amount").unwrap(),
           driver: config.get("esc-config", "driver").unwrap(),
           autolevel: config.get("auto-level", "autolevel").unwrap(),
       };

       return data;


    }



//Y



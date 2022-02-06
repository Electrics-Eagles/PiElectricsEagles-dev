use crate::controller::Controller;



pub fn calibrate_esc() {
    let  mut args = std::env::args();

    if args.len() > 1 {
        let esc_number = args.nth(2).unwrap();
        let mut controller = Controller::new();
        //controller.set_throttle_external_pwm(init_throllite, init_throllite, init_throllite, init_throllite);


    }


}


pub fn  esc_test() {
    let  mut args = std::env::args();
    if args.len() > 2 {
        let esc_number = args.nth(2).unwrap().parse::<i32>().unwrap();
        let throttle = args.nth(3).unwrap().parse::<u16>().unwrap();
        let mut controller = Controller::new();
        match esc_number {
            1 => controller.set_throttle_external_pwm(throttle, 0, 0, 0),
            2 => controller.set_throttle_external_pwm(0, throttle, 0, 0),
            3 => controller.set_throttle_external_pwm(0, 0, throttle, 0),
            4 => controller.set_throttle_external_pwm(0, 0, 0, throttle),
            _ => println!("{}","error invalid argument."),
        }



    }

}

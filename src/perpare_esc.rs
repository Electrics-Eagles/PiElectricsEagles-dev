use crate::controller::Controller;
const MAX_THROLLITE: u16 = 2000;
const MIN_THROLLITE: u16 = 1000;

fn set_throttle_single_esc(controller: &mut Controller, esc_number: i32, throllite: u16) {
    match esc_number {
        1 => controller.set_throttle_external_pwm(throllite, 0, 0, 0),
        2 => controller.set_throttle_external_pwm(0, throllite, 0, 0),
        3 => controller.set_throttle_external_pwm(0, 0, throllite, 0),
        4 => controller.set_throttle_external_pwm(0, 0, 0, throllite),
        _ => println!("{}", "error invalid argument."),
    }
}

pub fn calibrate_esc() {
    let mut args = std::env::args();
    if args.len() > 1 {
        let esc_number = args.nth(2).unwrap().parse::<i32>().unwrap();
        let mut controller = Controller::new();
        let mut line = String::new();
        println!("Please connect to battery, and wait two second. Press Enter when ready!!!!! Signal will send max throllite to ESC.");
        set_throttle_single_esc(&mut controller, esc_number, MAX_THROLLITE);
        std::io::stdin().read_line(&mut line).unwrap();
        println!(
            "Now sending minumum throllite to ESC. Waiting some time until stop to beep motor."
        );
        set_throttle_single_esc(&mut controller, esc_number, MIN_THROLLITE);
        std::io::stdin().read_line(&mut line).unwrap();
        println!("Finished to cablibarte ESC!!! Please test ESC now without propellers or take with cautions to avoid serous injury!");
    }
}

pub fn esc_test() {
    let mut args = std::env::args();
    if args.len() > 2 {
        let esc_number = args.nth(2).unwrap().parse::<i32>().unwrap();
        let throttle = args.nth(3).unwrap().parse::<i32>().unwrap() as u16;
        let mut controller = Controller::new();
        set_throttle_single_esc(&mut controller, esc_number, throttle);
    }
}

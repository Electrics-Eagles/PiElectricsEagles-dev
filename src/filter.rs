#[allow(non_camel_case_types)]
use crate::config_parse::config_parser;
use crate::utils::sqrt;


pub struct Coff {
    a: f64,
    b: f64,
}

pub struct Filter {
    coff: Coff,
}


const dt: f32 = 0.006;

pub fn ABfilter(newVal: f32, conf_a: f32, conf_b: f32,turned_on:bool) -> f32 {
    let mut xk_1: f32 = 0.0;
    let mut vk_1: f32 = 0.0;
    let mut a: f32 = 0.0;
    let mut b: f32 = 0.0;
    let mut xk: f32 = 0.0;
    let mut vk: f32 = 0.0;
    let mut rk: f32 = 0.0;
    let mut xm: f32 = 0.0;
    if turned_on {
        let mut lambda = conf_a as f32 * dt * dt / conf_b as f32;
        let mut r = (4.0 + lambda - sqrt(8.0 * lambda + lambda * lambda)) / 4.0;
        a = 1.0 - r * r;
        b = 2.0 * (2.0 - a) - 4.0 * sqrt(1.0 - a);
        xm = newVal;
        xk = xk_1 + (vk_1 * dt);
        vk = vk_1;
        rk = xm - xk;
        xk += a * rk;
        vk += (b * rk) / dt;
        xk_1 = xk;
        vk_1 = vk;
        return xk_1;
    }
    else {
        return newVal;
    }
}







static mut result_lpf: f32 = 0.0;
pub  fn low_pass_filter(x: f32, delta_t: f32, filtration_period: f32,turned_on:bool) -> f32 {

    unsafe {
        if turned_on {
            let mut alpha = delta_t / filtration_period;
            result_lpf += alpha * (x - result_lpf);
            return result_lpf;
        }
        else {
            return x;
        }
    }
}









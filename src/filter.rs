#[allow(non_camel_case_types)]
/* Python code */
/*

import os
import sys
import matplotlib.pyplot as plt
import numpy as np


value=1
divider=10
array_of_data=[]
array=[]
result_array=[]

#y += alpha * (x-y);
#y = (a * y) + (x - (a * x)

def simple_parse():
    data=open(input(),"r").readlines()
    path_save=input()
    for a in range(1,len(data)):
        parsed_val=data[a].split("::")[value]
        #print(parsed_val)
        array_of_data.append(float(parsed_val))
        #print(array_of_data)
    for d in range(0,len(array_of_data)):
        delta_t = 0.01
        tau = 0.2
        alpha = delta_t / tau
        result=lpf(array_of_data[d], alpha,d)
        result_array.append(result)
    plt.plot(array_of_data)
    plt.plot(result_array)
        #vals.write(str(round(float(data[a*divider].split("::")[value]),2))+"\n")
simple_parse()
plt.show()


 */
/*
def lpf(x, alpha,d):
    global y
    if(d==0):
        y=0
    else:
        y += alpha*(x-y)
    return y
 */
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

pub fn ABfilter(newVal: f32, conf_a: f32, conf_b: f32) -> f32 {
    let mut xk_1: f32 = 0.0;
    let mut vk_1: f32 = 0.0;
    let mut a: f32 = 0.0;
    let mut b: f32 = 0.0;
    let mut xk: f32 = 0.0;
    let mut vk: f32 = 0.0;
    let mut rk: f32 = 0.0;

    let mut xm: f32 = 0.0;
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


/*
    pub  fn low_pass_filter(x: f64, delta_t: f64, filtration_period: f64) -> f64 {
        unsafe {
            let  alpha = delta_t / filtration_period;
            result_lpf += alpha * (x - result_lpf);
            return result_lpf;
        }
    }


    pub fn filter(x: f64, delta_t: f64, filtration_period: f64)  -> f64 {
        let result=selow_pass_filter(x,delta_t,filtration_period);
        unsafe { result_lpf = 0.0; }
        return result
    }
*/

// период дискретизации (измерений), process variation, noise variation
/*
    float dt = 0.02;
    float sigma_process = 3.0;
    float sigma_noise = 0.7;
    float ABfilter(float newVal) {
    static float xk_1, vk_1, a, b;
    static float xk, vk, rk;
    static float xm;
    float lambda = (float)sigma_process * dt * dt / sigma_noise;
    float r = (4 + lambda - (float)sqrt(8 * lambda + lambda * lambda)) / 4;
    a = (float)1 - r * r;
    b = (float)2 * (2 - a) - 4 * (float)sqrt(1 - a);
    xm = newVal;
    xk = xk_1 + ((float) vk_1 * dt );
    vk = vk_1;
    rk = xm - xk;
    xk += (float)a * rk;
    vk += (float)( b * rk ) / dt;
    xk_1 = xk;
    vk_1 = vk;
    return xk_1;
    }

 */

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

static mut result_lpf: f64 = 0.0;
pub fn low_pass_filter(x: f64, delta_t: f64, filtration_period: f64) -> f64 {
    unsafe {
        let mut alpha = delta_t / filtration_period;
        result_lpf += alpha * (x - result_lpf);
        return result_lpf;
    }
}

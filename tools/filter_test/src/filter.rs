static mut result_lpf: f64 = 0.0;
pub fn low_pass_filter(x: f64, delta_t: f64, filtration_period: f64) -> f64 {
    unsafe {
        let mut alpha = delta_t / filtration_period;
        result_lpf += alpha * (x - result_lpf);
        return result_lpf;
    }
}

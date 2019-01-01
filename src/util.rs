/*
Util functions for data generation, time measurement etc.
*/
use rand::prelude::*;
use time::PreciseTime;


pub fn random_vec(n: usize) -> Vec<i64> {
    let mut rng = rand::thread_rng();
    let random_numbers: Vec<i64> = (0..n).map(|_| {
        rng.gen_range(0, n as i64)
    }).collect();

    random_numbers
}


pub fn measure_time<T, R>(f: &Fn(T) -> R, arg: T) -> (R, i64) {
    let start = PreciseTime::now();
    let result = f(arg);
    let end = PreciseTime::now();

    let duration = start.to(end).num_milliseconds();

    (result, duration)
}

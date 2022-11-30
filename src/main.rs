use rayon::prelude::*;
use std::time::Instant;

fn main() {
    let n = 150_000_000;

    let begin = Instant::now();
    let sum: u128 = (0..n).map(|v| v * v).sum();
    let sequential_time = begin.elapsed().as_micros();

    println!("seq sum: {:?} | took: {} us", sum, sequential_time);

    let begin = Instant::now();
    let sum: u128 = (0..n).into_par_iter().map(|v| v * v).sum();
    let parallel_time = begin.elapsed().as_micros();

    println!("par sum: {:?} | took: {} us", sum, parallel_time);

    println!("speedup: {}", sequential_time as f32 / parallel_time as f32);
}

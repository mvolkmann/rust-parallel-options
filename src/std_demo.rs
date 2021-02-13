use std::{
    fs::File,
    io::{prelude::*, BufReader, Result},
    thread,
};

fn sum_file_sync(file_path: &str) -> Result<f64> {
    let f = File::open(file_path)?;
    let reader = BufReader::new(f);
    let mut sum = 0.0;
    for line in reader.lines() {
        if let Ok(n) = line?.parse::<f64>() {
            println!("{}", n);
            sum += n;
        }
    }
    Ok(sum)
}

pub fn serial() -> Result<(f64, f64)> {
    let sum1 = sum_file_sync("./numbers1.txt")?;
    let sum2 = sum_file_sync("./numbers3.txt")?;
    Ok((sum1, sum2))
}

pub fn concurrent() -> Result<(f64, f64)> {
    // std cannot do this.
    Ok((0.0, 0.0))
}

pub fn parallel_threads() -> Result<(f64, f64)> {
    // In terms of syntax, this seems like the worst option.
    let handle1 = thread::spawn(|| sum_file_sync("./numbers1.txt"));
    let handle2 = thread::spawn(|| sum_file_sync("./numbers3.txt"));
    // I don't like having to use unwrap here.
    let sum1 = handle1.join().unwrap()?;
    let sum2 = handle2.join().unwrap()?;
    Ok((sum1, sum2))
}

pub fn parallel_tasks() -> Result<(f64, f64)> {
    // std cannot do this.
    Ok((0.0, 0.0))
}

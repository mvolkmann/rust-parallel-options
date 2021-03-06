// prelude::* is needed to use the BufReader lines method.
use async_std::{
    fs::File,
    io::{BufReader, Result},
    prelude::*,
    task,
};
use futures::join;
// This is needed to call the "lines" method on a std::io::BufReader.
use std::io::prelude::*;
use std::thread;

fn sum_file_sync(file_path: &str) -> Result<f64> {
    let f = std::fs::File::open(file_path)?;
    let reader = std::io::BufReader::new(f);
    let mut sum = 0.0;
    for line in reader.lines() {
        if let Ok(n) = line?.parse::<f64>() {
            println!("{}", n);
            sum += n;
        }
    }
    Ok(sum)
}

async fn sum_file_async(file_path: &str) -> Result<f64> {
    // Must use async_std::fs::File instead of std::fs::File
    // in order to pass to async_std::io::BufReader.
    let f = File::open(file_path).await?;
    let reader = BufReader::new(f);
    let mut sum = 0.0;
    let mut stream = reader.lines();
    while let Some(Ok(line)) = stream.next().await {
        if let Ok(n) = line.parse::<f64>() {
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

/* Run serially using asynchronous functions.
let sum1 = sum_file_async("./numbers1.txt").await?;
// The next line doesn't run until
// the await from the previous line completes.
let sum2 = sum_file_async("./numbers3.txt").await?;
*/

pub async fn concurrent() -> Result<(f64, f64)> {
    let (result1, result2) = join!(
        sum_file_async("./numbers1.txt"),
        sum_file_async("./numbers3.txt")
    );
    Ok((result1?, result2?))
}

pub async fn parallel_threads() -> Result<(f64, f64)> {
    // In terms of syntax, this seems like the worst option.
    let handle1 = thread::spawn(|| sum_file_sync("./numbers1.txt"));
    let handle2 = thread::spawn(|| sum_file_sync("./numbers3.txt"));
    // The join! macro takes Futures.
    // The next line doesn't work because it is passing JoinHandles.
    //let (sum1, sum2) = join!(handle1, handle2);
    // I don't like having to use unwrap here.
    let sum1 = handle1.join().unwrap()?;
    let sum2 = handle2.join().unwrap()?;
    Ok((sum1, sum2))
}

pub async fn parallel_tasks() -> Result<(f64, f64)> {
    let handle1 = task::spawn(sum_file_async("./numbers1.txt"));
    let handle2 = task::spawn(sum_file_async("./numbers3.txt"));
    let (result1, result2) = join!(handle1, handle2);
    Ok((result1?, result2?))
}

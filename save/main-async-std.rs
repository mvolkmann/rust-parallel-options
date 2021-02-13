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

// This version is for running inside multiple threads.
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

// This version is for running concurrently inside a single thread.
// The return type, in this case a Result, is wrapped in a Future.
async fn sum_file_async(file_path: &str) -> Result<f64> {
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

#[async_std::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error + 'static>> {
    /* Run serially using synchronous functions.
    let sum1 = sum_file_sync("./numbers1.txt");
    let sum2 = sum_file_sync("./numbers3.txt");
    */

    /* Run serially using asynchronous functions.
    let sum1 = sum_file_async("./numbers1.txt").await?;
    // The next line doesn't run until
    // the await from the previous line completes.
    let sum2 = sum_file_async("./numbers3.txt").await?;
    */

    /* Run concurrently on the same thread.
    let (sum1, sum2) = join!(
        sum_file_async("./numbers1.txt"),
        sum_file_async("./numbers3.txt")
    );
    */

    /* Run in parallel using OS threads.
    // In terms of syntax, this seems like the worst option.
    let handle1 = thread::spawn(|| sum_file_sync("./numbers1.txt"));
    let handle2 = thread::spawn(|| sum_file_sync("./numbers3.txt"));
    // The async version can be run in a thread if block_on is used.
    //let handle1 = thread::spawn(|| task::block_on(sum_file_async("./numbers1.txt")));
    //let handle2 = thread::spawn(|| task::block_on(sum_file_async("./numbers3.txt")));
    // The join! macro takes Futures.
    // The next line doesn't work because it is passing JoinHandles.
    //let (sum1, sum2) = join!(handle1, handle2);
    // I don't like having to use unwrap here.
    let sum1 = handle1.join().unwrap()?;
    let sum2 = handle2.join().unwrap()?;
    */

    // Run in parallel using green threads (tasks).
    let handle1 = task::spawn(sum_file_async("./numbers1.txt"));
    let handle2 = task::spawn(sum_file_async("./numbers3.txt"));
    let (sum1, sum2) = join!(handle1, handle2);

    println!("sum1 = {:?}, sum2 = {:?}", sum1, sum2);
    Ok({})
}

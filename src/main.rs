use std::error::Error;

// mod std_demo;
// use std_demo::{concurrent_one_thread, parallel_os_threads, parallel_tasks, serial};
// fn main() -> Result<(), Box<dyn Error>> {
//     let (sum1, sum2) = serial()?;
//     println!("serial: sum1 = {:?}, sum2 = {:?}", sum1, sum2);

//     let (sum1, sum2) = concurrent_one_thread()?;
//     println!("concurrent: sum1 = {:?}, sum2 = {:?}", sum1, sum2);

//     let (sum1, sum2) = parallel_os_threads()?;
//     println!("threads: sum1 = {:?}, sum2 = {:?}", sum1, sum2);

//     let (sum1, sum2) = parallel_tasks()?;
//     println!("tasks: sum1 = {:?}, sum2 = {:?}", sum1, sum2);

//     Ok(())
// }

// mod async_std_demo;
// use async_std_demo::{concurrent_one_thread, parallel_os_threads, parallel_tasks, serial};
// #[async_std::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     let (sum1, sum2) = serial()?;
//     println!("serial: sum1 = {:?}, sum2 = {:?}", sum1, sum2);

//     //let (sum1, sum2) = concurrent_one_thread()?;
//     let (sum1, sum2) = concurrent_one_thread().await?;
//     println!("concurrent: sum1 = {:?}, sum2 = {:?}", sum1, sum2);

//     //let (sum1, sum2) = parallel_os_threads()?;
//     let (sum1, sum2) = parallel_os_threads().await?;
//     println!("threads: sum1 = {:?}, sum2 = {:?}", sum1, sum2);

//     //let (sum1, sum2) = parallel_tasks()?;
//     let (sum1, sum2) = parallel_tasks().await?;
//     println!("tasks: sum1 = {:?}, sum2 = {:?}", sum1, sum2);

//     Ok(())
// }

mod tokio_demo;
use tokio_demo::{concurrent_one_thread, parallel_os_threads, parallel_tasks, serial};
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (sum1, sum2) = serial()?;
    println!("serial: sum1 = {:?}, sum2 = {:?}", sum1, sum2);

    let (sum1, sum2) = concurrent_one_thread().await?;
    println!("concurrent: sum1 = {:?}, sum2 = {:?}", sum1, sum2);

    let (sum1, sum2) = parallel_os_threads().await?;
    println!("threads: sum1 = {:?}, sum2 = {:?}", sum1, sum2);

    let (sum1, sum2) = parallel_tasks().await?;
    println!("tasks: sum1 = {:?}, sum2 = {:?}", sum1, sum2);

    Ok(())
}

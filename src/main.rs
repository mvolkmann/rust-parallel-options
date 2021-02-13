use std::error::Error;

// mod std_demo;
// use std_demo::{concurrent, parallel_threads, parallel_tasks, serial};
// fn main() -> Result<(), Box<dyn Error>> {
//     let (sum1, sum2) = serial()?;
//     println!("serial: sum1 = {:?}, sum2 = {:?}", sum1, sum2);

//     let (sum1, sum2) = concurrent()?;
//     println!("concurrent: sum1 = {:?}, sum2 = {:?}", sum1, sum2);

//     let (sum1, sum2) = parallel_threads()?;
//     println!("threads: sum1 = {:?}, sum2 = {:?}", sum1, sum2);

//     let (sum1, sum2) = parallel_tasks()?;
//     println!("tasks: sum1 = {:?}, sum2 = {:?}", sum1, sum2);

//     Ok(())
// }

// mod async_std_demo;
// use async_std_demo::{concurrent, parallel_threads, parallel_tasks, serial};
// #[async_std::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     let (sum1, sum2) = serial()?;
//     println!("serial: sum1 = {:?}, sum2 = {:?}", sum1, sum2);

//     //let (sum1, sum2) = concurrent()?;
//     let (sum1, sum2) = concurrent().await?;
//     println!("concurrent: sum1 = {:?}, sum2 = {:?}", sum1, sum2);

//     //let (sum1, sum2) = parallel_threads()?;
//     let (sum1, sum2) = parallel_threads().await?;
//     println!("threads: sum1 = {:?}, sum2 = {:?}", sum1, sum2);

//     //let (sum1, sum2) = parallel_tasks()?;
//     let (sum1, sum2) = parallel_tasks().await?;
//     println!("tasks: sum1 = {:?}, sum2 = {:?}", sum1, sum2);

//     Ok(())
// }

mod tokio_demo;
use tokio_demo::{concurrent, parallel_tasks, parallel_threads, serial};
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (sum1, sum2) = serial()?;
    println!("serial: sum1 = {:?}, sum2 = {:?}", sum1, sum2);

    let (sum1, sum2) = concurrent().await?;
    println!("concurrent: sum1 = {:?}, sum2 = {:?}", sum1, sum2);

    let (sum1, sum2) = parallel_threads().await?;
    println!("threads: sum1 = {:?}, sum2 = {:?}", sum1, sum2);

    let (sum1, sum2) = parallel_tasks().await?;
    println!("tasks: sum1 = {:?}, sum2 = {:?}", sum1, sum2);

    Ok(())
}

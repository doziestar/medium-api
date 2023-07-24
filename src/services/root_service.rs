use std::error::Error;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use futures::stream::{self, StreamExt};
use log::{info};

#[warn(dead_code)]
async fn counter() -> Result<i32, Box<dyn Error>> {
    let array: Vec<i32> = (0..5000).collect();
    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = Arc::clone(&counter);

    stream::iter(array.into_iter())
        .for_each_concurrent(None, move |num| {
            if num > 3 {
                info!("num is greater than 3");
            } else {
                info!("num is less than 3");
            }
            counter_clone.fetch_add(1, Ordering::SeqCst);
            futures::future::ready(())
        })
        .await;

    Ok(counter.load(Ordering::SeqCst) as i32)
}
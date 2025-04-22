use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, mpsc};
use std::thread;

pub fn map_sum1<const N: usize>(data: Vec<u32>, f: fn(u32) -> u64) -> u64 {
    let len = data.len();
    let chunk = len.div_ceil(N);
    let mut handles = Vec::with_capacity(N);
    for slice in data.chunks(chunk) {
        let part = slice.to_vec();
        handles.push(thread::spawn(move || part.into_iter().map(f).sum::<u64>()));
    }
    handles.into_iter().map(|h| h.join().unwrap()).sum()
}

pub fn map_sum2<const N: usize>(data: Vec<u32>, f: fn(u32) -> u64) -> u64 {
    let counter = Arc::new(AtomicU64::new(0));
    let len = data.len();
    let chunk = len.div_ceil(N);
    let mut handles = Vec::with_capacity(N);
    for slice in data.chunks(chunk) {
        let atomic = Arc::clone(&counter);
        let part = slice.to_vec();
        handles.push(thread::spawn(move || {
            for v in part {
                atomic.fetch_add(f(v), Ordering::Relaxed);
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    counter.load(Ordering::Relaxed)
}

pub fn map_sum3<const N: usize>(data: Vec<u32>, f: fn(u32) -> u64) -> u64 {
    let (tx, rx) = mpsc::channel();
    let len = data.len();
    let chunk = len.div_ceil(N);
    for slice in data.chunks(chunk) {
        let tx_clone = tx.clone();
        let part = slice.to_vec();
        thread::spawn(move || {
            let sum: u64 = part.into_iter().map(f).sum();
            tx_clone.send(sum).unwrap();
        });
    }
    drop(tx);
    rx.iter().sum()
}

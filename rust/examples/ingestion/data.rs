use chrono::{DateTime, Utc};
use rand::Rng;
use std::{
    sync::mpsc::{channel, Receiver},
    thread,
    time::{Duration, Instant},
};

pub fn data_source() -> Receiver<(DateTime<Utc>, f64)> {
    let (tx, rx) = channel();
    thread::spawn(move || {
        let duration = Duration::from_secs(60);
        let start = Instant::now();
        let mut rng = rand::thread_rng();

        while Instant::now().duration_since(start) < duration {
            tx.send((Utc::now(), rng.gen_range(0.0..100.0))).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });
    rx
}

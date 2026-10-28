use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

use slint::{Rgba8Pixel, SharedPixelBuffer};

use crate::schedule;

/// Messages sent from the worker thread(s) back to the UI thread.
pub enum Msg {
    Rows(Vec<schedule::StreamRow>),
    Image(usize, SharedPixelBuffer<Rgba8Pixel>),
    Error(String),
}

/// Spawns a background thread that fetches the schedule, then fans
/// out thumbnail downloads across a bounded pool of 6 workers.
pub fn spawn(tx: mpsc::Sender<Msg>) {
    thread::spawn(move || run(tx));
}

fn run(tx: mpsc::Sender<Msg>) {
    let rows = match schedule::fetch_streams() {
        Ok(r) => r,
        Err(e) => {
            let _ = tx.send(Msg::Error(e));
            return;
        }
    };

    let _ = tx.send(Msg::Rows(rows.clone()));

    let jobs: Vec<(usize, String)> = rows
        .into_iter()
        .enumerate()
        .map(|(i, r)| (i, r.pic_url))
        .collect();

    let queue = Arc::new(Mutex::new(jobs.into_iter()));
    let worker_count = 6;

    for _ in 0..worker_count {
        let queue = Arc::clone(&queue);
        let tx = tx.clone();
        thread::spawn(move || loop {
            let next = {
                let mut q = queue.lock().unwrap();
                q.next()
            };
            let Some((i, url)) = next else { break };

            if let Some(buffer) = schedule::load_image(&url) {
                let _ = tx.send(Msg::Image(i, buffer));
            }
        });
    }
}
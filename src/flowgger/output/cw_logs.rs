use super::Output;
use crate::flowgger::config::Config;
use crate::flowgger::merger::Merger;
use std::io::{stdout, Write};
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct CloudWatchLogsOutput;

impl CloudWatchLogsOutput {
    pub fn new(_config: &Config) -> CloudWatchLogsOutput {
        CloudWatchLogsOutput
    }
}

impl Output for CloudWatchLogsOutput {
    fn start(&self, arx: Arc<Mutex<Receiver<Vec<u8>>>>, merger: Option<Box<dyn Merger>>) {
        let merger = match merger {
            Some(merger) => Some(merger.clone_boxed()),
            None => None,
        };
        thread::spawn(move || loop {
            let mut bytes = match { arx.lock().unwrap().recv() } {
                Ok(line) => line,
                Err(_) => return,
            };
            if let Some(ref merger) = merger {
                merger.frame(&mut bytes);
            }
            let out = String::from_utf8_lossy(&bytes);
            print!("CW: {}", out);
            let _ = stdout().flush();
        });
    }
}

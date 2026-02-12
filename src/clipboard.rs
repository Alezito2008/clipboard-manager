use std::{collections::VecDeque, sync::{Arc, Mutex}, thread, time::Duration};

use arboard::Clipboard;

const POLLING_RATE_MS: u64 = 500;
const MAX_ITEMS: usize = 5;

pub fn start_watcher(contents: Arc<Mutex<VecDeque<String>>>) {
    thread::spawn(move || {
        let mut clipboard = Clipboard::new().expect("Error getting clipboard");
        let mut last: String = String::new();

        loop {
            thread::sleep(Duration::from_millis(POLLING_RATE_MS));

            let Ok(clipboard_text) = clipboard.get_text() else {
                println!("Error getting clipboard text");
                continue;
            };

            if clipboard_text != last {
                let mut contents = contents.lock().unwrap();

                if contents.len() >= MAX_ITEMS {
                    contents.pop_front();
                }

                contents.push_back(clipboard_text.clone());
                println!("Contents: {contents:?}");
                last = clipboard_text;
            }
        }
    });
}

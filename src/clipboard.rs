use std::{
    collections::VecDeque,
    sync::{Arc, Mutex, mpsc::Receiver},
    thread,
    time::Duration,
};

use arboard::Clipboard;

const POLLING_RATE_MS: u64 = 500;
const MAX_ITEMS: usize = 5;

pub enum ClipboardCommand {
    Set(String),
}

pub fn start_watcher(contents: Arc<Mutex<VecDeque<String>>>, rx: Receiver<ClipboardCommand>) {
    thread::spawn(move || {
        let mut clipboard = Clipboard::new().expect("Error getting clipboard");
        let mut last: String = String::new();

        loop {
            thread::sleep(Duration::from_millis(POLLING_RATE_MS));

            while let Ok(cmd) = rx.try_recv() {
                match cmd {
                    ClipboardCommand::Set(text) => {
                        if let Err(err) = clipboard.set_text(&text) {
                            eprintln!("Error setting clipboard text: {err:?}")
                        }
                        
                        last = text;
                    }
                }
            }

            let Ok(clipboard_text) = clipboard.get_text() else {
                eprintln!("Error getting clipboard text");
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

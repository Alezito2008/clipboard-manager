use std::{env, fs};

use fs2::FileExt;

pub struct SingleInstance {
    _file: std::fs::File
}

impl SingleInstance {
    pub fn acquire() -> Option<Self> {
        let path = env::temp_dir().join("clipboard-manager.lock");

        let file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path).ok()?;

        if file.try_lock_exclusive().is_err() {
            return None
        }

        Some(Self { _file: file })
    }
}

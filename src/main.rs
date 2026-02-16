use std::{
    collections::VecDeque,
    sync::{
        Arc, Mutex,
        mpsc::{self, Receiver, Sender},
    },
};

mod app;
mod clipboard;
mod file_lock;

use app::MyApp;
use eframe::egui;

use crate::{clipboard::ClipboardCommand, file_lock::SingleInstance};

fn main() -> eframe::Result {
    let _instance = match SingleInstance::acquire() {
        Some(instance) => instance,
        None => {
            println!("Instance already running!");
            return Ok(())
        }
    };

    let contents = Arc::new(Mutex::new(VecDeque::<String>::new()));
    let (tx, rx): (Sender<ClipboardCommand>, Receiver<ClipboardCommand>) = mpsc::channel();

    clipboard::start_watcher(Arc::clone(&contents), rx);

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_visible(false)
            .with_inner_size((300.0, 500.0))
            .with_decorations(false)
            .with_movable_by_background(true)
            .with_always_on_top()
            .with_transparent(true),
        ..Default::default()
    };

    eframe::run_native(
        "Clipboard Manager",
        options,
        Box::new(|_| Ok(Box::new(MyApp::new(contents, tx)))),
    )
}

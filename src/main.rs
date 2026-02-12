use std::{collections::VecDeque, sync::{Arc, Mutex}};

mod clipboard;
mod app;

use app::MyApp;
use eframe::egui;

fn main() -> eframe::Result {
    let contents = Arc::new(Mutex::new(VecDeque::<String>::new()));

    clipboard::start_watcher(Arc::clone(&contents));
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size((300.0, 500.0))
        .with_decorations(false)
        .with_movable_by_background(true)
        .with_always_on_top(),
        ..Default::default()
    };

    eframe::run_native(
        "Clipboard Manager",
        options,
        Box::new(|_|
            Ok(Box::new(MyApp::new(contents))),
        )
    )
}
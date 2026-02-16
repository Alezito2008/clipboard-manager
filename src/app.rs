use std::{
    collections::VecDeque,
    sync::{Arc, Mutex, mpsc::Sender},
    time::Duration,
};

use eframe::egui::{self, Align, Color32, Id, Layout, RichText, Sense, scroll_area::ScrollSource};

use crate::clipboard;

pub struct MyApp {
    contents: Arc<Mutex<VecDeque<String>>>,
    clipboard_tx: Sender<clipboard::ClipboardCommand>
}

impl MyApp {
    pub fn new(contents: Arc<Mutex<VecDeque<String>>>, clipboard_tx: Sender<clipboard::ClipboardCommand>) -> Self {
        Self { contents, clipboard_tx }
    }
}

impl MyApp {
    fn setup_styles(&self, ui: &mut egui::Ui) {
        let visuals = &mut ui.style_mut().visuals;
        visuals.widgets.active.fg_stroke.color = egui::Color32::RED;
        visuals.widgets.noninteractive.fg_stroke.color = egui::Color32::WHITE;
    }

    fn handle_drag(&self, ui: &mut egui::Ui, ctx: &eframe::egui::Context) {
        let res = ui.interact(
            ui.available_rect_before_wrap(),
            Id::new("Drag"),
            Sense::click_and_drag(),
        );

        if res.drag_started_by(egui::PointerButton::Primary) {
            ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
        }
    }

    fn titlebar(&self, ui: &mut egui::Ui, ctx: &eframe::egui::Context) {
        egui::Frame::new()
            .inner_margin(egui::Margin::symmetric(10, 5))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.heading("Clipboard");
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        let close_button =
                            ui.add(egui::Button::new(RichText::new("x").size(20.0)).frame(false));
                        if close_button.clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                });
            });
    }

    fn clipboard_item(&self, ui: &mut egui::Ui, index: usize, text: &str) {
        let frame = egui::Frame::default()
            .stroke(ui.visuals().widgets.inactive.bg_stroke)
            .fill(Color32::from_white_alpha(10))
            .inner_margin(10)
            .outer_margin(5)
            .corner_radius(5);

        let mut prepared = frame.begin(ui);

        {
            let ui = &mut prepared.content_ui;

            ui.set_width(ui.available_width());
            egui::ScrollArea::vertical()
                .max_height(50.0)
                .id_salt(index)
                .scroll_source(ScrollSource::NONE)
                .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                .show(ui, |ui| {
                    ui.label(text.trim());
                });
        }

        let rect = prepared.allocate_space(ui).rect;
        let frame_response = ui.interact(rect, ui.id().with(index), Sense::click());

        if frame_response.hovered() {
            prepared.frame.fill = Color32::from_white_alpha(30);
            prepared.frame.stroke = egui::Stroke::new(1.0, Color32::CYAN);
            prepared.frame.inner_margin -= 1;
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }

        if frame_response.clicked() {
            self.clipboard_tx.send(
                clipboard::ClipboardCommand::Set(text.into())
            ).ok();
        }

        // prepared.paint(ui);

        let dots_rect = egui::Rect::from_min_size(
            egui::Pos2::new(rect.right() - 40.0, rect.top() + 12.0),
            egui::Vec2::new(30.0, 15.0),
        );

        ui.put(
            dots_rect,
            egui::Label::new(
                egui::RichText::new("Pin")
                    .color(egui::Color32::CYAN)
                    .underline(),
            ),
        );

        prepared.end(ui);
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        let contents = {
            let guard = self.contents.lock().unwrap();
            guard.clone() // free contents mutex
        };

        egui::TopBottomPanel::top("titlebar").show(ctx, |ui| {
            self.setup_styles(ui);
            self.handle_drag(ui, ctx);

            self.titlebar(ui, ctx);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.setup_styles(ui);

            egui::ScrollArea::vertical().show(ui, |ui| {
                for (index, text) in contents.iter().enumerate() {
                    self.clipboard_item(ui, index, text)
                }
            });
        });

        egui::TopBottomPanel::bottom("footer").show(ctx, |ui| {
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add(egui::Hyperlink::from_label_and_url(
                    "Source",
                    "https://github.com/Alezito2008",
                ))
            })
        });

        ctx.request_repaint_after(Duration::from_millis(100));
    }
}

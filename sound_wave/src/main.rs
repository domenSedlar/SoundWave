#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod file_maneger;
mod player;

use eframe::egui;

use crate::file_maneger::FileManager;
use player::SoundPlayer;
fn main(){

    // Log to stdout (if you run with `RUST_LOG=debug`).
    //tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(850.0, 480.0)),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

pub trait Window {
    fn get_window(&mut self, ui: &mut egui::Ui);
}

struct MyApp {
    fm: FileManager,
    pl: SoundPlayer
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            fm:FileManager::default(),
            pl: SoundPlayer::default()
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::TopBottomPanel::bottom("bottom_panel")
                .resizable(false)
                .min_height(50.0)
                .show_inside(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        self.pl.get_window(ui);
                    });
                });

            egui::SidePanel::left("left_panel")
                .resizable(true)
                .default_width(150.0)
                .width_range(80.0..=200.0)
                .show_inside(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("Left Panel");
                    });
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        file_maneger::lorem_ipsum(ui);
                    });
                });

            egui::TopBottomPanel::top("top_panel")
                .resizable(true)
                .min_height(32.0)
                .show_inside(ui, |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.heading("Top Panel");
                        });
                        file_maneger::lorem_ipsum(ui);
                    });
                });


            egui::CentralPanel::default().show_inside(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Central Panel");
                });
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.fm.get_window(ui);
                });
            });

        });
    }
}

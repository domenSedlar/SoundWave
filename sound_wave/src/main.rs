#![cfg_attr(not(debug_assertions), windows_subsystem = "main_window")]

mod main_window;

use eframe::egui;
use egui_extras::RetainedImage;
use gstreamer::glib::Char;
//use gstreamer::glib::OptionArg::String;

use crate::main_window::Windows;


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


struct MyApp {
    w: Windows,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            w: Windows::default(),
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
                        self.w.get_controller(ui);
                    });
                });

            egui::SidePanel::left("left_panel")
                .resizable(true)
                .default_width(150.0)
                .width_range(80.0..=200.0)
                .show_inside(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.heading("SoundWave");
                    });
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        self.w.get_tabs_window(ui);
                    });
                });

            egui::TopBottomPanel::top("top_panel")
                .resizable(true)
                .min_height(32.0)
                .show_inside(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            let (s, i) = self.w.get_current_song_data();
                            match i {
                                None => {}
                                Some(a) => {a.show_size(ui, egui::Vec2::new(200.0, 200.0));}
                            }
                            ui.heading(s);
                        });
                    self.w.get_top_window(ui);
                });

            egui::CentralPanel::default().show_inside(ui, |ui| {
                ui.vertical(|ui| {
                    ui.heading(&self.w.main_text);
                });
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.w.get_main_window(ui);
                });
            });

        });
    }
}

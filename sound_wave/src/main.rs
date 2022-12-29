#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod file_maneger;
mod controller;

use eframe::egui;
use gstreamer::glib::Char;
//use gstreamer::glib::OptionArg::String;

use crate::file_maneger::FileManager;
use controller::Controller;


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

pub struct Song {
    path: String,
    name: String,
    artist: String,
    album: String,
    year: String
}

impl Song{
    pub fn default() -> Song{
        return Song{
            path: String::new(),
            name: String::new(),
            artist: String::new(),
            album: String::new(),
            year: String::new()
        }
    }

    pub fn serialize(self, terminator: char) -> String{
        return format!(
            "{p}{t}{n}{t}{a}{t}{al}{t}{y}{t}",
            p = self.path,
            t = terminator,
            n = self.name,
            a = self.artist,
            al = self.album,
            y = self.year
        )
    }

    pub fn deserialize(s: String, terminator: char) -> Song{
        let mut vars: [String; 5] = [String::new(), String::new(), String::new(), String::new(), String::new()];
        let mut a = 0;
        let mut i = 0;

        let s = s.chars().collect::<Vec<char>>();

        while a < 5 && i < s.len()-1{
            if s.get(i).unwrap() == &';' {a+=1; continue}
            vars[a].push(*s.get(i).unwrap());
            i += 1;
        }

        return Song{
            path: String::from(String::from(&vars[0])),
            name: String::from(String::from(&vars[1])),
            artist: String::from(String::from(&vars[2])),
            album: String::from(String::from(&vars[3])),
            year: String::from(String::from(&vars[4]))
        }
    }
}

pub trait Window {
    fn get_window(&mut self, ui: &mut egui::Ui);
}

struct MyApp {
    fm: FileManager,
    pl: Controller
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            fm:FileManager::default(),
            pl: Controller::default()
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

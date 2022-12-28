#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fmt::Error;
// hide console window on Windows in release
use std::sync::mpsc;
use eframe::{App, egui, Renderer};
mod play_backend;
use play_backend::{start, Command};
use std::thread;
use egui::Ui;
use egui::Vec2;


use single_value_channel::channel_starting_with;

fn idk() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    //tracing_subscriber::fmt::init();
    let (tx1, rx1) = mpsc::channel::<Command>();
    let (mut rx2, tx2) = channel_starting_with::<u64>(0);
    let handle = thread::spawn(move || start(rx1, tx2));

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };

    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new( SoundPlayer {tx: tx1,rx: rx2, slider_value: 0, position: 0, repositiong: false})),
    );
    println!("done");
}

pub struct SoundPlayer {
    tx: mpsc::Sender<Command>,
    rx: single_value_channel::Receiver<u64>,
    slider_value: u64,
    position: u64,
    repositiong: bool
}

impl SoundPlayer {
    pub fn default() -> SoundPlayer {
        let (tx1, rx1) = mpsc::channel::<Command>();
        let (mut rx2, tx2) = channel_starting_with::<u64>(0);
        let handle = thread::spawn(move || start(rx1, tx2));
        SoundPlayer {tx: tx1,rx: rx2, slider_value: 0, position: 0, repositiong: false}
    }
}

impl super::Window for SoundPlayer {
    fn get_window(&mut self, ui: &mut Ui) {
        if self.repositiong{
            let b = *self.rx.latest();
            if b == self.slider_value{
                self.position = b;
                self.slider_value = self.position;
                self.repositiong = false;
            }
        }
        else if self.position != self.slider_value{
            self.tx.send(Command::SetPosInSeconds(self.slider_value));
            self.repositiong = true;
        }
        else {
            self.position = *self.rx.latest();
            self.slider_value = self.position;
        }

        ui.columns(3,|columns| {
            if columns[1].button("|>").clicked() {
                let a = self.tx.send(Command::PlayPause);
                println!("{:?}", a);
            }
            if columns[2].button(">>").clicked() {
                let a = self.tx.send(Command::Forward);
                println!("I work?");
            }
            if columns[0].button("<<").clicked() {
                let a = self.tx.send(Command::Back);
                println!("Back!");
            }
        });

        if ui.add(egui::Slider::new(&mut self.slider_value, 0..=100)).clicked(){
                ///TODO
                println!("tako bi lahko lažje naredil rewind");
            };
    }
}

impl eframe::App for SoundPlayer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.repositiong{
            let b = *self.rx.latest();
            if b == self.slider_value{
                self.position = b;
                self.slider_value = self.position;
                self.repositiong = false;
            }
        }
        else if self.position != self.slider_value{
            self.tx.send(Command::SetPosInSeconds(self.slider_value));
            self.repositiong = true;
        }
        else {
            self.position = *self.rx.latest();
            self.slider_value = self.position;
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.columns(3,|columns| {
                if columns[1].button("|>").clicked() {
                    let a = self.tx.send(Command::PlayPause);
                    println!("{:?}", a);
                }
                if columns[2].button(">>").clicked() {
                    let a = self.tx.send(Command::Forward);
                    println!("I work?");
                }
                if columns[0].button("<<").clicked() {
                    let a = self.tx.send(Command::Back);
                    println!("Back!");
                }
            });

            if ui.add(egui::Slider::new(&mut self.slider_value, 0..=100)).clicked(){
                ///TODO
                println!("tako bi lahko lažje naredil rewind");
            };
        });
        egui::Context::request_repaint_after(ctx, std::time::Duration::from_millis(1000));

    }
}
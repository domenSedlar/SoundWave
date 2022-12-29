use std::fmt::Error;
// hide console window on Windows in release
use std::sync::mpsc;
use eframe::{App, egui, Renderer};

use std::thread;
use egui::Ui;
use egui::Vec2;
use single_value_channel::channel_starting_with;

mod play_backend;
use play_backend::start;

pub struct Controller {
    tx: mpsc::Sender<Command>,
    rx: single_value_channel::Receiver<u64>,
    position: u64,
}

impl Controller {
    pub fn default() -> Controller {
        let (tx1, rx1) = mpsc::channel::<Command>();
        let (mut rx2, tx2) = channel_starting_with::<u64>(0);
        let handle = thread::spawn(move || start(rx1, tx2));
        Controller {tx: tx1,rx: rx2, position: 0}
    }
}

impl super::Window for Controller {
    fn get_window(&mut self, ui: &mut Ui) {

        self.position = *self.rx.latest();


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

        if ui.add(egui::Slider::new(&mut self.position, 0..=100)).drag_released(){
            ///TODO
            println!("tako bi lahko lažje naredil rewind");
            self.tx.send(Command::SetPosInSeconds(self.position));

        };
    }
}

impl eframe::App for Controller {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

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

            if ui.add(egui::Slider::new(&mut self.position, 0..=100)).clicked(){
                ///TODO
                println!("tako bi lahko lažje naredil rewind");
            };
        });
        egui::Context::request_repaint_after(ctx, std::time::Duration::from_millis(1000));

    }
}

pub enum Command {
    PlayPause,
    Forward,
    Back,
    SetPosInSeconds(u64),
    Quit,
}


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
        Box::new(|_cc| Box::new( Controller {tx: tx1,rx: rx2, position: 0})),
    );
    println!("done");
}

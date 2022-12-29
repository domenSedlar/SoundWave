use std::fmt::Error;
// hide console window on Windows in release
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use eframe::{App, egui, Renderer};

use std::thread;
use std::thread::JoinHandle;
use egui::Ui;
use egui::Vec2;
use single_value_channel::channel_starting_with;

mod play_backend;
use play_backend::start;
use super::Song;

pub struct Controller {
    player: Player,
    position: u64,
    list: Vec<Song>,
    index: usize
}

impl Controller {
    pub fn default() -> Controller {
        Controller {player: Player::default() , position: 0, list: Vec::new(), index: 0}
    }

    pub fn next(&mut self){
        match self.list.get(self.index){
            None => {}
            Some(a) => {self.player.play(&a.path);}
        }
    }
}

impl super::Window for Controller {
    fn get_window(&mut self, ui: &mut Ui) {
        ui.columns(3,|columns| {
            if columns[1].button("|>").clicked() {
                let a = self.player.send(Command::PlayPause);
                println!("{:?}", a);
            }
            if columns[2].button(">>").clicked() {
                let a = self.player.send(Command::Forward);
                println!("I work?");
            }
            if columns[0].button("<<").clicked() {
                let a = self.player.send(Command::Back);
                println!("Back!");
            }
        });

        if ui.add(egui::Slider::new(&mut self.position, 0..=100)).drag_released(){
            ///TODO
            println!("tako bi lahko lažje naredil rewind");
            self.player.send(Command::SetPosInSeconds(self.position));
        };

        if self.position == 100 {
            self.next();
        }
        self.position = self.player.recv();
    }
}

pub enum Command {
    PlayPause,
    Forward,
    Back,
    SetPosInSeconds(u64),
    Quit,
}

pub struct Player {
    tx: Option<mpsc::Sender<Command>>,
    rx: single_value_channel::Receiver<u64>,
    handle : Option<JoinHandle<()>>
}

impl Player {
    pub fn default() -> Player{
        let (rx, tx) = channel_starting_with::<u64>(0);

        Player{
            tx: Option::None,
            rx,
            handle: Option::None
        }
    }
    pub fn recv(&mut self) -> u64{
        match self.tx{
            None => {0}
            Some(_) => {*self.rx.latest()}
        }
    }

    pub fn send(&mut self, c: Command){
        match &self.tx {
            None => {}
            Some(a) => {a.send(c);}
        }
    }

    pub fn play(&mut self, song: &str){
        let (tx1, rx1) = mpsc::channel::<Command>();
        let (mut rx2, tx2) = channel_starting_with::<u64>(0);
        let handle = thread::spawn(move || start("/home/blue/Music/NewPlaylists/Jinx/Just_Dropped_In.mp3",rx1, tx2));
        self.tx = Option::Some(tx1);
        self.rx = rx2;
        self.handle = Option::Some(handle);
    }
}
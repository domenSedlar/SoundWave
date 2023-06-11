use std::fmt::Error;
// hide console window on Windows in release
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use eframe::{App, egui, Renderer};

use std::thread;
use std::thread::JoinHandle;
use egui::Ui;
use egui::Vec2;
use gstreamer::glib::listenv;
use single_value_channel::channel_starting_with;

use rand::Rng;

mod play_backend;
use play_backend::start;
use super::Song;

pub struct Controller {
    player: Player,
    position: u64,
    pub(crate) list: Vec<Song>,
    pub(crate) index: usize
}

impl Controller {
    pub fn default() -> Controller {
        Controller {player: Player::default() , position: 0, list: Vec::new(), index: 0}
    }

    pub fn play_next(&mut self, s: Song){
        let mut ls: Vec<Song> = vec![];
        let mut i = 0;
        let mut ds: Song;
        while self.list.len() > 0{
            ds = self.list.remove(0);
            if !(s.same_song(&ds)){
                ls.push(ds);
            }
            if i == self.index{
                ls.push(Song::clone(&s));
            }
            i += 1;
        }
        self.list = ls;
    }

    fn next(&mut self){
        self.index += 1;
        match self.list.get(self.index){
            None => {}
            Some(a) => {self.player.play(String::from(&a.path));}
        }
    }

    pub fn get_current_song(&self) -> Song{
        match &self.list.get(self.index) {
            None => {return Song::default()}
            Some(a) => {return Song::clone(a)}
        }
    }

    fn back(&mut self){
        if self.index == 0{
            return;
        }
        self.index -= 1;
        match self.list.get(self.index){
            None => {}
            Some(a) => {self.player.play(String::from(&a.path));}
        }
    }

    pub fn play(&mut self, list: Vec<Song>, i:usize){
        self.index = i;
        self.list = list;

        match self.list.get(self.index){
            None => {}
            Some(a) => {self.player.play(String::from(&a.path));}
        }
    }

    pub fn to(&mut self, i:usize){
        self.index = i;

        match self.list.get(self.index){
            None => {}
            Some(a) => {self.player.play(String::from(&a.path));}
        }
    }

    pub fn get_window(&mut self, ui: &mut Ui) {
        self.position = self.player.recv() * self.player.get_len() / 100;

        ui.columns(5,|columns| {
            if columns[0].button("<").clicked() {
                let a = self.back();
                println!("previous song");
            }
            if columns[2].button("|>").clicked() {
                let a = self.player.send(Command::PlayPause);
                println!("{:?}", a);
            }
            if columns[3].button(">>").clicked() {
                let a = self.player.send(Command::Forward);
                println!("I work?");
            }
            if columns[1].button("<<").clicked() {
                let a = self.player.send(Command::Back);
                println!("Back!");
            }
            if columns[4].button(">").clicked() {
                let a = self.next();
                println!("next");
            }
        });
        ui.horizontal(|ui| {
            ui.spacing_mut().slider_width = ui.available_width()- 100.0;
            if ui.add(egui::Slider::new(&mut self.position, 0..=self.player.get_len())).drag_released()
            {
                let q = self.player.get_len();
                self.player.send(Command::SetPosInSeconds(self.position * 100 / q));
            }
            ui.label(self.player.get_len().to_string());
            egui::Context::request_repaint_after(ui.ctx(), std::time::Duration::from_millis(250));
        });

        if self.position == self.player.get_len() {
            self.next();
        }
    }

    fn true_shuffle(mut ols: Vec<Song>) -> Vec<Song>{
        let mut ls = vec![];

        while ols.len() > 0 {
            ls.push(ols.remove(rand::thread_rng().gen_range(0..ols.len())));
        }
        ls
    }

    pub fn play_true_shuffle(&mut self, pls: &Vec<Song>){
        self.play(Controller::true_shuffle(Song::clone_ls(pls)),0);
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
    lrx: single_value_channel::Receiver<u64>,
    handle : Option<JoinHandle<()>>
}

impl Player {
    pub fn default() -> Player{
        let (rx, tx) = channel_starting_with::<u64>(0);
        let (lrx, ltx) = channel_starting_with::<u64>(100);

        Player{
            tx: Option::None,
            rx,
            lrx,
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

    fn play(&mut self, song: String){

        self.send(Command::Quit);

        let (tx1, rx1) = mpsc::channel::<Command>();
        let (mut rx2, tx2) = channel_starting_with::<u64>(0);
        let (mut lrx2, ltx2) = channel_starting_with::<u64>(100);
        println!("{}", &song);
        let handle = thread::spawn(move || start(song ,rx1, tx2, ltx2));
        self.tx = Option::Some(tx1);
        self.rx = rx2;
        self.lrx = lrx2;
        self.handle = Option::Some(handle);
    }

    pub fn get_len(&mut self) -> u64{
        match self.tx{
            None => {100}
            Some(_) => {*self.lrx.latest()}
        }
    }
}
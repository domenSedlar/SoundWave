use std::collections::HashMap;
use std::path::PathBuf;
use std::thread::current;
use egui::Color32;

use rfd::FileDialog;
use egui::*;
//use gstreamer::glib::OptionArg::String as OtherString;

mod controller;
mod file_manager;
mod song;
mod playlists;

use song::Song;
use file_manager::FileManager;
use controller::Controller;
use playlists::Playlists;


enum CurrentWindow{
    Files,
    Playlists,
    Playlist,
    Queue
}

pub struct Windows {
    file_manager: FileManager,
    controller: Controller,
    playlists: Playlists,

    current_window: CurrentWindow
}

impl Windows {
    pub fn default() -> Windows {
        return Windows {
            file_manager: FileManager::default(),
            controller: Controller::default(),
            playlists: Playlists::default(),
            current_window: CurrentWindow::Files
        }
    }
    pub fn get_tabs_window(&mut self, ui: &mut egui::Ui){
        if ui.button("Browse files").clicked(){
            self.current_window = CurrentWindow::Files;
        }
        if ui.button("Playlists").clicked(){
            self.current_window = CurrentWindow::Playlists;
        }
    }

    pub fn get_controller(&mut self, ui: &mut egui::Ui) {
        egui::SidePanel::left("Btn").max_width(5.0).show_inside(ui, |ui| {
            if ui.button("=").clicked(){
                self.current_window = CurrentWindow::Queue;
            }
        });
        self.controller.get_window(ui);
    }

    pub fn get_main_window(&mut self, ui: &mut egui::Ui){
        match self.current_window{
            CurrentWindow::Files => {self.get_file_manager_window(ui)},
            CurrentWindow::Queue => {self.get_queue_window(ui)}
            CurrentWindow::Playlists => {self.get_playlists_window(ui)}
            _ => {}
        }
    }

    fn get_playlists_window(&mut self, ui: &mut egui::Ui){
        self.playlists.get_main_window(ui);

    }

    fn get_queue_window(&mut self, ui: &mut egui::Ui){
        let text_style = TextStyle::Body;
        let row_height = ui.text_style_height(&text_style);
        let num_rows = self.controller.list.len();

        ScrollArea::vertical().auto_shrink([false; 2]).show_rows(
            ui,
            row_height,
            num_rows,
            |mut ui, row_range| {
                for row in row_range {
                    match &self.controller.list.get(row){
                        None => {}
                        Some(s) => {
                            let mut text = format!("{0} - {1}", s.artist, s.name);

                            if text == " - "{
                                text = file_manager::fm_backend::nm_from_path(&s.path);
                            }
                            let r = &s.get_panel2(ui, &row, row== self.controller.index);
                            if r.clicked(){
                                self.controller.to(row);
                            }

                        }
                    };

                }
            },
        );
    }

    pub fn get_file_manager_window(&mut self, ui: &mut egui::Ui) {
        if ui.button("add path").clicked(){
            let new_path = match FileDialog::new().pick_folder(){
                None => {String::new()}
                Some(a) => {String::from(a.to_str().unwrap())}
            };
            file_manager::fm_backend::add_path(file_manager::fm_backend::nm_from_path(&new_path), new_path);
            self.file_manager.current_location = String::new();
            self.file_manager.shown_location = String::from("meow");
            self.file_manager.paths = file_manager::fm_backend::get_paths();
        }
        if ui.button("..").clicked(){
            if self.file_manager.current_location != self.file_manager.root{
                self.file_manager.current_location.pop();
                self.file_manager.current_location = file_manager::fm_backend::path_from_name(&self.file_manager.current_location);
                println!("{}", self.file_manager.current_location);
            }
            else{
                self.file_manager.current_location = String::new();
            }
        }

        ///Showing root folders
        if self.file_manager.current_location == String::new(){
            if self.file_manager.current_location != self.file_manager.shown_location{
                println!("doin stuff");
                self.file_manager.items= (Vec::new(), Vec::new());

                for (k, v) in &self.file_manager.paths{
                    self.file_manager.items.0.push(String::from(k));
                }
                self.file_manager.shown_location = String::from(&self.file_manager.current_location);
            }

            let text_style = TextStyle::Body;
            let row_height = ui.text_style_height(&text_style);
            let num_rows = self.file_manager.items.0.len();
            ScrollArea::vertical().auto_shrink([false; 2]).show_rows(
                ui,
                row_height,
                num_rows,
                |ui, row_range| {
                    for row in row_range {
                        let text = match self.file_manager.items.0.get(row){
                            None => {String::new()}
                            Some(s) => {String::from(s)}
                        };
                        if text == String::new() {break;}
                        //self.buttons.insert(String::from(&text), String::from(&path));
                        if ui.button(&text).clicked(){
                            self.file_manager.current_location = String::from(self.file_manager.paths.get(&text).unwrap());
                            println!("{}", self.file_manager.current_location);
                            self.file_manager.root = String::from(self.file_manager.paths.get(&text).unwrap());
                        };
                    }
                },
            );
        }
        else {
            ///Showing subfolders
            if self.file_manager.current_location != self.file_manager.shown_location{
                self.file_manager.items = file_manager::fm_backend::ls_all_in_dir(&self.file_manager.current_location);
                self.file_manager.shown_location = String::from(&self.file_manager.current_location);
            }

            let curr_song = &self.controller.get_current_song();
            let num_of_dirs = self.file_manager.items.0.len();
            let text_style = TextStyle::Body;
            let row_height = ui.text_style_height(&text_style);
            let num_rows = num_of_dirs + self.file_manager.items.1.len();

            ScrollArea::vertical().auto_shrink([false; 2]).show_rows(
                ui,
                row_height,
                num_rows,
                |mut ui, row_range| {
                    for row in row_range {
                        ///folders
                        if row < num_of_dirs{
                            let path = match self.file_manager.items.0.get(row){
                                None => {String::new()}
                                Some(s) => {String::from(s)}
                            };
                            let text = file_manager::fm_backend::nm_from_path(&path);
                            self.file_manager.buttons.insert(String::from(&text), String::from(&path));
                            if ui.button(text).clicked(){
                                self.file_manager.current_location = path;
                                println!("{}", self.file_manager.current_location);
                            };
                        }
                        else{
                            ///songs
                            match self.file_manager.items.1.get(row-num_of_dirs){
                                None => {}
                                Some(s) => {
                                    let mut text = format!("{0} - {1}", s.artist, s.name);

                                    if text == " - "{
                                        text = file_manager::fm_backend::nm_from_path(&s.path);
                                    }
                                        let r = &s.get_panel2(ui, &row, curr_song == &s.path);
                                        if r.clicked(){
                                            let mut pls = Song::clone_ls(&self.file_manager.items.1);
                                            let i: usize = Song::find(&s.path, &mut pls);
                                            self.controller.play(pls, i);
                                        }

                                }
                            };

                        }
                    }
                },
            );
        }
    }

}

fn scroll_area_template(ui: &mut egui::Ui) {
    ui.label(
        "",
    );
    ui.add_space(4.0);

    let text_style = TextStyle::Body;
    let row_height = ui.text_style_height(&text_style);
    let num_rows = 10_000;
    ScrollArea::vertical().auto_shrink([false; 2]).show_rows(
        ui,
        row_height,
        num_rows,
        |ui, row_range| {
            for row in row_range {
                let text = format!("This is row {}/{}", row + 1, num_rows);
                ui.label(text);
            }
        },
    );
}

pub fn lorem_ipsum(ui: &mut egui::Ui) {
    ui.label(
        "Template Text",
    );
}
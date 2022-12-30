use std::collections::HashMap;
use std::path::PathBuf;
use std::thread::current;

use rfd::FileDialog;
use egui::*;
//use gstreamer::glib::OptionArg::String as OtherString;

use crate::Window;
mod fm_backend;
use super::Song;

pub struct FileManager{
    paths: HashMap<String,String>,
    buttons: HashMap<String,String>,
    current_location: String,
    shown_location: String,
    items: (Vec<String>, Vec<Song>),
    root: String
}

impl FileManager{
    pub fn default() -> FileManager{
        let paths = fm_backend::get_paths();
        let mut items= (Vec::new(), Vec::new());

        for (k, v) in &paths{
            items.0.push(String::from(k));
        }

        return FileManager{
            paths,
            buttons: HashMap::new(),
            current_location: String::new(),
            shown_location: String::new(),
            items,
            root: String::new()
        }
    }
}

impl super::Window for FileManager{
    fn get_window(&mut self, ui: &mut egui::Ui) {
        if ui.button("add path").clicked(){
            let new_path = match FileDialog::new().pick_folder(){
                None => {String::new()}
                Some(a) => {String::from(a.to_str().unwrap())}
            };
            fm_backend::add_path(fm_backend::nm_from_path(&new_path), new_path);
            self.current_location = String::new();
            self.shown_location = String::from("meow");
            self.paths = fm_backend::get_paths();
        }
        if ui.button("..").clicked(){
            if self.current_location != self.root{
                self.current_location.pop();
                self.current_location = fm_backend::path_from_name(&self.current_location);
                println!("{}", self.current_location);
            }
            else{
                self.current_location = String::new();
            }
        }

        ///Showing root folders
        if self.current_location == String::new(){
            if self.current_location != self.shown_location{
                println!("doin stuff");
                self.items= (Vec::new(), Vec::new());

                for (k, v) in &self.paths{
                    self.items.0.push(String::from(k));
                }
                self.shown_location = String::from(&self.current_location);
            }

            let text_style = TextStyle::Body;
            let row_height = ui.text_style_height(&text_style);
            let num_rows = self.items.0.len();
            ScrollArea::vertical().auto_shrink([false; 2]).show_rows(
                ui,
                row_height,
                num_rows,
                |ui, row_range| {
                    for row in row_range {
                        let text = match self.items.0.get(row){
                            None => {String::new()}
                            Some(s) => {String::from(s)}
                        };
                        if text == String::new() {break;}
                        //self.buttons.insert(String::from(&text), String::from(&path));
                        if ui.button(&text).clicked(){
                            self.current_location = String::from(self.paths.get(&text).unwrap());
                            println!("{}", self.current_location);
                            self.root = String::from(self.paths.get(&text).unwrap());
                        };
                    }
                },
            );
        }
        else {
            ///Showing subfolders
            if self.current_location != self.shown_location{
                self.items = fm_backend::ls_all_in_dir(&self.current_location);
                self.shown_location = String::from(&self.current_location);
            }

            let num_of_dirs = self.items.0.len();
            let text_style = TextStyle::Body;
            let row_height = ui.text_style_height(&text_style);
            let num_rows = num_of_dirs + self.items.1.len();

            ScrollArea::vertical().auto_shrink([false; 2]).show_rows(
                ui,
                row_height,
                num_rows,
                |ui, row_range| {
                    for row in row_range {
                        if row < num_of_dirs{
                            let path = match self.items.0.get(row){
                                None => {String::new()}
                                Some(s) => {String::from(s)}
                            };
                            let text = fm_backend::nm_from_path(&path);
                            self.buttons.insert(String::from(&text), String::from(&path));
                            if ui.button(text).clicked(){
                                self.current_location = path;
                                println!("{}", self.current_location);
                            };
                        }
                        else{
                            match self.items.1.get(row-num_of_dirs){
                                None => {}
                                Some(s) => {
                                    let mut text = format!("{0} - {1}", s.artist, s.name);

                                    if text == " - "{
                                        text = fm_backend::nm_from_path(&s.path);
                                    }

                                    if ui.button(text).clicked(){
                                        println!("play");
                                    };
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
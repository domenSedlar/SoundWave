use std::collections::HashMap;

use egui::*;
//use gstreamer::glib::OptionArg::String as OtherString;
use crate::MainWindow;
mod fm_backend;

pub struct FileManager{
    paths: HashMap<String,String>,
    path_names: Vec<String>,
    buttons: HashMap<String,String>,
    current_location: String,
}

impl FileManager{
    pub fn default() -> FileManager{
        let paths = fm_backend::get_paths();
        let mut path_names= Vec::new();

        for (k, v) in &paths{
            path_names.push(String::from(k));
        }

        return FileManager{
            paths,
            path_names,
            buttons: HashMap::new(),
            current_location: String::new()
        }
    }
}

impl super::MainWindow for FileManager{
    fn get_window(&mut self, ui: &mut egui::Ui) {
        if self.current_location == String::new(){
            let text_style = TextStyle::Body;
            let row_height = ui.text_style_height(&text_style);
            let num_rows = self.path_names.len();
            ScrollArea::vertical().auto_shrink([false; 2]).show_rows(
                ui,
                row_height,
                num_rows,
                |ui, row_range| {
                    for row in row_range {
                        let text = match self.path_names.get(row){
                            None => {String::new()}
                            Some(s) => {String::from(s)}
                        };
                        if text == String::new() {break;}
                        if ui.button(&text).clicked(){
                            self.current_location = text;
                            println!("{}", self.current_location);
                        };
                    }
                },
            );
        }

        else {
            let items = fm_backend::ls_all_in_dir(&self.current_location);
            let num_of_dirs = items.0.len();
            let text_style = TextStyle::Body;
            let row_height = ui.text_style_height(&text_style);
            let num_rows = num_of_dirs + items.1.len();

            ScrollArea::vertical().auto_shrink([false; 2]).show_rows(
                ui,
                row_height,
                num_rows,
                |ui, row_range| {
                    for row in row_range {
                        if row < num_of_dirs{
                            let path = match items.0.get(row){
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
                            let path = match items.1.get(row-num_of_dirs){
                                None => {String::new()}
                                Some(s) => {String::from(s)}
                            };
                            let text = fm_backend::nm_from_path(&path);

                            if ui.button(text).clicked(){
                                println!("play");
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
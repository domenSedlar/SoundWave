use std::collections::HashMap;
use egui::*;
use crate::MainWindow;

mod fm_backend;

pub struct FileManager{
    paths: HashMap<String,String>,
    current_location: String
}

impl FileManager{
    pub fn default() -> FileManager{
        return FileManager{
            paths: fm_backend::get_paths(),
            current_location: String::new()
        }
    }
}

impl super::MainWindow for FileManager{
    fn get_window(&mut self, ui: &mut egui::Ui) {
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
                    if ui.button(text).clicked(){
                        self.current_location = String::from("meow");
                        println!("{}", self.current_location);
                    };
                }
            },
        );
    }
}

pub fn lorem_ipsum(ui: &mut egui::Ui) {
    ui.label(
        "Template Text",
    );
}
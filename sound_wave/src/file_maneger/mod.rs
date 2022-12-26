use std::collections::HashMap;
use egui::*;

mod fm_backend;

struct FileManager{
    paths: HashMap<String,String>,
    current_location: String
}
impl FileManager{
    pub fn lorem_ipsum(ui: &mut egui::Ui) {
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
}

pub fn lorem_ipsum(ui: &mut egui::Ui) {
    ui.label(
        "Template Text",
    );
}
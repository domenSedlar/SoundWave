use egui::Color32;
use rfd::FileDialog;

use eframe::{egui};
use egui_extras::RetainedImage;

use std::env;
use std::fs;

pub struct PlaylistAdder {
    name_to_add: String,
    descp_to_add: String,
    ls_of_playls: [Vec<String>; 2],
    image: String,
    i: RetainedImage
}

impl PlaylistAdder{
    pub fn default(ls: [Vec<String>; 2]) -> PlaylistAdder{
        PlaylistAdder{
            name_to_add: "".to_string(),
            descp_to_add: "".to_string(),
            ls_of_playls: ls,
            image: "".to_string(),
            i: egui_extras::RetainedImage::from_svg_bytes_with_size(
                "./var/rustacean-flat-happy.svg",
                &fs::read("./var/rustacean-flat-happy.svg").unwrap(),
                egui_extras::image::FitTo::Size(200,100)).unwrap()

        }
    }

    pub fn get_adding_window(&mut self, ui: &mut egui::Ui) -> Option<[String;3]>{
        ui.horizontal(|ui| {
            ui.vertical(|ui| {

                ui.horizontal(|ui| {
                    ui.label("Name:");
                    ui.text_edit_singleline(&mut self.name_to_add);

                    if self.ls_of_playls[0].contains(&self.name_to_add) && self.name_to_add != String::new(){
                        ui.colored_label(Color32::RED,"name taken");
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Description:");
                    ui.text_edit_multiline(&mut self.descp_to_add);
                });
            });
            ui.vertical(|ui| {
                if ui.button("pick cover").clicked(){
                    self.image = match FileDialog::new().pick_file(){
                        None => {String::new()}
                        Some(a) => {
                            let b = String::from(a.to_str().unwrap());
                            self.i = egui_extras::RetainedImage::from_image_bytes(
                                &b,
                                &fs::read(&b).unwrap())
                                .unwrap();

                            b
                        }
                    };
                }
                self.i.show_size(ui, egui::Vec2::new(200.0, 200.0));

            });

        });

        if ui.button("add").clicked(){
            if !(self.ls_of_playls[0].contains(&self.name_to_add)){
                println!("wow");
                return Some([String::from(&self.name_to_add),String::from(&self.descp_to_add),String::from(&self.image)]);
            }
        }

        return Option::None;
    }
}
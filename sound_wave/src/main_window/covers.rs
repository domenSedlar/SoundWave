use rfd::FileDialog;

use eframe::{egui};
use egui_extras::RetainedImage;

use std::env;
use std::fs;
use std::collections::HashMap;

pub struct AlbumCovers {
    name: String,
    image: String,
    i: RetainedImage,
    covers: HashMap<String, Option<RetainedImage>>
}

impl AlbumCovers {
    pub fn default() -> AlbumCovers {
        let mut ls:HashMap<String, Option<RetainedImage>> = HashMap::new();
        let paths = fs::read_dir("var/AlbumCovers").unwrap();
        for i in paths{
            ls.insert(i.as_ref().unwrap().file_name().into_string().unwrap(), None);
        }
        AlbumCovers {
            name: "".to_string(),
            image: "".to_string(),
            i: egui_extras::RetainedImage::from_svg_bytes_with_size(
                "./var/rustacean-flat-happy.svg",
                &fs::read("./var/rustacean-flat-happy.svg").unwrap(),
                egui_extras::image::FitTo::Size(200,100)).unwrap(),

            covers: ls
        }
    }

    pub fn get_image(&mut self, cv_nm: &String) -> &Option<RetainedImage>{
        if self.covers.contains_key(cv_nm){
            if self.covers.get(cv_nm).unwrap().is_none()
            {
                self.covers.remove(cv_nm);
                self.covers.insert(String::from(cv_nm), Some(
                    egui_extras::RetainedImage::from_image_bytes(
                    cv_nm,
                    &fs::read(format!("./var/AlbumCovers/{}", cv_nm)).unwrap()).unwrap()));
            }

            return self.covers.get(cv_nm).unwrap();
        }

        return &None
    }

    pub fn get_adding_window(&mut self, ui: &mut egui::Ui){
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Name:");
                    ui.text_edit_singleline(&mut self.name);
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
                &self.i.show_size(ui, egui::Vec2::new(200.0, 200.0));

            });

        });
        if ui.button("Add").clicked(){
            self.add_cover();
        }
    }

    pub fn add_cover(&mut self){
        fs::copy(&self.image, format!("./var/AlbumCovers/{0}", &self.name)).expect("TODO: panic message");
        self.image = String::new();
        self.name = String::new();
        self.i = egui_extras::RetainedImage::from_svg_bytes_with_size(
            "./var/rustacean-flat-happy.svg",
            &fs::read("./var/rustacean-flat-happy.svg").unwrap(),
            egui_extras::image::FitTo::Size(200,100)).unwrap()
    }

}
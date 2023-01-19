mod add_playlist;
mod playlists_backend;

use playlists_backend::PlayLs;
use add_playlist::PlaylistAdder;

use egui::Response;
use egui_extras::RetainedImage;

use std::env;
use std::fs;

enum PlaylistsState {
    Details(String),
    Selected(String),
    Default
}

pub struct Playlists {
    adding: bool,
    selected: Option<String>,
    pub(crate) ls_of_playls: [Vec<String>; 2],
    covers: Vec<Option<RetainedImage>>,
    playlist_adder: PlaylistAdder,
    state: PlaylistsState
}

impl Playlists{
    pub fn default() -> Playlists{
        let a = PlayLs{};
        let ls = PlayLs::get_plsls();
        let mut cls = Vec::new();
        for s in &ls[0]{
            cls.push(PlayLs::get_cover(s));
        }
        Playlists{
            adding: false,
            selected: None,
            ls_of_playls: ls,
            covers: cls,
            playlist_adder: PlaylistAdder::default([vec![], vec![]]),
            state: PlaylistsState::Default
        }
    }

    pub fn copy_vec(ls: &Vec<String>) -> Vec<String>{
        let mut nls = Vec::new();
        for s in ls{
            nls.push(s.to_string());
        }
        nls
    }

    pub fn copy_vec_array(ls: &[Vec<String>; 2]) -> [Vec<String>; 2]{
        return [Playlists::copy_vec(&ls[0]), Playlists::copy_vec(&ls[1])]
    }

    pub fn get_main_window(&mut self, ui: &mut egui::Ui){
        match self.adding{
            true => {
                    match self.playlist_adder.get_adding_window(ui){
                        None => {}
                        Some(a) => {
                            PlayLs::add_pls(&a[0], &a[1], &a[2]);
                            self.adding = false;
                            self.playlist_adder = PlaylistAdder::default([vec![], vec![]]);
                            self.ls_of_playls = PlayLs::get_plsls();
                            self.covers.push(PlayLs::get_cover(&a[0]));
                        }
                    }
                if ui.button("cancel").clicked() {
                    self.adding = false;
                }
                }

            false => {
                if ui.button("add playlists").clicked(){
                    self.playlist_adder = PlaylistAdder::default(Playlists::copy_vec_array(&self.ls_of_playls));
                    self.adding = true
                }
            }
            _ => {}
        }

        for i in 0..self.ls_of_playls[0].len(){
            self.get_playlist_panel(ui, &i);
        }
    }

    pub fn get_playlist_panel(&mut self, ui: &mut egui::Ui, i: &usize){
        let r = ui.push_id(i, |ui| {
            let nm = self.ls_of_playls[0].get(*i).unwrap();
            match self.covers.get(*i).unwrap() {
                Some(a) => {
                    a.show_size(ui, egui::Vec2::new(100.0, 100.0));
                }
                None => {}
            }


            ui.label(nm);
        });

        let r = r.response.interact(egui::Sense::click());
        if r.clicked() {
            self.state = PlaylistsState::Selected(String::from(self.ls_of_playls[0].get(*i).unwrap()));
        }

    }
}
mod add_playlist;
pub mod playlists_backend;

use playlists_backend::PlayLs;
use add_playlist::PlaylistAdder;

use egui::Response;
use egui_extras::RetainedImage;

use std::env;
use std::fs;

use super::Song;

enum PlaylistsState {
    Details(usize),
    Selected(String),
    Default
}

pub struct Playlists {
    adding: bool,
    pub(crate) selected: Option<String>,
    pub(crate) ls_of_playls: [Vec<String>; 2],
    covers: Vec<Option<RetainedImage>>,
    playlist_adder: PlaylistAdder,
    state: PlaylistsState,
    pub(crate) playlist: Vec<Song>
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
            state: PlaylistsState::Default,
            playlist: vec![]
        }
    }

    pub fn add(&self,pth: &String, plyls: &String){
        PlayLs::add_song(pth ,plyls);
    }

    pub fn rm_song(&mut self, snm: &String){
        PlayLs::rm_song(snm, &self.selected.as_ref().unwrap(), Song::clone_ls(&self.playlist));
        let mut i = 0;
        while i < self.ls_of_playls.len(){
            if &self.ls_of_playls[0].get(i).unwrap() == &self.selected.as_ref().unwrap(){ break; }
            i += 1;
        }
        self.playlist = PlayLs::get_ls(
            self.ls_of_playls[0].get(i).unwrap());
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
        ///adding
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
        //panel
        //for i in 0..self.ls_of_playls[0].len(){
         //   ui.add_space(20.0);

          //  self.get_playlist_panel(ui, &i);
        //}
    }

    pub fn get_playlist_panel(&mut self, ui: &mut egui::Ui, i: &usize){
        let nm = self.ls_of_playls[0].get(*i).unwrap();

        let r = ui.push_id(i, |ui| {
            match self.covers.get(*i).unwrap() {
                Some(a) => {
                    a.show_size(ui, egui::Vec2::new(100.0, 100.0));
                }
                None => {}
            }
        });
        let r = r.response.interact(egui::Sense::click());

        ui.horizontal(|a|{
            if a.label(nm).clicked() || r.clicked(){
                self.selected = Some(String::from(self.ls_of_playls[0].get(*i).unwrap()));
                self.playlist = PlayLs::get_ls(
                    self.ls_of_playls[0].get(*i).unwrap());
                println!("wow");
            }
            if a.button("...").clicked(){
                self.state = PlaylistsState::Details(*i);
            }
        });
        match self.state{
            PlaylistsState::Details(u) => {
                if &u == i{
                    ui.label(self.ls_of_playls[1].get(*i).unwrap());
                }
            }
            _ => {}
        }

    }
}
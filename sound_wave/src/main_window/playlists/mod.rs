mod add_playlist;
mod playlists_backend;

use playlists_backend::PlayLs;
use add_playlist::PlaylistAdder;

pub struct Playlists {
    adding: bool,
    selected: Option<String>,
    ls_of_playls: [Vec<String>; 2],
    playlist_adder: PlaylistAdder
}

impl Playlists{
    pub fn default() -> Playlists{
        Playlists{
            adding: false,
            selected: None,
            ls_of_playls: PlayLs::get_plsls(),
            playlist_adder: PlaylistAdder::default([vec![], vec![]])
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

    pub fn get_adding_window(&mut self, ui: &mut egui::Ui){
        match self.adding{
            true => {
                    match self.playlist_adder.get_adding_window(ui){
                        None => {}
                        Some(a) => {
                            PlayLs::add_pls(&a[0], &a[1], &a[2]);
                            self.adding = false;
                            self.playlist_adder = PlaylistAdder::default([vec![], vec![]]);
                            self.ls_of_playls = PlayLs::get_plsls();
                        }
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
    }
}
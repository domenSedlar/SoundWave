mod add_playlist;
use add_playlist::PlaylistAdder;

pub struct Playlists {
    adding: bool,
    selected: Option<String>,
    ls_of_playls: Vec<String>,
    playlist_adder: PlaylistAdder
}

impl Playlists{
    pub fn default() -> Playlists{
        Playlists{
            adding: false,
            selected: None,
            ls_of_playls: vec![],
            playlist_adder: PlaylistAdder::default(vec![])
        }
    }

    pub fn copy_vec(ls: &Vec<String>) -> Vec<String>{
        let mut nls = Vec::new();
        for s in ls{
            nls.push(s.to_string());
        }
        nls
    }

    pub fn get_adding_window(&mut self, ui: &mut egui::Ui){
        match self.adding{
            true => {
                    if self.playlist_adder.get_adding_window(ui){
                        self.adding = false;
                        self.playlist_adder = PlaylistAdder::default(vec![]);
                    }
                }

            false => {
                if ui.button("add playlists").clicked(){
                    self.playlist_adder = PlaylistAdder::default(Playlists::copy_vec(&self.ls_of_playls));
                    self.adding = true
                }
            }
            _ => {}
        }
    }
}
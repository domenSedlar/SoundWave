use eframe::glow::FALSE;
use egui::{Color32, Response, menu, Button};
use egui_extras::{Size, StripBuilder, RetainedImage};
use vecshard::{ShardExt, VecShard};

pub struct Song {
    pub(crate) path: String,
    pub(crate) name: String,
    pub(crate) artist: String,
    pub(crate) album: String,
    pub(crate) year: String,
}

impl Song{
    pub fn default() -> Song{
        return Song{
            path: String::new(),
            name: String::new(),
            artist: String::new(),
            album: String::new(),
            year: String::new(),
        }
    }

    pub fn find(path: &str, ls: &Vec<Song>) -> usize{
        for i in 0..ls.len(){
            if &(ls.get(i).unwrap().path) == path{
                return i;
            }
        }

        return 0;
    }

    pub fn same_song(&self, s: &Song) -> bool{
        if self.path == s.path{
            return true;
        }
        else if self.name == s.name && self.artist == s.artist{
            return true;
        }
        return false;
    }

    pub fn clone(s: &Song) -> Song{
        return Song{
            path: String::from(&s.path),
            name: String::from(&s.name),
            artist: String::from(&s.artist),
            album: String::from(&s.album),
            year: String::from(&s.year),
        }
    }

    pub fn clone_ls(ls: &Vec<Song>) -> Vec<Song>{
        let mut nls : Vec<Song>= Vec::new();
        for s in ls{
            nls.push(Song::clone(s))
        }
        nls
    }

    pub fn serialize(self, terminator: char) -> String{
        return format!(
            "{p}{t}{n}{t}{a}{t}{al}{t}{y}{t}",
            p = self.path,
            t = terminator,
            n = self.name,
            a = self.artist,
            al = self.album,
            y = self.year
        )
    }

    pub fn deserialize(s: String, terminator: char) -> Song{
        let mut vars: [String; 5] = [String::new(), String::new(), String::new(), String::new(), String::new()];
        let mut a = 0;
        let mut i = 0;

        let s = s.chars().collect::<Vec<char>>();

        while a < 5 && i < s.len()-1{
            if s.get(i).unwrap() == &';' {
                a+=1;
                i += 1;
                continue
            }
            vars[a].push(*s.get(i).unwrap());
            i += 1;
        }

        return Song{
            path: String::from(&vars[0]),
            name: String::from(&vars[1]),
            artist: String::from(&vars[2]),
            album: String::from(&vars[3]),
            year: String::from(&vars[4]),
        }
    }

    pub fn nm_from_path(path : &String) -> String {
        let mut nm = String::new();

        for i in path.chars(){
            nm.push(i);
            if i == '\\' || i == '/'{
                nm = String::new();
            }
        }

        return nm;
    }

    pub fn get_panel(&self, ui: &mut egui::Ui, row: &usize, playing: bool, img: &Option<RetainedImage>) -> (Response, bool) {

        let dark_mode = ui.visuals().dark_mode;
        let faded_color = ui.visuals().window_fill();
        let faded_color = |color: Color32| -> Color32 {
            use egui::Rgba;
            let t = if dark_mode { 0.95 } else { 0.8 };
            egui::lerp(Rgba::from(color)..=Rgba::from(faded_color), t).into()
        };
        let mut c = false;
        let r = ui.push_id(row, |ui| {
            ui.horizontal(|a| {
                StripBuilder::new(a)
                    .size(Size::exact(60.0))
                    .size(Size::remainder())
                    .size(Size::exact(5.0))
                    .vertical(|mut strip| {
                        strip.cell(|ui| {
                            if playing{
                                ui.painter().rect_filled(
                                    ui.available_rect_before_wrap(),
                                    0.0,
                                    faded_color(Color32::BLUE),
                                );
                            }
                            match img{
                                None => {ui.label("LARGE\nIMAGE\nGOES\nHERE");}
                                Some(a) => {
                                    a.show_size(ui, egui::Vec2::new(80.0, 80.0));
                                }
                            }
                            egui::Grid::new(&self.path).show(ui, |a| {
                                if self.name == String::new(){
                                    a.label(Song::nm_from_path(&self.path).replace(".mp", ""));
                                }
                                else{
                                    a.label(&self.name);
                                }
                                a.end_row();
                                a.label(&self.artist);
                                a.end_row();
                            });
                            ui.label(&self.album);
                            if ui.button("...").clicked(){
                                c = true;
                            }
                        });
                    });

            });
        });

        let r = r.response.interact(egui::Sense::click());
        return (r, c)

    }

    pub fn first_alphabeticly(word1: &str, word2: &str) -> bool{
        let mut i = 0;
        let mut v1 : u32 = 0;
        let mut v2 : u32 = 0;

        while i< word1.len() && i < word2.len(){
            v1 = match word1.chars().nth(i) {
                None => {0}
                Some(c) => {c as u32}
            };
            v2 = match word2.chars().nth(i) {
                None => {0}
                Some(c) => {c as u32}
            };
            if (v1 < v2){
                return true;
            }
            if (v1 > v2){
                return false;
            }
            i += 1;
        }

        return true;
    }

    fn vs_to_vs(v: (VecShard<Song>, VecShard<Song>))->(Vec<Song>, Vec<Song>){
        let mut nv = (vec![], vec![]);
        for i in v.0{
            nv.0.push(i);
        }
        for i in v.1{
            nv.1.push(i);
        }

        return nv
    }


    fn sort_names(mut ls: Vec<Song>)->Vec<Song>{
        if ls.len() < 2{
            return ls;
        }

        let mut current_index = 0;
        let mut swap_marker: i32 = -1;
        let pivot = String::from(&ls.get(ls.len()-1).unwrap().name);
        while current_index < ls.len(){
            if Song::first_alphabeticly(&ls.get(current_index).unwrap().name, &pivot){
                swap_marker += 1;
                if Song::first_alphabeticly(&ls.get(current_index).unwrap().name, &ls.get(swap_marker as usize).unwrap().name){
                    ls.swap(current_index, swap_marker as usize);
                }
            }

            current_index += 1;
        }
        let ln = ls.len();
        let (mut ls1, mut ls2) = Song::vs_to_vs(ls.split_inplace_at(swap_marker as usize));
        if swap_marker > 1{
            ls1 = Song::sort_names(ls1);
        }
        if swap_marker as usize != ln -1{
            ls2 = Song::sort_names(ls2);
        }
        ls1.append(&mut ls2);
        return ls1
    }

    fn sort_artists(mut ls: Vec<Song>)->Vec<Song>{
        if ls.len() < 2{
            return ls;
        }

        let mut current_index = 0;
        let mut swap_marker: i32 = -1;
        let pivot = String::from(&ls.get(ls.len()-1).unwrap().artist);
        while current_index < ls.len(){
            if Song::first_alphabeticly(&ls.get(current_index).unwrap().artist, &pivot){
                swap_marker += 1;
                if Song::first_alphabeticly(&ls.get(current_index).unwrap().artist, &ls.get(swap_marker as usize).unwrap().artist){
                    ls.swap(current_index, swap_marker as usize);
                }
            }

            current_index += 1;
        }
        let ln = ls.len();
        let (mut ls1, mut ls2) = Song::vs_to_vs(ls.split_inplace_at(swap_marker as usize));
        if swap_marker > 1{
            ls1 = Song::sort_artists(ls1);
        }
        if swap_marker as usize != ln -1{
            ls2 = Song::sort_artists(ls2);
        }
        ls1.append(&mut ls2);
        return ls1
    }

    fn sort_albums(mut ls: Vec<Song>)->Vec<Song>{
        if ls.len() < 2{
            return ls;
        }

        let mut current_index = 0;
        let mut swap_marker: i32 = -1;
        let pivot = String::from(&ls.get(ls.len()-1).unwrap().album);
        while current_index < ls.len(){
            if Song::first_alphabeticly(&ls.get(current_index).unwrap().album, &pivot){
                swap_marker += 1;
                if Song::first_alphabeticly(&ls.get(current_index).unwrap().album, &ls.get(swap_marker as usize).unwrap().album){
                    ls.swap(current_index, swap_marker as usize);
                }
            }

            current_index += 1;
        }
        let ln = ls.len();
        let (mut ls1, mut ls2) = Song::vs_to_vs(ls.split_inplace_at(swap_marker as usize));
        if swap_marker > 1{
            ls1 = Song::sort_albums(ls1);
        }
        if swap_marker as usize != ln -1{
            ls2 = Song::sort_albums(ls2);
        }
        ls1.append(&mut ls2);
        return ls1
    }

    fn hide_blank_names(mut ls: Vec<Song>) -> Vec<Song>{
        let mut i = 0;
        let mut temp: Song;
        while ls.len() > i{
            if ls.get(i).unwrap().name == String::new(){
                temp = ls.remove(i);
                ls.push(temp);
            }
            i += 1;
        }

        return ls;
    }

    fn hide_blank_albums(mut ls: Vec<Song>) -> Vec<Song>{
        let mut i = 0;
        let mut temp: Song;
        while ls.len() > i{
            if ls.get(i).unwrap().album == String::new(){
                temp = ls.remove(i);
                ls.push(temp);
            }
            i += 1;
        }

        return ls;
    }

    fn hide_blank_artists(mut ls: Vec<Song>) -> Vec<Song>{
        let mut i = 0;
        let mut temp: Song;
        while ls.len() > i{
            if ls.get(i).unwrap().artist == String::new(){
                temp = ls.remove(i);
                ls.push(temp);
            }
            i += 1;
        }

        return ls;
    }


    pub fn sort_by_names(mut ls: Vec<Song>) -> Vec<Song>{
        ls = Song::sort_names(ls);
        ls = Song::hide_blank_names(ls);
        return ls
    }

    pub fn sort_by_artists(mut ls: Vec<Song>) -> Vec<Song>{
        ls = Song::sort_artists(ls);
        ls = Song::hide_blank_artists(ls);
        return ls
    }

    pub fn sort_by_albums(mut ls: Vec<Song>) -> Vec<Song>{
        ls = Song::sort_albums(ls);
        ls = Song::hide_blank_albums(ls);
        return ls
    }
}

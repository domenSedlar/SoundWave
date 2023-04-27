use std::env;
use std::fs;
use egui::TextBuffer;

use egui_extras::RetainedImage;

use super::super::Song;

pub struct PlayLs{

}

impl PlayLs{
    pub fn get_plsls() -> [Vec<String>; 2]{
        let mut ls :[Vec<String>; 2] = [vec![], vec![]];
        let data = fs::read_to_string("./var/playlists").unwrap();
        let mut a = 0;
        let mut l = 0;

        let mut arr: [String; 2] = [String::new(), String::new()];

        for c in data.chars(){
            if c == (28 as char){ a = 1; continue; }
            if c == '\r'{
                a = 0;
                l += 1;
                ls[0].push(String::from(&arr[0]));
                ls[1].push(String::from(&arr[1]));
                arr = [String::new(), String::new()];
                continue;
            }
            arr[a].push(c);
        }

        return ls
    }

    pub fn get_cover(s: &String) -> Option<egui_extras::RetainedImage>{
        return match fs::read(format!("./var/Covers/{0}", s)) {
            Ok(a) => {
                Option::Some(egui_extras::RetainedImage::from_image_bytes(
                    s,
                    &a)
                    .unwrap())
            }
            Err(_) => {
                return Option::None
            }
        }
    }

    pub fn save_plsls(ls: &[Vec<String>; 2]){
        let mut data = String::new();
        let mut i = 0;
        while i < ls[0].len(){
            data = format!("{data}{0}{2}{1}\r", ls[0].get(i).unwrap(), ls[1].get(i).unwrap(), 28 as char);
            i += 1;
        }
        fs::write("./var/playlists", data);
    }

    pub fn add_pls(name: &String, descp: &String, cover: &String){
        if cover != &String::new(){
            fs::copy(cover, format!("./var/Covers/{name}"));
        }
        let mut ls = PlayLs::get_plsls();
        ls[0].push(String::from(name));
        ls[1].push(String::from(descp));

        fs::File::create(format!("./var/PlaylistDir/{name}"));
        PlayLs::save_plsls(&ls);

    }

    pub fn get_ls(pth: &String) -> Vec<Song>{
        let mut sls: Vec<Song> = vec![];
        let ls: Vec<String> = {
            let mut b = vec![];
            for i in fs::read_to_string(format!("./var/PlaylistDir/{pth}"))
                .unwrap().split('\r').collect::<Vec<&str>>(){
                b.push(i.to_string());
            }
            b
        };

        for s in ls{
            if s == String::new(){
                continue;
            }
           sls.push(Song::deserialize(s.to_string(), ';'));
        }

        sls
    }

    pub fn add_song(pth: &String, plyls: &String){
        let ls = fs::read_to_string(format!("./var/PlaylistDir/{plyls}")).unwrap();
        if ls.contains(&*pth){
            return;
        }
        let ls = format!("{0}{1}\r", ls, &pth);
        fs::write(format!("./var/PlaylistDir/{plyls}"), ls);
    }

    pub fn rm_pls(i: &usize, mut ls: [Vec<String>; 2]) -> [Vec<String>; 2]{
        fs::remove_file(format!("./var/PlaylistDir/{0}", ls[0].get(*i).unwrap()));
        fs::remove_file(format!("./var/Covers/{0}", ls[0].get(*i).unwrap()));

        ls[0].remove(*i);
        ls[1].remove(*i);

        PlayLs::save_plsls(&ls);
        ls
    }

    pub fn rm_song(snm: &String, pnm: &String, sls: Vec<Song>){
        let mut ls = String::new();
        for i in sls{
            let b = Song::serialize(i, ';');
            if &b == snm { continue; }
            ls =  format!("{0}{1}\r", ls, b);
        }

        fs::write(format!("./var/PlaylistDir/{pnm}"), ls);
    }
}
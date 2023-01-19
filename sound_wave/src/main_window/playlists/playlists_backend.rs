use std::env;
use std::fs;


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
        fs::copy(cover, format!("./var/{name}"));
        let mut ls = PlayLs::get_plsls();
        ls[0].push(String::from(name));
        ls[1].push(String::from(descp));

        PlayLs::save_plsls(&ls);
    }
}
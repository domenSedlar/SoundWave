//use std::io;
//use std::fs::File;
use std::collections::HashMap;
use std::fs;
use std::env;
use vecshard::{ShardExt, VecShard};

use super::super::Song;
pub mod tag_reader;

fn abs_to_rel_path(abs: String, root: &String) -> String{
    let mut rel = abs.replace(root, "");
    let mut i = root.len() - 1;
    let mut cdir = String::new();
    let mut b = false;
    while i != 0
    {
        let c = &root.chars().nth(i).unwrap();

        if (c == &'/' || c == &'\\') && b{
            break;
        }
        else if c == &'/' || c == &'\\'{
            b = true;
        }
        cdir = format!("{0}{1}",c, cdir);
        i-=1;
    }
    rel = cdir + &rel;
    return rel

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

pub fn path_from_name(path : &String) -> String {
    let mut result = String::new();
    let mut word = String::new();

    for i in path.chars(){
        if i == '\\' || i == '/'{
            result = format!("{result}{word}");
            word = String::new();
        }
        word.push(i);

    }

    return result;
}

///dir is the functions current location and should when called usually be the same as root.
/// dir is there for the recursive function to track which folder to scan while root tells it where it started
pub fn scan_folder(dir: &String, root: &String) -> (Vec<String>, Vec<String>){
    let paths = fs::read_dir(dir).unwrap();

    let mut dir : Vec<String> = Vec::new();
    let mut files : Vec<String> = Vec::new();

    let mut song;

    for path in paths {
        if path.as_ref().unwrap().path().is_dir(){
            let temp = String::from(format!("{}", path.as_ref().unwrap().path().display()));
            let result = scan_folder(&temp, root);
            for i in result.0{
                dir.push(i);
            }
            for i in result.1{
                files.push(i);
            }
            dir.push(path.unwrap().path().display().to_string());
        }
        else {
            if (path.as_ref().unwrap().path().display().to_string()).ends_with(".mp3")
            {
                song = tag_reader::read_to_str(&path.unwrap().path().display().to_string());

                files.push(song);
            }
        }
    }
    return (dir, files)
}

//TODO Test
pub fn get_paths() -> HashMap<String, String>{
    let data_path = "./var/paths";

    let data = fs::read_to_string(data_path).unwrap();
    let mut paths : HashMap<String, String> = HashMap::new();
    let paths_str = data.split("\n").collect::<Vec<&str>>();

    for i in paths_str{
        if i == "" {continue}
        let pair = i.split("\r").collect::<Vec<&str>>();
        paths.insert(<Vec<&str> as AsRef<Vec<&str>>>::as_ref(&pair).get(0).unwrap().to_string(), pair.get(1).unwrap().to_string());
    }

    paths
}

//TODO Test
pub fn get_folders_or_files(folders: bool) -> Vec<String>{
    let mut data_path = "";

    if folders
    {
        data_path = "./var/folders";
    }
    else{
        data_path = "./var/files";
    }

    let mut ls = Vec::new();
    let data = fs::read_to_string(data_path).unwrap();
    let s = data.split("\n").collect::<Vec<&str>>();

    for i in s{
        if i == "" {continue}
        ls.push(i.to_string())
    }

    ls
}

pub fn get_songs(path: &String) -> Vec<Song>{
    let mut l = Vec::new();
    for s in ls_files_in_dir(path){
        l.push(Song::deserialize(s, ';'))
    }
    l
}

fn first_alphabeticly(word1: &str, word2: &str) -> bool{
    let mut i = 0;

    while i< word1.len() && i < word2.len(){
        if (word1.chars().nth(i).unwrap() as u32) < (word2.chars().nth(i).unwrap() as u32){
            return true;
        }
        if (word1.chars().nth(i).unwrap() as u32) > (word2.chars().nth(i).unwrap() as u32){
            return false;
        }
        i += 1;
    }

    return true;
}

fn vs_to_vs(v: (VecShard<String>, VecShard<String>))->(Vec<String>, Vec<String>){
    let mut nv = (vec![], vec![]);
    for i in v.0{
        nv.0.push(i);
    }
    for i in v.1{
        nv.1.push(i);
    }

    return nv
}


fn sort(mut ls: Vec<String>)->Vec<String>{
    if ls.len() < 2{
        return ls;
    }

    let mut current_index = 0;
    let mut swap_marker: i32 = -1;
    let pivot = String::from(ls.get(ls.len()-1).unwrap());
    while current_index < ls.len(){
        if first_alphabeticly(&ls.get(current_index).unwrap(), &pivot){
            swap_marker += 1;
            if first_alphabeticly(&ls.get(current_index).unwrap(), &ls.get(swap_marker as usize).unwrap()){
                ls.swap(current_index, swap_marker as usize);
            }
        }

        current_index += 1;
    }
    let ln = ls.len();
    let (mut ls1, mut ls2) = vs_to_vs(ls.split_inplace_at(swap_marker as usize));
    if swap_marker > 1{
        ls1 = sort(ls1);
    }
    if swap_marker as usize != ln -1{
        ls2 = sort(ls2);
    }
    ls1.append(&mut ls2);
    return ls1
}

pub fn ls_all_in_dir(dir: &String) -> (Vec<String>, Vec<Song>){
    let (dls, fls) = (get_folders_or_files(true), get_songs(dir));
    let mut items: (Vec<String>, Vec<Song>) = (Vec::new(), Vec::new());
    for s in dls{
        if &path_from_name(&String::from(&s)) == dir{
            items.0.push(s);
        }
    }

    for s in fls{
        if &path_from_name(&String::from(&s.path)) == dir{
            items.1.push(s);
        }
    }

    items.0 = sort(items.0);

    return items
}

pub fn ls_files_in_dir(dir: &String) -> Vec<String>{
    let ls = get_folders_or_files(false);
    let mut files : Vec<String> = Vec::new();
    for s in ls{
        if &path_from_name(&String::from(&s)) == dir{
            files.push(s);
        }
    }
    return files
}

pub fn save_paths(paths: &HashMap<String, String>){
    let data_path = "./var/paths";
    let _str_paths = path_to_str(paths);

    let mut data = format!("{_str_paths}");

    fs::write(data_path, data).expect("Unable to write to file");
}

pub fn save_folders_or_files(f: Vec<String>, folders: bool){
    let mut data_path = "";

    if folders
    {
        data_path = "./var/folders";
    }
    else{
        data_path = "./var/files";
    }

    let mut data = String::new();
    for i in f{
        data = format!("{data}{i}\n");
    }

    fs::write(data_path, data).expect("Unable to write to file");
}

pub fn path_to_str(paths: &HashMap<String,String>) -> String{

    let mut data = String::new();

    for (key, value) in paths
    {
        data = format!("{data}{key}\r{value}\n");
    }

    return data
}

pub fn add_path(key: String, value: String){
    let mut paths = get_paths();

    let mut data = (get_folders_or_files(true), get_folders_or_files(false));
    let items = scan_folder(&value, &value);

    paths.insert(key, value);

    for i in items.0{
        if !data.0.contains(&i){
            data.0.push(i);
        }
    }
    for i in items.1{
        if !data.1.contains(&i){
            data.1.push(i);
        }
    }

    save_folders_or_files(data.0, true);
    save_folders_or_files(data.1, false);

    save_paths(&paths);
}

fn print_scanned_folder() {
    let mut paths = HashMap::new();
    paths.insert(String::from("Music"), String::from("/home/blue/Projects/Programming/Code4/RustyFileManeger/main_window/Music/"));

    for (_, value) in paths
    {
        let list = scan_folder(&String::from(&value), &value);
        println!("{:?}", list.0);
        println!("{:?}", list.1);
    }
}
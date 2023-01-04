use std::collections::HashMap;
use std::path::PathBuf;
use std::thread::current;

use rfd::FileDialog;
use egui::*;
//use gstreamer::glib::OptionArg::String as OtherString;

use super::Song;
pub mod fm_backend;

pub struct FileManager{
    pub(crate) paths: HashMap<String,String>,
    pub(crate) buttons: HashMap<String,String>,
    pub(crate) current_location: String,
    pub(crate) shown_location: String,
    pub(crate) items: (Vec<String>, Vec<Song>),
    pub(crate) root: String,
}

impl FileManager{
    pub fn default() -> FileManager   {
        let paths = fm_backend::get_paths();
        let mut items= (Vec::new(), Vec::new());

        for (k, v) in &paths{
            items.0.push(String::from(k));
        }
        
        FileManager{
        paths,
        buttons: HashMap::new(),
        current_location: String::new(),
        shown_location: String::new(),
        items,
        root: String::new(),
        }
    }


}
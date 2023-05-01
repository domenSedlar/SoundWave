use std::collections::HashMap;
use std::path::PathBuf;
use std::thread::current;
use egui::Color32;

use rfd::FileDialog;
use egui::*;
use egui_extras::RetainedImage;
//use gstreamer::glib::OptionArg::String as OtherString;

mod controller;
mod file_manager;
mod song;
mod playlists;
mod covers;

use song::Song;
use file_manager::FileManager;
use controller::Controller;
use playlists::Playlists;
use crate::main_window::covers::AlbumCovers;

enum SortBy{
    Name,
    Artist,
    Album
}

enum MrCtx{
    No,
    First(String),
    Add(String)
}

#[derive(PartialEq)]
enum CurrentWindow{
    Files,
    Playlists,
    Playlist,
    Queue,
    AddingCovers
}

pub struct Windows {
    file_manager: FileManager,
    controller: Controller,
    playlists: Playlists,
    cover_adder: AlbumCovers,
    current_window: CurrentWindow,
    more: MrCtx,
    sort_options: bool,
    sorting_by: SortBy,
    pub(crate) main_text: String
}

impl Windows {
    pub fn default() -> Windows {
        return Windows {
            file_manager: FileManager::default(),
            controller: Controller::default(),
            playlists: Playlists::default(),
            cover_adder: AlbumCovers::default(),
            current_window: CurrentWindow::Files,
            more: MrCtx::No,
            sort_options: false,
            sorting_by: SortBy::Name,
            main_text: "Files".to_string()
        }
    }
    pub fn get_tabs_window(&mut self, ui: &mut egui::Ui){
        if ui.button("Browse files").clicked(){
            self.current_window = CurrentWindow::Files;
            self.main_text = String::from("Files")
        }
        if ui.button("Playlists").clicked(){
            self.current_window = CurrentWindow::Playlists;
            self.playlists.selected = None;
            self.main_text = String::from("Playlists")
        }
        if ui.button("Album Covers").clicked(){
            self.current_window = CurrentWindow::AddingCovers;
            self.main_text = String::from("Add Album Covers")
        }
    }

    pub fn get_top_window(&mut self, ui: &mut egui::Ui){
        if self.sort_options{
            ui.horizontal(|u |{
                u.label("Sort by: ");
                if u.button("Title").clicked(){
                    self.sorting_by = SortBy::Name;
                    self.sort_options = false;
                    self.change_sorting();
                }
                if u.button("Artist").clicked(){
                    self.sorting_by = SortBy::Artist;
                    self.sort_options = false;
                    self.change_sorting();
                }
                if u.button("Album").clicked(){
                    self.sorting_by = SortBy::Album;
                    self.sort_options = false;
                    self.change_sorting();
                }
            });
            return
        }
        ui.horizontal(|u|{
            if self.current_window == CurrentWindow::Files ||
                self.current_window == CurrentWindow::Playlists{
                if u.button("+").clicked() {
                    let ls: &Vec<Song>;

                    if self.current_window == CurrentWindow::Files {
                        ls = &self.file_manager.items.1;

                        for s in ls {
                            if {
                                let mut b = true;
                                for i in &self.controller.list {
                                    if s.name == i.name && s.artist == i.artist && s.album == i.album {
                                        b = false;
                                        break;
                                    }
                                }
                                b
                            }
                            {
                                self.controller.list.push(Song::clone(s));
                            }
                        }
                    } else if self.current_window == CurrentWindow::Playlists {
                        ls = &self.playlists.playlist;
                        for s in ls {
                            if {
                                let mut b = true;
                                for i in &self.controller.list {
                                    if s.name == i.name && s.artist == i.artist && s.album == i.album {
                                        b = false;
                                        break;
                                    }
                                }
                                b
                            }
                            {
                                self.controller.list.push(Song::clone(s));
                            }
                        }
                    }
                }

                if u.button("++").clicked() {
                    let ls: &Vec<Song>;

                    if self.current_window == CurrentWindow::Files {
                        ls = &self.file_manager.items.1;

                        for s in ls {
                            self.controller.list.push(Song::clone(s));
                        }
                    } else if self.current_window == CurrentWindow::Playlists {
                        ls = &self.playlists.playlist;
                        for s in ls {
                            self.controller.list.push(Song::clone(s));
                        }
                    }
                }

                if u.button("-").clicked() {
                    let ls: &Vec<Song>;

                    if self.current_window == CurrentWindow::Files {
                        ls = &self.file_manager.items.1;

                        for s in ls {
                            let mut j = 0;
                            for i in Song::clone_ls(&self.controller.list) {
                                if s.name == i.name && s.artist == i.artist && s.album == i.album {
                                    self.controller.list.remove(j);
                                }
                                j += 1;
                            }
                        }
                    } else if self.current_window == CurrentWindow::Playlists {
                        ls = &self.playlists.playlist;
                        for s in ls {
                            let mut j = 0;
                            for i in Song::clone_ls(&self.controller.list) {
                                if s.name == i.name && s.artist == i.artist && s.album == i.album {
                                    self.controller.list.remove(j);
                                }
                                j += 1;
                            }
                        }
                    }
                }
                if u.button("Sort by").clicked() {
                    self.sort_options = true;
                }
                if u.button("|>").clicked(){
                    match &self.current_window {
                        CurrentWindow::Files => {self.controller.play(Song::clone_ls(&self.file_manager.items.1), 0)}
                        CurrentWindow::Playlists => {self.controller.play(Song::clone_ls(&self.playlists.playlist), 0)}
                        CurrentWindow::Playlist => {self.controller.play(Song::clone_ls(&self.playlists.playlist), 0)}
                        _ => {}
                    }
                }
            }

            if self.current_window == CurrentWindow::Files ||
                self.current_window == CurrentWindow::Playlists ||
                self.current_window == CurrentWindow::Queue{
            if u.button("Shuffle Play").clicked() {
                match &self.current_window {
                    CurrentWindow::Files => { self.controller.play_true_shuffle(&self.file_manager.items.1) }
                    CurrentWindow::Playlists => { self.controller.play_true_shuffle(&self.playlists.playlist) }
                    CurrentWindow::Playlist => { self.controller.play_true_shuffle(&self.playlists.playlist) }
                    CurrentWindow::Queue => {
                        let ls = Song::clone_ls(&self.controller.list);
                        self.controller.play_true_shuffle(&ls)
                    }
                    _ => {}
                }
            }
            }
        });
    }

    pub fn get_controller(&mut self, ui: &mut egui::Ui) {
        egui::SidePanel::left("Btn").max_width(5.0).show_inside(ui, |ui| {
            if ui.button("=").clicked(){
                self.current_window = CurrentWindow::Queue;
            }
        });
        self.controller.get_window(ui);
    }

    pub fn get_main_window(&mut self, ui: &mut egui::Ui){
        match &self.current_window{
            CurrentWindow::Files => {self.get_file_manager_window(ui)},
            CurrentWindow::Queue => {self.get_queue_window(ui)},
            CurrentWindow::Playlists => {
                match &self.playlists.selected{
                    None => {self.get_playlists_window(ui)}
                    Some(a) => {self.get_playlist_window(ui)}
                }

            },
            CurrentWindow::Playlist => {self.get_playlist_window(ui)},
            CurrentWindow::AddingCovers => {self.cover_adder.get_adding_window(ui)}
            _ => {}
        }
    }

    fn get_playlist_ctx(&mut self, ui: &mut egui::Ui){
        menu::bar(ui, |a| {
            if a.button("-").clicked(){
                &self.playlists.rm_song(
                    &file_manager::fm_backend::tag_reader::read_to_str(&match &self.more{
                    MrCtx::No => {String::new()}
                    MrCtx::First(a) => {a.to_string()}
                    MrCtx::Add(b) => {b.to_string()}
                }));
                self.more = MrCtx::No;
            }
            if a.button("+ Playlist").clicked() {
                self.more = MrCtx::Add(match &self.more{
                    MrCtx::No => {String::new()}
                    MrCtx::First(a) => {a.to_string()}
                    MrCtx::Add(b) => {b.to_string()}
                })
            }
            if a.button("Play next").clicked(){
                self.controller.play_next(
                    file_manager::fm_backend::tag_reader::read_to_song(String::from(&match &self.more{
                        MrCtx::No => {String::new()}
                        MrCtx::First(a) => {a.to_string()}
                        MrCtx::Add(b) => {b.to_string()}
                    })));
                self.more = MrCtx::No;
            }
            if a.button("X").clicked(){
                self.more = MrCtx::No;
            }
        });
    }

    fn get_playlist_window(&mut self, ui: &mut egui::Ui){
        let ls = Song::clone_ls(&self.playlists.playlist);

        let text_style = TextStyle::Body;
        let row_height = ui.text_style_height(&text_style);
        let num_rows = ls.len();

        ScrollArea::vertical().auto_shrink([false; 2]).show_rows(
            ui,
            row_height,
            num_rows,
            |mut ui, row_range| {
                for row in row_range {
                    match ls.get(row){
                        None => {}
                        Some(s) => {
                            let mut text = format!("{0} - {1}", s.artist, s.name);

                            if text == " - "{
                                text = file_manager::fm_backend::nm_from_path(&s.path);
                            }
                            let (r, m) = &s.get_panel(ui, &row,{
                                match self.controller.list.get(self.controller.index) {
                                    None => {false}
                                    Some(b) => {s.same_song(b) }
                                }
                            },
                            self.cover_adder.get_image(&s.album)
                            );
                            if *m{
                                self.more = MrCtx::First(String::from(&s.path))
                            }
                            match &self.more{
                                MrCtx::No => {}
                                MrCtx::First(p) => {
                                    if p == &s.path{
                                        self.get_playlist_ctx(ui);
                                    }
                                }
                                MrCtx::Add(p) => {
                                    if p == &s.path{
                                        self.get_adding_menu(ui);
                                    }
                                }
                            }
                            if r.clicked(){
                                self.controller.play(Song::clone_ls(&self.playlists.playlist), row);
                            }
                        }
                    };

                }
            },
        );
    }

    fn get_playlists_window(&mut self, ui: &mut egui::Ui){
        self.playlists.get_main_window(ui);

        for i in 0..self.playlists.ls_of_playls[0].len(){
            ui.add_space(20.0);

            self.playlists.get_playlist_panel(ui, &i);
        }
    }

    fn get_adding_menu(&mut self, ui: &mut egui::Ui){
        menu::bar(ui, |a| {
            for i in &self.playlists.ls_of_playls[0]{
                if a.button(i).clicked(){
                    &self.playlists.add(&file_manager::fm_backend::tag_reader::read_to_str(&match &self.more{
                        MrCtx::No => {String::new()}
                        MrCtx::First(a) => {a.to_string()}
                        MrCtx::Add(b) => {b.to_string()}
                    }) , i );
                    self.more = MrCtx::No;
                }
            }
            if a.button("X").clicked(){
                self.more = MrCtx::No;
            }
        });
    }

    fn get_ctx_menu(&mut self, ui: &mut egui::Ui){
        menu::bar(ui, |a| {
            if a.button("+ Playlist").clicked() {
                self.more = MrCtx::Add(match &self.more{
                    MrCtx::No => {String::new()}
                    MrCtx::First(a) => {a.to_string()}
                    MrCtx::Add(b) => {b.to_string()}
                })
            }
            if a.button("Play next").clicked(){
                self.controller.play_next(
                    file_manager::fm_backend::tag_reader::read_to_song(String::from(&match &self.more{
                        MrCtx::No => {String::new()}
                        MrCtx::First(a) => {a.to_string()}
                        MrCtx::Add(b) => {b.to_string()}
                    })));
                self.more = MrCtx::No;
            }
            if a.button("X").clicked(){
                self.more = MrCtx::No;
            }
        });
    }

    fn get_queue_window(&mut self, ui: &mut egui::Ui){
        let text_style = TextStyle::Body;
        let row_height = ui.text_style_height(&text_style);
        let num_rows = self.controller.list.len();

        ScrollArea::vertical().auto_shrink([false; 2]).show_rows(
            ui,
            row_height,
            num_rows,
            |mut ui, row_range| {
                for row in row_range {
                    match &self.controller.list.get(row){
                        None => {}
                        Some(s) => {
                            let mut text = format!("{0} - {1}", s.artist, s.name);

                            if text == " - "{
                                text = file_manager::fm_backend::nm_from_path(&s.path);
                            }
                            let (r, m) = &s.get_panel(
                                                        ui, &row,
                                                      row== self.controller.index,
                                                      self.cover_adder.get_image(&s.album));
                            if *m{
                                self.more = MrCtx::First(String::from(&s.path))
                            }
                            match &self.more{
                                MrCtx::No => {}
                                MrCtx::First(p) => {
                                    if p == &s.path{
                                        self.get_ctx_menu(ui);
                                    }
                                }
                                MrCtx::Add(p) => {
                                    if p == &s.path{
                                        self.get_adding_menu(ui);
                                    }
                                }
                            }
                            if r.clicked(){
                                self.controller.to(row);
                            }
                        }
                    };

                }
            },
        );
    }

    pub fn get_file_manager_window(&mut self, ui: &mut egui::Ui) {
        if ui.button("add path").clicked(){
            let new_path = match FileDialog::new().pick_folder(){
                None => {String::new()}
                Some(a) => {String::from(a.to_str().unwrap())}
            };
            file_manager::fm_backend::add_path(file_manager::fm_backend::nm_from_path(&new_path), new_path);
            self.file_manager.current_location = String::new();
            self.file_manager.shown_location = String::from("meow");
            self.file_manager.paths = file_manager::fm_backend::get_paths();
        }
        if ui.button("..").clicked(){
            if self.file_manager.current_location != self.file_manager.root{
                self.file_manager.current_location.pop();
                self.file_manager.current_location = file_manager::fm_backend::path_from_name(&self.file_manager.current_location);
                println!("{}", self.file_manager.current_location);
            }
            else{
                self.file_manager.current_location = String::new();
            }
        }

        ///Showing root folders
        if self.file_manager.current_location == String::new(){
            if self.file_manager.current_location != self.file_manager.shown_location{
                println!("doin stuff");
                self.file_manager.items= (Vec::new(), Vec::new());

                for (k, v) in &self.file_manager.paths{
                    self.file_manager.items.0.push(String::from(k));
                }
                self.file_manager.shown_location = String::from(&self.file_manager.current_location);
            }

            let text_style = TextStyle::Body;
            let row_height = ui.text_style_height(&text_style);
            let num_rows = self.file_manager.items.0.len();
            ScrollArea::vertical().auto_shrink([false; 2]).show_rows(
                ui,
                row_height,
                num_rows,
                |ui, row_range| {
                    for row in row_range {
                        let text = match self.file_manager.items.0.get(row){
                            None => {String::new()}
                            Some(s) => {String::from(s)}
                        };
                        if text == String::new() {break;}
                        //self.buttons.insert(String::from(&text), String::from(&path));
                        if ui.button(&text).clicked(){
                            self.file_manager.current_location = String::from(self.file_manager.paths.get(&text).unwrap());
                            println!("{}", self.file_manager.current_location);
                            self.file_manager.root = String::from(self.file_manager.paths.get(&text).unwrap());
                        };
                    }
                },
            );
        }
        else {
            ///Showing subfolders
            if self.file_manager.current_location != self.file_manager.shown_location{
                self.file_manager.items = file_manager::fm_backend::ls_all_in_dir(&self.file_manager.current_location);
                self.file_manager.items.1 = Song::sort_by_names(Song::clone_ls(&self.file_manager.items.1));
                self.file_manager.shown_location = String::from(&self.file_manager.current_location);
            }

            //let curr_song = &self.controller.get_current_song();
            let num_of_dirs = self.file_manager.items.0.len();
            let text_style = TextStyle::Body;
            let row_height = ui.text_style_height(&text_style);
            let num_rows = num_of_dirs + self.file_manager.items.1.len();

            ScrollArea::vertical().auto_shrink([false; 2]).show_rows(
                ui,
                row_height,
                num_rows,
                |mut ui, row_range| {
                    for row in row_range {
                        ///folders
                        if row < num_of_dirs{
                            let path = match self.file_manager.items.0.get(row){
                                None => {String::new()}
                                Some(s) => {String::from(s)}
                            };
                            let text = file_manager::fm_backend::nm_from_path(&path);
                            self.file_manager.buttons.insert(String::from(&text), String::from(&path));
                            if ui.button(text).clicked(){
                                self.file_manager.current_location = path;
                                println!("{}", self.file_manager.current_location);
                            };
                        }
                        else{
                            ///songs
                            match self.file_manager.items.1.get(row-num_of_dirs){
                                None => {}
                                Some(s) => {
                                    let mut text = format!("{0} - {1}", s.artist, s.name);

                                    if text == " - "{
                                        text = file_manager::fm_backend::nm_from_path(&s.path);
                                    }
                                    let (r, m) = &s.get_panel(ui, &row,{
                                        match self.controller.list.get(self.controller.index) {
                                            None => {false}
                                            Some(b) => {s.same_song(b) }
                                        }
                                    }, self.cover_adder.get_image(&s.album));
                                    if *m{
                                        self.more = MrCtx::First(String::from(&s.path))
                                    }
                                    if r.clicked(){
                                        let mut pls = Song::clone_ls(&self.file_manager.items.1);
                                        let i: usize = Song::find(&s.path, &mut pls);
                                        self.controller.play(pls, i);
                                    }
                                    match &self.more{
                                        MrCtx::No => {}
                                        MrCtx::First(p) => {
                                            if p == &s.path{
                                                &self.get_ctx_menu(ui);
                                            }
                                        }
                                        MrCtx::Add(p) => {
                                            if p == &s.path {
                                                &self.get_adding_menu(ui);
                                            }
                                        }
                                    }

                                }
                            };

                        }
                    }
                },
            );
        }
    }

    fn change_sorting(&mut self){
        match &self.sorting_by {
            SortBy::Name => {
                self.file_manager.items.1 = Song::sort_by_names(Song::clone_ls(&self.file_manager.items.1));
                self.playlists.playlist = Song::sort_by_names(Song::clone_ls(&self.playlists.playlist));
            }
            SortBy::Artist => {
                self.file_manager.items.1 = Song::sort_by_artists(Song::clone_ls(&self.file_manager.items.1));
                self.playlists.playlist = Song::sort_by_artists(Song::clone_ls(&self.playlists.playlist));
            }
            SortBy::Album => {
                self.file_manager.items.1 = Song::sort_by_albums(Song::clone_ls(&self.file_manager.items.1));
                self.playlists.playlist = Song::sort_by_albums(Song::clone_ls(&self.playlists.playlist));
            }
        }

    }

    pub fn get_current_song_data(&mut self) -> (String, &Option<RetainedImage>){
        let s = self.controller.get_current_song();
        let mut nm = String::from(&s.path);
        if s.name != String::new(){
            nm = format!("{0} - {1}", s.name, s.artist);
        }
        let i = self.cover_adder.get_image(&s.album);
        return (nm, i)
    }
}

fn scroll_area_template(ui: &mut egui::Ui) {
    ui.label(
        "",
    );
    ui.add_space(4.0);

    let text_style = TextStyle::Body;
    let row_height = ui.text_style_height(&text_style);
    let num_rows = 10_000;
    ScrollArea::vertical().auto_shrink([false; 2]).show_rows(
        ui,
        row_height,
        num_rows,
        |ui, row_range| {
            for row in row_range {
                let text = format!("This is row {}/{}", row + 1, num_rows);
                ui.label(text);
            }
        },
    );
}

pub fn lorem_ipsum(ui: &mut egui::Ui) {
    ui.label(
        "Template Text",
    );
}
use eframe::glow::FALSE;
use egui::{Color32, Response, menu, Button};
use egui_extras::{Size, StripBuilder};

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

    pub fn get_panel(&self, ui: &mut egui::Ui, row: &usize, playing: bool) -> (Response, bool) {

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
                            ui.label("LARGE\nIMAGE\nGOES\nHERE");
                            egui::Grid::new(&self.path).show(ui, |a| {
                                a.label(&self.name);
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
}

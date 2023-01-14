use egui::Response;

pub struct Song {
    pub(crate) path: String,
    pub(crate) name: String,
    pub(crate) artist: String,
    pub(crate) album: String,
    pub(crate) year: String
}

impl Song{
    pub fn default() -> Song{
        return Song{
            path: String::new(),
            name: String::new(),
            artist: String::new(),
            album: String::new(),
            year: String::new()
        }
    }

    pub fn find(path: &str, ls: &mut Vec<Song>) -> usize{
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
            year: String::from(&s.year)
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
            year: String::from(&vars[4])
        }
    }

    pub fn get_panel(&self, ui: &mut egui::Ui) {

        ui.label("LARGE\nIMAGE\nGOES\nHERE");
        egui::Grid::new(&self.path).show(ui, |ui| {
            ui.label(&self.name);
            ui.end_row();
            ui.label(&self.artist);
            ui.end_row();
        });
        ui.label(&self.album);

    }

    pub fn get_panel2(&self, ui: &mut egui::Ui, row: &usize) -> Response {

            let r = ui.push_id(row, |ui| {
                ui.horizontal(|a| {
            a.label("LARGE\nIMAGE\nGOES\nHERE");
            egui::Grid::new(&self.path).show(a, |a| {
                a.label(&self.name);
                a.end_row();
                a.label(&self.artist);
                a.end_row();
            });
            a.label(&self.album);
                });        });

        let r = r.response.interact(egui::Sense::click());
        return r

    }
}

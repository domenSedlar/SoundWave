use std::fs;
use super::super::Song;

pub fn read_to_song(file: String) -> Song{
    let mut data = fs::read(&file).expect("Unable to read file");

    data.drain(0..(data.len() - 128));

    let mut s = Song::default();
    s.path = file;

    if !(data[0] as char == 'T' && data[1] as char == 'A' && data[2] as char == 'G'){
        return s
    }

    //Title
    let mut t = String::new();
    for i in &data[3..32]{
        t.push(*i as char);
    }
    s.name = t.trim_matches(char::from(0)).parse().unwrap();

    //Artist
    let mut t = String::new();
    for i in &data[33..62]{
        t.push(*i as char);
    }
    s.artist = t.trim_matches(char::from(0)).parse().unwrap();

    //Album
    let mut t = String::new();
    for i in &data[63..92]{
        t.push(*i as char);
    }
    s.album = t.trim_matches(char::from(0)).parse().unwrap();

    //Year
    t = String::new();
    for i in &data[93..96]{
        t.push(*i as char);
    }
    s.year = t.trim_matches(char::from(0)).parse().unwrap();

    return s
}

pub fn read_to_str(file: String) -> String{
    let mut data = fs::read(&file).expect("Unable to read file");

    data.drain(0..(data.len() - 128));

    let mut s = String::from(&file);

    if !(data[0] as char == 'T' && data[1] as char == 'A' && data[2] as char == 'G'){
        return s
    }
    s.push(';');

    //Title
    let mut t = String::new();
    for i in &data[3..32]{
        s.push(*i as char);
    }
    s.push(';');


    //Artist
    let mut t = String::new();
    for i in &data[33..62]{
        s.push(*i as char);
    }
    s.push(';');

    //Album
    let mut t = String::new();
    for i in &data[63..92]{
        s.push(*i as char);
    }
    s.push(';');

    //Year
    t = String::new();
    for i in &data[93..96]{
        s.push(*i as char);
    }
    s.push(';');

    s = s.replace('\0', "");

    return s
}
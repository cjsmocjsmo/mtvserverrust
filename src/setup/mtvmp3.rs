use id3::{Tag, TagLike};
use mp3_duration;
use std::path::Path;

pub fn get_tag_info(x: String) -> (String, String, String, String) {
    let tag = Tag::read_from_path(x).unwrap();
    let artist = tag.artist().unwrap().to_string();
    let album = tag.album().unwrap().to_string();
    let song = tag.title().unwrap().to_string();
    let genre = tag.genre().unwrap().to_string();

    (artist, album, song, genre)
}

pub fn get_duration(x: String) -> String {
    let path = Path::new(&x);
    let dur_sec = mp3_duration::from_path(&path).expect("this is duration exception");
    
    
    // let dur_sec = mp3_duration::from_path(&path).unwrap();
    let dur_min = dur_sec.div_f32(60.0);
    let dur_str = format!("{:?}", dur_min);
    let mut durvec = vec![];
    for i in dur_str.chars() {
        durvec.push(i);
    }

    let mut newvec = vec![];
    let mut count = 0;
    for c in durvec {
        count = count + 1;
        if count < 5 {
            newvec.push(c);
        } else {
            count = 0;
            break;
        };
    }

    let duration: String = newvec.into_iter().collect();

    duration
}

pub fn split_sep1(x: String) -> Vec<String> {
    let xsplit = x.split("_");
    let mut xvec = vec![];
    for word in xsplit {
        xvec.push(word.to_string());
    }

    xvec
}

pub fn split_sep2(x: String) -> Vec<String> {
    let yslit = x.split(" ");
    let mut yvec = vec![];
    for word in yslit {
        yvec.push(word.to_string());
    }

    yvec
}

pub fn split_sep3(x: String) -> Vec<String> {
    let filesplit = x.split("_-_");
    let mut fvec = vec![];
    for file in filesplit {
        fvec.push(file.to_string());
    }

    fvec
}

pub fn check_artist(x: String, y: String) -> bool {
    let f = split_sep1(x);
    let t = split_sep2(y);
    if f != t {
        return false;
    } else {
        return true;
    }
}

pub fn check_album(x: String, y: String) -> bool {
    let f = split_sep1(x);
    let t = split_sep2(y);
    if f != t {
        return false;
    } else {
        return true;
    }
}

pub fn check_song(f: String, t: String) -> bool{
    let mut xx = split_sep3(f);
    let count = xx.len() - 1;
    xx.drain(0..count);
    let yy = split_sep2(t);
    let mut pussy = false;
    for x in xx.clone() {
        let fuck = split_sep1(x);
        if yy == fuck {
            pussy = true;
        } else {
            pussy = false;
        }
    } 

    pussy
}


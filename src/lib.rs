use byte_unit::Byte;
use md5::{Digest, Md5};
use std::env;
use std::path::Path;
use walkdir::WalkDir;

fn set_env_var(p1: String, p2: String) -> Result<(), Box<dyn std::error::Error>> {
    env::set_var(p1.clone(), p2);
    let value = env::var(p1.clone());
    if value.is_err() {
        println!("Error: key not found not found");
    } else {
        println!("key is set to: {}", value.unwrap());
    }

    Ok(())
}

pub fn set_all_env_vars() {
    let media1 = String::from("MTV_MEDIA_PATH");
    let media2 = String::from("/media/charliepi/FOO/media");
    let _media_env_set = set_env_var(media1, media2).is_ok();

    let music1 = String::from("MTV_MUSIC_PATH");
    let music2 = String::from("/media/charliepi/FOO/media/music");
    let _music_env_set = set_env_var(music1, music2).is_ok();

    let music_thumb1 = String::from("MTV_MUSIC_THUMBNAIL_PATH");
    let music_thumb2 = String::from("/media/charliepi/FOO/media/music_thumbnails");
    let _music_env_set = set_env_var(music_thumb1, music_thumb2).is_ok();

    let movies1 = String::from("MTV_MOVIES_PATH");
    let movies2 = String::from("/media/charliepi/FOO/media/movies");
    let _music_env_set = set_env_var(movies1, movies2).is_ok();

    let movies_thumb1 = String::from("MTV_MOVIES_THUMBNAIL_PATH");
    let movies_thumb2 = String::from("/media/charliepi/FOO/media/movies_thumbnails");
    let _movies_env_set = set_env_var(movies_thumb1, movies_thumb2).is_ok();

    let movies_metadata1 = String::from("MTV_MOVIES_METADATA_PATH");
    let movies_metadata2 = String::from("/media/charliepi/FOO/media/metadata_movies");
    let _movies_metadata_env_set = set_env_var(movies_metadata1, movies_metadata2).is_ok();

    let music_metadata1 = String::from("MTV_MUSIC_METADATA_PATH");
    let music_metadata2 = String::from("/media/charliepi/FOO/media/metadata_music");
    let _music_metadata_env_set = set_env_var(music_metadata1, music_metadata2).is_ok();
}

fn walk_music_dir_mp3() -> Vec<String> {
    let mut mp3vec = Vec::new();
    let mtv_music_path = env::var("MTV_MUSIC_PATH").expect("$MTV_MUSIC_PATH is not set");
    for e in WalkDir::new(mtv_music_path.clone())
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            let fname = e.path().to_string_lossy().to_string();

            if fname.ends_with(".mp3") {
                mp3vec.push(fname);
            } else {
                continue;
            }
        }
    }

    mp3vec
}

fn walk_music_dir_images() -> Vec<String> {
    let mut musicimagevec = Vec::new();
    let mtv_music_path = env::var("MTV_MUSIC_PATH").expect("$MTV_MUSIC_PATH is not set");
    for e in WalkDir::new(mtv_music_path.clone())
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            let fname = e.path().to_string_lossy().to_string();

            if fname.ends_with(".jpg") {
                musicimagevec.push(fname);
            } else if fname.ends_with(".jpeg") {
                musicimagevec.push(fname);
            } else if fname.ends_with(".png") {
                musicimagevec.push(fname);
            } else if fname.ends_with(".webp") {
                musicimagevec.push(fname);
            } else if fname.ends_with(".avif") {
                musicimagevec.push(fname);
            } else {
                continue;
            }
        }
    }

    musicimagevec
}

pub fn walk_movies_dir() -> Vec<String> {
    let mut moviesvec = Vec::new();
    let mtv_movies_path = env::var("MTV_MOVIES_PATH").expect("$MTV_MOVIES_PATH is not set");
    for e in WalkDir::new(mtv_movies_path.clone())
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            let fname = e.path().to_string_lossy().to_string();

            if fname.ends_with(".jpg") {
                moviesvec.push(fname);
            } else {
                continue;
            }
        }
    }

    moviesvec
}

pub fn walk_movies_thumb_dir() -> Vec<String> {
    let mut moviesthumbvec = Vec::new();
    let mtv_movies_thumb_path =
        env::var("MTV_MOVIES_THUMBNAIL_PATH").expect("$MTV_MOVIES_THUMBNAIL_PATH is not set");
    for e in WalkDir::new(mtv_movies_thumb_path.clone())
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            let fname = e.path().to_string_lossy().to_string();

            if fname.ends_with(".jpg") {
                moviesthumbvec.push(fname);
            } else if fname.ends_with(".jpeg") {
                moviesthumbvec.push(fname);
            } else if fname.ends_with(".png") {
                moviesthumbvec.push(fname);
            } else if fname.ends_with(".webp") {
                moviesthumbvec.push(fname);
            } else if fname.ends_with(".avif") {
                moviesthumbvec.push(fname);
            } else {
                continue;
            }
        }
    }

    moviesthumbvec
}

pub fn walk_metadata_music() -> Vec<String> {
    let mut metadatamusicvec = Vec::new();
    let mtv_metadata_music_path =
        env::var("MTV_MUSIC_METADATA_PATH").expect("$MTV_MUSIC_METADATA_PATH is not set");
    for e in WalkDir::new(mtv_metadata_music_path.clone())
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            let fname = e.path().to_string_lossy().to_string();

            if fname.ends_with(".json") {
                metadatamusicvec.push(fname);
            } else {
                continue;
            }
        }
    }

    metadatamusicvec
}

pub fn walk_metadata_movies() -> Vec<String> {
    let mut metadatamoviesvec = Vec::new();
    let mtv_metadata_movies_path =
        env::var("MTV_MOVIES_METADATA_PATH").expect("$MTV_MUSIC_METADATA_PATH is not set");
    for e in WalkDir::new(mtv_metadata_movies_path.clone())
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            let fname = e.path().to_string_lossy().to_string();

            if fname.ends_with(".json") {
                metadatamoviesvec.push(fname);
            } else {
                continue;
            }
        }
    }

    metadatamoviesvec
}

pub fn process_music_images() {
    let mtv_music_path = env::var("MTV_MUSIC_PATH").expect("$MTV_MUSIC_PATH is not set");

    let mp3_imagesvec = walk_music_dir_images();

    let mut jpgcount = 0;
    for jpg in mp3_imagesvec {
        let jpg_id = get_md5(jpg.clone());
        let base_dir = split_base_dir(jpg.clone(), mtv_music_path.clone());
        let ext = split_ext(jpg.clone());
        jpgcount = jpgcount + 1;
        println!("this is full path:\n {}", jpg);
        println!("this is jpg id: \n {}", jpg_id);
        println!("this is jpg basedir:\n {}", base_dir);
        println!("this is ext {}", ext);
        // println!("this is f: {:?}", i);
    }
    println!("There are {} jpgs", jpgcount.to_string());
}

pub fn process_mp3s() {
    let mtv_music_path = env::var("MTV_MUSIC_PATH").expect("$MTV_MUSIC_PATH is not set");

    let mp3svec = walk_music_dir_mp3();

    let mut mp3count = 0;
    for mp3 in mp3svec {
        let mp3_id = get_md5(mp3.clone());
        let mp3_base_dir = split_base_dir(mp3.clone(), mtv_music_path.clone());
        let ext = split_ext(mp3.clone());
        mp3count = mp3count + 1;
        println!("this is full path:\n {}", mp3);
        println!("this is mp3 id: \n {}", mp3_id);
        println!("This is mp3 basedir:\n {}", mp3_base_dir);
        println!("This is ext: {}", ext);
    }
    println!("There are {} mp3s", mp3count.to_string());
}

fn split_ext(astring: String) -> String {
    let path = Path::new(&astring);
    let boo_results = path.extension();
    let boo = match boo_results {
        Some(b) => b.to_string_lossy().to_string(),
        None => String::from("split_ext did not work"),
    };

    boo
}

fn split_base_dir(x: String, v: String) -> String {
    let path = Path::new(&x);
    let boo_results = path.strip_prefix(v);
    let boo = match boo_results {
        Ok(b) => b.to_string_lossy().to_string(),
        Err(error) => panic!("it didnt work: {:?}", error),
    };

    boo
    // let items: Vec<&str> = x.split("/media/charliepi/FOO/music/").collect();
    // items[1].to_string()
    // set MTV_MEDIA_PATH=/media/charliepi/FOO/music/music/
}

fn get_md5(astring: String) -> String {
    let mut hasher2 = Md5::new();
    hasher2.update(astring.clone());
    let a_id = hasher2.finalize();
    let foo = format!("{:x}", a_id);

    foo
}

pub fn get_size(addr: String) -> String {
    let total_size = WalkDir::new(addr)
        .min_depth(1)
        .max_depth(5)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.metadata().ok())
        .filter(|metadata| metadata.is_file())
        .fold(0, |acc, m| acc + m.len());

    let btos = total_size.to_string();
    let result = Byte::from_str(btos).unwrap();
    let size = result.get_appropriate_unit(true).to_string();

    size
}

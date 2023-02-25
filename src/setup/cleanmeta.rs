extern crate glob;
use glob::glob;
use std::env;
use std::fs;

pub fn clean_movie_meta_dir() {
    let movie_meta_dir_path = env::var("MTV_MOVIES_METADATA_PATH").unwrap();
    let glob_str = movie_meta_dir_path + "/*.json";
    for e in glob(glob_str.as_str()).expect("Failed to read glob pattern") {

        let rm_path = e.unwrap();

        println!("{:?}", rm_path);
        fs::remove_file(rm_path).expect("File delete failed");
        println!("File deleted successfully!");
    }
}

pub fn clean_music_meta_dir() {
    let music_meta_dir_path = env::var("MTV_MUSIC_METADATA_PATH").unwrap();
    let glob_str = music_meta_dir_path + "/*.json";
    for e in glob(glob_str.as_str()).expect("Failed to read glob pattern") {
        let rm_path = e.unwrap();

        println!("{:?}", rm_path);
        fs::remove_file(rm_path).expect("File delete failed");
        println!("File deleted successfully!");
    }
}

pub fn clean_meta() {
    clean_movie_meta_dir();
    clean_music_meta_dir();
}

use std::env;
use walkdir::WalkDir;

pub fn walk_music_dir_mp3() -> Vec<String> {
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

pub fn walk_music_dir_images() -> Vec<String> {
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

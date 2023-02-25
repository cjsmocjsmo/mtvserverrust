use std::env;
pub fn set_env_var(p1: String, p2: String) -> Result<(), Box<dyn std::error::Error>> {
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
    let music2 = String::from("/media/charliepi/FOO/media/music/");
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

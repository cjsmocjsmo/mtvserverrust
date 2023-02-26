use std::env;

fn main() {
    // mtvserver::set_all_env_vars();

    mtvserver::clean_meta();

    mtvserver::process_music_images();

    mtvserver::process_mp3s();

    let _movievec = mtvserver::walk_movies_dir();
    let _moviethumbvec = mtvserver::walk_movies_thumb_dir();

    let _music_metadata = mtvserver::walk_metadata_music();
    let _movies_metadata = mtvserver::walk_metadata_movies();

    let mtv_media_path = env::var("MTV_MEDIA_PATH").expect("$MTV_MEDIA_PATH is not set");

    println!(
        "Total size: {} .",
        mtvserver::media_total_size(mtv_media_path)
    );
}

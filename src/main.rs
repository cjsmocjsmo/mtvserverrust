use std::env;

mod setup;

fn main() {
    setup::envvars::set_all_env_vars();

    setup::cleanmeta::clean_meta();

    // mtvserver::process_music_images();

    mtvserver::process_mp3s();

    let _movievec = setup::mtvwalkdirs::walk_movies_dir();
    let _moviethumbvec = setup::mtvwalkdirs::walk_movies_thumb_dir();

    let _music_metadata = setup::mtvwalkdirs::walk_metadata_music();
    let _movies_metadata = setup::mtvwalkdirs::walk_metadata_movies();

    let mtv_media_path = env::var("MTV_MEDIA_PATH").expect("$MTV_MEDIA_PATH is not set");

    println!(
        "Total size: {} .",
        setup::misc::media_total_size(mtv_media_path)
    );
}

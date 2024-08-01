use std::env;

// Production settings
pub fn set_env_vars() {
    let mov_path = env::var("MTV_MOVIES_PATH");
    if mov_path.is_err() {
        env::set_var("MTV_MOVIES_PATH", "/home/pimedia/PINAS/bazmnt/MTV/Movies/");
    }
    println!("MTV_MOVIES_PATH: {:?}", env::var("MTV_MOVIES_PATH"));

    let tv_path = env::var("MTV_TV_PATH");
    if tv_path.is_err() {
        env::set_var("MTV_TV_PATH", "/home/pimedia/PINAS/bazmnt/MTV/TVShows/");
    }

    let thumb_path = env::var("MTV_THUMBNAIL_PATH");
    if thumb_path.is_err() {
        env::set_var("MTV_THUMBNAIL_PATH", "/usr/share/MTV/thumbnails/");
    }

    let static_path = env::var("MTV_STATIC_PATH");
    if static_path.is_err() {
        env::set_var("MTV_STATIC_PATH", "/usr/share/MTV/static");
    }

    let raw_addr = env::var("MTV_RAW_ADDR");
    if raw_addr.is_err() {
        env::set_var("MTV_RAW_ADDR", "192.168.0.99");
    }

    let server_addr = env::var("MTV_SERVER_ADDR");
    if server_addr.is_err() {
        env::set_var("MTV_SERVER_ADDR", "http://192.168.0.99");
    }

    let server_port = env::var("MTV_SERVER_PORT");
    if server_port.is_err() {
        env::set_var("MTV_SERVER_PORT", "8080");
    }

    // let docker_var = env::var("MTV_DOCKER_VAR");
    // if docker_var.is_err() {
    //     env::set_var("MTV_DOCKER_VAR", "NONE");
    // }

    let offset = env::var("MTV_OFFSET");
    if offset.is_err() {
        env::set_var("MTV_OFFSET", "25");
    }

    let db_path = env::var("MTV_DB_PATH");
    if db_path.is_err() {
        env::set_var("MTV_DB_PATH", "/usr/share/MTV/mtv.db");
    }

    let posters_path = env::var("MTV_POSTER_PATH");
    if posters_path.is_err() {
        env::set_var("MTV_POSTER_PATH", "/home/pimedia/PINAS/bazmnt/MTV/Posters/");
    }

    let setup_path = env::var("MTV_SETUP_PATH");
    if setup_path.is_err() {
        env::set_var("MTV_SETUP_PATH", "/usr/share/MTV/");
    }

    let mtv_path = env::var("MTV_MTV_PATH");
    if mtv_path.is_err() {
        env::set_var("MTV_MTV_PATH", "/usr/share/MTV/");
    }

}


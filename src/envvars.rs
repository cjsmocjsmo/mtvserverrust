use std::env;

// Production settings
pub fn set_env_vars() {

    // let usb1 = env::var("MTV_USB1");
    // if usb1.is_err() {
    //     env::set_var("MTV_USB1", "/media/pi/PiTB/media");
    // }

    // let usb2 = env::var("MTV_USB2");
    // if usb2.is_err() {
    //     env::set_var("MTV_USB2", "None");
    // }

    // let usb3 = env::var("MTV_USB3");
    // if usb3.is_err() {
    //     env::set_var("MTV_USB3", "None");
    // }

    // let usb4 = env::var("MTV_USB4");
    // if usb4.is_err() {
    //     env::set_var("MTV_USB4", "None");
    // }
    let mov_path = env::var("MTV_MOVIES_PATH");
    if mov_path.is_err() {
        env::set_var("MTV_MOVES_PATH", "/home/pimedia/PINAS/bazmnt/MTV/Movies/");
    }
    println!("MTV_MOVIES_PATH: {:?}", env::var("MTV_MOVIES_PATH"));

    let tv_path = env::var("MTV_TV_PATH");
    if tv_path.is_err() {
        env::set_var("MTV_TV_PATH", "/home/pimedia/PINAS/bazmnt/MTV/TVShows/");
    }

    let thumb_path = env::var("MTV_MOVIES_THUMBNAIL_PATH");
    if thumb_path.is_err() {
        env::set_var("MTV_MOVIES_THUMBNAIL_PATH", "/usr/share/mtvsetup/mtvsetup/thumbnails/");
    }

    let static_path = env::var("MTV_STATIC_PATH");
    if static_path.is_err() {
        env::set_var("MTV_STATIC_PATH", "/usr/share/mtvsetup/mtvsetup/static");
    }

    let raw_addr = env::var("MTV_RAW_ADDR");
    if raw_addr.is_err() {
        env::set_var("MTV_RAW_ADDR", "192.168.0.97");
    }

    let server_addr = env::var("MTV_SERVER_ADDR");
    if server_addr.is_err() {
        env::set_var("MTV_SERVER_ADDR", "http://192.168.0.97");
    }

    let server_port = env::var("MTV_SERVER_PORT");
    if server_port.is_err() {
        env::set_var("MTV_SERVER_PORT", "8080");
    }

    let docker_var = env::var("MTV_DOCKER_VAR");
    if docker_var.is_err() {
        env::set_var("MTV_DOCKER_VAR", "NONE");
    }

    let offset = env::var("MTV_OFFSET");
    if offset.is_err() {
        env::set_var("MTV_OFFSET", "25");
    }

    let db_path = env::var("MTV_DB_PATH");
    if db_path.is_err() {
        env::set_var("MTV_DB_PATH", "/usr/share/mtvsetup/mtvsetup/db/mtv.db");
    }
    // let db_check_file_path = env::var("MTV_DB_CHECK_FILE_PATH");
    // if db_check_file_path.is_err() {
    //     env::set_var("MTV_DB_CHECK_FILE_PATH", "/usr/share/mtvsetup/mtvsetup/db/db_check_file.txt");
    // }

    let db_path = env::var("MTV_FILE_PATH");
    if db_path.is_err() {
        env::set_var("MTV_FILE_PATH", "/usr/share/mtvsetup/mtvsetup/setup.mtv");
    }

}


// Settings for test server
// pub fn set_env_vars() {

//     let usb1 = env::var("MTV_USB1");
//     if usb1.is_err() {
//         env::set_var("MTV_USB1", "/media/pi/USBMOVIES/media");
//     }
//     let usb2 = env::var("MTV_USB2");
//     if usb2.is_err() {
//         env::set_var("MTV_USB2", "/media/pi/USB2/media");
//     }
//     let usb3 = env::var("MTV_USB3");
//     if usb3.is_err() {
//         env::set_var("MTV_USB3", "/media/pi/USB3/media");
//     }
//     let usb4 = env::var("MTV_USB4");
//     if usb4.is_err() {
//         env::set_var("MTV_USB4", "None");
//     }
//     let thumb_path = env::var("MTV_MOVIES_THUMBNAIL_PATH");
//     if thumb_path.is_err() {
//         env::set_var("MTV_MOVIES_THUMBNAIL_PATH", "/usr/share/mtvsetup/mtvsetup/thumbnails/");
//     }
//     let static_path = env::var("MTV_STATIC_PATH");
//     if static_path.is_err() {
//         env::set_var("MTV_STATIC_PATH", "/usr/share/mtvsetup/mtvsetup/static");
//     }
//     let raw_addr = env::var("MTV_RAW_ADDR");
//     if raw_addr.is_err() {
//         env::set_var("MTV_RAW_ADDR", "192.168.0.26");
//     }
//     let server_addr = env::var("MTV_SERVER_ADDR");
//     if server_addr.is_err() {
//         env::set_var("MTV_SERVER_ADDR", "http://192.168.0.26");
//     }
//     let server_port = env::var("MTV_SERVER_PORT");
//     if server_port.is_err() {
//         env::set_var("MTV_SERVER_PORT", "8080");
//     }

//     let docker_var = env::var("MTV_DOCKER_VAR");
//     if docker_var.is_err() {
//         env::set_var("MTV_DOCKER_VAR", "NONE");
//     }
//     let offset = env::var("MTV_OFFSET");
//     if offset.is_err() {
//         env::set_var("MTV_OFFSET", "25");
//     }
//     let db_path = env::var("MTV_DB_PATH");
//     if db_path.is_err() {
//         env::set_var("MTV_DB_PATH", "/usr/share/mtvsetup/mtvsetup/mtv.db");
//     }
// let db_path = env::var("MTV_FILE_PATH");
//     if db_path.is_err() {
//         env::set_var("MTV_FILE_PATH", "/usr/share/mtvsetup/mtvsetup/setup.mtv");
//     }


// }
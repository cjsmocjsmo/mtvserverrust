use std::env;
use json::object;

fn main() {

    let dockervar = get_docker_var();
    if dockervar == "docker var not set".to_string() {
        mtvserver::set_all_env_vars();
        println!("should be /media/charliepi/FOO/media :\n {}", env::var("MTV_MEDIA_PATH").unwrap());
    } else {
        println!("{}", "docker var is set so docker will set the env vars for us");
    };

    mtvserver::clean_meta();

    process_music_images();

    process_mp3s();

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

fn get_docker_var() -> String {
    let docker_var_results = env::var("MTV_DOCKER_VAR");
    let docker_var = match docker_var_results{
        Ok(docker_var) => docker_var,
        Err(_error) => "docker var not set".to_string(),
    };

    docker_var
}

fn process_music_images() {
    // let mtv_music_path = env::var("MTV_MUSIC_PATH").expect("$MTV_MUSIC_PATH is not set");

    let mp3_imagesvec = mtvserver::walk_music_dir_images();

    let mut image_count = 0;

    for jpg in mp3_imagesvec {
        image_count = image_count + 1;

        let dims = mtvserver::get_image_dims(&jpg);
        let newdims = mtvserver::normalize_music_image(dims);
        let base_dir = mtvserver::split_base_dir(&jpg);
        let file_name = mtvserver::split_filename(&jpg);
        let extension = mtvserver::split_ext(&jpg);

        let artist_results = mtvserver::image_split_artist(&base_dir);
        println!("this is artist: {}", artist_results);

        let album_results = mtvserver::image_split_album(&base_dir);

        let imginfo = object! {
            imageid: mtvserver::get_md5(&jpg),
            filename_artist: artist_results,
            filename_album: album_results,
            basedir: &*base_dir,
            filename: &*file_name,
            ext: &*extension,
            width: newdims.0,
            height: newdims.1,
            idx: image_count,
            fsize: mtvserver::get_file_size(&jpg),
            fullpath: &*jpg,
            b64img: mtvserver::to_base64_str(&jpg, newdims.0, newdims.1),
        };

        let ifo = json::stringify(imginfo.dump());
        // "/media/charliepi/FOO/media/metadata_music"
        let mtv_music_metadata_path =
            env::var("MTV_MUSIC_METADATA_PATH").expect("$MTV_MUSIC_METADATA_PATH is not set");

        let a = format!("{}/", mtv_music_metadata_path.as_str());
        let b = format!("Music_Image_Meta_{}.json", image_count);
        let outpath = a + &b;

        // println!("\n\n\n ifo {:#?}", ifo);
        std::fs::write(outpath, ifo).unwrap();

        // put it in a db
    }
    println!("There are {} jpgs", image_count);
}

fn process_mp3s() {
    let mp3svec = mtvserver::walk_music_dir_mp3();
    let mut named_incorrectly_vec = vec![];

    let mut index = 0;
    for mp3 in mp3svec {
        index = index + 1;

        let voodoo: &String = &"None".to_string();
        let tags = mtvserver::get_tag_info(&mp3);
        let base_dir = mtvserver::split_base_dir(&mp3);
        let filename_results = mtvserver::split_filename(&mp3);
        let music_artist_results = mtvserver::music_split_artist(&base_dir);
        let music_album_results = mtvserver::music_split_album(&base_dir);
        let duration_results = mtvserver::get_duration(&mp3);
        let artc = mtvserver::check_artist(&music_artist_results, &tags.0);
        let albc = mtvserver::check_album(&music_album_results, &tags.1);
        let sc = mtvserver::check_song(&filename_results);

        if artc == true && albc == true && sc == true {
            println!("\n they all match:\n {}", &mp3);

            let mp3_info = object! {
                mp3id: mtvserver::get_md5(&mp3),
                fullpath: &*mp3,
                basedir: &*base_dir,
                filename: filename_results,
                ext: mtvserver::split_ext(&mp3),
                imgurl: &**voodoo,
                mp3_url: &**voodoo,
                tag_artist: &*tags.0,
                tag_album: &*tags.1,
                tag_title: &*tags.2,
                tag_genre: &*tags.3,
                idx: index,
                fsize: mtvserver::get_file_size(&mp3),
                filename_artist: &*music_artist_results,
                filename_album: &*music_album_results,
                duration: duration_results,
            };

            let mfo: String = json::stringify(mp3_info.dump());

            let mtv_music_metadata_path =
                env::var("MTV_MUSIC_METADATA_PATH").expect("$MTV_MUSIC_METADATA_PATH is not set");

            let a = format!("{}/", mtv_music_metadata_path.as_str());
            let b = format!("Music_File_Meta_{}.json", index);
            let outpath = a + &b;
            std::fs::write(outpath, mfo.clone()).unwrap();

            println!("\n\n\n mp3info {}", mfo.clone());
        } else {
            // println!("{:?}", mp3.clone());
            named_incorrectly_vec.push(mp3.clone());
        }
    }
    println!(
        "there are {} mp3s named incorrectly",
        named_incorrectly_vec.len()
    );

    for name in named_incorrectly_vec {
        println!("nameed incorrectly with tags {}", name);
    }
    println!("There are {} mp3s", index.to_string());
}
use std::env;

fn main() {
    let setup_status = mtv_setup();
    println!("{}", setup_status);

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

fn mtv_setup() -> String {
    let dockervar = mtvserver::get_docker_var();
    if dockervar == "docker var not set".to_string() {
        mtvserver::set_all_env_vars();
        println!(
            "should be /media/charliepi/FOO/media :\n {}",
            env::var("MTV_MEDIA_PATH").unwrap()
        );
    } else {
        println!("docker var is set so docker will set the env vars for us");
    };

    // mtvserver::clean_meta();

    process_music_images();

    process_mp3s();

    "Setup Complete".to_string()
}

fn process_music_images() {
    let mp3_imagesvec = mtvserver::walk_music_dir_images();

    let mut image_count = 0;

    for jpg in mp3_imagesvec {
        image_count = image_count + 1;

        let id = mtvserver::get_md5(&jpg);

        let dims = mtvserver::get_image_dims(&jpg);
        let newdims = mtvserver::normalize_music_image(dims);
        let width_r = newdims.0.to_string();
        let height_r = newdims.1.to_string();

        let base_dir = mtvserver::split_base_dir(&jpg);
        let file_name = mtvserver::split_filename(&jpg);
        let extension = mtvserver::split_ext(&jpg);

        let artist_results = mtvserver::image_split_artist(&base_dir);
        println!("this is artist: {}", artist_results);
        let album_results = mtvserver::image_split_album(&base_dir);

        let fsize_results = mtvserver::get_file_size(&jpg).to_string();
        let fullpath = &jpg.to_string();
        let b64image = mtvserver::to_base64_str(&jpg, newdims.0, newdims.1);

        mtvserver::write_image_json_to_file(
            id,
            width_r,
            height_r,
            base_dir,
            file_name,
            extension,
            artist_results,
            album_results,
            fsize_results,
            b64image,
            fullpath.to_string(),
            image_count.to_string(),
        );

        // put it in a db
    }
    println!("There are {} jpgs", image_count);
}

fn process_mp3s() {
    let mp3svec = mtvserver::walk_music_dir_mp3();

    let mut index = 0;
    for mp3 in mp3svec {
        index = index + 1;

        let id = mtvserver::get_md5(&mp3);
        let voodoo: &String = &"None".to_string();
        let tags = mtvserver::get_tag_info(&mp3);
        let artist = tags.0;
        let album = tags.1;
        let song = tags.2;
        let genre = tags.3;
        let base_dir = mtvserver::split_base_dir(&mp3);
        let filename_results = mtvserver::split_filename(&mp3);
        let music_artist_results = mtvserver::music_split_artist(&base_dir);
        let music_album_results = mtvserver::music_split_album(&base_dir);
        let duration_results = mtvserver::get_duration(&mp3);
        let artc = mtvserver::check_artist(&music_artist_results, &artist);
        let albc = mtvserver::check_album(&music_album_results, &album);
        let sc = mtvserver::check_song(&filename_results, &song);
        let fullpath = &mp3.to_string();
        let extension = mtvserver::split_ext(&mp3);
        let idx = index.to_string();
        let fsize_results = mtvserver::get_file_size(&mp3).to_string();

        mtvserver::write_music_json_to_file(
            id,
            voodoo.to_string(),
            artist,
            album,
            song,
            genre,
            base_dir,
            filename_results,
            music_artist_results,
            music_album_results,
            duration_results,
            artc,
            albc,
            sc,
            fullpath.to_string(),
            extension,
            idx,
            fsize_results,

        );

        // if artc == true && albc == true && sc == true {
        //     println!("\n they all match:\n {}", &mp3);

        //     let mp3_info = object! {
        //         mp3id: mtvserver::get_md5(&mp3),
        //         fullpath: &*mp3,
        //         basedir: &*base_dir,
        //         filename: filename_results,
        //         ext: mtvserver::split_ext(&mp3),
        //         imgurl: &**voodoo,
        //         mp3_url: &**voodoo,
        //         tag_artist: &*tags.0,
        //         tag_album: &*tags.1,
        //         tag_title: &*tags.2,
        //         tag_genre: &*tags.3,
        //         idx: index,
        //         fsize: mtvserver::get_file_size(&mp3),
        //         filename_artist: &*music_artist_results,
        //         filename_album: &*music_album_results,
        //         duration: duration_results,
        //     };

        //     let mfo: String = json::stringify(mp3_info.dump());

        //     let mtv_music_metadata_path =
        //         env::var("MTV_MUSIC_METADATA_PATH").expect("$MTV_MUSIC_METADATA_PATH is not set");

        //     let a = format!("{}/", mtv_music_metadata_path.as_str());
        //     let b = format!("Music_File_Meta_{}.json", index);
        //     let outpath = a + &b;
        //     std::fs::write(outpath, mfo.clone()).unwrap();

        //     println!("\n\n\n mp3info {}", mfo.clone());
        // } else {
        //     // println!("{:?}", mp3.clone());
        //     named_incorrectly_vec.push(mp3.clone());
        
    }
    // println!(
    //     "there are {} mp3s named incorrectly",
    //     named_incorrectly_vec.len()
    // );

    // for name in named_incorrectly_vec {
    //     println!("nameed incorrectly with tags {}", name);
    // }
    // println!("There are {} mp3s", index.to_string());
}

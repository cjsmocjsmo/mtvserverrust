use base64::{alphabet, engine, Engine as _};
use byte_unit::Byte;
use filesize::PathExt;
use glob::glob;
use id3::{Tag, TagLike};
use image;
use md5::{Digest, Md5};
use mp3_duration;
use std::env;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

fn clean_movie_meta_dir() {
    let movie_meta_dir_path = env::var("MTV_MOVIES_METADATA_PATH").unwrap();
    let glob_str = movie_meta_dir_path + "/*.json";
    for e in glob(glob_str.as_str()).expect("Failed to read glob pattern") {
        let rm_path = e.unwrap();

        println!("{:?}", rm_path);
        fs::remove_file(rm_path).expect("File delete failed");
        println!("File deleted successfully!");
    }
}

fn clean_music_meta_dir() {
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

// perform a check here to see if we are using docker no need to set env vars if so

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
    let music2 = String::from("/media/charliepi/FOO/media/music/C/");
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

pub fn get_md5(astring: &String) -> String {
    let mut hasher2 = Md5::new();
    hasher2.update(&astring);
    let a_id = hasher2.finalize();
    let foo = format!("{:x}", a_id);

    foo
}

pub fn get_file_size(x: &String) -> u64 {
    let path = Path::new(&x);
    let realsize = path.size_on_disk().unwrap();

    realsize
}

pub fn media_total_size(addr: String) -> String {
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

pub fn split_ext(astring: &String) -> String {
    let path = Path::new(&astring);
    let boo_results = path.extension();
    let boo = match boo_results {
        Some(b) => b.to_string_lossy().to_string(),
        None => String::from("split_ext did not work"),
    };

    let ext = String::from(".") + boo.as_str();

    ext
}

pub fn split_base_dir(astring: &String) -> String {
    let mysplit = astring.split("/");
    let mut myvec = vec![];

    for my in mysplit {
        myvec.push(my);
    }

    let path = env::var("MTV_MUSIC_PATH").unwrap();
    let envsplit = path.split("/");

    let mut envvec = vec![];

    for env in envsplit {
        envvec.push(env);
    }

    let count = envvec.len() - 1;
    myvec.drain(0..count);
    myvec.pop();

    let base_dir = myvec.join("/");

    base_dir
}

pub fn image_split_artist(x: &String) -> String {
    let filesplit = x.split("/");
    let mut filenamevec = vec![];
    for file in filesplit {
        filenamevec.push(file);
    }

    let mut fin = vec![];
    for f in filenamevec {
        fin.push(f);
    }

    String::from(fin[1])
}

pub fn music_split_artist(x: &String) -> String {
    let filesplit = x.split("/");
    let mut filenamevec = vec![];
    for file in filesplit {
        filenamevec.push(file);
    }

    let mut fin = vec![];
    for f in filenamevec {
        fin.push(f);
    }

    String::from(fin[1])
}

pub fn image_split_album(x: &String) -> String {
    let filesplit = x.split("/");
    let mut filenamevec = vec![];
    for file in filesplit {
        filenamevec.push(file);
    }

    let album_result = filenamevec.last();
    let album = match album_result {
        Some(album) => album.to_string(),
        None => "None".to_string(),
    };

    album.to_string()
}

pub fn music_split_album(x: &String) -> String {
    let filesplit = x.split("/");
    let mut filenamevec = vec![];
    for file in filesplit {
        filenamevec.push(file);
    }

    let count = &filenamevec.len() - 2;
    filenamevec.drain(0..count);
    let mut album = "";
    for f in filenamevec {
        album = f;
    }

    String::from(album)
}

pub fn split_filename(x: &String) -> String {
    let filesplit = x.split("/");
    let mut filenamevec = vec![];
    for file in filesplit {
        filenamevec.push(file);
    }

    let count = &filenamevec.len() - 1;
    filenamevec.drain(0..count);
    let mut finalvec = "";
    for f in filenamevec {
        finalvec = f;
    }

    let fname = finalvec.split(".");
    let mut svec = vec![];
    // let mut foo = "";
    for f in fname {
        svec.push(f);
    }
    svec.pop();

    let filename = svec.get(0).unwrap();

    filename.to_string()
}

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

pub fn get_tag_info(x: &String) -> (String, String, String, String) {
    let tag = Tag::read_from_path(x).unwrap();
    let artist = tag.artist().unwrap().to_string();
    let album = tag.album().unwrap().to_string();
    let song = tag.title().unwrap().to_string();
    let genre = tag.genre().unwrap().to_string();

    (artist, album, song, genre)
}

pub fn get_duration(x: &String) -> String {
    let path = Path::new(&x);
    let dur_sec = mp3_duration::from_path(&path).expect("this is duration exception");

    // let dur_sec = mp3_duration::from_path(&path).unwrap();
    let dur_min = dur_sec.div_f32(60.0);
    let dur_str = format!("{:?}", dur_min);
    let mut durvec = vec![];
    for i in dur_str.chars() {
        durvec.push(i);
    }

    let mut newvec = vec![];
    let mut count: i32 = 0;
    for c in durvec {
        count = count + 1;
        if count < 5 {
            newvec.push(c);
        } else {
            break;
        };
    }

    let duration: String = newvec.into_iter().collect();

    duration
}

pub fn split_sep1(x: String) -> Vec<String> {
    let xsplit = x.split("_");
    let mut xvec = vec![];
    for word in xsplit {
        xvec.push(word.to_string());
    }

    xvec
}

pub fn split_sep2(x: String) -> Vec<String> {
    let yslit = x.split(" ");
    let mut yvec = vec![];
    for word in yslit {
        yvec.push(word.to_string());
    }

    yvec
}

pub fn split_sep3(x: String) -> Vec<String> {
    let filesplit = x.split("_-_");
    let mut fvec = vec![];
    for file in filesplit {
        fvec.push(file.to_string());
    }

    fvec
}

pub fn check_artist(x: &String, y: &String) -> bool {
    let f = split_sep1((&x).to_string());
    let t = split_sep2((&y).to_string());
    if f != t {
        return false;
    } else {
        return true;
    }
}

pub fn check_album(x: &String, y: &String) -> bool {
    let f = split_sep1((&x).to_string());
    let t = split_sep2((&y).to_string());
    if f != t {
        return false;
    } else {
        return true;
    }
}

pub fn check_song(f: &String) -> bool {
    let mut xx = split_sep3((&f).to_string());
    let count = xx.len() - 1;
    xx.drain(0..count);
    let yy = split_sep2((&f).to_string());
    let mut pussy = false;
    for x in xx.clone() {
        let fuck = split_sep1(x);
        if yy == fuck {
            pussy = true;
        } else {
            pussy = false;
        }
    }

    pussy
}

pub fn get_image_dims(x: &String) -> (u32, u32) {
    let dims = image::image_dimensions(&x).expect("get image dims has failed");

    dims
}

pub fn normalize_music_image(dims: (u32, u32)) -> (u32, u32) {
    let largest: u32;

    if dims.0 == dims.1 {
        largest = dims.0;
    } else if dims.0 > dims.1 {
        largest = dims.0;
    } else {
        largest = dims.1;
    }

    let resizetup: (u32, u32);
    if largest < 100 {
        resizetup = (100, 100);
    } else if largest < 200 {
        resizetup = (200, 200);
    } else if largest < 300 {
        resizetup = (300, 300);
    } else {
        resizetup = (300, 300);
    }

    resizetup
}

pub fn to_base64_str(x: &String, w: u32, h: u32) -> String {
    let img_result = image::open(&x);
    let img = match img_result {
        Ok(img) => img,
        Err(error) => panic!("problem opening file {:?}", error),
    };
    let thumb = img.thumbnail(w, h);
    let thumb_bytes = thumb.into_bytes();

    let alphabet =
        alphabet::Alphabet::new("+/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789")
            .unwrap();

    // a very weird config that encodes with padding but requires no padding when decoding...?
    let crazy_config = engine::GeneralPurposeConfig::new()
        .with_decode_allow_trailing_bits(true)
        .with_encode_padding(true)
        .with_decode_padding_mode(engine::DecodePaddingMode::RequireNone);

    let crazy_engine = engine::GeneralPurpose::new(&alphabet, crazy_config);

    let encoded = crazy_engine.encode(thumb_bytes);

    encoded
}

// use image::codecs::png::PngEncoder;
// use image::io::Reader as ImageReader;
// use image::{ColorType, ImageEncoder};
// use std::io::BufWriter;
// use std::num::NonZeroU32;

// use fast_image_resize as fr;

// fn resize_image(x: String, dimstup: (u32, u32)) {
//     // Read source image from file
//     let img = ImageReader::open(x).unwrap().decode().unwrap();
//     let width = NonZeroU32::new(img.width()).unwrap();
//     let height = NonZeroU32::new(img.height()).unwrap();
//     let mut src_image = fr::Image::from_vec_u8(
//         width,
//         height,
//         img.to_rgba8().into_raw(),
//         fr::PixelType::U8x4,
//     )
//     .unwrap();

//     // Multiple RGB channels of source image by alpha channel
//     // (not required for the Nearest algorithm)
//     let alpha_mul_div = fr::MulDiv::default();
//     alpha_mul_div
//         .multiply_alpha_inplace(&mut src_image.view_mut())
//         .unwrap();

//     // Create container for data of destination image
//     let dst_width = NonZeroU32::new(dimstup.0).unwrap();
//     let dst_height = NonZeroU32::new(dimstup.1).unwrap();
//     let mut dst_image = fr::Image::new(dst_width, dst_height, src_image.pixel_type());

//     // Get mutable view of destination image data
//     let mut dst_view = dst_image.view_mut();

//     // Create Resizer instance and resize source image
//     // into buffer of destination image
//     let mut resizer = fr::Resizer::new(fr::ResizeAlg::Convolution(fr::FilterType::Lanczos3));
//     resizer.resize(&src_image.view(), &mut dst_view).unwrap();

//     // Divide RGB channels of destination image by alpha
//     alpha_mul_div.divide_alpha_inplace(&mut dst_view).unwrap();

//     // Write destination image as PNG-file
//     let mut result_buf = BufWriter::new(Vec::new());
//     PngEncoder::new(&mut result_buf)
//         .write_image(
//             dst_image.buffer(),
//             dst_width.get(),
//             dst_height.get(),
//             ColorType::Rgba8,
//         )
//         .unwrap();
// }

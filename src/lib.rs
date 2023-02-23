use std::env;
// use std::io::BufWriter;
// use std::num::NonZeroU32;

// use fast_image_resize as fr;
// use image::codecs::png::PngEncoder;
// use image::io::Reader as ImageReader;
// use image::{ColorType, ImageEncoder};
use json::object;
// use std::path::Path;
// use filesize::PathExt;
use id3::{Tag, TagLike};

mod setup;

// #[derive(Debug)]
// struct ImageInfo {
//     imageid: String,
//     fullpath: String,
//     basedir: String,
//     filename: String,
//     ext: String,
//     b64img: String,
//     width: u32,
//     height: u32,
//     idx: u32,
//     fsize: u32,
// }
// struct Mp3Info {
//     imageid: String,
//     fullpath: String,
//     basedir: String,
//     filename: String,
//     ext: String,
//     imgurl: String,
//     artist: String,
//     album: String,
//     song: String,
//     genre: String,
//     idx: u32,
//     fsize: u64,
// }

// pub fn get_file_size(x: String) -> u64 {
//     let path = Path::new(&x);
//     let realsize = path.size_on_disk().unwrap();

//     realsize
// }

pub fn process_music_images() {
    // let mtv_music_path = env::var("MTV_MUSIC_PATH").expect("$MTV_MUSIC_PATH is not set");

    let mp3_imagesvec = setup::mtvwalkdirs::walk_music_dir_images();

    let mut image_count = 0;

    for jpg in mp3_imagesvec {
        image_count = image_count + 1;

        let dims = setup::mtvimageops::get_image_dims(jpg.clone());
        let newdims = setup::mtvimageops::normalize_music_image(jpg.clone(), dims);
        // println!("{}", jpg.clone());

        let base_dir = setup::splitstrings::split_base_dir(jpg.clone());
        let file_name = setup::splitstrings::split_filename(jpg.clone());
        let extension = setup::splitstrings::split_ext(jpg.clone());

        let imginfo = object! {
            imageid: setup::misc::get_md5(jpg.clone()),
            fullpath: jpg.clone(),
            basedir: base_dir.clone(),
            filename: file_name.clone(),
            ext: extension.clone(),
            b64img: setup::mtvimageops::to_base64_str(jpg.clone(), newdims.0, newdims.1),
            width: newdims.0,
            height: newdims.1,
            idx: image_count,
            fsize: setup::misc::get_file_size(jpg.clone()),
        };

        let ifo = json::stringify(imginfo.dump());
        // "/media/charliepi/FOO/media/metadata_music"
        let mtv_music_metadata_path =
            env::var("MTV_MUSIC_METADATA_PATH")
                .expect("$MTV_MUSIC_METADATA_PATH is not set");


        let a = format!("{}/", mtv_music_metadata_path.as_str());
        let b = format!("MusicMeta_{}.json", image_count);
        let outpath = a + &b;

        println!("\n\n\n ifo {:#?}", ifo);
        std::fs::write(
            outpath,
            ifo,
        )
        .unwrap();

        // put it in a db
    }
    println!("There are {} jpgs", image_count);
}

pub fn get_tag_info(x: String) -> (String, String, String, String) {
    let tag = Tag::read_from_path(x).unwrap();
    let artist = tag.artist().unwrap().to_string();
    let album = tag.album().unwrap().to_string();
    let song = tag.title().unwrap().to_string();
    let genre = tag.genre().unwrap().to_string();

    (artist, album, song, genre)
}

pub fn process_mp3s() {
    let mp3svec = setup::mtvwalkdirs::walk_music_dir_mp3();

    let mut index = 0;
    for mp3 in mp3svec {
        index = index + 1;

        let voodoo = "None".to_string();

        let tags = get_tag_info(mp3.clone());

        let mp3_info = object! {
            mp3id: setup::misc::get_md5(mp3.clone()),
            fullpath: mp3.clone(),
            basedir: setup::splitstrings::split_base_dir(mp3.clone()),
            filename: setup::splitstrings::split_filename(mp3.clone()),
            ext: setup::splitstrings::split_ext(mp3.clone()),
            imgurl: voodoo.clone(),
            artist: tags.0,
            album: tags.1,
            song: tags.2,
            genre: tags.3,
            idx: index,
            fsize: setup::misc::get_file_size(mp3.clone()),
        };

        mp3_info.dump();

        println!("\n\n\n mp3info {}", mp3_info);
    }
    println!("There are {} mp3s", index.to_string());
}

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

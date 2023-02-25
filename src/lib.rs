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
// use id3::{Tag, TagLike};

mod setup;

pub fn process_music_images() {
    // let mtv_music_path = env::var("MTV_MUSIC_PATH").expect("$MTV_MUSIC_PATH is not set");

    let mp3_imagesvec = setup::mtvwalkdirs::walk_music_dir_images();

    let mut image_count = 0;

    for jpg in mp3_imagesvec {
        image_count = image_count + 1;

        let dims = setup::mtvimageops::get_image_dims(&jpg);
        let newdims = setup::mtvimageops::normalize_music_image(dims);
        let base_dir = setup::splitstrings::split_base_dir(&jpg);
        let file_name = setup::splitstrings::split_filename(&jpg);
        let extension = setup::splitstrings::split_ext(&jpg);

        let artist_results = setup::splitstrings::image_split_artist(&base_dir);
        println!("this is artist: {}", artist_results);

        let album_results = setup::splitstrings::image_split_album(&base_dir);

        // let music_artist_results = setup::splitstrings::music_split_artist(base_dir.clone());
        // println!("album is: {}", music_artist_results);

        let imginfo = object! {
            imageid: setup::misc::get_md5(&jpg),
            filename_artist: artist_results,
            filename_album: album_results,
            basedir: base_dir.clone(),
            filename: file_name.clone(),
            ext: extension.clone(),
            width: newdims.0,
            height: newdims.1,
            idx: image_count,
            fsize: setup::misc::get_file_size(&jpg),
            fullpath: &*jpg,
            b64img: setup::mtvimageops::to_base64_str(&jpg, newdims.0, newdims.1),

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

pub fn process_mp3s() {
    let mp3svec = setup::mtvwalkdirs::walk_music_dir_mp3();
    let mut named_incorrectly_vec = vec![];

    let mut index = 0;
    for mp3 in mp3svec {
        index = index + 1;

        let voodoo: &String = &"None".to_string();
        let tags = setup::mtvmp3::get_tag_info(&mp3);
        let base_dir = setup::splitstrings::split_base_dir(&mp3);
        let filename_results = setup::splitstrings::split_filename(&mp3);
        let music_artist_results = setup::splitstrings::music_split_artist(&base_dir);
        let music_album_results = setup::splitstrings::music_split_album(&base_dir);
        let duration_results = setup::mtvmp3::get_duration(&mp3);
        let artc = setup::mtvmp3::check_artist(&music_artist_results, &tags.0);
        let albc = setup::mtvmp3::check_album(&music_album_results, &tags.1);
        let sc = setup::mtvmp3::check_song(&filename_results, &tags.2);

        if artc == true && albc == true && sc == true {
            println!("\n they all match:\n {}", &mp3);

            let mp3_info = object! {
                mp3id: setup::misc::get_md5(&mp3),
                fullpath: &*mp3,
                basedir: &*base_dir,
                filename: filename_results,
                ext: setup::splitstrings::split_ext(&mp3),
                imgurl: &**voodoo,
                mp3_url: &**voodoo,
                tag_artist: &*tags.0,
                tag_album: &*tags.1,
                tag_title: &*tags.2,
                tag_genre: &*tags.3,
                idx: index,
                fsize: setup::misc::get_file_size(&mp3),
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

use std::env;
use std::io::BufWriter;
use std::num::NonZeroU32;

use image::codecs::png::PngEncoder;
use image::io::Reader as ImageReader;
use image::{ColorType, ImageEncoder};

use fast_image_resize as fr;

mod setup;

pub fn process_music_images() {
    let mtv_music_path = env::var("MTV_MUSIC_PATH").expect("$MTV_MUSIC_PATH is not set");

    let mp3_imagesvec = setup::mtvwalkdirs::walk_music_dir_images();

    let mut jpgcount = 0;
    let mut jpegcount = 0;
    let mut pngcount = 0;
    let mut webpcount = 0;
    let mut avifcount = 0;
    for jpg in mp3_imagesvec {
        let image_id = setup::misc::get_md5(jpg.clone());
        let ext = setup::splitstrings::split_ext(jpg.clone());
        if ext == "jpg" {
            jpgcount = jpgcount + 1;
            let dims = setup::mtvimageops::get_image_dims(jpg.clone());
            let newdims = setup::mtvimageops::normalize_music_image(jpg.clone(), dims);
            println!("{:?}", newdims);
        } else if ext == "jpeg" {
            jpegcount = jpegcount + 1;
            println!("{}", jpg);
        } else if ext == "png" {
            pngcount = pngcount + 1;
            let dims2 = setup::mtvimageops::get_image_dims(jpg.clone());
            println!("THIS IS PNG DIMS{:?}", dims2);
        } else if ext == "webp" {
            webpcount = webpcount + 1;
            println!("{}", jpg);
        } else if ext == "avif" {
            avifcount = avifcount + 1;
            println!("{}", jpg);
        } else {
            println!("this is wtf why are you here {}", jpg);
        }

        let base_dir = setup::splitstrings::split_base_dir(jpg.clone(), mtv_music_path.clone());

        // println!("this is full path:\n {}", jpg);
        // println!("this is jpg id: \n {}", image_id);
        // println!("this is jpg basedir:\n {}", base_dir);
        // println!("this is ext {}", ext);
        // println!("this is f: {:?}", i);
    }
    println!("There are {} jpgs", jpgcount.to_string());
    println!("There are {} jpegs", jpegcount.to_string());
    println!("There are {} pngs", pngcount.to_string());

    println!("There are {} webp", webpcount.to_string());
    println!("There are {} avif", avifcount.to_string());
}

pub fn process_mp3s() {
    let mtv_music_path = env::var("MTV_MUSIC_PATH").expect("$MTV_MUSIC_PATH is not set");

    let mp3svec = setup::mtvwalkdirs::walk_music_dir_mp3();

    let mut mp3count = 0;
    for mp3 in mp3svec {
        let mp3_id = setup::misc::get_md5(mp3.clone());
        let mp3_base_dir = setup::splitstrings::split_base_dir(mp3.clone(), mtv_music_path.clone());
        let ext = setup::splitstrings::split_ext(mp3.clone());
        mp3count = mp3count + 1;
        println!("this is full path:\n {}", mp3);
        println!("this is mp3 id: \n {}", mp3_id);
        println!("This is mp3 basedir:\n {}", mp3_base_dir);
        println!("This is ext: {}", ext);
    }
    println!("There are {} mp3s", mp3count.to_string());
}

fn resize_image(x: String, dimstup: (u32, u32)) {
    // Read source image from file
    let img = ImageReader::open(x).unwrap().decode().unwrap();
    let width = NonZeroU32::new(img.width()).unwrap();
    let height = NonZeroU32::new(img.height()).unwrap();
    let mut src_image = fr::Image::from_vec_u8(
        width,
        height,
        img.to_rgba8().into_raw(),
        fr::PixelType::U8x4,
    )
    .unwrap();

    // Multiple RGB channels of source image by alpha channel
    // (not required for the Nearest algorithm)
    let alpha_mul_div = fr::MulDiv::default();
    alpha_mul_div
        .multiply_alpha_inplace(&mut src_image.view_mut())
        .unwrap();

    // Create container for data of destination image
    let dst_width = NonZeroU32::new(dimstup.0).unwrap();
    let dst_height = NonZeroU32::new(dimstup.1).unwrap();
    let mut dst_image = fr::Image::new(dst_width, dst_height, src_image.pixel_type());

    // Get mutable view of destination image data
    let mut dst_view = dst_image.view_mut();

    // Create Resizer instance and resize source image
    // into buffer of destination image
    let mut resizer = fr::Resizer::new(fr::ResizeAlg::Convolution(fr::FilterType::Lanczos3));
    resizer.resize(&src_image.view(), &mut dst_view).unwrap();

    // Divide RGB channels of destination image by alpha
    alpha_mul_div.divide_alpha_inplace(&mut dst_view).unwrap();

    // Write destination image as PNG-file
    let mut result_buf = BufWriter::new(Vec::new());
    PngEncoder::new(&mut result_buf)
        .write_image(
            dst_image.buffer(),
            dst_width.get(),
            dst_height.get(),
            ColorType::Rgba8,
        )
        .unwrap();
}

use image;
use base64::{engine, alphabet, Engine as _};
// use image::codecs::png::PngEncoder;
// use image::io::Reader as ImageReader;
use image::GenericImageView;
// use image::{ColorType, ImageEncoder};
// use std::io::BufWriter;
// use std::num::NonZeroU32;

// use fast_image_resize as fr;

pub fn get_image_dims(x: String) -> (u32, u32) {
    let img = image::open(x).unwrap();

    let dims = img.dimensions();

    dims
}

pub fn normalize_music_image(img: String, dims: (u32, u32)) -> (u32, u32) {
    let mut largest = 0;

    if dims.0 == dims.1 {
        largest = dims.0;
    } else if dims.0 > dims.1 {
        largest = dims.0;
    } else {
        largest = dims.1;
    }

    let mut resizetup = (0, 0);
    if largest < 100 {
        resizetup = (100, 100);
    } else if largest < 200 {
        resizetup = (200, 200);
    } else if largest < 300 {
        resizetup = (300, 300);
    } else {
        resizetup = (300, 300);
    }

    // resize_image(img, resizetup);

    resizetup
}

pub fn to_base64_str(x: String, w: u32, h: u32) -> String{
    // println!("{}", x.clone());
    let img_result = image::open(x.clone());
    let img = match img_result {
        Ok(img) => img,
        Err(error) => panic!("problem opening file {:?}", error),
    };
    let thumb = img.thumbnail(w, h);
    let thumb_bytes = thumb.into_bytes();
    // let thumb_bytes = image::open(x).unwrap().into_bytes();
    
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


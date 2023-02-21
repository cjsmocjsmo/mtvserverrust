use image::GenericImageView;
// use std::io::Cursor;
// use image::io::Reader as ImageReader;

// use std::io::BufWriter;
// use std::num::NonZeroU32;

// use image::codecs::png::PngEncoder;
// use image::io::Reader as ImageReader;
use image;

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

// pub fn process_music_images() {
//     let mtv_music_path = env::var("MTV_MUSIC_PATH").expect("$MTV_MUSIC_PATH is not set");

//     let mp3_imagesvec = setup::mtvwalkdirs::walk_music_dir_images();

//     let mut jpgcount = 0;
//     let mut jpegcount = 0;
//     let mut pngcount = 0;
//     let mut webpcount = 0;
//     let mut avifcount = 0;
//     for jpg in mp3_imagesvec {
//         let image_id = setup::misc::get_md5(jpg.clone());
//         let ext = setup::splitstrings::split_ext(jpg.clone());
//         if ext == "jpg" {
//             jpgcount = jpgcount + 1;
//             let dims = get_image_dims(jpg.clone());
//             let newdims = normalize_music_image(jpg.clone(), dims);
//             println!("{:?}", newdims);
//         } else if ext == "jpeg" {
//             jpegcount = jpegcount + 1;
//             println!("{}", jpg);
//         } else if ext == "png" {
//             pngcount = pngcount + 1;
//             let dims2 = get_image_dims(jpg.clone());
//             println!("THIS IS PNG DIMS{:?}", dims2);
//         } else if ext == "webp" {
//             webpcount = webpcount + 1;
//             println!("{}", jpg);
//         } else if ext == "avif" {
//             avifcount = avifcount + 1;
//             println!("{}", jpg);
//         } else {
//             println!("this is wtf why are you here {}", jpg);
//         }

        
//         let base_dir = setup::splitstrings::split_base_dir(jpg.clone(), mtv_music_path.clone());
        
        
//         // println!("this is full path:\n {}", jpg);
//         // println!("this is jpg id: \n {}", image_id);
//         // println!("this is jpg basedir:\n {}", base_dir);
//         // println!("this is ext {}", ext);
//         // println!("this is f: {:?}", i);
//     }
//     println!("There are {} jpgs", jpgcount.to_string());
//     println!("There are {} jpegs", jpegcount.to_string());
//     println!("There are {} pngs", pngcount.to_string());

//     println!("There are {} webp", webpcount.to_string());
//     println!("There are {} avif", avifcount.to_string());
// }
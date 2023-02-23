use byte_unit::Byte;
use filesize::PathExt;
use md5::{Digest, Md5};
use std::path::Path;
use walkdir::WalkDir;

pub fn get_md5(astring: String) -> String {
    let mut hasher2 = Md5::new();
    hasher2.update(astring.clone());
    let a_id = hasher2.finalize();
    let foo = format!("{:x}", a_id);

    foo
}

pub fn get_file_size(x: String) -> u64 {
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

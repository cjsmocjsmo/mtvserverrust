use id3::{Tag, TagLike};

pub fn get_tag_info(x: String) -> (String, String, String, String) {
    let tag = Tag::read_from_path(x).unwrap();
    let artist = tag.artist().unwrap().to_string();
    let album = tag.album().unwrap().to_string();
    let song = tag.title().unwrap().to_string();
    let genre = tag.genre().unwrap().to_string();

    (artist, album, song, genre)
}
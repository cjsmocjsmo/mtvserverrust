use std::path::Path;

pub fn split_ext(astring: String) -> String {
    let path = Path::new(&astring);
    let boo_results = path.extension();
    let boo = match boo_results {
        Some(b) => b.to_string_lossy().to_string(),
        None => String::from("split_ext did not work"),
    };

    boo
}

pub fn split_base_dir(x: String, v: String) -> String {
    let path = Path::new(&x);
    let boo_results = path.strip_prefix(v);
    let boo = match boo_results {
        Ok(b) => b.to_string_lossy().to_string(),
        Err(error) => panic!("it didnt work: {:?}", error),
    };

    boo
    // let items: Vec<&str> = x.split("/media/charliepi/FOO/music/").collect();
    // items[1].to_string()
    // set MTV_MEDIA_PATH=/media/charliepi/FOO/music/music/
}

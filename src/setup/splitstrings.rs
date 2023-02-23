use std::env;
use std::path::Path;

pub fn split_ext(astring: String) -> String {
    let path = Path::new(&astring);
    let boo_results = path.extension();
    let boo = match boo_results {
        Some(b) => b.to_string_lossy().to_string(),
        None => String::from("split_ext did not work"),
    };

    let ext = String::from(".") + boo.as_str();

    ext
}

pub fn split_base_dir(astring: String) -> String {
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

pub fn split_filename(x: String) -> String {
    let filesplit = x.split("/");
    let mut filenamevec = vec!();
    for file in filesplit {
        filenamevec.push(file);
    }

    let count = &filenamevec.len() -1;
    filenamevec.drain(0..count);
    let mut finalvec = "";
    for f in filenamevec {
        finalvec = f;
    }

    let fname = finalvec.split(".");
    let mut svec = vec!();
    // let mut foo = "";
    for f in fname {
        svec.push(f);

    }
    svec.pop();

    let filename = svec.get(0).unwrap();

    filename.to_string()

    
}

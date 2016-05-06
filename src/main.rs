use std::io;
use std::io::Read;
use std::fs;
use std::path::PathBuf;

fn add_cmdline(mut path: PathBuf) -> PathBuf {
    path.push("cmdline");
    path
}

fn get_cmdlines_paths() -> io::Result<Vec<PathBuf>> {
    let paths: Vec<PathBuf> = try!(fs::read_dir("/proc"))
        .filter(|res| res.is_ok())
        .map(|res| res.unwrap().path())
        .filter(|path| path.is_dir())
        .filter(|path| path.file_name().unwrap().to_str().unwrap().parse::<i32>().is_ok())
        .map(add_cmdline)
        .collect();
    Ok(paths)
}

fn is_emacs_running(paths: Vec<PathBuf>) -> bool {
    let mut count = 0;

    for path in paths {
        let mut file = fs::File::open(path).unwrap();
        let mut content = String::new();

        file.read_to_string(& mut content);
        content.trim();

        if content.find("emacs").is_some() {
            count += 1;
        }
    }

    if count >= 2 {
        return true;
    } else {
        return false;
    }
}

fn main() {
    let paths = get_cmdlines_paths();
    let is_running = is_emacs_running(paths.unwrap());
    println!("is_emacs_running: {}", is_running);
}

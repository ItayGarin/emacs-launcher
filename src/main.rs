use std::io;
use std::io::Read;
use std::fs;
use std::path::PathBuf;
// use std::ffi::OsStr;
use std::process::{Child, Command};

fn launch_emacs_server() -> io::Result<Child> {
    Command::new("/usr/bin/emacs")
        .spawn()
}


fn launch_emacs_client() -> io::Result<Child> {
    Command::new("/usr/bin/emacsclient")
        .arg("-c")
        .spawn()
}


fn add_cmdline_to_path(mut path: PathBuf) -> PathBuf {
    path.push("cmdline");
    path
}

fn is_file_name_an_int(path: &PathBuf) -> Option<bool> {
    let file_name = match path.file_name() {
        Some(name) => name,
        None => return None,
    };

    let str_file_name = match file_name.to_str() {
        Some(name) => name,
        None => return None,
    };

    Some(str_file_name.parse::<i32>().is_ok())
}

fn get_cmdlines_paths() -> io::Result<Vec<PathBuf>> {
    let paths = try!(fs::read_dir("/proc"))
        .filter(|res| res.is_ok())
        .map(|res| res.unwrap().path())
        .filter(|path| path.is_dir())
        .filter(|path| is_file_name_an_int(path).unwrap_or(false))
        .map(add_cmdline_to_path)
        .collect();
    Ok(paths)
}

fn is_emacs_running(paths: Vec<PathBuf>) -> io::Result<bool> {
    let mut count = 0;

    for path in paths {
        let mut file = try!(fs::File::open(path));

        let mut content = String::new();
        try!(file.read_to_string(& mut content));
        content.trim();

        if content.find("emacs").is_some() {
            count += 1;
        }
    }

    if count >= 2 {
        return Ok(true);
    } else {
        return Ok(false);
    }
}

fn main() {
    let paths = get_cmdlines_paths()
        .expect("Failed to retrieve the /proc cmdline paths");

    let is_running = is_emacs_running(paths)
        .expect("Failed to read the processes' 'cmdlines' while checking Emacs's status");

    match is_running {
        true => launch_emacs_client().expect("Failed to launch an Emacs client"),
        false => launch_emacs_server().expect("Failed to launch an Emacs server"),
    };
}

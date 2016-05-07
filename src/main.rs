use std::io;
use std::io::Read;
use std::fs;
use std::path::PathBuf;
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


fn add_cmdline(mut path: PathBuf) -> PathBuf {
    path.push("cmdline");
    path
}

fn get_cmdlines_paths() -> io::Result<Vec<PathBuf>> {
    let paths = try!(fs::read_dir("/proc"))
        .filter(|res| res.is_ok())
        .map(|res| res.unwrap().path())
        .filter(|path| path.is_dir())
        .filter(|path| path.file_name().unwrap().to_str().unwrap().parse::<i32>().is_ok())
        .map(add_cmdline)
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

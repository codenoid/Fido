use colored::*;
use clap::{Arg, App};
use nix::sys::statvfs::statvfs;
use walkdir::WalkDir;

macro_rules! cast {
    ($x:expr) => {
        u64::from($x)
    };
}

fn main() {

    let matches = App::new("Fido")
        .version("0.1.0")
        .author("codenoid <jihantoro@pm.me>")
        .about("This app help you build a distributed data store")
        .arg(Arg::with_name("cmd")
                .required(true)
                .short("c")
                .long("cmd")
                .takes_value(true)
                .help("0 (generate symlink in master path) | 1 (move file from temporary location to bricks and generate symbolic file"))
        .arg(Arg::with_name("path")
                .required(true)
                .short("p")
                .long("path")
                .takes_value(true)
                .help("path to be mirrored"))
        .arg(Arg::with_name("bricks")
                .required(false)
                .short("b")
                .long("bricks")
                .takes_value(true)
                .help("path to folder that contain mounted disk / brick, ex: brick-1, brick-2, and so on..."))
        .get_matches();

    let cmd = matches.value_of("cmd").unwrap();
    let path = matches.value_of("path").unwrap();
    let brick_path = matches.value_of("bricks").unwrap();

    if path.clone().chars().last().unwrap() != '/' {
    	println!("the end of --path must be /");
    	::std::process::exit(1);
    }

    if cmd == "0" {
        // bricks = /data/ -> /data/brick-1
        for brick in WalkDir::new(brick_path.clone().to_string()).min_depth(1).into_iter().filter_map(|e| e.ok()) {
        	let listed_path = brick.path().display().to_string();
        	let pure_path = listed_path.replace(brick_path.clone(), "");

        	if brick.file_type().is_dir() {
        		let listed_folder = format!("{}{}", path, pure_path);
        		println!("[INFO] mkdir -p folder : {}", listed_folder);
        		::std::fs::create_dir_all(listed_folder);
        	} else {
	        	// gerimis, mau balik, lanjut di rumah
	        	let build_symlink_path = format!("{}{}", path, pure_path);
	            println!("[INFO] ln -s : {}", build_symlink_path);

	            let args = &["-s", &listed_path, &build_symlink_path];

				let status = ::std::process::Command::new("/bin/ln")
                    .args(args)
                    .status()
                    .expect("failed to execute process");

                if status.success() {
                	println!("{}", "[INFO] Successfully linkin path...".green().on_black());
                } else {
                	println!("{}", "[INFO] Path linkin failed...".blue());
                }
        	}
        }
    }

    if cmd == "1" {
	    for entry in WalkDir::new(path).min_depth(1).into_iter().filter_map(|e| e.ok()) {
	        if entry.path_is_symlink() == false {
                let get_brick = get_available_brick(brick_path.clone().to_string());
                if get_brick == "" {
                    println!("FATAL!!!! NEED MORE BRICK !!!");
                    ::std::process::exit(1);
                }

                let file_path = entry.path().display();
	        }
	    }
    }

    println!("{}", cmd);
}

fn is_available(path: String) -> bool {
    let stat = statvfs(path.as_bytes()).unwrap();

    let total_space = cast!(stat.block_size()) * cast!(stat.blocks());
    let avail_space = cast!(stat.block_size()) * cast!(stat.blocks_available());
    let used = total_space - avail_space;
    let usage = used * 100 / total_space;

    return usage < 95
}

fn get_available_brick(path: String) -> String {
    for entry in WalkDir::new(path).min_depth(1).max_depth(1).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_dir() {
            let brick_path = entry.path().display().to_string();
            if is_available(brick_path.clone()) {
                return brick_path
            }
        }
    }
    return String::from("");
}
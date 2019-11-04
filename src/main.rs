extern crate clap;

use walkdir::WalkDir;
use clap::{Arg, App};
use nix::sys::statvfs::statvfs;

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

    for entry in WalkDir::new(path).min_depth(1).into_iter().filter_map(|e| e.ok()) {
        if entry.path_is_symlink() == false {

        	if cmd == "0" {
        		let file_path = entry.path().display();

        		// bricks = /data/ -> /data/brick-1
        		for entry in WalkDir::new(path).min_depth(1).into_iter().filter_map(|e| e.ok()) {
        	}

        	if cmd == "1" {
	        	get_brick = get_available_brick(brick_path);
	        	if get_brick == "" {
	        		println!("FATAL!!!! NEED MORE BRICK !!!");
	        		::std::process::exit(1)
	        	}
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
    		println!("brick path : {}", brick_path);
    		if is_available(brick_path.clone()) {
    			return brick_path
    		}
    	}
    }
    return String::from("");
}
use bson::doc;
use clap::{App, Arg};
use colored::*;
use mime_db::lookup;
use mongodb::{options::ClientOptions, Client};
use nix::sys::statvfs::statvfs;
#[allow(dead_code)]
#[allow(unreachable_code)]
use std::time::UNIX_EPOCH;
use walkdir::WalkDir;

macro_rules! cast {
    ($x:expr) => {
        u64::from($x)
    };
}

fn main() {
    let matches = App::new("Fido")
        .version("0.1.2")
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
                .required(true)
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

    if brick_path.clone().chars().last().unwrap() != '/' {
        println!("the end of --path must be /");
        ::std::process::exit(1);
    }

    // Parse a connection string into an options struct.
    let mut client_options = match ClientOptions::parse("mongodb://localhost:27017") {
        Ok(expr) => expr,
        Err(_e) => ::std::process::exit(1),
    };

    // Manually set an option.
    client_options.app_name = Some("Fido".to_string());

    // Get a handle to the deployment.
    let database = match Client::with_options(client_options) {
        Ok(result) => result.database("fido_meta"),
        Err(_e) => ::std::process::exit(1),
    };

    if cmd == "0" {
        // bricks = /data/ -> /data/brick-1
        for brick in WalkDir::new(brick_path.clone().to_string())
            .min_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if brick.path_is_symlink() {
                continue;
            }
            let listed_path = brick.path().display().to_string();
            let pure_path = listed_path.replace(brick_path.clone(), "");

            if !brick.file_type().is_dir() {
                // get path metadata
                let md = ::std::fs::metadata(listed_path.clone()).unwrap();

                // gerimis, mau balik, lanjut di rumah
                println!("[INFO] ln -s : {}", pure_path);

                let mime = match lookup(pure_path.clone()) {
                    Some(result) => result,
                    None => "",
                };

                let doc = doc! {
                    "original_path": pure_path.clone(),
                    "shared_path": listed_path,
                    "node_path": brick_path,
                    "replication_node": "",
                    "mime_type": mime,
                    "file_size": md.len() / 1000, // get kb value as int
                    "created_at": md.created().unwrap().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                };

                let ok = database.collection("link").insert_one(doc, None);

                match ok {
                    Ok(_) => {
                        println!(
                            "{}",
                            "[META] Successfully save path to database..."
                                .blue()
                                .on_black()
                        );
                    }
                    Err(_) => {
                        println!("{}", "[META] Failed to save meta data...".red());
                    }
                }
            }
        }
    }

    if cmd == "1" {
        loop {
            for entry in WalkDir::new(path)
                .min_depth(1)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let listed_path = entry.path().display().to_string();
                let pure_path = listed_path.replace(path, "");

                if entry.path_is_symlink() == false {
                    let get_brick = get_available_brick(brick_path.clone().to_string());
                    if get_brick == "" {
                        println!("FATAL!!!! NEED MORE BRICK !!!");
                        ::std::process::exit(1);
                    }

                    // brick_path + pure path
                    let move_path = format!("{}/{}", get_brick, pure_path);

                    // get path metadata
                    let md = ::std::fs::metadata(listed_path.clone()).unwrap();

                    if md.is_dir() {
                        ::std::fs::create_dir_all(move_path).unwrap();
                    } else {
                        // doesn't work with different mount point
                        // ::std::fs::rename(listed_path.clone(), move_path.clone());

                        if let Ok(time) = md.modified() {
                            match time.elapsed() {
                                Ok(diff) => {
                                    // if file hasn't been modified since 3 minute ago
                                    // then proceed the file
                                    if diff.as_secs() > 150 {
                                        println!(
                                            "[MOVING] \nFROM {:?} \nTO {:?}",
                                            listed_path.clone(),
                                            move_path.clone()
                                        );

                                        let move_file = ::std::process::Command::new("/bin/mv")
                                            .arg(listed_path.clone())
                                            .arg(move_path.clone())
                                            .status()
                                            .expect("failed to execute process");

                                        if move_file.success() {
                                            println!(
                                                "{}",
                                                "[MOVE] Successfully shard the file path..."
                                                    .green()
                                                    .on_black()
                                            );

                                            let mime = match lookup(pure_path.clone()) {
                                                Some(result) => result,
                                                None => "",
                                            };

                                            let doc = doc! {
                                                "original_path": pure_path.clone(),
                                                "shared_path": move_path.clone(),
                                                "node_path": get_brick.clone(),
                                                "replication_node": "",
                                                "mime_type": mime,
                                                "file_size": md.len() / 1000, // get kb value as int
                                                "created_at": md.created().unwrap().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                                            };

                                            let ok =
                                                database.collection("link").insert_one(doc, None);
                                            match ok {
                                                Ok(_) => {
                                                    println!(
	                                                    "{}",
	                                                    "[META] Successfully save path to database..."
	                                                        .blue()
	                                                        .on_black()
	                                                );
                                                }
                                                Err(_) => {
                                                    println!(
                                                        "{}",
                                                        "[META] Failed to save meta data...".red()
                                                    );
                                                }
                                            }
                                        } else {
                                            println!("{}", "[ERROR] Failed to mv file...".red());
                                            ::std::process::exit(1);
                                        }
                                    }
                                }
                                Err(_err) => {}
                            }
                        } else {
                            println!("Not supported on this platform");
                        }
                    }
                }
            }
        }
    }
}

fn is_available(path: String) -> bool {
    let stat = statvfs(path.as_bytes()).unwrap();

    let total_space = cast!(stat.block_size()) * cast!(stat.blocks());
    let avail_space = cast!(stat.block_size()) * cast!(stat.blocks_available());
    let used = total_space - avail_space;
    let usage = used * 100 / total_space;

    return usage < 95;
}

fn get_available_brick(path: String) -> String {
    for entry in WalkDir::new(path)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_dir() {
            let brick_path = entry.path().display().to_string();
            if is_available(brick_path.clone()) {
                return brick_path;
            }
        }
    }
    return String::from("");
}

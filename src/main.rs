use chrono::Utc;
use clap::Parser;
use std::env;
use std::fs::{File, exists, rename};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(name = "rdel")]
#[command(version = "0.6")]
#[command(about = r#"A command to delete files and directories to trash
Copyright (C) 2025 Dilnavas Roshan"#
)]
#[command(long_about=None)]
struct Args {
    file_paths: Vec<String>,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let trash_path = get_trash_dir_path();

    for path in args.file_paths {
        let path = Path::new(&path);
        let mut trash_file_path = trash_path.clone();
        trash_file_path.push("files");
        let mut trash_info_path = trash_path.clone();
        trash_info_path.push("info");
        if let Some(filename) = path.file_name() {
            let filename = filename.to_str().expect("invalid UTF-8 string");
            let filename = match need_postfix(&trash_file_path, filename) {
                Some(name) => name,
                None => String::from(filename),
            };
            trash_file_path.push(&filename);
            create_trash_info_file(&filename, &trash_info_path, &path)?;
        } else {
            continue;
        }

        rename(path, &trash_file_path)?;
    }

    Ok(())
}

fn get_trash_dir_path() -> PathBuf {
    let xdg_data_dir = env::var("XDG_DATA_HOME");

    let mut trash_path = match xdg_data_dir {
        Ok(path) => PathBuf::from(path),
        Err(_e) => {
            let home_dir = env::var("HOME");
            PathBuf::from(home_dir.expect("Error"))
        }
    };

    trash_path.push(".local");
    trash_path.push("share");
    trash_path.push("Trash");
    trash_path
}

fn create_trash_info_file(
    filename: &str,
    info_path: &Path,
    original_path: &Path,
) -> std::io::Result<()> {
    let info_filename = String::from(filename) + ".trashinfo";
    let info_path = info_path.join(info_filename);
    let mut file = File::create(info_path)?;
    let time_stamp = get_time_stamp();
    let content = std::format!(
        "[Trash Info]\nPath={}\nDeletionDate={}\n",
        original_path
            .to_str()
            .expect("path is not a valid UTF-8 string"),
        time_stamp
    );
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn get_time_stamp() -> String {
    let now = Utc::now();
    now.format("%Y-%m-%dT%H:%M:%S").to_string()
}

fn need_postfix(trash_file_path: &Path, filename: &str) -> Option<String> {
    match exists(trash_file_path.join(filename)) {
        Ok(val) => {
            if val == false {
                return None;
            }
        }
        Err(_) => panic!("file operation error"),
    }

    let mut postfix = 1;
    loop {
        let new_filename = std::format!("{}.{}", filename, postfix);
        match exists(trash_file_path.join(&new_filename)) {
            Ok(val) => {
                if val == false {
                    return Some(new_filename);
                }
            }
            Err(_) => panic!("file operation error"),
        }
        postfix += 1;
    }
}

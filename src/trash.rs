use chrono::Utc;
use std::env;
use std::fs::{File, exists, rename};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

pub struct Trash {
    path: PathBuf,
}

impl Trash {
    pub fn new() -> Self {
        let mut insatnce = Self {
            path: PathBuf::from(""),
        };

        insatnce.load_path();
        insatnce
    }

    pub fn delete(&self, path: &Path) -> std::io::Result<()> {
        let filename = path
            .file_name()
            .expect("failed to extract filename")
            .to_str()
            .expect("invalid UTF-8 string");
        let trash_filename = self.create_filename(filename);
        self.create_trash_info_file(path, &trash_filename)?;
        let mut file_path = self.get_file_path();
        file_path.push(trash_filename);
        rename(path, file_path)?;
        Ok(())
    }

    fn load_path(&mut self) {
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
        self.path.push(trash_path);
    }

    fn get_file_path(&self) -> PathBuf {
        let mut path = self.path.clone();
        path.push("files");
        path
    }

    fn get_info_path(&self) -> PathBuf {
        let mut path = self.path.clone();
        path.push("info");
        path
    }

    fn create_filename(&self, filename: &str) -> String {
        let path: &Path = &self.path;
        match exists(path.join(filename)) {
            Ok(val) => {
                if val == false {
                    return String::from(filename);
                }
            }
            Err(_) => panic!("file operation error"),
        }

        let mut postfix = 1;
        loop {
            let new_filename = std::format!("{}.{}", filename, postfix);
            match exists(path.join(&new_filename)) {
                Ok(val) => {
                    if val == false {
                        return new_filename;
                    }
                }
                Err(_) => panic!("file operation error"),
            }
            postfix += 1;
        }
    }

    fn get_time_stamp() -> String {
        let now = Utc::now();
        now.format("%Y-%m-%dT%H:%M:%S").to_string()
    }

    fn create_trash_info_file(&self, path: &Path, trash_filename: &str) -> std::io::Result<()> {
        let info_filename = String::from(trash_filename) + ".trashinfo";
        let mut info_path = self.get_info_path();
        info_path.push(info_filename);
        let mut file = File::create(info_path)?;
        let time_stamp = Self::get_time_stamp();
        let content = std::format!(
            "[Trash Info]\nPath={}\nDeletionDate={}\n",
            path.to_str().expect("path is not a valid UTF-8 string"),
            time_stamp
        );
        file.write_all(content.as_bytes())?;
        Ok(())
    }
}

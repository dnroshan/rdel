use clap::Parser;
use std::path::Path;

pub mod trash;

#[derive(Parser, Debug)]
#[command(name = "rdel")]
#[command(version = "0.6")]
#[command(about = r#"A command to delete files and directories to trash
Copyright (C) 2025 Dilnavas Roshan"#)]
#[command(long_about=None)]

struct Args {
    file_paths: Vec<String>,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let trash = trash::Trash::new();

    for path in args.file_paths {
        let path = Path::new(&path);
        trash.delete(path)?;
    }

    Ok(())
}

use std::fs;
use std::io::Result;
use std::path::Path;

use clap::{App, Arg};
use human_bytes::human_bytes;
use walkdir::WalkDir;

pub struct Config {
    dir: String,
    dry_run: bool,
}

impl Config {
    pub fn parse() -> Self {
        let matches = App::new("rsclean")
            .about("A tool to gain space by removing Rust dependencies")
            .arg(
                Arg::new("dir")
                    .help("Directory you want to clean up")
                    .takes_value(true)
                    .required(true),
            )
            .arg(
                Arg::new("dryrun")
                    .long("dry-run")
                    .short('d')
                    .help("Show what would have been deleted")
                    .takes_value(false),
            )
            .get_matches();

        Self {
            dir: matches.get_one::<String>("dir").unwrap().to_string(),
            dry_run: matches.is_present("dryrun"),
        }
    }

    pub fn run(&self) -> Result<()> {
        let deleted_dirs = find_dirs(&self.dir);
        let reclamed_size = delete_dirs(deleted_dirs, self.dry_run)?;

        println!("Space reclamed: {}", human_bytes(reclamed_size as f64));

        Ok(())
    }
}

fn find_dirs<P>(path: P) -> Vec<DeletedDirectory>
where
    P: AsRef<Path>,
{
    let mut results = vec![];

    for entry in WalkDir::new(path)
        .into_iter()
        .flatten()
        .filter(|e| e.file_type().is_dir() && e.file_name() == "target")
    {
        let name = entry.path().display().to_string();
        let size = size_of_dir(&name).unwrap();

        results.push(DeletedDirectory { size, name });
    }

    results
}

fn size_of_dir(dir: &String) -> Result<u64> {
    let size = WalkDir::new(dir)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.metadata().ok())
        .fold(0, |acc, m| acc + m.len());

    Ok(size)
}

fn delete_dirs(dirs: Vec<DeletedDirectory>, dry_run: bool) -> Result<u64> {
    let mut total_size = 0;

    for dir in dirs {
        total_size += dir.size;
        if dry_run {
            println!("{} {}", human_bytes(dir.size as f64), dir.name);
        } else {
            fs::remove_dir_all(dir.name)?;
        }
    }

    Ok(total_size)
}

struct DeletedDirectory {
    size: u64,
    name: String,
}

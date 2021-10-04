use std::{fs::{File, read_dir}, path::{Path, PathBuf}};

use anyhow::{anyhow, Result};
use indicatif::{ProgressBar, ProgressStyle};
use libosu::beatmap::Beatmap;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

const DATA_PATH: &str = "./data";

fn main() -> Result<()> {
    println!("hello world!");

    let data_path = PathBuf::from(DATA_PATH);
    if !data_path.exists() {
        return Err(anyhow!("missing data path {}", DATA_PATH));
    }

    let dir: Vec<PathBuf> = read_dir("./data")?
        .map(|e| e.unwrap().path())
        .collect();

    let progress = ProgressBar::new(dir.len() as u64)
        .with_style(
            ProgressStyle::default_bar()
                .template("{msg:17} [{wide_bar}] {percent:>3}% {pos:>7}/{len:7}")
        );

    dir.par_iter().for_each(|path| {
        let filename = path.file_name()
            .and_then(|file| file
                .to_str()
                .map(|s| s.to_owned()))
            .unwrap_or("???".into());

        progress.set_message(filename.clone());
        if let Err(e) = process(&path) {
            progress.println(format!(
                "Error occurred while processing {:?}",
                &filename
            ));
            progress.println(format!("{:?}", e));
            progress.println("");
        }
        progress.inc(1);
    });

    Ok(())
}

fn process(path: &Path) -> Result<()> {
    let fd = File::open(path)?;

    Beatmap::parse(fd)?;

    Ok(())
}

extern crate indicatif;
extern crate rayon;
extern crate serde_json;

use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use serde_json::Value;
use std::error::Error;
use std::path::Path;
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args().collect();
    let path = Path::new(match args.len() {
        1 => Path::new("."),
        _ => Path::new(&args[1]),
    });

    let entries: Vec<_> = std::fs::read_dir(path)?.filter_map(Result::ok).collect();
    let pbar = ProgressBar::new(entries.len() as u64);
    pbar.set_style(
        ProgressStyle::default_bar()
            .template("{spinnerx} [{elapsed_precise}] [{bar:40}] {pos}/{len} ({eta})"),
    );
    let over: Vec<_> = entries
        .par_iter()
        .filter_map(|entry| {
            let fname = entry.path().to_str().unwrap().to_owned();
            let result = is_over_720p(&fname);
            pbar.inc(1);
            match result {
                Ok(true) => Some(entry),
                _ => None,
            }
        })
        .collect();
    pbar.finish();
    for fname in over {
        println!("{}", fname.file_name().into_string().unwrap());
    }
    Ok(())
}

fn is_over_720p(filename: &str) -> Result<bool, Box<dyn Error>> {
    let args = &["-v", "quiet", "-of", "json", "-show_streams", filename];
    let output = Command::new("ffprobe").args(args).output()?;
    let stdout = std::str::from_utf8(&output.stdout)?;
    let probe: Value = serde_json::from_str(stdout)?;
    if let Value::Array(ref streams) = probe["streams"] {
        Ok(streams.iter().any(|s| match s["height"] {
            Value::Number(ref n) => n.as_i64().unwrap() > 720,
            _ => false,
        }))
    } else {
        Ok(false)
    }
}

extern crate indicatif;
extern crate rayon;
extern crate serde_json;

use serde_json::Value;
use rayon::prelude::*;
use indicatif::ProgressBar;
use std::process::Command;
use std::path::Path;
use std::error::Error;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let path = Path::new(match args.len() {
        1 => Path::new("."),
        _ => Path::new(&args[1]),
    });

    let entries: Vec<_> = std::fs::read_dir(path).unwrap().collect();
    let pbar = ProgressBar::new(entries.len() as u64);
    let over: Vec<_> = entries
        .par_iter()
        .filter_map(|entry| {
            let fname = entry.as_ref().unwrap().path().to_str().unwrap().to_owned();
            let result = is_over_720p(&fname);
            pbar.inc(1);
            match result {
                Ok(true) => Some(fname),
                _ => None,
            }
        })
        .collect();
    pbar.finish();
    for fname in over {
        println!("{}", fname);
    }
}

fn is_over_720p(filename: &str) -> Result<bool, Box<Error>> {
    let args = &["-v", "quiet", "-of", "json", "-show_streams", filename];
    let output = Command::new("ffprobe").args(args).output()?;
    let stdout = std::str::from_utf8(&output.stdout)?;
    let probe: Value = serde_json::from_str(stdout)?;
    match probe["streams"] {
        Value::Array(ref streams) => Ok(streams.iter().any(|s| match s["height"] {
            Value::Number(ref n) => n.as_i64().unwrap() > 720,
            _ => false,
        })),
        _ => Ok(false),
    }
}

extern crate rayon;
extern crate serde_json;

use serde_json::Value;
use rayon::prelude::*;
use std::process::Command;
use std::path::Path;
use std::error::Error;
use std::fs;

fn main() {
    let path = &Path::new(".");
    let entries: Vec<_> = fs::read_dir(path).unwrap().collect();
    let over: Vec<_> = entries
        .par_iter()
        .filter_map(|entry| {
            let fname = entry.as_ref().unwrap().path().to_str().unwrap().to_owned();
            match is_over_720p(&fname) {
                Ok(true) => Some(fname),
                _ => None,
            }
        })
        .collect();
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
            Value::Number(ref n) => n.as_i64().unwrap() > 700,
            _ => false,
        })),
        _ => Ok(false),
    }
}

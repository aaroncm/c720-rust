extern crate rustc_serialize;
use rustc_serialize::json::Json;
use std::process::Command;
use std::path::Path;
use std::error::Error;
use std::fs;

fn main() {
    let path = &Path::new(".");
    for entry in fs::read_dir(path).unwrap() {
        let path = entry.unwrap().path();
        let filename = path.to_str().unwrap();
        match is_over_720p(&filename) {
            Ok(true) => println!("{}", filename),
            _ => (),
        }
    }
}

fn is_over_720p(filename: &str) -> Result<bool, Box<Error>> {
    let args = &["-v", "quiet", "-of", "json", "-show_streams", filename];
    let output = try!(Command::new("ffprobe").args(args).output());
    let stdout = try!(std::str::from_utf8(&output.stdout));
    if let Ok(probe_json) = Json::from_str(stdout) {
        if let Some(&Json::Array(ref streams)) = probe_json.find("streams") {
            for stream in streams {
                if let Some(&Json::U64(height)) = stream.find("height") {
                    return Ok(height > 700);
                }
            }
        }
    }
    Ok(false)
}

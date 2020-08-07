use regex::Regex;
use std::fs::{read_dir, File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::vec::Vec;

pub fn touch(path: &Path) -> io::Result<()> {
    match OpenOptions::new().create(true).write(true).open(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn echo(s: &str, path: &Path) -> io::Result<()> {
    let mut f = File::create(path)?;

    f.write_all(s.as_bytes())
}

pub fn cat(path: &Path) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

pub fn ignore_case_regex() -> Regex {
    Regex::new(r"^(assets|\.+)").unwrap()
}

pub fn list() -> io::Result<Vec<String>> {
    Ok(read_dir("content")?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.file_type().ok()?.is_dir() {
                let file_name = entry.file_name().to_string_lossy().into_owned();

                if !ignore_case_regex().is_match(&file_name) {
                    Some(file_name)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect())
}

#[macro_export]
macro_rules! abs_path {
    ($x: expr) => {{
        let path = format!("content/{}", $x);
        Path::new(&path).to_owned()
    }};
}

#[macro_export]
macro_rules! md_path {
    ($x: expr) => {{
        let path = format!("content/{}/index.md", $x);
        Path::new(&path).to_owned()
    }};
}

#[macro_export]
macro_rules! html_path {
    ($x: expr) => {{
        let path = format!("content/{}/index.html", $x);
        Path::new(&path).to_owned()
    }};
}

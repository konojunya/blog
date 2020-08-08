use regex::Regex;
use std::fs::{read_dir, File, OpenOptions};
use std::io::prelude::*;
use std::io::{stdout, Write};
use std::path::Path;
use std::time::Duration;
use std::vec::Vec;
use std::{io, thread};

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

pub fn print_progress(slug: &str, progress: usize, complete: bool) {
    if progress == 0 {
        print!("\x1b[38;5;6mBuilding []: {}\r\x1b[m", slug);
    } else {
        let percent = progress / 5;
        let arrow = vec!["="; percent];
        print!(
            "\x1b[38;5;6mBuilding [{}>]: {}\r\x1b[m",
            arrow.join(""),
            slug
        );
        if complete {
            println!(
                "\x1b[38;5;2mFinished [====================>]: {}\r\x1b[m",
                slug
            );
        }
    }

    stdout().flush().unwrap();
    thread::sleep(Duration::from_millis(100));
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

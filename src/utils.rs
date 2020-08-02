use regex::Regex;
use std::fs::{read_dir, DirEntry, File, OpenOptions};
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
    Regex::new(r"/(\.|index.html|assets)").unwrap()
}

pub fn double_quote_regex() -> Regex {
    Regex::new(r#""(?P<slug>.*)""#).unwrap()
}

pub fn list() -> Vec<Result<DirEntry, io::Error>> {
    let list = read_dir("content");
    let paths = list.unwrap();

    let ps: Vec<_> = paths
        .filter(move |path| {
            let p = format!("{:?}", path.as_ref().unwrap().path());
            let is_dir = Path::new(&p).is_dir();

            !ignore_case_regex().is_match(&p) && !is_dir
        })
        .collect();

    ps
}

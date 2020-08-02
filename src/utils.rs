use crate::markdown;
use regex::Regex;
use std::fs::{create_dir_all, read_dir, File, OpenOptions, ReadDir};
use std::io;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use std::path::Path;

fn touch(path: &Path) -> io::Result<()> {
    match OpenOptions::new().create(true).write(true).open(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

fn echo(s: &str, path: &Path) -> io::Result<()> {
    let mut f = File::create(path)?;

    f.write_all(s.as_bytes())
}

fn cat(path: &Path) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

pub fn ignore_case_regex() -> Regex {
    Regex::new(r"/\.").unwrap()
}

pub fn double_quote_regex() -> Regex {
    Regex::new(r#""(?P<slug>.*)""#).unwrap()
}

pub fn list() -> io::Result<ReadDir> {
    read_dir("content")
}

pub fn create_markdown_file(slug: &str) -> io::Result<()> {
    let mut tmp = String::new();
    match cat(&Path::new("templates/basic.md")) {
        Err(why) => println!("{}", format!("Error: {:?}", why.kind())),
        Ok(s) => {
            tmp = s;
        }
    }

    let dname = format!("content/{}", slug);
    let dir_path = &Path::new(&dname);
    if !dir_path.exists() {
        if let Err(why) = create_dir_all(dname) {
            return Err(Error::new(
                ErrorKind::AlreadyExists,
                format!("{}", format!("Error: {:?}", why.kind())),
            ));
        }
    }

    let fname = format!("content/{}/index.md", slug);
    let new_filepath = &Path::new(&fname);

    if new_filepath.exists() {
        return Err(Error::new(
            ErrorKind::AlreadyExists,
            format!("{} is already exists", slug),
        ));
    }

    if let Err(why) = touch(new_filepath) {
        println!("{}", format!("Error: {:?}", why.kind()))
    }

    echo(&tmp, new_filepath)
}

pub fn build_all() {
    let list = list();

    match list {
        Err(why) => println!("Error: {:?}", why.kind()),
        Ok(paths) => {
            for path in paths {
                let p = format!("{:?}", path.unwrap().path());
                if !ignore_case_regex().is_match(&p) {
                    let slug = double_quote_regex().replace_all(&p, "$slug");
                    build_specific(&slug);
                }
            }
        }
    }
}

pub fn build_specific(path: &str) {
    let p = format!("{}/index.md", path);
    let markdown_path = &Path::new(&p);

    match cat(markdown_path) {
        Err(why) => println!("Error: {:?}", why.kind()),
        Ok(s) => {
            let html = markdown::utils::md2html(markdown::utils::get_content(&s));

            let fname = format!("{}/index.html", path);
            let html_path = &Path::new(&fname);

            if !html_path.exists() {
                if let Err(why) = touch(html_path) {
                    println!("Error: {:?}", why.kind());
                }
            }

            if let Err(why) = echo(&html, html_path) {
                println!("Error: {:?}", why.kind());
            }
        }
    }
}

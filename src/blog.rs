use crate::{markdown, utils};
use handlebars::Handlebars;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind};
use std::path::Path;
use std::{fs, io};

#[derive(Serialize, Deserialize)]
pub struct Entry {
    pub slug: String,
    pub content: String,
    pub meta: markdown::utils::Meta,
}

impl Entry {
    pub fn new(slug: &str, content: &str, meta: markdown::utils::Meta) -> Entry {
        Entry {
            slug: slug.to_owned(),
            content: content.to_owned(),
            meta: meta,
        }
    }
}

pub fn create_markdown_file(slug: &str) -> io::Result<()> {
    let mut tmp = String::new();
    match utils::cat(&Path::new("templates/basic.md")) {
        Err(why) => println!("{}", format!("Error: {:?}", why.kind())),
        Ok(s) => {
            tmp = s;
        }
    }

    let dname = format!("content/{}", slug);
    let dir_path = &Path::new(&dname);
    if !dir_path.exists() {
        if let Err(why) = fs::create_dir_all(dname) {
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

    if let Err(why) = utils::touch(new_filepath) {
        println!("{}", format!("Error: {:?}", why.kind()))
    }

    utils::echo(&tmp, new_filepath)
}

pub fn build_all() {
    let paths = utils::list();

    for path in paths {
        let path = format!("{:?}", path.unwrap().path());
        let slug = utils::double_quote_regex().replace_all(&path, "$slug");

        build_specific(&slug);
    }
}

pub fn build_specific(path: &str) {
    let p = format!("{}/index.md", path);
    let markdown_path = &Path::new(&p);

    match utils::cat(markdown_path) {
        Err(why) => println!("Error: {:?}", why.kind()),
        Ok(s) => {
            let html = markdown::utils::md2html(markdown::utils::get_content(&s));
            let fname = format!("{}/index.html", path);
            let html_path = &Path::new(&fname);

            if !html_path.exists() {
                if let Err(why) = utils::touch(html_path) {
                    println!("Error: {:?}", why.kind());
                }
            }

            if let Err(why) = utils::echo(&html, html_path) {
                println!("Error: {:?}", why.kind());
            }
        }
    }
}

pub fn get_entries() -> Vec<Entry> {
    let paths = utils::list();
    let mut entries: Vec<Entry> = vec![];
    let re = Regex::new(r"content/").unwrap();

    for path in paths {
        let path = format!("{:?}", path.unwrap().path());
        let slug = utils::double_quote_regex().replace_all(&path, "$slug");
        let p = format!("{}/index.md", &slug);
        let markdown_path = Path::new(&p);
        let md = utils::cat(&markdown_path).unwrap_or("".to_owned());
        let content = markdown::utils::get_content(&md);
        let meta = markdown::utils::get_meta(&md, &markdown_path);
        let slug = re.replace_all(&slug, "");

        entries.push(Entry::new(&slug, &content, meta));
    }

    entries
}

pub fn build_top() {
    let template = Path::new("templates/index.html");

    match utils::cat(&template) {
        Err(why) => println!("{:?}", why.kind()),
        Ok(html) => {
            let entries = get_entries();
            let handlebars = Handlebars::new();

            match handlebars.render_template(&html, &entries) {
                Err(why) => println!("{:?}", why),
                Ok(html) => {
                    let top_path = Path::new("content/index.html");

                    if let Err(why) = utils::touch(&top_path) {
                        println!("{:?}", why.kind());
                    }

                    if let Err(why) = utils::echo(&html, &top_path) {
                        println!("{:?}", why.kind());
                    }
                }
            }
        }
    }
}

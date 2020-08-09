use crate::{abs_path, command, html_path, markdown, md_path, utils};
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind};
use std::path::Path;
use std::sync;
use std::{fs, io};
use threadpool::ThreadPool;

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    pub slug: String,
    pub content: String,
    pub meta: markdown::utils::Meta,
}

impl Entry {
    pub fn new(slug: &str) -> Entry {
        let md_path = md_path!(slug);
        let md = utils::cat(&md_path).unwrap_or("".to_owned());
        let content = markdown::utils::md2html(markdown::utils::get_content(&md));
        let meta = markdown::utils::Meta::new(slug);

        Entry {
            slug: slug.to_owned(),
            content: content.to_owned(),
            meta: meta,
        }
    }
}

pub fn create_markdown_file(slug: &str) -> io::Result<()> {
    let md = utils::cat(&Path::new("templates/basic.md"))?;
    let dir_path = abs_path!(slug);

    if !dir_path.exists() {
        fs::create_dir_all(&dir_path)?;
    }

    let new_filepath = &md_path!(&slug);

    if new_filepath.exists() {
        return Err(Error::new(
            ErrorKind::AlreadyExists,
            format!("{} is already exists", slug),
        ));
    }

    utils::touch(new_filepath)?;
    utils::echo(&md, new_filepath)
}

pub fn build_all(option: command::BuildOption) -> io::Result<()> {
    let slugs = utils::list()?;
    let pool = ThreadPool::new(4);
    let (tx, rx) = sync::mpsc::channel();
    let build_option = command::BuildOption { silent: true };

    for slug in slugs.to_owned() {
        let tx = tx.clone();
        pool.execute(move || {
            build_specific(slug.to_owned(), build_option).unwrap();
            tx.send(slug).unwrap();
        });
    }

    if !option.silent {
        let receives = rx.iter().take(slugs.len()).enumerate();
        for (i, slug) in receives {
            let per = (i + 1) as f32 / slugs.len() as f32;
            let progress = per * 100f32;
            utils::print_progress(&slug, progress.round(), false);
        }

        utils::print_progress("all blog entries", 100f32, true);
    }

    pool.join();

    Ok(())
}

pub fn build_specific(slug: String, option: command::BuildOption) -> io::Result<()> {
    if !option.silent {
        utils::print_progress(&slug, 0f32, false);
    }

    // initialize handlebars
    let template = Path::new("templates/post.html");
    let header_template = Path::new("templates/header.html");
    let mut handlebars = Handlebars::new();
    let html_path = html_path!(&slug);

    // progress pring
    if !option.silent {
        utils::print_progress(&slug, 25f32, false);
    }

    if let Err(why) =
        handlebars.register_partial("headerPartial", utils::cat(&header_template).unwrap())
    {
        println!("{:?}", why);
    }

    if !option.silent {
        utils::print_progress(&slug, 50f32, false);
    }

    if !html_path.exists() {
        utils::touch(&html_path)?;
    }

    if !option.silent {
        utils::print_progress(&slug, 75f32, false);
    }

    let entry = Entry::new(&slug);
    let template = handlebars
        .render_template(&utils::cat(&template).unwrap(), &entry)
        .unwrap();

    let result = utils::echo(&template, &html_path);

    if !option.silent {
        utils::print_progress(&slug, 100f32, true);
    }

    result
}

pub fn get_entries() -> io::Result<Vec<Entry>> {
    let slugs = utils::list()?;
    let mut entries: Vec<Entry> = vec![];

    for slug in slugs {
        entries.push(Entry::new(&slug));
    }

    Ok(entries)
}

pub fn build_top() -> io::Result<()> {
    let template = Path::new("templates/index.html");
    let header_template = Path::new("templates/header.html");
    let html = utils::cat(&template)?;
    let entries = get_entries()?;
    let mut handlebars = Handlebars::new();
    let header_template = utils::cat(&header_template)?;

    if let Err(why) = handlebars.register_partial("headerPartial", &header_template) {
        println!("{:?}", why);
    }

    match handlebars.render_template(&html, &entries) {
        Err(why) => {
            println!("{:?}", why);
            Ok(())
        }
        Ok(html) => {
            let top_path = Path::new("content/index.html");
            utils::touch(&top_path)?;
            utils::echo(&html, &top_path)?;

            Ok(())
        }
    }
}

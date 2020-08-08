use crate::{blog, utils};
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::io;
use std::sync::mpsc::channel;
use std::time::Duration;

#[derive(Copy, Clone)]
pub struct BuildOption {
    pub silent: bool,
}

pub fn create_template(slug: &str) -> io::Result<()> {
    blog::create_markdown_file(slug)
}

pub fn build(slug: Option<&str>, option: BuildOption) -> io::Result<()> {
    match slug {
        Some(s) => {
            blog::build_specific(s.to_owned(), option)?;
        }
        None => {
            blog::build_all(option)?;
        }
    }

    blog::build_top()
}

pub fn list() -> io::Result<()> {
    let slugs = utils::list()?;

    for slug in slugs {
        println!("{}", slug);
    }

    Ok(())
}

pub fn watch() -> io::Result<()> {
    let slugs = utils::list()?;
    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_millis(100)).unwrap();
    let option = BuildOption { silent: false };

    for slug in slugs {
        watcher
            .watch(
                format!("content/{}/index.md", slug),
                RecursiveMode::NonRecursive,
            )
            .unwrap();
    }

    loop {
        match rx.recv() {
            Ok(_) => {
                blog::build_all(option)?;
            }
            Err(e) => {
                println!("watch error: {:?}", e);
            }
        }
    }
}

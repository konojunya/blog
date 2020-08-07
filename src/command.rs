use crate::{blog, utils};
use std::io;

pub fn create_template(slug: &str) -> io::Result<()> {
    blog::create_markdown_file(slug)
}

pub fn build(slug: Option<&str>) -> io::Result<()> {
    match slug {
        Some(s) => {
            blog::build_specific(&s)?;
        }
        None => {
            blog::build_all()?;
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

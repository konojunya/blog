use crate::{blog, utils};
use regex::Regex;

pub fn create_template(slug: &str) {
    if let Err(why) = blog::create_markdown_file(slug) {
        println!("{:?}", why.kind());
    }
}

pub fn build(slug: Option<&str>) {
    match slug {
        Some(slug) => {
            blog::build_specific(&slug);
        }
        None => {
            blog::build_all();
        }
    }

    blog::build_top();
}

pub fn list() {
    let paths = utils::list();
    let only_slug = Regex::new(r"content/").unwrap();

    for path in paths {
        let p = format!("{:?}", path.unwrap().path());
        let o = only_slug.replace_all(&p, "");
        let slug = utils::double_quote_regex().replace_all(&o, "$slug");
        println!("{}", slug);
    }
}

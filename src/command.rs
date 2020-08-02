use crate::utils;
use regex::Regex;

pub fn create_template(slug: &str) {
    if let Err(why) = utils::create_markdown_file(slug) {
        println!("{:?}", why.kind());
    }
}

pub fn build(slug: Option<&str>) {
    match slug {
        Some(s) => {
            let path = format!("content/{}", s);
            utils::build_specific(&path);
        }
        None => utils::build_all(),
    }
}

pub fn list() {
    let list = utils::list();
    let only_slug = Regex::new(r"content/").unwrap();

    match list {
        Err(why) => println!("Error: {:?}", why.kind()),
        Ok(paths) => {
            for path in paths {
                let p = format!("{:?}", path.unwrap().path());
                if !utils::ignore_case_regex().is_match(&p) {
                    let o = only_slug.replace_all(&p, "");
                    let slug = utils::double_quote_regex().replace_all(&o, "$slug");
                    println!("{}", slug);
                }
            }
        }
    }
}

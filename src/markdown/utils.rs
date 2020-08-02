use pulldown_cmark::{html, Parser};
use regex::Regex;

pub struct Meta {
    pub titie: String,
    pub excerpt: String,
    pub created_at: i64,
    pub update_at: i64,
    pub tags: Vec<String>,
}

pub fn get_meta(md: &str) -> Meta {
    let sp: Vec<&str> = md.splitn(3, "---").collect();
    let meta_str = sp[1];

    // title
    let title_regex = Regex::new(r"title: (?P<title>.*)").unwrap();
    let title_cap = title_regex.captures(&meta_str).unwrap();
    let title = title_cap.name("title").unwrap();

    //  excerpt
    let excerpt_regex = Regex::new(r"excerpt: (?P<excerpt>.*)").unwrap();
    let excerpt_cap = excerpt_regex.captures(&meta_str).unwrap();
    let excerpt = excerpt_cap.name("excerpt").unwrap();

    // created_at
    // updated_at

    // tags
    let tags_regex = Regex::new(r"- (.*)").unwrap();
    let mut tags = Vec::new();
    for tag in tags_regex.captures_iter(&meta_str) {
        tags.push(format!("{}", &tag[1]));
    }

    Meta {
        titie: title.as_str().to_owned(),
        excerpt: excerpt.as_str().to_owned(),
        created_at: 0,
        update_at: 0,
        tags: tags,
    }
}

pub fn get_content(md: &str) -> &str {
    let sp: Vec<&str> = md.splitn(3, "---").collect();
    sp[2]
}

pub fn md2html(md: &str) -> String {
    let parser = Parser::new(md);
    let mut html = String::new();
    html::push_html(&mut html, parser);

    let html = html;
    html
}

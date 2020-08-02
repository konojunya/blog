use chrono::{DateTime, Local, TimeZone};
use pulldown_cmark::{html, Parser};
use regex::Regex;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde::Deserialize;
use std::path::Path;
use std::time::UNIX_EPOCH;

#[derive(Deserialize)]
pub struct Meta {
    pub title: String,
    pub excerpt: String,
    pub tags: Vec<String>,
    pub created_at: u64,
    pub update_at: u64,
}

impl Meta {
    pub fn new(
        title: String,
        excerpt: String,
        tags: Vec<String>,
        created_at: u64,
        update_at: u64,
    ) -> Meta {
        Meta {
            title: title,
            excerpt: excerpt,
            tags: tags,
            created_at: created_at,
            update_at: update_at,
        }
    }
}

impl Serialize for Meta {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Entry", 5)?;
        s.serialize_field("title", &self.title)?;
        s.serialize_field("excerpt", &self.excerpt)?;
        s.serialize_field("tags", &self.tags)?;

        let created: DateTime<Local> = Local.timestamp(self.created_at as i64, 0);
        s.serialize_field("created_at", &created.format("%Y-%m-%d").to_string())?;

        let updated: DateTime<Local> = Local.timestamp(self.update_at as i64, 0);
        s.serialize_field("update_at", &updated.format("%Y-%m-%d").to_string())?;
        s.end()
    }
}

pub fn get_meta(md: &str, path: &Path) -> Meta {
    let sp: Vec<&str> = md.splitn(3, "---").collect();
    let meta_str = sp[1];

    // title
    let re = Regex::new(r"title: (?P<title>.*)").unwrap();
    let cap = re.captures(&meta_str).unwrap();
    let title = cap.name("title").unwrap();

    //  excerpt
    let re = Regex::new(r"excerpt: (?P<excerpt>.*)").unwrap();
    let cap = re.captures(&meta_str).unwrap();
    let excerpt = cap.name("excerpt").unwrap();

    // tags
    let re = Regex::new(r"tags: (?P<tags>.*)").unwrap();
    let cap = re.captures(&meta_str).unwrap();
    let tags: Vec<String> = cap
        .name("tags")
        .unwrap()
        .as_str()
        .split(",")
        .map(|s| s.to_owned())
        .collect();

    if let Err(why) = path.metadata() {
        println!("Error: {:?}", why.kind());
    }

    let metadata = path.metadata().unwrap();

    Meta::new(
        title.as_str().to_owned(),
        excerpt.as_str().to_owned(),
        tags,
        metadata
            .created()
            .unwrap()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        metadata
            .modified()
            .unwrap()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    )
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_content() {
        let md = r#"
---
title: title
excerpt: excerpt
tags: tag1,tag2
---

# title"#;
        assert_eq!(
            get_content(&md),
            r#"

# title"#
        );
    }

    #[test]
    fn test_md2html() {
        let md = r#"
---
title: title
excerpt: excerpt
tags: tag1,tag2
---

# title
"#;
        assert_eq!(
            md2html(get_content(&md)),
            r#"<h1>title</h1>
"#
        );
    }
}

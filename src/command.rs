use crate::utils;

pub fn create_template(slug: &str) {
    match utils::create_markdown_file(slug) {
        Ok(_) => println!("content/{}.md was created!", slug),
        Err(why) => println!("{:?}", why.kind()),
    }
}

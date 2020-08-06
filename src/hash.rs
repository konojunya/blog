use crate::utils;
use crypto::digest::Digest;
use crypto::sha1::Sha1;
use std::io::Error;
use std::path::Path;

pub struct Commit {
    slug: String,
    bytes: usize,
    content: String,
}

impl Commit {
    pub fn new(slug: &str) -> Result<Commit, Error> {
        let path = format!("content/{}/index.md", slug);
        let path = Path::new(&path);
        let content = utils::cat(&path)?;
        let bytes = content.len();

        Ok(Commit {
            slug: slug.to_owned(),
            bytes: bytes,
            content: content.to_string(),
        })
    }

    pub fn hash(&self) -> String {
        let commit = format!("{} {:?}\0{}", self.slug, self.bytes, self.content);
        sha1(&commit)
    }
}

pub fn sha1(commit: &str) -> String {
    let mut hash = Sha1::new();
    hash.input_str(&commit);
    hash.result_str()
}

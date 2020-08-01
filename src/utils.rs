use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::path::Path;

fn touch(path: &Path) -> io::Result<()> {
    match OpenOptions::new().create(true).write(true).open(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

fn echo(s: &str, path: &Path) -> io::Result<()> {
    let mut f = File::create(path)?;

    f.write_all(s.as_bytes())
}

fn cat(path: &Path) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

pub fn create_markdown_file(filename: &str) -> io::Result<()> {
    // content dirの中にtemplate dirの中のbasic.mdをコピーした内容でfilenameをつけてcreate fileする
    // もし同じ名前のfileが存在したら already existsとかいってエラーを返す

    let mut tmp = String::new();
    match cat(&Path::new("templates/basic.md")) {
        Err(why) => println!("{}", format!("Error: {:?}", why.kind())),
        Ok(s) => {
            tmp = s;
        }
    }

    let fname = format!("content/{}.md", filename);
    let new_filepath = &Path::new(&fname);

    if let Err(why) = touch(new_filepath) {
        println!("{}", format!("Error: {:?}", why.kind()))
    }

    echo(&tmp, new_filepath)
}

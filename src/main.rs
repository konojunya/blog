#[macro_use]
extern crate clap;
extern crate chrono;
extern crate handlebars;
extern crate pulldown_cmark;
extern crate regex;
extern crate serde;

mod blog;
mod cli;
mod command;
mod markdown;
mod utils;

fn main() {
    let matches = cli::build_cli().get_matches();

    // create command
    if let Some(ref matches) = matches.subcommand_matches("create") {
        let slug = matches.value_of("slug").unwrap();

        if let Err(why) = command::create_template(slug) {
            println!("{:?}", why.kind());
        }
    }

    // build command
    if let Some(ref matches) = matches.subcommand_matches("build") {
        if let Err(why) = command::build(matches.value_of("slug")) {
            println!("{:?}", why.kind());
        }
    }

    // list command
    if let Some(_) = matches.subcommand_matches("list") {
        if let Err(why) = command::list() {
            println!("{:?}", why.kind());
        }
    }
}

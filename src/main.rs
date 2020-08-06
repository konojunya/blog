#[macro_use]
extern crate clap;
extern crate chrono;
extern crate crypto;
extern crate handlebars;
extern crate pulldown_cmark;
extern crate regex;
extern crate serde;

mod blog;
mod cli;
mod command;
mod hash;
mod markdown;
mod utils;

fn main() {
    let matches = cli::build_cli().get_matches();

    // create command
    if let Some(ref matches) = matches.subcommand_matches("create") {
        let slug = matches.value_of("slug").unwrap();
        command::create_template(slug);
    }

    // build command
    if let Some(ref matches) = matches.subcommand_matches("build") {
        command::build(matches.value_of("slug"));
    }

    // list command
    if let Some(_) = matches.subcommand_matches("list") {
        command::list();
    }
}

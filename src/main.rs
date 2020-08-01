#[macro_use]
extern crate clap;
extern crate pulldown_cmark;
extern crate regex;

mod cli;
mod command;
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

#[macro_use]
extern crate clap;
extern crate chrono;
extern crate handlebars;
extern crate pulldown_cmark;
extern crate regex;
extern crate serde;
extern crate threadpool;

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
        let build_option = command::BuildOption {
            silent: matches.is_present("silent"),
        };

        if matches.is_present("watch") {
            if let Err(why) = command::watch() {
                println!("{:?}", why.kind());
            }
        }

        if let Err(why) = command::build(matches.value_of("slug"), build_option) {
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

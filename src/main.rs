#[macro_use]
extern crate clap;

mod cli;
mod command;
mod utils;

fn main() {
    let matches = cli::build_cli().get_matches();

    // create command
    if let Some(ref matches) = matches.subcommand_matches("create") {
        let slug = matches.value_of("slug").expect("hoge");
        command::create_template(slug);
    }
}

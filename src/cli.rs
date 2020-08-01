use clap::{App, Arg, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(
            SubCommand::with_name("create")
                .about("create blog template")
                .arg(
                    Arg::with_name("slug").help(
                        "blog create <slug> will create [slug].html in the contents directory",
                    ),
                ),
        )
}

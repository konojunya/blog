use clap::{App, AppSettings, Arg, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::DeriveDisplayOrder)
        .subcommand(
            SubCommand::with_name("create")
                .about("create blog template")
                .arg(
                    Arg::with_name("slug")
                        .help(
                            "blog create [slug] will create [slug].html in the contents directory",
                        )
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("build")
                .about("build all or specific markdown to html")
                .arg(
                    Arg::with_name("slug").help(
                        "blog build [slug] will build specific markdown in content directory.",
                    ),
                )
                .arg(
                    Arg::with_name("silent")
                        .short("s")
                        .long("silent")
                        .help("the log at build will not be output."),
                )
                .arg(
                    Arg::with_name("watch")
                        .short("w")
                        .long("watch")
                        .help("watch file changes."),
                ),
        )
        .subcommand(SubCommand::with_name("list").about("list of contents"))
}

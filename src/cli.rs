use clap::{App, Arg};

pub fn build_cli() -> App<'static, 'static> {
    App::new("lxc-run")
        .version("0.1")
        .author(clap::crate_authors!("\n"))
        .about("Run LXC containers")
        .arg(
            Arg::with_name("name")
                .index(1)
                .help("Name of the container")
                .required(true),
        )
        .arg(
            Arg::with_name("path")
                .short("p")
                .long("path")
                .help("Path of the container")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("logfile")
                .long("logfile")
                .help("Logfile for the container")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("loglevel")
                .long("loglevel")
                .help("Loglevel for the container")
                .takes_value(true)
                .required(false),
        )
}

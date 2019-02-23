use std::process::exit;

use failure::*;

mod cli;
mod lxc;
mod util;

fn cmd_start(args: &clap::ArgMatches) -> Result<(), Error> {
    let sname = args.value_of("name").unwrap();
    let spath = args.value_of("path").unwrap();

    let container = lxc::Lxc::new(sname, spath)?;

    if !container.may_control() {
        bail!("Insufficient permissions");
    }

    if container.is_running() {
        bail!("Container already running");
    }

    container.start(false)
}

fn cmd_stop(args: &clap::ArgMatches) -> Result<(), Error> {
    let sname = args.value_of("name").unwrap();
    let spath = args.value_of("path").unwrap();
    let force = args.is_present("force");
    let timeout = match args.value_of("timeout").unwrap_or("-1").parse::<i32>()
    {
        Ok(n) => n,
        Err(n) => bail!("Invalid timeout: {:?}", n),
    };

    let container = lxc::Lxc::new(sname, spath)?;

    if !container.may_control() {
        bail!("Insufficient permissions");
    }

    if !container.is_running() {
        bail!("Container not running");
    }

    if force {
        return container.stop();
    }

    return container.shutdown(timeout);
}

fn cmd_exec(args: &clap::ArgMatches) -> i32 {
    let sname = args.value_of("name").unwrap();
    let spath = args.value_of("path").unwrap();
    let vals: Vec<&str> = args.values_of("command").unwrap().collect();

    let container = match lxc::Lxc::new(sname, spath) {
        Ok(c) => c,
        Err(_) => return 1,
    };

    if !container.may_control() {
        eprintln!("Insufficient permissions");
        return 1;
    }

    if !container.is_running() {
        eprintln!("Container not running");
        return 1;
    }

    container.attach_run_wait(vals[0], vals)
}

fn cmd_list(args: &clap::ArgMatches) -> Result<(), Error> {
    lxc::list_all_containers(args.value_of("path").unwrap())
}

fn do_cmd(
    args: &clap::ArgMatches,
    func: fn(args: &clap::ArgMatches) -> Result<(), Error>,
) {
    if let Err(err) = func(args) {
        eprintln!("error: {}", err);
        exit(1);
    }
}

fn main() {
    let matches = cli::build_cli().get_matches();

    if let Some(args) = matches.subcommand_matches("start") {
        do_cmd(args, cmd_start);
    } else if let Some(args) = matches.subcommand_matches("stop") {
        do_cmd(args, cmd_stop);
    } else if let Some(args) = matches.subcommand_matches("list") {
        do_cmd(args, cmd_list);
    } else if matches.subcommand_matches("version").is_some() {
        let version = lxc::get_version();
        println!("driver_version: {}", version);
    } else if let Some(exec) = matches.subcommand_matches("exec") {
        exit(cmd_exec(exec));
    } else {
        println!("{}", matches.usage())
    }
}

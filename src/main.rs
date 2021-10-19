use algebra::hop;
use clap::{App, Arg};
use models::{Link, LinkPair, HomeType};
use prod::prod_models::Prod;

mod algebra;
mod models;
mod prod;
mod program;

fn main() {

    const APPVERSION: &str = env!("CARGO_PKG_VERSION");

    let app = App::new("Hop")
        .version(APPVERSION)
        .author("Sanj Sahayam")
        .about("Hop to frequently used directories")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("c")
                .value_name("HOP_HOME")
                .help("Absolute path to the hop home directory. Defaults to ~/.hop if not specified")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("list")
                .short("l")
                .long("list")
                .help("Lists hoppable directories"),
        )
        .arg(
            Arg::with_name("table")
                .short("t")
                .long("table")
                .help("tabulate hoppable directories"),
        )
        .arg(
            Arg::with_name("jump")
                .short("j")
                .long("jump")
                .value_name("NAME")
                .help("Jump to a named directory")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("mark")
                .short("m")
                .long("mark")
                .value_names(&["NAME", "PATH"])
                .help("Mark a named directory")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("delete")
                .short("d")
                .long("delete")
                .value_name("NAME")
                .help("Delete a named directory")
                .takes_value(true),
        );

    let mut app2 = app.clone(); //we need this close to display usage on error
    let matches = app.get_matches();

    let hop_home =
        matches
        .value_of("config")
        .map(|hd| HomeType::Absolute(hd.to_string()))
        .unwrap_or_else(|| HomeType::Relative(".hop".to_string()));

    println!("hop_home: {}", &hop_home);

    let hop_program = hop::HopProgram {
        value: Prod,
        hop_home_dir: hop_home,
    };

    if matches.is_present("list") {
        program::handle_list(&hop_program)
    } else if matches.is_present("table") {
        program::handle_table(&hop_program)
    } else if let Some(jump_target) = matches.value_of("jump") {
        program::handle_jump(&hop_program, jump_target)
    } else if let Some(m) = matches.values_of("mark") {
        let mut values = m.clone();
        let link = values.next().expect("expected link name");
        let target = values.next().expect("expected target value");

        program::handle_mark(&hop_program, &LinkPair::new(link, target))
    } else if let Some(d) = matches.value_of("delete") {
        program::handle_delete(&hop_program, &Link(d.to_string()))
    } else {
        let _result = app2.print_help();
        println!();
    };
}

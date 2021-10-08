use std::error::Error;
use clap::{App, Arg};
use models::{Link, LinkPair, HopEffect};
use std::io;
use algebra::hop;
use prod::prod_models::Prod;

mod models;
mod algebra;
mod prod;

fn main() -> Result<(), Box<dyn Error>>{
    let app = App::new("Hop")
        .version("0.1.0")
        .author("Sanj Sahayam")
        .about("Hop to frequently used directories")
        .arg(
            Arg::with_name("list")
                .short("l")
                .long("list")
                .help("Lists hoppable directories")
        )
        .arg(
            Arg::with_name("jump")
                .short("j")
                .long("jump")
                .value_name("NAME")
                .help("Jump to a named directory")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("mark")
                .short("m")
                .long("mark")
                .value_names(&["NAME", "PATH"])
                .help("Mark a named directory")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("delete")
                .short("d")
                .long("delete")
                .value_name("NAME")
                .help("Delete a named directory")
                .takes_value(true)
        );

     let mut app2 = app.clone(); //we need this close to display usage on error
     let matches = app.get_matches();

    let hop_program = hop::HopProgram { value: Prod, cfg_dir: ".hop".to_string() };

    let program =
        if matches.is_present("list") {
            handle_list(&hop_program)
        } else if let Some(jump_target) = matches.value_of("jump") {
            handle_jump(&hop_program, jump_target)
        } else if let Some(m) = matches.values_of("mark") {
            let mut values = m.clone();
            let link = values.next().expect("expected link name");
            let target = values.next().expect("expected target value");

            handle_mark(&hop_program, &LinkPair::new(link, target))
        } else if let Some(d) = matches.value_of("delete") {
            handle_delete(&hop_program, &Link(d.to_string()))
        } else {
            let _result = app2.print_help();
            println!();
        };

    Ok(program)
}

fn handle_list(hop_program: &hop::HopProgram<Prod>) {
    let action = hop_program.list_links();
    on_error(action, "Could not retrieve list of links")
}

fn handle_jump(hop_program: &hop::HopProgram<Prod>, jump_target: &str) {
    let action = hop_program.jump_target(Link::new(jump_target));
    on_error(action, &format!("Could not retrieve jump target: {}", jump_target))
}

fn handle_mark(hop_program: &hop::HopProgram<Prod>, link_pair: &LinkPair) {
    let action = hop_program.mark_dir(link_pair);
    on_error(action, &format!("Could not mark directory: {}", link_pair))
}

fn handle_delete(hop_program: &hop::HopProgram<Prod>, link: &Link) {
    let action = hop_program.delete_link(link);
    on_error(action, &format!("Could not delete link: {}", link))
}

fn on_error<T>(effect: HopEffect<T>, message: &str) {
    match effect {
        Ok(_) => (),
        Err(e) => eprintln!("{}\nError: {}", message, e)
    }
}

//TODO: move this somewhere else
fn io_error(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::Other, message.clone())
}

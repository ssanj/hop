use super::*;

use std::io;
use ansi_term::Color::{Red, Yellow};

pub fn handle_list(hop_program: &hop::HopProgram<Prod>) {
    let action = hop_program.list_links();
    match action {
        Ok(entries) =>
            entries
            .iter()
            .for_each(|lp| println!("{}", lp.link)),
        Err(e) => handle_error(e, "Could not retrieve list of links")

    }
}

pub fn handle_table(hop_program: &hop::HopProgram<Prod>) {
    let action = hop_program.tabulate_links();
    match action {
        Ok(entries) =>
            entries
            .iter()
            .for_each(|lp| println!("{} {} {}", lp.link, Yellow.paint("->") ,lp.target)),
        Err(e) => handle_error(e, "Could not retrieve list of links"),
    }
}

pub fn handle_jump(hop_program: &hop::HopProgram<Prod>, jump_target: &str) {
    let action = hop_program.jump_target(Link::new(jump_target));
    handle_action(
        action,
        &format!("Could not retrieve jump target: {}", jump_target),
    )
}

pub fn handle_mark(hop_program: &hop::HopProgram<Prod>, link_pair: &LinkPair) {
    let action = hop_program.mark_dir(link_pair);
    handle_action(action, &format!("Could not mark directory: {}", link_pair))
}

pub fn handle_delete(hop_program: &hop::HopProgram<Prod>, link: &Link) {
    let action = hop_program.delete_link(link);
    handle_action(action, &format!("Could not delete link: {}", link))
}

pub fn io_error(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::Other, message)
}

fn handle_action<T>(effect: HopEffect<T>, message: &str) {
    match effect {
        Ok(_) => (),
        Err(e) => {
            println!("{}",Yellow.paint(format!("{}", message)));
            eprintln!("{}",Red.paint(format!("Error: {}", e)))
        }
    }
}

fn handle_error(error: io::Error, message: &str) {
    println!("{}",Yellow.paint(format!("{}", message)));
    eprintln!("{}",Red.paint(format!("Error: {}", error)))
}

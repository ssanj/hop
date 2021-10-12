use super::*;

use std::io;
use ansi_term::Color::{Red, Yellow};

pub fn handle_list(hop_program: &hop::HopProgram<Prod>) {
    let action = hop_program.list_links();
    on_error(action, "Could not retrieve list of links")
}

pub fn handle_table(hop_program: &hop::HopProgram<Prod>) {
    let action = hop_program.tabulate_links();
    on_error(action, "Could not retrieve list of links")
}

pub fn handle_jump(hop_program: &hop::HopProgram<Prod>, jump_target: &str) {
    let action = hop_program.jump_target(Link::new(jump_target));
    on_error(
        action,
        &format!("Could not retrieve jump target: {}", jump_target),
    )
}

pub fn handle_mark(hop_program: &hop::HopProgram<Prod>, link_pair: &LinkPair) {
    let action = hop_program.mark_dir(link_pair);
    on_error(action, &format!("Could not mark directory: {}", link_pair))
}

pub fn handle_delete(hop_program: &hop::HopProgram<Prod>, link: &Link) {
    let action = hop_program.delete_link(link);
    on_error(action, &format!("Could not delete link: {}", link))
}

pub fn io_error(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::Other, message)
}

fn on_error<T>(effect: HopEffect<T>, message: &str) {
    match effect {
        Ok(_) => (),
        Err(e) => {
            println!("{}",Yellow.paint(format!("{}", message)));
            eprintln!("{}",Red.paint(format!("Error: {}", e)))
        }
    }
}

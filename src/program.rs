use crate::algebra::hop::DeleteStatus;

use super::*;

use ansi_term::Color::{Red, Yellow};
use std::io;

pub fn handle_list(hop_program: &hop::HopProgram<Prod>) {
    let action = hop_program.list_links();

    fn handler(lp:&LinkPair) {
        println!("{}", lp.link)
    }

    handle_links(action, handler)
}

pub fn handle_table(hop_program: &hop::HopProgram<Prod>) {
    let action = hop_program.tabulate_links();

    fn handler(lp:&LinkPair) {
        println!("{} {} {}", lp.link, Yellow.paint("->"), lp.target)
    }

    handle_links(action, handler)
}

fn handle_links(action: io::Result<Vec<LinkPair>>, handler: fn(&LinkPair)) {
    match action {
        Ok(entries) => {
            if entries.is_empty() {
                println!("No entries to list.\nPlease create some entries with {}\nPlease use {} for more information", Yellow.paint("hop -m <link> <path>"), Yellow.paint("hop -h"))
            } else {
                entries.iter().for_each(handler)
            }

        },
        Err(e) => handle_error(e, "Could not retrieve list of links"),
    }
}

pub fn handle_jump(hop_program: &hop::HopProgram<Prod>, jump_target: &str) {
    let action = hop_program.jump_target(Link::new(jump_target));
    match action {
        Ok(link) => println!("{}", link),
        Err(e) => handle_error(
            e,
            &format!("Could not retrieve jump target: {}", jump_target),
        ),
    }
}

pub fn handle_mark(hop_program: &hop::HopProgram<Prod>, link_pair: &LinkPair) {
    let action = hop_program.mark_dir(link_pair);
    match action {
        Ok(target) => println!(
            "Created link from {} {} {}",
            link_pair.link,
            Yellow.paint("->"),
            target
        ),
        Err(e) => handle_error(e, &format!("Could not mark directory: {}", link_pair)),
    }
}

pub fn handle_delete(hop_program: &hop::HopProgram<Prod>, link: &Link) {
    let action = hop_program.delete_link(link);
    match action {
        Ok(DeleteStatus::DeleteAborted) => println!("Aborting delete of {}", link),
        Ok(DeleteStatus::DeleteSucceeded(pair)) => {
            println!(
                "Removed link {} {} {}",
                link,
                Yellow.paint("->"),
                pair.target
            )
        }
        Err(e) => handle_error(e, &format!("Could not delete link: {}", link)),
    }
}

pub fn io_error(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::Other, message)
}

pub fn io_error_ex(message: &str, e: io::Error) -> io::Error {
    io_error(&format!("{}\n{}", message, e.to_string()))
}

/// Creates an error with both the `current_error` and the `original_error` that
/// cause the current error.
pub fn io_error_ex_nested(message: &str, current_error: io::Error, original_error: io::Error) -> io::Error {
    io_error(&format!("{}\n{}\n{}", message, current_error.to_string(), original_error.to_string()))
}

fn handle_error(error: io::Error, message: &str) {
    println!("{}", Yellow.paint(message));
    eprintln!("{}", Red.paint(format!("Error: {}", error)))
}

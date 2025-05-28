#![windows_subsystem = "windows"]    // prevents quick cmd window popup on run
use std::process;

mod website;
mod bot;

fn main() {

    if let Some(links) = website::run().ok() {
        if links.is_empty(){
            eprintln!("No new articles found: nothing will be posted");
            process::exit(0)
        } else {
            let _ = bot::run(links);
        }
    }
}


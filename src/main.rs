use std::io::{self, BufRead};
use std::env;

fn print_usage() {
    println!("{}", "Usage : cut_utf16_head num");
}

fn main() {
    let mut skip = 0;
    for arg in env::args() {
        if arg == "-h" || arg == "--help" {
            print_usage();
            return;
        }
        if skip == 0 {
            match arg.parse::<usize>() {
                Ok(n) => skip = n,
                Err(_e) => skip = 0,
            }
        }
    }

    let stdin = io::stdin();
    for ln in stdin.lock().lines() {
        if let Ok(line) = ln {

            let count = line.chars().count();
            let mut chars = line.chars();

            if skip >= count {
                println!("{}", "");
                continue;
            }

            let mut i = 0;
            while i < skip {
                chars.next();
                i = i + 1;
            }
            println!("{}", chars.as_str());
        }
    }
}

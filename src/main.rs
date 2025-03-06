use std::io::{self, BufRead, Write};
use std::env;
use std::process;

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
    let mut stdout = io::stdout();
    for ln in stdin.lock().lines() {
        if let Ok(line) = ln {

            let count = line.chars().count();
            let mut chars = line.chars();

            if skip >= count {
                if let Err(_) = writeln!(stdout) {
                    process::exit(1);
                }
                continue;
            }

            let mut i = 0;
            while i < skip {
                chars.next();
                i = i + 1;
            }
            if let Err(_) = writeln!(stdout, "{}", chars.as_str()) {
                process::exit(1);
            }
        }
    }
}

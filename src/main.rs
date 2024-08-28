use std::env::args;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process::exit;

#[derive(Default)]
struct GrepOptions {
    case_insensitive: bool,
    show_line_numbers: bool,
    invert_match: bool,
}

fn main() {
    let args = args().collect::<Vec<String>>();
    match run(args) {
        Ok(()) => exit(0),
        Err(err) => {
            eprintln!("{}: {}", "Problem encountered", err);
            exit(1);
        }
    };
}

fn run(args: Vec<String>) -> Result<(), String> {
    if args.len() < 2 {
        return Err("Usage: grep [options] <pattern> [file_name]".to_string());
    }

    let mut options = GrepOptions::default();
    let mut pattern = String::new();
    let mut file_name: Option<String> = None;

    let mut iter = args[1..].iter();
    while let Some(arg) = iter.next() {
        if arg.starts_with("-") {
            for flag in arg[1..].chars() {
                match flag {
                    'i' => options.case_insensitive = true,
                    'n' => options.show_line_numbers = true,
                    'v' => options.invert_match = true,
                    _ => return Err(format!("Unknown option: -{}", flag))?,
                }
            }
        } else if pattern.is_empty() {
            pattern = arg.to_string();
        } else if file_name.is_none() {
            file_name = Some(arg.to_string());
        } else {
            return Err(
                "Too many argument given: Usage: grep [options] <pattern> [file_name]".to_string(),
            );
        }
    }

    if pattern.is_empty() {
        return Err("Pattern is required".to_string());
    }

    match file_name {
        Some(file_name) => {
            let file = File::open(file_name).map_err(|err| format!("Filename error: {}", err))?;
            let reader = BufReader::new(file);
            grep(reader, &pattern, &options);
        }
        None => {
            let stdin = io::stdin();
            let reader = stdin.lock();
            grep(reader, &pattern, &options);
        }
    }

    Ok(())
}

fn grep<R: BufRead>(reader: R, pattern: &str, options: &GrepOptions) {
    let mut line_number = 0;
    for line in reader.lines() {
        let line = line.expect("Could not read line");
        line_number += 1;

        let matched = if options.case_insensitive {
            line.to_lowercase().contains(&pattern.to_lowercase())
        } else {
            line.contains(pattern)
        };

        let should_print = if options.invert_match {
            !matched
        } else {
            matched
        };

        if should_print {
            if options.show_line_numbers {
                println!("{}: {}", line_number, line);
            } else {
                println!("{}", line);
            }
        }
    }
}

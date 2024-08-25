use std::{io, usize};

struct CliArgs {
    file_path: std::path::PathBuf,
    pattern: String,
}

fn main() -> io::Result<()> {
    let mut args = std::env::args();
    let cli_args = parse_args(&mut args).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    let buffer = std::fs::read_to_string(&cli_args.file_path)?;

    for line in buffer.lines() {
        if let Some(_) = grep_line(line, &cli_args.pattern) {
            println!("{}", line)
        };
    }
    Ok(())
}

fn parse_args(args: &mut std::env::Args) -> Result<CliArgs, String> {
    let pattern = args
        .nth(1)
        .ok_or_else(|| "Pattern argument missing".to_string())?;
    let file_path = args
        .next()
        .ok_or_else(|| "File path argument missing".to_string())?;

    if args.next().is_some() {
        let error_message = format!("Expected 2 args, {:?} received", 2 + args.count());
        Err(error_message)
    } else {
        Ok(CliArgs {
            pattern,
            file_path: std::path::PathBuf::from(file_path),
        })
    }
}

fn grep_line(s: &str, pattern: &String) -> Option<usize> {
    s.find(pattern)
}

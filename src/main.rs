use std::io::{self, BufRead};

struct CliArgs {
    file_path: std::path::PathBuf,
    pattern: String,
}

fn main() -> std::io::Result<()> {
    let mut args = std::env::args();
    let cli_args = parse_args(&mut args).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    let file = std::fs::File::open(cli_args.file_path)?;
    let mut reader = io::BufReader::new(file);
    let mut buffer = String::new();

    while 0 != reader.read_line(&mut buffer)? {
        if grep_rs::check_pattern(&buffer, &cli_args.pattern).is_some() {
            print!("{}", buffer);
        };
        buffer.clear();
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

use std::env;
use std::path::Path;

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let mut args = env::args();
    let binary = args.next().unwrap_or_else(|| "json-to-cli".to_string());
    let path = match args.next() {
        Some(value) => value,
        None => return Err(format!("usage: {binary} <path-to-json-file>")),
    };

    if args.next().is_some() {
        return Err("expected a single file path argument".to_string());
    }

    let path = Path::new(&path);
    if !path.exists() {
        return Err(format!("file not found: {}", path.display()));
    }

    Ok(())
}

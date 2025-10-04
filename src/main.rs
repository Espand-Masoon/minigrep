use std::env;
use std::fs;
use std::process;
use std::io;
use minigrep::search;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    if let Err(error) = run(config) {
        eprintln!("{}", error);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), io::Error> {
    // Reading the file content
    let file_content = fs::read_to_string(config.file_path)?;
    let result_lines = search(config.query, &file_content, config.case_sensitive);
    for line in result_lines {
        println!("{line}");
    }
    Ok(())
}

struct Config<'a> {
    query: &'a str,
    file_path: &'a str,
    case_sensitive: bool,
}

impl<'a> Config<'a> {
    fn build(args: &'a Vec<String>) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("The command needs at least 2 arguments!");
        }

        // Check environment variables
        let mut case_sensitive: bool = true;
        if let Ok(case_sensitive_env) = env::var("CASE_SENSITIVE") {
            match case_sensitive_env.to_lowercase().as_str() {
                "false" | "0" => case_sensitive = false,
                _ => {},
            }
        }

        // Check command options
        let mut args_iter = args.iter().skip(3);
        while let Some(argument) = args_iter.next() {
            match argument.as_str() {
                "--case-insensitive" | "-ci" => {case_sensitive = false},
                _ => {},
            }
        }

        let query = &args[1];
        let file_path = &args[2];
        Ok(Self {
            query,
            file_path,
            case_sensitive,
        })
    }
}


use std::env;
use std::fs;
use std::process;
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });

    if let Err(error) = run(config) {
        println!("{}", error);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), io::Error> {
    println!("We're looking for {}", config.query);
    // Reading the file content
    let file_content = fs::read_to_string(config.file_path)?;
    println!("In this file: {}", config.file_path);
    println!("The file content is:\n{file_content}");
    Ok(())
}

struct Config<'a> {
    query: &'a str,
    file_path: &'a str,
}

impl<'a> Config<'a> {
    fn build(args: &'a Vec<String>) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("The command needs 2 arguments!");
        }

        let query = &args[1];
        let file_path = &args[2];
        Ok(Self {
            query,
            file_path,
        })
    }
}


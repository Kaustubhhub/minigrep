use std::{env, error::Error, fs, process};

use minigrep::{search, search_case_insensitive};
fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);

    let config_data = Config::build(&args).unwrap_or_else(|err| {
        println!("Error parsing arguements : {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config_data) {
        println!("Application error : {}", e);
        process::exit(1);
    };
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;

    let result = if config.ignore_case {
        search_case_insensitive(&config.query, &content)
    } else {
        search(&config.query, &content)
    };

    for line in result {
        println!("{line}");
    }

    Ok(())
}

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        // println!("{:?}", args);
        if args.len() < 3 {
            return Err("usage : cargo run -- <query> <file_path> <ignore_case>");
        }

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
            ignore_case,
        })
    }
}

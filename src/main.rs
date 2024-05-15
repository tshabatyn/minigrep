use std::env;

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();

    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents = std::fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            eprintln!("Not enough arguments");
            std::process::exit(1);
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}

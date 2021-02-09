mod lib;



fn main() {
    let args: Vec<String> = std::env::args().collect();

    let config = lib::Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    println!("Searching for: {}, in file: {}", config.query, config.filename);

    if let Err(e) = lib::run(config) {
        println!("Application error: {}", e);

        std::process::exit(1);
    }
}
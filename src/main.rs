use std::env;

mod indexer;
mod searcher;

enum CMD {
    INDEX,
    SEARCH,
    HELP,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let command: CMD = match args.get(1).map(|s| s.as_str()) {
        Some("-i") | Some("--index") => CMD::INDEX,
        Some("-s") | Some("--search") => CMD::SEARCH,
        _ => CMD::HELP,
    };

    match command {
        CMD::INDEX => {
            if let Some(path) = args.get(2) {
                indexer::index(path);
            } else {
                println!("Error: --index requires a path.");
                print_help();
            }
        }
        CMD::SEARCH => {
            if let Some(query) = args.get(2) {
                searcher::search(query);
            } else {
                println!("Error: --search requires a query.");
                print_help();
            }
        }
        CMD::HELP => {
            print_help();
        }
    }
}

fn print_help() {
    println!(
        "Semantics Usage:\n\t./semantics --index <path-to-wiki-sentences.txt> \n\t./semantics --search <query>"
    );
}

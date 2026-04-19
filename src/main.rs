use std::env;

mod core;
mod indexer;
mod searcher;

enum CMD {
    INDEX,
    SEARCH,
    HELP,
}

const DEFAULT_TOP_K: usize = 5;

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
                let mut top_k: u64 = DEFAULT_TOP_K as u64;
                if let Some(index_of_top) =
                    args.iter().position(|item| item == "--top" || item == "-t")
                {
                    if let Some(value_string) = args.get(index_of_top + 1) {
                        if let Ok(parsed_number) = value_string.parse::<u64>() {
                            top_k = parsed_number;
                        } else {
                            println!(
                                "Warning: Invalid number provided for --top. Falling back to default: 5"
                            );
                        }
                    }
                }

                searcher::search(query, top_k);
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

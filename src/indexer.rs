use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    time::Instant,
};

pub fn index(path: &str) {
    let path = Path::new(path);
    if !path.exists() || !path.is_file() {
        panic!("Error: Invalid input source - {:?}", path.as_os_str());
    }
    println!("Indexing path: {:?}", path.as_os_str());

    let start = Instant::now();
    let file: File = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            panic!("Error opening file: {}", e);
        }
    };
    let reader: BufReader<File> = BufReader::new(file);
    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                panic!("Error reading line: {}", e);
            }
        };
        println!("{}\n", line);

        // todo!(index line in the vector database)
        // step 1: generate vector embeddings of the line
        // step 2: save the embeddings and payload in the qudrant database
        // handle cases when data has been already indexed...
        // don't need to do it again, don't need to generate the embeddings
    }
    println!("Time elapsed while reading file: {:?}", start.elapsed());
}

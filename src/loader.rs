use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn read_fragments_fasta(file_path: &str) -> Vec<String> {
    let mut sequences = Vec::<String>::new();
    let mut current_sequence = String::new();

    let file = File::open(file_path).expect("Could not open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let line = line.trim();
                if line.starts_with('>') {
                    if !current_sequence.is_empty() {
                        sequences.push(current_sequence.clone());
                        current_sequence.clear();
                    }
                } else {
                    current_sequence.push_str(line);
                }
            },
            Err(e) => panic!("Error reading line: {}", e),
        }
    }

    // Add the last sequence
    if !current_sequence.is_empty() {
        sequences.push(current_sequence);
    }

    sequences
}
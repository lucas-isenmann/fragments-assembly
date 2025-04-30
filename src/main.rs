mod loader;
mod assembly_graph;
mod solver;
mod fragments_stats;

use fragments_stats::print_fragments_stats;
use loader::{read_fragments_fasta, read_fragments_fastq};
use crate::solver::solver;
use std::fs;
use std::borrow::Cow;

fn main() {

    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("solve") => {

            let m = args.get(3)
            .map(|s| s.parse::<usize>().unwrap_or(100))
            .unwrap_or(100);
            
            let threshold = args.get(4)
            .map(|s| s.parse::<usize>().unwrap_or(50))
            .unwrap_or(50);

            let fragments = read_fragments_fastq(&args[2]);
            let fragments: Vec<&[u8]> = fragments.iter().map(|s| s.as_bytes()).collect();

            print_fragments_stats(&fragments);
            let result = solver(&fragments, m, threshold);
            let result_str = format!("{}", String::from_utf8_lossy(&result));

            let output_file = format!("{}.randPath.fasta", args[2]);
            fs::write(&output_file, result_str)
                .expect("Unable to write result to file");
            println!("Result written to {}", output_file);
        }
        _ => println!("Usage: program solve <fasta_file>"),
    }
}

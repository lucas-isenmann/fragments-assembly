mod loader;
mod assembly_graph;
mod solver;

use loader::read_fragments_fasta;
use crate::solver::solver;


fn main() {

    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("solve") => {
            let fragments = read_fragments_fasta(&args[2]);
            let fragments: Vec<&[u8]> = fragments.iter().map(|s| s.as_bytes()).collect();
            let result = solver(&fragments, 1, 0);
            println!("{}", String::from_utf8_lossy(&result));
        }
        _ => println!("Usage: program solve <fasta_file>"),
    }
}

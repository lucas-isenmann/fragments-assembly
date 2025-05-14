
use crate::{assembly_graph::build_common_prefix_suffix_matrix, loader::{read_fragments_fasta, read_fragments_fastq}};
use std::{cell::Ref, collections::HashMap, fs::File, io::{self, BufRead, BufReader}};
use crossterm::{
    cursor::{MoveTo, MoveToColumn}, event::{self, DisableMouseCapture, EnableMouseCapture, MouseButton, MouseEventKind}, execute, style::{Color, SetBackgroundColor}, terminal::{disable_raw_mode, enable_raw_mode, Clear, EnterAlternateScreen, LeaveAlternateScreen, SetSize}
};


// Coverage 


// @m0/14708/CCS Read=0;length=14708bp;startpos=24434;chromosome=>random_reference_with_len_100000_and_0.6_GC;numberOfErrors=0;totalErrorProb=0.0000;passes=1.3502902862699715;passesLeft=1;passesRight=2;cutPosition=9556


pub fn read_simlord_fragments_fastq(file_path: &str) -> Vec<(String, usize, usize)> {
    let mut fragments = Vec::new();

    let file = File::open(file_path).expect("Could not open file");
    let reader = BufReader::new(file);

    let mut i = 0;
    let mut length: usize = 0;
    let mut start_position: usize = 0;
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let line = line.trim();
                if i % 4 == 0 {
                    for part in line.split(';') {
                        if let Some(eq_pos) = part.find('=') {
                            let (key, value) = part.split_at(eq_pos);
                            match key {
                                "length" => {
                                    if let Some(bp_pos) = value.find("bp") {
                                        if let Ok(len) = value[1..bp_pos].parse::<usize>() {
                                            length = len;
                                        }
                                    }
                                },
                                "startpos" => {
                                    if let Ok(pos) = value[1..].parse::<usize>() {
                                        start_position = pos;
                                    }
                                },
                                _ => (), // Ignore other fields
                            }
                        }
                    }
                }
                else if i % 4 == 1 {
                    if !line.starts_with("A") && !line.starts_with("C") && !line.starts_with("G") && !line.starts_with("T"){
                        panic!("Error reading line: {line}");
                    }
                    fragments.push((line.to_string(), length, start_position));
                }
                i += 1;
            },
            Err(e) => panic!("Error reading line: {}", e),
        }
    }
    fragments
}



pub fn coverage(dna_fasta: &str, fragments_fastq: &str) {
    let reference_sequence = read_fragments_fasta(dna_fasta);
    let reference_sequence = &reference_sequence[0];
    let fragment_sequence = read_simlord_fragments_fastq(fragments_fastq);
    
    let n = reference_sequence.len();
    println!("Reference length: {n}");
    println!("Fragments length: {}", fragment_sequence.len());

    let mut coverage_stats = vec![0;n];

    for i in 0..fragment_sequence.len(){
        let (fragment, length, start_position) = &fragment_sequence[i];
        for k in *start_position..(*start_position + length){
            coverage_stats[k] += 1;
        }
    }

    let mut uncovered_intervals = vec![];
    for i in 0..n {
        if coverage_stats[i] == 0 {
            if uncovered_intervals.len() == 0 {
                uncovered_intervals.push((i,i));
            } else {
                let nn = uncovered_intervals.len();
                if uncovered_intervals.last().unwrap().1 == i-1 {
                    let a = uncovered_intervals.last().unwrap().0;
                    uncovered_intervals[nn-1] = (a,i);
                } else {
                    uncovered_intervals.push((i,i));
                }
            }
        }
    }

    println!("{uncovered_intervals:?}");
    
    let min_coverage = coverage_stats.iter().min().cloned().unwrap_or(0);
    let max_coverage = coverage_stats.iter().max().cloned().unwrap_or(0);
    let mean_coverage = coverage_stats.iter().sum::<usize>() as f64 /
                       coverage_stats.len() as f64;
    
    println!("\nCoverage Statistics:");
    println!("Minimum coverage: {}", min_coverage);
    println!("Mean coverage: {:.2}", mean_coverage);
    println!("Maximum coverage: {}", max_coverage);
}










pub fn print_fragments_stats(fragments: &[&[u8]]){
    // Convert to vector of lengths for easier manipulation
    let lengths: Vec<usize> = fragments.iter().map(|frag| frag.len()).collect();
    
    // Calculate minimum length
    let min_length = lengths.iter().min().unwrap();
    
    // Calculate maximum length
    let max_length = lengths.iter().max().unwrap();
    
    // Calculate mean length
    let mean_length = lengths.iter().sum::<usize>() as f64 / lengths.len() as f64;
    
    // Calculate median length
    let mut sorted_lengths = lengths.clone();
    sorted_lengths.sort_unstable();
    let mid = sorted_lengths.len() / 2;
    let median_length = if sorted_lengths.len() % 2 == 0 {
        (sorted_lengths[mid - 1] as f64 + sorted_lengths[mid] as f64) / 2.0
    } else {
        sorted_lengths[mid] as f64
    };

    println!("Number of fragments: {}", fragments.len());
    println!("Minimum length: {}", min_length);
    println!("Maximum length: {}", max_length);
    println!("Mean length: {:.2}", mean_length);
    println!("Median length: {:.2}", median_length);

}


macro_rules! printr {
    ($stdout:expr, $fmt:expr) => {{
        println!($fmt);
        execute!($stdout, MoveToColumn(0)).unwrap();
    }};
    ($stdout:expr, $fmt:expr, $($arg:tt)*) => {{
        println!($fmt, $($arg)*);
        execute!($stdout, MoveToColumn(0)).unwrap();
    }};
}


fn truncate_fragment(fragment: &[u8], chars: usize) -> String {
    String::from_utf8_lossy(&fragment[..std::cmp::min(chars, fragment.len())])
        .into_owned()
}


pub fn explore_fragments(fragments: &[&[u8]]){
    let n = fragments.len();
    let ag = build_common_prefix_suffix_matrix(fragments);

    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
    // execute!(stdout, (SetBackgroundColor(Color::Rgb {
    //     r: 211,
    //     g: 211,
    //     b: 211,
    // }))).unwrap();
    // execute!(stdout, SetSize(60, 60)).unwrap();

    let mut current_index = 0;

    loop {
        // Clear screen and show current fragment
        execute!(stdout, crossterm::terminal::Clear(crossterm::terminal::ClearType::All)).unwrap();
        execute!(stdout, MoveTo(0, 0)).unwrap();

        let current_fragment = fragments.get(current_index).ok_or("No fragments available").unwrap();
        printr!(stdout, "=====  Fragment {}/{}: Length: {}", current_index, n,  current_fragment.len());
        // printr!(stdout, "{}...", String::from_utf8_lossy(&current_fragment[..std::cmp::min(20, current_fragment.len())]));

        printr!(stdout, "{}...{}", 
            truncate_fragment(current_fragment, 20),
            truncate_fragment(&current_fragment[current_fragment.len().saturating_sub(20)..], 20)
        );



        // Top left neighbors
        let mut indices: Vec<_> = (0..n).filter(|&i| i != current_index).collect();
        indices.sort_by(|&a, &b| ag[current_index][b].partial_cmp(&ag[current_index][a]).unwrap());
        let top_in_indices = &indices[..std::cmp::min(10, indices.len())];

        printr!(stdout, "\nTop {} left neighbors:", top_in_indices.len());
        printr!(stdout, "Index\t Overlap\t end");
        for &idx in top_in_indices {
            let fragment_end = truncate_fragment(&fragments[idx][fragments[idx].len().saturating_sub(20)..], 20);
            printr!(stdout, "{}\t {}\t ...{}", idx , ag[current_index][idx], fragment_end);
        }

        // Top out-neighbor
        let mut indices: Vec<_> = (0..n).filter(|&i| i != current_index).collect();
        indices.sort_by(|&a, &b| ag[b][current_index].partial_cmp(&ag[a][current_index]).unwrap());
        let top_out_indices = &indices[..std::cmp::min(10, indices.len())];

        printr!(stdout, "\nTop {} right neighbors:", top_out_indices.len());
        for &idx in top_out_indices {
            let fragment_end = truncate_fragment(&fragments[idx], 20);
            printr!(stdout, "{}\t {}\t ...{}", idx, ag[idx][current_index], fragment_end);
        }



        // Show navigation help
        printr!(stdout, "\nNavigation:");
        printr!(stdout, "→ Right arrow: Next fragment    ← Left arrow: Previous fragment");
        printr!(stdout, "q: Quit");

        // Read event
        match event::read().unwrap() {
            event::Event::Key(key) => match key.code {
                event::KeyCode::Char('q') => break,
                event::KeyCode::Right => {
                    current_index = (current_index + 1) % fragments.len();
                }
                event::KeyCode::Left => {
                    current_index = (current_index - 1 + fragments.len()) % fragments.len();
                }
                _ => {}
            },
            event::Event::Mouse(mouse) => match mouse.kind {
                MouseEventKind::Down(_) => {

                    if mouse.row >= 5 && (mouse.row as usize) < 5+top_in_indices.len() {
                        current_index = top_in_indices[(mouse.row as usize)-5];
                    }

                    if mouse.row >= 17 && (mouse.row as usize) < 17+top_out_indices.len() {
                        current_index = top_out_indices[(mouse.row as usize)-17];
                    }

                }
                _ => {}
            }
            _ => {}
        }
    }

    // Cleanup
    disable_raw_mode().unwrap();
    execute!(
        stdout,
        LeaveAlternateScreen,
        DisableMouseCapture
    ).unwrap();

    
}
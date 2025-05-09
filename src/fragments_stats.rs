
use crate::assembly_graph::build_common_prefix_suffix_matrix;
use std::io;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture}, execute, style::{Color, SetBackgroundColor}, terminal::{disable_raw_mode, enable_raw_mode, Clear, EnterAlternateScreen, LeaveAlternateScreen, SetSize}
};

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




pub fn explore_fragments(fragments: &[&[u8]]){
    let n = fragments.len();
    let ag = build_common_prefix_suffix_matrix(fragments);

    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, DisableMouseCapture).unwrap();
    execute!(stdout, (SetBackgroundColor(Color::Rgb {
        r: 211,
        g: 211,
        b: 211,
    }))).unwrap();
    execute!(stdout, SetSize(60, 60)).unwrap();

    let mut current_index = 0;

    loop {
        // Clear screen and show current fragment
        execute!(stdout, crossterm::terminal::Clear(crossterm::terminal::ClearType::All)).unwrap();
        

        let current_fragment = fragments.get(current_index).ok_or("No fragments available").unwrap();
        println!("==================");
        print!("Fragment {}/{n}:", current_index + 1);
        println!();
        print!("{}...", String::from_utf8_lossy(&current_fragment[..std::cmp::min(20, current_fragment.len())]));

        // Show navigation help
        println!("\nNavigation:");
        println!("→ Right arrow: Next fragment");
        println!("← Left arrow: Previous fragment");
        println!("q: Quit");

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
            _ => {}
        }
    }

    // Cleanup
    disable_raw_mode().unwrap();
    execute!(
        stdout,
        LeaveAlternateScreen
    ).unwrap();

    
}
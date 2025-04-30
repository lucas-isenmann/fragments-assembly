

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
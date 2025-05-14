use std::{self, cmp::min, collections::HashMap, fs::File, io::{stdout, Write}};

/// Finds the length of the common prefix-suffix between two strings
///
/// Args:
///     prefix: First string to compare
///     suffix: Second string to compare
///
/// Returns:
///     Number of common characters from start of prefix and end of suffix
pub fn longest_common_prefix_suffix(prefix: &[u8], suffix: &[u8]) -> usize {
    let m = prefix.len();
    let n = suffix.len();
    
    for k in (1..(min(m, n)+1)).rev() {
        let mut ok = true;
        for i in 0..k{
            if prefix[i] != suffix[n-k+i] {
                ok = false;
                break;
            }
        }
        if ok {
            return k;
        }
    }
    return 0;
}


/// Constructs a matrix where entry [i][j] represents the length of common
/// prefix-suffix between sequences[i] and sequences[j]
pub fn build_common_prefix_suffix_matrix(sequences: &[&[u8]]) -> Vec<Vec<usize>> {
    let n = sequences.len();
    let mut f = vec![vec![0; n]; n];

    // Create a HashMap to store weight frequencies
    let mut weight_distribution = HashMap::new();
    
    for i in 0..n {
        print!("\rCreate assembly graph: {i}/{n}");
        stdout().flush().unwrap();
        for j in 0..n {
            f[i][j] = longest_common_prefix_suffix(sequences[i], sequences[j]);
            *weight_distribution.entry(f[i][j]).or_insert(0) += 1;
        }
    }

    // Write distribution to file
    let mut file = match File::create("weight_distribution.txt") {
        Ok(file) => file,
        Err(e) => panic!("Could not create file: {}", e),
    };
    
    // Sort weights and write distribution
    let mut weights: Vec<_> = weight_distribution.keys().cloned().collect();
    weights.sort();
    
    for weight in weights {
        let count = weight_distribution[&weight];
        writeln!(file, "{} {}", weight, count).expect("Could not write to file");
    }

    println!();
    f
}






pub fn longest_noise_overlap(
    prefix: &[u8],
    suffix: &[u8],
    substitution_threshold: usize,
) -> (usize, usize) {
    let m = prefix.len();
    let n = suffix.len();
    
    for k in (1..=std::cmp::min(m, n)).rev() {
        let mut mismatches = 0;
        let mut ok = true;
        
        for i in 0..k {
            if prefix[i] != suffix[n-k+i] {
                mismatches += 1;
                if mismatches > substitution_threshold {
                    ok = false;
                    break;
                }
            }
        }
        
        if ok && mismatches <= substitution_threshold {
            return (k, mismatches);
        }
    }
    
    (0,0)
}





/// Constructs a matrix where entry [i][j] represents the length of common
/// prefix-suffix between sequences[i] and sequences[j]
pub fn build_noise_overlap_matrix(sequences: &[&[u8]], substitution_threshold: usize) -> Vec<Vec<usize>> {
    let nb_fragments = sequences.len();
    let mut f = vec![vec![0; nb_fragments]; nb_fragments];

    
    for i in 0..nb_fragments {
        println!("Create assembly graph: {i}/{nb_fragments}");
        // stdout().flush().unwrap();

        let prefix = sequences[i];
        let m = prefix.len();

        let mut noise = vec![0;m];
        let mut counter = vec![0;m];

        for j in 0..nb_fragments{
            if j == i {
                continue;
            }
            let suffix = sequences[j];
            let n = suffix.len();
            
            for k in (1..=std::cmp::min(m, n)).rev() {
                let mut mismatches = 0;
                let mut ok = true;
                
                for j in 0..k {
                    if prefix[j] != suffix[n-k+j] {
                        noise[j] += 1;
                        
                        mismatches += 1;
                        if mismatches > substitution_threshold {
                            for jj in 0..=j{
                                noise[jj] -= 1;
                            }
                            ok = false;
                            break;
                        }
                    }
                }
                
                if ok && mismatches <= substitution_threshold {
                    for j in 0..k{
                        counter[j] += 1;
                    }
                }
            }

        }

        // Fix noise
        for k in 0..m{
            if counter[k] >= 10 && noise[k]*100 > counter[k]*80 {
                println!("noise detected at position {k} in fragment {i} (value: {}): {} {}", prefix[k], noise[k], counter[k]);

            }
        }
        
        

    }

    
    f
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_prefix_suffix() {
        assert_eq!(longest_common_prefix_suffix("aabb".as_bytes(), "aaab".as_bytes()), 3);
        //    aabb
        //   aaab
        assert_eq!(longest_common_prefix_suffix("aaab".as_bytes(), "aabb".as_bytes()), 0);
        
    }

    #[test]
    fn test_matrix_build() {
        let sequences = vec!["aabb".as_bytes(), "aaab".as_bytes(), "baab".as_bytes()];
        let matrix = build_common_prefix_suffix_matrix(&sequences);
        
        println!("{matrix:?}")
    }
}
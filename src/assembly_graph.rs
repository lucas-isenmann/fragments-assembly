use std::{self, cmp::min};

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
    
    for i in 0..n {
        for j in 0..n {
            f[i][j] = longest_common_prefix_suffix(sequences[i], sequences[j]);
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
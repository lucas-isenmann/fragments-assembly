use std::result;
use rand::Rng;
use crate::{assembly_graph::build_common_prefix_suffix_matrix, loader::read_fragments_fasta};



fn weight(x: usize) -> usize {
    x
}


pub fn solver(fragments: &[&[u8]], m : usize,  threshold: usize) -> Vec<u8>{
    let mut rng = rand::rng();
    let ag = build_common_prefix_suffix_matrix(fragments);
    let n = fragments.len();

    println!("{n}");
  


    let x = rng.random_range(0..n);
    let mut result = Vec::from(fragments[x]);
    let mut visited = vec![false; n];
    let mut n_visited = 1;

    visited[x] = true;
    let mut start = x;
    let mut end = x;






    loop {


        let mut candidates = vec![];
        let mut sum = 0;

        for i in 0..n {
            if visited[i] == false {
                if ag[i][end] > threshold {
                    candidates.push((i, false, ag[i][end]));
                    sum += weight(ag[i][end]);
                }
                if ag[start][i] > threshold {
                    candidates.push((i, true, ag[start][i]));
                    sum += weight(ag[start][i]);
                }
            }
        }

        println!("{n_visited}/{} {} {}", visited.len(), candidates.len(), result.len());


        if sum == 0{
            return result
        }


        let mut r = rng.random_range(0..sum);

        for (i, t, w) in candidates {
            if r <= weight(w) {
                println!("{w}");
                if t == false {
                    result.extend_from_slice(&fragments[i][w..]);
                    end = i;
                } else {
                    result.splice(0..0, fragments[i][0..w].iter().cloned());
                    start = i;
                }
                visited[i] = true;
                n_visited += 1;
                break;
            } else {
                r -= weight(w);

            }
        }

    }

}
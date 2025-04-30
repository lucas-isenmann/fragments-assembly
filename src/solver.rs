use std::result;
use rand::Rng;
use crate::{assembly_graph::build_common_prefix_suffix_matrix, loader::read_fragments_fasta};



fn weight(x: usize) -> usize {
    x*x*x
}


pub fn solver(fragments: &[&[u8]], m : usize,  threshold: usize) -> Vec<u8>{
    let mut rng = rand::rng();
    let ag = build_common_prefix_suffix_matrix(fragments);
    let n = fragments.len();

    let mut best_contig = vec![];
    let mut best_ratio = 0.;
  
    for run in 0..m {

        let x = rng.random_range(0..n);
        let mut result = Vec::from(fragments[x]);
        let mut visited = vec![false; n];
        let mut n_visited = 1;

        visited[x] = true;
        let mut start = x;
        let mut end = x;
        let mut total_overlap = 0;

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

            print!("\rRun{run}/{m}: {n_visited}/{} {} {}", visited.len(), candidates.len(), result.len());

            if sum == 0{
                println!("total_overlap: {total_overlap} final_size: {}", result.len());
                let ratio = (total_overlap as f64)/ (result.len() as f64);
                println!("ratio {:.3}", ratio );
                if ratio > best_ratio{
                    best_ratio = ratio;
                    best_contig = result.clone();
                }
                break;
                 
            }


            let mut r = rng.random_range(0..sum);

            for (i, t, w) in candidates {
                if r <= weight(w) {
                    total_overlap += w;
                    // println!("choose {i} overlap: {w}");
                    if t == false {
                        result.extend_from_slice(&fragments[i][w..]);
                        end = i;
                    } else {
                        result.splice(0..0, fragments[i][0..(fragments[i].len()-w)].iter().cloned());
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

    return best_contig;


    

}
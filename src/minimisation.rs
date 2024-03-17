use crate::data_structures::WeightedTreeAutomatonMatrix;
use nalgebra::DMatrix;
use std::collections::VecDeque;
use log::{debug, info};
use crate::step_i::step_i_forward;
use crate::step_ii::step_ii_backward;
use crate::step_iii::step_iii_solve;

type DynamicMatrix = DMatrix<f64>;

// All singular values below eps are considered equal to 0 when used in numeric algorithms.
const EPS: f64 = 0.00001;

//pub fn minimize_automaton(automaton_matrix: WeightedTreeAutomatonMatrix) -> DynamicSizedMatrix {
pub fn minimize_automaton(
    automaton_matrix: WeightedTreeAutomatonMatrix,
) -> WeightedTreeAutomatonMatrix {
    info!("Minimizing:");
    let f = step_i_forward(&automaton_matrix);
    info!("F: {f}");
    let b = step_ii_backward(&automaton_matrix, &f);
    info!("B: {b}");
    let new_a = step_iii_solve(&automaton_matrix, &f, &b);
    return new_a;
}



pub fn tzeng(
    column: &mut DynamicMatrix,
    b: &mut DynamicMatrix,
    m: &Vec<DynamicMatrix>,
) { 
    let mut column_queue = VecDeque::new();   
    //let mut temp_b = b.clone();
    column_queue.push_back(column.clone());
    let mut filled_columns = 0;
    let max_rank = b.column_iter().count();
    while column_queue.len() != 0 {
        if b.rank(EPS) == max_rank {
            return;
        }

        debug!("column_queue.len() {}", column_queue.len());
        let temp_column = column_queue.pop_front().unwrap();

        let mut new_b = b.clone();
        for i in 0..column.len() {
            new_b[(filled_columns, i)] = temp_column[(i, 0)];
        }
        debug!("new_b {new_b}");
        let new_rank = new_b.rank(EPS);
        let old_rank = b.rank(EPS);        

        if new_rank > old_rank {
            debug!("independant");
            *b = new_b;
            for c in m {
                let new_column = c * &column.clone();
                column_queue.push_back(new_column);
            }
            filled_columns += 1;
        }
    }
    return;
}


// This function gives you the next tuple in a lexicographical order.
// See https://www.baeldung.com/cs/permutations-with-repetition for inspiration.
// x = previous tuple
// n is the pool from witch to draw the numbers inside the tuple: {1, ..., n}
// k = length of tuple, for ex. k = 4 -> [0, 0, 0, 0]
// force_n = bool whether we should force the return tuple to contain an instance of n.
pub fn next_tuple(mut x: Vec<usize>, n: usize, force_n: bool) -> Option<Vec<usize>> {
    let k = x.len();
    if k == 0 {
        return None;
    }
    let mut j = k - 1;
    while x[j] == n {
        if j == 0 {
            return None;
        }
        j = j - 1;
    }

    for l in j..k {
        if x[l] == n {
            x[l] = 1;
        } else {
            x[l] = x[l] + 1;
        }
    }
    // Here we check whether x contains an instance of n.
    if x.contains(&n) || force_n == false {
        return Some(x);
    } else {
        x = next_tuple(x.clone(), n, force_n).unwrap();
        return Some(x);
    }
}
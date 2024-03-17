use crate::{data_structures::WeightedTreeAutomatonMatrix, minimisation::next_tuple};
use nalgebra::DMatrix;
use log::debug;

type DynamicMatrix = DMatrix<f64>;

// All singular values below eps are considered equal to 0 when used in numeric algorithms.
const EPS: f64 = 0.00001;

pub fn step_i_forward(automaton_matrix: &WeightedTreeAutomatonMatrix) -> DynamicMatrix {
    let n = automaton_matrix.num_states;
    // Since we don't (yet) know how big the final matrix will be, we simply add rows each run.
    // There could be potential optimisations where I initialise a giant Matrix first and then just delete empty lines
    // at the end.
    let mut f = DynamicMatrix::from_element(n, n, 0.0);
    let mut i: usize = 0;
    let mut j: usize = 0;
    while i <= j {
        // forall σ ∈ Σ do:
        for σ in automaton_matrix.ranked_alphabet.clone().into_iter() {
            let rk_σ: &usize = automaton_matrix.ranked_alphabet.get(&σ.0).unwrap();
            // Note that for rank zero matrizes v is always = μ_σ. We make a simple if statement here to
            // make the code more readable.
            debug!("------------------");
            debug!("i = {i}, j = {j}");
            debug!("Case: {σ:?}, rk_σ = {rk_σ}");
            debug!("Current temp matrix {f}");

            // We don't need to deal with the final weight vector here.
            if σ.0 == "!" {
                continue;
            }
            // By definition j has to be smaller or equal to n so we break.
            if j >= n {
                break;
            }
            // rk_σ == 0 && j == 0 implies that there are no rows in F yet and we are currently looking at a row (Since all σ with rk_σ == 0 are by definition rows) so we just add the current one.
            if *rk_σ == 0 && j == 0 {
                let v = automaton_matrix.transition_matrices.get(&σ.0).unwrap();
                for index in 0..v.len() {
                    f[(0, index)] = v[(0, index)];
                }
                j += 1;
                debug!("New F: {f}");
            } 
            // Nothing happens here. Since rk_σ != 0, that means we need at least i = 1 so that there is a F_l.
            // Otherwise v does not have size 1xn and there is no way to check if v in the vector space since v isn't a vector but a matrix.
            if i == 0 && *rk_σ != 0 {
                continue;
            }

            let μ_σ = automaton_matrix.transition_matrices.get(&σ.0).unwrap();
            let mut x: Vec<usize> = vec![1; *rk_σ];
            // We start with x = [1, ..., 1], but we only want to use that tuple if we are in case i = 1, since otherwise
            // it doesn't contain at least one instance of i.
            // As a result we call next_tuple if i!=1 to get a tuple with at least one i.
            if i > 1 && x != [] {
                x = next_tuple(x, i, true).unwrap();
            }

            // Here we loop over all possible tuples which contain at least one instance of i.
            loop {
                let mut v = DMatrix::from_element(1, 1, 1.0);
                debug!("v beginning: {v}");
                for index in &x {
                    let f_index_vector = f.select_rows([index - 1].iter());
                    debug!("Kron Prod multiplying with: {f_index_vector}");
                    v = v.kronecker(&f_index_vector);
                }
                // Case rk_σ = 0 implies that v is still [1] (Since the for-loop above doesn't run since x = []), so v = v * μ_σ = μ_σ which is exactly what we want.
                v = v * μ_σ;
                debug!("Current μ_σ: {μ_σ} Current v: {v}");

                let mut new_f = f.clone();
                for index in 0..v.len() {
                    new_f[(j, index)] = v[(0, index)];
                }
                debug!("New matrix: {new_f}");

                if new_f.rank(EPS) > f.rank(EPS) {
                    debug!("New row was independent -> setting F = new_F, j+=1");
                    f = new_f;
                    j += 1;
                }
                debug!("Computing next tuple: x:{x:?} i:{i} k:{}", *rk_σ);
                if let Some(temp_x) = next_tuple(x, i,  true) {
                    x = temp_x;
                } else {
                    debug!("Finished tuple");
                    break;
                }
            }
        }
        i += 1;
    }
    return f;
}

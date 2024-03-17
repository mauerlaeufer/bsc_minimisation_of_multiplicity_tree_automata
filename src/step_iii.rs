use crate::data_structures::WeightedTreeAutomatonMatrix;
use nalgebra::DMatrix;
use std::collections::HashMap;
use log::debug;

type DynamicMatrix = DMatrix<f64>;

// All singular values below eps are considered equal to 0 when used in numeric algorithms.
const EPS: f64 = 0.00001;



pub fn step_iii_solve(
    automaton_matrix: &WeightedTreeAutomatonMatrix,
    f: &DynamicMatrix,
    b: &DynamicMatrix,
) -> WeightedTreeAutomatonMatrix {
    let mut new_transition_matrices: HashMap<String, DynamicMatrix> = HashMap::new();
    let mut new_f = DynamicMatrix::from_element(0, automaton_matrix.num_states, 0.0);
    // Ranks are saved in variables instead of being computed on the fly since there is no defined rank for [] in nalgebra.
    // This way we start by 0 and keep the for loop clean.
    let mut old_rank = 0;
    let mut current_row_num = 0;
    for row_num in 0..(f.row_iter().count()) {
        debug!("New F = {new_f} with row_num: {row_num}");
        let mut temp_new_f = new_f.clone().resize_vertically(current_row_num + 1, 0.0);
        let cur_row = f.clone().row(row_num)*b;
        for index in 0..automaton_matrix.num_states {
            temp_new_f[(current_row_num, index)] = cur_row[index];
        }
        let new_rank = temp_new_f.rank(EPS);
        // If new_rank is bigger than the previous one that implies that the added row is
        // linearly independent from the the other rows in new_f.
        // As such we save temp_new_f and edit the other values to reflect this new new_f.
        if new_rank > old_rank {
            old_rank = new_rank;
            current_row_num += 1;
            new_f = temp_new_f;
        }
    }
    debug!("B: {b}");
    debug!("F: {f}");
    debug!("Tilde F: {new_f}");

    // A new γ is easily computed: γ * new_F
    let new_gamma = &new_f * automaton_matrix.transition_matrices.get("!").unwrap();
    debug!("New Gamma: {new_gamma}");
    new_transition_matrices.insert("!".to_string(), new_gamma);


    // We only want to compute the k-fold-kronecker product once for each k, since it is quite an expensive operation.
    // Since we will need the u-fold-kronecker product with u being max(rk(σ)) anyways, we can just compute all preceding
    // k-fold-kronecker products, put them in a hashmap and then pick the onces we actually need later.
    let mut f_kron_k: HashMap<usize, DynamicMatrix> = HashMap::new();
    // The k-fold-kronecker product is defined as the identity matrix.
    let identity = DynamicMatrix::from_element(1, 1, 1.0);
    f_kron_k.insert(0, identity);
    // The 1-fold-kronecker product is defined as new_f.
    f_kron_k.insert(1, new_f.clone());
    // Compute the higest rk(σ).
    let max_rk_μ_σ = automaton_matrix.ranked_alphabet.clone().into_values().max().unwrap();
    // Fill the hashmap with all needed kronecker products.
    for cur_k in 2..(max_rk_μ_σ + 1) {
        let new_k_fold = f_kron_k.get(&(cur_k - 1)).unwrap().kronecker(&new_f);
        debug!("Cur_k: {cur_k} with cron: {new_k_fold}");
        f_kron_k.insert(cur_k, new_k_fold);
    }

    // Count the rows to get the new number of states!
    let new_n = (&new_f).row_iter().count();
    // Creating a decomposition of FB.
    let new_f_decomp = (&new_f).clone().transpose().svd(true, true);

    // Computing the new transition matrices for each symbol.
    for μ_σ in automaton_matrix.transition_matrices.iter() {
        // The final weight vector is dealt with at an other place.
        if μ_σ.0 == "!" {
            continue;
        } 
        let rk_σ = automaton_matrix.ranked_alphabet.get(μ_σ.0).unwrap();
        let old_μ_σ =
            automaton_matrix.transition_matrices.get(μ_σ.0).unwrap();
        debug!("rk_σ: {rk_σ} old_μ_σ: {old_μ_σ} b: {b}");
        let right_side = f_kron_k.get(rk_σ).unwrap() * old_μ_σ;
        debug!("Left side: {}", (&new_f).transpose());
        debug!("Right side Full: {right_side}");

        // Create a new, empty Matrix of the right dimensions for μ_σ.
        // The dimensions are known, since they are based on new_n and rk_σ.
        let mut new_μ_σ = DynamicMatrix::from_element(new_n.pow(*rk_σ as u32), new_n, 0.0);
        let mut current_row_index = 0;
        for row in right_side.row_iter() {
            debug!("Right side: {}", row.transpose());
            let new_row = new_f_decomp.solve(&row.transpose(), EPS).unwrap();
            debug!("The new row: {new_row}");
            for index in 0..new_n {
                debug!("Writing into {current_row_index}, {index}");
                new_μ_σ[(current_row_index, index)] = new_row[(index, 0)];
            }
            current_row_index += 1;
        }
        debug!("New version: {new_μ_σ}");
        new_transition_matrices.insert(μ_σ.0.clone(), new_μ_σ);
    }

    // The old ranked alphabet stays the same, only the final weight vector changes its rank, since the number of
    // states might have changed.
    let mut new_ranked_alphabet = automaton_matrix.ranked_alphabet.clone();
    new_ranked_alphabet.insert("!".to_owned(), new_n);

    
    let new_automaton = WeightedTreeAutomatonMatrix {
        num_states: new_n,
        grammar_type: automaton_matrix.grammar_type.clone(),
        ranked_alphabet: new_ranked_alphabet,
        transition_matrices: new_transition_matrices,
    };
    return new_automaton;
}
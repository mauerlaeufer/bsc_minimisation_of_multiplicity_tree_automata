use crate::{data_structures::WeightedTreeAutomatonMatrix, minimisation::next_tuple, minimisation::tzeng};
use nalgebra::DMatrix;
use log::debug;

type DynamicMatrix = DMatrix<f64>;

pub fn step_ii_backward(
    automaton_matrix: &WeightedTreeAutomatonMatrix,
    f: &DynamicMatrix,
) -> DynamicMatrix {
    // Array in which to save Matricies M
    let mut m: Vec<DynamicMatrix> = Vec::new();

    // We will use the identity matrix a couple of times in step II, so it makes sense to just create it once and then use references to this
    // instance later. 
    let identity = DMatrix::<f64>::identity(automaton_matrix.num_states, automaton_matrix.num_states);

    let f_index_vector = f.row_iter();

    // F probably has a couple of rows which consits only of zeros - this is a result of our way of computing it.
    // This part of the code cleans up F, that way we minimize the amount of computation we have to do later.
    let mut f_clean = DynamicMatrix::from_element(0, automaton_matrix.num_states,0.0,);
    let mut current_row_num = 0;
    for f in f_index_vector {
        if f.amax() != 0.0 {
            f_clean = f_clean.clone().resize_vertically(current_row_num + 1, 0.0);

            for i in 0..f.column_iter().count() {
                f_clean[(current_row_num, i)] = f[(0, i)];
            }
            current_row_num += 1;
        }
    }

    let f = &f_clean;

    // Number of trees t for which μ(t) is a row in f
    // In other words the number of rows in f.
    // This value is used each time a new tuple is generated, so we compute it once at the start.
    let t_num = f.row_iter().count();


    for σ in automaton_matrix.ranked_alphabet.clone().into_iter() {
        let rk_σ: &usize = automaton_matrix.ranked_alphabet.get(&σ.0).unwrap();
        // If rk_σ == 0, this means that the element isn't part of C^1_Σ,S so it gets skipped.
        // We also deal with the final weight vector later, so we skip it here.
        if *rk_σ == 0 || σ.0 == "!" {
            continue;
        }

        // Get the transition matrix for σ.
        let μ_σ = automaton_matrix.transition_matrices.get(&σ.0).unwrap();
        // Since the kronecker product isn't commutative we need to compute all possible positions of the identity matrix (square).
        // Its current position is saved in this variable. 

        for identity_position in 0..*rk_σ {
            debug!("σ: {} - rk_σ: {rk_σ} - identity_position: {identity_position}", σ.0);
            debug!("Computing (t_1 ⊗ identity ⊗ rk_σ) * μ_σ where the position of identity can also be 1 or rk_σ and rk_σ is >= 1");
            // x is a vector which contains the current combination of elements c ∈ C^1_Σ,S that we want to use to compute the current m.
            // In each loop the vector is changed and another permutation is chosen until all permutations are done. See the next_tuple() function.
            let mut x: Vec<usize> = vec![0; *rk_σ];
            // This loop iterates over all possible permutations for this specific identity position and breaks afterwards.
            loop {
                let mut cur_position = 0;
                // New element is initialised as a [1] matrix. That way we can compute new_element.kronecker(val) without changing val, which makes the loop clean.
                let mut new_element_m = DMatrix::from_element(1, 1, 1.0);
                
                debug!("x: {x:?}");
                for index in &x {
                    let f_index_vector = f.select_rows([*index].iter());
                    debug!("Vec: {}", f_index_vector);
                    if f_index_vector.amax() == 0.0 {
                        cur_position = 0;
                        break;
                    }
                    debug!("Cur Index: {} - cur_position: {cur_position} - identity_position: {identity_position}", *index);
                    if cur_position == identity_position {
                        new_element_m = new_element_m.kronecker(&identity);
                        debug!("Kroenecker with identity");
                    } else {
                        debug!("Kroenecker with: {f_index_vector}");
                        new_element_m = new_element_m.kronecker(&f_index_vector);
                    }
                    debug!("Done");

                    cur_position += 1;
                }
                debug!("cur {cur_position} new_element_m {new_element_m}");
                if cur_position != 0{
                    debug!("μ_σ: {μ_σ}");
                    new_element_m = new_element_m * μ_σ;
                    debug!("New element for M: {new_element_m}");
                        if new_element_m.amax() != 0.0 {
                        if m.contains(&new_element_m) == false && (new_element_m != identity){
                            m.push(new_element_m.clone());
                        }
                    }
                }
                

                // Check if there are still possible tuples to compute a new element of M for.
                if let Some(temp_x) = next_tuple(x, t_num - 1,  false) {
                    x = temp_x;
                } else {
                    debug!("Finished tuple");
                    break;
                }
            }
        }
    }
    debug!("M with len: {}", m.len());
    for element in &m {
        debug!("{element} ∈ M");
    }
    let mut γ = automaton_matrix.transition_matrices.get("!").unwrap().clone();
    let mut b = DynamicMatrix::from_element(automaton_matrix.num_states,automaton_matrix.num_states,0.0,);
    // γ ∈ V by definition of V.
    tzeng(
        &mut γ,
        &mut b,
        &m,
    );
    return b;
}


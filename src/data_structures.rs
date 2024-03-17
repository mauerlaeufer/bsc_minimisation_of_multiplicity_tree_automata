#![allow(dead_code, unused_variables, unused_imports, unused_mut)]

use log::{debug, error, info};
use nalgebra::DMatrix;
use std::{
    collections::{HashMap, HashSet},
    fmt,
};

type DynamicMatrix = DMatrix<f64>;

// This defines the way the rules are saved in the WeightedTreeAutomaton
#[derive(Debug, Clone)]
pub enum GrammarType {
    BottomUp,
    TopDown,
}

// "!" -> This is the Index of the final weight vector in the case of a bottom-up automaton.
#[derive(Debug)]
pub struct WeightedTreeAutomatonStruct {
    pub num_states: usize,
    pub grammar_type: GrammarType,
    pub ranked_alphabet: HashMap<String, usize>,
    pub transition_structs: HashMap<String, Vec<Production>>,
}

#[derive(Debug, Clone)]
pub struct WeightedTreeAutomatonMatrix {
    pub num_states: usize,
    pub grammar_type: GrammarType,
    pub ranked_alphabet: HashMap<String, usize>,
    pub transition_matrices: HashMap<String, DynamicMatrix>,
}

impl fmt::Display for WeightedTreeAutomatonMatrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = format!(
            "Number of States: {} \nGrammar Type: {:?} \nRanked Alphabet: ",
            self.num_states, self.grammar_type
        );
        for element in self.ranked_alphabet.clone().into_iter() {
            result.push_str(&format!("\n{}: {}", &element.0, element.1));
        }
        result.push_str("\nTransition Matrices:");
        for element in self.transition_matrices.clone().into_iter() {
            result.push_str(&format!(
                "\n{}: {}",
                &element.0,
                element.1.clone()
            ));
        }
        write!(f, "{}", result)
    }
}
impl fmt::Display for WeightedTreeAutomatonStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = format!(
            "Number of States: {} \nGrammar Type: {:?} \nRanked Alphabet: ",
            self.num_states, self.grammar_type
        );
        for element in self.ranked_alphabet.clone().into_iter() {
            result.push_str(&format!("\n{}: {}", &element.0, element.1));
        }
        result.push_str("\nProductions:");
        for element in self.transition_structs.clone().into_iter() {
            for prod in element.1.clone().into_iter() {
                result.push_str(&format!("\n{}: {}", &element.0, prod));
            }
        }

        write!(f, "{}", result)
    }
}

#[derive(Debug, Clone)]
// We need to use vectors inside the production, because we either look at bottom-up, in which case we need to
// encode more than one left side state, or in the case of top-down we are look at multiple right side states.
// There might be an argument to not save the root_symbol separately, but instead save it in the overlaying data structure:
// HashMap<String, Vec<data_structures::Production>>,
// but this way the Production struct can be used by itself.
pub struct Production {
    pub root_symbol: String,
    pub left_states: Vec<String>,
    pub right_states: Vec<String>,
    pub cost: f64,
}

impl fmt::Display for Production {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = format!(
            "Root Symbol: {} \nLeft States: {:?} \nRight States: {:?} \nCost: {}",
            self.root_symbol, self.left_states, self.right_states, self.cost
        );
        write!(f, "{}", result)
    }
}

pub fn prod_vec_to_automaton(
    automaton_struct: WeightedTreeAutomatonStruct,
) -> WeightedTreeAutomatonMatrix {
    debug!("Complete automaton: {:?}", automaton_struct);
    debug!("Converting Bottom-Up");
    let mut transition_matrices: HashMap<String, DynamicMatrix> = HashMap::new();

    // Ordering of states is always expected to be alpha-numerical.
    // Here we create a hashmap that contains the index of each state if states are ordered alphanumerical.
    let mut unordered_state_set = HashSet::new();
    for prod_vec in automaton_struct.transition_structs.values().into_iter() {
        for prod in prod_vec.iter() {
            for left_state in prod.left_states.clone() {
                unordered_state_set.insert(left_state);
            }
            for right_state in prod.right_states.clone() {
                unordered_state_set.insert(right_state);
            }
        }
    }

    let mut ordered_state_list: Vec<String> = unordered_state_set.into_iter().collect();
    ordered_state_list.sort();
    let mut ordered_state_hashmap = HashMap::new();
    for i in 0..ordered_state_list.len() {
        ordered_state_hashmap.insert(ordered_state_list[i].clone(), i);
    }

    for element in automaton_struct.ranked_alphabet.clone() {
        debug!("\n\n\nElement: {:?}", element);

        let mut row_length;
        // If the rank of the symbol is 0 -> row length = 1, else:
        // The row length is n^k with n = num of states & k = rank of the symbol
        match element.1 {
            0 => row_length = 1,
            //_ => row_length = element.1.pow(automaton_struct.num_states as u32),
            _ => row_length = automaton_struct.num_states.pow(element.1 as u32),
        }

        // If the production is a normal on the row length is equal the number of states. Else
        // it is 1 because it is the final weight vector. We also reset the row_length here in case
        // we are dealing with a final weight vector.
        let mut column_length;
        if element.0 == "!" {
            column_length = 1;
            row_length = automaton_struct.num_states;
        } else {
            column_length = automaton_struct.num_states;
        }
        debug!("{row_length}x{column_length}");
        let mut temp_matrix = DynamicMatrix::from_element(row_length, column_length, 0.0);

        let productions = automaton_struct
            .transition_structs
            .get(&element.0)
            .unwrap()
            .clone();
        for prod in productions.iter() {
            debug!("Left states: {:?}", prod.left_states);

            // The y value is simple, it is just the index of the right side state:
            // Ex.: 0 -> q2 0
            // Where Q = {q1, q2}
            // y = 1, because q2 has index 1 as saved in ordered_state_hashmap.
            // If we are considering the final weight vector it is simply 0 as indicated by the missing right side states.

            let mut y;
            match prod.right_states.len() {
                0 => y = 0,
                _ => {
                    y = ordered_state_hashmap
                        .get(&prod.right_states[0])
                        .unwrap()
                        .to_owned()
                }
            }

            // If there aren't any left side states that means we are in a leaf. As a result the x value is 0. Otherwise we use
            // a nifty trick to calculate the index of the state combination:
            // Let i be the index of state q as saved in ordered_state_hashmap, n the number of states, rk the rank of the production symbol:
            // x = 0;
            // for r in rk..0 {
            //   x += i*n.pow(rk - r)
            // }

            let mut x = 0;
            let num_states = automaton_struct.num_states;
            let rank = prod.left_states.len();
            for r in (0..rank).rev() {
                let q = prod.left_states[r].clone();
                let i = ordered_state_hashmap.get(&q).unwrap();
                x += i * num_states.pow((rank - 1 - r).try_into().unwrap());
                debug!("q=> x:{x} = {i} * {num_states}.pow({r})");
            }
            debug!("Adding {} at {x}:{y}", prod.cost);
            temp_matrix[(x, y)] = prod.cost;
        }
        transition_matrices.insert(element.0, temp_matrix);
    }

    for element in transition_matrices.clone() {
        debug!("{}: {}", element.0, element.1);
    }

    let automaton_matrix = WeightedTreeAutomatonMatrix {
        num_states: automaton_struct.num_states,
        grammar_type: automaton_struct.grammar_type,
        ranked_alphabet: automaton_struct.ranked_alphabet,
        transition_matrices,
    };
    return automaton_matrix;
}

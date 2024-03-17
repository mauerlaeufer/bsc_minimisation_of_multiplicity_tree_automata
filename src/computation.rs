#![allow(dead_code, unused_variables, unused_imports)]

use crate::data_structures::Production;
use crate::data_structures::{self, WeightedTreeAutomatonMatrix};
use nalgebra::DMatrix;
use trees::Tree;
use trees::{tr, Node};
use log::{debug, info};


type DynamicMatrix = DMatrix<f64>;

pub fn minimize_automaton(automaton_struct: WeightedTreeAutomatonMatrix) -> WeightedTreeAutomatonMatrix {
    return automaton_struct
}


// This is a simple, naive implementation that computes ||A||(t).
pub fn compute_val_for_tree(
    automaton_struct: WeightedTreeAutomatonMatrix,
    tree: Tree<char>,
) -> f64 {
    debug!("Tree to be evaluated: {tree}");
    fn calculate_weight_rec(auto: &WeightedTreeAutomatonMatrix, node: &Node<char>) -> DynamicMatrix {
        let data = node.data();
        let num_children = node.degree();
        debug!(
            "Current Matrix: {}",
            auto.transition_matrices
                .get(&data.to_string())
                .unwrap()
                .clone()
        );

        // In the following computation we simply assume that the tree & the automaton are defined correctly. The program will panic if
        // for example the dimensions don't match.
        debug!("Number of children: {}", num_children);
        match num_children {
            0 => {
                debug!(
                    "Case 0 Children, returning data: {}",
                    auto.transition_matrices
                        .get(&data.to_string())
                        .unwrap()
                        .clone()
                );
                return auto
                    .transition_matrices
                    .get(&data.to_string())
                    .unwrap()
                    .clone();
            }
            1 => {
                let val = auto
                    .transition_matrices
                    .get(&data.to_string())
                    .unwrap()
                    .clone();
                let child = node.front().unwrap();
                debug!(
                    "Case 1 Children, returning val: {} * calc_value({:?})",
                    val, child
                );
                let child_val = calculate_weight_rec(auto, child);
                let new_val = child_val * val;
                return new_val;
            }
            _ => {
                let child_iter = node.iter();
                let child_vec: Vec<&Node<char>> = child_iter.collect();
                debug!(
                    "Case {num_children} Children, iterating over: {}",
                    child_vec.len()
                );
                let weight_zero = calculate_weight_rec(auto, child_vec[0]);
                let weight_one = calculate_weight_rec(auto, child_vec[1]);
                let mut t_result = weight_zero.kronecker(&weight_one);
                for child_num_x in 2..child_vec.len() {
                    debug!("Looking at child number {child_num_x}");
                    let weight_x = calculate_weight_rec(auto, child_vec[child_num_x]);
                    t_result = t_result.kronecker(&weight_x);
                }
                let weight_self = auto
                    .transition_matrices
                    .get(&data.to_string())
                    .unwrap()
                    .clone();
                t_result = t_result * weight_self;
                return t_result;
            }
        }
    }
    let mut result = calculate_weight_rec(&automaton_struct, tree.root());
    result = result
        * automaton_struct
            .transition_matrices
            .get("!")
            .unwrap()
            .clone();
    debug!("Result: {}", result);
    return result[(0, 0)];
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;
    #[test]
    fn simple_counter_example() {
        let struct_automaton =
            parser::read_bottom_up_grammar("grammar/bottom_up/counter_tree");
        let automaton = data_structures::prod_vec_to_automaton(struct_automaton);
        let temp_tree_1 = trees::Tree::<char>::from_tuple('c');
        let temp_tree_2 = trees::Tree::<char>::from_tuple(('a', 'c'));
        let temp_tree_3 = trees::Tree::<char>::from_tuple(('a', ('a', 'c')));
        let temp_tree_4 = trees::Tree::<char>::from_tuple(('b', 'c', 'c'));
        let temp_tree_5 = trees::Tree::<char>::from_tuple(('a', ('b', 'c', 'c')));
        let temp_tree_6 = trees::Tree::<char>::from_tuple(('a', ('b', 'c', ('a', 'c'))));
        let val_1 = compute_val_for_tree(automaton.clone(), temp_tree_1);
        let val_2 = compute_val_for_tree(automaton.clone(), temp_tree_2);
        let val_3 = compute_val_for_tree(automaton.clone(), temp_tree_3);
        let val_4 = compute_val_for_tree(automaton.clone(), temp_tree_4);
        let val_5 = compute_val_for_tree(automaton.clone(), temp_tree_5);
        let val_6 = compute_val_for_tree(automaton.clone(), temp_tree_6);
        assert_eq!(val_1, 0.0);
        assert_eq!(val_2, 1.0);
        assert_eq!(val_3, 2.0);
        assert_eq!(val_4, 0.0);
        assert_eq!(val_5, 1.0);
        assert_eq!(val_6, 2.0);
        
    }
    #[test]
    fn simple_computation_example() {
        let struct_automaton =
            parser::read_bottom_up_grammar("grammar/bottom_up/computation_tree");
        let automaton = data_structures::prod_vec_to_automaton(struct_automaton);
        
        let temp_tree_1 = trees::Tree::<char>::from_tuple('0');
        let temp_tree_2 = trees::Tree::<char>::from_tuple('1');
        let temp_tree_3 = trees::Tree::<char>::from_tuple(('+', '1', '1'));
        let temp_tree_4 = trees::Tree::<char>::from_tuple(('x', '1', '1'));
        let temp_tree_5 = trees::Tree::<char>::from_tuple(('x', '0', '1'));
        let temp_tree_6 = trees::Tree::<char>::from_tuple(('+', ('x', '1', '1'), ('+', '1', '1')));
        let temp_tree_7 = tr('x')
            / (-(tr('+') / (-tr('1') - tr('1')))
                - (tr('+')
                    / (-tr('1') - (tr('+') / (-tr('1') - (tr('+') / (-tr('1') - tr('1'))))))));
        let val_1 = compute_val_for_tree(automaton.clone(), temp_tree_1);
        let val_2 = compute_val_for_tree(automaton.clone(), temp_tree_2);
        let val_3 = compute_val_for_tree(automaton.clone(), temp_tree_3);
        let val_4 = compute_val_for_tree(automaton.clone(), temp_tree_4);
        let val_5 = compute_val_for_tree(automaton.clone(), temp_tree_5);
        let val_6 = compute_val_for_tree(automaton.clone(), temp_tree_6);
        let val_7 = compute_val_for_tree(automaton.clone(), temp_tree_7);
        assert_eq!(val_1, 0.0);
        assert_eq!(val_2, 1.0);
        assert_eq!(val_3, 2.0);
        assert_eq!(val_4, 1.0);
        assert_eq!(val_5, 0.0);
        assert_eq!(val_6, 3.0);
        assert_eq!(val_7, 8.0);
    }
    #[test]
    fn simple_rgb_example() {
        let struct_automaton =
            parser::read_bottom_up_grammar("grammar/bottom_up/rgb_tree");
        let automaton = data_structures::prod_vec_to_automaton(struct_automaton);

        let temp_tree_1 = trees::Tree::<char>::from_tuple('R');
        let temp_tree_2 = trees::Tree::<char>::from_tuple('G');
        let temp_tree_3 = trees::Tree::<char>::from_tuple(('+', 'G', 'B'));
        let temp_tree_4 = trees::Tree::<char>::from_tuple(('+', 'B', 'B'));
        let temp_tree_5 = trees::Tree::<char>::from_tuple(('+', 'R', 'R'));
        let temp_tree_6 = trees::Tree::<char>::from_tuple(('+', ('+', 'R', 'R'), ('+', 'G', 'B')));
        let val_1 = compute_val_for_tree(automaton.clone(), temp_tree_1);
        let val_2 = compute_val_for_tree(automaton.clone(), temp_tree_2);
        let val_3 = compute_val_for_tree(automaton.clone(), temp_tree_3);
        let val_4 = compute_val_for_tree(automaton.clone(), temp_tree_4);
        let val_5 = compute_val_for_tree(automaton.clone(), temp_tree_5);
        let val_6 = compute_val_for_tree(automaton.clone(), temp_tree_6);
        assert_eq!(val_1, 256000000.0);
        assert_eq!(val_2, 256.0);
        assert_eq!(val_3, 128128.0);
        assert_eq!(val_4, 256000.0);
        assert_eq!(val_5, 256000000.0);
        assert_eq!(val_6, 128064064.0);
    }
}

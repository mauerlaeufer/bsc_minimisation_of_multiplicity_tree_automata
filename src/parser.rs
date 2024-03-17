#![allow(dead_code, unused_variables, unused_imports, unused_mut)]

use nalgebra::DMatrix;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use crate::data_structures;
use crate::data_structures::Production;

use log::{debug, info};


/*
    This file contains the parser functions which take different formats of text file and then turn them 
    into the internal data structures. These functions are quite long and often not that pretty. 
    This is a direct result of their function, namely parsing data. 
*/



// The bottom-up grammar format is defined by me. For a definition please look into the text-part.
pub fn read_bottom_up_grammar(
    path: &str
) -> data_structures::WeightedTreeAutomatonStruct {
    debug!("Reading bottom-up grammar");
    let mut production_rules: HashMap<String, Vec<Production>> = HashMap::new();

    let grammar_file: File = File::open(path).unwrap();
    let grammar_reader: BufReader<File> = BufReader::new(grammar_file);

    // Since we iterate over the grammar anyways, lets save some useful information.
    // We create a set for all states:
    let mut state_set = HashSet::new();
    // And a ranked alphabet:
    let mut ranked_alphabet: HashMap<String, usize> = HashMap::new();

    for line in grammar_reader.lines() {
        let line_copy: String = line.unwrap();
        // Skip emtpy lines!
        if line_copy == "" {
            continue;
        }


        let mut parts: std::str::SplitWhitespace<'_> = line_copy.split_whitespace();


        // First element is root state or indicates that it is the final weight vector!
        let root_symbol = parts.next().unwrap().to_owned();

        // If the element is the final weight vector we will have to take special care of that!
        if root_symbol == "!" {
            // Create production struct for the final weight vector.
            let state = parts.next().unwrap().to_owned();
            let cost = parts.next().unwrap().parse().unwrap();
            let production = data_structures::Production {
                root_symbol: "!".to_owned(),
                left_states: vec![state],
                right_states: vec![],
                cost,
            };
            production_rules
                .entry(root_symbol)
                .and_modify(|e: &mut Vec<Production>| e.push(production.clone()))
                .or_insert(vec![production]);   
            continue;
        }

        let mut left_states = vec![];
        let mut current = parts.next().unwrap().to_owned();

        // Find the left side states!
        while current != "->" {
            left_states.push(current.clone());
            state_set.insert(current);
            current = parts.next().unwrap().to_owned();
        }

        ranked_alphabet.insert(root_symbol.clone(), left_states.len());

        // The next element after the '->' is always the right side state!
        let right_states = vec![parts.next().unwrap().to_owned()];
        // the next element after the right side state is always the cost!
        let cost = parts.next().unwrap().parse().unwrap();

        // Create production struct for each production and add them to the rules.
        let production = data_structures::Production {
            root_symbol: root_symbol.clone(),
            left_states,
            right_states,
            cost,
        };

        debug!("{production:?}");

        production_rules
            .entry(root_symbol)
            .and_modify(|e| e.push(production.clone()))
            .or_insert(vec![production]);   
    }


    let num_states = state_set.len();
    // The final weight vector always has size of number of states.
    ranked_alphabet.insert("!".to_owned(), num_states);
    

    let return_value = data_structures::WeightedTreeAutomatonStruct {
        num_states,
        grammar_type: data_structures::GrammarType::BottomUp,
        ranked_alphabet,
        transition_structs: production_rules
    };
    return return_value;
}


// This reads grammars of the form which are used by the Berkley parser. For more details please look at https://github.com/slavpetrov/berkeleyparser
// This code currently has no way to work with top down grammars, but the basic data-structures are build in a way that an extension should be easily possible.
// Writing a top-down to bottom-up parser should allow the application of the minimisation on top-down automata.
// This code can be considered irrelevant for the thesis itself but is included in case that at some point somebody wants to build upon this code.

pub fn read_top_down_automaton(
    path_grammar: &str,
    path_lexicon: &str,
) -> data_structures::WeightedTreeAutomatonStruct {

    let mut production_rules: HashMap<String, Vec<Production>> = HashMap::new();
    // Since we iterate over the grammar anyways, lets save some useful information.
    // We create a set for all states:
    let mut state_set: HashSet<String> = HashSet::new();
    // And a ranked alphabet:
    let mut ranked_alphabet: HashMap<String, usize> = HashMap::new();

    debug!("Reading top-down grammar");
    let grammar_file: File = File::open(path_grammar).unwrap();
    let grammar_reader = BufReader::new(grammar_file);
    // Go line by line.
    for line in grammar_reader.lines() {
        // Split each line by Whitespace and '->'
        let line_copy: String = line.unwrap().replace(" -> ", " ");
        let mut parts: std::str::SplitWhitespace<'_> = line_copy.split_whitespace();
        // Find out length and set the root state
        let num = parts.clone().count();
        let left_state = parts.next().unwrap().to_owned();
        state_set.insert(left_state.clone());


        // Create a vec which contains all right target states.
        let mut right_states = vec![];
        for _i in 0..(num - 2) {
            let current_state = parts.next().unwrap().to_owned();
            state_set.insert(current_state.clone());
            right_states.push(current_state);
        }
        // The last element of the line is the cost of all productions on that line.
        let cost: f64 = parts.next().unwrap().parse().unwrap();

        // The root symbol is encoded as part of the left state.
        let root_symbol = left_state.split("_").next().unwrap().to_owned();

        ranked_alphabet.insert(root_symbol.clone(), right_states.len());

        // Create production struct for each production and add them to the rules.
        let production = data_structures::Production {
            root_symbol: root_symbol.clone(),
            left_states: vec![left_state.clone()],
            right_states,
            cost,
        };
        debug!("{production:?}");
        production_rules
            .entry(root_symbol)
            .and_modify(|e| e.push(production.clone()))
            .or_insert(vec![production]);
    }

    debug!("Reading top-down lexicon");
    let lexicon_file = File::open(path_lexicon).unwrap();
    let lexicon_reader = BufReader::new(lexicon_file);
    // Line by line
    for line in lexicon_reader.lines() {
        // Split the line by whitespace, '[' and by ','.
        let mut line_copy: String = line.unwrap();
        line_copy = line_copy.replace("[", "").replace(",", "").replace("]", "");
        let mut parts = line_copy.split_whitespace();

        // Each line contains productions for multiple left side states which consist out of a Letter & a number.
        // The number is based on the amount of states inside the square brackets, so:
        // A B [0, 1]
        // has two productions, with the left side being A_0 and A_1, both have a transition to B, with B being a root symbol.
        let left_state_start = parts.next().unwrap();
        // There is no right state, since these are only leaf nodes. Instead the root symbol is extracted.
        let root_symbol = parts.next().unwrap().to_owned();
        let state_count = parts.clone().count();
        for i in 0..(state_count) {
            let left_state = left_state_start.to_owned() + "_" + &i.to_string();

            state_set.insert(left_state.clone());
            ranked_alphabet.insert(root_symbol.clone(), 0);

            let cost = parts.next().unwrap().parse().unwrap();
            let production: data_structures::Production = data_structures::Production {
                root_symbol: root_symbol.clone(),
                left_states: vec![left_state],
                right_states: vec![],
                cost,
            };
            debug!("{production:?}");

            production_rules
                .entry(root_symbol.clone())
                .and_modify(|e| e.push(production.clone()))
                .or_insert(vec![production]);
        }
    }


    let return_value = data_structures::WeightedTreeAutomatonStruct {
        num_states: state_set.len(),
        grammar_type: data_structures::GrammarType::TopDown,
        ranked_alphabet,
        transition_structs: production_rules
    };
    return return_value
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn read_bottom_up_compu_example() {
        let example_tree_compu = read_bottom_up_grammar("grammar/bottom_up/computation_tree");
    }
    #[test]
    fn read_bottom_up_counter_example() {
        let example_tree_compu = read_bottom_up_grammar("grammar/bottom_up/counter_tree");
    }
    #[test]
    fn read_bottom_up_rgb_example() {
        let example_tree_compu = read_bottom_up_grammar("grammar/bottom_up/rgb_tree");
    }
}

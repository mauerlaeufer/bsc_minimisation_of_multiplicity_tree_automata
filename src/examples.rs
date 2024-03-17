use crate::computation;
use crate::minimisation;
use crate::parser;
use crate::data_structures;
use trees::tr;

pub fn counter_example() {
    println!("Counter example: ");
    let struct_automaton = parser::read_bottom_up_grammar("automata/bottom_up/counter_tree");
    let automaton = data_structures::prod_vec_to_automaton(struct_automaton);
    let minimised_automaton = minimisation::minimize_automaton(automaton.clone());
    let struct_automaton_times_two = parser::read_bottom_up_grammar("automata/bottom_up/counter_tree_times_two");
    let automaton_times_two = data_structures::prod_vec_to_automaton(struct_automaton_times_two);
    let minimised_automaton_times_two = minimisation::minimize_automaton(automaton_times_two.clone());
    
    println!("\n\nA: Counter automaton:\n\n{}", automaton);
    println!("\n\nÃ: Counter automaton minimised :\n\n{}", minimised_automaton);
    println!("\n\nB: Counter automaton x2:\n\n{}", automaton_times_two);
    println!("\n\nB̃: Counter automaton x2 minimised:\n\n{}", minimised_automaton_times_two);


    let temp_tree_1 = trees::Tree::<char>::from_tuple('a');
    let temp_tree_2 = trees::Tree::<char>::from_tuple('b');
    let temp_tree_3 = trees::Tree::<char>::from_tuple(('s', 'b', 'a'));
    let temp_tree_4 = trees::Tree::<char>::from_tuple(('s', 'a', 'a'));
    let temp_tree_5 = trees::Tree::<char>::from_tuple(('s', 'b', 'b'));
    let temp_tree_6 = trees::Tree::<char>::from_tuple(('s', ('s', 'b', 'b'), 'b'));
    let val_1 = computation::compute_val_for_tree(automaton.clone(), temp_tree_1.clone());
    let val_2 = computation::compute_val_for_tree(automaton.clone(), temp_tree_2.clone());
    let val_3 = computation::compute_val_for_tree(automaton.clone(), temp_tree_3.clone());
    let val_4 = computation::compute_val_for_tree(automaton.clone(), temp_tree_4.clone());
    let val_5 = computation::compute_val_for_tree(automaton.clone(), temp_tree_5.clone());
    let val_6 = computation::compute_val_for_tree(automaton.clone(), temp_tree_6.clone());
    let val_1_minimised = computation::compute_val_for_tree(minimised_automaton.clone(), temp_tree_1.clone());
    let val_2_minimised = computation::compute_val_for_tree(minimised_automaton.clone(), temp_tree_2.clone());
    let val_3_minimised = computation::compute_val_for_tree(minimised_automaton.clone(), temp_tree_3.clone());
    let val_4_minimised = computation::compute_val_for_tree(minimised_automaton.clone(), temp_tree_4.clone());
    let val_5_minimised = computation::compute_val_for_tree(minimised_automaton.clone(), temp_tree_5.clone());
    let val_6_minimised = computation::compute_val_for_tree(minimised_automaton.clone(), temp_tree_6.clone());
    let val_1_times_two = computation::compute_val_for_tree(automaton_times_two.clone(), temp_tree_1.clone());
    let val_2_times_two = computation::compute_val_for_tree(automaton_times_two.clone(), temp_tree_2.clone());
    let val_3_times_two = computation::compute_val_for_tree(automaton_times_two.clone(), temp_tree_3.clone());
    let val_4_times_two = computation::compute_val_for_tree(automaton_times_two.clone(), temp_tree_4.clone());
    let val_5_times_two = computation::compute_val_for_tree(automaton_times_two.clone(), temp_tree_5.clone());
    let val_6_times_two = computation::compute_val_for_tree(automaton_times_two.clone(), temp_tree_6.clone());
    let val_1_minimised_times_two = computation::compute_val_for_tree(minimised_automaton_times_two.clone(), temp_tree_1.clone());
    let val_2_minimised_times_two = computation::compute_val_for_tree(minimised_automaton_times_two.clone(), temp_tree_2.clone());
    let val_3_minimised_times_two = computation::compute_val_for_tree(minimised_automaton_times_two.clone(), temp_tree_3.clone());
    let val_4_minimised_times_two = computation::compute_val_for_tree(minimised_automaton_times_two.clone(), temp_tree_4.clone());
    let val_5_minimised_times_two = computation::compute_val_for_tree(minimised_automaton_times_two.clone(), temp_tree_5.clone());
    let val_6_minimised_times_two = computation::compute_val_for_tree(minimised_automaton_times_two.clone(), temp_tree_6.clone());
    println!("Tree: {}", temp_tree_1);
    println!("||A||(t): 1.0           = {:.2}", val_1);
    println!("||Ã||(t): 1.0           ≈ {:.2}", val_1_minimised);
    println!("||B||(t): 1.0           = {:.2}", val_1_times_two);
    println!("||B̃||(t): 1.0           ≈ {:.2}", val_1_minimised_times_two);
    println!("Tree: {}", temp_tree_2);
    println!("||A||(t): -1.0          = {:.2}", val_2);    
    println!("||Ã||(t): -1.0          ≈ {:.2}", val_2_minimised);    
    println!("||B||(t): -1.0          = {:.2}", val_2_times_two);    
    println!("||B̃||(t): -1.0          ≈ {:.2}", val_2_minimised_times_two);    
    println!("Tree: {}", temp_tree_3);
    println!("||A||(t): 0.0           = {:.2}", val_3);
    println!("||Ã||(t): 0.0           ≈ {:.2}", val_3_minimised);
    println!("||B||(t): 0.0           = {:.2}", val_3_times_two);
    println!("||B̃||(t): 0.0           ≈ {:.2}", val_3_minimised_times_two);
    println!("Tree: {}", temp_tree_4);
    println!("||A||(t): 2.0           = {:.2}", val_4);
    println!("||Ã||(t): 2.0           ≈ {:.2}", val_4_minimised);
    println!("||B||(t): 2.0           = {:.2}", val_4_times_two);
    println!("||B̃||(t): 2.0           ≈ {:.2}", val_4_minimised_times_two);
    println!("Tree: {}", temp_tree_5);
    println!("||A||(t): -2.0           = {:.2}", val_5);
    println!("||Ã||(t): -2.0           ≈ {:.2}", val_5_minimised);
    println!("||B||(t): -2.0           = {:.2}", val_5_times_two);
    println!("||B̃||(t): -2.0           ≈ {:.2}", val_5_minimised_times_two);
    println!("Tree: {}", temp_tree_6);
    println!("||A||(t): -3.0           = {:.2}", val_6);        
    println!("||Ã||(t): -3.0           ≈ {:.2}", val_6_minimised);        
    println!("||B||(t): -3.0           = {:.2}", val_6_times_two);        
    println!("||B̃||(t): -3.0           ≈ {:.2}", val_6_minimised_times_two);        
    println!();
}


pub fn computation_example() {
    println!("Computation example:");

    let struct_automaton = parser::read_bottom_up_grammar("automata/bottom_up/computation_tree");
    let automaton = data_structures::prod_vec_to_automaton(struct_automaton);
    let minimised_automaton = minimisation::minimize_automaton(automaton.clone());

    let struct_automaton_times_two = parser::read_bottom_up_grammar("automata/bottom_up/computation_tree_times_two");
    let automaton_times_two = data_structures::prod_vec_to_automaton(struct_automaton_times_two);
    let minimised_automaton_times_two = minimisation::minimize_automaton(automaton_times_two.clone());
    println!("\n\nComputation Automaton:\n\n{}", automaton);
    println!("\n\nComputation Automaton minimised :\n\n{}", minimised_automaton);
    println!("\n\nComputation Automaton x2:\n\n{}", automaton_times_two);
    println!("\n\nComputation Automaton x2 minimised:\n\n{}", minimised_automaton_times_two);

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
    let val_1 = computation::compute_val_for_tree(automaton.clone(), temp_tree_1.clone());
    let val_2 = computation::compute_val_for_tree(automaton.clone(), temp_tree_2.clone());
    let val_3 = computation::compute_val_for_tree(automaton.clone(), temp_tree_3.clone());
    let val_4 = computation::compute_val_for_tree(automaton.clone(), temp_tree_4.clone());
    let val_5 = computation::compute_val_for_tree(automaton.clone(), temp_tree_5.clone());
    let val_6 = computation::compute_val_for_tree(automaton.clone(), temp_tree_6.clone());
    let val_7 = computation::compute_val_for_tree(automaton.clone(), temp_tree_7.clone());
    let val_1_minimised = computation::compute_val_for_tree(minimised_automaton.clone(), temp_tree_1.clone());
    let val_2_minimised = computation::compute_val_for_tree(minimised_automaton.clone(), temp_tree_2.clone());
    let val_3_minimised = computation::compute_val_for_tree(minimised_automaton.clone(), temp_tree_3.clone());
    let val_4_minimised = computation::compute_val_for_tree(minimised_automaton.clone(), temp_tree_4.clone());
    let val_5_minimised = computation::compute_val_for_tree(minimised_automaton.clone(), temp_tree_5.clone());
    let val_6_minimised = computation::compute_val_for_tree(minimised_automaton.clone(), temp_tree_6.clone());
    let val_7_minimised = computation::compute_val_for_tree(minimised_automaton.clone(), temp_tree_7.clone());
    let val_1_times_two = computation::compute_val_for_tree(automaton_times_two.clone(), temp_tree_1.clone());
    let val_2_times_two = computation::compute_val_for_tree(automaton_times_two.clone(), temp_tree_2.clone());
    let val_3_times_two = computation::compute_val_for_tree(automaton_times_two.clone(), temp_tree_3.clone());
    let val_4_times_two = computation::compute_val_for_tree(automaton_times_two.clone(), temp_tree_4.clone());
    let val_5_times_two = computation::compute_val_for_tree(automaton_times_two.clone(), temp_tree_5.clone());
    let val_6_times_two = computation::compute_val_for_tree(automaton_times_two.clone(), temp_tree_6.clone());
    let val_7_times_two = computation::compute_val_for_tree(automaton_times_two.clone(), temp_tree_7.clone());
    let val_1_minimised_times_two = computation::compute_val_for_tree(minimised_automaton_times_two.clone(), temp_tree_1.clone());
    let val_2_minimised_times_two = computation::compute_val_for_tree(minimised_automaton_times_two.clone(), temp_tree_2.clone());
    let val_3_minimised_times_two = computation::compute_val_for_tree(minimised_automaton_times_two.clone(), temp_tree_3.clone());
    let val_4_minimised_times_two = computation::compute_val_for_tree(minimised_automaton_times_two.clone(), temp_tree_4.clone());
    let val_5_minimised_times_two = computation::compute_val_for_tree(minimised_automaton_times_two.clone(), temp_tree_5.clone());
    let val_6_minimised_times_two = computation::compute_val_for_tree(minimised_automaton_times_two.clone(), temp_tree_6.clone());
    let val_7_minimised_times_two = computation::compute_val_for_tree(minimised_automaton_times_two.clone(), temp_tree_7.clone());
    println!("Tree: {}", temp_tree_1);
    println!("||A||(t): 0.0           = {:.2}", val_1);
    println!("||Ã||(t): 0.0           ≈ {:.2}", val_1_minimised);
    println!("||B||(t): (0.0) x 2     = {:.2}", val_1_times_two);
    println!("||B̃||(t): (0.0) x 2     ≈ {:.2}", val_1_minimised_times_two);
    println!("Tree: {}", temp_tree_2);
    println!("||A||(t): 1.0           = {:.2}", val_2);    
    println!("||Ã||(t): 1.0           ≈ {:.2}", val_2_minimised);    
    println!("||B||(t): (1.0) x 2     = {:.2}", val_2_times_two);    
    println!("||B̃||(t): (1.0) x 2     ≈ {:.2}", val_2_minimised_times_two);    
    println!("Tree: {}", temp_tree_3);
    println!("||A||(t): 2.0           = {:.2}", val_3);
    println!("||Ã||(t): 2.0           ≈ {:.2}", val_3_minimised);
    println!("||B||(t): (2.0) x 2     = {:.2}", val_3_times_two);
    println!("||B̃||(t): (2.0) x 2     ≈ {:.2}", val_3_minimised_times_two);
    println!("Tree: {}", temp_tree_4);
    println!("||A||(t): 1.0           = {:.2}", val_4);
    println!("||Ã||(t): 1.0           ≈ {:.2}", val_4_minimised);
    println!("||B||(t): (1.0) x 2     = {:.2}", val_4_times_two);
    println!("||B̃||(t): (1.0) x 2     ≈ {:.2}", val_4_minimised_times_two);
    println!("Tree: {}", temp_tree_5);
    println!("||A||(t): 0.0           = {:.2}", val_5);
    println!("||Ã||(t): 0.0           ≈ {:.2}", val_5_minimised);
    println!("||B||(t): (0.0) x 2     = {:.2}", val_5_times_two);
    println!("||B̃||(t): (0.0) x 2     ≈ {:.2}", val_5_minimised_times_two);
    println!("Tree: {}", temp_tree_6);
    println!("||A||(t): 3.0           = {:.2}", val_6);        
    println!("||Ã||(t): 3.0           ≈ {:.2}", val_6_minimised);        
    println!("||B||(t): (3.0) x 2     = {:.2}", val_6_times_two);        
    println!("||B̃||(t): (3.0) x 2     ≈ {:.2}", val_6_minimised_times_two);        
    println!("Tree: {}", temp_tree_7);
    println!("||A||(t): 8.0           = {:.2}", val_7);        
    println!("||Ã||(t): 8.0           ≈ {:.2}", val_7_minimised);        
    println!("||B||(t): (8.0) x 2     = {:.2}", val_7_times_two);        
    println!("||B̃||(t): (8.0) x 2     ≈ {:.2}", val_7_minimised_times_two);        
    println!();

}



pub fn simple_rgb_example() {
    println!("Simple computation example with a minimised automaton: ");

    let struct_automaton = parser::read_bottom_up_grammar("automata/bottom_up/rgb_tree");
    let automaton = data_structures::prod_vec_to_automaton(struct_automaton);
    let minimised_automaton = minimisation::minimize_automaton(automaton.clone());

    let struct_automaton_times_two = parser::read_bottom_up_grammar("automata/bottom_up/rgb_tree_times_two");
    let automaton_times_two = data_structures::prod_vec_to_automaton(struct_automaton_times_two);
    let minimised_automaton_times_two = minimisation::minimize_automaton(automaton_times_two.clone());
   
    println!("\n\nrgb automaton:\n\n{}", automaton);
    println!("\n\nrgb automaton minimised :\n\n{}", minimised_automaton);
    println!("\n\nrgb automaton x2:\n\n{}", automaton_times_two);
    println!("\n\nrgb automaton x2 minimised:\n\n{}", minimised_automaton_times_two);


    let temp_tree_1 = trees::Tree::<char>::from_tuple('R');
    let temp_tree_2 = trees::Tree::<char>::from_tuple('G');
    let temp_tree_3 = trees::Tree::<char>::from_tuple(('+', 'G', 'B'));
    let temp_tree_4 = trees::Tree::<char>::from_tuple(('+', 'B', 'B'));
    let temp_tree_5 = trees::Tree::<char>::from_tuple(('+', 'R', 'R'));
    let temp_tree_6 = trees::Tree::<char>::from_tuple(('+', ('+', 'R', 'R'), ('+', 'G', 'B')));
    let val_1 = computation::compute_val_for_tree(automaton.clone(), temp_tree_1.clone());
    let val_2 = computation::compute_val_for_tree(automaton.clone(), temp_tree_2.clone());
    let val_3 = computation::compute_val_for_tree(automaton.clone(), temp_tree_3.clone());
    let val_4 = computation::compute_val_for_tree(automaton.clone(), temp_tree_4.clone());
    let val_5 = computation::compute_val_for_tree(automaton.clone(), temp_tree_5.clone());
    let val_6 = computation::compute_val_for_tree(automaton.clone(), temp_tree_6.clone());
    let val_1_minimised = computation::compute_val_for_tree(minimised_automaton.clone(), temp_tree_1.clone());
    let val_2_minimised = computation::compute_val_for_tree(minimised_automaton.clone(), temp_tree_2.clone());
    let val_3_minimised = computation::compute_val_for_tree(minimised_automaton.clone(), temp_tree_3.clone());
    let val_4_minimised = computation::compute_val_for_tree(minimised_automaton.clone(), temp_tree_4.clone());
    let val_5_minimised = computation::compute_val_for_tree(minimised_automaton.clone(), temp_tree_5.clone());
    let val_6_minimised = computation::compute_val_for_tree(minimised_automaton.clone(), temp_tree_6.clone());
    let val_1_times_two = computation::compute_val_for_tree(automaton_times_two.clone(), temp_tree_1.clone());
    let val_2_times_two = computation::compute_val_for_tree(automaton_times_two.clone(), temp_tree_2.clone());
    let val_3_times_two = computation::compute_val_for_tree(automaton_times_two.clone(), temp_tree_3.clone());
    let val_4_times_two = computation::compute_val_for_tree(automaton_times_two.clone(), temp_tree_4.clone());
    let val_5_times_two = computation::compute_val_for_tree(automaton_times_two.clone(), temp_tree_5.clone());
    let val_6_times_two = computation::compute_val_for_tree(automaton_times_two.clone(), temp_tree_6.clone());
    let val_1_minimised_times_two = computation::compute_val_for_tree(minimised_automaton_times_two.clone(), temp_tree_1.clone());
    let val_2_minimised_times_two = computation::compute_val_for_tree(minimised_automaton_times_two.clone(), temp_tree_2.clone());
    let val_3_minimised_times_two = computation::compute_val_for_tree(minimised_automaton_times_two.clone(), temp_tree_3.clone());
    let val_4_minimised_times_two = computation::compute_val_for_tree(minimised_automaton_times_two.clone(), temp_tree_4.clone());
    let val_5_minimised_times_two = computation::compute_val_for_tree(minimised_automaton_times_two.clone(), temp_tree_5.clone());
    let val_6_minimised_times_two = computation::compute_val_for_tree(minimised_automaton_times_two.clone(), temp_tree_6.clone());
    println!("Tree: {}", temp_tree_1);
    println!("||A||(t): 256000000                 = {:.2}", val_1);
    println!("||Ã||(t): 256000000                 ≈ {:.2}", val_1_minimised);
    println!("||B||(t): (256000000) xrgbx 2       = {:.2}", val_1_times_two);
    println!("||B̃||(t): (256000000) xrgbx 2       ≈ {:.2}", val_1_minimised_times_two);
    println!("Tree: {}", temp_tree_2);
    println!("||A||(t): 256                       = {:.2}", val_2);    
    println!("||Ã||(t): 256                       ≈ {:.2}", val_2_minimised);    
    println!("||B||(t): (256) xrgbx 2             = {:.2}", val_2_times_two);    
    println!("||B̃||(t): (256) xrgbx 2             ≈ {:.2}", val_2_minimised_times_two);    
    println!("Tree: {}", temp_tree_3);
    println!("||A||(t): 128128                    = {:.2}", val_3);
    println!("||Ã||(t): 128128                    ≈ {:.2}", val_3_minimised);
    println!("||B||(t): (128128) xrgbx 2          = {:.2}", val_3_times_two);
    println!("||B̃||(t): (128128) xrgbx 2          ≈ {:.2}", val_3_minimised_times_two);
    println!("Tree: {}", temp_tree_4);
    println!("||A||(t): 256000                    = {:.2}", val_4);
    println!("||Ã||(t): 256000                    ≈ {:.2}", val_4_minimised);
    println!("||B||(t): (256000) xrgbx 2          = {:.2}", val_4_times_two);
    println!("||B̃||(t): (256000) xrgbx 2          ≈ {:.2}", val_4_minimised_times_two);
    println!("Tree: {}", temp_tree_5);
    println!("||A||(t): 256000000                 = {:.2}", val_5);
    println!("||Ã||(t): 256000000                 ≈ {:.2}", val_5_minimised);
    println!("||B||(t): (256000000) xrgbx 2       = {:.2}", val_5_times_two);
    println!("||B̃||(t): (256000000) xrgbx 2       ≈ {:.2}", val_5_minimised_times_two);
    println!("Tree: {}", temp_tree_6);
    println!("||A||(t): 128064064                 = {:.2}", val_6);        
    println!("||Ã||(t): 128064064                 ≈ {:.2}", val_6_minimised);        
    println!("||B||(t): (128064064) xrgbx 2       = {:.2}", val_6_times_two);        
    println!("||B̃||(t): (128064064) xrgbx 2       ≈ {:.2}", val_6_minimised_times_two);        
    println!();
}

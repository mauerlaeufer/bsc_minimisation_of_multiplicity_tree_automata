#![allow(confusable_idents)]

pub mod computation;
pub mod data_structures;
pub mod examples;
pub mod minimisation;
pub mod parser;
pub mod step_i;
pub mod step_ii;
pub mod step_iii;

// Used for command line arguments.
use clap::Parser;
use log::debug;
use crate::examples::*;

/// A multiplicity tree automata implementation with a accompanying minimizing algorithm. The 'automaton' folder needs to be in the same directory as the binary.
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    /// Automaton: {counter, computation, rgb, all}
    #[arg(short, long)]
    automaton: String,
}

fn main() {
    env_logger::init();
    let args = Args::parse();
    debug!("Debug mode enabled.");
    println!("Execution {}", args.automaton);
    if args.automaton == "counter" {
        counter_example();
    } else if args.automaton == "computation" {
        computation_example();
    } else if args.automaton == "rgb" {
        simple_rgb_example();
    } else if args.automaton == "all" {
        counter_example();
        computation_example();
        simple_rgb_example();
    } else {
        println!("Please enter a valid automaton - see --help.")
    }
}

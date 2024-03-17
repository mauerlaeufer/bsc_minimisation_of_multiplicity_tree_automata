# Minimization of multiplicity tree automata
This is the accompanying source code for the bachelor thesis "Minimization of multiplicity tree automata" by Hanno Krümpelmann.
An implementation in Rust of the minimisation algorithm described by Kiefer et. al. in 'Minimisation of Multiplicity Tree Automata'.

# Notes about output
One can easily see that the results of minimisation on most automata return matrices which, while containing the correct results, are 
slightly inaccurate. This is a simple result of numerical instability and the inability to represent certain fractions as floating point
variables. 

## Executing the binary
This binary can be executed on the commandline by running:
`./minimization_of_multiplicity_tree_automata.bin --automaton all`
where the argument `all` can be replaced by either `rgb`, `computation` or `counter` to run the minimization on only one of the examples.
These examples are all described in detail in the text of the thesis.
Note that the binary needs to be in the same directory as the folder `automata`. 

## Debug log

Debug information can be generated by calling:
`RUST_LOG=minimisation_of_multiplicity_tree_automata::parse=debug cargo run -- -a counter` (Tested on Linux)
where `minimisation_of_multiplicity_tree_automata::parse` can be replaced by the module one which one wants to generate debug information. For example: `minimisation_of_multiplicity_tree_automata::step_i`.
On Windows one can edit the ./cargo/config.toml file and apply the `RUST_LOG` value in the same manner.

Should one wish to only print a single value instead of all debug values in one module, there is an easy way to achieve this:
1. Go to the relevant file in the source code.
2. Look for the specific code `debug!("Print specific debug value {value}");`.
3. Replace `debug` with `println`. The code above for example should be `println!("Print specific debug value {value}");`.
4. Follow the steps below called 'Building a binary and executing it'.

By default all debug logging is disabled.

## Building a binary and executing it
The computer needs a installation of rust & cargo to build the binary. For installation instructions regarding rust & cargo look [here](https://www.rust-lang.org/tools/install).
After the installation all that is needed to be done is:
`cargo build`
and the binary will appear as `./target/debug/minimization_of_multiplicity_tree_automata`.
To execute the code in the same manner as detailed in 'Executing the binary':
`cargo run -- -a all`
One final thing to note, if one builds the binary and tries to execute it in `./target/debug/` it will fail. The folder `automata` needs to be in the same directory as the binary.

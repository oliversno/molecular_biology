use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let dna_string = fs::read_to_string(filename)
        .expect("Could not read file");
    let rna_string = dna_string.replace("T", "U");
    println!("{}", rna_string);
}


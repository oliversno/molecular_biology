use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let dna_string = fs::read_to_string(filename)
        .expect("Could not read file");
    let dna_string = dna_string.chars().rev().collect::<String>();
    let dna_string = swap_replace(&dna_string, "A", "T");
    let dna_string = swap_replace(&dna_string, "G", "C");
    println!("{}", dna_string);
}

fn swap_replace(s: &str, a: &str, b: &str) -> String {
    s = &s.replace(&a, "_");
    s = &s.replace(&b, &a);
    s = &s.replace("_", &b);
    return s.to_string();
}


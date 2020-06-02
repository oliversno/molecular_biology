use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let dna_string = fs::read_to_string(filename)
        .expect("Could not read file");
    let mut dna_string = dna_string.chars().rev().collect::<String>();
    let mut dna_string = swap_replace(&mut dna_string, "A", "T");
    let dna_string = swap_replace(&mut dna_string, "G", "C");
    println!("{}", dna_string);
}

fn swap_replace(s: &mut String, a: &str, b: &str) -> String {
    let s = &mut s.replace(&a, ".");
    let s = &mut s.replace(&b, &a);
    let s = &mut s.replace(".", &b);
    return s.to_string();
}


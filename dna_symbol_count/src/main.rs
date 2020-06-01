use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("filename is {}", filename);
    let dna_string = fs::read_to_string(filename)
        .expect("Could not read file");
    let symbols = count_symbols(dna_string);
    println!("{:?}", symbols);
}

fn count_symbols(dna_string: String) -> (i32, i32, i32, i32) {
    let mut num_a = 0;
    let mut num_c = 0;
    let mut num_g = 0;
    let mut num_t = 0;
    for symbol in dna_string.chars() {
        match symbol {
            'A' => num_a = num_a + 1,
            'C' => num_c = num_c + 1,
            'G' => num_g = num_g + 1,
            'T' => num_t = num_t + 1,
            _ => {
                println!("sybmol did not match A, C, G, T");
                break;
            }
        }
    }
    return (num_a, num_c, num_g, num_t);
}

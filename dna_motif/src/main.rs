use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let dna_string = &args[1];
    let query_string = &args[2];
    let mut v = find_all(dna_string.to_string(), query_string);
    v.sort();
    println!("{:?}", v);
}

fn find_all(mut s: String, q: &str) -> Vec<usize> {
    let mut res = Vec::new();
    loop{
        for elem in s.match_indices(q){
            let index = elem.0;
            res.push(index+1);
        }
        let mut chars: Vec<char> = s.chars().collect();
        for index in &res{
            chars[*index] = '.';
        }
        let s_new: String = chars.into_iter().collect();
        if s_new == s{
            break;
        }
        s = s_new;
    }
    res
}


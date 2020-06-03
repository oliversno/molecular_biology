use std::env;
use std::fs;

fn main(){
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let rna_string = fs::read_to_string(filename)
        .expect("Could not read file");
    let protein = translate(rna_string);
    println!("{}", protein);
}

fn translate(s: String) -> String{
    let mut res = String::new();
    let bytes = s.into_bytes();
    for chunk in bytes.chunks_exact(3){
        let amino_acid = codec(chunk);
        if amino_acid == "STOP"{
            break;
        }
        res.push_str(&amino_acid);
    }
    res
}

fn codec(rna: &[u8]) -> &str{
    assert!(rna.len() == 3);
    let first_char = rna[0];
    let second_char = rna[1];
    let third_char = rna[2];
    if first_char == b'U' && second_char == b'U'{
        if third_char == b'U' || third_char == b'C'{ return "F" }
        return "L"
    }
    if first_char == b'U' && second_char == b'C'{
        return "S"
    }
    if first_char == b'U' && second_char == b'A'{
        if third_char == b'U' || third_char == b'C'{ return "Y" }
        return "STOP"
    }
    if first_char == b'U' && second_char == b'G'{
        if third_char == b'U' || third_char == b'C'{ return "C" }
        if third_char == b'A' {return "STOP" }
        return "W"
    }
    if first_char == b'C' && second_char == b'U'{
        return "L"
    }
    if first_char == b'C' && second_char == b'C'{
        return "P"
    }
    if first_char == b'C' && second_char == b'A'{
        if third_char == b'U' || third_char == b'C'{ return "H" }
        return "Q"
    }
    if first_char == b'C' && second_char == b'G'{
        return "R"
    }
    if first_char == b'A' && second_char == b'U'{
        if third_char == b'G' { return "M" }
        return "I"
    }
    if first_char == b'A' && second_char == b'C'{
        return "T"
    }
    if first_char == b'A' && second_char == b'A'{
        if third_char == b'U' || third_char == b'C'{ return "N" }
        return "K"
    }
    if first_char == b'A' && second_char == b'G'{
        if third_char == b'U' || third_char == b'C'{ return "S" }
        return "R"
    }
    if first_char == b'G' && second_char == b'U'{
        return "V"
    }
    if first_char == b'G' && second_char == b'C'{
        return "A"
    }
    if first_char == b'G' && second_char == b'A'{
        if third_char == b'U' || third_char == b'C'{ return "D" }
        return "E"
    }
    if first_char == b'G' && second_char == b'G'{
        return "G"
    }
    return " "
}
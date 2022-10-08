use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("Hello, world!");
}


fn bnf_format_emojis() {
    let path = "/Users/seb/Documents/KTH/03/INDA/repos/smonten-parser/parser/src/emojis.txt";
    let mut emojis: Vec<String> = vec![];
    if let Ok(lines) = read_lines(path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                emojis.push(ip);
            }
        }
    } else {
        println!("could not read file");
    }

    for (i, emoji) in emojis.iter().enumerate() {
        if i != 0 {
            print!("| ");
        }
        print!("\"{}\" ", emoji);
    }
    println!();


}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
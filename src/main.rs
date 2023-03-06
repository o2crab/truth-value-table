#![allow(dead_code)]

mod alphabet;
mod formula;

use formula::{Alphabet, Formula};

fn main() {
    println!();
    println!("===   TRUTH VALUE TABLE CALCULATOR ===");
    println!();
    println!("Available alphabets:");
    println!("  A, B, ..., Z, t, f, !, &, |, ->, =, (, )");

    loop {
        println!();
        println!();
        println!("enter a formula (alphabets separated by a space), or 'q' to quit:");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("failed to read input");
        if input == "q\n" {
            println!("Bye!");
            break;
        }
        println!();

        let formula = parse_str(&input).expect("failed to parse the sentence");

        print!("{}", formula.truth_value_table());
    }
}


fn parse_str(s: &str) -> Result<Formula, ()> {
    let sentence: Vec<_> = s.split_whitespace().map(|x| Alphabet::try_from(x).expect("invalid alphabet")).collect();

    Formula::parse(&sentence)
}

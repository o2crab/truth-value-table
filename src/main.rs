mod alphabet;

use alphabet::Alphabet;

fn main() {
    let formula = vec![
        Alphabet::Letter('P'),
        Alphabet::Conjunction,
        Alphabet::Letter('Q'),
        Alphabet::Implicature,
        Alphabet::Letter('R'),
    ];

    for x in formula {
        print!("{} ", x);
    }
    println!();
}

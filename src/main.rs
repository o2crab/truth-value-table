mod alphabet;

use alphabet::Alphabet;

fn main() {
    let formula = vec![
        Alphabet::Letter('P'),
        Alphabet::Conjunction,
        Alphabet::Negation,
        Alphabet::Letter('Q'),
        Alphabet::Implicature,
        Alphabet::Letter('R'),
        Alphabet::Disjunction,
        Alphabet::Letter('S'),
    ];

    for x in formula {
        print!("{} ", x);
    }
    println!();

    println!("{}, {}",
        Alphabet::True,
        Alphabet::False,
    );
}

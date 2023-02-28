mod alphabet;
mod formula;

use alphabet::Alphabet;
use formula::Formula;

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

    let v = vec![
        Alphabet::True,
        Alphabet::False,
        Alphabet::Equivalence,
    ];
    for x in v {
        println!("{}", x);
    }

    let formula = Formula::Conjunction(
        Box::new(Formula::Letter('P')),
        Box::new(Formula::Implicature(
            Box::new(Formula::Letter('P')),
            Box::new(Formula::Letter('Q'))
        ))
    );
    println!("{}", formula);

    let formula = Formula::Implicature(
        Box::new(Formula::Conjunction(
            Box::new(Formula::Letter('P')),
            Box::new(Formula::Letter('P'))
        )),
        Box::new(Formula::Letter('Q'))
    );
    println!("{}", formula);
}

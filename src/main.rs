mod alphabet;
mod formula;

use alphabet::Alphabet;
use formula::{Formula, SecondaryFuncName::*};

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

    let formula =
        Formula::SecondaryFunc {
            name: Conjunction,
            lhs: Box::new(Formula::Letter('P')),
            rhs: 
                Box::new(
                    Formula::SecondaryFunc {
                        name: Implicature,
                        lhs: Box::new(Formula::Letter('P')),
                        rhs: Box::new(Formula::Letter('Q'))
                    }
                )
        };
    println!("{}", formula);

    let formula =
        Formula::SecondaryFunc {
            name: Implicature,
            lhs:
                Box::new(
                    Formula::SecondaryFunc {
                        name: Conjunction,
                        lhs: Box::new(Formula::Letter('P')),
                        rhs: Box::new(Formula::Letter('Q'))
                    }
                ),
            rhs: Box::new(Formula::Letter('P'))
        };
    println!("{}", formula);
}

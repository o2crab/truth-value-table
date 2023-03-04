mod alphabet;
mod formula;

use formula::{Formula, SecondaryFuncName::*};

fn main() {
    let formula =
        Formula::SecondaryFunc {
            name: Equivalence,
            lhs:
                Box::new(
                    Formula::SecondaryFunc {
                        name: Conjunction,
                        lhs: Box::new(Formula::Letter('P')),
                        rhs:
                            Box::new(
                                Formula::SecondaryFunc {
                                    name: Conjunction,
                                    lhs: Box::new(Formula::Letter('Q')),
                                    rhs: Box::new(Formula::Letter('R'))
                                }
                            )
                        }
                ),
            rhs: 
                Box::new(
                    Formula::SecondaryFunc {
                        name: Conjunction,
                        lhs:
                            Box::new(
                                Formula::SecondaryFunc {
                                    name: Conjunction,
                                    lhs: Box::new(Formula::Letter('P')),
                                    rhs: Box::new(Formula::Letter('Q'))
                                }
                            ),
                        rhs: Box::new(Formula::Letter('R'))
                    }
                )
        };

    print!("{}", formula.truth_value_table());

    println!();


    let f1 =
    Formula::SecondaryFunc {
        name: Conjunction,
        lhs: Box::new(
            Formula::Negation(Box::new(Formula::Letter('P')))
        ),
        rhs: Box::new(
            Formula::SecondaryFunc {
                name: Disjunction,
                lhs: Box::new(Formula::Letter('P')),
                rhs: Box::new(Formula::Letter('Q'))
            }
        )
    };
    let f2 =
    Formula::Letter('Q');
    let formula =
    Formula::SecondaryFunc {
        name: Implicature,
        lhs: Box::new(f1),
        rhs: Box::new(f2),
    };
    print!("{}", formula.truth_value_table());
}

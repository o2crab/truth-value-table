use std::collections::{HashMap, BTreeSet};
pub use crate::alphabet::{Alphabet, SecondaryFuncName};

#[derive(PartialEq, Eq, Debug)]
pub enum Formula {
    Letter(char),
    True,
    False,
    Negation(Box<Formula>),
    SecondaryFunc {
        name: SecondaryFuncName,
        lhs: Box<Formula>,
        rhs: Box<Formula>,
    },
}

impl std::fmt::Display for Formula {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sentence = self.to_sentence();
        let mut s = String::new();
        for x in sentence {
            s.push_str(&format!("{:2}", x.to_string()));
        }
        write!(f, "{}", s)
    }
}

impl Formula {
    pub fn truth_value_table(&self) -> String {
        let mut letters = BTreeSet::new();
        for x in self.to_sentence() {
            if let Alphabet::Letter(c) = x {
                letters.insert(c);
            }
        }
        let letters: Vec<_> = letters.into_iter().collect();

        let mut header = String::new();
        for c in &letters {
            header.push(*c);
            header.push(' ');
        }
        header.push('|');
        header.push(' ');
        header.push_str(&format!("{}", self));

        let mut table = header;
        table.push('\n');

        let letter_cnt = letters.len();
        for i in 0..1<<letter_cnt {
            let mut interpretation = HashMap::new();
            for j in 0..letter_cnt {
                if i & 1<<(letter_cnt - j - 1) != 0 {
                    interpretation.insert(letters[j], true);
                } else {
                    interpretation.insert(letters[j], false);
                }
            }
            let mut row = String::new();
            for c in &letters {
                let tf =
                    if *interpretation.get(c).unwrap() {
                        'T'
                    } else {
                        'F'
                    };
                row.push(tf);
                row.push(' ');
            }
            row.push('|');
            row.push(' ');
            row.push_str(&self.evaluate(&interpretation).to_string_tf());
            table.push_str(&row);
            table.push('\n');
        }
        table
    }

    pub fn evaluate(&self, interpretation: &HashMap<char, bool>) -> EvalFormula {
        match self {
            Self::Letter(c) => EvalFormula::Letter(*c, *interpretation.get(c).unwrap()),
            Self::True => EvalFormula::True,
            Self::False => EvalFormula::False,
            Self::Negation(f) => {
                let f = f.evaluate(interpretation);
                let truth_value = ! f.is_true();
                EvalFormula::Negation(Box::new(f), truth_value)
            },
            Self::SecondaryFunc { name, lhs, rhs } => {
                let lhs = lhs.evaluate(interpretation);
                let rhs = rhs.evaluate(interpretation);
                let is_true = match name {
                    SecondaryFuncName::Conjunction => lhs.is_true() && rhs.is_true(),
                    SecondaryFuncName::Disjunction => lhs.is_true() || rhs.is_true(),
                    SecondaryFuncName::Implicature => ! (lhs.is_true() && ! rhs.is_true()),
                    SecondaryFuncName::Equivalence => lhs.is_true() == rhs.is_true()
                };
                EvalFormula::SecondaryFunc { name: *name, lhs: Box::new(lhs), rhs: Box::new(rhs), truth_value: is_true }
            }
        }
    }

    fn precedence(&self) -> usize {
        match self {
            Self::SecondaryFunc { name, ..} => name.precedence(),
            Self::Negation(..)      => 4,
            Self::True              => 5,
            Self::False             => 5,
            Self::Letter(..)        => 5, // the highest precedence
        }
    }

    fn to_sentence(&self) -> Vec<Alphabet> {
        match self {
            Self::Letter(c) => vec![Alphabet::Letter(*c)],
            Self::True => vec![Alphabet::True],
            Self::False => vec![Alphabet::False],
            Self::Negation(sub) => {
                if sub.precedence() < self.precedence() {
                    let mut v = vec![
                        Alphabet::Negation,
                        Alphabet::OpenBracket
                        ];
                    let mut sub = sub.to_sentence();
                    v.append(&mut sub);
                    v.push(Alphabet::CloseBracket);
                    v
                } else {
                    let mut v = vec![Alphabet::Negation];
                    let mut sub = sub.to_sentence();
                    v.append(&mut sub);
                    v
                }
            },
            Self::SecondaryFunc { name, lhs, rhs } => {
                let func_symbol =
                    match name {
                        SecondaryFuncName::Conjunction => Alphabet::Conjunction,
                        SecondaryFuncName::Disjunction => Alphabet::Disjunction,
                        SecondaryFuncName::Implicature => Alphabet::Implicature,
                        SecondaryFuncName::Equivalence => Alphabet::Equivalence,
                    };
                let lhs =
                    if lhs.precedence() <= self.precedence() {
                        let mut v = vec![Alphabet::OpenBracket];
                        let mut lhs = lhs.to_sentence();
                        v.append(&mut lhs);
                        v.push(Alphabet::CloseBracket);
                        v
                    } else {
                        lhs.to_sentence()
                    };
                let mut rhs =
                    if rhs.precedence() <= self.precedence() {
                        let mut v = vec![Alphabet::OpenBracket];
                        let mut rhs = rhs.to_sentence();
                        v.append(&mut rhs);
                        v.push(Alphabet::CloseBracket);
                        v
                    } else {
                        rhs.to_sentence()
                    };

                let mut v = lhs;
                v.push(func_symbol);
                v.append(&mut rhs);
                v
            }
        }
    }

    // construct a Formula from an array of alphabets
    pub fn parse(sentence: &[crate::alphabet::Alphabet]) -> Result<Self, ()> {
        let mut pos = 0;

        let (mut formula, bracketed) = Self::get_subformula(&sentence, &mut pos)?;

        while pos < sentence.len() {
            formula =
            match &sentence[pos] {
                Alphabet::SecondaryFunc(name) => {
                    pos += 1;
                    let (rhs, _bracketed) = Self::get_subformula(sentence, &mut pos)?;
                    if bracketed || formula.precedence() > name.precedence() {
                        Formula::SecondaryFunc {
                            name: *name,
                            lhs: Box::new(formula),
                            rhs: Box::new(rhs)
                        }
                    } else if formula.precedence() == name.precedence() {
                        return Err(());
                    } else { // ! bracketed && formula.precedence() < name.precedence. In this case, formula is SecondaryFunc
                        if let Self::SecondaryFunc { name: name0, lhs: lhs0, rhs: rhs0 } = formula {
                            Formula::SecondaryFunc {
                                name: name0,
                                lhs: lhs0,
                                rhs: Box::new(Formula::SecondaryFunc {
                                    name: *name,
                                    lhs: rhs0,
                                    rhs: Box::new(rhs)
                                })
                            }
                        } else {
                            return Err(());
                        }
                    }
                },
                _ => {
                    return Err(());
                }
            };
        }

        Ok(formula)
    }

    fn get_subformula(sentence: &[Alphabet], pos: &mut usize) -> Result<(Self, bool), ()> {
        if *pos >= sentence.len() {
            return Err(());
        }

        match sentence[*pos] {
            Alphabet::Letter(c) => {
                *pos += 1;
                Ok((Self::Letter(c), false))
            },
            Alphabet::True => {
                *pos += 1;
                Ok((Self::True, false))
            },
            Alphabet::False => {
                *pos += 1;
                Ok((Self::False, false))
            },
            Alphabet::Negation => {
                *pos += 1;
                let (subf, _bracketed) = Self::get_subformula(sentence, pos)?;
                Ok((Self::Negation(Box::new(subf)), false))
            },
            Alphabet::OpenBracket => {
                Ok((Self::get_bracketed(sentence, pos)?, true))
            },
            _ => {
                Err(())
            }
        }
    }

    fn get_bracketed(sentence: &[Alphabet], pos: &mut usize) -> Result<Self, ()> {
        if *pos >= sentence.len() {
            return Err(());
        }

        if sentence[*pos] != Alphabet::OpenBracket {
            return Err(());
        }
        *pos += 1;

        let (mut formula, bracketed) = Self::get_subformula(sentence, pos)?;

        loop {
            formula =
            match &sentence[*pos] {
                Alphabet::SecondaryFunc(name) => {
                    *pos += 1;
                    let (rhs, _bracketed) = Self::get_subformula(sentence, pos)?;
                    if bracketed || formula.precedence() > name.precedence() {
                        Formula::SecondaryFunc {
                            name: *name,
                            lhs: Box::new(formula),
                            rhs: Box::new(rhs)
                        }
                    } else { // ! bracketed && formula.precedence() < name.precedence. In this case, formula is SecondaryFunc
                        if let Self::SecondaryFunc { name: name0, lhs: lhs0, rhs: rhs0 } = formula {
                            Formula::SecondaryFunc {
                                name: name0,
                                lhs: lhs0,
                                rhs: Box::new(Formula::SecondaryFunc {
                                    name: *name,
                                    lhs: rhs0,
                                    rhs: Box::new(rhs)
                                })
                            }
                        } else {
                            return Err(());
                        }
                    }
                },
                _ => {
                    return Err(());
                }
            };

            if *pos >= sentence.len() {
                return Err(());
            }

            if sentence[*pos] == Alphabet::CloseBracket {
                *pos += 1;
                break;
            }
        }

        Ok(formula)
    }
}


#[cfg(test)]
mod fmt_tests {
    use super::{Formula::*, SecondaryFuncName::*};

    #[test]
    fn fmt_letter() {
        assert_eq!(
            format!(
                "{}",
                Letter('P')),
            String::from("P")
        );
    }


    #[test]
    fn fmt_true() {
        assert_eq!(
            format!(
                "{}",
                True
            ),
            String::from("⊤")
        );
    }
    #[test]
    fn fmt_false() {
        assert_eq!(
            format!(
                "{}",
                False
            ),
            String::from("⊥")
        );
    }

    #[test]
    fn fmt_negation() {
        assert_eq!(
            format!(
                "{}",
                Negation(
                    Box::new( Letter('P') )
                )
            ),
            String::from("¬ P")
        );
    }

    #[test]
    fn fmt_conjunction() {
        assert_eq!(
            format!(
                "{}",
                SecondaryFunc {
                    name: Conjunction,
                    lhs: Box::new( Letter('P') ),
                    rhs: Box::new( Letter('Q') )}
            ),
            String::from("P ∧ Q")
        );
    }

    #[test]
    fn fmt_disjunction() {
        assert_eq!(
            format!(
                "{}",
                SecondaryFunc {
                    name: Disjunction,
                    lhs: Box::new( Letter('P') ),
                    rhs: Box::new( Letter('Q') )}
            ),
            String::from("P ∨ Q")
        );
    }

    #[test]
    fn fmt_implicature() {
        assert_eq!(
            format!(
                "{}",
                SecondaryFunc {
                    name: Implicature,
                    lhs: Box::new( Letter('P') ),
                    rhs: Box::new( Letter('Q') )}
            ),
            String::from("P → Q")
        );
    }

    #[test]
    fn fmt_equivalence() {
        assert_eq!(
            format!(
                "{}",
                SecondaryFunc {
                    name: Equivalence,
                    lhs: Box::new( Letter('P') ),
                    rhs: Box::new( Letter('Q') )
                }
            ),
            String::from("P ↔ Q")
        );
    }

    #[test]
    fn parentheses_for_same_precedence() {
        assert_eq!(
            format!(
                "{}",
                SecondaryFunc {
                    name: Conjunction,
                    lhs: Box::new( Letter('P') ),
                    rhs: Box::new(
                        SecondaryFunc {
                            name: Conjunction,
                            lhs: Box::new( Letter('Q') ),
                            rhs: Box::new( Letter('R') )
                        }
                    )
                }
            ),
            String::from("P ∧ (Q ∧ R)")
        );
    }

    #[test]
    fn parentheses_for_lower_precedence() {
        // lhs
        assert_eq!(
            format!(
                "{}",
                SecondaryFunc {
                    name: Conjunction,
                    lhs: Box::new(
                        SecondaryFunc {
                            name: Implicature,
                            lhs: Box::new( Letter('P') ),
                            rhs: Box::new( Letter('Q') )
                        }
                    ),
                    rhs: Box::new( Letter('R') )
                }
            ),
            String::from("(P → Q) ∧ R")
        );

        // rhs
        assert_eq!(
            format!(
                "{}",
                SecondaryFunc {
                    name: Conjunction,
                    lhs: Box::new( Letter('P') ),
                    rhs: Box::new(
                        SecondaryFunc {
                            name: Implicature,
                            lhs: Box::new( Letter('Q') ),
                            rhs: Box::new( Letter('R') )
                        }
                    )
                }
            ),
            String::from("P ∧ (Q → R)")
        );
    }

    #[test]
    fn no_parentheses_for_higher_precedence() {
        // lhs
        assert_eq!(
            format!(
                "{}",
                SecondaryFunc {
                    name: Implicature,
                    lhs: Box::new(
                        SecondaryFunc {
                            name: Conjunction,
                            lhs: Box::new( Letter('P') ),
                            rhs: Box::new( Letter('Q') )
                        }
                    ),
                    rhs: Box::new( Letter('R') )
                }
            ),
            String::from("P ∧ Q → R")
        );

        // rhs
        assert_eq!(
            format!(
                "{}",
                SecondaryFunc {
                    name: Implicature,
                    lhs: Box::new( Letter('P') ),
                    rhs: Box::new(
                        SecondaryFunc {
                            name: Conjunction,
                            lhs: Box::new( Letter('Q') ),
                            rhs: Box::new( Letter('R') )
                        }
                    )
                }
            ),
            String::from("P → Q ∧ R")
        );
    }
}

#[cfg(test)]
mod parse_test {
    use super::*;

    #[test]
    fn parse_letters() {
        assert_eq!(
            Formula::parse(&[Alphabet::Letter('P')]),
            Ok(Formula::Letter('P'))
        );
        assert_eq!(
            Formula::parse(&[Alphabet::Letter('Q')]),
            Ok(Formula::Letter('Q'))
        );
    }

    #[test]
    fn parse_truth_values() {
        assert_eq!(
            Formula::parse(&[Alphabet::True]),
            Ok(Formula::True)
        );
        assert_eq!(
            Formula::parse(&[Alphabet::False]),
            Ok(Formula::False)
        );
    }

    #[test]
    fn parse_negation() {
        assert_eq!(
            Formula::parse(&[
                Alphabet::Negation,
                Alphabet::True
                ]),
            Ok(Formula::Negation(
                Box::new(Formula::True)
            ))
        );
    }

    fn parse_secondary_funcs() {
        let names = [
            SecondaryFuncName::Conjunction,
            SecondaryFuncName::Disjunction,
            SecondaryFuncName::Implicature,
            SecondaryFuncName::Equivalence,
        ];

        for name in names {
            assert_eq!(
                Formula::parse(&[
                    Alphabet::Letter('P'),
                    Alphabet::SecondaryFunc(name),
                    Alphabet::Letter('Q'),
                ]),
                Ok(Formula::SecondaryFunc {
                    name,
                    lhs: Box::new(Formula::Letter('P')),
                    rhs: Box::new(Formula::Letter('Q'))
                })
            )
        }
    }

    fn parse_brackets() {
        assert_eq!(
            Formula::parse(&[
                Alphabet::Negation,
                Alphabet::OpenBracket,
                Alphabet::Letter('P'),
                Alphabet::SecondaryFunc(SecondaryFuncName::Implicature),
                Alphabet::Letter('Q'),
                Alphabet::CloseBracket,
            ]),
            Ok(Formula::Negation(
                Box::new(Formula::SecondaryFunc {
                    name: SecondaryFuncName::Implicature,
                    lhs: Box::new(Formula::Letter('P')),
                    rhs: Box::new(Formula::Letter('Q'))
                })
            ))
        )
    }
}

// ∧
// ∨
// →
// ↔


// an evaluated formula
// stores a truth value for each subformula
pub enum EvalFormula {
    Letter(char, bool),
    True,
    False,
    Negation(Box<EvalFormula>, bool),
    SecondaryFunc {
        name: SecondaryFuncName,
        lhs: Box<EvalFormula>,
        rhs: Box<EvalFormula>,
        truth_value: bool,
    },
}

impl std::fmt::Display for EvalFormula {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sentence = self.to_sentence();
        let mut s = (String::new(), String::new());
        for (c, b) in sentence {
            s.0.push_str(&format!("{:2}", c.to_string()));
            let b =
                match b {
                    Some(true) => "T",
                    Some(false) => "F",
                    _ => ""
                };
            s.1.push_str(&format!("{:2}", b));
        }
        writeln!(f, "{}", s.0)?;
        write!(f, "{}", s.1)
    }
}

impl EvalFormula {
    // output only truth values
    pub fn to_string_tf(&self) -> String {
        let sentence = self.to_sentence();
        let mut s = String::new();
        for (_c, b) in sentence {
            let b =
                match b {
                    Some(true) => "T",
                    Some(false) => "F",
                    _ => ""
                };
            s.push_str(&format!("{:2}", b));
        }
        s
    }

    fn is_true(&self) -> bool {
        match self {
            Self::Letter(_, b) => *b,
            Self::True => true,
            Self::False => false,
            Self::Negation(_, b) => *b,
            Self::SecondaryFunc { name: _, lhs: _, rhs: _, truth_value } => *truth_value
        }
    }

    fn to_sentence(&self) -> Vec<(Alphabet, Option<bool>)> {
        let is_true = self.is_true();
        match self {
            Self::Letter(c, _) => vec![(Alphabet::Letter(*c), Some(is_true))],
            Self::True => vec![(Alphabet::True, Some(is_true))],
            Self::False => vec![(Alphabet::False, Some(is_true))],
            Self::Negation(sub, _) => {
                if sub.precidence() < self.precidence() {
                    let mut v = vec![
                        (Alphabet::Negation, Some(is_true)),
                        (Alphabet::OpenBracket, None)
                        ];
                    let mut sub = sub.to_sentence();
                    v.append(&mut sub);
                    v.push((Alphabet::CloseBracket, None));
                    v
                } else {
                    let mut v = vec![(Alphabet::Negation, Some(is_true))];
                    let mut sub = sub.to_sentence();
                    v.append(&mut sub);
                    v
                }
            },
            Self::SecondaryFunc { name, lhs, rhs, truth_value: _ } => {
                let func_symbol =
                    match name {
                        SecondaryFuncName::Conjunction => Alphabet::Conjunction,
                        SecondaryFuncName::Disjunction => Alphabet::Disjunction,
                        SecondaryFuncName::Implicature => Alphabet::Implicature,
                        SecondaryFuncName::Equivalence => Alphabet::Equivalence,
                    };
                let lhs =
                    if lhs.precidence() <= self.precidence() {
                        let mut v = vec![(Alphabet::OpenBracket, None)];
                        let mut lhs = lhs.to_sentence();
                        v.append(&mut lhs);
                        v.push((Alphabet::CloseBracket, None));
                        v
                    } else {
                        lhs.to_sentence()
                    };
                let mut rhs =
                    if rhs.precidence() <= self.precidence() {
                        let mut v = vec![(Alphabet::OpenBracket, None)];
                        let mut rhs = rhs.to_sentence();
                        v.append(&mut rhs);
                        v.push((Alphabet::CloseBracket, None));
                        v
                    } else {
                        rhs.to_sentence()
                    };
                let mut v = lhs;
                v.push((func_symbol, Some(is_true)));
                v.append(&mut rhs);
                v
            }
        }
    }

    fn precidence(&self) -> usize {
        match self {
            Self::SecondaryFunc { name, ..} => {
                match name {
                    SecondaryFuncName::Equivalence => 1, // the lowest precedence
                    SecondaryFuncName::Implicature => 2,
                    SecondaryFuncName::Conjunction => 3,
                    SecondaryFuncName::Disjunction => 3,
                }
            },
            Self::Negation(..)      => 4,
            Self::True              => 5,
            Self::False             => 5,
            Self::Letter(..)        => 5, // the highest precedence
        }
    }
}

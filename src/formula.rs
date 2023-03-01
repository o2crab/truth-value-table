pub enum SecondaryFuncName {
    Conjunction,
    Disjunction,
    Implicature,
    Equivalence,
}

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
        match self {
            Self::Letter(c)             => write!(f, "{}", c),
            Self::True                  => write!(f, "⊤"), // ⊤; \u{22A4}
            Self::False                 => write!(f, "⊥"), // ⊥: \u{22A5}
            Self::Negation(f1)          => write!(f, "¬ {}", *f1), // ¬: \u{00AC}
            Self::SecondaryFunc { name, lhs, rhs } => {
                let func_symbol =
                    match name {
                        SecondaryFuncName::Conjunction => "∧", // ∧: \u{2227}
                        SecondaryFuncName::Disjunction => "∨", // ∨: \u{2228}
                        SecondaryFuncName::Implicature => "→", // →: \u{2192}
                        SecondaryFuncName::Equivalence => "↔", // ↔: \u{2194}
                    };
                let lhs =
                    if lhs.precidence() <= self.precidence() {
                        format!("({})", *lhs)
                    } else {
                        lhs.to_string()
                    };
                let rhs =
                    if rhs.precidence() <= self.precidence() {
                        format!("({})", *rhs)
                    } else {
                        rhs.to_string()
                    };
                write!(f, "{} {} {}", lhs, func_symbol, rhs)
            },
        }
    }
}

impl Formula {
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

// ∧
// ∨
// →
// ↔
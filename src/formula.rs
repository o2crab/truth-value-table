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
                    if lhs.precidence() < self.precidence() {
                        format!("({})", *lhs)
                    } else {
                        lhs.to_string()
                    };
                let rhs =
                    if rhs.precidence() < self.precidence() {
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

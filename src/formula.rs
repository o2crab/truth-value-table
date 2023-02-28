pub enum Formula {
    Letter(char),
    True,
    False,
    Negation(Box<Formula>),
    Conjunction(Box<Formula>, Box<Formula>),
    Disjunction(Box<Formula>, Box<Formula>),
    Implicature(Box<Formula>, Box<Formula>),
    Equivalence(Box<Formula>, Box<Formula>),
}

impl std::fmt::Display for Formula {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Letter(c)             => write!(f, "{}", c),
            Self::True                  => write!(f, "⊤"), // \u{22A4}
            Self::False                 => write!(f, "⊥"), // \u{22A5}
            Self::Negation(f1)          => write!(f, "¬ {}", *f1), // \u{00AC}
            Self::Conjunction(f1, f2)   => write!(f, "{} ∧ {}", *f1, *f2), // \u{2227}
            Self::Disjunction(f1, f2)   => write!(f, "{} ∨ {}", *f1, *f2), // \u{2228}
            Self::Implicature(f1, f2)   => write!(f, "{} → {}", *f1, *f2), // \u{2192}
            Self::Equivalence(f1, f2)   => write!(f, "{} ↔ {}", *f1, *f2), // \u{2194}
        }
    }
}

pub enum Alphabet {
    Letter(char),
    True,
    False,
    Negation,
    Conjunction,
    Disjunction,
    Implicature,
    Equivalence,
}

impl std::fmt::Display for Alphabet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Alphabet::Letter(c)     => write!(f, "{}", c),
            Alphabet::True          => write!(f, "⊤"), // \u{22A4}
            Alphabet::False         => write!(f, "⊥"), // \u{22A5}
            Alphabet::Negation      => write!(f, "¬"), // \u{00AC}
            Alphabet::Conjunction   => write!(f, "∧"), // \u{2227}
            Alphabet::Disjunction   => write!(f, "∨"), // \u{2228}
            Alphabet::Implicature   => write!(f, "→"), // \u{2192}
            Alphabet::Equivalence   => write!(f, "↔"), // \u{2194}
        }
    }
}

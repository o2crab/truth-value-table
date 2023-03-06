// enum Alphabet represents the alphabet of propositional logic.
// Letter represents a propositional letter.
pub enum Alphabet {
    OpenBracket,
    CloseBracket,
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
            Self::OpenBracket   => write!(f, "("),
            Self::CloseBracket  => write!(f, ")"),
            Self::Letter(c)     => write!(f, "{}", c),
            Self::True          => write!(f, "⊤"), // \u{22A4}
            Self::False         => write!(f, "⊥"), // \u{22A5}
            Self::Negation      => write!(f, "¬"), // \u{00AC}
            Self::Conjunction   => write!(f, "∧"), // \u{2227}
            Self::Disjunction   => write!(f, "∨"), // \u{2228}
            Self::Implicature   => write!(f, "→"), // \u{2192}
            Self::Equivalence   => write!(f, "↔"), // \u{2194}
        }
    }
}

impl std::convert::TryFrom<&str> for Alphabet {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "(" => Ok( Self::OpenBracket ),
            ")" => Ok( Self::CloseBracket ),
            "t" => Ok( Self::True ),
            "f" => Ok( Self::False ),
            "!" => Ok( Self::Negation ),
            "&" => Ok( Self::Conjunction ),
            "|" => Ok( Self::Disjunction ),
            "->" => Ok( Self::Implicature ),
            "=" => Ok( Self::Equivalence ),
            x if
                x.chars().count() == 1 &&
                x.chars().next().unwrap().is_uppercase()
                => {
                    Ok( Self::Letter(x.chars().next().unwrap()))
            },
            _ => Err("Not an alphabet")
        }
    }
}

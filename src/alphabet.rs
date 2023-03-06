#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum SecondaryFuncName {
    Conjunction,
    Disjunction,
    Implicature,
    Equivalence,
}

impl SecondaryFuncName {
    pub fn precedence(&self) -> usize {
        match self {
            Self::Equivalence => 1,
            Self::Implicature => 2,
            Self::Conjunction => 3,
            Self::Disjunction => 3,
        }
    }
}

// enum Alphabet represents the alphabet of propositional logic.
// Letter represents a propositional letter.
#[derive(PartialEq, Eq)]
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
    SecondaryFunc(SecondaryFuncName),
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
            Self::SecondaryFunc(name) => {
                let s =
                match name {
                    SecondaryFuncName::Conjunction => "∧", // \u{2227}
                    SecondaryFuncName::Disjunction => "∨", // \u{2228}
                    SecondaryFuncName::Implicature => "→", // \u{2192}
                    SecondaryFuncName::Equivalence => "↔", // \u{2194}
                };
                write!(f, "{}", s)
            },
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
            "&" => Ok( Self::SecondaryFunc(SecondaryFuncName::Conjunction)),
            "|" => Ok( Self::SecondaryFunc(SecondaryFuncName::Disjunction)),
            "->" => Ok( Self::SecondaryFunc(SecondaryFuncName::Implicature)),
            "=" => Ok( Self::SecondaryFunc(SecondaryFuncName::Equivalence)),
            // "&" => Ok( Self::Conjunction ),
            // "|" => Ok( Self::Disjunction ),
            // "->" => Ok( Self::Implicature ),
            // "=" => Ok( Self::Equivalence ),
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

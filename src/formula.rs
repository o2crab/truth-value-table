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

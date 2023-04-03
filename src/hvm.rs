// Enum for representing the different types of terms
#[derive(Debug, Clone)]
pub enum Term {
    Lambda(String, Box<Term>),              // λvar. term
    Application(Box<Term>, Box<Term>),      // (term term)
    Constructor(String, Vec<Term>),         // (ctr term term ...)
    Integer(i32),                           // num (machine int)
    Operator(String, Box<Term>, Box<Term>), // (op2 term term)
    Let(String, Box<Term>, Box<Term>),      // let var = term; term
}

// Struct for representing a rule
#[derive(Debug, Clone)]
pub struct Rule {
    lhs: Term,
    rhs: Term,
}

// Struct for representing a file
#[derive(Debug, Clone)]
pub struct File {
    rules: Vec<Rule>,
}


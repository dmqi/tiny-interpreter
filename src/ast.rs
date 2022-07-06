#[derive(Debug, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Var(String, Expression), 
    Func(String, Expression),
    Expression(Expression),
}

// The expression represents for a simple computation 
// "a - b" or "2 * (1 + 3)" or "f(a, b) + 2"
#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Ident(String), // a variable or a function
    Integer(i32), 
    Float(f64),
    Operation(Op, Vec<Expression>),
    // The prev stores definition, the last stores variables
    Call(Box<Expression>, Vec<Expression>) 
}

#[derive(Debug, PartialEq, Clone)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

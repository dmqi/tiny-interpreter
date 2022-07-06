#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Illegal,
    End,

    Ident(String),
    Integer(String),
    Function,
    Var,

    // Operators 
    Assign, // :=
    Define, // =
    Add,
    Sub,
    Mul,
    Div,

    // Delimiters
    Comma, 
    LeftBracket,
    RightBracket,
}

impl Default for Token {
    fn default() -> Self {
        return Token::Illegal;
    }
}

pub fn lookup(ident: &str) -> Token {
    match ident {
        "fun" => Token::Function,
        "var" => Token::Var,
        _ => Token::Ident(ident.to_string()),
    }
}

#[test]
fn test_lookup() {
    assert_eq!(lookup("fun"), Token::Function);
    assert_eq!(lookup("test"), Token::Ident("test".to_string()));
}

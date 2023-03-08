// Parser gets the token and generates the AST

// Path: src/lexer.rs

enum TokenType {
    ILLEGAL,
    EOF,
    IDENT,
    INT,
    ASSIGN,
    PLUS,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    FUNCTION,
    LET,
}

fn main() {
    let mut lexer = Lexer::new("let x = 5;".to_string());
    let mut token = lexer.next_token();
    while token.token_type != TokenType::EOF {
        println!("{:?}", token);
        token = lexer.next_token();
    }
}
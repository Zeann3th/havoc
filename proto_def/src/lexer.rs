use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[token("message")]
    Message,

    #[token("service")]
    Service,

    #[token("rpc")]
    Rpc,

    #[token("returns")]
    Returns,

    #[token("option")]
    Option,

    #[token("repeated")]
    Repeated,

    #[token(".")]
    Dot,

    #[token(",")]
    Comma,

    #[token("package")]
    Package,

    #[token("import")]
    Import,

    #[token("syntax")]
    Syntax,

    #[regex(r#""([^"\\]|\\.)*""#, |lex| {
        let slice = lex.slice();
        Some(slice[1..slice.len() - 1].to_string())
    })]
    Literal(String),

    #[token("int32")]
    Int32,

    #[token("int64")]
    Int64,

    #[token("uint32")]
    Uint32,

    #[token("uint64")]
    Uint64,

    #[token("sint32")]
    Sint32,

    #[token("sint64")]
    Sint64,

    #[token("fixed32")]
    Fixed32,

    #[token("fixed64")]
    Fixed64,

    #[token("sfixed32")]
    Sfixed32,

    #[token("sfixed64")]
    Sfixed64,

    #[token("bool")]
    Bool,

    #[token("string")]
    Str,

    #[token("bytes")]
    Bytes,

    #[token("double")]
    Double,

    #[token("float")]
    Float,

    #[regex(r"[0-9]+", |lex| lex.slice().parse().ok())]
    Number(u32),

    #[token(";")]
    Semicolon,

    #[token("{")]
    LeftBrace,

    #[token("}")]
    RightBrace,

    #[token("(")]
    LeftParen,

    #[token(")")]
    RightParen,

    #[token("=")]
    Equal,

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| Some(lex.slice().to_string()))]
    Identifier(String),

    #[regex(r"[ \t\n\r\f]+", logos::skip)]
    #[regex(r"//[^\n]*", logos::skip)]
    #[regex(r"/\*([^*]|\*[^/])*\*/", logos::skip)]
    Error,
}

pub struct Lexer;

impl Lexer {
    pub fn new() -> Self {
        Lexer
    }

    pub fn lex(&self, input: &str) -> Result<Vec<Token>, String> {
        let mut lexer = Token::lexer(input);
        let mut tokens = Vec::new();

        while let Some(result) = lexer.next() {
            match result {
                Ok(token) => tokens.push(token),
                Err(_) => {
                    let span = lexer.span();
                    let slice = &input[span.clone()];
                    return Err(format!(
                        "Failed to tokenize proto file at {:?}: '{}'",
                        span, slice
                    ));
                }
            }
        }

        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_keywords() {
        let input = r#"package my.pkg; import "auth.proto"; option java_package = "com.example";"#;
        let lexer = Lexer::new();
        let tokens = lexer.lex(input).unwrap();

        let expected = vec![
            Token::Package,
            Token::Identifier("my".into()),
            Token::Dot,
            Token::Identifier("pkg".into()),
            Token::Semicolon,
            Token::Import,
            Token::Literal("auth.proto".into()),
            Token::Semicolon,
            Token::Option,
            Token::Identifier("java_package".into()),
            Token::Equal,
            Token::Literal("com.example".into()),
            Token::Semicolon,
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_tokenize_message_block() {
        let input = r#"message User { string name = 1; int32 age = 2; }"#;
        let lexer = Lexer::new();
        let tokens = lexer.lex(input).unwrap();

        let expected = vec![
            Token::Message,
            Token::Identifier("User".into()),
            Token::LeftBrace,
            Token::Str,
            Token::Identifier("name".into()),
            Token::Equal,
            Token::Number(1),
            Token::Semicolon,
            Token::Int32,
            Token::Identifier("age".into()),
            Token::Equal,
            Token::Number(2),
            Token::Semicolon,
            Token::RightBrace,
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_tokenize_service_rpc() {
        let input = r#"service Auth { rpc Login (LoginRequest) returns (LoginResponse); }"#;
        let lexer = Lexer::new();
        let tokens = lexer.lex(input).unwrap();

        let expected = vec![
            Token::Service,
            Token::Identifier("Auth".into()),
            Token::LeftBrace,
            Token::Rpc,
            Token::Identifier("Login".into()),
            Token::LeftParen,
            Token::Identifier("LoginRequest".into()),
            Token::RightParen,
            Token::Returns,
            Token::LeftParen,
            Token::Identifier("LoginResponse".into()),
            Token::RightParen,
            Token::Semicolon,
            Token::RightBrace,
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_tokenize_with_repeated_fields() {
        let input = r#"message Post { repeated string tags = 1; }"#;
        let lexer = Lexer::new();
        let tokens = lexer.lex(input).unwrap();

        let expected = vec![
            Token::Message,
            Token::Identifier("Post".into()),
            Token::LeftBrace,
            Token::Repeated,
            Token::Str,
            Token::Identifier("tags".into()),
            Token::Equal,
            Token::Number(1),
            Token::Semicolon,
            Token::RightBrace,
        ];

        assert_eq!(tokens, expected);
    }
}

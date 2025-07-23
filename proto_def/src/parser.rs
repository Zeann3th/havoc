use std::collections::HashMap;

use crate::{
    lexer::Token,
    model::{Field, Message, Proto, RpcMethod, Service},
};

pub struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn next(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.pos);
        if token.is_some() {
            self.pos += 1;
        }
        token
    }

    fn expect(&mut self, expected: &Token) -> Result<(), String> {
        match self.next() {
            Some(t) if t == expected => Ok(()),
            _ => Err(format!(
                "Expected {:?}, found {:?} at position {}",
                expected,
                self.peek(),
                self.pos
            )),
        }
    }

    fn parse_dotted_identifier(&mut self) -> Result<String, String> {
        let mut parts = Vec::new();

        match self.next() {
            Some(Token::Identifier(name)) => parts.push(name.clone()),
            _ => return Err("Expected identifier".into()),
        }

        while let Some(Token::Dot) = self.peek() {
            self.next();
            match self.next() {
                Some(Token::Identifier(name)) => parts.push(name.clone()),
                _ => return Err("Expected identifier after '.'".into()),
            }
        }

        Ok(parts.join("."))
    }

    pub fn parse(&mut self) -> Result<Proto, String> {
        let mut content = Proto::default();

        while let Some(token) = self.peek() {
            match token {
                Token::Package => {
                    self.next();
                    content.package = Some(self.parse_dotted_identifier()?);
                    self.expect(&Token::Semicolon)?;
                }
                Token::Syntax => {
                    self.next();
                    self.expect(&Token::Equal)?;
                    if let Some(Token::Literal(value)) = self.next() {
                        content.syntax = Some(value.clone());
                    } else {
                        return Err("Expected string literal after 'syntax ='".into());
                    }
                    self.expect(&Token::Semicolon)?;
                }
                Token::Import => {
                    self.next();
                    if let Some(Token::Literal(name)) = self.next() {
                        content.imports.push(name.clone());
                    }
                    self.expect(&Token::Semicolon)?;
                }
                Token::Option => {
                    self.next();
                    if let Some(Token::Identifier(name)) = self.next() {
                        let name = name.clone();
                        self.expect(&Token::Equal)?;
                        if let Some(Token::Literal(value)) = self.next() {
                            content.options.insert(name, value.clone());
                        } else {
                            return Err("Expected option value after '='".into());
                        }
                        self.expect(&Token::Semicolon)?;
                    }
                }
                Token::Message => {
                    content.messages.push(self.parse_message()?);
                }
                Token::Service => {
                    content.services.push(self.parse_service()?);
                }
                _ => return Err(format!("Unexpected token: {:?}", token)),
            }
        }

        Ok(content)
    }

    fn parse_message(&mut self) -> Result<Message, String> {
        self.expect(&Token::Message)?;
        let name = match self.next() {
            Some(Token::Identifier(n)) => n.clone(),
            _ => return Err("Expected identifier after 'message'".into()),
        };
        self.expect(&Token::LeftBrace)?;

        let mut fields = Vec::new();
        while let Some(token) = self.peek() {
            match token {
                Token::RightBrace => {
                    self.next();
                    break;
                }
                Token::Identifier(_)
                | Token::Repeated
                | Token::Int32
                | Token::Str
                | Token::Bool
                | Token::Int64
                | Token::Uint32
                | Token::Uint64
                | Token::Bytes
                | Token::Double
                | Token::Float
                | Token::Sfixed32
                | Token::Sfixed64
                | Token::Fixed32
                | Token::Fixed64
                | Token::Sint32
                | Token::Sint64 => {
                    fields.push(self.parse_field()?);
                }
                _ => return Err(format!("Unexpected token in message: {:?}", token)),
            }
        }

        Ok(Message { name, fields })
    }

    fn parse_field(&mut self) -> Result<Field, String> {
        let mut repeated = false;
        if let Some(Token::Repeated) = self.peek() {
            self.next();
            repeated = true;
        }

        let field_type = match self.next() {
            Some(Token::Str) => "string".to_string(),
            Some(Token::Bool) => "bool".to_string(),
            Some(Token::Int32) => "int32".to_string(),
            Some(Token::Int64) => "int64".to_string(),
            Some(Token::Uint32) => "uint32".to_string(),
            Some(Token::Uint64) => "uint64".to_string(),
            Some(Token::Sfixed32) => "sfixed32".to_string(),
            Some(Token::Sfixed64) => "sfixed64".to_string(),
            Some(Token::Double) => "double".to_string(),
            Some(Token::Float) => "float".to_string(),
            Some(Token::Bytes) => "bytes".to_string(),
            Some(Token::Identifier(t)) => t.clone(),
            Some(other) => return Err(format!("Unexpected field type: {:?}", other)),
            None => return Err("Unexpected end of input while reading field type".to_string()),
        };

        let name = match self.next() {
            Some(Token::Identifier(n)) => n.clone(),
            Some(other) => return Err(format!("Expected field name, got {:?}", other)),
            None => return Err("Unexpected end of input while reading field name".to_string()),
        };

        match self.next() {
            Some(Token::Equal) => {}
            Some(other) => return Err(format!("Expected '=', got {:?}", other)),
            None => return Err("Unexpected end of input; expected '='".to_string()),
        }

        let number = match self.next() {
            Some(Token::Number(n)) => *n,
            Some(other) => return Err(format!("Expected field number, got {:?}", other)),
            None => return Err("Unexpected end of input; expected field number".to_string()),
        };

        match self.next() {
            Some(Token::Semicolon) => {}
            Some(other) => return Err(format!("Expected ';', got {:?}", other)),
            None => return Err("Unexpected end of input; expected ';'".to_string()),
        }

        Ok(Field {
            name,
            number,
            field_type,
            repeated,
        })
    }

    fn parse_service(&mut self) -> Result<Service, String> {
        self.expect(&Token::Service)?;
        let name = match self.next() {
            Some(Token::Identifier(n)) => n.clone(),
            _ => return Err("Expected identifier after 'service'".into()),
        };
        self.expect(&Token::LeftBrace)?;

        let mut methods = HashMap::new();
        while let Some(token) = self.peek() {
            match token {
                Token::RightBrace => {
                    self.next();
                    break;
                }
                Token::Rpc => {
                    let method = self.parse_rpc()?;
                    if methods.contains_key(&method.name) {
                        return Err(format!("Duplicate RPC method: {}", method.name));
                    }
                    methods.insert(method.name.clone(), method);
                }
                _ => return Err(format!("Unexpected token in service: {:?}", token)),
            }
        }

        Ok(Service { name, methods })
    }

    fn parse_rpc(&mut self) -> Result<RpcMethod, String> {
        self.expect(&Token::Rpc)?;
        let name = match self.next() {
            Some(Token::Identifier(n)) => n.clone(),
            _ => return Err("Expected identifier after 'rpc'".into()),
        };

        self.expect(&Token::LeftParen)?;
        let req_type = match self.next() {
            Some(Token::Identifier(t)) => t.clone(),
            _ => return Err("Expected request type".into()),
        };
        self.expect(&Token::RightParen)?;

        self.expect(&Token::Returns)?;
        self.expect(&Token::LeftParen)?;
        let res_type = match self.next() {
            Some(Token::Identifier(t)) => t.clone(),
            _ => return Err("Expected response type".into()),
        };
        self.expect(&Token::RightParen)?;
        self.expect(&Token::Semicolon)?;

        Ok(RpcMethod {
            name,
            request: req_type,
            response: res_type,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        lexer::Lexer,
        model::{Field, Proto, RpcMethod},
        parser::Parser,
    };

    fn parse_input(input: &str) -> Proto {
        let lexer = Lexer::new();
        let tokens = lexer.lex(input).expect("Lexing failed");
        let mut parser = Parser::new(&tokens);
        parser.parse().expect("Parsing failed")
    }

    #[test]
    fn test_parse_package_and_import() {
        let proto = parse_input(
            r#"
            package my.test;
            import "common.proto";
        "#,
        );

        assert_eq!(proto.package, Some("my.test".to_string()));
        assert_eq!(proto.imports, vec!["common.proto"]);
    }

    #[test]
    fn test_parse_option() {
        let proto = parse_input(
            r#"
            option java_package = "com.example";
        "#,
        );

        assert_eq!(
            proto.options.get("java_package"),
            Some(&"com.example".to_string())
        );
    }

    #[test]
    fn test_parse_message_with_fields() {
        let proto = parse_input(
            r#"
            message User {
                string name = 1;
                int32 age = 2;
                repeated string tags = 3;
            }
        "#,
        );

        assert_eq!(proto.messages.len(), 1);
        let msg = &proto.messages[0];
        assert_eq!(msg.name, "User");
        assert_eq!(msg.fields.len(), 3);
        assert_eq!(
            msg.fields[0],
            Field {
                name: "name".into(),
                field_type: "string".into(),
                number: 1,
                repeated: false,
            }
        );
        assert_eq!(
            msg.fields[2],
            Field {
                name: "tags".into(),
                field_type: "string".into(),
                number: 3,
                repeated: true,
            }
        );
    }

    #[test]
    fn test_parse_service_with_rpc() {
        let proto = parse_input(
            r#"
            service AuthService {
                rpc Login (LoginRequest) returns (LoginResponse);
                rpc Logout (LogoutRequest) returns (LogoutResponse);
            }
        "#,
        );

        assert_eq!(proto.services.len(), 1);
        let svc = &proto.services[0];
        assert_eq!(svc.name, "AuthService");
        assert_eq!(svc.methods.len(), 2);
        assert_eq!(
            svc.methods.get("Login"),
            Some(&RpcMethod {
                name: "Login".into(),
                request: "LoginRequest".into(),
                response: "LoginResponse".into(),
            })
        );
    }

    #[test]
    fn test_parse_full_proto() {
        let proto = parse_input(
            r#"
            syntax = "proto3";
            package test;

            import "common.proto";

            option go_package = "github.com/example/project";

            message Person {
                string name = 1;
                int32 age = 2;
            }

            service Greeter {
                rpc SayHello (HelloRequest) returns (HelloResponse);
            }
        "#,
        );

        assert_eq!(proto.package, Some("test".to_string()));
        assert_eq!(proto.imports, vec!["common.proto"]);
        assert_eq!(
            proto.options.get("go_package"),
            Some(&"github.com/example/project".to_string())
        );
        assert_eq!(proto.messages.len(), 1);
        assert_eq!(proto.services.len(), 1);
    }
}

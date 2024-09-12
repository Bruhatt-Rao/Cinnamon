use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Openparen,
    Closeparen,
    Openquote,
    Closequote,
    Integer,
    Assign,
    Binaryoper,
    Identifier,
    Equals,
    Eof,
}

#[derive(Debug)]
pub struct Token {
    pub value: String,
    pub ty: TokenType,
}

impl Token {
    fn new(v: String, ty: TokenType) -> Token {
        let value = v.replace('"', "");
        Token { value, ty }
    }
}

pub fn parse_config(args: &[String]) -> &str {
    let file_path = &args[1];
    file_path
}

pub fn tokenize(program: String) -> Vec<Token> {
    let mut keywords = HashMap::new();
    keywords.insert("var", TokenType::Assign);

    let mut lines: Vec<String> = program.split("").map(|s| s.to_string()).collect();
    lines.remove(0);
    lines.remove(lines.len()-1);
    let mut tokens = vec![];

    while lines.len() > 0 {
        let mut l = lines[0].clone();
        if l == "(" {
            let t = Token::new(l, TokenType::Openparen);
            tokens.push(t);
            lines.remove(0);
        } else if l == ")" {
            let t = Token::new(l, TokenType::Closeparen);
            tokens.push(t);
            lines.remove(0);
        } else if l == "+" || l == "-" || l == "*" || l == "/" {
            let t = Token::new(l, TokenType::Binaryoper);
            tokens.push(t);
            lines.remove(0);
        } else if l == "=" {
            let t = Token::new(l, TokenType::Equals);
            tokens.push(t);
            lines.remove(0);
        } else {
            let c = l.chars().next().expect("string is empty");
            if c.is_numeric() {
                let mut num = "".to_string();
                while lines.len() > 0 && c.is_numeric() {
                    l = lines[0].clone();
                    let c = l.chars().next().expect("string is empty");
                    if !c.is_numeric() {
                        break;
                    }
                    num.push_str(&l);
                    lines.remove(0);
                }
                let t = Token::new(num.to_string(), TokenType::Integer);
                tokens.push(t);

            } else if c.is_alphanumeric() {
                let mut iden = "".to_string();
                while lines.len() > 0 {
                    l = lines[0].clone();
                    let c = l.chars().next().expect("string is empty");
                    if !c.is_alphanumeric() {
                        break;
                    }
                    iden.push_str(&l);
                    lines.remove(0);
                }
                if keywords.contains_key(&iden[..]) {
                    let t = Token::new(iden.clone(), keywords.get(&iden[..]).expect("REASON").clone());
                    tokens.push(t);
                } else {
                    let t = Token::new(iden.to_owned(), TokenType::Identifier);
                    tokens.push(t);
                }
            } else if l == " " || l == "\n" || l == "\r" || l == "\t" {
                println!("removed: {:?}", l);
                lines.remove(0);
            } else {
                println!("Unknown Character: {:?}", l);
                lines.remove(0);
            }
        }
    }

    let t = Token::new("EOF".to_string(), TokenType::Eof);
    tokens.push(t);
    println!("{:?}", tokens);
    tokens
}
#[derive(Debug)]
pub enum TokenKind {
    Return,
    Int,
    Semi,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub value: Option<String>,
}

pub fn lex(contents: &str) -> Vec<Token> {
    let mut buf: String = String::new();
    let mut tokens: Vec<Token> = vec![];

    let mut i = 0;
    for _ in 0..contents.len() {
        if i >= contents.len() {
            break;
        }

        let c = contents.chars().nth(i).unwrap();
        if c.is_alphabetic() {
            buf.push(c);
            i += 1;

            while contents.chars().nth(i).unwrap().is_alphanumeric() {
                buf.push(contents.chars().nth(i).unwrap());
                i += 1;
            }

            i -= 1;

            match buf.as_str() {
                "return" => tokens.push(Token {
                    kind: TokenKind::Return,
                    value: Some(buf.clone()),
                }),
                _ => {
                    eprintln!("Unexpected token: {}", buf);
                    buf.clear();
                    std::process::exit(1);
                }
            }

            buf.clear();
        } else if c.is_numeric() {
            buf.push(c);
            i += 1;

            while contents.chars().nth(i).unwrap().is_numeric() {
                buf.push(contents.chars().nth(i).unwrap());
                i += 1;
            }

            i -= 1;

            tokens.push(Token {
                kind: TokenKind::Int,
                value: Some(buf.clone()),
            });

            buf.clear();
        } else if c == ';' {
            tokens.push(Token {
                kind: TokenKind::Semi,
                value: None,
            });

            buf.clear();
        } else {
            eprintln!("Unexpected token: {} at index {}", c, i);
            buf.clear();
            //std::process::exit(1);
        }

        i += 1;
    }

    tokens
}

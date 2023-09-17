use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Token {
    Number(i32),
    Comma,
    Lparen,
    Rparen,
    Plus,
    Hyphen,
    Star,
    Fslash,
    Ident(String),
    Ignore,
    _Illegal,
}

pub struct Lexer<'a> {
    chars: Chars<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            chars: input.chars(),
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(token) = self.next_token() {
            // Dont push ignored tokens
            if token == Token::Ignore {
                continue;
            }

            tokens.push(token)
        }
        tokens
    }

    fn next_token(&mut self) -> Option<Token> {
        let next_char = self.chars.next()?;

        match next_char {
            '(' => Some(Token::Lparen),
            ')' => Some(Token::Rparen),
            ',' => Some(Token::Comma),
            '0'..='9' => {
                let mut num = next_char.to_digit(10)? as i32;

                while let Some(next_char) = self.chars.clone().next() {
                    if let Some(digit) = next_char.to_digit(10) {
                        num = num * 10 + digit as i32;
                        self.chars.next();
                    } else {
                        break;
                    }
                }

                Some(Token::Number(num))
            }
            'a'..='z' => {
                let mut func_name = String::from(next_char);

                while let Some(next_char) = self.chars.clone().next() {
                    if next_char.is_alphabetic() {
                        func_name.push(next_char);
                        self.chars.next();
                    } else {
                        break;
                    }
                }

                match func_name.as_str() {
                    "avg" => Some(Token::Ident("avg".to_string())),
                    "diff" => Some(Token::Ident("diff".to_string())),
                    // Else it is probably an identifier
                    _ => Some(Token::Ident(func_name)),
                }
            }
            // MATH
            '+' => Some(Token::Plus),
            '-' => Some(Token::Hyphen),
            '*' => Some(Token::Star),
            '/' => Some(Token::Fslash),
            // SPACES AND UNDERSCORES
            ' ' => Some(Token::Ignore),
            '\n' => Some(Token::Ignore),

            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lexer_avg() {
        let mut lexer = Lexer::new("avg(1,2,3)");
        let tokens = lexer.tokenize();

        assert_eq!(
            tokens,
            vec![
                Token::Ident("avg".to_string()),
                Token::Lparen,
                Token::Number(1),
                Token::Comma,
                Token::Number(2),
                Token::Comma,
                Token::Number(3),
                Token::Rparen,
            ]
        );
    }
}

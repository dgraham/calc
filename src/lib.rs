#[derive(Debug, PartialEq)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Number(char),
    Operator(Op),
    Whitespace,
    Unrecognized(char),
}

pub fn scan(text: &str) -> Vec<Token> {
    text.chars()
        .map(|ch| {
            match ch {
                '0'...'9' => Token::Number(ch),
                '+' => Token::Operator(Op::Add),
                '-' => Token::Operator(Op::Subtract),
                '*' => Token::Operator(Op::Multiply),
                '/' => Token::Operator(Op::Divide),
                ' ' => Token::Whitespace,
                _ => Token::Unrecognized(ch),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{scan, Token, Op};

    #[test]
    fn it_scans() {
        let tokens = scan("1 + 2");
        assert_eq!(5, tokens.len());
        assert_eq!(vec![Token::Number('1'),
                        Token::Whitespace,
                        Token::Operator(Op::Add),
                        Token::Whitespace,
                        Token::Number('2')],
                   tokens);
    }

    #[test]
    fn it_scans_unrecognized_tokens() {
        let tokens = scan("1 a 2");
        assert_eq!(5, tokens.len());
        assert_eq!(vec![Token::Number('1'),
                        Token::Whitespace,
                        Token::Unrecognized('a'),
                        Token::Whitespace,
                        Token::Number('2')],
                   tokens);
    }
}

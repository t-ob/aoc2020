#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Binop {
    A,
    M,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    V(i64),
    B(Binop),
    L,
    R,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    V(i64),
    A(Box<Expr>, Box<Expr>),
    M(Box<Expr>, Box<Expr>),
    E(Box<Expr>),
}

impl Expr {
    pub fn new(tokens: &[Token]) -> Result<Box<Expr>, ()> {
        let mut expr = None;

        let mut i = 0;
        while i < tokens.len() {
            match (expr, tokens[i]) {
                (None, Token::V(v)) => {
                    expr = Some(Box::new(Expr::V(v)));
                    i += 1;
                }
                (None, Token::L) => {
                    let mut j = i + 1;
                    let mut lparens = 0;
                    loop {
                        match (&tokens[j], lparens) {
                            (Token::L, _) => lparens += 1,
                            (Token::R, 0) => break,
                            (Token::R, _) => lparens -= 1,
                            _ => {}
                        }
                        j += 1;
                    }
                    expr = Some(Expr::new(&tokens[i + 1..j])?);
                    i = j + 1;
                }
                (Some(acc_expr), Token::B(binop)) => {
                    let rhs: Box<Expr>;
                    match &tokens[i + 1] {
                        Token::V(w) => {
                            rhs = Box::new(Expr::V(*w));
                            i += 2;
                        }
                        Token::L => {
                            let mut j = i + 2;
                            let mut lparens = 0;
                            loop {
                                match (&tokens[j], lparens) {
                                    (Token::L, _) => lparens += 1,
                                    (Token::R, 0) => break,
                                    (Token::R, _) => lparens -= 1,
                                    _ => {}
                                }
                                j += 1;
                            }
                            rhs = Expr::new(&tokens[i + 2..j])?;
                            i = j + 1;
                        }
                        _ => return Err(()),
                    }
                    match binop {
                        Binop::A => expr = Some(Box::new(Expr::A(acc_expr, rhs))),
                        Binop::M => expr = Some(Box::new(Expr::M(acc_expr, rhs))),
                    }
                }
                _ => return Err(()),
            }
        }

        Ok(expr.unwrap())
    }
}

#[derive(PartialEq, Eq)]
pub enum Mode {
    InOrder,
    AdditionTakesPrecedence,
}

pub fn read(line: &str, mode: Mode) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    let preprocessed = line.replace("(", "( ").replace(")", " )");
    let mut raw_tokens = preprocessed.split_ascii_whitespace();

    let mut additions = 0;

    loop {
        match raw_tokens.next() {
            None => break,
            Some("+") => {
                tokens.push(Token::B(Binop::A));
                additions += 1
            }
            Some("*") => tokens.push(Token::B(Binop::M)),
            Some("(") => tokens.push(Token::L),
            Some(")") => tokens.push(Token::R),
            Some(v) => tokens.push(Token::V(v.parse().expect("Expected int"))),
        }
    }

    if mode == Mode::AdditionTakesPrecedence {
        for current_addition in 0..additions {
            let mut addition = 0;
            let mut rparen_idx = None;
            let mut lparen_idx = None;
            for (idx, token) in tokens.iter().enumerate() {
                match (token, addition == current_addition) {
                    (Token::B(Binop::A), false) => addition += 1,
                    (Token::B(Binop::A), true) => {
                        rparen_idx = Some(idx);
                        let mut lparens = 0;
                        loop {
                            rparen_idx = Some(rparen_idx.unwrap() + 1);
                            match (lparens, &tokens[rparen_idx.unwrap()]) {
                                (_, Token::L) => lparens += 1,
                                (1, Token::R) => break,
                                (_, Token::R) => lparens -= 1,
                                (0, _) => break,
                                _ => continue,
                            }
                        }

                        lparen_idx = Some(idx);
                        let mut rparens = 0;
                        loop {
                            lparen_idx = Some(lparen_idx.unwrap() - 1);
                            match (rparens, &tokens[lparen_idx.unwrap()]) {
                                (_, Token::R) => rparens += 1,
                                (1, Token::L) => break,
                                (_, Token::L) => rparens -= 1,
                                (0, _) => break,
                                _ => continue,
                            }
                        }
                        break;
                    }
                    _ => continue,
                }
            }
            tokens.insert(rparen_idx.unwrap() + 1, Token::R);
            tokens.insert(lparen_idx.unwrap(), Token::L);
        }
    }

    tokens
}

pub fn eval(expr: Box<Expr>) -> i64 {
    match *expr {
        Expr::V(x) => x,
        Expr::A(lhs, rhs) => eval(lhs) + eval(rhs),
        Expr::M(lhs, rhs) => eval(lhs) * eval(rhs),
        Expr::E(expr) => eval(expr),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expr_eval() {
        // 2 * 3 + (4 * 5)
        let expr = Box::new(Expr::A(
            Box::new(Expr::M(Box::new(Expr::V(2)), Box::new(Expr::V(3)))),
            Box::new(Expr::E(Box::new(Expr::M(
                Box::new(Expr::V(4)),
                Box::new(Expr::V(5)),
            )))),
        ));

        assert_eq!(eval(expr), 26)
    }

    #[test]
    fn test_read_eval() {
        assert_eq!(eval(Expr::new(&read("1", Mode::InOrder)).unwrap()), 1);
        assert_eq!(eval(Expr::new(&read("1 + 2", Mode::InOrder)).unwrap()), 3);
        assert_eq!(
            eval(Expr::new(&read("1 + 2 * 3 + 4 * 5 + 6", Mode::InOrder)).unwrap()),
            71
        );
        assert_eq!(
            eval(Expr::new(&read("1 + (2 * 3) + (4 * (5 + 6))", Mode::InOrder)).unwrap()),
            51
        );
        assert_eq!(
            eval(
                Expr::new(&read(
                    "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2",
                    Mode::InOrder
                ))
                .unwrap()
            ),
            13632
        );
        assert_eq!(
            eval(
                Expr::new(&read(
                    "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2",
                    Mode::AdditionTakesPrecedence
                ))
                .unwrap()
            ),
            23340
        );
    }

    #[test]
    fn test_read_addition_takes_precedence() {
        assert_eq!(
            read("1 * 2", Mode::AdditionTakesPrecedence),
            vec![Token::V(1), Token::B(Binop::M), Token::V(2)]
        );
        assert_eq!(
            read("1 + 2", Mode::AdditionTakesPrecedence),
            vec![
                Token::L,
                Token::V(1),
                Token::B(Binop::A),
                Token::V(2),
                Token::R
            ]
        );
        assert_eq!(
            read("1 + 2 * 3 + 4 * 5 + 6", Mode::AdditionTakesPrecedence),
            vec![
                Token::L,
                Token::V(1),
                Token::B(Binop::A),
                Token::V(2),
                Token::R,
                Token::B(Binop::M),
                Token::L,
                Token::V(3),
                Token::B(Binop::A),
                Token::V(4),
                Token::R,
                Token::B(Binop::M),
                Token::L,
                Token::V(5),
                Token::B(Binop::A),
                Token::V(6),
                Token::R
            ]
        );
        assert_eq!(
            read("1 + (2 * 3) + (4 * (5 + 6))", Mode::AdditionTakesPrecedence),
            vec![
                Token::L,
                Token::L,
                Token::V(1),
                Token::B(Binop::A),
                Token::L,
                Token::V(2),
                Token::B(Binop::M),
                Token::V(3),
                Token::R,
                Token::R,
                Token::B(Binop::A),
                Token::L,
                Token::V(4),
                Token::B(Binop::M),
                Token::L,
                Token::L,
                Token::V(5),
                Token::B(Binop::A),
                Token::V(6),
                Token::R,
                Token::R,
                Token::R,
                Token::R,
            ]
        )
    }
}

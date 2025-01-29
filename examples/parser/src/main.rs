use core::panic;
use std::sync::LazyLock;

use spannify::config::Config;
use spannify::core::StdoutSpanner;
use spannify::spf;

static SPANNER: LazyLock<StdoutSpanner> =
    LazyLock::new(|| StdoutSpanner::new().with_config(Config::new().with_skip(1)));

struct ExpressionParser<T>
where
    T: Iterator<Item = Token>,
{
    tokens: T,

    current_token: Option<Token>,
    peek_token: Option<Token>,
}

impl<T> ExpressionParser<T>
where
    T: Iterator<Item = Token>,
{
    fn new(mut tokens: T) -> Self {
        Self {
            current_token: tokens.next(),
            peek_token: tokens.next(),

            tokens,
        }
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Expression {
        let _span = spf!(
            SPANNER,
            "parse_expression: current=`{:?}`",
            self.current_token.unwrap()
        );

        let mut left = match self.current_token {
            Some(Token::Number(v)) => Expression::Integer(v),
            Some(Token::LParen) => self.parse_grouped_expression(),
            v => panic!("unexpected token: `{:?}`", v),
        };

        while self.peek_token.is_some() && precedence < self.peek_token.unwrap().precedence() {
            left = match self.peek_token {
                Some(Token::Add | Token::Sub | Token::Div | Token::Mul) => {
                    self.advance_tokens();
                    self.parse_infix_expression(left)
                }
                _ => {
                    return left;
                }
            };
        }

        left
    }

    fn parse_grouped_expression(&mut self) -> Expression {
        let _span = spf!(
            SPANNER,
            "parse_grouped_expression: current=`{:?}`",
            self.current_token.unwrap(),
        );
        self.advance_tokens();

        let expr = self.parse_expression(Precedence::Lowest);

        self.advance_tokens();

        expr
    }

    fn advance_tokens(&mut self) {
        let _span = spf!(
            SPANNER,
            "advance_tokens: current=`{:?}`",
            self.current_token.unwrap(),
        );

        std::mem::swap(&mut self.current_token, &mut self.peek_token);
        self.peek_token = self.tokens.next();
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Expression {
        let _span = spf!(
            SPANNER,
            "parse_infix_expression: current=`{:?}`",
            self.current_token.unwrap()
        );

        let token = self.current_token;
        let operator = token.unwrap();
        let precedence = self.current_token.unwrap().precedence();
        self.advance_tokens();
        let right = self.parse_expression(precedence);

        Expression::InfixExpression(InfixExpression {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Token {
    Number(i64),
    LParen,
    RBrace,
    Add,
    Sub,
    Mul,
    Div,
}

impl Token {
    fn try_from(s: &str) -> Option<Self> {
        let tok = match s {
            "(" => Token::LParen,
            ")" => Token::RBrace,
            "+" => Token::Add,
            "-" => Token::Sub,
            "*" => Token::Mul,
            "/" => Token::Div,
            v => match v.parse::<i64>() {
                Ok(v) => Token::Number(v),
                _ => return None,
            },
        };

        Some(tok)
    }

    fn precedence(&self) -> Precedence {
        match self {
            Self::Add | Self::Sub => Precedence::Sum,
            Self::Div | Self::Mul => Precedence::Product,
            _ => Precedence::Lowest,
        }
    }
}

#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub enum Precedence {
    Lowest,
    Sum,
    Product,
}

#[derive(Debug)]
pub enum Expression {
    InfixExpression(InfixExpression),
    Integer(i64),
}

#[allow(unused)]
#[derive(Debug)]
pub struct InfixExpression {
    left: Box<Expression>,
    operator: Token,
    right: Box<Expression>,
}

fn main() {
    let expr = "10 + 13 - 23 / ( 103 - 10 ) + 1";
    let tokens: Vec<Token> = expr
        .split_whitespace()
        .map(Token::try_from)
        .collect::<Option<Vec<_>>>()
        .unwrap();

    let mut parser = ExpressionParser::new(tokens.into_iter());

    let _expression = parser.parse_expression(Precedence::Lowest);
}

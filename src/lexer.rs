use nom::{
    branch::alt,
    bytes::{complete::is_not, complete::take_while, streaming::tag},
    character::{
        complete::{alpha1, alphanumeric1, digit1, multispace0},
        is_alphabetic,
    },
    combinator::{map, map_res, recognize},
    multi::many0,
    sequence::{delimited, pair},
    IResult,
};
use std::str::{from_utf8, FromStr, Utf8Error};

#[derive(PartialEq, Debug, Clone)]
pub enum Tokens {
    LParen,
    RParen,
    Let,
    Assign,
    SemiColon,
    Plus,
    Minus,
    Equals,
    IntegerLiteral(i64),
    StringLiteal(String),
    Identifier(String),
    Comma,
    Dot,
    EOF,
}

fn lex_operator(input: &[u8]) -> IResult<&[u8], Tokens> {
    map(alt((tag("="), tag("+"), tag("-"))), |op: &[u8]| match op {
        b"=" => Tokens::Assign,
        b"+" => Tokens::Plus,
        b"-" => Tokens::Minus,
        _ => Tokens::EOF,
    })(input)
}

fn lex_punctuations(input: &[u8]) -> IResult<&[u8], Tokens> {
    map(alt((tag(";"), tag(","), tag("."))), |op: &[u8]| match op {
        b";" => Tokens::SemiColon,
        b"," => Tokens::Comma,
        b"." => Tokens::Dot,
        _ => Tokens::EOF,
    })(input)
}

fn lex_ident(input: &[u8]) -> IResult<&[u8], Tokens> {
    map_res(
        recognize(pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_")))),
        )),
        |s| {
            let c = from_utf8(s);
            c.map(|syntax| match syntax {
                "let" => Tokens::Let,
                _ => Tokens::Identifier(syntax.to_string()),
            })
        },
    )(input)
}

fn lex_integer(input: &[u8]) -> IResult<&[u8], Tokens> {
    map(
        map_res(map_res(digit1, std::str::from_utf8), FromStr::from_str),
        Tokens::IntegerLiteral,
    )(input)
}

fn lex_token(input: &[u8]) -> IResult<&[u8], Tokens> {
    alt((lex_operator, lex_ident, lex_integer, lex_punctuations))(input)
}

fn lex_tokens(input: &[u8]) -> IResult<&[u8], Vec<Tokens>> {
    many0(delimited(multispace0, lex_token, multispace0))(input)
}
pub struct Lexer;

impl Lexer {
    pub fn lex_tokens(bytes: &[u8]) -> IResult<&[u8], Vec<Tokens>> {
        lex_tokens(bytes)
            .map(|(slice, result)| (slice, [&result[..], &vec![Tokens::EOF][..]].concat()))
    }
}

#[test]
fn test_lex() {
    let input = b"let x = 2;#";
    let (_, result) = lex_tokens(input).unwrap();

    let expected = vec![
        Tokens::Let,
        Tokens::Identifier("x".to_owned()),
        Tokens::Assign,
        Tokens::IntegerLiteral(2),
        Tokens::SemiColon,
    ];

    assert_eq!(result, expected)
}

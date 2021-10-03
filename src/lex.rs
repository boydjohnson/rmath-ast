use nom::{
    branch::alt,
    character::complete::{char, i64, none_of, one_of, space0},
    combinator::{map, map_res, opt, recognize},
    multi::{many0, many1},
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

#[derive(Debug, PartialEq)]
pub enum Token {
    LParens,
    RParens,
    Add,
    Sub,
    Div,
    Mul,
    Pow,
    Eq,
    String(String),
    Int(i64),
    Float(f64),
}

pub fn parse(s: &str) -> Vec<Token> {
    let mut tokens = vec![];

    let mut val = s.to_string();

    while !val.is_empty() {
        if let Ok((remaining, tok)) = parse_token(&val) {
            tokens.push(tok);
            val = remaining.to_string();
        }
    }
    tokens
}

pub fn parse_token(s: &str) -> IResult<&str, Token> {
    lparens(s)
        .or_else(|_| rparens(s))
        .or_else(|_| add(s))
        .or_else(|_| sub(s))
        .or_else(|_| div(s))
        .or_else(|_| mul(s))
        .or_else(|_| pow(s))
        .or_else(|_| eq(s))
        .or_else(|_| float(s))
        .or_else(|_| int(s))
        .or_else(|_| string(s))
}

fn char_delimited<'a, G: FnMut(char) -> Token, E: nom::error::ParseError<&'a str>>(
    c: char,
    g: G,
) -> impl FnMut(&'a str) -> IResult<&'a str, Token, E> {
    map(delimited(space0, char(c), space0), g)
}

fn lparens(s: &str) -> nom::IResult<&str, Token> {
    char_delimited('(', |_| Token::LParens)(s)
}

fn rparens(s: &str) -> nom::IResult<&str, Token> {
    char_delimited(')', |_| Token::RParens)(s)
}

fn add(s: &str) -> nom::IResult<&str, Token> {
    char_delimited('+', |_| Token::Add)(s)
}

fn sub(s: &str) -> nom::IResult<&str, Token> {
    char_delimited('-', |_| Token::Sub)(s)
}

fn div(s: &str) -> nom::IResult<&str, Token> {
    char_delimited('/', |_| Token::Div)(s)
}

fn mul(s: &str) -> nom::IResult<&str, Token> {
    char_delimited('*', |_| Token::Mul)(s)
}

fn pow(s: &str) -> nom::IResult<&str, Token> {
    char_delimited('^', |_| Token::Pow)(s)
}

fn eq(s: &str) -> nom::IResult<&str, Token> {
    char_delimited('=', |_| Token::Eq)(s)
}

fn int(s: &str) -> nom::IResult<&str, Token> {
    map(i64, Token::Int)(s)
}

fn float(input: &str) -> IResult<&str, Token> {
    map(
        map_res(
            alt((
                // Case one: .42
                recognize(tuple((
                    char('.'),
                    decimal,
                    opt(tuple((one_of("eE"), opt(one_of("+-")), decimal))),
                ))), // Case two: 42e42 and 42.42e42
                recognize(tuple((
                    decimal,
                    opt(preceded(char('.'), decimal)),
                    one_of("eE"),
                    opt(one_of("+-")),
                    decimal,
                ))), // Case three: 42. and 42.42
                recognize(tuple((decimal, char('.'), opt(decimal)))),
            )),
            |v| v.parse::<f64>(),
        ),
        Token::Float,
    )(input)
}

fn decimal(input: &str) -> IResult<&str, &str> {
    recognize(many1(terminated(one_of("0123456789"), many0(char('_')))))(input)
}

fn string(input: &str) -> IResult<&str, Token> {
    map(recognize(many1(none_of("()+-/*^ "))), |v: &str| {
        Token::String(v.into())
    })(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_token() {
        assert_eq!(parse_token("(1 + 2)"), Ok(("1 + 2)", Token::LParens)));

        assert_eq!(parse_token("1 + 2)"), Ok((" + 2)", Token::Int(1))));

        assert_eq!(parse_token(" + 2)"), Ok(("2)", Token::Add)));

        assert_eq!(parse_token("2)"), Ok((")", Token::Int(2))));

        assert_eq!(parse_token(")"), Ok(("", Token::RParens)));

        assert_eq!(
            parse_token("4.5 + d.TOTALVALUE"),
            Ok((" + d.TOTALVALUE", Token::Float(4.5)))
        );
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            parse("(d.TOTALVALUE + rbern(seed = 159)) / d.BUILDINGVALUE"),
            vec![
                Token::LParens,
                Token::String("d.TOTALVALUE".into()),
                Token::Add,
                Token::String("rbern".into()),
                Token::LParens,
                Token::String("seed".into()),
                Token::Eq,
                Token::Int(159),
                Token::RParens,
                Token::RParens,
                Token::Div,
                Token::String("d.BUILDINGVALUE".into())
            ]
        );

        assert_eq!(
            parse("as_double( d.TOTALVALUE ) / 4.56789"),
            vec![
                Token::String("as_double".into()),
                Token::LParens,
                Token::String("d.TOTALVALUE".into()),
                Token::RParens,
                Token::Div,
                Token::Float(4.56789)
            ]
        );

        assert_eq!(
            parse("(log10(10.5 ^ 4) * 4.5) / 5.5"),
            vec![
                Token::LParens,
                Token::String("log10".into()),
                Token::LParens,
                Token::Float(10.5),
                Token::Pow,
                Token::Int(4),
                Token::RParens,
                Token::Mul,
                Token::Float(4.5),
                Token::RParens,
                Token::Div,
                Token::Float(5.5),
            ]
        );
    }
}

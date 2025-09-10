use super::super::descriptor::Literal;
use chumsky::prelude::*;

fn quoted_string<'a>(quote: char) -> impl Parser<'a, &'a str, String, extra::Err<Rich<'a, char>>> {
    let normal_char = none_of([quote, '\\']);

    let escape_char = just('\\').ignore_then(one_of(['\'', '"', 'n', '\\']));

    let content_char = escape_char
        .map(|c| match c {
            'n' => '\n',
            other => other,
        })
        .or(normal_char);

    just(quote)
        .ignore_then(content_char.repeated().collect::<String>())
        .then_ignore(just(quote))
}

fn single_quote<'a>() -> impl Parser<'a, &'a str, String, extra::Err<Rich<'a, char>>> {
    quoted_string('\'')
}

fn double_quote<'a>() -> impl Parser<'a, &'a str, String, extra::Err<Rich<'a, char>>> {
    quoted_string('"')
}

pub fn parse<'a>() -> impl Parser<'a, &'a str, Literal, extra::Err<Rich<'a, char>>> {
    single_quote().map(Literal).or(double_quote().map(Literal))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal() {
        let _ = parse().parse(r#""hello""#).unwrap();
        let _ = parse().parse(r#"'hello'"#).unwrap();
        let _ = parse().parse(r#"'hello\'world'"#).unwrap();
        let _ = parse().parse(r#"'hello\nworld'"#).unwrap();
        let _ = parse().parse(r#"'hello\\world'"#).unwrap();
        let _ = parse().parse(r#"'hello\"world'"#).unwrap();
    }
}

use nom::{
    bytes::complete::{tag, take_while_m_n},
    character::complete::{alphanumeric1, multispace0},
    combinator::recognize,
    sequence::{delimited, pair},
    IResult,
};

/// ## normal parser for easy string and split string
/// depend on what split sign
pub fn parse_normal(input: &str, sign: char) -> IResult<&str, &str> {
    recognize(pair(
        alphanumeric1,
        take_while_m_n(0, usize::MAX, |c: char| c == sign || c.is_alphanumeric()),
    ))(input)
}

/// ## ⚡️ parse normal value 🆗
/// use in property | value | script variable name
/// - parse xxx
/// - parse xxx_zzz
pub fn parse_value(input: &str) -> IResult<&str, &str> {
    parse_normal(input, '_')
}

/// ## parse sign then get parse_value
/// format: `_xxx_zzz` | `@sss_vvv`
pub fn parse_sign_key<'a>(input: &'a str, sign: &'a str) -> IResult<&'a str, (&'a str, &'a str)> {
    let (input, sign) = tag(sign)(input)?;
    let (input, value) = parse_value(input)?;
    Ok((input, (sign, value)))
}

/// ## trim any parser left and right multispace(if exist)
#[allow(unused_mut)]
pub fn trim<'a, P, O>(mut parser: P) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    P: FnMut(&'a str) -> IResult<&'a str, O>,
{
    delimited(multispace0, parser, multispace0)
}

#[cfg(test)]
mod normal {
    use super::*;
    #[test]
    fn test_parse_value() {
        let simple = "test";
        let complex = "test_input";
        let more = "test_input_value";
        let res1 = parse_value(simple).unwrap();
        let res2 = parse_value(complex).unwrap();
        let res3 = parse_value(more).unwrap();
        assert_eq!(res1, ("", "test"));
        assert_eq!(res2, ("", "test_input"));
        assert_eq!(res3, ("", "test_input_value"));
    }
}


use nom::{IResult, digit};
use std;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Token {
    /// A literal character to print
    Literal(u8),
    /// A placeholder for a derived value
    Placeholder {
        right_align: bool,
        preceding_plus: bool,
        pad_sign: bool,
        prefix: bool,
        pad_zero: bool,
        width: Option<Width>,
        precision: Option<Width>,
        modifier: Modifier,
        token_type: TokenType,
    },
    /// Indicates an error in parsing a placeholder. Silently stop and return (this seems to be c
    /// behaviour).
    Error,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum TokenType {
    SignedInteger,
    UnsignedInteger,
    UnsignedOctal,
    UnsignedHex,
    UnsignedHexUpper,
    Float,
    FloatUpper,
    Scientific,
    ScientificUpper,
    ShortestFloat,
    ShortestFloatUpper,
    HexFloat,
    HexFloatUpper,
    Character,
    String,
    PointerAddress,
    /// In c, write nothing and store the number of chars. We ignore this
    StoreChars,
    PercentLiteral,
}

/// Whether the type is long, size_t, etc
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Modifier {
    /// No modifier
    None,
    /// char
    Char,
    /// short
    Short,
    /// long
    Long,
    /// long long
    LongLong,
    /// intmax_t
    IntMax,
    /// size_t
    Size,
    /// ptrdiff_t
    PtrDiff,
    /// long double
    LongDouble
}

/// Width can either be static value, or read off va_list
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Width {
    Static(usize),
    Dynamic
}

named!(pub parse_format_str<Vec<Token> >, many0!(
    alt!( parse_placeholder | map_opt!(take!(1), single_byte) ) // map_opt, but never fails
));

#[test]
fn test_parse_whole_str() {

    let input_1 = b"Test string: %-+ #010.15s.";
    let res = vec![
        Token::Literal(b'T'),
        Token::Literal(b'e'),
        Token::Literal(b's'),
        Token::Literal(b't'),
        Token::Literal(b' '),
        Token::Literal(b's'),
        Token::Literal(b't'),
        Token::Literal(b'r'),
        Token::Literal(b'i'),
        Token::Literal(b'n'),
        Token::Literal(b'g'),
        Token::Literal(b':'),
        Token::Literal(b' '),
        Token::Placeholder {
            right_align: true,
            preceding_plus: true,
            pad_sign: true,
            prefix: true,
            pad_zero: true,
            width: Some(Width::Static(10)),
            precision: Some(Width::Static(15)),
            modifier: Modifier::None,
            token_type: TokenType::String,
        },
        Token::Literal(b'.'),
    ];
    assert_eq!(parse_format_str(input_1), parse_success(res));

}

named!(parse_placeholder<Token>, do_parse!(
    tag!(b"%") >>
    right_align: map!(opt!(tag!(b"-")), |o: Option<_>| o.is_some()) >>
    preceding_plus: map!(opt!(tag!(b"+")), |o: Option<_>| o.is_some()) >>
    pad_sign: map!(opt!(tag!(b" ")), |o: Option<_>| o.is_some()) >>
    prefix: map!(opt!(tag!(b"#")), |o: Option<_>| o.is_some()) >>
    pad_zero: map!(opt!(tag!(b"0")), |o: Option<_>| o.is_some()) >>
    width: opt!(parse_number_or_dynamic) >>
    precision: opt!(parse_decimal) >>
    modifier: parse_modifier >>
    token_type: parse_placeholder_type >>
    (Token::Placeholder {
        right_align: right_align,
        preceding_plus: preceding_plus,
        pad_sign: pad_sign,
        prefix: prefix,
        pad_zero: pad_zero,
        width: width,
        precision: precision,
        modifier: modifier,
        token_type: token_type,
    })
));

named!(parse_placeholder_type<TokenType>, alt!(
    map!(one_of!(b"di"), |_| TokenType::SignedInteger)
    | map!(tag!(b"u"), |_| TokenType::UnsignedInteger)
    | map!(tag!(b"o"), |_| TokenType::UnsignedOctal)
    | map!(tag!(b"x"), |_| TokenType::UnsignedHex)
    | map!(tag!(b"X"), |_| TokenType::UnsignedHexUpper)
    | map!(tag!(b"f"), |_| TokenType::Float)
    | map!(tag!(b"F"), |_| TokenType::FloatUpper)
    | map!(tag!(b"e"), |_| TokenType::Scientific)
    | map!(tag!(b"E"), |_| TokenType::ScientificUpper)
    | map!(tag!(b"g"), |_| TokenType::ShortestFloat)
    | map!(tag!(b"G"), |_| TokenType::ShortestFloatUpper)
    | map!(tag!(b"a"), |_| TokenType::HexFloat)
    | map!(tag!(b"A"), |_| TokenType::HexFloatUpper)
    | map!(tag!(b"c"), |_| TokenType::Character)
    | map!(tag!(b"s"), |_| TokenType::String)
    | map!(tag!(b"p"), |_| TokenType::PointerAddress)
    | map!(tag!(b"n"), |_| TokenType::StoreChars)
    | map!(tag!(b"%"), |_| TokenType::PercentLiteral)
));

named!(parse_modifier<Modifier>,
    map!(opt!(alt!(
        map!(one_of!(b"hh"), |_| Modifier::Char)
        | map!(tag!(b"h"), |_| Modifier::Short)
        | map!(tag!(b"l"), |_| Modifier::Long)
        | map!(tag!(b"ll"), |_| Modifier::LongLong)
        | map!(tag!(b"j"), |_| Modifier::IntMax)
        | map!(tag!(b"z"), |_| Modifier::Size)
        | map!(tag!(b"t"), |_| Modifier::PtrDiff)
        | map!(tag!(b"L"), |_| Modifier::LongDouble)
    )), |val: Option<Modifier>| val.unwrap_or(Modifier::None))
);

named!(parse_number_or_dynamic<Width>, alt!(
    map!(tag!(b"*"), |_| Width::Dynamic)
    | map!(parse_number, Width::Static)
));

named!(parse_number<usize>, map_opt!(digit, |width: &[u8]| {
    // this bit should never fail, but since panics would result in UB, play safe
    std::str::from_utf8(width) .ok()
        .and_then(|width: &str| width.parse::<usize>().ok())
}));

named!(parse_decimal<Width>, do_parse!(
    tag!(b".") >>
    width: parse_number_or_dynamic >>
    (width)
));

#[test]
fn test_parse_number() {
    let input_1 = b"1234";
    assert_eq!(parse_number(input_1), parse_success(1234));
}

// helpers

/// helper function to turn single byte slice into token literal
fn single_byte(i: &[u8]) -> Option<Token> {
    if let Some(&num) = i.get(0) {
        Some(Token::Literal(num))
    } else {
        None
    }
}

/// Helper function for tests
#[cfg(test)]
fn parse_success<T>(out: T) -> IResult<&'static [u8], T> {
    IResult::Done(&b""[..], out)
}

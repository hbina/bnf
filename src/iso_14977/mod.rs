use nom::Slice;
use std::str::Chars;

/// Parses for the given character skipping `gap-separator`s.
///
/// # Note
///
/// This function is equivalent to [`parse_for_str`] except that it only tries a character.
pub fn parse_for_token<F>(
    f: F,
) -> impl Fn(&str) -> nom::IResult<&str, Token, nom::error::VerboseError<&str>>
where
    F: Fn(&str) -> nom::IResult<&str, Token, nom::error::VerboseError<&str>>,
{
    nom::sequence::delimited(parse_gap_separator, f, parse_gap_separator)
}

/// Parse for a sequence of gap-separators.
///
/// This is generally used to consume and forget about gap-separators.
/// In the future, one might want to do additional stuff with the return value, so we just keep it
/// i.e. for formatting purposes.
///
/// # Example
///
/// ```
/// # use bnf::iso_14977::parse_gap_separator;
/// # fn main() -> Result<(), Box<dyn std::error::Error>>
/// # {
/// let (left, parsed) = parse_gap_separator(" \r\n\t\n\u{000B}")?;
/// assert_eq!(left, "");
/// assert_eq!(parsed, " \r\n\t\n\u{000B}");
/// # Ok(())
/// # }
/// ```
///
pub fn parse_gap_separator(
    input: &str,
) -> nom::IResult<&str, &str, nom::error::VerboseError<&str>> {
    let (input, rest) = nom::bytes::complete::take_while(is_gap_separator)(input)?;
    Ok((input, rest))
}

/// Checks if a given `char` is a gap-separator as defined by 6.4 Gap-Separator.
///
/// AFAIK, this is only place where it actually defines what non-printing characters are.
/// According to the spec:
/// 1.  Space;
/// 2.  Horizontal tabulation;
/// 3.  New-line;
/// 4.  Vertical tabulation.
///
/// I have no idea if this list is comprehensive or not.
///
/// # NOTE
///
/// 1.  For vertical tabulation, it is quite tricky. Rust does not support vertical tabulation
///     natively in its strings module nor it will in the near future (the RFC was pulled back IIRC).
///     AFAIK, checking against its unicode value is sufficient `\u{0000B}`.
///     Admittedly, character encodings make my head spin so I might be totally wrong.
///
pub fn is_gap_separator(chr: char) -> bool {
    chr == ' ' || chr == '\n' || chr == '\r' || chr == '\t' || chr == '\u{000B}'
}

pub fn lexical_analysis(input: &str) -> Option<Token> {
    let mut input = input.next();
    match input {
        Some('c') => {}
        None => {}
        _ => {}
    }
    None
}

pub fn parse_ebnf(input: &str) -> Vec<Token> {
    let mut result = vec![];
    loop {
        match lexical_analysis(&input[..]) {
            Some(x) => result.push(x),
            None => break,
        }
    }
    result
}

pub enum Operator {
    Repetition,
    Except,
    Concatenate,
    DefinitionSeparator,
    Defining,
    Terminator,
}

pub enum BracketStart {}

pub enum BracketEnd {}

pub enum Symbol {
    Operator(Operator),
    BracketStart(BracketStart),
    BracketEnd(BracketEnd),
}

#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    Symbol(Symbol),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_syntax() {
        let syntax = r#"letter = "A" | "B" | "C" | "D" | "E" | "F" | "G" | "H" | "I" | "J" |
        "K" | "L" | "M" | "N" | "O" | "P" | "Q" | "R" | "S" | "T" | "U" | "V" | "W" | "X" | "Y" |
        "Z";
        vowel = "A" | "E" | "I" | "O" | "U";
        consonant = letter - vowel;
        ee = {"A"}-, "E";"#;
        parse_ebnf(syntax).unwrap();
    }
}

//! Common parsers for BNF-like text.
//!

use iso_14977::structure::{MetaIdentifier, Syntax, SyntaxRule};
use iso_14977::symbols::Symbol;
use nom::InputTakeAtPosition;
use std::convert::TryFrom;

pub type Result<'a> = nom::IResult<&'a str, &'a str, nom::error::VerboseError<&'a str>>;

/// Parses the bytes between an opening and closing tag.
///
/// This is a convenient function that is usually used for 2 purposes:
/// 1.  Parsing syntactic-primaries that are usually enclosed in some form of opening and closing tags.
/// 2.  Parsing BNF because the meta-identifiers in it is usually enclosed by
/// `<` and `>`.
///
/// # Example
///
/// ```
/// # use bnf::iso_14977::parsers::parse_between_tags;
/// # fn main() -> Result<(), Box<dyn std::error::Error>>
/// # {
/// let (left, parsed) = parse_between_tags::<&str>("   \n\r\n\t <hello> \t\n\t\t\r abc", "<",">")?;
/// assert_eq!(left, "abc");
/// assert_eq!(parsed, "hello");
/// let (left2, parsed2) = parse_between_tags::<&str>("    !@#$hello%^&* abc", "!@#$", "%^&*")?;
/// assert_eq!(left2, "abc");
/// assert_eq!(parsed2, "hello");
/// # Ok(())
/// # }
/// ```
///
pub fn parse_between_tags<'a, 'b>(
    input: &'a str,
    begin_tag: &'b str,
    end_tag: &'b str,
) -> Result<'a> {
    let (input_leftover, matched) = nom::sequence::delimited(
        nom::sequence::preceded(parse_gap_separator, nom::bytes::complete::tag(begin_tag)),
        nom::bytes::complete::take_until(end_tag),
        nom::sequence::terminated(nom::bytes::complete::tag(end_tag), parse_gap_separator),
    )(input)?;
    Ok((input_leftover, matched))
}

/// Parses for the given bytes skipping gap-separators.
///
/// This is just a convenient function over `nom::bytes:complete::tag` because of Section 6.4
/// Gap-Separator e.g. that non-printing characters generally have no meaning.
///
/// However, one must use this function only when:
/// 1.  Before a syntax;
/// 2.  Between any two gap-free-symbols of a syntax,
/// 3.  After a syntax.
///
/// Otherwise, whitespaces must not be skipped.
///
/// # Example
///
/// ```
/// # use bnf::iso_14977::parsers::parse_for_str;
/// # fn main() -> Result<(), Box<dyn std::error::Error>>
/// # {
/// let (left, parsed) = parse_for_str::<&str>(" \r\n\t\n hello world", "hello")?;
/// assert_eq!(left, "world");
/// assert_eq!(parsed, "hello");
/// # Ok(())
/// # }
/// ```
///
/// # Note
///
/// 1.  Do not use this to parse `first-quote-symbol` or `second-quote-symbol` because this will possibly
///     skip the non-printing characters _inside_ the terminal-strings which _does have_ syntactic
///     meaning. Use [`parse_between_tags`] instead.
pub fn parse_for_str<'a, 'b>(input: &'a str, expect: &'b str) -> Result<'a> {
    let (input_leftover, matched) = nom::sequence::preceded(
        parse_gap_separator,
        nom::sequence::terminated(nom::bytes::complete::tag(expect), parse_gap_separator),
    )(input)?;
    Ok((input_leftover, matched))
}

/// Parses for the given character skipping `gap-separator`s.
///
/// # Note
///
/// This function is equivalent to [`parse_for_str`] except that it only tries a character.
pub fn parse_for_char(
    input: &str,
    expect: char,
) -> nom::IResult<&str, char, nom::error::VerboseError<&str>> {
    let (input_leftover, matched) = nom::sequence::preceded(
        parse_gap_separator,
        nom::sequence::terminated(nom::character::complete::char(expect), parse_gap_separator),
    )(input)?;
    Ok((input_leftover, matched))
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
/// # use bnf::iso_14977::parsers::parse_gap_separator;
/// # fn main() -> Result<(), Box<dyn std::error::Error>>
/// # {
/// let (left, parsed) = parse_gap_separator::<&str>(" \r\n\t\n\u{000B}")?;
/// assert_eq!(left, "");
/// assert_eq!(parsed, " \r\n\t\n\u{000B}");
/// # Ok(())
/// # }
/// ```
///
pub fn parse_gap_separator(input: &str) -> Result {
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

/// Parses for a `meta-identifier`.
///
/// The specification states that a `meta-identifier` consists of an ordered list of one or more
/// `meta-identifier-characters` subject to the condition that the first `meta-identifier-character`
/// is a letter. Note that `meta-identifier-characters` is either a decimal digit or a letter i.e.
/// it is alphanumeric.
pub fn parse_meta_identifier(
    input: &str,
) -> nom::IResult<&str, MetaIdentifier, nom::error::VerboseError<&str>> {
    let (input_leftover, matched) = nom::sequence::delimited(
        parse_gap_separator,
        nom::sequence::pair(
            nom::character::complete::alpha1,
            nom::character::complete::alphanumeric0,
        ),
        parse_gap_separator,
    )(input)?;
    Ok((
        input_leftover,
        MetaIdentifier::from(format!("{}{}", matched.0, matched.1)),
    ))
}

/// Parse for a specific `symbol`.
///
/// See [`Symbol`].
pub fn parse_symbol(
    input: &str,
    symbol: Symbol,
) -> nom::IResult<&str, Symbol, nom::error::VerboseError<&str>> {
    let (input_leftover, matched) = parse_for_str(input, Into::into(symbol))?;
    Ok((input_leftover, Symbol::try_from(matched).unwrap()))
}

/// Parse for a `syntax-rule`.
pub fn parse_syntax_rule(
    input: &str,
) -> nom::IResult<&str, SyntaxRule, nom::error::VerboseError<&str>> {
    let (input, matched_meta_identifier) = parse_meta_identifier(input)?;
    let (input, _) = parse_symbol(input, Symbol::DefinitionSeparator)?;
    let (input, rest) = nom::bytes::complete::take_while(is_gap_separator)(input)?;
    Ok((input, SyntaxRule::from(rest)))
}

pub fn parse_syntax(input: &str) -> nom::IResult<&str, Syntax, nom::error::VerboseError<&str>> {
    let (input_leftover, matched) = nom::multi::many0(parse_syntax_rule)(input)?;
    Ok((input_leftover, Syntax::from(matched)))
}

pub fn parse_ebnf(input: &str) -> nom::IResult<&str, Syntax, nom::error::VerboseError<&str>> {
    let (input, g) = nom::combinator::all_consuming(parse_syntax)(input)?;
    Ok((input, g))
}

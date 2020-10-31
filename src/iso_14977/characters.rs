pub fn parse_letter(input: &str) -> nom::IResult<&str, String, nom::error::VerboseError<&str>> {
    let (input_leftover, matched) = nom::character::complete::one_of(
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ",
    )(input)?;
    let result = String::from(matched);
    Ok((input_leftover, result))
}

pub fn parse_decimal_digit(
    input: &str,
) -> nom::IResult<&str, String, nom::error::VerboseError<&str>> {
    let (input_leftover, matched) = nom::character::complete::one_of("0123456789")(input)?;
    let result = String::from(matched);
    Ok((input_leftover, result))
}

pub fn parse_concatenate_symbol(
    input: &str,
) -> nom::IResult<&str, String, nom::error::VerboseError<&str>> {
    let (input_leftover, matched) = nom::bytes::complete::tag(",")(input)?;
    let result = String::from(matched);
    Ok((input_leftover, result))
}

pub fn parse_defining_symbol(
    input: &str,
) -> nom::IResult<&str, String, nom::error::VerboseError<&str>> {
    let (input_leftover, matched) = nom::bytes::complete::tag("=")(input)?;
    let result = String::from(matched);
    Ok((input_leftover, result))
}

pub fn parse_definition_seperator_symbol(
    input: &str,
) -> nom::IResult<&str, String, nom::error::VerboseError<&str>> {
    let (input_leftover, matched) = nom::branch::alt((
        nom::bytes::complete::tag(","),
        nom::bytes::complete::tag("/"),
        nom::bytes::complete::tag("!"),
    ))(input)?;
    let result = String::from(matched);
    Ok((input_leftover, result))
}

pub fn parse_end_comment_symbol(
    input: &str,
) -> nom::IResult<&str, String, nom::error::VerboseError<&str>> {
    let (input_leftover, matched) = nom::bytes::complete::tag("*)")(input)?;
    let result = String::from(matched);
    Ok((input_leftover, result))
}

pub fn parse_end_group_symbol(
    input: &str,
) -> nom::IResult<&str, String, nom::error::VerboseError<&str>> {
    let (input_leftover, matched) = nom::bytes::complete::tag(")")(input)?;
    let result = String::from(matched);
    Ok((input_leftover, result))
}

pub fn parse_end_option_symbol(
    input: &str,
) -> nom::IResult<&str, String, nom::error::VerboseError<&str>> {
    let (input_leftover, matched) = nom::branch::alt((
        (nom::bytes::complete::tag("]")),
        (nom::bytes::complete::tag("/)")),
    ))(input)?;
    let result = String::from(matched);
    Ok((input_leftover, result))
}

pub fn parse_end_repeat_symbol(
    input: &str,
) -> nom::IResult<&str, String, nom::error::VerboseError<&str>> {
    let (input_leftover, matched) = nom::branch::alt((
        (nom::bytes::complete::tag("}")),
        (nom::bytes::complete::tag(":)")),
    ))(input)?;
    let result = String::from(matched);
    Ok((input_leftover, result))
}

pub fn parse_except_symbol(
    input: &str,
) -> nom::IResult<&str, String, nom::error::VerboseError<&str>> {
    let (input_leftover, matched) = nom::bytes::complete::tag("-")(input)?;
    let result = String::from(matched);
    Ok((input_leftover, result))
}

pub fn parse_first_quote_symbol(
    input: &str,
) -> nom::IResult<&str, String, nom::error::VerboseError<&str>> {
    let (input_leftover, matched) = nom::bytes::complete::tag("'")(input)?;
    let result = String::from(matched);
    Ok((input_leftover, result))
}

pub fn parse_repetition_symbol(
    input: &str,
) -> nom::IResult<&str, String, nom::error::VerboseError<&str>> {
    let (input_leftover, matched) = nom::bytes::complete::tag("*")(input)?;
    let result = String::from(matched);
    Ok((input_leftover, result))
}

pub fn parse_second_quote_symbol(
    input: &str,
) -> nom::IResult<&str, String, nom::error::VerboseError<&str>> {
    let (input_leftover, matched) = nom::bytes::complete::tag("\"")(input)?;
    let result = String::from(matched);
    Ok((input_leftover, result))
}

pub fn parse_special_sequence_symbol(
    input: &str,
) -> nom::IResult<&str, String, nom::error::VerboseError<&str>> {
    let (input_leftover, matched) = nom::bytes::complete::tag("?")(input)?;
    let result = String::from(matched);
    Ok((input_leftover, result))
}

pub fn parse_start_comment_symbol(
    input: &str,
) -> nom::IResult<&str, String, nom::error::VerboseError<&str>> {
    let (input_leftover, matched) = nom::bytes::complete::tag("(*")(input)?;
    let result = String::from(matched);
    Ok((input_leftover, result))
}

pub fn parse_group_symbol(
    input: &str,
) -> nom::IResult<&str, String, nom::error::VerboseError<&str>> {
    let (input_leftover, matched) = nom::bytes::complete::tag("(")(input)?;
    let result = String::from(matched);
    Ok((input_leftover, result))
}

pub fn parse_start_option_symbol(
    input: &str,
) -> nom::IResult<&str, String, nom::error::VerboseError<&str>> {
    let (input_leftover, matched) = nom::branch::alt((
        nom::bytes::complete::tag("["),
        nom::bytes::complete::tag("(/"),
    ))(input)?;
    let result = String::from(matched);
    Ok((input_leftover, result))
}

pub fn parse_start_repeat_symbol(
    input: &str,
) -> nom::IResult<&str, String, nom::error::VerboseError<&str>> {
    let (input_leftover, matched) = nom::branch::alt((
        nom::bytes::complete::tag("{"),
        nom::bytes::complete::tag("(:"),
    ))(input)?;
    let result = String::from(matched);
    Ok((input_leftover, result))
}

pub fn parse_terminator_symbol(
    input: &str,
) -> nom::IResult<&str, String, nom::error::VerboseError<&str>> {
    let (input_leftover, matched) = nom::branch::alt((
        nom::bytes::complete::tag(";"),
        nom::bytes::complete::tag("."),
    ))(input)?;
    let result = String::from(matched);
    Ok((input_leftover, result))
}

pub fn parse_other_character(
    input: &str,
) -> nom::IResult<&str, String, nom::error::VerboseError<&str>> {
    let (input_leftover, matched) = nom::character::complete::one_of(" :+_%@&#$<>\\ˆ`˜")(input)?;
    let result = String::from(matched);
    Ok((input_leftover, result))
}

pub fn parse_space_character(
    input: &str,
) -> nom::IResult<&str, String, nom::error::VerboseError<&str>> {
    let (input_leftover, matched) = nom::bytes::complete::tag(" ")(input)?;
    let result = String::from(matched);
    Ok((input_leftover, result))
}

pub fn parse_horizontal_tabulation_character(
    input: &str,
) -> nom::IResult<&str, String, nom::error::VerboseError<&str>> {
    let (input_leftover, matched) = nom::bytes::complete::tag("\t")(input)?;
    let result = String::from(matched);
    Ok((input_leftover, result))
}

pub fn parse_vertical_tabulation_character(
    input: &str,
) -> nom::IResult<&str, String, nom::error::VerboseError<&str>> {
    let (input_leftover, matched) = nom::branch::alt((
        nom::bytes::complete::tag("\\v"),
        nom::bytes::complete::tag("\u{000B}"),
    ))(input)?;
    let result = String::from(matched);
    Ok((input_leftover, result))
}

pub fn parse_form_feed(input: &str) -> nom::IResult<&str, String, nom::error::VerboseError<&str>> {
    let (input_leftover, matched) = nom::bytes::complete::tag("\\f")(input)?;
    let result = String::from(matched);
    Ok((input_leftover, result))
}

pub fn parse_new_line(input: &str) -> nom::IResult<&str, String, nom::error::VerboseError<&str>> {
    let (input_leftover, matched) = nom::bytes::complete::tag("\n")(input)?;
    let result = String::from(matched);
    Ok((input_leftover, result))
}

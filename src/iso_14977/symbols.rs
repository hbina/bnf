#[derive(Debug, PartialOrd, PartialEq, Eq, Ord)]
pub enum Symbol {
    Terminator,
    Defining,
    DefinitionSeparator,
    Concatenate,
    Except,
    Repetition,
}

/// TODO: Introduce a custom error type for this?
impl std::convert::TryFrom<char> for Symbol {
    type Error = String;

    fn try_from(from: char) -> Result<Self, String> {
        match from {
            '*' => Ok(Symbol::Repetition),
            '-' => Ok(Symbol::Except),
            ',' => Ok(Symbol::Concatenate),
            '|' => Ok(Symbol::DefinitionSeparator),
            '=' => Ok(Symbol::Defining),
            ';' => Ok(Symbol::Terminator),
            _ => Err(format!("The character `{}` is not a valid operator for Extended BNF. See Section 4 of ISO 14977.", from))
        }
    }
}

impl std::convert::TryFrom<&str> for Symbol {
    type Error = String;

    fn try_from(from: &str) -> Result<Self, String> {
        match from {
            "*" => Ok(Symbol::Repetition),
            "-" => Ok(Symbol::Except),
            "," => Ok(Symbol::Concatenate),
            "|" => Ok(Symbol::DefinitionSeparator),
            "=" => Ok(Symbol::Defining),
            ";" => Ok(Symbol::Terminator),
            _ => Err(format!("The character `{}` is not a valid operator for Extended BNF. See Section 4 of ISO 14977.", from))
        }
    }
}

impl From<Symbol> for char {
    fn from(from: Symbol) -> Self {
        match from {
            Symbol::Terminator => ';',
            Symbol::Defining => '=',
            Symbol::DefinitionSeparator => '|',
            Symbol::Concatenate => ',',
            Symbol::Except => '-',
            Symbol::Repetition => '*',
        }
    }
}

impl From<Symbol> for &str {
    fn from(from: Symbol) -> Self {
        match from {
            Symbol::Terminator => ";",
            Symbol::Defining => "=",
            Symbol::DefinitionSeparator => "|",
            Symbol::Concatenate => ",",
            Symbol::Except => "-",
            Symbol::Repetition => "*",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_ebnf_operator_precedence() {
        assert!(Symbol::Repetition > Symbol::Except);
    }

    #[test]
    fn test_convert_from_char() {
        assert_eq!(Symbol::try_from('*'), Ok(Symbol::Repetition));
        assert_eq!(Symbol::try_from('-'), Ok(Symbol::Except));
        assert_eq!(Symbol::try_from(','), Ok(Symbol::Concatenate));
        assert_eq!(Symbol::try_from('|'), Ok(Symbol::DefinitionSeparator));
        assert_eq!(Symbol::try_from('='), Ok(Symbol::Defining));
        assert_eq!(Symbol::try_from(';'), Ok(Symbol::Terminator));
    }
}

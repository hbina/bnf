pub struct MetaIdentifier {
    name: String,
}

impl From<&str> for MetaIdentifier {
    fn from(_: &str) -> Self {
        unimplemented!()
    }
}

impl From<String> for MetaIdentifier {
    fn from(from: String) -> Self {
        Self { name: from }
    }
}

pub struct Syntax {
    vec: Vec<SyntaxRule>,
}

impl From<Vec<SyntaxRule>> for Syntax {
    fn from(_: Vec<SyntaxRule>) -> Self {
        unimplemented!()
    }
}

pub struct SyntaxRule {
    identifier: MetaIdentifier,
    definitions: DefinitionList,
}

impl From<&str> for SyntaxRule {
    fn from(_: &str) -> Self {
        unimplemented!()
    }
}

pub type DefinitionList = Vec<SingleDefinition>;
pub type SingleDefinition = Vec<SyntacticFactor>;

enum SyntacticTerm {
    A(SyntacticFactor),
    B(SyntacticFactor, SyntacticException),
}

/// NOTE: The specification for this requires a bit more work to avoid Russell-like paradoxes.
/// In particular, I suspect its creation must be checked against the LHS operand of the
/// exception operator.
struct SyntacticException {}

enum SyntacticFactor {
    A(u32, SyntacticPrimary),
    B(SyntacticPrimary),
}

enum SyntacticPrimary {
    OptionalSequence(DefinitionList),
    RepeatedSequence(DefinitionList),
    GroupedSequence(DefinitionList),
    MetaIdentifier(String),
    TerminalString(String),
    // TODO
    // SpecialSequence,
    EmptySequence,
}

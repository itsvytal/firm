/// Formatting options when generating Firm DSL.
#[derive(Debug, Clone)]
pub struct GeneratorOptions {
    pub indent_style: IndentStyle,
    pub field_order: FieldOrder,
    pub blank_lines_between_entities: bool,
}

impl Default for GeneratorOptions {
    fn default() -> Self {
        Self {
            indent_style: IndentStyle::Spaces(4),
            field_order: FieldOrder::Alphabetical,
            blank_lines_between_entities: true,
        }
    }
}

/// Which kinds of indents to use when generating DSL.
#[derive(Debug, Clone)]
pub enum IndentStyle {
    Spaces(usize),
    Tabs,
}

/// How to order entity fields.
#[derive(Debug, Clone)]
pub enum FieldOrder {
    Unordered,
    Alphabetical,
}

impl IndentStyle {
    pub fn indent_string(&self, level: usize) -> String {
        match self {
            IndentStyle::Spaces(size) => " ".repeat(level * size),
            IndentStyle::Tabs => "\t".repeat(level),
        }
    }
}

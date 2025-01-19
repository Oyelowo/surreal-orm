pub const KEYWORDS: [&str; 51] = [
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "Self", "self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", "async", "await", "dyn", "abstract", "become", "box", "do", "final", "macro",
    "override", "priv", "unsized", "virtual", "yield", "try", "union",
];

pub struct RustReservedKeyword;

impl RustReservedKeyword {
    pub fn is_keyword(word: impl Into<String>) -> bool {
        let word: String = word.into();
        KEYWORDS.contains(&word.as_str())
    }
}

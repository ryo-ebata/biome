use biome_json_syntax::{JsonSyntaxKind, JsonSyntaxToken};

pub use crate::generated::node_factory::*;

pub fn token(kind: JsonSyntaxKind) -> JsonSyntaxToken {
    if let Some(text) = kind.to_string() {
        JsonSyntaxToken::new_detached(kind, text, [], [])
    } else {
        panic!("token kind {kind:?} cannot be transformed to text")
    }
}

pub fn ident(text: &str) -> JsonSyntaxToken {
    JsonSyntaxToken::new_detached(JsonSyntaxKind::IDENT, text, [], [])
}

use crate::{exports::yggdrasil::json::ast::Guest, peg::core::cst::SyntaxNode};

mod ast;

pub struct Json5Host {}

pub struct JsonNumberNative {
    node: SyntaxNode,
}

pub struct JsonStringNative {
    node: SyntaxNode,
}

pub struct JsonArrayNative {
    node: SyntaxNode,
}

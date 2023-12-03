mod ast;

pub struct Json5Host {}

pub struct JsonNumberNative {
    node: crate::peg::core::cst::SyntaxNode,
}

pub struct JsonStringNative {
    node: crate::peg::core::cst::SyntaxNode,
}

pub struct JsonArrayNative {
    node: crate::peg::core::cst::SyntaxNode,
}

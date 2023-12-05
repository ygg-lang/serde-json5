use super::*;
use crate::exports::yggdrasil::json::ast::*;

#[automatically_derived]
impl Guest for Json5Host {
    type JsonNumberNode = JsonNumberNative;
    type JsonStringNode = JsonStringNative;
    type JsonArrayNode = JsonArrayNative;
}
#[automatically_derived]
impl GuestJsonNumberNode for JsonNumberNative {}
#[automatically_derived]
impl GuestJsonStringNode for JsonStringNative {}

#[automatically_derived]
impl GuestJsonArrayNode for JsonArrayNative {
    fn ctor(super_: SyntaxNode) -> Result<JsonArrayNode, ParseError> {
        Ok(JsonArrayNode::new(Self { node: super_ }))
    }

    fn parse_string(text: String, offset: u32) -> Result<JsonArrayNode, ParseError> {
        Ok(JsonArrayNode::new(Self { node: SyntaxNode::ctor(&text, offset)? }))
    }

    fn get_text(&self) -> String {
        self.node.get_text()
    }

    fn item(&self) -> Vec<JsonNode> {
        let mut children = Vec::with_capacity(self.node.count_children() as usize);
        let mut iter = self.node.get_children(false);
        loop {
            match iter.next() {
                Some(s) => match s.get_rule().get_tag() {
                    "string" => children.push(JsonNode::Str(JsonStringNode::new(JsonStringNative { node: s }))),
                    #[cfg(debug_assertions)]
                    s => unreachable!(
                        "branch tag `{}` is not possible here, check whether the grammar version is correct",
                        s.get_rule().get_tag()
                    ),
                    _ => break,
                },
                None => break,
            }
        }
        return children;
    }
}

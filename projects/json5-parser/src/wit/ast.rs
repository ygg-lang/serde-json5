use super::*;
use crate::{
    codegen::{Json5Parser, Json5Rule},
    exports::yggdrasil::json::ast::{
        GuestJsonArrayNode, GuestJsonNumberNode, GuestJsonStringNode, JsonArrayNode, JsonNode, ParseError,
    },
};

impl Guest for Json5Host {
    type JsonNumberNode = JsonNumberNative;
    type JsonStringNode = JsonStringNative;
    type JsonArrayNode = JsonArrayNative;
}

impl GuestJsonNumberNode for JsonNumberNative {}

impl GuestJsonStringNode for JsonStringNative {}

impl GuestJsonArrayNode for JsonArrayNative {
    fn ctor(super_: SyntaxNode) -> Result<JsonArrayNode, ParseError> {
        Ok(JsonArrayNode::new(Self { node: super_ }))
    }

    fn parse_string(text: String, offset: u32) -> Result<JsonArrayNode, ParseError> {
        Json5Parser::parse_cst(input, Json5Rule::Value)?
    }

    fn item(&self) -> Vec<JsonNode> {
        todo!()
    }
}

use super::*;
use crate::exports::yggdrasil::json5::ast::*;
use wasi_yggdrasil::exports::peg::core::cst::SyntaxNode;

#[automatically_derived]
impl Guest for Json5Host {
    fn parse_json5_value(text: String, offset: u32) -> Result<Json5ValueNode, ParseError> {
        todo!()
    }
    fn parse_json5_object_key(text: String, offset: u32) -> Result<Json5ObjectKeyNode, ParseError> {
        todo!()
    }
    fn parse_json5_string(text: String, offset: u32) -> Result<Json5StringNode, ParseError> {
        todo!()
    }
    fn parse_json5_double_string_element(text: String, offset: u32) -> Result<Json5DoubleStringElementNode, ParseError> {
        todo!()
    }
    fn parse_json5_single_string_element(text: String, offset: u32) -> Result<Json5SingleStringElementNode, ParseError> {
        todo!()
    }
    fn parse_json5_boolean(text: String, offset: u32) -> Result<Json5BooleanNode, ParseError> {
        todo!()
    }
    fn parse_json5_object(text: String, offset: u32) -> Result<Json5ObjectNode, ParseError> {
        todo!()
    }
    fn parse_json5_object_pair(text: String, offset: u32) -> Result<Json5ObjectPairNode, ParseError> {
        todo!()
    }
    fn parse_json5_array(text: String, offset: u32) -> Result<Json5ArrayNode, ParseError> {
        todo!()
    }
    fn parse_json5_hex_escape(text: String, offset: u32) -> Result<Json5HexEscapeNode, ParseError> {
        todo!()
    }
    fn parse_json5_any_escape(text: String, offset: u32) -> Result<Json5AnyEscapeNode, ParseError> {
        todo!()
    }
    fn parse_json5_double_string_text(text: String, offset: u32) -> Result<Json5DoubleStringTextNode, ParseError> {
        todo!()
    }
    fn parse_json5_single_string_text(text: String, offset: u32) -> Result<Json5SingleStringTextNode, ParseError> {
        todo!()
    }
    fn parse_json5_number(text: String, offset: u32) -> Result<Json5NumberNode, ParseError> {
        todo!()
    }
    fn parse_json5_null(text: String, offset: u32) -> Result<Json5NullNode, ParseError> {
        todo!()
    }
    fn parse_json5_identifier(text: String, offset: u32) -> Result<Json5IdentifierNode, ParseError> {
        todo!()
    }
    fn parse_json5_colon(text: String, offset: u32) -> Result<Json5ColonNode, ParseError> {
        todo!()
    }
    fn parse_json5_comma(text: String, offset: u32) -> Result<Json5CommaNode, ParseError> {
        todo!()
    }
    fn parse_json5_comment(text: String, offset: u32) -> Result<Json5CommentNode, ParseError> {
        todo!()
    }
    fn parse_json5_white_space(text: String, offset: u32) -> Result<Json5WhiteSpaceNode, ParseError> {
        todo!()
    }
    fn parse_json5_string0(text: String, offset: u32) -> Result<Json5String0Node, ParseError> {
        todo!()
    }
    fn parse_json5_string1(text: String, offset: u32) -> Result<Json5String1Node, ParseError> {
        todo!()
    }
    fn parse_json5_boolean0(text: String, offset: u32) -> Result<Json5Boolean0Node, ParseError> {
        todo!()
    }
    fn parse_json5_boolean1(text: String, offset: u32) -> Result<Json5Boolean1Node, ParseError> {
        todo!()
    }
}
#[automatically_derived]
impl Json5ObjectNode {
    pub fn from_node(node: Node<NativeSyntaxData>) -> Self {
        Self { node: SyntaxNode::new(node) }
    }
}
#[automatically_derived]
impl Json5ObjectPairNode {
    pub fn from_node(node: Node<NativeSyntaxData>) -> Self {
        Self { node: SyntaxNode::new(node) }
    }
}
#[automatically_derived]
impl Json5ArrayNode {
    pub fn from_node(node: Node<NativeSyntaxData>) -> Self {
        Self { node: SyntaxNode::new(node) }
    }
}
#[automatically_derived]
impl Json5HexEscapeNode {
    pub fn from_node(node: Node<NativeSyntaxData>) -> Self {
        Self { node: SyntaxNode::new(node) }
    }
}
#[automatically_derived]
impl Json5AnyEscapeNode {
    pub fn from_node(node: Node<NativeSyntaxData>) -> Self {
        Self { node: SyntaxNode::new(node) }
    }
}
#[automatically_derived]
impl Json5DoubleStringTextNode {
    pub fn from_node(node: Node<NativeSyntaxData>) -> Self {
        Self { node: SyntaxNode::new(node) }
    }
}
#[automatically_derived]
impl Json5SingleStringTextNode {
    pub fn from_node(node: Node<NativeSyntaxData>) -> Self {
        Self { node: SyntaxNode::new(node) }
    }
}
#[automatically_derived]
impl Json5NumberNode {
    pub fn from_node(node: Node<NativeSyntaxData>) -> Self {
        Self { node: SyntaxNode::new(node) }
    }
}
#[automatically_derived]
impl Json5NullNode {
    pub fn from_node(node: Node<NativeSyntaxData>) -> Self {
        Self { node: SyntaxNode::new(node) }
    }
}
#[automatically_derived]
impl Json5IdentifierNode {
    pub fn from_node(node: Node<NativeSyntaxData>) -> Self {
        Self { node: SyntaxNode::new(node) }
    }
}
#[automatically_derived]
impl Json5ColonNode {
    pub fn from_node(node: Node<NativeSyntaxData>) -> Self {
        Self { node: SyntaxNode::new(node) }
    }
}
#[automatically_derived]
impl Json5CommaNode {
    pub fn from_node(node: Node<NativeSyntaxData>) -> Self {
        Self { node: SyntaxNode::new(node) }
    }
}
#[automatically_derived]
impl Json5CommentNode {
    pub fn from_node(node: Node<NativeSyntaxData>) -> Self {
        Self { node: SyntaxNode::new(node) }
    }
}
#[automatically_derived]
impl Json5WhiteSpaceNode {
    pub fn from_node(node: Node<NativeSyntaxData>) -> Self {
        Self { node: SyntaxNode::new(node) }
    }
}
#[automatically_derived]
impl Json5String0Node {
    pub fn from_node(node: Node<NativeSyntaxData>) -> Self {
        Self { node: SyntaxNode::new(node) }
    }
}
#[automatically_derived]
impl Json5String1Node {
    pub fn from_node(node: Node<NativeSyntaxData>) -> Self {
        Self { node: SyntaxNode::new(node) }
    }
}
#[automatically_derived]
impl Json5Boolean0Node {
    pub fn from_node(node: Node<NativeSyntaxData>) -> Self {
        Self { node: SyntaxNode::new(node) }
    }
}
#[automatically_derived]
impl Json5Boolean1Node {
    pub fn from_node(node: Node<NativeSyntaxData>) -> Self {
        Self { node: SyntaxNode::new(node) }
    }
}

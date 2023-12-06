use super::*;
use crate::exports::yggdrasil::json5::ast::*;
use wasi_yggdrasil::exports::peg::core::cst::SyntaxNode;

#[automatically_derived]
impl Guest for Json5Host {
    type Json5ObjectNode = ObjectNative;
    type Json5ObjectPairNode = ObjectPairNative;
    type Json5ArrayNode = ArrayNative;
    type Json5HexEscapeNode = HexEscapeNative;
    type Json5AnyEscapeNode = AnyEscapeNative;
    type Json5DoubleStringTextNode = DoubleStringTextNative;
    type Json5SingleStringTextNode = SingleStringTextNative;
    type Json5NumberNode = NumberNative;
    type Json5NullNode = NullNative;
    type Json5IdentifierNode = IdentifierNative;
    type Json5ColonNode = ColonNative;
    type Json5CommaNode = CommaNative;
    type Json5CommentNode = CommentNative;
    type Json5WhiteSpaceNode = WhiteSpaceNative;
    type Json5String0Node = String0Native;
    type Json5String1Node = String1Native;
    type Json5Boolean0Node = Boolean0Native;
    type Json5Boolean1Node = Boolean1Native;
}
#[automatically_derived]
impl GuestJson5ObjectNode for ObjectNative {
    fn ctor(super_: SyntaxNode) -> Result<Json5ObjectNode, ParseError> {
        Ok(Json5ObjectNode::new(Self { node: super_ }))
    }
    fn parse_string(text: String, offset: u32) -> Result<Json5ObjectNode, ParseError> {
        let input = Rc::from(text);
        let language = NativeLanguage { name: "json5", glob: &["*.json5"] };
        let mut tree = crate::wit::parse_cst::parse_cst(&input, Json5Token::Object)?;
        Ok(Json5ObjectNode::new(Self { node: SyntaxNode::new(NativeSyntaxData::new(input, tree.next().unwrap(), &language))? }))
    }
    fn get_text(&self) -> String {
        self.node.get_text()
    }
}
#[automatically_derived]
impl GuestJson5ObjectPairNode for ObjectPairNative {
    fn ctor(super_: SyntaxNode) -> Result<Json5ObjectPairNode, ParseError> {
        Ok(Json5ObjectPairNode::new(Self { node: super_ }))
    }
    fn parse_string(text: String, offset: u32) -> Result<Json5ObjectPairNode, ParseError> {
        let input = Rc::from(text);
        let language = NativeLanguage { name: "json5", glob: &["*.json5"] };
        let mut tree = crate::wit::parse_cst::parse_cst(&input, Json5Token::Object)?;
        Ok(Json5ObjectPairNode::new(Self {
            node: SyntaxNode::new(NativeSyntaxData::new(input, tree.next().unwrap(), &language))?,
        }))
    }
    fn get_text(&self) -> String {
        self.node.get_text()
    }
}
#[automatically_derived]
impl GuestJson5ArrayNode for ArrayNative {
    fn ctor(super_: SyntaxNode) -> Result<Json5ArrayNode, ParseError> {
        Ok(Json5ArrayNode::new(Self { node: super_ }))
    }
    fn parse_string(text: String, offset: u32) -> Result<Json5ArrayNode, ParseError> {
        let input = Rc::from(text);
        let language = NativeLanguage { name: "json5", glob: &["*.json5"] };
        let mut tree = crate::wit::parse_cst::parse_cst(&input, Json5Token::Object)?;
        Ok(Json5ArrayNode::new(Self { node: SyntaxNode::new(NativeSyntaxData::new(input, tree.next().unwrap(), &language))? }))
    }
    fn get_text(&self) -> String {
        self.node.get_text()
    }
}
#[automatically_derived]
impl GuestJson5HexEscapeNode for HexEscapeNative {
    fn ctor(super_: SyntaxNode) -> Result<Json5HexEscapeNode, ParseError> {
        Ok(Json5HexEscapeNode::new(Self { node: super_ }))
    }
    fn parse_string(text: String, offset: u32) -> Result<Json5HexEscapeNode, ParseError> {
        let input = Rc::from(text);
        let language = NativeLanguage { name: "json5", glob: &["*.json5"] };
        let mut tree = crate::wit::parse_cst::parse_cst(&input, Json5Token::Object)?;
        Ok(Json5HexEscapeNode::new(Self {
            node: SyntaxNode::new(NativeSyntaxData::new(input, tree.next().unwrap(), &language))?,
        }))
    }
    fn get_text(&self) -> String {
        self.node.get_text()
    }
}
#[automatically_derived]
impl GuestJson5AnyEscapeNode for AnyEscapeNative {
    fn ctor(super_: SyntaxNode) -> Result<Json5AnyEscapeNode, ParseError> {
        Ok(Json5AnyEscapeNode::new(Self { node: super_ }))
    }
    fn parse_string(text: String, offset: u32) -> Result<Json5AnyEscapeNode, ParseError> {
        let input = Rc::from(text);
        let language = NativeLanguage { name: "json5", glob: &["*.json5"] };
        let mut tree = crate::wit::parse_cst::parse_cst(&input, Json5Token::Object)?;
        Ok(Json5AnyEscapeNode::new(Self {
            node: SyntaxNode::new(NativeSyntaxData::new(input, tree.next().unwrap(), &language))?,
        }))
    }
    fn get_text(&self) -> String {
        self.node.get_text()
    }
}
#[automatically_derived]
impl GuestJson5DoubleStringTextNode for DoubleStringTextNative {
    fn ctor(super_: SyntaxNode) -> Result<Json5DoubleStringTextNode, ParseError> {
        Ok(Json5DoubleStringTextNode::new(Self { node: super_ }))
    }
    fn parse_string(text: String, offset: u32) -> Result<Json5DoubleStringTextNode, ParseError> {
        let input = Rc::from(text);
        let language = NativeLanguage { name: "json5", glob: &["*.json5"] };
        let mut tree = crate::wit::parse_cst::parse_cst(&input, Json5Token::Object)?;
        Ok(Json5DoubleStringTextNode::new(Self {
            node: SyntaxNode::new(NativeSyntaxData::new(input, tree.next().unwrap(), &language))?,
        }))
    }
    fn get_text(&self) -> String {
        self.node.get_text()
    }
}
#[automatically_derived]
impl GuestJson5SingleStringTextNode for SingleStringTextNative {
    fn ctor(super_: SyntaxNode) -> Result<Json5SingleStringTextNode, ParseError> {
        Ok(Json5SingleStringTextNode::new(Self { node: super_ }))
    }
    fn parse_string(text: String, offset: u32) -> Result<Json5SingleStringTextNode, ParseError> {
        let input = Rc::from(text);
        let language = NativeLanguage { name: "json5", glob: &["*.json5"] };
        let mut tree = crate::wit::parse_cst::parse_cst(&input, Json5Token::Object)?;
        Ok(Json5SingleStringTextNode::new(Self {
            node: SyntaxNode::new(NativeSyntaxData::new(input, tree.next().unwrap(), &language))?,
        }))
    }
    fn get_text(&self) -> String {
        self.node.get_text()
    }
}
#[automatically_derived]
impl GuestJson5NumberNode for NumberNative {
    fn ctor(super_: SyntaxNode) -> Result<Json5NumberNode, ParseError> {
        Ok(Json5NumberNode::new(Self { node: super_ }))
    }
    fn parse_string(text: String, offset: u32) -> Result<Json5NumberNode, ParseError> {
        let input = Rc::from(text);
        let language = NativeLanguage { name: "json5", glob: &["*.json5"] };
        let mut tree = crate::wit::parse_cst::parse_cst(&input, Json5Token::Object)?;
        Ok(Json5NumberNode::new(Self { node: SyntaxNode::new(NativeSyntaxData::new(input, tree.next().unwrap(), &language))? }))
    }
    fn get_text(&self) -> String {
        self.node.get_text()
    }
}
#[automatically_derived]
impl GuestJson5NullNode for NullNative {
    fn ctor(super_: SyntaxNode) -> Result<Json5NullNode, ParseError> {
        Ok(Json5NullNode::new(Self { node: super_ }))
    }
    fn parse_string(text: String, offset: u32) -> Result<Json5NullNode, ParseError> {
        let input = Rc::from(text);
        let language = NativeLanguage { name: "json5", glob: &["*.json5"] };
        let mut tree = crate::wit::parse_cst::parse_cst(&input, Json5Token::Object)?;
        Ok(Json5NullNode::new(Self { node: SyntaxNode::new(NativeSyntaxData::new(input, tree.next().unwrap(), &language))? }))
    }
    fn get_text(&self) -> String {
        self.node.get_text()
    }
}
#[automatically_derived]
impl GuestJson5IdentifierNode for IdentifierNative {
    fn ctor(super_: SyntaxNode) -> Result<Json5IdentifierNode, ParseError> {
        Ok(Json5IdentifierNode::new(Self { node: super_ }))
    }
    fn parse_string(text: String, offset: u32) -> Result<Json5IdentifierNode, ParseError> {
        let input = Rc::from(text);
        let language = NativeLanguage { name: "json5", glob: &["*.json5"] };
        let mut tree = crate::wit::parse_cst::parse_cst(&input, Json5Token::Object)?;
        Ok(Json5IdentifierNode::new(Self {
            node: SyntaxNode::new(NativeSyntaxData::new(input, tree.next().unwrap(), &language))?,
        }))
    }
    fn get_text(&self) -> String {
        self.node.get_text()
    }
}
#[automatically_derived]
impl GuestJson5ColonNode for ColonNative {
    fn ctor(super_: SyntaxNode) -> Result<Json5ColonNode, ParseError> {
        Ok(Json5ColonNode::new(Self { node: super_ }))
    }
    fn parse_string(text: String, offset: u32) -> Result<Json5ColonNode, ParseError> {
        let input = Rc::from(text);
        let language = NativeLanguage { name: "json5", glob: &["*.json5"] };
        let mut tree = crate::wit::parse_cst::parse_cst(&input, Json5Token::Object)?;
        Ok(Json5ColonNode::new(Self { node: SyntaxNode::new(NativeSyntaxData::new(input, tree.next().unwrap(), &language))? }))
    }
    fn get_text(&self) -> String {
        self.node.get_text()
    }
}
#[automatically_derived]
impl GuestJson5CommaNode for CommaNative {
    fn ctor(super_: SyntaxNode) -> Result<Json5CommaNode, ParseError> {
        Ok(Json5CommaNode::new(Self { node: super_ }))
    }
    fn parse_string(text: String, offset: u32) -> Result<Json5CommaNode, ParseError> {
        let input = Rc::from(text);
        let language = NativeLanguage { name: "json5", glob: &["*.json5"] };
        let mut tree = crate::wit::parse_cst::parse_cst(&input, Json5Token::Object)?;
        Ok(Json5CommaNode::new(Self { node: SyntaxNode::new(NativeSyntaxData::new(input, tree.next().unwrap(), &language))? }))
    }
    fn get_text(&self) -> String {
        self.node.get_text()
    }
}
#[automatically_derived]
impl GuestJson5CommentNode for CommentNative {
    fn ctor(super_: SyntaxNode) -> Result<Json5CommentNode, ParseError> {
        Ok(Json5CommentNode::new(Self { node: super_ }))
    }
    fn parse_string(text: String, offset: u32) -> Result<Json5CommentNode, ParseError> {
        let input = Rc::from(text);
        let language = NativeLanguage { name: "json5", glob: &["*.json5"] };
        let mut tree = crate::wit::parse_cst::parse_cst(&input, Json5Token::Object)?;
        Ok(Json5CommentNode::new(Self {
            node: SyntaxNode::new(NativeSyntaxData::new(input, tree.next().unwrap(), &language))?,
        }))
    }
    fn get_text(&self) -> String {
        self.node.get_text()
    }
}
#[automatically_derived]
impl GuestJson5WhiteSpaceNode for WhiteSpaceNative {
    fn ctor(super_: SyntaxNode) -> Result<Json5WhiteSpaceNode, ParseError> {
        Ok(Json5WhiteSpaceNode::new(Self { node: super_ }))
    }
    fn parse_string(text: String, offset: u32) -> Result<Json5WhiteSpaceNode, ParseError> {
        let input = Rc::from(text);
        let language = NativeLanguage { name: "json5", glob: &["*.json5"] };
        let mut tree = crate::wit::parse_cst::parse_cst(&input, Json5Token::Object)?;
        Ok(Json5WhiteSpaceNode::new(Self {
            node: SyntaxNode::new(NativeSyntaxData::new(input, tree.next().unwrap(), &language))?,
        }))
    }
    fn get_text(&self) -> String {
        self.node.get_text()
    }
}
#[automatically_derived]
impl GuestJson5String0Node for String0Native {
    fn ctor(super_: SyntaxNode) -> Result<Json5String0Node, ParseError> {
        Ok(Json5String0Node::new(Self { node: super_ }))
    }
    fn parse_string(text: String, offset: u32) -> Result<Json5String0Node, ParseError> {
        let input = Rc::from(text);
        let language = NativeLanguage { name: "json5", glob: &["*.json5"] };
        let mut tree = crate::wit::parse_cst::parse_cst(&input, Json5Token::Object)?;
        Ok(Json5String0Node::new(Self {
            node: SyntaxNode::new(NativeSyntaxData::new(input, tree.next().unwrap(), &language))?,
        }))
    }
    fn get_text(&self) -> String {
        self.node.get_text()
    }
}
#[automatically_derived]
impl GuestJson5String1Node for String1Native {
    fn ctor(super_: SyntaxNode) -> Result<Json5String1Node, ParseError> {
        Ok(Json5String1Node::new(Self { node: super_ }))
    }
    fn parse_string(text: String, offset: u32) -> Result<Json5String1Node, ParseError> {
        let input = Rc::from(text);
        let language = NativeLanguage { name: "json5", glob: &["*.json5"] };
        let mut tree = crate::wit::parse_cst::parse_cst(&input, Json5Token::Object)?;
        Ok(Json5String1Node::new(Self {
            node: SyntaxNode::new(NativeSyntaxData::new(input, tree.next().unwrap(), &language))?,
        }))
    }
    fn get_text(&self) -> String {
        self.node.get_text()
    }
}
#[automatically_derived]
impl GuestJson5Boolean0Node for Boolean0Native {
    fn ctor(super_: SyntaxNode) -> Result<Json5Boolean0Node, ParseError> {
        Ok(Json5Boolean0Node::new(Self { node: super_ }))
    }
    fn parse_string(text: String, offset: u32) -> Result<Json5Boolean0Node, ParseError> {
        let input = Rc::from(text);
        let language = NativeLanguage { name: "json5", glob: &["*.json5"] };
        let mut tree = crate::wit::parse_cst::parse_cst(&input, Json5Token::Object)?;
        Ok(Json5Boolean0Node::new(Self {
            node: SyntaxNode::new(NativeSyntaxData::new(input, tree.next().unwrap(), &language))?,
        }))
    }
    fn get_text(&self) -> String {
        self.node.get_text()
    }
}
#[automatically_derived]
impl GuestJson5Boolean1Node for Boolean1Native {
    fn ctor(super_: SyntaxNode) -> Result<Json5Boolean1Node, ParseError> {
        Ok(Json5Boolean1Node::new(Self { node: super_ }))
    }
    fn parse_string(text: String, offset: u32) -> Result<Json5Boolean1Node, ParseError> {
        let input = Rc::from(text);
        let language = NativeLanguage { name: "json5", glob: &["*.json5"] };
        let mut tree = crate::wit::parse_cst::parse_cst(&input, Json5Token::Object)?;
        Ok(Json5Boolean1Node::new(Self {
            node: SyntaxNode::new(NativeSyntaxData::new(input, tree.next().unwrap(), &language))?,
        }))
    }
    fn get_text(&self) -> String {
        self.node.get_text()
    }
}

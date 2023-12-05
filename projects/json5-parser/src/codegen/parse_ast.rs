use super::*;
use crate::exports::yggdrasil::language_id::ast::*;

#[automatically_derived]
impl Guest for Json5ParserHost {
    type ValueNodeNode = ValueNodeNative;
    type ObjectNodeNode = ObjectNodeNative;
    type ObjectPairNodeNode = ObjectPairNodeNative;
    type ObjectKeyNodeNode = ObjectKeyNodeNative;
    type ArrayNodeNode = ArrayNodeNative;
    type StringNodeNode = StringNodeNative;
    type DoubleStringElementNodeNode = DoubleStringElementNodeNative;
    type SingleStringElementNodeNode = SingleStringElementNodeNative;
    type HexEscapeNodeNode = HexEscapeNodeNative;
    type AnyEscapeNodeNode = AnyEscapeNodeNative;
    type DoubleStringTextNodeNode = DoubleStringTextNodeNative;
    type SingleStringTextNodeNode = SingleStringTextNodeNative;
    type NumberNodeNode = NumberNodeNative;
    type BooleanNodeNode = BooleanNodeNative;
    type NullNodeNode = NullNodeNative;
    type IdentifierNodeNode = IdentifierNodeNative;
    type ColonNodeNode = ColonNodeNative;
    type CommaNodeNode = CommaNodeNative;
    type CommentNodeNode = CommentNodeNative;
    type WhiteSpaceNodeNode = WhiteSpaceNodeNative;
    type String0NodeNode = String0NodeNative;
    type String1NodeNode = String1NodeNative;
    type Boolean0NodeNode = Boolean0NodeNative;
    type Boolean1NodeNode = Boolean1NodeNative;
}
#[automatically_derived]
impl GuestObjectNodeNode for ObjectNodeNative {
    fn ctor(super_: SyntaxNode) -> Result<ObjectNodeNode, ParseError> {
        Ok(ObjectNodeNode::new(Self { node: super_ }))
    }

    fn parse_string(text: String, offset: u32) -> Result<ObjectNodeNode, ParseError> {
        Ok(ObjectNodeNode::new(Self { node: SyntaxNode::ctor(&text, offset)? }))
    }

    fn get_text(&self) -> String {
        self.node.get_text()
    }
    fn comma(&self) -> Vec<JsonNode> {
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
    fn object_pair(&self) -> Vec<JsonNode> {
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
#[automatically_derived]
impl GuestObjectPairNodeNode for ObjectPairNodeNative {
    fn ctor(super_: SyntaxNode) -> Result<ObjectPairNodeNode, ParseError> {
        Ok(ObjectPairNodeNode::new(Self { node: super_ }))
    }

    fn parse_string(text: String, offset: u32) -> Result<ObjectPairNodeNode, ParseError> {
        Ok(ObjectPairNodeNode::new(Self { node: SyntaxNode::ctor(&text, offset)? }))
    }

    fn get_text(&self) -> String {
        self.node.get_text()
    }
}
#[automatically_derived]
impl GuestArrayNodeNode for ArrayNodeNative {
    fn ctor(super_: SyntaxNode) -> Result<ArrayNodeNode, ParseError> {
        Ok(ArrayNodeNode::new(Self { node: super_ }))
    }

    fn parse_string(text: String, offset: u32) -> Result<ArrayNodeNode, ParseError> {
        Ok(ArrayNodeNode::new(Self { node: SyntaxNode::ctor(&text, offset)? }))
    }

    fn get_text(&self) -> String {
        self.node.get_text()
    }
    fn comma(&self) -> Vec<JsonNode> {
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
    fn value(&self) -> Vec<JsonNode> {
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
#[automatically_derived]
impl GuestHexEscapeNodeNode for HexEscapeNodeNative {
    fn ctor(super_: SyntaxNode) -> Result<HexEscapeNodeNode, ParseError> {
        Ok(HexEscapeNodeNode::new(Self { node: super_ }))
    }

    fn parse_string(text: String, offset: u32) -> Result<HexEscapeNodeNode, ParseError> {
        Ok(HexEscapeNodeNode::new(Self { node: SyntaxNode::ctor(&text, offset)? }))
    }

    fn get_text(&self) -> String {
        self.node.get_text()
    }
}
#[automatically_derived]
impl GuestAnyEscapeNodeNode for AnyEscapeNodeNative {
    fn ctor(super_: SyntaxNode) -> Result<AnyEscapeNodeNode, ParseError> {
        Ok(AnyEscapeNodeNode::new(Self { node: super_ }))
    }

    fn parse_string(text: String, offset: u32) -> Result<AnyEscapeNodeNode, ParseError> {
        Ok(AnyEscapeNodeNode::new(Self { node: SyntaxNode::ctor(&text, offset)? }))
    }

    fn get_text(&self) -> String {
        self.node.get_text()
    }
}
#[automatically_derived]
impl GuestDoubleStringTextNodeNode for DoubleStringTextNodeNative {
    fn ctor(super_: SyntaxNode) -> Result<DoubleStringTextNodeNode, ParseError> {
        Ok(DoubleStringTextNodeNode::new(Self { node: super_ }))
    }

    fn parse_string(text: String, offset: u32) -> Result<DoubleStringTextNodeNode, ParseError> {
        Ok(DoubleStringTextNodeNode::new(Self { node: SyntaxNode::ctor(&text, offset)? }))
    }

    fn get_text(&self) -> String {
        self.node.get_text()
    }
}
#[automatically_derived]
impl GuestSingleStringTextNodeNode for SingleStringTextNodeNative {
    fn ctor(super_: SyntaxNode) -> Result<SingleStringTextNodeNode, ParseError> {
        Ok(SingleStringTextNodeNode::new(Self { node: super_ }))
    }

    fn parse_string(text: String, offset: u32) -> Result<SingleStringTextNodeNode, ParseError> {
        Ok(SingleStringTextNodeNode::new(Self { node: SyntaxNode::ctor(&text, offset)? }))
    }

    fn get_text(&self) -> String {
        self.node.get_text()
    }
}
#[automatically_derived]
impl GuestNumberNodeNode for NumberNodeNative {
    fn ctor(super_: SyntaxNode) -> Result<NumberNodeNode, ParseError> {
        Ok(NumberNodeNode::new(Self { node: super_ }))
    }

    fn parse_string(text: String, offset: u32) -> Result<NumberNodeNode, ParseError> {
        Ok(NumberNodeNode::new(Self { node: SyntaxNode::ctor(&text, offset)? }))
    }

    fn get_text(&self) -> String {
        self.node.get_text()
    }
}
#[automatically_derived]
impl GuestNullNodeNode for NullNodeNative {
    fn ctor(super_: SyntaxNode) -> Result<NullNodeNode, ParseError> {
        Ok(NullNodeNode::new(Self { node: super_ }))
    }

    fn parse_string(text: String, offset: u32) -> Result<NullNodeNode, ParseError> {
        Ok(NullNodeNode::new(Self { node: SyntaxNode::ctor(&text, offset)? }))
    }

    fn get_text(&self) -> String {
        self.node.get_text()
    }
}
#[automatically_derived]
impl GuestIdentifierNodeNode for IdentifierNodeNative {
    fn ctor(super_: SyntaxNode) -> Result<IdentifierNodeNode, ParseError> {
        Ok(IdentifierNodeNode::new(Self { node: super_ }))
    }

    fn parse_string(text: String, offset: u32) -> Result<IdentifierNodeNode, ParseError> {
        Ok(IdentifierNodeNode::new(Self { node: SyntaxNode::ctor(&text, offset)? }))
    }

    fn get_text(&self) -> String {
        self.node.get_text()
    }
}
#[automatically_derived]
impl GuestColonNodeNode for ColonNodeNative {
    fn ctor(super_: SyntaxNode) -> Result<ColonNodeNode, ParseError> {
        Ok(ColonNodeNode::new(Self { node: super_ }))
    }

    fn parse_string(text: String, offset: u32) -> Result<ColonNodeNode, ParseError> {
        Ok(ColonNodeNode::new(Self { node: SyntaxNode::ctor(&text, offset)? }))
    }

    fn get_text(&self) -> String {
        self.node.get_text()
    }
}
#[automatically_derived]
impl GuestCommaNodeNode for CommaNodeNative {
    fn ctor(super_: SyntaxNode) -> Result<CommaNodeNode, ParseError> {
        Ok(CommaNodeNode::new(Self { node: super_ }))
    }

    fn parse_string(text: String, offset: u32) -> Result<CommaNodeNode, ParseError> {
        Ok(CommaNodeNode::new(Self { node: SyntaxNode::ctor(&text, offset)? }))
    }

    fn get_text(&self) -> String {
        self.node.get_text()
    }
}
#[automatically_derived]
impl GuestCommentNodeNode for CommentNodeNative {
    fn ctor(super_: SyntaxNode) -> Result<CommentNodeNode, ParseError> {
        Ok(CommentNodeNode::new(Self { node: super_ }))
    }

    fn parse_string(text: String, offset: u32) -> Result<CommentNodeNode, ParseError> {
        Ok(CommentNodeNode::new(Self { node: SyntaxNode::ctor(&text, offset)? }))
    }

    fn get_text(&self) -> String {
        self.node.get_text()
    }
}
#[automatically_derived]
impl GuestWhiteSpaceNodeNode for WhiteSpaceNodeNative {
    fn ctor(super_: SyntaxNode) -> Result<WhiteSpaceNodeNode, ParseError> {
        Ok(WhiteSpaceNodeNode::new(Self { node: super_ }))
    }

    fn parse_string(text: String, offset: u32) -> Result<WhiteSpaceNodeNode, ParseError> {
        Ok(WhiteSpaceNodeNode::new(Self { node: SyntaxNode::ctor(&text, offset)? }))
    }

    fn get_text(&self) -> String {
        self.node.get_text()
    }
}
#[automatically_derived]
impl GuestString0NodeNode for String0NodeNative {
    fn ctor(super_: SyntaxNode) -> Result<String0NodeNode, ParseError> {
        Ok(String0NodeNode::new(Self { node: super_ }))
    }

    fn parse_string(text: String, offset: u32) -> Result<String0NodeNode, ParseError> {
        Ok(String0NodeNode::new(Self { node: SyntaxNode::ctor(&text, offset)? }))
    }

    fn get_text(&self) -> String {
        self.node.get_text()
    }
}
#[automatically_derived]
impl GuestString1NodeNode for String1NodeNative {
    fn ctor(super_: SyntaxNode) -> Result<String1NodeNode, ParseError> {
        Ok(String1NodeNode::new(Self { node: super_ }))
    }

    fn parse_string(text: String, offset: u32) -> Result<String1NodeNode, ParseError> {
        Ok(String1NodeNode::new(Self { node: SyntaxNode::ctor(&text, offset)? }))
    }

    fn get_text(&self) -> String {
        self.node.get_text()
    }
}
#[automatically_derived]
impl GuestBoolean0NodeNode for Boolean0NodeNative {
    fn ctor(super_: SyntaxNode) -> Result<Boolean0NodeNode, ParseError> {
        Ok(Boolean0NodeNode::new(Self { node: super_ }))
    }

    fn parse_string(text: String, offset: u32) -> Result<Boolean0NodeNode, ParseError> {
        Ok(Boolean0NodeNode::new(Self { node: SyntaxNode::ctor(&text, offset)? }))
    }

    fn get_text(&self) -> String {
        self.node.get_text()
    }
}
#[automatically_derived]
impl GuestBoolean1NodeNode for Boolean1NodeNative {
    fn ctor(super_: SyntaxNode) -> Result<Boolean1NodeNode, ParseError> {
        Ok(Boolean1NodeNode::new(Self { node: super_ }))
    }

    fn parse_string(text: String, offset: u32) -> Result<Boolean1NodeNode, ParseError> {
        Ok(Boolean1NodeNode::new(Self { node: SyntaxNode::ctor(&text, offset)? }))
    }

    fn get_text(&self) -> String {
        self.node.get_text()
    }
}

use super::*;
use crate::exports::yggdrasil::json5::ast::*;
use wasi_yggdrasil::exports::peg::core::cst::SyntaxNode;

#[automatically_derived]
impl Guest for Json5Host {
    type Json5ObjectNode = ObjectNativeNode;
    type Json5ObjectPairNode = ObjectPairNativeNode;
    type Json5ArrayNode = ArrayNativeNode;
    type Json5HexEscapeNode = HexEscapeNativeNode;
    type Json5AnyEscapeNode = AnyEscapeNativeNode;
    type Json5DoubleStringTextNode = DoubleStringTextNativeNode;
    type Json5SingleStringTextNode = SingleStringTextNativeNode;
    type Json5NumberNode = NumberNativeNode;
    type Json5NullNode = NullNativeNode;
    type Json5IdentifierNode = IdentifierNativeNode;
    type Json5ColonNode = ColonNativeNode;
    type Json5CommaNode = CommaNativeNode;
    type Json5CommentNode = CommentNativeNode;
    type Json5WhiteSpaceNode = WhiteSpaceNativeNode;
    type Json5String0Node = String0NativeNode;
    type Json5String1Node = String1NativeNode;
    type Json5Boolean0Node = Boolean0NativeNode;
    type Json5Boolean1Node = Boolean1NativeNode;
    fn parse_string(_: String, _: u32, _: Json5Token) -> Result<Json5Type, ParseError> {
        unimplemented!()
    }
}
#[automatically_derived]
impl Json5ValueNode {
    pub fn from_node(node: Node<NativeSyntaxData>) -> Result<Self, ParseError> {
        if cfg!(debug_assertions) {
            let expect = vec![String::from("")];
            let current = node.get_rule().into_inner::<NativeSyntaxRule>().name.to_string();
            if !expect.contains(&current) {
                return Err(ParseError::InvalidRule(InvalidRule { expect, current, range: node.get_range() }));
            }
        }
        let node = node.first_child().unwrap();
        let tag = match node.borrow().tag.as_ref() {
            Some(s) => unsafe { transmute::<&str, &str>(s.as_str()) },
            None => Err(ParseError::MissingRoot)?,
        };
        Some(match tag {
            "object" => Self::Object(Json5ObjectNode::from_node(node)?),
            "array" => Self::Array(Json5ArrayNode::from_node(node)?),
            "string" => Self::String(Json5StringNode::from_node(node)?),
            "number" => Self::Number(Json5NumberNode::from_node(node)?),
            "boolean" => Self::Boolean(Json5BooleanNode::from_node(node)?),
            "null" => Self::Null(Json5NullNode::from_node(node)?),
            _ => Err(ParseError::MissingRoot)?,
        })
    }
}
#[automatically_derived]
impl Json5ObjectKeyNode {
    pub fn from_node(node: Node<NativeSyntaxData>) -> Result<Self, ParseError> {
        if cfg!(debug_assertions) {
            let expect = vec![String::from("")];
            let current = node.get_rule().into_inner::<NativeSyntaxRule>().name.to_string();
            if !expect.contains(&current) {
                return Err(ParseError::InvalidRule(InvalidRule { expect, current, range: node.get_range() }));
            }
        }
        let node = node.first_child().unwrap();
        let tag = match node.borrow().tag.as_ref() {
            Some(s) => unsafe { transmute::<&str, &str>(s.as_str()) },
            None => Err(ParseError::MissingRoot)?,
        };
        Some(match tag {
            "identifier" => Self::Identifier(Json5IdentifierNode::from_node(node)?),
            "string" => Self::String(Json5StringNode::from_node(node)?),
            _ => Err(ParseError::MissingRoot)?,
        })
    }
}
#[automatically_derived]
impl Json5StringNode {
    pub fn from_node(node: Node<NativeSyntaxData>) -> Result<Self, ParseError> {
        if cfg!(debug_assertions) {
            let expect = vec![String::from("")];
            let current = node.get_rule().into_inner::<NativeSyntaxRule>().name.to_string();
            if !expect.contains(&current) {
                return Err(ParseError::InvalidRule(InvalidRule { expect, current, range: node.get_range() }));
            }
        }
        let node = node.first_child().unwrap();
        let tag = match node.borrow().tag.as_ref() {
            Some(s) => unsafe { transmute::<&str, &str>(s.as_str()) },
            None => Err(ParseError::MissingRoot)?,
        };
        Some(match tag {
            "string0" => Self::String0(Json5String0Node::from_node(node)?),
            "string1" => Self::String1(Json5String1Node::from_node(node)?),
            _ => Err(ParseError::MissingRoot)?,
        })
    }
}
#[automatically_derived]
impl Json5DoubleStringElementNode {
    pub fn from_node(node: Node<NativeSyntaxData>) -> Result<Self, ParseError> {
        if cfg!(debug_assertions) {
            let expect = vec![String::from("")];
            let current = node.get_rule().into_inner::<NativeSyntaxRule>().name.to_string();
            if !expect.contains(&current) {
                return Err(ParseError::InvalidRule(InvalidRule { expect, current, range: node.get_range() }));
            }
        }
        let node = node.first_child().unwrap();
        let tag = match node.borrow().tag.as_ref() {
            Some(s) => unsafe { transmute::<&str, &str>(s.as_str()) },
            None => Err(ParseError::MissingRoot)?,
        };
        Some(match tag {
            "hex_escape" => Self::HexEscape(Json5HexEscapeNode::from_node(node)?),
            "any_escape" => Self::AnyEscape(Json5AnyEscapeNode::from_node(node)?),
            "double_string_text" => Self::DoubleStringText(Json5DoubleStringTextNode::from_node(node)?),
            _ => Err(ParseError::MissingRoot)?,
        })
    }
}
#[automatically_derived]
impl Json5SingleStringElementNode {
    pub fn from_node(node: Node<NativeSyntaxData>) -> Result<Self, ParseError> {
        if cfg!(debug_assertions) {
            let expect = vec![String::from("")];
            let current = node.get_rule().into_inner::<NativeSyntaxRule>().name.to_string();
            if !expect.contains(&current) {
                return Err(ParseError::InvalidRule(InvalidRule { expect, current, range: node.get_range() }));
            }
        }
        let node = node.first_child().unwrap();
        let tag = match node.borrow().tag.as_ref() {
            Some(s) => unsafe { transmute::<&str, &str>(s.as_str()) },
            None => Err(ParseError::MissingRoot)?,
        };
        Some(match tag {
            "hex_escape" => Self::HexEscape(Json5HexEscapeNode::from_node(node)?),
            "any_escape" => Self::AnyEscape(Json5AnyEscapeNode::from_node(node)?),
            "single_string_text" => Self::SingleStringText(Json5SingleStringTextNode::from_node(node)?),
            _ => Err(ParseError::MissingRoot)?,
        })
    }
}
#[automatically_derived]
impl Json5BooleanNode {
    pub fn from_node(node: Node<NativeSyntaxData>) -> Result<Self, ParseError> {
        if cfg!(debug_assertions) {
            let expect = vec![String::from("")];
            let current = node.get_rule().into_inner::<NativeSyntaxRule>().name.to_string();
            if !expect.contains(&current) {
                return Err(ParseError::InvalidRule(InvalidRule { expect, current, range: node.get_range() }));
            }
        }
        let node = node.first_child().unwrap();
        let tag = match node.borrow().tag.as_ref() {
            Some(s) => unsafe { transmute::<&str, &str>(s.as_str()) },
            None => Err(ParseError::MissingRoot)?,
        };
        Some(match tag {
            "true" => Self::True(Json5Boolean0Node::from_node(node)?),
            "false" => Self::False(Json5Boolean1Node::from_node(node)?),
            _ => Err(ParseError::MissingRoot)?,
        })
    }
}

#[automatically_derived]
impl GuestJson5ObjectNode for ObjectNativeNode {
    fn ctor(base: SyntaxNode) -> Result<Json5ObjectNode, ParseError> {
        Ok(Json5ObjectNode::new(ObjectNativeNode { node: base.into_inner() }))
    }

    fn parse_string(text: String, offset: u32) -> Result<Json5ObjectNode, ParseError> {
        let text = Rc::from(text);
        let lang = NativeLanguage { name: "", glob: &[] };
        match parse_cst::parse_cst(&text, Json5Token::Object)?.next() {
            Some(s) => {
                let node = NativeSyntaxData::new(text.clone(), s, &lang);
                Ok(Json5ObjectNode::new(ObjectNativeNode { node }))
            }
            None => Err(ParseError::MissingRoot),
        }
    }

    fn get_rule(&self) -> SyntaxRule {
        self.node.get_rule()
    }

    fn get_string(&self) -> String {
        self.node.get_text()
    }

    fn get_range(&self) -> TextRange {
        self.node.get_range()
    }
}
#[automatically_derived]
impl Json5ObjectNode {
    pub fn from_node(node: Node<NativeSyntaxData>) -> Result<Self, ParseError> {
        if cfg!(debug_assertions) {
            let expect = vec![String::from("")];
            let current = node.get_rule().into_inner::<NativeSyntaxRule>().name.to_string();
            if !expect.contains(&current) {
                return Err(ParseError::InvalidRule(InvalidRule { expect, current, range: node.get_range() }));
            }
        }
        Ok(Self::new(ObjectNativeNode { node }))
    }
}

#[automatically_derived]
impl GuestJson5ObjectPairNode for ObjectPairNativeNode {
    fn ctor(base: SyntaxNode) -> Result<Json5ObjectPairNode, ParseError> {
        Ok(Json5ObjectPairNode::new(ObjectPairNativeNode { node: base.into_inner() }))
    }

    fn parse_string(text: String, offset: u32) -> Result<Json5ObjectPairNode, ParseError> {
        let text = Rc::from(text);
        let lang = NativeLanguage { name: "", glob: &[] };
        match parse_cst::parse_cst(&text, Json5Token::ObjectPair)?.next() {
            Some(s) => {
                let node = NativeSyntaxData::new(text.clone(), s, &lang);
                Ok(Json5ObjectPairNode::new(ObjectPairNativeNode { node }))
            }
            None => Err(ParseError::MissingRoot),
        }
    }

    fn get_rule(&self) -> SyntaxRule {
        self.node.get_rule()
    }

    fn get_string(&self) -> String {
        self.node.get_text()
    }

    fn get_range(&self) -> TextRange {
        self.node.get_range()
    }
}
#[automatically_derived]
impl Json5ObjectPairNode {
    pub fn from_node(node: Node<NativeSyntaxData>) -> Result<Self, ParseError> {
        if cfg!(debug_assertions) {
            let expect = vec![String::from("")];
            let current = node.get_rule().into_inner::<NativeSyntaxRule>().name.to_string();
            if !expect.contains(&current) {
                return Err(ParseError::InvalidRule(InvalidRule { expect, current, range: node.get_range() }));
            }
        }
        Ok(Self::new(ObjectPairNativeNode { node }))
    }
}

#[automatically_derived]
impl GuestJson5ArrayNode for ArrayNativeNode {
    fn ctor(base: SyntaxNode) -> Result<Json5ArrayNode, ParseError> {
        Ok(Json5ArrayNode::new(ArrayNativeNode { node: base.into_inner() }))
    }

    fn parse_string(text: String, offset: u32) -> Result<Json5ArrayNode, ParseError> {
        let text = Rc::from(text);
        let lang = NativeLanguage { name: "", glob: &[] };
        match parse_cst::parse_cst(&text, Json5Token::Array)?.next() {
            Some(s) => {
                let node = NativeSyntaxData::new(text.clone(), s, &lang);
                Ok(Json5ArrayNode::new(ArrayNativeNode { node }))
            }
            None => Err(ParseError::MissingRoot),
        }
    }

    fn get_rule(&self) -> SyntaxRule {
        self.node.get_rule()
    }

    fn get_string(&self) -> String {
        self.node.get_text()
    }

    fn get_range(&self) -> TextRange {
        self.node.get_range()
    }
}
#[automatically_derived]
impl Json5ArrayNode {
    pub fn from_node(node: Node<NativeSyntaxData>) -> Result<Self, ParseError> {
        if cfg!(debug_assertions) {
            let expect = vec![String::from("")];
            let current = node.get_rule().into_inner::<NativeSyntaxRule>().name.to_string();
            if !expect.contains(&current) {
                return Err(ParseError::InvalidRule(InvalidRule { expect, current, range: node.get_range() }));
            }
        }
        Ok(Self::new(ArrayNativeNode { node }))
    }
}

#[automatically_derived]
impl GuestJson5HexEscapeNode for HexEscapeNativeNode {
    fn ctor(base: SyntaxNode) -> Result<Json5HexEscapeNode, ParseError> {
        Ok(Json5HexEscapeNode::new(HexEscapeNativeNode { node: base.into_inner() }))
    }

    fn parse_string(text: String, offset: u32) -> Result<Json5HexEscapeNode, ParseError> {
        let text = Rc::from(text);
        let lang = NativeLanguage { name: "", glob: &[] };
        match parse_cst::parse_cst(&text, Json5Token::HexEscape)?.next() {
            Some(s) => {
                let node = NativeSyntaxData::new(text.clone(), s, &lang);
                Ok(Json5HexEscapeNode::new(HexEscapeNativeNode { node }))
            }
            None => Err(ParseError::MissingRoot),
        }
    }

    fn get_rule(&self) -> SyntaxRule {
        self.node.get_rule()
    }

    fn get_string(&self) -> String {
        self.node.get_text()
    }

    fn get_range(&self) -> TextRange {
        self.node.get_range()
    }
}
#[automatically_derived]
impl Json5HexEscapeNode {
    pub fn from_node(node: Node<NativeSyntaxData>) -> Result<Self, ParseError> {
        if cfg!(debug_assertions) {
            let expect = vec![String::from("")];
            let current = node.get_rule().into_inner::<NativeSyntaxRule>().name.to_string();
            if !expect.contains(&current) {
                return Err(ParseError::InvalidRule(InvalidRule { expect, current, range: node.get_range() }));
            }
        }
        Ok(Self::new(HexEscapeNativeNode { node }))
    }
}

#[automatically_derived]
impl GuestJson5AnyEscapeNode for AnyEscapeNativeNode {
    fn ctor(base: SyntaxNode) -> Result<Json5AnyEscapeNode, ParseError> {
        Ok(Json5AnyEscapeNode::new(AnyEscapeNativeNode { node: base.into_inner() }))
    }

    fn parse_string(text: String, offset: u32) -> Result<Json5AnyEscapeNode, ParseError> {
        let text = Rc::from(text);
        let lang = NativeLanguage { name: "", glob: &[] };
        match parse_cst::parse_cst(&text, Json5Token::AnyEscape)?.next() {
            Some(s) => {
                let node = NativeSyntaxData::new(text.clone(), s, &lang);
                Ok(Json5AnyEscapeNode::new(AnyEscapeNativeNode { node }))
            }
            None => Err(ParseError::MissingRoot),
        }
    }

    fn get_rule(&self) -> SyntaxRule {
        self.node.get_rule()
    }

    fn get_string(&self) -> String {
        self.node.get_text()
    }

    fn get_range(&self) -> TextRange {
        self.node.get_range()
    }
}
#[automatically_derived]
impl Json5AnyEscapeNode {
    pub fn from_node(node: Node<NativeSyntaxData>) -> Result<Self, ParseError> {
        if cfg!(debug_assertions) {
            let expect = vec![String::from("")];
            let current = node.get_rule().into_inner::<NativeSyntaxRule>().name.to_string();
            if !expect.contains(&current) {
                return Err(ParseError::InvalidRule(InvalidRule { expect, current, range: node.get_range() }));
            }
        }
        Ok(Self::new(AnyEscapeNativeNode { node }))
    }
}

#[automatically_derived]
impl GuestJson5DoubleStringTextNode for DoubleStringTextNativeNode {
    fn ctor(base: SyntaxNode) -> Result<Json5DoubleStringTextNode, ParseError> {
        Ok(Json5DoubleStringTextNode::new(DoubleStringTextNativeNode { node: base.into_inner() }))
    }

    fn parse_string(text: String, offset: u32) -> Result<Json5DoubleStringTextNode, ParseError> {
        let text = Rc::from(text);
        let lang = NativeLanguage { name: "", glob: &[] };
        match parse_cst::parse_cst(&text, Json5Token::DoubleStringText)?.next() {
            Some(s) => {
                let node = NativeSyntaxData::new(text.clone(), s, &lang);
                Ok(Json5DoubleStringTextNode::new(DoubleStringTextNativeNode { node }))
            }
            None => Err(ParseError::MissingRoot),
        }
    }

    fn get_rule(&self) -> SyntaxRule {
        self.node.get_rule()
    }

    fn get_string(&self) -> String {
        self.node.get_text()
    }

    fn get_range(&self) -> TextRange {
        self.node.get_range()
    }
}
#[automatically_derived]
impl Json5DoubleStringTextNode {
    pub fn from_node(node: Node<NativeSyntaxData>) -> Result<Self, ParseError> {
        if cfg!(debug_assertions) {
            let expect = vec![String::from("")];
            let current = node.get_rule().into_inner::<NativeSyntaxRule>().name.to_string();
            if !expect.contains(&current) {
                return Err(ParseError::InvalidRule(InvalidRule { expect, current, range: node.get_range() }));
            }
        }
        Ok(Self::new(DoubleStringTextNativeNode { node }))
    }
}

#[automatically_derived]
impl GuestJson5SingleStringTextNode for SingleStringTextNativeNode {
    fn ctor(base: SyntaxNode) -> Result<Json5SingleStringTextNode, ParseError> {
        Ok(Json5SingleStringTextNode::new(SingleStringTextNativeNode { node: base.into_inner() }))
    }

    fn parse_string(text: String, offset: u32) -> Result<Json5SingleStringTextNode, ParseError> {
        let text = Rc::from(text);
        let lang = NativeLanguage { name: "", glob: &[] };
        match parse_cst::parse_cst(&text, Json5Token::SingleStringText)?.next() {
            Some(s) => {
                let node = NativeSyntaxData::new(text.clone(), s, &lang);
                Ok(Json5SingleStringTextNode::new(SingleStringTextNativeNode { node }))
            }
            None => Err(ParseError::MissingRoot),
        }
    }

    fn get_rule(&self) -> SyntaxRule {
        self.node.get_rule()
    }

    fn get_string(&self) -> String {
        self.node.get_text()
    }

    fn get_range(&self) -> TextRange {
        self.node.get_range()
    }
}
#[automatically_derived]
impl Json5SingleStringTextNode {
    pub fn from_node(node: Node<NativeSyntaxData>) -> Result<Self, ParseError> {
        if cfg!(debug_assertions) {
            let expect = vec![String::from("")];
            let current = node.get_rule().into_inner::<NativeSyntaxRule>().name.to_string();
            if !expect.contains(&current) {
                return Err(ParseError::InvalidRule(InvalidRule { expect, current, range: node.get_range() }));
            }
        }
        Ok(Self::new(SingleStringTextNativeNode { node }))
    }
}

#[automatically_derived]
impl GuestJson5NumberNode for NumberNativeNode {
    fn ctor(base: SyntaxNode) -> Result<Json5NumberNode, ParseError> {
        Ok(Json5NumberNode::new(NumberNativeNode { node: base.into_inner() }))
    }

    fn parse_string(text: String, offset: u32) -> Result<Json5NumberNode, ParseError> {
        let text = Rc::from(text);
        let lang = NativeLanguage { name: "", glob: &[] };
        match parse_cst::parse_cst(&text, Json5Token::Number)?.next() {
            Some(s) => {
                let node = NativeSyntaxData::new(text.clone(), s, &lang);
                Ok(Json5NumberNode::new(NumberNativeNode { node }))
            }
            None => Err(ParseError::MissingRoot),
        }
    }

    fn get_rule(&self) -> SyntaxRule {
        self.node.get_rule()
    }

    fn get_string(&self) -> String {
        self.node.get_text()
    }

    fn get_range(&self) -> TextRange {
        self.node.get_range()
    }
}
#[automatically_derived]
impl Json5NumberNode {
    pub fn from_node(node: Node<NativeSyntaxData>) -> Result<Self, ParseError> {
        if cfg!(debug_assertions) {
            let expect = vec![String::from("")];
            let current = node.get_rule().into_inner::<NativeSyntaxRule>().name.to_string();
            if !expect.contains(&current) {
                return Err(ParseError::InvalidRule(InvalidRule { expect, current, range: node.get_range() }));
            }
        }
        Ok(Self::new(NumberNativeNode { node }))
    }
}

#[automatically_derived]
impl GuestJson5NullNode for NullNativeNode {
    fn ctor(base: SyntaxNode) -> Result<Json5NullNode, ParseError> {
        Ok(Json5NullNode::new(NullNativeNode { node: base.into_inner() }))
    }

    fn parse_string(text: String, offset: u32) -> Result<Json5NullNode, ParseError> {
        let text = Rc::from(text);
        let lang = NativeLanguage { name: "", glob: &[] };
        match parse_cst::parse_cst(&text, Json5Token::Null)?.next() {
            Some(s) => {
                let node = NativeSyntaxData::new(text.clone(), s, &lang);
                Ok(Json5NullNode::new(NullNativeNode { node }))
            }
            None => Err(ParseError::MissingRoot),
        }
    }

    fn get_rule(&self) -> SyntaxRule {
        self.node.get_rule()
    }

    fn get_string(&self) -> String {
        self.node.get_text()
    }

    fn get_range(&self) -> TextRange {
        self.node.get_range()
    }
}
#[automatically_derived]
impl Json5NullNode {
    pub fn from_node(node: Node<NativeSyntaxData>) -> Result<Self, ParseError> {
        if cfg!(debug_assertions) {
            let expect = vec![String::from("")];
            let current = node.get_rule().into_inner::<NativeSyntaxRule>().name.to_string();
            if !expect.contains(&current) {
                return Err(ParseError::InvalidRule(InvalidRule { expect, current, range: node.get_range() }));
            }
        }
        Ok(Self::new(NullNativeNode { node }))
    }
}

#[automatically_derived]
impl GuestJson5IdentifierNode for IdentifierNativeNode {
    fn ctor(base: SyntaxNode) -> Result<Json5IdentifierNode, ParseError> {
        Ok(Json5IdentifierNode::new(IdentifierNativeNode { node: base.into_inner() }))
    }

    fn parse_string(text: String, offset: u32) -> Result<Json5IdentifierNode, ParseError> {
        let text = Rc::from(text);
        let lang = NativeLanguage { name: "", glob: &[] };
        match parse_cst::parse_cst(&text, Json5Token::Identifier)?.next() {
            Some(s) => {
                let node = NativeSyntaxData::new(text.clone(), s, &lang);
                Ok(Json5IdentifierNode::new(IdentifierNativeNode { node }))
            }
            None => Err(ParseError::MissingRoot),
        }
    }

    fn get_rule(&self) -> SyntaxRule {
        self.node.get_rule()
    }

    fn get_string(&self) -> String {
        self.node.get_text()
    }

    fn get_range(&self) -> TextRange {
        self.node.get_range()
    }
}
#[automatically_derived]
impl Json5IdentifierNode {
    pub fn from_node(node: Node<NativeSyntaxData>) -> Result<Self, ParseError> {
        if cfg!(debug_assertions) {
            let expect = vec![String::from("")];
            let current = node.get_rule().into_inner::<NativeSyntaxRule>().name.to_string();
            if !expect.contains(&current) {
                return Err(ParseError::InvalidRule(InvalidRule { expect, current, range: node.get_range() }));
            }
        }
        Ok(Self::new(IdentifierNativeNode { node }))
    }
}

#[automatically_derived]
impl GuestJson5ColonNode for ColonNativeNode {
    fn ctor(base: SyntaxNode) -> Result<Json5ColonNode, ParseError> {
        Ok(Json5ColonNode::new(ColonNativeNode { node: base.into_inner() }))
    }

    fn parse_string(text: String, offset: u32) -> Result<Json5ColonNode, ParseError> {
        let text = Rc::from(text);
        let lang = NativeLanguage { name: "", glob: &[] };
        match parse_cst::parse_cst(&text, Json5Token::Colon)?.next() {
            Some(s) => {
                let node = NativeSyntaxData::new(text.clone(), s, &lang);
                Ok(Json5ColonNode::new(ColonNativeNode { node }))
            }
            None => Err(ParseError::MissingRoot),
        }
    }

    fn get_rule(&self) -> SyntaxRule {
        self.node.get_rule()
    }

    fn get_string(&self) -> String {
        self.node.get_text()
    }

    fn get_range(&self) -> TextRange {
        self.node.get_range()
    }
}
#[automatically_derived]
impl Json5ColonNode {
    pub fn from_node(node: Node<NativeSyntaxData>) -> Result<Self, ParseError> {
        if cfg!(debug_assertions) {
            let expect = vec![String::from("")];
            let current = node.get_rule().into_inner::<NativeSyntaxRule>().name.to_string();
            if !expect.contains(&current) {
                return Err(ParseError::InvalidRule(InvalidRule { expect, current, range: node.get_range() }));
            }
        }
        Ok(Self::new(ColonNativeNode { node }))
    }
}

#[automatically_derived]
impl GuestJson5CommaNode for CommaNativeNode {
    fn ctor(base: SyntaxNode) -> Result<Json5CommaNode, ParseError> {
        Ok(Json5CommaNode::new(CommaNativeNode { node: base.into_inner() }))
    }

    fn parse_string(text: String, offset: u32) -> Result<Json5CommaNode, ParseError> {
        let text = Rc::from(text);
        let lang = NativeLanguage { name: "", glob: &[] };
        match parse_cst::parse_cst(&text, Json5Token::Comma)?.next() {
            Some(s) => {
                let node = NativeSyntaxData::new(text.clone(), s, &lang);
                Ok(Json5CommaNode::new(CommaNativeNode { node }))
            }
            None => Err(ParseError::MissingRoot),
        }
    }

    fn get_rule(&self) -> SyntaxRule {
        self.node.get_rule()
    }

    fn get_string(&self) -> String {
        self.node.get_text()
    }

    fn get_range(&self) -> TextRange {
        self.node.get_range()
    }
}
#[automatically_derived]
impl Json5CommaNode {
    pub fn from_node(node: Node<NativeSyntaxData>) -> Result<Self, ParseError> {
        if cfg!(debug_assertions) {
            let expect = vec![String::from("")];
            let current = node.get_rule().into_inner::<NativeSyntaxRule>().name.to_string();
            if !expect.contains(&current) {
                return Err(ParseError::InvalidRule(InvalidRule { expect, current, range: node.get_range() }));
            }
        }
        Ok(Self::new(CommaNativeNode { node }))
    }
}

#[automatically_derived]
impl GuestJson5CommentNode for CommentNativeNode {
    fn ctor(base: SyntaxNode) -> Result<Json5CommentNode, ParseError> {
        Ok(Json5CommentNode::new(CommentNativeNode { node: base.into_inner() }))
    }

    fn parse_string(text: String, offset: u32) -> Result<Json5CommentNode, ParseError> {
        let text = Rc::from(text);
        let lang = NativeLanguage { name: "", glob: &[] };
        match parse_cst::parse_cst(&text, Json5Token::Comment)?.next() {
            Some(s) => {
                let node = NativeSyntaxData::new(text.clone(), s, &lang);
                Ok(Json5CommentNode::new(CommentNativeNode { node }))
            }
            None => Err(ParseError::MissingRoot),
        }
    }

    fn get_rule(&self) -> SyntaxRule {
        self.node.get_rule()
    }

    fn get_string(&self) -> String {
        self.node.get_text()
    }

    fn get_range(&self) -> TextRange {
        self.node.get_range()
    }
}
#[automatically_derived]
impl Json5CommentNode {
    pub fn from_node(node: Node<NativeSyntaxData>) -> Result<Self, ParseError> {
        if cfg!(debug_assertions) {
            let expect = vec![String::from("")];
            let current = node.get_rule().into_inner::<NativeSyntaxRule>().name.to_string();
            if !expect.contains(&current) {
                return Err(ParseError::InvalidRule(InvalidRule { expect, current, range: node.get_range() }));
            }
        }
        Ok(Self::new(CommentNativeNode { node }))
    }
}

#[automatically_derived]
impl GuestJson5WhiteSpaceNode for WhiteSpaceNativeNode {
    fn ctor(base: SyntaxNode) -> Result<Json5WhiteSpaceNode, ParseError> {
        Ok(Json5WhiteSpaceNode::new(WhiteSpaceNativeNode { node: base.into_inner() }))
    }

    fn parse_string(text: String, offset: u32) -> Result<Json5WhiteSpaceNode, ParseError> {
        let text = Rc::from(text);
        let lang = NativeLanguage { name: "", glob: &[] };
        match parse_cst::parse_cst(&text, Json5Token::WhiteSpace)?.next() {
            Some(s) => {
                let node = NativeSyntaxData::new(text.clone(), s, &lang);
                Ok(Json5WhiteSpaceNode::new(WhiteSpaceNativeNode { node }))
            }
            None => Err(ParseError::MissingRoot),
        }
    }

    fn get_rule(&self) -> SyntaxRule {
        self.node.get_rule()
    }

    fn get_string(&self) -> String {
        self.node.get_text()
    }

    fn get_range(&self) -> TextRange {
        self.node.get_range()
    }
}
#[automatically_derived]
impl Json5WhiteSpaceNode {
    pub fn from_node(node: Node<NativeSyntaxData>) -> Result<Self, ParseError> {
        if cfg!(debug_assertions) {
            let expect = vec![String::from("")];
            let current = node.get_rule().into_inner::<NativeSyntaxRule>().name.to_string();
            if !expect.contains(&current) {
                return Err(ParseError::InvalidRule(InvalidRule { expect, current, range: node.get_range() }));
            }
        }
        Ok(Self::new(WhiteSpaceNativeNode { node }))
    }
}

#[automatically_derived]
impl GuestJson5String0Node for String0NativeNode {
    fn ctor(base: SyntaxNode) -> Result<Json5String0Node, ParseError> {
        Ok(Json5String0Node::new(String0NativeNode { node: base.into_inner() }))
    }

    fn parse_string(text: String, offset: u32) -> Result<Json5String0Node, ParseError> {
        let text = Rc::from(text);
        let lang = NativeLanguage { name: "", glob: &[] };
        match parse_cst::parse_cst(&text, Json5Token::String0)?.next() {
            Some(s) => {
                let node = NativeSyntaxData::new(text.clone(), s, &lang);
                Ok(Json5String0Node::new(String0NativeNode { node }))
            }
            None => Err(ParseError::MissingRoot),
        }
    }

    fn get_rule(&self) -> SyntaxRule {
        self.node.get_rule()
    }

    fn get_string(&self) -> String {
        self.node.get_text()
    }

    fn get_range(&self) -> TextRange {
        self.node.get_range()
    }
}
#[automatically_derived]
impl Json5String0Node {
    pub fn from_node(node: Node<NativeSyntaxData>) -> Result<Self, ParseError> {
        if cfg!(debug_assertions) {
            let expect = vec![String::from("")];
            let current = node.get_rule().into_inner::<NativeSyntaxRule>().name.to_string();
            if !expect.contains(&current) {
                return Err(ParseError::InvalidRule(InvalidRule { expect, current, range: node.get_range() }));
            }
        }
        Ok(Self::new(String0NativeNode { node }))
    }
}

#[automatically_derived]
impl GuestJson5String1Node for String1NativeNode {
    fn ctor(base: SyntaxNode) -> Result<Json5String1Node, ParseError> {
        Ok(Json5String1Node::new(String1NativeNode { node: base.into_inner() }))
    }

    fn parse_string(text: String, offset: u32) -> Result<Json5String1Node, ParseError> {
        let text = Rc::from(text);
        let lang = NativeLanguage { name: "", glob: &[] };
        match parse_cst::parse_cst(&text, Json5Token::String1)?.next() {
            Some(s) => {
                let node = NativeSyntaxData::new(text.clone(), s, &lang);
                Ok(Json5String1Node::new(String1NativeNode { node }))
            }
            None => Err(ParseError::MissingRoot),
        }
    }

    fn get_rule(&self) -> SyntaxRule {
        self.node.get_rule()
    }

    fn get_string(&self) -> String {
        self.node.get_text()
    }

    fn get_range(&self) -> TextRange {
        self.node.get_range()
    }
}
#[automatically_derived]
impl Json5String1Node {
    pub fn from_node(node: Node<NativeSyntaxData>) -> Result<Self, ParseError> {
        if cfg!(debug_assertions) {
            let expect = vec![String::from("")];
            let current = node.get_rule().into_inner::<NativeSyntaxRule>().name.to_string();
            if !expect.contains(&current) {
                return Err(ParseError::InvalidRule(InvalidRule { expect, current, range: node.get_range() }));
            }
        }
        Ok(Self::new(String1NativeNode { node }))
    }
}

#[automatically_derived]
impl GuestJson5Boolean0Node for Boolean0NativeNode {
    fn ctor(base: SyntaxNode) -> Result<Json5Boolean0Node, ParseError> {
        Ok(Json5Boolean0Node::new(Boolean0NativeNode { node: base.into_inner() }))
    }

    fn parse_string(text: String, offset: u32) -> Result<Json5Boolean0Node, ParseError> {
        let text = Rc::from(text);
        let lang = NativeLanguage { name: "", glob: &[] };
        match parse_cst::parse_cst(&text, Json5Token::Boolean0)?.next() {
            Some(s) => {
                let node = NativeSyntaxData::new(text.clone(), s, &lang);
                Ok(Json5Boolean0Node::new(Boolean0NativeNode { node }))
            }
            None => Err(ParseError::MissingRoot),
        }
    }

    fn get_rule(&self) -> SyntaxRule {
        self.node.get_rule()
    }

    fn get_string(&self) -> String {
        self.node.get_text()
    }

    fn get_range(&self) -> TextRange {
        self.node.get_range()
    }
}
#[automatically_derived]
impl Json5Boolean0Node {
    pub fn from_node(node: Node<NativeSyntaxData>) -> Result<Self, ParseError> {
        if cfg!(debug_assertions) {
            let expect = vec![String::from("")];
            let current = node.get_rule().into_inner::<NativeSyntaxRule>().name.to_string();
            if !expect.contains(&current) {
                return Err(ParseError::InvalidRule(InvalidRule { expect, current, range: node.get_range() }));
            }
        }
        Ok(Self::new(Boolean0NativeNode { node }))
    }
}

#[automatically_derived]
impl GuestJson5Boolean1Node for Boolean1NativeNode {
    fn ctor(base: SyntaxNode) -> Result<Json5Boolean1Node, ParseError> {
        Ok(Json5Boolean1Node::new(Boolean1NativeNode { node: base.into_inner() }))
    }

    fn parse_string(text: String, offset: u32) -> Result<Json5Boolean1Node, ParseError> {
        let text = Rc::from(text);
        let lang = NativeLanguage { name: "", glob: &[] };
        match parse_cst::parse_cst(&text, Json5Token::Boolean1)?.next() {
            Some(s) => {
                let node = NativeSyntaxData::new(text.clone(), s, &lang);
                Ok(Json5Boolean1Node::new(Boolean1NativeNode { node }))
            }
            None => Err(ParseError::MissingRoot),
        }
    }

    fn get_rule(&self) -> SyntaxRule {
        self.node.get_rule()
    }

    fn get_string(&self) -> String {
        self.node.get_text()
    }

    fn get_range(&self) -> TextRange {
        self.node.get_range()
    }
}
#[automatically_derived]
impl Json5Boolean1Node {
    pub fn from_node(node: Node<NativeSyntaxData>) -> Result<Self, ParseError> {
        if cfg!(debug_assertions) {
            let expect = vec![String::from("")];
            let current = node.get_rule().into_inner::<NativeSyntaxRule>().name.to_string();
            if !expect.contains(&current) {
                return Err(ParseError::InvalidRule(InvalidRule { expect, current, range: node.get_range() }));
            }
        }
        Ok(Self::new(Boolean1NativeNode { node }))
    }
}

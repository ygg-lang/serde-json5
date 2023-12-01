#![allow(unused_variables)]
use super::*;
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for ValueNode<'i> {
    type Rule = Json5Rule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(Json5Parser::parse_cst(input, Json5Rule::Value)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one("object") {
            return Ok(Self::Object(s));
        }
        if let Ok(s) = pair.take_tagged_one("array") {
            return Ok(Self::Array(s));
        }
        if let Ok(s) = pair.take_tagged_one("string") {
            return Ok(Self::String(s));
        }
        if let Ok(s) = pair.take_tagged_one("number") {
            return Ok(Self::Number(s));
        }
        if let Ok(s) = pair.take_tagged_one("boolean") {
            return Ok(Self::Boolean(s));
        }
        if let Ok(s) = pair.take_tagged_one("null") {
            return Ok(Self::Null(s));
        }
        Err(YggdrasilError::invalid_node(Json5Rule::Value, _span))
    }

    fn get_rule(&self) -> Self::Rule {
        Json5Rule::Value
    }

    fn get_str(&self) -> &'i str {
        match self {
            Self::Object(s) => s.get_str(),
            Self::Array(s) => s.get_str(),
            Self::String(s) => s.get_str(),
            Self::Number(s) => s.get_str(),
            Self::Boolean(s) => s.get_str(),
            Self::Null(s) => s.get_str(),
        }
    }

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::Object(s) => s.get_range(),
            Self::Array(s) => s.get_range(),
            Self::String(s) => s.get_range(),
            Self::Number(s) => s.get_range(),
            Self::Boolean(s) => s.get_range(),
            Self::Null(s) => s.get_range(),
        }
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for ObjectNode<'i> {
    type Rule = Json5Rule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(Json5Parser::parse_cst(input, Json5Rule::Object)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        Json5Rule::Object
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> ObjectNode<'i> {
    pub fn comma(&self) -> Vec<CommaNode<'i>> {
        self.pair.take_tagged_items("comma").collect::<Result<Vec<_>, _>>().unwrap()
    }
    pub fn object_pair(&self) -> Vec<ObjectPairNode<'i>> {
        self.pair.take_tagged_items("object_pair").collect::<Result<Vec<_>, _>>().unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for ObjectPairNode<'i> {
    type Rule = Json5Rule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(Json5Parser::parse_cst(input, Json5Rule::ObjectPair)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        Json5Rule::ObjectPair
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> ObjectPairNode<'i> {
    pub fn colon(&self) -> ColonNode<'i> {
        self.pair.take_tagged_one("colon").unwrap()
    }
    pub fn object_key(&self) -> ObjectKeyNode<'i> {
        self.pair.take_tagged_one("object_key").unwrap()
    }
    pub fn value(&self) -> ValueNode<'i> {
        self.pair.take_tagged_one("value").unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for ObjectKeyNode<'i> {
    type Rule = Json5Rule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(Json5Parser::parse_cst(input, Json5Rule::ObjectKey)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one("identifier") {
            return Ok(Self::Identifier(s));
        }
        if let Ok(s) = pair.take_tagged_one("string") {
            return Ok(Self::String(s));
        }
        Err(YggdrasilError::invalid_node(Json5Rule::ObjectKey, _span))
    }

    fn get_rule(&self) -> Self::Rule {
        Json5Rule::ObjectKey
    }

    fn get_str(&self) -> &'i str {
        match self {
            Self::Identifier(s) => s.get_str(),
            Self::String(s) => s.get_str(),
        }
    }

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::Identifier(s) => s.get_range(),
            Self::String(s) => s.get_range(),
        }
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for ArrayNode<'i> {
    type Rule = Json5Rule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(Json5Parser::parse_cst(input, Json5Rule::Array)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        Json5Rule::Array
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> ArrayNode<'i> {
    pub fn comma(&self) -> Vec<CommaNode<'i>> {
        self.pair.take_tagged_items("comma").collect::<Result<Vec<_>, _>>().unwrap()
    }
    pub fn value(&self) -> Vec<ValueNode<'i>> {
        self.pair.take_tagged_items("value").collect::<Result<Vec<_>, _>>().unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for StringNode<'i> {
    type Rule = Json5Rule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(Json5Parser::parse_cst(input, Json5Rule::String)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one("string_0") {
            return Ok(Self::String0(s));
        }
        if let Ok(s) = pair.take_tagged_one("string_1") {
            return Ok(Self::String1(s));
        }
        Err(YggdrasilError::invalid_node(Json5Rule::String, _span))
    }

    fn get_rule(&self) -> Self::Rule {
        Json5Rule::String
    }

    fn get_str(&self) -> &'i str {
        match self {
            Self::String0(s) => s.get_str(),
            Self::String1(s) => s.get_str(),
        }
    }

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::String0(s) => s.get_range(),
            Self::String1(s) => s.get_range(),
        }
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for DoubleStringElementNode<'i> {
    type Rule = Json5Rule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(Json5Parser::parse_cst(input, Json5Rule::DoubleStringElement)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one("hex_escape") {
            return Ok(Self::HexEscape(s));
        }
        if let Ok(s) = pair.take_tagged_one("any_escape") {
            return Ok(Self::AnyEscape(s));
        }
        if let Ok(s) = pair.take_tagged_one("double_string_text") {
            return Ok(Self::DoubleStringText(s));
        }
        Err(YggdrasilError::invalid_node(Json5Rule::DoubleStringElement, _span))
    }

    fn get_rule(&self) -> Self::Rule {
        Json5Rule::DoubleStringElement
    }

    fn get_str(&self) -> &'i str {
        match self {
            Self::HexEscape(s) => s.get_str(),
            Self::AnyEscape(s) => s.get_str(),
            Self::DoubleStringText(s) => s.get_str(),
        }
    }

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::HexEscape(s) => s.get_range(),
            Self::AnyEscape(s) => s.get_range(),
            Self::DoubleStringText(s) => s.get_range(),
        }
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for SingleStringElementNode<'i> {
    type Rule = Json5Rule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(Json5Parser::parse_cst(input, Json5Rule::SingleStringElement)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one("hex_escape") {
            return Ok(Self::HexEscape(s));
        }
        if let Ok(s) = pair.take_tagged_one("any_escape") {
            return Ok(Self::AnyEscape(s));
        }
        if let Ok(s) = pair.take_tagged_one("single_string_text") {
            return Ok(Self::SingleStringText(s));
        }
        Err(YggdrasilError::invalid_node(Json5Rule::SingleStringElement, _span))
    }

    fn get_rule(&self) -> Self::Rule {
        Json5Rule::SingleStringElement
    }

    fn get_str(&self) -> &'i str {
        match self {
            Self::HexEscape(s) => s.get_str(),
            Self::AnyEscape(s) => s.get_str(),
            Self::SingleStringText(s) => s.get_str(),
        }
    }

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::HexEscape(s) => s.get_range(),
            Self::AnyEscape(s) => s.get_range(),
            Self::SingleStringText(s) => s.get_range(),
        }
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for HexEscapeNode<'i> {
    type Rule = Json5Rule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(Json5Parser::parse_cst(input, Json5Rule::HexEscape)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        Json5Rule::HexEscape
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> HexEscapeNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for AnyEscapeNode<'i> {
    type Rule = Json5Rule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(Json5Parser::parse_cst(input, Json5Rule::AnyEscape)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        Json5Rule::AnyEscape
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> AnyEscapeNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for DoubleStringTextNode<'i> {
    type Rule = Json5Rule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(Json5Parser::parse_cst(input, Json5Rule::DoubleStringText)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        Json5Rule::DoubleStringText
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> DoubleStringTextNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for SingleStringTextNode<'i> {
    type Rule = Json5Rule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(Json5Parser::parse_cst(input, Json5Rule::SingleStringText)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        Json5Rule::SingleStringText
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> SingleStringTextNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for NumberNode<'i> {
    type Rule = Json5Rule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(Json5Parser::parse_cst(input, Json5Rule::Number)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        Json5Rule::Number
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> NumberNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for BooleanNode<'i> {
    type Rule = Json5Rule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(Json5Parser::parse_cst(input, Json5Rule::Boolean)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one("true") {
            return Ok(Self::True(s));
        }
        if let Ok(s) = pair.take_tagged_one("false") {
            return Ok(Self::False(s));
        }
        Err(YggdrasilError::invalid_node(Json5Rule::Boolean, _span))
    }

    fn get_rule(&self) -> Self::Rule {
        Json5Rule::Boolean
    }

    fn get_str(&self) -> &'i str {
        match self {
            Self::True(s) => s.get_str(),
            Self::False(s) => s.get_str(),
        }
    }

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::True(s) => s.get_range(),
            Self::False(s) => s.get_range(),
        }
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for NullNode<'i> {
    type Rule = Json5Rule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(Json5Parser::parse_cst(input, Json5Rule::Null)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        Json5Rule::Null
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> NullNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for IdentifierNode<'i> {
    type Rule = Json5Rule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(Json5Parser::parse_cst(input, Json5Rule::Identifier)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        Json5Rule::Identifier
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> IdentifierNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for ColonNode<'i> {
    type Rule = Json5Rule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(Json5Parser::parse_cst(input, Json5Rule::COLON)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        Json5Rule::COLON
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> ColonNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for CommaNode<'i> {
    type Rule = Json5Rule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(Json5Parser::parse_cst(input, Json5Rule::COMMA)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        Json5Rule::COMMA
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> CommaNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for CommentNode<'i> {
    type Rule = Json5Rule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(Json5Parser::parse_cst(input, Json5Rule::Comment)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        Json5Rule::Comment
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> CommentNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for WhiteSpaceNode<'i> {
    type Rule = Json5Rule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(Json5Parser::parse_cst(input, Json5Rule::WhiteSpace)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        Json5Rule::WhiteSpace
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> WhiteSpaceNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for String0Node<'i> {
    type Rule = Json5Rule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(Json5Parser::parse_cst(input, Json5Rule::String0)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        Json5Rule::String0
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> String0Node<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for String1Node<'i> {
    type Rule = Json5Rule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(Json5Parser::parse_cst(input, Json5Rule::String1)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        Json5Rule::String1
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> String1Node<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for Boolean0Node<'i> {
    type Rule = Json5Rule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(Json5Parser::parse_cst(input, Json5Rule::Boolean0)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        Json5Rule::Boolean0
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> Boolean0Node<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for Boolean1Node<'i> {
    type Rule = Json5Rule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(Json5Parser::parse_cst(input, Json5Rule::Boolean1)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        Json5Rule::Boolean1
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> Boolean1Node<'i> {}

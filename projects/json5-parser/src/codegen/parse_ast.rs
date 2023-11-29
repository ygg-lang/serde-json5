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
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        Json5Rule::String
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> StringNode<'i> {
    pub fn string_element(&self) -> Vec<StringElementNode<'i>> {
        self.pair.take_tagged_items("string_element").collect::<Result<Vec<_>, _>>().unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for StringElementNode<'i> {
    type Rule = Json5Rule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(Json5Parser::parse_cst(input, Json5Rule::StringElement)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one("hex_digit") {
            return Ok(Self::HexDigit(s));
        }
        if let Ok(s) = pair.take_tagged_one("escaped") {
            return Ok(Self::Escaped(s));
        }
        if let Ok(s) = pair.take_tagged_one("string_text") {
            return Ok(Self::StringText(s));
        }
        Err(YggdrasilError::invalid_node(Json5Rule::StringElement, _span))
    }

    fn get_rule(&self) -> Self::Rule {
        Json5Rule::StringElement
    }

    fn get_str(&self) -> &'i str {
        match self {
            Self::HexDigit(s) => s.get_str(),
            Self::Escaped(s) => s.get_str(),
            Self::StringText(s) => s.get_str(),
        }
    }

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::HexDigit(s) => s.get_range(),
            Self::Escaped(s) => s.get_range(),
            Self::StringText(s) => s.get_range(),
        }
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for HexDigitNode<'i> {
    type Rule = Json5Rule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(Json5Parser::parse_cst(input, Json5Rule::HexDigit)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        Json5Rule::HexDigit
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> HexDigitNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for EscapedNode<'i> {
    type Rule = Json5Rule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(Json5Parser::parse_cst(input, Json5Rule::Escaped)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        Json5Rule::Escaped
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> EscapedNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for StringTextNode<'i> {
    type Rule = Json5Rule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(Json5Parser::parse_cst(input, Json5Rule::StringText)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        Json5Rule::StringText
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> StringTextNode<'i> {}
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
impl<'i> YggdrasilNode<'i> for StringElement0Node<'i> {
    type Rule = Json5Rule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(Json5Parser::parse_cst(input, Json5Rule::StringElement0)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        Json5Rule::StringElement0
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> StringElement0Node<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for StringElement1Node<'i> {
    type Rule = Json5Rule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(Json5Parser::parse_cst(input, Json5Rule::StringElement1)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        Json5Rule::StringElement1
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> StringElement1Node<'i> {}
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

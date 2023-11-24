use super::*;
#[automatically_derived]
impl YggdrasilNode for ValueNode {
    type Rule = Json5Rule;

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::Array(s) => s.get_range(),
            Self::Boolean(s) => s.get_range(),
            Self::Null(s) => s.get_range(),
            Self::Number(s) => s.get_range(),
            Self::Object(s) => s.get_range(),
            Self::String(s) => s.get_range(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one::<ArrayNode>(Cow::Borrowed("array")) {
            return Ok(Self::Array(s));
        }
        if let Ok(s) = pair.take_tagged_one::<BooleanNode>(Cow::Borrowed("boolean")) {
            return Ok(Self::Boolean(s));
        }
        if let Ok(s) = pair.take_tagged_one::<NullNode>(Cow::Borrowed("null")) {
            return Ok(Self::Null(s));
        }
        if let Ok(s) = pair.take_tagged_one::<NumberNode>(Cow::Borrowed("number")) {
            return Ok(Self::Number(s));
        }
        if let Ok(s) = pair.take_tagged_one::<ObjectNode>(Cow::Borrowed("object")) {
            return Ok(Self::Object(s));
        }
        if let Ok(s) = pair.take_tagged_one::<StringNode>(Cow::Borrowed("string")) {
            return Ok(Self::String(s));
        }
        Err(YggdrasilError::invalid_node(Json5Rule::Value, _span))
    }
}
#[automatically_derived]
impl FromStr for ValueNode {
    type Err = YggdrasilError<Json5Rule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<Json5Rule>> {
        Self::from_cst(Json5Parser::parse_cst(input, Json5Rule::Value)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for ObjectNode {
    type Rule = Json5Rule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            object_pair: pair
                .take_tagged_items::<ObjectPairNode>(Cow::Borrowed("object_pair"))
                .collect::<Result<Vec<_>, _>>()?,
            span: Range { start: _span.start() as usize, end: _span.end() as usize },
        })
    }
}
#[automatically_derived]
impl FromStr for ObjectNode {
    type Err = YggdrasilError<Json5Rule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<Json5Rule>> {
        Self::from_cst(Json5Parser::parse_cst(input, Json5Rule::Object)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for ObjectPairNode {
    type Rule = Json5Rule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            colon: pair.take_tagged_one::<ColonNode>(Cow::Borrowed("colon"))?,
            object_key: pair.take_tagged_one::<ObjectKeyNode>(Cow::Borrowed("object_key"))?,
            value: pair.take_tagged_one::<ValueNode>(Cow::Borrowed("value"))?,
            span: Range { start: _span.start() as usize, end: _span.end() as usize },
        })
    }
}
#[automatically_derived]
impl FromStr for ObjectPairNode {
    type Err = YggdrasilError<Json5Rule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<Json5Rule>> {
        Self::from_cst(Json5Parser::parse_cst(input, Json5Rule::ObjectPair)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for ColonNode {
    type Rule = Json5Rule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as usize, end: _span.end() as usize } })
    }
}
#[automatically_derived]
impl FromStr for ColonNode {
    type Err = YggdrasilError<Json5Rule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<Json5Rule>> {
        Self::from_cst(Json5Parser::parse_cst(input, Json5Rule::COLON)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for ObjectKeyNode {
    type Rule = Json5Rule;

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::Identifier(s) => s.get_range(),
            Self::String(s) => s.get_range(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("identifier")) {
            return Ok(Self::Identifier(s));
        }
        if let Ok(s) = pair.take_tagged_one::<StringNode>(Cow::Borrowed("string")) {
            return Ok(Self::String(s));
        }
        Err(YggdrasilError::invalid_node(Json5Rule::ObjectKey, _span))
    }
}
#[automatically_derived]
impl FromStr for ObjectKeyNode {
    type Err = YggdrasilError<Json5Rule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<Json5Rule>> {
        Self::from_cst(Json5Parser::parse_cst(input, Json5Rule::ObjectKey)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for ArrayNode {
    type Rule = Json5Rule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            value: pair.take_tagged_items::<ValueNode>(Cow::Borrowed("value")).collect::<Result<Vec<_>, _>>()?,
            span: Range { start: _span.start() as usize, end: _span.end() as usize },
        })
    }
}
#[automatically_derived]
impl FromStr for ArrayNode {
    type Err = YggdrasilError<Json5Rule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<Json5Rule>> {
        Self::from_cst(Json5Parser::parse_cst(input, Json5Rule::Array)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for StringNode {
    type Rule = Json5Rule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            string_element: pair
                .take_tagged_items::<StringElementNode>(Cow::Borrowed("string_element"))
                .collect::<Result<Vec<_>, _>>()?,
            span: Range { start: _span.start() as usize, end: _span.end() as usize },
        })
    }
}
#[automatically_derived]
impl FromStr for StringNode {
    type Err = YggdrasilError<Json5Rule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<Json5Rule>> {
        Self::from_cst(Json5Parser::parse_cst(input, Json5Rule::String)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for StringElementNode {
    type Rule = Json5Rule;

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::Escaped(s) => s.get_range(),
            Self::HexDigit(s) => s.get_range(),
            Self::StringText(s) => s.get_range(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one::<EscapedNode>(Cow::Borrowed("escaped")) {
            return Ok(Self::Escaped(s));
        }
        if let Ok(s) = pair.take_tagged_one::<HexDigitNode>(Cow::Borrowed("hex_digit")) {
            return Ok(Self::HexDigit(s));
        }
        if let Ok(s) = pair.take_tagged_one::<StringTextNode>(Cow::Borrowed("string_text")) {
            return Ok(Self::StringText(s));
        }
        Err(YggdrasilError::invalid_node(Json5Rule::StringElement, _span))
    }
}
#[automatically_derived]
impl FromStr for StringElementNode {
    type Err = YggdrasilError<Json5Rule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<Json5Rule>> {
        Self::from_cst(Json5Parser::parse_cst(input, Json5Rule::StringElement)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for HexDigitNode {
    type Rule = Json5Rule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as usize, end: _span.end() as usize } })
    }
}
#[automatically_derived]
impl FromStr for HexDigitNode {
    type Err = YggdrasilError<Json5Rule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<Json5Rule>> {
        Self::from_cst(Json5Parser::parse_cst(input, Json5Rule::HexDigit)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for EscapedNode {
    type Rule = Json5Rule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as usize, end: _span.end() as usize } })
    }
}
#[automatically_derived]
impl FromStr for EscapedNode {
    type Err = YggdrasilError<Json5Rule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<Json5Rule>> {
        Self::from_cst(Json5Parser::parse_cst(input, Json5Rule::Escaped)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for StringTextNode {
    type Rule = Json5Rule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as usize, end: _span.end() as usize } })
    }
}
#[automatically_derived]
impl FromStr for StringTextNode {
    type Err = YggdrasilError<Json5Rule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<Json5Rule>> {
        Self::from_cst(Json5Parser::parse_cst(input, Json5Rule::StringText)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for NumberNode {
    type Rule = Json5Rule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as usize, end: _span.end() as usize } })
    }
}
#[automatically_derived]
impl FromStr for NumberNode {
    type Err = YggdrasilError<Json5Rule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<Json5Rule>> {
        Self::from_cst(Json5Parser::parse_cst(input, Json5Rule::Number)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for BooleanNode {
    type Rule = Json5Rule;

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::False => Range::default(),
            Self::True => Range::default(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Some(_) = pair.find_first_tag("false") {
            return Ok(Self::False);
        }
        if let Some(_) = pair.find_first_tag("true") {
            return Ok(Self::True);
        }
        Err(YggdrasilError::invalid_node(Json5Rule::Boolean, _span))
    }
}
#[automatically_derived]
impl FromStr for BooleanNode {
    type Err = YggdrasilError<Json5Rule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<Json5Rule>> {
        Self::from_cst(Json5Parser::parse_cst(input, Json5Rule::Boolean)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for NullNode {
    type Rule = Json5Rule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as usize, end: _span.end() as usize } })
    }
}
#[automatically_derived]
impl FromStr for NullNode {
    type Err = YggdrasilError<Json5Rule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<Json5Rule>> {
        Self::from_cst(Json5Parser::parse_cst(input, Json5Rule::Null)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for IdentifierNode {
    type Rule = Json5Rule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as usize, end: _span.end() as usize } })
    }
}
#[automatically_derived]
impl FromStr for IdentifierNode {
    type Err = YggdrasilError<Json5Rule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<Json5Rule>> {
        Self::from_cst(Json5Parser::parse_cst(input, Json5Rule::Identifier)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for WhiteSpaceNode {
    type Rule = Json5Rule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as usize, end: _span.end() as usize } })
    }
}
#[automatically_derived]
impl FromStr for WhiteSpaceNode {
    type Err = YggdrasilError<Json5Rule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<Json5Rule>> {
        Self::from_cst(Json5Parser::parse_cst(input, Json5Rule::WhiteSpace)?)
    }
}

#![allow(dead_code, unused_imports, non_camel_case_types)]
#![allow(missing_docs, rustdoc::missing_crate_level_docs)]
#![allow(clippy::unnecessary_cast)]
#![doc = include_str!("readme.md")]

mod parse_ast;
mod parse_cst;

use core::str::FromStr;
use std::{borrow::Cow, ops::Range, sync::OnceLock};
use yggdrasil_rt::*;

type Input<'i> = Box<State<'i, Json5Rule>>;
type Output<'i> = Result<Box<State<'i, Json5Rule>>, Box<State<'i, Json5Rule>>>;

#[doc = include_str!("railway.min.svg")]
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Json5Parser {}

impl YggdrasilParser for Json5Parser {
    type Rule = Json5Rule;
    fn parse_cst(input: &str, rule: Self::Rule) -> OutputResult<Json5Rule> {
        self::parse_cst::parse_cst(input, rule)
    }
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Json5Rule {
    Value,
    Object,
    ObjectPair,
    ObjectKey,
    Array,
    String,
    StringElement,
    HexDigit,
    Escaped,
    StringText,
    Number,
    Boolean,
    Null,
    Identifier,
    COLON,
    COMMA,
    Comment,
    WhiteSpace,
    StringElement0,
    StringElement1,
    Boolean0,
    Boolean1,
    /// Label for unnamed text literal
    HiddenText,
}

impl YggdrasilRule for Json5Rule {
    fn is_ignore(&self) -> bool {
        matches!(
            self,
            Self::HiddenText
                | Self::Comment
                | Self::WhiteSpace
                | Self::StringElement0
                | Self::StringElement1
                | Self::Boolean0
                | Self::Boolean1
        )
    }

    fn get_style(&self) -> &'static str {
        match self {
            Self::Value => "",
            Self::Object => "",
            Self::ObjectPair => "",
            Self::ObjectKey => "",
            Self::Array => "",
            Self::String => "",
            Self::StringElement => "",
            Self::HexDigit => "",
            Self::Escaped => "",
            Self::StringText => "",
            Self::Number => "",
            Self::Boolean => "",
            Self::Null => "",
            Self::Identifier => "",
            Self::COLON => "",
            Self::COMMA => "",
            Self::Comment => "",
            Self::WhiteSpace => "",
            Self::StringElement0 => "",
            Self::StringElement1 => "",
            Self::Boolean0 => "",
            Self::Boolean1 => "",
            _ => "",
        }
    }
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ValueNode<'i> {
    Object(Box<ValueNode<'i>>),
    Array(Box<ValueNode<'i>>),
    String(Box<ValueNode<'i>>),
    Number(Box<ValueNode<'i>>),
    Boolean(Box<ValueNode<'i>>),
    Null(Box<ValueNode<'i>>),
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ObjectNode<'i> {
    pair: TokenPair<'i, Json5Rule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ObjectPairNode<'i> {
    pair: TokenPair<'i, Json5Rule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ObjectKeyNode<'i> {
    Identifier(IdentifierNode<'i>),
    String(StringNode<'i>),
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ArrayNode<'i> {
    pair: TokenPair<'i, Json5Rule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StringNode<'i> {
    pair: TokenPair<'i, Json5Rule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StringElementNode<'i> {
    HexDigit(StringElement0Node<'i>),
    Escaped(StringElement1Node<'i>),
    StringText(StringTextNode<'i>),
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HexDigitNode<'i> {
    pair: TokenPair<'i, Json5Rule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EscapedNode<'i> {
    pair: TokenPair<'i, Json5Rule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StringTextNode<'i> {
    pair: TokenPair<'i, Json5Rule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NumberNode<'i> {
    pair: TokenPair<'i, Json5Rule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BooleanNode<'i> {
    True(Boolean0Node<'i>),
    False(Boolean1Node<'i>),
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NullNode<'i> {
    pair: TokenPair<'i, Json5Rule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IdentifierNode<'i> {
    pair: TokenPair<'i, Json5Rule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ColonNode<'i> {
    pair: TokenPair<'i, Json5Rule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CommaNode<'i> {
    pair: TokenPair<'i, Json5Rule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CommentNode<'i> {
    pair: TokenPair<'i, Json5Rule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WhiteSpaceNode<'i> {
    pair: TokenPair<'i, Json5Rule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StringElement0Node<'i> {
    pair: TokenPair<'i, Json5Rule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StringElement1Node<'i> {
    pair: TokenPair<'i, Json5Rule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Boolean0Node<'i> {
    pair: TokenPair<'i, Json5Rule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Boolean1Node<'i> {
    pair: TokenPair<'i, Json5Rule>,
}

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
    DoubleStringElement,
    SingleStringElement,
    HexEscape,
    AnyEscape,
    DoubleStringText,
    SingleStringText,
    Number,
    Boolean,
    Null,
    Identifier,
    COLON,
    COMMA,
    Comment,
    WhiteSpace,
    String0,
    String1,
    Boolean0,
    Boolean1,
    /// Label for unnamed text literal
    HiddenText,
}

impl YggdrasilRule for Json5Rule {
    fn is_ignore(&self) -> bool {
        matches!(self, Self::HiddenText | Self::Comment | Self::WhiteSpace)
    }

    fn get_style(&self) -> &'static str {
        match self {
            Self::Value => "",
            Self::Object => "",
            Self::ObjectPair => "",
            Self::ObjectKey => "",
            Self::Array => "",
            Self::String => "",
            Self::DoubleStringElement => "",
            Self::SingleStringElement => "",
            Self::HexEscape => "",
            Self::AnyEscape => "",
            Self::DoubleStringText => "",
            Self::SingleStringText => "",
            Self::Number => "",
            Self::Boolean => "",
            Self::Null => "",
            Self::Identifier => "",
            Self::COLON => "",
            Self::COMMA => "",
            Self::Comment => "",
            Self::WhiteSpace => "",
            Self::String0 => "",
            Self::String1 => "",
            Self::Boolean0 => "",
            Self::Boolean1 => "",
            _ => "",
        }
    }
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ObjectNodeNative {
    node: crate::peg::core::cst::SyntaxNode,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ObjectPairNodeNative {
    node: crate::peg::core::cst::SyntaxNode,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ArrayNodeNative {
    node: crate::peg::core::cst::SyntaxNode,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HexEscapeNodeNative {
    node: crate::peg::core::cst::SyntaxNode,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AnyEscapeNodeNative {
    node: crate::peg::core::cst::SyntaxNode,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DoubleStringTextNodeNative {
    node: crate::peg::core::cst::SyntaxNode,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SingleStringTextNodeNative {
    node: crate::peg::core::cst::SyntaxNode,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NumberNodeNative {
    node: crate::peg::core::cst::SyntaxNode,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NullNodeNative {
    node: crate::peg::core::cst::SyntaxNode,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IdentifierNodeNative {
    node: crate::peg::core::cst::SyntaxNode,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ColonNodeNative {
    node: crate::peg::core::cst::SyntaxNode,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CommaNodeNative {
    node: crate::peg::core::cst::SyntaxNode,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CommentNodeNative {
    node: crate::peg::core::cst::SyntaxNode,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WhiteSpaceNodeNative {
    node: crate::peg::core::cst::SyntaxNode,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct String0NodeNative {
    node: crate::peg::core::cst::SyntaxNode,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct String1NodeNative {
    node: crate::peg::core::cst::SyntaxNode,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Boolean0NodeNative {
    node: crate::peg::core::cst::SyntaxNode,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Boolean1NodeNative {
    node: crate::peg::core::cst::SyntaxNode,
}

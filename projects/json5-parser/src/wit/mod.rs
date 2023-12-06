#![allow(dead_code, unused_imports, non_camel_case_types)]
#![allow(missing_docs, rustdoc::missing_crate_level_docs)]
#![allow(clippy::unnecessary_cast)]
#![doc = include_str!("readme.md")]

mod parse_ast;
mod parse_cst;

use core::str::FromStr;
use std::{borrow::Cow, ops::Range, sync::OnceLock};
use wasi_yggdrasil::{exports::peg::core::cst::SyntaxNode, state, OutputResult, Regex, State, YggdrasilParser, YggdrasilRule};

type Input<'i> = Box<State<'i, Json5>>;
type Output<'i> = Result<Box<State<'i, Json5>>, Box<State<'i, Json5>>>;

#[doc = include_str!("railway.min.svg")]
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Json5Host {}

impl YggdrasilRule for Json5 {
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
pub struct ObjectNative {
    node: SyntaxNode,
}
#[derive(Clone, Debug, Hash)]
pub struct ObjectPairNative {
    node: SyntaxNode,
}
#[derive(Clone, Debug, Hash)]
pub struct ArrayNative {
    node: SyntaxNode,
}
#[derive(Clone, Debug, Hash)]
pub struct HexEscapeNative {
    node: SyntaxNode,
}
#[derive(Clone, Debug, Hash)]
pub struct AnyEscapeNative {
    node: SyntaxNode,
}
#[derive(Clone, Debug, Hash)]
pub struct DoubleStringTextNative {
    node: SyntaxNode,
}
#[derive(Clone, Debug, Hash)]
pub struct SingleStringTextNative {
    node: SyntaxNode,
}
#[derive(Clone, Debug, Hash)]
pub struct NumberNative {
    node: SyntaxNode,
}
#[derive(Clone, Debug, Hash)]
pub struct NullNative {
    node: SyntaxNode,
}
#[derive(Clone, Debug, Hash)]
pub struct IdentifierNative {
    node: SyntaxNode,
}
#[derive(Clone, Debug, Hash)]
pub struct ColonNative {
    node: SyntaxNode,
}
#[derive(Clone, Debug, Hash)]
pub struct CommaNative {
    node: SyntaxNode,
}
#[derive(Clone, Debug, Hash)]
pub struct CommentNative {
    node: SyntaxNode,
}
#[derive(Clone, Debug, Hash)]
pub struct WhiteSpaceNative {
    node: SyntaxNode,
}
#[derive(Clone, Debug, Hash)]
pub struct String0Native {
    node: SyntaxNode,
}
#[derive(Clone, Debug, Hash)]
pub struct String1Native {
    node: SyntaxNode,
}
#[derive(Clone, Debug, Hash)]
pub struct Boolean0Native {
    node: SyntaxNode,
}
#[derive(Clone, Debug, Hash)]
pub struct Boolean1Native {
    node: SyntaxNode,
}

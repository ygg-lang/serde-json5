#![allow(dead_code, unused_imports, non_camel_case_types)]
#![allow(missing_docs, rustdoc::missing_crate_level_docs)]
#![allow(clippy::unnecessary_cast)]
#![doc = include_str!("readme.md")]

mod parse_ast;
mod parse_cst;

use crate::exports::yggdrasil::json5::ast::Json5Token;
use core::str::FromStr;
use std::{
    borrow::Cow,
    cmp::Ordering,
    hash::{Hash, Hasher},
    ops::Range,
    rc::Rc,
    sync::OnceLock,
};
use wasi_yggdrasil::{
    exports::peg::core::cst::{GuestSyntaxNode, SyntaxNode},
    state,
    syntax_node::{NativeLanguage, NativeSyntaxData},
    Node, OutputResult, Regex, State, YggdrasilParser, YggdrasilRule,
};

type Input<'i> = Box<State<'i, Json5Token>>;
type Output<'i> = Result<Box<State<'i, Json5Token>>, Box<State<'i, Json5Token>>>;

#[doc = include_str!("railway.min.svg")]
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Json5Host {}

impl Hash for Json5Token {
    fn hash<H: Hasher>(&self, state: &mut H) {
        todo!()
    }
}

impl Ord for Json5Token {
    fn cmp(&self, other: &Self) -> Ordering {
        todo!()
    }
}

impl PartialOrd for Json5Token {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        todo!()
    }
}

impl YggdrasilRule for Json5Token {
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
            Self::Colon => "",
            Self::Comma => "",
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
pub struct ObjectNative {
    node: Node<NativeSyntaxData>,
}
pub struct ObjectPairNative {
    node: Node<NativeSyntaxData>,
}
pub struct ArrayNative {
    node: Node<NativeSyntaxData>,
}
pub struct HexEscapeNative {
    node: Node<NativeSyntaxData>,
}
pub struct AnyEscapeNative {
    node: Node<NativeSyntaxData>,
}
pub struct DoubleStringTextNative {
    node: Node<NativeSyntaxData>,
}
pub struct SingleStringTextNative {
    node: Node<NativeSyntaxData>,
}
pub struct NumberNative {
    node: Node<NativeSyntaxData>,
}
pub struct NullNative {
    node: Node<NativeSyntaxData>,
}
pub struct IdentifierNative {
    node: Node<NativeSyntaxData>,
}
pub struct ColonNative {
    node: Node<NativeSyntaxData>,
}
pub struct CommaNative {
    node: Node<NativeSyntaxData>,
}
pub struct CommentNative {
    node: Node<NativeSyntaxData>,
}
pub struct WhiteSpaceNative {
    node: Node<NativeSyntaxData>,
}
pub struct String0Native {
    node: Node<NativeSyntaxData>,
}
pub struct String1Native {
    node: Node<NativeSyntaxData>,
}
pub struct Boolean0Native {
    node: Node<NativeSyntaxData>,
}
pub struct Boolean1Native {
    node: Node<NativeSyntaxData>,
}

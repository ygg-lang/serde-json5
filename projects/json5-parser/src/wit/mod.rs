#![allow(dead_code, unused_imports, non_camel_case_types)]
#![allow(missing_docs, rustdoc::missing_crate_level_docs)]
#![allow(clippy::unnecessary_cast)]
#![doc = include_str!("readme.md")]

mod parse_ast;
mod parse_cst;

use crate::exports::yggdrasil::json5::ast::Json5Token;
use core::{
    cmp::Ordering,
    hash::{Hash, Hasher},
    mem::transmute,
    ops::Range,
    str::FromStr,
};
use std::{borrow::Cow, rc::Rc, sync::OnceLock};
use wasi_yggdrasil::{
    exports::peg::core::cst::{GuestSyntaxNode, SyntaxNode},
    state,
    syntax_node::{NativeLanguage, NativeSyntaxData, NativeSyntaxRule},
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
pub struct ObjectNativeNode {
    node: Node<NativeSyntaxData>,
}
pub struct ObjectPairNativeNode {
    node: Node<NativeSyntaxData>,
}
pub struct ArrayNativeNode {
    node: Node<NativeSyntaxData>,
}
pub struct HexEscapeNativeNode {
    node: Node<NativeSyntaxData>,
}
pub struct AnyEscapeNativeNode {
    node: Node<NativeSyntaxData>,
}
pub struct DoubleStringTextNativeNode {
    node: Node<NativeSyntaxData>,
}
pub struct SingleStringTextNativeNode {
    node: Node<NativeSyntaxData>,
}
pub struct NumberNativeNode {
    node: Node<NativeSyntaxData>,
}
pub struct NullNativeNode {
    node: Node<NativeSyntaxData>,
}
pub struct IdentifierNativeNode {
    node: Node<NativeSyntaxData>,
}
pub struct ColonNativeNode {
    node: Node<NativeSyntaxData>,
}
pub struct CommaNativeNode {
    node: Node<NativeSyntaxData>,
}
pub struct CommentNativeNode {
    node: Node<NativeSyntaxData>,
}
pub struct WhiteSpaceNativeNode {
    node: Node<NativeSyntaxData>,
}
pub struct String0NativeNode {
    node: Node<NativeSyntaxData>,
}
pub struct String1NativeNode {
    node: Node<NativeSyntaxData>,
}
pub struct Boolean0NativeNode {
    node: Node<NativeSyntaxData>,
}
pub struct Boolean1NativeNode {
    node: Node<NativeSyntaxData>,
}

use crate::codegen::{Json5Parser, Json5Rule, ValueNode};
use indextree::{Arena, NodeId};
use nyar_error::NyarError;
use rctree::Node;
use std::{
    borrow::Cow,
    fmt::{Debug, Formatter},
    ops::Range,
    rc::Rc,
};
use yggdrasil_rt::{TokenPair, YggdrasilNode, YggdrasilParser, YggdrasilRule};

#[test]
fn test() {
    // Create a new arena
    let input = Rc::from("{a:1,}");

    let root = Json5Parser::parse_cst(&input, Json5Rule::Value).unwrap();
    let root = root.into_iter().next().unwrap();

    let parent = SyntaxData::new(input.clone(), &root);

    for child in root.into_inner() {
        parent.append(SyntaxData::new(input.clone(), &child))
    }

    println!("{:?}", parent);
}

pub struct SyntaxData {
    text: Rc<str>,
    span: Range<usize>,
}

impl Debug for SyntaxData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SyntaxNode").finish()
    }
}

impl SyntaxData {
    pub fn new<R: YggdrasilRule>(input: Rc<str>, pair: &TokenPair<R>) -> Node<SyntaxData> {
        Node::new(SyntaxData { text: input, span: pair.get_span().get_range() })
    }
}

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

    let parent = SyntaxData::new(input.clone(), root);

    for node in parent.descendants() {
        println!("{:?}", node);
    }
}

pub struct JsonRoot {
    syntax: SyntaxData,
}

pub struct SyntaxData {
    language: &'static str,
    rule: String,
    text: Rc<str>,
    span: Range<usize>,
}

impl Debug for SyntaxData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SyntaxNode")
            .field("language", &self.language)
            .field("rule", &self.rule)
            .field("text", &&self.text[self.span.clone()])
            .finish()
    }
}

impl SyntaxData {
    pub fn new<R: YggdrasilRule>(input: Rc<str>, pair: TokenPair<R>) -> Node<SyntaxData> {
        let rule = pair.get_rule();
        let parent = Node::new(SyntaxData {
            language: "json5",
            rule: format!("{:?}", rule),
            text: input.clone(),
            span: pair.get_span().get_range(),
        });
        for child in pair.into_inner() {
            parent.append(SyntaxData::new(input.clone(), child))
        }
        parent
    }
}

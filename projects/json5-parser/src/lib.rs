// #![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

mod wit;

use crate::exports::yggdrasil::json5::ast::{Json5ArrayNode, Json5ObjectNode, Json5ValueNode};
use wasi_yggdrasil::{exports::peg::core::types::TextRange, syntax_node::NativeSyntaxData, Node};
// use wasi_yggdrasil::exports::peg::core::errors::Result;
use crate::wit::{ArrayNative, Json5Host};

wit_bindgen::generate!({

    world: "host",


    with: {
        "peg:core/types": wasi_yggdrasil::exports::peg::core::types,
        "peg:core/cst": wasi_yggdrasil::exports::peg::core::cst,
    }
});

export!(Json5Host);

// #[automatically_derived]
// impl Json5ValueNode {
//     pub fn from_node(node: Node<NativeSyntaxData>) -> Option<Self> {
//         let node = node.first_child()?;
//         let tag: &str = node.borrow().tag.as_ref()?;
//         Some(match tag {
//             "array" => Json5ValueNode::Array(Json5ArrayNode::from_node(node)?),
//             _ => None?,
//         })
//     }
// }

// #![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

mod wit;
// use wasi_yggdrasil::exports::peg::core::errors::Result;
use crate::wit::Json5Host;

wit_bindgen::generate!({
    world: "host",
    with: {

        "peg:core/types": wasi_yggdrasil::exports::peg::core::types,

        "peg:core/cst": wasi_yggdrasil::exports::peg::core::cst,
    }
});

export!(Json5Host);

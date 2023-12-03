// #![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

mod codegen;
mod errors;

mod value;
mod wit;

use crate::wit::Json5Host;

pub use crate::value::parse;

wit_bindgen::generate!({

    world: "host",
});

export!(Json5Host);

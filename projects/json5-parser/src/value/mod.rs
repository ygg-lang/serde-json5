use std::str::FromStr;
use nyar_error::{FileCache, FileID, NyarError};
use crate::codegen::ValueNode;

pub fn parse(file: FileID, cache: &mut FileCache) -> Result<ValueNode, NyarError> {
    let text = cache.fetch(&file)?.to_string();
    match ValueNode::from_str(&text) {
        Ok(o) => {
            Ok(o)
        }
        Err(e) => {
            Err(NyarError::from(e).with_file(file))
        }
    }
}
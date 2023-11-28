use crate::codegen::ValueNode;
use nyar_error::{FileCache, FileID, NyarError};
use std::str::FromStr;

pub fn parse(file: FileID, cache: &mut FileCache) -> Result<ValueNode, NyarError> {
    let text = cache.fetch(&file)?.to_string();
    match ValueNode::from_str(&text) {
        Ok(o) => Ok(o),
        Err(e) => Err(NyarError::from(e).with_file(file)),
    }
}

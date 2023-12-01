use crate::codegen::ValueNode;
use nyar_error::{NyarError, SourceCache, SourceID};
use yggdrasil_rt::YggdrasilNode;

#[cfg(test)]
mod tests;

/// Parse the file with from source cache
pub fn parse(file: SourceID, cache: &mut SourceCache) -> Result<ValueNode, NyarError> {
    let text = cache.fetch(&file)?.text();
    match ValueNode::from_str(&text, 0) {
        Ok(o) => Ok(o),
        Err(e) => Err(NyarError::from(e).with_file(file)),
    }
}

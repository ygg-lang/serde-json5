use crate::codegen::{Json5Rule, ValueNode};
use nyar_error::{NyarError, SourceCache, SourceID};
use yggdrasil_rt::{YggdrasilError, YggdrasilNode};

mod parse_tree;
#[cfg(test)]
mod tests;

/// Parse the file with from source cache
pub fn parse(file: SourceID, cache: &mut SourceCache) -> Result<ValueNode, YggdrasilError<Json5Rule>> {
    let text = cache.fetch(&file).unwrap().text();
    match ValueNode::from_str(&text, 0) {
        Ok(o) => Ok(o),
        Err(e) => Err(e),
    }
}

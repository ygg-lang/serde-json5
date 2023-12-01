use crate::codegen::{ObjectKeyNode, SingleStringElementNode, SingleStringTextNode, String1Node, StringNode};
use yggdrasil_rt::YggdrasilNode;

#[test]
fn test_object_key() -> std::io::Result<()> {
    match ObjectKeyNode::from_str("a", 0) {
        Ok(_) => {}
        Err(e) => println!("1:{e}"),
    }
    match ObjectKeyNode::from_str("'a'", 0) {
        Ok(_) => {}
        Err(e) => println!("2:{e}"),
    }
    match ObjectKeyNode::from_str("\"a\"", 0) {
        Ok(_) => {}
        Err(e) => println!("3:{e}"),
    }
    Ok(())
}

#[test]
fn test_string() -> std::io::Result<()> {
    match StringNode::from_str("'a'", 0) {
        Ok(_) => {}
        Err(e) => println!("1:{e}"),
    }
    match String1Node::from_str("'a'", 0) {
        Ok(_) => {}
        Err(e) => println!("2:{e}"),
    }
    match SingleStringElementNode::from_str("a", 0) {
        Ok(_) => {}
        Err(e) => println!("3:{e}"),
    }
    match SingleStringTextNode::from_str("a", 0) {
        Ok(_) => {}
        Err(e) => println!("4:{e}"),
    }

    Ok(())
}

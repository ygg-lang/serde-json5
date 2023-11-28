use nyar_error::{FileCache};
use json5_parser::parse;

#[test]
fn ready() {
    println!("it works!")
}


#[test]
fn test_external() -> std::io::Result<()> {
    let mut store = FileCache::default();
    let id = store.load_text("{\"a\" ", "test.json");
    match parse(id, &mut store) {
        Ok(_) => {}
        Err(e) => {
            e.as_report().eprint(&store)?
        }
    }
    Ok(())
}
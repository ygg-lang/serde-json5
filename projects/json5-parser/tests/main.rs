use json5_parser::parse;
use nyar_error::FileCache;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_external() -> std::io::Result<()> {
    let mut store = FileCache::default();
    let id = store.load_text("{\"a\": {\"b\": 1 2},  ", "test.json");
    match parse(id, &mut store) {
        Ok(_) => {}
        Err(e) => e.as_report().eprint(&store)?,
    }
    Ok(())
}

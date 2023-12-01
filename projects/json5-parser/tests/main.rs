use json5_parser::parse;
use nyar_error::SourceCache;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_external() -> std::io::Result<()> {
    let mut store = SourceCache::default();
    let id = store.load_text("{a: 1}", "test.json");
    match parse(id, &mut store) {
        Ok(_) => {}
        Err(e) => e.as_report().eprint(&store)?,
    }
    Ok(())
}

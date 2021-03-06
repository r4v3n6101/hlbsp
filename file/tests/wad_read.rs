#[test]
fn print_entries_name_and_type() {
    let file = std::fs::read(env!("WAD_TEST")).unwrap();
    let wad = file::wad::Archive::parse(&file).unwrap();
    wad.entries()
        .for_each(|(name, e)| println!("{} - {}", name, e.etype()));
}

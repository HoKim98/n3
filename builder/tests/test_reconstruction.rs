use std::fs;

fn recon(source: &str) {
    let parser = n3_builder::Parser::new();

    let source_recon1 = format!("{:?}", parser.parse_file(source).unwrap());
    println!("{}", &source_recon1);
    let source_recon2 = format!("{:?}", parser.parse_file(&source_recon1).unwrap());

    assert_eq!(source_recon1, source_recon2);
}

#[test]
fn test_dummy() {
    let source = fs::read_to_string("tests/data/nodes/__user__/sample/dummy.n3").unwrap();

    recon(&source);
}

#[test]
fn test_all_externs() {
    for source in n3_builder::n3_std::get_sources().values() {
        recon(&source);
    }
}

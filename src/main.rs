mod parser;
fn main() -> anyhow::Result<()> {
    let path = std::env::args().nth(1).expect("missing file path");
    let contents = std::fs::read_to_string(path)?;
    dbg!(parser::parse_str(&contents));
    Ok(())
}

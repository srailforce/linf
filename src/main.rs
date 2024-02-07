use linf::SymbolicLink;

fn main() -> anyhow::Result<()> {
    let symbolic_links = SymbolicLink::load("./.symbolic_links")?;
    symbolic_links.update()?;
    Ok(())
}
use linf::{request_admin, SymbolicLink};

fn main() -> anyhow::Result<()> {
    request_admin()?;  
    let symbolic_links = SymbolicLink::load("./.symbolic_links")?;
    symbolic_links.update()?;
    Ok(())
}
use std::io::Read;
use std::path::Path;
use std::fs::OpenOptions;
use std::fs::read_dir;

use anyhow::Ok;

pub struct SymbolicLink {
    source: String,
    files: Vec<String>,
}

impl SymbolicLink {
    pub fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let mut file = OpenOptions::new()
            .read(true)
            .open(&path)
            .map_err(|_| anyhow::anyhow!("failed to open file {}", &path.as_ref().display()))?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        let mut lines = buf.lines();
        let source = lines.next().ok_or(anyhow::anyhow!("No source"))?.to_owned();
        let files = lines.map(ToOwned::to_owned).collect();
        Ok(Self {
            source,
            files,
        })
    }

    pub fn update(self: &Self) -> anyhow::Result<()> {
        todo!()
    }
    
    pub(crate) fn make_symbolic_link(path: impl AsRef<Path>, target: impl AsRef<Path>) {
        todo!()
    }
}
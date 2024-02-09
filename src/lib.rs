use std::io::Read;

use console::Style;
use std::fs::OpenOptions;
use std::path::Path;
use std::path::PathBuf;

#[cfg(windows)]
use windows::{
    core::HSTRING, Win32::Foundation::BOOLEAN, Win32::Storage::FileSystem::CreateSymbolicLinkW,
    Win32::Storage::FileSystem::SYMBOLIC_LINK_FLAGS,
    Win32::Storage::FileSystem::SYMBOLIC_LINK_FLAG_DIRECTORY,
};

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
        Ok(Self { source, files })
    }

    pub fn update(&self) -> anyhow::Result<()> {
        self.files
            .iter()
            .map(|file| {
                (
                    PathBuf::from_iter(vec![&self.source, file]),
                    PathBuf::from(file),
                )
            })
            .try_for_each(|(target, path)| SymbolicLink::make_symbolic_link(path, target))
    }

    #[cfg(not(windows))]
    pub(crate) fn make_symbolic_link(
        path: impl AsRef<Path>,
        target: impl AsRef<Path>,
    ) -> anyhow::Result<()> {
    }

    #[cfg(windows)]
    pub(crate) fn make_symbolic_link(
        path: impl AsRef<Path>,
        target: impl AsRef<Path>,
    ) -> anyhow::Result<()> {
        let bright_green = Style::new().green().bright();
        println!(
            "{:} -> {:}",
            bright_green.apply_to(path.as_ref().display()),
            target.as_ref().display()
        );
        let path = path.as_ref();
        if !target.as_ref().exists() {
            Err(anyhow::anyhow!("{} not exists", target.as_ref().display()))?;
        }
        if path.exists() {
            if path.is_dir() {
                std::fs::remove_dir(path)?;
            } else {
                std::fs::remove_file(path)?;
            }
        }

        let flag = if target.as_ref().is_dir() {
            SYMBOLIC_LINK_FLAG_DIRECTORY
        } else {
            SYMBOLIC_LINK_FLAGS(Default::default())
        };

        let path = HSTRING::from(path.as_os_str());
        let target = HSTRING::from(target.as_ref().as_os_str());

        unsafe {
            if let BOOLEAN(0) = CreateSymbolicLinkW(&path, &target, flag) {
                Err(anyhow::anyhow!("failed to create symbolic link"))?;
            }
        }

        Ok(())
    }
}

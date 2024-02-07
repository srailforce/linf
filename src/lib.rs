use std::io::Read;
use std::os::windows;
use std::path::Path;
use std::fs::OpenOptions;
use std::fs::read_dir;

use anyhow::Ok;
use ::windows::core::HSTRING;
use windows::Win32::Foundation::APPMODEL_ERROR_DYNAMIC_PROPERTY_INVALID;
use ::windows::Win32::Foundation::BOOLEAN;
use ::windows::Win32::Storage::FileSystem::CreateSymbolicLinkW;
use ::windows::Win32::Storage::FileSystem::SYMBOLIC_LINK_FLAGS;
use ::windows::Win32::Storage::FileSystem::SYMBOLIC_LINK_FLAG_DIRECTORY;

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
    
    pub(crate) fn make_symbolic_link(path: impl AsRef<Path>, target: impl AsRef<Path>) -> anyhow::Result<()> {
        let path = path.as_ref();
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
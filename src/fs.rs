use std::path::Path;

pub fn must_read(path: &Path) -> Vec<u8> {
    std::fs::read(path)
        .unwrap_or_else(|err| panic!("Failed to read provided file; err={err}; path={path:?}"))
}

pub fn must_write(path: &Path, data: &[u8]) {
    std::fs::write(path, data)
        .unwrap_or_else(|err| panic!("Failed to read provided file; err={err}; path={path:?}"));
}

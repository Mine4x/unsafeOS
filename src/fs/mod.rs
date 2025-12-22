extern crate alloc;
use alloc::{string::{String, ToString}, vec::Vec};
use spin::Mutex;

mod ramfs;
use ramfs::RamFs;

static ROOT_FS: Mutex<Option<RamFs>> = Mutex::new(None);

pub async fn init() {
    let mut __guard__ = ROOT_FS.lock();
    let fs = RamFs::new();
    fs.create_dir("/welcome").unwrap();
    fs.write("/welcome/hello.txt", b"hello! welcome to unsafeOS! this filesystem only runs on your memmory!").unwrap();
    *__guard__ = Some(fs);
}

fn with_fs<R>(f: impl FnOnce(&RamFs) -> Result<R, String>) -> Result<R, String> {
    let guard = ROOT_FS.lock();
    let fs = guard.as_ref().ok_or_else(|| "Filesystem not initialized".to_string())?;
    f(fs)
}

/// Read a file;
pub fn read(path: &str) -> Result<Vec<u8>, String> {
    with_fs(|fs| fs.read(path))
}

/// Write contents to a existing file or create a new file
pub fn write(path: &str, data: &[u8]) -> Result<(), String> {
    with_fs(|fs| fs.write(path, data))
}

/// Create a Dir
pub fn create_dir(path: &str) -> Result<(), String> {
    with_fs(|fs| fs.create_dir(path))
}

/// List the contents of a Dir
pub fn list_dir(path: &str) -> Result<Vec<String>, String> {
    with_fs(|fs| fs.list_dir(path))
}
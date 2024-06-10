use std::fs::File;
use std::io::Read;

use crate::PauseMenuDataMgr;

pub fn get_pmdm(path: &str) -> Option<Vec<u8>> {
    // read pmdm file
    let mut file = File::open(path).ok()?;
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).ok()?;

    if buffer.len() != std::mem::size_of::<PauseMenuDataMgr>() {
        panic!("File size not the same as PMDM size");
    }

    Some(buffer)
}

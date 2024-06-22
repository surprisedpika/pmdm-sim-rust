use std::fs::File;
use std::io;
use std::io::Read;

use serde_json;

use crate::PauseMenuDataMgr;

pub fn read_dump(path: &str) -> io::Result<(u64, Vec<u8>)> {
    // Read dump
    let mut file = File::open(path)?;
    let mut buffer = vec![];
    file.read_to_end(&mut buffer)?;

    // Seperate address and data
    let address = u64::from_le_bytes(buffer[..0x8].try_into().unwrap());
    let data = buffer[0x8..].to_vec();

    // Check dump size
    assert_eq!(
        data.len(), std::mem::size_of::<PauseMenuDataMgr>(),
        "PMDM dump size does not match (expected 0x{:x}, found 0x{:x})",
        std::mem::size_of::<PauseMenuDataMgr>() + 8, buffer.len()
    );

    Ok((address, data))
}

pub fn read_translations(path: &str) -> io::Result<serde_json::Value> {
    let file = File::open(path)?;
    let translations = serde_json::from_reader(file).unwrap();
    Ok(translations)
}

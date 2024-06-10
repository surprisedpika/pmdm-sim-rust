use std::fs::File;
use std::io;
use std::io::Read;

use crate::PauseMenuDataMgr;

pub fn read_dump(path: &str) -> io::Result<(u64, Vec<u8>)> {
    // Read dump
    let mut file = File::open(path)?;
    let mut buffer = vec![];
    file.read_to_end(&mut buffer)?;

    // Seperate address and data
    let address = u64::from_le_bytes(buffer[..8].try_into().unwrap());
    let data = buffer[8..].to_vec();

    // Check dump size
    assert_eq!(
        data.len(), std::mem::size_of::<PauseMenuDataMgr>(),
        "PMDM dump size does not match (expected 0x{:x}, found 0x{:x})",
        std::mem::size_of::<PauseMenuDataMgr>() + 8, data.len()
    );

    Ok((address, data))
}

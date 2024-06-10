use std::fs::File;
use std::io::Read;

use crate::PauseMenuDataMgr;

pub fn get_pmdm(path: &str) -> Option<(u64, Vec<u8>)> {
    // read pmdm file
    let mut file = File::open(path).ok()?;
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).ok()?;

    let address = u64::from_le_bytes((&buffer[0..8]).try_into().unwrap());
    let data = buffer[8..].to_vec();

    if data.len() != std::mem::size_of::<PauseMenuDataMgr>() {
        panic!("File size not the same as PMDM size");
    }

    Some((address, data))
}

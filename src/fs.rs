use std::fs::File;
use std::io::Read;

pub fn get_pmdm(path: &str) -> Option<Vec<u8>> {
    // read pmdm file
    let mut file = File::open(path).ok()?;
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).ok()?;

    Some(buffer)
}

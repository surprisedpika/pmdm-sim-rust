use std::fs::File;
use std::io::Read;

fn clean_dump(data: Vec<u8>) -> Vec<u8> {
    //TODO: Implement
    return data;
}

pub fn get_pmdm(path: &str, is_dirty: bool) -> Option<Vec<u8>> {
    // read pmdm file
    let mut file = File::open(path).ok()?;
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).ok()?;

    // clean up if dirty
    if is_dirty {
        buffer = clean_dump(buffer);
    }

    Some(buffer)
}

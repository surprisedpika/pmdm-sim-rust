#![feature(generic_const_exprs)]
#![feature(map_try_insert)]

mod fs;
mod mem;
mod pmdm;

use fs::*;
use mem::*;
use pmdm::*;

fn main() {
    // Initialize PMDM
    let (pmdm_address, pmdm_data) = read_dump("pmdm.bin").unwrap();
    let mut memory = Memory::init(pmdm_address, pmdm_data);
    let pmdm: PauseMenuDataMgr = memory.read(pmdm_address).unwrap();

    // Initialize translations
    let translations = read_translations("botw_names.json").unwrap();

    let first_item: PouchItem = memory.read(u64::from_le(
        pmdm.item_lists.list1.start_end.next
    )).unwrap();
    println!("firstItem: {}", translate_name(
        first_item.name.to_string().as_str(), translations
    ).unwrap());
}

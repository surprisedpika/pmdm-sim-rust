#![feature(generic_const_exprs)]
#![feature(map_try_insert)]
#![feature(slice_ptr_get)]

mod fs;
mod mem;
mod pmdm;
mod types;

use fs::*;
use mem::*;
use pmdm::*;
use types::*;

const PMDM_BASE: u64 = 0xa982c8b0;

fn main() {
    // Initialize PMDM
    let (pmdm_address, pmdm_data) = read_dump("pmdm.bin").unwrap();
    println!("PauseMenuDataMgr::sInstance == 0x{:x}", pmdm_address);
    println!("Heap base: 0x{:x}", pmdm_address - PMDM_BASE);
    let mut memory = Memory::init(pmdm_address, pmdm_data);
    let pmdm: Box<PauseMenuDataMgr> = memory.read(pmdm_address).unwrap();

    // Initialize translations
    let translations = read_translations("botw_names.json").unwrap();

    let first_item: Box<PouchItem> = memory.read(u64::from_le(
        pmdm.item_lists.list1.start_end.next
    )).unwrap();
    let actor_name = first_item.name.to_string();
    println!("firstItem: {}", if let Some(name) = translate_name(
        actor_name.as_str(), translations
    ) { name } else { actor_name });
}

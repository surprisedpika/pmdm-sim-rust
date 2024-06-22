#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(map_try_insert)]
#![feature(offset_of_nested)]
#![feature(slice_ptr_get)]

mod fs;
mod mem;
mod pmdm;
mod traits;
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
    let pmdm_ptr = Pointer::<PauseMenuDataMgr>::new(pmdm_address);
    let mut pmdm = pmdm_ptr.read(&memory).unwrap();

    // Initialize translations
    let translations = read_translations("botw_names.json").unwrap();

    let list1 = pmdm.item_lists.list1;
    let first_item: Box<PouchItem> = (
        list1.start_end.next.to_ne() - list1.offset as u64
    ).cast().read(&memory).unwrap();
    let actor_name = first_item.name.to_string();
    println!("firstItem: {}", if let Some(name) = translate_name(
        actor_name.as_str(), translations
    ) { name } else { actor_name });
}

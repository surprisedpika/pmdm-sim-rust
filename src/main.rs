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
}

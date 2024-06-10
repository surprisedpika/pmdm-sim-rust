#![feature(generic_const_exprs)]
#![feature(map_try_insert)]

mod mem;
mod pmdm;
mod fs;

use mem::*;
use pmdm::*;
use fs::*;

fn main() {
    let (pmdm_address, pmdm_data) = get_pmdm("pmdm.bin").unwrap();
    let memory = Memory::init(pmdm_address, pmdm_data);
    let pmdm: PauseMenuDataMgr = memory.read(pmdm_address).unwrap();
}

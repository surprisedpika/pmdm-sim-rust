#![feature(generic_const_exprs)]
#![feature(map_try_insert)]

mod mem;
mod pmdm;
mod fs;

use mem::*;
use pmdm::*;
use fs::*;

fn main() {
    println!("Hello, World!");
}

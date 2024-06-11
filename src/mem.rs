use std::collections::HashMap;
use std::marker::PhantomData;
use std::mem;

const ASLR_START: u64 = 0x8000000;
const ASLR_END: u64 = 0x8000000000;

pub struct Memory {
    memory: HashMap<u64, Vec<u8>>,
}

impl Memory {
    // Initialize memory with data
    pub fn init(address: u64, data: Vec<u8>) -> Memory {
        Memory { memory: HashMap::from([(address, data)]) }
    }

    // Read object from memory
    pub fn read<T>(&self, address: u64) -> Result<Box<T>, String> where [(); mem::size_of::<T>()]: {
        let end = address + mem::size_of::<T>() as u64;

        // Check if object is in ASLR range
        if address < ASLR_START || end > ASLR_END {
            return Err(format!("Address range {:x}-{:x} is outside ASLR range", address, end));
        }

        // Find block containing address range
        let (start, block) = self.memory.iter().find(
            |&(start, block)| *start <= address && end - *start <= block.len() as u64
        ).ok_or(format!("Uninitialized memory in range {:x}-{:x}", address, end))?;

        // Read and box object
        Ok(unsafe { Box::from_raw(mem::transmute::<*mut u8, *mut T>(Box::into_raw(block[
            (address - *start) as usize..(end - *start) as usize
        ].to_vec().into_boxed_slice()).as_mut_ptr())) })
    }

    // Write object to memory
    pub fn write<T>(&mut self, address: u64, object: Box<T>) -> Result<(), String> {
        let end = address + mem::size_of::<T>() as u64;

        // Check if object is in ASLR range
        if address < ASLR_START || end > ASLR_END {
            return Err(format!("Address range {:x}-{:x} is outside ASLR range", address, end));
        }

        // Find block containing address range, if it exists
        let block = match self.memory.iter_mut().find(
            |(start, block)| **start <= address && end - **start <= block.len() as u64
        ).ok_or(()) {
            Ok(block) => block,
            Err(_) => {
                // Remove blocks fully enclosed in address range
                self.memory.retain(|&start, block| !(
                    address < start && start < end && start + block.len() as u64 <= end
                ));

                // Find remainder of block containing address range
                // end and remove block, if it exists
                let next_block = if let Some(block) = self.memory.clone().iter().find(
                    |&(start, block)| address < *start && *start <= end
                    && end < *start + block.len() as u64
                ) { &self.memory.remove(block.0).unwrap()[(end - *block.0) as usize..] } else {
                    &[]
                };

                // Find block containing or adjacent to address, if it exists
                match self.memory.iter_mut().find(
                    |(start, block)| **start <= address && address <= **start + block.len() as u64
                ).ok_or(()) {
                    Ok(block) => {
                        // Resize block
                        let block_remainder = end - *block.0 - block.1.len() as u64;
                        block.1.resize_with(
                            block.1.len() + block_remainder as usize + next_block.len(),
                            Default::default
                        );

                        // Write next block data
                        block.1[(end - block.0) as usize..].copy_from_slice(next_block);
                        block
                    },
                    Err(_) => {
                        // Create block
                        let block = self.memory.try_insert(
                            address, vec![0u8; mem::size_of::<T>() + next_block.len()]
                        ).unwrap();

                        // Write next block data
                        block[mem::size_of::<T>()..].copy_from_slice(next_block);
                        (&address, block)
                    },
                }
            },
        };

        // Write object
        unsafe { block.1[
            (address - *block.0) as usize..(end - *block.0) as usize
        ].as_mut_ptr().copy_from(mem::transmute(Box::into_raw(object)), mem::size_of::<T>()); }

        Ok(())
    }
}

pub struct Pointer<T = u8> {
    pub address: u64,
    phantom: PhantomData<T>,
}

impl<T> Pointer<T> {
    // Create pointer to address
    pub fn new(address: u64) -> Pointer<T> { Pointer { address, phantom: PhantomData } }

    // Dereference and read from pointer
    pub fn read(&self, memory: &Memory) -> Result<Box<T>, String> where [(); mem::size_of::<T>()]: {
        memory.read(self.address)
    }

    // Dereference and write to pointer
    pub fn write(&self, object: Box<T>, memory: &mut Memory) -> Result<(), String> {
        memory.write(self.address, object)
    }
}

use std::collections::HashMap;
use std::mem;

const ASLR_START: u64 = 0x8000000;
const ASLR_END: u64 = 0x8000000000;

struct Memory {
    memory: HashMap<u64, Vec<u8>>,
}

impl Memory {
    // Initialize memory with data
    fn init(address: u64, data: Vec<u8>) -> Memory {
        Memory { memory: HashMap::from([(address, data)]) }
    }

    // Read object from memory
    fn read<T>(&self, address: u64) -> Result<T, String> where [(); mem::size_of::<T>()]: {
        let end = address + mem::size_of::<T>() as u64;

        // Check if object is in ASLR range
        if address < ASLR_START || end > ASLR_END {
            panic!("Address range {:x}-{:x} is outside ASLR range", address, end);
        }

        // Find block containing address range
        let (start, block) = self.memory.iter().find(
            |(start, block)| **start <= address && end - **start <= block.len() as u64
        ).ok_or(format!("Uninitialized memory in range {:x}-{:x}", address, end))?;

        // Read object
        unsafe { Ok(mem::transmute_copy::<[u8; mem::size_of::<T>()], T>(
            block[(address - *start) as usize..(end - *start) as usize].try_into().unwrap()
        )) }
    }

    // Write object to memory
    fn write<T>(&mut self, address: u64, object: T) {
        let end = address + mem::size_of::<T>() as u64;

        // Check if object is in ASLR range
        if address < ASLR_START || end > ASLR_END {
            panic!("Address range {:x}-{:x} is outside ASLR range", address, end);
        }

        // Find block containing address range, if it exists
        let block = match self.memory.iter_mut().find(
            |(start, block)| **start <= address && end - **start <= block.len() as u64
        ).ok_or(()) {
            Ok(block) => block,
            Err(_) => {
                // Remove blocks fully enclosed in address range
                self.memory.retain(|start, block| !(
                    address < *start && *start < end && *start + block.len() as u64 <= end
                ));

                // Find remainder of block containing address range
                // end and remove block, if it exists
                let next_block = if let Some(block) = self.memory.clone().iter().find(
                    |(start, block)| address < **start && **start <= end
                    && end < **start + block.len() as u64
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
        unsafe { block.1[(address - *block.0) as usize..(end - *block.0) as usize].copy_from_slice(
            mem::transmute_copy::<T, &[u8]>(&object)
        ); }
    }
}

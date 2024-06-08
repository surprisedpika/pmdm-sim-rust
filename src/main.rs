const NUM_INGREDIENTS_MAX: i32 = 5;
const POUCH_ITEM_SIZE: u32 = 0x298;
const NUM_POUCH_ITEMS: u32 = 420;
const MEMORY_SIZE: usize = (POUCH_ITEM_SIZE as usize) * (NUM_POUCH_ITEMS as usize) * 8;

#[repr(i32)]
enum PouchItemType {
    Sword = 0,
    Bow = 1,
    Arrow = 2,
    Shield = 3,
    ArmorHead = 4,
    ArmorUpper = 5,
    ArmorLower = 6,
    Material = 7,
    Food = 8,
    KeyItem = 9,
    Invalid = -1,
}

#[repr(i32)]
enum ItemUse {
    WeaponSmallSword = 0,
    WeaponLargeSword = 1,
    WeaponSpear = 2,
    WeaponBow = 3,
    WeaponShield = 4,
    ArmorHead = 5,
    ArmorUpper = 6,
    ArmorLower = 7,
    Item = 8,
    ImportantItem = 9,
    CureItem = 10,
    Invalid = -1,
}

struct ListNode {
    prev: u64,
    next: u64,
}

struct FixedSafeString {
    vptr: u64,
    string_top: u64,
    buffer_size: i32,
    buffer: [u8; 64],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct CookData {
    health_recover: i32,
    effect_duration: i32,
    sell_price: i32,
    effect_id: f32,
    effect_level: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct WeaponData {
    modifier_value: u32,
    unused1: u32,
    modifier: u32,
    unused2: u32,
    unused3: u32,
}

#[repr(C)]
union Data {
    cook: CookData,
    weapon: WeaponData,
}

struct FreeList {
    free: u64,
    work: u64,
}

struct Node<T> {
    next_node: u64,
    elem: T,
}

struct FixedObjArray<T> {
    ptr_num: i32,
    ptr_num_max: i32,
    ptrs: u64,
    free_list: FreeList,
    work: [Node<T>; NUM_INGREDIENTS_MAX as usize],
}

struct PouchItem {
    vptr: u64,
    list_node: ListNode,
    item_type: PouchItemType,
    item_use: ItemUse,
    value: i32,
    equipped: bool,
    in_inventory: bool,
    // 2 bytes padding
    name: FixedSafeString,
    // 4 bytes padding
    data: Data,
    // 4 bytes padding
    ingredients: FixedObjArray<FixedSafeString>,
    // 4 bytes padding at the end of each ingredients.mWork[n]
}

/* PouchItem in memory: 

*/

fn encode_pouch_item(memory: &mut [u8; MEMORY_SIZE], item: PouchItem, start_address: usize) {
    if start_address > MEMORY_SIZE - (POUCH_ITEM_SIZE as usize) {
        println!(
            "Memory out of bounds for PouchItem from: {} to {}",
            start_address,
            start_address + (POUCH_ITEM_SIZE as usize)
        );
        return;
    }

    let mut current_address: usize = start_address;
    let mut current_size: usize;

    // vptr
    current_size = 8;
    let vptr = item.vptr.to_le_bytes();
    memory[current_address..current_address + current_size].copy_from_slice(&vptr);
    current_address += current_size;

    // list_node
    current_size = 8;
    let list_node_mprev = item.list_node.prev.to_le_bytes();
    memory[current_address..current_address + current_size].copy_from_slice(&list_node_mprev);
    current_address += current_size;

    // list_node
    current_size = 8;
    let list_node_mnext = item.list_node.next.to_le_bytes();
    memory[current_address..current_address + current_size].copy_from_slice(&list_node_mnext);
    current_address += current_size;

    // item_type
    current_size = 4;
    let m_type = (item.item_type as u32).to_le_bytes();
    memory[current_address..current_address + current_size].copy_from_slice(&m_type);
    current_address += current_size;

    // item_use
    current_size = 4;
    let m_use = (item.item_use as u32).to_le_bytes();
    memory[current_address..current_address + current_size].copy_from_slice(&m_use);
    current_address += current_size;

    // value
    current_size = 4;
    let m_value = item.value.to_le_bytes();
    memory[current_address..current_address + current_size].copy_from_slice(&m_value);
    current_address += current_size;

    // equipped
    memory[current_address] = item.equipped as u8;
    current_address += 1;

    // in_inventory
    memory[current_address] = item.in_inventory as u8;
    current_address += 1;

    // padding
    current_address += 2;

    // name
    let m_name = item.name;

    // name.vptr
    current_size = 8;
    let name_vptr = m_name.vptr.to_le_bytes();
    memory[current_address..current_address + current_size].copy_from_slice(&name_vptr);
    current_address += current_size;

    // name.string_top
    current_size = 8;
    let name_string_top = m_name.string_top.to_le_bytes();
    memory[current_address..current_address + current_size].copy_from_slice(&name_string_top);
    current_address += current_size;

    // name.buffer_size
    current_size = 4;
    let name_buffer_size = m_name.buffer_size.to_le_bytes();
    memory[current_address..current_address + current_size].copy_from_slice(&name_buffer_size);
    current_address += current_size;

    // name.mBuffer (64 total bytes)
    let name_m_buffer = m_name.buffer;
    for byte in name_m_buffer.iter() {
        memory[current_address] = *byte;
        current_address += 1;
    }

    // padding
    current_address += 4;

    // data
    let m_data = item.data;
    unsafe {
        const TOTAL_M_DATA_SIZE: usize = 20;
        const TOTAL_M_DATA_ITEMS: usize = 5;

        let m_data_bits: [u8; TOTAL_M_DATA_SIZE] = std::mem::transmute(m_data);
        current_size = (TOTAL_M_DATA_SIZE / TOTAL_M_DATA_ITEMS) as usize;

        for i in 0..current_size {
            let start_index = i * current_size;
            let end_index = start_index + current_size;

            let item_big_endian = &m_data_bits[start_index..end_index];

            let mut item_little_endian = [0; TOTAL_M_DATA_SIZE / TOTAL_M_DATA_ITEMS];
            for (j, &element) in item_big_endian.iter().enumerate() {
                item_little_endian[3 - j] = element;
            }

            memory[current_address..current_address + current_size].copy_from_slice(
                &item_little_endian
            );
            current_address += current_size;
        }
    }

    // padding
    current_address += 4;

    //TODO: mIngredients
}

fn decode_pouch_item(memory: &mut [u8; MEMORY_SIZE], address: usize) -> Option<PouchItem> {
    if address > MEMORY_SIZE - (POUCH_ITEM_SIZE as usize) {
        println!("Memory out of bounds: {}", address);
        return None;
    }
    panic!("Unimplemented");
}

fn main() {
    let memory: [u8; MEMORY_SIZE] = [0; MEMORY_SIZE];
}

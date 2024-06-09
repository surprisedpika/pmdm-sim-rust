use std::mem;

use serde_json::Value;

const NUM_POUCH_ITEMS_MAX: usize = 420;
const NUM_INGREDIENTS_MAX: usize = 5;

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
    ArmorLower = 6,
    ArmorUpper = 7,
    Item = 8,
    ImportantItem = 9,
    CureItem = 10,
    Invalid = -1,
}

#[repr(C)]
struct ListNode {
    prev: u64,
    next: u64,
}

#[repr(C)]
pub struct FixedSafeString<const L: usize> {
    pub vptr: u64,
    pub string_top: u64,
    pub buffer_size: i32,
    pub buffer: [u8; L],
}

#[repr(C)]
struct CookData {
    health_recover: i32,
    effect_duration: i32,
    sell_price: i32,
    effect_id: f32,
    effect_level: f32,
}

#[repr(C)]
struct WeaponData {
    modifier_value: u32,
    unused1: u32,
    modifier: u32,
}

#[repr(C)]
union Data {
    cook: mem::ManuallyDrop<CookData>,
    weapon: mem::ManuallyDrop<WeaponData>,
}

#[repr(C)]
struct FreeList {
    free: u64,
    work: u64,
}

#[repr(C)]
struct Node<T> {
    next_node: u64,
    elem: T,
}

#[repr(C)]
struct FixedObjArray<T, const L: usize> {
    ptr_num: i32,
    ptr_num_max: i32,
    ptrs: u64,
    free_list: FreeList,
    work: [Node<T>; L],
}

#[repr(C)]
struct PouchItem {
    vptr: u64,
    list_node: ListNode,
    item_type: PouchItemType,
    item_use: ItemUse,
    value: i32,
    equipped: bool,
    in_inventory: bool,
    name: FixedSafeString<64>,
    data: Data,
    ingredients: FixedObjArray<FixedSafeString<64>, NUM_INGREDIENTS_MAX>,
}

// fn get_readable_name(string: FixedSafeString<64>, lang_data: Value) -> Option<String> {
//     let trimmed_string: Vec<u8> = string.buffer
//         .iter()
//         .take_while(|&&x| x != 0)
//         .cloned()
//         .collect();
//     String::from_utf8(trimmed_string.to_vec())
//         .ok()
//         .and_then(|internal_name|
//             lang_data
//                 .get(&internal_name)?
//                 .as_str()
//                 .map(|name| name.to_string())
//         )
// }

fn get_internal_name(string: FixedSafeString<64>) -> Option<String> {
    let trimmed_string: Vec<u8> = string.buffer
        .iter()
        .take_while(|&&x| x != 0)
        .cloned()
        .collect();
    String::from_utf8(trimmed_string).ok()
}

fn get_readable_name(internal_name: String, lang_data: Value) -> Option<String> {
    lang_data
        .get(&internal_name)?
        .as_str()
        .map(|name| name.to_string())
}
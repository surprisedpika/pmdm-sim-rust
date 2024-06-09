use std::mem;

use serde_json::Value;

pub const NUM_POUCH_ITEMS_MAX: usize = 420;
pub const NUM_INGREDIENTS_MAX: usize = 5;

#[repr(i32)]
pub enum PouchItemType {
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
pub enum ItemUse {
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
pub struct ListNode {
    pub prev: u64,
    pub next: u64,
}

#[repr(C)]
pub struct FixedSafeString<const L: usize> {
    pub vptr: u64,
    pub string_top: u64,
    pub buffer_size: i32,
    pub buffer: [u8; L],
}

impl<const L: usize> ToString for FixedSafeString<L> {
    fn to_string(&self) -> String {
        let trimmed_string: Vec<u8> = self.buffer.iter().take_while(
            |&&x| x != 0
        ).cloned().collect();
        String::from_utf8(trimmed_string).unwrap()
    }
}

#[repr(C)]
pub struct CookData {
    pub health_recover: i32,
    pub effect_duration: i32,
    pub sell_price: i32,
    pub effect_id: f32,
    pub effect_level: f32,
}

#[repr(C)]
pub struct WeaponData {
    pub modifier_value: u32,
    unused: u32,
    pub modifier: u32,
}

#[repr(C)]
pub union Data {
    pub cook: mem::ManuallyDrop<CookData>,
    pub weapon: mem::ManuallyDrop<WeaponData>,
}

#[repr(C)]
pub struct FreeList {
    pub free: u64,
    pub work: u64,
}

#[repr(C)]
pub struct Node<T> {
    pub next_node: u64,
    pub elem: T,
}

#[repr(C)]
pub struct FixedObjArray<T, const L: usize> {
    pub ptr_num: i32,
    pub ptr_num_max: i32,
    pub ptrs: u64,
    pub free_list: FreeList,
    pub work: [Node<T>; L],
}

#[repr(C)]
pub struct PouchItem {
    pub vptr: u64,
    pub list_node: ListNode,
    pub item_type: PouchItemType,
    pub item_use: ItemUse,
    pub value: i32,
    pub equipped: bool,
    pub in_inventory: bool,
    pub name: FixedSafeString<64>,
    pub data: Data,
    pub ingredients: FixedObjArray<FixedSafeString<64>, NUM_INGREDIENTS_MAX>,
}

pub fn get_readable_name(internal_name: &str, lang_data: Value) -> Option<String> {
    lang_data.get(internal_name)?.as_str().map(String::from)
}

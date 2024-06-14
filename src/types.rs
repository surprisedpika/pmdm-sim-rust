use std::marker::PhantomData;
use std::mem;

use serde_json;

use crate::mem::*;

pub const NUM_POUCH_ITEMS_MAX: i32 = 420;
pub const NUM_INGREDIENTS_MAX: i32 = 5;

pub const NUM_POUCH_CATEGORIES: i32 = 7;
pub const NUM_TAB_MAX: i32 = 50;
pub const NUM_GRABBABLE_ITEMS: i32 = 5;

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum PouchCategory {
    Sword = 0,
    Bow = 1,
    Shield = 2,
    Armor = 3,
    Material = 4,
    Food = 5,
    KeyItem = 6,
    Invalid = -1,
}

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum WeaponModifier {
    None = 0x0,
    AddAtk = 0x1,
    AddLife = 0x2,
    AddCrit = 0x4,
    AddThrow = 0x8,
    AddSpreadFire = 0x10,
    AddZoomRapid = 0x20,
    AddRapidFire = 0x40,
    AddSurfMaster = 0x80,
    AddGuard = 0x100,
    IsYellow = 0x80000000,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ListNode {
    pub prev: Pointer<ListNode>,
    pub next: Pointer<ListNode>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct FixedSafeString<const L: usize> {
    pub vptr: Pointer<Pointer>,
    pub string_top: Pointer,
    pub buffer_size: i32,
    pub buffer: [u8; L],
}

impl<const L: usize> ToString for FixedSafeString<L> {
    fn to_string(&self) -> String {
        let trimmed_string: Vec<u8> = self.buffer
            .iter()
            .take_while(|&&x| x != 0)
            .cloned()
            .collect();
        String::from_utf8(trimmed_string).unwrap()
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct CookData {
    pub health_recover: i32,
    pub effect_duration: i32,
    pub sell_price: i32,
    pub effect_id: f32,
    pub effect_level: f32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct WeaponData {
    pub modifier_value: u32,
    unused: u32,
    pub modifier: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union Data {
    pub cook: mem::ManuallyDrop<CookData>,
    pub weapon: mem::ManuallyDrop<WeaponData>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct FreeList {
    pub free: Pointer,
    pub work: Pointer,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Node<T> {
    pub next_node: Pointer<Node<T>>,
    pub elem: T,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct FixedObjArray<T, const L: i32> where [(); L as usize]: {
    pub ptr_num: i32,
    pub ptr_num_max: i32,
    pub ptrs: Pointer<Pointer<T>>,
    pub free_list: FreeList,
    pub work: [Node<T>; L as usize],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct PouchItem {
    pub vptr: Pointer,
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

pub fn translate_name(actor_name: &str, lang_data: serde_json::Value) -> Option<String> {
    lang_data.get(actor_name)?.as_str().map(String::from)
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct MutexType {
    pub state: u8,
    pub is_recursive: bool,
    pub lock_level: i32,
    pub nest_count: i32,
    pub owner_thread: Pointer,
    pub mutex: i32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct CriticalSection {
    // IDisposer
    pub vptr: Pointer,
    pub disposer_heap: Pointer,
    pub list_node: ListNode,

    // CriticalSection
    pub critical_section_inner: MutexType,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct OffsetList<T> {
    // ListImpl
    pub start_end: ListNode,
    pub count: i32,

    // OffsetList
    pub offset: i32,

    phantom: PhantomData<T>,
}

impl<T> OffsetList<T> {
    pub fn nth(&self, n: i32, memory: &Memory) -> Pointer<T> {
        if self.count as u32 <= n as u32 { return Pointer::new(0u64); }
        let mut node = self.start_end.next;
        for _ in 0..n { node = node.read(memory).unwrap().next; }
        (node - self.offset as u64).cast()
    }

    pub fn sort(&self, memory: &mut Memory, cmp: fn(&Memory, Pointer<T>, Pointer<T>) -> i32) {}
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SafeArray<T, const N: i32> where [(); N as usize]: {
    pub buffer: [T; N as usize],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Lists {
    pub list1: OffsetList<PouchItem>,
    pub list2: OffsetList<PouchItem>,
    pub buffer: SafeArray<PouchItem, NUM_POUCH_ITEMS_MAX>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct GrabbedItemInfo {
    pub item: Pointer<PouchItem>,
    _8: bool,
    _9: bool,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct TypedBitFlag<Enum> {
    bits: Enum,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct WeaponModifierInfo {
    flags: TypedBitFlag<WeaponModifier>,
    value: i32,
}

#[derive(Clone)]
pub struct GameDataItem {
    name: String,
    equipped: bool,
    value: i32,
    data: Data,
}

pub type GameData = Vec<GameDataItem>;

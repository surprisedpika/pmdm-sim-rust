use std::mem;

use serde_json::Value;

pub const NUM_POUCH_ITEMS_MAX: i32 = 420;
pub const NUM_INGREDIENTS_MAX: i32 = 5;

pub const NUM_POUCH_CATEGORIES: i32 = 7;
pub const NUM_TAB_MAX: i32 = 50;
pub const NUM_GRABBABLE_ITEMS: i32 = 5;

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
        let trimmed_string: Vec<u8> = self.buffer
            .iter()
            .take_while(|&&x| x != 0)
            .cloned()
            .collect();
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
pub struct FixedObjArray<T, const L: i32> where [(); L as usize]: {
    pub ptr_num: i32,
    pub ptr_num_max: i32,
    pub ptrs: u64,
    pub free_list: FreeList,
    pub work: [Node<T>; L as usize],
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

pub fn translate_name(internal_name: &str, lang_data: Value) -> Option<String> {
    lang_data.get(internal_name)?.as_str().map(String::from)
}

#[repr(C)]
pub struct MutexType {
    pub state: u8,
    pub is_recursive: bool,
    pub lock_level: i32,
    pub nest_count: i32,
    pub owner_thread: u64,
    pub mutex: i32,
}

#[repr(C)]
pub struct CriticalSection {
    // IDisposer
    pub vptr: u64,
    pub disposer_heap: u64,
    pub list_node: ListNode,

    // CriticalSection
    pub critical_section_inner: MutexType,
}

#[repr(C)]
pub struct OffsetList {
    // ListImpl
    pub start_end: ListNode,
    pub count: i32,

    // OffsetList
    pub offset: i32,
}

#[repr(C)]
pub struct SafeArray<T, const N: i32> where [(); N as usize]: {
    pub buffer: [T; N as usize],
}

#[repr(C)]
pub struct Lists {
    pub list1: OffsetList,
    pub list2: OffsetList,
    pub buffer: SafeArray<PouchItem, NUM_POUCH_ITEMS_MAX>,
}

#[repr(C)]
pub struct GrabbedItemInfo {
    pub item: u64,
    _8: bool,
    _9: bool,
}

#[repr(C)]
pub struct PauseMenuDataMgr {
    pub vptr: u64,
    pub singleton_disposer_buf: [u32; 0x8],
    pub crit_section: CriticalSection,
    pub item_lists: Lists,
    pub list_heads: SafeArray<u64, NUM_POUCH_CATEGORIES>,
    pub tabs: SafeArray<u64, NUM_TAB_MAX>,
    pub tabs_type: SafeArray<PouchItemType, NUM_TAB_MAX>,
    pub last_added_item: u64,
    pub last_added_item_tab: i32,
    pub last_added_item_slot: i32,
    pub num_tabs: i32,
    pub grabbed_items: SafeArray<GrabbedItemInfo, NUM_GRABBABLE_ITEMS>,
    item_444f0: u64,
    _444f8: i32,
    _444fc: i32,
    _44500: i32,
    _44504: u32,
    _44508: u32,
    _4450c: u32,
    _44510: u32,
    _44514: u32,
    pub rito_soul_item: u64,
    pub goron_soul_item: u64,
    pub zora_soul_item: u64,
    pub gerudo_soul_item: u64,
    pub can_see_health_bar: bool,
    pub newly_added_item: PouchItem,
    pub is_pouch_for_quest: bool,
    pub equipped_weapons: SafeArray<u64, 4>,
    pub category_to_sort: PouchCategory,
}

use crate::types::*;

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

impl PauseMenuDataMgr {}

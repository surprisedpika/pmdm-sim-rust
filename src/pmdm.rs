use crate::mem::*;
use crate::types::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct PauseMenuDataMgr {
    pub vptr: Pointer,
    pub singleton_disposer_buf: [u32; 0x8],
    pub crit_section: CriticalSection,
    pub item_lists: Lists,
    pub list_heads: SafeArray<Pointer<Pointer<PouchItem>>, NUM_POUCH_CATEGORIES>,
    pub tabs: SafeArray<Pointer<PouchItem>, NUM_TAB_MAX>,
    pub tabs_type: SafeArray<PouchItemType, NUM_TAB_MAX>,
    pub last_added_item: Pointer<PouchItem>,
    pub last_added_item_tab: i32,
    pub last_added_item_slot: i32,
    pub num_tabs: i32,
    pub grabbed_items: SafeArray<GrabbedItemInfo, NUM_GRABBABLE_ITEMS>,
    item_444f0: Pointer<PouchItem>,
    _444f8: i32,
    _444fc: i32,
    _44500: i32,
    _44504: u32,
    _44508: u32,
    _4450c: u32,
    _44510: u32,
    _44514: u32,
    pub rito_soul_item: Pointer<PouchItem>,
    pub goron_soul_item: Pointer<PouchItem>,
    pub zora_soul_item: Pointer<PouchItem>,
    pub gerudo_soul_item: Pointer<PouchItem>,
    pub can_see_health_bar: bool,
    pub newly_added_item: PouchItem,
    pub is_pouch_for_quest: bool,
    pub equipped_weapons: SafeArray<Pointer<PouchItem>, 4>,
    pub category_to_sort: PouchCategory,
}

impl PauseMenuDataMgr {
    // Pick up item
    pub fn get(
        &self, memory: &mut Memory, name: &str, item_type: PouchItemType, value: i32,
        modifier: Pointer<WeaponModifierInfo>
    ) {}

    // Remove item slot while unpaused
    pub fn remove(&self, memory: &mut Memory, item: Pointer<PouchItem>) {}

    // Remove item slot while paused
    pub fn drop(&self, memory: &mut Memory, item: Pointer<PouchItem>) {}

    // Damage or shoot item
    pub fn set_value(&self, memory: &mut Memory, item: Pointer<PouchItem>, value: i32) {}

    // Equip or enable item
    pub fn equip(&self, memory: &mut Memory, item: Pointer<PouchItem>) {}

    // Unequip or disable item
    pub fn unequip(&self, memory: &mut Memory, item_ptr: Pointer<PouchItem>) {
        let mut item = item_ptr.read(memory).unwrap();
        item.equipped = false;
        self.sync(memory);
    }

    // Open inventory
    pub fn pause(&self, memory: &mut Memory) {}

    // Sync GameData
    pub fn sync(&self, memory: &mut Memory) {}

    // Save file
    pub fn save(&self, memory: &Memory) /* -> GameData */ {}

    // Load file
    pub fn load(&self, memory: &mut Memory, file: GameData) {}

    // Break slots
    pub fn offset(&self, memory: &mut Memory, num: u32) {}
}

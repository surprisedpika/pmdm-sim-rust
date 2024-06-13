use std::mem;
use std::ptr;

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
    // Update self from memory
    fn update(&mut self, memory: &mut Memory, this: Pointer<Self>) where Self: Sized {
        unsafe { ptr::copy_nonoverlapping(
            Box::into_raw(this.read(memory).unwrap()), ptr::from_mut(self), mem::size_of::<Self>()
        ); }
    }

    // Pick up item
    pub fn get(
        &mut self, memory: &mut Memory, this: Pointer<Self>, name: &str, item_type: PouchItemType,
        value: i32, modifier: Pointer<WeaponModifierInfo>
    ) {
        self.update(memory, this);
    }

    // Remove item slot while unpaused
    pub fn remove(&mut self, memory: &mut Memory, this: Pointer<Self>, item: Pointer<PouchItem>) {}

    // Remove item slot while paused
    pub fn drop(&mut self, memory: &mut Memory, this: Pointer<Self>, item: Pointer<PouchItem>) {
        (item.cast() + mem::offset_of!(PouchItem, in_inventory) as u64).write(
            memory, Box::new(false)
        ).unwrap();
        self.update(memory, this);
    }

    // Damage or shoot item
    pub fn set_value(
        &mut self, memory: &mut Memory, this: Pointer<Self>, item: Pointer<PouchItem>, value: i32
    ) {
        (item.cast() + mem::offset_of!(PouchItem, value) as u64).write(
            memory, Box::new(value.to_le())
        ).unwrap();
        self.update(memory, this);
    }

    // Equip or enable item
    pub fn equip(&mut self, memory: &mut Memory, this: Pointer<Self>, item: Pointer<PouchItem>) {
        (item.cast() + mem::offset_of!(PouchItem, equipped) as u64).write(
            memory, Box::new(true)
        ).unwrap();
        self.sync(memory, this);
        self.update(memory, this);
    }

    // Unequip or disable item
    pub fn unequip(&mut self, memory: &mut Memory, this: Pointer<Self>, item: Pointer<PouchItem>) {
        (item.cast() + mem::offset_of!(PouchItem, equipped) as u64).write(
            memory, Box::new(false)
        ).unwrap();
        self.sync(memory, this);
        self.update(memory, this);
    }

    // Open inventory
    pub fn pause(&self, memory: &Memory, this: Pointer<Self>) {
        let list1 = self.item_lists.list1;
        if list1.count == 0 { return; }

        // Traverse list1 until mStartEnd reached
        let mut node = list1.start_end.next;
        let mut visited_nodes = vec![node];

        while node != (this + mem::offset_of!(Self, item_lists.list1.start_end) as u64).cast() {
            // Prevent cyclic nodes from hanging
            if visited_nodes.contains(&node) { panic!("Game has frozen due to cyclic nodes"); }
            visited_nodes.push(node);
            node = node.read(memory).unwrap().next;
        }
    }

    // Sync GameData
    pub fn sync(&mut self, memory: &mut Memory, this: Pointer<Self>) {
        self.update(memory, this);
    }

    // Save file
    pub fn save(&self, memory: &Memory, this: Pointer<Self>) /* -> GameData */ {}

    // Load file
    pub fn load(&mut self, memory: &mut Memory, this: Pointer<Self>, file: GameData) {
        self.update(memory, this);
    }

    // Break slots
    pub fn offset(&mut self, memory: &mut Memory, this: Pointer<Self>, num: u32) {
        let lists = self.item_lists;

        (this.cast() + mem::offset_of!(Self, item_lists.list1.count) as u64).write(
            memory, Box::new(lists.list1.count - num as i32)
        ).unwrap();
        (this.cast() + mem::offset_of!(Self, item_lists.list2.count) as u64).write(
            memory, Box::new(lists.list2.count + num as i32)
        ).unwrap();
        self.update(memory, this);
    }
}

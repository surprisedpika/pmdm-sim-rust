use std::mem;

use crate::mem::*;
use crate::traits::*;
use crate::types::*;

const MASTER_SWORD: &str = "Weapon_Sword_070";
const REPEATABLE_KEY_ITEMS: [&str; 10] = [
    "Obj_DLC_HeroSeal_Gerudo",
    "Obj_DLC_HeroSeal_Goron",
    "Obj_DLC_HeroSeal_Rito",
    "Obj_DLC_HeroSeal_Zora",
    "Obj_DLC_HeroSoul_Gerudo",
    "Obj_DLC_HeroSoul_Goron",
    "Obj_DLC_HeroSoul_Rito",
    "Obj_DLC_HeroSoul_Zora",
    "Obj_DungeonClearSeal",
    "Obj_KorokNuts",
];

#[derive(Clone, Copy)]
#[repr(C)]
pub struct PauseMenuDataMgr {
    pub vptr: Pointer<Pointer>,
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
    fn get_item_head(&self, memory: &Memory, category: PouchCategory) -> Pointer<PouchItem> {
        let p_head = self.list_heads.buffer[category as u32 as usize];
        if p_head != Pointer::NULLPTR { *p_head.read(memory).unwrap() } else { Pointer::NULLPTR }
    }

    fn reset_item(&mut self, memory: &mut Memory, this: Pointer<Self>) {
        (this.cast() + mem::offset_of!(Self, newly_added_item.item_type) as u64).write(
            memory, Box::new(PouchItemType::default())
        ).unwrap();
        self.update(memory, &this);
        (this.cast() + mem::offset_of!(Self, newly_added_item.item_use) as u64).write(
            memory, Box::new(ItemUse::default())
        ).unwrap();
        self.update(memory, &this);
        (this.cast() + mem::offset_of!(Self, newly_added_item.value) as u64).write(
            memory, Box::new(i32::default())
        ).unwrap();
        self.update(memory, &this);
        (this.cast() + mem::offset_of!(Self, newly_added_item.equipped) as u64).write(
            memory, Box::new(bool::default())
        ).unwrap();
        self.update(memory, &this);
        (this.cast() + mem::offset_of!(Self, newly_added_item.in_inventory) as u64).write(
            memory, Box::new(bool::default())
        ).unwrap();
        self.update(memory, &this);
        self.newly_added_item.name.clear(memory);
        (this.cast() + mem::offset_of!(Self, newly_added_item.data.cook.effect_id) as u64).write(
            memory, Box::new((CookEffectId::default() as i32 as f32).to_le_bytes())
        ).unwrap();
        self.update(memory, &this);
        (this.cast() + mem::offset_of!(Self, newly_added_item.data.cook.effect_level) as u64).write(
            memory, Box::new(f32::default().to_le_bytes())
        ).unwrap();
        self.update(memory, &this);
    }

    fn reset_item_and_pointers(&mut self, memory: &mut Memory, this: Pointer<Self>) {
        (this.cast() + mem::offset_of!(Self, last_added_item) as u64).write(memory, Box::new(
            Pointer::<PouchItem>::NULLPTR
        )).unwrap();
        self.update(memory, &this);
        (this.cast() + mem::offset_of!(Self, item_444f0) as u64).write(memory, Box::new(
            Pointer::<PouchItem>::NULLPTR
        )).unwrap();
        self.update(memory, &this);
        (this.cast() + mem::offset_of!(Self, _444f8) as u64).write(memory, Box::new(-1)).unwrap();
        self.update(memory, &this);
        self.reset_item(memory, this);
    }

    fn update_inventory_info(&self, memory: &Memory, this: Pointer<Self>) {}

    fn update_list_heads(&mut self, memory: &mut Memory, this: Pointer<Self>) {}

    fn update_after_adding_item(&mut self, memory: &mut Memory, this: Pointer<Self>) {
        if self.item_lists.list1.count == 0 { return; }

        (this.cast() + mem::offset_of!(Self, category_to_sort) as u64).write(
            memory, Box::new((PouchCategory::Invalid as i32).to_le())
        ).unwrap();
        self.update(memory, &this);
        let list1 = this.cast::<OffsetList<PouchItem>>() + mem::offset_of!(
            Self, item_lists.list1
        ) as u64;
        list1.read(memory).unwrap().sort(memory, list1, Self::sort_predicate);

        self.update_inventory_info(memory, this);
        self.update_list_heads(memory, this);
        self.sync(memory, this);
    }

    fn sort_predicate(memory: &Memory, lhs: Pointer<PouchItem>, rhs: Pointer<PouchItem>) -> i32 {
        0
    }

    // Check for loops in list1
    fn traverse_list1(&self, memory: &Memory, this: Pointer<Self>) {
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

    // Check for loops in list2
    fn traverse_list2(&self, memory: &Memory, this: Pointer<Self>) {
        let list2 = self.item_lists.list2;
        if list2.count == 0 { return; }

        // Traverse list1 until mStartEnd reached
        let mut node = list2.start_end.next;
        let mut visited_nodes = vec![node];

        while node != (this + mem::offset_of!(Self, item_lists.list2.start_end) as u64).cast() {
            // Prevent cyclic nodes from hanging
            if visited_nodes.contains(&node) { panic!("Game has frozen due to cyclic nodes"); }
            visited_nodes.push(node);
            node = node.read(memory).unwrap().next;
        }
    }

    // Pick up item
    pub fn get(
        &mut self, memory: &mut Memory, this: Pointer<Self>, name: &str, item_type: PouchItemType,
        value: i32, modifier: Pointer<WeaponModifierInfo>
    ) {
        if item_type == PouchItemType::KeyItem && !REPEATABLE_KEY_ITEMS.contains(&name) {
            self.traverse_list1(memory, this);
            let mut item_ptr = self.get_item_head(memory, PouchCategory::KeyItem);

            while item_ptr != Pointer::NULLPTR && item_ptr.read(
                memory
            ).unwrap().item_type == PouchItemType::KeyItem {
                let mut item = item_ptr.read(memory).unwrap();
                if item.in_inventory && item.name.is_equal_str(
                    memory, item_ptr.cast() + mem::offset_of!(PouchItem, name) as u64, name
                ) { return; }
                self.update(memory, &this);

                item_ptr = self.item_lists.list1.next(
                    memory, this.cast() + mem::offset_of!(Self, item_lists.list1) as u64, item_ptr
                );
            }
        }
        else if item_type == PouchItemType::Sword && name == MASTER_SWORD {
            self.traverse_list1(memory, this);
            let mut item_ptr = self.get_item_head(memory, PouchCategory::Sword);

            if item_ptr != Pointer::NULLPTR {
                let mut item = item_ptr.read(memory).unwrap();

                while item.item_type == PouchItemType::Sword {
                    if !item.in_inventory || !item.name.is_equal_str(
                        memory, item_ptr.cast() + mem::offset_of!(PouchItem, name) as u64, name
                    ) {
                        item_ptr = self.item_lists.list1.next(memory, this.cast() + mem::offset_of!(
                            Self, item_lists.list1
                        ) as u64, item_ptr);
                        if item_ptr == Pointer::NULLPTR { break; }
                        item.update(memory, &item_ptr);
                        continue;
                    }

                    (item_ptr.cast() + mem::offset_of!(PouchItem, value) as u64).write(
                        memory, Box::new(i32::default())
                    ).unwrap();
                    self.update(memory, &this);
                    item.update(memory, &item_ptr);
                    (item_ptr.cast() + mem::offset_of!(PouchItem, equipped) as u64).write(
                        memory, Box::new(bool::default())
                    ).unwrap();
                    self.update(memory, &this);
                    item.update(memory, &item_ptr);

                    (this.cast() + mem::offset_of!(Self, last_added_item) as u64).write(
                        memory, Box::new(if item.value > 0 { item_ptr } else { Pointer::NULLPTR })
                    ).unwrap();
                    self.update(memory, &this);
                    self.reset_item(memory, this);
                    return;
                }
            }
        }

        if item_type == PouchItemType::Invalid { return; }

        // TODO

        self.update_after_adding_item(memory, this);
    }

    // Remove item slot while unpaused
    pub fn remove(&mut self, memory: &mut Memory, this: Pointer<Self>, item: Pointer<PouchItem>) {
        if self.item_444f0 == item {
            (this.cast() + mem::offset_of!(Self, item_444f0) as u64).write(
                memory, Box::new(Pointer::<PouchItem>::NULLPTR)
            ).unwrap();
            self.update(memory, &this);
        }
        if self.last_added_item == item {
            (this.cast() + mem::offset_of!(Self, last_added_item) as u64).write(
                memory, Box::new(Pointer::<PouchItem>::NULLPTR)
            ).unwrap();
            self.update(memory, &this);
        }

        self.item_lists.list1.erase(
            memory, this.cast() + mem::offset_of!(Self, item_lists.list1) as u64, item
        );

        self.sync(memory, this);
    }

    // Remove item slot while paused
    pub fn drop(&mut self, memory: &mut Memory, this: Pointer<Self>, item: Pointer<PouchItem>) {
        (item.cast() + mem::offset_of!(PouchItem, in_inventory) as u64).write(
            memory, Box::new(false)
        ).unwrap();
        self.update(memory, &this);
    }

    // Damage or shoot item
    pub fn set_value(
        &mut self, memory: &mut Memory, this: Pointer<Self>, item: Pointer<PouchItem>, value: i32
    ) {
        (item.cast() + mem::offset_of!(PouchItem, value) as u64).write(
            memory, Box::new(value.to_le())
        ).unwrap();
        self.update(memory, &this);
    }

    // Equip or enable item
    pub fn equip(&mut self, memory: &mut Memory, this: Pointer<Self>, item: Pointer<PouchItem>) {
        (item.cast() + mem::offset_of!(PouchItem, equipped) as u64).write(
            memory, Box::new(true)
        ).unwrap();
        self.update(memory, &this);
        self.sync(memory, this);
    }

    // Unequip or disable item
    pub fn unequip(&mut self, memory: &mut Memory, this: Pointer<Self>, item: Pointer<PouchItem>) {
        (item.cast() + mem::offset_of!(PouchItem, equipped) as u64).write(
            memory, Box::new(false)
        ).unwrap();
        self.update(memory, &this);
        self.sync(memory, this);
    }

    // Open inventory
    pub fn pause(&self, memory: &Memory, this: Pointer<Self>) { self.traverse_list1(memory, this); }

    // Sync GameData
    pub fn sync(&mut self, memory: &mut Memory, this: Pointer<Self>) {
        self.update(memory, &this);
    }

    // Save file
    pub fn save(&self, memory: &Memory, this: Pointer<Self>) -> GameData {
        vec![]
    }

    // Load file
    pub fn load(&mut self, memory: &mut Memory, this: Pointer<Self>, file: GameData) {
        self.update(memory, &this);
    }

    // Break slots
    pub fn offset(&mut self, memory: &mut Memory, this: Pointer<Self>, num: u32) {
        let lists = self.item_lists;

        (this.cast() + mem::offset_of!(Self, item_lists.list1.count) as u64).write(memory, Box::new(
            (i32::from_le(lists.list1.count) - num as i32).to_le()
        )).unwrap();
        self.update(memory, &this);
        (this.cast() + mem::offset_of!(Self, item_lists.list2.count) as u64).write(memory, Box::new(
            (i32::from_le(lists.list2.count) + num as i32).to_le()
        )).unwrap();
        self.update(memory, &this);
    }
}

impl Updatable for PauseMenuDataMgr {}

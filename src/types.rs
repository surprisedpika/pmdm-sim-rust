use std::marker::PhantomData;
use std::mem;

use serde_json;

use crate::mem::*;
use crate::traits::*;

pub const NUM_POUCH_ITEMS_MAX: i32 = 420;
pub const NUM_INGREDIENTS_MAX: i32 = 5;

pub const NUM_POUCH_CATEGORIES: i32 = 7;
pub const NUM_TAB_MAX: i32 = 50;
pub const NUM_GRABBABLE_ITEMS: i32 = 5;

#[derive(Clone, Copy, Default, PartialEq)]
#[repr(i32)]
pub enum PouchItemType {
    Sword,
    Bow,
    Arrow,
    Shield,
    ArmorHead,
    ArmorUpper,
    ArmorLower,
    Material,
    Food,
    KeyItem,
    #[default] Invalid,
}

impl Updatable for PouchItemType {}

#[derive(Clone, Copy, Default, PartialEq)]
#[repr(i32)]
pub enum PouchCategory {
    Sword,
    Bow,
    Shield,
    Armor,
    Material,
    Food,
    KeyItem,
    #[default] Invalid,
}

impl Updatable for PouchCategory {}

#[derive(Clone, Copy, Default, PartialEq)]
#[repr(i32)]
pub enum ItemUse {
    WeaponSmallSword,
    WeaponLargeSword,
    WeaponSpear,
    WeaponBow,
    WeaponShield,
    ArmorHead,
    ArmorLower,
    ArmorUpper,
    Item,
    ImportantItem,
    CureItem,
    #[default] Invalid,
}

impl Updatable for ItemUse {}

#[derive(Clone, Copy, Default, PartialEq)]
#[repr(u32)]
pub enum WeaponModifier {
    #[default] None,
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

impl Updatable for WeaponModifier {}

#[derive(Clone, Copy, Default, PartialEq)]
#[repr(i32)]
pub enum CookEffectId {
    #[default] None = -1,
    LifeRecover = 1,
    LifeMaxUp,
    ResistHot = 4,
    ResistCold,
    ResistElectric,
    AttackUp = 10,
    DefenseUp,
    Quietness,
    MovingSpeed,
    GutsRecover,
    ExGutsMaxUp,
    Fireproof,
}

impl Updatable for CookEffectId {}

#[derive(Clone, Copy, Default)]
#[repr(C)]
pub struct ListNode {
    pub prev: Pointer<Self>,
    pub next: Pointer<Self>,
}

impl Updatable for ListNode {}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct FixedSafeStringVTable {
    pub super_dtor: Pointer,
    pub super_assure_termination_impl: Pointer,
    pub dtor: Pointer,
    pub assure_termination_impl: Pointer,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct FixedSafeString<const L: usize> {
    pub vptr: Pointer<FixedSafeStringVTable>,
    pub string_top: Pointer<i8>,
    pub buffer_size: i32,
    pub buffer: [u8; L],
}

impl<const L: usize> FixedSafeString<L> {
    pub fn assure_termination_impl(&mut self, memory: &mut Memory, this: Pointer<Self>) {
        **(self.vptr.cast::<Pointer>().to_ne() + mem::offset_of!(
            FixedSafeStringVTable, assure_termination_impl
        ) as u64).read(memory).unwrap();
        (self.string_top.to_ne() + i32::from_le(self.buffer_size) as u64 - 1).write(
            memory, Box::new(Default::default())
        ).unwrap();
        self.update(memory, &this);
    }

    pub fn is_equal(&mut self, memory: &mut Memory, this: Pointer<Self>, other: Self) -> bool {
        self.assure_termination_impl(memory, this);
        if self.string_top == other.string_top { return true; }

        for i in 0..=0x80000 {
            let current = *(self.string_top.to_ne() + i).read(memory).unwrap();

            if current != *(other.string_top.to_ne() + i).read(memory).unwrap() { return false; }
            if current == i8::default() { return true; }
        }

        false
    }

    pub fn is_equal_str(&mut self, memory: &mut Memory, this: Pointer<Self>, other: &str) -> bool {
        self.assure_termination_impl(memory, this);

        for i in 0..=0x80000 {
            let current = *(self.string_top.to_ne() + i).read(memory).unwrap();

            if current != other.as_bytes()[i as usize] as i8 { return false; }
            if current == i8::default() { return true; }
        }

        false
    }

    pub fn clear(&mut self, memory: &mut Memory) {
        self.string_top.to_ne().write(memory, Box::new(Default::default())).unwrap();
    }
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

impl<const L: usize> Updatable for FixedSafeString<L> {}

impl<const L: usize> Constructor for FixedSafeString<L> where [(); mem::size_of::<Self>()]: {
    fn ctor(&mut self, memory: &mut Memory, this: Pointer<Self>) {
        (this.cast() + mem::offset_of!(Self, string_top) as u64).write(memory, Box::new((
            this.cast::<i8>() + mem::offset_of!(Self, buffer) as u64
        ).to_le())).unwrap();
        self.update(memory, &this);

        (this.cast() + mem::offset_of!(Self, buffer_size) as u64).write(memory, Box::new((
            L as i32
        ).to_le())).unwrap();
        self.update(memory, &this);
        self.assure_termination_impl(memory, this);

        self.string_top.to_ne().write(memory, Box::new(Default::default())).unwrap();
        self.update(memory, &this);
    }
}

#[derive(Clone, Copy, Default)]
#[repr(C)]
pub struct CookData {
    pub health_recover: i32,
    pub effect_duration: i32,
    pub sell_price: i32,
    pub effect_id: f32,
    pub effect_level: f32,
}

impl Updatable for CookData {}

#[derive(Clone, Copy, Default)]
#[repr(C)]
pub struct WeaponData {
    pub modifier_value: u32,
    unused: u32,
    pub modifier: u32,
}

impl Updatable for WeaponData {}

#[derive(Clone, Copy)]
#[repr(C)]
pub union Data {
    pub cook: CookData,
    pub weapon: WeaponData,
}

impl Default for Data { fn default() -> Self { Self { cook: Default::default() } } }

impl Updatable for Data {}

#[derive(Clone, Copy, Default)]
#[repr(C)]
pub struct FreeListNode {
    pub next_free: Pointer<Self>,
}

impl Updatable for FreeListNode {}

#[derive(Clone, Copy, Default)]
#[repr(C)]
pub struct FreeList {
    pub free: Pointer<FreeListNode>,
    pub work: Pointer,
}

impl Updatable for FreeList {}

#[derive(Clone, Copy, Default)]
#[repr(C)]
pub struct ObjArrayWorkNode<T> {
    pub item: T,
    pub pointer: Pointer<T>,
}

impl<T> Updatable for ObjArrayWorkNode<T> {}

#[derive(Clone, Copy)]
#[repr(C)]
pub union ObjArrayNode<T> where T: Copy {
    pub next_node: Pointer<Self>,
    pub item: T,
}

impl<T> Default for ObjArrayNode<T> where T: Copy {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct FixedObjArray<T, const N: i32> where [(); N as usize]: {
    pub ptr_num: i32,
    pub ptr_num_max: i32,
    pub ptrs: Pointer<Pointer<T>>,
    pub free_list: FreeList,
    pub work: [ObjArrayWorkNode<T>; N as usize],
}

impl<T, const N: i32> Updatable for FixedObjArray<T, N> where [(); N as usize]: {}

impl<T, const N: i32> Constructor for FixedObjArray<T, N> where [(); N as usize]:, T: Copy {
    fn ctor(&mut self, memory: &mut Memory, this: Pointer<Self>) {
        let element_size = mem::size_of::<ObjArrayNode<T>>() as i32;

        let idx_multiplier = element_size / 0x8;
        (this.cast() + mem::offset_of!(Self, work) as u64).write(memory, unsafe { Box::from_raw(
            vec![u64::default(); (N * idx_multiplier) as usize].as_mut_ptr()
        ) }).unwrap();
        self.update(memory, &this);
        let ptrs = this.cast::<Pointer>() + mem::offset_of!(Self, work) as u64;

        (this.cast() + mem::offset_of!(Self, work) as u64).write(memory, Box::new(
            FreeListNode::default()
        )).unwrap();
        self.update(memory, &this);
        (this.cast() + mem::offset_of!(Self, free_list.free) as u64).write(memory, Box::new((
            this.cast::<FreeListNode>() + mem::offset_of!(Self, work) as u64
        ).to_le())).unwrap();
        self.update(memory, &this);

        for i in 0..N - 1 {
            let next_free = ptrs.cast::<FreeListNode>() + ((i + 1) * idx_multiplier) as u64;
            next_free.write(memory, Box::new(Default::default())).unwrap();
            self.update(memory, &this);
            (ptrs.cast() + (i * idx_multiplier) as u64).write(memory, Box::new(FreeListNode {
                next_free: next_free.to_le()
            })).unwrap();
            self.update(memory, &this);
        }

        (ptrs.cast() + ((N - 1) * idx_multiplier) as u64).write(memory, Box::new(
            FreeListNode::default()
        )).unwrap();
        self.update(memory, &this);

        (this.cast() + mem::offset_of!(Self, free_list.work) as u64).write(memory, Box::new((
            this.cast::<u8>() + mem::offset_of!(Self, work) as u64
        ).to_le())).unwrap();
        self.update(memory, &this);

        (this.cast() + mem::offset_of!(Self, ptrs) as u64).write(memory, Box::new((
            this.cast::<Pointer<T>>() + mem::offset_of!(Self, work) as u64 + element_size as u64
        ).to_le())).unwrap();
        self.update(memory, &this);
        (this.cast() + mem::offset_of!(Self, ptr_num) as u64).write(memory, Box::new(
            i32::default()
        )).unwrap();
        self.update(memory, &this);
        (this.cast() + mem::offset_of!(Self, ptr_num_max) as u64).write(memory, Box::new(
            N.to_le()
        )).unwrap();
        self.update(memory, &this);
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct PouchItem {
    pub vptr: Pointer<Pointer>,
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

impl Updatable for PouchItem {}

impl Constructor for PouchItem {
    fn ctor(&mut self, memory: &mut Memory, this: Pointer<Self>) {
        (this.cast() + mem::offset_of!(Self, list_node) as u64).write(memory, Box::new(
            ListNode::default()
        )).unwrap();
        self.update(memory, &this);
        (this.cast() + mem::offset_of!(Self, item_type) as u64).write(memory, Box::new((
            PouchItemType::default() as i32
        ).to_le())).unwrap();
        self.update(memory, &this);
        (this.cast() + mem::offset_of!(Self, item_use) as u64).write(memory, Box::new((
            ItemUse::default() as i32
        ).to_le())).unwrap();
        self.update(memory, &this);
        (this.cast() + mem::offset_of!(Self, value) as u64).write(memory, Box::new(
            i32::default()
        )).unwrap();
        self.update(memory, &this);
        (this.cast() + mem::offset_of!(Self, equipped) as u64).write(memory, Box::new(
            bool::default()
        )).unwrap();
        self.update(memory, &this);
        (this.cast() + mem::offset_of!(Self, in_inventory) as u64).write(
            memory, Box::new(true)
        ).unwrap();
        self.update(memory, &this);
        self.name.ctor(memory, this.cast() + mem::offset_of!(Self, name) as u64);
        (this.cast() + mem::offset_of!(Self, data) as u64).write(memory, Box::new(
            Data::default()
        )).unwrap();
        self.update(memory, &this);
        self.ingredients.ctor(memory, this.cast() + mem::offset_of!(Self, ingredients) as u64);
        (this.cast() + mem::offset_of!(Self, data.cook.effect_id) as u64).write(memory, Box::new((
            CookEffectId::default() as i32 as f32
        ).to_le_bytes())).unwrap();
        self.update(memory, &this);
        (this.cast() + mem::offset_of!(Self, data.cook.effect_level) as u64).write(
            memory, Box::new(f32::default().to_le_bytes())
        ).unwrap();
        self.update(memory, &this);

        for _ in 0..NUM_INGREDIENTS_MAX {
            let ptr = self.ingredients.free_list.free.cast::<FixedSafeString<64>>().to_ne();
            if ptr != Pointer::NULLPTR {
                (this.cast() + mem::offset_of!(Self, ingredients.free_list.free) as u64).write(
                    memory, ptr.cast::<Pointer<FreeListNode>>().read(memory).unwrap()
                ).unwrap();
                self.update(memory, &this);
            }

            ptr.read(memory).unwrap().ctor(memory, ptr);
            self.update(memory, &this);

            (this.cast() + mem::offset_of!(Self, ingredients.ptrs) as u64 + (i32::from_le(
                self.ingredients.ptr_num
            ) * 0x8) as u64).write(memory, Box::new(ptr.to_le())).unwrap();
            self.update(memory, &this);
            (this.cast() + mem::offset_of!(Self, ingredients.ptr_num) as u64).write(
                memory, Box::new((i32::from_le(self.ingredients.ptr_num) + 1).to_le())
            ).unwrap();
            self.update(memory, &this);
        }
    }
}

pub fn translate_name(actor_name: &str, lang_data: serde_json::Value) -> Option<String> {
    lang_data.get(actor_name)?.as_str().map(String::from)
}

#[derive(Clone, Copy, Default)]
#[repr(C)]
pub struct MutexType {
    pub state: u8,
    pub is_recursive: bool,
    pub lock_level: i32,
    pub nest_count: i32,
    pub owner_thread: Pointer,
    pub mutex: i32,
}

impl Updatable for MutexType {}

#[derive(Clone, Copy, Default)]
#[repr(C)]
pub struct CriticalSection {
    // IDisposer
    pub vptr: Pointer<Pointer>,
    pub disposer_heap: Pointer,
    pub list_node: ListNode,

    // CriticalSection
    pub critical_section_inner: MutexType,
}

impl Updatable for CriticalSection {}

#[derive(Clone, Copy, Default)]
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
    fn obj_to_list_node(&self, obj: Pointer<T>) -> Pointer<ListNode> {
        (obj + self.offset as u64).cast()
    }

    fn list_node_to_obj(&self, node: Pointer<ListNode>) -> Pointer<T> {
        (node - self.offset as u64).cast()
    }

    fn list_node_to_obj_with_null_check(&self, node: Pointer<ListNode>) -> Pointer<T> {
        if node == Pointer::NULLPTR { Pointer::NULLPTR } else { self.list_node_to_obj(node) }
    }

    pub fn erase(&mut self, memory: &mut Memory, this: Pointer<Self>, item: Pointer<T>) where [
        (); mem::size_of::<Self>()
    ]: {
        let node_ptr = (item + i32::from_le(self.offset) as u64).cast::<ListNode>();
        let node = node_ptr.read(memory).unwrap();

        if node.prev != Pointer::NULLPTR {
            (node.prev.to_ne() + mem::offset_of!(ListNode, next) as u64).write(
                memory, node.next.to_ne().read(memory).unwrap()
            ).unwrap();
            self.update(memory, &this);
        }
        if node.next != Pointer::NULLPTR {
            (node.next.to_ne() + mem::offset_of!(ListNode, prev) as u64).write(
                memory, node.prev.to_ne().read(memory).unwrap()
            ).unwrap();
            self.update(memory, &this);
        }

        (node_ptr.cast() + mem::offset_of!(ListNode, prev) as u64).write(memory, Box::new(
            ListNode::default()
        )).unwrap();
        self.update(memory, &this);

        (this.cast() + mem::offset_of!(Self, count) as u64).write(memory, Box::new((i32::from_le(
            self.count
        ) - 1).to_le())).unwrap();
        self.update(memory, &this);
    }

    pub fn prev(&self, memory: &Memory, this: Pointer<Self>, obj: Pointer<T>) -> Pointer<T> {
        let prev_node = self.obj_to_list_node(obj).read(memory).unwrap().prev.to_ne();
        if prev_node == this.cast() + mem::offset_of!(Self, start_end) as u64 { Pointer::NULLPTR }
        else { self.list_node_to_obj(prev_node) }
    }

    pub fn next(&self, memory: &Memory, this: Pointer<Self>, obj: Pointer<T>) -> Pointer<T> {
        let next_node = self.obj_to_list_node(obj).read(memory).unwrap().next.to_ne();
        if next_node == this.cast() + mem::offset_of!(Self, start_end) as u64 { Pointer::NULLPTR }
        else { self.list_node_to_obj(next_node) }
    }

    pub fn nth(&self, memory: &Memory, n: i32) -> Pointer<T> {
        if i32::from_le(self.count) as u32 <= n as u32 { return Pointer::new(0u64); }
        let mut node = self.start_end.next.to_ne();
        for _ in 0..n { node = node.read(memory).unwrap().next.to_ne(); }
        self.list_node_to_obj_with_null_check(node)
    }

    pub fn sort(&mut self, memory: &mut Memory, this: Pointer<Self>, cmp: fn(
        &Memory, Pointer<T>, Pointer<T>
    ) -> i32) {}
}

impl<T> Updatable for OffsetList<T> {}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SafeArray<T, const N: i32> where [(); N as usize]: {
    pub buffer: [T; N as usize],
}

impl<T: Copy + Default, const N: i32> Default for SafeArray<T, N> where [(); N as usize]: {
    fn default() -> Self { Self { buffer: [Default::default(); N as usize] } }
}

impl<T, const N: i32> Updatable for SafeArray<T, N> where [(); N as usize]: {}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Lists {
    pub list1: OffsetList<PouchItem>,
    pub list2: OffsetList<PouchItem>,
    pub buffer: SafeArray<PouchItem, NUM_POUCH_ITEMS_MAX>,
}

impl Updatable for Lists {}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct GrabbedItemInfo {
    pub item: Pointer<PouchItem>,
    _8: bool,
    _9: bool,
}

impl Updatable for GrabbedItemInfo {}

#[derive(Clone, Copy, Default)]
#[repr(C)]
pub struct TypedBitFlag<Enum> {
    bits: Enum,
}

impl<Enum> Updatable for TypedBitFlag<Enum> {}

#[derive(Clone, Copy, Default)]
#[repr(C)]
pub struct WeaponModifierInfo {
    flags: TypedBitFlag<WeaponModifier>,
    value: i32,
}

impl Updatable for WeaponModifierInfo {}

#[derive(Clone)]
pub struct GameDataItem {
    name: String,
    equipped: bool,
    value: i32,
    data: Data,
}

pub type GameData = Vec<GameDataItem>;

use pokered_data::items::ItemId;
use serde::{Deserialize, Serialize};

pub const MAX_ITEM_QUANTITY: u8 = 99;
pub const BAG_ITEM_CAPACITY: usize = 20;
pub const PC_ITEM_CAPACITY: usize = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InventoryError {
    InventoryFull,
    ItemNotFound,
    NotEnoughItems,
    IndexOutOfBounds,
    SameIndex,
    ZeroQuantity,
    QuantityOverflow,
}

/// Gen1 item inventory: count + (ItemId, quantity) pairs.
/// Bag = capacity 20, PC = capacity 50, max 99 per slot.
/// Overflow at 99 spills into a new slot (matching ASM `AddItemToInventory_`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Inventory {
    items: Vec<(ItemId, u8)>,
    capacity: usize,
}

impl Inventory {
    pub fn new(capacity: usize) -> Self {
        Self {
            items: Vec::new(),
            capacity,
        }
    }

    pub fn new_bag() -> Self {
        Self::new(BAG_ITEM_CAPACITY)
    }

    pub fn new_pc() -> Self {
        Self::new(PC_ITEM_CAPACITY)
    }

    pub fn count(&self) -> usize {
        self.items.len()
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn is_full(&self) -> bool {
        self.items.len() >= self.capacity
    }

    pub fn get(&self, index: usize) -> Option<(ItemId, u8)> {
        self.items.get(index).copied()
    }

    pub fn items(&self) -> &[(ItemId, u8)] {
        &self.items
    }

    pub fn add_item(&mut self, item: ItemId, quantity: u8) -> Result<(), InventoryError> {
        if quantity == 0 {
            return Err(InventoryError::ZeroQuantity);
        }

        let mut remaining = quantity;

        for slot in self.items.iter_mut() {
            if slot.0 == item && remaining > 0 {
                let space = MAX_ITEM_QUANTITY.saturating_sub(slot.1);
                if space > 0 {
                    let add = remaining.min(space);
                    slot.1 += add;
                    remaining -= add;
                }
            }
            if remaining == 0 {
                return Ok(());
            }
        }

        while remaining > 0 {
            if self.is_full() {
                return Err(InventoryError::InventoryFull);
            }
            let add = remaining.min(MAX_ITEM_QUANTITY);
            self.items.push((item, add));
            remaining -= add;
        }

        Ok(())
    }

    pub fn remove_item_at(&mut self, index: usize, quantity: u8) -> Result<(), InventoryError> {
        if quantity == 0 {
            return Err(InventoryError::ZeroQuantity);
        }
        if index >= self.items.len() {
            return Err(InventoryError::IndexOutOfBounds);
        }
        let current_qty = self.items[index].1;
        if quantity > current_qty {
            return Err(InventoryError::NotEnoughItems);
        }
        let new_qty = current_qty - quantity;
        if new_qty == 0 {
            self.items.remove(index);
        } else {
            self.items[index].1 = new_qty;
        }
        Ok(())
    }

    pub fn remove_item(&mut self, item: ItemId, quantity: u8) -> Result<(), InventoryError> {
        if quantity == 0 {
            return Err(InventoryError::ZeroQuantity);
        }
        let index = self
            .items
            .iter()
            .position(|&(id, _)| id == item)
            .ok_or(InventoryError::ItemNotFound)?;
        self.remove_item_at(index, quantity)
    }

    pub fn toss_item(&mut self, index: usize, quantity: u8) -> Result<(), InventoryError> {
        self.remove_item_at(index, quantity)
    }

    pub fn swap(&mut self, a: usize, b: usize) -> Result<(), InventoryError> {
        if a == b {
            return Err(InventoryError::SameIndex);
        }
        let len = self.items.len();
        if a >= len || b >= len {
            return Err(InventoryError::IndexOutOfBounds);
        }
        self.items.swap(a, b);
        Ok(())
    }

    pub fn has_item(&self, item: ItemId, quantity: u8) -> bool {
        let total: u16 = self
            .items
            .iter()
            .filter(|&&(id, _)| id == item)
            .map(|&(_, qty)| qty as u16)
            .sum();
        total >= quantity as u16
    }

    pub fn item_quantity(&self, item: ItemId) -> u16 {
        self.items
            .iter()
            .filter(|&&(id, _)| id == item)
            .map(|&(_, qty)| qty as u16)
            .sum()
    }

    pub fn find_item(&self, item: ItemId) -> Option<usize> {
        self.items.iter().position(|&(id, _)| id == item)
    }

    pub fn use_item(&mut self, index: usize) -> Result<ItemId, InventoryError> {
        if index >= self.items.len() {
            return Err(InventoryError::IndexOutOfBounds);
        }
        let item_id = self.items[index].0;
        self.remove_item_at(index, 1)?;
        Ok(item_id)
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}

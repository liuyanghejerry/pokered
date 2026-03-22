use crate::items::inventory::{Inventory, InventoryError};
use crate::main_menu::MenuInput;
use pokered_data::item_data::get_item_data;
use pokered_data::items::ItemId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShopMenuChoice {
    Buy,
    Sell,
    Quit,
}

#[derive(Debug, Clone)]
pub struct ShopMenuState {
    cursor: usize,
}

impl ShopMenuState {
    const ITEMS: [ShopMenuChoice; 3] = [
        ShopMenuChoice::Buy,
        ShopMenuChoice::Sell,
        ShopMenuChoice::Quit,
    ];

    pub fn new() -> Self {
        Self { cursor: 0 }
    }

    pub fn update_frame(&mut self, input: MenuInput) -> Option<ShopMenuChoice> {
        if input.b {
            return Some(ShopMenuChoice::Quit);
        }
        if input.up {
            self.cursor_up();
        } else if input.down {
            self.cursor_down();
        }
        if input.a {
            return Some(Self::ITEMS[self.cursor]);
        }
        None
    }

    fn cursor_up(&mut self) {
        if self.cursor == 0 {
            self.cursor = Self::ITEMS.len() - 1;
        } else {
            self.cursor -= 1;
        }
    }

    fn cursor_down(&mut self) {
        self.cursor += 1;
        if self.cursor >= Self::ITEMS.len() {
            self.cursor = 0;
        }
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn current_choice(&self) -> ShopMenuChoice {
        Self::ITEMS[self.cursor]
    }
}

impl Default for ShopMenuState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuyResult {
    Success { total_cost: u32 },
    NotEnoughMoney,
    BagFull,
    InvalidItem,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SellResult {
    Success { total_value: u32 },
    Unsellable,
    NotInBag,
    InvalidItem,
}

pub fn buy_price(item: ItemId, quantity: u8) -> Option<u32> {
    let data = get_item_data(item)?;
    Some(data.price as u32 * quantity as u32)
}

pub fn sell_price(item: ItemId, quantity: u8) -> Option<u32> {
    let data = get_item_data(item)?;
    Some((data.price as u32 / 2) * quantity as u32)
}

pub fn can_sell(item: ItemId) -> bool {
    if let Some(data) = get_item_data(item) {
        !data.is_key_item && data.price > 0
    } else {
        false
    }
}

pub fn try_buy(item: ItemId, quantity: u8, money: &mut u32, bag: &mut Inventory) -> BuyResult {
    let cost = match buy_price(item, quantity) {
        Some(c) => c,
        None => return BuyResult::InvalidItem,
    };
    if *money < cost {
        return BuyResult::NotEnoughMoney;
    }
    if bag.add_item(item, quantity).is_err() {
        return BuyResult::BagFull;
    }
    *money -= cost;
    BuyResult::Success { total_cost: cost }
}

pub fn try_sell(
    bag_index: usize,
    quantity: u8,
    money: &mut u32,
    bag: &mut Inventory,
) -> SellResult {
    let (item, owned) = match bag.get(bag_index) {
        Some(entry) => entry,
        None => return SellResult::NotInBag,
    };
    if !can_sell(item) {
        return SellResult::Unsellable;
    }
    if quantity > owned {
        return SellResult::NotInBag;
    }
    let value = match sell_price(item, quantity) {
        Some(v) => v,
        None => return SellResult::InvalidItem,
    };
    match bag.remove_item_at(bag_index, quantity) {
        Ok(()) => {}
        Err(InventoryError::IndexOutOfBounds) | Err(InventoryError::NotEnoughItems) => {
            return SellResult::NotInBag;
        }
        Err(_) => return SellResult::NotInBag,
    }
    *money = money.saturating_add(value);
    SellResult::Success { total_value: value }
}

#[derive(Debug, Clone)]
pub struct ShopInventory {
    items: Vec<ItemId>,
}

impl ShopInventory {
    pub fn new(items: Vec<ItemId>) -> Self {
        Self { items }
    }

    pub fn items(&self) -> &[ItemId] {
        &self.items
    }

    pub fn get(&self, index: usize) -> Option<ItemId> {
        self.items.get(index).copied()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

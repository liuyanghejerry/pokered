use super::inventory::Inventory;
use super::shop::*;
use crate::main_menu::MenuInput;
use pokered_data::items::ItemId;

fn menu_up() -> MenuInput {
    MenuInput {
        up: true,
        ..MenuInput::none()
    }
}

fn menu_down() -> MenuInput {
    MenuInput {
        down: true,
        ..MenuInput::none()
    }
}

fn menu_a() -> MenuInput {
    MenuInput {
        a: true,
        ..MenuInput::none()
    }
}

fn menu_b() -> MenuInput {
    MenuInput {
        b: true,
        ..MenuInput::none()
    }
}

#[test]
fn shop_menu_initial_state() {
    let menu = ShopMenuState::new();
    assert_eq!(menu.cursor(), 0);
    assert_eq!(menu.current_choice(), ShopMenuChoice::Buy);
}

#[test]
fn shop_menu_navigate_all_choices() {
    let expected = [
        ShopMenuChoice::Buy,
        ShopMenuChoice::Sell,
        ShopMenuChoice::Quit,
    ];
    for (i, choice) in expected.iter().enumerate() {
        let mut menu = ShopMenuState::new();
        for _ in 0..i {
            menu.update_frame(menu_down());
        }
        let result = menu.update_frame(menu_a());
        assert_eq!(result, Some(*choice), "index={i}");
    }
}

#[test]
fn shop_menu_b_quits() {
    let mut menu = ShopMenuState::new();
    let result = menu.update_frame(menu_b());
    assert_eq!(result, Some(ShopMenuChoice::Quit));
}

#[test]
fn shop_menu_cursor_wraps() {
    let mut menu = ShopMenuState::new();
    menu.update_frame(menu_up());
    assert_eq!(menu.cursor(), 2);
    menu.update_frame(menu_down());
    assert_eq!(menu.cursor(), 0);
}

#[test]
fn shop_menu_default() {
    let menu = ShopMenuState::default();
    assert_eq!(menu.cursor(), 0);
}

#[test]
fn buy_price_for_potion() {
    let price = buy_price(ItemId::Potion, 1).unwrap();
    assert_eq!(price, 300);
}

#[test]
fn buy_price_quantity_multiplied() {
    let price = buy_price(ItemId::Potion, 5).unwrap();
    assert_eq!(price, 1500);
}

#[test]
fn sell_price_is_half_buy() {
    let buy = buy_price(ItemId::Potion, 1).unwrap();
    let sell = sell_price(ItemId::Potion, 1).unwrap();
    assert_eq!(sell, buy / 2);
}

#[test]
fn can_sell_regular_item() {
    assert!(can_sell(ItemId::Potion));
    assert!(can_sell(ItemId::UltraBall));
}

#[test]
fn cannot_sell_key_item() {
    assert!(!can_sell(ItemId::OldAmber));
}

#[test]
fn try_buy_success() {
    let mut money: u32 = 1000;
    let mut bag = Inventory::new_bag();
    let result = try_buy(ItemId::Potion, 2, &mut money, &mut bag);
    assert_eq!(result, BuyResult::Success { total_cost: 600 });
    assert_eq!(money, 400);
    assert_eq!(bag.item_quantity(ItemId::Potion), 2);
}

#[test]
fn try_buy_not_enough_money() {
    let mut money: u32 = 100;
    let mut bag = Inventory::new_bag();
    let result = try_buy(ItemId::Potion, 1, &mut money, &mut bag);
    assert_eq!(result, BuyResult::NotEnoughMoney);
    assert_eq!(money, 100);
}

#[test]
fn try_buy_bag_full() {
    let mut money: u32 = 999999;
    let mut bag = Inventory::new_bag();
    let filler_items = [
        ItemId::MasterBall,
        ItemId::UltraBall,
        ItemId::GreatBall,
        ItemId::PokeBall,
        ItemId::TownMap,
        ItemId::Bicycle,
        ItemId::Antidote,
        ItemId::BurnHeal,
        ItemId::IceHeal,
        ItemId::Awakening,
        ItemId::ParlyzHeal,
        ItemId::FullRestore,
        ItemId::MaxPotion,
        ItemId::HyperPotion,
        ItemId::SuperPotion,
        ItemId::Potion,
        ItemId::EscapeRope,
        ItemId::Repel,
        ItemId::FireStone,
        ItemId::ThunderStone,
    ];
    for item in &filler_items {
        bag.add_item(*item, 1).unwrap();
    }
    let result = try_buy(ItemId::MoonStone, 1, &mut money, &mut bag);
    assert_eq!(result, BuyResult::BagFull);
    assert_eq!(money, 999999);
}

#[test]
fn try_sell_success() {
    let mut money: u32 = 0;
    let mut bag = Inventory::new_bag();
    bag.add_item(ItemId::Potion, 5).unwrap();
    let result = try_sell(0, 3, &mut money, &mut bag);
    assert_eq!(result, SellResult::Success { total_value: 450 });
    assert_eq!(money, 450);
    assert_eq!(bag.item_quantity(ItemId::Potion), 2);
}

#[test]
fn try_sell_unsellable_key_item() {
    let mut money: u32 = 0;
    let mut bag = Inventory::new_bag();
    bag.add_item(ItemId::OldAmber, 1).unwrap();
    let result = try_sell(0, 1, &mut money, &mut bag);
    assert_eq!(result, SellResult::Unsellable);
    assert_eq!(money, 0);
    assert_eq!(bag.item_quantity(ItemId::OldAmber), 1);
}

#[test]
fn try_sell_not_in_bag() {
    let mut money: u32 = 0;
    let mut bag = Inventory::new_bag();
    let result = try_sell(0, 1, &mut money, &mut bag);
    assert_eq!(result, SellResult::NotInBag);
}

#[test]
fn try_sell_quantity_exceeds_owned() {
    let mut money: u32 = 0;
    let mut bag = Inventory::new_bag();
    bag.add_item(ItemId::Potion, 2).unwrap();
    let result = try_sell(0, 5, &mut money, &mut bag);
    assert_eq!(result, SellResult::NotInBag);
    assert_eq!(bag.item_quantity(ItemId::Potion), 2);
}

#[test]
fn shop_inventory_basic() {
    let shop = ShopInventory::new(vec![ItemId::PokeBall, ItemId::Potion, ItemId::Antidote]);
    assert_eq!(shop.len(), 3);
    assert!(!shop.is_empty());
    assert_eq!(shop.get(0), Some(ItemId::PokeBall));
    assert_eq!(shop.get(1), Some(ItemId::Potion));
    assert_eq!(shop.get(3), None);
}

#[test]
fn shop_inventory_empty() {
    let shop = ShopInventory::new(vec![]);
    assert!(shop.is_empty());
    assert_eq!(shop.len(), 0);
}

pub use pokered_data as data;

pub mod battle;
pub mod game_state;
pub mod items;
pub mod main_menu;
pub mod overworld;
pub mod pokemon;
pub mod slots;
pub mod text;
pub mod title_screen;

#[cfg(test)]
mod title_screen_tests;

#[cfg(test)]
mod main_menu_tests;

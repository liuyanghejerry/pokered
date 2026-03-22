pub use pokered_data as data;

pub mod battle;
pub mod game_state;
pub mod items;
pub mod main_menu;
pub mod naming_screen;
pub mod options_menu;
pub mod overworld;
pub mod pokemon;
pub mod save_menu;
pub mod slots;
pub mod start_menu;
pub mod text;
pub mod title_screen;

#[cfg(test)]
mod main_menu_tests;

#[cfg(test)]
mod naming_screen_tests;

#[cfg(test)]
mod options_menu_tests;

#[cfg(test)]
mod save_menu_tests;

#[cfg(test)]
mod start_menu_tests;

#[cfg(test)]
mod title_screen_tests;

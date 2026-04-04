//! Pokémon Red/Blue - App Library
//!
//! This crate provides the core game logic shared between native and web builds.

pub mod battle_config;
pub mod battle_sfx;
pub mod game;
pub mod render;

#[cfg(not(target_arch = "wasm32"))]
pub mod audio;

#[cfg(not(target_arch = "wasm32"))]
pub mod direct_battle;

#[cfg(not(target_arch = "wasm32"))]
pub mod tools;

#[cfg(not(target_arch = "wasm32"))]
pub mod cli;

pub use game::PokemonGame;
pub use render::BattleVisualEffects;

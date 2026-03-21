//! NPC interaction system — talk, trainer sight, item pickup.
//!
//! Implements M4.4 (NPC移动和交互) interaction logic ported from:
//! - engine/overworld/emotion_bubbles.asm (trainer notice animation)
//! - engine/battle/trainer_ai.asm (TrainerWalkUpToPlayer)
//! - home/overworld.asm (HandleNPCInteraction)

use super::npc_movement::{npc_in_front_of_player, NpcRuntimeState};
use super::player_movement::direction_delta;
use super::Direction;

/// Result of an NPC interaction attempt (pressing A near an NPC).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InteractionResult {
    /// No NPC in front of the player.
    NoTarget,
    /// Regular NPC dialog — show text_id from the map's text table.
    Talk { npc_index: u8, text_id: u8 },
    /// Trainer battle trigger.
    TrainerBattle {
        npc_index: u8,
        trainer_class: u8,
        trainer_set: u8,
    },
    /// Item pickup (item ball NPC).
    ItemPickup { npc_index: u8, item_id: u8 },
    /// NPC already defeated/collected — shows post-defeat dialog.
    AlreadyDefeated { npc_index: u8, text_id: u8 },
}

/// Attempt to interact with the NPC the player is facing.
///
/// In the original game, pressing A checks the tile in front of
/// the player for an NPC sprite, then dispatches based on the NPC's
/// type (regular text, trainer, or item ball).
pub fn try_interact(
    npcs: &[NpcRuntimeState],
    player_x: u16,
    player_y: u16,
    facing: Direction,
) -> InteractionResult {
    let npc = match npc_in_front_of_player(npcs, player_x, player_y, facing) {
        Some(n) => n,
        None => return InteractionResult::NoTarget,
    };

    if npc.defeated {
        return InteractionResult::AlreadyDefeated {
            npc_index: npc.npc_index,
            text_id: npc.text_id,
        };
    }

    if npc.is_trainer {
        return InteractionResult::TrainerBattle {
            npc_index: npc.npc_index,
            trainer_class: npc.trainer_class,
            trainer_set: npc.trainer_set,
        };
    }

    if npc.item_id != 0 {
        return InteractionResult::ItemPickup {
            npc_index: npc.npc_index,
            item_id: npc.item_id,
        };
    }

    InteractionResult::Talk {
        npc_index: npc.npc_index,
        text_id: npc.text_id,
    }
}

/// Result of a trainer line-of-sight check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrainerSighting {
    pub npc_index: u8,
    pub trainer_class: u8,
    pub trainer_set: u8,
    pub distance: u8,
}

/// Check if any trainer NPC can see the player.
///
/// In the original game (engine/overworld/map_sprites.asm, CheckFightingMapTrainers),
/// each trainer sprite checks if the player is in their line of sight.
/// The trainer's range determines how far they can see in their facing direction.
/// Range of 0 means the trainer only battles when talked to.
pub fn check_trainer_line_of_sight(
    npcs: &[NpcRuntimeState],
    player_x: u16,
    player_y: u16,
) -> Option<TrainerSighting> {
    for npc in npcs {
        if !npc.visible || !npc.is_trainer || npc.defeated || npc.range == 0 {
            continue;
        }

        let (dx, dy) = direction_delta(npc.facing);
        let mut check_x = npc.x as i32;
        let mut check_y = npc.y as i32;

        for dist in 1..=npc.range {
            check_x += dx as i32;
            check_y += dy as i32;

            if check_x < 0 || check_y < 0 {
                break;
            }

            if check_x as u16 == player_x && check_y as u16 == player_y {
                return Some(TrainerSighting {
                    npc_index: npc.npc_index,
                    trainer_class: npc.trainer_class,
                    trainer_set: npc.trainer_set,
                    distance: dist,
                });
            }
        }
    }
    None
}

/// Process an item pickup: mark the NPC as defeated (collected) and
/// return the item_id. The caller should add the item to the player's
/// inventory.
pub fn collect_item(npcs: &mut [NpcRuntimeState], npc_index: u8) -> Option<u8> {
    let npc = npcs.iter_mut().find(|n| n.npc_index == npc_index)?;
    if npc.item_id == 0 || npc.defeated {
        return None;
    }
    npc.defeated = true;
    npc.visible = false;
    Some(npc.item_id)
}

/// Mark a trainer NPC as defeated after losing a battle.
pub fn mark_trainer_defeated(npcs: &mut [NpcRuntimeState], npc_index: u8) {
    if let Some(npc) = npcs.iter_mut().find(|n| n.npc_index == npc_index) {
        npc.defeated = true;
    }
}

/// Check if a sign is at the tile the player is facing.
/// Signs are interacted with by pressing A while facing them.
pub fn check_sign_interaction(
    signs: &[(u8, u8, u8)],
    player_x: u16,
    player_y: u16,
    facing: Direction,
) -> Option<u8> {
    let (dx, dy) = direction_delta(facing);
    let target_x = (player_x as i32 + dx as i32) as u8;
    let target_y = (player_y as i32 + dy as i32) as u8;

    signs
        .iter()
        .find(|&&(sx, sy, _)| sx == target_x && sy == target_y)
        .map(|&(_, _, text_id)| text_id)
}

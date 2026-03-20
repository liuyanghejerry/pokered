use crate::battle::state::Pokemon;
use pokered_data::species::Species;
use serde::{Deserialize, Serialize};

pub const PARTY_LENGTH: usize = 6;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartyError {
    PartyFull,
    PartyEmpty,
    IndexOutOfBounds,
    CannotRemoveLast,
    SameIndex,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Party {
    mons: Vec<Pokemon>,
}

impl Party {
    pub fn new() -> Self {
        Self { mons: Vec::new() }
    }

    pub fn from_pokemon(pokemon: Vec<Pokemon>) -> Result<Self, PartyError> {
        if pokemon.len() > PARTY_LENGTH {
            return Err(PartyError::PartyFull);
        }
        Ok(Self { mons: pokemon })
    }

    pub fn count(&self) -> usize {
        self.mons.len()
    }

    pub fn is_full(&self) -> bool {
        self.mons.len() >= PARTY_LENGTH
    }

    pub fn is_empty(&self) -> bool {
        self.mons.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<&Pokemon> {
        self.mons.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Pokemon> {
        self.mons.get_mut(index)
    }

    pub fn leader(&self) -> Option<&Pokemon> {
        self.mons.first()
    }

    pub fn leader_level(&self) -> u8 {
        self.mons.first().map_or(0, |p| p.level)
    }

    pub fn add(&mut self, pokemon: Pokemon) -> Result<usize, PartyError> {
        if self.is_full() {
            return Err(PartyError::PartyFull);
        }
        let index = self.mons.len();
        self.mons.push(pokemon);
        Ok(index)
    }

    pub fn remove(&mut self, index: usize) -> Result<Pokemon, PartyError> {
        if index >= self.mons.len() {
            return Err(PartyError::IndexOutOfBounds);
        }
        if self.mons.len() <= 1 {
            return Err(PartyError::CannotRemoveLast);
        }
        Ok(self.mons.remove(index))
    }

    pub fn swap(&mut self, a: usize, b: usize) -> Result<(), PartyError> {
        if a == b {
            return Err(PartyError::SameIndex);
        }
        let len = self.mons.len();
        if a >= len || b >= len {
            return Err(PartyError::IndexOutOfBounds);
        }
        self.mons.swap(a, b);
        Ok(())
    }

    pub fn species_list(&self) -> Vec<Species> {
        self.mons.iter().map(|p| p.species).collect()
    }

    pub fn find_species(&self, species: Species) -> Option<usize> {
        self.mons.iter().position(|p| p.species == species)
    }

    pub fn alive_count(&self) -> usize {
        self.mons.iter().filter(|p| p.hp > 0).count()
    }

    pub fn all_fainted(&self) -> bool {
        !self.mons.is_empty() && self.mons.iter().all(|p| p.hp == 0)
    }

    pub fn first_alive_index(&self) -> Option<usize> {
        self.mons.iter().position(|p| p.hp > 0)
    }

    pub fn heal_all(&mut self) {
        for mon in &mut self.mons {
            mon.hp = mon.max_hp;
            mon.status = crate::battle::state::StatusCondition::None;
        }
    }

    pub fn to_vec(&self) -> Vec<Pokemon> {
        self.mons.clone()
    }

    pub fn into_vec(self) -> Vec<Pokemon> {
        self.mons
    }

    pub fn iter(&self) -> impl Iterator<Item = &Pokemon> {
        self.mons.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Pokemon> {
        self.mons.iter_mut()
    }
}

impl Default for Party {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Vec<Pokemon>> for Party {
    fn from(mons: Vec<Pokemon>) -> Self {
        debug_assert!(mons.len() <= PARTY_LENGTH);
        Self { mons }
    }
}

impl From<Party> for Vec<Pokemon> {
    fn from(party: Party) -> Self {
        party.mons
    }
}

pub mod link_battle;
pub mod protocol;
pub mod transport;

#[cfg(test)]
mod link_battle_tests;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkState {
    None,
    InCableClub,
    StartTrade,
    StartBattle,
    Battling,
    Trading,
}

impl LinkState {
    pub fn is_connected(&self) -> bool {
        !matches!(self, LinkState::None)
    }

    pub fn is_battling(&self) -> bool {
        matches!(self, LinkState::Battling)
    }

    pub fn is_trading(&self) -> bool {
        matches!(self, LinkState::Trading)
    }
}

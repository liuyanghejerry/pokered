use super::protocol::NetworkMessage;
use super::transport::{NetworkTransport, TransportError};
use crate::battle::state::Pokemon;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinkTradeState {
    Idle,
    WaitingForTradeResponse,
    PeerRequestedTrade,
    SelectingMon,
    WaitingForPeerSelection,
    BothSelected { local_index: u8, remote_index: u8 },
    WaitingForPeerConfirm { local_index: u8, remote_index: u8 },
    PeerConfirmedWaitingLocal { local_index: u8, remote_index: u8 },
    Trading { local_index: u8, remote_index: u8 },
    Completed,
    Cancelled,
    Error(String),
}

#[derive(Debug, PartialEq, Eq)]
pub enum LinkTradePollResult {
    Pending,
    TradeRequested,
    TradeAccepted,
    TradeDeclined,
    PeerSelectedMon(u8),
    BothSelected {
        local_index: u8,
        remote_index: u8,
    },
    PeerConfirmed,
    TradeExecute {
        local_index: u8,
        remote_index: u8,
        received_pokemon: Pokemon,
    },
    PeerCancelled,
    Disconnected,
    Error(String),
}

#[derive(Debug)]
pub struct LinkTradeManager {
    state: LinkTradeState,
    local_selection: Option<u8>,
    remote_selection: Option<u8>,
    local_confirmed: bool,
    remote_confirmed: bool,
}

impl LinkTradeManager {
    pub fn new() -> Self {
        Self {
            state: LinkTradeState::Idle,
            local_selection: None,
            remote_selection: None,
            local_confirmed: false,
            remote_confirmed: false,
        }
    }

    pub fn state(&self) -> &LinkTradeState {
        &self.state
    }

    pub fn is_completed(&self) -> bool {
        matches!(
            self.state,
            LinkTradeState::Completed | LinkTradeState::Cancelled | LinkTradeState::Error(_)
        )
    }

    pub fn request_trade(
        &mut self,
        transport: &mut dyn NetworkTransport,
    ) -> Result<(), TransportError> {
        if self.state != LinkTradeState::Idle {
            return Err(TransportError::IoError(
                "cannot request trade: not idle".into(),
            ));
        }
        transport.send(NetworkMessage::RequestTrade)?;
        self.state = LinkTradeState::WaitingForTradeResponse;
        Ok(())
    }

    pub fn accept_trade(
        &mut self,
        transport: &mut dyn NetworkTransport,
    ) -> Result<(), TransportError> {
        if self.state != LinkTradeState::PeerRequestedTrade {
            return Err(TransportError::IoError("no pending trade request".into()));
        }
        transport.send(NetworkMessage::AcceptTrade)?;
        self.state = LinkTradeState::SelectingMon;
        Ok(())
    }

    pub fn decline_trade(
        &mut self,
        transport: &mut dyn NetworkTransport,
    ) -> Result<(), TransportError> {
        if self.state != LinkTradeState::PeerRequestedTrade {
            return Err(TransportError::IoError("no pending trade request".into()));
        }
        transport.send(NetworkMessage::DeclineTrade)?;
        self.state = LinkTradeState::Idle;
        Ok(())
    }

    pub fn select_mon(
        &mut self,
        transport: &mut dyn NetworkTransport,
        party_index: u8,
    ) -> Result<(), TransportError> {
        match &self.state {
            LinkTradeState::SelectingMon
            | LinkTradeState::WaitingForPeerSelection
            | LinkTradeState::BothSelected { .. } => {}
            _ => {
                return Err(TransportError::IoError("not in selection state".into()));
            }
        }
        transport.send(NetworkMessage::SelectMon(party_index))?;
        self.local_selection = Some(party_index);
        self.local_confirmed = false;
        self.remote_confirmed = false;
        self.try_transition_to_both_selected();
        Ok(())
    }

    pub fn confirm_trade(
        &mut self,
        transport: &mut dyn NetworkTransport,
        pokemon: Pokemon,
    ) -> Result<(), TransportError> {
        let (local_idx, remote_idx) = match &self.state {
            LinkTradeState::BothSelected {
                local_index,
                remote_index,
            } => (*local_index, *remote_index),
            LinkTradeState::PeerConfirmedWaitingLocal {
                local_index,
                remote_index,
            } => (*local_index, *remote_index),
            _ => {
                return Err(TransportError::IoError("not in confirm state".into()));
            }
        };
        transport.send(NetworkMessage::ConfirmTrade)?;
        transport.send(NetworkMessage::TradeComplete(pokemon))?;
        self.local_confirmed = true;

        if self.remote_confirmed {
            self.state = LinkTradeState::Trading {
                local_index: local_idx,
                remote_index: remote_idx,
            };
        } else {
            self.state = LinkTradeState::WaitingForPeerConfirm {
                local_index: local_idx,
                remote_index: remote_idx,
            };
        }
        Ok(())
    }

    pub fn cancel_trade(
        &mut self,
        transport: &mut dyn NetworkTransport,
    ) -> Result<(), TransportError> {
        transport.send(NetworkMessage::CancelTrade)?;
        self.reset_selection();
        self.state = LinkTradeState::SelectingMon;
        Ok(())
    }

    pub fn poll(&mut self, transport: &mut dyn NetworkTransport) -> LinkTradePollResult {
        let msg = match transport.try_recv() {
            Ok(Some(msg)) => msg,
            Ok(None) => return LinkTradePollResult::Pending,
            Err(TransportError::Disconnected) => {
                self.state = LinkTradeState::Cancelled;
                return LinkTradePollResult::Disconnected;
            }
            Err(e) => {
                let msg = format!("{}", e);
                self.state = LinkTradeState::Error(msg.clone());
                return LinkTradePollResult::Error(msg);
            }
        };

        self.handle_message(msg)
    }

    pub fn poll_blocking(&mut self, transport: &mut dyn NetworkTransport) -> LinkTradePollResult {
        let msg = match transport.recv() {
            Ok(msg) => msg,
            Err(TransportError::Disconnected) => {
                self.state = LinkTradeState::Cancelled;
                return LinkTradePollResult::Disconnected;
            }
            Err(e) => {
                let msg = format!("{}", e);
                self.state = LinkTradeState::Error(msg.clone());
                return LinkTradePollResult::Error(msg);
            }
        };

        self.handle_message(msg)
    }

    fn handle_message(&mut self, msg: NetworkMessage) -> LinkTradePollResult {
        match (&self.state, msg) {
            (LinkTradeState::Idle, NetworkMessage::RequestTrade) => {
                self.state = LinkTradeState::PeerRequestedTrade;
                LinkTradePollResult::TradeRequested
            }
            (LinkTradeState::WaitingForTradeResponse, NetworkMessage::AcceptTrade) => {
                self.state = LinkTradeState::SelectingMon;
                LinkTradePollResult::TradeAccepted
            }
            (LinkTradeState::WaitingForTradeResponse, NetworkMessage::DeclineTrade) => {
                self.state = LinkTradeState::Idle;
                LinkTradePollResult::TradeDeclined
            }

            (
                LinkTradeState::SelectingMon
                | LinkTradeState::WaitingForPeerSelection
                | LinkTradeState::BothSelected { .. },
                NetworkMessage::SelectMon(idx),
            ) => {
                self.remote_selection = Some(idx);
                self.remote_confirmed = false;
                self.try_transition_to_both_selected();
                if self.local_selection.is_some() {
                    LinkTradePollResult::BothSelected {
                        local_index: self.local_selection.unwrap(),
                        remote_index: idx,
                    }
                } else {
                    LinkTradePollResult::PeerSelectedMon(idx)
                }
            }

            (
                LinkTradeState::BothSelected {
                    local_index,
                    remote_index,
                },
                NetworkMessage::ConfirmTrade,
            ) => {
                self.remote_confirmed = true;
                let (li, ri) = (*local_index, *remote_index);
                if self.local_confirmed {
                    self.state = LinkTradeState::Trading {
                        local_index: li,
                        remote_index: ri,
                    };
                } else {
                    self.state = LinkTradeState::PeerConfirmedWaitingLocal {
                        local_index: li,
                        remote_index: ri,
                    };
                }
                LinkTradePollResult::PeerConfirmed
            }

            (
                LinkTradeState::WaitingForPeerConfirm {
                    local_index,
                    remote_index,
                },
                NetworkMessage::ConfirmTrade,
            ) => {
                self.remote_confirmed = true;
                let (li, ri) = (*local_index, *remote_index);
                self.state = LinkTradeState::Trading {
                    local_index: li,
                    remote_index: ri,
                };
                LinkTradePollResult::PeerConfirmed
            }

            (
                LinkTradeState::Trading {
                    local_index,
                    remote_index,
                },
                NetworkMessage::TradeComplete(pokemon),
            ) => {
                let (li, ri) = (*local_index, *remote_index);
                self.state = LinkTradeState::Completed;
                LinkTradePollResult::TradeExecute {
                    local_index: li,
                    remote_index: ri,
                    received_pokemon: pokemon,
                }
            }

            (_, NetworkMessage::CancelTrade) => {
                self.reset_selection();
                self.state = LinkTradeState::SelectingMon;
                LinkTradePollResult::PeerCancelled
            }

            (_, NetworkMessage::Disconnect) => {
                self.state = LinkTradeState::Cancelled;
                LinkTradePollResult::Disconnected
            }

            (state, msg) => {
                let err = format!("unexpected message {:?} in state {:?}", msg, state);
                self.state = LinkTradeState::Error(err.clone());
                LinkTradePollResult::Error(err)
            }
        }
    }

    fn try_transition_to_both_selected(&mut self) {
        if let (Some(local), Some(remote)) = (self.local_selection, self.remote_selection) {
            self.state = LinkTradeState::BothSelected {
                local_index: local,
                remote_index: remote,
            };
        } else if self.local_selection.is_some() {
            self.state = LinkTradeState::WaitingForPeerSelection;
        }
    }

    fn reset_selection(&mut self) {
        self.local_selection = None;
        self.remote_selection = None;
        self.local_confirmed = false;
        self.remote_confirmed = false;
    }

    pub fn reset_for_new_trade(&mut self) {
        self.reset_selection();
        if matches!(
            self.state,
            LinkTradeState::Completed | LinkTradeState::Cancelled
        ) {
            self.state = LinkTradeState::Idle;
        }
    }
}

impl Default for LinkTradeManager {
    fn default() -> Self {
        Self::new()
    }
}

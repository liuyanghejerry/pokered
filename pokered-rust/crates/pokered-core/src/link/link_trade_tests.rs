use super::link_trade::*;
use super::protocol::*;
use super::transport::*;
use crate::battle::state::{Pokemon, StatusCondition};
use pokered_data::moves::MoveId;
use pokered_data::species::Species;
use pokered_data::types::PokemonType;

fn make_test_pokemon(species: Species, level: u8) -> Pokemon {
    Pokemon {
        species,
        level,
        hp: 100,
        max_hp: 100,
        attack: 50,
        defense: 40,
        speed: 60,
        special: 55,
        type1: PokemonType::Normal,
        type2: PokemonType::Normal,
        moves: [MoveId::Tackle, MoveId::None, MoveId::None, MoveId::None],
        pp: [35, 0, 0, 0],
        pp_ups: [0; 4],
        status: StatusCondition::None,
        dv_bytes: [0xAB, 0xCD],
        stat_exp: [0; 5],
        total_exp: 1000,
        is_traded: false,
    }
}

#[test]
fn test_trade_request_accept_flow() {
    let (mut t_a, mut t_b) = ChannelTransport::new_pair();
    let mut mgr_a = LinkTradeManager::new();
    let mut mgr_b = LinkTradeManager::new();

    assert_eq!(*mgr_a.state(), LinkTradeState::Idle);
    assert_eq!(*mgr_b.state(), LinkTradeState::Idle);

    mgr_a.request_trade(&mut t_a).unwrap();
    assert_eq!(*mgr_a.state(), LinkTradeState::WaitingForTradeResponse);

    let result_b = mgr_b.poll_blocking(&mut t_b);
    assert_eq!(result_b, LinkTradePollResult::TradeRequested);
    assert_eq!(*mgr_b.state(), LinkTradeState::PeerRequestedTrade);

    mgr_b.accept_trade(&mut t_b).unwrap();
    assert_eq!(*mgr_b.state(), LinkTradeState::SelectingMon);

    let result_a = mgr_a.poll_blocking(&mut t_a);
    assert_eq!(result_a, LinkTradePollResult::TradeAccepted);
    assert_eq!(*mgr_a.state(), LinkTradeState::SelectingMon);
}

#[test]
fn test_trade_request_decline_flow() {
    let (mut t_a, mut t_b) = ChannelTransport::new_pair();
    let mut mgr_a = LinkTradeManager::new();
    let mut mgr_b = LinkTradeManager::new();

    mgr_a.request_trade(&mut t_a).unwrap();
    mgr_b.poll_blocking(&mut t_b);

    mgr_b.decline_trade(&mut t_b).unwrap();
    assert_eq!(*mgr_b.state(), LinkTradeState::Idle);

    let result_a = mgr_a.poll_blocking(&mut t_a);
    assert_eq!(result_a, LinkTradePollResult::TradeDeclined);
    assert_eq!(*mgr_a.state(), LinkTradeState::Idle);
}

#[test]
fn test_request_trade_when_not_idle_fails() {
    let (mut t_a, mut t_b) = ChannelTransport::new_pair();
    let mut mgr_a = LinkTradeManager::new();

    mgr_a.request_trade(&mut t_a).unwrap();
    let result = mgr_a.request_trade(&mut t_b);
    assert!(result.is_err());
}

#[test]
fn test_accept_trade_without_request_fails() {
    let (mut t_a, _t_b) = ChannelTransport::new_pair();
    let mut mgr = LinkTradeManager::new();

    let result = mgr.accept_trade(&mut t_a);
    assert!(result.is_err());
}

#[test]
fn test_decline_trade_without_request_fails() {
    let (mut t_a, _t_b) = ChannelTransport::new_pair();
    let mut mgr = LinkTradeManager::new();

    let result = mgr.decline_trade(&mut t_a);
    assert!(result.is_err());
}

fn setup_selecting_pair(
    t_a: &mut ChannelTransport,
    t_b: &mut ChannelTransport,
) -> (LinkTradeManager, LinkTradeManager) {
    let mut mgr_a = LinkTradeManager::new();
    let mut mgr_b = LinkTradeManager::new();

    mgr_a.request_trade(t_a).unwrap();
    mgr_b.poll_blocking(t_b);
    mgr_b.accept_trade(t_b).unwrap();
    mgr_a.poll_blocking(t_a);

    assert_eq!(*mgr_a.state(), LinkTradeState::SelectingMon);
    assert_eq!(*mgr_b.state(), LinkTradeState::SelectingMon);
    (mgr_a, mgr_b)
}

#[test]
fn test_mon_selection_both_select() {
    let (mut t_a, mut t_b) = ChannelTransport::new_pair();
    let (mut mgr_a, mut mgr_b) = setup_selecting_pair(&mut t_a, &mut t_b);

    mgr_a.select_mon(&mut t_a, 0).unwrap();
    assert_eq!(*mgr_a.state(), LinkTradeState::WaitingForPeerSelection);

    mgr_b.select_mon(&mut t_b, 1).unwrap();
    assert_eq!(*mgr_b.state(), LinkTradeState::WaitingForPeerSelection);

    let result_b = mgr_b.poll_blocking(&mut t_b);
    assert_eq!(
        result_b,
        LinkTradePollResult::BothSelected {
            local_index: 1,
            remote_index: 0,
        }
    );
    assert_eq!(
        *mgr_b.state(),
        LinkTradeState::BothSelected {
            local_index: 1,
            remote_index: 0,
        }
    );

    let result_a = mgr_a.poll_blocking(&mut t_a);
    assert_eq!(
        result_a,
        LinkTradePollResult::BothSelected {
            local_index: 0,
            remote_index: 1,
        }
    );
}

#[test]
fn test_peer_selects_first_then_local() {
    let (mut t_a, mut t_b) = ChannelTransport::new_pair();
    let (mut mgr_a, mut mgr_b) = setup_selecting_pair(&mut t_a, &mut t_b);

    mgr_b.select_mon(&mut t_b, 2).unwrap();

    let result_a = mgr_a.poll_blocking(&mut t_a);
    assert_eq!(result_a, LinkTradePollResult::PeerSelectedMon(2));
    assert_eq!(*mgr_a.state(), LinkTradeState::SelectingMon);

    mgr_a.select_mon(&mut t_a, 0).unwrap();
    assert_eq!(
        *mgr_a.state(),
        LinkTradeState::BothSelected {
            local_index: 0,
            remote_index: 2,
        }
    );
}

#[test]
fn test_select_mon_wrong_state_fails() {
    let (mut t_a, _t_b) = ChannelTransport::new_pair();
    let mut mgr = LinkTradeManager::new();

    let result = mgr.select_mon(&mut t_a, 0);
    assert!(result.is_err());
}

#[test]
fn test_full_trade_execute() {
    let (mut t_a, mut t_b) = ChannelTransport::new_pair();
    let (mut mgr_a, mut mgr_b) = setup_selecting_pair(&mut t_a, &mut t_b);

    mgr_a.select_mon(&mut t_a, 0).unwrap();
    mgr_b.select_mon(&mut t_b, 1).unwrap();
    mgr_b.poll_blocking(&mut t_b);
    mgr_a.poll_blocking(&mut t_a);

    let pokemon_a = make_test_pokemon(Species::Pikachu, 25);
    let pokemon_b = make_test_pokemon(Species::Charizard, 36);

    mgr_a.confirm_trade(&mut t_a, pokemon_a.clone()).unwrap();
    assert!(matches!(
        mgr_a.state(),
        LinkTradeState::WaitingForPeerConfirm { .. }
    ));

    let result_b = mgr_b.poll_blocking(&mut t_b);
    assert_eq!(result_b, LinkTradePollResult::PeerConfirmed);
    assert!(matches!(
        mgr_b.state(),
        LinkTradeState::PeerConfirmedWaitingLocal { .. }
    ));

    mgr_b.confirm_trade(&mut t_b, pokemon_b.clone()).unwrap();
    assert!(matches!(mgr_b.state(), LinkTradeState::Trading { .. }));

    let result_b = mgr_b.poll_blocking(&mut t_b);
    assert!(matches!(
        result_b,
        LinkTradePollResult::TradeExecute {
            local_index: 1,
            remote_index: 0,
            ..
        }
    ));
    assert_eq!(*mgr_b.state(), LinkTradeState::Completed);

    let result_a = mgr_a.poll_blocking(&mut t_a);
    assert_eq!(result_a, LinkTradePollResult::PeerConfirmed);

    let result_a = mgr_a.poll_blocking(&mut t_a);
    assert!(matches!(
        result_a,
        LinkTradePollResult::TradeExecute {
            local_index: 0,
            remote_index: 1,
            ..
        }
    ));
    assert_eq!(*mgr_a.state(), LinkTradeState::Completed);
}

#[test]
fn test_confirm_trade_wrong_state_fails() {
    let (mut t_a, _t_b) = ChannelTransport::new_pair();
    let mut mgr = LinkTradeManager::new();

    let pokemon = make_test_pokemon(Species::Pikachu, 25);
    let result = mgr.confirm_trade(&mut t_a, pokemon);
    assert!(result.is_err());
}

#[test]
fn test_cancel_trade_returns_to_selecting() {
    let (mut t_a, mut t_b) = ChannelTransport::new_pair();
    let (mut mgr_a, mut mgr_b) = setup_selecting_pair(&mut t_a, &mut t_b);

    mgr_a.select_mon(&mut t_a, 0).unwrap();
    mgr_b.select_mon(&mut t_b, 1).unwrap();
    mgr_b.poll_blocking(&mut t_b);
    mgr_a.poll_blocking(&mut t_a);

    mgr_a.cancel_trade(&mut t_a).unwrap();
    assert_eq!(*mgr_a.state(), LinkTradeState::SelectingMon);

    let result_b = mgr_b.poll_blocking(&mut t_b);
    assert_eq!(result_b, LinkTradePollResult::PeerCancelled);
    assert_eq!(*mgr_b.state(), LinkTradeState::SelectingMon);
}

#[test]
fn test_disconnect_during_trade() {
    let (mut t_a, mut t_b) = ChannelTransport::new_pair();
    let (_mgr_a, mut mgr_b) = setup_selecting_pair(&mut t_a, &mut t_b);

    t_a.send(NetworkMessage::Disconnect).unwrap();

    let result_b = mgr_b.poll_blocking(&mut t_b);
    assert_eq!(result_b, LinkTradePollResult::Disconnected);
    assert_eq!(*mgr_b.state(), LinkTradeState::Cancelled);
}

#[test]
fn test_channel_drop_causes_disconnect() {
    let (mut t_a, t_b) = ChannelTransport::new_pair();
    let mut mgr_a = LinkTradeManager::new();

    mgr_a.request_trade(&mut t_a).unwrap();
    drop(t_b);

    let result = mgr_a.poll_blocking(&mut t_a);
    assert_eq!(result, LinkTradePollResult::Disconnected);
    assert_eq!(*mgr_a.state(), LinkTradeState::Cancelled);
}

#[test]
fn test_poll_returns_pending_when_no_message() {
    let (mut t_a, _t_b) = ChannelTransport::new_pair();
    let mut mgr = LinkTradeManager::new();

    let result = mgr.poll(&mut t_a);
    assert_eq!(result, LinkTradePollResult::Pending);
}

#[test]
fn test_reset_for_new_trade() {
    let (mut t_a, mut t_b) = ChannelTransport::new_pair();
    let (mut mgr_a, mut mgr_b) = setup_selecting_pair(&mut t_a, &mut t_b);

    mgr_a.select_mon(&mut t_a, 0).unwrap();
    mgr_b.select_mon(&mut t_b, 1).unwrap();
    mgr_b.poll_blocking(&mut t_b);
    mgr_a.poll_blocking(&mut t_a);

    let pokemon_a = make_test_pokemon(Species::Pikachu, 25);
    let pokemon_b = make_test_pokemon(Species::Charizard, 36);
    mgr_a.confirm_trade(&mut t_a, pokemon_a).unwrap();
    mgr_b.poll_blocking(&mut t_b);
    mgr_b.confirm_trade(&mut t_b, pokemon_b).unwrap();
    mgr_b.poll_blocking(&mut t_b);
    assert_eq!(*mgr_b.state(), LinkTradeState::Completed);

    mgr_b.reset_for_new_trade();
    assert_eq!(*mgr_b.state(), LinkTradeState::Idle);
}

#[test]
fn test_unexpected_message_causes_error() {
    let (mut t_a, mut t_b) = ChannelTransport::new_pair();
    let mut mgr_a = LinkTradeManager::new();

    t_b.send(NetworkMessage::ConfirmTrade).unwrap();

    let result = mgr_a.poll_blocking(&mut t_a);
    assert!(matches!(result, LinkTradePollResult::Error(_)));
    assert!(matches!(mgr_a.state(), LinkTradeState::Error(_)));
}

#[test]
fn test_reselect_mon_resets_confirmation() {
    let (mut t_a, mut t_b) = ChannelTransport::new_pair();
    let (mut mgr_a, mut mgr_b) = setup_selecting_pair(&mut t_a, &mut t_b);

    mgr_a.select_mon(&mut t_a, 0).unwrap();
    mgr_b.select_mon(&mut t_b, 1).unwrap();
    mgr_b.poll_blocking(&mut t_b);
    mgr_a.poll_blocking(&mut t_a);

    mgr_a.select_mon(&mut t_a, 2).unwrap();
    assert_eq!(
        *mgr_a.state(),
        LinkTradeState::BothSelected {
            local_index: 2,
            remote_index: 1,
        }
    );

    let result_b = mgr_b.poll_blocking(&mut t_b);
    assert_eq!(
        result_b,
        LinkTradePollResult::BothSelected {
            local_index: 1,
            remote_index: 2,
        }
    );
}

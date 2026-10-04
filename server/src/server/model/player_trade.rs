use database::model::InventoryRecord;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerTradePhase {
    Requested,
    Accepted,
    Locked,
    Confirmed,
}

impl PlayerTradePhase {
    pub fn blocks_actions(self) -> bool { self != Self::Requested }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlayerTradeItem {
    pub inventory_index: usize,
    pub item: InventoryRecord,
    pub amount: i16,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlayerTrade {
    pub session_id: u64,
    pub partner_id: u32,
    pub requested_by: u32,
    pub phase: PlayerTradePhase,
    pub items: Vec<PlayerTradeItem>,
    pub zeny: u32,
    pub requested_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlayerTradeReceipt {
    pub session_id: u64,
    pub partner_id: u32,
}

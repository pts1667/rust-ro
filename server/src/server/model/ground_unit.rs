use models::status::StatusSnapshot;
use movement::position::Position;

use super::map_instance::MapInstanceKey;
use super::map_item::{MapItem, MapItemSnapshot, MapItemType};

#[derive(Clone, Debug)]
pub(crate) struct GroundUnitSnapshot {
    pub id: u32,
    pub skill_id: u32,
    pub map: MapInstanceKey,
    pub x: u16,
    pub y: u16,
    pub hp: u16,
    pub expires_at: u128,
    pub view_id: u32,
    pub used: bool,
}

impl GroundUnitSnapshot {
    pub fn alive(&self, tick: u128) -> bool {
        self.hp > 0 && self.expires_at > tick
    }

    pub fn map_item(&self) -> MapItem {
        MapItem::new(self.id, self.view_id as i16, MapItemType::SkillUnit)
    }

    pub fn snapshot(&self) -> MapItemSnapshot {
        MapItemSnapshot::new(self.map_item(), Position {
            x: self.x,
            y: self.y,
            dir: 0,
        })
    }

    pub fn status(&self) -> StatusSnapshot {
        StatusSnapshot::new_for_skill_unit(self.hp > 0)
    }
}

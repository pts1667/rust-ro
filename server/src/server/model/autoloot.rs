//! `@autoloot` and `@autolootitem`: drops of the killer that go straight to the inventory.

pub const AUTOLOOT_ITEM_SLOTS: usize = 10;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct AutoLoot {
    /// Items dropping at this rate (1 = 0.01%) or less are looted, 0 is off.
    pub rate: u16,
    pub items: [i32; AUTOLOOT_ITEM_SLOTS],
}

impl AutoLoot {
    pub fn takes(&self, item_id: i32, drop_rate: u16) -> bool {
        self.rate > 0 && drop_rate <= self.rate || item_id != 0 && self.items.contains(&item_id)
    }

    pub fn is_off(&self) -> bool {
        *self == Self::default()
    }
}

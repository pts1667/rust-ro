#![allow(dead_code)]
use crate::enums::*;

#[derive(WithMaskValueU32, Debug, Copy, Clone, PartialEq, Eq)]
pub enum MapRestrictionZone {
    #[mask_value = 16]
    Zone0,
    Zone1,
    Zone2,
    Zone3,
    Zone4,
    Zone5,
    Zone6,
    Zone7,
    Zone8,
    Zone9,
    Zone10,
    Zone11,
    Zone12,
    Zone13,
    Zone14,
    Zone15,
    Zone16,
    Zone17,
    Zone18,
    Zone19,
    Zone20,
    Zone21,
    Zone22,
    Zone23,
    Zone24,
    Zone25,
    Zone26,
    Zone27,
}

impl MapRestrictionZone {
    pub fn from_zone(zone: i32) -> Option<Self> {
        let zones = [
            Self::Zone0,
            Self::Zone1,
            Self::Zone2,
            Self::Zone3,
            Self::Zone4,
            Self::Zone5,
            Self::Zone6,
            Self::Zone7,
            Self::Zone8,
            Self::Zone9,
            Self::Zone10,
            Self::Zone11,
            Self::Zone12,
            Self::Zone13,
            Self::Zone14,
            Self::Zone15,
            Self::Zone16,
            Self::Zone17,
            Self::Zone18,
            Self::Zone19,
            Self::Zone20,
            Self::Zone21,
            Self::Zone22,
            Self::Zone23,
            Self::Zone24,
            Self::Zone25,
            Self::Zone26,
            Self::Zone27,
        ];
        usize::try_from(zone).ok().and_then(|index| zones.get(index)).copied()
    }
}

#[derive(WithMaskValueU16, Debug, Copy, Clone, PartialEq, Eq)]
pub enum MapActorType {
    #[mask_value = 1]
    Player,
    Monster,
    Pet,
    Homunculus,
    Mercenary,
    Item,
    Skill,
    Npc,
    Chat,
    Elemental,
    #[mask_value = 4095]
    All,
}

impl From<crate::enums::actor::CombatActorKind> for MapActorType {
    fn from(kind: crate::enums::actor::CombatActorKind) -> Self {
        use crate::enums::actor::CombatActorKind;
        match kind {
            CombatActorKind::Player => Self::Player,
            CombatActorKind::Monster => Self::Monster,
            CombatActorKind::Npc => Self::Npc,
            CombatActorKind::Homunculus => Self::Homunculus,
            CombatActorKind::Mercenary => Self::Mercenary,
            CombatActorKind::Pet => Self::Pet,
            CombatActorKind::SkillUnit => Self::Skill,
        }
    }
}

#[derive(WithMaskValueU32, Debug, Copy, Clone, PartialEq, Eq)]
pub enum SkillDamageMap {
    #[mask_value = 1]
    Normal,
    Pvp,
    Gvg,
    Battleground,
    Flag,
}

#[derive(WithMaskValueU8, Debug, Copy, Clone, PartialEq, Eq)]
pub enum NightmareDropLocation {
    #[mask_value = 1]
    Inventory,
    Equipped,
    #[mask_all]
    All,
}

#[derive(WithMaskValueU64, Debug, Copy, Clone, PartialEq, Eq)]
pub enum MapPropertyFlags {
    // PARTY - Show attack cursor on non-party members (PvP)
    IsParty,
    // GUILD - Show attack cursor on non-guild members (GvG)
    IsGuild,
    // SIEGE - Show emblem over characters heads when in GvG (WoE castle)
    IsSiege,
    // USE_SIMPLE_EFFECT - Automatically enable /mineffect
    UseSimpleEffect,
    // DISABLE_LOCKON - Only allow attacks on other players with shift key or /ns active
    IsNoLockOn,
    // COUNT_PK - Show the PvP counter
    CountPk,
    // NO_PARTY_FORMATION - Prevents party creation/modification (Might be used for instance dungeons)
    PartyLock,
    // BATTLEFIELD - Unknown (Does something for battlegrounds areas)
    IsBattleground,
    // DISABLE_COSTUMEITEM - Disable costume sprites
    IsNoCostum,
    // USECART - Allow opening cart inventory (Well force it to always allow it)
    IsUseCart,
    // SUNMOONSTAR_MIRACLE - Blocks Star Gladiator's Miracle from activating
    IsSummonstarMiracle,
    // Unused bits. 1 - 10 is 0x1 length and 11 is 0x15 length. May be used for future settings.
    Unused,
}

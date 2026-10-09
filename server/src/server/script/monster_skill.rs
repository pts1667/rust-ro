use models::enums::skill_enums::SkillEnum;
use models::status_change::StatusChangeKind;

use super::metadata::SkillMetadata;
use crate::server::service::script_combat_service::BreakSlot;

/// Monster skills whose effect the metadata cannot describe on its own. Each rule is keyed by `SkillEnum` here,
/// so call sites ask `metadata.monster_skill()` instead of comparing names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum MonsterSkill {
    /// Hits every target in the area with one status at full chance.
    AreaStatus(StatusChangeKind),
    /// Every target gets one of four statuses, picked per cast.
    DragonFear,
    /// Vanishes each target and drains its SP.
    WideSoulDrain,
    SelfDestruction,
    Expulsion,
    InvincibleOff,
    DarkBreath,
    MagicalAttack,
    AntiMagic,
    CriticalSlash,
    VampireGift,
    /// Breaks one equipment slot of a player on a successful hit.
    Break(BreakSlot),
    /// A status the caster gains on itself.
    SelfStatus(StatusChangeKind),
    EvilLand,
    GrandDarkness,
    EarthQuake,
}

impl MonsterSkill {
    pub fn of(skill: SkillEnum) -> Option<Self> {
        use StatusChangeKind as Kind;
        Some(match skill {
            SkillEnum::NpcWidebleeding => Self::AreaStatus(Kind::Bleeding),
            SkillEnum::NpcWideconfuse => Self::AreaStatus(Kind::Confusion),
            SkillEnum::NpcWidecurse => Self::AreaStatus(Kind::Curse),
            SkillEnum::NpcWidesilence => Self::AreaStatus(Kind::Silence),
            SkillEnum::NpcWidesleep => Self::AreaStatus(Kind::Sleep),
            SkillEnum::NpcWidestone => Self::AreaStatus(Kind::Stone),
            SkillEnum::NpcWidefreeze => Self::AreaStatus(Kind::Freeze),
            SkillEnum::NpcWidestun => Self::AreaStatus(Kind::Stun),
            SkillEnum::NpcWidehelldignity => Self::AreaStatus(Kind::HellPower),
            SkillEnum::NpcDragonfear => Self::DragonFear,
            SkillEnum::NpcWidesouldrain => Self::WideSoulDrain,
            SkillEnum::NpcSelfdestruction => Self::SelfDestruction,
            SkillEnum::NpcExpulsion => Self::Expulsion,
            SkillEnum::NpcInvincibleoff => Self::InvincibleOff,
            SkillEnum::NpcDarkbreath => Self::DarkBreath,
            SkillEnum::NpcMagicalattack => Self::MagicalAttack,
            SkillEnum::NpcAntimagic => Self::AntiMagic,
            SkillEnum::NpcCriticalslash => Self::CriticalSlash,
            SkillEnum::NpcVampireGift => Self::VampireGift,
            SkillEnum::NpcArmorbrake => Self::Break(BreakSlot::Armor),
            SkillEnum::NpcHelmbrake => Self::Break(BreakSlot::Helm),
            SkillEnum::NpcShieldbrake => Self::Break(BreakSlot::Shield),
            SkillEnum::NpcPowerup => Self::SelfStatus(Kind::IncAttackRate),
            SkillEnum::NpcWeaponbraker => Self::SelfStatus(Kind::WeaponBreaker),
            SkillEnum::NpcCriticalwound => Self::SelfStatus(Kind::CriticalWound),
            SkillEnum::NpcHellpower => Self::SelfStatus(Kind::HellPower),
            SkillEnum::NpcEvilland => Self::EvilLand,
            SkillEnum::NpcGranddarkness => Self::GrandDarkness,
            SkillEnum::NpcEarthquake => Self::EarthQuake,
            _ => return None,
        })
    }

    /// The monster skill a skill name refers to; `None` for player skills and unknown names.
    pub fn of_name(name: &str) -> Option<Self> {
        SkillMetadata::find_by_name(name)?.monster_skill()
    }

    /// Area skills reach every target in range instead of one chosen target.
    pub fn is_area(self) -> bool {
        matches!(self, Self::AreaStatus(_) | Self::DragonFear | Self::WideSoulDrain)
    }

    /// Skills a monster casts as a direct effect on its target, without a damage roll.
    pub fn is_direct_support(self) -> bool {
        self.is_area() || matches!(self, Self::SelfDestruction | Self::Expulsion | Self::InvincibleOff | Self::DarkBreath)
    }
}

impl SkillMetadata {
    pub(crate) fn monster_skill(&self) -> Option<MonsterSkill> {
        MonsterSkill::of(SkillEnum::try_from_value(self.id).ok()?)
    }

    /// Monster skills are named `NPC_*`; this is the only place that prefix is read.
    pub fn is_monster(&self) -> bool {
        self.name.starts_with("NPC_")
    }

    /// The `NPC_WIDE*` family, which includes skills with no server rule (for example `NPC_WIDESIGHT`).
    pub fn is_wide_monster(&self) -> bool {
        self.name.starts_with("NPC_WIDE")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn monster_rules_follow_the_skill_names_in_metadata() {
        let expected = [
            ("NPC_WIDEBLEEDING", MonsterSkill::AreaStatus(StatusChangeKind::Bleeding)),
            ("NPC_WIDECONFUSE", MonsterSkill::AreaStatus(StatusChangeKind::Confusion)),
            ("NPC_WIDECURSE", MonsterSkill::AreaStatus(StatusChangeKind::Curse)),
            ("NPC_WIDESILENCE", MonsterSkill::AreaStatus(StatusChangeKind::Silence)),
            ("NPC_WIDESLEEP", MonsterSkill::AreaStatus(StatusChangeKind::Sleep)),
            ("NPC_WIDESTONE", MonsterSkill::AreaStatus(StatusChangeKind::Stone)),
            ("NPC_WIDEFREEZE", MonsterSkill::AreaStatus(StatusChangeKind::Freeze)),
            ("NPC_WIDESTUN", MonsterSkill::AreaStatus(StatusChangeKind::Stun)),
            ("NPC_WIDEHELLDIGNITY", MonsterSkill::AreaStatus(StatusChangeKind::HellPower)),
            ("NPC_DRAGONFEAR", MonsterSkill::DragonFear),
            ("NPC_WIDESOULDRAIN", MonsterSkill::WideSoulDrain),
            ("NPC_SELFDESTRUCTION", MonsterSkill::SelfDestruction),
            ("NPC_EXPULSION", MonsterSkill::Expulsion),
            ("NPC_INVINCIBLEOFF", MonsterSkill::InvincibleOff),
            ("NPC_DARKBREATH", MonsterSkill::DarkBreath),
            ("NPC_MAGICALATTACK", MonsterSkill::MagicalAttack),
            ("NPC_ANTIMAGIC", MonsterSkill::AntiMagic),
            ("NPC_CRITICALSLASH", MonsterSkill::CriticalSlash),
            ("NPC_VAMPIRE_GIFT", MonsterSkill::VampireGift),
            ("NPC_ARMORBRAKE", MonsterSkill::Break(BreakSlot::Armor)),
            ("NPC_HELMBRAKE", MonsterSkill::Break(BreakSlot::Helm)),
            ("NPC_SHIELDBRAKE", MonsterSkill::Break(BreakSlot::Shield)),
            ("NPC_POWERUP", MonsterSkill::SelfStatus(StatusChangeKind::IncAttackRate)),
            ("NPC_WEAPONBRAKER", MonsterSkill::SelfStatus(StatusChangeKind::WeaponBreaker)),
            ("NPC_CRITICALWOUND", MonsterSkill::SelfStatus(StatusChangeKind::CriticalWound)),
            ("NPC_HELLPOWER", MonsterSkill::SelfStatus(StatusChangeKind::HellPower)),
            ("NPC_EVILLAND", MonsterSkill::EvilLand),
            ("NPC_GRANDDARKNESS", MonsterSkill::GrandDarkness),
            ("NPC_EARTHQUAKE", MonsterSkill::EarthQuake),
        ];
        for (name, rule) in expected {
            assert_eq!(MonsterSkill::of_name(name), Some(rule), "{name}");
        }
        assert_eq!(MonsterSkill::of_name("NPC_WIDESIGHT"), None);
        assert_eq!(MonsterSkill::of_name("SM_BASH"), None);
    }

    #[test]
    fn direct_support_is_the_area_family_plus_the_four_direct_effects() {
        assert!(MonsterSkill::AreaStatus(StatusChangeKind::Stun).is_direct_support());
        assert!(MonsterSkill::DragonFear.is_direct_support());
        assert!(MonsterSkill::DarkBreath.is_direct_support());
        assert!(!MonsterSkill::MagicalAttack.is_direct_support());
        assert!(!MonsterSkill::SelfStatus(StatusChangeKind::HellPower).is_direct_support());
        assert!(!MonsterSkill::Expulsion.is_area());
    }

    #[test]
    fn element_change_skills_name_their_element_in_metadata() {
        use models::enums::element::Element;
        use models::enums::EnumWithStringValue;
        for (name, element) in [
            ("NPC_CHANGEWATER", Element::Water),
            ("NPC_CHANGEGROUND", Element::Earth),
            ("NPC_CHANGEFIRE", Element::Fire),
            ("NPC_CHANGEWIND", Element::Wind),
            ("NPC_CHANGEPOISON", Element::Poison),
            ("NPC_CHANGEHOLY", Element::Holy),
            ("NPC_CHANGEDARKNESS", Element::Dark),
            ("NPC_CHANGETELEKINESIS", Element::Ghost),
        ] {
            let metadata = SkillMetadata::find_by_name(name).unwrap();
            assert_eq!(metadata.element(1).and_then(|value| Element::try_from_string(value).ok()), Some(element), "{name}");
        }
    }
}

use models::enums::EnumWithMaskValueU32;
use models::enums::bonus::BonusType;
use models::enums::mob::{MobCapability, MobClass};
use models::status::{Status, StatusSnapshot};
use models::status_change::{StatusChangeKind, StatusDisplayFlag};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct StealthState {
    pub hiding: bool,
    pub cloaking: bool,
    pub chase_walk: bool,
    pub invisible: bool,
    pub perfect_hiding: bool,
}

impl StealthState {
    pub fn from_status_options(status: &Status, options: u64) -> Self {
        let mut stealth = Self::from_changes(status.active_statuses.iter().map(|change| change.kind));
        stealth.hiding |= options & u64::from(StatusDisplayFlag::Hiding.as_flag()) != 0;
        stealth.cloaking |= options & u64::from(StatusDisplayFlag::Cloaking.as_flag()) != 0;
        stealth.chase_walk |= options & u64::from(StatusDisplayFlag::ChaseWalk.as_flag()) != 0;
        stealth.invisible = options & u64::from(StatusDisplayFlag::Invisible.as_flag()) != 0;
        stealth
    }

    pub fn from_status(status: &Status) -> Self {
        Self::from_status_options(status, status.state)
    }

    pub fn from_snapshot(status: &StatusSnapshot) -> Self {
        Self::from_changes(status.active_statuses().iter().map(|change| change.kind))
    }

    fn from_changes(kinds: impl Iterator<Item = StatusChangeKind>) -> Self {
        let mut stealth = Self::default();
        for kind in kinds {
            match kind {
                StatusChangeKind::Hiding => stealth.hiding = true,
                StatusChangeKind::Cloaking => stealth.cloaking = true,
                StatusChangeKind::ChaseWalk => stealth.chase_walk = true,
                _ => {}
            }
        }
        stealth
    }

    pub fn hidden(self) -> bool {
        self.hiding || self.cloaking || self.chase_walk
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct VisibilityObserver {
    pub boss: bool,
    pub detector: bool,
    pub see_hidden: bool,
    pub ignores_stealth: bool,
}

impl VisibilityObserver {
    pub fn player(status: &StatusSnapshot) -> Self {
        Self {
            see_hidden: status
                .bonuses()
                .iter()
                .any(|bonus| matches!(bonus.bonus(), BonusType::EnableSeeHidden))
                || status.has_status_change(StatusChangeKind::Intravision),
            ..Self::default()
        }
    }

    pub fn monster(status: &StatusSnapshot) -> Self {
        Self {
            boss: *status.mob_class() == MobClass::Boss,
            detector: status.has_mob_capability(MobCapability::Detector),
            ..Self::default()
        }
    }

    pub fn npc() -> Self {
        Self {
            ignores_stealth: true,
            ..Self::default()
        }
    }

    fn detects(self, target: StealthState) -> bool {
        self.ignores_stealth || self.boss || self.detector && !target.perfect_hiding
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetingMode {
    Direct,
    DirectHiddenSkill,
    SkillCompletion { can_hit_hidden: bool },
    Area { can_hit_hidden: bool },
}

pub fn can_see(observer: VisibilityObserver, target: StealthState) -> bool {
    !target.invisible && (!target.hidden() || observer.see_hidden || observer.detects(target))
}

pub fn can_target(observer: VisibilityObserver, target: StealthState, mode: TargetingMode) -> bool {
    if target.invisible {
        return false;
    }
    if observer.detects(target) {
        return true;
    }
    match mode {
        TargetingMode::Direct => !target.hidden(),
        TargetingMode::DirectHiddenSkill => !target.cloaking && !target.chase_walk,
        TargetingMode::SkillCompletion { can_hit_hidden } | TargetingMode::Area { can_hit_hidden } => !target.hiding || can_hit_hidden,
    }
}

#[cfg(test)]
mod tests {
    use models::enums::element::Element;
    use models::enums::mob::MobRace;
    use models::enums::size::Size;

    use super::*;
    fn observer(class: MobClass, capabilities: u32) -> VisibilityObserver {
        let mut status = StatusSnapshot::new_for_mob(
            1002,
            100,
            0,
            100,
            0,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            150,
            0,
            0,
            Size::Medium,
            Element::Neutral,
            MobRace::Demon,
            1,
        );
        status.set_mob_class(class);
        status.set_mob_capabilities(capabilities);
        VisibilityObserver::monster(&status)
    }
    #[test]
    fn immunity_and_race_do_not_detect_stealth_but_detector_flags_and_actual_boss_class_do() {
        for target in [
            StealthState {
                hiding: true,
                ..Default::default()
            },
            StealthState {
                cloaking: true,
                ..Default::default()
            },
            StealthState {
                chase_walk: true,
                ..Default::default()
            },
        ] {
            assert!(!can_see(observer(MobClass::Normal, 0), target));
            assert!(!can_see(
                observer(MobClass::Guardian, MobCapability::StatusImmune.as_flag()),
                target
            ));
            assert!(!can_target(
                observer(
                    MobClass::Battlefield,
                    MobCapability::StatusImmune.as_flag() | MobCapability::SkillImmune.as_flag()
                ),
                target,
                TargetingMode::Direct
            ));
            for observer in [
                observer(MobClass::Normal, MobCapability::Detector.as_flag()),
                observer(MobClass::Boss, 0),
                VisibilityObserver::npc(),
            ] {
                assert!(can_see(observer, target));
                assert!(can_target(observer, target, TargetingMode::Direct));
            }
        }
    }
    #[test]
    fn intravision_shows_players_without_removing_hiding_target_immunity() {
        let observer = VisibilityObserver {
            see_hidden: true,
            ..Default::default()
        };
        let target = StealthState {
            hiding: true,
            ..Default::default()
        };
        assert!(can_see(observer, target));
        assert!(!can_target(observer, target, TargetingMode::Direct));
        assert!(can_target(observer, target, TargetingMode::DirectHiddenSkill));
    }
    #[test]
    fn cloaking_does_not_evade_already_committed_skills_or_area_attacks_but_hiding_does() {
        let ordinary = VisibilityObserver::default();
        assert!(can_target(
            ordinary,
            StealthState {
                cloaking: true,
                ..Default::default()
            },
            TargetingMode::SkillCompletion { can_hit_hidden: false }
        ));
        assert!(can_target(
            ordinary,
            StealthState {
                chase_walk: true,
                ..Default::default()
            },
            TargetingMode::Area { can_hit_hidden: false }
        ));
        assert!(!can_target(
            ordinary,
            StealthState {
                hiding: true,
                ..Default::default()
            },
            TargetingMode::Area { can_hit_hidden: false }
        ));
        assert!(can_target(
            ordinary,
            StealthState {
                hiding: true,
                ..Default::default()
            },
            TargetingMode::Area { can_hit_hidden: true }
        ));
    }
    #[test]
    fn perfect_hiding_prevents_detector_acquisition_and_admin_invisibility_is_never_targetable() {
        let target = StealthState {
            hiding: true,
            perfect_hiding: true,
            ..Default::default()
        };
        assert!(!can_see(observer(MobClass::Normal, MobCapability::Detector.as_flag()), target));
        assert!(can_see(observer(MobClass::Boss, 0), target));
        assert!(!can_target(
            VisibilityObserver {
                boss: true,
                detector: true,
                see_hidden: true,
                ..Default::default()
            },
            StealthState {
                invisible: true,
                ..Default::default()
            },
            TargetingMode::DirectHiddenSkill
        ));
    }
}

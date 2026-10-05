use models::enums::actor::CombatActorKind;
use models::enums::bonus::BonusType;
use models::enums::element::Element;
use models::enums::mob::{MobMode, MobRace};
use models::enums::size::Size;
use models::enums::EnumWithMaskValueU32;
use models::status::{Status, StatusSnapshot};
use models::status_bonus::StatusBonus;
use models::status_change::{StatusChange, StatusChangeKind, StatusChangeRequest, StatusStartFlag};

use super::ScriptSkillActor;
use crate::server::model::map_item::MapItemType;
use crate::server::model::script::Script;
use crate::server::service::status_effect_service::{StatusChangeOutcome, StatusEffectService};

#[derive(Clone, Debug, PartialEq)]
pub struct NpcSkillState {
    pub script: std::sync::Arc<Script>,
    pub id: u32,
    pub sprite: u16,
    pub x: u16,
    pub y: u16,
    pub dir: u16,
    pub level: u32,
    pub stat_point: u32,
    pub parameters: [u16; 6],
    pub hp: u32,
    pub sp: u32,
    pub max_hp: u32,
    pub max_sp: u32,
    pub size: Size,
    pub speed: u16,
    pub attack_min: u16,
    pub attack_max: u16,
    pub attack_range: u16,
    pub attack_motion: i16,
    pub attack_delay: i16,
    pub damage_motion: i16,
    pub damage_immune: bool,
    pub sex: u8,
    pub looks: [u16; 12],
    pub dead_sit: u8,
    pub group_id: i32,
    pub active_statuses: Vec<StatusChange>,
    pub base_status: StatusSnapshot,
}

impl NpcSkillState {
    pub fn uninitialized(script: &Script) -> Self {
        let mut npc = Self {
            script: std::sync::Arc::new(script.clone()),
            id: script.id, sprite: script.sprite, x: script.x, y: script.y, dir: script.dir,
            level: 0, stat_point: 0, parameters: [0; 6], hp: 0, sp: 0, max_hp: 0, max_sp: 0,
            size: Size::Small, speed: 200, attack_min: 0, attack_max: 0, attack_range: 0,
            attack_motion: 0, attack_delay: 0, damage_motion: 0, damage_immune: false,
            sex: 0, looks: [0; 12], dead_sit: 0, group_id: 0, active_statuses: vec![],
            base_status: StatusSnapshot::new_for_mob(u32::from(script.sprite), 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, Size::Small, Element::Neutral, MobRace::DemiHuman, 0),
        };
        npc.base_status.set_combat_actor_kind(CombatActorKind::Npc);
        npc
    }

    pub fn new(script: &Script) -> Self {
        let mut npc = Self::uninitialized(script);
        npc.initialize_for_cast();
        npc
    }

    pub fn initialize_for_cast(&mut self) {
        if self.max_hp == 0 {
            self.hp = 1;
            self.sp = 1;
            self.max_hp = 1;
            self.max_sp = 1;
            self.attack_range = 1 + self.size as u16;
            self.base_status.set_element(Element::Neutral);
            self.base_status.set_element_level(1);
            self.base_status.set_race(MobRace::DemiHuman);
            self.base_status.set_size(self.size);
            self.base_status.set_base_speed(self.speed);
            self.base_status.set_speed(self.speed);
        }
        self.recalculate_misc();
    }

    pub fn recalculate_misc(&mut self) {
        let [str, agi, vit, int, dex, luk] = self.parameters.map(|value| u32::from(value).saturating_add(self.stat_point).min(u32::from(u16::MAX)) as u16);
        self.base_status.set_base_str(str);
        self.base_status.set_base_agi(agi);
        self.base_status.set_base_vit(vit);
        self.base_status.set_base_int(int);
        self.base_status.set_base_dex(dex);
        self.base_status.set_base_luk(luk);
        self.base_status.set_base_level(self.level);
        self.base_status.set_matk_min(Self::matk(int, 7));
        self.base_status.set_matk_max(Self::matk(int, 5));
        self.base_status.set_hit((self.level + u32::from(dex)).clamp(1, i16::MAX as u32) as i16);
        self.base_status.set_flee((self.level + u32::from(agi)).clamp(1, i16::MAX as u32) as i16);
        self.base_status.set_perfect_dodge(0.0);
        self.base_status.set_crit(0.0);
    }

    fn matk(int: u16, divisor: u16) -> u16 {
        (u32::from(int) + u32::from(int / divisor).pow(2)).min(u32::from(u16::MAX)) as u16
    }

    pub fn current_script(&self, map: &str) -> std::sync::Arc<Script> {
        let mut script = self.script.as_ref().clone();
        script.map_name = map.trim_end_matches(".gat").into();
        script.x = self.x;
        script.y = self.y;
        script.dir = self.dir;
        script.sprite = self.sprite;
        std::sync::Arc::new(script)
    }

    pub fn status(&self) -> Status {
        Status {
            combat_actor_kind: CombatActorKind::Npc, job: u32::from(self.sprite), base_level: self.level,
            hp: self.hp, sp: self.sp, max_hp: self.max_hp, max_sp: self.max_sp,
            str: self.base_status.str(), agi: self.base_status.agi(), vit: self.base_status.vit(),
            int: self.base_status.int(), dex: self.base_status.dex(), luk: self.base_status.luk(),
            speed: self.speed, size: self.size, active_statuses: self.active_statuses.clone(),
            ..Status::default()
        }
    }

    pub fn snapshot(&self) -> StatusSnapshot {
        if self.max_hp == 0 {
            return self.base_status.clone();
        }
        let status = self.status();
        let mut snapshot = self.base_status.clone();
        snapshot.set_job(u32::from(self.sprite));
        snapshot.set_combat_actor_kind(CombatActorKind::Npc);
        snapshot.set_hp(self.hp);
        snapshot.set_sp(self.sp);
        snapshot.set_max_hp(self.max_hp);
        snapshot.set_max_sp(self.max_sp);
        snapshot.set_active_statuses(self.active_statuses.clone());
        let bonuses = self.active_statuses.iter().flat_map(StatusChange::bonuses).collect::<Vec<_>>();
        for bonus in &bonuses { bonus.add_bonus_to_status(&mut snapshot); }
        StatusEffectService::adjust_status_attributes(&status, &mut snapshot);
        snapshot.set_hit((i32::from(snapshot.hit()) + i32::from(snapshot.dex()) - i32::from(self.base_status.dex())).clamp(1, i16::MAX as i32) as i16);
        snapshot.set_flee((i32::from(snapshot.flee()) + i32::from(snapshot.agi()) - i32::from(self.base_status.agi())).clamp(1, i16::MAX as i32) as i16);
        let flat_matk = bonuses.iter().filter_map(|bonus| match bonus { BonusType::Matk(value) => Some(i32::from(*value)), _ => None }).sum::<i32>();
        snapshot.set_matk_min((i32::from(snapshot.matk_min()) + i32::from(Self::matk(snapshot.int(), 7)) - i32::from(Self::matk(self.base_status.int(), 7)) + flat_matk).clamp(0, u16::MAX as i32) as u16);
        snapshot.set_matk_max((i32::from(snapshot.matk_max()) + i32::from(Self::matk(snapshot.int(), 5)) - i32::from(Self::matk(self.base_status.int(), 5)) + flat_matk).clamp(0, u16::MAX as i32) as u16);
        for bonus in &bonuses {
            if !matches!(bonus, BonusType::AspdPercentage(_)) { bonus.add_percentage_bonus_to_status(&mut snapshot); }
        }
        let (mut hp_flat, mut sp_flat, mut hp_rate, mut sp_rate) = (0, 0, if status.has_status_change(StatusChangeKind::Berserk) { 200 } else { 0 }, 0);
        for bonus in &bonuses {
            match bonus {
                BonusType::Maxhp(value) => hp_flat += i32::from(*value),
                BonusType::Maxsp(value) => sp_flat += i32::from(*value),
                BonusType::MaxhpPercentage(value) => hp_rate += i32::from(*value),
                BonusType::MaxspPercentage(value) => sp_rate += i32::from(*value),
                _ => {}
            }
        }
        snapshot.set_max_hp(StatusEffectService::maximum_pool(self.max_hp, hp_flat, hp_rate, u32::from(self.max_hp > 0)));
        snapshot.set_max_sp(StatusEffectService::maximum_pool(self.max_sp, sp_flat, sp_rate, 0));
        snapshot.set_bonuses(bonuses.into_iter().map(StatusBonus::new).collect());
        StatusEffectService::adjust_snapshot_for_target(&status, &mut snapshot, false);
        snapshot.set_hp(snapshot.hp().min(snapshot.max_hp()));
        snapshot.set_sp(snapshot.sp().min(snapshot.max_sp()));
        let attack_rate = 100 + status.status_change(StatusChangeKind::Provoke).map_or(0, |change| change.values[1])
            - if status.has_status_change(StatusChangeKind::Curse) { 25 } else { 0 }
            - status.status_change(StatusChangeKind::StripWeapon).map_or(0, |change| change.values[1]);
        snapshot.set_atk_left_side(0);
        snapshot.set_atk_right_side((i64::from(self.attack_max) * i64::from(attack_rate.max(0)) / 100).min(i32::MAX as i64) as i32);
        snapshot
    }

    fn install_status(&mut self, status: Status) {
        self.hp = status.hp;
        self.sp = status.sp;
        self.active_statuses = status.active_statuses;
        let snapshot = self.snapshot();
        self.hp = snapshot.hp();
        self.sp = snapshot.sp();
    }

    pub fn start_status(&mut self, mut request: StatusChangeRequest, tick: u128, roll: u16) -> Result<StatusChangeOutcome, String> {
        let snapshot = self.snapshot();
        let mut status = self.status();
        (status.str, status.agi, status.vit, status.int, status.dex, status.luk) = (snapshot.str(), snapshot.agi(), snapshot.vit(), snapshot.int(), snapshot.dex(), snapshot.luk());
        status.max_hp = snapshot.max_hp();
        status.max_sp = snapshot.max_sp();
        let berserk_entry = request.kind == StatusChangeKind::Berserk && !request.has_flag(StatusStartFlag::Loaded);
        let berserk_refill = berserk_entry && request.values[1] == 0;
        request = StatusEffectService::normalize_request_for_target(request, &snapshot, false);
        request = StatusEffectService::request_with_resistance(&status, &snapshot, request);
        let outcome = StatusEffectService::apply_status_for_target(&mut status, request, tick, roll, false)?;
        self.install_status(status);
        if outcome.started && berserk_entry {
            let mut status = self.status();
            StatusEffectService::finalize_berserk_entry_with_refill(&mut status, self.snapshot().max_hp(), berserk_refill);
            self.install_status(status);
        }
        Ok(outcome)
    }

    pub fn end_status(&mut self, kind: Option<StatusChangeKind>, tick: u128) -> Vec<StatusChangeKind> {
        let mut status = self.status();
        let removed = StatusEffectService::end_status_at(&mut status, kind, tick);
        self.install_status(status);
        removed
    }

    pub fn dispel(&mut self, clear_buffs: bool) -> Vec<StatusChangeKind> {
        let mut status = self.status();
        let removed = if clear_buffs { StatusEffectService::clear_buffs(&mut status) } else { StatusEffectService::dispel_statuses(&mut status, false) };
        self.install_status(status);
        removed
    }

    pub fn heal(&mut self, hp: u32, sp: u32, raw: bool) -> (u32, u32) {
        if self.hp == 0 { return (0, 0); }
        let snapshot = self.snapshot();
        let hp = if !raw && self.active_statuses.iter().any(|change| matches!(change.kind, StatusChangeKind::Berserk | StatusChangeKind::NoRecovery)) { 0 } else { hp };
        let sp = if !raw && self.active_statuses.iter().any(|change| change.kind == StatusChangeKind::NoRecovery) { 0 } else { sp };
        let (old_hp, old_sp) = (self.hp, self.sp);
        self.hp = self.hp.saturating_add(hp).min(snapshot.max_hp());
        self.sp = self.sp.saturating_add(sp).min(snapshot.max_sp());
        (self.hp - old_hp, self.sp - old_sp)
    }

    pub fn absorb_magic_rod(&mut self, flags: u32, skill_id: u32, level: u8) -> Option<u32> {
        let mut status = self.status();
        status.max_sp = self.snapshot().max_sp();
        let result = StatusEffectService::absorb_magic_rod(&mut status, flags, skill_id, level);
        self.install_status(status);
        result
    }

    pub fn damage(&mut self, amount: u32, flags: u32, skill_id: u32, versus: bool) -> u32 {
        self.admit_damage(amount, flags, skill_id, versus, |damage| damage)
    }

    pub fn admit_damage(&mut self, amount: u32, flags: u32, skill_id: u32, versus: bool, modifier: impl FnOnce(u32) -> u32) -> u32 {
        if self.damage_immune || self.hp == 0 { return 0; }
        let mut status = self.status();
        let snapshot = self.snapshot();
        status.max_hp = snapshot.max_hp();
        status.max_sp = snapshot.max_sp();
        let admitted = modifier(StatusEffectService::apply_incoming_skill_damage_flags(&mut status, amount, flags, versus, skill_id));
        let actual = admitted.min(status.hp);
        status.hp -= actual;
        self.install_status(status);
        actual
    }

    pub fn tick(&mut self, tick: u128) -> Vec<StatusChangeKind> {
        let mut status = self.status();
        status.max_hp = self.snapshot().max_hp();
        status.max_sp = self.snapshot().max_sp();
        let damage = StatusEffectService::periodic_damage_for_target(&mut status, tick, fastrand::u32(200..800), false);
        status.hp = status.hp.saturating_sub(damage);
        let mut removed = StatusEffectService::periodic_resources(&mut status, tick);
        removed.extend(StatusEffectService::expire_statuses(&mut status, tick));
        self.install_status(status);
        removed
    }

    pub fn actor(&self, map: String, instance: u8) -> ScriptSkillActor {
        let status = self.snapshot();
        let attack_rate = 100 + self.active_statuses.iter().find(|change| change.kind == StatusChangeKind::Provoke).map_or(0, |change| change.values[1])
            - if self.active_statuses.iter().any(|change| change.kind == StatusChangeKind::Curse) { 25 } else { 0 }
            - self.active_statuses.iter().find(|change| change.kind == StatusChangeKind::StripWeapon).map_or(0, |change| change.values[1]);
        let raw_attack = if status.has_status_change(StatusChangeKind::MaximizePower) { self.attack_max.max(self.attack_min) } else { fastrand::u16(self.attack_min.min(self.attack_max)..=self.attack_min.max(self.attack_max)) };
        ScriptSkillActor {
            id: self.id, credit_id: self.id, object_type: MapItemType::Npc, map, instance,
            x: self.x, y: self.y, dir: self.dir, status,
            raw_attack: (u64::from(raw_attack) * attack_rate.max(0) as u64 / 100).min(u32::MAX as u64) as u32,
            mode: MobMode::CanMove.as_flag() | MobMode::CanAttack.as_flag(),
            attack_motion: self.attack_motion.max(0) as u32,
        }
    }
}

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

use models::status::StatusSnapshot;
use movement::position::Position;

use crate::server::model::map_item::{MapItem, MapItemSnapshot, MapItemType, ToMapItem, ToMapItemSnapshot};
use crate::server::model::movement::{Movable, Movement};

/// Mob action state machine
#[derive(Clone, Debug)]
pub enum MobAction {
    /// Mob is idle, waiting for next action
    Idle,
    /// Mob is moving along a path
    Moving,
    /// Mob is chasing a target
    Chasing { target_id: u32 },
    /// Mob is attacking a target
    Attacking { target_id: u32, last_attack_at: u128 },
    /// Mob is flinching from damage (cannot move)
    Flinching { until: u128 },
    /// Mob is returning to spawn area
    Returning,
}

impl Default for MobAction {
    fn default() -> Self {
        MobAction::Idle
    }
}

pub struct MobTiming {
    /// Tick when mob can move again (after flinch/damage)
    pub canmove_tick: AtomicU64,
    /// Tick when mob can attack again (after attack animation)
    pub canattack_tick: AtomicU64,
}

impl MobTiming {
    pub fn new() -> Self {
        Self {
            canmove_tick: AtomicU64::new(0),
            canattack_tick: AtomicU64::new(0),
        }
    }

    /// Set canmove_tick (called when mob takes damage)
    pub fn set_canmove_tick(&self, tick: u128) {
        self.canmove_tick.store(tick as u64, Ordering::Release);
    }

    /// Get canmove_tick (called by mob movement thread)
    pub fn get_canmove_tick(&self) -> u128 {
        self.canmove_tick.load(Ordering::Acquire) as u128
    }

    /// Set canattack_tick (called after mob attacks)
    pub fn set_canattack_tick(&self, tick: u128) {
        self.canattack_tick.store(tick as u64, Ordering::Release);
    }

    /// Get canattack_tick (called by AI to check attack cooldown)
    pub fn get_canattack_tick(&self) -> u128 {
        self.canattack_tick.load(Ordering::Acquire) as u128
    }
}

impl Default for MobTiming {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for MobTiming {
    fn clone(&self) -> Self {
        Self {
            canmove_tick: AtomicU64::new(self.canmove_tick.load(Ordering::Relaxed)),
            canattack_tick: AtomicU64::new(self.canattack_tick.load(Ordering::Relaxed)),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ModeChange {
    Set(u32),
    Add(u32),
}

#[derive(Setters, Clone)]
pub struct Mob {
    pub id: u32,
    pub name: String,
    pub name_english: String,
    pub mob_id: i16,
    pub spawn_id: u32,
    pub dir: u16,
    pub summoned: bool,
    pub summon_owner: Option<u32>,
    pub summon_ai: u16,
    pub expires_at: Option<u128>,
    pub event_entry: Option<u32>,
    pub event_npc: Option<crate::server::model::events::map_event::ScriptNpcCallback>,
    pub next_summon_action: u128,
    pub script_cast_until: u128,
    pub last_attack_flags: u32,
    pub last_attacker_id: u32,
    pub last_credit_id: u32,
    pub last_attack_skill: u32,
    pub last_damage: u32,
    pub last_cast_skill: u32,
    pub last_cast_at: u128,
    pub last_proc_depth: u8,
    pub status: StatusSnapshot,
    pub base_status: StatusSnapshot,
    pub status_effects: models::status::Status,
    pub base_atk1: u16,
    pub base_atk2: u16,
    pub base_atk_delay: u32,
    #[set]
    pub x: u16,
    #[set]
    pub y: u16,
    pub map_view: Vec<MapItem>,
    pub is_view_char: bool,
    pub movements: Vec<Movement>,
    pub damages: HashMap<u32, u32>,
    pub actor_damages: HashMap<u32, crate::server::model::action::DamageContribution>,
    pub last_attacked_at: u128,
    pub to_remove: bool,
    pub last_moved_at: u128,
    pub damage_motion: u32,
    pub timing: MobTiming,
    pub action: MobAction,
    /// AI behavior mode flags (see MobMode enum)
    pub mode: u32,
    /// Attack range (range1 from database)
    pub attack_range: u16,
    /// Chase range (range3 from database)
    pub chase_range: u16,
    /// Attack delay in ms (time between attacks)
    pub atk_delay: u32,
    /// Attack motion duration in ms
    pub atk_motion: u32,
    /// Minimum attack damage
    pub atk1: u16,
    /// Maximum attack damage
    pub atk2: u16,
    /// Current target for passive mobs (set when attacked)
    pub target_id: Option<u32>,
    pub skill_ready_at: HashMap<usize, u128>,
    pub skill_spawn_done: bool,
    pub reborn: bool,
    mode_before_change: Option<u32>,
    pub loot_items: Vec<(i32, u16, bool)>,
    pub looted: bool,
    pub friendly_guilds: Vec<u32>,
    pub castle_owner: u32,
    pub trickcasting_until: u128,
    pub trickcasting_speed_lost: u16,
    pub bg_id: u32,
    pub damage_immune: bool,
    pub steal_flags: u8,
}

pub struct MobMovement {
    pub id: u32,
    pub from: Position,
    pub to: Position,
}

impl Movable for Mob {
    fn movements_mut(&mut self) -> &mut Vec<Movement> {
        &mut self.movements
    }

    fn movements(&self) -> &Vec<Movement> {
        &self.movements
    }

    fn set_movement(&mut self, movements: Vec<Movement>) {
        self.movements = movements;
    }
}

impl Mob {
    fn status_from_snapshot(snapshot: &StatusSnapshot) -> models::status::Status {
        models::status::Status {
            base_level: snapshot.base_level(),
            mob_class: *snapshot.mob_class(),
            mob_capabilities: snapshot.mob_capabilities(),
            combat_actor_kind: *snapshot.combat_actor_kind(),
            hp: snapshot.hp(),
            max_hp: snapshot.max_hp(),
            sp: snapshot.sp(),
            max_sp: snapshot.max_sp(),
            str: snapshot.str(),
            agi: snapshot.agi(),
            vit: snapshot.vit(),
            int: snapshot.int(),
            dex: snapshot.dex(),
            luk: snapshot.luk(),
            speed: snapshot.speed(),
            effects: snapshot.effects().clone(),
            ..Default::default()
        }
    }

    pub fn start_status(
        &mut self,
        request: models::status_change::StatusChangeRequest,
        tick: u128,
        roll: u16,
    ) -> Result<crate::server::service::status_effect_service::StatusChangeOutcome, String> {
        use models::enums::EnumWithMaskValueU32;
        use models::enums::element::Element;
        
        use models::status_change::StatusChangeKind;
        let immune = self.resists_status(&request);
        if immune {
            return Ok(crate::server::service::status_effect_service::StatusChangeOutcome {
                started: false,
                removed: vec![],
            });
        }
        self.status_effects.hp = self.status.hp();
        self.status_effects.sp = self.status.sp();
        let mut request = request;
        let upkeep_kind = request.kind;
        let berserk_entry = request.kind == StatusChangeKind::Berserk && !request.has_flag(models::status_change::StatusStartFlag::Loaded);
        let berserk_refill = berserk_entry && request.values[1] == 0;
        let fixed_upkeep_status = matches!(
            upkeep_kind,
            StatusChangeKind::Cloaking | StatusChangeKind::MaximizePower | StatusChangeKind::ChaseWalk
        );
        if request.kind == StatusChangeKind::Cloaking {
            request.values[0] = 10;
            request.values[3] |= models::status_change::CloakingFlag::AdjacentWall.as_flag() as i32
                | models::status_change::CloakingFlag::AllowSkills.as_flag() as i32;
        }
        if matches!(request.kind, StatusChangeKind::Deluge | StatusChangeKind::Volcano | StatusChangeKind::ViolentGale) && !request.has_flag(models::status_change::StatusStartFlag::Loaded) {
            let element = match request.kind { StatusChangeKind::Deluge => Element::Water, StatusChangeKind::Volcano => Element::Fire, _ => Element::Wind };
            request.values[1] = i32::from(*self.status.element() == element);
        }
        request = crate::server::service::status_effect_service::StatusEffectService::normalize_request_for_target(request, &self.status, false);
        request = crate::server::service::status_effect_service::StatusEffectService::request_with_resistance(
            &self.status_effects,
            &self.status,
            request,
        );
        if request.kind == StatusChangeKind::Poison && request.duration_ms > 0 {
            request.duration_ms /= 2;
        }
        let base_attributes = (
            self.status_effects.str,
            self.status_effects.agi,
            self.status_effects.vit,
            self.status_effects.int,
            self.status_effects.dex,
            self.status_effects.luk,
        );
        (
            self.status_effects.str,
            self.status_effects.agi,
            self.status_effects.vit,
            self.status_effects.int,
            self.status_effects.dex,
            self.status_effects.luk,
        ) = (
            self.status.str(),
            self.status.agi(),
            self.status.vit(),
            self.status.int(),
            self.status.dex(),
            self.status.luk(),
        );
        let outcome =
            crate::server::service::status_effect_service::StatusEffectService::apply_status_for_target(&mut self.status_effects, request, tick, roll, false);
        (
            self.status_effects.str,
            self.status_effects.agi,
            self.status_effects.vit,
            self.status_effects.int,
            self.status_effects.dex,
            self.status_effects.luk,
        ) = base_attributes;
        let outcome = outcome?;
        if outcome.started {
            if fixed_upkeep_status {
                if let Some(change) = self
                    .status_effects
                    .active_statuses
                    .iter_mut()
                    .find(|change| change.kind == upkeep_kind)
                {
                    change.expires_at = Some(tick + 10000);
                }
            }
            if self.blocks_movement() {
                self.movements.clear();
            }
            if self.blocks_attack() {
                self.action = MobAction::Idle;
            }
            self.recalculate_status();
            if berserk_entry {
                let max_hp = self.status.max_hp();
                crate::server::service::status_effect_service::StatusEffectService::finalize_berserk_entry_with_refill(
                    &mut self.status_effects,
                    max_hp,
                    berserk_refill,
                );
                self.recalculate_status();
            }
        }
        Ok(outcome)
    }

    /// Mirrors SC_MODECHANGE: a change that lands back on the original mode cancels the status.
    pub fn change_mode(&mut self, change: ModeChange) {
        use models::enums::EnumWithMaskValueU32;
        use models::enums::mob::MobMode;
        let original = self.mode_before_change.unwrap_or(self.mode);
        let requested = match change {
            ModeChange::Set(mode) => mode,
            ModeChange::Add(mode) => {
                let removed = if mode & MobMode::Aggressive.as_flag() == 0 { MobMode::Aggressive.as_flag() } else { 0 };
                (self.mode & !removed) | mode
            }
        };
        if requested == original || requested == self.mode && self.mode_before_change.is_some() {
            self.mode = original;
            self.mode_before_change = None;
        } else {
            self.mode_before_change = Some(original);
            self.mode = requested;
        }
    }

    pub fn resists_status(&self, request: &models::status_change::StatusChangeRequest) -> bool {
        
        use models::enums::element::Element;
        use models::enums::mob::MobRace;
        use models::status_change::{StatusChangeKind, StatusStartFlag};
        (!request.has_flag(StatusStartFlag::NoAvoid)
            && self.status.has_mob_capability(models::enums::mob::MobCapability::StatusImmune)
            && request.kind.metadata().flags.get("BossResist").copied().unwrap_or(false))
            || (*self.status.element() == Element::Undead || *self.status.race() == MobRace::RUndead)
                && matches!(
                    request.kind,
                    StatusChangeKind::Freeze | StatusChangeKind::Stone | StatusChangeKind::StoneWait
                )
    }

    pub fn end_status(&mut self, kind: Option<models::status_change::StatusChangeKind>) -> Vec<models::status_change::StatusChangeKind> {
        let removed = crate::server::service::status_effect_service::StatusEffectService::end_status(&mut self.status_effects, kind);
        if !removed.is_empty() {
            self.recalculate_status();
        }
        removed
    }

    pub fn dispel_statuses(&mut self) -> Vec<models::status_change::StatusChangeKind> {
        self.status_effects.hp = self.status.hp();
        self.status_effects.sp = self.status.sp();
        let removed = crate::server::service::status_effect_service::StatusEffectService::dispel_statuses(&mut self.status_effects, true);
        if !removed.is_empty() { self.recalculate_status(); }
        self.lose_target();
        removed
    }

    pub fn speed_up_trickcasting(&mut self, step: u16, min_speed: u16) {
        let speed = self.status.speed().saturating_sub(step).max(min_speed);
        let lost = self.status.speed().saturating_sub(speed);
        self.status.set_speed(speed);
        self.base_status.set_speed(self.base_status.speed().saturating_sub(lost));
        self.trickcasting_speed_lost = self.trickcasting_speed_lost.saturating_add(lost);
    }

    pub fn end_trickcasting(&mut self) {
        let lost = std::mem::take(&mut self.trickcasting_speed_lost);
        self.status.set_speed(self.status.speed().saturating_add(lost));
        self.base_status.set_speed(self.base_status.speed().saturating_add(lost));
        self.trickcasting_until = 0;
    }

    pub fn tick_statuses(&mut self, tick: u128) -> Vec<models::status_change::StatusChangeKind> {
        if self.trickcasting_until != 0 && tick >= self.trickcasting_until {
            self.end_trickcasting();
        }
        self.status_effects.hp = self.status.hp();
        let damage = crate::server::service::status_effect_service::StatusEffectService::periodic_damage_for_target(
            &mut self.status_effects,
            tick,
            fastrand::u32(200..800),
            false,
        );
        if damage > 0 {
            self.set_hp(self.hp().saturating_sub(damage));
        }
        let expired = crate::server::service::status_effect_service::StatusEffectService::expire_statuses(&mut self.status_effects, tick);
        if !expired.is_empty() {
            self.recalculate_status();
        }
        if self.blocks_movement() {
            self.movements.clear();
        }
        expired
    }

    pub fn blocks_movement(&self) -> bool {
        self.status_effects.blocks_movement()
    }

    pub fn blocks_attack(&self) -> bool {
        self.status_effects.blocks_attack()
    }

    pub fn absorb_magic_rod(&mut self, flags: u32, skill_id: u32, level: u8) -> Option<u32> {
        self.status_effects.hp = self.status.hp();
        self.status_effects.sp = self.status.sp();
        let absorbed = crate::server::service::status_effect_service::StatusEffectService::absorb_magic_rod(
            &mut self.status_effects,
            flags,
            skill_id,
            level,
        );
        if absorbed.is_some() {
            self.recalculate_status();
        }
        absorbed
    }

    pub fn apply_incoming_damage(&mut self, damage: u32, physical: bool) -> u32 {
        self.status_effects.hp = self.status.hp();
        let result = crate::server::service::status_effect_service::StatusEffectService::apply_incoming_damage(
            &mut self.status_effects,
            damage,
            physical,
        );
        self.recalculate_status();
        result
    }

    pub fn apply_incoming_damage_flags(&mut self, damage: u32, flags: u32) -> u32 {
        self.apply_incoming_skill_damage_flags(damage, flags, 0)
    }

    pub fn apply_incoming_skill_damage_flags(&mut self, damage: u32, flags: u32, skill_id: u32) -> u32 {
        self.status_effects.hp = self.status.hp();
        self.status_effects.sp = self.status.sp();
        let result = crate::server::service::status_effect_service::StatusEffectService::apply_incoming_skill_damage_flags(
            &mut self.status_effects,
            damage,
            flags,
            false,
            skill_id,
        );
        self.recalculate_status();
        result
    }

    fn recalculate_status(&mut self) {
        use models::enums::bonus::BonusType;
        let mut snapshot = self.base_status.clone();
        snapshot.set_hp(self.status_effects.hp);
        snapshot.set_sp(self.status_effects.sp);
        snapshot.set_active_statuses(self.status_effects.active_statuses.clone());
        snapshot.set_effects(self.status_effects.effects.clone());
        let bonuses = self
            .status_effects
            .active_statuses
            .iter()
            .flat_map(|change| change.bonuses())
            .collect::<Vec<_>>();
        for bonus in &bonuses {
            bonus.add_bonus_to_status(&mut snapshot);
        }
        crate::server::service::status_effect_service::StatusEffectService::adjust_status_attributes(&self.status_effects, &mut snapshot);
        snapshot.set_hit((i32::from(snapshot.hit()) + i32::from(snapshot.dex()) - i32::from(self.base_status.dex())).clamp(1, i16::MAX as i32) as i16);
        snapshot.set_flee((i32::from(snapshot.flee()) + i32::from(snapshot.agi()) - i32::from(self.base_status.agi())).clamp(1, i16::MAX as i32) as i16);
        let matk = |int: u16, divisor: u32| { let int = u32::from(int); int + (int / divisor).pow(2) };
        let flat_matk = bonuses.iter().filter_map(|bonus| if let BonusType::Matk(value) = bonus { Some(i32::from(*value)) } else { None }).sum::<i32>();
        snapshot.set_matk_min((i64::from(snapshot.matk_min()) + i64::from(matk(snapshot.int(), 7)) - i64::from(matk(self.base_status.int(), 7)) + i64::from(flat_matk)).clamp(0, u16::MAX as i64) as u16);
        snapshot.set_matk_max((i64::from(snapshot.matk_max()) + i64::from(matk(snapshot.int(), 5)) - i64::from(matk(self.base_status.int(), 5)) + i64::from(flat_matk)).clamp(0, u16::MAX as i64) as u16);
        for bonus in &bonuses {
            if !matches!(bonus, models::enums::bonus::BonusType::AspdPercentage(_)) { bonus.add_percentage_bonus_to_status(&mut snapshot); }
        }
        let mut hp_flat = 0_i32;
        let mut sp_flat = 0_i32;
        let mut hp_rate = if self
            .status_effects
            .has_status_change(models::status_change::StatusChangeKind::Berserk)
        {
            200
        } else {
            0
        };
        let mut sp_rate = 0_i32;
        for bonus in &bonuses {
            match bonus {
                BonusType::Maxhp(value) => hp_flat += i32::from(*value),
                BonusType::Maxsp(value) => sp_flat += i32::from(*value),
                BonusType::MaxhpPercentage(value) => hp_rate += i32::from(*value),
                BonusType::MaxspPercentage(value) => sp_rate += i32::from(*value),
                _ => {}
            }
        }
        snapshot.set_max_hp(
            crate::server::service::status_effect_service::StatusEffectService::maximum_pool(
                self.base_status.max_hp(),
                hp_flat,
                hp_rate,
                1,
            ),
        );
        snapshot.set_max_sp(
            crate::server::service::status_effect_service::StatusEffectService::maximum_pool(
                self.base_status.max_sp(),
                sp_flat,
                sp_rate,
                0,
            ),
        );
        snapshot.set_bonuses(bonuses.iter().map(|bonus| models::status_bonus::StatusBonus::new(*bonus)).collect());
        crate::server::service::status_effect_service::StatusEffectService::adjust_snapshot_for_target(
            &self.status_effects,
            &mut snapshot,
            false,
        );
        snapshot.set_hp(snapshot.hp().min(snapshot.max_hp()));
        snapshot.set_sp(snapshot.sp().min(snapshot.max_sp()));
        let mut attack_rate = 100_i32;
        let mut attack_flat = 0_i32;
        for bonus in &bonuses {
            match bonus {
                BonusType::Atk(value) => attack_flat += *value as i32,
                BonusType::AtkPercentage(value) => attack_rate += *value as i32,
                _ => {}
            }
        }
        if let Some(change) = self.status_effects.status_change(models::status_change::StatusChangeKind::Provoke) {
            attack_rate += change.values[1];
        }
        if self
            .status_effects
            .has_status_change(models::status_change::StatusChangeKind::Curse)
        {
            attack_rate -= 25;
        }
        if let Some(change) = self
            .status_effects
            .status_change(models::status_change::StatusChangeKind::StripWeapon)
        {
            attack_rate -= change.values[1];
        }
        self.atk1 = ((self.base_atk1 as i32 + attack_flat).max(0) as i64 * attack_rate.max(0) as i64 / 100).min(u16::MAX as i64) as u16;
        self.atk2 = ((self.base_atk2 as i32 + attack_flat).max(0) as i64 * attack_rate.max(0) as i64 / 100).min(u16::MAX as i64) as u16;
        self.atk_delay = (self.base_atk_delay as f32 * ((200.0 - snapshot.aspd()).max(1.0) / (200.0 - self.base_status.aspd()).max(1.0)))
            .ceil()
            .max(100.0) as u32;
        self.status_effects.hp = snapshot.hp();
        self.status_effects.sp = snapshot.sp();
        self.status = snapshot;
    }

    pub fn new(
        id: u32,
        x: u16,
        y: u16,
        mob_id: i16,
        spawn_id: u32,
        name: String,
        name_english: String,
        damage_motion: u32,
        status: StatusSnapshot,
        mode: u32,
        attack_range: u16,
        chase_range: u16,
        atk_delay: u32,
        atk_motion: u32,
        atk1: u16,
        atk2: u16,
    ) -> Mob {
        Mob {
            id,
            x,
            y,
            mob_id,
            spawn_id,
            dir: 0,
            summoned: false,
            summon_owner: None,
            summon_ai: 0,
            expires_at: None,
            event_entry: None,
            event_npc: None,
            next_summon_action: 0,
            script_cast_until: 0,
            last_attack_flags: 0,
            last_attacker_id: 0,
            last_credit_id: 0,
            last_attack_skill: 0,
            last_damage: 0,
            last_cast_skill: 0,
            last_cast_at: 0,
            last_proc_depth: 0,
            base_status: status.clone(),
            status_effects: Self::status_from_snapshot(&status),
            status,
            base_atk1: atk1,
            base_atk2: atk2,
            base_atk_delay: atk_delay,
            name,
            name_english,
            map_view: vec![],
            is_view_char: false,
            movements: vec![],
            damages: Default::default(),
            actor_damages: Default::default(),
            last_attacked_at: 0,
            to_remove: false,
            last_moved_at: 0,
            damage_motion,
            timing: MobTiming::new(),
            action: MobAction::Idle,
            mode,
            attack_range,
            chase_range,
            atk_delay,
            atk_motion,
            atk1,
            atk2,
            target_id: None,
            skill_ready_at: HashMap::new(),
            skill_spawn_done: false,
            reborn: false,
            mode_before_change: None,
            loot_items: Vec::new(),
            looted: false,
            friendly_guilds: Vec::new(),
            castle_owner: 0,
            trickcasting_until: 0,
            trickcasting_speed_lost: 0,
            bg_id: 0,
            damage_immune: false,
            steal_flags: 0,
        }
    }

    #[inline]
    pub fn x(&self) -> u16 {
        self.x
    }

    #[inline]
    pub fn y(&self) -> u16 {
        self.y
    }

    pub fn update_map_view(&mut self, map_items: Vec<MapItem>) {
        self.is_view_char = !map_items.is_empty();
        self.map_view = map_items;
    }

    pub fn update_position(&mut self, x: u16, y: u16) {
        #[cfg(feature = "debug_mob_movement")]
        {
            if crate::server::model::path::manhattan_distance(self.x, self.y, x, y) > 2 {
                error!("mob teleported old ({},{}) new ({},{})", self.x, self.y, x, y);
            }
        }
        self.face_towards(x, y);
        self.x = x;
        self.y = y;
    }

    pub fn face_towards(&mut self, x: u16, y: u16) {
        self.dir = crate::server::script::skill::ScriptSkillService::direction_to(self.x, self.y, x, y, self.dir);
    }

    pub fn add_attack(&mut self, attacker_id: u32, damage: u32) {
        if damage == 0 {
            return;
        }
        let hp = self.status.hp();
        if damage > hp {
            self.set_hp(0);
        } else {
            self.set_hp(hp - damage);
        }

        let entry = self.damages.entry(attacker_id).or_insert(0);
        *entry = entry.saturating_add(damage);
    }

    pub fn should_die(&self) -> bool {
        self.status.hp() == 0
    }

    pub fn set_hp(&mut self, hp: u32) {
        self.status.set_hp(hp);
        self.status_effects.hp = hp;
    }

    pub fn hp(&self) -> u32 {
        self.status.hp()
    }

    pub fn set_to_remove(&mut self) {
        self.to_remove = true;
    }

    pub fn is_present(&self) -> bool {
        !self.to_remove
    }

    pub fn attacker_with_higher_damage(&self) -> u32 {
        let mut higher_damage: u32 = 0;
        let mut attacker_with_higher_damage = 0;
        for (attacker_id, damage) in self.damages.iter() {
            if *damage > higher_damage {
                attacker_with_higher_damage = *attacker_id;
                higher_damage = *damage;
            }
        }
        attacker_with_higher_damage
    }

    pub fn set_last_moved_at(&mut self, tick: u128) {
        self.last_moved_at = tick;
    }

    /// Check if mob can move at the given tick (atomic check for movement
    /// thread)
    pub fn can_move(&self, tick: u128) -> bool {
        !self.status_effects.blocks_movement() && tick >= self.timing.get_canmove_tick()
    }

    // --- State machine queries ---

    pub fn is_flinching(&self) -> bool {
        matches!(self.action, MobAction::Flinching { .. })
    }

    pub fn is_idle(&self) -> bool {
        matches!(self.action, MobAction::Idle)
    }

    /// Check if mob can start a new action (not flinching)
    pub fn can_act(&self) -> bool {
        !self.is_flinching()
    }

    // --- State machine transitions ---

    /// Flinching interrupts any action
    pub fn transition_to_flinching(&mut self, tick: u128) {
        let until = tick + self.damage_motion as u128;
        self.action = MobAction::Flinching { until };
        self.timing.set_canmove_tick(until);
        self.movements.clear();
    }

    /// Can transition to Moving from: Idle only
    pub fn transition_to_moving(&mut self) -> bool {
        if matches!(self.action, MobAction::Idle) {
            self.action = MobAction::Moving;
            true
        } else {
            false
        }
    }

    /// Transition to Idle - from Moving, Flinching (after timeout)
    pub fn transition_to_idle(&mut self) -> bool {
        match self.action {
            MobAction::Moving | MobAction::Flinching { .. } => {
                self.action = MobAction::Idle;
                true
            }
            _ => false,
        }
    }

    /// Update flinch state - call each tick to check if flinch is done
    pub fn update_flinch(&mut self, tick: u128) {
        if let MobAction::Flinching { until } = self.action {
            if tick >= until {
                self.action = MobAction::Idle;
            }
        }
    }

    /// Update movement state - call when movement completes
    pub fn update_movement_complete(&mut self) {
        if matches!(self.action, MobAction::Moving) && !self.is_moving() {
            self.action = MobAction::Idle;
        }
    }

    /// Transition to Chasing from Idle or Moving
    /// Transition to Chasing from Idle, Moving, or Attacking (when target moves
    /// away)
    pub fn transition_to_chasing(&mut self, target_id: u32) -> bool {
        match self.action {
            MobAction::Idle | MobAction::Moving | MobAction::Attacking { .. } => {
                self.action = MobAction::Chasing { target_id };
                self.movements.clear();
                true
            }
            _ => false,
        }
    }

    /// Transition to Attacking from Idle or Chasing
    pub fn transition_to_attacking(&mut self, target_id: u32, tick: u128) -> bool {
        match self.action {
            MobAction::Idle | MobAction::Chasing { .. } => {
                self.action = MobAction::Attacking {
                    target_id,
                    last_attack_at: tick,
                };
                self.movements.clear();
                true
            }
            _ => false,
        }
    }

    /// Check if mob can attack at the given tick
    pub fn can_attack_at(&self, tick: u128) -> bool {
        tick >= self.timing.get_canattack_tick()
    }

    /// Update last attack time
    pub fn update_last_attack(&mut self, tick: u128) {
        if let MobAction::Attacking { last_attack_at, .. } = &mut self.action {
            *last_attack_at = tick;
        }
    }

    /// Get current target from Chasing/Attacking state or stored target_id
    pub fn get_target_id(&self) -> Option<u32> {
        match &self.action {
            MobAction::Chasing { target_id } => Some(*target_id),
            MobAction::Attacking { target_id, .. } => Some(*target_id),
            _ => self.target_id,
        }
    }

    /// Lose target - return to Idle
    pub fn lose_target(&mut self) {
        self.target_id = None;
        match self.action {
            MobAction::Chasing { .. } | MobAction::Attacking { .. } => {
                self.action = MobAction::Idle;
            }
            _ => {}
        }
    }

    /// Transition to flinching and store attacker as target for passive mobs
    pub fn transition_to_flinching_with_attacker(&mut self, attacker_id: u32, tick: u128) {
        if self.target_id.is_none() {
            self.target_id = Some(attacker_id);
        }
        self.transition_to_flinching(tick);
    }

    /// Check if mob is currently chasing
    pub fn is_chasing(&self) -> bool {
        matches!(self.action, MobAction::Chasing { .. })
    }

    /// Check if mob is currently attacking
    pub fn is_attacking(&self) -> bool {
        matches!(self.action, MobAction::Attacking { .. })
    }

    pub fn position(&self) -> Position {
        Position {
            x: self.x,
            y: self.y,
            dir: self.dir,
        }
    }
}

#[cfg(test)]
mod status_change_tests {
    use models::enums::EnumWithMaskValueU32;
    use models::enums::mob::MobMode;
    use models::status_change::{StatusChangeKind, StatusChangeRequest};

    use super::*;

    fn mob(mode: u32) -> Mob {
        let mut snapshot = crate::tests::common::mob_helper::create_test_mob_status(1000, 10, 20);
        snapshot.set_def(20);
        Mob::new(
            111,
            5,
            5,
            1002,
            1,
            "Poring".into(),
            "Poring".into(),
            0,
            snapshot,
            mode,
            1,
            9,
            1000,
            500,
            10,
            20,
        )
    }

    #[test]
    fn mode_changes_toggle_back_to_the_original_mode_and_drop_aggression_when_adding_passive_modes() {
        let original = MobMode::CanMove.as_flag() | MobMode::Aggressive.as_flag() | MobMode::CanAttack.as_flag();
        let mut mob = mob(original);
        mob.change_mode(ModeChange::Set(MobMode::CanMove.as_flag()));
        assert_eq!(mob.mode, MobMode::CanMove.as_flag());
        mob.change_mode(ModeChange::Set(MobMode::CanMove.as_flag()));
        assert_eq!(mob.mode, original);
        mob.change_mode(ModeChange::Add(MobMode::Assist.as_flag()));
        assert_eq!(mob.mode, MobMode::CanMove.as_flag() | MobMode::CanAttack.as_flag() | MobMode::Assist.as_flag());
    }

    #[test]
    fn keeping_sets_def_and_blocks_actions_and_elemental_change_rewrites_element() {
        let mut mob = mob(0);
        let keeping = StatusChangeRequest::guaranteed(StatusChangeKind::Keeping, 30_000, 1);
        assert!(mob.start_status(keeping, 0, 0).is_ok());
        assert_eq!(mob.status.def(), 90);
        assert!(mob.blocks_attack() && mob.blocks_movement());
        let mut change = StatusChangeRequest::guaranteed(StatusChangeKind::ElementalChange, 30_000, 3);
        change.values[1] = models::enums::EnumWithNumberValue::value(&models::enums::element::Element::Fire) as i32;
        assert!(mob.start_status(change, 0, 0).is_ok());
        assert_eq!(*mob.status.element(), models::enums::element::Element::Fire);
        assert_eq!(mob.status.element_level(), 3);
    }

    #[test]
    fn movement_and_attack_facing_are_retained_in_mob_snapshots() {
        let mut mob = mob(MobMode::CanMove.as_flag() | MobMode::CanAttack.as_flag());
        mob.face_towards(4, 5);
        assert_eq!(mob.position().dir, 2);
        assert_eq!(mob.to_map_item_snapshot().position.dir, 2);
        mob.update_position(6, 5);
        assert_eq!(mob.position().dir, 6);
        assert_eq!(mob.to_map_item_snapshot().position.dir, 6);
        mob.face_towards(6, 5);
        assert_eq!(mob.dir, 6);
    }

    #[test]
    fn blessing_halves_undead_and_demon_intelligence_and_dexterity_without_curing_stone() {
        for race in [models::enums::mob::MobRace::RUndead, models::enums::mob::MobRace::Demon] {
            let mut mob = mob(0);
            mob.base_status.set_race(race);
            mob.status.set_race(race);
            mob.base_status.set_base_int(80);
            mob.status.set_base_int(80);
            mob.base_status.set_base_dex(40);
            mob.status.set_base_dex(40);
            mob.base_status.set_matk_min(201);
            mob.status.set_matk_min(201);
            mob.base_status.set_matk_max(336);
            mob.status.set_matk_max(336);
            mob.base_status.set_hit(50);
            mob.status.set_hit(50);
            mob.start_status(StatusChangeRequest::guaranteed(StatusChangeKind::Blessing, 60000, 10), 0, 0).unwrap();
            assert_eq!((mob.status.int(), mob.status.dex(), mob.status.hit(), mob.status.matk_min()), (40, 20, 30, 65));
            assert_eq!(mob.status.str(), mob.base_status.str());
            assert_eq!(mob.status.status_change(StatusChangeKind::Blessing).unwrap().values[1], 0);
        }
        let mut mob = mob(0);
        mob.start_status(StatusChangeRequest::guaranteed(StatusChangeKind::Stone, 60000, 1), 0, 0).unwrap();
        mob.start_status(StatusChangeRequest::guaranteed(StatusChangeKind::Blessing, 60000, 10), 1, 0).unwrap();
        assert!(mob.status.has_status_change(StatusChangeKind::Stone));
        assert_eq!(mob.status.status_change(StatusChangeKind::Blessing).unwrap().values[1], 10);
    }

    #[test]
    fn mob_ailment_resistance_uses_buffed_attributes_without_compounding_them() {
        let mut mob = mob(0);
        mob.start_status(StatusChangeRequest::guaranteed(StatusChangeKind::IncVit, 60000, 99), 0, 0)
            .unwrap();
        assert_eq!(mob.status.vit(), 100);
        let mut stun = StatusChangeRequest::guaranteed(StatusChangeKind::Stun, 10000, 1);
        stun.flags = 0;
        assert!(!mob.start_status(stun, 0, 0).unwrap().started);
        assert_eq!(mob.status.vit(), 100);
        assert_eq!(mob.status_effects.vit, 1);
    }

    #[test]
    fn freeze_stops_mob_actions_and_expiration_restores_base_defense() {
        let mut mob = mob(MobMode::CanMove.as_flag() | MobMode::CanAttack.as_flag());
        assert!(
            mob.start_status(StatusChangeRequest::guaranteed(StatusChangeKind::Freeze, 1000, 1), 0, 0)
                .unwrap()
                .started
        );
        assert!(mob.blocks_movement());
        assert!(mob.blocks_attack());
        assert_eq!(mob.status.def(), 10);
        assert_eq!(mob.tick_statuses(1000), vec![StatusChangeKind::Freeze]);
        assert!(!mob.blocks_movement());
        assert_eq!(mob.status.def(), 20);
        assert_eq!(mob.status.hp(), 1000);
    }

    #[test]
    fn berserk_combines_hp_rates_before_rounding_and_refills_only_on_fresh_entry() {
        let mut mob = mob(MobMode::CanMove.as_flag() | MobMode::CanAttack.as_flag());
        mob.base_status.set_max_hp(256);
        mob.status.set_max_hp(256);
        mob.status.set_hp(256);
        mob.start_status(StatusChangeRequest::guaranteed(StatusChangeKind::MercHpUp, 60000, 2), 0, 0)
            .unwrap();
        assert_eq!(mob.status.max_hp(), 281);
        mob.start_status(StatusChangeRequest::guaranteed(StatusChangeKind::Berserk, 60000, 1), 0, 0)
            .unwrap();
        assert_eq!((mob.status.max_hp(), mob.hp(), mob.status.sp()), (793, 793, 0));
        assert_eq!(
            mob.status_effects.status_change(StatusChangeKind::Berserk).unwrap().values[1],
            39
        );
        mob.tick_statuses(10000);
        assert_eq!(mob.hp(), 754);
        mob.recalculate_status();
        assert_eq!((mob.hp(), mob.status.max_hp()), (754, 793));
    }

    #[test]
    fn boss_mobs_resist_full_rate_ailments_unless_noavoid_is_explicit() {
        let mut mob = mob(MobMode::Boss.as_flag());
        mob.status.set_mob_class(models::enums::mob::MobClass::Boss);
        mob.status.set_mob_capabilities(models::enums::mob::MobCapability::StatusImmune.as_flag());
        let mut request = StatusChangeRequest::guaranteed(StatusChangeKind::Freeze, 1000, 1);
        request.flags = 0;
        assert!(!mob.start_status(request, 0, 0).unwrap().started);
        assert!(mob.status_effects.active_statuses.is_empty());
        assert!(
            mob.start_status(StatusChangeRequest::guaranteed(StatusChangeKind::Freeze, 1000, 1), 0, 0)
                .unwrap()
                .started
        );
    }

    #[test]
    fn poisoned_mobs_use_non_player_periodic_damage_and_cures_stop_it() {
        let mut mob = mob(0);
        mob.start_status(StatusChangeRequest::guaranteed(StatusChangeKind::Poison, 10000, 1), 0, 0)
            .unwrap();
        mob.tick_statuses(1000);
        assert_eq!(mob.hp(), 993);
        mob.end_status(Some(StatusChangeKind::Poison));
        mob.tick_statuses(2000);
        assert_eq!(mob.hp(), 993);
    }
}

impl ToMapItem for Mob {
    fn to_map_item(&self) -> MapItem {
        MapItem::new(self.id, self.mob_id, MapItemType::Mob)
    }
}

impl ToMapItemSnapshot for Mob {
    fn to_map_item_snapshot(&self) -> MapItemSnapshot {
        MapItemSnapshot {
            map_item: self.to_map_item(),
            position: Position {
                x: self.x,
                y: self.y,
                dir: self.dir,
            },
            guild_id: 0,
            bg_id: self.bg_id,
        }
    }
}

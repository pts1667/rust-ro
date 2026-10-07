use models::enums::{EnumWithMaskValueU32, EnumWithMaskValueU64, EnumWithNumberValue};
use models::enums::item::{EquipmentLocation, ItemType};
use models::status::{Status, StatusSnapshot};
use models::status_change::{StatusChange, StatusChangeKind, StatusChangeRequest, StatusDisplayFlag, StatusHealthFlag, StatusStartFlag};
use packets::packets::{Packet, PacketZcMsgStateChange, PacketZcMsgStateChange2};
use rand::Rng;
use std::sync::mpsc::SyncSender;

use crate::server::model::events::client_notification::{AreaNotification, AreaNotificationRangeType, CharNotification, Notification};
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::status_service::StatusService;
use crate::server::state::character::Character;
use crate::server::Server;

const NPC_DEFENDER_DIVISOR: u32 = 8;
const KAIZEL_KYRIE_MS: i32 = 2000;
const KAIZEL_KYRIE_LEVEL: i32 = 10;

#[derive(Debug, PartialEq, Eq)]
pub struct StatusChangeOutcome {
    pub started: bool,
    pub removed: Vec<StatusChangeKind>,
}

pub struct StatusEffectService;

impl StatusEffectService {
    pub fn normalize_request_for_target(mut request: StatusChangeRequest, snapshot: &StatusSnapshot, player: bool) -> StatusChangeRequest {
        if request.has_flag(StatusStartFlag::Loaded) { return request; }
        match request.kind {
            StatusChangeKind::Quagmire => {
                request.values[1] = (if player { 5 } else { 10 }) * request.values[0];
            }
            StatusChangeKind::Blessing => {
                let debuff = !player && (*snapshot.race() == models::enums::mob::MobRace::RUndead
                    || *snapshot.race() == models::enums::mob::MobRace::Demon
                    || *snapshot.element() == models::enums::element::Element::Undead);
                request.values[1] = if debuff { 0 } else { request.values[0] };
            }
            _ => {}
        }
        request
    }

    pub fn adjust_status_attributes(status: &Status, snapshot: &mut StatusSnapshot) {
        if status.status_change(StatusChangeKind::Blessing).is_some_and(|change| change.values[1] == 0) {
            snapshot.set_bonus_int(snapshot.bonus_int() - (snapshot.int() / 2) as i16);
            snapshot.set_bonus_dex(snapshot.bonus_dex() - (snapshot.dex() / 2) as i16);
        }
    }

    pub fn request_with_resistance(status: &Status, snapshot: &StatusSnapshot, mut request: StatusChangeRequest) -> StatusChangeRequest {
        if request.has_flag(StatusStartFlag::NoAvoid) || request.has_flag(StatusStartFlag::NoRateReduction) { return request; }
        if let Some(ailment) = request.kind.ailment() {
            let applies = |effect: models::enums::status::StatusEffect| effect == ailment || effect == models::enums::status::StatusEffect::AllStatusEffect;
            let total = snapshot.bonuses_raw().iter().filter_map(|bonus| if let models::enums::bonus::BonusType::ResistanceToStatusPercentage(effect, amount) = bonus { applies(*effect).then_some(*amount) } else { None }).sum::<f32>();
            let active = status.active_statuses.iter().flat_map(|change| change.bonuses()).filter_map(|bonus| if let models::enums::bonus::BonusType::ResistanceToStatusPercentage(effect, amount) = bonus { applies(effect).then_some(amount) } else { None }).sum::<f32>();
            request.rate = (request.rate as f32 * (1.0 - (total - active) / 100.0).max(0.0)).clamp(0.0, u16::MAX as f32) as u16;
            if let Some(siegfried) = status.status_change(StatusChangeKind::Siegfried) {
                if matches!(request.kind, StatusChangeKind::Blind | StatusChangeKind::Stone | StatusChangeKind::Freeze | StatusChangeKind::Stun | StatusChangeKind::Curse | StatusChangeKind::Sleep | StatusChangeKind::Silence) {
                    request.rate = (i32::from(request.rate) * (100 - siegfried.values[2]).max(0) / 100).clamp(0, i32::from(u16::MAX)) as u16;
                }
            }
            if matches!(request.kind, StatusChangeKind::Freeze | StatusChangeKind::Stone | StatusChangeKind::StoneWait) {
                request.rate = (request.rate as i32 * (100 - snapshot.mdef().max(0) as i32).max(0) / 100).clamp(0, u16::MAX as i32) as u16;
            }
        }
        request
    }

    pub fn permits_equipment(status: &Status, item_type: ItemType, location: u64) -> bool {
        use StatusChangeKind::*;
        if status.active_statuses.iter().any(|change| change.kind.metadata().states.get("NoEquipItem").copied().unwrap_or(false)) { return false; }
        if item_type == ItemType::Weapon && status.has_status_change(StripWeapon) { return false; }
        if item_type != ItemType::Armor { return true; }
        ![(StripShield, EquipmentLocation::HandLeft), (StripArmor, EquipmentLocation::Armor), (StripHelm, EquipmentLocation::HeadTop)]
            .iter().any(|(kind, slot)| status.has_status_change(*kind) && location & slot.as_flag() != 0)
    }

    pub fn apply_status(status: &mut Status, request: StatusChangeRequest, tick: u128, roll: u16) -> Result<StatusChangeOutcome, String> {
        Self::apply_status_for_target(status, request, tick, roll, true)
    }

    pub fn apply_status_for_target(status: &mut Status, request: StatusChangeRequest, tick: u128, roll: u16, player: bool) -> Result<StatusChangeOutcome, String> {
        use StatusChangeKind::*;
        if request.duration_ms < -1 {
            return Err("Status duration must be nonnegative or INFINITE_TICK (-1)".into());
        }
        let mut outcome = StatusChangeOutcome { started: false, removed: vec![] };
        if player && request.kind == Blessing && !request.has_flag(StatusStartFlag::Loaded) {
            if let Some(kind) = [Curse, Stone].into_iter().find(|kind| status.has_status_change(*kind)) {
                outcome.removed = Self::end_status(status, Some(kind));
                outcome.started = true;
                return Ok(outcome);
            }
        }
        for (name, enabled) in &request.kind.metadata().end_return {
            if *enabled { if let Some(kind) = StatusChangeKind::from_name(name).filter(|kind| status.has_status_change(*kind)) { outcome.removed.extend(Self::end_status(status, Some(kind))); } }
        }
        if !outcome.removed.is_empty() { outcome.started = true; return Ok(outcome); }
        let unavoidable = request.has_flag(StatusStartFlag::NoAvoid);
        let mut duration = request.duration_ms;
        let mut rate = request.rate as i32;
        if !unavoidable && request.kind.ailment().is_some() {
            let resistance = match request.kind {
                Poison | DeadlyPoison | Stun | Silence | Bleeding => status.vit as i32 * 100,
                Sleep => status.int as i32 * 100,
                Curse => status.luk as i32 * 100,
                Blind => (status.vit as i32 + status.int as i32) * 50,
                Confusion => (status.str as i32 + status.int as i32) * 50,
                _ => 0,
            }.clamp(0, 10_000);
            if request.kind == Curse && status.luk == 0 { return Ok(outcome); }
            if !request.has_flag(StatusStartFlag::NoRateReduction) {
                let luck_resistance = if request.kind == Curse { status.luk as i32 * 10 - status.base_level as i32 * 10 } else { status.luk as i32 * 10 };
                rate = (rate - rate * resistance / 10_000 - luck_resistance).clamp(0, u16::MAX as i32);
                for active in &status.active_statuses {
                    if matches!(active.kind, CommonStatusResist | StatusResistance) {
                        rate = rate * (100 - active.values[0].clamp(0, 100)) / 100;
                    }
                }
                rate = ((rate + 9) / 10 * 10).min(u16::MAX as i32);
            }
            if duration > 0 && !request.has_flag(StatusStartFlag::NoDurationReduction) {
                let duration_resistance = match request.kind {
                    Poison | DeadlyPoison => status.vit as i32 * 75,
                    Curse => status.vit as i32 * 100,
                    Stone | StoneWait | Freeze => 0,
                    _ => resistance,
                }.clamp(0, 10_000);
                let flat = if matches!(request.kind, Poison | DeadlyPoison) { status.luk as i32 * 100 } else { status.luk as i32 * 10 };
                duration = (duration as i64 * (10_000 - duration_resistance) as i64 / 10_000 - flat as i64).max(1) as i32;
            }
        }
        if !unavoidable && (rate == 0 || (roll % 10_000) as i32 >= rate) { return Ok(outcome); }
        if request.kind == Coma {
            status.hp = 1;
            status.sp = 1;
            outcome.started = true;
            return Ok(outcome);
        }
        if request.kind == Spirit && !Self::spirit_matches_job(status, request.values[1]) { return Ok(outcome); }
        if request.kind == Spurt && !status.weapons.is_empty() { return Ok(outcome); }
        if request.kind.metadata().fail.iter().any(|(name, fails)| *fails && StatusChangeKind::from_name(name).is_some_and(|kind| status.has_status_change(kind))) && !request.has_flag(StatusStartFlag::Loaded) {
            return Ok(outcome);
        }
        if let Some(existing) = status.status_change(request.kind) {
            if (request.kind.ailment().is_some() || request.values[0] < existing.values[0]) && !request.has_flag(StatusStartFlag::Loaded) && !request.kind.metadata().flags.get("OverlapIgnoreLevel").copied().unwrap_or(false) {
                return Ok(outcome);
            }
        }
        let conflicts: &[StatusChangeKind] = match request.kind {
            IncreaseAgi => &[DecreaseAgi], DecreaseAgi => &[IncreaseAgi],
            Kyrie => &[Assumptio], Assumptio => &[Kyrie],
            Quagmire => &[IncreaseAgi, TwoHandQuicken, Adrenaline, WindWalk, Concentrate],
            Freeze | Stun | Stone => &[Dancing],
            Aspersio | FireWeapon | WaterWeapon | WindWeapon | EarthWeapon | ShadowWeapon | GhostWeapon | EnchantPoison | EnchantArms =>
                &[Aspersio, FireWeapon, WaterWeapon, WindWeapon, EarthWeapon, ShadowWeapon, GhostWeapon, EnchantPoison, EnchantArms],
            _ => &[],
        };
        for (name, ends) in &request.kind.metadata().end_on_start {
            if *ends { if let Some(kind) = StatusChangeKind::from_name(name) { outcome.removed.extend(Self::end_status(status, Some(kind))); } }
        }
        for conflict in conflicts {
            if *conflict != request.kind { outcome.removed.extend(Self::end_status(status, Some(*conflict))); }
        }
        if matches!(request.kind, AspdPotion0 | AspdPotion1 | AspdPotion2 | AspdPotion3) {
            let potions = [AspdPotion0, AspdPotion1, AspdPotion2, AspdPotion3];
            if potions.iter().any(|kind| kind.id() > request.kind.id() && status.has_status_change(*kind)) { return Ok(outcome); }
            for kind in potions { if kind != request.kind { outcome.removed.extend(Self::end_status(status, Some(kind))); } }
        }
        let mut values = request.values;
        if request.kind == JointBeat {
            if let Some(existing) = status.status_change(JointBeat) {
                if existing.values[1] as u32 & models::status_change::JointBreak::Neck.as_flag() != 0 { return Ok(outcome); }
                values[1] |= existing.values[1];
            }
        }
        if !request.has_flag(StatusStartFlag::Loaded) {
            match request.kind {
                IncreaseAgi | DecreaseAgi => values[1] = 2 + values[0],
                Endure => values[1] = 7,
                AspdPotion0 | AspdPotion1 | AspdPotion2 | AspdPotion3 => values[1] = 50 * (2 + request.kind.id() as i32 - AspdPotion0.id() as i32),
                TwoHandQuicken | MercQuicken => values[1] = 300,
                Adrenaline => values[2] = if values[1] != 0 { 300 } else { 200 },
                Overthrust => values[2] = if values[1] != 0 { 5 * values[0] } else { 5 },
                Concentrate => { values[1] = 2 + values[0]; values[2] = status.agi as i32 * values[1] / 100; values[3] = status.dex as i32 * values[1] / 100; }
                Blessing if player => values[1] = values[0],
                Quagmire => { if values[1] == 0 { values[1] = (if player { 5 } else { 10 }) * values[0]; } }
                Kyrie => { values[1] = (status.max_hp as u64 * (values[0].max(0) as u64 * 2 + 10) / 100).min(i32::MAX as u64) as i32; values[2] = values[0] / 2 + 5; }
                WindWalk => values[1] = (values[0] + 1) / 2,
                MercFleeUp | MercAttackUp | MercHitUp => values[1] = 15 * values[0],
                MercHpUp | MercSpUp => values[1] = 5 * values[0],
                MagicMirror => values[1] = 20 * (1 + (values[0].max(1) - 1) % 5),
                ExplosionSpirits => values[1] = 75 + 25 * values[0],
                AutoGuard => values[1] = (0..values[0].max(0).min(10)).map(|level| (5 - level / 2).max(1)).sum(),
                SignumCrucis => { values[1] = 10 + 4 * values[0]; duration = -1; }
                Volcano => { let level = values[0].clamp(1, 5) as usize - 1; values[1] = if values[1] != 0 { 10 * values[0] } else { 0 }; values[2] = [10, 14, 17, 19, 20][level]; }
                ViolentGale => { let level = values[0].clamp(1, 5) as usize - 1; values[1] = if values[1] != 0 { 3 * values[0] } else { 0 }; values[2] = [10, 14, 17, 19, 20][level]; }
                Deluge => { let level = values[0].clamp(1, 5) as usize - 1; values[1] = if values[1] != 0 { [5, 9, 12, 14, 15][level] } else { 0 }; values[2] = [10, 14, 17, 19, 20][level]; }
                CriticalWound => values[1] = 20 * values[0],
                HomAvoid => values[1] = if values[3] == 1 { 40 } else { 10 } * values[0],
                Longing => values[1] = 500 - 100 * values[0],
                Kaizel => values[1] = 10 * values[0],
                Kaahi => { values[1] = 200 * values[0]; values[2] = 5 * values[0]; }
                Kaite => values[1] = 1 + values[0] / 5,
                Kaupe => match values[0] {
                    1 | 2 => { values[1] = 33 * values[0]; values[2] = 1; }
                    3 => { values[1] = 100; values[2] = 1; }
                    level => { values[1] = 100; values[2] = level - 2; }
                },
                HomDefence => values[1] = 2 * values[0],
                HomChange => { values[1] = 30 * values[0]; values[2] = 20 * values[0]; }
                Fleet => { values[1] = 30 * values[0]; values[2] = 5 + 5 * values[0]; }
                Bloodlust => { values[1] = 20 + 10 * values[0]; values[2] = 9 * values[0]; values[3] = 20; }
                Defender => { values[1] = 5 + 15 * values[0]; values[2] = 50; values[3] = 250 - 50 * values[0]; }
                Parrying => values[1] = 20 + 3 * values[0],
                ReflectShield => values[1] = 10 + 3 * values[0],
                SafetyWall => values[1] = values[0] + 1,
                WeaponBreaker => values[1] = values[0] * 2 * 100,
                Provoke => { if !(values[0] == 10 && values[1] == 0 && values[2] == 100) { values[1] = 2 + 3 * values[0]; values[2] = 5 + 5 * values[0]; } }
                Regeneration => { values[1] = if values[0] == 1 { 2 } else { values[0] }; values[2] = values[0]; }
                Berserk => { values[1] = (status.max_hp as u64 * 3 * 5 / 100).min(i32::MAX as u64) as i32; values[3] = if values[3] > 0 { values[3] } else { 10000 }; }
                Spirit if values[1] == models::enums::skill_enums::SkillEnum::SlHigh.id() as i32 => {
                    let limit = status.base_level.saturating_sub(10).min(50) as i32;
                    let increase = |stat: u16| (limit - stat as i32).clamp(0, 255);
                    values[2] = increase(status.str) * 65536 + increase(status.agi) * 256 + increase(status.vit);
                    values[3] = increase(status.int) * 65536 + increase(status.dex) * 256 + increase(status.luk);
                }
                StripWeapon => values[1] = 25,
                StripArmor => values[1] = 40,
                StripShield => values[1] = 15,
                StripHelm => values[1] = 40,
                MagicRod => values[1] = 20 * values[0],
                Hiding => { values[1] = duration / 1000; values[2] = 0; values[3] = values[0] + 3; }
                Cloaking | MaximizePower => { values[1] = if duration > 0 { duration } else { 10000 }; duration = -1; }
                ChaseWalk => {
                    values[1] = if duration > 0 { duration } else { 10000 };
                    values[2] = 35 - 5 * values[0];
                    if status.status_change(Spirit).is_some_and(|change| change.values[1] == models::enums::skill_enums::SkillEnum::SlRogue.id() as i32) { values[2] -= 40; }
                    values[3] = 10 + 2 * values[0]; duration = -1;
                }
                Sight | Ruwach => {
                    let id = if values[1] > 0 { values[1] as u32 } else if request.kind == Sight { models::enums::skill_enums::SkillEnum::MgSight.id() } else { models::enums::skill_enums::SkillEnum::AlRuwach.id() };
                    values[2] = crate::server::script::skill::metadata::SkillMetadata::find(id).and_then(|skill| skill.splash(values[0].clamp(1, 255) as u8)).unwrap_or(0);
                    values[1] = duration / 20; values[3] = 0;
                }
                ArmorChange => { let antimagic = values[1] == models::enums::skill_enums::SkillEnum::NpcAntimagic.id() as i32; values[0] = 1 + (values[0].max(1) - 1) % 5; values[1] = if antimagic { -20 } else { 20 } * values[0]; values[2] = -values[1]; }
                _ => {}
            }
        }
        if matches!(request.kind, StripWeapon | StripShield | StripArmor | StripHelm) && values[3] == 1 { values[1] = 0; }
        let expires_at = if duration == -1 || request.kind == TrickDead { None } else { Some(tick.saturating_add(duration as u128)) };
        let interval = match request.kind { Sight | Ruwach => 20, Hiding | Dancing => 1000, Cloaking | ChaseWalk | MaximizePower => values[1].max(1) as u128, _ => Self::periodic_interval(request.kind) };
        let change = StatusChange { kind: request.kind, values, started_at: tick, expires_at, next_periodic_at: tick.saturating_add(interval), flags: request.flags, inherited_from: None };
        if let Some(existing) = status.active_statuses.iter_mut().find(|change| change.kind == request.kind) { *existing = change; } else { status.active_statuses.push(change); }
        if let Some(effect) = request.kind.ailment() { if !status.effects.contains(&effect) { status.effects.push(effect); } }
        if request.kind == DeadlyPoison && status.hp > status.max_hp / 4 {
            status.hp = status.hp.saturating_sub(status.max_hp / 10).max(status.max_hp / 4);
        }
        if request.kind == Berserk && !request.has_flag(StatusStartFlag::Loaded) {
            if request.values[1] == 0 { status.hp = status.max_hp.saturating_mul(3); status.sp = 0; }
            let mut endure = StatusChangeRequest::guaranteed(Endure, duration, 10); endure.values[3] = 1; endure.flags |= StatusStartFlag::NoIcon.as_flag();
            let _ = Self::apply_status(status, endure, tick, 0)?;
        }
        outcome.started = true;
        Ok(outcome)
    }

    pub fn finalize_berserk_entry(status: &mut Status, max_hp: u32) {
        Self::finalize_berserk_entry_with_refill(status, max_hp, true);
    }

    pub fn finalize_berserk_entry_with_refill(status: &mut Status, max_hp: u32, refill: bool) {
        let Some(change) = status.active_statuses.iter_mut().find(|change| change.kind == StatusChangeKind::Berserk) else { return; };
        status.max_hp = max_hp;
        if refill { status.hp = max_hp; status.sp = 0; }
        change.values[1] = (max_hp / 20).min(i32::MAX as u32) as i32;
        change.values[2] = change.expires_at.map_or(0, |expiry| expiry.saturating_sub(change.started_at) / change.values[3].max(1) as u128).min(i32::MAX as u128) as i32;
    }

    pub fn end_status(status: &mut Status, kind: Option<StatusChangeKind>) -> Vec<StatusChangeKind> {
        let previous = status.active_statuses.iter().filter(|change| kind.is_none_or(|kind| change.kind == kind)).cloned().collect::<Vec<_>>();
        let removed = status.active_statuses.iter().filter(|change| kind.is_none_or(|kind| change.kind == kind)).map(|change| change.kind).collect::<Vec<_>>();
        status.active_statuses.retain(|change| kind.is_some_and(|kind| change.kind != kind));
        if let Some(kind) = kind {
            if let Some(effect) = kind.ailment() {
                if !status.active_statuses.iter().any(|change| change.kind.ailment() == Some(effect)) { status.effects.retain(|existing| *existing != effect); }
            }
        } else { status.effects.clear(); }
        let mut removed = removed;
        if kind.is_some() {
            for change in previous {
                for (name, enabled) in &change.kind.metadata().end_on_end { if *enabled { if let Some(child) = StatusChangeKind::from_name(name).filter(|child| status.has_status_change(*child)) { removed.extend(Self::end_status(status, Some(child))); } } }
                if change.kind == StatusChangeKind::Berserk {
                    if change.values[1] > 0 && status.hp > 100 { status.hp = 100; }
                    if status.status_change(StatusChangeKind::Endure).is_some_and(|endure| endure.values[3] == 1) { removed.extend(Self::end_status(status, Some(StatusChangeKind::Endure))); }
                    if status.hp > 0 {
                        let duration = crate::server::script::skill::metadata::SkillMetadata::all().iter().find(|skill| skill.name == "LK_BERSERK").and_then(|skill| skill.duration(change.values[0].clamp(1, 255) as u8, false)).unwrap_or(0);
                        let tick = change.expires_at.unwrap_or(change.started_at);
                        let mut recovery = StatusChangeRequest::guaranteed(StatusChangeKind::Regeneration, duration, 10);
                        recovery.values[3] = (models::status_change::RegenerationBlock::Hp.as_flag() | models::status_change::RegenerationBlock::Sp.as_flag()) as i32;
                        let _ = Self::apply_status(status, recovery, tick, 0);
                    }
                }
            }
        }
        removed
    }

    pub fn end_status_at(status: &mut Status, kind: Option<StatusChangeKind>, tick: u128) -> Vec<StatusChangeKind> {
        let removed = Self::end_status(status, kind);
        if removed.contains(&StatusChangeKind::Berserk) {
            if let Some(change) = status.active_statuses.iter_mut().find(|change| change.kind == StatusChangeKind::Regeneration) {
                let duration = change.expires_at.map(|expiry| expiry.saturating_sub(change.started_at));
                change.started_at = tick; change.expires_at = duration.map(|duration| tick + duration);
            }
        }
        removed
    }

    pub fn expire_statuses(status: &mut Status, tick: u128) -> Vec<StatusChangeKind> {
        if status.has_status_change(StatusChangeKind::AutoBerserk) {
            if status.hp > 0 && status.hp < status.max_hp / 4 && !status.has_status_change(StatusChangeKind::Provoke) { let mut request = StatusChangeRequest::guaranteed(StatusChangeKind::Provoke, -1, 10); request.values[3] = 1; let _ = Self::apply_status(status, request, tick, 0); }
            else if status.hp >= status.max_hp / 4 && status.status_change(StatusChangeKind::Provoke).is_some_and(|change| change.values[3] == 1) { Self::end_status(status, Some(StatusChangeKind::Provoke)); }
        }
        if status.hp <= 100 { if let Some(change) = status.active_statuses.iter_mut().find(|change| change.kind == StatusChangeKind::Berserk) { change.expires_at = Some(tick); } }
        let petrifying = status.active_statuses.iter().filter(|change| change.kind == StatusChangeKind::StoneWait && change.expired(tick)).cloned().collect::<Vec<_>>();
        let expired = status.active_statuses.iter().filter(|change| change.expired(tick)).map(|change| change.kind).collect::<Vec<_>>();
        let mut removed = vec![];
        for kind in &expired { removed.extend(Self::end_status_at(status, Some(*kind), tick)); }
        if expired.contains(&StatusChangeKind::HomChange) { status.hp = status.hp.min(10); status.sp = status.sp.min(10); }
        for change in petrifying { let _ = Self::apply_status(status, StatusChangeRequest::guaranteed(StatusChangeKind::Stone, change.values[2].max(1), change.values[0]), tick, 0); }
        removed
    }

    pub fn spirit_matches_job(status: &Status, spirit: i32) -> bool {
        use models::enums::class::JobName::*;
        use models::enums::skill_enums::SkillEnum::*;
        use models::enums::EnumWithNumberValue;
        let Ok(job) = models::enums::class::JobName::try_from_value(status.job as usize) else { return false; };
        let Ok(spirit) = models::enums::skill_enums::SkillEnum::try_from_value(spirit as u32) else { return false; };
        match spirit {
            SlRogue => matches!(job, Rogue | Stalker | BabyRogue), SlMonk => matches!(job, Monk | Champion | BabyMonk),
            SlAssasin => matches!(job, Assassin | AssassinCross | BabyAssassin), SlAlchemist => matches!(job, Alchemist | Creator | BabyAlchemist),
            SlBarddancer => matches!(job, Bard | Dancer | Clown | Gypsy | BabyBard | BabyDancer), SlBlacksmith => matches!(job, Blacksmith | Whitesmith | BabyBlacksmith),
            SlCrusader => matches!(job, Crusader | Paladin | BabyCrusader), SlHunter => matches!(job, Hunter | Sniper | BabyHunter),
            SlKnight => matches!(job, Knight | LordKnight | BabyKnight), SlPriest => matches!(job, Priest | HighPriest | BabyPriest),
            SlSage => matches!(job, Sage | Professor | BabySage), SlSoullinker => matches!(job, SoulLinker),
            SlStar => matches!(job, StarGladiator | StarGladiatorUnion), SlSupernovice => matches!(job, SuperNovice | SuperBaby),
            SlWizard => matches!(job, Wizard | HighWizard | BabyWizard),
            SlHigh => status.base_level < 70 && matches!(job, SwordsmanHigh | MageHigh | ArcherHigh | AcolyteHigh | MerchantHigh | ThiefHigh),
            _ => false,
        }
    }

    pub fn dispel_statuses(status: &mut Status, monster: bool) -> Vec<StatusChangeKind> {
        if status.job == models::enums::class::JobName::SoulLinker.value() as u32 || status.status_change(StatusChangeKind::Spirit).is_some_and(|change| change.values[1] == models::enums::skill_enums::SkillEnum::SlRogue.id() as i32) { return vec![]; }
        let kinds = status.active_statuses.iter().filter(|change| change.kind.dispellable() && !(monster && change.kind == StatusChangeKind::Assumptio)).map(|change| change.kind).collect::<Vec<_>>();
        for kind in &kinds { if *kind == StatusChangeKind::Berserk { if let Some(change) = status.active_statuses.iter_mut().find(|change| change.kind == *kind) { change.values[1] = 0; } } Self::end_status(status, Some(*kind)); }
        kinds
    }

    pub fn remove_on_death(status: &mut Status) -> Vec<StatusChangeKind> {
        let mut removed = vec![];
        loop {
            let kinds = status.active_statuses.iter().filter(|change| change.kind.removed_by_death()).map(|change| change.kind).collect::<Vec<_>>();
            if kinds.is_empty() { break; }
            for kind in kinds { removed.extend(Self::end_status(status, Some(kind))); }
        }
        removed
    }

    pub fn clear_buffs(status: &mut Status) -> Vec<StatusChangeKind> {
        let kinds = status.active_statuses.iter().filter(|change| {
            let flags = &change.kind.metadata().flags;
            !flags.get("NoClearbuff").copied().unwrap_or(false) && !flags.get("NoClearBuff").copied().unwrap_or(false)
                && (!flags.get("Debuff").copied().unwrap_or(false) || flags.get("RemoveChemicalProtect").copied().unwrap_or(false))
        }).map(|change| change.kind).collect::<Vec<_>>();
        for kind in &kinds { if *kind == StatusChangeKind::Berserk { if let Some(change) = status.active_statuses.iter_mut().find(|change| change.kind == *kind) { change.values[1] = 0; } } Self::end_status(status, Some(*kind)); }
        kinds
    }

    pub fn periodic_damage(status: &mut Status, tick: u128, bleeding_damage: u32) -> u32 {
        Self::periodic_damage_for_target(status, tick, bleeding_damage, true)
    }

    pub fn periodic_damage_for_target(status: &mut Status, tick: u128, bleeding_damage: u32, player: bool) -> u32 {
        let slow_poison = status.has_status_change(StatusChangeKind::SlowPoison);
        let mut damage = 0_u32;
        for change in &mut status.active_statuses {
            let interval = if change.kind == StatusChangeKind::Berserk { change.values[3].max(1) as u128 } else { Self::periodic_interval(change.kind) };
            if interval == u128::MAX || tick < change.next_periodic_at || change.expired(tick) { continue; }
            let periods = (1 + (tick - change.next_periodic_at) / interval).min(100) as u32;
            change.next_periodic_at = change.next_periodic_at.saturating_add(interval * periods as u128);
            let tick_damage = match change.kind {
                StatusChangeKind::Poison if !slow_poison => 2 + status.max_hp.saturating_mul(if player { 3 } else { 1 }) / 200,
                StatusChangeKind::DeadlyPoison if !slow_poison => 2 + status.max_hp / if player { 50 } else { 100 },
                StatusChangeKind::Stone => status.max_hp / 100,
                StatusChangeKind::Bleeding => bleeding_damage,
                StatusChangeKind::Berserk => change.values[1].max(0) as u32,
                _ => 0,
            };
            let floor = if change.kind == StatusChangeKind::Bleeding { 0 } else if change.kind == StatusChangeKind::Berserk { 100 } else { status.max_hp / 4 };
            let change_damage = tick_damage.saturating_mul(periods).min(status.hp.saturating_sub(damage).saturating_sub(floor));
            damage = damage.saturating_add(change_damage);
        }
        damage
    }

    fn periodic_interval(kind: StatusChangeKind) -> u128 {
        match kind { StatusChangeKind::Poison | StatusChangeKind::DeadlyPoison => 1000, StatusChangeKind::Stone => 5000, StatusChangeKind::Bleeding | StatusChangeKind::Berserk => 10000, _ => u128::MAX }
    }

    pub fn periodic_resources(status: &mut Status, tick: u128) -> Vec<StatusChangeKind> {
        use StatusChangeKind::*;
        let rogue_spirit = status.status_change(Spirit).is_some_and(|change| change.values[1] == models::enums::skill_enums::SkillEnum::SlRogue.id() as i32);
        let mut ended = vec![];
        let mut chase_strength = None;
        for change in &mut status.active_statuses {
            let interval = match change.kind { Hiding | Dancing => 1000, Cloaking | ChaseWalk | MaximizePower => change.values[1].max(1) as u128, _ => continue };
            if tick < change.next_periodic_at || change.expired(tick) { continue; }
            let periods = (1 + (tick - change.next_periodic_at) / interval).min(100);
            for _ in 0..periods {
                let due = change.next_periodic_at;
                change.next_periodic_at = due.saturating_add(interval);
                let cost = if change.kind == Dancing {
                    change.values[2] += 1;
                    Self::dance_upkeep(change.values[0] & 0xFFFF, change.values[2])
                } else if change.kind == Hiding {
                    change.values[1] = change.values[1].saturating_sub(1);
                    if change.values[1] <= 0 { ended.push(change.kind); break; }
                    u32::from(change.values[1] % change.values[3].max(1) == 0)
                } else if change.kind == ChaseWalk { change.values[3].max(0) as u32 } else { 1 };
                if status.sp < cost { ended.push(change.kind); break; }
                status.sp -= cost;
                if change.kind == ChaseWalk {
                    let duration = crate::server::script::skill::metadata::SkillMetadata::find(models::enums::skill_enums::SkillEnum::StChasewalk.id()).and_then(|skill| skill.duration(change.values[0].clamp(1, 5) as u8, true)).unwrap_or(30000);
                    chase_strength = Some((due, StatusChangeRequest::guaranteed(ChaseWalkStrength, duration.saturating_mul(if rogue_spirit { 10 } else { 1 }), 1 << (change.values[0].clamp(1, 5) - 1))));
                }
            }
        }
        for kind in &ended { Self::end_status_at(status, Some(*kind), tick); }
        if !status.has_status_change(ChaseWalkStrength) { if let Some((due, request)) = chase_strength { let _ = Self::apply_status(status, request, due, 0); } }
        ended
    }

    /// SP the performer pays for the given second of a performance, one point every few seconds depending on the skill.
    fn dance_upkeep(skill_id: i32, second: i32) -> u32 {
        use models::enums::skill_enums::SkillEnum::*;
        let every = [
            (BdRichmankim, 3), (BdDrumbattlefield, 3), (BdRingnibelungen, 3), (BdSiegfried, 3), (BaDissonance, 3), (BaAssassincross, 3), (DcUglydance, 3),
            (BdLullaby, 4), (BdEternalchaos, 4), (BdRokisweil, 4), (DcFortunekiss, 4),
            (CgHermode, 5), (BdIntoabyss, 5), (BaWhistle, 5), (DcHumming, 5), (BaPoembragi, 5), (DcServiceforyou, 5),
            (BaAppleidun, 6), (DcDontforgetme, 10), (CgMoonlit, 10),
        ];
        let every = every.iter().find(|(skill, _)| skill.id() as i32 == skill_id).map_or(0, |(_, seconds)| *seconds);
        u32::from(every > 0 && second % every == 0)
    }

    pub fn maximum_pool(base: u32, flat: i32, rate: i32, minimum: u32) -> u32 {
        let base = (i64::from(base) + i64::from(flat)).max(i64::from(minimum)) as u64;
        (base.saturating_mul((100 + rate).max(0) as u64) / 100).clamp(u64::from(minimum), u64::from(u32::MAX)) as u32
    }

    pub fn absorb_magic_rod(status: &mut Status, flags: u32, skill_id: u32, level: u8) -> Option<u32> {
        use models::status_bonus::BattleFlag;
        if flags & BattleFlag::Magic.as_flag() == 0 || flags & BattleFlag::Skill.as_flag() == 0 { return None; }
        let metadata = crate::server::script::skill::metadata::SkillMetadata::find(skill_id)?;
        if metadata.unit.is_some() && skill_id != models::enums::skill_enums::SkillEnum::WzWaterball.id() { return None; }
        let change = status.status_change(StatusChangeKind::MagicRod)?;
        let cost = metadata.requires.as_ref().and_then(|requires| requires.get("SpCost")).and_then(|value| crate::server::script::skill::metadata::SkillMetadata::json_level_value(value, level, "Amount")).unwrap_or(0).max(0) as u32;
        let mut gain = cost.saturating_mul(change.values[1].max(0) as u32) / 100;
        if skill_id == models::enums::skill_enums::SkillEnum::WzWaterball.id() && level > 1 { gain /= u32::from(level | 1).pow(2); }
        let before = status.sp;
        status.sp = status.sp.saturating_add(gain).min(status.max_sp);
        Some(status.sp.saturating_sub(before))
    }

    fn is_fire_skill(skill_id: u32) -> bool {
        crate::server::script::skill::metadata::SkillMetadata::find(skill_id).is_some_and(|metadata| metadata.element(1) == Some("Fire"))
    }

    /// A fire hit tears one web layer; the status ends with the last one.
    fn weaken_spider_web(status: &mut Status) {
        let Some(change) = status.active_statuses.iter_mut().find(|change| change.kind == StatusChangeKind::SpiderWeb) else { return };
        change.values[0] -= 1;
        if change.values[0] <= 0 {
            Self::end_status(status, Some(StatusChangeKind::SpiderWeb));
        }
    }

    /// A weapon hit on a Kaahi holder converts SP into HP before the damage lands.
    fn kaahi_recovery(status: &mut Status) {
        let Some(change) = status.status_change(StatusChangeKind::Kaahi) else { return };
        let (heal, cost) = (change.values[1].max(0) as u32, change.values[2].max(0) as u32);
        if status.hp >= status.max_hp || status.sp < cost { return; }
        status.sp -= cost;
        status.hp = status.hp.saturating_add(heal.min(status.max_hp - status.hp));
    }

    /// Consumes the Kaizel of a character that just died and returns the HP they come back with.
    pub fn consume_kaizel(status: &mut Status, tick: u128) -> Option<u32> {
        let percent = status.status_change(StatusChangeKind::Kaizel)?.values[1].max(1) as u32;
        let revived = (u64::from(status.max_hp) * u64::from(percent) / 100).max(1) as u32;
        Self::clear_buffs(status);
        Self::end_status(status, Some(StatusChangeKind::Kaizel));
        let kyrie = StatusChangeRequest::guaranteed(StatusChangeKind::Kyrie, KAIZEL_KYRIE_MS, KAIZEL_KYRIE_LEVEL);
        let _ = Self::apply_status(status, kyrie, tick, 0);
        status.hp = revived;
        Some(revived)
    }

    pub fn apply_incoming_damage(status: &mut Status, damage: u32, physical: bool) -> u32 {
        use models::status_bonus::BattleFlag;
        let flags = if physical { BattleFlag::Weapon.as_flag() } else { BattleFlag::Magic.as_flag() };
        Self::apply_incoming_damage_flags(status, damage, flags, false)
    }

    pub fn apply_incoming_damage_flags(status: &mut Status, damage: u32, flags: u32, pvp: bool) -> u32 {
        Self::apply_incoming_skill_damage_flags(status, damage, flags, pvp, 0)
    }

    pub fn apply_incoming_skill_damage_flags(status: &mut Status, damage: u32, flags: u32, pvp: bool, skill_id: u32) -> u32 {
        Self::apply_incoming_skill_damage_with_roll(status, damage, flags, pvp, skill_id, fastrand::u8(0..100))
    }

    pub fn apply_incoming_damage_with_roll(status: &mut Status, damage: u32, flags: u32, pvp: bool, guard_roll: u8) -> u32 {
        Self::apply_incoming_skill_damage_with_roll(status, damage, flags, pvp, 0, guard_roll)
    }

    pub fn apply_incoming_skill_damage_with_roll(status: &mut Status, mut damage: u32, flags: u32, pvp: bool, skill_id: u32, guard_roll: u8) -> u32 {
        use models::status_bonus::BattleFlag;
        let physical = flags & BattleFlag::Weapon.as_flag() != 0;
        let magical = flags & BattleFlag::Magic.as_flag() != 0;
        if damage == 0 { return 0; }
        if flags == 0 || [models::enums::skill_enums::SkillEnum::PaPressure.id(), models::enums::skill_enums::SkillEnum::HwGravitation.id()].contains(&skill_id) {
            let removable = status.active_statuses.iter().filter(|change| change.kind.removed_by_damage()).map(|change| change.kind).collect::<Vec<_>>();
            for kind in removable { Self::end_status(status, Some(kind)); }
            return damage;
        }
        if status.has_status_change(StatusChangeKind::Invincible) { return 0; }
        if status.has_status_change(StatusChangeKind::Barrier) { return 1; }
        if status.has_status_change(StatusChangeKind::TrickDead) { return 0; }
        if let Some(change) = status.active_statuses.iter_mut().find(|change| change.kind == StatusChangeKind::Kaupe) {
            if (guard_roll as i32) < change.values[1] {
                change.values[2] -= 1;
                if change.values[2] <= 0 { Self::end_status(status, Some(StatusChangeKind::Kaupe)); }
                return 0;
            }
        }
        if physical { Self::kaahi_recovery(status); }
        if flags & BattleFlag::Short.as_flag() != 0 && !magical {
            if let Some(change) = status.active_statuses.iter_mut().find(|change| change.kind == StatusChangeKind::SafetyWall) {
                change.values[1] -= 1;
                if change.values[1] <= 0 { Self::end_status(status, Some(StatusChangeKind::SafetyWall)); }
                return 0;
            }
        }
        if status.has_status_change(StatusChangeKind::SpiderWeb) && Self::is_fire_skill(skill_id) {
            damage = damage.saturating_mul(2);
            Self::weaken_spider_web(status);
        }
        if status.has_status_change(StatusChangeKind::Armor) && flags & BattleFlag::Long.as_flag() != 0 && flags & (BattleFlag::Weapon.as_flag() | BattleFlag::Misc.as_flag()) != 0 { damage /= NPC_DEFENDER_DIVISOR; }
        if physical && status.status_change(StatusChangeKind::AutoGuard).is_some_and(|change| (guard_roll as i32) < change.values[1]) { return 0; }
        if physical && status.status_change(StatusChangeKind::Parrying).is_some_and(|change| (guard_roll as i32) < change.values[1]) { return 0; }
        if physical && flags & BattleFlag::Long.as_flag() != 0 && status.has_status_change(StatusChangeKind::Pneuma) { return 0; }
        if physical && flags & BattleFlag::Long.as_flag() != 0 { if let Some(change) = status.status_change(StatusChangeKind::Defender) { damage = damage.saturating_mul((100 - change.values[1]).clamp(0, 100) as u32) / 100; } }
        if let Some(change) = status.status_change(StatusChangeKind::ArmorChange) { let resistance = if physical { change.values[1] } else if magical { change.values[2] } else { 0 }; damage = (damage as u64 * (100 - resistance).max(0) as u64 / 100).min(u32::MAX as u64) as u32; }
        if status.has_status_change(StatusChangeKind::LexAeterna) { damage = damage.saturating_mul(2); Self::end_status(status, Some(StatusChangeKind::LexAeterna)); }
        if status.has_status_change(StatusChangeKind::Assumptio) { damage = if pvp { (damage as u64 * 2 / 3) as u32 } else { damage / 2 }; }
        if physical && status.has_status_change(StatusChangeKind::EnergyCoat) && status.max_sp > 0 {
            let step = ((status.sp as u64 * 100 / status.max_sp as u64).saturating_sub(1) / 20).min(4) as u32;
            let cost = (status.max_sp as u64 * (10 + 5 * step) as u64 / 1000).max(1) as u32;
            if status.sp >= cost { status.sp -= cost; damage = (damage as u64 * (100 - 6 * (step + 1)) as u64 / 100) as u32; }
            else { Self::end_status(status, Some(StatusChangeKind::EnergyCoat)); }
        }
        if let Some(change) = status.active_statuses.iter_mut().find(|change| change.kind == StatusChangeKind::Kyrie) {
            change.values[2] = change.values[2].saturating_sub(1);
            if physical || skill_id == models::enums::skill_enums::SkillEnum::TfThrowstone.id() {
                let absorbed = damage.min(change.values[1].max(0) as u32);
                change.values[1] = (i64::from(change.values[1]) - i64::from(damage)).clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32;
                damage -= absorbed;
            } else {
                change.values[1] = (i64::from(change.values[1]) - i64::from(damage)).clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32;
            }
            if change.values[1] <= 0 || change.values[2] <= 0 || skill_id == models::enums::skill_enums::SkillEnum::AlHolylight.id() { Self::end_status(status, Some(StatusChangeKind::Kyrie)); }
        }
        if damage > 0 {
            let removable = status.active_statuses.iter().filter(|change| change.kind.removed_by_damage()).map(|change| change.kind).collect::<Vec<_>>();
            for kind in removable { Self::end_status(status, Some(kind)); }
        }
        damage
    }

    pub fn start(server: &Server, character: &mut Character, mut request: StatusChangeRequest, tick: u128, sender: &SyncSender<Notification>) -> Result<bool, String> {
        let kind = request.kind;
        let pet_recovery_start = !request.has_flag(StatusStartFlag::Loaded);
        let berserk_entry = kind == StatusChangeKind::Berserk && !request.has_flag(StatusStartFlag::Loaded);
        let berserk_refill = berserk_entry && request.values[1] == 0;
        if character.status.hp == 0 && !request.has_flag(StatusStartFlag::Loaded) { return Ok(false); }
        let old_hp = character.status.hp;
        let old_sp = character.status.sp;
        let snapshot = crate::server::service::status_service::StatusService::instance().to_snapshot(&character.status);
        let mut stripped_indices = vec![];
        if !request.has_flag(StatusStartFlag::Loaded) && matches!(kind, StatusChangeKind::StripWeapon | StatusChangeKind::StripShield | StatusChangeKind::StripArmor | StatusChangeKind::StripHelm) {
            use models::enums::bonus::BonusType;
            let (protection, bonus, location, item_type) = match kind {
                StatusChangeKind::StripWeapon => (StatusChangeKind::ProtectWeapon, BonusType::UnstripableWeapon, EquipmentLocation::HandRight.as_flag() | EquipmentLocation::HandLeft.as_flag(), ItemType::Weapon),
                StatusChangeKind::StripShield => (StatusChangeKind::ProtectShield, BonusType::UnstripableShield, EquipmentLocation::HandLeft.as_flag(), ItemType::Armor),
                StatusChangeKind::StripArmor => (StatusChangeKind::ProtectArmor, BonusType::UnstripableArmor, EquipmentLocation::Armor.as_flag(), ItemType::Armor),
                _ => (StatusChangeKind::ProtectHelm, BonusType::UnstripableHelm, EquipmentLocation::HeadTop.as_flag(), ItemType::Armor),
            };
            if character.status.has_status_change(protection) || snapshot.bonuses_raw().iter().any(|value| **value == bonus || **value == BonusType::Unstripable) { return Ok(false); }
            stripped_indices = character.inventory.iter().enumerate().filter_map(|(index, item)| item.as_ref().filter(|item| item.item_type() == item_type && item.equip as u64 & location != 0).map(|_| index)).collect();
            if stripped_indices.is_empty() { return Ok(false); }
            request.values[3] = 1;
        }
        if matches!(kind, StatusChangeKind::Deluge | StatusChangeKind::Volcano | StatusChangeKind::ViolentGale) && !request.has_flag(StatusStartFlag::Loaded) {
            let element = match kind { StatusChangeKind::Deluge => models::enums::element::Element::Water, StatusChangeKind::Volcano => models::enums::element::Element::Fire, _ => models::enums::element::Element::Wind };
            request.values[1] = i32::from(*snapshot.element() == element);
        }
        request = Self::normalize_request_for_target(request, &snapshot, true);
        request = Self::request_with_resistance(&character.status, &snapshot, request);
        character.status.max_hp = snapshot.max_hp();
        character.status.max_sp = snapshot.max_sp();
        let old_stats = (character.status.str, character.status.agi, character.status.vit, character.status.int, character.status.dex, character.status.luk);
        if kind.ailment().is_some() { character.status.str = snapshot.str(); character.status.agi = snapshot.agi(); character.status.vit = snapshot.vit(); character.status.int = snapshot.int(); character.status.dex = snapshot.dex(); character.status.luk = snapshot.luk(); }
        let result = Self::apply_status(&mut character.status, request, tick, rand::thread_rng().gen_range(0..10_000));
        (character.status.str, character.status.agi, character.status.vit, character.status.int, character.status.dex, character.status.luk) = old_stats;
        let result = result?;
        for kind in &result.removed { Self::send_icon(character, *kind, false, tick, sender); }
        if result.started {
            if berserk_entry {
                let max_hp = StatusService::instance().to_snapshot(&character.status).max_hp();
                Self::finalize_berserk_entry_with_refill(&mut character.status, max_hp, berserk_refill);
            }
            for index in stripped_indices { server.inventory_service().takeoff_equip_item(character, index); }
            Self::send_icon(character, kind, true, tick, sender);
            if character.status.blocks_movement() { server.character_service().cancel_movement(character, tick); character.clear_pending_skill(); }
            if character.status.blocks_attack() || kind.metadata().flags.get("StopAttacking").copied().unwrap_or(false) { character.clear_attack(); }
            if character.status.blocks_casting() { character.clear_skill_in_use(); server.script_skill_service().cancel_queued_cast(character); }
            if old_hp != character.status.hp || old_sp != character.status.sp {
                server.character_service().update_hp_sp(character, character.status.hp, character.status.sp);
            }
            server.character_service().reload_client_side_status(character);
            Self::send_visual_status(character, sender);
            if pet_recovery_start { server.script_world_service().pet_status_started(character, kind, tick); }
        }
        Ok(result.started)
    }

    pub fn start_alternatives(server: &Server, character: &mut Character, requests: Vec<StatusChangeRequest>, tick: u128, sender: &SyncSender<Notification>) -> Result<bool, String> {
        for request in requests {
            if Self::start(server, character, request, tick, sender)? { return Ok(true); }
        }
        Ok(false)
    }

    pub fn end(server: &Server, character: &mut Character, kind: Option<StatusChangeKind>, tick: u128, sender: &SyncSender<Notification>) {
        let hp_sp = (character.status.hp, character.status.sp);
        let removed = Self::end_status_at(&mut character.status, kind, tick);
        if hp_sp != (character.status.hp, character.status.sp) { server.character_service().update_hp_sp(character, character.status.hp, character.status.sp); }
        for kind in &removed { Self::send_icon(character, *kind, false, tick, sender); }
        if !removed.is_empty() { server.character_service().reload_client_side_status(character); Self::send_visual_status(character, sender); }
        if character.status.blocks_movement() && !character.movements.is_empty() { server.character_service().cancel_movement(character, tick); }
    }

    pub fn tick(server: &Server, character: &mut Character, tick: u128, sender: &SyncSender<Notification>) {
        if let Some(effect) = crate::server::script::skill::ScriptSkillService::splasher_expiration(character.char_id, &character.status, tick) { server.add_to_next_tick(crate::server::model::events::game_event::GameEvent::CharacterScriptSkill(effect)); }
        let hp_sp = (character.status.hp, character.status.sp);
        let damage = Self::periodic_damage(&mut character.status, tick, rand::thread_rng().gen_range(200..800));
        if damage > 0 { server.character_service().take_damage(character, damage); }
        let mut removed = Self::periodic_resources(&mut character.status, tick);
        removed.extend(Self::expire_statuses(&mut character.status, tick));
        if character.status.hp == 0 && damage > 0 {
            removed.extend(Self::remove_on_death(&mut character.status));
            character.pending_item_skill = None; character.script_skill_state.casting_until = 0; character.script_skill_state.running = false;
            server.script_skill_service().cancel_queued_cast(character);
            character.movements.clear(); character.clear_attack(); character.clear_pending_skill(); character.clear_skill_in_use();
            character.action = crate::server::state::character::CharacterAction::Dead;
        }
        for kind in &removed { Self::send_icon(character, *kind, false, tick, sender); }
        if hp_sp != (character.status.hp, character.status.sp) { server.character_service().update_hp_sp(character, character.status.hp, character.status.sp); }
        if character.status.blocks_movement() && !character.movements.is_empty() { server.character_service().cancel_movement(character, tick); }
        if !removed.is_empty() { server.character_service().reload_client_side_status(character); Self::send_visual_status(character, sender); }
    }

    pub fn visual_state_packet(id: u32, status: &Status) -> Vec<u8> {
        Self::visual_state_packet_with_options(id, status, status.state)
    }

    pub fn visual_state_packet_with_options(id: u32, status: &Status, base_options: u64) -> Vec<u8> {
        use StatusChangeKind::*;
        let mut body = 0_u16;
        let mut health = 0_u16;
        let mut options = (status.state | base_options) as u32;
        for kind in [StatusDisplayFlag::Sight, StatusDisplayFlag::Hiding, StatusDisplayFlag::Cloaking, StatusDisplayFlag::ChaseWalk, StatusDisplayFlag::Orcish, StatusDisplayFlag::Wedding, StatusDisplayFlag::Ruwach, StatusDisplayFlag::Christmas, StatusDisplayFlag::Summer] { options &= !kind.as_flag(); }
        for change in &status.active_statuses {
            match change.kind {
                Stone => body = 1, Freeze => body = 2, Stun => body = 3, Sleep => body = 4,
                Poison => health |= StatusHealthFlag::Poison.as_flag() as u16,
                DeadlyPoison => health |= StatusHealthFlag::DeadlyPoison.as_flag() as u16,
                Curse => health |= StatusHealthFlag::Curse.as_flag() as u16,
                Silence => health |= StatusHealthFlag::Silence.as_flag() as u16,
                Confusion => health |= StatusHealthFlag::Confusion.as_flag() as u16,
                Blind => health |= StatusHealthFlag::Blind.as_flag() as u16,
                Angelus => health |= StatusHealthFlag::Angelus.as_flag() as u16,
                Bleeding => health |= StatusHealthFlag::Bleeding.as_flag() as u16,
                Sight => options |= StatusDisplayFlag::Sight.as_flag(), Hiding => options |= StatusDisplayFlag::Hiding.as_flag(), Cloaking => options |= StatusDisplayFlag::Cloaking.as_flag(),
                ChaseWalk => options |= StatusDisplayFlag::ChaseWalk.as_flag(),
                Orcish => options |= StatusDisplayFlag::Orcish.as_flag(), Wedding => options |= StatusDisplayFlag::Wedding.as_flag(), Ruwach => options |= StatusDisplayFlag::Ruwach.as_flag(),
                Christmas => options |= StatusDisplayFlag::Christmas.as_flag(), Summer => options |= StatusDisplayFlag::Summer.as_flag(),
                _ => {}
            }
        }
        let mut packet = Vec::with_capacity(15);
        packet.extend_from_slice(&0x0229_u16.to_le_bytes());
        packet.extend_from_slice(&id.to_le_bytes());
        packet.extend_from_slice(&body.to_le_bytes());
        packet.extend_from_slice(&health.to_le_bytes());
        packet.extend_from_slice(&options.to_le_bytes());
        packet.push(0);
        packet
    }

    pub fn send_visual_status(character: &Character, sender: &SyncSender<Notification>) {
        let packet = Self::visual_state_packet_with_options(character.char_id, &character.status, character.options);
        if let Err(error) = sender.try_send(Notification::Area(AreaNotification::new(character.current_map_name().clone(), character.current_map_instance(), AreaNotificationRangeType::Fov { x: character.x, y: character.y, exclude_id: None }, packet))) { warn!("Unable to notify status appearance: {}", error); }
    }

    pub fn send_icon(character: &Character, kind: StatusChangeKind, enabled: bool, tick: u128, sender: &SyncSender<Notification>) {
        let Some(icon) = kind.icon() else { return; };
        let packetver = GlobalConfigService::instance().packetver();
        let packet = if enabled {
            let Some(change) = character.status.status_change(kind) else { return; };
            if change.flags & StatusStartFlag::NoIcon.as_flag() != 0 { return; }
            let mut packet = PacketZcMsgStateChange2::new(packetver);
            packet.set_aid(character.char_id);
            packet.set_index(icon as i16);
            packet.set_state(true);
            packet.set_remain_ms(change.remaining_ms(tick));
            packet.fill_raw();
            packet.raw
        } else {
            let mut packet = PacketZcMsgStateChange::new(packetver);
            packet.set_aid(character.char_id);
            packet.set_index(icon as i16);
            packet.set_state(false);
            packet.fill_raw();
            packet.raw
        };
        if let Err(error) = sender.try_send(Notification::Char(CharNotification::new(character.char_id, packet))) { warn!("Unable to notify status change: {}", error); }
    }

    pub fn adjust_snapshot(status: &Status, snapshot: &mut StatusSnapshot) {
        Self::adjust_snapshot_for_target(status, snapshot, true);
    }

    pub fn adjust_snapshot_for_target(status: &Status, snapshot: &mut StatusSnapshot, player: bool) {
        use StatusChangeKind::*;
        let mut haste = 0;
        let mut slow = 0;
        let mut attack_delay_penalty = 0;
        let mut berserk_haste = 0;
        let mut quicken = 0;
        let mut potion_haste = 0;
        for change in &status.active_statuses {
            let [value, second, third, _] = change.values;
            match change.kind {
                IncreaseAgi => haste = haste.max(25), WindWalk => { haste = haste.max(2 * value); snapshot.set_flee(snapshot.flee().saturating_add(second as i16)); }
                SpeedUp0 | SpeedUp1 => haste = haste.max(value),
                Run => haste = haste.max(55),
                Agiup => haste = haste.max(value),
                Invincible => haste = haste.max(50),
                Keeping => snapshot.set_def(90),
                SpiderWeb => snapshot.set_flee(snapshot.flee() / 2),
                ElementalChange => if let Ok(element) = models::enums::element::Element::try_from_value(second as usize) { snapshot.set_element(element); snapshot.set_element_level(value.clamp(1, 4) as u8); },
                Cloaking => {
                    if change.values[3] as u32 & models::status_change::CloakingFlag::AdjacentWall.as_flag() != 0 { haste = haste.max(if value >= 10 { 25 } else { 3 * value - 3 }); }
                    else { slow = slow.max(if value < 3 { 300 } else { 30 - 3 * value }); }
                }
                ChaseWalk => { if third < 0 { haste = haste.max(-third); } else { slow = slow.max(third); } }
                Hiding => {
                    if let Some(skill) = snapshot.known_skills().iter().find(|skill| skill.value == models::enums::skill_enums::SkillEnum::RgTunneldrive && skill.level > 0) { slow = slow.max(120 - 6 * i32::from(skill.level)); }
                }
                CartBoost => haste = haste.max(20), HomAvoid => haste = haste.max(second),
                Fleet => quicken = quicken.max(second),
                Defender => attack_delay_penalty += change.values[3],
                JointBeat => {
                    use models::status_change::JointBreak;
                    let flags = second as u32;
                    if flags & JointBreak::Ankle.as_flag() != 0 { slow = slow.max(50); }
                    if flags & JointBreak::Knee.as_flag() != 0 { slow = slow.max(30 + if flags & JointBreak::Ankle.as_flag() != 0 { 50 } else { 0 }); attack_delay_penalty += 100; }
                    if flags & JointBreak::Wrist.as_flag() != 0 { attack_delay_penalty += 250; }
                }
                StripWeapon => { snapshot.set_atk_right_side((snapshot.atk_right_side() as i64 * (100 - second).max(0) as i64 / 100).clamp(0, i32::MAX as i64) as i32); }
                StripShield => snapshot.set_def((snapshot.def() as i32 * (100 - second).max(0) / 100).clamp(-32768, 32767) as i16),
                Berserk => {
                    snapshot.set_def(0); snapshot.set_mdef(0); snapshot.set_flee(snapshot.flee() / 2); snapshot.set_atk_left_side(snapshot.atk_left_side().saturating_mul(2)); snapshot.set_atk_right_side(snapshot.atk_right_side().saturating_mul(2)); berserk_haste = 300; haste = haste.max(25);
                }
                DecreaseAgi => slow = slow.max(25), Quagmire => slow = slow.max(50), Curse => { slow = slow.max(300); snapshot.set_bonus_luk(-(snapshot.base_luk() as i16)); snapshot.set_atk_left_side(snapshot.atk_left_side() * 3 / 4); snapshot.set_atk_right_side(snapshot.atk_right_side() * 3 / 4); }
                Wedding => slow = slow.max(100), SlowDown => slow = slow.max(value),
                AspdPotion0 | AspdPotion1 | AspdPotion2 | AspdPotion3 => potion_haste = potion_haste.max(second),
                TwoHandQuicken | MercQuicken => quicken = quicken.max(second), Adrenaline => quicken = quicken.max(third),
                AssnCros => quicken = quicken.max(second),
                DontForgetMe => { attack_delay_penalty += second; slow = slow.max(third); }
                Longing => { attack_delay_penalty += second; slow = slow.max(50 - 10 * value); }
                EternalChaos => snapshot.set_def(0),
                Nibelungen if !player || status.right_hand_weapon().is_some_and(|weapon| weapon.level == 4) => {
                    snapshot.set_atk_right_side(snapshot.atk_right_side().saturating_add(second));
                }
                Freeze | Stone => { snapshot.set_def(snapshot.def() / 2); snapshot.set_mdef((snapshot.mdef() as i32 * 5 / 4).clamp(-32768, 32767) as i16); snapshot.set_element(if change.kind == Freeze { models::enums::element::Element::Water } else { models::enums::element::Element::Earth }); snapshot.set_element_level(1); }
                Poison | DeadlyPoison => snapshot.set_def(snapshot.def() * 3 / 4),
                Blind => { snapshot.set_hit(snapshot.hit() * 3 / 4); snapshot.set_flee(snapshot.flee() * 3 / 4); }
                IncFleeRate => snapshot.set_flee((snapshot.flee() as i32 * (100 + value) / 100).clamp(-32768, 32767) as i16),
                Provoke if !player => snapshot.set_def((snapshot.def() as i32 * (100 - third).max(0) / 100).clamp(-32768, 32767) as i16),
                SignumCrucis => snapshot.set_def((snapshot.def() as i32 * (100 - second).max(0) / 100).clamp(-32768, 32767) as i16),
                MdefRate => snapshot.set_mdef((snapshot.mdef() as i32 * (100 + value).max(0) / 100).clamp(-32768, 32767) as i16),
                DefSet => snapshot.set_def(value.clamp(-32768, 32767) as i16), MdefSet => snapshot.set_mdef(value.clamp(-32768, 32767) as i16),
                SteelBody => { snapshot.set_def(90); snapshot.set_mdef(90); slow = slow.max(25); attack_delay_penalty += 250; }
                _ => {}
            }
        }
        let item_speed = snapshot.bonuses_raw().iter().map(|bonus| if let models::enums::bonus::BonusType::SpeedPercentage(value) = bonus { i32::from(*value) } else { 0 }).sum::<i32>();
        if item_speed > 0 {
            haste = haste.max(item_speed);
        } else {
            slow = slow.max(-item_speed);
        }
        if player && matches!(models::enums::class::JobName::try_from_value(status.job as usize), Ok(models::enums::class::JobName::Assassin | models::enums::class::JobName::AssassinCross | models::enums::class::JobName::BabyAssassin)) {
            haste = haste.max(i32::from(snapshot.known_skill_level(models::enums::skill_enums::SkillEnum::TfMiss)));
        }
        if player && snapshot.state() & models::enums::skill::SkillState::Riding.as_flag() != 0 {
            haste = haste.max(25);
            let cavalier_mastery = i32::from(snapshot.known_skill_level(models::enums::skill_enums::SkillEnum::KnCavaliermastery));
            attack_delay_penalty += 500 - 100 * cavalier_mastery;
        }
        if status.active_statuses.iter().any(|change| matches!(change.kind, Freeze | Stun | Sleep | Stone)) { snapshot.set_flee(0); }
        snapshot.set_speed((snapshot.speed() as i64 * (100 + slow - haste).max(40) as i64 / 100).clamp(10, u16::MAX as i64) as u16);
        if status.has_status_change(Defender) || status.has_status_change(Armor) { snapshot.set_speed(snapshot.speed().max(200)); }
        if status.has_status_change(SteelBody) { snapshot.set_speed(200); }
        if let Some(change) = status.status_change(WalkSpeed).filter(|change| change.values[0] > 0) { snapshot.set_speed((snapshot.speed() as u32 * 100 / change.values[0] as u32).clamp(10, u16::MAX as u32) as u16); }
        if player {
            let fastest = (15_000 / GlobalConfigService::battle_option("max_walk_speed").max(1)) as u16;
            snapshot.set_speed(snapshot.speed().max(fastest));
        }
        let bonus_haste = snapshot.bonuses_raw().iter().filter_map(|bonus| {
            if let models::enums::bonus::BonusType::AspdPercentage(value) = bonus { Some(*value * 10.0) } else { None }
        }).sum::<f32>();
        let attack_delay_rate = (1000.0 - bonus_haste - quicken as f32 - potion_haste as f32 - berserk_haste as f32 + attack_delay_penalty as f32).max(0.0);
        snapshot.set_aspd(200.0 - (200.0 - snapshot.aspd()) * attack_delay_rate / 1000.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use models::enums::status::StatusEffect;

    fn status() -> Status { Status { hp: 10_000, max_hp: 10_000, sp: 100, max_sp: 100, vit: 1, int: 1, agi: 40, dex: 60, luk: 1, ..Status::default() } }

    #[test]
    fn blessing_cures_ailments_and_refreshes_without_stacking() {
        let mut status = status();
        start_for_test(&mut status, StatusChangeKind::Curse, 10000, 1, 100);
        start_for_test(&mut status, StatusChangeKind::Blessing, 1000, 10, 200);
        start_for_test(&mut status, StatusChangeKind::Blessing, 2000, 10, 300);
        assert!(!status.effects.contains(&StatusEffect::Curse));
        assert_eq!(status.active_statuses.len(), 1);
        assert_eq!(status.active_statuses[0].expires_at, Some(2300));
        assert_eq!(StatusEffectService::expire_statuses(&mut status, 2299), vec![]);
        assert_eq!(StatusEffectService::expire_statuses(&mut status, 2300), vec![StatusChangeKind::Blessing]);
    }

    #[test]
    fn npc_barrier_invincible_and_defender_shape_incoming_damage() {
        use models::status_bonus::BattleFlag;
        let melee = BattleFlag::Weapon.as_flag() | BattleFlag::Short.as_flag();
        let ranged = BattleFlag::Weapon.as_flag() | BattleFlag::Long.as_flag();
        let mut status = status();
        start_for_test(&mut status, StatusChangeKind::Armor, 10000, 1, 0);
        assert_eq!(StatusEffectService::apply_incoming_skill_damage_with_roll(&mut status, 800, ranged, false, 0, 99), 100);
        assert_eq!(StatusEffectService::apply_incoming_skill_damage_with_roll(&mut status, 800, melee, false, 0, 99), 800);
        start_for_test(&mut status, StatusChangeKind::Barrier, 10000, 1, 0);
        assert_eq!(StatusEffectService::apply_incoming_skill_damage_with_roll(&mut status, 800, melee, false, 0, 99), 1);
        start_for_test(&mut status, StatusChangeKind::Invincible, 10000, 1, 0);
        assert_eq!(StatusEffectService::apply_incoming_skill_damage_with_roll(&mut status, 800, melee, false, 0, 99), 0);
    }

    #[test]
    fn fire_skills_double_damage_on_a_spider_web_and_tear_one_layer() {
        use models::enums::skill_enums::SkillEnum;
        use models::status_bonus::BattleFlag;
        let magic = BattleFlag::Magic.as_flag() | BattleFlag::Long.as_flag() | BattleFlag::Skill.as_flag();
        let mut status = status();
        start_for_test(&mut status, StatusChangeKind::SpiderWeb, 10000, 2, 0);
        let cold = SkillEnum::MgColdbolt.id();
        let fire = SkillEnum::MgFirebolt.id();
        assert_eq!(StatusEffectService::apply_incoming_skill_damage_with_roll(&mut status, 100, magic, false, cold, 99), 100);
        assert_eq!(StatusEffectService::apply_incoming_skill_damage_with_roll(&mut status, 100, magic, false, fire, 99), 200);
        assert_eq!(status.status_change(StatusChangeKind::SpiderWeb).map(|change| change.values[0]), Some(1));
        assert_eq!(StatusEffectService::apply_incoming_skill_damage_with_roll(&mut status, 100, magic, false, fire, 99), 200);
        assert!(!status.has_status_change(StatusChangeKind::SpiderWeb));
        assert_eq!(StatusEffectService::apply_incoming_skill_damage_with_roll(&mut status, 100, magic, false, fire, 99), 100);
    }

    #[test]
    fn safety_wall_blocks_melee_hits_by_level_but_not_magic_or_ranged() {
        use models::status_bonus::BattleFlag;
        let melee = BattleFlag::Weapon.as_flag() | BattleFlag::Short.as_flag();
        let magic = BattleFlag::Magic.as_flag() | BattleFlag::Long.as_flag();
        let mut status = status();
        start_for_test(&mut status, StatusChangeKind::SafetyWall, 10000, 2, 0);
        assert_eq!(StatusEffectService::apply_incoming_skill_damage_with_roll(&mut status, 100, magic, false, 0, 99), 100);
        for _ in 0..3 {
            assert_eq!(StatusEffectService::apply_incoming_skill_damage_with_roll(&mut status, 100, melee, false, 0, 99), 0);
        }
        assert!(!status.has_status_change(StatusChangeKind::SafetyWall));
        assert_eq!(StatusEffectService::apply_incoming_skill_damage_with_roll(&mut status, 100, melee, false, 0, 99), 100);
    }

    #[test]
    fn status_rolls_and_flags_apply_without_consuming_on_failed_chance() {
        let mut status = status();
        let mut request = StatusChangeRequest::guaranteed(StatusChangeKind::Poison, 10000, 1);
        request.flags = StatusStartFlag::NoDurationReduction.as_flag();
        request.rate = 2500;
        assert!(!StatusEffectService::apply_status(&mut status, request.clone(), 100, 9999).unwrap().started);
        assert!(StatusEffectService::apply_status(&mut status, request, 100, 0).unwrap().started);
    }

    #[test]
    fn poison_damage_stops_at_quarter_health_and_cure_removes_ticking() {
        let mut status = status();
        start_for_test(&mut status, StatusChangeKind::Poison, 100000, 1, 0);
        let damage = StatusEffectService::periodic_damage(&mut status, 1000, 400);
        assert_eq!(damage, 152);
        status.hp = 2600;
        assert_eq!(StatusEffectService::periodic_damage(&mut status, 2000, 400), 100);
        StatusEffectService::end_status(&mut status, Some(StatusChangeKind::Poison));
        assert_eq!(StatusEffectService::periodic_damage(&mut status, 3000, 400), 0);
    }

    #[test]
    fn higher_attack_speed_potions_replace_and_resist_weaker_potions() {
        let mut status = status();
        start_for_test(&mut status, StatusChangeKind::AspdPotion0, 10000, 0, 0);
        start_for_test(&mut status, StatusChangeKind::AspdPotion2, 10000, 0, 100);
        start_for_test(&mut status, StatusChangeKind::AspdPotion1, 10000, 0, 200);
        assert_eq!(status.active_statuses.len(), 1);
        assert_eq!(status.active_statuses[0].kind, StatusChangeKind::AspdPotion2);
        assert_eq!(status.active_statuses[0].values[1], 200);
    }

    #[test]
    fn kyrie_absorbs_damage_and_disappears_after_capacity_or_hit_count() {
        let mut status = status();
        start_for_test(&mut status, StatusChangeKind::Kyrie, 10000, 10, 0);
        assert_eq!(StatusEffectService::apply_incoming_damage(&mut status, 2000, true), 0);
        assert_eq!(StatusEffectService::apply_incoming_damage(&mut status, 2000, true), 1000);
        assert!(!status.has_status_change(StatusChangeKind::Kyrie));
    }

    #[test]
    fn berserk_fills_pools_drains_health_and_blocks_recovery_after_ending() {
        let mut status = status();
        start_for_test(&mut status, StatusChangeKind::Berserk, 30000, 1, 0);
        assert_eq!((status.hp, status.sp), (30000, 0));
        assert_eq!(StatusEffectService::periodic_damage(&mut status, 10000, 400), 1500);
        status.hp = 101;
        assert_eq!(StatusEffectService::periodic_damage(&mut status, 20000, 400), 1);
        status.hp = 100;
        let ended = StatusEffectService::expire_statuses(&mut status, 20000);
        assert!(ended.contains(&StatusChangeKind::Berserk));
        assert!(!status.has_status_change(StatusChangeKind::Endure));
        assert!(status.has_status_change(StatusChangeKind::Regeneration));
        assert!(status.status_change(StatusChangeKind::Regeneration).unwrap().bonuses().contains(&models::enums::bonus::BonusType::DisableSpRegen));
    }

    #[test]
    fn stripping_blocks_only_the_affected_equipment_type_and_slot() {
        let mut status = status();
        start_for_test(&mut status, StatusChangeKind::StripShield, 30000, 1, 0);
        assert!(!StatusEffectService::permits_equipment(&status, ItemType::Armor, EquipmentLocation::HandLeft.as_flag()));
        assert!(StatusEffectService::permits_equipment(&status, ItemType::Weapon, EquipmentLocation::HandLeft.as_flag()));
        assert!(StatusEffectService::permits_equipment(&status, ItemType::Armor, EquipmentLocation::HeadTop.as_flag()));
        start_for_test(&mut status, StatusChangeKind::StripWeapon, 30000, 1, 0);
        assert!(!StatusEffectService::permits_equipment(&status, ItemType::Weapon, EquipmentLocation::HandLeft.as_flag()));
        assert!(!StatusEffectService::permits_equipment(&status, ItemType::Weapon, EquipmentLocation::HandRight.as_flag()));
        StatusEffectService::expire_statuses(&mut status, 30000);
        assert!(StatusEffectService::permits_equipment(&status, ItemType::Weapon, EquipmentLocation::HandRight.as_flag()));
    }

    #[test]
    fn petrification_wait_allows_movement_then_stone_stops_it_and_a_second_cast_cures_it() {
        let mut status = status();
        let mut request = StatusChangeRequest::guaranteed(StatusChangeKind::StoneWait, 1000, 5);
        request.values[2] = 10000;
        assert!(StatusEffectService::apply_status(&mut status, request.clone(), 0, 0).unwrap().started);
        assert!(!status.blocks_movement());
        assert!(status.blocks_casting());
        StatusEffectService::expire_statuses(&mut status, 1000);
        assert!(status.has_status_change(StatusChangeKind::Stone));
        assert!(status.blocks_movement());
        let result = StatusEffectService::apply_status(&mut status, request, 1001, 0).unwrap();
        assert!(result.started);
        assert_eq!(result.removed, vec![StatusChangeKind::Stone]);
        assert!(!status.has_status_change(StatusChangeKind::StoneWait));
    }

    #[test]
    fn cloaking_drains_sp_at_its_interval_and_ends_only_when_the_next_charge_fails() {
        let mut status = Status { hp: 100, sp: 2, ..Status::default() };
        start_for_test(&mut status, StatusChangeKind::Cloaking, 500, 1, 0);
        assert_eq!(status.status_change(StatusChangeKind::Cloaking).unwrap().expires_at, None);
        assert!(StatusEffectService::periodic_resources(&mut status, 499).is_empty());
        assert_eq!(status.sp, 2);
        assert!(StatusEffectService::periodic_resources(&mut status, 500).is_empty());
        assert_eq!(status.sp, 1);
        assert!(StatusEffectService::periodic_resources(&mut status, 1000).is_empty());
        assert_eq!(status.sp, 0);
        assert_eq!(StatusEffectService::periodic_resources(&mut status, 1500), vec![StatusChangeKind::Cloaking]);
        assert!(!status.has_status_change(StatusChangeKind::Cloaking));
    }

    #[test]
    fn hiding_uses_remaining_seconds_for_its_charge_and_blocks_movement_without_tunnel_drive() {
        let mut status = Status { hp: 100, sp: 2, ..Status::default() };
        start_for_test(&mut status, StatusChangeKind::Hiding, 30000, 1, 0);
        assert!(status.blocks_movement());
        StatusEffectService::periodic_resources(&mut status, 1000);
        assert_eq!(status.sp, 2);
        StatusEffectService::periodic_resources(&mut status, 2000);
        assert_eq!(status.sp, 1);
        StatusEffectService::periodic_resources(&mut status, 6000);
        assert_eq!(status.sp, 0);
        assert_eq!(StatusEffectService::periodic_resources(&mut status, 10000), vec![StatusChangeKind::Hiding]);
        assert!(!status.blocks_movement());
    }

    #[test]
    fn chase_walk_strength_starts_after_upkeep_and_survives_the_stealth_status() {
        let mut status = Status { hp: 100, sp: 16, ..Status::default() };
        start_for_test(&mut status, StatusChangeKind::ChaseWalk, 10000, 3, 0);
        assert!(!status.has_status_change(StatusChangeKind::ChaseWalkStrength));
        StatusEffectService::periodic_resources(&mut status, 10000);
        assert_eq!(status.sp, 0);
        let strength = status.status_change(StatusChangeKind::ChaseWalkStrength).unwrap();
        assert_eq!((strength.values[0], strength.expires_at), (4, Some(40000)));
        assert_eq!(StatusEffectService::periodic_resources(&mut status, 20000), vec![StatusChangeKind::ChaseWalk]);
        assert!(status.has_status_change(StatusChangeKind::ChaseWalkStrength));
        StatusEffectService::expire_statuses(&mut status, 40000);
        assert!(!status.has_status_change(StatusChangeKind::ChaseWalkStrength));
    }

    #[test]
    fn magic_rod_absorbs_direct_magic_and_water_ball_but_allows_ground_units() {
        use models::enums::skill_enums::SkillEnum;
        use models::status_bonus::BattleFlag;
        let mut status = Status { hp: 100, sp: 0, max_sp: 100, ..Status::default() };
        start_for_test(&mut status, StatusChangeKind::MagicRod, 1200, 5, 0);
        let flags = BattleFlag::Magic.as_flag() | BattleFlag::Long.as_flag() | BattleFlag::Skill.as_flag();
        assert_eq!(StatusEffectService::absorb_magic_rod(&mut status, flags, SkillEnum::MgFirebolt.id(), 1), Some(12));
        assert_eq!(status.sp, 12);
        assert_eq!(StatusEffectService::absorb_magic_rod(&mut status, flags, SkillEnum::WzWaterball.id(), 5), Some(1));
        assert_eq!(status.sp, 13);
        assert_eq!(StatusEffectService::absorb_magic_rod(&mut status, flags, SkillEnum::WzStormgust.id(), 10), None);
        assert_eq!(status.sp, 13);
    }

    #[test]
    fn blessing_cures_curse_before_stone_and_only_then_grants_stats() {
        let mut status = Status { hp: 100, ..Default::default() };
        start_for_test(&mut status, StatusChangeKind::Curse, 60000, 1, 0);
        start_for_test(&mut status, StatusChangeKind::Stone, 60000, 1, 0);
        let request = StatusChangeRequest::guaranteed(StatusChangeKind::Blessing, 60000, 10);
        let first = StatusEffectService::apply_status(&mut status, request.clone(), 1, 0).unwrap();
        assert_eq!(first.removed, vec![StatusChangeKind::Curse]);
        assert!(status.has_status_change(StatusChangeKind::Stone));
        assert!(!status.has_status_change(StatusChangeKind::Blessing));
        let second = StatusEffectService::apply_status(&mut status, request.clone(), 2, 0).unwrap();
        assert_eq!(second.removed, vec![StatusChangeKind::Stone]);
        assert!(!status.has_status_change(StatusChangeKind::Blessing));
        StatusEffectService::apply_status(&mut status, request, 3, 0).unwrap();
        assert_eq!(status.status_change(StatusChangeKind::Blessing).unwrap().values[1], 10);
    }

    #[test]
    fn quagmire_reduces_player_stats_by_five_and_other_actors_by_ten_per_level() {
        for (player, decrease) in [(true, 25), (false, 50)] {
            let mut status = Status { hp: 100, agi: 12, dex: 16, ..Default::default() };
            StatusEffectService::apply_status_for_target(&mut status,
                StatusChangeRequest::guaranteed(StatusChangeKind::Quagmire, 60000, 5), 0, 0, player,
            ).unwrap();
            let change = status.status_change(StatusChangeKind::Quagmire).unwrap();
            assert_eq!(change.values[1..3], [decrease, 0]);
            assert_eq!(change.bonuses(), vec![models::enums::bonus::BonusType::Agi(-decrease as i8), models::enums::bonus::BonusType::Dex(-decrease as i8)]);
        }
    }

    #[test]
    fn loaded_quagmire_uses_the_saved_second_value_for_both_stats_and_preserves_other_values() {
        let mut status = Status { hp: 100, ..Default::default() };
        let mut request = StatusChangeRequest::guaranteed(StatusChangeKind::Quagmire, 60000, 3);
        request.flags |= StatusStartFlag::Loaded.as_flag();
        request.values = [3, 17, 99, 4];
        StatusEffectService::apply_status_for_target(&mut status, request, 0, 0, false).unwrap();
        let change = status.status_change(StatusChangeKind::Quagmire).unwrap();
        assert_eq!(change.values, [3, 17, 99, 4]);
        assert_eq!(change.bonuses(), vec![models::enums::bonus::BonusType::Agi(-17), models::enums::bonus::BonusType::Dex(-17)]);
    }

    #[test]
    fn pressure_bypasses_lex_assumptio_and_kyrie_without_consuming_them() {
        use models::enums::skill_enums::SkillEnum;
        use models::status_bonus::BattleFlag;
        let flags = BattleFlag::Misc.as_flag() | BattleFlag::Long.as_flag() | BattleFlag::Skill.as_flag();
        for kind in [StatusChangeKind::LexAeterna, StatusChangeKind::Assumptio, StatusChangeKind::Kyrie] {
            let mut status = Status { hp: 1000, max_hp: 1000, ..Default::default() };
            start_for_test(&mut status, kind, 60000, 10, 0);
            let before = status.status_change(kind).unwrap().clone();
            assert_eq!(StatusEffectService::apply_incoming_skill_damage_with_roll(&mut status, 800, flags, false, SkillEnum::PaPressure.id(), 99), 800);
            assert_eq!(status.status_change(kind), Some(&before));
        }
    }

    #[test]
    fn kyrie_loses_its_pool_on_magic_and_blocks_stone_fling_and_holy_light_ends_it() {
        use models::enums::skill_enums::SkillEnum;
        use models::status_bonus::BattleFlag;
        let magic = BattleFlag::Magic.as_flag() | BattleFlag::Long.as_flag() | BattleFlag::Skill.as_flag();
        let misc = BattleFlag::Misc.as_flag() | BattleFlag::Long.as_flag() | BattleFlag::Skill.as_flag();
        let mut status = Status { hp: 1000, max_hp: 1000, ..Default::default() };
        start_for_test(&mut status, StatusChangeKind::Kyrie, 60000, 10, 0);
        assert_eq!(StatusEffectService::apply_incoming_skill_damage_with_roll(&mut status, 50, magic, false, SkillEnum::MgFirebolt.id(), 99), 50);
        assert_eq!(status.status_change(StatusChangeKind::Kyrie).unwrap().values[1..3], [250, 9]);
        assert_eq!(StatusEffectService::apply_incoming_skill_damage_with_roll(&mut status, 50, misc, false, SkillEnum::TfThrowstone.id(), 99), 0);
        assert_eq!(status.status_change(StatusChangeKind::Kyrie).unwrap().values[1..3], [200, 8]);
        assert_eq!(StatusEffectService::apply_incoming_skill_damage_with_roll(&mut status, 10, magic, false, SkillEnum::AlHolylight.id(), 99), 10);
        assert!(!status.has_status_change(StatusChangeKind::Kyrie));
    }

    #[test]
    fn attack_speed_rates_add_quicken_berserk_potion_and_bonus_before_scaling_delay() {
        let mut status = Status { hp: 1000, max_hp: 1000, ..Default::default() };
        for kind in [StatusChangeKind::TwoHandQuicken, StatusChangeKind::AspdPotion2, StatusChangeKind::Berserk] {
            start_for_test(&mut status, kind, 60000, 1, 0);
        }
        let mut snapshot = StatusSnapshot::_from(&status);
        snapshot.set_aspd(150.0);
        snapshot.set_bonuses(vec![models::status_bonus::StatusBonus::new(models::enums::bonus::BonusType::AspdPercentage(15.0))]);
        StatusEffectService::adjust_snapshot(&status, &mut snapshot);
        assert_eq!(snapshot.aspd(), 197.5);
    }

    #[test]
    fn attack_speed_penalties_apply_independently_of_status_start_order() {
        for order in [[StatusChangeKind::Defender, StatusChangeKind::TwoHandQuicken], [StatusChangeKind::TwoHandQuicken, StatusChangeKind::Defender]] {
            let mut status = Status { hp: 1000, max_hp: 1000, ..Default::default() };
            for kind in order { start_for_test(&mut status, kind, 60000, 1, 0); }
            let mut snapshot = StatusSnapshot::_from(&status);
            snapshot.set_aspd(150.0);
            snapshot.set_bonuses(vec![models::status_bonus::StatusBonus::new(models::enums::bonus::BonusType::AspdPercentage(15.0))]);
            StatusEffectService::adjust_snapshot(&status, &mut snapshot);
            assert_eq!(snapshot.aspd(), 162.5);
        }
    }

    fn start_for_test(status: &mut Status, kind: StatusChangeKind, duration: i32, value: i32, tick: u128) {
        assert!(StatusEffectService::apply_status(status, StatusChangeRequest::guaranteed(kind, duration, value), tick, 0).unwrap().started || kind == StatusChangeKind::AspdPotion1);
    }
}

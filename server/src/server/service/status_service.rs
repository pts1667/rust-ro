use std::sync::{Arc, Once};

use models::enums::bonus::BonusType;
use models::enums::class::JobName;
use models::enums::item::{EquipmentLocation, ItemType};
use models::enums::{EnumStackable, EnumWithMaskValueU32, EnumWithMaskValueU64, EnumWithNumberValue, EnumWithStringValue};
use models::item::Wearable;
use models::status::{Status, StatusSnapshot};
use models::status_bonus::StatusBonus;
use crate::server::script::ItemVm;

use crate::repository::model::item_model::ItemModel;
use crate::server::model::item_combos;
use crate::server::script::item_script_handler::ItemScriptHost;
use crate::server::service::item_service::ItemService;
use crate::server::service::global_config_service::GlobalConfigService;

static mut SERVICE_INSTANCE: Option<StatusService> = None;
static SERVICE_INSTANCE_INIT: Once = Once::new();

#[allow(dead_code)]
pub struct StatusService {
    configuration_service: &'static GlobalConfigService,
    item_script_vm: Arc<ItemVm>,
}

impl StatusService {
    pub fn new(configuration_service: &'static GlobalConfigService, item_script_vm: Arc<ItemVm>) -> StatusService {
        StatusService {
            configuration_service,
            item_script_vm,
        }
    }

    pub fn instance() -> &'static StatusService {
        unsafe { (*&raw const SERVICE_INSTANCE).as_ref().unwrap() }
    }

    pub fn init(configuration_service: &'static GlobalConfigService, item_script_vm: Arc<ItemVm>) {
        SERVICE_INSTANCE_INIT.call_once(|| unsafe {
            SERVICE_INSTANCE = Some(StatusService::new(configuration_service, item_script_vm));
        });
    }

    #[inline(always)]
    pub fn to_snapshot_cached(&self, status: &Status, _tick: u128) -> StatusSnapshot {
        self.to_snapshot(status)
    }

    //#[metrics::elapsed]
    pub fn to_snapshot(&self, status: &Status) -> StatusSnapshot {
        let mut snapshot = StatusSnapshot::_from(status);
        let job = JobName::from_value(status.job as usize);
        let job_config = self.configuration_service.get_job_config(snapshot.job());
        let index_for_job_level = (status.job_level.max(1) - 1) as usize;
        let index_for_base_level = (status.base_level.max(1) - 1).min(99) as usize; // TODO if we want to scale some stats after lvl 99, need to remove min(99) and implement formula
        job_config.bonus_stats().get(index_for_job_level).map(|bonus| {
            snapshot.set_bonus_str(*bonus.get("str").unwrap_or(&0_i16));
            snapshot.set_bonus_agi(*bonus.get("agi").unwrap_or(&0_i16));
            snapshot.set_bonus_dex(*bonus.get("dex").unwrap_or(&0_i16));
            snapshot.set_bonus_vit(*bonus.get("vit").unwrap_or(&0_i16));
            snapshot.set_bonus_int(*bonus.get("int").unwrap_or(&0_i16));
            snapshot.set_bonus_luk(*bonus.get("luk").unwrap_or(&0_i16));
        });
        let mut bonuses: Vec<BonusType> = vec![];

        for equipment in status.equipped_weapons().iter() {
            let start = bonuses.len();
            let item_model = self.configuration_service.get_item(equipment.item_id());
            if !matches!(equipment.card0, 254..=256) {
                for card in [equipment.card0, equipment.card1, equipment.card2, equipment.card3] {
                    if let Some(item) = (card > 0).then(|| self.configuration_service.find_item(card as i32)).flatten() {
                        self.collect_bonuses_at(status, &mut bonuses, item, equipment.location());
                    }
                }
            }
            self.collect_bonuses_at(status, &mut bonuses, item_model, equipment.location());
            if equipment.location() & EquipmentLocation::HandLeft.as_flag() != 0
                && equipment.location() & EquipmentLocation::HandRight.as_flag() == 0
            {
                let left_bonuses = bonuses.split_off(start);
                snapshot.set_left_hand_bonuses(left_bonuses.clone());
                bonuses.extend(left_bonuses.into_iter().filter(|bonus| {
                    !matches!(
                        bonus,
                        BonusType::ElementWeapon(_) | BonusType::WeaponAtk(_) | BonusType::WeaponRefineAtk(_)
                            | BonusType::DoubleAttackChancePercentage(_) | BonusType::DoubleAttackAdditionalChancePercentage(_)
                    )
                }));
            }
        }
        status.equipped_ammo().map(|ammo| {
            let item_model = self.configuration_service.get_item(ammo.item_id());
            let start = bonuses.len();
            self.collect_bonuses(status, &mut bonuses, item_model);
            let ammo_bonuses = bonuses.split_off(start);
            if let Some(element) = ammo_bonuses
                .iter()
                .filter_map(|bonus| {
                    if let BonusType::ElementWeapon(element) = bonus {
                        Some(*element)
                    } else {
                        None
                    }
                })
                .last()
            {
                if let Some(mut ammo) = snapshot.ammo().as_ref().copied() {
                    ammo.set_element(element);
                    snapshot.set_ammo(Some(ammo));
                }
            }
            bonuses.extend(
                ammo_bonuses
                    .into_iter()
                    .filter(|bonus| !matches!(bonus, BonusType::ElementWeapon(_) | BonusType::WeaponAtk(_) | BonusType::WeaponRefineAtk(_)
                        | BonusType::DoubleAttackChancePercentage(_) | BonusType::DoubleAttackAdditionalChancePercentage(_)
                        | BonusType::FleePercentage(_) | BonusType::ConditionalWeaponAtk(_, _)
                        | BonusType::ConditionalWeaponDamagePercentage(_, _) | BonusType::ResistanceMiscAttackPercentage(_))),
            );
        });

        for equipment in status.equipped_gears().iter() {
            let item_model = self.configuration_service.get_item(equipment.item_id());
            if !matches!(equipment.card0, 254..=256) {
                for card in [equipment.card0, equipment.card1, equipment.card2, equipment.card3] {
                    if let Some(item) = (card > 0).then(|| self.configuration_service.find_item(card as i32)).flatten() {
                        self.collect_bonuses_at(status, &mut bonuses, item, equipment.location());
                    }
                }
            }
            if let Some(def) = item_model.defense {
                snapshot.set_def(snapshot.def() + def)
            }
            self.collect_bonuses_at(status, &mut bonuses, item_model, equipment.location());
        }

        self.collect_pet_bonuses(status, &mut bonuses);

        self.collect_combo_bonuses(status, &mut bonuses);

        // Apply skills bonuses
        for temporary_bonus in status.temporary_bonuses.iter() {
            bonuses.push(*temporary_bonus.bonus());
        }
        for change in &status.active_statuses {
            bonuses.extend(change.bonuses());
        }
        for auto_bonus in &status.active_auto_bonuses {
            if auto_bonus.definition.source_pet_id == 0 {
                bonuses.extend(auto_bonus.bonuses.iter().copied());
            } else if crate::server::script::pet_auto_bonus::source_is_active(status, &auto_bonus.definition) {
                let mut script_status = status.clone();
                script_status.equipment_bonuses = models::status_bonus::StatusBonuses::new(bonuses.iter().copied().map(StatusBonus::new).collect());
                let host = ItemScriptHost::bonuses(script_status, 0);
                let (host, result) = futures::executor::block_on(self.item_script_vm.run_pet_program(host, auto_bonus.definition.program_id));
                if let Err(error) = result {
                    error!("Failed to calculate Wasm pet automatic bonus {}: {}", auto_bonus.definition.program_id, error);
                } else {
                    bonuses.extend(host.bonuses.drain());
                }
            }
        }

        if let Some(chance) = bonuses.iter().filter_map(|bonus| if let BonusType::DoubleAttackChancePercentage(value) = bonus { Some(*value) } else { None }).max() {
            bonuses.retain(|bonus| !matches!(bonus, BonusType::DoubleAttackChancePercentage(_)));
            bonuses.push(BonusType::DoubleAttackChancePercentage(chance.max(0)));
        }
        let kaina = crate::server::service::script_character_service::learned_level(status, models::enums::skill_enums::SkillEnum::SlKaina.id());
        if kaina > 0 {
            bonuses.push(BonusType::Maxsp(30 * i32::from(kaina)));
        }
        bonuses.extend(Self::passive_skill_bonuses(status));
        bonuses = BonusType::merge_enums(&bonuses);
        snapshot.set_bonuses(bonuses.iter().map(|bonus| StatusBonus::new(*bonus)).collect());
        let mut known_skills = snapshot.known_skills().clone();
        for grant in crate::server::service::script_character_service::taekwon_rank_skill_grants(status) {
            if let Some(known) = known_skills.iter_mut().find(|known| known.value == grant.value) {
                known.level = known.level.max(grant.level);
            } else {
                known_skills.push(grant);
            }
        }
        for bonus in &bonuses {
            if let BonusType::EnableSkillId(id, level) = bonus {
                if let Ok(value) = models::enums::skill_enums::SkillEnum::try_from_value(*id) {
                    if let Some(known) = known_skills.iter_mut().find(|known| known.value == value) {
                        known.level = known.level.max(*level);
                    } else {
                        known_skills.push(models::status::KnownSkill { value, level: *level });
                    }
                }
            }
        }
        snapshot.set_known_skills(known_skills);
        if status.riding && snapshot.known_skill_level(models::enums::skill_enums::SkillEnum::KnRiding) > 0 {
            snapshot.set_state(snapshot.state() | models::enums::skill::SkillState::Riding.as_flag());
        }
        if status.falcon && snapshot.known_skill_level(models::enums::skill_enums::SkillEnum::HtFalcon) > 0 {
            snapshot.set_state(snapshot.state() | models::enums::skill::SkillState::Falcon.as_flag());
        }

        let firearm = matches!(snapshot.right_hand_weapon_type(), models::enums::weapon::WeaponType::Revolver
            | models::enums::weapon::WeaponType::Rifle | models::enums::weapon::WeaponType::Gatling
            | models::enums::weapon::WeaponType::Shotgun | models::enums::weapon::WeaponType::Grenade);
        if firearm {
            let single_action = snapshot.known_skill_level(models::enums::skill_enums::SkillEnum::GsSingleaction);
            if single_action > 0 {
                let rate = f32::from(single_action.div_ceil(2));
                if let Some(BonusType::AspdPercentage(value)) = bonuses.iter_mut().find(|bonus| matches!(bonus, BonusType::AspdPercentage(_))) { *value += rate; }
                else { bonuses.push(BonusType::AspdPercentage(rate)); }
                snapshot.set_bonuses(bonuses.iter().map(|bonus| StatusBonus::new(*bonus)).collect());
            }
        }

        use models::enums::skill_enums::SkillEnum;
        let hilt_binding = if snapshot.known_skill_level(SkillEnum::BsHiltbinding) > 0 { 1 } else { 0 };
        let owls_eye = i16::from(snapshot.known_skill_level(SkillEnum::AcOwl));
        let dragonology = (i16::from(snapshot.known_skill_level(SkillEnum::SaDragonology)) + 1) / 2;
        snapshot.set_bonus_str(snapshot.bonus_str().saturating_add(hilt_binding));
        snapshot.set_bonus_dex(snapshot.bonus_dex().saturating_add(owls_eye));
        snapshot.set_bonus_int(snapshot.bonus_int().saturating_add(dragonology));

        bonuses
            .iter()
            .filter(|bonus| !matches!(bonus, BonusType::Maxhp(_) | BonusType::Maxsp(_) | BonusType::Matk(_)))
            .for_each(|bonus| bonus.add_bonus_to_status(&mut snapshot));
        crate::server::service::status_effect_service::StatusEffectService::adjust_status_attributes(status, &mut snapshot);
        if status.has_status_change(models::status_change::StatusChangeKind::Curse) {
            snapshot.set_bonus_luk(-(snapshot.base_luk() as i16));
        }
        // TODO [([base_hp*(1 + VIT/100)* trans_mod]+HPAdditions)*ItemHPMultipliers] https://irowiki.org/classic/Max_HP
        let hp_rebirth_modifier: f32 = if job.is_rebirth() {
            1.25
        } else if status.taekwon_ranked && job == JobName::Taekwon && status.base_level >= 90 {
            3.0
        } else {
            1.0
        };
        snapshot.set_max_hp(
            (job_config.base_hp()[index_for_base_level] as f32 * (1.0 + snapshot.vit() as f32 / 100.0) * hp_rebirth_modifier).floor()
                as u32,
        );
        // TODO https://irowiki.org/classic/Max_SP
        snapshot.set_max_sp(
            (job_config.base_sp()[index_for_base_level] as f32 * (1.0 + snapshot.int() as f32 / 100.0) * hp_rebirth_modifier).floor()
                as u32,
        );
        // TODO 1 + YourLUK*0.3 + Critical Increasing Cards)*CritModifier - TargetLUK/5
        snapshot.set_crit(Self::truncate(snapshot.crit() + (1.0 + snapshot.luk() as f32 * 0.3), 1));
        snapshot.set_hit((snapshot.hit() + status.base_level as i16 + snapshot.dex() as i16).max(0));
        let passive_hit = i16::from(snapshot.known_skill_level(SkillEnum::AcVulture))
            + 2 * i16::from(snapshot.known_skill_level(SkillEnum::BsWeaponresearch))
            + if firearm { 2 * i16::from(snapshot.known_skill_level(SkillEnum::GsSingleaction))
                + i16::from(snapshot.known_skill_level(SkillEnum::GsSnakeeye)) } else { 0 };
        snapshot.set_flee((snapshot.flee() + status.base_level as i16 + snapshot.agi() as i16).max(0));
        let improved_dodge_per_level = if matches!(job, JobName::Assassin | JobName::AssassinCross | JobName::BabyAssassin
            | JobName::Rogue | JobName::Stalker | JobName::BabyRogue) { 4 } else { 3 };
        let passive_flee = i16::from(snapshot.known_skill_level(SkillEnum::TfMiss)) * improved_dodge_per_level
            + i16::from(snapshot.known_skill_level(SkillEnum::MoDodge)) * 3 / 2;
        let perfect_dodge_bonus = bonuses
            .iter()
            .filter_map(|bonus| match bonus {
                BonusType::PerfectDodge(value) => Some(*value as f32),
                _ => None,
            })
            .sum::<f32>();
        snapshot.set_perfect_dodge((1.0 + snapshot.luk() as f32 / 10.0 + perfect_dodge_bonus).clamp(0.0, 100.0));
        if let Some(auto_blitz) = Self::auto_blitz_proc(status, &snapshot) {
            bonuses.push(BonusType::CombatProc(auto_blitz, 1));
            snapshot.set_bonuses(bonuses.iter().map(|bonus| StatusBonus::new(*bonus)).collect());
        }
        snapshot.set_aspd(snapshot.aspd() + self.aspd(&snapshot));
        snapshot.set_matk_min(
            ((snapshot.int() + ((snapshot.int() as f32 / 7.0).floor() as u16).pow(2)) as f32 * snapshot.matk_item_modifier()).floor()
                as u16,
        );
        snapshot.set_matk_max(
            ((snapshot.int() + ((snapshot.int() as f32 / 5.0).floor() as u16).pow(2)) as f32 * snapshot.matk_item_modifier()).floor()
                as u16,
        );
        for bonus in &bonuses {
            match bonus {
                BonusType::Maxhp(value) => snapshot.set_max_hp((snapshot.max_hp() as i64 + *value as i64).clamp(1, u32::MAX as i64) as u32),
                BonusType::Maxsp(value) => snapshot.set_max_sp((snapshot.max_sp() as i64 + *value as i64).clamp(1, u32::MAX as i64) as u32),
                BonusType::Matk(value) => {
                    snapshot.set_matk_min((snapshot.matk_min() as i32 + *value as i32).clamp(0, u16::MAX as i32) as u16);
                    snapshot.set_matk_max((snapshot.matk_max() as i32 + *value as i32).clamp(0, u16::MAX as i32) as u16);
                }
                _ => {}
            }
        }
        snapshot.set_fist_atk(self.fist_atk(&snapshot, snapshot.right_hand_weapon_type().is_ranged()));
        snapshot.set_atk_left_side(self.status_atk_left_side(&snapshot));
        self.set_status_atk_right_side(&mut snapshot);
        let hp_before_rate = snapshot.max_hp();
        let sp_before_rate = snapshot.max_sp();
        bonuses.iter().filter(|bonus| !matches!(bonus, BonusType::AspdPercentage(_)))
            .for_each(|bonus| bonus.add_percentage_bonus_to_status(&mut snapshot));
        snapshot.set_hit(snapshot.hit().saturating_add(passive_hit));
        snapshot.set_flee(snapshot.flee().saturating_add(passive_flee));
        let hp_rate = bonuses
            .iter()
            .filter_map(|bonus| {
                if let BonusType::MaxhpPercentage(value) = bonus {
                    Some(i32::from(*value))
                } else {
                    None
                }
            })
            .sum::<i32>()
            + if status.has_status_change(models::status_change::StatusChangeKind::Berserk) {
                200
            } else {
                0
            };
        let sp_rate = bonuses
            .iter()
            .filter_map(|bonus| {
                if let BonusType::MaxspPercentage(value) = bonus {
                    Some(i32::from(*value))
                } else {
                    None
                }
            })
            .sum::<i32>();
        snapshot
            .set_max_hp(crate::server::service::status_effect_service::StatusEffectService::maximum_pool(hp_before_rate, 0, hp_rate, 1));
        snapshot
            .set_max_sp(crate::server::service::status_effect_service::StatusEffectService::maximum_pool(sp_before_rate, 0, sp_rate, 1));
        let battle = &self.configuration_service.config().battle;
        snapshot.set_max_hp(snapshot.max_hp().min(battle.get("max_hp") as u32));
        snapshot.set_max_sp(snapshot.max_sp().min(battle.get("max_sp") as u32));
        let attack_rate = bonuses
            .iter()
            .filter_map(|bonus| {
                if let BonusType::AtkPercentage(value) = bonus {
                    Some(i32::from(*value))
                } else {
                    None
                }
            })
            .sum::<i32>();
        snapshot.set_atk_left_side(
            (i64::from(snapshot.atk_left_side()) * i64::from((100 + attack_rate).max(0)) / 100).clamp(0, i64::from(i32::MAX)) as i32,
        );
        snapshot.set_atk_right_side(
            (i64::from(snapshot.atk_right_side()) * i64::from((100 + attack_rate).max(0)) / 100).clamp(0, i64::from(i32::MAX)) as i32,
        );
        crate::server::service::status_effect_service::StatusEffectService::adjust_snapshot(status, &mut snapshot);
        snapshot
    }

    #[inline]
    pub fn collect_bonuses(&self, status: &Status, mut bonuses: &mut Vec<BonusType>, item_model: &ItemModel) {
        if item_model.item_bonuses_are_dynamic {
            self.collect_dynamic_script(status, &mut bonuses, &item_model);
        } else {
            item_model.bonuses.iter().for_each(|bonus| bonuses.push(*bonus))
        }
    }

    pub fn collect_bonuses_at(&self, status: &Status, bonuses: &mut Vec<BonusType>, item_model: &ItemModel, owner_location: u64) {
        let start = bonuses.len();
        self.collect_bonuses(status, bonuses, item_model);
        for bonus in &mut bonuses[start..] {
            if let BonusType::AutoBonus(definition, _) = bonus {
                definition.source_location = owner_location;
            }
        }
    }

    /// Every worn combo applies the script of its set once.
    fn collect_combo_bonuses(&self, status: &Status, bonuses: &mut Vec<BonusType>) {
        let worn = item_combos::worn_items(status);
        if worn.is_empty() {
            return;
        }
        let is_card = |id: i32| self.configuration_service.find_item(id).is_some_and(|item| item.item_type == ItemType::Card);
        for set in item_combos::catalog() {
            let count = set.active_count(&worn, &is_card);
            if count == 0 {
                continue;
            }
            let dynamic = ItemService::script_metadata(set.id).is_some_and(|script| script.dynamic);
            if dynamic {
                for _ in 0..count {
                    self.run_combo_script(status, bonuses, set.id);
                }
            } else {
                let fixed = set.static_bonuses.get_or_init(|| {
                    let mut fixed = vec![];
                    self.run_combo_script(&Status::default(), &mut fixed, set.id);
                    fixed
                });
                for _ in 0..count {
                    bonuses.extend(fixed.iter().copied());
                }
            }
        }
    }

    fn run_combo_script(&self, status: &Status, bonuses: &mut Vec<BonusType>, script_id: u32) {
        let mut script_status = status.clone();
        script_status.equipment_bonuses = models::status_bonus::StatusBonuses::new(bonuses.iter().copied().map(StatusBonus::new).collect());
        let host = ItemScriptHost::bonuses(script_status, script_id);
        let (host, result) = futures::executor::block_on(self.item_script_vm.run_item(host, script_id));
        if let Err(error) = result {
            error!("Failed to execute Wasm combo script {}: {}", script_id, error);
        } else {
            bonuses.extend(host.bonuses.drain());
        }
    }

    fn collect_pet_bonuses(&self, status: &Status, bonuses: &mut Vec<BonusType>) {
        let Some(pet) = status.script_context.as_ref().and_then(|context| context.pet.as_ref()) else {
            return;
        };
        bonuses.extend(pet.support_bonuses.iter().copied());
        let mut script_status = status.clone();
        script_status.equipment_bonuses = models::status_bonus::StatusBonuses::new(bonuses.iter().copied().map(StatusBonus::new).collect());
        let host = ItemScriptHost::bonuses(script_status, 0);
        let (host, result) = futures::executor::block_on(self.item_script_vm.run_pet(host, u32::from(pet.class_id)));
        if let Err(error) = result {
            error!("Failed to execute Wasm pet bonus script {}: {}", pet.class_id, error);
        } else {
            bonuses.extend(host.bonuses.drain());
        }
    }

    // #[metrics::elapsed]
    #[inline]
    fn collect_dynamic_script(&self, status: &Status, bonuses: &mut &mut Vec<BonusType>, item_model: &&ItemModel) {
        let mut script_status = status.clone();
        script_status.equipment_bonuses =
            models::status_bonus::StatusBonuses::new(bonuses.iter().copied().map(models::status_bonus::StatusBonus::new).collect());
        let host = ItemScriptHost::bonuses(script_status, item_model.id as u32);
        let (host, result) = futures::executor::block_on(self.item_script_vm.run_item(host, item_model.id as u32));
        if let Err(error) = result {
            error!("Failed to execute Wasm item script {}: {}", item_model.id, error);
        } else {
            bonuses.extend(host.bonuses.drain());
        }
    }

    #[inline]
    fn passive_skill_bonuses(status: &Status) -> Vec<BonusType> {
        use models::enums::element::Element;
        use models::enums::skill_enums::SkillEnum;
        let level = |skill: SkillEnum| i32::from(crate::server::service::script_character_service::learned_level(status, skill.id()));
        let mut bonuses = Vec::new();
        let trust = level(SkillEnum::CrTrust);
        if trust > 0 {
            bonuses.push(BonusType::Maxhp(200 * trust));
            bonuses.push(BonusType::ResistanceDamageFromElementPercentage(Element::Holy, (5 * trust) as i8));
        }
        let skin_tempering = level(SkillEnum::BsSkintemper);
        if skin_tempering > 0 {
            bonuses.push(BonusType::ResistanceDamageFromElementPercentage(Element::Neutral, skin_tempering as i8));
            bonuses.push(BonusType::ResistanceDamageFromElementPercentage(Element::Fire, (5 * skin_tempering) as i8));
        }
        let mana_recharge = level(SkillEnum::HpManarecharge);
        if mana_recharge > 0 {
            bonuses.push(BonusType::SpConsumption((-4 * mana_recharge) as i8));
        }
        bonuses.extend(crate::server::script::skill::star_gladiator::anger_bonuses(status));
        bonuses.extend(crate::server::script::skill::star_gladiator::devil_bonuses(status));
        let soul_drain = level(SkillEnum::HwSouldrain);
        if soul_drain > 0 {
            bonuses.push(BonusType::MaxspPercentage((2 * soul_drain) as i8));
        }
        bonuses
    }

    /// A falcon-carrying archer sometimes follows a normal arrow shot with a free Blitz Beat.
    fn auto_blitz_proc(status: &Status, snapshot: &StatusSnapshot) -> Option<models::status_bonus::CombatProc> {
        use models::enums::skill_enums::SkillEnum;
        use models::status_bonus::{AutoSpellFlag, BattleFlag, CombatProc, CombatProcKind, CombatTrigger};
        let learned = snapshot.known_skill_level(SkillEnum::HtBlitzbeat);
        if learned == 0 || !status.falcon || *snapshot.right_hand_weapon_type() != models::enums::weapon::WeaponType::Bow {
            return None;
        }
        let level = learned.min(((status.job_level + 9) / 10).clamp(1, u32::from(u8::MAX)) as u8);
        let per_mille = snapshot.luk() as i32 * 10 / 3 + 1;
        // Arrow shots halve spell procs, the doubling restores the intended per-mille chance.
        let mut proc = CombatProc::new(CombatTrigger::Attack, CombatProcKind::Spell, per_mille * 10 * 2);
        proc.value = SkillEnum::HtBlitzbeat.id();
        proc.level = i16::from(level);
        proc.flags = AutoSpellFlag::OtherTarget.as_flag();
        proc.battle_flags = BattleFlag::normalize(0, true);
        Some(proc)
    }

    fn truncate(x: f32, decimals: u32) -> f32 {
        let y = 10i32.pow(decimals) as f32;
        (x * y).round() / y
    }

    #[inline]
    pub fn attack_per_seconds(&self, aspd: f32) -> f32 {
        50_f32 / (200_f32 - aspd.min(self.max_aspd()))
    }

    fn max_aspd(&self) -> f32 {
        self.configuration_service.config().battle.get("max_aspd") as f32
    }

    #[inline]
    pub fn attack_motion(&self, status: &StatusSnapshot) -> u32 {
        let aspd = status.aspd();
        (1000.0 / self.attack_per_seconds(aspd)).round() as u32
    }

    #[inline]
    pub fn attack_delay(&self, status: &StatusSnapshot) -> u32 {
        self.attack_motion(status) / 2
    }

    pub fn client_aspd(&self, aspd: f32) -> i32 {
        ((200_f32 - aspd.min(self.max_aspd())) * 10.0).round() as i32
    }

    pub fn cast_time_reduction(&self, status: &StatusSnapshot) -> f32 {
        (1.0 - status.dex() as f32 / 150.0).max(0.0) * (1.0 + status.cast_time()).max(0.0)
    }

    pub fn skill_cast_modifier(status: &StatusSnapshot, skill_id: u32) -> f32 {
        let metadata = crate::server::script::skill::metadata::SkillMetadata::find(skill_id);
        let ignores = |flag: &str| metadata.is_some_and(|metadata| metadata.cast_time_flags.get(flag).copied().unwrap_or(false));
        let dex = if ignores("IgnoreDex") {
            1.0
        } else {
            (1.0 - status.dex() as f32 / 150.0).max(0.0)
        };
        let status_rate = status
            .active_statuses()
            .iter()
            .flat_map(|change| change.bonuses())
            .filter_map(|bonus| {
                if let BonusType::CastTimePercentage(value) = bonus {
                    Some(i32::from(value))
                } else {
                    None
                }
            })
            .sum::<i32>();
        let item_rate = status.cast_time() - status_rate as f32 / 100.0;
        let item = if ignores("IgnoreItemBonus") {
            1.0
        } else {
            (1.0 + item_rate).max(0.0)
        };
        let specific = status
            .bonuses_raw()
            .iter()
            .filter_map(|bonus| {
                if let BonusType::CastTimeWhenUsingSkillIdPercentage(id, value) = bonus {
                    (*id == skill_id).then_some(i32::from(*value))
                } else {
                    None
                }
            })
            .sum::<i32>();
        let mut modifier = dex * item * (1.0 + specific as f32 / 100.0).max(0.0);
        if !ignores("IgnoreStatus") {
            if status.has_status_change(models::status_change::StatusChangeKind::Memorize) {
                modifier *= 0.5;
            }
            if let Some(change) = status.status_change(models::status_change::StatusChangeKind::SlowCast) {
                modifier *= 1.0 + (20 * change.values[0]) as f32 / 100.0;
            }
            if let Some(change) = status.status_change(models::status_change::StatusChangeKind::Suffragium) {
                modifier *= (1.0 - (15 * change.values[0]) as f32 / 100.0).max(0.0);
            }
        }
        modifier
    }

    pub fn skill_after_cast_delay(status: &StatusSnapshot, skill_id: u32, base_delay: u32) -> u32 {
        let ignores_item = crate::server::script::skill::metadata::SkillMetadata::find(skill_id)
            .is_some_and(|metadata| metadata.cast_delay_flags.get("IgnoreItemBonus").copied().unwrap_or(false));
        let rate = if ignores_item {
            0
        } else {
            status
                .bonuses_raw()
                .iter()
                .filter_map(|bonus| {
                    if let BonusType::SkillDelayIncDecPercentage(value) = bonus {
                        Some(i32::from(*value))
                    } else {
                        None
                    }
                })
                .sum::<i32>()
        };
        let mut delay = u64::from(base_delay) * (100 + rate).max(0) as u64 / 100;
        if crate::server::script::skill::ScriptSkillService::spirit_owner_matches(status.status_change(models::status_change::StatusChangeKind::Spirit), skill_id) {
            delay /= 2;
        }
        delay.min(u64::from(u32::MAX)) as u32
    }

    ///  PRE-RE formula: 200-(WD-([WD*AGI/25]+[WD*DEX/100])/10)*(1-SM)  https://irowiki.org/classic/ASPD
    /// [] - Square brackets hold the same priority as normal brackets, but
    /// indicate that the value of the contents should be rounded down to the
    /// nearest whole number (integer) once calculated. http://calc.free-ro.com/
    fn aspd(&self, status: &StatusSnapshot) -> f32 {
        let weapon_delay = self.weapon_delay(status) as f32 / 10.0;
        let speed_modifier = 0_f32;
        200.0
            - (weapon_delay
                - ((((weapon_delay * (status.agi() as f32)) / 25.0).floor() + ((weapon_delay * (status.dex() as f32)) / 100.0).floor())
                    / 10.0)
                    * (1.0 - speed_modifier))
    }

    #[inline]
    fn weapon_delay(&self, status: &StatusSnapshot) -> u32 {
        let weapon = status.right_hand_weapon_type();
        *self
            .configuration_service
            .get_job_config(status.job())
            .base_aspd()
            .get(weapon.as_str())
            .unwrap_or(&2000)
    }

    /// PRE-RE https://irowiki.org/classic/Attacks
    /// UI left side atk in status info panel
    /// https://web.archive.org/web/20060717223009/http://rodatazone.simgaming.net/mechanics/substats.php
    ///
    /// Atk stands for Attack and gives an indication of how much damage you
    /// will do when you hit something. The visible components of the Atk
    /// score are your Strength plus the Atk of the weapon you are using on the
    /// left and the damage bonus from any pluses the weapon might have on the
    /// right. The real value on the left of your Atk score includes hidden
    /// bonuses from Strength, Dexterity and Luck. For fists, the true value
    /// is equal to: STR + [STR/10]^2 + [DEX/5] + [LUK/5] where [] indicates you
    /// round the value inside down before continuing and ^2 indicates squaring.
    /// For weapons, the true value is equal to: STR + [STR/10]^2 + [DEX/5] +
    /// [LUK/5] + WeaponAtk + AtkBonusCards where [] indicates you round the
    /// value inside down before continuing and ^2 indicates squaring.
    /// For missile weapons, the true value is equal to: DEX + [DEX/10]^2 +
    /// [STR/5] + [LUK/5] + WeaponAtk + AtkBonusCards where [] indicates you
    /// round the value inside down before continuing and ^2 indicates squaring.
    /// Not counting the value of WeaponAtk and AtkBonusCards, this true value
    /// is often referred to as the base damage. This base damage is basically
    /// the your Atk with bare fists.
    #[inline]
    fn status_atk_left_side(&self, status: &StatusSnapshot) -> i32 {
        (i32::from(status.fist_atk())
            + i32::from(status.weapon_atk())
            + i32::from(status.left_weapon_atk())
            + i32::from(status.base_atk())
            + status.bonus_atk() as i32)
            .max(0)
    }

    #[inline]
    pub(crate) fn fist_atk(&self, status: &StatusSnapshot, is_ranged: bool) -> u16 {
        let mut str;
        let dex;

        if is_ranged {
            str = status.dex();
            dex = status.str();
        } else {
            str = status.str();
            dex = status.dex();
        }
        // For homunculus
        // dstr = str / 10;
        // str += dstr*dstr;
        let dstr = str / 10;
        str += dstr * dstr;
        str += dex / 5 + status.luk() / 5;
        str
    }

    /// UI right side atk in status info panel
    /// https://web.archive.org/web/20060717223009/http://rodatazone.simgaming.net/mechanics/substats.php
    /// https://web.archive.org/web/20060717222819/http://rodatazone.simgaming.net/items/upgrading.php
    #[inline]
    pub fn set_status_atk_right_side(&self, status: &mut StatusSnapshot) {
        let atk_right = i32::from(status.weapon_upgrade_damage()) + i32::from(status.left_weapon_upgrade_damage());
        let mut overupgrade_right_hand_atk_bonus = 0;
        let mut overupgrade_left_hand_atk_bonus = 0;
        status.right_hand_weapon().map(|w| {
            if w.level() == 1 {
                if w.refine() > 7 {
                    overupgrade_right_hand_atk_bonus = (w.refine() - 7) * 3;
                }
            } else if w.level() == 2 {
                if w.refine() > 6 {
                    overupgrade_right_hand_atk_bonus = (w.refine() - 6) * 5;
                }
            } else if w.level() == 3 {
                if w.refine() > 5 {
                    overupgrade_right_hand_atk_bonus = (w.refine() - 5) * 8;
                }
            } else if w.level() == 4 {
                if w.refine() > 4 {
                    overupgrade_right_hand_atk_bonus = (w.refine() - 4) * 13;
                }
            }
        });
        status.left_hand_weapon().map(|w| {
            if w.level() == 1 {
                if w.refine() > 7 {
                    overupgrade_left_hand_atk_bonus = (w.refine() - 7) * 3;
                }
            } else if w.level() == 2 {
                if w.refine() > 6 {
                    overupgrade_left_hand_atk_bonus = (w.refine() - 6) * 5;
                }
            } else if w.level() == 3 {
                if w.refine() > 5 {
                    overupgrade_left_hand_atk_bonus = (w.refine() - 5) * 8;
                }
            } else if w.level() == 4 {
                if w.refine() > 4 {
                    overupgrade_left_hand_atk_bonus = (w.refine() - 4) * 13;
                }
            }
        });
        status.set_atk_right_side(atk_right);
        status.set_overupgrade_right_hand_atk_bonus(overupgrade_right_hand_atk_bonus);
        status.set_overupgrade_left_hand_atk_bonus(overupgrade_left_hand_atk_bonus);
    }

    #[inline]
    pub fn character_vit_def(&self, status_snapshot: &StatusSnapshot) -> u16 {
        let modifier = status_snapshot
            .bonuses_raw()
            .iter()
            .filter_map(|bonus| match bonus {
                BonusType::VitDefPercentage(value) => Some(*value as i32),
                _ => None,
            })
            .sum::<i32>();
        let mut defense = status_snapshot.vit() as i32 * (100 + modifier).max(0) / 100;
        if let Some(change) = status_snapshot
            .active_statuses()
            .iter()
            .find(|change| change.kind == models::status_change::StatusChangeKind::Angelus)
        {
            defense += defense * change.values[0] * 5 / 100;
        }
        if let Some(change) = status_snapshot.status_change(models::status_change::StatusChangeKind::Provoke) {
            defense = defense * (100 - change.values[2]).max(0) / 100;
        }
        if let Some(change) = status_snapshot.status_change(models::status_change::StatusChangeKind::JointBeat) {
            use models::status_change::JointBreak;
            let mut reduction = 0;
            if change.values[1] as u32 & JointBreak::Shoulder.as_flag() != 0 {
                reduction += 50;
            }
            if change.values[1] as u32 & JointBreak::Waist.as_flag() != 0 {
                reduction += 25;
            }
            defense = defense * (100 - reduction) / 100;
        }
        defense.clamp(0, u16::MAX as i32) as u16
    }

    pub fn character_regen_hp(&self, status_snapshot: &StatusSnapshot) -> u32 {
        // var HPR = Math.max( 1, Math.floor(MAX_HP / 200) );
        // HPR += Math.floor( VIT / 5 );
        // HPR = Math.floor( HPR * (1 + HPR_MOD * 0.01) );
        let hp_regen =
            1.0_f32.max((status_snapshot.max_hp() as f32 / 200.0).floor()) as u32 + (status_snapshot.vit() as f32 / 5.0).floor() as u32;
        if status_snapshot
            .bonuses_raw()
            .iter()
            .any(|bonus| matches!(bonus, BonusType::DisableHpRegen))
        {
            return 0;
        }
        let mut modifier = status_snapshot
            .bonuses_raw()
            .iter()
            .filter_map(|bonus| match bonus {
                BonusType::NaturalHpRecoveryPercentage(value) => Some(*value as i32),
                _ => None,
            })
            .sum::<i32>();
        if let Some(change) = status_snapshot
            .status_change(models::status_change::StatusChangeKind::Regeneration)
            .filter(|change| change.values[3] == 0)
        {
            modifier += change.values[1] * 100;
        }
        (hp_regen as u64 * (100 + modifier).max(0) as u64 / 100).min(u32::MAX as u64) as u32
    }

    pub fn character_regen_sp(&self, status_snapshot: &StatusSnapshot) -> u32 {
        // var SPR = 1;
        // SPR += Math.floor( MAX_SP / 100 );
        // SPR += Math.floor( INT / 6 );
        // if (INT >= 120) {
        //  SPR += Math.floor(INT / 2 - 56);
        // }
        // SPR = Math.floor( SPR * (1 + SPR_MOD * 0.01) );
        let mut sp_regen =
            1 + (status_snapshot.max_sp() as f32 / 100.0).floor() as u32 + (status_snapshot.int() as f32 / 6.0).floor() as u32;
        if status_snapshot.int() >= 120  {
            sp_regen += ((status_snapshot.int() as f32 / 2.0) - 56.0).floor() as u32;
        }
        if status_snapshot
            .bonuses_raw()
            .iter()
            .any(|bonus| matches!(bonus, BonusType::DisableSpRegen))
        {
            return 0;
        }
        if status_snapshot.has_status_change(models::status_change::StatusChangeKind::ExplosionSpirits)
            && !crate::server::script::skill::ScriptSkillService::spirit_rules(status_snapshot.status_change(models::status_change::StatusChangeKind::Spirit)).explosion_sp_regen
        {
            return 0;
        }
        let mut modifier = status_snapshot
            .bonuses_raw()
            .iter()
            .filter_map(|bonus| match bonus {
                BonusType::NaturalSpRecoveryPercentage(value) => Some(*value as i32),
                _ => None,
            })
            .sum::<i32>();
        if let Some(change) = status_snapshot
            .status_change(models::status_change::StatusChangeKind::Regeneration)
            .filter(|change| change.values[3] == 0)
        {
            modifier += change.values[2] * 100;
        }
        (sp_regen as u64 * (100 + modifier).max(0) as u64 / 100).min(u32::MAX as u64) as u32
    }
}

#[cfg(test)]
mod tests {
    use models::enums::bonus::BonusType;
    use models::enums::skill_enums::SkillEnum;
    use models::status::Status;
    use models::status_bonus::TemporaryStatusBonus;

    use crate::server::service::global_config_service::GlobalConfigService;
    use crate::server::service::status_service::StatusService;
    use crate::tests::common;

    #[test]
    fn classic_skill_timing_keeps_item_and_specific_rates_when_dex_is_ignored() {
        use models::status::StatusSnapshot;
        use models::status_bonus::StatusBonus;
        let mut snapshot = StatusSnapshot::_from(&Status {
            dex: 75,
            ..Status::default()
        });
        snapshot.set_cast_time(0.2);
        snapshot.set_bonuses(vec![
            StatusBonus::new(BonusType::CastTimePercentage(20)),
            StatusBonus::new(BonusType::CastTimeWhenUsingSkillIdPercentage(SkillEnum::MgFirebolt.id(), -50)),
            StatusBonus::new(BonusType::SkillDelayIncDecPercentage(-50)),
        ]);
        assert!((StatusService::skill_cast_modifier(&snapshot, SkillEnum::MgFirebolt.id()) - 0.3).abs() < 0.0001);
        snapshot.set_base_dex(150);
        assert_eq!(StatusService::skill_cast_modifier(&snapshot, SkillEnum::MgFirebolt.id()), 0.0);
        assert!((StatusService::skill_cast_modifier(&snapshot, SkillEnum::ChPalmstrike.id()) - 1.2).abs() < 0.0001);
        assert_eq!(
            StatusService::skill_after_cast_delay(&snapshot, SkillEnum::MgFirebolt.id(), 1000),
            500
        );
    }

    #[test]
    fn trust_skin_tempering_and_mana_recharge_passives_scale_with_their_level() {
        use models::enums::element::Element;
        use models::status::KnownSkill;
        let status = Status {
            known_skills: vec![
                KnownSkill { value: SkillEnum::CrTrust, level: 5 },
                KnownSkill { value: SkillEnum::BsSkintemper, level: 5 },
                KnownSkill { value: SkillEnum::HpManarecharge, level: 5 },
            ],
            ..Status::default()
        };
        let bonuses = StatusService::passive_skill_bonuses(&status);
        assert!(bonuses.contains(&BonusType::Maxhp(1000)));
        assert!(bonuses.contains(&BonusType::ResistanceDamageFromElementPercentage(Element::Holy, 25)));
        assert!(bonuses.contains(&BonusType::ResistanceDamageFromElementPercentage(Element::Neutral, 5)));
        assert!(bonuses.contains(&BonusType::ResistanceDamageFromElementPercentage(Element::Fire, 25)));
        assert!(bonuses.contains(&BonusType::SpConsumption(-20)));
        assert!(StatusService::passive_skill_bonuses(&Status::default()).is_empty());
    }

    #[test]
    fn auto_blitz_needs_a_falcon_and_a_bow_and_caps_the_level_by_job_level() {
        use models::enums::weapon::WeaponType;
        use models::enums::EnumWithMaskValueU64;
        use models::item::WearWeapon;
        use models::status::{KnownSkill, StatusSnapshot};
        let mut status = Status {
            luk: 99,
            job_level: 11,
            falcon: true,
            known_skills: vec![KnownSkill { value: SkillEnum::HtBlitzbeat, level: 5 }, KnownSkill { value: SkillEnum::HtFalcon, level: 1 }],
            weapons: vec![WearWeapon {
                item_id: 1701, attack: 15, level: 1, weapon_type: WeaponType::Bow, location: models::enums::item::EquipmentLocation::HandRight.as_flag(),
                refine: 0, element: models::enums::element::Element::Neutral, card0: 0, card1: 0, card2: 0, card3: 0, ranked_forged: false, inventory_index: 0, range: 9,
            }],
            ..Status::default()
        };
        let proc = StatusService::auto_blitz_proc(&status, &StatusSnapshot::_from(&status)).unwrap();
        assert_eq!((proc.value, proc.level, proc.rate), (SkillEnum::HtBlitzbeat.id(), 2, 6620));
        status.falcon = false;
        assert!(StatusService::auto_blitz_proc(&status, &StatusSnapshot::_from(&status)).is_none());
    }

    #[test]
    fn test_snapshot_bonuses_have_temporary_bonuses() {
        // Given
        common::before_all();
        let service = StatusService::new(GlobalConfigService::instance(), common::test_item_vm());
        let mut status = Status::default();
        status.temporary_bonuses.add(TemporaryStatusBonus::with_duration(
            BonusType::Agi(10),
            0,
            0,
            1000,
            SkillEnum::AlIncagi.id() as u16,
        ));
        // When
        let snapshot = service.to_snapshot(&status);
        // Then
        assert!(
            snapshot
                .bonuses()
                .iter()
                .any(|status_bonus| { matches!(status_bonus.bonus(), BonusType::Agi(10)) }),
            "missing agi bonus"
        );
    }
}

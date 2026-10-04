use configuration::configuration::GameConfig;
use models::enums::EnumWithStringValue;
use models::enums::element::Element;
use models::enums::mob::{MobRace, MobGroup};
use models::enums::size::Size;
use models::status::{KnownSkill, Look, Status, StatusSnapshot};
use models::status_bonus::{StatusBonuses, TemporaryStatusBonuses};

use crate::repository::model::char_model::CharSelectModel;
use crate::repository::model::mob_model::MobModel;

pub struct StatusFromDb;
impl StatusFromDb {
    pub fn from_char_model(char_model: &CharSelectModel, configuration: &GameConfig, known_skills: Vec<KnownSkill>) -> Status {
        Status {
            taekwon_ranked: false,
            spirit_sphere_count: 0,
            job: char_model.class as u32,
            hp: char_model.hp as u32,
            sp: char_model.sp as u32,
            max_hp: char_model.max_hp as u32,
            max_sp: char_model.max_sp as u32,
            str: char_model.str as u16,
            agi: char_model.agi as u16,
            vit: char_model.vit as u16,
            int: char_model.int as u16,
            dex: char_model.dex as u16,
            luk: char_model.luk as u16,
            speed: configuration.default_char_speed,
            look: Some(Look {
                hair: char_model.hair as u16,
                hair_color: char_model.hair_color as u32,
                clothes_color: char_model.clothes_color as u32,
                body: char_model.body as u32,
                weapon: char_model.weapon as u32,
                shield: char_model.shield as u32,
                head_top: char_model.head_top as u32,
                head_middle: char_model.head_mid as u32,
                head_bottom: char_model.head_bottom as u32,
                robe: char_model.robe as u32,
            }),
            zeny: char_model.zeny as u32,
            base_level: char_model.base_level as u32,
            job_level: char_model.job_level as u32,
            status_point: char_model.status_point as u32,
            skill_point: char_model.skill_point as u32,
            base_exp: char_model.base_exp as u32,
            job_exp: char_model.job_exp as u32,
            state: 0,
            is_male: char_model.sex != "F",
            size: Default::default(),
            weapons: vec![],
            equipments: vec![],
            ammo: None,
            known_skills,
            effects: vec![],
            equipment_bonuses: StatusBonuses::default(),
            temporary_bonuses: TemporaryStatusBonuses::default(),
            active_statuses: vec![],
            active_auto_bonuses: vec![],
            bonus_periodic_ticks: Default::default(),
            script_context: None,
            script_skill_grants: Default::default(),
            mob_class: Default::default(),
        }
    }

    pub fn from_mob_model(mob_model: &MobModel) -> StatusSnapshot {
        let mut snapshot = StatusSnapshot::new_for_mob(
            mob_model.id as u32,
            mob_model.hp as u32,
            mob_model.sp as u32,
            mob_model.hp as u32,
            mob_model.sp as u32,
            mob_model.str as u16,
            mob_model.agi as u16,
            mob_model.vit as u16,
            mob_model.int as u16,
            mob_model.dex as u16,
            mob_model.luk as u16,
            mob_model.atk1 as u16,
            mob_model.atk2 as u16,
            (mob_model.int + (mob_model.int / 7).pow(2)).clamp(0, u16::MAX as i32) as u16,
            (mob_model.int + (mob_model.int / 5).pow(2)).clamp(0, u16::MAX as i32) as u16,
            mob_model.speed as u16,
            mob_model.def as u16,
            mob_model.mdef as u16,
            Size::from_string(&mob_model.size),
            Element::from_string(mob_model.element.as_str()),
            MobRace::from_string(mob_model.race.as_str()),
            mob_model.element_level as u8,
        );
        snapshot.set_hit((mob_model.level + mob_model.dex).clamp(0, i16::MAX as i32) as i16);
        snapshot.set_base_level(mob_model.level.max(1) as u32);
        snapshot.set_mob_class(mob_model.battle_class());
        snapshot.set_flee((mob_model.level + mob_model.agi).clamp(0, i16::MAX as i32) as i16);
        snapshot.set_mob_groups(mob_model.race_groups.iter().filter_map(|name| MobGroup::try_from_string_ignore_case(name).or_else(|_| MobGroup::try_from_string_ignore_case(&format!("RC2_{name}"))).ok()).collect());
        snapshot
    }
}

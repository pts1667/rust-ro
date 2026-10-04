use models::enums::element::Element;
use models::enums::mob::{MobClass, MobDamageMode, MobMode, MobRace};
use models::enums::{EnumWithMaskValueU32, EnumWithStringValue};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MobModels {
    mobs: Vec<MobModel>,
}

impl From<Vec<MobModel>> for MobModels {
    fn from(mobs: Vec<MobModel>) -> Self {
        MobModels { mobs }
    }
}

impl From<MobModels> for Vec<MobModel> {
    fn from(mob_models: MobModels) -> Self {
        mob_models.mobs
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Drop {
    pub item_name: String,
    pub item_id: i32,
    pub rate: u16,
    pub is_card: bool,
}

#[derive(SettersAll, Clone, Debug, Serialize, Deserialize)]
pub struct MobModel {
    pub id: i32,
    pub name: String,
    pub name_english: String,
    pub level: i32,
    pub hp: i32,
    pub sp: i32,
    pub atk1: i32,
    pub atk2: i32,
    pub def: i32,
    pub mdef: i32,
    pub str: i32,
    pub agi: i32,
    pub vit: i32,
    pub int: i32,
    pub dex: i32,
    pub luk: i32,
    pub range1: i16,
    pub range2: i16,
    pub range3: i16,
    pub scale: i16,
    pub race: String,
    #[serde(default, alias = "class")]
    pub monster_class: Option<MobClass>,
    #[serde(default)]
    pub race_groups: Vec<String>,
    pub element: String,
    pub element_level: i8,
    pub mode: i16,
    #[serde(default)]
    pub damage_modes: Vec<MobDamageMode>,
    pub speed: i32,
    pub atk_delay: i32,
    pub atk_motion: i32,
    pub damage_motion: i32,
    pub exp: i32,
    pub job_exp: i32,
    pub drops: Vec<Drop>,
    #[serde(default)]
    pub mvp_drops: Vec<Drop>,
    pub size: String,
}

impl MobModel {
    pub fn battle_class(&self) -> MobClass {
        self.monster_class.unwrap_or_else(|| {
            if self.mode as u32 & MobMode::Boss.as_flag() != 0 {
                MobClass::Boss
            } else {
                MobClass::Normal
            }
        })
    }
}

impl Default for MobModel {
    fn default() -> Self {
        MobModel {
            id: 0,
            name: "".to_string(),
            name_english: "".to_string(),
            level: 0,
            hp: 0,
            sp: 0,
            atk1: 0,
            atk2: 0,
            def: 0,
            mdef: 0,
            str: 1,
            agi: 1,
            vit: 1,
            int: 1,
            dex: 1,
            luk: 1,
            range1: 0,
            range2: 0,
            range3: 0,
            scale: 0,
            race: "DemiHuman".to_string(),
            monster_class: None,
            race_groups: Vec::new(),
            element: Element::Neutral.as_str().to_string(),
            element_level: 0,
            mode: 0,
            damage_modes: Vec::new(),
            speed: 0,
            atk_delay: 0,
            atk_motion: 0,
            damage_motion: 0,
            exp: 0,
            job_exp: 0,
            drops: Default::default(),
            mvp_drops: Default::default(),
            size: "Medium".to_string(),
        }
    }
}

#[cfg(test)]
mod class_tests {
    use super::*;

    #[test]
    fn default_monster_data_uses_parseable_canonical_race_and_element() {
        let snapshot = crate::server::model::status::StatusFromDb::from_mob_model(&MobModel::default());
        assert_eq!(snapshot.race(), &MobRace::DemiHuman);
        assert_eq!(snapshot.element(), &Element::Neutral);
        assert_eq!(snapshot.mob_class(), &MobClass::Normal);
    }

    #[test]
    fn explicit_monster_class_overrides_legacy_immunity_and_survives_status_recalculation() {
        crate::tests::common::before_all();
        let mut model = MobModel {
            id: 1002,
            hp: 100,
            sp: 20,
            element_level: 1,
            race: "Plant".into(),
            mode: MobMode::Boss.as_flag() as i16,
            ..MobModel::default()
        };
        assert_eq!(model.battle_class(), MobClass::Boss);
        model.monster_class = Some(MobClass::Normal);
        assert_eq!(model.battle_class(), MobClass::Normal);
        for class in [
            MobClass::Normal,
            MobClass::Boss,
            MobClass::Guardian,
            MobClass::Battlefield,
            MobClass::Event,
        ] {
            model.monster_class = Some(class);
            let encoded = serde_json::to_vec(&model).unwrap();
            let decoded: MobModel = serde_json::from_slice(&encoded).unwrap();
            assert_eq!(decoded.battle_class(), class);
            let snapshot = crate::server::model::status::StatusFromDb::from_mob_model(&decoded);
            let mut mob = crate::server::state::mob::Mob::new(
                123,
                10,
                10,
                1002,
                0,
                "Test".into(),
                "Test".into(),
                0,
                snapshot,
                0,
                1,
                10,
                1000,
                100,
                1,
                2,
            );
            assert_eq!(mob.status.mob_class(), &class);
            assert_eq!(mob.status_effects.mob_class, class);
            let request =
                models::status_change::StatusChangeRequest::guaranteed(models::status_change::StatusChangeKind::Blessing, 1000, 1);
            assert!(mob.start_status(request, 0, 0).unwrap().started);
            assert_eq!(mob.status.mob_class(), &class);
        }
        let mut old = serde_json::to_value(&model).unwrap();
        old.as_object_mut().unwrap().remove("monster_class");
        assert_eq!(serde_json::from_value::<MobModel>(old).unwrap().battle_class(), MobClass::Boss);
    }
}

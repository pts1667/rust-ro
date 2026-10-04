use models::enums::EnumWithStringValue;
use models::enums::element::Element;
use models::enums::mob::MobRace;
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
    pub element: String,
    pub element_level: i8,
    pub mode: i16,
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
            race: MobRace::DemiHuman.as_str().to_string(),
            element: Element::Neutral.as_str().to_string(),
            element_level: 0,
            mode: 0,
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

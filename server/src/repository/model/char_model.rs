use database::DatabaseError;
pub use database::model::CharacterRecord as CharSelectModel;
use packets::packets::CharacterInfoNeoUnion;

pub struct CharacterInfoNeoUnionWrapped {
    pub data: CharacterInfoNeoUnion,
}

impl From<&CharSelectModel> for CharacterInfoNeoUnionWrapped {
    fn from(character: &CharSelectModel) -> Self {
        let mut character_info_neo_union = CharacterInfoNeoUnion::new(0);

        character_info_neo_union.set_gid(character.char_id as u32);
        character_info_neo_union.set_exp(character.base_exp as u32);
        character_info_neo_union.set_exp_64(character.base_exp as u64);
        character_info_neo_union.set_money(character.zeny as u32);
        character_info_neo_union.set_jobexp(character.job_exp as u32);
        character_info_neo_union.set_jobexp_64(character.job_exp as u64);
        character_info_neo_union.set_joblevel(character.job_level as u32);
        character_info_neo_union.set_bodystate(0_u32);
        character_info_neo_union.set_healthstate(0_u32);
        character_info_neo_union.set_effectstate(character.option);
        character_info_neo_union.set_virtue(character.karma);
        character_info_neo_union.set_honor(character.manner);
        character_info_neo_union.set_status_point(character.status_point as u16);
        character_info_neo_union.set_hp(character.hp as u32);
        character_info_neo_union.set_hp_16(character.hp as u16);
        character_info_neo_union.set_maxhp(character.max_hp as u32);
        character_info_neo_union.set_maxhp_16(character.max_hp as u16);
        character_info_neo_union.set_sp(character.sp as u16);
        character_info_neo_union.set_maxsp(character.max_sp as u16);
        character_info_neo_union.set_speed(100_u16); // TODO make this configurable SPEED
        character_info_neo_union.set_class(character.class as u16);
        character_info_neo_union.set_head(character.hair as u16);
        character_info_neo_union.set_body(character.body as u16);
        character_info_neo_union.set_weapon(character.weapon as u16);
        character_info_neo_union.set_level(character.base_level as u16);
        character_info_neo_union.set_skill_point(character.skill_point as u16);
        character_info_neo_union.set_head_bottom(character.head_bottom as u16);
        character_info_neo_union.set_shield(character.shield as u16);
        character_info_neo_union.set_head_top(character.head_top as u16);
        character_info_neo_union.set_head_mid(character.head_mid as u16);
        character_info_neo_union.set_hair_color(character.hair_color as u16);
        character_info_neo_union.set_body_color(character.clothes_color as u16);
        let name: String = character.name.clone();
        let mut name_as_array = [0 as char; 24];
        for (i, c) in name.chars().take(24).enumerate() {
            name_as_array[i] = c;
        }
        character_info_neo_union.set_name(name_as_array);
        character_info_neo_union.set_str(character.str as u8);
        character_info_neo_union.set_agi(character.agi as u8);
        character_info_neo_union.set_vit(character.vit as u8);
        character_info_neo_union.set_int(character.int as u8);
        character_info_neo_union.set_dex(character.dex as u8);
        character_info_neo_union.set_luk(character.luk as u8);
        character_info_neo_union.set_char_num(character.char_num as i8);
        character_info_neo_union.set_b_is_changed_char_name(character.rename as u16);
        let mut last_map: String = character.last_map.clone();
        if last_map.is_empty() {
            last_map = "prontera".to_string();
        }
        last_map += ".gat";
        let mut last_map_as_array = [0 as char; 16];
        for (i, c) in last_map.chars().take(16).enumerate() {
            last_map_as_array[i] = c;
        }
        character_info_neo_union.set_last_map(last_map_as_array);
        character_info_neo_union.set_delete_date(character.delete_date as u32);
        character_info_neo_union.set_robe(character.robe as u32);
        character_info_neo_union.set_slot_addon(0);
        character_info_neo_union.set_rename_addon(0);
        character_info_neo_union.set_sex(if character.sex.clone() == "M" { 1 } else { 0 });
        character_info_neo_union.fill_raw();
        let wrapped = CharacterInfoNeoUnionWrapped {
            data: character_info_neo_union,
        };
        wrapped
    }
}

#[derive(Debug, Default)]
pub struct CharInsertModel {
    pub account_id: i32,
    pub char_num: i16,
    pub name: String,
    pub class: i16,
    pub zeny: i32,
    pub status_point: i16,
    pub str: i16,
    pub agi: i16,
    pub vit: i16,
    pub int: i16,
    pub dex: i16,
    pub luk: i16,
    pub max_hp: i32,
    pub hp: i32,
    pub max_sp: i32,
    pub sp: i32,
    pub hair: i16,
    pub hair_color: i32,
    pub last_map: String,
    pub last_x: i16,
    pub last_y: i16,
    pub save_map: String,
    pub save_x: i16,
    pub save_y: i16,
    pub sex: String,
    pub inventory_slots: i32,
}

impl TryFrom<&CharInsertModel> for CharSelectModel {
    type Error = DatabaseError;

    fn try_from(character: &CharInsertModel) -> Result<Self, Self::Error> {
        Ok(Self {
            base_level: 1,
            job_level: 1,
            account_id: character.account_id,
            char_num: character.char_num,
            name: character.name.clone(),
            class: character.class,
            zeny: character.zeny,
            status_point: character.status_point,
            str: character.str,
            agi: character.agi,
            vit: character.vit,
            int: character.int,
            dex: character.dex,
            luk: character.luk,
            max_hp: character.max_hp,
            hp: character.hp,
            max_sp: character.max_sp,
            sp: character.sp,
            hair: character.hair,
            hair_color: i16::try_from(character.hair_color).map_err(|_| DatabaseError::new("hair_color is out of bounds".into()))?,
            last_map: character.last_map.clone(),
            last_x: character.last_x,
            last_y: character.last_y,
            save_map: character.save_map.clone(),
            save_x: character.save_x,
            save_y: character.save_y,
            sex: character.sex.clone(),
            inventory_slots: i16::try_from(character.inventory_slots)
                .map_err(|_| DatabaseError::new("inventory_slots is out of bounds".into()))?,
            ..Self::default()
        })
    }
}

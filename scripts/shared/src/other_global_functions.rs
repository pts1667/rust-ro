#![allow(
    unused_imports,
    unused_labels,
    unused_assignments,
    unused_mut,
    unused_parens,
    unused_variables,
    unreachable_code
)]

use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, constants, runtime};

pub fn f_bye(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    crate::other_global_functions::f_rand(
        ctx,
        args![
            "Bye. See you again.",
            "Later.",
            "Goodbye.",
            "Good luck!",
            "Have a nice day!",
            "Byebye!!!",
        ],
    )
}

pub fn f_canchangejob(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    Ok(Val::from(ctx.call(Function::GetSkillLv, args!["NV_BASIC"])?.number()? > 8))
}

pub fn f_canopenstorage(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    Ok(Val::from(
        !(ctx.call(Function::GetSkillLv, args!["NV_BASIC"])?.number()? < 6
            && ctx.call(Function::GetSkillLv, args!["SU_BASIC_SKILL"])?.number()? < 1),
    ))
}

pub fn f_cleargarbage(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("turtle").get()? == 20 {
        ctx.var("misc_quest")
            .set(Val::from(ctx.var("misc_quest").get()?.number()? | 65536))?;
    }
    if (ctx.var("misc_quest").get()?.number()? & 65536) != 0 {
        ctx.var("turtle").set(Val::from(0))?;
    }
    for name in [
        "adv_qsk",
        "adv_qsk2",
        "res_skill",
        "wizard_m2",
        "new_mes_flag0",
        "new_mes_flag1",
        "new_mes_flag2",
        "new_mes_flag3",
        "new_mes_flag4",
        "new_mes_flag5",
        "new_lvup0",
        "new_lvup1",
        "new_joblvup",
        "dtseligible",
    ] {
        ctx.var(name).set(Val::from(0))?;
    }
    ctx.var("misc_quest")
        .set(Val::from(ctx.var("misc_quest").get()?.number()? & !128))?;
    Ok(Val::from(0))
}

pub fn f_clearjobvar(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    for name in [
        "jblvl",
        "firstaid",
        "playdead",
        "got_bandage",
        "got_novnametag",
        "job_acolyte_q",
        "job_acolyte_q2",
        "job_archer_q",
        "job_magician_q",
        "job_merchant_q",
        "job_merchant_q2",
        "job_merchant_q3",
        "job_sword_q",
        "swtest",
        "job_thief_q",
        "supnov_q",
        "assin_q",
        "assin_q2",
        "assin_q3",
        "bsmith_q",
        "bsmith_q2",
        "hntr_q",
        "hntr_q2",
        "knight_q",
        "knight_q2",
        "priest_q",
        "priest_q2",
        "priest_q3",
        "wiz_q",
        "wiz_q2",
        "rogue_q",
        "rogue_q2",
        "alch_q",
        "alch_q2",
        "crus_q",
        "monk_q",
        "job_monk_c",
        "sage_q",
        "sage_q2",
        "danc_q",
        "bard_q",
        "taek_q",
        "tk_q",
        "stgl_q",
        "soul_q",
        "guns_q",
        "ninj_q",
    ] {
        ctx.var(name).set(Val::from(0))?;
    }
    Ok(Val::from(0))
}

pub fn f_getarmortype(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let location = ctx.call(
        Function::GetItemInfo,
        args![runtime::arg(&args, 0, Val::from(0)), constants::ITEMINFO_LOCATIONS],
    )?;
    if location == constants::EQP_HEAD_LOW {
        Ok(Val::from("Lower Headgear"))
    } else if location == constants::EQP_HAND_R {
        crate::other_global_functions::f_getweapontype(ctx, args![runtime::arg(&args, 0, Val::from(0))])
    } else if location == constants::EQP_GARMENT {
        Ok(Val::from("Garment"))
    } else if location == constants::EQP_ACC_L || location == constants::EQP_ACC_R || location == constants::EQP_ACC_RL {
        Ok(Val::from("Accessory"))
    } else if location == constants::EQP_ARMOR {
        Ok(Val::from("Armor"))
    } else if location == constants::EQP_HAND_L {
        Ok(Val::from("Shield"))
    } else if location == constants::EQP_SHOES {
        Ok(Val::from("Shoes"))
    } else if location == constants::EQP_HEAD_TOP {
        Ok(Val::from("Upper Headgear"))
    } else if location == constants::EQP_HEAD_MID {
        Ok(Val::from("Middle Headgear"))
    } else if location == constants::EQP_COSTUME_HEAD_TOP {
        Ok(Val::from("Costume Upper Headgear"))
    } else if location == constants::EQP_COSTUME_HEAD_MID {
        Ok(Val::from("Costume Midle Headgear"))
    } else if location == constants::EQP_COSTUME_HEAD_LOW {
        Ok(Val::from("Costume Lower Headgear"))
    } else if location == constants::EQP_COSTUME_GARMENT {
        Ok(Val::from("Costume Garment"))
    } else if location == constants::EQP_AMMO {
        Ok(Val::from("Ammo"))
    } else if location == constants::EQP_SHADOW_ARMOR {
        Ok(Val::from("Shadow Armor"))
    } else if location == constants::EQP_SHADOW_WEAPON {
        Ok(Val::from("Shadow Weapon"))
    } else if location == constants::EQP_SHADOW_SHIELD {
        Ok(Val::from("Shadow Shield"))
    } else if location == constants::EQP_SHADOW_SHOES {
        Ok(Val::from("Shadow Shoes"))
    } else if location == constants::EQP_SHADOW_ACC_R || location == constants::EQP_SHADOW_ACC_L || location == constants::EQP_SHADOW_ACC_RL
    {
        Ok(Val::from("Shadow Accessory"))
    } else {
        Ok(Val::from("Unknown Equip"))
    }
}

pub fn f_getnumsuffix(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let n = runtime::arg(&args, 0, Val::from(0));
    let l_mod = n.clone().try_rem(Val::from(10))?;
    if l_mod == 1 && n != 11 {
        Ok(n + Val::from("st"))
    } else if l_mod == 2 && n != 12 {
        Ok(n + Val::from("nd"))
    } else if l_mod == 3 && n != 13 {
        Ok(n + Val::from("rd"))
    } else {
        Ok(n + Val::from("th"))
    }
}

fn give_perm_skill(ctx: &Ctx, skill: &str) -> Result<(), Stop> {
    ctx.call(Function::Skill, args![skill, 1, constants::SKILL_PERM])?;
    Ok(())
}

pub fn f_getplatinumskills(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    give_perm_skill(ctx, "NV_FIRSTAID")?;
    match ctx.var("BaseClass").get()?.number()? {
        constants::JOB_NOVICE => {
            if ctx.var("Class").get()? != constants::JOB_SUPER_NOVICE {
                give_perm_skill(ctx, "NV_TRICKDEAD")?;
            }
        }
        constants::JOB_SWORDMAN => {
            give_perm_skill(ctx, "SM_MOVINGRECOVERY")?;
            give_perm_skill(ctx, "SM_FATALBLOW")?;
            give_perm_skill(ctx, "SM_AUTOBERSERK")?;
        }
        constants::JOB_MAGE => give_perm_skill(ctx, "MG_ENERGYCOAT")?,
        constants::JOB_ARCHER => {
            give_perm_skill(ctx, "AC_MAKINGARROW")?;
            give_perm_skill(ctx, "AC_CHARGEARROW")?;
        }
        constants::JOB_ACOLYTE => give_perm_skill(ctx, "AL_HOLYLIGHT")?,
        constants::JOB_MERCHANT => {
            give_perm_skill(ctx, "MC_CARTREVOLUTION")?;
            give_perm_skill(ctx, "MC_CHANGECART")?;
            give_perm_skill(ctx, "MC_LOUD")?;
            if constants::PACKETVER >= 20150826 {
                give_perm_skill(ctx, "MC_CARTDECORATE")?;
            }
        }
        constants::JOB_THIEF => {
            give_perm_skill(ctx, "TF_SPRINKLESAND")?;
            give_perm_skill(ctx, "TF_BACKSLIDING")?;
            give_perm_skill(ctx, "TF_PICKSTONE")?;
            give_perm_skill(ctx, "TF_THROWSTONE")?;
        }
        _ => {}
    }
    match ctx.var("BaseJob").get()?.number()? {
        constants::JOB_KNIGHT => give_perm_skill(ctx, "KN_CHARGEATK")?,
        constants::JOB_PRIEST => give_perm_skill(ctx, "PR_REDEMPTIO")?,
        constants::JOB_WIZARD => give_perm_skill(ctx, "WZ_SIGHTBLASTER")?,
        constants::JOB_BLACKSMITH => {
            give_perm_skill(ctx, "BS_UNFAIRLYTRICK")?;
            give_perm_skill(ctx, "BS_GREED")?;
        }
        constants::JOB_HUNTER => give_perm_skill(ctx, "HT_PHANTASMIC")?,
        constants::JOB_ASSASSIN => {
            give_perm_skill(ctx, "AS_SONICACCEL")?;
            give_perm_skill(ctx, "AS_VENOMKNIFE")?;
        }
        constants::JOB_CRUSADER => give_perm_skill(ctx, "CR_SHRINK")?,
        constants::JOB_MONK => {
            give_perm_skill(ctx, "MO_KITRANSLATION")?;
            give_perm_skill(ctx, "MO_BALKYOUNG")?;
        }
        constants::JOB_SAGE => {
            give_perm_skill(ctx, "SA_CREATECON")?;
            give_perm_skill(ctx, "SA_ELEMENTWATER")?;
            give_perm_skill(ctx, "SA_ELEMENTGROUND")?;
            give_perm_skill(ctx, "SA_ELEMENTFIRE")?;
            give_perm_skill(ctx, "SA_ELEMENTWIND")?;
        }
        constants::JOB_ROGUE => give_perm_skill(ctx, "RG_CLOSECONFINE")?,
        constants::JOB_ALCHEMIST => give_perm_skill(ctx, "AM_BIOETHICS")?,
        constants::JOB_BARD => give_perm_skill(ctx, "BA_PANGVOICE")?,
        constants::JOB_DANCER => give_perm_skill(ctx, "DC_WINKCHARM")?,
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn f_getplural(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let upper = runtime::arg(&args, 1, Val::from(0)).is_true();
    let mut l_str_s = runtime::arg(&args, 0, Val::from(0));
    let l_format_s = if runtime::countstr(&l_str_s, &Val::from(" ")).is_true() {
        let mut l_tmp_s = runtime::explode(&l_str_s, &Val::from(" "));
        let size = l_tmp_s.len() as i32;
        let mut index = 0;
        if runtime::compare(&l_str_s, &Val::from(" of ")).is_true()
            || runtime::compare(&l_str_s, &Val::from(" in ")).is_true()
            || runtime::compare(&l_str_s, &Val::from(" on ")).is_true()
        {
            let mut i = 1;
            while i < size {
                if runtime::strlen(&runtime::local_get(&l_tmp_s, &Val::from(i), true)) == 2
                    && runtime::compare(&Val::from("of|in|on"), &runtime::local_get(&l_tmp_s, &Val::from(i), true)).is_true()
                {
                    break;
                }
                index += 1;
                i += 1;
            }
        } else {
            index = size - 1;
        }
        l_str_s = runtime::local_get(&l_tmp_s, &Val::from(index), true);
        runtime::local_set(&mut l_tmp_s, &Val::from(index), Val::from("%s"), true);
        runtime::implode(&l_tmp_s, &Val::from(" "))?
    } else {
        Val::from("%s")
    };
    let len = runtime::strlen(&l_str_s).number()?;
    if len < 3 {
        return format_word(&l_format_s, &l_str_s, upper);
    }
    let mut l_suffix_s: Vec<Val> = Vec::new();
    runtime::local_set(
        &mut l_suffix_s,
        &Val::from(0),
        runtime::charat(&l_str_s, &Val::from(len - 1))?,
        true,
    );
    runtime::local_set(
        &mut l_suffix_s,
        &Val::from(1),
        runtime::substr(&l_str_s, &Val::from(len - 2), &Val::from(len - 1))?,
        true,
    );
    let last_char = runtime::local_get(&l_suffix_s, &Val::from(0), true);
    let last_two = runtime::local_get(&l_suffix_s, &Val::from(1), true);
    let result_s = if !runtime::compare(&Val::from("abcdefghijklmnopqrstuvwxyz"), &last_char).is_true() {
        l_str_s
    } else if runtime::compare(
        &Val::from("fish|glasses|sunglasses|clothes|boots|shoes|greaves|sandals|wings|ears"),
        &l_str_s,
    )
    .is_true()
    {
        l_str_s
    } else if last_char == "s" || last_char == "x" || last_char == "z" || last_two == "ch" || last_two == "sh" {
        l_str_s + Val::from("es")
    } else if (last_char == "f" || last_two == "fe") && last_two != "ff" {
        if runtime::compare(&Val::from("belief|cliff|chief|dwarf|grief|gulf|proof|roof"), &l_str_s).is_true() {
            l_str_s + Val::from("s")
        } else {
            runtime::substr(&l_str_s, &Val::from(0), &Val::from(len - 2 - (last_two == "fe") as i32))? + Val::from("ves")
        }
    } else if last_char == "y" && !runtime::compare(&Val::from("aeiou"), &runtime::charat(&last_two, &Val::from(0))?).is_true() {
        runtime::delchar(&l_str_s, &Val::from(len - 1)) + Val::from("ies")
    } else if last_char == "o"
        && runtime::compare(
            &Val::from("buffalo|domino|echo|grotto|halo|hero|mango|mosquito|potato|tomato|tornado|torpedo|veto|volcano"),
            &l_str_s,
        )
        .is_true()
    {
        l_str_s + Val::from("es")
    } else {
        l_str_s + Val::from("s")
    };
    format_word(&l_format_s, &result_s, upper)
}

fn format_word(format: &Val, word: &Val, upper: bool) -> Result<Val, Stop> {
    let text = runtime::sprintf(format, &[word.clone()])?;
    Ok(if upper { runtime::strtoupper(&text) } else { text })
}

pub fn f_getpositionname(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let subject1 = runtime::arg(&args, 0, Val::from(999));
    // Computed before the matches, as the original did: it reads eqi_shadow_armor.
    let no_case1 = !subject1.loosely_equals(&ctx.constant("EQI_ACC_L")?)
        && !subject1.loosely_equals(&ctx.constant("EQI_ACC_R")?)
        && !subject1.loosely_equals(&ctx.constant("EQI_SHOES")?)
        && !subject1.loosely_equals(&ctx.constant("EQI_GARMENT")?)
        && !subject1.loosely_equals(&ctx.constant("EQI_HEAD_LOW")?)
        && !subject1.loosely_equals(&ctx.constant("EQI_HEAD_MID")?)
        && !subject1.loosely_equals(&ctx.constant("EQI_HEAD_TOP")?)
        && !subject1.loosely_equals(&ctx.constant("EQI_ARMOR")?)
        && !subject1.loosely_equals(&ctx.constant("EQI_HAND_L")?)
        && !subject1.loosely_equals(&ctx.constant("EQI_HAND_R")?)
        && !subject1.loosely_equals(&ctx.constant("EQI_COSTUME_HEAD_TOP")?)
        && !subject1.loosely_equals(&ctx.constant("EQI_COSTUME_HEAD_MID")?)
        && !subject1.loosely_equals(&ctx.constant("EQI_COSTUME_HEAD_LOW")?)
        && !subject1.loosely_equals(&ctx.constant("EQI_COSTUME_GARMENT")?)
        && !subject1.loosely_equals(&ctx.constant("EQI_AMMO")?)
        && !subject1.loosely_equals(&ctx.var("eqi_shadow_armor").get()?)
        && !subject1.loosely_equals(&ctx.constant("EQI_SHADOW_WEAPON")?)
        && !subject1.loosely_equals(&ctx.constant("EQI_SHADOW_SHIELD")?)
        && !subject1.loosely_equals(&ctx.constant("EQI_SHADOW_SHOES")?)
        && !subject1.loosely_equals(&ctx.constant("EQI_SHADOW_ACC_R")?)
        && !subject1.loosely_equals(&ctx.constant("EQI_SHADOW_ACC_L")?);
    if subject1.loosely_equals(&ctx.constant("EQI_ACC_L")?) {
        return Ok(Val::from("Accessory 1"));
    }
    if subject1.loosely_equals(&ctx.constant("EQI_ACC_R")?) {
        return Ok(Val::from("Accessory 2"));
    }
    if subject1.loosely_equals(&ctx.constant("EQI_SHOES")?) {
        return Ok(Val::from("Shoes"));
    }
    if subject1.loosely_equals(&ctx.constant("EQI_GARMENT")?) {
        return Ok(Val::from("Robe"));
    }
    if subject1.loosely_equals(&ctx.constant("EQI_HEAD_LOW")?) {
        return Ok(Val::from("Head 3"));
    }
    if subject1.loosely_equals(&ctx.constant("EQI_HEAD_MID")?) {
        return Ok(Val::from("Head 2"));
    }
    if subject1.loosely_equals(&ctx.constant("EQI_HEAD_TOP")?) {
        return Ok(Val::from("Head"));
    }
    if subject1.loosely_equals(&ctx.constant("EQI_ARMOR")?) {
        return Ok(Val::from("Body"));
    }
    if subject1.loosely_equals(&ctx.constant("EQI_HAND_L")?) {
        return Ok(Val::from("Left hand"));
    }
    if subject1.loosely_equals(&ctx.constant("EQI_HAND_R")?) {
        return Ok(Val::from("Right hand"));
    }
    if subject1.loosely_equals(&ctx.constant("EQI_COSTUME_HEAD_TOP")?) {
        return Ok(Val::from("Upper Costume Headgear"));
    }
    if subject1.loosely_equals(&ctx.constant("EQI_COSTUME_HEAD_MID")?) {
        return Ok(Val::from("Middle Costume Headgear"));
    }
    if subject1.loosely_equals(&ctx.constant("EQI_COSTUME_HEAD_LOW")?) {
        return Ok(Val::from("Lower Costume Headgear"));
    }
    if subject1.loosely_equals(&ctx.constant("EQI_COSTUME_GARMENT")?) {
        return Ok(Val::from("Costume Garment"));
    }
    if subject1.loosely_equals(&ctx.constant("EQI_AMMO")?) {
        return Ok(Val::from("Arrow/Ammunition"));
    }
    if subject1.loosely_equals(&ctx.var("eqi_shadow_armor").get()?) {
        return Ok(Val::from("Shadow Armor"));
    }
    if subject1.loosely_equals(&ctx.constant("EQI_SHADOW_WEAPON")?) {
        return Ok(Val::from("Shadow Weapon"));
    }
    if subject1.loosely_equals(&ctx.constant("EQI_SHADOW_SHIELD")?) {
        return Ok(Val::from("Shadow Shield"));
    }
    if subject1.loosely_equals(&ctx.constant("EQI_SHADOW_SHOES")?) {
        return Ok(Val::from("Shadow Shoes"));
    }
    if subject1.loosely_equals(&ctx.constant("EQI_SHADOW_ACC_R")?) {
        return Ok(Val::from("Shadow Accessory 2"));
    }
    if subject1.loosely_equals(&ctx.constant("EQI_SHADOW_ACC_L")?) {
        return Ok(Val::from("Shadow Accessory 1"));
    }
    if no_case1 {
        return Ok(Val::from("Unknown"));
    }
    Ok(Val::from(0))
}

pub fn f_getweapontype(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let view = ctx.call(
        Function::GetItemInfo,
        args![runtime::arg(&args, 0, Val::from(0)), constants::ITEMINFO_VIEW],
    )?;
    let names: [(i32, &str); 21] = [
        (1, "Dagger"),
        (2, "One-handed Sword"),
        (3, "Two-handed Sword"),
        (4, "One-handed Spear"),
        (5, "Two-handed Spear"),
        (6, "One-handed Axe"),
        (7, "Two-handed Axe"),
        (8, "Mace"),
        (10, "Staff"),
        (11, "Bow"),
        (12, "Knuckle"),
        (13, "Instrument"),
        (14, "Whip"),
        (15, "Book"),
        (16, "Katar"),
        (17, "Revolver"),
        (18, "Rifle"),
        (19, "Gatling gun"),
        (20, "Shotgun"),
        (21, "Grenade Launcher"),
        (22, "Shuriken"),
    ];
    for (id, name) in names {
        if view == id {
            return Ok(Val::from(name));
        }
    }
    Ok(Val::from("Unknown Weapon"))
}

pub fn f_hi(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    crate::other_global_functions::f_rand(ctx, args!["Hi!", "Hello!", "Good day!", "How are you?", "Hello there."])
}

pub fn f_insertcomma(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_str_s = runtime::arg(&args, 0, Val::from(0));
    let mut l_i = runtime::strlen(&l_str_s).try_sub(Val::from(3))?;
    while l_i.number()? > 0 {
        l_str_s = runtime::insertchar(&l_str_s, &Val::from(","), &l_i)?;
        l_i = l_i.try_sub(Val::from(3))?;
    }
    Ok(l_str_s)
}

pub fn f_insertplural(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    runtime::sprintf(
        &runtime::arg(&args, 3, Val::from("%d %s")),
        &[
            runtime::arg(&args, 0, Val::from(0)),
            if runtime::arg(&args, 0, Val::from(0)) == 1 {
                runtime::arg(&args, 1, Val::from(0))
            } else {
                crate::other_global_functions::f_getplural(
                    ctx,
                    args![runtime::arg(&args, 1, Val::from(0)), runtime::arg(&args, 2, Val::from(0))],
                )?
            },
        ],
    )
}

pub fn f_ischarm(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let id = runtime::arg(&args, 0, Val::from(0)).number()?;
    Ok(Val::from(
        (4700..=4999).contains(&id) || (29000..=29689).contains(&id) || (310000..=311091).contains(&id),
    ))
}

pub fn f_isequipcardhack(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let pos = runtime::arg(&args, 0, Val::from(0));
    for i in 0..4 {
        let card = runtime::arg(&args, i + 1, Val::from(0));
        let card_chk = ctx.call(Function::GetEquipCardId, args![pos.clone(), i])?;
        if !card.loosely_equals(&card_chk) {
            return Ok(Val::from(1));
        }
    }
    Ok(Val::from(0))
}

pub fn f_isequipidhack(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let id_chk = ctx.call(Function::GetEquipId, args![runtime::arg(&args, 0, Val::from(0))])?;
    let id = runtime::arg(&args, 1, Val::from(0));
    if !id.loosely_equals(&id_chk) {
        return Ok(Val::from(1));
    }
    Ok(Val::from(0))
}

pub fn f_isequiprefinehack(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let refine_chk = ctx.call(Function::GetEquipRefineryCnt, args![runtime::arg(&args, 0, Val::from(0))])?;
    let refine = runtime::arg(&args, 1, Val::from(0));
    if !refine.loosely_equals(&refine_chk) {
        return Ok(Val::from(1));
    }
    Ok(Val::from(0))
}

pub fn f_itemname(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let element = runtime::arg(&args, 1, Val::from(0));
    let grade = runtime::arg(&args, 2, Val::from(0));
    let refine = runtime::arg(&args, 3, Val::from(0));
    let mut l_t_s = if refine.is_true() {
        (Val::from("+") + refine) + Val::from(" ")
    } else {
        Val::from("")
    };
    if grade == 1 {
        l_t_s = l_t_s + Val::from("VS ");
    } else if grade == 2 {
        l_t_s = l_t_s + Val::from("VVS ");
    } else if grade == 3 {
        l_t_s = l_t_s + Val::from("VVVS ");
    } else if grade != 0 {
        l_t_s = (l_t_s + grade) + Val::from("xVS ");
    }
    if element == 1 {
        l_t_s = l_t_s + Val::from("Ice ");
    } else if element == 2 {
        l_t_s = l_t_s + Val::from("Earth ");
    } else if element == 3 {
        l_t_s = l_t_s + Val::from("Fire ");
    } else if element == 4 {
        l_t_s = l_t_s + Val::from("Wind ");
    } else if element != 0 {
        l_t_s = l_t_s + Val::from("Strange ");
    }
    Ok(Val::from("^000090") + l_t_s + ctx.call(Function::GetItemName, args![runtime::arg(&args, 0, Val::from(0))])? + Val::from("^000000"))
}

pub fn f_load1skills(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    for i in 0..14 {
        if runtime::op(
            &ctx.var("adv_qsk").get()?,
            "|",
            &Val::from(ctx.call(Function::Pow, args![2, i])?.loosely_equals(&ctx.var("adv_qsk").get()?)),
        )?
        .is_true()
        {
            ctx.call(Function::Skill, args![144 + i, 1, 0])?;
        }
    }
    ctx.var("adv_qsk").set(Val::from(0))?;
    Ok(Val::from(0))
}

pub fn f_load2skills(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    for i in 0..19 {
        if runtime::op(
            &ctx.var("adv_qsk2").get()?,
            "|",
            &Val::from(ctx.call(Function::Pow, args![2, i])?.loosely_equals(&ctx.var("adv_qsk2").get()?)),
        )?
        .is_true()
        {
            ctx.call(Function::Skill, args![1001 + i, 1, 0])?;
        }
    }
    ctx.var("adv_qsk2").set(Val::from(0))?;
    Ok(Val::from(0))
}

pub fn f_rand(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    Ok(runtime::arg(
        &args,
        ctx.call(Function::Rand, vec![Val::from(args.len() as i32)])?.number()?,
        Val::from(0),
    ))
}

pub fn f_savequestskills(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.var("adv_qsk").set(Val::from(0))?;
    ctx.var("adv_qsk2").set(Val::from(0))?;
    for i in 0..14 {
        if ctx.call(Function::GetSkillLv, args![144 + i])?.is_true() {
            ctx.var("adv_qsk").set(runtime::op(
                &ctx.var("adv_qsk").get()?,
                "|",
                &ctx.call(Function::Pow, args![2, i])?,
            )?)?;
        }
    }
    for i in 0..19 {
        if ctx.call(Function::GetSkillLv, args![1001 + i])?.is_true() {
            ctx.var("adv_qsk2").set(runtime::op(
                &ctx.var("adv_qsk2").get()?,
                "|",
                &ctx.call(Function::Pow, args![2, i])?,
            )?)?;
        }
    }
    Ok(Val::from(0))
}

pub fn f_sexmes(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    Ok(runtime::arg(&args, ctx.var("Sex").get()?.number()?, Val::from(0)))
}

pub fn job_change(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::JobChange,
        args![runtime::arg(&args, 0, Val::from(0)), ctx.var("Upper").get()?],
    )?;
    Ok(Val::from(0))
}

pub fn time2str(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_time_left = runtime::arg(&args, 0, Val::from(0)).try_sub(ctx.call(Function::GetTimeTick, args![2])?)?;
    let l_days = l_time_left.clone().try_div(Val::from(86400))?;
    l_time_left = l_time_left.try_sub(l_days.clone().try_mul(Val::from(86400))?)?;
    let l_hours = l_time_left.clone().try_div(Val::from(3600))?;
    l_time_left = l_time_left.try_sub(l_hours.clone().try_mul(Val::from(3600))?)?;
    let l_minutes = l_time_left.clone().try_div(Val::from(60))?;
    l_time_left = l_time_left.try_sub(l_minutes.clone().try_mul(Val::from(60))?)?;
    let mut l_time_s = Val::from("");
    if l_days.number()? > 1 {
        l_time_s = l_time_s + l_days + Val::from(" days, ");
    } else if l_days.number()? > 0 {
        l_time_s = l_time_s + l_days + Val::from(" day, ");
    }
    if l_hours.number()? > 1 {
        l_time_s = l_time_s + l_hours + Val::from(" hours, ");
    } else if l_hours.number()? > 0 {
        l_time_s = l_time_s + l_hours + Val::from(" hour, ");
    }
    if l_minutes.number()? > 1 {
        l_time_s = l_time_s + l_minutes + Val::from(" minutes, ");
    } else if l_minutes.number()? > 0 {
        l_time_s = l_time_s + l_minutes + Val::from(" minute, ");
    }
    if l_time_left.number()? > 1 || l_time_left == 0 {
        l_time_s = l_time_s + l_time_left + Val::from(" seconds");
    } else if l_time_left == 1 {
        l_time_s = l_time_s + l_time_left + Val::from(" second");
    }
    Ok(l_time_s)
}

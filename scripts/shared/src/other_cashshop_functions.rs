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

pub fn f_cashcity(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    'b1: {
        let subject1 = runtime::arg(&args, 0, Val::from(0));
        let mut matched1 = false;
        if !matched1 && subject1 == 1 {
            matched1 = true;
        }
        if matched1 {
            match ctx.menu(&["Prontera", "Izlude", "Geffen", "Payon", "Morocc", "Alberta", "Al de Baran"])? {
                0 => {
                    ctx.warp("prontera", 122, 87)?;
                    return Err(Stop::End);
                }
                1 => {
                    ctx.warp("izlude", 91, 105)?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.warp("geffen", 128, 48)?;
                    return Err(Stop::End);
                }
                3 => {
                    ctx.warp("payon", 164, 123)?;
                    return Err(Stop::End);
                }
                4 => {
                    ctx.warp("morocc", 160, 100)?;
                    return Err(Stop::End);
                }
                5 => {
                    ctx.warp("alberta", 117, 50)?;
                    return Err(Stop::End);
                }
                6 => {
                    ctx.warp("aldebaran", 140, 110)?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        }
        if !matched1 && subject1 == 2 {
            matched1 = true;
        }
        if matched1 {
            match ctx.menu(&["Juno", "Lighthalzen", "Einbroch", "Einbech", "Hugel"])? {
                0 => {
                    ctx.warp("yuno", 160, 170)?;
                    return Err(Stop::End);
                }
                1 => {
                    ctx.warp("lighthalzen", 190, 310)?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.warp("einbroch", 230, 190)?;
                    return Err(Stop::End);
                }
                3 => {
                    ctx.warp("einbech", 187, 120)?;
                    return Err(Stop::End);
                }
                4 => {
                    ctx.warp("hugel", 92, 165)?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        }
        if !matched1 && subject1 == 3 {
            matched1 = true;
        }
        if matched1 {
            match ctx.menu(&["Rachel", "Veins"])? {
                0 => {
                    ctx.warp("rachel", 120, 125)?;
                    return Err(Stop::End);
                }
                1 => {
                    ctx.warp("veins", 215, 105)?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        }
        if !matched1 && subject1 == 4 {
            matched1 = true;
        }
        if matched1 {
            match ctx.menu(&["Amatsu", "Kunlun", "Ayothaya", "Luoyang"])? {
                0 => {
                    ctx.warp("amatsu", 110, 140)?;
                    return Err(Stop::End);
                }
                1 => {
                    ctx.warp("gonryun", 160, 115)?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.warp("ayothaya", 220, 170)?;
                    return Err(Stop::End);
                }
                3 => {
                    ctx.warp("louyang", 217, 95)?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        }
        if !matched1 && subject1 == 5 {
            matched1 = true;
        }
        if matched1 {
            match ctx.menu(&["Amatsu", "Kunlun", "Ayothaya", "Luoyang", "Moscovia", "Dewata", "Brasilis"])? {
                0 => {
                    ctx.warp("amatsu", 110, 140)?;
                    return Err(Stop::End);
                }
                1 => {
                    ctx.warp("gonryun", 160, 115)?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.warp("ayothaya", 220, 170)?;
                    return Err(Stop::End);
                }
                3 => {
                    ctx.warp("louyang", 217, 95)?;
                    return Err(Stop::End);
                }
                4 => {
                    ctx.warp("moscovia", 224, 195)?;
                    return Err(Stop::End);
                }
                5 => {
                    ctx.warp("dewata", 200, 107)?;
                    return Err(Stop::End);
                }
                6 => {
                    ctx.warp("brasilis", 196, 181)?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        }
    }
    return Ok(Val::from(0));
}

pub fn f_cashdungeon(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    'b1: {
        let subject1 = runtime::arg(&args, 0, Val::from(0));
        let mut matched1 = false;
        if !matched1 && subject1 == 1 {
            matched1 = true;
        }
        if matched1 {
            match ctx.menu(&[
                "Nogg Road",
                "Mjolnir Dead Pit",
                "Umbala Dungeon",
                "Einbroch Mine Dungeon",
                "Payon Dungeon",
                "Toy Dungeon",
                "Glast Heim Underprison",
                "Luoyang Dungeon",
                "Hermit's Checkers",
                "Izlude Dungeon",
                "Turtle Island Dungeon",
                "Clock Tower B3f",
                "Clock Tower 3f",
                "Glast Heim Culvert 2f",
                "Sphinx Dungeon 4f",
                "Inside Pyramid 4f",
                "Prontera Culvert 3f",
                "Amatsu Dungeon 1f (Tatami Maze)",
                "Somatology Laboratory 1st floor",
                "Ayothya Ancient Shrine 2nd floor",
            ])? {
                0 => {
                    ctx.warp("mag_dun01", 125, 71)?;
                    return Err(Stop::End);
                }
                1 => {
                    ctx.warp("mjo_dun02", 80, 297)?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.warp("um_dun02", 125, 122)?;
                    return Err(Stop::End);
                }
                3 => {
                    ctx.warp("ein_dun01", 261, 262)?;
                    return Err(Stop::End);
                }
                4 => {
                    ctx.warp("pay_dun03", 155, 150)?;
                    return Err(Stop::End);
                }
                5 => {
                    ctx.warp("xmas_dun01", 133, 130)?;
                    return Err(Stop::End);
                }
                6 => {
                    ctx.warp("gl_prison", 140, 15)?;
                    return Err(Stop::End);
                }
                7 => {
                    ctx.warp("lou_dun03", 165, 38)?;
                    return Err(Stop::End);
                }
                8 => {
                    ctx.warp("gon_dun02", 251, 263)?;
                    return Err(Stop::End);
                }
                9 => {
                    ctx.warp("iz_dun02", 350, 335)?;
                    return Err(Stop::End);
                }
                10 => {
                    ctx.warp("tur_dun02", 165, 30)?;
                    return Err(Stop::End);
                }
                11 => {
                    ctx.warp("alde_dun03", 275, 180)?;
                    return Err(Stop::End);
                }
                12 => {
                    ctx.warp("c_tower3", 34, 42)?;
                    return Err(Stop::End);
                }
                13 => {
                    ctx.warp("gl_sew02", 292, 295)?;
                    return Err(Stop::End);
                }
                14 => {
                    ctx.warp("in_sphinx4", 120, 120)?;
                    return Err(Stop::End);
                }
                15 => {
                    ctx.warp("moc_pryd04", 195, 4)?;
                    return Err(Stop::End);
                }
                16 => {
                    ctx.warp("prt_sewb3", 20, 175)?;
                    return Err(Stop::End);
                }
                17 => {
                    ctx.warp("ama_dun01", 222, 144)?;
                    return Err(Stop::End);
                }
                18 => {
                    ctx.warp("lhz_dun01", 19, 153)?;
                    return Err(Stop::End);
                }
                19 => {
                    ctx.warp("ayo_dun02", 70, 240)?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        }
        if !matched1 && subject1 == 2 {
            matched1 = true;
        }
        if matched1 {
            match ctx.menu(&[
                "Thor Volcano 2f",
                "Ice Dungeon Entrance",
                "Nameless Island Entrance",
                "Niflheim",
                "Labyrinth Forest 2f",
                "Ruins of Juperos Entrance",
                "Ant Hell 2f",
                "Kiel Hyre's Academy Entrance",
                "Thanatos Tower Entrance",
                "Abyss Lake Entrance",
                "Rachel Sanctuary Entrance",
                "Odin Temple 2f",
            ])? {
                0 => {
                    ctx.warp("thor_v02", 77, 208)?;
                    return Err(Stop::End);
                }
                1 => {
                    ctx.warp("ra_fild01", 237, 333)?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.warp("ve_fild07", 127, 131)?;
                    return Err(Stop::End);
                }
                3 => {
                    ctx.warp("niflheim", 206, 179)?;
                    return Err(Stop::End);
                }
                4 => {
                    ctx.warp("prt_maze02", 100, 174)?;
                    return Err(Stop::End);
                }
                5 => {
                    ctx.warp("jupe_cave", 36, 54)?;
                    return Err(Stop::End);
                }
                6 => {
                    ctx.warp("anthell02", 36, 265)?;
                    return Err(Stop::End);
                }
                7 => {
                    ctx.warp("yuno_fild08", 70, 171)?;
                    return Err(Stop::End);
                }
                8 => {
                    ctx.warp("hu_fild01", 140, 160)?;
                    return Err(Stop::End);
                }
                9 => {
                    ctx.warp("hu_fild05", 168, 302)?;
                    return Err(Stop::End);
                }
                10 => {
                    ctx.warp("ra_temple", 117, 173)?;
                    return Err(Stop::End);
                }
                11 => {
                    ctx.warp("odin_tem02", 257, 374)?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        }
        if !matched1 && subject1 == 3 {
            matched1 = true;
        }
        if matched1 {
            match ctx.menu(&[
                "Bio Lab (2nd Floor)",
                "Ice Dungeon (3rd Floor)",
                "Odin Temple (3rd Floor)",
                "Thor Volcano (3rd Floor)",
                "Abyss Lake (3rd Floor)",
                "Juperos Ruins (2nd Floor)",
            ])? {
                0 => {
                    ctx.warp("lhz_dun02", 145, 149)?;
                    return Err(Stop::End);
                }
                1 => {
                    ctx.warp("ice_dun03", 150, 176)?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.warp("odin_tem03", 278, 235)?;
                    return Err(Stop::End);
                }
                3 => {
                    ctx.warp("thor_v03", 144, 170)?;
                    return Err(Stop::End);
                }
                4 => {
                    ctx.warp("abyss_03", 97, 104)?;
                    return Err(Stop::End);
                }
                5 => {
                    ctx.warp("juperos_02", 130, 159)?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        }
    }
    return Ok(Val::from(0));
}

pub fn f_cashpartycall(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx
        .call(Function::IsPartyLeader, args![ctx.call(Function::GetCharacterId, args![1])?])?
        .is_true()
    {
        ctx.call(
            Function::PartyWarp,
            args![
                "RandomAll",
                0,
                0,
                ctx.call(Function::GetCharacterId, args![1])?,
                ctx.call(Function::StrCharInfo, args![3])?,
                3,
                3
            ],
        )?;
    } else {
        ctx.call(Function::ItemSkill, args!["AL_TELEPORT", 1])?;
    }
    return Ok(Val::from(0));
}

pub fn f_cashreset(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("Class").get()? != constants::JOB_NOVICE
        && ctx.var("Weight").get()? == 0
        && !ctx.call(Function::CheckRiding, args![])?.is_true()
        && !ctx.call(Function::CheckFalcon, args![])?.is_true()
        && !ctx.call(Function::CheckCart, args![])?.is_true()
        && !ctx.call(Function::IsMounting, args![])?.is_true()
    {
        ctx.call(Function::ResetSkills, args![])?;
        ctx.call(Function::EndStatus, args![ctx.constant("SC_ALL")?])?;
        if ctx.call(Function::CountItem, args![12213])?.is_true() {
            ctx.items().take(12213, 1)?;
        }
    }
    return Ok(Val::from(0));
}

pub fn f_cashsiegetele(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    match ctx.menu(&[
        "Neuschwanstein (aldeg_cas01)",
        "Hohenschwangau (aldeg_cas02)",
        "Nuernberg (aldeg_cas03)",
        "Wuerzburg (aldeg_cas04)",
        "Rothenburg (aldeg_cas05)",
        "Repherion (gefg_cas01)",
        "Eeyorbriggar (gefg_cas02)",
        "Yesnelph (gefg_cas03)",
        "Bergel (gefg_cas04)",
        "Mersetzdeitz (gefg_cas05)",
        "Bright Arbor (payg_cas01)",
        "Sacred Altar (payg_cas02)",
        "Holy Shadow (payg_cas03)",
        "Scarlet Palace (payg_cas04)",
        "Bamboo Grove Hill (payg_cas05)",
        "Kriemhild (prtg_cas01)",
        "Swanhild (prtg_cas02)",
        "Fadhgridh (prtg_cas03)",
        "Skoegul (prtg_cas04)",
        "Gondul (prtg_cas05)",
    ])? {
        0 => {
            ctx.warp("alde_gld", 48, 91)?;
            return Err(Stop::End);
        }
        1 => {
            ctx.warp("alde_gld", 103, 245)?;
            return Err(Stop::End);
        }
        2 => {
            ctx.warp("alde_gld", 142, 87)?;
            return Err(Stop::End);
        }
        3 => {
            ctx.warp("alde_gld", 236, 243)?;
            return Err(Stop::End);
        }
        4 => {
            ctx.warp("alde_gld", 269, 90)?;
            return Err(Stop::End);
        }
        5 => {
            ctx.warp("gef_fild13", 217, 75)?;
            return Err(Stop::End);
        }
        6 => {
            ctx.warp("gef_fild13", 307, 237)?;
            return Err(Stop::End);
        }
        7 => {
            ctx.warp("gef_fild13", 77, 297)?;
            return Err(Stop::End);
        }
        8 => {
            ctx.warp("gef_fild13", 190, 276)?;
            return Err(Stop::End);
        }
        9 => {
            ctx.warp("gef_fild13", 312, 91)?;
            return Err(Stop::End);
        }
        10 => {
            ctx.warp("pay_gld", 121, 232)?;
            return Err(Stop::End);
        }
        11 => {
            ctx.warp("pay_gld", 297, 116)?;
            return Err(Stop::End);
        }
        12 => {
            ctx.warp("pay_gld", 318, 293)?;
            return Err(Stop::End);
        }
        13 => {
            ctx.warp("pay_gld", 140, 164)?;
            return Err(Stop::End);
        }
        14 => {
            ctx.warp("pay_gld", 202, 264)?;
            return Err(Stop::End);
        }
        15 => {
            ctx.warp("prt_gld", 141, 64)?;
            return Err(Stop::End);
        }
        16 => {
            ctx.warp("prt_gld", 240, 131)?;
            return Err(Stop::End);
        }
        17 => {
            ctx.warp("prt_gld", 153, 133)?;
            return Err(Stop::End);
        }
        18 => {
            ctx.warp("prt_gld", 126, 240)?;
            return Err(Stop::End);
        }
        19 => {
            ctx.warp("prt_gld", 195, 240)?;
            return Err(Stop::End);
        }
        _ => {}
    }
    return Ok(Val::from(0));
}

pub fn f_cashsiegetele2(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    match ctx.menu(&[
        "Mardoll (arug_cas01)",
        "Cyr (arug_cas02)",
        "Horn (arug_cas03)",
        "Gefn (arug_cas04)",
        "Badanis (arug_cas05)",
        "Himinn (schg_cas01)",
        "Andlangr (schg_cas02)",
        "Vidblainn (schg_cas03)",
        "Hljod (schg_cas04)",
        "Schatirnil (schg_cas05)",
    ])? {
        0 => {
            ctx.warp("aru_gld", 159, 272)?;
            return Err(Stop::End);
        }
        1 => {
            ctx.warp("aru_gld", 86, 50)?;
            return Err(Stop::End);
        }
        2 => {
            ctx.warp("aru_gld", 63, 157)?;
            return Err(Stop::End);
        }
        3 => {
            ctx.warp("aru_gld", 307, 354)?;
            return Err(Stop::End);
        }
        4 => {
            ctx.warp("aru_gld", 298, 110)?;
            return Err(Stop::End);
        }
        5 => {
            ctx.warp("sch_gld", 300, 100)?;
            return Err(Stop::End);
        }
        6 => {
            ctx.warp("sch_gld", 284, 249)?;
            return Err(Stop::End);
        }
        7 => {
            ctx.warp("sch_gld", 102, 196)?;
            return Err(Stop::End);
        }
        8 => {
            ctx.warp("sch_gld", 140, 83)?;
            return Err(Stop::End);
        }
        9 => {
            ctx.warp("sch_gld", 70, 320)?;
            return Err(Stop::End);
        }
        _ => {}
    }
    return Ok(Val::from(0));
}

pub fn f_cashstore(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.fx().cutin("kafra_01", 2)?;
    ctx.lines_as(
        "Kafra Employee",
        args!["Welcome to the Kafra Corporation.", "Here, let me open your Storage for you."],
    )?;
    ctx.close_window()?;
    ctx.call(Function::OpenStorage, args![])?;
    ctx.fx().cutin("", 255)?;
    return Ok(Val::from(0));
}

pub fn f_cashtele(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    'b1: {
        let subject1 = runtime::arg(&args, 0, Val::from(0));
        let mut matched1 = false;
        if !matched1 && subject1 == 1 {
            matched1 = true;
        }
        if matched1 {
            match ctx.menu(&["Prontera", "Geffen", "Al de Baran", "Izlude", "Savepoint"])? {
                0 => {
                    ctx.warp("prontera", 119, 77)?;
                    return Err(Stop::End);
                }
                1 => {
                    ctx.warp("geffen", 119, 39)?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.warp("aldebaran", 165, 107)?;
                    return Err(Stop::End);
                }
                3 => {
                    ctx.warp("izlude", 91, 105)?;
                    return Err(Stop::End);
                }
                4 => {
                    ctx.warp("SavePoint", 0, 0)?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        }
        if !matched1 && subject1 == 2 {
            matched1 = true;
        }
        if matched1 {
            match ctx.menu(&["Payon", "Alberta", "Morocc", "Comodo", "Savepoint"])? {
                0 => {
                    ctx.warp("payon", 158, 55)?;
                    return Err(Stop::End);
                }
                1 => {
                    ctx.warp("alberta", 115, 57)?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.warp("morocc", 158, 48)?;
                    return Err(Stop::End);
                }
                3 => {
                    ctx.warp("comodo", 217, 148)?;
                    return Err(Stop::End);
                }
                4 => {
                    ctx.warp("SavePoint", 0, 0)?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        }
        if !matched1 && subject1 == 3 {
            matched1 = true;
        }
        if matched1 {
            match ctx.menu(&["Luoyang", "Amatsu", "Kunlun Field", "Ayothaya", "Savepoint"])? {
                0 => {
                    ctx.warp("louyang", 214, 101)?;
                    return Err(Stop::End);
                }
                1 => {
                    ctx.warp("amatsu", 112, 145)?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.warp("gonryun", 160, 118)?;
                    return Err(Stop::End);
                }
                3 => {
                    ctx.warp("ayothaya", 216, 175)?;
                    return Err(Stop::End);
                }
                4 => {
                    ctx.warp("SavePoint", 0, 0)?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        }
        if !matched1 && subject1 == 4 {
            matched1 = true;
        }
        if matched1 {
            match ctx.menu(&["Lutie Field", "Umbala", "Niflheim", "Savepoint"])? {
                0 => {
                    ctx.warp("xmas", 148, 131)?;
                    return Err(Stop::End);
                }
                1 => {
                    ctx.warp("umbala", 93, 154)?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.warp("niflheim", 187, 189)?;
                    return Err(Stop::End);
                }
                3 => {
                    ctx.warp("SavePoint", 0, 0)?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        }
        if !matched1 && subject1 == 5 {
            matched1 = true;
        }
        if matched1 {
            match ctx.menu(&["Juno", "Einbroch", "Lighthalzen", "Hugel", "Savepoint"])? {
                0 => {
                    ctx.warp("yuno", 157, 124)?;
                    return Err(Stop::End);
                }
                1 => {
                    ctx.warp("einbroch", 230, 192)?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.warp("lighthalzen", 158, 94)?;
                    return Err(Stop::End);
                }
                3 => {
                    ctx.warp("hugel", 93, 159)?;
                    return Err(Stop::End);
                }
                4 => {
                    ctx.warp("SavePoint", 0, 0)?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        }
        if !matched1 && subject1 == 6 {
            matched1 = true;
        }
        if matched1 {
            match ctx.menu(&["Rachel", "Veins", "Savepoint"])? {
                0 => {
                    ctx.warp("rachel", 118, 132)?;
                    return Err(Stop::End);
                }
                1 => {
                    ctx.warp("veins", 214, 125)?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.warp("SavePoint", 0, 0)?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        }
    }
    return Ok(Val::from(0));
}

pub fn f_snowball(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.fx().cutin("rutie_snownow03", 2)?;
    ctx.lines_as("Snowman", args!["Merry Christmas!"])?;
    match ctx.menu(&[
        "Restore Some HP/SP",
        "Strengthen My Body!",
        "Restore My SP Fully.",
        "Return to Savepoint",
    ])? {
        0 => {
            ctx.lines(args!["Aha!", "Quiet night~"])?;
            ctx.call(Function::PercentHeal, args![70, 70])?;
            ctx.close_window()?;
            ctx.fx().cutin("", 255)?;
        }
        1 => {
            ctx.lines(args!["Blessings.", "Holy night~"])?;
            ctx.call(
                Function::UnitSkillToId,
                args![ctx.call(Function::GetCharacterId, args![3])?, "SM_ENDURE", 10],
            )?;
            ctx.close_window()?;
            ctx.fx().cutin("", 255)?;
        }
        2 => {
            ctx.lines(args!["Hey!", "White Christmas~"])?;
            ctx.call(Function::PercentHeal, args![0, 100])?;
            ctx.close_window()?;
            ctx.fx().cutin("", 255)?;
        }
        3 => {
            ctx.lines(args!["Jingle Bells~", "Jingle Bells~ Jingle Bells!"])?;
            ctx.close_window()?;
            ctx.fx().cutin("", 255)?;
            ctx.warp("SavePoint", 0, 0)?;
        }
        _ => {}
    }
    return Err(Stop::End);
}

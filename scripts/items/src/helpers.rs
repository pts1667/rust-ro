//! The `callfunc` helpers of item scripts, ported from the numeric ABI `functions.rs`.

use script_sdk_2::{Function, ItemUse, Stop, Val};

type Destination = (&'static str, &'static str, i32, i32);

pub fn call(item: &ItemUse, arguments: Vec<Val>) -> Result<Val, Stop> {
    let name = arguments.first().ok_or_else(|| error("Missing compiled helper name"))?.text();
    let rest = &arguments[1..];
    match name.as_str() {
        "F_Rand" => {
            if rest.is_empty() {
                return Err(error("F_Rand requires at least one value"));
            }
            let index = item.call(Function::Rand, vec![Val::from(rest.len() as i32)])?.number()? as usize;
            return rest.get(index).cloned().ok_or_else(|| error("Random selection was out of bounds"));
        }
        "F_CashStore" => {
            item.call(Function::Cutin, vec![Val::from("kafra_01"), Val::from(2)])?;
            item.mes("[Kafra Employee]\nWelcome to the Kafra Corporation.\nHere, let me open your Storage for you.")?;
            item.close_window()?;
            item.call(Function::OpenStorage, vec![])?;
            item.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
        }
        "F_CashPartyCall" => {
            let party = item.call(Function::GetCharacterId, vec![Val::from(1)])?;
            let map = item.call(Function::StrCharInfo, vec![Val::from(3)])?;
            item.call(Function::PartyWarp, vec![Val::from("RandomAll"), Val::from(0), Val::from(0), party, map, Val::from(3), Val::from(3)])?;
        }
        "F_CashReset" => {
            let has_option = [Function::CheckRiding, Function::CheckFalcon, Function::CheckCart, Function::IsMounting]
                .into_iter()
                .map(|function| item.call(function, vec![]))
                .collect::<Result<Vec<_>, _>>()?
                .iter()
                .any(Val::is_true);
            if item.read_any("Class")?.number()? != 0 && item.read_any("Weight")?.number()? == 0 && !has_option {
                item.call(Function::ResetSkills, vec![])?;
                item.call(Function::EndStatus, vec![Val::from("SC_ALL")])?;
                if item.call(Function::CountItem, vec![Val::from(12213)])?.is_true() {
                    item.call(Function::DelItem, vec![Val::from(12213), Val::from(1)])?;
                }
            } else {
                return Err(error("Skills can only be reset with an empty inventory and no active mounts or companions"));
            }
        }
        "F_CashDungeon" => dungeon(item, rest.first().ok_or_else(|| error("Missing dungeon scroll type"))?.number()?)?,
        "F_CashCity" => city(item, rest.first().ok_or_else(|| error("Missing city wing type"))?.number()?)?,
        "F_Snowball" => {
            item.call(Function::Cutin, vec![Val::from("rutie_snownow03"), Val::from(2)])?;
            item.mes("[Snowman]\nMerry Christmas!")?;
            match item.menu(&["Restore Some HP/SP", "Strengthen My Body!", "Restore My SP Fully.", "Return to Savepoint"])? {
                0 => {
                    item.call(Function::PercentHeal, vec![Val::from(70), Val::from(70)])?;
                }
                1 => {
                    item.call(Function::UnitSkill, vec![Val::from("SM_ENDURE"), Val::from(10)])?;
                }
                2 => {
                    item.call(Function::PercentHeal, vec![Val::from(0), Val::from(100)])?;
                }
                3 => {
                    item.call(Function::Warp, vec![Val::from("SavePoint"), Val::from(0), Val::from(0)])?;
                }
                _ => return Err(error("Invalid snowball selection")),
            }
            item.close_window()?;
            item.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
        }
        _ => return Err(error(&format!("Unknown compiled helper {name}"))),
    }
    Ok(Val::from(0))
}

fn error(message: &str) -> Stop {
    Stop::Error(message.into())
}

fn choose_destination(item: &ItemUse, destinations: &[Destination]) -> Result<(), Stop> {
    let options = destinations.iter().map(|destination| destination.0).collect::<Vec<_>>();
    let (_, map, x, y) = destinations[item.menu(&options)?];
    item.call(Function::Warp, vec![Val::from(map), Val::from(x), Val::from(y)])?;
    Ok(())
}

fn city(item: &ItemUse, kind: i32) -> Result<(), Stop> {
    let destinations: &[Destination] = match kind {
        1 => &[("Prontera", "prontera", 122, 87), ("Izlude", "izlude", 91, 105), ("Geffen", "geffen", 128, 48),
            ("Payon", "payon", 164, 123), ("Morocc", "morocc", 160, 100), ("Alberta", "alberta", 117, 50), ("Al de Baran", "aldebaran", 140, 110)],
        2 => &[("Juno", "yuno", 160, 170), ("Lighthalzen", "lighthalzen", 190, 310), ("Einbroch", "einbroch", 230, 190),
            ("Einbech", "einbech", 187, 120), ("Hugel", "hugel", 92, 165)],
        3 => &[("Rachel", "rachel", 120, 125), ("Veins", "veins", 215, 105)],
        4 | 5 => &[("Amatsu", "amatsu", 110, 140), ("Kunlun", "gonryun", 160, 115), ("Ayothaya", "ayothaya", 220, 170), ("Luoyang", "louyang", 217, 95)],
        _ => return Err(error("Unknown city wing type")),
    };
    choose_destination(item, destinations)
}

fn dungeon(item: &ItemUse, kind: i32) -> Result<(), Stop> {
    let destinations: &[Destination] = match kind {
        1 => &[("Nogg Road", "mag_dun01", 125, 71), ("Mjolnir Dead Pit", "mjo_dun02", 80, 297), ("Umbala Dungeon", "um_dun02", 125, 122),
            ("Einbroch Mine", "ein_dun01", 261, 262), ("Payon Dungeon", "pay_dun03", 155, 150), ("Toy Dungeon", "xmas_dun01", 133, 130),
            ("Glast Heim Underprison", "gl_prison", 140, 15), ("Luoyang Dungeon", "lou_dun03", 165, 38), ("Hermit's Checkers", "gon_dun02", 251, 263),
            ("Izlude Dungeon", "iz_dun02", 350, 335), ("Turtle Island", "tur_dun02", 165, 30), ("Clock Tower B3F", "alde_dun03", 275, 180),
            ("Clock Tower 3F", "c_tower3", 34, 42), ("Glast Heim Culvert", "gl_sew02", 292, 295), ("Sphinx 4F", "in_sphinx4", 120, 120),
            ("Pyramid 4F", "moc_pryd04", 195, 4), ("Prontera Culvert 3F", "prt_sewb3", 20, 175), ("Amatsu Dungeon", "ama_dun01", 222, 144),
            ("Bio Lab 1F", "lhz_dun01", 19, 153), ("Ayothaya Shrine", "ayo_dun02", 70, 240)],
        2 => &[("Thor Volcano 2F", "thor_v02", 77, 208), ("Ice Dungeon Entrance", "ra_fild01", 237, 333), ("Nameless Island Entrance", "ve_fild07", 127, 131),
            ("Niflheim", "niflheim", 206, 179), ("Labyrinth Forest 2F", "prt_maze02", 100, 174), ("Juperos Entrance", "jupe_cave", 36, 54),
            ("Ant Hell 2F", "anthell02", 36, 265), ("Kiel Academy", "yuno_fild08", 70, 171), ("Thanatos Entrance", "hu_fild01", 140, 160),
            ("Abyss Lake Entrance", "hu_fild05", 168, 302), ("Rachel Sanctuary", "ra_temple", 117, 173), ("Odin Temple 2F", "odin_tem02", 257, 374)],
        3 => &[("Bio Lab 2F", "lhz_dun02", 145, 149), ("Ice Dungeon 3F", "ice_dun03", 150, 176), ("Odin Temple 3F", "odin_tem03", 278, 235),
            ("Thor Volcano 3F", "thor_v03", 144, 170), ("Abyss Lake 3F", "abyss_03", 97, 104), ("Juperos Ruins 2F", "juperos_02", 130, 159)],
        _ => return Err(error("Unknown dungeon scroll type")),
    };
    choose_destination(item, destinations)
}

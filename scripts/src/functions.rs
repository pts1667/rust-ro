use script_sdk::{Context, Function, Reply, Value};

pub fn call(ctx: &Context, arguments: Vec<Value>) -> Reply {
    let name = arguments.first().ok_or("Missing compiled helper name")?.text();
    let arguments = &arguments[1..];
    match name.as_str() {
        "F_Rand" => {
            if arguments.is_empty() { return Err("F_Rand requires at least one value".into()); }
            let index = ctx.call(Function::Rand, vec![(arguments.len() as i32).into()])?.number_value()? as usize;
            return arguments.get(index).cloned().ok_or_else(|| "Random selection was out of bounds".into());
        }
        "F_CashStore" => {
            ctx.call(Function::Cutin, vec!["kafra_01".into(), 2.into()])?;
            ctx.mes("[Kafra Employee]\nWelcome to the Kafra Corporation.\nHere, let me open your Storage for you.")?;
            ctx.close()?;
            ctx.call(Function::OpenStorage, vec![])?;
            ctx.call(Function::Cutin, vec!["".into(), 255.into()])?;
        }
        "F_CashPartyCall" => {
            let party = ctx.call(Function::GetCharacterId, vec![1.into()])?;
            let map = ctx.call(Function::StrCharInfo, vec![3.into()])?;
            ctx.call(Function::PartyWarp, vec!["RandomAll".into(), 0.into(), 0.into(), party, map, 3.into(), 3.into()])?;
        }
        "F_CashReset" => {
            let has_option = [Function::CheckRiding, Function::CheckFalcon, Function::CheckCart, Function::IsMounting]
                .into_iter().map(|function| ctx.call(function, vec![])).collect::<Result<Vec<_>, _>>()?.iter().any(Value::truthy);
            if ctx.read("Class")?.number_value()? != 0 && ctx.read("Weight")?.number_value()? == 0 && !has_option {
                ctx.call(Function::ResetSkills, vec![])?;
                ctx.call(Function::EndStatus, vec!["SC_ALL".into()])?;
                if ctx.call(Function::CountItem, vec![12213.into()])?.truthy() {
                    ctx.call(Function::DelItem, vec![12213.into(), 1.into()])?;
                }
            } else {
                return Err("Skills can only be reset with an empty inventory and no active mounts or companions".into());
            }
        }
        "F_CashDungeon" => dungeon(ctx, arguments.first().ok_or("Missing dungeon scroll type")?.number_value()?)?,
        "F_CashCity" => city(ctx, arguments.first().ok_or("Missing city wing type")?.number_value()?)?,
        "F_Snowball" => {
            ctx.call(Function::Cutin, vec!["rutie_snownow03".into(), 2.into()])?;
            ctx.mes("[Snowman]\nMerry Christmas!")?;
            let selected = ctx.select(&["Restore Some HP/SP".into(), "Strengthen My Body!".into(), "Restore My SP Fully.".into(), "Return to Savepoint".into()])?;
            match selected {
                0 => { ctx.call(Function::PercentHeal, vec![70.into(), 70.into()])?; }
                1 => { ctx.call(Function::UnitSkill, vec!["SM_ENDURE".into(), 10.into()])?; }
                2 => { ctx.call(Function::PercentHeal, vec![0.into(), 100.into()])?; }
                3 => { ctx.call(Function::Warp, vec!["SavePoint".into(), 0.into(), 0.into()])?; }
                _ => return Err("Invalid snowball selection".into()),
            }
            ctx.close()?;
            ctx.call(Function::Cutin, vec!["".into(), 255.into()])?;
        }
        _ => return Err(format!("Unknown compiled helper {name}")),
    }
    Ok(Value::default())
}

type Destination = (&'static str, &'static str, i32, i32);

fn choose_destination(ctx: &Context, destinations: &[Destination]) -> Result<(), String> {
    let options = destinations.iter().map(|destination| destination.0.to_string()).collect::<Vec<_>>();
    let selected = ctx.select(&options)?;
    let (_, map, x, y) = destinations[selected];
    ctx.call(Function::Warp, vec![map.into(), x.into(), y.into()])?;
    Ok(())
}

fn city(ctx: &Context, kind: i32) -> Result<(), String> {
    let destinations: &[Destination] = match kind {
        1 => &[("Prontera", "prontera", 122, 87), ("Izlude", "izlude", 91, 105), ("Geffen", "geffen", 128, 48),
            ("Payon", "payon", 164, 123), ("Morocc", "morocc", 160, 100), ("Alberta", "alberta", 117, 50), ("Al de Baran", "aldebaran", 140, 110)],
        2 => &[("Juno", "yuno", 160, 170), ("Lighthalzen", "lighthalzen", 190, 310), ("Einbroch", "einbroch", 230, 190),
            ("Einbech", "einbech", 187, 120), ("Hugel", "hugel", 92, 165)],
        3 => &[("Rachel", "rachel", 120, 125), ("Veins", "veins", 215, 105)],
        4 | 5 => &[("Amatsu", "amatsu", 110, 140), ("Kunlun", "gonryun", 160, 115), ("Ayothaya", "ayothaya", 220, 170), ("Luoyang", "louyang", 217, 95)],
        _ => return Err("Unknown city wing type".into()),
    };
    choose_destination(ctx, destinations)
}

fn dungeon(ctx: &Context, kind: i32) -> Result<(), String> {
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
        _ => return Err("Unknown dungeon scroll type".into()),
    };
    choose_destination(ctx, destinations)
}

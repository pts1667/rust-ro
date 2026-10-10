use script_sdk_2::constants::{
    EAJL_2, EAJL_2_1, EAJL_2_2, EAJL_UPPER, EAJ_BASEMASK, JOB_ACOLYTE, JOB_ACOLYTE_HIGH, JOB_ARCHER, JOB_ARCHER_HIGH, JOB_GUNSLINGER, JOB_MAGE,
    JOB_MAGE_HIGH, JOB_MERCHANT, JOB_MERCHANT_HIGH, JOB_NINJA, JOB_NOVICE, JOB_NOVICE_HIGH, JOB_SUPER_NOVICE, JOB_SWORDMAN, JOB_SWORDMAN_HIGH,
    JOB_TAEKWON, JOB_THIEF, JOB_THIEF_HIGH, LOOK_CLOTHES_COLOR, LOOK_HAIR, LOOK_HAIR_COLOR,
};
use script_sdk_2::{Bound, Ctx, Script, Stop, VariableScope};
use serde::Deserialize;

pub fn counter(ctx: &Ctx) -> Script {
    let counters = ctx.increment(&[(VariableScope::Npc, "counter", 1), (VariableScope::NpcInstance, "counter", 1)])?;
    ctx.mes(&format!("Npc counter variable: {}\nNPC instance counter variable: {}", counters[0], counters[1]))?;
    ctx.next()?;
    ctx.mes("Close")?;
    ctx.close()
}

/// Writes a four element array in each persistent scope and reads its second element back.
pub fn variables(ctx: &Ctx) -> Script {
    for text in [false, true] {
        for prefix in ["", "#", "$"] {
            let name = format!("{prefix}c{}", if text { "$" } else { "" });
            let array = ctx.var(&name);
            if text {
                array.set_array((1..=4).map(|element| format!("{element}s")))?;
            } else {
                array.set_array(1..=4)?;
            }
            ctx.mes(&format!("{} {name} array index 1", array.get_at(1)?.text()))?;
            ctx.next()?;
        }
    }
    ctx.close()
}

#[derive(Deserialize)]
struct Destination {
    category: String,
    group: String,
    name: String,
    map: String,
    x: i32,
    y: i32,
}

const WARP_CATEGORIES: [&str; 7] = ["Towns", "Fields", "Dungeons", "Guild Castles", "Guild Dungeons", "Instances", "Special Areas"];

pub fn warper(ctx: &Ctx) -> Script {
    let destinations: Vec<Destination> = serde_json::from_str(include_str!("../../../config/wasm/warps.json")).map_err(|error| Stop::Error(error.to_string()))?;
    let last = ctx.var("lastwarp$").get()?.text();
    let mut menu = vec![format!("Last Warp [{last}]")];
    menu.extend(WARP_CATEGORIES.iter().map(|category| category.to_string()));
    let category = ctx.menu(&menu)?;
    if category == 0 {
        if last.is_empty() {
            ctx.mes("You haven't warped anywhere yet.")?;
            return ctx.close();
        }
        let x = ctx.var("lastwarpx").get()?.number()?;
        let y = ctx.var("lastwarpy").get()?.number()?;
        ctx.warp(&last, x, y)?;
        return ctx.close();
    }
    let category = WARP_CATEGORIES[category - 1];
    let mut groups: Vec<&str> = vec![];
    for destination in destinations.iter().filter(|destination| destination.category == category) {
        if !groups.contains(&destination.group.as_str()) {
            groups.push(&destination.group);
        }
    }
    let group = groups[ctx.menu(&groups)?];
    let choices: Vec<&Destination> = destinations.iter().filter(|destination| destination.category == category && destination.group == group).collect();
    let selected = if choices.len() == 1 { 0 } else { ctx.menu(&choices.iter().map(|destination| destination.name.as_str()).collect::<Vec<_>>())? };
    let destination = choices[selected];
    ctx.var("lastwarp$").set(destination.map.as_str())?;
    ctx.var("lastwarpx").set(destination.x)?;
    ctx.var("lastwarpy").set(destination.y)?;
    ctx.warp(&destination.map, destination.x, destination.y)?;
    ctx.close()
}

pub fn stylist(ctx: &Ctx) -> Script {
    let selection = ctx.menu(&["Cloth color", "Hairstyle", "Hair color"])?;
    let flag = ["max_cloth_color", "max_hair_style", "max_hair_color"][selection];
    let look = [LOOK_CLOTHES_COLOR, LOOK_HAIR, LOOK_HAIR_COLOR][selection];
    let maximum = ctx.battle_flag(flag)?;
    let original = ctx.player().look(look)?;
    let mut style = 1;
    loop {
        ctx.player().set_look(look, style)?;
        ctx.player().message(&format!("This is style #{style}."))?;
        let next = if style < maximum { style + 1 } else { 1 };
        let previous = if style > 1 { style - 1 } else { maximum };
        let options = [format!("Next ({next})"), format!("Previous ({previous})"), "Jump to...".into(), format!("Revert to original ({original})"), "Finish".into()];
        match ctx.menu(&options)? {
            0 => style = next,
            1 => style = previous,
            2 => {
                ctx.player().message(&format!("Choose a style between 1 - {maximum}."))?;
                let input = ctx.input_number(0, maximum)?;
                style = if input.bound == Bound::Within { input.value } else { ctx.rand_range(1, maximum)? };
            }
            3 => style = original,
            _ => return ctx.close(),
        }
    }
}

const FIRST_JOBS: [i32; 10] = [JOB_SWORDMAN, JOB_MAGE, JOB_ARCHER, JOB_ACOLYTE, JOB_MERCHANT, JOB_THIEF, JOB_SUPER_NOVICE, JOB_TAEKWON, JOB_GUNSLINGER, JOB_NINJA];
const HIGH_FIRST_JOBS: [i32; 6] = [JOB_SWORDMAN_HIGH, JOB_MAGE_HIGH, JOB_ARCHER_HIGH, JOB_ACOLYTE_HIGH, JOB_MERCHANT_HIGH, JOB_THIEF_HIGH];

pub fn job_master(ctx: &Ctx) -> Script {
    ctx.mes("[Job Master]")?;
    if ctx.var("SkillPoint").get()?.number()? > 0 {
        ctx.mes("Please use your remaining skill points first.")?;
        return ctx.close();
    }
    let class = ctx.player().class()?;
    let eac = ctx.ea_class(None)?;
    let last_job = ctx.var("lastJob").get()?.number()?;
    let mut choices = vec![];
    let (base_level, job_level);
    if eac & EAJL_2 != 0 && eac & EAJL_UPPER == 0 && ctx.ro_class(eac | EAJL_UPPER)? >= 0 {
        choices.push(JOB_NOVICE_HIGH);
        (base_level, job_level) = (99, 50);
    } else if class == JOB_NOVICE || class == JOB_NOVICE_HIGH {
        (base_level, job_level) = (1, 10);
        if class == JOB_NOVICE_HIGH && last_job > 0 {
            let last_mask = ctx.ea_class(Some(last_job))?;
            choices.push(ctx.ro_class((last_mask & EAJ_BASEMASK) | EAJL_UPPER)?);
        } else if class == JOB_NOVICE {
            choices.extend(FIRST_JOBS);
        } else {
            choices.extend(HIGH_FIRST_JOBS);
        }
    } else if eac & EAJL_2 == 0 {
        (base_level, job_level) = (1, 40);
        if eac & EAJL_UPPER != 0 && last_job > 0 {
            choices.push(last_job + JOB_NOVICE_HIGH);
        } else {
            for branch in [EAJL_2_1, EAJL_2_2] {
                let candidate = ctx.ro_class(eac | branch)?;
                if candidate > 0 {
                    choices.push(candidate);
                }
            }
        }
    } else {
        ctx.mes("No more jobs are available.")?;
        return ctx.close();
    }
    if ctx.player().base_level()? < base_level || ctx.player().job_level()? < job_level {
        ctx.mes(&format!("You need base level {base_level} and job level {job_level} to continue."))?;
        return ctx.close();
    }
    if choices.is_empty() {
        ctx.mes("No more jobs are available.")?;
        return ctx.close();
    }
    let mut names = vec![];
    for choice in &choices {
        names.push(if *choice == JOB_NOVICE_HIGH { "Rebirth".into() } else { ctx.job_name(*choice)? });
    }
    names.push("Cancel".into());
    let selected = ctx.menu(&names)?;
    if selected == choices.len() {
        return ctx.close();
    }
    let target = choices[selected];
    if target == JOB_SUPER_NOVICE && ctx.player().base_level()? < 45 {
        ctx.mes("A base level of 45 is required to turn into a Super Novice.")?;
        return ctx.close();
    }
    ctx.next()?;
    ctx.mes(&format!("Do you want to change into {}?", names[selected]))?;
    if ctx.menu(&["Change class", "Cancel"])? == 0 {
        if target == JOB_NOVICE_HIGH {
            ctx.var("lastJob").set(class)?;
        }
        ctx.player().change_job(target)?;
        if target == JOB_NOVICE_HIGH {
            ctx.player().reset_level(1)?;
        }
        ctx.mes(&format!("You are now a {}!", names[selected]))?;
    }
    ctx.close()
}

pub fn mount_master(ctx: &Ctx) -> Script {
    ctx.mes("[Mount Master]")?;
    let married = ctx.player().partner_id(None)? != 0;
    let options = ["Toggle cart", "Toggle falcon", "Toggle Peco Peco", if married { "Divorce" } else { "Marry" }, "Cancel"];
    let player = ctx.player();
    match ctx.menu(&options)? {
        0 => player.set_cart(!player.has_cart(None)?)?,
        1 => player.set_falcon(!player.has_falcon(None)?)?,
        2 => player.set_riding(!player.is_riding(None)?)?,
        3 if married => {
            player.divorce()?;
            ctx.mes("You are no longer married.")?;
        }
        3 => {
            ctx.mes("Enter the name of your online partner.")?;
            let partner = ctx.input_text(0, usize::MAX)?.value;
            ctx.mes(if player.marry(&partner)? { "Congratulations!" } else { "The marriage could not be performed." })?;
        }
        _ => {}
    }
    ctx.close()
}

/// Rents a falcon to hunters or a Peco Peco to knights and crusaders. Its placement gives the title, the animal
/// (`"falcon"` or `"peco"`), the job, the fee and the job's name.
pub fn breeder(ctx: &Ctx) -> Script {
    let arguments = ctx.arguments()?;
    let [title, kind, job, price, job_name] = arguments.as_slice() else {
        return Err("Breeder arguments are invalid".into());
    };
    let (title, falcon, job_name) = (title.text(), kind.text() == "falcon", job_name.text());
    let (job, price) = (job.number()?, price.number()?);
    let (animal, skill) = if falcon { ("Falcon", "HT_FALCON") } else { ("Peco Peco", "KN_RIDING") };
    let player = ctx.player();
    if ctx.var("BaseJob").get()?.number()? != job {
        ctx.mes_as(&title, &format!("This {animal} rental service is strictly for {job_name}s."))?;
        return ctx.close();
    }
    ctx.mes_as(&title, &format!("Would you like to rent a {animal}? The rental fee is {price} zeny."))?;
    ctx.next()?;
    if ctx.menu(&[format!("Rent {animal}"), "Cancel".into()])? != 0 {
        return ctx.close();
    }
    ctx.mes(&format!("[{title}]"))?;
    let zeny = player.zeny()?;
    if zeny < price {
        ctx.mes("You do not have enough zeny.")?;
    } else if player.skill_level(skill)? == 0 {
        ctx.mes(&format!("You must first learn the {} skill before I can rent one to you.", if falcon { "Falcon Mastery" } else { "Peco Peco Ride" }))?;
    } else if if falcon { player.has_falcon(None)? } else { player.is_riding(None)? } {
        ctx.mes(&format!("You already have a {animal}."))?;
    } else if !falcon && player.is_mounting(None)? {
        ctx.mes("Please remove your cash mount.")?;
    } else {
        player.set_zeny(zeny - price)?;
        if falcon {
            player.set_falcon(true)?;
        } else {
            player.set_riding(true)?;
        }
    }
    ctx.close()
}

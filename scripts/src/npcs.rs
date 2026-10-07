use script_sdk::{Context, Function, Request, Value, Variable, VariableScope};
use serde::Deserialize;

pub fn run(ctx: &Context, id: u32) -> Result<(), String> {
    match id {
        1 => counter(ctx),
        2 => variables(ctx),
        3 => warper(ctx),
        4 => stylist(ctx),
        5 => job_master(ctx),
        6 => ctx.call(Function::Shop, vec![]).map(|_| ()),
        7 => mount_master(ctx),
        8 => crate::castle_npcs::steward(ctx),
        9 => crate::castle_npcs::lever(ctx),
        10 => crate::castle_npcs::kafra(ctx),
        11 => crate::wedding_npcs::staff(ctx),
        12 => crate::wedding_npcs::bishop(ctx),
        13 => crate::wedding_npcs::divorce(ctx),
        14 => breeder(ctx),
        15 => crate::battleground_arena::npc(ctx),
        16 => crate::battleground_kvm::npc(ctx),
        17 => crate::battleground_tierra::npc(ctx),
        18 => crate::battleground_npcs::npc(ctx),
        19 => crate::castle_npcs::flag(ctx),
        10_000..=99_999 => crate::generated::run_npc(ctx, id).unwrap_or_else(|| Err(format!("Unknown NPC script {id}"))),
        _ => Err(format!("Unknown NPC script {id}")),
    }
}

fn counter(ctx: &Context) -> Result<(), String> {
    let Value::Array(counters) = ctx.request(Request::VariablesIncrement(vec![
        Variable {
            scope: VariableScope::Npc,
            name: "counter".into(),
            index: 0,
            value: 1.into(),
        },
        Variable {
            scope: VariableScope::NpcInstance,
            name: "counter".into(),
            index: 0,
            value: 1.into(),
        },
    ]))?
    else {
        return Err("Invalid NPC counters".into());
    };
    let shared = counters.first().ok_or("Missing NPC counter")?.number_value()?;
    let instance = counters.get(1).ok_or("Missing instance counter")?.number_value()?;
    ctx.mes(format!(
        "Npc counter variable: {shared}\nNPC instance counter variable: {instance}"
    ))?;
    ctx.next()?;
    ctx.mes("Close")?;
    ctx.close()
}

fn variables(ctx: &Context) -> Result<(), String> {
    for string in [false, true] {
        for (scope, label) in [
            (VariableScope::Character, "c"),
            (VariableScope::Account, "#c"),
            (VariableScope::Server, "$c"),
        ] {
            let name = if string { "c$" } else { "c" };
            ctx.request(Request::VariablesWrite(
                (0..4)
                    .map(|index| Variable {
                        scope,
                        name: name.into(),
                        index,
                        value: if string {
                            Value::String(format!("{}s", index + 1))
                        } else {
                            Value::Number(index as i32 + 1)
                        },
                    })
                    .collect(),
            ))?;
            let value = ctx.request(Request::VariableRead {
                scope,
                name: name.into(),
                index: 1,
            })?;
            ctx.mes(format!("{value} {label}{} array index 1", if string { "$" } else { "" }))?;
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

fn warper(ctx: &Context) -> Result<(), String> {
    let destinations: Vec<Destination> = serde_json::from_str(include_str!("../../config/wasm/warps.json")).map_err(|e| e.to_string())?;
    let categories = [
        "Towns",
        "Fields",
        "Dungeons",
        "Guild Castles",
        "Guild Dungeons",
        "Instances",
        "Special Areas",
    ];
    let last = ctx.read("lastwarp$")?.text();
    let mut menu = vec![format!("Last Warp [{last}]")];
    menu.extend(categories.iter().map(|s| (*s).to_string()));
    let category = ctx.select(&menu)?;
    if category == 0 {
        if last.is_empty() {
            ctx.mes("You haven't warped anywhere yet.")?;
            return ctx.close();
        }
        ctx.call(Function::Warp, vec![
            last.into(),
            ctx.read("lastwarpx")?,
            ctx.read("lastwarpy")?,
        ])?;
        return ctx.close();
    }
    let mut groups = vec![];
    for destination in destinations.iter().filter(|d| d.category == categories[category - 1]) {
        if !groups.contains(&destination.group) {
            groups.push(destination.group.clone());
        }
    }
    let selected_group = ctx.select(&groups)?;
    let choices: Vec<_> = destinations
        .iter()
        .filter(|d| d.category == categories[category - 1] && d.group == groups[selected_group])
        .collect();
    let selected = if choices.len() == 1 {
        0
    } else {
        ctx.select(&choices.iter().map(|d| d.name.clone()).collect::<Vec<_>>())?
    };
    let destination = choices[selected];
    ctx.request(Request::VariablesWrite(vec![
        Variable {
            scope: VariableScope::Character,
            name: "lastwarp$".into(),
            index: 0,
            value: destination.map.clone().into(),
        },
        Variable {
            scope: VariableScope::Character,
            name: "lastwarpx".into(),
            index: 0,
            value: destination.x.into(),
        },
        Variable {
            scope: VariableScope::Character,
            name: "lastwarpy".into(),
            index: 0,
            value: destination.y.into(),
        },
    ]))?;
    ctx.call(Function::Warp, vec![
        destination.map.clone().into(),
        destination.x.into(),
        destination.y.into(),
    ])?;
    ctx.close()
}

fn number(ctx: &Context, function: Function, arguments: Vec<Value>) -> Result<i32, String> {
    ctx.call(function, arguments)?.number_value()
}

fn stylist(ctx: &Context) -> Result<(), String> {
    let selection = ctx.select(&["Cloth color".into(), "Hairstyle".into(), "Hair color".into()])?;
    let flag = ["max_cloth_color", "max_hair_style", "max_hair_color"][selection];
    let look = ctx.constant(["LOOK_CLOTHES_COLOR", "LOOK_HAIR", "LOOK_HAIR_COLOR"][selection])?;
    let maximum = number(ctx, Function::GetBattleFlag, vec![flag.into()])?;
    let original = number(ctx, Function::GetLook, vec![look.clone()])?;
    let mut style = 1;
    loop {
        ctx.call(Function::SetLook, vec![look.clone(), style.into()])?;
        ctx.call(Function::Message, vec![format!("This is style #{style}.").into()])?;
        let next = if style < maximum { style + 1 } else { 1 };
        let previous = if style > 1 { style - 1 } else { maximum };
        match ctx.select(&[
            format!("Next ({next})"),
            format!("Previous ({previous})"),
            "Jump to...".into(),
            format!("Revert to original ({original})"),
            "Finish".into(),
        ])? {
            0 => style = next,
            1 => style = previous,
            2 => {
                ctx.call(Function::Message, vec![format!("Choose a style between 1 - {maximum}.").into()])?;
                style = number(ctx, Function::InputNumber, vec![])?;
                if !(0..=maximum).contains(&style) {
                    style = number(ctx, Function::Rand, vec![1.into(), maximum.into()])?;
                }
            }
            3 => style = original,
            _ => return ctx.close(),
        }
    }
}

fn job_master(ctx: &Context) -> Result<(), String> {
    ctx.mes("[Job Master]")?;
    if ctx.read("SkillPoint")?.number_value()? > 0 {
        ctx.mes("Please use your remaining skill points first.")?;
        return ctx.close();
    }
    let class = ctx.read("Class")?.number_value()?;
    let eac = number(ctx, Function::EaClass, vec![])?;
    let base_mask = ctx.constant("EAJ_BASEMASK")?.number_value()?;
    let upper_mask = ctx.constant("EAJL_UPPER")?.number_value()?;
    let second_mask = ctx.constant("EAJL_2")?.number_value()?;
    let last_job = ctx.read("lastJob")?.number_value()?;
    let novice = ctx.constant("Job_Novice")?.number_value()?;
    let high_novice = ctx.constant("Job_Novice_High")?.number_value()?;
    let mut choices = vec![];
    let requirement;
    if eac & second_mask != 0 && eac & upper_mask == 0 && number(ctx, Function::RoClass, vec![(eac | upper_mask).into()])? >= 0 {
        choices.push(high_novice);
        requirement = (99, 50);
    } else if class == novice || class == high_novice {
        requirement = (1, 10);
        if class == high_novice && last_job > 0 {
            let last_mask = number(ctx, Function::EaClass, vec![last_job.into()])?;
            choices.push(number(ctx, Function::RoClass, vec![
                ((last_mask & base_mask) | upper_mask).into(),
            ])?);
        } else {
            for name in if class == novice {
                vec![
                    "Job_Swordsman",
                    "Job_Mage",
                    "Job_Archer",
                    "Job_Acolyte",
                    "Job_Merchant",
                    "Job_Thief",
                    "Job_Super_Novice",
                    "Job_Taekwon",
                    "Job_Gunslinger",
                    "Job_Ninja",
                ]
            } else {
                vec![
                    "Job_Swordsman_High",
                    "Job_Mage_High",
                    "Job_Archer_High",
                    "Job_Acolyte_High",
                    "Job_Merchant_High",
                    "Job_Thief_High",
                ]
            } {
                choices.push(ctx.constant(name)?.number_value()?);
            }
        }
    } else if eac & second_mask == 0 {
        requirement = (1, 40);
        if eac & upper_mask != 0 && last_job > 0 {
            choices.push(last_job + high_novice);
        } else {
            for flag in ["EAJL_2_1", "EAJL_2_2"] {
                let candidate = number(ctx, Function::RoClass, vec![(eac | ctx.constant(flag)?.number_value()?).into()])?;
                if candidate > 0 {
                    choices.push(candidate);
                }
            }
        }
    } else {
        ctx.mes("No more jobs are available.")?;
        return ctx.close();
    }
    if ctx.read("BaseLevel")?.number_value()? < requirement.0 || ctx.read("JobLevel")?.number_value()? < requirement.1 {
        ctx.mes(format!(
            "You need base level {} and job level {} to continue.",
            requirement.0, requirement.1
        ))?;
        return ctx.close();
    }
    if choices.is_empty() {
        ctx.mes("No more jobs are available.")?;
        return ctx.close();
    }
    let mut names = vec![];
    for choice in &choices {
        names.push(if *choice == high_novice {
            "Rebirth".into()
        } else {
            ctx.call(Function::JobName, vec![(*choice).into()])?.text()
        });
    }
    names.push("Cancel".into());
    let selected = ctx.select(&names)?;
    if selected == choices.len() {
        return ctx.close();
    }
    let target = choices[selected];
    if target == ctx.constant("Job_Super_Novice")?.number_value()? && ctx.read("BaseLevel")?.number_value()? < 45 {
        ctx.mes("A base level of 45 is required to turn into a Super Novice.")?;
        return ctx.close();
    }
    ctx.next()?;
    ctx.mes(format!("Do you want to change into {}?", names[selected]))?;
    if ctx.select(&["Change class".into(), "Cancel".into()])? == 0 {
        if target == high_novice {
            ctx.write("lastJob", class.into())?;
        }
        ctx.call(Function::JobChange, vec![target.into()])?;
        if target == high_novice {
            ctx.call(Function::ResetLevel, vec![1.into()])?;
        }
        ctx.mes(format!("You are now a {}!", names[selected]))?;
    }
    ctx.close()
}

fn mount_master(ctx: &Context) -> Result<(), String> {
    ctx.mes("[Mount Master]")?;
    let mut options = vec!["Toggle cart".to_string(), "Toggle falcon".into(), "Toggle Peco Peco".into()];
    let married = number(ctx, Function::GetPartnerId, vec![])? != 0;
    options.push(if married { "Divorce".into() } else { "Marry".into() });
    options.push("Cancel".into());
    match ctx.select(&options)? {
        0 => {
            let cart = number(ctx, Function::CheckCart, vec![])? != 0;
            ctx.call(Function::SetCart, vec![i32::from(!cart).into()])?;
        }
        1 => {
            let falcon = number(ctx, Function::CheckFalcon, vec![])? != 0;
            ctx.call(Function::SetFalcon, vec![i32::from(!falcon).into()])?;
        }
        2 => {
            let riding = number(ctx, Function::CheckRiding, vec![])? != 0;
            ctx.call(Function::SetRiding, vec![i32::from(!riding).into()])?;
        }
        3 if married => {
            ctx.call(Function::Divorce, vec![])?;
            ctx.mes("You are no longer married.")?;
        }
        3 => {
            ctx.mes("Enter the name of your online partner.")?;
            let name = ctx.call(Function::InputString, vec![])?.text();
            if number(ctx, Function::Marriage, vec![name.into()])? == 1 {
                ctx.mes("Congratulations!")?;
            } else {
                ctx.mes("The marriage could not be performed.")?;
            }
        }
        _ => {}
    }
    ctx.close()
}

fn breeder(ctx: &Context) -> Result<(), String> {
    let Value::Array(args) = ctx.request(Request::Arguments)? else {
        return Err("NPC arguments are invalid".into());
    };
    let [title, kind, job, price, job_name] = args.as_slice() else {
        return Err("Breeder arguments are invalid".into());
    };
    let (title, kind, job_name) = (format!("[{}]", title.text()), kind.text(), job_name.text());
    let (job, price) = (job.number_value()?, price.number_value()?);
    let falcon = kind == "falcon";
    let (animal, skill) = if falcon { ("Falcon", "HT_FALCON") } else { ("Peco Peco", "KN_RIDING") };
    ctx.mes(&title)?;
    if ctx.read("BaseJob")?.number_value()? != job {
        ctx.mes(format!("This {animal} rental service is strictly for {job_name}s."))?;
        return ctx.close();
    }
    ctx.mes(format!("Would you like to rent a {animal}? The rental fee is {price} zeny."))?;
    ctx.next()?;
    if ctx.select(&[format!("Rent {animal}"), "Cancel".into()])? != 0 {
        return ctx.close();
    }
    ctx.mes(&title)?;
    let zeny = ctx.read("Zeny")?.number_value()?;
    if zeny < price {
        ctx.mes("You do not have enough zeny.")?;
    } else if number(ctx, Function::GetSkillLv, vec![skill.into()])? == 0 {
        ctx.mes(format!("You must first learn the {} skill before I can rent one to you.", if falcon { "Falcon Mastery" } else { "Peco Peco Ride" }))?;
    } else if number(ctx, if falcon { Function::CheckFalcon } else { Function::CheckRiding }, vec![])? != 0 {
        ctx.mes(format!("You already have a {animal}."))?;
    } else if !falcon && number(ctx, Function::IsMounting, vec![])? != 0 {
        ctx.mes("Please remove your cash mount.")?;
    } else {
        ctx.write("Zeny", (zeny - price).into())?;
        ctx.call(if falcon { Function::SetFalcon } else { Function::SetRiding }, vec![])?;
    }
    ctx.close()
}

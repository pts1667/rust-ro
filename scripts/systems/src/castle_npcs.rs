use script_sdk::{Context, Function, Request, Value};

const CASTLE_GUILD: i32 = 1;
const CASTLE_ECONOMY: i32 = 2;
const CASTLE_DEFENSE: i32 = 3;
const CASTLE_INVESTED_ECONOMY: i32 = 4;
const CASTLE_INVESTED_DEFENSE: i32 = 5;
const CASTLE_KAFRA: i32 = 9;
const CASTLE_GUARDIAN: i32 = 10;
const GD_KAFRACONTRACT: i32 = 10001;
const GD_GUARDRESEARCH: i32 = 10002;
const ECONOMY_COSTS: [i32; 20] = [
    5000, 10000, 20000, 35000, 55000, 80000, 110000, 145000, 185000, 230000, 280000, 335000, 395000, 460000, 530000, 605000, 685000,
    770000, 860000, 955000,
];
const DEFENSE_COSTS: [i32; 20] = [
    10000, 20000, 40000, 70000, 110000, 160000, 220000, 290000, 370000, 460000, 560000, 670000, 790000, 920000, 1060000, 1210000,
    1370000, 1540000, 1720000, 1910000,
];

fn number(ctx: &Context, function: Function, arguments: Vec<Value>) -> Result<i32, String> {
    ctx.call(function, arguments)?.number_value()
}

fn arguments(ctx: &Context) -> Result<Vec<Value>, String> {
    match ctx.request(Request::Arguments)? {
        Value::Array(values) => Ok(values),
        _ => Err("NPC arguments are invalid".into()),
    }
}

fn argument(args: &[Value], index: usize) -> Result<&Value, String> {
    args.get(index).ok_or_else(|| "NPC argument is missing".to_string())
}

fn castle_data(ctx: &Context, map: &str, field: i32) -> Result<i32, String> {
    number(ctx, Function::GetCastleData, vec![map.into(), field.into()])
}

fn set_castle_data(ctx: &Context, map: &str, field: i32, value: i32) -> Result<(), String> {
    ctx.call(Function::SetCastleData, vec![map.into(), field.into(), value.into()]).map(|_| ())
}

fn guild_name(ctx: &Context, guild: i32) -> Result<String, String> {
    Ok(ctx.call(Function::GetGuildInfo, vec![guild.into(), 0.into()])?.text())
}

fn own_guild(ctx: &Context) -> Result<i32, String> {
    number(ctx, Function::GetCharacterId, vec![2.into()])
}

fn spend_zeny(ctx: &Context, cost: i32) -> Result<bool, String> {
    let current = ctx.read("Zeny")?.number_value()?;
    if current < cost {
        return Ok(false);
    }
    ctx.write("Zeny", (current - cost).into())?;
    Ok(true)
}

pub fn steward(ctx: &Context) -> Result<(), String> {
    let args = arguments(ctx)?;
    let map = argument(&args, 0)?.text();
    let title = format!("[{}]", argument(&args, 1)?.text().split('#').next().unwrap_or_default());
    let master_room = (argument(&args, 2)?.number_value()?, argument(&args, 3)?.number_value()?);
    let slots = args[4..]
        .chunks_exact(3)
        .map(|slot| slot[0].number_value())
        .collect::<Result<Vec<_>, String>>()?;
    let guild = castle_data(ctx, &map, CASTLE_GUILD)?;
    ctx.mes(&title)?;
    if guild == 0 {
        ctx.mes("I have been waiting for a master to fulfill my destiny.\nBrave soul... fate will guide you towards your future...")?;
        return ctx.close();
    }
    if ctx.call(Function::GetGuildInfo, vec![guild.into(), 2.into()])?.number_value()? != 1 {
        let master = ctx.call(Function::GetGuildInfo, vec![guild.into(), 1.into()])?.text();
        ctx.mes(format!("No matter how much you pester me, I'll still follow my master ^ff0000{master}^000000. Where are the Guardians?! Send these ruffians away right now!"))?;
        return ctx.close();
    }
    ctx.mes("Welcome. My honorable master...\nYour humble servant is here to serve you.")?;
    ctx.next()?;
    let menu = [
        "Castle briefing",
        "Invest in commercial growth",
        "Invest in Castle Defenses",
        "Summon Guardian",
        "Hire / Fire a Kafra Employee",
        "Go into Master's room",
    ]
    .map(String::from);
    match ctx.select(&menu)? {
        0 => briefing(ctx, &map, &title)?,
        1 => invest(ctx, &map, &title, CASTLE_ECONOMY, CASTLE_INVESTED_ECONOMY, &ECONOMY_COSTS, "commercial growth")?,
        2 => invest(ctx, &map, &title, CASTLE_DEFENSE, CASTLE_INVESTED_DEFENSE, &DEFENSE_COSTS, "Castle Defenses")?,
        3 => summon_guardian(ctx, &map, &title, guild, &slots)?,
        4 => kafra_contract(ctx, &map, &title, guild)?,
        _ => {
            ctx.mes(&title)?;
            ctx.mes("Do you want to visit the room where our valuables are stored?\nThat room is restricted to you... you are the only one with access to it.")?;
            ctx.next()?;
            if ctx.select(&["Go into Master's room.".to_string(), "Cancel".into()])? == 0 {
                ctx.mes(&title)?;
                ctx.mes("I'll show you the secret path. Follow me...please.\nWhen you want to return here, please press the secret switch.")?;
                ctx.close()?;
                ctx.call(Function::Warp, vec![map.into(), master_room.0.into(), master_room.1.into()])?;
                return Ok(());
            }
            ctx.mes(&title)?;
            ctx.mes("Goods are produced once a day... if you don't remove them in time, they will not be produced anymore.\nTherefore, it will be better if you check up on them from time to time.")?;
        }
    }
    ctx.close()
}

fn briefing(ctx: &Context, map: &str, title: &str) -> Result<(), String> {
    ctx.mes(title)?;
    let mut report = format!(
        "I will report the Castle briefing, Master.\n\n^0000ffNow, the commercial growth level is {}.",
        castle_data(ctx, map, CASTLE_ECONOMY)?
    );
    let invested = castle_data(ctx, map, CASTLE_INVESTED_ECONOMY)?;
    if invested != 0 {
        report += &format!("\n You invested {invested} times in past 1 day.");
    }
    report += &format!("\n Now, the Castle Defense level is {}.^000000", castle_data(ctx, map, CASTLE_DEFENSE)?);
    let invested = castle_data(ctx, map, CASTLE_INVESTED_DEFENSE)?;
    if invested != 0 {
        report += &format!("\n ^0000ff- You invested {invested} times in past 1 day.^000000");
    }
    ctx.mes(report + "\n\nThat's all I have to report, Master.")
}

fn invest(ctx: &Context, map: &str, title: &str, level_field: i32, invested_field: i32, costs: &[i32; 20], subject: &str) -> Result<(), String> {
    let level = castle_data(ctx, map, level_field)?;
    let invested = castle_data(ctx, map, invested_field)?;
    let base = costs[((level - 1).max(0) / 5).min(19) as usize];
    let cost = if invested != 0 { base * 4 } else { base };
    ctx.mes(title)?;
    ctx.mes(format!("If you invest in {subject}, the castle will grow stronger. Initially, you are able to invest just once but if you pay more money, you will be able to invest twice."))?;
    if level >= 100 {
        return ctx.mes(format!("^ff0000The level of {subject} is at its highest, 100%. No more investments are needed.^000000"));
    }
    if invested >= 2 {
        return ctx.mes("^ff0000You have already invested twice today. You cannot invest any more.^000000");
    }
    ctx.mes(if invested == 0 {
        format!("The current investment amount required is ^ff0000{cost}^000000 zeny. Will you invest?")
    } else {
        format!("You've invested once today... if you wish to invest once more, ^ff0000{cost}^000000 more zeny will be needed.")
    })?;
    ctx.next()?;
    if ctx.select(&[format!("Invest in {subject}"), "Cancel".into()])? != 0 {
        ctx.mes(title)?;
        return ctx.mes("I'll do as you bid, my master... There is no hurry. We will do our best.");
    }
    ctx.mes(title)?;
    if !spend_zeny(ctx, cost)? {
        return ctx.mes("I'm sorry but there is not enough zeny to invest. You will have to try again when you have the funds, Master.");
    }
    set_castle_data(ctx, map, invested_field, invested + 1)?;
    ctx.mes("We finished the investment safely. I expect that our level will be increased by tomorrow.")
}

fn summon_guardian(ctx: &Context, map: &str, title: &str, guild: i32, slots: &[i32]) -> Result<(), String> {
    ctx.mes(title)?;
    ctx.mes("Will you summon a Guardian? It'll be a protector to defend us loyally.\nPlease select a guardian to defend us.")?;
    ctx.next()?;
    let mut labels = vec![];
    for (index, kind) in slots.iter().enumerate() {
        let name = match kind {
            1 => "Guardian Soldier",
            2 => "Guardian Archer",
            _ => "Guardian Knight",
        };
        let status = if castle_data(ctx, map, CASTLE_GUARDIAN + index as i32)? != 0 { "Implemented" } else { "Not Implemented" };
        labels.push(format!("{name} - {status}"));
    }
    let slot = ctx.select(&labels)?;
    ctx.mes(title)?;
    ctx.mes("Will you summon the chosen guardian? 10,000 zeny are required to summon a Guardian.")?;
    ctx.next()?;
    ctx.mes(title)?;
    if ctx.select(&["Summon".to_string(), "Cancel".into()])? != 0 {
        return ctx.mes("I did as you ordered. But please remember if you the have money to spare, it'll be better to set it up.");
    }
    if number(ctx, Function::GetGuildSkillLevel, vec![guild.into(), GD_GUARDRESEARCH.into()])? == 0 {
        return ctx.mes("Master, we have not the resources to Summon the Guardian. If you want to accumulate them, you have to learn the Guild skill. We failed to summon the Guardian.");
    }
    if castle_data(ctx, map, CASTLE_GUARDIAN + slot as i32)? == 1 {
        return ctx.mes("Master, you already have summoned that Guardian. We cannot summon another.");
    }
    if !spend_zeny(ctx, 10_000)? {
        return ctx.mes("Well... I'm sorry but we don't have funds to summon the Guardian. We failed to summon the Guardian.");
    }
    set_castle_data(ctx, map, CASTLE_GUARDIAN + slot as i32, 1)?;
    ctx.call(Function::GuardianSummon, vec![map.into(), (slot as i32).into()])?;
    ctx.mes("We completed the summoning of the Guardian. Our defenses are now increased with it in place.")
}

fn kafra_contract(ctx: &Context, map: &str, title: &str, guild: i32) -> Result<(), String> {
    ctx.mes(title)?;
    if castle_data(ctx, map, CASTLE_KAFRA)? == 1 {
        ctx.mes("We are currently hiring a Kafra Employee... Do you want to fire the Kafra Employee?")?;
        ctx.next()?;
        ctx.mes(title)?;
        if ctx.select(&["Fire".to_string(), "Cancel".into()])? != 0 {
            return ctx.mes("She worked hard in my opinion. It was a good decision to keep her.");
        }
        set_castle_data(ctx, map, CASTLE_KAFRA, 0)?;
        return ctx.mes("....\nI have discharged the Kafra Employee... But... are you unsatisfied with something?");
    }
    ctx.mes("Will you contact the kafra Main Office and Hire a Employee for our Castle?\n^ff0000 10,000 zeny is required for their services. ")?;
    ctx.next()?;
    ctx.mes(title)?;
    if ctx.select(&["Hire.".to_string(), "Cancel".into()])? != 0 {
        return ctx.mes("I did as you ordered, but some of our members will be unhappy. It will be better to hire a Kafra Employee quickly.");
    }
    if number(ctx, Function::GetGuildSkillLevel, vec![guild.into(), GD_KAFRACONTRACT.into()])? == 0 {
        return ctx.mes("Master, we can't hire a Kafra Employee because we don't have a contract with the Kafra Main Office. If you want to obtain a contract with the Kafra Main Office, you will need to learn the Guild skill first.");
    }
    if !spend_zeny(ctx, 10_000)? {
        return ctx.mes("Well... I'm sorry but we don't have enough funds to hire a Kafra Employee.");
    }
    set_castle_data(ctx, map, CASTLE_KAFRA, 1)?;
    ctx.mes("We obtained a contract with the kafra Main Office, and hired a Kafra Employee.\nThe Contract terms of the hired Kafra Employee are for 1 month and after this term, you will need to pay an additional fee.\nIt will be useful for our members.")
}

pub fn flag(ctx: &Context) -> Result<(), String> {
    let args = arguments(ctx)?;
    let map = argument(&args, 0)?.text();
    let return_point = (argument(&args, 1)?.number_value()?, argument(&args, 2)?.number_value()?);
    if argument(&args, 3)?.number_value()? != 0 {
        return Ok(());
    }
    let guild = castle_data(ctx, &map, CASTLE_GUILD)?;
    const EDICT: &str = "[ Edict of the Divine Rune-Midgarts Kingdom ]";
    if guild == 0 {
        ctx.mes(EDICT)?;
        ctx.mes(" 
1. Follow the ordinance of The Divine Rune-Midgarts Kingdom, 
We declare that
there is no formal master of this castle.
 
2. To the one who can 
overcome all trials
and destroy the Emperium,
the king will endow the one with
ownership of this castle.")?;
        return ctx.close();
    }
    if own_guild(ctx)? == guild {
        ctx.mes("[ Echoing Voice ]")?;
        ctx.mes("Brave ones...
Do you wish to return to your honorable place?")?;
        ctx.next()?;
        if ctx.select(&["Return to the guild castle.".to_string(), "Quit.".into()])? == 0 {
            ctx.close()?;
            if own_guild(ctx)? == castle_data(ctx, &map, CASTLE_GUILD)? {
                ctx.call(Function::Warp, vec![map.as_str().into(), return_point.0.into(), return_point.1.into()])?;
            }
            return Ok(());
        }
        return ctx.close();
    }
    let name = guild_name(ctx, guild)?;
    let master = ctx.call(Function::GetGuildInfo, vec![guild.into(), 1.into()])?.text();
    ctx.mes(EDICT)?;
    ctx.mes(format!(" 
1. Follow the ordinance of The Divine Rune-Midgarts Kingdom, 
we approve that this place is in
the private prossession of ^ff0000{name}^000000 Guild.
 
2. The guild Master of ^ff0000{name}^000000 Guild is
^ff0000{master}^000000
If there is anyone who objects to this,
prove your strength and honor with a steel blade in your hand."))?;
    ctx.close()
}

pub fn lever(ctx: &Context) -> Result<(), String> {
    let args = arguments(ctx)?;
    let castle = argument(&args, 0)?.text();
    let destination = (argument(&args, 1)?.text(), argument(&args, 2)?.number_value()?, argument(&args, 3)?.number_value()?);
    let restricted = argument(&args, 4)?.number_value()? != 0;
    let guild = castle_data(ctx, &castle, CASTLE_GUILD)?;
    if restricted {
        ctx.mes("[Ringing Voice]")?;
        if guild == 0 {
            ctx.mes("'Those who overcome an ordeal shows a great deal of bravery... and will find their way to another ordeal.'")?;
            return ctx.close();
        }
        ctx.mes("'Only the truly brave can take the test.'")?;
        ctx.next()?;
    }
    ctx.mes(" \nThere's a small lever. Will you pull it?")?;
    ctx.next()?;
    if ctx.select(&["Pull.".to_string(), "Do not.".into()])? != 0 {
        return ctx.close();
    }
    if restricted && own_guild(ctx)? != guild {
        ctx.mes(" \nNothing happened.")?;
        return ctx.close();
    }
    ctx.close()?;
    ctx.call(Function::Warp, vec![destination.0.into(), destination.1.into(), destination.2.into()]).map(|_| ())
}

pub fn kafra(ctx: &Context) -> Result<(), String> {
    let args = arguments(ctx)?;
    let map = argument(&args, 0)?.text();
    let region = argument(&args, 1)?.text();
    let destination = (argument(&args, 2)?.text(), argument(&args, 3)?.number_value()?, argument(&args, 4)?.number_value()?);
    let guild = castle_data(ctx, &map, CASTLE_GUILD)?;
    if guild == 0 || castle_data(ctx, &map, CASTLE_KAFRA)? == 0 {
        return ctx.close();
    }
    ctx.mes("[Kafra Employee]")?;
    if own_guild(ctx)? != guild {
        let name = guild_name(ctx, guild)?;
        ctx.mes(format!("I am instructed to only offer my services to the ^ff0000{name}^000000 Guild. Please try another Kafra Employee around here. Sorry for the inconvenience."))?;
        return ctx.close();
    }
    ctx.mes(format!("Welcome. ^ff0000{}^000000 Member.\nThe Kafra Corporation will stay with you wherever you go.", guild_name(ctx, guild)?))?;
    ctx.next()?;
    let menu = ["Use Storage", "Use Teleport Service", "Rent a Pushcart", "Cancel"].map(String::from);
    ctx.mes("[Kafra Employee]")?;
    match ctx.select(&menu)? {
        0 => {
            if number(ctx, Function::GetSkillLv, vec![1.into()])? < 6 {
                ctx.mes("I'm sorry, but you need the Novice's Basic Skill Level 6 to use the Storage Service.")?;
                return ctx.close();
            }
            ctx.mes("Here, let me open your Storage for you.\nThank you for using the Kafra Service.")?;
            ctx.close()?;
            ctx.call(Function::OpenStorage, vec![]).map(|_| ())
        }
        1 => {
            ctx.mes("Please choose your destination.")?;
            ctx.next()?;
            if ctx.select(&[format!("{region} -> 200z"), "Cancel".to_string()])? != 0 {
                return ctx.close();
            }
            if !spend_zeny(ctx, 200)? {
                ctx.mes("[Kafra Employee]")?;
                ctx.mes(format!("I'm sorry, but you don't have enough zeny for the Teleport Service. The fee to teleport to {region} is 200 zeny."))?;
                return ctx.close();
            }
            ctx.close()?;
            ctx.call(Function::Warp, vec![destination.0.into(), destination.1.into(), destination.2.into()]).map(|_| ())
        }
        2 => {
            if ctx.read("BaseClass")?.number_value()? != 5 {
                ctx.mes("I'm sorry, but the Pushcart rental service is only available to Merchants, Blacksmiths, Master Smiths, Alchemists, Biochemists, Mechanics and Geneticists.")?;
                return ctx.close();
            }
            if number(ctx, Function::CheckCart, vec![])? != 0 {
                ctx.mes("You already have a Pushcart equipped. Unfortunately, we can't rent more than one to each customer at a time.")?;
                return ctx.close();
            }
            ctx.mes("The Pushcart rental fee is 800 zeny. Would you like to rent a Pushcart?")?;
            ctx.next()?;
            if ctx.select(&["Rent a Pushcart.".to_string(), "Cancel".into()])? == 0 {
                if !spend_zeny(ctx, 800)? {
                    ctx.mes("[Kafra Employee]")?;
                    ctx.mes("I'm sorry, but you don't have enough zeny to pay the Pushcart rental fee of 800 zeny.")?;
                    return ctx.close();
                }
                ctx.call(Function::SetCart, vec![1.into()])?;
            }
            ctx.close()
        }
        _ => {
            ctx.mes("We, here at Kafra Corporation, are always endeavoring to provide you with the best services. We hope that we meet your adventuring needs and standards of excellence.")?;
            ctx.close()
        }
    }
}

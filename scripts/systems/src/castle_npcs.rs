use script_sdk_2::{Ctx, Script, Stop, Val, args};

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

fn argument(arguments: &[Val], index: usize) -> Result<&Val, Stop> {
    arguments.get(index).ok_or_else(|| Stop::from("NPC argument is missing"))
}

/// Takes `cost` zeny from the player. Returns false, and takes nothing, when they have less.
fn spend_zeny(ctx: &Ctx, cost: i32) -> Result<bool, Stop> {
    let zeny = ctx.player().zeny()?;
    if zeny < cost {
        return Ok(false);
    }
    ctx.player().set_zeny(zeny - cost)?;
    Ok(true)
}

/// The castle's steward. Its placement gives the castle map, the NPC name, the master's room position, then three values
/// per guardian slot, the first of which is the guardian kind.
pub fn steward(ctx: &Ctx) -> Script {
    let arguments = ctx.arguments()?;
    let map = argument(&arguments, 0)?.text();
    let name = argument(&arguments, 1)?.text();
    let title = name.split('#').next().unwrap_or_default();
    let master_room = (argument(&arguments, 2)?.number()?, argument(&arguments, 3)?.number()?);
    let slots = arguments[4..].chunks_exact(3).map(|slot| slot[0].number()).collect::<Result<Vec<_>, Stop>>()?;
    let guild = ctx.castle().data(&map, CASTLE_GUILD)?;
    ctx.mes(&format!("[{title}]"))?;
    if guild == 0 {
        ctx.mes("I have been waiting for a master to fulfill my destiny.\nBrave soul... fate will guide you towards your future...")?;
        return ctx.close();
    }
    if !ctx.guild().is_master(guild)? {
        let master = ctx.guild().master_name(guild)?;
        ctx.mes(&format!("No matter how much you pester me, I'll still follow my master ^ff0000{master}^000000. Where are the Guardians?! Send these ruffians away right now!"))?;
        return ctx.close();
    }
    ctx.mes("Welcome. My honorable master...\nYour humble servant is here to serve you.")?;
    ctx.next()?;
    let menu = ["Castle briefing", "Invest in commercial growth", "Invest in Castle Defenses", "Summon Guardian", "Hire / Fire a Kafra Employee", "Go into Master's room"];
    match ctx.menu(&menu)? {
        0 => briefing(ctx, &map, title)?,
        1 => invest(ctx, &map, title, CASTLE_ECONOMY, CASTLE_INVESTED_ECONOMY, &ECONOMY_COSTS, "commercial growth")?,
        2 => invest(ctx, &map, title, CASTLE_DEFENSE, CASTLE_INVESTED_DEFENSE, &DEFENSE_COSTS, "Castle Defenses")?,
        3 => summon_guardian(ctx, &map, title, guild, &slots)?,
        4 => kafra_contract(ctx, &map, title, guild)?,
        _ => {
            ctx.mes_as(title, "Do you want to visit the room where our valuables are stored?\nThat room is restricted to you... you are the only one with access to it.")?;
            ctx.next()?;
            if ctx.menu(&["Go into Master's room.", "Cancel"])? == 0 {
                ctx.mes_as(title, "I'll show you the secret path. Follow me...please.\nWhen you want to return here, please press the secret switch.")?;
                ctx.close_window()?;
                return ctx.warp(&map, master_room.0, master_room.1);
            }
            ctx.mes_as(title, "Goods are produced once a day... if you don't remove them in time, they will not be produced anymore.\nTherefore, it will be better if you check up on them from time to time.")?;
        }
    }
    ctx.close()
}

fn briefing(ctx: &Ctx, map: &str, title: &str) -> Script {
    let castle = ctx.castle();
    ctx.mes(&format!("[{title}]"))?;
    let mut report = format!("I will report the Castle briefing, Master.\n\n^0000ffNow, the commercial growth level is {}.", castle.data(map, CASTLE_ECONOMY)?);
    let invested = castle.data(map, CASTLE_INVESTED_ECONOMY)?;
    if invested != 0 {
        report += &format!("\n You invested {invested} times in past 1 day.");
    }
    report += &format!("\n Now, the Castle Defense level is {}.^000000", castle.data(map, CASTLE_DEFENSE)?);
    let invested = castle.data(map, CASTLE_INVESTED_DEFENSE)?;
    if invested != 0 {
        report += &format!("\n ^0000ff- You invested {invested} times in past 1 day.^000000");
    }
    ctx.mes(&(report + "\n\nThat's all I have to report, Master."))
}

fn invest(ctx: &Ctx, map: &str, title: &str, level_field: i32, invested_field: i32, costs: &[i32; 20], subject: &str) -> Script {
    let castle = ctx.castle();
    let level = castle.data(map, level_field)?;
    let invested = castle.data(map, invested_field)?;
    let base = costs[((level - 1).max(0) / 5).min(19) as usize];
    let cost = if invested != 0 { base * 4 } else { base };
    let offer = format!("If you invest in {subject}, the castle will grow stronger. Initially, you are able to invest just once but if you pay more money, you will be able to invest twice.");
    if level >= 100 {
        return ctx.mes_as(title, &format!("{offer}\n^ff0000The level of {subject} is at its highest, 100%. No more investments are needed.^000000"));
    }
    if invested >= 2 {
        return ctx.mes_as(title, &format!("{offer}\n^ff0000You have already invested twice today. You cannot invest any more.^000000"));
    }
    let price = if invested == 0 {
        format!("The current investment amount required is ^ff0000{cost}^000000 zeny. Will you invest?")
    } else {
        format!("You've invested once today... if you wish to invest once more, ^ff0000{cost}^000000 more zeny will be needed.")
    };
    ctx.mes_as(title, &format!("{offer}\n{price}"))?;
    ctx.next()?;
    if ctx.menu(&[format!("Invest in {subject}"), "Cancel".into()])? != 0 {
        return ctx.mes_as(title, "I'll do as you bid, my master... There is no hurry. We will do our best.");
    }
    ctx.mes(&format!("[{title}]"))?;
    if !spend_zeny(ctx, cost)? {
        return ctx.mes("I'm sorry but there is not enough zeny to invest. You will have to try again when you have the funds, Master.");
    }
    castle.set_data(map, invested_field, invested + 1)?;
    ctx.mes("We finished the investment safely. I expect that our level will be increased by tomorrow.")
}

fn summon_guardian(ctx: &Ctx, map: &str, title: &str, guild: i32, slots: &[i32]) -> Script {
    let castle = ctx.castle();
    ctx.mes_as(title, "Will you summon a Guardian? It'll be a protector to defend us loyally.\nPlease select a guardian to defend us.")?;
    ctx.next()?;
    let mut labels = vec![];
    for (slot, kind) in (0..).zip(slots) {
        let name = match kind {
            1 => "Guardian Soldier",
            2 => "Guardian Archer",
            _ => "Guardian Knight",
        };
        let status = if castle.data(map, CASTLE_GUARDIAN + slot)? != 0 { "Implemented" } else { "Not Implemented" };
        labels.push(format!("{name} - {status}"));
    }
    let slot = ctx.menu(&labels)? as i32;
    ctx.mes_as(title, "Will you summon the chosen guardian? 10,000 zeny are required to summon a Guardian.")?;
    ctx.next()?;
    ctx.mes(&format!("[{title}]"))?;
    if ctx.menu(&["Summon", "Cancel"])? != 0 {
        return ctx.mes("I did as you ordered. But please remember if you the have money to spare, it'll be better to set it up.");
    }
    if ctx.guild().skill_level(guild, GD_GUARDRESEARCH)? == 0 {
        return ctx.mes("Master, we have not the resources to Summon the Guardian. If you want to accumulate them, you have to learn the Guild skill. We failed to summon the Guardian.");
    }
    if castle.data(map, CASTLE_GUARDIAN + slot)? == 1 {
        return ctx.mes("Master, you already have summoned that Guardian. We cannot summon another.");
    }
    if !spend_zeny(ctx, 10_000)? {
        return ctx.mes("Well... I'm sorry but we don't have funds to summon the Guardian. We failed to summon the Guardian.");
    }
    castle.set_data(map, CASTLE_GUARDIAN + slot, 1)?;
    castle.summon_guardian(map, slot)?;
    ctx.mes("We completed the summoning of the Guardian. Our defenses are now increased with it in place.")
}

fn kafra_contract(ctx: &Ctx, map: &str, title: &str, guild: i32) -> Script {
    let castle = ctx.castle();
    ctx.mes(&format!("[{title}]"))?;
    if castle.data(map, CASTLE_KAFRA)? == 1 {
        ctx.mes("We are currently hiring a Kafra Employee... Do you want to fire the Kafra Employee?")?;
        ctx.next()?;
        ctx.mes(&format!("[{title}]"))?;
        if ctx.menu(&["Fire", "Cancel"])? != 0 {
            return ctx.mes("She worked hard in my opinion. It was a good decision to keep her.");
        }
        castle.set_data(map, CASTLE_KAFRA, 0)?;
        return ctx.mes("....\nI have discharged the Kafra Employee... But... are you unsatisfied with something?");
    }
    ctx.mes("Will you contact the kafra Main Office and Hire a Employee for our Castle?\n^ff0000 10,000 zeny is required for their services. ")?;
    ctx.next()?;
    ctx.mes(&format!("[{title}]"))?;
    if ctx.menu(&["Hire.", "Cancel"])? != 0 {
        return ctx.mes("I did as you ordered, but some of our members will be unhappy. It will be better to hire a Kafra Employee quickly.");
    }
    if ctx.guild().skill_level(guild, GD_KAFRACONTRACT)? == 0 {
        return ctx.mes("Master, we can't hire a Kafra Employee because we don't have a contract with the Kafra Main Office. If you want to obtain a contract with the Kafra Main Office, you will need to learn the Guild skill first.");
    }
    if !spend_zeny(ctx, 10_000)? {
        return ctx.mes("Well... I'm sorry but we don't have enough funds to hire a Kafra Employee.");
    }
    castle.set_data(map, CASTLE_KAFRA, 1)?;
    ctx.mes("We obtained a contract with the kafra Main Office, and hired a Kafra Employee.\nThe Contract terms of the hired Kafra Employee are for 1 month and after this term, you will need to pay an additional fee.\nIt will be useful for our members.")
}

const EDICT: &str = "[ Edict of the Divine Rune-Midgarts Kingdom ]";

/// A flag of a castle or of its town. Its placement gives the castle map, the position members return to, and whether
/// the flag is only decoration.
pub fn flag(ctx: &Ctx) -> Script {
    let arguments = ctx.arguments()?;
    let map = argument(&arguments, 0)?.text();
    let return_point = (argument(&arguments, 1)?.number()?, argument(&arguments, 2)?.number()?);
    if argument(&arguments, 3)?.number()? != 0 {
        return Ok(());
    }
    let guild = ctx.castle().data(&map, CASTLE_GUILD)?;
    if guild == 0 {
        ctx.lines(args![
            EDICT,
            " ",
            "1. Follow the ordinance of The Divine Rune-Midgarts Kingdom, ",
            "We declare that",
            "there is no formal master of this castle.",
            " ",
            "2. To the one who can ",
            "overcome all trials",
            "and destroy the Emperium,",
            "the king will endow the one with",
            "ownership of this castle.",
        ])?;
        return ctx.close();
    }
    if ctx.player().guild_id()? == guild {
        ctx.mes("[ Echoing Voice ]\nBrave ones...\nDo you wish to return to your honorable place?")?;
        ctx.next()?;
        if ctx.menu(&["Return to the guild castle.", "Quit."])? == 0 {
            ctx.close_window()?;
            if ctx.player().guild_id()? == ctx.castle().data(&map, CASTLE_GUILD)? {
                ctx.warp(&map, return_point.0, return_point.1)?;
            }
            return Ok(());
        }
        return ctx.close();
    }
    let name = ctx.guild().name(guild)?;
    let master = ctx.guild().master_name(guild)?;
    ctx.lines(args![
        EDICT,
        " ",
        "1. Follow the ordinance of The Divine Rune-Midgarts Kingdom, ",
        "we approve that this place is in",
        format!("the private prossession of ^ff0000{name}^000000 Guild."),
        " ",
        format!("2. The guild Master of ^ff0000{name}^000000 Guild is"),
        format!("^ff0000{master}^000000"),
        "If there is anyone who objects to this,",
        "prove your strength and honor with a steel blade in your hand.",
    ])?;
    ctx.close()
}

/// A lever in a castle or guild dungeon. Its placement gives the castle map, the destination, and whether only the
/// owning guild may use it.
pub fn lever(ctx: &Ctx) -> Script {
    let arguments = ctx.arguments()?;
    let castle = argument(&arguments, 0)?.text();
    let destination = (argument(&arguments, 1)?.text(), argument(&arguments, 2)?.number()?, argument(&arguments, 3)?.number()?);
    let restricted = argument(&arguments, 4)?.number()? != 0;
    let guild = ctx.castle().data(&castle, CASTLE_GUILD)?;
    if restricted {
        if guild == 0 {
            ctx.mes_as("Ringing Voice", "'Those who overcome an ordeal shows a great deal of bravery... and will find their way to another ordeal.'")?;
            return ctx.close();
        }
        ctx.mes_as("Ringing Voice", "'Only the truly brave can take the test.'")?;
        ctx.next()?;
    }
    ctx.mes(" \nThere's a small lever. Will you pull it?")?;
    ctx.next()?;
    if ctx.menu(&["Pull.", "Do not."])? != 0 {
        return ctx.close();
    }
    if restricted && ctx.player().guild_id()? != guild {
        ctx.mes(" \nNothing happened.")?;
        return ctx.close();
    }
    ctx.close_window()?;
    ctx.warp(&destination.0, destination.1, destination.2)
}

const KAFRA: &str = "Kafra Employee";

/// The Kafra of a castle the guild hired. Its placement gives the castle map, the region name of the teleport, and its
/// destination.
pub fn kafra(ctx: &Ctx) -> Script {
    let arguments = ctx.arguments()?;
    let map = argument(&arguments, 0)?.text();
    let region = argument(&arguments, 1)?.text();
    let destination = (argument(&arguments, 2)?.text(), argument(&arguments, 3)?.number()?, argument(&arguments, 4)?.number()?);
    let guild = ctx.castle().data(&map, CASTLE_GUILD)?;
    if guild == 0 || ctx.castle().data(&map, CASTLE_KAFRA)? == 0 {
        return ctx.close();
    }
    ctx.mes(&format!("[{KAFRA}]"))?;
    if ctx.player().guild_id()? != guild {
        let name = ctx.guild().name(guild)?;
        ctx.mes(&format!("I am instructed to only offer my services to the ^ff0000{name}^000000 Guild. Please try another Kafra Employee around here. Sorry for the inconvenience."))?;
        return ctx.close();
    }
    ctx.mes(&format!("Welcome. ^ff0000{}^000000 Member.\nThe Kafra Corporation will stay with you wherever you go.", ctx.guild().name(guild)?))?;
    ctx.next()?;
    ctx.mes(&format!("[{KAFRA}]"))?;
    match ctx.menu(&["Use Storage", "Use Teleport Service", "Rent a Pushcart", "Cancel"])? {
        0 => {
            if ctx.player().skill_level(1)? < 6 {
                ctx.mes("I'm sorry, but you need the Novice's Basic Skill Level 6 to use the Storage Service.")?;
                return ctx.close();
            }
            ctx.mes("Here, let me open your Storage for you.\nThank you for using the Kafra Service.")?;
            ctx.close_window()?;
            ctx.player().open_storage()
        }
        1 => {
            ctx.mes("Please choose your destination.")?;
            ctx.next()?;
            if ctx.menu(&[format!("{region} -> 200z"), "Cancel".into()])? != 0 {
                return ctx.close();
            }
            if !spend_zeny(ctx, 200)? {
                ctx.mes_as(KAFRA, &format!("I'm sorry, but you don't have enough zeny for the Teleport Service. The fee to teleport to {region} is 200 zeny."))?;
                return ctx.close();
            }
            ctx.close_window()?;
            ctx.warp(&destination.0, destination.1, destination.2)
        }
        2 => {
            if ctx.var("BaseClass").get()?.number()? != 5 {
                ctx.mes("I'm sorry, but the Pushcart rental service is only available to Merchants, Blacksmiths, Master Smiths, Alchemists, Biochemists, Mechanics and Geneticists.")?;
                return ctx.close();
            }
            if ctx.player().has_cart(None)? {
                ctx.mes("You already have a Pushcart equipped. Unfortunately, we can't rent more than one to each customer at a time.")?;
                return ctx.close();
            }
            ctx.mes("The Pushcart rental fee is 800 zeny. Would you like to rent a Pushcart?")?;
            ctx.next()?;
            if ctx.menu(&["Rent a Pushcart.", "Cancel"])? == 0 {
                if !spend_zeny(ctx, 800)? {
                    ctx.mes_as(KAFRA, "I'm sorry, but you don't have enough zeny to pay the Pushcart rental fee of 800 zeny.")?;
                    return ctx.close();
                }
                ctx.player().set_cart(true)?;
            }
            ctx.close()
        }
        _ => {
            ctx.mes("We, here at Kafra Corporation, are always endeavoring to provide you with the best services. We hope that we meet your adventuring needs and standards of excellence.")?;
            ctx.close()
        }
    }
}

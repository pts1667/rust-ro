use script_sdk_2::{Ctx, Script, Stop};

use crate::battleground_arena::{WAITING_ROOM, read};

const BRAVERY_BADGE: i32 = 7828;
const VALOR_BADGE: i32 = 7829;
const WEAPON_BADGE_COST: i32 = 100;
const RETURN_VARIABLE: &str = "bat_return";

const ROLE_RECRUITER: i32 = 0;
const ROLE_TELEPORTER: i32 = 1;
const ROLE_PRINCE: i32 = 2;
const ROLE_GENERAL: i32 = 3;
const ROLE_EXCHANGER: i32 = 4;
const ROLE_DECORATION: i32 = 5;

const RETURN_POINTS: [(&str, &str, i32, i32); 8] = [
    ("Prontera.", "prontera", 116, 72),
    ("Morocc Ruins.", "moc_ruins", 152, 48),
    ("Al De Baran.", "aldebaran", 168, 112),
    ("Geffen.", "geffen", 120, 39),
    ("Payon.", "payon", 161, 58),
    ("Lighthalzen.", "lighthalzen", 159, 93),
    ("Rachel.", "rachel", 115, 124),
    ("Morocc.", "morocc", 156, 46),
];

struct Noble {
    title: &'static str,
    greeting: &'static str,
    cutins: (&'static str, &'static str),
    topics: [(&'static str, &'static [&'static [&'static str]]); 2],
    accept: &'static [&'static [&'static str]],
    joins: [&'static str; 2],
    refusal: &'static str,
}

const PRINCE_CROIX: Noble = Noble {
    title: "[Prince Croix]",
    greeting: "Wise adventurer, why don't you lend us your power for victory?",
    cutins: ("bat_crua1", "bat_crua2"),
    topics: [
        (
            "What's the reason for the Battle?",
            &[
                &[
                    "Maroll's great king, Marcel Marollo VII, is very sick lately.",
                    "His Majesty has declared that he will be leaving the future of Maroll to me or the 3rd prince, General Guillaume.",
                ],
                &[
                    "General Guillaume may have an advantage in this battle as he is the great general of Maroll, but that doesn't automatically mean he'll win.",
                    "I want to win this battle so that I can bring prosperity to the people of Maroll. They've suffered enough from war...",
                ],
            ],
        ),
        (
            "Tell me about General Guillaume",
            &[
                &[
                    "The 3rd Prince Guillaume is the great general of Maroll.",
                    "It's a waste of time to explain to you how great a leader or warlord he is, since he commands the great military power of Maroll.",
                ],
                &[
                    "Unfortunately, there's something he and his followers are unaware of:",
                    "Do the people of Maroll really want them to spend so much money on military power?",
                    "We have suffered enough from wars.",
                    "I believe weapons aren't the best way to bring prosperity to a nation.",
                ],
                &["I do not wish to shed blood, but I have no choice but to fight for the possibility of peace and for the sake of my people."],
            ],
        ),
    ],
    accept: &[&[
        "Thank you so much. I feel like I can win with the help of adventurers like you.",
        "Now, please go downstairs and join your comrades in sharpening their skills to fight the enemy!",
    ]],
    joins: ["Yes, I want to join you.", "Yes, I want to join you."],
    refusal: "For Maroll!",
};

const GENERAL_GUILLAUME: Noble = Noble {
    title: "[General Guillaume]",
    greeting: "Hot-blooded adventurer, we need your ability to win this battle.",
    cutins: ("bat_kiyom2", "bat_kiyom1"),
    topics: [
        (
            "What's the reason for the Battle?",
            &[
                &[
                    "Our great king, Marcel Marollo VII, is very sick lately.",
                    "His Majesty has declared that he has chosen either me or Prince Croix as the next king amongst his 9 sons.",
                ],
                &["Two kings can't share a nation! Only the one victorious from His Majesty's appointed battle will be enthroned."],
                &[
                    "This is, however, not just a battle between us. This battle will determine the future of this country.",
                    "I pledge on my honor to prove that I'm the one who can protect this Maroll from outside threats.",
                ],
            ],
        ),
        (
            "Tell me about Prince Croix",
            &[
                &[
                    "The 5th Prince Croix is currently titled as the Prime Minister of Maroll.",
                    "He thinks all national matters of a nation can be discussed and determined on a desk,",
                    "and believes in peaceful co-existence with other countries.",
                ],
                &["He's too ignorant to admit that so-called peace is built on countless lives that are sacrificed in wars while normal citizens and upper classes can live, oblivious to the horrors that allow them to live that way."],
                &[
                    "He's too naive to understand the reality....",
                    "I can't leave Maroll to someone like him who lives in a dream!",
                ],
                &[
                    "His unrealistic beliefs will drown this country in poverty and make the people weak. If he becomes the king, Maroll will never rest from the onslaughts of other countries.",
                    "I want to teach him what makes this small country so powerful and prosperous. It's military power!",
                ],
            ],
        ),
    ],
    accept: &[
        &["Welcome to my army, comrade.", "Your eyes tell me that you're a soldier that I can trust."],
        &["Now, go upstairs and apply for battle with your comrades.", "I'm sure they'll welcome you whole-heartedly!"],
    ],
    joins: ["Yes, I want to join you.", "I want to join your army!"],
    refusal: "I'll be the one who will capture the flag!",
};

const WEAPON_CATEGORIES: [(&str, &str, &[(i32, i32)]); 5] = [
    (
        "Dagger/OneSword/TwoSword/TwoSpear",
        "Dagger, One-Handed Sword, Two-Handed Sword, and Two-Handed Spear",
        &[(13036, BRAVERY_BADGE), (13037, VALOR_BADGE), (13411, BRAVERY_BADGE), (13410, VALOR_BADGE), (1183, BRAVERY_BADGE), (1184, VALOR_BADGE), (1425, BRAVERY_BADGE), (1482, VALOR_BADGE)],
    ),
    (
        "Staff/Mace/TwoAxe/Shuriken",
        "Staff / Mace / Two-Handed Axe / Huuma Shuriken",
        &[
            (1632, BRAVERY_BADGE),
            (1633, VALOR_BADGE),
            (1634, BRAVERY_BADGE),
            (1635, VALOR_BADGE),
            (1543, BRAVERY_BADGE),
            (1542, VALOR_BADGE),
            (1380, BRAVERY_BADGE),
            (1379, VALOR_BADGE),
            (13305, BRAVERY_BADGE),
            (13306, VALOR_BADGE),
        ],
    ),
    (
        "Bow/Katar/Music/Whip",
        "Bow / Katar / Musical Instrument / Whip",
        &[(1739, BRAVERY_BADGE), (1738, VALOR_BADGE), (1279, BRAVERY_BADGE), (1280, VALOR_BADGE), (1924, BRAVERY_BADGE), (1923, VALOR_BADGE), (1978, BRAVERY_BADGE), (1977, VALOR_BADGE)],
    ),
    ("Book/Knuckle", "Book / Knuckle", &[(1574, BRAVERY_BADGE), (1575, VALOR_BADGE), (1824, BRAVERY_BADGE), (1823, VALOR_BADGE)]),
    (
        "Revolver/Rifle/Gatling/Shotgun/Launcher",
        "Revolver / Rifle / Gatling Gun / Shotgun / Grenade Launcher",
        &[(13108, BRAVERY_BADGE), (13171, VALOR_BADGE), (13172, BRAVERY_BADGE), (13173, VALOR_BADGE), (13174, VALOR_BADGE)],
    ),
];

const GARMENTS: [(i32, i32); 6] = [(2538, 50), (2539, 50), (2540, 50), (2435, 50), (2436, 50), (2437, 50)];
const ARMORS: [(i32, i32); 7] = [(2376, 80), (2377, 80), (2378, 80), (2379, 80), (2380, 80), (2381, 80), (2382, 80)];
const ACCESSORY_JOBS: [(&str, i32, &str); 7] = [
    ("Gunslinger", 2733, "This item is for Gunslinger only."),
    ("Swordman/Taekwon Master", 2720, "This item is for Swordman and Taekwon Master Class only."),
    ("Thief", 2721, "This item is for Thief Class only."),
    ("Acolyte", 2722, "This item is for Acolyte Class only."),
    ("Magician", 2723, "This item is for Magician Class only."),
    ("Archer", 2724, "This item is for Archer Class only."),
    ("Merchant", 2725, "This item is for Merchant Class only."),
];
const ACCESSORY_COST: i32 = 500;
const CONSUMABLES: [(i32, i32); 5] = [(12269, 10), (12270, 10), (12271, 5), (12272, 10), (12273, 10)];

const EXCHANGER: &str = "Erundek";

fn recruiter(ctx: &Ctx, return_code: i32) -> Script {
    const TITLE: &str = "Maroll Battle Recruiter";
    ctx.mes_as(TITLE, "Good day, adventurer.\nI'm a knight from a far country called Maroll Kingdom.")?;
    ctx.next()?;
    ctx.mes_as(TITLE, "The two princes of the kingdom are now battling for the throne of Maroll, and are in need of experienced soldiers like you.\nHow would you like to lend your power to one of the princes in the Maroll Kingdom?")?;
    ctx.next()?;
    if ctx.menu(&["Join", "Don't Join"])? == 1 {
        ctx.mes_as(TITLE, "I'll always be stationed here for more soldiers. Feel free to come back whenever you're interested.")?;
        return ctx.close();
    }
    ctx.mes_as(TITLE, "May the war god bless you.")?;
    ctx.close_window()?;
    ctx.var(RETURN_VARIABLE).set(return_code)?;
    ctx.warp(WAITING_ROOM.map, WAITING_ROOM.x, WAITING_ROOM.y)
}

fn teleporter(ctx: &Ctx) -> Script {
    const TITLE: &str = "Teleporter";
    ctx.mes_as(TITLE, "Do you wish to leave the battlefield? Use my services to return to town.")?;
    ctx.next()?;
    if ctx.menu(&["Leave", "Don't Leave"])? == 1 {
        ctx.mes_as(TITLE, "I'll be here whenever you're in need of my services.")?;
        return ctx.close();
    }
    let code = read(ctx, RETURN_VARIABLE)?;
    let (name, map, x, y) = RETURN_POINTS[usize::try_from(code - 1).ok().filter(|index| *index < RETURN_POINTS.len()).unwrap_or(0)];
    ctx.mes_as(TITLE, &format!("You will be sent back to {name}"))?;
    ctx.close_window()?;
    ctx.warp(map, x, y)
}

fn show_pages(ctx: &Ctx, noble: &Noble, pages: &[&[&str]]) -> Script {
    for (index, lines) in pages.iter().enumerate() {
        if index > 0 {
            ctx.next()?;
        }
        ctx.mes(noble.title)?;
        ctx.mes(&lines.join("\n"))?;
    }
    Ok(())
}

fn noble(ctx: &Ctx, noble: &Noble) -> Script {
    let fx = ctx.fx();
    fx.cutin(noble.cutins.0, 2)?;
    ctx.mes(noble.title)?;
    ctx.mes(noble.greeting)?;
    ctx.next()?;
    let topic = ctx.menu(&[noble.topics[0].0, noble.topics[1].0])?;
    fx.cutin(noble.cutins.1, 2)?;
    show_pages(ctx, noble, noble.topics[topic].1)?;
    ctx.next()?;
    if ctx.menu(&[noble.joins[topic], "End Conversation"])? == 0 {
        fx.cutin(noble.cutins.0, 2)?;
        show_pages(ctx, noble, noble.accept)?;
    } else {
        ctx.mes(noble.title)?;
        ctx.mes(noble.refusal)?;
    }
    ctx.close_window()?;
    fx.cutin(noble.cutins.0, 255)?;
    fx.cutin(noble.cutins.1, 255)
}

fn badge_label(badge: i32) -> &'static str {
    if badge == BRAVERY_BADGE { "(BB)" } else { "(VB)" }
}

/// Takes `cost` of `badge` for one `item`, or says the player has too few badges.
fn trade(ctx: &Ctx, item: i32, badge: i32, cost: i32, too_few: &str) -> Script {
    let items = ctx.items();
    if items.count(badge)? < cost {
        return ctx.mes(too_few);
    }
    ctx.mes("Thank you for exchanging.")?;
    items.take(badge, cost)?;
    items.give(item, 1)
}

fn exchange_weapon(ctx: &Ctx) -> Script {
    let items = ctx.items();
    ctx.mes_as(EXCHANGER, "You chose ^3131FFWeapon^000000.\nThe following weapons are available for exchange with the battlefield badges.\nPlease note that items for ^3131FFBravery Badges are indicated as (BB)^000000, and ^3131FFValor Badges as (VB)^000000.")?;
    ctx.next()?;
    let categories: Vec<&str> = WEAPON_CATEGORIES.iter().map(|category| category.0).collect();
    let (_, description, weapons) = WEAPON_CATEGORIES[ctx.menu(&categories)?];
    ctx.mes_as(EXCHANGER, &format!("The following items are available in the ^3131FF{description}^000000 category."))?;
    ctx.next()?;
    let mut options = Vec::with_capacity(weapons.len());
    for (item, badge) in weapons {
        options.push(format!("{}{}", items.name(*item)?, badge_label(*badge)));
    }
    let (item, badge) = weapons[ctx.menu(&options)?];
    let name = items.name(item)?;
    let badge_name = items.name(badge)?;
    let label = badge_label(badge);
    ctx.mes_as(EXCHANGER, &format!("You chose ^3131FF{name}{label}^000000.\nYou can exchange for this item with ^FF0000{WEAPON_BADGE_COST} {badge_name}^000000.\nWould you like to exchange?"))?;
    ctx.next()?;
    if ctx.menu(&["Do not exchange", "Exchange"])? == 1 {
        ctx.mes_as(EXCHANGER, &format!("Would you like to spend ^FF0000{WEAPON_BADGE_COST} {badge_name}^000000 and receive a ^3131FF{name}{label}^000000?"))?;
        ctx.next()?;
        ctx.mes_as(EXCHANGER, "Remember, Battleground Reward Items are ^FF0000Character Bound^000000. Are you sure you want this item?")?;
        ctx.next()?;
        if ctx.menu(&["Yes", "No"])? == 0 {
            ctx.mes(&format!("[{EXCHANGER}]"))?;
            trade(ctx, item, badge, WEAPON_BADGE_COST, "I'm sorry, but you don't have enough badges to exchange.")?;
            return ctx.close();
        }
    }
    ctx.mes_as(EXCHANGER, "Do you need more time to check the items?")?;
    ctx.close()
}

fn exchange_priced_item(ctx: &Ctx, item: i32, cost: i32, note: Option<&str>) -> Script {
    let items = ctx.items();
    let name = items.name(item)?;
    let bravery = items.name(BRAVERY_BADGE)?;
    let valor = items.name(VALOR_BADGE)?;
    ctx.mes_as(EXCHANGER, &format!("You chose ^3131FF{name}^000000."))?;
    if let Some(note) = note {
        ctx.mes(note)?;
    }
    ctx.mes(&format!("You can exchange for this item with ^FF0000{cost} {bravery} or {cost} {valor}^000000.\nWould you like to exchange?"))?;
    ctx.next()?;
    if ctx.menu(&["Do not exchange", "Exchange"])? == 0 {
        ctx.mes_as(EXCHANGER, "Do you need more time to check the items?")?;
        return ctx.close();
    }
    ctx.mes_as(EXCHANGER, &format!("Which Badge do you want to exchange?\nYou need ^3131FF{cost} Badges^000000 to exchange."))?;
    ctx.next()?;
    ctx.mes_as(EXCHANGER, "Remember, Battleground Reward Items are ^FF0000Character Bound^000000. Are you sure you want this item?")?;
    ctx.next()?;
    let choice = ctx.menu(&["Bravery Badge", "Valor Badge", "Cancel"])?;
    ctx.mes(&format!("[{EXCHANGER}]"))?;
    if choice == 2 {
        ctx.mes("You cancelled the exchange.")?;
        return ctx.close();
    }
    let badge = if choice == 0 { BRAVERY_BADGE } else { VALOR_BADGE };
    let too_few = format!("You do not have enough {}s.", items.name(badge)?);
    trade(ctx, item, badge, cost, &too_few)?;
    ctx.close()
}

/// Offers the items of a list, each for its price in either badge.
fn exchange_listed(ctx: &Ctx, items: &[(i32, i32)]) -> Script {
    let mut options = Vec::with_capacity(items.len());
    for (item, _) in items {
        options.push(ctx.items().name(*item)?);
    }
    let (item, cost) = items[ctx.menu(&options)?];
    exchange_priced_item(ctx, item, cost, None)
}

fn exchange_armor(ctx: &Ctx) -> Script {
    ctx.mes_as(EXCHANGER, "You chose ^3131FFArmor^000000.\nThe following armors are available for exchange with the battlefield badges.")?;
    ctx.next()?;
    let items: &[(i32, i32)] = if ctx.menu(&["Garments / Shoes", "Armor"])? == 0 { &GARMENTS } else { &ARMORS };
    exchange_listed(ctx, items)
}

fn exchange_accessory(ctx: &Ctx) -> Script {
    ctx.mes_as(EXCHANGER, "You chose ^3131FFAccessory^000000.\nYou can exchange the Medal of Honors with your Badges according to the job classes, as follows:")?;
    ctx.next()?;
    let jobs: Vec<&str> = ACCESSORY_JOBS.iter().map(|job| job.0).collect();
    let (_, item, note) = ACCESSORY_JOBS[ctx.menu(&jobs)?];
    exchange_priced_item(ctx, item, ACCESSORY_COST, Some(note))
}

fn exchange_consumable(ctx: &Ctx) -> Script {
    ctx.mes_as(EXCHANGER, "You chose ^3131FFConsumable^000000.\nThe following consumable items are available for exchange with the battlefield badges:")?;
    ctx.next()?;
    exchange_listed(ctx, &CONSUMABLES)
}

fn exchanger(ctx: &Ctx) -> Script {
    ctx.mes_as(EXCHANGER, "Do you have the battlefield badges?\nI can exchange Bravery Badges and Valor Badges for reward items.")?;
    ctx.next()?;
    if ctx.menu(&["Exchange Badges", "Check the Catalog"])? == 1 {
        ctx.mes_as(EXCHANGER, "We have many items, so please take a look and purchase deliberately. Every exchange menu lists the items together with their prices.")?;
        return ctx.close();
    }
    ctx.mes_as(EXCHANGER, "Which type of items would you like to exchange?\nTo check more information about the reward items, please use our ^3131FFCatalog^000000.")?;
    ctx.next()?;
    match ctx.menu(&["Weapon", "Armor", "Accessory", "Consumable"])? {
        0 => exchange_weapon(ctx),
        1 => exchange_armor(ctx),
        2 => exchange_accessory(ctx),
        _ => exchange_consumable(ctx),
    }
}

/// A battleground NPC outside the arenas. Its placement gives the role, then a recruiter's town code.
pub fn npc(ctx: &Ctx) -> Script {
    let arguments = ctx.arguments()?;
    let role = arguments.first().ok_or("Battleground NPC needs a role")?.number()?;
    match role {
        ROLE_RECRUITER => recruiter(ctx, arguments.get(1).ok_or("Recruiter needs a return code")?.number()?),
        ROLE_TELEPORTER => teleporter(ctx),
        ROLE_PRINCE => noble(ctx, &PRINCE_CROIX),
        ROLE_GENERAL => noble(ctx, &GENERAL_GUILLAUME),
        ROLE_EXCHANGER => exchanger(ctx),
        ROLE_DECORATION => Ok(()),
        _ => Err(Stop::Error(format!("Unknown battleground NPC role {role}"))),
    }
}

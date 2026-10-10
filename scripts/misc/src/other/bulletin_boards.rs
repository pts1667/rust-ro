#![allow(
    unused_imports,
    unused_labels,
    unused_assignments,
    unused_mut,
    unused_parens,
    unused_variables,
    unreachable_code
)]

use script_sdk_2::{Ctx, Script, Stop, Val, args};

/// Shows each page in its own dialogue box, in order, then closes the window.
fn board(ctx: &Ctx, speaker: &str, pages: &[&[&str]]) -> Script {
    for (i, page) in pages.iter().enumerate() {
        if i > 0 {
            ctx.next()?;
        }
        ctx.lines_as(speaker, page.iter().map(|line| Val::from(*line)).collect())?;
    }
    ctx.close()
}

pub fn bulletin_board_1(ctx: &Ctx) -> Script {
    board(
        ctx,
        "Alberta: The Port City",
        &[
            &["Welcome to Alberta, the Port City."],
            &["In Alberta, you can find the Merchant guild where adventurers can change their job to merchant"],
            &[
                "As a city that provides dependable sea travel, Alberta has provided the means for the foreign commerce that has brought prosperity to the Rune-Midgarts Kingdom.",
            ],
            &["On the docks, you may find representatives from foreign lands that will guide tourists to their countries."],
            &[
                "Among seafarers, there is a rumor of a place known as ^338C60Turtle Island^000000. Intrepid adventurers may seek to investigate this rumor and learn the truth for themselves",
            ],
            &[
                "There is also a sunken ship that has been discovered near Alberta, and it has become a popular area for adventurers to explore",
            ],
            &[
                "From Alberta, ^1F3A11Payon^000000 is located to the Northwest. You can board passenger ships at the dock to travel to ^5E5C69Izlude^000000.",
                "Enjoy your travels.",
            ],
        ],
    )
}

pub fn bulletin_board_2(ctx: &Ctx) -> Script {
    board(
        ctx,
        "Geffen: The City of Magic",
        &[
            &["Welcome to Geffen, the City of Magic. Geffen is well known for its various legends related to magic."],
            &[
                "Points of interest in the city include the Forge, where people can change their jobs to Blacksmith, the Magic Academy for aspiring mages, and the Geffen Tower for the Wizard job change.",
            ],
            &["Underneath the Geffen Tower lies a dungeon in which dreadful monsters are rumored to appear."],
            &["It is said that the lost city of Gefenia, a place of elven lore and legend, is hidden within the depths of this dungeon."],
            &[
                "From Geffen, the ^828E28Orc Village^000000 is located to the South, ^4C6055Glast Heim^000000 to the West, ^6D6FE0Prontera^000000 to the far East, and ^744B2DMt. Mjolnir^000000 to the North. Enjoy your stay here in Geffen.",
            ],
        ],
    )
}

pub fn bulletin_board_4(ctx: &Ctx) -> Script {
    board(
        ctx,
        "Byalan Island",
        &[
            &[
                "^6B1312Caution!^000000",
                "The Izlude dungeon is comprised of 5 levels. The first few levels are suited for newer adventurers.",
            ],
            &["As you venture deeper into the Byalan dungeon, you will encounter stronger, more powerful monsters."],
            &[
                "With a few exceptions, most of the monsters in this dungeon are of the Water attribute. Therefore, a Wind attribute weapon will help you greatly",
            ],
        ],
    )
}

pub fn bulletin_board_06(ctx: &Ctx) -> Script {
    board(
        ctx,
        "Payon: The Mountain City",
        &[
            &[
                "Welcome to Payon, the mountain city. Payon has recently been renovated, so we hope you enjoy the clean, nice streets and buildings.",
            ],
            &[
                "Payon is famous for being a city of archery. For a long time, its citizens have made their living by hunting with bows and arrows. Bow crafting and training have also been developed here in Payon",
            ],
            &[
                "Payon has various armor and weapons, especially for the Archer class. If you wish to become an Archer, it would be best to become familiar with this city.",
            ],
            &[
                "Payon Palace is located in the center of the city. The Archer Village is located in the Northern part of Payon. There, people can change their jobs to Archer.",
            ],
            &["Near the Archer Village, you can find the ^2F0400Payon Cave^000000 where Undead monsters reside."],
            &[
                "From Payon, ^123972Alberta^000000 is located to the Southeast, and ^866C4BMorocc^000000 is to the West. ^5E5C69Izlude^000000 and ^6D6FE0Prontera^000000 are North of Payon.",
                "Enjoy your travels.",
            ],
        ],
    )
}

pub fn bulletin_board_07(ctx: &Ctx) -> Script {
    board(
        ctx,
        "Archer Village",
        &[
            &["Welcome to the Payon Archer Village where Novices can change their jobs to Archer."],
            &[
                "The Archer Village provides Bows and Tights that are available for purchase. These are necessities for new Archers and Hunters.",
            ],
            &[
                "If you wish to become an Archer, it is suggested to become familiar with this village. For aspiring Hunters, the Hunter Guild is located in a field that is East of Payon.",
            ],
        ],
    )
}

pub fn bulletin_board_09(ctx: &Ctx) -> Script {
    board(
        ctx,
        "Prontera: Capital of the",
        &[
            &["[Rune-Midgarts Kingdom]", "Welcome to Prontera, the capital city of Rune-Midgard."],
            &[
                "[Rune-Midgarts Kingdom]",
                "Prontera is located in the center of the Rune-Midgard continent and is very well-known as a city of flourishing commerce.",
            ],
            &[
                "[Rune-Midgarts Kingdom]",
                "In this city, you can fint the Sanctuary, where people can change their jobs to Acolyte and Priest.",
            ],
            &[
                "[Rune-Midgarts Kingdom]",
                "You can also find the Castle, where people can change their jobs to Crusader.",
            ],
            &[
                "[Rune-Midgarts Kingdom]",
                "Please feel free to explore the streets of Prontera, as there are various tourist attractions within the city.",
            ],
            &[
                "[Rune-Midgarts Kingdom]",
                "When you need to upgrade your weapons and armors, please visit the building in the 5 o'clock direction from the fountain in the center of Prontera.",
            ],
            &[
                "[Rune-Midgarts Kingdom]",
                "From Prontera, ^5E5C69Izlude^000000 is located to the Southeast, ^1F3A11Payon^000000 to the far South, ^683C1FGeffen^000000 to the far West and ^2D3832Al De Baran^000000 to the far North.",
                "Enjoy your time in Prontera.",
            ],
        ],
    )
}

pub fn bulletin_board_11(ctx: &Ctx) -> Script {
    board(
        ctx,
        "Morocc: The Frontier Town",
        &[
            &[
                "Welcome to Morocc, the City of the Desert. Morocc was built on an oasis, so this town can accomodate its many visitors and travelers.",
            ],
            &[
                "Morocc Castle lies in the center of this city. Please feel free to explore this town, and enjoy its unique atmosphere. However, watch your pockets and beware of Rogues and Thieves.",
            ],
            &[
                "From Morocc, the ^660000Pyramid Dungeon^000000 can be found to the Northwest, and the ^660000Sphinx Dungeon^000000 can be found to the West. To the Southeast, you may find ^660000Ant Hell^000000.",
            ],
            &[
                "The Assassin Guild is rumored to be located to the Southeast. When you head East from Morocc, and then North, you will arrive at ^3355FFProntera^000000.",
            ],
        ],
    )
}

pub fn bulletin_board_12(ctx: &Ctx) -> Script {
    board(
        ctx,
        "Comodo: The Beach City",
        &[
            &[
                "Welcome! This town of Comodo is",
                "surrounded by many ancient relics",
                "from a forgotten era.",
            ],
            &[
                "Only in Comodo can you find the",
                "Dancer Guild and Bard Guild which",
                "provide the opportunity for",
                "adventurers to change their jobs to",
                "Dancers and Bards.",
            ],
            &[
                "You can also visit the Casino,",
                "which is a popular as a tourist",
                "attraction and place to lounge",
            ],
            &[
                "You can buy Berserk Potions, a",
                "special product of Comodo, from the",
                "Tool Dealers in the area.",
            ],
            &[
                "There are 3 caves around Comodo",
                "that are inhabited by many",
                "different monsters. If you enter",
                "these areas, please be extremely",
                "careful.",
            ],
            &[
                "From Comodo, you can travel to",
                "^866C4BMorocc^000000 through the East Cave",
                "and to ^7D2272Umbala^000000 through the North",
                "Cave.",
            ],
            &[
                "To the East, you can find ^BF2B0DParos",
                "^BF2B0DLighthouse^000000, where the Rogue Guild",
                "is located. Enjoy the Comodo night",
                "life~",
            ],
        ],
    )
}

pub fn bulletin_board_13(ctx: &Ctx) -> Script {
    board(
        ctx,
        "Umbala: The Utan Village",
        &[
            &["Welcome to Umbala,", "the village of the Utan tribe."],
            &[
                "Umbala, as well as the Utan tribe",
                "which speak their own unique",
                "language, was recently discovered",
                "by a few intrepid adventurers.",
            ],
            &[
                "Scholars believe that Umbala may be",
                "the border between our world",
                "and another realm. They believe the",
                "junction between the worlds might",
                "be the Yggdrasil tree to the North.",
            ],
            &[
                "Points of interest include the",
                "Chief's House, the Shaman's House,",
                "and the Bungee Jump Area. Thousands",
                "of tourists visit the Bungee Jump",
                "Area to test their courage.",
            ],
            &[
                "From Umbala, head South to go to",
                "^D91B73Comodo^000000. Please enjoy your",
                "stay here in Umbala.",
            ],
        ],
    )
}

pub fn bulletin_board(ctx: &Ctx) -> Script {
    board(
        ctx,
        "Orc Village",
        &[
            &["^6B1312Caution!^000000", "Beyond this point", "lies the Orc Village."],
            &[
                "Be aware that this village is",
                "teeming with dangerous Orcs, namely",
                "Orc Warriors, Orc Ladies and High",
                "Orcs. Two boss monsters, ^6B1312Orc Hero^000000",
                "and ^6B1312Orc Lord^000000 will also appear at certain times.",
            ],
        ],
    )
}

pub fn bulletin_board_l253(ctx: &Ctx) -> Script {
    board(
        ctx,
        "Kobold Village",
        &[&[
            "^6B1312Caution!^000000",
            "You're heading to the Kobold Village.",
            "Please be aware this village is filled with many kobolds.",
        ]],
    )
}

pub fn bulletin_board_15(ctx: &Ctx) -> Script {
    board(
        ctx,
        "Goblin Village",
        &[&[
            "^6B1312Caution!^000000",
            "You're heading to the Goblin Village.",
            "Please be aware this village is filled with many goblins.",
        ]],
    )
}

pub fn bulletin_board_17(ctx: &Ctx) -> Script {
    board(
        ctx,
        "Juno: Capital of",
        &[
            &[
                "[The Schwarzwald Republic]",
                "Welcome to Juno, the City of Sages.",
                "Juno is kept aloft in the air by",
                "the power of the Ymir Heart",
                "Pieces",
            ],
            &[
                "[The Schwarzwald Republic]",
                "Those interested in becoming Sages",
                "should visit the Sage Castle for",
                "more information on the Sage job",
                "and its requirements.",
            ],
            &[
                "[The Schwarzwald Republic]",
                "Other notable places include the",
                "Monster Museum, Magic Academy",
                "and the Juno Library.",
            ],
            &[
                "[The Schwarzwald Republic]",
                "Somewhere around Juno, there is",
                "information regarding secret access",
                "to the world where adventurers may",
                "be reborn with newfound strength.",
            ],
            &[
                "[The Schwarzwald Republic]",
                "To the Southeast of Juno lies ^6B1312Nogg",
                "^6B1312Road^000000, the Magma Dungeon. Nogg Road",
                "is infamous for its aggressive",
                "creatures, so be careful",
            ],
            &[
                "[The Schwarzwald Republic]",
                "From Juno, ^2D3832Al De Baran^000000, a city of",
                "the Rune-Midgarts Kingdom, is",
                "located to the South.",
            ],
        ],
    )
}

pub fn bulletin_board_18(ctx: &Ctx) -> Script {
    board(
        ctx,
        "Al De Baran: The Border City",
        &[
            &[
                "Welcome to Al De Baran, the border",
                "city of the Rune-Midgarts Kingdom.",
                "Al De Baran's beautiful canals and",
                "majestic Clock Tower are a source",
                "of pride for its citizens.",
            ],
            &[
                "Adventurers can explore the Clock",
                "Tower located in the city's center.",
                "Other notable places are the Kafra",
                "Corporation Headquarters, and the",
                "Alchemist Guild which provides the",
                "Alchemist job change.",
            ],
            &[
                "There is a fully trained Santa",
                "Claus somewhere in Al De Baran who",
                "can send you to the magical town of",
                "^1D2585Lutie^000000. If you're interested in",
                "seeing it for yourself, you must",
                "seek Santa Claus.",
            ],
            &[
                "From this city, ^60D5FDJuno^000000 is located to",
                "the North, and ^6D6FE0Prontera^000000 is located",
                "to the South.",
            ],
        ],
    )
}

pub fn bulletin_board_22(ctx: &Ctx) -> Script {
    board(
        ctx,
        "Winter Town, Lutie",
        &[
            &[
                "Welcome to Lutie, the town of snowfall.",
                "Manufacturing toys in the toy factory is the main",
                "business of this town.",
            ],
            &[
                "You can access to the toy factory dungeon",
                "at the north of Lutie.",
                "Please remember to visit Lutie in Christmas season.",
                "There are various event held with joy.",
                "Please beware of ^6B1312Stormy Knight^000000 and ^6B1312Hatii^000000 the boss monsters of the toy factory dungeon.",
            ],
        ],
    )
}

pub fn bulletin_board_25(ctx: &Ctx) -> Script {
    board(
        ctx,
        "City of the Dead, Niflheim",
        &[
            &[
                "Welcome to Niflheim, the City of the Dead.",
                "Niflheim was known as the other world where",
                "you come after the death.",
                "However, recently people found out a secret path behind of a mysterious tree.",
                "So, you will find many other people travelling around this area.",
            ],
            &[
                "As a tourist attraction, The Witch's castle is suggested.",
                "Unlike normal towns, it is prohibited to save respawn point or",
                "warp point inside Niflheim. Also monsters spawn within the town as well.",
            ],
            &[
                "Especially, please be attentive with a boss monster",
                "called ^6B1312Lord of the Death^000000.",
            ],
            &[
                "When you go ahead west, you will arrive at ^6B1312Valley of Gyoll^000000",
                "where all the powerful and fearful monsters dwell upon.",
                "It is suggested to leave the area immediately in case of a new solo adventurer.",
            ],
        ],
    )
}

pub fn bulletin_board_28(ctx: &Ctx) -> Script {
    board(
        ctx,
        "Glast Heim",
        &[
            &[
                "Glast Heim is an enormous dungeon with countless levels.",
                "This dungeon is definitely not for new or experienced adventurers",
                "but for dungeon experts.",
            ],
            &[
                "There are many fearsome monsters such as ^6B1312Dark Lord^000000,",
                "^6B1312Owl Baron^000000, ^6B1312Owl Duke^000000, ^6B1312Dark Illusion^000000, ^6B1312Bloody Knight^000000, ^6B1312Abysmal Knight^000000, ^6B1312Chimera^000000 and various types of doomed swords.",
            ],
            &[
                "However, more difficult the expedition is, greater the reward is.",
                "Therefore, this dungeon is pretty popular among dungeon experts.",
                "Enjoy your dungeon expedition.",
            ],
        ],
    )
}

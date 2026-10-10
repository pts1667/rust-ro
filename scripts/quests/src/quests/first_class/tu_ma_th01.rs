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

pub fn trace_of_battle_1(ctx: &Ctx) -> Script {
    let class_thief = ctx.var("BaseClass").get()? == constants::JOB_THIEF;
    let class_mage = ctx.var("BaseClass").get()? == constants::JOB_MAGE;
    if (!class_thief && !class_mage) || (ctx.ea_class(None)? & constants::EAJL_BABY) != 0 {
        ctx.lines(args![
            "^3355FFThere are signs that show",
            "that some violent scuffle",
            "might have occurred here,",
            "but you can't tell what had",
            "happened exactly...^000000"
        ])?;
    } else if (ctx.var("tu_thief01").get()?.number()? < 5 && class_thief) || (class_mage && ctx.var("tu_magician01").get()?.number()? < 8) {
        ctx.lines(args![
            "^3355FFYou find traces of poison",
            "used in a battle. You sense",
            "its potency and decide not",
            "to get too close to it.^000000"
        ])?;
        if class_mage {
            ctx.call(Function::NpcSpecialEffect, args![constants::EF_VENOMDUST2])?;
        }
    } else if ctx.var("tu_thief01").get()?.number()? == 5 && class_thief {
        ctx.lines(args![
            "^3355FFYou find traces of poison",
            "used in a battle. A feeling",
            "of acute dizziness overcomes",
            "you after examining the scene.^000000"
        ])?;
        ctx.var("tu_thief01").set(6)?;
        ctx.call(Function::StartStatus, args![ctx.constant("SC_POISON")?, 60000, 0])?;
        ctx.call(Function::NpcSpecialEffect, args![constants::EF_VENOMDUST2])?;
    } else if ctx.var("tu_thief01").get()?.number()? == 6 && class_thief {
        ctx.lines(args![
            "^3355FFYou find traces of poison",
            "used in a battle. A feeling",
            "of acute dizziness overcomes",
            "you after examining the scene.^000000"
        ])?;
        ctx.call(Function::StartStatus, args![ctx.constant("SC_POISON")?, 60000, 0])?;
        ctx.call(Function::NpcSpecialEffect, args![constants::EF_VENOMDUST2])?;
    } else if ctx.var("tu_thief01").get()?.number()? == 7 && class_thief {
        ctx.lines(args![
            "^3355FFYou find traces of poison",
            "used in a battle. You sense",
            "its potency and decide not",
            "to get too close to it.^000000"
        ])?;
    } else if (class_thief && ctx.var("tu_thief01").get()?.number()? == 8) || (class_mage && ctx.var("tu_magician01").get()?.number()? == 8)
    {
        ctx.lines(args![
            "^3355FFThere are traces of a battle",
            "that seem to be leading in",
            "different directions. From the",
            "peculiar smell that permeates",
            "the area, it seems that some",
            "kind of lethal poison was used.^000000"
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFStill...",
            "The trail of this",
            "battle decidedly",
            "heads southward.^000000"
        ])?;
        if ctx.var("tu_magician01").get()?.number()? == 8 {
            ctx.var("tu_magician01").set(9)?;
        } else {
            ctx.var("tu_thief01").set(9)?;
        }
    } else {
        ctx.lines(args!["^3355FFYou find signs of", "a heated pursuit", "that head south."])?;
    }
    ctx.close()
}

pub fn trace_of_battle_2(ctx: &Ctx) -> Script {
    let class_thief = ctx.var("BaseClass").get()? == constants::JOB_THIEF;
    let class_mage = ctx.var("BaseClass").get()? == constants::JOB_MAGE;
    if (!class_thief && !class_mage) || (ctx.ea_class(None)? & constants::EAJL_BABY) != 0 {
        ctx.lines(args![
            "^3355FFThere are signs that show",
            "that some violent scuffle",
            "might have occurred here,",
            "but you can't tell what had",
            "happened exactly...^000000"
        ])?;
    } else if (ctx.var("tu_thief01").get()?.number()? < 9 && class_thief) || (ctx.var("tu_magician01").get()?.number()? < 9 && class_mage) {
        ctx.lines(args![
            "^3355FFThere are signs",
            "that many people",
            "have traveled through",
            "this particular area.^000000"
        ])?;
    } else if (ctx.var("tu_thief01").get()?.number()? == 9 && class_thief) || (ctx.var("tu_magician01").get()?.number()? == 9 && class_mage)
    {
        ctx.lines(args![
            "^3355FFYou've found traces",
            "of the pursuit which",
            "continue eastward.^000000"
        ])?;
        if ctx.var("tu_magician01").get()?.number()? == 9 {
            ctx.var("tu_magician01").set(10)?;
        } else {
            ctx.var("tu_thief01").set(10)?;
        }
    } else {
        ctx.lines(args!["^3355FFThese traces of", "the pursuit lead", "towards the east.^000000"])?;
    }
    ctx.close()
}

pub fn trace_of_battle_3(ctx: &Ctx) -> Script {
    let class_thief = ctx.var("BaseClass").get()? == constants::JOB_THIEF;
    let class_mage = ctx.var("BaseClass").get()? == constants::JOB_MAGE;
    if (!class_thief && !class_mage) || (ctx.ea_class(None)? & constants::EAJL_BABY) != 0 {
        ctx.lines(args![
            "^3355FFThese look like",
            "traces of some kind",
            "of pursuit or battle, but",
            "you can't really tell for sure.^000000"
        ])?;
    } else if (ctx.var("tu_thief01").get()?.number()? < 10 && class_thief) || (ctx.var("tu_magician01").get()?.number()? < 10 && class_mage)
    {
        ctx.lines(args![
            "^3355FFThere are signs",
            "that many people",
            "have traveled through",
            "this particular area.^000000"
        ])?;
    } else if (ctx.var("tu_thief01").get()?.number()? == 10 && class_thief)
        || (ctx.var("tu_magician01").get()?.number()? == 10 && class_mage)
    {
        ctx.lines(args![
            "^3355FFYou examine these traces",
            "and notice that one set of",
            "footprints looks almost too",
            "pronounced, as if it had been",
            "made for somebody to find.^000000"
        ])?;
        if ctx.var("tu_magician01").get()?.number()? == 10 {
            ctx.var("tu_magician01").set(11)?;
        } else {
            ctx.var("tu_thief01").set(11)?;
        }
    } else {
        ctx.lines(args!["^3355FFIt's a very", "strange looking", "set of footprints.^000000"])?;
    }
    ctx.close()
}

pub fn trace_of_battle_4(ctx: &Ctx) -> Script {
    let class_thief = ctx.var("BaseClass").get()? == constants::JOB_THIEF;
    let class_mage = ctx.var("BaseClass").get()? == constants::JOB_MAGE;
    if (!class_thief && !class_mage) || (ctx.ea_class(None)? & constants::EAJL_BABY) != 0 {
        ctx.lines(args![
            "^3355FFThere are signs that show",
            "that some violent scuffle",
            "might have occurred here,",
            "but you can't tell what had",
            "happened exactly...^000000"
        ])?;
    } else if (ctx.var("tu_thief01").get()?.number()? < 11 && class_thief) || (ctx.var("tu_magician01").get()?.number()? < 11 && class_mage)
    {
        ctx.lines(args![
            "^3355FFThere are signs",
            "that many people",
            "have traveled through",
            "this particular area.^000000"
        ])?;
    } else if (ctx.var("tu_thief01").get()?.number()? == 11 && class_thief)
        || (ctx.var("tu_magician01").get()?.number()? == 11 && class_mage)
    {
        ctx.lines(args![
            "^3355FFFrom these traces,",
            "you see that another",
            "set of footprints has",
            "been added. It looks like",
            "someone else got involved.",
            "These prints are distinctly",
            "clearer and much smaller.^000000"
        ])?;
        if ctx.var("tu_magician01").get()?.number()? == 11 {
            ctx.var("tu_magician01").set(12)?;
        } else {
            ctx.var("tu_thief01").set(12)?;
        }
    } else {
        ctx.lines(args![
            "^3355FFFrom these traces,",
            "you see that another",
            "person has gotten involved",
            "in this heated scuffle.^000000"
        ])?;
    }
    ctx.close()
}

pub fn trace_of_battle_5(ctx: &Ctx) -> Script {
    let class_thief = ctx.var("BaseClass").get()? == constants::JOB_THIEF;
    let class_mage = ctx.var("BaseClass").get()? == constants::JOB_MAGE;
    if (!class_thief && !class_mage) || (ctx.ea_class(None)? & constants::EAJL_BABY) != 0 {
        ctx.lines(args![
            "^3355FFThese look like",
            "traces of some kind",
            "of pursuit or battle, but",
            "you can't really tell for sure.^000000"
        ])?;
    } else if (ctx.var("tu_thief01").get()?.number()? < 12 && class_thief) || (ctx.var("tu_magician01").get()?.number()? < 12 && class_mage)
    {
        ctx.lines(args![
            "^3355FFThere are signs",
            "that many people",
            "have traveled through",
            "this particular area.^000000"
        ])?;
    } else if (ctx.var("tu_thief01").get()?.number()? == 12 && class_thief)
        || (ctx.var("tu_magician01").get()?.number()? == 12 && class_mage)
    {
        ctx.lines(args![
            "^3355FFThe traces of the battle",
            "now split and head towards",
            "the north and south. However,^000000"
        ])?;
        if ctx.call(Function::CountItem, args![506])? == 0 {
            ctx.mes("^3355FFthere is a puddle of strong poison that you must neutralize before you can investigate this scene.^000000")?;
            return ctx.close();
        }
        ctx.mes("^3355FFthere is a puddle of strong poison that you must neutralize before you can investigate this scene.^000000")?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFFortunately, you can",
            "temporarily nullify the",
            "poison by using one",
            "of your Green Potions.^000000"
        ])?;
        ctx.next()?;
        'b1: {
            let subject1 = runtime::select_values(ctx, &[Val::from("Use Green Potion.:Don't use it.")])?;
            let mut matched1 = false;
            if !matched1 && subject1 == 1 {
                matched1 = true;
            }
            if matched1 {
                ctx.lines(args![
                    "^3355FFThe poison weakens",
                    "and some of it evaporates,",
                    "revealing a piece of cloth that",
                    "was hidden in that puddle.^000000"
                ])?;
                ctx.next()?;
                let subject2 = runtime::select_values(ctx, &[Val::from("Don't investigate.:Investigate.")])?;
                if subject2 == 1 {
                    ctx.lines_as(
                        ctx.player().name()?,
                        args!["A piece of cloth", "is nothing to be", "concerned about."],
                    )?;
                    return ctx.close();
                } else if subject2 == 2 {
                    ctx.lines_as(
                        ctx.player().name()?,
                        args![
                            "Hey... There's",
                            "blood on this cloth",
                            "and some writing on",
                            "it that I can't recognize.",
                            "Hopefully, this'll provide",
                            "some sort of clue to all this?"
                        ],
                    )?;
                    if ctx.var("tu_thief01").get()?.number()? == 12 {
                        ctx.var("tu_thief01").set(13)?;
                    } else {
                        ctx.var("tu_magician01").set(13)?;
                    }
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFYou take the piece",
                        "of cloth from the puddle",
                        "of poison and keep it with you.^000000"
                    ])?;
                    return ctx.close();
                }
            }
            if !matched1 && subject1 == 2 {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as(
                    ctx.player().name()?,
                    args![
                        "Hmm...",
                        "It'll probably be",
                        "faster if I follow this",
                        "trail, rather than stop to",
                        "investigate this scene."
                    ],
                )?;
                return ctx.close();
            }
        }
    } else {
        ctx.lines(args![
            "^3355FFYou can't find",
            "anything else here,",
            "aside from the traces",
            "that split and lead both",
            "northward and southward.^000000"
        ])?;
    }
    ctx.close()
}

pub fn trace_of_battle_6(ctx: &Ctx) -> Script {
    let class_thief = ctx.var("BaseClass").get()? == constants::JOB_THIEF;
    let class_mage = ctx.var("BaseClass").get()? == constants::JOB_MAGE;
    if (!class_thief && !class_mage) || (ctx.ea_class(None)? & constants::EAJL_BABY) != 0 {
        ctx.lines(args![
            "^3355FFThese look like",
            "traces of some kind",
            "of pursuit or battle, but",
            "you can't really tell for sure.^000000"
        ])?;
    } else if (ctx.var("tu_thief01").get()?.number()? < 13 && class_thief) || (ctx.var("tu_magician01").get()?.number()? < 13 && class_mage)
    {
        ctx.lines(args![
            "^3355FFThere are signs",
            "that many people",
            "have traveled through",
            "this particular area.^000000"
        ])?;
    } else {
        ctx.lines(args![
            "^3355FFThese footprints",
            "look like they're",
            "heading towards the",
            "north from the south.",
            "But you can't really",
            "be sure just yet.^000000"
        ])?;
    }
    ctx.close()
}

pub fn trace_of_battle_7(ctx: &Ctx) -> Script {
    let class_thief = ctx.var("BaseClass").get()? == constants::JOB_THIEF;
    let class_mage = ctx.var("BaseClass").get()? == constants::JOB_MAGE;
    if (!class_thief && !class_mage) || (ctx.ea_class(None)? & constants::EAJL_BABY) != 0 {
        ctx.lines(args![
            "^3355FFThese look like",
            "traces of some kind",
            "of pursuit or battle, but",
            "you can't really tell for sure.^000000"
        ])?;
    } else if (ctx.var("tu_thief01").get()?.number()? < 13 && class_thief) || (ctx.var("tu_magician01").get()?.number()? < 13 && class_mage)
    {
        ctx.lines(args![
            "^3355FFThere are signs",
            "that many people",
            "have traveled through",
            "this particular area.^000000"
        ])?;
    } else {
        ctx.lines(args![
            "^3355FFThe trail here looks pretty",
            "muddled, since it looks like",
            "they battled here for quite a",
            "while. But the footprints are",
            "definitely heading south.^000000"
        ])?;
    }
    ctx.close()
}

pub fn trace_of_battle_8(ctx: &Ctx) -> Script {
    let class_thief = ctx.var("BaseClass").get()? == constants::JOB_THIEF;
    let class_mage = ctx.var("BaseClass").get()? == constants::JOB_MAGE;
    if (!class_thief && !class_mage) || (ctx.ea_class(None)? & constants::EAJL_BABY) != 0 {
        ctx.lines(args![
            "^3355FFThese look like",
            "traces of some kind",
            "of pursuit or battle, but",
            "you can't really tell for sure.^000000"
        ])?;
    } else if (ctx.var("tu_thief01").get()?.number()? < 13 && class_thief) || (ctx.var("tu_magician01").get()?.number()? < 13 && class_mage)
    {
        ctx.lines(args![
            "^3355FFThere are two",
            "distinct sets of",
            "footprints in this",
            "area, but they don't",
            "hold any significance",
            "right about now.^000000"
        ])?;
    } else if (ctx.var("tu_thief01").get()?.number()? == 13 && class_thief)
        || (ctx.var("tu_magician01").get()?.number()? == 13 && class_mage)
    {
        ctx.lines(args![
            "^3355FFAround here, it",
            "looks like there are",
            "only two sets of footprints.",
            "What happened to the other",
            "set that you found earlier?^000000"
        ])?;
        if ctx.var("tu_magician01").get()?.number()? == 13 {
            ctx.var("tu_magician01").set(14)?;
        } else {
            ctx.var("tu_thief01").set(14)?;
        }
    } else {
        ctx.lines(args![
            "^3355FFFrom the evidence",
            "that you've found here,",
            "it looks like the battle",
            "involves only two people",
            "from this point onward.^000000"
        ])?;
    }
    ctx.close()
}

pub fn trace_of_battle_9(ctx: &Ctx) -> Script {
    let class_thief = ctx.var("BaseClass").get()? == constants::JOB_THIEF;
    let class_mage = ctx.var("BaseClass").get()? == constants::JOB_MAGE;
    if (!class_thief && !class_mage) || (ctx.ea_class(None)? & constants::EAJL_BABY) != 0 {
        ctx.lines(args![
            "^3355FFThese look like",
            "traces of some kind",
            "of pursuit or battle, but",
            "you can't really tell for sure.^000000"
        ])?;
    } else if (ctx.var("tu_thief01").get()?.number()? < 14 && class_thief) || (ctx.var("tu_magician01").get()?.number()? < 14 && class_mage)
    {
        ctx.lines(args![
            "^3355FFTraces of some",
            "sort of battle are",
            "scattered all over",
            "this particular area.^000000"
        ])?;
    } else if (ctx.var("tu_thief01").get()?.number()? == 14 && class_thief)
        || (ctx.var("tu_magician01").get()?.number()? == 14 && class_mage)
    {
        ctx.lines(args![
            "^3355FFYou follow the trail",
            "and although traces from",
            "the north and southeast mix",
            "together, it looks like the battle continues towards the east.",
            "But you should check this",
            "spot a little bit more...^000000"
        ])?;
        if ctx.var("tu_magician01").get()?.number()? == 14 {
            ctx.var("tu_magician01").set(15)?;
        } else {
            ctx.var("tu_thief01").set(15)?;
        }
    } else if (ctx.var("tu_thief01").get()?.number()? == 15 && class_thief)
        || (ctx.var("tu_magician01").get()?.number()? == 15 && class_mage)
    {
        if ctx.call(Function::Rand, args![1, 10])? == 7 {
            ctx.lines(args![
                "^3355FFAfter investigating this",
                "area more thoroughly,",
                "you find another piece of",
                "cloth stained with blood.",
                "You decide to keep it with",
                "you, hoping that it will",
                "provide more clues.^000000"
            ])?;
            if ctx.var("tu_magician01").get()?.number()? == 15 {
                ctx.var("tu_magician01").set(16)?;
            } else {
                ctx.var("tu_thief01").set(16)?;
            }
        } else {
            ctx.lines(args![
                "^3355FFYou don't find anything,",
                "but you still can't shake",
                "the feeling that there is",
                "some important clue that",
                "you have to find here. It won't hurt to keep investigating here.^000000"
            ])?;
        }
    } else {
        ctx.lines(args![
            "^3355FFYou better continue",
            "following this trail",
            "which leads westward.^000000"
        ])?;
    }
    ctx.close()
}

pub fn trace_of_battle_10(ctx: &Ctx) -> Script {
    let class_thief = ctx.var("BaseClass").get()? == constants::JOB_THIEF;
    let class_mage = ctx.var("BaseClass").get()? == constants::JOB_MAGE;
    if (!class_thief && !class_mage) || (ctx.ea_class(None)? & constants::EAJL_BABY) != 0 {
        ctx.lines(args![
            "^3355FFThese look like",
            "traces of some kind",
            "of pursuit or battle, but",
            "you can't really tell for sure.^000000"
        ])?;
    } else if (ctx.var("tu_thief01").get()?.number()? < 16 && class_thief) || (ctx.var("tu_magician01").get()?.number()? < 16 && class_mage)
    {
        ctx.lines(args![
            "^3355FFThere are signs",
            "showing that a lot",
            "of people were in",
            "this area earlier.^000000"
        ])?;
    } else if (ctx.var("tu_thief01").get()?.number()? == 16 && class_thief)
        || (ctx.var("tu_magician01").get()?.number()? == 16 && class_mage)
    {
        ctx.lines(args![
            "^3355FFThe lead set of footprints,",
            "probably belonging to the one",
            "who was being pursued, look",
            "more erratic, as if exhaustion",
            "and desperation were setting in. These traces lead to the west.^000000"
        ])?;
        if ctx.var("tu_magician01").get()?.number()? == 16 {
            ctx.var("tu_magician01").set(17)?;
        } else {
            ctx.var("tu_thief01").set(17)?;
        }
    } else {
        ctx.lines(args!["^3355FFThe trail from", "this point heads", "towards the west.^000000"])?;
    }
    ctx.close()
}

pub fn trace_of_battle_11(ctx: &Ctx) -> Script {
    let class_thief = ctx.var("BaseClass").get()? == constants::JOB_THIEF;
    let class_mage = ctx.var("BaseClass").get()? == constants::JOB_MAGE;
    if (!class_thief && !class_mage) || (ctx.ea_class(None)? & constants::EAJL_BABY) != 0 {
        ctx.lines(args![
            "^3355FFThese look like",
            "traces of some kind",
            "of pursuit or battle, but",
            "you can't really tell for sure.^000000"
        ])?;
    } else if (ctx.var("tu_thief01").get()?.number()? < 17 && class_thief) || (ctx.var("tu_magician01").get()?.number()? < 17 && class_mage)
    {
        ctx.lines(args![
            "^3355FFThere are signs",
            "showing that a lot",
            "of people were in",
            "this area earlier.^000000"
        ])?;
    } else if (ctx.var("tu_thief01").get()?.number()? == 17 && class_thief)
        || (ctx.var("tu_magician01").get()?.number()? == 17 && class_mage)
    {
        ctx.lines(args![
            "^3355FFFollowing the",
            "trail, you see signs",
            "that blood was spilled",
            "in this area. It looks like",
            "someone was injured",
            "pretty badly around here.^000000"
        ])?;
        if ctx.var("tu_magician01").get()?.number()? == 17 {
            ctx.var("tu_magician01").set(18)?;
        } else {
            ctx.var("tu_thief01").set(18)?;
        }
    } else {
        ctx.lines(args![
            "^3355FFSince someone involved",
            "in this conflict was bleeding,",
            "further traces of this pursuit",
            "might be easier to find now.^000000"
        ])?;
    }
    ctx.close()
}

pub fn trace_of_battle_12(ctx: &Ctx) -> Script {
    let class_thief = ctx.var("BaseClass").get()? == constants::JOB_THIEF;
    let class_mage = ctx.var("BaseClass").get()? == constants::JOB_MAGE;
    if (!class_thief && !class_mage) || (ctx.ea_class(None)? & constants::EAJL_BABY) != 0 {
        ctx.lines(args![
            "^3355FFThese look like",
            "traces of some kind",
            "of pursuit or battle, but",
            "you can't really tell for sure.^000000"
        ])?;
    } else if (ctx.var("tu_thief01").get()?.number()? < 18 && class_thief) || (ctx.var("tu_magician01").get()?.number()? < 18 && class_mage)
    {
        ctx.lines(args![
            "^3355FFThere are signs",
            "showing that a lot",
            "of people were in",
            "this area earlier.^000000"
        ])?;
    } else if (ctx.var("tu_thief01").get()?.number()? == 18 && class_thief)
        || (ctx.var("tu_magician01").get()?.number()? == 18 && class_mage)
    {
        ctx.lines(args![
            "^3355FFThese traces lead",
            "northward and it looks",
            "like whoever is doing",
            "the chasing is getting",
            "much closer to his prey.^000000"
        ])?;
        if ctx.var("tu_magician01").get()?.number()? == 18 {
            ctx.var("tu_magician01").set(19)?;
        } else {
            ctx.var("tu_thief01").set(19)?;
        }
    } else {
        ctx.lines(args![
            "^3355FFThe trail of this",
            "pursuit now leads",
            "towards the north.^000000"
        ])?;
    }
    ctx.close()
}

pub fn trace_of_battle_13(ctx: &Ctx) -> Script {
    let class_thief = ctx.var("BaseClass").get()? == constants::JOB_THIEF;
    let class_mage = ctx.var("BaseClass").get()? == constants::JOB_MAGE;
    if (!class_thief && !class_mage) || (ctx.ea_class(None)? & constants::EAJL_BABY) != 0 {
        ctx.lines(args![
            "^3355FFThese look like",
            "traces of some kind",
            "of pursuit or battle, but",
            "you can't really tell for sure.^000000"
        ])?;
    } else if ctx.var("tu_thief01").get()?.number()? < 19 && class_thief {
        ctx.lines(args![
            "^3355FFThere are signs",
            "showing that a lot",
            "of people were in",
            "this area earlier.^000000"
        ])?;
    } else if ctx.var("tu_magician01").get()?.number()? < 19 && class_mage {
        ctx.mes("A large group of people seem to have gone by.")?;
    } else if (ctx.var("tu_thief01").get()?.number()? == 19 && class_thief)
        || (ctx.var("tu_magician01").get()?.number()? == 19 && class_mage)
    {
        ctx.lines(args![
            "^3355FFJudging from these",
            "traces, it looks like",
            "even more people have",
            "joined the battle which",
            "now seems to be leading",
            "in the southwest direction.^000000"
        ])?;
        if ctx.var("tu_magician01").get()?.number()? == 19 {
            ctx.var("tu_magician01").set(20)?;
        } else {
            ctx.var("tu_thief01").set(20)?;
        }
    } else {
        ctx.lines(args![
            "^3355FFIt looks like",
            "the battle heads",
            "towards the southwest",
            "from this particular point.^000000"
        ])?;
    }
    ctx.close()
}

pub fn trace_of_battle_14(ctx: &Ctx) -> Script {
    let class_thief = ctx.var("BaseClass").get()? == constants::JOB_THIEF;
    let class_mage = ctx.var("BaseClass").get()? == constants::JOB_MAGE;
    if (!class_thief && !class_mage) || (ctx.ea_class(None)? & constants::EAJL_BABY) != 0 {
        ctx.lines(args![
            "^3355FFThese look like",
            "traces of some kind",
            "of pursuit or battle, but",
            "you can't really tell for sure.^000000"
        ])?;
    } else if (ctx.var("tu_thief01").get()?.number()? < 20 && class_thief) || (ctx.var("tu_magician01").get()?.number()? < 20 && class_mage)
    {
        ctx.lines(args![
            "^3355FFThere are signs",
            "showing that a lot",
            "of people were in",
            "this area earlier.^000000"
        ])?;
    } else if (ctx.var("tu_thief01").get()?.number()? == 20 && class_thief)
        || (ctx.var("tu_magician01").get()?.number()? == 20 && class_mage)
    {
        ctx.lines(args![
            "^3355FFIn this area, it looks",
            "like even more people",
            "joined in this battle and",
            "the pursuit clearly heads",
            "towards the south.^000000"
        ])?;
        if ctx.var("tu_magician01").get()?.number()? == 20 {
            ctx.var("tu_magician01").set(21)?;
        } else {
            ctx.var("tu_thief01").set(21)?;
        }
    } else {
        ctx.lines(args!["^3355FFThe trail of", "this battle heads", "towards the south.^000000"])?;
    }
    ctx.close()
}

pub fn trace_of_battle_15(ctx: &Ctx) -> Script {
    let class_thief = ctx.var("BaseClass").get()? == constants::JOB_THIEF;
    let class_mage = ctx.var("BaseClass").get()? == constants::JOB_MAGE;
    if (!class_thief && !class_mage) || (ctx.ea_class(None)? & constants::EAJL_BABY) != 0 {
        ctx.lines(args![
            "^3355FFThese look like",
            "traces of some kind",
            "of pursuit or battle, but",
            "you can't really tell for sure.^000000"
        ])?;
    } else if ctx.var("tu_thief01").get()?.number()? < 21 && class_thief {
        ctx.lines(args![
            "^3355FFThere are signs",
            "showing that a lot",
            "of people were in",
            "this area earlier.^000000"
        ])?;
    } else if ctx.var("tu_magician01").get()?.number()? < 21 && class_mage {
        ctx.mes("A large group of people seem to have gone by.")?;
    } else if (ctx.var("tu_thief01").get()?.number()? == 21 && class_thief)
        || (ctx.var("tu_magician01").get()?.number()? == 21 && class_mage)
    {
        ctx.lines(args![
            "^3355FFThis area is clearly",
            "marked with signs",
            "of a violent battle, with",
            "traces of poison strewn",
            "all over the ground.^000000"
        ])?;
        ctx.next()?;
        let subject1 = runtime::select_values(
            ctx,
            &[Val::from("Continue following the traces.:Further investigate the area.")],
        )?;
        if subject1 == 1 {
            ctx.lines(args![
                "^3355FFYou examine the",
                "trail, but can't really",
                "discern the direction",
                "in which the battle",
                "continues...^000000"
            ])?;
            return ctx.close();
        } else if subject1 == 2 {
            ctx.lines(args![
                "^3355FFYou find a bunch of",
                "traps that use different",
                "kinds of poison. The ones",
                "that have been set off are",
                "mixed with the ones which",
                "haven't been triggered.^000000"
            ])?;
            if ctx.var("tu_thief01").get()?.number()? == 21 {
                ctx.var("tu_thief01").set(22)?;
            } else {
                ctx.var("tu_magician01").set(22)?;
            }
            return ctx.close();
        }
    } else if (ctx.var("tu_thief01").get()?.number()? == 22 && class_thief)
        || (ctx.var("tu_magician01").get()?.number()? == 22 && class_mage)
    {
        if ctx.call(Function::Rand, args![1, 3])? == 3 {
            ctx.lines(args![
                "^3355FFOne trap in particular",
                "stands out to you more",
                "than the rest. Perhaps",
                "you should pour some",
                "Green Potion on it to",
                "neutralize it first.^000000"
            ])?;
            ctx.next()?;
            let subject2 = runtime::select_values(ctx, &[Val::from("Pour Green Potion.:Don't use Green Potion.")])?;
            if subject2 == 1 {
                if ctx.items().count(506)? < 1 {
                    ctx.lines(args![
                        "^3355FFUnfortunately, you",
                        "don't have a Green",
                        "Potion that you can",
                        "use to pour on this trap...^000000"
                    ])?;
                    return ctx.close();
                } else {
                    ctx.lines(args![
                        "^3355FFPouring that",
                        "Green Potion didn't",
                        "really make anything",
                        "happen. Perhaps you",
                        "should try something else.^000000"
                    ])?;
                    ctx.items().take(506, 1)?;
                    if ctx.var("tu_thief01").get()?.number()? == 22 {
                        ctx.var("tu_thief01").set(23)?;
                    } else {
                        ctx.var("tu_magician01").set(23)?;
                    }
                    return ctx.close();
                }
            } else if subject2 == 2 {
                ctx.lines_as(
                    ctx.player().name()?,
                    args!["I guess...", "I'll try investigating", "this area a little more?"],
                )?;
                return ctx.close();
            }
        } else {
            ctx.lines(args![
                "^3355FFBy sheer accident,",
                "you set off one of",
                "the traps in the area.^000000"
            ])?;
            ctx.call(Function::StartStatus, args![ctx.constant("SC_POISON")?, 60000, 0])?;
            ctx.call(Function::NpcSpecialEffect, args![constants::EF_VENOMDUST2])?;
            ctx.call(Function::PercentHeal, args![-30, 0])?;
        }
    } else if (ctx.var("tu_thief01").get()?.number()? == 23 && class_thief)
        || (ctx.var("tu_magician01").get()?.number()? == 23 && class_mage)
    {
        if ctx.items().count(511)? > 0 && ctx.items().count(716)? > 0 {
            ctx.lines(args![
                "^3355FFYou try grinding",
                "a Green Herb and",
                "sprinkling it on the",
                "trap and then place",
                "a Red Gemstone on it.",
                "The gem glows and",
                "slowly melts away...^000000"
            ])?;
            ctx.next()?;
            ctx.lines(args![
                "^3355FFAlthough you don't",
                "fully understand the",
                "science of using poisons",
                "or antidotes, you managed",
                "to successfully dismantle",
                "the trap. Now you can safely",
                "check what might be inside.^000000"
            ])?;
            ctx.items().take(511, 1)?;
            ctx.items().take(716, 1)?;
            if ctx.var("tu_thief01").get()?.number()? == 23 {
                ctx.var("tu_thief01").set(24)?;
            } else {
                ctx.var("tu_magician01").set(24)?;
            }
        } else if ctx.items().count(511)? > 0 {
            ctx.lines(args![
                "^3355FFYou should try to",
                "dismantle this trap",
                "by using other catalysts",
                "related to the curing or",
                "use of poison. You do have",
                "a Green Herb on you, so you",
                "try sprinkling it on the trap.^000000"
            ])?;
            ctx.next()?;
            ctx.lines(args![
                "^3355FFHowever, nothing",
                "happens. It seems that",
                "you need another catalyst",
                "in addition to the Green Herb",
                "that you have in order to",
                "dismantle this trap.^000000"
            ])?;
        } else if ctx.items().count(716)? > 0 {
            ctx.lines(args![
                "^3355FFYou should try to",
                "dismantle this trap",
                "by using other catalysts",
                "related to the curing or use",
                "of poison. You do have a",
                "Red Gemstone, so you grind",
                "it and sprinkle it on the trap.^000000"
            ])?;
            ctx.next()?;
            ctx.lines(args![
                "^3355FFHowever, nothing",
                "happens. It seems that",
                "you need another catalyst to",
                "use with the Red Gemstone",
                "that you have in order to",
                "dismantle this trap.^000000"
            ])?;
        } else {
            ctx.lines(args![
                "^3355FFYou should try to",
                "dismantle this trap",
                "by using other catalysts",
                "related to the curing or",
                "use of poison. But what",
                "items should you bring?^000000"
            ])?;
        }
    } else if (ctx.var("tu_thief01").get()?.number()? == 24 && class_thief)
        || (ctx.var("tu_magician01").get()?.number()? == 24 && class_mage)
    {
        ctx.lines(args![
            "^3355FFInside the dismantled",
            "trap, you find another",
            "piece of strange cloth",
            "that's stained with blood.",
            "You take it with you in",
            "hopes that it provides",
            "some kind of evidence.^000000"
        ])?;
        if ctx.var("tu_thief01").get()?.number()? == 24 {
            ctx.var("tu_thief01").set(25)?;
        } else {
            ctx.var("tu_magician01").set(25)?;
        }
    } else {
        ctx.lines(args![
            "^3355FFYou examine the area",
            "a little further and guess",
            "that the battle might head",
            "towards the south.^000000"
        ])?;
    }
    ctx.close()
}

pub fn trace_of_battle_16(ctx: &Ctx) -> Script {
    let class_thief = ctx.var("BaseClass").get()? == constants::JOB_THIEF;
    let class_mage = ctx.var("BaseClass").get()? == constants::JOB_MAGE;
    if (!class_thief && !class_mage) || (ctx.ea_class(None)? & constants::EAJL_BABY) != 0 {
        ctx.lines(args![
            "^3355FFThese look like",
            "traces of some kind",
            "of pursuit or battle, but",
            "you can't really tell for sure.^000000"
        ])?;
    } else if (ctx.var("tu_thief01").get()?.number()? < 25 && class_thief) || (ctx.var("tu_magician01").get()?.number()? < 25 && class_mage)
    {
        ctx.lines(args![
            "^3355FFThere are signs",
            "that a large group",
            "of people have been in",
            "this area for some reason.^000000"
        ])?;
    } else if (ctx.var("tu_thief01").get()?.number()? == 25 && class_thief)
        || (ctx.var("tu_magician01").get()?.number()? == 25 && class_mage)
    {
        ctx.lines(args![
            "^3355FFThese traces lead",
            "to the edge of the cliff.",
            "marking the end of the trail.",
            "Apparently, the one who was",
            "being chased met his fate here.^000000"
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFThere are many footprints",
            "leading to the edge of the",
            "cliff and all of them leave",
            "this scene, save for the set",
            "of footprints that distinctly",
            "belong to the person who",
            "was pursued all this time...^3355FF"
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFIt looks like you've",
            "learned all that you can",
            "from this investigation. You",
            "better return and report your",
            "findings to Yierhan soon.^000000"
        ])?;
        if ctx.var("tu_magician01").get()?.number()? == 25 {
            ctx.var("tu_magician01").set(26)?;
        } else {
            ctx.var("tu_thief01").set(26)?;
        }
    } else {
        ctx.lines(args![
            "^3355FFThis is the end",
            "of the trail. There",
            "aren't any more traces",
            "of the battle left to find.^000000"
        ])?;
    }
    ctx.close()
}

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

pub fn mischna(ctx: &Ctx) -> Script {
    let mut convert = Val::from(0);
    let mut element = Val::from(0);
    let mut idx = Val::from(0);
    let mut req_items: Vec<Val> = Vec::new();
    let mut req_item_names: Vec<Val> = Vec::new();
    let mut req_skills: Vec<Val> = Vec::new();
    let mut req_skill_names: Vec<Val> = Vec::new();
    let mut skills: Vec<Val> = Vec::new();
    let mut skill_names: Vec<Val> = Vec::new();
    if ctx.var("BaseJob").get()? != constants::JOB_SAGE {
        ctx.lines_as(
            "Mishuna",
            args![
                "Good day, I'm Mishuna, one",
                "of the instructors here in the",
                "Schweicherbil Magic Academy.",
                "By any chance, are you a Sage",
                "or Scholar? Oh... You're not?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Mishuna",
            args![
                "Oh, that's too bad.",
                "My apologies. But if you",
                "happen to know any, or meet",
                "any in your journeys, please",
                "direct them to me if they haven't heard of the lessons I provide."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Mishuna",
            args![
                "I'm sorry to bother",
                "you, and I thank you",
                "for your time. Good",
                "day to you, adventurer."
            ],
        )?;
        return ctx.close();
    }
    if ctx.var("sag_sk").get()? == 100 {
        element = ctx.call(Function::GetSkillLv, args!["SA_ELEMENTFIRE"])?
            + ctx.call(Function::GetSkillLv, args!["SA_ELEMENTGROUND"])?
            + ctx.call(Function::GetSkillLv, args!["SA_ELEMENTWIND"])?
            + ctx.call(Function::GetSkillLv, args!["SA_ELEMENTWATER"])?;
        convert = ctx.call(Function::GetSkillLv, args!["SA_CREATECON"])?;
        if element.is_true() && convert.is_true() {
            ctx.lines_as(
                "Mishuna",
                args![
                    "If you have any Sage or",
                    "Scholar friends who haven't",
                    "learned the skills from Sir",
                    "Barmundt's scrolls, then",
                    "please refer them to me."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Mishuna",
                args![
                    "I trust that you are",
                    "finding that these",
                    "element based skills",
                    "are very useful in battle.",
                    "Knowledge truly equates",
                    "to power in the long run..."
                ],
            )?;
            return ctx.close();
        }
        ctx.lines_as(
            "Mishuna",
            args![
                "Ah, you must have",
                "forgotten what I taught",
                "you somehow. Perhaps",
                "you lost your copy of the",
                "skill scroll I gave you? No",
                "matter, I'll help you remember."
            ],
        )?;
        ctx.next()?;
        if element == 0 {
            ctx.lines_as(
                "Mishuna",
                args![
                    "You'll be given the chance to",
                    "choose which kind of ^FF0000Elemental",
                    "Change^000000 skill that you want, even one that you didn't previously",
                    "learn, so long as you fulfill",
                    "the skill's requirements."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Mishuna",
                args![
                    "Keep in mind that once",
                    "you learn your Elemental",
                    "Change skill, you won't be",
                    "able to change it. Now, which",
                    "skill would you like to learn?"
                ],
            )?;
            ctx.next()?;
            runtime::local_set(&mut req_skills, &Val::from(0), Val::from(280), false);
            runtime::local_set(&mut req_skills, &Val::from(1), Val::from(283), false);
            runtime::local_set(&mut req_skills, &Val::from(2), Val::from(282), false);
            runtime::local_set(&mut req_skills, &Val::from(3), Val::from(281), false);
            runtime::local_set(&mut req_skill_names, &Val::from(0), Val::from("Blaze"), true);
            runtime::local_set(&mut req_skill_names, &Val::from(1), Val::from("Quake"), true);
            runtime::local_set(&mut req_skill_names, &Val::from(2), Val::from("Tornado"), true);
            runtime::local_set(&mut req_skill_names, &Val::from(3), Val::from("Tsunami"), true);
            runtime::local_set(&mut skills, &Val::from(0), Val::from(1018), false);
            runtime::local_set(&mut skills, &Val::from(1), Val::from(1017), false);
            runtime::local_set(&mut skills, &Val::from(2), Val::from(1019), false);
            runtime::local_set(&mut skills, &Val::from(3), Val::from(1008), false);
            runtime::local_set(&mut skill_names, &Val::from(0), Val::from("Fire"), true);
            runtime::local_set(&mut skill_names, &Val::from(1), Val::from("Earth"), true);
            runtime::local_set(&mut skill_names, &Val::from(2), Val::from("Wind"), true);
            runtime::local_set(&mut skill_names, &Val::from(3), Val::from("Water"), true);
            'l1: loop {
                idx = Val::from(ctx.menu(&[
                    "Fire Elemental Change",
                    "Earth Elemental Change",
                    "Wind Elemental Change",
                    "Water Elemental Change",
                ])? as i32);
                if ctx.call(Function::GetSkillLv, args![runtime::local_get(&req_skills, &idx, false)])? == 0 {
                    ctx.lines_as(
                        "Mishuna",
                        args![
                            "I'm sorry, but you haven't",
                            ((Val::from("learned ^FF0000Endow ") + runtime::local_get(&req_skill_names, &idx, true))
                                + Val::from("^000000, the skill")),
                            ((Val::from("required for ^FF0000") + runtime::local_get(&skill_names, &idx, true)) + Val::from(" Elemental")),
                            "Change^000000. You'll need to learn",
                            ((Val::from("Endow ") + runtime::local_get(&req_skill_names, &idx, true)) + Val::from(" or select another")),
                            "Elemental Change skill."
                        ],
                    )?;
                    ctx.next()?;
                } else {
                    ctx.lines_as(
                        "Mishuna",
                        args![
                            "Very well, I shall",
                            (Val::from("teach you the ^FF0000") + runtime::local_get(&skill_names, &idx, true)),
                            "Elemental Change^000000 skill",
                            "and the ^FF0000Elemental Converter",
                            "Creation skill^000000. Please remain",
                            "still while I chant this spell."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Mishuna", args!["%$#@!#$% Yap~~"])?;
                    ctx.call(Function::SpecialEffect, args![constants::EF_RUWACH])?;
                    ctx.next()?;
                    ctx.lines_as("Mishuna", args!["Yap!"])?;
                    ctx.call(Function::SpecialEffect, args![constants::EF_BRANDISHSPEAR])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFYou've successfully",
                        ((Val::from("learned the ") + runtime::local_get(&skill_names, &idx, true)) + Val::from(" Elemental")),
                        "Change skill and the Elemental",
                        "Converter Creation skill.^000000"
                    ])?;
                    ctx.call(
                        Function::Skill,
                        args![runtime::local_get(&skills, &idx, false), 1, constants::SKILL_PERM,],
                    )?;
                    if convert == 0 {
                        ctx.call(Function::Skill, args!["SA_CREATECON", 1, constants::SKILL_PERM])?;
                    }
                    ctx.next()?;
                    break 'l1;
                }
            }
        } else if convert == 0 {
            ctx.lines_as(
                "Mishuna",
                args![
                    "Alright, I'm going",
                    "to cast a spell that",
                    "will help you remember",
                    "the skills you forgot.",
                    "Don't move, and try to",
                    "stay as still as possible..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Mishuna", args!["%$#@!#$% Yap~~"])?;
            ctx.call(Function::SpecialEffect, args![constants::EF_RUWACH])?;
            ctx.next()?;
            ctx.lines_as("Mishuna", args!["Yap!"])?;
            ctx.call(Function::SpecialEffect, args![constants::EF_BRANDISHSPEAR])?;
            ctx.next()?;
            ctx.lines(args![
                "^3355FFYou successfully recalled",
                "the Elemental Coverter",
                "Creation skill and are",
                "able to use it again.^000000"
            ])?;
            ctx.call(Function::Skill, args!["SA_CREATECON", 1, constants::SKILL_PERM])?;
            ctx.next()?;
        }
        ctx.lines_as(
            "Mishuna",
            args![
                "Ah, you've learned these",
                "skills as quickly as I thought",
                "you would. Very well then,",
                "I hope you adeptly use these",
                "talents for the right purposes.",
                Val::from("Farewell for now, ") + ctx.player().name()? + Val::from(".")
            ],
        )?;
        return ctx.close();
    } else if ctx.var("sag_sk").get()? == 0 {
        ctx.lines_as(
            "Mishuna",
            args![
                "Good day, I'm Mishuna, one",
                "of the instructors here in the",
                "Schweicherbil Magic Academy.",
                "How may I be of service?"
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("I seek new knowledge.")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Mishuna",
            args![
                Val::from("Ah, you must be ") + ctx.player().name()? + Val::from("."),
                "I've looked forward to meeting",
                "you. In the noble pursuit of",
                "knowledge, might I suggest",
                "reading the recently restored",
                "scrolls of Sir Barmundt?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Mishuna",
            args![
                "Sir Barmundt's scrolls contain",
                "knowledge about the 4 elements,",
                "which are Fire, Water, Earth, and Wind. The knowledge of these",
                "scrolls can be applied in the",
                "use of 2 new Sage skills."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Mishuna",
            args![
                "The first is called ^FF0000Elemental",
                "Change^000000, which enables you",
                "to change a monster's attribute",
                "according to the specific element of the Elemental Change skill",
                "that you have learned."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Mishuna",
            args![
                "The second is called",
                "Elemental Converter Creation,",
                "which enables you to create",
                "converter items that are required to use the Elemental Change skill."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Mishuna",
            args![
                "Although the knowledge of",
                "these two skills has been",
                "lost for years, we've finally",
                "been able to recover most",
                "of it. So, do you think you're",
                "ready to learn these skills?"
            ],
        )?;
        ctx.next()?;
        if ctx.menu(&["Maybe later.", "Yes, I am."])? == 0 {
            ctx.lines_as(
                "Mishuna",
                args![
                    "Ah, you must be busy right",
                    "now. No problem, just come",
                    "back when you think you're",
                    "ready to learn. Well then,",
                    "farewell and have a good day~"
                ],
            )?;
            return ctx.close();
        }
        ctx.lines_as(
            "Mishuna",
            args![
                "Very well, then. First, you",
                "must learn the Elemental",
                "Coverter Creation skill, which",
                "is essential to learning the",
                "Elemental Change skill."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Mishuna",
            args![
                "Please bring the required",
                "materials so that we can",
                "construct a basic elemental",
                "converter in order for you to",
                "learn the skill. Let's see,",
                "you will need to bring..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Mishuna",
            args![
                "^ff00007 Horns^000000,",
                "^ff000012 Rainbow Shells^000000,",
                "^ff000010 Snail's Shells^000000,",
                "^ff00004 Blank Scrolls^000000 and",
                "^ff000010 Scorpion Tails^000000.^000000"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Mishuna",
            args![
                "Alright, I shall be",
                "nexting here for your",
                "return. Remember that we",
                "need these items to create",
                "a converter so that you can learn the skill from my example..."
            ],
        )?;
        ctx.var("sag_sk").set(Val::from(1))?;
        return ctx.close();
    } else if ctx.var("sag_sk").get()? == 1 {
        if ctx.items().count(904)? < 10
            || ctx.items().count(947)? < 7
            || ctx.items().count(1013)? < 12
            || ctx.items().count(946)? < 10
            || ctx.items().count(7433)? < 4
        {
            ctx.lines_as(
                "Mishuna",
                args![
                    "Hm, you still haven't",
                    "gathered all of the materials",
                    "required to create an elemental",
                    "coverter. Let me remind you",
                    "what to bring so that you",
                    "don't forget next time..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Mishuna",
                args![
                    "^ff00007 Horns^000000,",
                    "^ff000012 Rainbow Shells^000000,",
                    "^ff000010 Snail's Shells^000000,",
                    "^ff00004 Blank Scrolls^000000 and",
                    "^ff000010 Scorpion Tails^000000.^000000"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Mishuna",
                args![
                    "Don't forget that we need",
                    "all of these items to create",
                    "a converter so that you can",
                    "learn the Elemental Converter",
                    "Creation skill by watching",
                    "me demonstrate it for you."
                ],
            )?;
            return ctx.close();
        } else {
            ctx.lines_as(
                "Mishuna",
                args![
                    "Great, you brought everything.",
                    "Now, let me explain the skill.",
                    "The skills you learn as a Sage",
                    "determine what kind of elemental converters that you can craft."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Mishuna",
                args![
                    "The ^FF0000Endow Blaze^000000 skill enables",
                    "you to create Fire elemental",
                    "converters. The ^FF0000Endow Quake^000000",
                    "skill enables the creation",
                    "of Earth elemental converters."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Mishuna",
                args![
                    "^FF0000Endow Tornado^000000 enables",
                    "the creation of Wind elemental",
                    "converters, and ^FF0000Endow Tsunami^000000",
                    "enables the creation of Water",
                    "elemental converters. That",
                    "all makes sense, right?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Mishuna",
                args![
                    "Now, your elemental coverter",
                    "creation success rate depends",
                    "on the level of the Endow Blaze, Endow Quake, Endow Tornado,",
                    "or Endow Tsunami skills, and",
                    "your abilities."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Mishuna",
                args![
                    "Now, please take this copy",
                    "of Barmundt's scroll, and use",
                    "it as a reference when you try",
                    "to craft elemental converters",
                    "when you use the Elemental",
                    "Converter Creation skill."
                ],
            )?;
            ctx.next()?;
            ctx.lines(args![
                "^3355FFYou have learned the",
                "Elemental Converter",
                "Creation skill by reviewing",
                "your copy of Barmundt's scroll.^000000"
            ])?;
            ctx.call(Function::SpecialEffect, args![constants::EF_ABSORBSPIRITS])?;
            ctx.items().take(904, 10)?;
            ctx.items().take(947, 7)?;
            ctx.items().take(1013, 12)?;
            ctx.items().take(946, 10)?;
            ctx.items().take(7433, 4)?;
            ctx.var("sag_sk").set(Val::from(2))?;
            ctx.call(Function::Skill, args!["SA_CREATECON", 1, constants::SKILL_PERM])?;
            ctx.next()?;
            ctx.lines_as(
                "Mishuna",
                args![
                    Val::from("Wow, ") + ctx.player().name()? + Val::from("!"),
                    "You learned that skill",
                    "really quickly! No wonder",
                    "people say that you're one",
                    "of the best Sages around!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Mishuna",
                args![
                    "Now you're ready to",
                    "learn the other skill,",
                    "Elemental Change. Alright,",
                    "I need to prepare a few things",
                    "for this lesson, so we'll meet",
                    "and discuss this later, okay?"
                ],
            )?;
            return ctx.close();
        }
    } else if ctx.var("sag_sk").get()? == 2 {
        if ctx.call(Function::GetSkillLv, args!["SA_CREATECON"])? == 0 {
            ctx.call(Function::Skill, args!["SA_CREATECON", 1, constants::SKILL_PERM])?;
            ctx.mes("- I recalled ^ff0000Elemental Converter Creation skill^000000 While I talk to Mishuna! -")?;
            ctx.next()?;
        }
        ctx.lines_as(
            "Mishuna",
            args![
                "You'll be given the chance to",
                "choose which kind of ^FF0000Elemental",
                "Change^000000 skill that you want, even one that you didn't previously",
                "learn, so long as you fulfill",
                "the skill's requirements."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Mishuna",
            args![
                "Keep in mind that once",
                "you learn your Elemental",
                "Change skill, you won't be",
                "able to change it. Now, which",
                "skill would you like to learn?"
            ],
        )?;
        ctx.next()?;
        runtime::local_set(&mut req_skills, &Val::from(0), Val::from(280), false);
        runtime::local_set(&mut req_skills, &Val::from(1), Val::from(283), false);
        runtime::local_set(&mut req_skills, &Val::from(2), Val::from(282), false);
        runtime::local_set(&mut req_skills, &Val::from(3), Val::from(281), false);
        runtime::local_set(&mut req_skill_names, &Val::from(0), Val::from("Blaze"), true);
        runtime::local_set(&mut req_skill_names, &Val::from(1), Val::from("Quake"), true);
        runtime::local_set(&mut req_skill_names, &Val::from(2), Val::from("Tornado"), true);
        runtime::local_set(&mut req_skill_names, &Val::from(3), Val::from("Tsunami"), true);
        runtime::local_set(&mut req_item_names, &Val::from(0), Val::from("Red Bloods"), true);
        runtime::local_set(&mut req_item_names, &Val::from(1), Val::from("Green Lives"), true);
        runtime::local_set(&mut req_item_names, &Val::from(2), Val::from("Wind of Verdure"), true);
        runtime::local_set(&mut req_item_names, &Val::from(3), Val::from("Crystal Blues"), true);
        runtime::local_set(&mut skill_names, &Val::from(0), Val::from("Fire"), true);
        runtime::local_set(&mut skill_names, &Val::from(1), Val::from("Earth"), true);
        runtime::local_set(&mut skill_names, &Val::from(2), Val::from("Wind"), true);
        runtime::local_set(&mut skill_names, &Val::from(3), Val::from("Water"), true);
        loop {
            idx = Val::from(ctx.menu(&[
                "Fire Elemental Change",
                "Earth Elemental Change",
                "Wind Elemental Change",
                "Water Elemental Change",
            ])? as i32);
            if ctx.call(Function::GetSkillLv, args![runtime::local_get(&req_skills, &idx, false)])? == 0 {
                ctx.lines_as(
                    "Mishuna",
                    args![
                        "I'm sorry, but you have not",
                        ((Val::from("learned ^FF0000Endow ") + runtime::local_get(&req_skill_names, &idx, true))
                            + Val::from("^000000, the skill")),
                        ((Val::from("required for the ") + runtime::local_get(&skill_names, &idx, true))
                            + Val::from(" Elemental Change skill. Please learn Endow")),
                        (runtime::local_get(&req_skill_names, &idx, true)
                            + Val::from(" or select another Elemental Change skill for me to teach you."))
                    ],
                )?;
                ctx.next()?;
            } else {
                ctx.lines_as(
                    "Mishuna",
                    args![
                        "Very well, then. Please",
                        "bring the following items",
                        "so that you can learn the",
                        (runtime::local_get(&skill_names, &idx, true) + Val::from(" Elemental Change skill."))
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mishuna",
                    args![
                        ((Val::from("^FF000020 ") + runtime::local_get(&req_item_names, &idx, true)) + Val::from("^000000,")),
                        "^FF00001 Payon Solution^000000 and",
                        "^FF00001 Morocc Solution^000000. Please",
                        "return to me once you have",
                        "all the materials ready."
                    ],
                )?;
                ctx.var("sag_sk").set(Val::from(10 * (idx.number()? + 1)))?;
                return ctx.close();
            }
        }
    } else if ctx.var("sag_sk").get()? == 10
        || ctx.var("sag_sk").get()? == 20
        || ctx.var("sag_sk").get()? == 30
        || ctx.var("sag_sk").get()? == 40
    {
        if ctx.call(Function::GetSkillLv, args!["SA_CREATECON"])? == 0 {
            ctx.call(Function::Skill, args!["SA_CREATECON", 1, constants::SKILL_PERM])?;
            ctx.mes("- I recalled ^ff0000Elemental Converter Creation skill^000000 While I talk to Mishuna! -")?;
            ctx.next()?;
        }
        idx = Val::from(ctx.var("sag_sk").get()?.number()? / 10 - 1);
        runtime::local_set(&mut req_items, &Val::from(0), Val::from(990), false);
        runtime::local_set(&mut req_items, &Val::from(1), Val::from(993), false);
        runtime::local_set(&mut req_items, &Val::from(2), Val::from(992), false);
        runtime::local_set(&mut req_items, &Val::from(3), Val::from(991), false);
        runtime::local_set(&mut req_item_names, &Val::from(0), Val::from("Red Bloods"), true);
        runtime::local_set(&mut req_item_names, &Val::from(1), Val::from("Green Lives"), true);
        runtime::local_set(&mut req_item_names, &Val::from(2), Val::from("Wind of Verdure"), true);
        runtime::local_set(&mut req_item_names, &Val::from(3), Val::from("Crystal Blues"), true);
        runtime::local_set(&mut skills, &Val::from(0), Val::from(1018), false);
        runtime::local_set(&mut skills, &Val::from(1), Val::from(1017), false);
        runtime::local_set(&mut skills, &Val::from(2), Val::from(1019), false);
        runtime::local_set(&mut skills, &Val::from(3), Val::from(1008), false);
        runtime::local_set(&mut skill_names, &Val::from(0), Val::from("Fire"), true);
        runtime::local_set(&mut skill_names, &Val::from(1), Val::from("Earth"), true);
        runtime::local_set(&mut skill_names, &Val::from(2), Val::from("Wind"), true);
        runtime::local_set(&mut skill_names, &Val::from(3), Val::from("Water"), true);
        if ctx
            .call(Function::CountItem, args![runtime::local_get(&req_items, &idx, false)])?
            .number()?
            < 20
            || ctx.items().count(1089)? < 1
            || ctx.items().count(1088)? < 1
        {
            ctx.lines_as(
                "Mishuna",
                args![
                    "Are you having trouble",
                    "gathering all the required",
                    "items? Just in case, let me",
                    "remind you of what you need",
                    (Val::from("to bring me to learn the ") + runtime::local_get(&skill_names, &idx, true)),
                    "Elemental Change skill."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Mishuna",
                args![
                    ((Val::from("^ff000020 ") + runtime::local_get(&req_item_names, &idx, true)) + Val::from("^000000,")),
                    "^ff00001 Payon Solution^000000 and",
                    "^ff00001 Morocc Solution^000000. Please",
                    "don't forget and have the",
                    "materials ready for the next",
                    "time you see me, alright?"
                ],
            )?;
            return ctx.close();
        }
        ctx.lines_as(
            "Mishuna",
            args![
                "Ah, you're back. I can now",
                "finally teach you about the",
                (runtime::local_get(&skill_names, &idx, true) + Val::from(" Elemental Change skill.")),
                "This skill has the chance to",
                "permanently change a targeted",
                ((Val::from("monster's attribute to ") + runtime::local_get(&skill_names, &idx, true)) + Val::from("."))
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Mishuna",
            args![
                "Remember that you must",
                "use an elemental converter to",
                "cast this skill, and that it has a success rate, similarly to the",
                "Elemental Converter Creation",
                "skill. So be aware of that."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Mishuna",
            args![
                Val::from("Now, ") + ctx.player().name()? + Val::from(","),
                "I'm going to cast a spell",
                "that will help you memorize",
                ((Val::from("the ") + runtime::local_get(&skill_names, &idx, true)) + Val::from(" Elemental Change")),
                "skill. Try to stay still..."
            ],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFMishuna begins to chant",
            "a strange incantation as",
            "a soft blue glow surrounds",
            "his body and slowly grows",
            "brighter and more intense.^000000"
        ])?;
        ctx.call(Function::SpecialEffect, args![constants::EF_RUWACH])?;
        ctx.next()?;
        ctx.lines_as("Mishuna", args!["@#$%^~ Yap!"])?;
        ctx.call(Function::SpecialEffect, args![constants::EF_BRANDISHSPEAR])?;
        ctx.call(Function::DelItem, args![runtime::local_get(&req_items, &idx, false), 20])?;
        ctx.items().take(1089, 1)?;
        ctx.items().take(1088, 1)?;
        ctx.var("sag_sk").set(Val::from(100))?;
        ctx.call(Function::Skill, args![runtime::local_get(&skills, &idx, false), 1, 0])?;
        ctx.next()?;
        ctx.lines_as(
            "Mishuna",
            args![
                Val::from(ctx.player().name()?) + Val::from("..."),
                "I'm happy to say that you've",
                "successfully memorized the",
                (runtime::local_get(&skill_names, &idx, true) + Val::from(" Elemental Change skill.")),
                "I hope that it serves you well",
                "in battle. Farewell for now~"
            ],
        )?;
        return ctx.close();
    } else {
        ctx.lines_as(
            "Mishuna",
            args![
                "If you have any Sage or",
                "Scholar friends who haven't",
                "learned the skills from Sir",
                "Barmundt's scrolls, then",
                "please refer them to me."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Mishuna",
            args![
                "I trust that you are",
                "finding that these",
                "element based skills",
                "are very useful in battle.",
                "Knowledge truly equates",
                "to power in the long run..."
            ],
        )?;
        return ctx.close();
    }
    Ok(())
}

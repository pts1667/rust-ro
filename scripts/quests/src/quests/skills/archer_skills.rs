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

pub fn roberto(ctx: &Ctx) -> Script {
    if ctx.var("BaseClass").get()? == constants::JOB_ARCHER {
        if ctx.call(Function::GetSkillLv, args!["AC_MAKINGARROW"])? == 1 {
            ctx.lines_as(
                "Roberto",
                args![
                    "Ooh, you're from my home town!",
                    "Nice to see you!",
                    "How are you?",
                    "Ah! That arrow!",
                    "You made it, didn't you!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Roberto",
                args![
                    "Haha...!",
                    "Do you think it's a lot better?",
                    "Haha... anyways, I am glad.",
                    "Come back with once in a while",
                    "with news from home.",
                    "Then byebye~"
                ],
            )?;
            return ctx.close();
        }
        if ctx.player().job_level()? >= 30
            || ctx.var("BaseJob").get()? == constants::JOB_HUNTER
            || ctx.var("BaseJob").get()? == constants::JOB_BARD
            || ctx.var("BaseJob").get()? == constants::JOB_DANCER
        {
            ctx.lines_as("Roberto", args!["Eh!", " ", "You are..."])?;
            ctx.next()?;
            if ctx.items().count(907)? > 19
                && ctx.items().count(921)? > 6
                && ctx.items().count(906)? > 40
                && ctx.items().count(1019)? > 12
                && ctx.items().count(501)? > 0
            {
                ctx.lines_as(
                    "Roberto",
                    args![
                        "You brought them!",
                        "Thank you very much.",
                        "Then, as I promised, I will teach you the skill."
                    ],
                )?;
                ctx.next()?;
                ctx.items().take(907, 20)?;
                ctx.items().take(921, 7)?;
                ctx.items().take(906, 41)?;
                ctx.items().take(1019, 13)?;
                ctx.items().take(501, 1)?;
                ctx.call(Function::Skill, args!["AC_MAKINGARROW", 1, constants::SKILL_PERM])?;
                ctx.lines_as(
                    "Roberto",
                    args![
                        "No need to worry about arrows now.",
                        "Oh, and did you happen to see",
                        "someone called Jason in Payon?",
                        "Be careful. He is a",
                        "ferocious one."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Roberto",
                    args![
                        "You just have to be careful of Jason in Payon.",
                        "Remember.",
                        "Then bubye~ Thank you for the presents~"
                    ],
                )?;
                return ctx.close();
            }
            ctx.lines_as(
                "Roberto",
                args![
                    "An archer in Morocc!?",
                    "Nice to see you! Meeting a fellow",
                    "archer in a place like this! *sniffsniff*!",
                    "I came alone to Morocc..",
                    "but I was a newcomer, and the pressure... waaah~",
                    "I was very lonely~"
                ],
            )?;
            ctx.next()?;
            match ctx.menu(&["It must be hard. It's ok have faith.", "Keep suffering."])? {
                0 => {
                    ctx.lines_as(
                        "Roberto",
                        args![
                            "Yes. Thank you...",
                            "You must be having a hard",
                            "time in a place like this.",
                            "Isn't it hard to find arrows?",
                            "That's why I make my own."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.var("@menu").set(runtime::select_values(ctx, &[Val::from("Eh, really?!")])?)?;
                    ctx.lines_as(
                        "Roberto",
                        args![
                            "Yeah! I gather different items",
                            "and make arrows using them.",
                            "It is a useful skill to help me",
                            "survive alone in this tough world.",
                            "If you'd like, I can teach you."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.var("@menu")
                        .set(runtime::select_values(ctx, &[Val::from("That would be wonderful.")])?)?;
                    ctx.lines_as(
                        "Roberto",
                        args![
                            "But.. I can't do it for free.",
                            "Nothing is free in this world~",
                            "Mmm... How about this?",
                            "You bring me what I ask for.",
                            "Then I will teach you the skill."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Roberto",
                        args![
                            "I've been very lonely since I left my hometown.",
                            "I would like to treat my homesickness",
                            "with things from there.",
                            "Bring me 20 Resins from the trees in the ",
                            "Payon forest, and 1 Red Potion",
                            "sold in the store."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Roberto",
                        args![
                            "Also, 13 Trunks from the Willows that",
                            "live near the Payon Forest,",
                            "41 Pointed Scale,",
                            "7 Mushroom Spores.",
                            "If you bring me all of these,"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Roberto",
                        args!["I will teach you the skill.", "Then.. I'll be waiting.", "For news from our home."],
                    )?;
                    return ctx.close();
                }
                1 => {
                    let title = if ctx.var("Sex").get()? == constants::SEX_MALE {
                        "mister"
                    } else {
                        "miss"
                    };
                    ctx.lines_as(
                        "Roberto",
                        args![Val::from(format!("...hey {title}.")), "...be careful at night."],
                    )?;
                    return ctx.close();
                }
                _ => {}
            }
        }
        ctx.lines_as(
            "Roberto",
            args![
                "Hmm... Do you",
                "have something to say?",
                "I have nothing.",
                "Difference in levels",
                "cuts off conversations."
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as(
        "?",
        args![
            "Eh... First time seeing an archer or something?",
            "Just go where you were going.",
            "I only talk to high level archers.",
            "Won't open my mouth otherwise!"
        ],
    )?;
    ctx.close()
}

pub fn jason(ctx: &Ctx) -> Script {
    if ctx.var("BaseClass").get()? == constants::JOB_ARCHER {
        if ctx.call(Function::GetSkillLv, args!["AC_CHARGEARROW"])? == 1 {
            ctx.lines_as(
                "Jason",
                args![
                    "Eh, we meet again.",
                    "Ehhhh so weird.",
                    "Whenever I see someone again",
                    "I start eh-ing a lot.",
                    "Ehhh... anyways nice to see you again.",
                    "Ehhhh... don't come any more ehh..."
                ],
            )?;
            return ctx.close();
        }
        if ctx.player().job_level()? >= 35
            || ctx.var("BaseJob").get()? == constants::JOB_HUNTER
            || ctx.var("BaseJob").get()? == constants::JOB_BARD
            || ctx.var("BaseJob").get()? == constants::JOB_DANCER
        {
            ctx.lines_as(
                "Jason",
                args![
                    "Darn... my wound isn't healing.",
                    "Bleh.. I was too careless... ",
                    "to become like this.. err...",
                    "But still, hurting me like this",
                    "giving me so many injuries..."
                ],
            )?;
            ctx.next()?;
            ctx.lines(args![
                "What should I do about Roberto.",
                "Mmmm... Ah!",
                "You? How long have you been there?",
                "Mmm... very high level.",
                "Someone like you would definitely be",
                "able to know how to use Arrow Repel."
            ])?;
            ctx.next()?;
            match ctx.menu(&["What is that?", "Teach me."])? {
                0 => {
                    ctx.lines_as(
                        "Jason",
                        args![
                            "...you're kidding, right?",
                            "Oh my, you don't even know",
                            "Arrow Repel at that level?",
                            "You're a strange person."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Jason", args!["(Jason was in the lala land.)"])?;
                    ctx.next()?;
                    ctx.lines_as("Jason", args!["Well, ok. I'll teach you what", "Arrow Repel is."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Jason",
                        args![
                            "Arrow Repel is a skill that allows you to",
                            "push the opponent away as soon as you attack.",
                            "You can only use it when you aim exactly",
                            "at the target. But unlike magic, ",
                            "it doesn't de-spell."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Jason",
                        args![
                            "It is very useful for an archer",
                            "that is weak in close ranges.",
                            "If you would like to learn,",
                            "come find me again.",
                            "There are some necessary materials."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Jason",
                        args![
                            "First, because you must modify a bow",
                            "bring a crossbow you do not use.",
                            "10 Tentacles, 10 Bill of Birds,",
                            "3 Yoyo Tails.. these are very elastic.",
                            "Also, 2 Emeralds. And last but not least...",
                            "36 bottles of Banana Juice that I love!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Jason",
                        args![
                            "......Ehem!",
                            "If you bring all of these,",
                            "I shall teach you Arrow Repel.",
                            "Then, see you again.",
                            "(I'm going to be mad if you don't bring the Banana Juice.)"
                        ],
                    )?;
                    return ctx.close();
                }
                1 => {
                    if ctx.items().count(721)? > 1
                        && ctx.items().count(942)? > 2
                        && ctx.items().count(962)? > 9
                        && ctx.items().count(925)? > 9
                        && ctx.items().count(532)? > 35
                    {
                        ctx.lines_as(
                            "Jason",
                            args!["Ok! Perfect!", "I shall teach you the nationally", "renowned skill, Arrow Repel!"],
                        )?;
                        ctx.next()?;
                        ctx.items().take(721, 2)?;
                        ctx.items().take(942, 3)?;
                        ctx.items().take(962, 10)?;
                        ctx.items().take(925, 10)?;
                        ctx.items().take(532, 36)?;
                        ctx.call(Function::Skill, args!["AC_CHARGEARROW", 1, constants::SKILL_PERM])?;
                        ctx.lines_as(
                            "Jason",
                            args![
                                "Oh, works better than I expected!",
                                "Won't be needing to modify the bow!",
                                "You can take this back~",
                                "And enjoy using your newly inherited",
                                "skill in fields and dungeons!",
                                "He~heh~!"
                            ],
                        )?;
                        return ctx.close();
                    }
                    ctx.lines_as(
                        "Jason",
                        args![
                            "Mmm... too bad.",
                            "You are missing some things.",
                            "Once again, you need 2 Emeralds,",
                            "3 Yoyo Tails, 10 Tentacles,",
                            "10 Bill of Birds, and last but",
                            "not least 36 bottles of Banana juice!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Jason", args!["Make sure you have all of them and come again!"])?;
                    return ctx.close();
                }
                _ => {}
            }
        }
        ctx.lines_as(
            "Jason",
            args![
                "Ooh... you are an archer.",
                "If you try a little more",
                "you will have a great",
                "reputation as an archer!",
                "Exert yourself!"
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as("?", args!["What does life need from", "a lonely lad like me?"])?;
    ctx.close()
}

pub fn jason_ontouch(ctx: &Ctx) -> Script {
    ctx.lines_as("???", args!["Errrrrrr..."])?;
    ctx.close()
}

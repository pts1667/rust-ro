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

pub fn sister_linus(ctx: &Ctx) -> Script {
    if ctx.var("BaseJob").get()? == constants::JOB_PRIEST {
        if ctx.var("priest_sk").get()? == 100 {
            if ctx.call(Function::GetSkillLv, args!["PR_REDEMPTIO"])? == 0 {
                ctx.lines_as(
                    "Sister Linus",
                    args![
                        "Hm? We've met before,",
                        "haven't we? Then again,",
                        "all Priests begin to look",
                        "the same after a while. Ah,",
                        "were you one of the ones",
                        "to whom I taught Redemptio?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Sister Linus",
                    args![
                        "Ah, you've forgotten, have",
                        "you? Well, it's no problem for",
                        "me to teach you again. This skill enables you to revive your fallen",
                        "Party Members by sacrificing",
                        "your own life for them."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Sister Linus",
                    args![
                        "^3131FFOh holy and venerable one,",
                        "we pray to you. Please show",
                        "us your mercy and guide us",
                        "with your light. Give us the",
                        "strength to walk the path of",
                        "love and sacrifice. Redemptio!^000000"
                    ],
                )?;
                ctx.call(Function::Skill, args!["PR_REDEMPTIO", 1, constants::SKILL_PERM])?;
                ctx.next()?;
                ctx.lines_as(
                    "Sister Linus",
                    args![
                        "There...",
                        "You should be able",
                        "to perform Redemptio",
                        "now. I hope you use it",
                        "well on your adventures~"
                    ],
                )?;
                return ctx.close();
            } else {
                ctx.lines_as(
                    "Sister Linus",
                    args![
                        "I'll always be praying",
                        "for your safety. Be careful,",
                        "and I hope that you can bring",
                        "love and compassion to all",
                        "whom you meet in your travels."
                    ],
                )?;
                return ctx.close();
            }
        } else if ctx.var("priest_sk").get()? == 0 {
            ctx.var("redemp").set(1014)?;
            if ctx.call(Function::GetSkillLv, args!["PR_REDEMPTIO"])?.is_true() {
                ctx.lines_as(
                    "Sister Linus",
                    args![
                        "I'll always be praying",
                        "for your safety. Be careful,",
                        "and I hope that you can bring",
                        "love and compassion to all",
                        "whom you meet in your travels."
                    ],
                )?;
                ctx.var("priest_sk").set(100)?;
                return ctx.close();
            }
            ctx.lines_as(
                "Sister Linus",
                args![
                    "Please have mercy and",
                    "spread your light through",
                    "the world. Guide her with",
                    "your benevolent wisdom...",
                    "Bless her, and may she",
                    "be protected by your grace."
                ],
            )?;
            ctx.next()?;
            ctx.menu(&["Whom are you praying for, sister?"])?;
            // Only one option, so the 1-based answer stored in @menu is always 1.
            ctx.var("@menu").set(1)?;
            ctx.lines_as(
                "Sister Linus",
                args![
                    "Oh! You startled me!",
                    "Whom am I praying for?",
                    "Well, I once knew a young,",
                    "playful and merry nun who was",
                    "also a bit brazen. But I have",
                    "many joyful memories of her."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sister Linus",
                args![
                    "One day, a group of adventurers",
                    "came to Prontera Church in hopes of hiring a Priest to accompany",
                    "them to ^3131DDGlast Heim^000000. It must have been fate that she was the only",
                    "Priest that was available."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sister Linus",
                args![
                    "I remember that she was",
                    "so determined to join those",
                    "adventurers, and we had no",
                    "choice but to let her go.",
                    "Still, many of us believed",
                    "that it was too dangerous..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sister Linus",
                args![
                    "I hear that, at first, their",
                    "ragtag team was able to",
                    "successfully exterminate",
                    "a great number of monsters.",
                    "But supporting them as a Priest",
                    "must have been tough for her..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sister Linus",
                args![
                    "Then, tragedy struck.",
                    "Surrounded by Wanderers,",
                    "the adventurers quickly fell",
                    "in defeat, one by one. Soon,",
                    "the young nun was the only one of the group still standing."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sister Linus",
                args![
                    "Alone against impossible",
                    "odds, that poor girl had no",
                    "choice but to try a desperate",
                    "gamble. She began to chant",
                    "the ancient holy spell..."
                ],
            )?;
            ctx.next()?;
            ctx.menu(&["Ancient spell...?"])?;
            ctx.var("@menu").set(1)?;
            ctx.lines_as(
                "Sister Linus",
                args![
                    "Yes! ^FF0000Redemptio^000000!",
                    "A skill that only the",
                    "most talented Priests",
                    "can perform! And here she",
                    "was, a young nun with very",
                    "little experience, trying it!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sister Linus",
                args![
                    "Redemptio is the last",
                    "resort skill that can be",
                    "used to nobly save the",
                    "lives of others at the cost",
                    "of ^3131FFsacrificing your own life^000000."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sister Linus",
                args![
                    "Yes, I remember the days",
                    "she wasn't very interested",
                    "in studying the Priest and",
                    "Acolyte skills. Surprisingly,",
                    "she had learned enough to",
                    "attempt to cast Redemptio..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sister Linus",
                args![
                    "Miraculously, she succeeded",
                    "in reviving her party members.",
                    "They quickly used a Yggdrasil",
                    "Leaf to restore her life, and they defeated the Wanderers and",
                    "returned home safely."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sister Linus",
                args![
                    "Because of this success, she",
                    "was promoted to High Priest",
                    "despite the many mistakes she",
                    "made in the past. Now she travels the world, helping adventurers",
                    "in any way that she can."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sister Linus",
                args!["But, well, she'll", "always be that spunky", "and joyful girl to me."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.player().name()?,
                args![
                    "Wait, you still haven't",
                    "answered my question.",
                    "Are you praying for this",
                    "nun? And if so, is she",
                    "in some kind of danger?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sister Linus",
                args![
                    "Well, a few months ago,",
                    "she went on a mission to",
                    "^3131FFLighthalzen^000000, a city in the",
                    "Schwarzwald Republic. I was",
                    "just worried since I haven't",
                    "heard from her in a while."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sister Linus",
                args![
                    "I come here to pray for",
                    "her everyday, and hope that",
                    "she'll come back safely and",
                    "share stories of her adventures",
                    "with me. Ooh, we'll talk all night long! It'll be so much fun!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sister Linus",
                args![
                    "Goodness, I've been",
                    "running my mouth! I'm",
                    "sorry to keep you, did you",
                    "have somewhere to go? Well,",
                    "I'll be praying for your safety~"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.player().name()?,
                args![
                    "Before I go, would you",
                    "please tell me the name",
                    "of that High Priest you were",
                    "talking about? If I see her,",
                    "I'll ask her to contact you."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sister Linus",
                args![
                    "Oh, thank you! Her name",
                    "is High Priest Sorin. Um,",
                    "^3131FFMargaretha Sorin^000000. If you",
                    "happen to find her, please",
                    "let her know that I am",
                    "praying for her safety."
                ],
            )?;
            ctx.var("priest_sk").set(1)?;
            return ctx.close();
        } else if ctx.var("priest_sk").get()? == 1 {
            ctx.lines_as("Sister Linus", args!["Oh, hello~", "How are you", "doing today?"])?;
            ctx.next()?;
            if ctx.menu(&["Please teach me ^3131FFRedemptio^000000.", "Cancel"])? == 0 {
                ctx.lines_as(
                    "Sister Linus",
                    args![
                        "Ooh, Redemptio would be",
                        "a good skill for you to learn~",
                        "It's difficult to cast, but when you succeed, you can revive all",
                        "of your defeated Party Members."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Sister Linus",
                    args![
                        "Of course, the drawback",
                        "is that you must ^3131FFsacrifice",
                        "your own life^000000 in order to",
                        "use the skill. Now, to learn",
                        "Redemptio, you must first",
                        "learn ^3131FFLevel 1 Resurrection^000000."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Sister Linus",
                    args![
                        "Ah, you must",
                        "also bring me",
                        "^3131FF30 Holy Waters and",
                        "20 Blue Gemstones^000000",
                        "so you can attempt",
                        "to learn the skill."
                    ],
                )?;
                ctx.var("priest_sk").set(2)?;
                ctx.next()?;
                ctx.lines_as(
                    "Sister Linus",
                    args![
                        "Um, but if you fail to",
                        "learn Redemptio, you'll",
                        "lose 1 Holy Water and",
                        "1 Blue Gemstone and",
                        "you'll have to try it again..."
                    ],
                )?;
                return ctx.close();
            }
            ctx.lines_as(
                "Sister Linus",
                args![
                    "Praying gives me a",
                    "sense of peace and",
                    "comfort. Perhaps that",
                    "is one of the reasons",
                    "why I chose this job..."
                ],
            )?;
            return ctx.close();
        } else if ctx.var("priest_sk").get()? == 2 {
            if ctx.call(Function::GetSkillLv, args!["ALL_RESURRECTION"])?.number()? > 0
                && ctx.items().count(523)? > 29
                && ctx.items().count(717)? > 19
            {
                ctx.lines_as(
                    "Sister Linus",
                    args![
                        "Ooh, it seems like you've",
                        "brought everything that you",
                        "need to learn Redemptio.",
                        "Are you ready to try it? If you",
                        "fail, you'll lose 1 Holy Water",
                        "and 1 Blue Gemstone, okay?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Sister Linus",
                    args!["Now, please", "concentrate and", "repeat this special", "prayer after me."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Sister Linus",
                    args![
                        "^3131FFOh holy and venerable one,",
                        "we pray to you. Please show",
                        "us your mercy and guide us",
                        "with your light. Give us the",
                        "strength to walk the path of",
                        "love and sacrifice. Redemptio!^000000"
                    ],
                )?;
                ctx.next()?;
                let redem_s = ctx.player().job_level()? + ctx.call(Function::GetSkillLv, args!["ALL_RESURRECTION"])?.number()?;
                let roll = ctx.rand_range(1, 100)?;
                let success = if redem_s < 31 {
                    roll > 20 && roll < 41
                } else if redem_s < 41 {
                    roll > 10 && roll < 41
                } else {
                    roll > 10 && roll < 51
                };
                if success {
                    ctx.call(Function::SpecialEffect, args![constants::EF_HEALSP])?;
                    ctx.lines_as(
                        "Sister Linus",
                        args![
                            "Congratulations!",
                            "You successfully",
                            "learned Redemptio!",
                            "Please remember to only",
                            "use this skill in the most",
                            "critical situations."
                        ],
                    )?;
                    ctx.items().take(717, 20)?;
                    ctx.items().take(523, 30)?;
                    ctx.var("priest_sk").set(100)?;
                    ctx.call(Function::Skill, args!["PR_REDEMPTIO", 1, constants::SKILL_PERM])?;
                    return ctx.close();
                } else {
                    ctx.call(Function::SpecialEffect, args![constants::EF_POISONHIT])?;
                    ctx.lines_as(
                        "Sister Linus",
                        args![
                            "Oh no! I'm sorry,",
                            "but you failed to",
                            "learn Redemptio. Well,",
                            "I'll be waiting right here,",
                            "so we can try again when",
                            "you're ready, okay?"
                        ],
                    )?;
                    ctx.items().take(717, 1)?;
                    ctx.items().take(523, 1)?;
                    return ctx.close();
                }
            } else {
                ctx.lines_as(
                    "Sister Linus",
                    args![
                        "If you want to try to learn",
                        "Redemptio, please bring",
                        "^3131FF20 Blue Gemstones^000000 and",
                        "^3131FF30 Holy Waters^000000. Ah, and",
                        "you need to learn ^3131FFLevel 1",
                        "Resurrection^000000 beforehand."
                    ],
                )?;
                return ctx.close();
            }
        }
    } else {
        ctx.lines_as(
            "Sister Linus",
            args![
                "Please have mercy and",
                "spread your light through",
                "the world. Guide her with",
                "your benevolent wisdom...",
                "Bless her, and may she",
                "be protected by your grace."
            ],
        )?;
        return ctx.close();
    }
    Ok(())
}

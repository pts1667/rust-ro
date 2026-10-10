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

pub fn master_miller(ctx: &Ctx) -> Script {
    if ctx.var("Upper").get()? == 2 {
        ctx.lines_as(
            "Master Miller",
            args![
                "Well, aren't you an",
                "adorable little child~",
                "Where's your mommy?",
                "This place is dangerous, so",
                "please go home soon, okay?"
            ],
        )?;
        return ctx.close();
    }
    if ctx.var("Class").get()? == constants::JOB_NOVICE {
        if !shared::other_global_functions::f_canchangejob(ctx, vec![])?.is_true() {
            ctx.lines_as(
                "Master Miller",
                args![
                    "Interested in becoming",
                    "a Gunslinger, eh? You've",
                    "got potential, but you're",
                    "not yet experienced enough.",
                    "Just train yourself a bit more,",
                    "and then come back, you hear?"
                ],
            )?;
            return ctx.close();
        }
        if ctx.var("guns_q").get()? == 0 {
            ctx.lines_as(
                "Master Miller",
                args![
                    "I'm Miller, a full time",
                    "Gunslinger drillmaster, and",
                    "full time guardian for Lady",
                    "Selena. Now, what do you",
                    "need? If it's not important, then I can't make the time for you."
                ],
            )?;
            ctx.next()?;
            if ctx.menu(&["Nothing.", "I want to become a Gunslinger."])? == 0 {
                ctx.lines_as(
                    "Master Miller",
                    args![
                        "Don't waste my time.",
                        "If you do want to become",
                        "a Gunslinger, then come",
                        "back and talk to me."
                    ],
                )?;
                return ctx.close();
            }
            ctx.lines_as(
                "Master Miller",
                args![
                    "Hm. You're pretty young, but",
                    "your eyes tell me that you're",
                    "pretty ambitious. You'll need to pass our interview and educational",
                    "course to become a Gunslinger. Do you want to apply for the job?"
                ],
            )?;
            ctx.next()?;
            if ctx.menu(&["Give me some time to think.", "Sure!"])? == 0 {
                ctx.lines_as(
                    "Master Miller",
                    args![
                        "Understandable.",
                        "If you do decide that",
                        "you want to become",
                        "a Gunslinger, then let",
                        "me know right away.",
                        "I'll get you started."
                    ],
                )?;
                return ctx.close();
            }
            ctx.lines_as(
                "Master Miller",
                args![
                    "Great, great. Alright then,",
                    "let's get you started. Take",
                    "this letter to Mr. Wise Bull",
                    "Horn in Payon. He's a shaman",
                    "that will judge whether or not",
                    "you qualify to be a Gunslinger."
                ],
            )?;
            ctx.var("guns_q").set(1)?;
            ctx.quests().start(6020)?;
            return ctx.close();
        } else if ctx.var("guns_q").get()? == 1 {
            ctx.lines_as(
                "Master Miller",
                args![
                    "Take that letter of",
                    "introduction I've written",
                    "for you to Mr. Wise Bull",
                    "Horn in Payon. He'll test",
                    "you to see if you're really",
                    "Gunslinger material."
                ],
            )?;
            return ctx.close();
        } else if ctx.var("guns_q").get()? == 2 {
            ctx.lines_as(
                "Master Miller",
                args![
                    "Hmm... Wise Bull Horn",
                    "asked you to collect the",
                    "items you need to make the",
                    "voucher? Hm. I guess that's",
                    "part of his qualification test."
                ],
            )?;
            return ctx.close();
        } else if ctx.var("guns_q").get()? == 3 {
            ctx.lines_as(
                "Master Miller",
                args![
                    "Wise Bull Horn asked",
                    "you to bring him some",
                    "Milk? He must really like",
                    "you if he's already asking",
                    "for favors. Good luck, friend."
                ],
            )?;
            return ctx.close();
        } else if ctx.var("guns_q").get()? == 4 {
            ctx.lines_as(
                "Master Miller",
                args![
                    "I expect to hear good",
                    "news from you soon. You",
                    "know, I have no doubt that",
                    "you'll become a Gunslinger."
                ],
            )?;
            return ctx.close();
        } else if ctx.var("guns_q").get()? == 5 {
            if ctx.var("SkillPoint").get()? != 0 {
                ctx.lines_as(
                    "Master Miller",
                    args![
                        "Hey, you have leftover",
                        "Skill Points. You better",
                        "use them all up before you",
                        "come and talk to me again."
                    ],
                )?;
                return ctx.close();
            }
            ctx.lines_as(
                "Master Miller",
                args![
                    "Oh, you've brought a",
                    "voucher from Wise Bull Horn?",
                    "It's been a while since he's",
                    "given one to anybody, so",
                    "I'm really proud of you!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Master Miller",
                args![
                    "If Wise Bull Horn approves,",
                    "then I have no reason to",
                    "reject you. Alright then, I'll",
                    "promote you to a Gunslinger.",
                    "But first, let me explain",
                    "our job in more detail."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Master Miller",
                args![
                    "As a Gunslinger, you must",
                    "keep your gun with you at",
                    "all times. The Gunslinger",
                    "Guild keeps track of every Gun",
                    "and Bullet, so you can only get",
                    "them from our guild members."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Master Miller",
                args![
                    "Don't worry, Gunslinger",
                    "Guildsmen can be found almost",
                    "anywhere these days. Anyway,",
                    "it has to be this way by order of our guild leader, Lady Selena."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Master Miller",
                args![
                    "You might get the chance to",
                    "meet her one of these days.",
                    "Anyway, just now that we have",
                    "to regulate Gun and Bullet sales to keep them away from evil",
                    "or irresponsible folk."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Master Miller",
                args![
                    "In any case, it's always",
                    "a pleasure for me to talk",
                    "to another Gunslinger, so",
                    "let's keep in touch. May the",
                    "power of the earth protect",
                    "you in all of your adventures~"
                ],
            )?;
            shared::other_global_functions::job_change(ctx, args![constants::JOB_GUNSLINGER])?;
            ctx.var("guns_q").set(6)?;
            ctx.quests().complete(6024)?;
            if ctx.call(Function::Rand, args![2])?.is_true() {
                ctx.items().give(13100, 1)?;
            } else {
                ctx.items().give(13150, 1)?;
            }
            return ctx.close();
        }
    } else if ctx.var("Class").get()? == constants::JOB_GUNSLINGER {
        ctx.lines_as(
            "Master Miller",
            args![
                "Oh! Long time, no see,",
                "friend. How have you been?",
                "I hope you've been keeping",
                "you Gun well maintained.",
                "Take care of it, and it'll take",
                "care of you. Remember it."
            ],
        )?;
        return ctx.close();
    } else {
        ctx.lines_as(
            "Master Miller",
            args!["If you don't have", "any business with me,", "then please go on your way."],
        )?;
        return ctx.close();
    }
    Ok(())
}

fn voucher_requirements(ctx: &Ctx) -> Result<(), Stop> {
    ctx.lines_as(
        "Wise Bull Horn",
        args![
            "I can make a voucher that",
            "will demonstrate your desire",
            "to become a warrior of the",
            "earth for you to present to",
            "Gunslinger drillmasters.",
            "I shall need these items..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Wise Bull Horn",
        args![
            "^3355FF1 Trunk^000000,",
            "^3355FF3 Fluffs^000000,",
            "^3355FF3 Zargons^000000,",
            "^3355FF10 Shells^000000,",
            "^3355FF3 Green Herbs^000000, and",
            "^3355FF3 Rainbow Shells^000000."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Wise Bull Horn",
        args![
            "After I complete the",
            "voucher, you may bring",
            "it to Black Fox, and he",
            "will help you achieve",
            "your goal of becoming",
            "a Gunslinger."
        ],
    )
}

fn gunslinger_chant(ctx: &Ctx) -> Result<(), Stop> {
    ctx.lines_as(
        "Wise Bull Horn",
        args![
            "Eeh~Yeah~Eeh~Hooom",
            "Eeh~Yeah~Eeh~Hooom",
            "Maaaaarrraaa Neeey~",
            "Yippee Yippee Yai Yocaiyay~"
        ],
    )
}

pub fn wise_bull_horn(ctx: &Ctx) -> Script {
    if ctx.var("guns_q").get()? == 1 {
        ctx.lines_as(
            "Wise Bull Horn",
            args!["Hello, young wolf.", "What business has", "brought you before me?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.player().name()?,
            args![
                "Mr. Miller sent me to",
                "deliver this letter to you.",
                "Actually, I'm interested in",
                "becoming a Gunslinger..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Wise Bull Horn",
            args![
                "Miller, you said?",
                "Hm, the Black Fox doesn't",
                "give introductions for anyone",
                "he doesn't believe will make",
                "a good Gunslinger. Yes, I think",
                "I know why he sent you to me."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Wise Bull Horn",
            args![
                "I can see it in your eyes:",
                "you've got a warm heart and a",
                "strong sense of responsibility.",
                "All you need is the blessing",
                "of the Earth to protect you",
                "as a Gunslinger."
            ],
        )?;
        ctx.next()?;
        voucher_requirements(ctx)?;
        ctx.var("guns_q").set(2)?;
        ctx.quests().change(6020, 6021)?;
        ctx.close()
    } else if ctx.var("guns_q").get()? == 2 {
        if ctx.items().count(912)? < 3
            || ctx.items().count(914)? < 3
            || ctx.items().count(1019)? < 1
            || ctx.items().count(935)? < 10
            || ctx.items().count(511)? < 3
            || ctx.items().count(1013)? < 3
        {
            voucher_requirements(ctx)?;
            return ctx.close();
        }
        ctx.items().take(912, 3)?;
        ctx.items().take(914, 3)?;
        ctx.items().take(1019, 1)?;
        ctx.items().take(935, 10)?;
        ctx.items().take(511, 3)?;
        ctx.items().take(1013, 3)?;
        ctx.var("guns_q").set(3)?;
        ctx.quests().change(6021, 6022)?;
        ctx.lines_as(
            "Wise Bull Horn",
            args![
                "Ah, you've returned",
                "with everything I need.",
                "Please give me some time",
                "to make the voucher. If you",
                "come back in a little while,",
                "I should be finished with it."
            ],
        )?;
        ctx.close()
    } else if ctx.var("guns_q").get()? == 3 {
        ctx.lines_as(
            "Wise Bull Horn",
            args![
                "Oh, you've arrived just",
                "in time. It's been a while",
                "since I've made one of these vouchers, so I might be a little",
                "rusty. Still, this really takes me back to the days of my youth."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Wise Bull Horn",
            args![
                "I've been serving in this",
                "position of choosing worthy",
                "recipients of Gunslinger",
                "vouchers for a few decades",
                "now. But before that, I was a",
                "young adventurer just like you."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Wise Bull Horn",
            args![
                "It feels like it was only",
                "yesterday when I held my own",
                "little voucher as a Gunslinger,",
                "a warrior of the earth. That's",
                "when I met Selena's father...",
                "How can time pass so quickly?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Wise Bull Horn",
            args![
                "Ah... I really appreciate",
                "Selena and Black Fox for all",
                "of their help in recruiting",
                "young Gunslingers. I'm very",
                "old now, and can't do everything by myself. *Sigh...* Such is life."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Wise Bull Horn",
            args![
                "Before you leave, may",
                "I ask you for a small favor?",
                "I'm thirsty, and would like",
                "a cold glass of Milk. Would",
                "you please bring me some?"
            ],
        )?;
        ctx.var("guns_q").set(4)?;
        ctx.quests().change(6022, 6023)?;
        ctx.close()
    } else if ctx.var("guns_q").get()? == 4 {
        if ctx.items().count(519)? < 1 {
            ctx.lines_as(
                "Wise Bull Horn",
                args![
                    "I'm an old man that will",
                    "soon be reunited with mother",
                    "earth. Would you do this old",
                    "Gunslinger a favor a bring me",
                    "a cold glass of Milk, please?"
                ],
            )?;
            return ctx.close();
        }
        ctx.items().take(519, 1)?;
        ctx.var("guns_q").set(5)?;
        ctx.quests().change(6023, 6024)?;
        ctx.lines_as(
            "Wise Bull Horn",
            args![
                "Oh, thank you for your!",
                "generosity--I see that",
                "you've brought me some",
                "Milk. Ahhhh, delicious~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Wise Bull Horn",
            args![
                "I admire the patience,",
                "gentleness, and kindness",
                "that you've proven by bringing",
                "this to me. Yes, those are traits we all want Gunslingers to have."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Wise Bull Horn",
            args![
                "Now, please take this voucher",
                "to Miller, the Black Fox, with",
                "my wholehearted approval.",
                "I hope that you will use your",
                "gun to uphold justice as a",
                "noble warrior of the earth."
            ],
        )?;
        ctx.next()?;
        gunslinger_chant(ctx)?;
        ctx.close()
    } else if ctx.var("guns_q").get()? == 5 {
        ctx.lines_as(
            "Wise Bull Horn",
            args![
                "Please take this voucher",
                "to Miller, the Black Fox, with",
                "my wholehearted approval.",
                "I hope that you will use your",
                "gun to uphold justice as a",
                "noble warrior of the earth."
            ],
        )?;
        ctx.next()?;
        gunslinger_chant(ctx)?;
        ctx.close()
    } else if ctx.var("guns_q").get()? == 6 {
        ctx.lines_as(
            "Wise Bull Horn",
            args![
                "AAh, long time no see.",
                "I hope that you become",
                "a smart beast, and use",
                "your powers as a Gunslinger to protect what is good and just."
            ],
        )?;
        ctx.close()
    } else {
        ctx.lines_as("Wise Bull Horn", args!["Zzzzzz~", "^333333*Phew*^000000"])?;
        ctx.close()
    }
}

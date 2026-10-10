use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn treasure_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input_s = Val::from("");
    let mut l_partymembercount = Val::from(0);
    if ctx.call(Function::CheckWeight, vec![Val::from(907), Val::from(300)])? == 0 {
        ctx.lines(args![
            "^3355FFYou're carrying too many",
            "items: there's no way you",
            "can carry the Treasure",
            "Chest with you for now.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("treasure_nd").get()? == 5 {
        ctx.lines(args![
            "^3355FFThere's something",
            "here, buried just",
            "beneath the ground.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Unngh! I-it won't...!"],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFNo matter how hard",
            "you try, you can't dig",
            "out the chest or open it.",
            "There's some magical",
            "aura surrounding the chest",
            "that might be stopping you.^000000"
        ])?;
        ctx.var("treasure_nd").set(Val::from(6))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("treasure_nd").get()? == 6 {
            ctx.lines(args![
                "^3355FFThat Treasure Hunter in",
                "Morocc must know something",
                "about the chest's magical",
                "protection. It might be",
                "a good idea to ask him.^000000"
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if (ctx.var("treasure_nd").get()? == 7 || ctx.var("treasure_nd").get()? == 8) {
            ctx.lines(args![
                "^3355FFYou tried casting the",
                "spell on the document",
                "you received, but nothing",
                "happened. You should probably",
                "find the other man over in",
                "Comodo to see what he knows.^000000"
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if (ctx.var("treasure_nd").get()? == 9 || ctx.var("treasure_nd").get()? == 10) {
            ctx.lines(args![
                "^3355FFYou'll need to combine",
                "the two halves of the spell",
                "on the document you received",
                "to remove the chest's magical",
                "protection to get the treasure.^000000"
            ])?;
            ctx.next()?;
            let (input, status) = runtime::input_text(ctx, None, None)?;
            l_input_s = input;
            runtime::party_members(ctx, ctx.call(Function::GetCharacterId, vec![Val::from(1)])?, Val::from(0))?;
            l_partymembercount = ctx.var("$@partymembercount").get()?;
            if l_partymembercount.clone().number()? > 1 {
                if ((l_input_s.clone() == "OpenSesame" && ctx.var("treasure_nd").get()? == 9)
                    || (l_input_s.clone() == "UnlockTreasure" && ctx.var("treasure_nd").get()? == 10))
                {
                    ctx.lines(args![
                        "^3355FFThe Z Gang must have split",
                        "the spell document in two",
                        "parts because one person",
                        "isn't enough to open this",
                        "chest. It's a good thing",
                        "you brought a friend.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFYou cast the spell,",
                        "the chest opens, and",
                        "you grab all the loot inside.^000000"
                    ])?;
                    if ctx.var("zdan_edq").get()? == 0 {
                        ctx.var("treasure_nd").set(Val::from(11))?;
                    } else {
                        ctx.var("treasure_nd").set(Val::from(12))?;
                    }
                    ctx.call(Function::GetItem, vec![Val::from(7725), Val::from(1)])?;
                    ctx.call(Function::GetItem, vec![Val::from(604), Val::from(1)])?;
                    ctx.call(Function::GetItem, vec![Val::from(1157), Val::from(1)])?;
                    ctx.call(
                        Function::StartStatus,
                        vec![ctx.constant("SC_CURSE")?, Val::from(10000), Val::from(0), Val::from(10000)],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["N-no! What's going on?", "This Emerald... It must", "be cursed by evil magic!"],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines(args![
                        "^3355FFNothing happened.",
                        "You might not be",
                        "combining the spell",
                        "correctly. Perhaps if",
                        "you made it all one word...^000000"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            } else {
                ctx.lines(args![
                    "^3355FFThe treasure chest still",
                    "won't open. Then again,",
                    "this doesn't seem to be",
                    "a one man job...^000000"
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else if ctx.var("treasure_nd").get()? == 11 && ctx.var("zdan_edq").get()?.number()? > 0 {
            ctx.lines(args![
                "^3355FFSomone must have",
                "checked the treasure",
                "site after you left:",
                "whoever it was left",
                "telltale signs of",
                "his presence.^000000"
            ])?;
            ctx.var("treasure_nd").set(Val::from(12))?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    Ok(Val::from(0))
}

pub fn treasure(ctx: &Ctx) -> Script {
    treasure_body(ctx, Vec::new()).map(|_| ())
}

fn scholar_zgang_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("jewel_nd").get()?.number()? < 9 {
        ctx.lines_as(
            "Scholar",
            args![
                "I'm an antique appraiser.",
                "You may think of me as similar",
                "to jewel appraisers, but I can",
                "appraise the historical value",
                "of antiques in addition to their quality and monetary value."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("jewel_nd").get()? == 9 {
            ctx.lines_as(
                "Scholar",
                args![
                    "Oooh. You must be the",
                    "one sent by Ibrahim, right?",
                    "You're the one that found",
                    "that rare emerald, one of",
                    "the accursed jewels?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Scholar",
                args![
                    "I'm very interested in the",
                    "Cursed Jewels, and have been",
                    "researching them. You came",
                    "to just the right person if you",
                    "wanted to learn more about",
                    "the emerald that you found."
                ],
            )?;
            ctx.var("jewel_nd").set(Val::from(11))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("jewel_nd").get()? == 10 {
            ctx.lines_as("Scholar", args!["Hello, how may", "I help you today?"])?;
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("Ask About Cursed Jewel")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["I came to ask you", "about a Cursed Jewel."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Scholar",
                args!["Let's see... Ah, did", "you come to ask about", "the Diamond of Destruction?"],
            )?;
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("No, it's the Unlucky Emerald.")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as(
                "Scholar",
                args![
                    "Ah, the Unlucky Emerald?",
                    "Yes, that was the most recent",
                    "Cursed Jewel to be discovered.",
                    "Let's see, where did I put all",
                    "of my notes about that?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Scholar",
                args!["Would you mind coming", "back later? I'm not sure", "where I misplaced my notes..."],
            )?;
            ctx.var("jewel_nd").set(Val::from(11))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("jewel_nd").get()? == 11 {
            ctx.lines_as(
                "Scholar",
                args![
                    "Ah, let's see now.",
                    "There's a record here",
                    "entitled, ''The Story of the",
                    "Cursed Jewel.'' According",
                    "to this, no one knows where",
                    "these jewels were first found."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Scholar",
                args![
                    "The first incident involving",
                    "the jewel involved a Comodo",
                    "resident that used the jewel",
                    "to pay a gambling debt. He",
                    "committed suicide after he",
                    "paid his debt, which is a pity."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Scholar",
                args![
                    "He didn't tell anyone",
                    "where he found the jewel",
                    "before he died, so we have",
                    "no way of determining the",
                    "jewel's true origins."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Scholar",
                args![
                    "After it was sold, the",
                    "jewel came into a rich man's",
                    "possession. However, he lost",
                    "his fortune and was completely",
                    "ruined after obtaining it. Hence the moniker, ''Unlucky Emerald.''"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Scholar",
                args![
                    "Then the emerald came into",
                    "the hands of a noble lady",
                    "who mysteriously died at",
                    "a young age. The emerald's",
                    "had thousands of owners",
                    "who just... Died."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Scholar",
                args![
                    "What's disturbing is that",
                    "the emerald's curse seems",
                    "to grow stronger with each",
                    "life it takes away. Um, and",
                    "your jewel perfectly matches",
                    "the descriptions, so, uh..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Scholar",
                args![
                    "You are the new owner",
                    "of the Unlucky Emerald.",
                    "No one will buy that from",
                    "you while it's still cursed.",
                    "For the sake of your life,",
                    "you must break that curse."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Scholar",
                args![
                    "Now, holy spells won't work",
                    "on this kind of curse. You'll",
                    "need to talk to a shaman.",
                    "I think he lives in... Alberta?",
                    "He can summon the dead",
                    "and act as a medium for you."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Scholar",
                args![
                    "The idea is for you to talk",
                    "to the spirits killed by this",
                    "emerald through that shaman.",
                    "And I'm sure he'll be pretty",
                    "excited to see that you have",
                    "the Unlucky Emerald with you."
                ],
            )?;
            ctx.var("jewel_nd").set(Val::from(12))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("jewel_nd").get()? == 12 {
            ctx.lines_as(
                "Scholar",
                args![
                    "Please find that shaman",
                    "in Alberta, and ask if he can",
                    "help you break the curse",
                    "on that Unlucky Emerald."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("jewel_nd").get()?.number()? > 28 {
            ctx.lines_as(
                "Scholar",
                args![
                    "Wait, you're telling me",
                    "that the emerald was cursed",
                    "by the vindictive spirit of",
                    "some warrior? Interesting...",
                    "I should record this in my",
                    "research notes. Thank you!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Scholar",
                args![
                    "Some jewels are said to",
                    "be possessed by ghosts",
                    "or evil spirits that will bring",
                    "misfortune to their owners,",
                    "drive them to death or",
                    "insanity, that sort of thing."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
}

pub fn scholar_zgang(ctx: &Ctx) -> Script {
    scholar_zgang_body(ctx, Vec::new()).map(|_| ())
}

fn shaman_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("jewel_nd").get()? == 0 {
        ctx.lines_as(
            "Shaman",
            args![
                "The spiritual realm is",
                "the source of my sorcery.",
                "Do not understimate the",
                "power of the spirits!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if (ctx.var("jewel_nd").get()?.number()? > 0 && ctx.var("jewel_nd").get()?.number()? < 12) {
            ctx.lines_as(
                "Shaman",
                args!["Oh? Heh heh!", "You're carrying", "something very", "valuable, aren't you?"],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("jewel_nd").get()? == 12 {
                ctx.lines_as(
                    "Shaman",
                    args![
                        "You! Yes... I can sense",
                        "them... The angry spirits of",
                        "your ancestors! You haven't",
                        "made any offerings to them,",
                        "have you? You must appease",
                        "them before they torment you!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Shaman",
                    args![
                        "Quickly, now! It is",
                        "imperative that you give me",
                        "500,000 zeny immediately",
                        "so that I can comfort their",
                        "souls before they wreck",
                        "havoc on your life!"
                    ],
                )?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("Actually, I have a Cursed Jewel.")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as(
                    "Shaman",
                    args![
                        "Jewel...? Oh, a Cursed",
                        "Jewel! Hahaha! So that's",
                        "what that evil aura was!",
                        "My mistake! Say, do you",
                        "mind if I take a look at it?"
                    ],
                )?;
                ctx.next()?;
                if ctx.call(Function::CountItem, vec![Val::from(7725)])?.number()? > 0 {
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
                    ctx.lines_as(
                        "Shaman",
                        args![
                            "So this bauble's the",
                            "Unlucky Emerald? Yes,",
                            "it's definitely cursed.",
                            "If you don't break the",
                            "curse soon, it'll consume",
                            "your soul. Not a good thing."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Shaman",
                        args![
                            "I need to perform",
                            "a ritual to break the",
                            "curse. I'll charge you",
                            "200,000 zeny for the service.",
                            "It's expensive, but I'm saving",
                            "your life and risking mine."
                        ],
                    )?;
                    ctx.var("jewel_nd").set(Val::from(13))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Shaman",
                        args![
                            "Hm? Did you forget to",
                            "bring the jewel? Hurry,",
                            "and bring it back to me.",
                            "You need to break that",
                            "curse before the jewel",
                            "can devour your soul!"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            } else {
                if ctx.var("jewel_nd").get()? == 13 {
                    ctx.lines_as(
                        "Shaman",
                        args![
                            "Did you bring the money?",
                            "We must appease the spirits",
                            "possessing the soul, and to",
                            "do that, I will need to perform",
                            "a ritual that will be taxing",
                            "on my body and spirit."
                        ],
                    )?;
                    ctx.next()?;
                    if ctx.var("Zeny").get()?.number()? > 199999 {
                        ctx.lines_as(
                            "Shaman",
                            args![
                                "Good, good. I see that",
                                "you've brought the money.",
                                "I can now begin the ritual.",
                                "If all goes well, you'll be",
                                "free of your curse, and I can",
                                "buy some Prontera real estate."
                            ],
                        )?;
                        ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(200000))?))?;
                        ctx.var("jewel_nd").set(Val::from(14))?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Shaman",
                            args!["Please come back soon", "after I've completed the", "preparations for the ritual."],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Shaman",
                            args![
                                "You... You don't have",
                                "200,000 zeny? I'm sorry,",
                                "but I cannot perform the",
                                "ritual for free, even if",
                                "your soul is at stake."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                } else {
                    if ctx.var("jewel_nd").get()? == 14 {
                        ctx.lines_as(
                            "Shaman",
                            args![
                                "Hmm... You know the",
                                "money you gave me?",
                                "Apparently, it wasn't",
                                "enough. One of the spirits",
                                "was about to enter me, but..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Shaman",
                            args![
                                "It left in disgust after",
                                "it learned that I was trying",
                                "to break the curse for only",
                                "200,000 zeny. I think...",
                                "I think I need 100,000 zeny",
                                "more for this to be effective."
                            ],
                        )?;
                        ctx.var("jewel_nd").set(Val::from(15))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if ctx.var("jewel_nd").get()? == 15 {
                            ctx.lines_as(
                                "Shaman",
                                args![
                                    "Good, you've returned!",
                                    "Did you bring the 100,000",
                                    "zeny like I asked? It's a small",
                                    "price considering that I'm",
                                    "saving your very soul."
                                ],
                            )?;
                            ctx.next()?;
                            if ctx.var("Zeny").get()?.number()? > 99999 {
                                ctx.lines_as(
                                    "Shaman",
                                    args!["Ah, perfect!", "Hopefully, the", "spirits will be more", "cooperative this time."],
                                )?;
                                ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(100000))?))?;
                                ctx.var("jewel_nd").set(Val::from(16))?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Shaman",
                                    args![
                                        "Yes, the vindictive",
                                        "spirit in this jewel",
                                        "will be most impressed",
                                        "by your sincere efforts."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                ctx.lines_as(
                                    "Shaman",
                                    args![
                                        "You didn't? Do you know",
                                        "what it's like to have",
                                        "your soul consumed? Let",
                                        "me tell you, it's a lot",
                                        "more painful than parting",
                                        "with a paltry 100,000 zeny!"
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        } else {
                            if ctx.var("jewel_nd").get()? == 16 {
                                ctx.lines_as(
                                    "Shaman",
                                    args![
                                        "At long last, the",
                                        "spirits and I are one.",
                                        "The ritual may now begin.",
                                        "Ohhhhhhhhhhhmmmmm... "
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Shaman", args!["Hhhkkk... Arrrgh...!"])?;
                                ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
                                ctx.next()?;
                                ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args![".....?"])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Shaman",
                                    args![
                                        "Th-this emerald!",
                                        "So many lives! Money",
                                        "alone won't break this",
                                        "curse! It's time to get",
                                        "serious and suppress",
                                        "this jewel's power!"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Shaman",
                                    args![
                                        "Remember the items",
                                        "I tell you to bring, and",
                                        "come back with them",
                                        "as quickly as you can!",
                                        "Both of our lives are in",
                                        "danger until you do!"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Shaman",
                                    args![
                                        "Bring me",
                                        "^FF00001 Holy Water^000000,",
                                        "^FF00001 Red Blood^000000,",
                                        "^FF00001 Witherless Rose^000000,",
                                        "^FF00001 Crystal Blue^000000, and",
                                        "^FF00001 Wind of Verdure^000000."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Shaman",
                                    args![
                                        "Hurry! Although the spirits",
                                        "are eternal, they don't like",
                                        "to be kept waiting! Don't",
                                        "question this great irony,",
                                        "and get those items now!"
                                    ],
                                )?;
                                ctx.var("jewel_nd").set(Val::from(17))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                if ctx.var("jewel_nd").get()? == 17 {
                                    ctx.lines_as(
                                        "Shaman",
                                        args![
                                            "You brought the items?!",
                                            "Quickly! Place them in",
                                            "my hands! We can't afford",
                                            "to waste any more time!"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    if ((((ctx.call(Function::CountItem, vec![Val::from(990)])?.number()? > 0
                                        && ctx.call(Function::CountItem, vec![Val::from(991)])?.number()? > 0)
                                        && ctx.call(Function::CountItem, vec![Val::from(992)])?.number()? > 0)
                                        && ctx.call(Function::CountItem, vec![Val::from(748)])?.number()? > 0)
                                        && ctx.call(Function::CountItem, vec![Val::from(523)])?.number()? > 0)
                                    {
                                        ctx.lines_as(
                                            "Shaman",
                                            args![
                                                "Thank goodness.",
                                                "Now... Let me begin...",
                                                "Uhhkk... Argh! Wait!",
                                                "I... I see someone...",
                                                "Covered in blood? Yes...",
                                                "He's staring at something."
                                            ],
                                        )?;
                                        ctx.call(Function::DelItem, vec![Val::from(990), Val::from(1)])?;
                                        ctx.call(Function::DelItem, vec![Val::from(991), Val::from(1)])?;
                                        ctx.call(Function::DelItem, vec![Val::from(992), Val::from(1)])?;
                                        ctx.call(Function::DelItem, vec![Val::from(748), Val::from(1)])?;
                                        ctx.call(Function::DelItem, vec![Val::from(523), Val::from(1)])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Shaman",
                                            args![
                                                "There's a sword behind it.",
                                                "He's... He's just been in",
                                                "a fight? I can smell...",
                                                "His bloodlust. I can't tell",
                                                "if he cursed the jewel of",
                                                "if he's another victim..."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Shaman",
                                            args![
                                                "Damn it! It's not clear",
                                                "to me! We need to figure",
                                                "out if what I'm seeing is",
                                                "the root of the curse...",
                                                "What can we do to--Right!"
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Shaman",
                                            args![
                                                "I'm not sure how helpful",
                                                "he'll be, but there is an",
                                                "archaeologist in Juno that",
                                                "might know more about this",
                                                "specific jewel. Hopefully,",
                                                "he will have the answer."
                                            ],
                                        )?;
                                        ctx.var("jewel_nd").set(Val::from(18))?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Shaman",
                                            args![
                                                "For now, the jewel's",
                                                "curse is beyond my power.",
                                                "However, if you can determine",
                                                "the source of the curse, then",
                                                "I will be able to help you."
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else {
                                        ctx.lines_as(
                                            "Shaman",
                                            args!["You... Forgot?!", "Your body and soul are in", "jeopardy and you forgot?!"],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Shaman",
                                            args![
                                                "Bring me",
                                                "^FF00001 Holy Water^000000,",
                                                "^FF00001 Red Blood^000000,",
                                                "^FF00001 Witherless Rose^000000,",
                                                "^FF00001 Crystal Blue^000000, and",
                                                "^FF00001 Wind of Verdure^000000!"
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                } else if ctx.var("jewel_nd").get()? == 18 {
                                    ctx.lines_as(
                                        "Shaman",
                                        args![
                                            "That antique appraiser...",
                                            "He wouldn't know anything",
                                            "more about the jewel, but",
                                            "that archaeologist in Juno",
                                            "will hopefully have some",
                                            "new information we can use."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else if ctx.var("jewel_nd").get()? == 19 {
                                    ctx.lines_as(
                                        "Shaman",
                                        args!["Ah, you're back.", "So did you learn", "anything new about", "the Unlucky Emerald?"],
                                    )?;
                                    ctx.next()?;
                                    let choice = runtime::select_values(ctx, &[Val::from("The archaeologist said...")])?;
                                    ctx.var("@menu").set(choice)?;
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args!["The archaeologist said...", "Well, he said that there", "is no curse on the jewel."],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Shaman",
                                        args![
                                            "What? How dare he refute",
                                            "what I just told you?!",
                                            "Is he calling me a liar?",
                                            "There was a vindictive",
                                            "warrior's soul inside",
                                            "that very emerald!"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Shaman",
                                        args![
                                            "Then again, there's no",
                                            "point fighting someone",
                                            "that doesn't believe in",
                                            "the spirit realm. However,",
                                            "I wasn't lying. My honor",
                                            "as a shaman is at stake!"
                                        ],
                                    )?;
                                    ctx.var("jewel_nd").set(Val::from(20))?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Shaman",
                                        args![
                                            "In fact, I'll prove it to you",
                                            "by reading the message",
                                            "inside this emerald. Bring",
                                            "me the jewel, and I'll summon",
                                            "the warrior spirit inside it.",
                                            "Just let me rest a bit first."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else if ctx.var("jewel_nd").get()? == 20 {
                                    if ctx.call(Function::CountItem, vec![Val::from(7725)])?.number()? > 0 {
                                        ctx.lines_as(
                                            "Shaman",
                                            args![
                                                "Alright. By my pride",
                                                "as a shaman, I'll provide",
                                                "this service to you free",
                                                "of charge! Can't have",
                                                "anyone thinking I'm",
                                                "some kind of quack..."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("Shaman", args!["Hmmmm...", "Hmmmm...", "I... I see."])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Shaman",
                                            args![
                                                "Huh. Well, what do you",
                                                "know? There is no curse.",
                                                "However, it's not true that",
                                                "only Comodo's prodigal son",
                                                "was killed over this jewel.",
                                                "There are many more..."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Shaman",
                                            args![
                                                "Go yonder to the town",
                                                "of Geffen, head west, and",
                                                "seek the high ground that",
                                                "is over the bridge. You know...",
                                                "Leave Geffen through the west",
                                                "gate and find a high place."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Shaman",
                                            args![
                                                "Trust me. You'll find",
                                                "something there that will",
                                                "react to the power of that",
                                                "emerald you possess."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Shaman",
                                            args!["Hmpf! That arrogant", "archaeologist! He doesn't", "know what he's talking about!"],
                                        )?;
                                        ctx.var("jewel_nd").set(Val::from(21))?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else {
                                        ctx.lines_as(
                                            "Shaman",
                                            args!["You don't have the", "emerald with you?", "I hope you didn't", "lose it somewhere."],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                } else {
                                    if ctx.var("jewel_nd").get()? == 21 {
                                        ctx.lines_as(
                                            "Shaman",
                                            args![
                                                "Go yonder to the town",
                                                "of Geffen, head west, and",
                                                "seek the high ground that",
                                                "is over the bridge. You know...",
                                                "Leave Geffen through the west",
                                                "gate and find a high place."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Shaman",
                                            args![
                                                "Trust me. You'll find",
                                                "something there that will",
                                                "react to the power of that",
                                                "emerald you possess.",
                                                "Don't forget to bring",
                                                "that emerald with you!"
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if (ctx.var("jewel_nd").get()? == 22 || ctx.var("jewel_nd").get()? == 23) {
                                        ctx.lines_as(
                                            "Shaman",
                                            args![
                                                "What did I tell you?",
                                                "That jewel brings very",
                                                "bad fortune. Though, I'm",
                                                "not sure why the warrior",
                                                "spirit inside wouldn't talk",
                                                "to me. What's with that?"
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else {
                                        ctx.lines_as(
                                            "Shaman",
                                            args![
                                                "It's been a long time",
                                                "since I've done these",
                                                "rituals. I feel thoroughly",
                                                "drained of my spiritual energy."
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn shaman(ctx: &Ctx) -> Script {
    shaman_body(ctx, Vec::new()).map(|_| ())
}

fn archeologist_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("jewel_nd").get()?.number()? < 18 {
        ctx.lines_as(
            "Archeologist",
            args![
                "There's so much to do!",
                "How am I going to handle",
                "all of this research? Don't",
                "panic, don't panic, just",
                "one thing at a time!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("jewel_nd").get()? == 18 {
            ctx.lines_as(
                "Archeologist",
                args![
                    "Oh, I've heard about you",
                    "from that antique appraiser",
                    "in Comodo... You're the one",
                    "that dig up the Unlucky",
                    "Emerald, right? Would",
                    "you mind if I take a look?"
                ],
            )?;
            ctx.next()?;
            if ctx.call(Function::CountItem, vec![Val::from(7725)])?.number()? > 0 {
                ctx.lines_as(
                    "Archeologist",
                    args![
                        "Ah, it's no wonder so",
                        "many people want it.",
                        "This emerald is so huge",
                        "and... It's beautiful! So...",
                        "I'm guessing you needed",
                        "my help with something?"
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Ask About the Jewel's Curse:Show Off Jewel")])? {
                    1 => {
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_SMILE")?])?;
                        ctx.lines_as(
                            "Archeologist",
                            args![
                                "The jewel's curse?",
                                "Hah! You believe in that",
                                "old superstition? Oh, so",
                                "you wanted to break the",
                                "jewel's curse? Mmpf. Hahah!",
                                "You're not dead yet, right?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Archeologist",
                            args![
                                "It's all a load of bunk!",
                                "Only one man died while",
                                "keeping this jewel in his",
                                "possession, that gambler in",
                                "Comodo that used it to pay",
                                "off his gambling debts."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Archeologist",
                            args![
                                "This jewel happened to be",
                                "lost for a long time, and it",
                                "just reappeared. That's all.",
                                "Lots of myths and legends",
                                "surround extremely valuable",
                                "gems like this one."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Archeologist",
                            args![
                                "Anyway, there's no curse.",
                                "Believe what you want to,",
                                "but that happens to be my",
                                "professional opinion,",
                                "based on my research."
                            ],
                        )?;
                        ctx.var("jewel_nd").set(Val::from(19))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as("Archeologist", args!["Wow...", "That jewel sure", "is freakin' huge..."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            } else {
                ctx.lines_as(
                    "Archeologist",
                    args![
                        "You lost the jewel?",
                        "Well, maybe it's not",
                        "all bad. History shows",
                        "that many people were",
                        "ruined by suddenly coming",
                        "upon vast fortunes, you know?"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else {
            if ctx.var("jewel_nd").get()? == 22 {
                ctx.lines_as(
                    "Archeologist",
                    args![
                        "The vindictive soul",
                        "of a warrior is just",
                        "drifting around in the",
                        "world of the living?",
                        "I don't think I want to",
                        "believe what you're saying."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Archeologist",
                    args![
                        "I personally feel that",
                        "old shaman's a fraud, but...",
                        "You should have the freedom",
                        "to decide what you believe.",
                        "But don't let it bother you."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("jewel_nd").get()? == 23 {
                ctx.lines_as(
                    "Archeologist",
                    args![
                        "Are you talking about",
                        "ghosts again? Fine. For",
                        "the sake of the argument,",
                        "let's say the jewel actually",
                        "possesses the soul of",
                        "some fallen warrior."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Archeologist",
                    args![
                        "It's not unfathomable for",
                        "people with huge debts to",
                        "kill themselves, or for",
                        "warriors to perish while",
                        "fighting Cobolds. Still...",
                        "I'll see what I can learn."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Archeologist",
                    args![
                        "That reminds me...",
                        "I've got some records about",
                        "an adventurer group that died",
                        "in the field west of Geffen",
                        "that you were talking about.",
                        "Let me see if I can find them."
                    ],
                )?;
                ctx.var("jewel_nd").set(Val::from(24))?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("jewel_nd").get()? == 24 {
                ctx.lines_as(
                    "Archeologist",
                    args![
                        "Ah, you're back! I found",
                        "those records I told you",
                        "about. It's a witness's",
                        "account of what happened",
                        "when those people died",
                        "west of Geffen."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Archeologist",
                    args![
                        "What's interesting is that",
                        "the warrior you keep talking",
                        "about wasn't killed by Cobolds",
                        "at all, according to what this",
                        "guy wrote. Let's see...",
                        "Ah, it's over here."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Archeologist",
                    args![
                        "''^666666When the chief and his",
                        "soldiers arrived, the battle",
                        "was over. Dead bodies lay",
                        "everywhere, but there was",
                        "one warrior covered in blood,",
                        "half conscious, but living.^000000''"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Archeologist",
                    args![
                        "Strange, isn't it?",
                        "This is from a journal",
                        "written by a retired soldier",
                        "that lives in Prontera now.",
                        "It might be a good idea to",
                        "see if you can talk to him."
                    ],
                )?;
                ctx.var("jewel_nd").set(Val::from(25))?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("jewel_nd").get()? == 25 {
                ctx.lines_as(
                    "Archeologist",
                    args![
                        "There's a retired soldier",
                        "that actually witnessed what",
                        "happened in the field west",
                        "of Geffen. You should try",
                        "to find him in Prontera, and",
                        "see what you can learn."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("jewel_nd").get()?.number()? > 28 {
                ctx.lines_as(
                    "Archeologist",
                    args![
                        "Ah, that's... That's",
                        "very profound. That one",
                        "warrior went berserk when",
                        "he realized he was too weak",
                        "to protect his comrades?",
                        "He lost total control..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Archeologist",
                    args![
                        "I better talk to that",
                        "old soldier and get the",
                        "full story on record.",
                        "It'll be a really good history",
                        "lesson for Swordmen everywhere.",
                        "Thank you for letting me know."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Archeologist",
                    args![
                        "I'm so busy! Listen,",
                        "would you mind coming",
                        "back later when I'm not",
                        "so swamped with work?"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
    Ok(Val::from(0))
}

pub fn archeologist(ctx: &Ctx) -> Script {
    archeologist_body(ctx, Vec::new()).map(|_| ())
}

fn old_soldier_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(907), Val::from(200)])? == 0 {
        ctx.lines_as(
            "Retired Soldier",
            args![
                "Why are you carrying",
                "so much stuff with you?",
                "Isn't there a Kafra Storage",
                "service for keeping your junk?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("jewel_nd").get()? == 25 {
        ctx.lines_as(
            "Retired Soldier",
            args![
                "It's strange that so",
                "many people have been",
                "wanting to talk to me for",
                "some reason, but I'll",
                "admit that it's kind of nice."
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Ask About Past Battles:Ask About Fallen Warrior")])? {
            1 => {
                ctx.lines_as(
                    "Retired Soldier",
                    args![
                        "Don't get me started.",
                        "Once I start talking about",
                        "the old days, I'll go on all",
                        "night. Those were real hard",
                        "times to live through, but good",
                        "memories to remember..."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Retired Soldier",
                    args![
                        "The fallen warrior...?",
                        "There's only one that brings",
                        "a chill down my spine whenever",
                        "I think of him. He was about",
                        "your age when it happened..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Retired Soldier",
                    args![
                        "We received a report about",
                        "a Cobold threat, so we were",
                        "dispatched to the fields",
                        "west of Geffen. But the",
                        "battle was over once we",
                        "arrived. We were too late."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Retired Soldier",
                    args![
                        "All the Cobolds were slain,",
                        "but... There was one warrior",
                        "left, covered in blood. When",
                        "he saw us, he just started",
                        "attacking everyone! He",
                        "had completely lost it."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Retired Soldier",
                    args![
                        "We had no choice but to",
                        "kill him. It was self defense,",
                        "but I can't help but pity him.",
                        "All his comrades had been",
                        "killed: I can't imagine how",
                        "that must have felt."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Retired Soldier",
                    args![
                        "We arrived too late to",
                        "see that battle, but I'm",
                        "pretty sure it was horrific.",
                        "You must be pretty interested",
                        "in learning about this soldier."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Retired Soldier",
                    args![
                        "Well, I'm just glad that you",
                        "were able to know more",
                        "about this story. There's",
                        "much you can learn from",
                        "other people's experiences."
                    ],
                )?;
                ctx.var("jewel_nd").set(Val::from(26))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else {
        if (ctx.var("jewel_nd").get()?.number()? > 25 && ctx.var("jewel_nd").get()?.number()? < 29) {
            ctx.lines_as(
                "Retired Soldier",
                args![
                    "I don't regret that we",
                    "had to kill that warrior.",
                    "We did had we had to",
                    "back in those days."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("jewel_nd").get()? == 29 {
                ctx.lines_as(
                    "Retired Soldier",
                    args![
                        "Oh, it's you again.",
                        "Did you want to ask me",
                        "more about that warrior?",
                        "I'm afraid I already told",
                        "you everything I know."
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Sorry...:I met the warrior's spirit.")])? {
                    1 => {
                        ctx.lines_as("Retired Soldier", args!["It's fine."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as("Retired Soldier", args!["What? How is that", "even possible?"])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^3355FFYou explain to the",
                            "retired soldier everything",
                            "that has happened so far.^000000"
                        ])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Retired Soldier",
                            args![
                                "That's quite a story.",
                                "Still, I'm surprised that",
                                "he didn't want to talk to",
                                "you about what happened.",
                                "I guess he still harbors",
                                "bad feelings about that day..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Retired Soldier",
                            args![
                                "Well, I guess it doesn't",
                                "really matter now. Chalk",
                                "this all up to a really good",
                                "learning experience."
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(
                            ctx,
                            &[Val::from("I feel so sorry for him.:Do you believe the jewel is really cursed?")],
                        )? {
                            1 => {
                                ctx.lines_as(
                                    "Retired Soldier",
                                    args![
                                        "Well...",
                                        "I don't know what to tell",
                                        "you. I mean, you can't really",
                                        "put the blame on anybody.",
                                        "He just lost his mind..."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    "Retired Soldier",
                                    args![
                                        "I'm not really sure if the",
                                        "jewel was cursed, if that",
                                        "warrior started the curse,",
                                        "or if the jewel isn't cursed",
                                        "at all. I'm just an old,",
                                        "simple soldier."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Retired Soldier",
                                    args![
                                        "One thing's for sure:",
                                        "if there was a curse, that",
                                        "soldier should have been",
                                        "able to overcome it if his will",
                                        "was strong enough. It's harsh",
                                        "to say it like this, but..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Retired Soldier",
                                    args![
                                        "True warriors have the",
                                        "strength to endure, to",
                                        "fight temptation, to keep",
                                        "their focus. That guy...",
                                        "He wasn't cut out to fight."
                                    ],
                                )?;
                                ctx.var("jewel_nd").set(Val::from(30))?;
                                ctx.next()?;
                                if ctx.call(Function::CountItem, vec![Val::from(7725)])?.number()? > 0 {
                                    ctx.lines_as(
                                        "Retired Soldier",
                                        args![
                                            "You know what?",
                                            "If you're so worried",
                                            "about a curse, why don't",
                                            "you leave that jewel with me?",
                                            "Don't worry, I'll make sure",
                                            "to compensate you for it."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Retired Soldier",
                                        args![
                                            "I'd rather keep it to myself,",
                                            "rather than risk it messing",
                                            "around with other people.",
                                            "I know my will's tough enough",
                                            "to resist its curse, you know,",
                                            "if it actually exists."
                                        ],
                                    )?;
                                    ctx.call(Function::DelItem, vec![Val::from(7725), Val::from(1)])?;
                                    ctx.var("jewel_nd").set(Val::from(31))?;
                                    ctx.call(Function::GetItem, vec![Val::from(603), Val::from(1)])?;
                                    {
                                        if ctx.var("BaseLevel").get()?.number()? < 66 {
                                            ctx.call(Function::GetItem, vec![Val::from(607), Val::from(1)])?;
                                        } else {
                                            if (ctx.var("BaseLevel").get()?.number()? > 65 && ctx.var("BaseLevel").get()?.number()? < 75) {
                                                ctx.call(Function::GetExperience, vec![Val::from(50000), Val::from(0)])?;
                                            } else {
                                                if (ctx.var("BaseLevel").get()?.number()? > 74
                                                    && ctx.var("BaseLevel").get()?.number()? < 81)
                                                {
                                                    ctx.call(Function::GetExperience, vec![Val::from(180000), Val::from(0)])?;
                                                } else if (ctx.var("BaseLevel").get()?.number()? > 80
                                                    && ctx.var("BaseLevel").get()?.number()? < 86)
                                                {
                                                    ctx.call(Function::GetExperience, vec![Val::from(360000), Val::from(0)])?;
                                                } else if (ctx.var("BaseLevel").get()?.number()? > 85
                                                    && ctx.var("BaseLevel").get()?.number()? < 91)
                                                {
                                                    ctx.call(Function::GetExperience, vec![Val::from(500000), Val::from(0)])?;
                                                } else if (ctx.var("BaseLevel").get()?.number()? > 90
                                                    && ctx.var("BaseLevel").get()?.number()? < 96)
                                                {
                                                    ctx.call(Function::GetExperience, vec![Val::from(800000), Val::from(0)])?;
                                                } else if (ctx.var("BaseLevel").get()?.number()? > 95
                                                    && ctx.var("BaseLevel").get()?.number()? < 99)
                                                {
                                                    ctx.call(Function::GetExperience, vec![Val::from(1000000), Val::from(0)])?;
                                                } else {
                                                    ctx.call(Function::GetItem, vec![Val::from(617), Val::from(1)])?;
                                                }
                                            }
                                        }
                                    }
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Retired Soldier",
                                        args![
                                            "If there's anything",
                                            "you learn from all this,",
                                            "it's that you should never",
                                            "succumb to your desperation.",
                                            "Rage gives you nothing:",
                                            "its power is an illusion."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    ctx.lines_as(
                                        "Retired Soldier",
                                        args![
                                            "You know what?",
                                            "If you're so worried",
                                            "about a curse, why don't",
                                            "you leave that jewel with me?",
                                            "Don't worry, I'll make sure",
                                            "to compensate you for it."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Retired Soldier",
                                        args![
                                            "I'd rather keep it to myself,",
                                            "rather than risk it messing",
                                            "around with other people.",
                                            "I know my will's tough enough",
                                            "to resist its curse, you know,",
                                            "if it actually exists."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            } else {
                if ctx.var("jewel_nd").get()? == 30 {
                    if ctx.call(Function::CountItem, vec![Val::from(7725)])?.number()? > 0 {
                        ctx.call(Function::DelItem, vec![Val::from(7725), Val::from(1)])?;
                        ctx.var("jewel_nd").set(Val::from(31))?;
                        ctx.call(Function::GetItem, vec![Val::from(603), Val::from(1)])?;
                        {
                            if ctx.var("BaseLevel").get()?.number()? < 66 {
                                ctx.call(Function::GetItem, vec![Val::from(607), Val::from(1)])?;
                            } else {
                                if (ctx.var("BaseLevel").get()?.number()? > 65 && ctx.var("BaseLevel").get()?.number()? < 75) {
                                    ctx.call(Function::GetExperience, vec![Val::from(50000), Val::from(0)])?;
                                } else {
                                    if (ctx.var("BaseLevel").get()?.number()? > 74 && ctx.var("BaseLevel").get()?.number()? < 81) {
                                        ctx.call(Function::GetExperience, vec![Val::from(80000), Val::from(0)])?;
                                    } else if (ctx.var("BaseLevel").get()?.number()? > 80 && ctx.var("BaseLevel").get()?.number()? < 86) {
                                        ctx.call(Function::GetExperience, vec![Val::from(150000), Val::from(0)])?;
                                    } else if (ctx.var("BaseLevel").get()?.number()? > 85 && ctx.var("BaseLevel").get()?.number()? < 91) {
                                        ctx.call(Function::GetExperience, vec![Val::from(200000), Val::from(0)])?;
                                    } else if (ctx.var("BaseLevel").get()?.number()? > 90 && ctx.var("BaseLevel").get()?.number()? < 96) {
                                        ctx.call(Function::GetExperience, vec![Val::from(400000), Val::from(0)])?;
                                    } else if (ctx.var("BaseLevel").get()?.number()? > 95 && ctx.var("BaseLevel").get()?.number()? < 99) {
                                        ctx.call(Function::GetExperience, vec![Val::from(500000), Val::from(0)])?;
                                    } else {
                                        ctx.call(Function::GetItem, vec![Val::from(617), Val::from(1)])?;
                                    }
                                }
                            }
                        }
                        ctx.next()?;
                        ctx.lines_as(
                            "Retired Soldier",
                            args![
                                "If there's anything",
                                "you learn from all this,",
                                "it's that you should never",
                                "succumb to your desperation.",
                                "Rage gives you nothing:",
                                "its power is an illusion."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Retired Soldier",
                            args![
                                "You know what?",
                                "If you're so worried",
                                "about a curse, why don't",
                                "you leave that jewel with me?",
                                "Don't worry, I'll make sure",
                                "to compensate you for it."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Retired Soldier",
                            args![
                                "I'd rather keep it to myself,",
                                "rather than risk it messing",
                                "around with other people.",
                                "I know my will's tough enough",
                                "to resist its curse, you know,",
                                "if it actually exists."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                } else {
                    if ctx.var("jewel_nd").get()? == 31 {
                        ctx.lines_as(
                            "Retired Soldier",
                            args![
                                "Well, you learned all there",
                                "is to know about the Unlucky",
                                "Emerald. It's time you forget",
                                "about the curse or whatever,",
                                "and find new adventures.",
                                "Ah, to be young again..."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Retired Soldier",
                            args![
                                "Ah, how I long for",
                                "the old days. Being",
                                "retired is just so...",
                                "It's so boring! I guess",
                                "it's time to find a hobby."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
            }
        }
    }
    Ok(Val::from(0))
}

pub fn old_soldier(ctx: &Ctx) -> Script {
    old_soldier_body(ctx, Vec::new()).map(|_| ())
}

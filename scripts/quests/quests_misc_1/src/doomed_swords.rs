#![allow(
    unused_imports,
    unused_labels,
    unused_assignments,
    unused_mut,
    unused_parens,
    unused_variables,
    unreachable_code
)]

use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn q2_has(ctx: &Ctx, flag: i32) -> Result<bool, Stop> {
    Ok((ctx.var("dmdswrd_q2").get()?.number()? & flag) != 0)
}

fn q2_any(ctx: &Ctx, flags: &[i32]) -> Result<bool, Stop> {
    for &flag in flags {
        if q2_has(ctx, flag)? {
            return Ok(true);
        }
    }
    Ok(false)
}

fn q2_add(ctx: &Ctx, flag: i32) -> Script {
    ctx.var("dmdswrd_q2").set(Val::from(ctx.var("dmdswrd_q2").get()?.number()? | flag))
}

pub fn ghatu_magum(ctx: &Ctx) -> Script {
    if ctx.var("event_magum").get()? == 0 {
        if q2_any(ctx, &[1, 2, 4, 8, 16, 32])? {
            ctx.lines_as(
                "Ghatu",
                args![
                    "I've heard of a strange",
                    "blacksmith who lives in",
                    "seclusion deep in the",
                    "Payon Forest. The man",
                    "was once famous for his",
                    "legendary smithing skill..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Ghatu",
                args![
                    "Rumor has it that he's",
                    "hiding in the mountains",
                    "since too many people want to use his talents for the wrong",
                    "purposes. Apparently, there is a godly quality to his weapons..."
                ],
            )?;
            ctx.var("event_magum").set(Val::from(1))?;
            return ctx.close();
        }
        return ghatu_rumor(ctx);
    } else if ctx.var("event_magum").get()? == 1 {
        return ghatu_rumor(ctx);
    }
    ctx.lines_as(
        "Ghatu",
        args![
            "That mysterious blacksmith...",
            "I wonder how he made those",
            "enchanted weapons of his.",
            "I've heard that he might have",
            "been dabbling in the dark arts,",
            "but that doesn't seem right..."
        ],
    )?;
    return ctx.close();
}

fn ghatu_rumor(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Ghatu",
        args![
            "Remember that blacksmith",
            "I told you about last time? It",
            "seems that his weapons were",
            "in such great demand because",
            "he would enchant them with ",
            "tremendously powerful magic."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Ghatu",
        args![
            "However, the man was forced",
            "by the power hungry and the",
            "oppressive to create weapons",
            "for selfish and immoral ends.",
            "It's no surprise that he went",
            "into hiding in the end..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Ghatu",
        args![
            "It's sad, really.",
            "That smith used to be",
            "a pretty easy going guy",
            "until he was forced to work",
            "against his will. Slowly, he became gloomy and intimidating..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Ghatu",
        args![
            "But I suppose blacksmiths",
            "can never really give up the",
            "hammer. If you can manage to",
            "find him, perhaps you can ask",
            "him to forge something for you?"
        ],
    )?;
    ctx.var("event_magum").set(Val::from(5))?;
    return ctx.close();
}

pub fn veeyop_magum(ctx: &Ctx) -> Script {
    if q2_has(ctx, 1)? {
        ctx.lines_as(
            "Veeyop",
            args![
                "You know, talking",
                "about Mysteltainn and",
                "the death of Baldur just...",
                "It didn't occur to me until",
                "now just how morbid it all",
                "sounds. Yeah, yeah, I know."
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as(
        "Veeyop",
        args![
            "Have you heard the",
            "legend of Mysteltainn?",
            "It's a tree, known for being",
            "the only thing able to harm",
            "Baldur, god of light. In fact,",
            "just a twig from it killed him."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Veeyop",
        args![
            "Now, I'm not sure if this",
            "is true, but I've heard that",
            "someone actually forged",
            "a sword so powerful, it's",
            "worthy of the name,",
            "''Mysteltainn.''"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Veeyop",
        args![
            "There's only one person",
            "in the world who can forge",
            "that Mysteltainn sword. I...",
            "I think he used to live in",
            "Prontera, but then he moved",
            "away for some weird reason."
        ],
    )?;
    if ctx.var("event_magum").get()? == 1 {
        ctx.var("event_magum").set(Val::from(5))?;
    }
    q2_add(ctx, 1)?;
    return ctx.close();
}

pub fn cetsu_magum(ctx: &Ctx) -> Script {
    if q2_has(ctx, 2)? {
        ctx.lines_as(
            "Cetsu",
            args![
                "I keep thinking about",
                "Grimtooth, and it occurred",
                "to me that the spell used to",
                "to endow that dagger with its",
                "strength probably isn't magic",
                "that we're familiar with."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Cetsu",
            args![
                "There's all sorts of",
                "creepy stories going",
                "around. I even hear",
                "that the Grimtooth",
                "might have some",
                "sort of weird curse?"
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as(
        "Cetsu",
        args![
            "You know, there are",
            "stories of an incredibly",
            "powerful dagger that's",
            "stronger than steel.",
            "I think it was made",
            "out of ogre teeth?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Cetsu",
        args![
            "Anyway, it wasn't just",
            "the ogre teeth that gave",
            "the dagger its power. I think",
            "it had to be enchanted with",
            "a special spell or something."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Cetsu",
        args![
            "This dagger, the Grimtooth,",
            "can only be made by one person",
            "in the entire world. This guy used to live in Prontera, but then he",
            "moved away for some reason."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Cetsu",
        args![
            "I wonder...",
            "Did he go into",
            "hiding? His weapons",
            "did seem to be pretty",
            "high in demand back then..."
        ],
    )?;
    if ctx.var("event_magum").get()? == 1 {
        ctx.var("event_magum").set(Val::from(5))?;
    }
    q2_add(ctx, 2)?;
    return ctx.close();
}

pub fn nain_magum(ctx: &Ctx) -> Script {
    if ctx.var("dmdswrd_q2").get()? == 4 {
        ctx.lines_as(
            "Nain",
            args![
                "It doesn't matter how",
                "powerful the Executioner",
                "is: if it were to end up in",
                "my hands, I would get rid",
                "of it right away. I'd never risk losing my mind to that curse..."
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as(
        "Nain",
        args![
            "Long ago, one sword",
            "was used to behead all",
            "the criminals that had been",
            "sentenced to death. That",
            "accursed blade is known",
            "as the Executioner."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Nain",
        args![
            "As the Executioner slayed",
            "more criminals, the rage and",
            "bloodlust of its victims began",
            "to accumulate upon the blade.",
            "Although the sword gained great strength, it was tainted by evil."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Nain",
        args![
            "The last person to wield",
            "the Executioner almost lost",
            "his mind to the sword. He saved himself by giving it to a talented",
            "blacksmith who would destroy it for him, thus saving his soul."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Nain",
        args![
            "This mysterious blacksmith",
            "was never seen again in the",
            "city of Prontera, but rumor has",
            "it that if you can find him, he",
            "can forge that accursed",
            "Executioner anew..."
        ],
    )?;
    if ctx.var("event_magum").get()? == 1 {
        ctx.var("event_magum").set(Val::from(5))?;
    }
    q2_add(ctx, 4)?;
    return ctx.close();
}

pub fn mysterious_man_magum(ctx: &Ctx) -> Script {
    if ctx.call(Function::CheckWeight, args![1201, 1])? == 0 {
        ctx.lines_as(
            "Mysterious Man",
            args![
                "Hold it.",
                "You're carrying",
                "far too many items",
                "with you. Speak to me",
                "after you've placed your",
                "goods into Kafra Storage."
            ],
        )?;
        return ctx.close();
    }
    if ctx.var("event_magum").get()? == 0 {
        if q2_any(ctx, &[1, 2, 4, 8, 16, 32])? {
            ctx.lines_as("Mysterious Man", args!["Well...?", "What the hell", "are you looking at?"])?;
            ctx.next()?;
            if runtime::select_values(ctx, &[Val::from("Have you heard of the Doomed Swords?:I... I...")])? == 1 {
                ctx.lines_as(
                    "Mysterious Man",
                    args!["...!", "How the hell would", "I know about that?", "Leave me alone!"],
                )?;
                return ctx.close();
            }
            ctx.lines_as("Mysterious Man", args!["..."])?;
            ctx.next()?;
            ctx.lines_as("Mysterious Man", args!["...", "......"])?;
            ctx.next()?;
            ctx.lines_as("Mysterious Man", args!["Get lost."])?;
            return ctx.close();
        }
        ctx.lines_as("Mysterious Man", args!["Well...?", "What the hell", "are you looking at?"])?;
        ctx.next()?;
        if runtime::select_values(ctx, &[Val::from("Um, er...:Nothing, sir.")])? == 1 {
            ctx.lines_as("Mysterious Man", args!["What...?!"])?;
            ctx.next()?;
            ctx.lines(args!["^3355FFThis guy is", "really intimidating!^000000"])?;
            return ctx.close();
        }
        ctx.lines_as(
            "Mysterious Man",
            args![
                "Nothing, huh?",
                "Well, right now, I'm",
                "looking at a bothersome",
                "adventurer! Get outta here",
                "and leave me the hell alone!"
            ],
        )?;
        return ctx.close();
    } else if ctx.var("event_magum").get()? == 1 {
        ctx.lines_as("Mysterious Man", args!["Well...?", "What the hell", "are you looking at?"])?;
        ctx.next()?;
        if runtime::select_values(ctx, &[Val::from("Do you happen to be a blacksmith?:No-Nothing!")])? == 1 {
            ctx.lines_as(
                "Mysterious Blacksmith",
                args![
                    "Hmpf. So you're not",
                    "a total fool after all. Yes,",
                    "I used to do smithing, but",
                    "I don't do the simple work",
                    "that most Blacksmiths",
                    "can do nowadays..."
                ],
            )?;
            return ctx.close();
        }
        ctx.lines_as("Mysterious Blacksmith", args!["...", "......"])?;
        ctx.next()?;
        ctx.lines_as("Mysterious Blacksmith", args!["Get outta my sight."])?;
        return ctx.close();
    } else if ctx.var("event_magum").get()? == 5 {
        ctx.lines_as(
            "Mysterious Blacksmith",
            args!["Well...?", "What the hell", "are you looking at?"],
        )?;
        ctx.next()?;
        if runtime::select_values(ctx, &[Val::from("I want you to make me a Doomed Sword.:Er, nothing!")])? == 1 {
            ctx.lines_as(
                "Mysterious Blacksmith",
                args![
                    "Hm. I don't know where",
                    "the hell you may have heard",
                    "of me, but I guess one of you",
                    "adventurers would find me soon",
                    "enough. Now, which doomed",
                    "sword did you wish to possess?"
                ],
            )?;
            ctx.next()?;
            let subject1 = runtime::select_values(ctx, &[Val::from("Mysteltainn.:Grimtooth.:Executioner.:I ch-changed my mind!")])?;
            if subject1 == 1 {
                if q2_has(ctx, 1)? {
                    ctx.lines_as(
                        "Mysterious Blacksmith",
                        args!["You want...", "that sword?", "Don't speak its", "name so lightly!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mysterious Blacksmith",
                        args![
                            "Do you understand the curse",
                            "on that sword? The Mysteltainn",
                            "derives its dark power from the",
                            "twig that was used to kill Baldur, god of light. Let me relate the",
                            "story as briefly as I can..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mysterious Blacksmith",
                        args![
                            "In the era of the gods,",
                            "the beautiful, pure and",
                            "joyful Baldur was loved by",
                            "all living creatures, save for",
                            "one: Loki, the god of trickery."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mysterious Blacksmith",
                        args![
                            "When Loki, out of jealousy,",
                            "decided to kill Baldur, the",
                            "goddess Freyja had a dream",
                            "about Baldur's death. Fearing",
                            "the realization of her dream,",
                            "she counseled with the gods."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mysterious Blacksmith",
                        args![
                            "To protect Baldur, the gods",
                            "decided to extract an oath",
                            "to never harm Baldur from",
                            "every creature, object and",
                            "natural force. All who were",
                            "asked agreed to make this oath."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mysterious Blacksmith",
                        args![
                            "Of all the beings and objects",
                            "in the universe, Freya only",
                            "neglected to ask one tree to",
                            "make this oath: the Mysteltainn. Freyja believed it was far too",
                            "small and insignificant to ask."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mysterious Blacksmith",
                        args![
                            "Believing that everything",
                            "in the universe had sworn",
                            "not to harm Baldur, the gods",
                            "made it their new pastime to",
                            "throw daggers and knives at",
                            "the now nigh invincible god."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mysterious Blacksmith",
                        args![
                            "However, Loki was determined",
                            "to kill Baldur and, disguising",
                            "himself, politely asked Freyja",
                            "if there was any object in the",
                            "world that did not take the",
                            "oath to not harm Baldur."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mysterious Blacksmith",
                        args![
                            "Freyja suspected nothing and",
                            "told Loki about the secret of",
                            "Mysteltainn. The next time the gods played their game of throwing",
                            "objects at Baldur, Loki was there with a small Mysteltainn twig."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mysterious Blacksmith",
                        args![
                            "Then, to compound his",
                            "treachery, Loki tricked",
                            "Hod, Baldur's blind twin",
                            "brother, into throwing the",
                            "twig into Baldur's heart. And",
                            "so, the god of light was slain."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mysterious Blacksmith",
                        args![
                            "Over the years,",
                            "Mysteltainn has been",
                            "fashioned into the sword",
                            "that you may be familiar",
                            "with today. Its power is",
                            "strictly forbidden by the gods."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mysterious Blacksmith",
                        args![
                            "However, if you're willing",
                            "to risk that sword's curse",
                            "for the sake of its power,",
                            "I will forge it for you if you",
                            "can bring me the following",
                            "items. They'll make sense..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mysterious Blacksmith",
                        args![
                            "^0099FF1 Young Twig^000000,",
                            "^0099FF1 Emperium^000000,",
                            "^0099FF1 Loki's Whispers^000000,",
                            "^0099FF1 Mother's Nightmare^000000 and",
                            "^0099FF1 Foolishness of the Blind^000000.",
                            "That is what I need."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mysterious Blacksmith",
                        args![
                            "I'm not asking for much.",
                            "You are only bringing the raw",
                            "materials to forge the sword,",
                            "and an Emperium to prove your",
                            "worthiness to me. I will wait",
                            "for your return, adventurer."
                        ],
                    )?;
                    q2_add(ctx, 8)?;
                    ctx.var("event_magum").set(Val::from(6))?;
                    return ctx.close();
                }
                ctx.lines_as(
                    "Mysterious Blacksmith",
                    args![
                        "Just by looking at you,",
                        "I can tell that you don't know",
                        "enough about the Mysteltainn",
                        "to fully understand all of the",
                        "risks that come with wielding that sword. Yes, you're too green."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mysterious Blacksmith",
                    args![
                        "I can't take the risk",
                        "of creating that accursed",
                        "sword for you if you haven't",
                        "learned enough about it to",
                        "be fully prepared for any of",
                        "the consequences..."
                    ],
                )?;
                return ctx.close();
            } else if subject1 == 2 {
                if q2_has(ctx, 2)? {
                    ctx.lines_as(
                        "Mysterious Blacksmith",
                        args![
                            "Ah, yes. You must be",
                            "wondering how such a small",
                            "dagger can contain such power.",
                            "It's simple. I cast forbidden",
                            "curse magic to inbue the dagger with its awesome destructiveness."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mysterious Blacksmith",
                        args![
                            "The curse I cast is so",
                            "powerful that if I use it on",
                            "a metal dagger, it would",
                            "immediately melt down. Ogre",
                            "tooth is the only material that",
                            "can withstand the curse magic."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mysterious Blacksmith",
                        args![
                            "Of course, it's only",
                            "fair to warn you that",
                            "the power of the curse",
                            "is such that the more you",
                            "use the Grimtooth, the more it burns away at your very soul..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mysterious Blacksmith",
                        args![
                            "I am willing to forge",
                            "the Grimtooth if you can",
                            "prove worthy to wield its",
                            "power and provide all of the",
                            "materials I need to create it."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mysterious Blacksmith",
                        args![
                            "I will need...",
                            "^0099FF1 Emperium^000000,",
                            "^0099FF100 Ogre Teeth^000000,",
                            "^0099FF10 Blades of Darkness^000000,",
                            "^0099FF5 Cursed Rubies^000000 and",
                            "^0099FF1 Broken Sword Handle^000000."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mysterious Blacksmith",
                        args![
                            "I don't need the",
                            "Emperium to create",
                            "the accursed sword, but I do",
                            "require you to prove that you",
                            "are worthy of the Grimtooth's",
                            "power. Can you truly handle it?"
                        ],
                    )?;
                    q2_add(ctx, 16)?;
                    ctx.var("event_magum").set(Val::from(6))?;
                    return ctx.close();
                }
                ctx.lines_as(
                    "Mysterious Blacksmith",
                    args![
                        "Hmpf. You don't really",
                        "know what you're asking",
                        "for, do you? I won't create",
                        "that kind of weapon for any",
                        "ignorant fool. Learn more about",
                        "Grimtooth before you ask again!"
                    ],
                )?;
                return ctx.close();
            } else if subject1 == 3 {
                if q2_has(ctx, 4)? {
                    ctx.lines_as(
                        "Mysterious Blacksmith",
                        args![
                            "So you're telling me that",
                            "you want a sword that's been",
                            "cursed by all the souls that've",
                            "died from the thousands of death sentences it has carried out?",
                            "Ha ha ha! Very interesting!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mysterious Blacksmith",
                        args![
                            "Yes... Each time this",
                            "sword cut down a guilty",
                            "criminal, the blade was",
                            "cursed by the rage and",
                            "regret of the slain."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mysterious Blacksmith",
                        args![
                            "Over time, the Executioner",
                            "accumulated a truly dreadful",
                            "power from the hatred of those",
                            "it had killed. However, he who",
                            "uses this sword risks becoming consumed by insanity and hatred."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mysterious Blacksmith",
                        args![
                            "The last person for whom",
                            "I created this sword asked",
                            "me to destroy it, fearing that",
                            "it would corrupt his mind with",
                            "its bloodlust. But if you think you can endure, I may forge it..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mysterious Blacksmith",
                        args![
                            "Just bring me...",
                            "^0099FF50 Amulets^000000,",
                            "^0099FF1 Emperium^000000,",
                            "^0099FF2 Executioner's Gloves^000000,",
                            "^0099FF10 Bloody Edges^000000 and",
                            "^0099FF3 Necklaces of Oblivion^000000."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mysterious Blacksmith",
                        args![
                            "Those are the items I need",
                            "to create a sword imbued with",
                            "evil power. As for the Emperium, consider it my test to see if you",
                            "are truly capable of wielding",
                            "the accursed Executioner..."
                        ],
                    )?;
                    q2_add(ctx, 32)?;
                    ctx.var("event_magum").set(Val::from(6))?;
                    return ctx.close();
                }
                ctx.lines_as(
                    "Mysterious Blacksmith",
                    args![
                        "Do you even know what",
                        "you are asking of me...?",
                        "Fool! Go and learn more",
                        "about the Executioner! Then,",
                        "maybe you'll understand how",
                        "dangerous that sword truly is!"
                    ],
                )?;
                return ctx.close();
            } else if subject1 == 4 {
                ctx.lines_as("Mysterious Blacksmith", args!["Hmm...?", "That a fact?"])?;
                return ctx.close();
            }
        }
        return ctx.close();
    } else if ctx.var("event_magum").get()? == 6 {
        ctx.lines_as(
            "Mysterious Blacksmith",
            args![
                "Hmmm...",
                "Have you brought",
                "the required items?",
                "Or did you forget what",
                "you needed to bring to me?"
            ],
        )?;
        ctx.next()?;
        'b2: {
            let subject2 = runtime::select_values(
                ctx,
                &[Val::from(
                    "Er, what were the items again?:Yes, I brought all the required items.:Oh! Um, never mind!",
                )],
            )?;
            let mut matched2 = false;
            if !matched2 && subject2 == 1 {
                matched2 = true;
            }
            if matched2 {
                ctx.lines_as(
                    "Mysterious Blacksmith",
                    args!["Hmpf. I thought so.", "Now, which sword did", "you want me to forge?"],
                )?;
                ctx.next()?;
                'b3: {
                    let subject3 = runtime::select_values(
                        ctx,
                        &[Val::from("Mysteltainn.:Grimtooth.:Executioner.:Wait! I just remembered!")],
                    )?;
                    let mut matched3 = false;
                    if !matched3 && subject3 == 1 {
                        matched3 = true;
                    }
                    if matched3 {
                        if q2_any(ctx, &[1, 8])? {
                            ctx.lines_as(
                                "Mysterious Blacksmith",
                                args![
                                    "^0099FF1 Young Twig^000000,",
                                    "^0099FF1 Emperium^000000,",
                                    "^0099FF1 Loki's Whispers^000000,",
                                    "^0099FF1 Mother's Nightmare^000000 and",
                                    "^0099FF1 Foolishness of the Blind^000000.",
                                    "That is what I need."
                                ],
                            )?;
                            return ctx.close();
                        }
                        ctx.lines_as(
                            "Mysterious Blacksmith",
                            args![
                                "Hmm. You don't understand",
                                "enough about Mysteltainn for",
                                "me to risk forging that sword",
                                "for you. Think again: which",
                                "sword did you ask me to create?"
                            ],
                        )?;
                        return ctx.close();
                    }
                    if !matched3 && subject3 == 2 {
                        matched3 = true;
                    }
                    if matched3 {
                        if q2_any(ctx, &[2, 16])? {
                            ctx.lines_as(
                                "Mysterious Blacksmith",
                                args![
                                    "I will need...",
                                    "^0099FF1 Emperium^000000,",
                                    "^0099FF100 Ogre Teeth^000000,",
                                    "^0099FF10 Blades of Darkness^000000,",
                                    "^0099FF5 Cursed Rubies^000000 and",
                                    "^0099FF1 Broken Sword Handle^000000."
                                ],
                            )?;
                            return ctx.close();
                        }
                        ctx.lines_as(
                            "Mysterious Blacksmith",
                            args![
                                "Hmm. You don't know enough",
                                "about the Grimtooth for me to",
                                "risk forging it for you. Learn",
                                "more about that dagger if you",
                                "really want to possess it..."
                            ],
                        )?;
                        return ctx.close();
                    }
                    if !matched3 && subject3 == 3 {
                        matched3 = true;
                    }
                    if matched3 {
                        if q2_any(ctx, &[4, 32])? {
                            ctx.lines_as(
                                "Mysterious Blacksmith",
                                args![
                                    "Just bring me...",
                                    "^0099FF50 Amulets^000000,",
                                    "^0099FF1 Emperium^000000,",
                                    "^0099FF2 Executioner's Gloves^000000,",
                                    "^0099FF10 Bloody Edges^000000 and",
                                    "^0099FF3 Necklaces of Oblivion^000000."
                                ],
                            )?;
                            return ctx.close();
                        }
                        ctx.lines_as(
                            "Mysterious Blacksmith",
                            args![
                                "You don't know anything",
                                "about the Executioner. Learn",
                                "more about that sword before",
                                "you consider asking me to",
                                "forge that monstrocity."
                            ],
                        )?;
                        return ctx.close();
                    }
                    if !matched3 && subject3 == 4 {
                        matched3 = true;
                    }
                    if matched3 {
                        ctx.lines_as(
                            "Mysterious Blacksmith",
                            args!["Fine.", "Then bring all", "the items I require", "when you are ready."],
                        )?;
                        return ctx.close();
                    }
                }
            }
            if !matched2 && subject2 == 2 {
                matched2 = true;
            }
            if matched2 {
                ctx.lines_as(
                    "Mysterious Blacksmith",
                    args!["So you think you're", "ready. Now, which sword", "did you want me to craft?"],
                )?;
                ctx.next()?;
                'b4: {
                    let subject4 = runtime::select_values(ctx, &[Val::from("Mysteltainn:Grimtooth:Executioner:I ch-changed my mind!")])?;
                    let mut matched4 = false;
                    if !matched4 && subject4 == 1 {
                        matched4 = true;
                    }
                    if matched4 {
                        if q2_has(ctx, 8)? {
                            ctx.lines_as(
                                "Mysterious Blacksmith",
                                args![
                                    "Mysteltainn. Hm.",
                                    "Let me make sure",
                                    "that you brought",
                                    "everything I need to",
                                    "create this sword..."
                                ],
                            )?;
                            ctx.next()?;
                            if ctx.items().count(7018)? < 1 {
                                ctx.lines_as(
                                    "Mysterious Blacksmith",
                                    args![
                                        "Well, well.",
                                        "You've forgotten",
                                        "to bring ^0099FF1 Young Twig^000000,",
                                        "the embodiment of the",
                                        "Mysteltainn twig used to kill Baldur. Hurry and bring it..."
                                    ],
                                )?;
                                return ctx.close();
                            }
                            if ctx.items().count(7019)? < 1 {
                                ctx.lines_as(
                                    "Mysterious Blacksmith",
                                    args![
                                        "Hm. You forgot to bring",
                                        "^0099FF1 Loki's Whispers^000000. We need",
                                        "that to imbue the sword with",
                                        "immense, evil power. Go and",
                                        "find that as quickly as you can!"
                                    ],
                                )?;
                                return ctx.close();
                            }
                            if ctx.items().count(7020)? < 1 {
                                ctx.lines_as(
                                    "Mysterious Blacksmith",
                                    args![
                                        "Hm, you still need to",
                                        "bring ^0099FF1 Mother's Nightmare^000000",
                                        "to instill the power of misery",
                                        "and grave portent to this blade..."
                                    ],
                                )?;
                                return ctx.close();
                            }
                            if ctx.items().count(7021)? < 1 {
                                ctx.lines_as(
                                    "Mysterious Blacksmith",
                                    args![
                                        "Hm, you still need to",
                                        "bring ^0099FF1 Foolishness",
                                        "of the Blind^000000 to instill the",
                                        "energy of tragic regret into",
                                        "the curse imbued into the blade..."
                                    ],
                                )?;
                                return ctx.close();
                            }
                            if ctx.items().count(714)? < 1 {
                                ctx.lines_as(
                                    "Mysterious Blacksmith",
                                    args![
                                        "You have everything",
                                        "I require to forge the",
                                        "Mysteltainn, but you must",
                                        "still prove yourself capable",
                                        "of wielding it. Bring ^0099FF1 Emperium^000000 and I will recognize your worth."
                                    ],
                                )?;
                                return ctx.close();
                            }
                            if ctx.items().count(7018)? > 0
                                && ctx.items().count(7019)? > 0
                                && ctx.items().count(7020)? > 0
                                && ctx.items().count(7021)? > 0
                                && ctx.items().count(714)? > 0
                            {
                                ctx.lines_as(
                                    "Mysterious Blacksmith",
                                    args![
                                        "Well done, adventurer.",
                                        "All seems to be in readiness.",
                                        "Give me a minute to summon",
                                        "the dark power to forge the",
                                        "forbidden sword, Mysteltainn."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Mysterious Blacksmith",
                                    args![
                                        "I... I never thought",
                                        "I would ever see this",
                                        "sword again. Take it,",
                                        "quickly! I d-don't want",
                                        "to touch it if I can avoid it."
                                    ],
                                )?;
                                ctx.items().take(7018, 1)?;
                                ctx.items().take(7019, 1)?;
                                ctx.items().take(7020, 1)?;
                                ctx.items().take(7021, 1)?;
                                ctx.items().take(714, 1)?;
                                ctx.items().give(1138, 1)?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Mysterious Blacksmith",
                                    args![
                                        "Be very careful.",
                                        "Don't let yourself",
                                        "be consumed by the",
                                        "power of that sword!",
                                        "By all means, resist",
                                        "Mysteltainn's tragic curse!"
                                    ],
                                )?;
                                return ctx.close();
                            }
                        } else if q2_has(ctx, 1)? {
                            ctx.lines_as(
                                "Mysterious Blacksmith",
                                args!["You want...", "that sword?", "Don't speak its", "name so lightly!"],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Mysterious Blacksmith",
                                args![
                                    "Do you understand the curse",
                                    "on that sword? The Mysteltainn",
                                    "derives its dark power from the",
                                    "twig that was used to kill Baldur, god of light. Let me relate the",
                                    "story as briefly as I can..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Mysterious Blacksmith",
                                args![
                                    "In the era of the gods,",
                                    "the beautiful, pure and",
                                    "joyful Baldur was loved by",
                                    "all living creatures, save for",
                                    "one: Loki, the god of trickery."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Mysterious Blacksmith",
                                args![
                                    "When Loki, out of jealousy,",
                                    "decided to kill Baldur, the",
                                    "goddess Freyja had a dream",
                                    "about Baldur's death. Fearing",
                                    "the realization of her dream,",
                                    "she counseled with the gods."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Mysterious Blacksmith",
                                args![
                                    "To protect Baldur, the gods",
                                    "decided to extract an oath",
                                    "to never harm Baldur from",
                                    "every creature, object and",
                                    "natural force. All who were",
                                    "asked agreed to make this oath."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Mysterious Blacksmith",
                                args![
                                    "Of all the beings and objects",
                                    "in the universe, Freya only",
                                    "neglected to ask one tree to",
                                    "make this oath: the Mysteltainn. Freyja believed it was far too",
                                    "small and insignificant to ask."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Mysterious Blacksmith",
                                args![
                                    "Believing that everything",
                                    "in the universe had sworn",
                                    "not to harm Baldur, the gods",
                                    "made it their new pastime to",
                                    "throw daggers and knives at",
                                    "the now nigh invincible god."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Mysterious Blacksmith",
                                args![
                                    "However, Loki was determined",
                                    "to kill Baldur and, disguising",
                                    "himself, politely asked Freyja",
                                    "if there was any object in the",
                                    "world that did not take the",
                                    "oath to not harm Baldur."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Mysterious Blacksmith",
                                args![
                                    "Freyja suspected nothing and",
                                    "told Loki about the secret of",
                                    "Mysteltainn. The next time the gods played their game of throwing",
                                    "objects at Baldur, Loki was there with a small Mysteltainn twig."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Mysterious Blacksmith",
                                args![
                                    "Then, to compound his",
                                    "treachery, Loki tricked",
                                    "Hod, Baldur's blind twin",
                                    "brother, into throwing the",
                                    "twig into Baldur's heart. And",
                                    "so, the god of light was slain."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Mysterious Blacksmith",
                                args![
                                    "Over the years,",
                                    "Mysteltainn has been",
                                    "fashioned into the sword",
                                    "that you may be familiar",
                                    "with today. Its power is",
                                    "strictly forbidden by the gods."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Mysterious Blacksmith",
                                args![
                                    "However, if you're willing",
                                    "to risk that sword's curse",
                                    "for the sake of its power,",
                                    "I will forge it for you if you",
                                    "can bring me the following",
                                    "items. They'll make sense..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Mysterious Blacksmith",
                                args![
                                    "^0099FF1 Young Twig^000000,",
                                    "^0099FF1 Emperium^000000,",
                                    "^0099FF1 Loki's Whispers^000000,",
                                    "^0099FF1 Mother's Nightmare^000000 and",
                                    "^0099FF1 Foolishness of the Blind^000000.",
                                    "That is what I need."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Mysterious Blacksmith",
                                args![
                                    "I'm not asking for much.",
                                    "You are only bringing the raw",
                                    "materials to forge the sword,",
                                    "and an Emperium to prove your",
                                    "worthiness to me. I will wait",
                                    "for your return, adventurer."
                                ],
                            )?;
                            q2_add(ctx, 8)?;
                            return ctx.close();
                        }
                        ctx.lines_as(
                            "Mysterious Blacksmith",
                            args![
                                "Hm. If you truly want",
                                "Mysteltainn, I advise you",
                                "to learn more about it. You",
                                "must know the risks involved",
                                "in wielding that sort of power..."
                            ],
                        )?;
                        return ctx.close();
                    }
                    if !matched4 && subject4 == 2 {
                        matched4 = true;
                    }
                    if matched4 {
                        if q2_has(ctx, 16)? {
                            ctx.lines_as(
                                "Mysterious Blacksmith",
                                args![
                                    "You want to me",
                                    "to forge Grimtooth.",
                                    "First, let me check",
                                    "if you brought all",
                                    "the items I require."
                                ],
                            )?;
                            ctx.next()?;
                            if ctx.items().count(7002)? < 100 {
                                ctx.lines_as(
                                    "Mysterious Blacksmith",
                                    args![
                                        "Hm. I still need",
                                        "^0099FF100 Ogre Teeth^000000 to",
                                        "create a blade that can",
                                        "withstand the might of the",
                                        "Grimtooth's curse magic.",
                                        "Go and bring them soon."
                                    ],
                                )?;
                                return ctx.close();
                            }
                            if ctx.items().count(724)? < 5 {
                                ctx.lines_as(
                                    "Mysterious Blacksmith",
                                    args![
                                        "In order to place",
                                        "the curse that grants",
                                        "the Grimtooth its immense",
                                        "power, I need you to bring",
                                        "^0099FF5 Cursed Rubies^000000. Hurry",
                                        "and bring them to me soon."
                                    ],
                                )?;
                                return ctx.close();
                            }
                            if ctx.items().count(7023)? < 10 {
                                ctx.lines_as(
                                    "Mysterious Blacksmith",
                                    args![
                                        "Hm. I still need",
                                        "^0099FF10 Blades of Darkness^000000",
                                        "in order to for me to forge",
                                        "the Grimtooth. Bring those",
                                        "here as soon as you can."
                                    ],
                                )?;
                                return ctx.close();
                            }
                            if ctx.items().count(7022)? < 1 {
                                ctx.lines_as(
                                    "Mysterious Blacksmith",
                                    args![
                                        "Hm. You almost have",
                                        "everything ready, but",
                                        "I'll need ^0099FF1 Broken Sword",
                                        "Handle^000000 in order to forge",
                                        "the Grimtooth. Hurry and",
                                        "bring me one of those..."
                                    ],
                                )?;
                                return ctx.close();
                            }
                            if ctx.items().count(714)? < 1 {
                                ctx.lines_as(
                                    "Mysterious Blacksmith",
                                    args![
                                        "You have everything I need",
                                        "to create the sword, but I won't forge it until you prove that you",
                                        "are capable of controlling its",
                                        "power. Bring me 1 Emperium",
                                        "as proof of your worthiness."
                                    ],
                                )?;
                                return ctx.close();
                            }
                            if ctx.items().count(7002)? > 99
                                && ctx.items().count(724)? > 4
                                && ctx.items().count(7023)? > 9
                                && ctx.items().count(7022)? > 0
                                && ctx.items().count(714)? > 0
                            {
                                ctx.lines_as(
                                    "Mysterious Blacksmith",
                                    args![
                                        "I see that you have",
                                        "come fully prepared.",
                                        "Perhaps you have grave",
                                        "need of Grimtooth's power.",
                                        "Give me a moment as I forge",
                                        "this accursed sword for you..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Mysterious Blacksmith",
                                    args![
                                        "There, it's finished!",
                                        "Now hurry and take it!",
                                        "I, I don't want to handle",
                                        "the Grimtooth longer than",
                                        "I have to! You'll understand",
                                        "soon enough once you use it..."
                                    ],
                                )?;
                                ctx.items().take(7002, 100)?;
                                ctx.items().take(724, 5)?;
                                ctx.items().take(7023, 10)?;
                                ctx.items().take(7022, 1)?;
                                ctx.items().take(714, 1)?;
                                ctx.items().give(1237, 1)?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Mysterious Blacksmith",
                                    args![
                                        "I know that you might",
                                        "have good intentions for",
                                        "Grimtooth's power, but do",
                                        "not overestimate yourself!",
                                        "Always be wary of Grimtooth,",
                                        "and don't let it eat your soul!"
                                    ],
                                )?;
                                return ctx.close();
                            }
                        } else if q2_has(ctx, 2)? {
                            ctx.lines_as(
                                "Mysterious Blacksmith",
                                args![
                                    "Ah, yes. You must be",
                                    "wondering how such a small",
                                    "dagger can contain such power.",
                                    "It's simple. I cast forbidden",
                                    "curse magic to inbue the dagger with its awesome destructiveness."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Mysterious Blacksmith",
                                args![
                                    "The curse I cast is so",
                                    "powerful that if I use it on",
                                    "a metal dagger, it would",
                                    "immediately melt down. Ogre",
                                    "tooth is the only material that",
                                    "can withstand the curse magic."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Mysterious Blacksmith",
                                args![
                                    "Of course, it's only",
                                    "fair to warn you that",
                                    "the power of the curse",
                                    "is such that the more you",
                                    "use the Grimtooth, the more it burns away at your very soul..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Mysterious Blacksmith",
                                args![
                                    "I am willing to forge",
                                    "the Grimtooth if you can",
                                    "prove worthy to wield its",
                                    "power and provide all of the",
                                    "materials I need to create it."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Mysterious Blacksmith",
                                args![
                                    "I will need...",
                                    "^0099FF1 Emperium^000000,",
                                    "^0099FF100 Ogre Teeth^000000,",
                                    "^0099FF10 Blades of Darkness^000000,",
                                    "^0099FF5 Cursed Rubies^000000 and",
                                    "^0099FF1 Broken Sword Handle^000000."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Mysterious Blacksmith",
                                args![
                                    "I don't need the",
                                    "Emperium to create",
                                    "the accursed sword, but I do",
                                    "require you to prove that you",
                                    "are worthy of the Grimtooth's",
                                    "power. Can you truly handle it?"
                                ],
                            )?;
                            q2_add(ctx, 16)?;
                            return ctx.close();
                        }
                        ctx.lines_as(
                            "Mysterious Blacksmith",
                            args![
                                "Hm. I can't even consider",
                                "forging Grimtooth for you",
                                "when you know so very little",
                                "about it. You better learn more",
                                "about that sword before you",
                                "ask me about it again."
                            ],
                        )?;
                        return ctx.close();
                    }
                    if !matched4 && subject4 == 3 {
                        matched4 = true;
                    }
                    if matched4 {
                        if q2_has(ctx, 32)? {
                            ctx.lines_as(
                                "Mysterious Blacksmith",
                                args![
                                    "The Executioner sword.",
                                    "Let me see if you've come",
                                    "prepared to wield that blade",
                                    "with your own two hands..."
                                ],
                            )?;
                            ctx.next()?;
                            if ctx.items().count(7017)? < 2 {
                                ctx.lines_as(
                                    "Mysterious Blacksmith",
                                    args![
                                        "Hm. In order for me",
                                        "to forge the Executioner,",
                                        "I need you to bring me",
                                        "^0099FF2 Executioner's Gloves^000000."
                                    ],
                                )?;
                                return ctx.close();
                            }
                            if ctx.items().count(7024)? < 10 {
                                ctx.lines_as(
                                    "Mysterious Blacksmith",
                                    args![
                                        "In order to craft the",
                                        "blade, I need at least",
                                        "^0099FF10 Bloody Edges^000000. Find",
                                        "those as quickly as you",
                                        "can and bring them to me."
                                    ],
                                )?;
                                return ctx.close();
                            }
                            if ctx.items().count(1008)? < 3 {
                                ctx.lines_as(
                                    "Mysterious Blacksmith",
                                    args![
                                        "In order to enchant",
                                        "the Executioner with",
                                        "its dark power, I will",
                                        "need you to bring me",
                                        "^0099FF3 Necklaces of Oblivion^000000.",
                                        "Go, hurry and get them!"
                                    ],
                                )?;
                                return ctx.close();
                            }
                            if ctx.items().count(609)? < 50 {
                                ctx.lines_as(
                                    "Mysterious Blacksmith",
                                    args![
                                        "Hm. You've forgotten to",
                                        "bring me ^0099FF50 Amulets^000000. I need",
                                        "those in order to imbue the",
                                        "incredible energies that",
                                        "give the Executioner its",
                                        "monstrous strength..."
                                    ],
                                )?;
                                return ctx.close();
                            }
                            if ctx.items().count(714)? < 1 {
                                ctx.lines_as(
                                    "Mysterious Blacksmith",
                                    args![
                                        "You have everything I need",
                                        "to forge the Executioner, but",
                                        "I'm still unsure of whether you",
                                        "are capable of handling its",
                                        "power. My fears will be allayed",
                                        "if you bring ^0099FF1 Emperium^000000."
                                    ],
                                )?;
                                return ctx.close();
                            }
                            if ctx.items().count(7017)? > 1
                                && ctx.items().count(7024)? > 9
                                && ctx.items().count(1008)? > 2
                                && ctx.items().count(609)? > 49
                                && ctx.items().count(714)? > 0
                            {
                                ctx.lines_as(
                                    "Mysterious Blacksmith",
                                    args![
                                        "Great, I see that you've",
                                        "prepared everything that",
                                        "I asked. Give me a moment",
                                        "while I summon the dark",
                                        "forces required to forge",
                                        "the Executioner..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Mysterious Blacksmith",
                                    args![
                                        "It's been a long time",
                                        "since I've seen this",
                                        "terrifying sword. Now",
                                        "take it! Be wary, and don't",
                                        "let its bloodlust consume you!"
                                    ],
                                )?;
                                ctx.items().take(7017, 2)?;
                                ctx.items().take(7024, 10)?;
                                ctx.items().take(1008, 3)?;
                                ctx.items().take(609, 50)?;
                                ctx.items().give(1169, 1)?;
                                return ctx.close();
                            }
                        } else if q2_has(ctx, 4)? {
                            ctx.lines_as(
                                "Mysterious Blacksmith",
                                args![
                                    "So you're telling me that",
                                    "you want a sword that's been",
                                    "cursed by all the souls that've",
                                    "died from the thousands of death sentences it has carried out?",
                                    "Ha ha ha! Very interesting!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Mysterious Blacksmith",
                                args![
                                    "Yes... Each time this",
                                    "sword cut down a guilty",
                                    "criminal, the blade was",
                                    "cursed by the rage and",
                                    "regret of the slain."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Mysterious Blacksmith",
                                args![
                                    "Over time, the Executioner",
                                    "accumulated a truly dreadful",
                                    "power from the hatred of those",
                                    "it had killed. However, he who",
                                    "uses this sword risks becoming consumed by insanity and hatred."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Mysterious Blacksmith",
                                args![
                                    "The last person for whom",
                                    "I created this sword asked",
                                    "me to destroy it, fearing that",
                                    "it would corrupt his mind with",
                                    "its bloodlust. But if you think you can endure, I may forge it..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Mysterious Blacksmith",
                                args![
                                    "Just bring me...",
                                    "^0099FF50 Amulets^000000,",
                                    "^0099FF1 Emperium^000000,",
                                    "^0099FF2 Executioner's Gloves^000000,",
                                    "^0099FF10 Bloody Edges^000000 and",
                                    "^0099FF3 Necklaces of Oblivion^000000."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Mysterious Blacksmith",
                                args![
                                    "Those are the items I need",
                                    "to create a sword imbued with",
                                    "evil power. As for the Emperium, consider it my test to see if you",
                                    "are truly capable of wielding",
                                    "the accursed Executioner..."
                                ],
                            )?;
                            q2_add(ctx, 4)?;
                            return ctx.close();
                        }
                        ctx.lines_as(
                            "Mysterious Blacksmith",
                            args![
                                "Hmpf. I don't think",
                                "you understand enough",
                                "about the Executioner to",
                                "ask me to forge it. You better",
                                "learn more about that accursed",
                                "sword if you really want it..."
                            ],
                        )?;
                        return ctx.close();
                    }
                    if !matched4 && subject4 == 4 {
                        matched4 = true;
                    }
                    if matched4 {
                        ctx.lines_as("Mysterious Blacksmith", args!["Hmm...?", "That a fact?"])?;
                        return ctx.close();
                    }
                }
            }
            if !matched2 && subject2 == 3 {
                matched2 = true;
            }
            if matched2 {
                ctx.lines_as(
                    "Mysterious Blacksmith",
                    args![
                        "Hmpf...",
                        "Well then, come",
                        "back when you know",
                        "exactly what you want",
                        "from me, adventurer."
                    ],
                )?;
                return ctx.close();
            }
        }
    }
    Ok(())
}

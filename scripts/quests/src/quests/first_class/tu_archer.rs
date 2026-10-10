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

pub fn bard_jet_tu(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Jet",
        args![
            "Every god never grows old",
            "Because of beautiful",
            "Goddess, Idun.",
            "Keeper of the apples of youth",
            "Goddess of immortality."
        ],
    )?;
    ctx.next()?;
    ctx.mes("[Jet]")?;
    if ctx.var("BaseJob").get()? == constants::JOB_ARCHER {
        ctx.lines(args![
            "Ooh, you're an Archer?",
            "There was a time when I too",
            "was an Archer. But I've changed jobs, so now I entertain the masses with my songs and humor."
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Jet",
            args![
                "Speaking of which...",
                "Let me tell you a joke!",
                "Trust me, it's hilarious~",
                "^333333*Ahem*^000000"
            ],
        )?;
        ctx.call(Function::Emotion, args![constants::ET_SURPRISE])?;
        ctx.next()?;
        ctx.lines_as(
            "Jet",
            args![
                "A termite walks",
                "into a bar and says,",
                "'Is the bar tender here?'",
                "Hahahahaha!",
                "Bwehehehehe!"
            ],
        )?;
        ctx.call(Function::Emotion, args![constants::ET_THROB])?;
        ctx.next()?;
        ctx.lines_as(
            "Jet",
            args![
                "Hahaha~!",
                "Goodness,",
                "I'm funny!",
                "Now, you try!",
                "Come on, make",
                "up a joke~!"
            ],
        )?;
        ctx.call(Function::Emotion, args![constants::ET_KIK])?;
        ctx.next()?;
        if ctx.var("Sex").get()? == constants::SEX_MALE {
            let choice = runtime::select_values(ctx, &[Val::from("A joke, eh?")])?;
            ctx.var("@menu").set(choice)?;
            ctx.call(Function::SpecialEffect, args![constants::EF_TALK_FROSTJOKE])?;
            ctx.lines_as("Jet", args!["Oh...", "My...", "God..."])?;
            ctx.next()?;
            ctx.lines_as(
                "Jet",
                args![
                    "That was the greatest",
                    "joke I've ever heard in my",
                    "life! You really have a natural",
                    "talent for showmanship! You",
                    "just have to become a Bard!"
                ],
            )?;
        } else {
            let choice = runtime::select_values(ctx, &[Val::from("Scream!")])?;
            ctx.var("@menu").set(choice)?;
            ctx.call(Function::SpecialEffect, args![constants::EF_TALK_SCREAM])?;
            ctx.lines_as(
                "Jet",
                args![
                    "W-Wow...",
                    "I don't know why, but that was truly amazing. You must have",
                    "what it takes to become a Dancer..."
                ],
            )?;
        }
        ctx.call(Function::Emotion, args![constants::ET_BEST])?;
        ctx.next()?;
        ctx.lines_as(
            "Jet",
            args!["But before any of that, you've got to get rid of the Archer look, no?"],
        )?;
        ctx.next()?;
        ctx.lines_as("Jet", args!["Actually I'm a member of", "the ^3131FFArcher's Guild^000000 'Icarus' which is to the ^3131FFleft of the job change office^000000. You should be able to", "learn a lot from there."])?;
        ctx.next()?;
        ctx.lines_as(
            "Jet",
            args![
                "Now now, go learn",
                "everything and change",
                "jobs so that you can",
                "come play with me!"
            ],
        )?;
    } else if ctx.var("BaseJob").get()? == constants::JOB_BARD {
        ctx.lines(args!["Ooh...!", "You're...!"])?;
        ctx.next()?;
        ctx.call(Function::SpecialEffect, args![constants::EF_TALK_FROSTJOKE])?;
        ctx.call(Function::Emotion, args![constants::ET_BEST])?;
        ctx.lines_as(
            "Jet",
            args![
                "Let's see...",
                "I've been working",
                "on a new joke here,",
                "but I could use some",
                "feedback. You ready?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Jet",
            args!["What did the male", "Merchant say to the", "dancing Isis? Give up?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Jet",
            args![
                "'Mammonite!'",
                "Get it? 'Cuz he",
                "throws money at her",
                "and the word Mammonite",
                "sounds-- wait. It's not funny",
                "if I have to explain it, huh?"
            ],
        )?;
    } else if ctx.var("BaseJob").get()? == constants::JOB_DANCER {
        ctx.lines(args![
            "Ooh...!",
            "Hello, hello!",
            "Wouldn't it be",
            "nice if we could",
            "perform together",
            "one of these days?"
        ])?;
    } else {
        ctx.lines(args![
            "You look bored...",
            "Is life really that",
            "mundane and tiresome?",
            "Then... I shall make",
            "you laugh..."
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Jet",
            args![
                "^333333*Ahem*^000000",
                "These are the",
                "famous last words",
                "of a mafia hit man..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Jet",
            args![
                "'Who put the",
                "violin in this",
                "violin case?!'",
                "Bwahahahahahahah!",
                "Get the joke?"
            ],
        )?;
    }
    return ctx.close();
}

pub fn sign_arc(ctx: &Ctx) -> Script {
    ctx.lines(args![" ", " Archer Job Change Office ---> ", " <--- Icarus ", " "])?;
    return ctx.close();
}

pub fn master_kavaruk(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Master Kavaruk",
        args![
            "Hello, young one.",
            "I am Master Kavaruk",
            "of the Icarus Archer Guild.",
            "I bid you welcome."
        ],
    )?;
    ctx.next()?;
    'b1: {
        let subject1 = Val::from(runtime::select_values(
            ctx,
            &[Val::from("Ask about 'Icarus.':Ask about recent news.:Talk about Archers.")],
        )?);
        let mut matched1 = false;
        let no_case1 =
            !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2)) && !subject1.loosely_equals(&Val::from(3));
        if !matched1 && subject1.loosely_equals(&Val::from(1)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as("Master Kavaruk", args!["Have you ever", "heard the story", "of Icarus before?"])?;
            ctx.next()?;
            ctx.lines_as("Master Kavaruk", args!["Icarus was a brave young man who flew through the sky using wings held together by beeswax. Out of curiosity, he flew too close to the sun, his wings melted, and he fell to his death."])?;
            ctx.next()?;
            ctx.lines_as(
                "Master Kavaruk",
                args![
                    "Although his is a tragic story, Icarus had a courageous heart",
                    "that we should look up to as",
                    "our standard."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Master Kavaruk", args!["The members of our guild are those who not only wish to be brave, but to reach out for our dreams like Icarus did. Our members are aspiring Archers, former Archers and current Archers."])?;
            ctx.next()?;
            ctx.mes("[Master Kavaruk]")?;
            if ctx.var("Class").get()? == constants::JOB_NOVICE {
                ctx.lines(args![
                    "If you'd like to become an Archer, why don't you go around and learn",
                    "a few things about the profession? I'm sure you'll be able to hear plenty of stories around here."
                ])?;
            } else if ctx.var("Class").get()? == constants::JOB_ARCHER {
                ctx.lines(args!["Since you're an Archer, you might", "benefit from going around and listening to everyone else's battle experiences. Becoming familiar with different fight scenarios will help you in the future."])?;
            } else if ctx.var("BaseJob").get()? == constants::JOB_HUNTER {
                ctx.lines_as(
                    "Master Kavaruk",
                    args![
                        "Arpesto is waiting",
                        "outside. Since he's a",
                        "veteran hunter, he can",
                        "be of great help to you."
                    ],
                )?;
            } else if ctx.var("BaseJob").get()? == constants::JOB_DANCER {
                ctx.lines(args![
                    "In fact, a few of our",
                    "members specialize in",
                    "capturing the fascination",
                    "of those watching with their",
                    "graceful dances, just like you."
                ])?;
                ctx.next()?;
                ctx.lines_as(
                    "Master Kavaruk",
                    args![
                        "...But I don't seem to see",
                        "any of them around here right now. They never seem to remain in one",
                        "place for very long. Still, I'm sure of them will turn up here",
                        "one of these days."
                    ],
                )?;
            } else if ctx.var("BaseJob").get()? == constants::JOB_BARD {
                ctx.lines(args![
                    "In fact, a few of our members",
                    "are highly skilled in singing and entertaining their allies, much in the same way you do."
                ])?;
                ctx.next()?;
                ctx.lines_as("Master Kavaruk", args!["...But I don't seem to see any of them around here right now. They always wander wherever the wind takes them, but I'm sure one of them will wander back here one", "of these days."])?;
            } else {
                ctx.lines(args!["They happen to know a lot of information about jobs related to Archers. Why don't you speak to them and learn more about", "these other jobs?"])?;
            }
            return ctx.close();
        }
        if !matched1 && subject1.loosely_equals(&Val::from(2)) {
            matched1 = true;
        }
        if matched1 {
            ctx.mes("[Master Kavaruk]")?;
            if ctx.var("BaseJob").get()? == constants::JOB_ARCHER {
                if ctx.var("tu_archer02").get()? == 0 {
                    ctx.lines(args![
                        "Oh, it's a good thing",
                        ((Val::from("you're here, ") + ctx.player().name()?) + Val::from(".")),
                        "I've just received request for support from the Alchemist",
                        "Guild in Al De Baran."
                    ])?;
                    ctx.next()?;
                    ctx.lines_as("Master Kavaruk", args!["They've been taking care", "of a Flora field as a way of supplying their funds, but recently some stray cats seem to have been destroying the plants there."])?;
                    ctx.next()?;
                    ctx.lines_as("Master Kavaruk", args!["Although the Alchemists have been using Alchemy to summon the Flora again to keep the field running, it's causing a strain on their resources."])?;
                    ctx.next()?;
                    ctx.lines_as("Master Kavaruk", args!["They are willing to give rewards for bringing materials such as ^3131FFManeater Blossoms^000000 and ^3131FFStems^000000 which will help them restore", "their Flora."])?;
                    ctx.next()?;
                    ctx.lines_as("Master Kavaruk", args!["If you're interested, it would probably be best to speak to the Alchemist Guild member who is waiting to hear from us at the ^3131FFshop next to the road south of Icarus^000000."])?;
                    ctx.var("tu_archer02").set(Val::from(1))?;
                    return ctx.close();
                } else if ctx.var("tu_archer02").get()? == 1 {
                    ctx.lines(args!["Hmmm...", ".........."])?;
                    ctx.next()?;
                    let choice = runtime::select_values(ctx, &[Val::from("Is something the matter?")])?;
                    ctx.var("@menu").set(choice)?;
                    ctx.lines_as("Master Kavaruk", args!["Actually, we have a problem.", "The position of master seems to have a lot of power, but I always need to remain here. I don't have the luxury of being able to leave my post."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Master Kavaruk",
                        args![
                            "I asked a member of Icarus to",
                            "wander about the Rune-Midgarts Kingdom and gather news and information about what has been going on recently."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Master Kavaruk", args!["He has been sending me letters containing detailed information, but all of a sudden, I've lost contact with him. I'm concerned over what may have happened."])?;
                    ctx.next()?;
                    ctx.lines_as("Master Kavaruk", args!["Would you please go find", "^3131FFArthail of the Wind^000000 for me? Although he doesn't like people, I've ordered for him to wander within the crowds to gather information."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Master Kavaruk",
                        args![
                            "Please find out if he is all right and help him with whatever he",
                            "may need. In the meantime, I will be waiting to hear from you. Thank you very much."
                        ],
                    )?;
                    ctx.var("tu_archer02").set(Val::from(2))?;
                    return ctx.close();
                } else if ctx.var("tu_archer02").get()? == 2 {
                    ctx.lines(args![
                        "Find Arthail",
                        "of the Wind for me.",
                        "He must be somewhere",
                        "in Prontera..."
                    ])?;
                    return ctx.close();
                } else if ctx.var("tu_archer02").get()? == 9 {
                    ctx.lines(args![
                        "Hmmm, I see. Thank you",
                        "for bringing me the news. As Arthail has said, I shall wait until he has more news for me.",
                        "You should also train and prepare for the future as well."
                    ])?;
                    ctx.var("tu_archer02").set(Val::from(10))?;
                    ctx.call(Function::GetExperience, args![2000, 1000])?;
                    return ctx.close();
                }
            }
            ctx.lines(args![
                "I don't know...",
                "Recently, I haven't heard any noteworthy news. For now, the warmth of the sun seems to be protecting the peace here."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Master Kavaruk",
                args!["But if I hear of", "any interesting rumors,", "I will let you know."],
            )?;
            return ctx.close();
        }
        if !matched1 && subject1.loosely_equals(&Val::from(3)) {
            matched1 = true;
        }
        if matched1 {
            ctx.mes("[Master Kavaruk]")?;
            if ctx.var("tu_archer01").get()?.number()? < 1 {
                if (ctx.var("Class").get()? == constants::JOB_NOVICE || ctx.var("Class").get()? == constants::JOB_BABY) {
                    ctx.lines(args!["Ah...", "So are you", "interested in", "becoming an Archer?"])?;
                } else if ((ctx.var("Class").get()? == constants::JOB_ARCHER_HIGH || ctx.var("BaseJob").get()? == constants::JOB_DANCER)
                    || ctx.var("BaseJob").get()? == constants::JOB_BARD)
                {
                    ctx.mes("Although I can't deny that you're an expert, I suppose it is a good idea to review the basics of archery from time to time. Would you like to try some refresher training?")?;
                } else if ctx.var("BaseJob").get()? == constants::JOB_HUNTER {
                    ctx.lines_as(
                        "Master Kavaruk",
                        args![
                            "Arpesto is waiting",
                            "outside. Since he's a",
                            "veteran hunter, he can",
                            "be of great help to you."
                        ],
                    )?;
                    return ctx.close();
                } else if (ctx.var("Class").get()? == constants::JOB_ARCHER || ctx.var("Class").get()? == constants::JOB_BABY_ARCHER) {
                    ctx.lines(args![
                        "As an Archer, I can see that you'd want to make sure that you have a strong grasp on the fundamentals",
                        "of our job. Would you like some specialized instruction?"
                    ])?;
                } else {
                    ctx.lines(args![
                        "Although you may not be able",
                        "to directly apply archery related knowledge, it's a good idea to understand the capabilities of",
                        "your Archer allies."
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Master Kavaruk",
                        args![
                            "How would you like",
                            "some specialized instruction to learn more about the Archer job?"
                        ],
                    )?;
                }
                ctx.next()?;
                match ctx.menu(&["Yes", "No"])? {
                    0 => {
                        ctx.var("tu_archer01").set(Val::from(1))?;
                        ctx.lines_as(
                            "Master Kavaruk",
                            args![
                                "Ah yes. Well then, please",
                                "take this message over to ^3131FFSeisner^000000 who is in the Training Grounds west of Icarus."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines(args!["^3355FFYou've received", "Kavaruk's message", "for Seisner.^000000"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Master Kavaruk",
                            args![
                                "Seisner is very diligent,",
                                "composed, intelligent and",
                                "extremely capable. I believe",
                                "that you can learn much from her."
                            ],
                        )?;
                        return ctx.close();
                    }
                    1 => {
                        ctx.lines_as(
                            "Master Kavaruk",
                            args![
                                "Ah, I see. Well,",
                                "Icarus will always welcome",
                                "you with open arms. If you",
                                "ever change your mind, feel",
                                "free to come back."
                            ],
                        )?;
                        return ctx.close();
                    }
                    _ => {}
                }
            } else if (ctx.var("tu_archer01").get()? == 1 || ctx.var("tu_archer01").get()? == 2) {
                ctx.lines(args![
                    "Hmm...?",
                    "What are you still",
                    "doing here? Go to the",
                    "hills west of Icarus and",
                    "talk to ^3131FFSeisner^000000 to begin your special training."
                ])?;
                return ctx.close();
            } else if ctx.var("tu_archer01").get()? == 3 {
                ctx.lines(args![
                    ((Val::from("So ") + ctx.player().name()?) + Val::from(",")),
                    "Did you have a good experience? Learning theory alone is never too enjoyable, but it is necessary."
                ])?;
                ctx.next()?;
                ctx.mes("[Master Kavaruk]")?;
                if ctx.var("BaseJob").get()? == constants::JOB_ARCHER {
                    ctx.mes("Now it is time to experience the principles you've just learned firsthand. Go speak to ^3131FFReidin Corse^000000, who is just outside of this building, and he'll tell you about the different skills.")?;
                    ctx.var("tu_archer01").set(Val::from(4))?;
                } else {
                    ctx.lines(args![
                        "Still, there's no need for you to do any field training since only Archers can actually participate",
                        "in that."
                    ])?;
                }
                return ctx.close();
            } else if (ctx.var("tu_archer01").get()? == 4 && ctx.var("BaseJob").get()? == constants::JOB_ARCHER) {
                ctx.mes("Reidin Corse is just outside of this building. Didn't you see him on your way in?")?;
                return ctx.close();
            }
        }
    }
    ctx.lines(args![
        "You're great",
        "just as you are now.",
        "Still, if you feel like you need something, speak to any of the guild members and listen to their life experiences."
    ])?;
    return ctx.close();
}

pub fn reidin_corse_tu(ctx: &Ctx) -> Script {
    let mut l_eagle = Val::from(0);
    let mut l_owl = Val::from(0);
    let mut l_skill_owl = Val::from(0);
    ctx.mes("[Reidin Corse]")?;
    if ctx.call(Function::CheckWeight, args![1201, 1])? == 0 {
        ctx.lines_as(
            "Reidin Corse",
            args![
                "Why are you carrying",
                "so much stuff? You better put everything you don't need into Kafra Storage."
            ],
        )?;
        return ctx.close();
    }
    if ctx.var("MaxWeight").get()?.number()? - ctx.var("Weight").get()?.number()? < 2000 {
        ctx.lines(args![
            "Hey, you're carrying an",
            "awful lot of stuff. You ought",
            "to put some of your things",
            "into Kafra Storage..."
        ])?;
        return ctx.close();
    }
    if (ctx.var("tu_archer01").get()?.number()? > 0
        && ((((ctx.var("BaseJob").get()? != constants::JOB_HUNTER) && (ctx.var("BaseJob").get()? != constants::JOB_BARD))
            && (ctx.var("BaseJob").get()? != constants::JOB_DANCER))
            || ctx.var("Upper").get()? == 2))
    {
        if ctx.var("tu_archer01").get()? == 4 {
            ctx.lines(args!["Ah, what is it?", "Can I help you", "with something?"])?;
            ctx.next()?;
            match ctx.menu(&["Tell me about skills.", "Er, not really..."])? {
                0 => {
                    ctx.lines_as(
                        "Reidin Corse",
                        args!["Huh. So Master", "Kavaruk told you", "to come to me, eh?", "Alright. Okay."],
                    )?;
                    ctx.call(Function::Emotion, args![constants::ET_KIK])?;
                    ctx.next()?;
                }
                1 => {
                    ctx.lines_as(
                        "Reidin Corse",
                        args![
                            "Huh...?",
                            "Alright. You sure",
                            "you've got nothing",
                            "to ask me? You had",
                            "that look, you know?"
                        ],
                    )?;
                    return ctx.close();
                }
                _ => {}
            }
            ctx.lines_as(
                "Reidin Corse",
                args![
                    "Fine. I'm an incredibly",
                    "busy guy, but I'll make time for you. I'll teach you what I know about Archer skills... On one condition!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Reidin Corse",
                args![
                    "From here on out, you gotta",
                    "call me ^3131FFChief^000000, got it? Stick with me and you'll become the second best archer in the world! How",
                    "about it, kid?"
                ],
            )?;
            ctx.next()?;
            match ctx.menu(&["You got it, Chief!", "Ugh, no thanks~!"])? {
                0 => {
                    ctx.lines_as(
                        "Reidin Corse",
                        args![
                            "Ho ho!",
                            "That's the way to go!",
                            "Good good~ Call me Chief",
                            "from now on, you hear?"
                        ],
                    )?;
                    ctx.next()?;
                }
                1 => {
                    ctx.lines_as(
                        "Reidin Corse",
                        args![
                            "Huh? What kind of attitude",
                            "is that? Ah, I get it. You're not mature enough to recognize greatness when it's right",
                            "before you."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Reidin Corse",
                        args!["Don't worry...", "When you come crawlin'", "back, I'll reconsider", "teaching you."],
                    )?;
                    return ctx.close();
                }
                _ => {}
            }
            ctx.lines_as("Reidin Corse", args!["Let's see...", "How much do you know?", "Mmmmmmmmmm..."])?;
            ctx.next()?;
            ctx.mes("^3355FFReidin Corse narrows his experienced eyes and sizes you up.^000000")?;
            ctx.next()?;
            ctx.mes("[Reidin Corse]")?;
            if ctx.player().job_level()? > 39 {
                ctx.mes("Hey! You're ready to change jobs, aren't you? Don't waste time here and just get a new job already! Eh, or you can ask Master Kavaruk for something to do.")?;
            } else if ctx.player().job_level()? < 5 {
                ctx.lines(args!["Alright! We got a fresh Archer here! You're lucky you came to me, I'm the best teacher you can find! But let me warn you, I teach at a really fast pace, so try to keep", "up. Okay? Good."])?;
                ctx.next()?;
                ctx.var("tu_archer01").set(Val::from(5))?;
                ctx.lines_as("Reidin Corse", args!["Okay, I'm ready to begin the lessons! Come back over here once you've got your bow and arrows and everything else ready, got it?"])?;
            } else {
                ctx.lines(args![
                    "Okay. It looks like you know",
                    "some stuff. But even if you're",
                    "a little experienced, there's still room for you to learn. So try and keep up, got it?"
                ])?;
                ctx.next()?;
                ctx.var("tu_archer01").set(Val::from(5))?;
                ctx.lines_as("Reidin Corse", args!["Well, I'm ready to teach whenever you're ready to learn. Come back once your bow and arrows and everything else is ready, alright?"])?;
            }
            return ctx.close();
        } else if ctx.var("tu_archer01").get()? == 5 {
            ctx.lines(args![
                "You ready? I think it's fair to remind you that I won't tolerate any complaining! Just do what",
                "I say and you'll be the second best Archer in the world! After me, of course~"
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Reidin Corse",
                args![
                    "If you don't like it, you can just quit! No reason to stick around",
                    "if you can't keep up with the Chief, anyway."
                ],
            )?;
            ctx.next()?;
            match ctx.menu(&["Please teach me... Chief!", "I want to quit."])? {
                0 => {
                    ctx.lines_as("Reidin Corse", args!["That's what I'm talkin' about! Yeah! Just trust me and you'll learn almost everything about Archer skills! Let's get started!"])?;
                    ctx.next()?;
                    let choice = runtime::select_values(ctx, &[Val::from("Yes, Chief!")])?;
                    ctx.var("@menu").set(choice)?;
                    ctx.call(Function::Emotion, args![constants::ET_OK])?;
                    ctx.lines_as("Reidin Corse", args!["Great...!", "I like your style!"])?;
                    ctx.next()?;
                    ctx.lines_as("Reidin Corse", args!["Now, open your Skill Window (Alt+S). You oughta be able to see two skills: ^FF0000Owl's Eye^000000 and ^FF0000Double Strafe^000000. First, I'll talk about Owl's Eye."])?;
                    ctx.next()?;
                    ctx.lines_as("Reidin Corse", args!["We Archers need to be able to preceive the movements of our targets from far away. Now, even during the night, nothing can escape the eyes of an owl."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Reidin Corse",
                        args![
                            "So we gotta train our vision so that we see the way owls do. Now",
                            "if you can't aim at your targets, you'd be horrible at archery, right? You gotta have Accuracy!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.mes("[Reidin Corse]")?;
                    l_skill_owl = ctx.call(Function::GetSkillLv, args!["AC_OWL"])?;
                    if l_skill_owl.number()? < 3 {
                        ctx.var("tu_archer01").set(Val::from(6))?;
                        ctx.lines(args![
                            "Now, your first assignment",
                            "is to learn ^3131FFLevel 3 Owl's Eye^000000!"
                        ])?;
                        ctx.mes("From Prontera, if you travel south, south and then west, you can fight Condors for your training.")?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Reidin Corse",
                            args![
                                "You can head somewhere",
                                "else if you want, but if you don't know any training areas, then",
                                "just follow my great advice."
                            ],
                        )?;
                        ctx.next()?;
                        'b4: {
                            let subject4 = Val::from(runtime::select_values(
                                ctx,
                                &[Val::from("I'll follow your advice, Chief!:I'll hunt somewhere else...")],
                            )?);
                            let mut matched4 = false;
                            let no_case4 = !subject4.loosely_equals(&Val::from(1)) && !subject4.loosely_equals(&Val::from(2));
                            if !matched4 && subject4.loosely_equals(&Val::from(1)) {
                                matched4 = true;
                            }
                            if matched4 {
                                ctx.mes("[Reidin Corse]")?;
                                ctx.lines(args![
                                    "I'll send you to",
                                    "Prontera for now,",
                                    "so don't forget the",
                                    "directions I gave you.",
                                    "Travel ^3131FFsouth^000000, ^3131FFsouth^000000,",
                                    "and then ^3131FFwest^000000."
                                ])?;
                                ctx.next()?;
                                match ctx.menu(&["Leave right away~", "W-wait, let me get ready!"])? {
                                    0 => {
                                        ctx.lines_as(
                                            "Reidin Corse",
                                            args![
                                                "When you struggle",
                                                "through the hardships",
                                                "of training, just think",
                                                "about trying to become as",
                                                "good as the greatest Archer",
                                                "ever: me."
                                            ],
                                        )?;
                                        ctx.call(Function::Emotion, args![constants::ET_THINK])?;
                                        ctx.next()?;
                                        ctx.lines_as("Reidin Corse", args!["Okay...!", "Off you go!"])?;
                                        ctx.mes("To Prontera!")?;
                                        ctx.close_window()?;
                                        ctx.warp("prontera", 116, 72)?;
                                        return ctx.end();
                                    }
                                    1 => {
                                        ctx.lines_as(
                                            "Reidin Corse",
                                            args![
                                                "Hm...?",
                                                "What do you possibly",
                                                "need to prepare? Alright,",
                                                "do whatever it is you",
                                                "need to do..."
                                            ],
                                        )?;
                                        return ctx.close();
                                    }
                                    _ => {}
                                }
                            }
                            if !matched4 && subject4.loosely_equals(&Val::from(2)) {
                                matched4 = true;
                            }
                            if matched4 {
                                ctx.call(Function::Emotion, args![constants::ET_FRET])?;
                                ctx.lines_as(
                                    "Reidin Corse",
                                    args![
                                        "Say what...?",
                                        "Alright, then.",
                                        "But the next time",
                                        "you come here, you better",
                                        "know Level 3 Owl's Eye!"
                                    ],
                                )?;
                                return ctx.close();
                            }
                        }
                    } else {
                        ctx.call(Function::Emotion, args![constants::ET_ANGER])?;
                        ctx.mes("Huh. So you already know a little about Owl's Eye already, huh? Well then, I guess I oughta talk about something you don't know about!")?;
                        ctx.var("tu_archer01").set(Val::from(7))?;
                        ctx.call(Function::GetExperience, args![500, 0])?;
                        return ctx.close();
                    }
                }
                1 => {
                    ctx.lines_as(
                        "Reidin Corse",
                        args![
                            "Huh. Okay.",
                            "And here I was",
                            "getting ready to",
                            "teach you all my",
                            "archery secrets!"
                        ],
                    )?;
                    return ctx.close();
                }
                _ => {}
            }
        } else if ctx.var("tu_archer01").get()? == 6 {
            l_owl = ctx.call(Function::GetSkillLv, args![43])?;
            if l_owl.number()? > 2 {
                ctx.lines(args!["Ah, you're", "finally back.", "You've kept me", "waiting for a while!"])?;
                ctx.next()?;
                ctx.call(Function::Emotion, args![constants::ET_SURPRISE])?;
                ctx.lines_as(
                    "Reidin Corse",
                    args![
                        "Ah, but your eyes look a lot sharper than they used to be.",
                        "Here, this is a small, special reward for you."
                    ],
                )?;
                ctx.call(Function::SpecialEffect, args![constants::EF_WIND])?;
                ctx.var("tu_archer01").set(Val::from(7))?;
                ctx.call(Function::GetExperience, args![500, 0])?;
                ctx.next()?;
                ctx.call(Function::Emotion, args![constants::ET_HNG])?;
                ctx.lines_as(
                    "Reidin Corse",
                    args![
                        "Not bad, but you really should master Owl's Eye. That means",
                        "you need to learn it all the way",
                        "up to Level 10!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Reidin Corse",
                    args![
                        "Now, I guess I'll",
                        "talk about a different",
                        "skill in my next lecture, kay?",
                        "Come back when you're ready",
                        "to learn something new~"
                    ],
                )?;
                return ctx.close();
            } else {
                ctx.lines(args![
                    "Hey! I told",
                    "you to learn",
                    "Level 3 Owl's Eye!",
                    "Try not to learn something",
                    "else by accident, okay?"
                ])?;
                return ctx.close();
            }
        } else if ctx.var("tu_archer01").get()? == 7 {
            ctx.lines(args![
                "Alright, open",
                "your Skill Window.",
                "Since you have Level 3",
                "Owl's Eye, now you oughta be",
                "able to see the Vulture's Eye",
                "skill in the list too."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Reidin Corse",
                args!["Now when you think", "about vultures, what", "exactly comes to mind?"],
            )?;
            ctx.next()?;
            match ctx.menu(&["Sharp, precise eyes.", "You, Chief."])? {
                0 => {
                    ctx.lines_as("Reidin Corse", args!["Exactly!"])?;
                }
                1 => {
                    ctx.call(Function::Emotion, args![constants::ET_BIGTHROB])?;
                    ctx.lines_as(
                        "Reidin Corse",
                        args![
                            "Hahahaha!",
                            "Well, I am famous",
                            "for my incredible",
                            "vision and precision!",
                            "^333333*Ahem*^000000 Anyway..."
                        ],
                    )?;
                }
                _ => {}
            }
            ctx.next()?;
            ctx.lines_as("Reidin Corse", args!["Vultures circle the sky, find a target on the ground, and then very swiftly swoop down and ensnare their prey in their claws."])?;
            ctx.next()?;
            ctx.call(Function::Emotion, args![constants::ET_THROB])?;
            ctx.lines_as("Reidin Corse", args!["No doubt about it.", "Vultures are awesome!"])?;
            ctx.next()?;
            ctx.lines_as("Reidin Corse", args!["The ^3131FFVulture's Eye^000000 skill allows", "you to train your vision to be like a vulture's. You'll increase your Attack Accuracy and can target enemies from further away."])?;
            ctx.next()?;
            ctx.call(Function::Emotion, args![constants::ET_STARE_ABOUT])?;
            ctx.lines_as(
                "Reidin Corse",
                args![
                    "I know that you can attack",
                    "from a distance now, but don't",
                    "be satisfied with just your current attack range. Learn Vulture's Eye so you can attack from even further distances."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Reidin Corse",
                args![
                    "Now for your",
                    "second assignment!",
                    "Go hunt Mandagora!",
                    "Mandagora don't move,",
                    "so they're perfect for target",
                    "practice from a distance!"
                ],
            )?;
            ctx.next()?;
            l_eagle = ctx.call(Function::GetSkillLv, args!["AC_VULTURE"])?;
            if l_eagle.number()? < 3 {
                ctx.mes("Alright, Mandagora usually live around Mt. Mjolnir. I can send you to Prontera, so just travel north, then east from there to find")?;
                ctx.mes("some of them.")?;
                ctx.next()?;
                ctx.lines_as(
                    "Reidin Corse",
                    args![
                        "Remember, the most",
                        "important thing is to level the Vulture's Eye skill and test the distance of your attack range."
                    ],
                )?;
                if ctx.var("MaxWeight").get()?.number()? - ctx.var("Weight").get()?.number()? < 1000 {
                    ctx.lines(args!["You've got plenty of arrows", "for that, so go for it!"])?;
                    ctx.var("tu_archer01").set(Val::from(8))?;
                } else {
                    ctx.next()?;
                    ctx.lines_as("Reidin Corse", args!["Here's a little something to encourage you. Since Mandagora are Earth property monsters, these Fire Arrows will work really well."])?;
                    ctx.var("tu_archer01").set(Val::from(8))?;
                    ctx.items().give(1752, 300)?;
                }
                ctx.next()?;
                ctx.lines_as("Reidin Corse", args!["Alright~", "Ready to go?"])?;
                ctx.next()?;
                match ctx.menu(&["Let's go!", "W-wait a minute!"])? {
                    0 => {
                        ctx.lines_as("Reidin Corse", args!["Alright...!", "Get a move on!"])?;
                        ctx.close_window()?;
                        ctx.warp("prontera", 116, 72)?;
                        return ctx.end();
                    }
                    1 => {
                        ctx.lines_as("Reidin Corse", args!["You still need", "to get ready?", "Hurry it up!"])?;
                        return ctx.close();
                    }
                    _ => {}
                }
            } else {
                ctx.lines(args!["Eh?", "Wait a minute..."])?;
                ctx.call(Function::Emotion, args![constants::ET_QUESTION])?;
                ctx.next()?;
                ctx.lines_as(
                    "Reidin Corse",
                    args![
                        "You already",
                        "learned Vulture's Eye!",
                        "Why did I spend so much time explaining about it? Well, I guess we're ready to move on to the next lesson..."
                    ],
                )?;
                ctx.var("tu_archer01").set(Val::from(9))?;
                ctx.items().give(1752, 500)?;
                return ctx.close();
            }
        } else if ctx.var("tu_archer01").get()? == 8 {
            l_eagle = ctx.call(Function::GetSkillLv, args!["AC_VULTURE"])?;
            if l_eagle.number()? < 3 {
                ctx.mes("Mandagora lives in the area north and east of Prontera. Learn up to Level 3 Vulture's Eye and get acquainted with your attack range.")?;
                ctx.next()?;
                ctx.lines_as("Reidin Corse", args!["Okay...", "Are you", "ready to go?"])?;
                ctx.next()?;
                match ctx.menu(&["Go.", "Wait a moment."])? {
                    0 => {
                        ctx.lines_as("Reidin Corse", args!["Alright...!", "Get a move on!"])?;
                        ctx.close_window()?;
                        ctx.warp("prontera", 116, 72)?;
                        return ctx.end();
                    }
                    1 => {
                        ctx.lines_as("Reidin Corse", args!["You still need", "to get ready?", "Hurry it up!"])?;
                        return ctx.close();
                    }
                    _ => {}
                }
            } else {
                ctx.lines(args!["Ah, so you've learned", "a little something about Vulture's Eye! What do you think about it now? Ah, and here's a little reward for you before I start the next lesson~"])?;
                ctx.var("tu_archer01").set(Val::from(9))?;
                ctx.items().give(1752, 500)?;
                return ctx.close();
            }
        } else if ctx.var("tu_archer01").get()? == 9 {
            ctx.lines(args![
                "Okay...",
                "Up till now we've",
                "studied Passive Skills.",
                "You know, Owl's Eye",
                "and Vulture's Eye."
            ])?;
            ctx.next()?;
            ctx.lines_as("Reidin Corse", args!["Even though you may not be doing anything, Passive Skills are always in effect. Now, it's time for me to teach you about an Active Skill", "for Archers. Ready?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Reidin Corse",
                args![
                    "Today I'll tell",
                    "you all about",
                    "^3131FFDouble Strafe^000000!",
                    "Whaddya think?",
                    "Exciting, yes?"
                ],
            )?;
            ctx.next()?;
            l_eagle = ctx.call(Function::GetSkillLv, args!["AC_DOUBLE"])?;
            if l_eagle.number()? > 2 {
                ctx.lines_as(ctx.player().name()?, args!["I...", "I already know", "about Double Strafe."])?;
                ctx.next()?;
                ctx.lines_as("Reidin Corse", args!["...", "......"])?;
                ctx.next()?;
                ctx.lines_as("Reidin Corse", args!["^333333*Sigh*^000000", "Next lesson..."])?;
                ctx.var("tu_archer01").set(Val::from(10))?;
                return ctx.close();
            } else {
                ctx.lines_as(
                    "Reidin Corse",
                    args!["Just like its name, Double Strafe allows you to attack enemies with your arrows twice in a row! Booyah~"],
                )?;
                ctx.next()?;
                ctx.lines_as("Reidin Corse", args!["As Double Strafe's level rises, its attack strength also increases. If you master this skill, you can do a great amount of damage to your enemies!"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Reidin Corse",
                    args![
                        "Hold your bow and arrow",
                        "like this, concentrate, aim,",
                        "then fire twice with all your",
                        "might! Practice Double Strafe",
                        "enough and you'll get used to it!"
                    ],
                )?;
                ctx.next()?;
                ctx.var("tu_archer01").set(Val::from(10))?;
                if l_eagle == 0 {
                    ctx.call(Function::SpecialEffect, args![constants::EF_WIND])?;
                    ctx.call(Function::GetExperience, args![0, 500])?;
                }
                ctx.call(Function::Emotion, args![constants::ET_BEST])?;
                ctx.next()?;
                ctx.lines_as("Reidin Corse", args!["Now, there's another skill known as ^FF0000Arrow Shower^000000. Where Double Strafe uses 2 Arrows, Arrow Shower fires a bunch of arrows at once."])?;
                ctx.next()?;
                ctx.lines_as("Reidin Corse", args!["Not even I can handle", "Arrow Shower all that well since it's really hard. Still, maybe if you trained more at it, you'd manage to pull it off?"])?;
                ctx.call(Function::Emotion, args![constants::ET_SMILE])?;
                return ctx.close();
            }
        } else if ctx.var("tu_archer01").get()? == 10 {
            ctx.mes("Now, I've taught you a lot of stuff and I wanna make sure you retain all of it. That's why I want you to take this midterm. Your mission: ^FF0000Attack the Rockers^000000!")?;
            ctx.call(Function::Emotion, args![constants::ET_BEST])?;
            ctx.next()?;
            ctx.lines_as(
                "Reidin Corse",
                args![
                    "There's a ship that",
                    "leads to Byalan Island in Izlude, Prontera's satellite city. Ride that ship to Byalan Island."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Reidin Corse", args!["In the center of that island is the entrance to a dungeon where a lot of Hydras live. Remember that it's a water dungeon, and that ^3131FFWater resists Fire^000000, ^3131FFbut is weak against Wind^000000."])?;
            ctx.next()?;
            ctx.lines_as(
                "Reidin Corse",
                args![
                    "Now go and bring ^3131FF10 Tentacles^000000",
                    "from hunting Hydra and ^3131FF1 Crystal Blue^000000, which you can get from hunting the Mushrooms there.",
                    "Bring all of that and you pass~"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Reidin Corse", args![((Val::from("It's easier to find Izlude if you pass by Prontera, so head towards the fields near Prontera. Alright, off you go, ") + ctx.player().name()?) + Val::from("~"))])?;
            ctx.var("tu_archer01").set(Val::from(11))?;
            return ctx.close();
        } else if ctx.var("tu_archer01").get()? == 11 {
            if ctx.items().count(962)? < 10 || ctx.call(Function::CountItem, args![991])? == 0 {
                ctx.lines(args![
                    "Hey...",
                    "You gotta bring",
                    "^3131FF10 Tentacles^000000 and",
                    "^3131FF1 Crystal Blue^000000 to pass my midterm! Go back to the dungeon on Byalan Island and get them!"
                ])?;
                return ctx.close();
            }
            if ctx.var("MaxWeight").get()?.number()? - ctx.var("Weight").get()?.number()? < 1000 {
                ctx.lines(args![
                    "Why are you carrying",
                    "so much stuff? You better put everything you don't need into Kafra Storage."
                ])?;
                return ctx.close();
            } else {
                ctx.lines(args!["Hey, you're back!", "Let's see now..."])?;
                ctx.next()?;
                ctx.call(Function::Emotion, args![constants::ET_BEST])?;
                ctx.lines_as("Reidin Corse", args!["Good! You pass.", "Don't forget that you gotta keep training and improve your skills. Before you know it, you'll be an expert almost as good as me!"])?;
                ctx.call(Function::SpecialEffect, args![constants::EF_WIND])?;
                ctx.var("tu_archer01").set(Val::from(12))?;
                ctx.call(Function::GetExperience, args![1000, 1000])?;
                ctx.items().give(1707, 1)?;
                return ctx.close();
            }
        } else if ctx.var("tu_archer01").get()? == 12 {
            ctx.lines(args![
                "Alright, now there's only",
                "one more skill you need to know about. Personally, I think this one is crucial for every Archer..."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Reidin Corse",
                args![
                    "Open your Skill Window.",
                    "Make sure that you learn",
                    "^FF0000Increase Concentration^000000.",
                    "Now, how can I describe",
                    "how to do it?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Reidin Corse", args!["I guess that first, you gotta quiet your mind and enter a state of complete concentration. Steadily draw your bowstring back. If there's even a millisecond of imbalance, you'll miss."])?;
            ctx.next()?;
            ctx.lines_as("Reidin Corse", args!["We use Increase Concentration", "to keep ourselves calm and focused to ^3131FFmaximize certain abilities and stats^000000. This skill enhances our performance as Archers immensely!"])?;
            ctx.next()?;
            ctx.lines_as(
                "Reidin Corse",
                args![
                    "You might not be able",
                    "to see it now, but as you become more skilled, you'll eventually recognize all the benefits of this skill."
                ],
            )?;
            ctx.var("tu_archer01").set(Val::from(13))?;
            return ctx.close();
        } else if ctx.var("tu_archer01").get()? == 13 {
            ctx.lines(args![
                "^333333*Sigh*^000000",
                "It was a pain in the",
                "ass, but we're finally",
                "done with all of your lessons."
            ])?;
            ctx.call(Function::Emotion, args![constants::ET_HNG])?;
            ctx.next()?;
            ctx.lines_as(
                "Reidin Corse",
                args![
                    "Ah, but it's not time for",
                    "you to relax yet! Since I've given you a midterm, it's only fitting that I also give you a final!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Reidin Corse",
                args![
                    "Surprised, huh?",
                    "Well, this test is a little harder, but I'll help you out so there's",
                    "no need for you to fret."
                ],
            )?;
            ctx.next()?;
            ctx.mes("[Reidin Corse]")?;
            ctx.lines(args![
                "Now, go to Mt. Mjolnir and hunt Floras. Your test will be to bring back ^3131FF5 Maneater Blossoms^000000 and",
                "^3131FF20 Stems^000000."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Reidin Corse",
                args![
                    "I know it's a little dangerous,",
                    "but I've got a pal who'll be there to help you. Of course, she's still in training, but..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Reidin Corse", args!["Oh...", "And one last thing.", "Take some of these."])?;
            ctx.next()?;
            ctx.mes("[Reidin Corse]")?;
            if ctx.var("MaxWeight").get()?.number()? - ctx.var("Weight").get()?.number()? < 2100 {
                ctx.lines(args![
                    "H-hey!",
                    "Why are you carrying",
                    "so much stuff? You better put everything you don't need into Kafra Storage."
                ])?;
                return ctx.close();
            } else {
                ctx.lines(args!["This stuff is just a little something to encourage you, so don't take it the wrong way! Now hurry up, get what I asked for,", "and come back!"])?;
                ctx.var("tu_archer01").set(Val::from(14))?;
                ctx.items().give(601, 10)?;
                ctx.items().give(501, 10)?;
                ctx.items().give(602, 1)?;
                return ctx.close();
            }
        } else if (ctx.var("tu_archer01").get()? == 14 || ctx.var("tu_archer01").get()? == 15) {
            if ctx.items().count(1032)? < 5 || ctx.items().count(905)? < 20 {
                ctx.lines(args![
                    "Remember, you",
                    "need to head over to",
                    "Mt. Mjolnir and get me",
                    "^3131FF5 Maneater Blossoms^000000",
                    "and ^3131FF20 Stems^000000."
                ])?;
                ctx.next()?;
                ctx.lines_as("Reidin Corse", args!["From Prontera, you'd get to Mt. Mjolnir by traveling ^3131FFnorth^000000, ^3131FFnorth^000000 and then ^3131FFeast^000000. But if you want,", "I can just send you there."])?;
                ctx.next()?;
                match ctx.menu(&["Go!", "W-Wait!"])? {
                    0 => {
                        ctx.lines_as(
                            "Reidin Corse",
                            args![
                                "Alright!",
                                "Oh, and if you get",
                                "the chance, give a hello to my Acolyte pal over there for me~"
                            ],
                        )?;
                        ctx.call(Function::Emotion, args![constants::ET_SMILE])?;
                        ctx.close_window()?;
                        ctx.call(Function::Warp, args!["mjolnir_11", 25, 221])?;
                        return ctx.end();
                    }
                    1 => {
                        ctx.lines_as(
                            "Reidin Corse",
                            args![
                                "Huh?",
                                "Um, sure. But",
                                "there's no reason to",
                                "get so nervous, even if",
                                "this my final exam for you."
                            ],
                        )?;
                        return ctx.close();
                    }
                    _ => {}
                }
            }
            ctx.lines(args![
                "Ha-ha~!",
                "You look more like",
                "a veteran than a rookie",
                "now! Yeap, I can see it",
                "in your eyes."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Reidin Corse",
                args!["Right, did you get a chance to meet my Acolyte pal, Mafra? She's pretty shy, but I hope you two got along."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Reidin Corse",
                args![
                    "I'm happy to tell",
                    "you that you've passed",
                    "my final exam! Here, take",
                    "this little reward!"
                ],
            )?;
            ctx.call(Function::SpecialEffect, args![constants::EF_WIND])?;
            ctx.var("tu_archer01").set(Val::from(16))?;
            ctx.call(Function::GetExperience, args![3000, 3000])?;
            ctx.items().give(1770, 500)?;
            ctx.next()?;
            ctx.lines_as(
                "Reidin Corse",
                args![
                    "Well, it's like you're a full fledged Archer now. Honestly,",
                    "I've got nothing more to teach you. Well, about archery, anyway."
                ],
            )?;
            ctx.call(Function::Emotion, args![constants::ET_SMILE])?;
            ctx.next()?;
            ctx.lines_as("Reidin Corse", args!["Well, if you talk to him, Master Kavaruk might have something for you to do. He happens to have me running around to doing errands", "for him too. That creep!"])?;
            ctx.call(Function::Emotion, args![constants::ET_HNG])?;
            ctx.next()?;
            ctx.lines_as("Reidin Corse", args![((Val::from("Well, ") + ctx.player().name()?) + Val::from(",")), "It'd make me really happy if you get to the point where you develop a true love for the art of archery. I'll see you sometime, okay?"])?;
            ctx.call(Function::Emotion, args![constants::ET_BEST])?;
            return ctx.close();
        } else if ctx.var("tu_archer01").get()? == 16 {
            ctx.lines(args![
                "I really like this town.",
                "There's so much lush greenery",
                "and the birds are always singing. Payon really is peaceful."
            ])?;
            ctx.next()?;
            ctx.lines_as("Reidin Corse", args!["Of course, I've got friends here that I can trust with my life. We always argue, but we all want what's best for each other."])?;
            ctx.call(Function::Emotion, args![constants::ET_HNG])?;
            ctx.next()?;
            ctx.lines_as(
                "Reidin Corse",
                args!["Wouldn't you", "agree that it's", "wonderful to have", "great friends?"],
            )?;
            return ctx.close();
        } else {
            ctx.lines(args![
                "I really like this town.",
                "Everything is green and birds singing all around.. such a peaceful place Payon is."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Reidin Corse",
                args!["I feel so happy laying in the grass and looking up at the sky."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Reidin Corse",
                args![
                    "It might also be because I have my trustable friends here with me. Haha.",
                    "We always argue, but we care for each other deep down inside."
                ],
            )?;
            ctx.call(Function::Emotion, args![constants::ET_HNG])?;
            ctx.next()?;
            ctx.lines_as("Reidin Corse", args!["..'friend' is a wonderful word. Don't you agree?"])?;
            return ctx.close();
        }
    }
    ctx.mes("Allow me to introduce myself. I'm the Archer of all Archers, ^3131FFReidin Corse^000000, Master of the Icarus Guild!")?;
    ctx.next()?;
    ctx.call(Function::Emotion, args![constants::ET_BEST])?;
    ctx.lines_as("Reidin Corse", args!["Right now, we're having a special event! The application fee for the Icarus Guild has been slashed by 50%! Join now and there'll be no annual fees!"])?;
    ctx.next()?;
    ctx.call(Function::Emotion, args![constants::ET_THROB])?;
    ctx.lines_as(
        "Reidin Corse",
        args![
            "During our special event, we'll have no job class restrictions!",
            "As an added bonus, all new male members will be introduced to beautiful Dancer girls!"
        ],
    )?;
    ctx.next()?;
    ctx.call(Function::Emotion, args![constants::ET_SMILE])?;
    ctx.lines_as(
        "Reidin Corse",
        args!["And new female members will have the special opportunity to go out on a date with me! Don't miss out!"],
    )?;
    ctx.next()?;
    ctx.call(Function::Emotion, args![constants::ET_MONEY])?;
    ctx.lines_as(
        "Reidin Corse",
        args![
            "Join right now",
            "for a one time fee of only ^3131FF1,000,000, zeny^000000! Hurry and join the Icarus Guild while",
            "this offer still lasts!"
        ],
    )?;
    ctx.next()?;
    ctx.call(
        Function::Emotion,
        args![
            constants::ET_HNG,
            ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Arpesto")])?
        ],
    )?;
    ctx.lines_as(
        "Arpesto",
        args![
            "Reidin...",
            "Stop messing around.",
            "Hey there, kid. The real",
            "master of the Icarus Guild",
            "is in that building."
        ],
    )?;
    ctx.next()?;
    ctx.call(Function::Emotion, args![constants::ET_HUK])?;
    ctx.lines_as("Reidin Corse", args!["Arpesto...!", "Why'd you...?!"])?;
    ctx.next()?;
    ctx.call(Function::Emotion, args![constants::ET_ANGER])?;
    ctx.lines_as(
        "Reidin Corse",
        args![
            "^333333*Sigh*^000000",
            "You got me, you got me.",
            "There's no application fee.",
            "But I'll still date all of",
            "those cute girls..."
        ],
    )?;
    return ctx.close();
}

pub fn seisner(ctx: &Ctx) -> Script {
    ctx.call(
        Function::NpcSpecialEffect,
        args![constants::EF_HIT2, constants::AREA, "#Target"],
    )?;
    ctx.lines_as("Seisner", args!["Aaaah!", "Double Strafe!"])?;
    ctx.call(
        Function::NpcSpecialEffect,
        args![constants::EF_HIT2, constants::AREA, "#Target"],
    )?;
    ctx.next()?;
    ctx.lines_as("Seisner", args!["I did it!", "Only an Archer", "could make that shot!"])?;
    ctx.next()?;
    ctx.lines_as(
        "Seisner",
        args![
            "Now, there are some",
            "Thief class people who also use arrows, but they're better suited to using Knives. We Archers are",
            "the arrow specialists!"
        ],
    )?;
    if (ctx.var("tu_archer01").get()? == 1 || ctx.var("tu_archer01").get()?.number()? > 2) {
        ctx.next()?;
        ctx.lines(args![
            "^3355FFYou show Seisner",
            "the letter that Master",
            "Kavaruk has written",
            "to her for you.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Seisner",
            args![
                "So Master Kavaruk",
                "wants me to teach you",
                "about Archers and Archery? Alright, what would you like to know more about?"
            ],
        )?;
        loop {
            ctx.next()?;
            match ctx.menu(&["About Archers.", "Stats for Archers", "End Conversation."])? {
                0 => {
                    ctx.lines_as("Seisner", args!["Archers specialize", "in shooting ^3131FFarrows^000000. We usually attack from a distance, but before we increase our Dodge Rate, we're pretty weak when it comes to one on one battles."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Seisner",
                        args![
                            "Therefore, Archers tend to attack before the enemy gets too close.",
                            "We also like to party with other people who can offer close",
                            "range protection."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Seisner",
                        args![
                            "Now, only Novices can",
                            "become Archers after reaching",
                            "Job Level 10 and learning all of the Basic Skills."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Seisner", args!["Well experienced Archers can move on and change to the Second Job Class, becoming ^3131FFHunters^000000, ^3131FFBards^000000 if they are male, or ^3131FFDancers^000000 if they are female."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Seisner",
                        args![
                            "If you reach Level 99",
                            "as a Second Job Class character, you can change to a Transcendent Class with Valkyrie's help."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Seisner", args!["Hunters can ultimately transcend into ^3131FFSnipers^000000, Bards into ^3131FFMinstrels^000000, and Dancers into ^3131FFGypsies^000000. I know that's pretty complex."])?;
                    if ctx.var("tu_archer01").get()? == 1 {
                        ctx.var("tu_archer01").set(Val::from(2))?;
                    }
                }
                1 => {
                    ctx.lines_as(
                        "Seisner",
                        args![
                            "So you want to",
                            "know more about Stats",
                            "and how they affect Archer abilities? I'll do my best to explain."
                        ],
                    )?;
                    ctx.next()?;
                    'l3: loop {
                        'b3: {
                            'b4: {
                                let subject4 = Val::from(runtime::select_values(
                                    ctx,
                                    &[Val::from(
                                        "STR:^3131FFAGI^000000:VIT:^FF9900INT^000000:^FF3131DEX^000000:LUK:End Conversation.",
                                    )],
                                )?);
                                let mut matched4 = false;
                                let no_case4 = !subject4.loosely_equals(&Val::from(1))
                                    && !subject4.loosely_equals(&Val::from(2))
                                    && !subject4.loosely_equals(&Val::from(3))
                                    && !subject4.loosely_equals(&Val::from(4))
                                    && !subject4.loosely_equals(&Val::from(5))
                                    && !subject4.loosely_equals(&Val::from(6))
                                    && !subject4.loosely_equals(&Val::from(7));
                                if !matched4 && subject4.loosely_equals(&Val::from(1)) {
                                    matched4 = true;
                                }
                                if matched4 {
                                    ctx.lines_as("Seisner", args!["STR typically increases the damage of attacks. However, Archer class characters use weapons that rely on ^FF3131DEX^000000 to increase their damage."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Seisner", args!["In a sense, STR might not be necessary for Archers, although I'm sure there are skills or situations where STR will come in handy."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Seisner", args!["For instance, if you run out of arrows (which is every Archer's nightmare) and need to equip a Knife, having some STR might", "be helpful."])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Seisner",
                                        args![
                                            "Increasing STR will also",
                                            "increase the amount of weight",
                                            "that any character can carry. So if you have more STR, you can carry around more arrows."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    if ctx.var("tu_archer01").get()? == 1 {
                                        ctx.var("tu_archer01").set(Val::from(2))?;
                                    }
                                    break 'b3;
                                }
                                if !matched4 && subject4.loosely_equals(&Val::from(2)) {
                                    matched4 = true;
                                }
                                if matched4 {
                                    ctx.lines_as(
                                        "Seisner",
                                        args![
                                            "^FF3131AGI^000000 as you know, ",
                                            "^3131Ffincreases Attack Speed",
                                            "(ASPD)^000000 and affects ^3131FF Dodge Rate^000000."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Seisner", args!["One of the drawbacks of being an Archer is having weaker strength and lower Dodge Rate. However, the Dodge Rate disadvantage is lessened by increasing your ^FF3131AGI^000000."])?;
                                    ctx.next()?;
                                    if ctx.var("tu_archer01").get()? == 1 {
                                        ctx.var("tu_archer01").set(Val::from(2))?;
                                    }
                                    break 'b3;
                                }
                                if !matched4 && subject4.loosely_equals(&Val::from(3)) {
                                    matched4 = true;
                                }
                                if matched4 {
                                    ctx.lines_as(
                                        "Seisner",
                                        args!["^3131FFVIT^000000 will increase", "your Maximum HP and", "resistance to damage."],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Seisner", args!["Although Archers don't", "specialize in having any special kind of defense, having VIT will help alleviate the defensive weakness of the Archer class."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Seisner", args!["VIT might be helpful", "depending on how you use it. Still, I wouldn't recommend focusing too much on increasing your VIT if you're an Archer."])?;
                                    ctx.next()?;
                                    if ctx.var("tu_archer01").get()? == 1 {
                                        ctx.var("tu_archer01").set(Val::from(2))?;
                                    }
                                    break 'b3;
                                }
                                if !matched4 && subject4.loosely_equals(&Val::from(4)) {
                                    matched4 = true;
                                }
                                if matched4 {
                                    ctx.lines_as(
                                        "Seisner",
                                        args![
                                            "^FF0000INT^000000 is more",
                                            "important to the",
                                            "Archer class than",
                                            "you would think~"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Seisner",
                                        args![
                                            "As you increase ^FF0000INT^000000, you'll raise your Maximum SP, meaning you",
                                            "can use special skills more often."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Seisner", args!["For Hunters, one of the Second Job Classes for Archers, having higher INT will increase the damage inflicted by ^3131FFFalcons^000000."])?;
                                    ctx.next()?;
                                    if ctx.var("tu_archer01").get()? == 1 {
                                        ctx.var("tu_archer01").set(Val::from(2))?;
                                    }
                                    break 'b3;
                                }
                                if !matched4 && subject4.loosely_equals(&Val::from(5)) {
                                    matched4 = true;
                                }
                                if matched4 {
                                    ctx.lines_as(
                                        "Seisner",
                                        args!["Now DEX is almost", "without a doubt the", "most important stat", "for any Archer."],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Seisner", args!["^FF3131DEX^000000 ^3131Ffincreases the damage^000000", "of Archer class weapons that fire arrows. These include ^3131FFBows^000000, ^3131FFInstruments^000000 and ^3131FFWhips^000000."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Seisner", args!["Dexterity enhances an Archer's ability to hit weak points and do more damage, which is why weapons that use arrows don't rely on STR like weapons used by other classes."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Seisner", args!["Just like any other class, ^FF3131DEX^000000 increases ^3131FFAccuracy (HIT)^000000 and slightly enhances Attack Speed (ASPD)."])?;
                                    ctx.next()?;
                                    if ctx.var("tu_archer01").get()? == 1 {
                                        ctx.var("tu_archer01").set(Val::from(2))?;
                                    }
                                    break 'b3;
                                }
                                if !matched4 && subject4.loosely_equals(&Val::from(6)) {
                                    matched4 = true;
                                }
                                if matched4 {
                                    ctx.lines_as(
                                        "Seisner",
                                        args!["^FF0000LUK^000000 affects your", "Perfect Dodge rate", "and Critical rate."],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Seisner", args!["For Hunters, one of the Second Classes for the Archer, LUK increases the chance of ^3131FFBlitz Beat^000000 occurring automatically. So Hunters who rely on their Falcons might want to invest in LUK."])?;
                                    ctx.next()?;
                                    if ctx.var("tu_archer01").get()? == 1 {
                                        ctx.var("tu_archer01").set(Val::from(2))?;
                                    }
                                    break 'b3;
                                }
                                if !matched4 && subject4.loosely_equals(&Val::from(7)) {
                                    matched4 = true;
                                }
                                if matched4 {
                                    ctx.lines_as("Seisner", args!["So is there", "anything else that", "you want to ask me?"])?;
                                    break 'b4;
                                }
                            }
                            break 'l3;
                        }
                    }
                }
                2 => {
                    ctx.lines_as("Seisner", args!["Once an Archer shoots an arrow,", "he can't stop its flight or change his target. That's why Archers need to be level headed and really careful in battle."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Seisner",
                        args!["Someday, I'd like to become a great Archer and use my skills for the good of Rune-Midgarts."],
                    )?;
                    return ctx.close();
                }
                _ => {}
            }
        }
    } else if ctx.var("tu_archer01").get()? == 2 {
        ctx.next()?;
        ctx.lines_as(
            "Seisner",
            args![
                "Thanks for listening.",
                "I know I gave you a lot of information, but I think it's important that everyone",
                "understands archery."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Seisner", args!["If you feel comfortable enough with the knowledge I've taught you, you should go back to Master Kavaruk. But if you still have questions, you can always come and ask me."])?;
        ctx.close_window()?;
        ctx.call(Function::SpecialEffect, args![constants::EF_WIND])?;
        ctx.var("tu_archer01").set(Val::from(3))?;
        if ctx.var("JobLevel").get()? == 1 {
            ctx.call(Function::GetExperience, args![0, 30])?;
        } else if (ctx.player().job_level()? > 1 && ctx.player().job_level()? < 11) {
            ctx.call(Function::GetExperience, args![0, 80])?;
        } else if (ctx.player().job_level()? > 10 && ctx.player().job_level()? < 21) {
            ctx.call(Function::GetExperience, args![0, 100])?;
        } else {
            ctx.call(Function::GetExperience, args![0, 120])?;
        }
        return ctx.end();
    }
    return ctx.close();
}

pub fn target(ctx: &Ctx) -> Script {
    return ctx.end();
}

pub fn acolyte_tu(ctx: &Ctx) -> Script {
    ctx.mes("[Acolyte]")?;
    if ctx.var("tu_archer01").get()? == 14 {
        if (ctx.time_field(constants::DT_HOUR)? >= 18 && ctx.time_field(constants::DT_HOUR)? < 22) {
            ctx.lines(args!["H-hello!", "Umm, umm...", "Are you R-Reidin Corse's", "friend t-too?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Acolyte",
                args![
                    "M-my name is Mafra.",
                    "Ever since he saved my life, Reidin has a-always been a good f-friend to me. He's such a great Archer",
                    "and a really nice person!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Acolyte",
                args!["Um, and...", "Uh... Oh no~", "What am I supposed", "t-to tell you...?"],
            )?;
            ctx.call(Function::Emotion, args![constants::ET_PROFUSELY_SWEAT])?;
            ctx.next()?;
            ctx.lines_as(
                "Acolyte",
                args![
                    "Oh, since I'm training to be an Acolyte, I'm supposed to help out the people he's teaching. So...",
                    "Let's h-help each other train!",
                    "Um, is that okay?"
                ],
            )?;
            ctx.next()?;
            match ctx.menu(&["Sure.", "No thanks.", "Reidin Corse is mine!"])? {
                0 => {
                    ctx.lines_as("Acolyte", args!["Wow!", "Thank you, thank you!", "I''ll try my very best!"])?;
                    ctx.var("tu_archer01").set(Val::from(15))?;
                    return ctx.close();
                }
                1 => {
                    ctx.lines_as(
                        "Acolyte",
                        args![
                            "Oh... Oh.",
                            "I'm so sorry.",
                            "If you don't need",
                            "my help, I guess",
                            "that's alright..."
                        ],
                    )?;
                    ctx.call(Function::Emotion, args![constants::ET_CRY])?;
                    return ctx.close();
                }
                2 => {
                    ctx.lines_as(
                        "Acolyte",
                        args![
                            "Eh?!",
                            "R-really?",
                            "I guess he doesn't",
                            "need to tell me if",
                            "he already has a girlfriend..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.mes("[Acolyte]")?;
                    if ctx.var("Sex").get()? == constants::SEX_MALE {
                        ctx.lines(args![
                            "W-wait!",
                            "Y-you're a man!",
                            "D-d-d-don't tease me",
                            "like that! I'm serious!"
                        ])?;
                    } else {
                        ctx.lines(args![
                            "And he's so brave",
                            "and funny and smart.",
                            "Y-you're lucky to have him.",
                            "^333333*Sniff*^000000 I... I..."
                        ])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Acolyte",
                            args![
                                "^333333*Sniffle*^000000",
                                "I'll d-do my best to help you!",
                                "I wish you both happiness! (Waaaaaah~!)"
                            ],
                        )?;
                    }
                    ctx.var("tu_archer01").set(Val::from(15))?;
                    return ctx.close();
                }
                _ => {}
            }
        } else {
            ctx.lines(args!["^666666Zzzzz...^000000", "Wh-wha...?", "Who are you?"])?;
            ctx.call(Function::NpcSpecialEffect, args![constants::EF_SLEEPATTACK])?;
            ctx.next()?;
            ctx.lines_as(
                "Acolyte",
                args![
                    "Wait, I know...",
                    "Y-you're... ^666666*Yawn*^000000",
                    "So sleepy. Take this for now..."
                ],
            )?;
            match ctx.rand(4)? {
                0 => {
                    runtime::npc_skill(ctx, &Val::from("AL_HEAL"), &Val::from(3), &Val::from(90), &Val::from(62))?;
                }
                1 => {
                    runtime::npc_skill(ctx, &Val::from("AL_HEAL"), &Val::from(9), &Val::from(90), &Val::from(62))?;
                }
                2 => {
                    runtime::npc_skill(ctx, &Val::from("AL_HEAL"), &Val::from(8), &Val::from(90), &Val::from(62))?;
                }
                _ => {}
            }
            match ctx.rand(3)? {
                0 => {
                    runtime::npc_skill(ctx, &Val::from("AL_INCAGI"), &Val::from(1), &Val::from(0), &Val::from(0))?;
                }
                1 => {
                    runtime::npc_skill(ctx, &Val::from("AL_INCAGI"), &Val::from(5), &Val::from(0), &Val::from(0))?;
                }
                2 => {
                    runtime::npc_skill(ctx, &Val::from("AL_INCAGI"), &Val::from(10), &Val::from(0), &Val::from(0))?;
                }
                _ => {}
            }
            match ctx.rand(3)? {
                0 => {
                    runtime::npc_skill(ctx, &Val::from("AL_BLESSING"), &Val::from(1), &Val::from(0), &Val::from(0))?;
                }
                1 => {
                    runtime::npc_skill(ctx, &Val::from("AL_BLESSING"), &Val::from(5), &Val::from(0), &Val::from(0))?;
                }
                2 => {
                    runtime::npc_skill(ctx, &Val::from("AL_BLESSING"), &Val::from(10), &Val::from(0), &Val::from(0))?;
                }
                _ => {}
            }
            return ctx.close();
        }
    } else if ctx.var("tu_archer01").get()? == 15 {
        if (ctx.time_field(constants::DT_HOUR)? >= 18 && ctx.time_field(constants::DT_HOUR)? < 22) {
            ctx.lines(args!["^666666Zzzzz...^000000", "Wh-wha...?", "Who are you?"])?;
            ctx.call(Function::NpcSpecialEffect, args![constants::EF_SLEEPATTACK])?;
            ctx.next()?;
            ctx.lines_as(
                "Acolyte",
                args!["Reidin Corse?", "I can't believe", "I feel asleep fo--", "^666666Zzzzzzz...^000000"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Acolyte",
                args![
                    "^666666Zzzz^000000--Oooh!",
                    "J-just take this before",
                    "I fall asleep again~! ^666666*Yawn*^000000"
                ],
            )?;
        } else {
            ctx.lines(args!["Okay~!", "Let me try", "casting a spell", "to help you! Yaa~p!"])?;
        }
        match ctx.rand(4)? {
            0 => {
                runtime::npc_skill(ctx, &Val::from("AL_HEAL"), &Val::from(3), &Val::from(90), &Val::from(62))?;
            }
            1 => {
                runtime::npc_skill(ctx, &Val::from("AL_HEAL"), &Val::from(10), &Val::from(90), &Val::from(62))?;
            }
            2 => {
                runtime::npc_skill(ctx, &Val::from("AL_HEAL"), &Val::from(8), &Val::from(90), &Val::from(62))?;
            }
            _ => {}
        }
        match ctx.rand(3)? {
            0 => {
                runtime::npc_skill(ctx, &Val::from("AL_INCAGI"), &Val::from(1), &Val::from(0), &Val::from(0))?;
            }
            1 => {
                runtime::npc_skill(ctx, &Val::from("AL_INCAGI"), &Val::from(5), &Val::from(0), &Val::from(0))?;
            }
            2 => {
                runtime::npc_skill(ctx, &Val::from("AL_INCAGI"), &Val::from(10), &Val::from(0), &Val::from(0))?;
            }
            _ => {}
        }
        match ctx.rand(3)? {
            0 => {
                runtime::npc_skill(ctx, &Val::from("AL_BLESSING"), &Val::from(1), &Val::from(0), &Val::from(0))?;
            }
            1 => {
                runtime::npc_skill(ctx, &Val::from("AL_BLESSING"), &Val::from(5), &Val::from(0), &Val::from(0))?;
            }
            2 => {
                runtime::npc_skill(ctx, &Val::from("AL_BLESSING"), &Val::from(10), &Val::from(0), &Val::from(0))?;
            }
            _ => {}
        }
        return ctx.close();
    }
    ctx.lines(args![
        "...",
        "W-why does",
        "traveling make",
        "me sooo sleepy...?",
        "^666666Zzzzzzzz...^000000"
    ])?;
    ctx.call(Function::NpcSpecialEffect, args![constants::EF_SLEEPATTACK])?;
    return ctx.close();
}

pub fn alchemist_guildmember_tu(ctx: &Ctx) -> Script {
    let mut l_alche_f = Val::from(0);
    let mut l_alche_s = Val::from(0);
    let mut l_total_zeny = Val::from(0);
    ctx.mes("[Alchemist Guildmember]")?;
    if ctx.var("tu_archer02").get()?.number()? > 0 {
        if (ctx.items().count(1032)? > 0 || ctx.items().count(905)? > 0) {
            ctx.lines(args![
                "Great...!",
                "I've been waiting",
                "for the Icarus Guild",
                "to send somebody",
                "to help us! Finally!"
            ])?;
            ctx.call(Function::Emotion, args![constants::ET_CRY])?;
            ctx.next()?;
            ctx.lines_as(
                "Alchemist Guildmember",
                args![
                    "Would you like to sell ^3131FFStems^000000",
                    "and ^3131FFManeater Blossoms^000000 to",
                    "the Alchemist Guild? We'll buy",
                    "each Stem for ^3131FF30 Zeny^000000 and each",
                    "Maneater Blossom for ^3131FF130 Zeny^000000."
                ],
            )?;
            ctx.next()?;
            match ctx.menu(&["Sell all Stems and Maneater Blossoms", "Don't sell anything."])? {
                0 => {
                    ctx.lines_as(
                        "Alchemist Guildmember",
                        args![
                            "Thank you very much!",
                            "This will really help",
                            "us in maintaining our",
                            "Flora field! I really",
                            "appreciate your help~"
                        ],
                    )?;
                    l_alche_f = ctx.call(Function::CountItem, args![1032])?;
                    l_alche_s = ctx.call(Function::CountItem, args![905])?;
                    l_total_zeny = Val::from(l_alche_f.number()? * 130 + l_alche_s.number()? * 30);
                    if ctx.call(Function::CountItem, args![1032])?.is_true() {
                        ctx.call(Function::DelItem, args![1032, l_alche_f.clone()])?;
                    }
                    if ctx.call(Function::CountItem, args![905])?.is_true() {
                        ctx.call(Function::DelItem, args![905, l_alche_s.clone()])?;
                    }
                    ctx.var("Zeny").set((ctx.var("Zeny").get()? + l_total_zeny.clone()))?;
                    return ctx.close();
                }
                1 => {
                    ctx.lines_as(
                        "Alchemist Guildmember",
                        args![
                            "Seriously...",
                            "I'm on the verge of begging!",
                            "The Alchemist Guild really needs lots of new supplies to keep our Flora field running!"
                        ],
                    )?;
                    ctx.call(Function::Emotion, args![constants::ET_CRY])?;
                    return ctx.close();
                }
                _ => {}
            }
        } else {
            ctx.lines(args![
                "I don't know if you've heard",
                "from Kavaruk, but our Flora field is getting ruined by stray cats."
            ])?;
            ctx.call(Function::Emotion, args![constants::ET_CRY])?;
            ctx.next()?;
            ctx.lines_as(
                "Alchemist Guildmember",
                args![
                    "No matter how much we chase",
                    "them away, they always manage to come back. We really have no choice but to replace what they destroy."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Alchemist Guildmember", args!["We'll reward you if you can bring us ^3131FFStems^000000 and ^3131FFManeater Blossoms^000000", "to help us in our field maintenance efforts. These Flora are crucial since they provide funds for our guild. Without them..."])?;
            return ctx.close();
        }
    }
    ctx.lines(args![
        "^666666*Sigh...*^000000",
        "Isn't anyone coming",
        "to help us? We're facing",
        "a really huge crisis here!"
    ])?;
    return ctx.close();
}

pub fn arthail(ctx: &Ctx) -> Script {
    ctx.mes("[Arthail]")?;
    if ctx.var("tu_archer02").get()?.number()? < 3 {
        ctx.lines(args![
            "I am the Bard",
            "who sings only for",
            "himself. The gentle",
            "breeze is the only",
            "audience I have."
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Arthail",
            args![
                "I am Arthail",
                "of the Wind.",
                "I sing for no man,",
                "so you had best be on",
                "your way. I'm sorry, but",
                "I have no songs to share."
            ],
        )?;
        if ctx.var("tu_archer02").get()? == 2 {
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("Master Kavaruk told me to find you!")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as(
                "Arthail",
                args![
                    "Really now?",
                    "I suppose it",
                    "can't be helped.",
                    "I really should",
                    "have contacted him."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Arthail", args!["However, I'm very tired from the long trip. I'm exhausted from the crowding of this city and the hundreds of talking people.", "I can't handle it all..."])?;
            ctx.next()?;
            ctx.lines_as("Arthail", args!["Why can't I be like everyone else? It seems that as I approach people, my heart retreats further away from them. I wonder why that is?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Arthail",
                args![
                    "I became a Bard to try to fix this social complex of mine, but to",
                    "no avail. Still, I feel much more at ease when I sing. Ah, but I've said too much already."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Arthail",
                args![
                    "Master Kavaruk",
                    "asked you to help",
                    "me? Well then, I have",
                    "a favor to ask of you."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Arthail", args!["Here in the capital of the", "Rune-Midgarts Kingdom, there's supposed to be a fountain in the central plaza. Everyone seems to stop and chat over there."])?;
            ctx.next()?;
            ctx.lines_as("Arthail", args!["However, I have a fear of other people and I can't bring myself to go near the crowds of people there. It's embarassing for me to admit this as a Bard..."])?;
            ctx.next()?;
            ctx.lines_as(
                "Arthail",
                args!["Would you please go and see if there's any news being spread around in that area for me?"],
            )?;
            ctx.var("tu_archer02").set(Val::from(3))?;
        }
    } else if ctx.var("tu_archer02").get()? == 3 {
        ctx.lines(args![
            "I'm ashamed to ask,",
            "but would you go the fountain in Prontera's central plaza and see",
            "if there's any news?"
        ])?;
    } else if ctx.var("tu_archer02").get()? == 4 {
        ctx.lines(args![
            "I see...",
            "Nothing much.",
            "Another person",
            "found an Emperium",
            "and is starting a",
            "brand new guild..."
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Arthail",
            args![
                "^3131FFEmperium^000000...",
                "There are a lot of ambitious people in Morocc that want one of those. Even in my hometown of Payon,",
                "there are people clamoring to obtain one..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Arthail",
            args![
                "Supposedly, having an Emperium",
                "will give you the power to change the world. People are bound to be attracted to it."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Arthail",
            args!["Hmm...?", "I sense something", "dark and ominous...", "I wonder what it could be?"],
        )?;
        ctx.call(Function::Emotion, args![constants::ET_SCRATCH])?;
        ctx.next()?;
        ctx.lines_as(
            "Arthail",
            args![
                "I'm probably just stressed and exhausted. I'll feel better once",
                "I get some rest. Would you like to join me and listen to the flowing water?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Arthail", args!["It's not the greatest feeling if you stay here for too long, but the sound of this water reminds me of home. Did you see the lake by the Icarus Guild? These waters remind me of that place."])?;
        ctx.next()?;
        ctx.lines_as(
            "Arthail",
            args![
                "^333333*Yawn...*^000000",
                "I haven't talked",
                "like this in so long.",
                "I'm so tired, I think",
                "I'll rest just a little..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Arthail", args![".........."])?;
        ctx.next()?;
        ctx.lines(args!["^3355FFArthail of the Wind", "has fallen fast asleep.^000000"])?;
        ctx.next()?;
        ctx.lines_as(ctx.player().name()?, args!["Ah...", "So peaceful~"])?;
        ctx.next()?;
        ctx.mes("^3355FFThe rhythm of the lapping waters gently eases you into a state of perfect relaxation. The cold, crisp water would feel so refreshing if you dipped your feet into it...^000000")?;
        ctx.next()?;
        ctx.lines_as("???", args!["..."])?;
        ctx.call(Function::SoundEffectAll, args!["se_littlewaves02.wav", 0])?;
        ctx.next()?;
        ctx.lines_as(ctx.player().name()?, args!["Huh?", "What was", "that noise?"])?;
        ctx.next()?;
        ctx.lines_as("????", args!["AAAAAAAAAAAAAAAHHHHHHHHHHH!!!!!"])?;
        ctx.call(Function::Emotion, args![constants::ET_HUK])?;
        ctx.call(Function::SoundEffectAll, args!["se_scream_w01.wav", 0])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.player().name()?,
            args!["What the--?!", "That sounds like", "screams coming from", "inside Prontera Castle!"],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFYou try to wake Arthail",
            "but he's deeply asleep",
            "and won't budge at all.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.player().name()?,
            args!["I can't help it then.", "I'd better go check", "this out on my own."],
        )?;
        ctx.var("tu_archer02").set(Val::from(5))?;
    } else if (ctx.var("tu_archer02").get()?.number()? > 4 && ctx.var("tu_archer02").get()?.number()? < 7) {
        ctx.mes("^333333Zzzzz...^000000")?;
    } else if ctx.var("tu_archer02").get()? == 8 {
        ctx.lines(args![
            "Where did you go?",
            "I thought you might have been kidnapped since you weren't",
            "here when I woke up."
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Arthail",
            args![
                "So everyone said nothing",
                "happened when you asked",
                "about the shrieks you heard?",
                "That's really peculiar..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Arthail", args!["Even the Emperium you mentioned earlier. I thought it was supposed to be a rare item, but why is it so easy for so many people to get", "one? It's very strange..."])?;
        ctx.next()?;
        ctx.lines_as("Arthail", args!["Huh. I better investigate this", "a little more, so I'll stay here to gather more information. For now, you should head back to Payon and let Master Kavaruk know that I'm doing alright."])?;
        ctx.next()?;
        ctx.lines_as(
            "Arthail",
            args![
                "And one day...",
                "I guess I'll finally",
                "have a song that",
                "I can share with you."
            ],
        )?;
        ctx.var("tu_archer02").set(Val::from(9))?;
    } else {
        ctx.lines(args!["...", "......"])?;
    }
    return ctx.close();
}

pub fn new_guild_master_tu(ctx: &Ctx) -> Script {
    ctx.lines_as("New Guild Master", args!["Hearken, all", "of you who seek", "fame and glory!"])?;
    ctx.next()?;
    ctx.lines_as(
        "New Guild Master",
        args![
            "I hold in my hand this ^3131FFEmperium^000000, sign that I am favored by destiny! Brave ones, I bid you, lend me",
            "your power!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "New Guild Master",
        args![
            "Nothing can stop us!",
            "So long as this jewel",
            "is in my possession,",
            "those who call themselves",
            "my comrades will never",
            "know the taste of defeat!"
        ],
    )?;
    if ctx.var("tu_archer02").get()? == 3 {
        ctx.var("tu_archer02").set(Val::from(4))?;
    }
    return ctx.close();
}

pub fn mage_tu(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Mage",
        args!["Did you see that", "new guild master?", "Talk about getting", "on your soapbox!"],
    )?;
    ctx.next()?;
    ctx.call(Function::Emotion, args![constants::ET_HNG])?;
    ctx.lines_as(
        "Mage",
        args![
            "Doesn't he know that there",
            "are a lot of famous guilds around nowadays? Why would anyone want",
            "to join his small no-name guild?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Mage", args!["Now, if I become a Wizard,", "I'm going to join the best guild around. Casting spells to help the strongest Knights and the most powerful Priests would make me", "look really cool!"])?;
    ctx.next()?;
    ctx.lines_as("Mage", args!["Bwahahah!", "Just thinking", "about it makes", "me so excited!"])?;
    return ctx.close();
}

pub fn minister_tu(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Minister",
        args![
            "The royal family",
            "is extremely busy",
            "at the moment, handling",
            "our nation's administration."
        ],
    )?;
    if ctx.var("tu_archer02").get()? == 5 {
        ctx.next()?;
        ctx.lines_as(
            ctx.player().name()?,
            args!["I heard screams", "from outside! Did", "something happen?"],
        )?;
        ctx.next()?;
        ctx.lines_as("Minister", args!["...", "......"])?;
        ctx.next()?;
        ctx.lines_as("Minister", args!["Ah, there are a few stray cats running around, those Wild Roses, so those noises you heard must have been them. Everything is fine here in the palace."])?;
        ctx.var("tu_archer02").set(Val::from(6))?;
    } else if ctx.var("tu_archer02").get()? == 7 {
        ctx.next()?;
        ctx.lines(args![
            "^3355FFThe urgency that the Minister expressed earlier has been",
            "replaced with a stone cold feeling of sternness. He probably won't tell you much more information,",
            "no matter how much you ask him."
        ])?;
    } else if ctx.var("tu_acolyte01").get()? == 25 {
        ctx.next()?;
        ctx.lines_as(
            ctx.player().name()?,
            args![
                "Um...",
                "I hear that",
                "the royal family",
                "might be in sort of",
                "trouble. Is there any",
                "way I could help?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Minister", args!["...", "......"])?;
        ctx.next()?;
        ctx.lines_as(
            "Minister",
            args![
                "You must be mistaken.",
                "The royal family is fine,",
                "especially since our",
                "kingdom is at peace."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Minister", args!["Perhaps you've gotten that idea because it's very difficult to meet any member of the royal family. Please understand that they're", "all very busy trying to serve", "the people."])?;
        ctx.next()?;
        ctx.lines_as("Minister", args!["As a holy servant, I'm sure that you'll find plenty of other people who have need of your help and abilities outside of the palace."])?;
    }
    return ctx.close();
}

pub fn sound_tu(ctx: &Ctx) -> Script {
    return ctx.end();
}

pub fn sound_tu_ontouch(ctx: &Ctx) -> Script {
    if ctx.var("tu_archer02").get()? == 6 {
        ctx.lines_as("Minister", args!["Contact the", "Prontera Church.", "Hurry, this is urgent!"])?;
        ctx.var("tu_archer02").set(Val::from(7))?;
        return ctx.close();
    }
    return ctx.end();
}

pub fn bishop_maugins(ctx: &Ctx) -> Script {
    ctx.mes("[Bishop Maugins]")?;
    if ctx.var("tu_archer02").get()? == 7 {
        ctx.lines(args![
            "Greetings.",
            "I doubt that you're here to change to the Monk job, but may I help you with something? Perhaps you're",
            "here for a confession?"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Bishop Maugins",
            args![
                "Hm? Work related to",
                "the Palace? Yes, I'm in",
                "charge of that area. So",
                "how may I help you?"
            ],
        )?;
        ctx.next()?;
        if ctx.menu(&["Has something happened to the Kingdom?", "Nothing."])? == 0 {
            ctx.lines_as("Bishop Maugins", args!["...!!"])?;
            ctx.next()?;
            ctx.lines_as("Bishop Maugins", args!["...", "......"])?;
            ctx.next()?;
            ctx.lines_as("Bishop Maugins", args!["Ho ho~", "Of course not!", "The king and I regularly write to each other, but I haven't heard of anything in particular. Please don't worry yourself."])?;
            ctx.next()?;
            ctx.var("tu_archer02").set(Val::from(8))?;
        }
        ctx.lines_as("Bishop Maugins", args!["Good luck on", "your journeys,", "brave adventurer."])?;
        runtime::npc_skill(ctx, &Val::from("AL_INCAGI"), &Val::from(10), &Val::from(0), &Val::from(0))?;
        runtime::npc_skill(ctx, &Val::from("AL_BLESSING"), &Val::from(10), &Val::from(0), &Val::from(0))?;
        return ctx.close();
    }
    ctx.lines(args![
        "I'm sorry, but I have",
        "some pressing matter",
        "to think about right now.",
        "Would you come back later?"
    ])?;
    return ctx.close();
}

use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn sign_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    shared::quests_the_sign_quest::f_updatesignvars(ctx, vec![])?;
    ctx.call(Function::Cutin, vec![Val::from("sign_01"), Val::from(4)])?;
    ctx.mes("^3355FFNext^000000")?;
    ctx.next()?;
    ctx.call(Function::Cutin, vec![Val::from("sign_01"), Val::from(255)])?;
    ctx.call(Function::Cutin, vec![Val::from("sign_02"), Val::from(4)])?;
    ctx.mes("^3355FFNext^000000")?;
    ctx.next()?;
    ctx.call(Function::Cutin, vec![Val::from("sign_02"), Val::from(255)])?;
    ctx.call(Function::Cutin, vec![Val::from("sign_03"), Val::from(4)])?;
    ctx.mes("^3355FFNext^000000")?;
    ctx.next()?;
    ctx.call(Function::Cutin, vec![Val::from("sign_03"), Val::from(255)])?;
    ctx.call(Function::Cutin, vec![Val::from("sign_04"), Val::from(4)])?;
    if !(ctx.var("sign_q").get()?.is_true()) {
        ctx.var("sign_q").set(Val::from(1))?;
    }
    ctx.mes("^3355FFClose^000000")?;
    ctx.close_window()?;
    ctx.call(Function::Cutin, vec![Val::from("sign_04"), Val::from(255)])?;
    return Err(Stop::End);
}

pub fn sign(ctx: &Ctx) -> Script {
    sign_body(ctx, Vec::new()).map(|_| ())
}

fn sign_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.var("$signbmps").set(Val::from(0))?;
    ctx.var("$signmazemonster").set(Val::from(0))?;
    ctx.var("$timezonestring$").set(Val::from("^FF0000GMT^000000"))?;
    return Err(Stop::End);
}

pub fn sign_oninit(ctx: &Ctx) -> Script {
    sign_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn archeologist_sign_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_pass_s = Val::from(0);
    let mut l_stime_s = Val::from(0);
    let mut l_stime_s1 = Val::from(0);
    shared::quests_the_sign_quest::f_updatesignvars(ctx, vec![])?;
    ctx.call(Function::Cutin, vec![Val::from("mets_alpha"), Val::from(2)])?;
    ctx.mes("[Metz]")?;
    if ctx.var("sign_q").get()?.number()? < 1 {
        ctx.lines(args![
            "Although you need everlasting patience in an archaeological excavation, the feeling you get when you find something makes",
            "all those long hours of study and research worth it."
        ])?;
    } else {
        if ctx.var("sign_q").get()?.number()? < 4 {
            let subject1 = ctx.var("sign_q").get()?;
            if subject1 == 1 {
                ctx.lines(args!["Hm...?", "Can I help you?"])?;
                ctx.next()?;
                'b2: {
                    match runtime::select_values(
                        ctx,
                        &[Val::from("I've been following these signs and...:I was just passing by...")],
                    )? {
                        1 => {
                            ctx.lines_as(
                                "Metz",
                                args!["Great...!", "Welcome to my", "humble lodgings.", "Hmm, let me see..."],
                            )?;
                            ctx.next()?;
                            if ctx.var("BaseLevel").get()?.number()? < 50 {
                                ctx.lines_as("Metz", args!["Uh, it pains me to say this, but I don't think you qualified to help me out. Once you gain enough experience though, I'll be happy to have you on board~"])?;
                                break 'b2;
                            } else {
                                ctx.lines_as(
                                    "Metz",
                                    args![
                                        "Hey, I think you might",
                                        "be well suited for the job!",
                                        "But do you think you could come back later? I've got my hands full with some other business."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Metz",
                                    args![
                                        "Oh right, would you tell",
                                        ((Val::from("me your name? ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                            + Val::from("?")),
                                        "Okay then, I'll remember that.",
                                        "Talk to you later, alright?"
                                    ],
                                )?;
                                ctx.var("sign_q").set(Val::from(2))?;
                                break 'b2;
                            }
                        }
                        2 => {
                            ctx.lines_as(
                                "Metz",
                                args![
                                    "Oh really?",
                                    "I see, I thought you",
                                    "were an applicant for",
                                    "the position I'm offering",
                                    "to brave adventurers."
                                ],
                            )?;
                        }
                        _ => {}
                    }
                }
            } else if subject1 == 2 {
                ctx.lines(args!["I'm sorry I made you wait,", "but I had some research to finish and it took longer than I expected. Now, before I tell you more about the job, I want to test your competency."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Metz",
                    args![
                        "The job I'm offering is",
                        "pretty risky and not just",
                        "anybody can handle it.",
                        "You'll actually go through",
                        "a series of tests conducted",
                        "by my trusted friends."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Metz",
                    args![
                        "Now, the first person",
                        "you must visit is ^FF0000Arian^000000",
                        "in Morocc. Please speak",
                        "to him and he'll give you",
                        "all the details about his",
                        "examination... I hope."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Metz",
                    args![
                        "Once you're finished with",
                        "the test, Arian will tell you",
                        "what to do next. Afterwards,",
                        "come back to me so that we",
                        "can finally talk business."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Metz",
                    args![
                        "Ah, almost forgot.",
                        "Arian won't talk to anybody",
                        "unless he knows them or",
                        "receives a message from me.",
                        "So if he's snubbed you in the past, just understand that's his way."
                    ],
                )?;
                ctx.var("sign_q").set(Val::from(3))?;
            } else if subject1 == 3 {
                ctx.lines(args![
                    "Hm...?",
                    "Shouldn't you leave",
                    "for Morocc to see Arian?",
                    "You better hurry in case",
                    "somebody else applies",
                    "for this little job."
                ])?;
            }
        } else {
            if ctx.var("sign_q").get()?.number()? < 13 {
                ctx.lines(args![
                    "I don't know if you realize",
                    "it, but I'm offering a golden",
                    "opportunity for the adventurer",
                    "who works for me. So don't",
                    "hesitate to see Arian."
                ])?;
                ctx.next()?;
                ctx.lines_as(
                    "Metz",
                    args![
                        "And just so you know,",
                        "it's not a good idea to",
                        "judge Arian by his looks.",
                        "He's more than meets",
                        "the eye, you know."
                    ],
                )?;
            } else {
                if ctx.var("sign_q").get()?.number()? < 15 {
                    ctx.lines(args![
                        "I'm impressed that",
                        "you managed to get",
                        "Arian's approval! Oh,",
                        "and how's Daewoon?",
                        "He's a character, isn't he?"
                    ])?;
                } else {
                    if ctx.var("sign_q").get()?.number()? < 20 {
                        ctx.lines(args![
                            "I'm not surprised",
                            "that Daewoon likes",
                            "you. Ah, but Jore is",
                            "always busy. Still, if you",
                            "know his schedule, you",
                            "should be alright."
                        ])?;
                    } else {
                        if ctx.var("sign_q").get()?.number()? < 25 {
                            ctx.lines(args![
                                "Jesqurienne is",
                                "a brilliant woman.",
                                "Although I worry a",
                                "little bit about her",
                                "overconfidence, she's",
                                "a good friend of mine."
                            ])?;
                        } else {
                            if ctx.var("sign_q").get()?.number()? < 35 {
                                ctx.lines(args![
                                    "Dearles...?",
                                    "Ah yes, he's one",
                                    "of my shadier friends.",
                                    "He's difficult to find",
                                    "and he's only truly kind",
                                    "to a select few, so..."
                                ])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Metz",
                                    args!["It certainly doesn't", "help that he's hopelessly", "addicted to gambling..."],
                                )?;
                            } else {
                                if ctx.var("sign_q").get()?.number()? < 53 {
                                    ctx.lines(args![
                                        "Ah, Bakerlan~",
                                        "I've heard that he's",
                                        "been quite busy lately.",
                                        "But that's how all big",
                                        "businessmen are, I suppose."
                                    ])?;
                                } else {
                                    if ctx.var("sign_q").get()? == 53 {
                                        ctx.lines(args![
                                            "Congratulations~",
                                            "You managed to pass",
                                            "all of the tests! You seem",
                                            "to be the perfect person",
                                            "to carry out this special",
                                            "assignment!"
                                        ])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Metz",
                                            args![
                                                "By now, you must have",
                                                "six Sobbing Starlight pieces.",
                                                "I'm sure that you want to know",
                                                "more about these fragments."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Metz",
                                            args![
                                                "I remember last summer,",
                                                "I found the wholly formed",
                                                "Sobbing Starlight north",
                                                "of Mount Mjolnir during",
                                                "one of my expeditions..."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Metz",
                                            args![
                                                "Although it was in perfect",
                                                "shape, once it was exposed",
                                                "to the air, it began to crack",
                                                "and shattered into the pieces",
                                                "you now hold in your hand."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("Metz", args!["Now, an ordinary artisan", "can't put the Sobbing Starlight", "back together. This mysterious stone has some strange properties. But it's imperative for me to get this stone reassembled."])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Metz",
                                            args![
                                                "Once restored, a strange",
                                                "pattern can be seen within",
                                                "the Sobbing Starlight. I guess",
                                                "that the pattern is a message",
                                                "written in an ancient language."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Metz",
                                            args![
                                                "Would you let me borrow",
                                                "the pieces for a second?",
                                                "I'll show you something",
                                                "quite interesting..."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines(args![
                                            "^3355FFOnce you hand the pieces",
                                            "of the Sobbing Starlight to",
                                            "Metz, he pulls out a seventh",
                                            "piece. Once gathered, they",
                                            "begin to emit a strange light.^000000"
                                        ])?;
                                        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_TELEPORTATION2")?])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Metz",
                                            args![
                                                "Since the pieces still",
                                                "respond to each other,",
                                                "I believe that it's possible",
                                                "for the Sobbing Starlight",
                                                "to be restored to its",
                                                "original form."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Metz",
                                            args![
                                                "We're still seeking",
                                                "an artisan of great",
                                                "skill for this task. Once",
                                                "know right away. For now,",
                                                "please hold on to these pieces."
                                            ],
                                        )?;
                                        ctx.var("sign_q").set(Val::from(54))?;
                                        ctx.call(Function::GetItem, vec![Val::from(7177), Val::from(1)])?;
                                    } else {
                                        if ctx.var("sign_q").get()? == 54 {
                                            if ctx.call(Function::Rand, vec![Val::from(1), Val::from(5)])? == 4 {
                                                ctx.lines(args![
                                                    "Ah, you've come",
                                                    "just in the nick of time!",
                                                    "I just found someone who",
                                                    "may be capable of restoring",
                                                    "the Sobbing Starlight."
                                                ])?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Metz",
                                                    args![
                                                        "His name is",
                                                        "^FF0000Engel Howard^000000,",
                                                        "a legendary Blacksmith",
                                                        "in Midgard. I don't",
                                                        "know where he is, but maybe",
                                                        "his family in Geffen may know."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Metz",
                                                    args![
                                                        "Unfortunately, that's all",
                                                        "the information that I have",
                                                        "to give you for now. You'll",
                                                        "have to investigate this lead",
                                                        ((Val::from("on your own, ")
                                                            + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                            + Val::from("."))
                                                    ],
                                                )?;
                                            } else {
                                                ctx.lines(args![
                                                    "Although I've made",
                                                    "some progress, I still",
                                                    "haven't found an artisan",
                                                    "capable of restoring the",
                                                    "Sobbing Starlight. Please",
                                                    "give me a little more time."
                                                ])?;
                                            }
                                        } else {
                                            if ctx.var("sign_q").get()?.number()? < 71 {
                                                ctx.lines(args![
                                                    "Keep up the good work.",
                                                    "I'm sorry that I don't have",
                                                    "have much information for",
                                                    "you to follow, but someone",
                                                    "with your abilities should",
                                                    "be able to find a way."
                                                ])?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Metz",
                                                    args![
                                                        "While you work things",
                                                        "out with Engel Howard",
                                                        "and his family in Geffen,",
                                                        "I'll continue my research",
                                                        "on the Sobbing Starlight."
                                                    ],
                                                )?;
                                            } else {
                                                if ctx.var("sign_q").get()? == 71 {
                                                    ctx.lines(args![
                                                        "Amazing...! You've",
                                                        "managed to restore",
                                                        "the Sobbing Starlight!",
                                                        "Now, I recently learned that",
                                                        "this stone can lead you to",
                                                        "an ancient place..."
                                                    ])?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Metz",
                                                        args![
                                                            "That's why we need to",
                                                            "know what the message in",
                                                            "the Sobbing Starlight means.",
                                                            "First, we need a Wizard that",
                                                            "is skilled in unlocking the",
                                                            "messages stored in gems..."
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Metz",
                                                        args![
                                                            "Hmm. It would probably",
                                                            "be best to visit the Wizards",
                                                            "on the top floor of the Geffen",
                                                            "Tower. There's someone I know",
                                                            "there who might just be up for",
                                                            "this task..."
                                                        ],
                                                    )?;
                                                    ctx.var("sign_q").set(Val::from(72))?;
                                                } else {
                                                    if ctx.var("sign_q").get()? == 72 {
                                                        ctx.lines(args![
                                                            "Now that the",
                                                            "Sobbing Starlight",
                                                            "is restored, we need",
                                                            "to find someone who can",
                                                            "break the seal on the",
                                                            "gem's message..."
                                                        ])?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Metz",
                                                            args![
                                                                "For now, please visit",
                                                                "the Wizards at the top",
                                                                "floor of the Geffen Tower.",
                                                                "I know that one of them is",
                                                                "capable of getting the gem's",
                                                                "text on to paper..."
                                                            ],
                                                        )?;
                                                    } else {
                                                        if ctx.var("sign_q").get()?.number()? < 76 {
                                                            ctx.lines(args![
                                                                "The Wizard we're looking",
                                                                "for isn't in Geffen? Hm, at least we know he's near Comodo. Now,",
                                                                "while you investigate that lead, I'll continue to gather more",
                                                                "information on this gem..."
                                                            ])?;
                                                        } else {
                                                            if ctx.var("sign_q").get()? == 76 {
                                                                ctx.lines(args![
                                                                    "Great...!",
                                                                    "You were able to get the",
                                                                    "text in the gem printed on a",
                                                                    "Record of Ancient Language?"
                                                                ])?;
                                                                ctx.next()?;
                                                                ctx.lines_as(
                                                                    "Metz",
                                                                    args![
                                                                        "Just as I thought.",
                                                                        "This language isn't",
                                                                        "one I recognize. It's",
                                                                        "probably ^333333too^000000 ancient."
                                                                    ],
                                                                )?;
                                                                ctx.next()?;
                                                                ctx.lines_as(
                                                                    "Metz",
                                                                    args![
                                                                        "Fortunately, I know",
                                                                        "one person who may",
                                                                        "be able to translate this.",
                                                                        "If he's not able to do it...",
                                                                        "We'll have to give up."
                                                                    ],
                                                                )?;
                                                                ctx.next()?;
                                                                ctx.lines_as(
                                                                    "Metz",
                                                                    args![
                                                                        "Bring this Record",
                                                                        "of Ancient Language",
                                                                        "to a man named Frank.",
                                                                        "I hope he'll be able to",
                                                                        "understand what it says..."
                                                                    ],
                                                                )?;
                                                                ctx.next()?;
                                                                'l3: loop {
                                                                    if !(true) {
                                                                        break 'l3;
                                                                    }
                                                                    'b3: {
                                                                        match runtime::select_values(
                                                                            ctx,
                                                                            &[Val::from("Who is Frank?:Where is he?:I see.")],
                                                                        )? {
                                                                            1 => {
                                                                                ctx.lines_as(
                                                                                    "Metz",
                                                                                    args![
                                                                                        "Frank Franklin has lived",
                                                                                        "in seclusion and only a few",
                                                                                        "people are aware of his skill.",
                                                                                        "But I can assure you that his",
                                                                                        "knowledge of ancient languages",
                                                                                        "is unrivaled by any mortal."
                                                                                    ],
                                                                                )?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as(
                                                                                    "Metz",
                                                                                    args![
                                                                                        "I suppose his interest",
                                                                                        "in history is what drives",
                                                                                        "him in work. Still, I've",
                                                                                        "heard that he doesn't like",
                                                                                        "meeting people. I hope you",
                                                                                        "can convince him to help us..."
                                                                                    ],
                                                                                )?;
                                                                                ctx.next()?;
                                                                            }
                                                                            2 => {
                                                                                ctx.lines_as(
                                                                                    "Metz",
                                                                                    args![
                                                                                        "It's said that",
                                                                                        "Frank Franklin lives",
                                                                                        "on the Alberta Sunken Ship",
                                                                                        "where he devotes his time",
                                                                                        "to his research. He may not",
                                                                                        "always be home though..."
                                                                                    ],
                                                                                )?;
                                                                                ctx.var("sign_q").set(Val::from(77))?;
                                                                                ctx.next()?;
                                                                            }
                                                                            3 => {
                                                                                ctx.lines_as(
                                                                                    "Metz",
                                                                                    args![
                                                                                        "Good luck, then.",
                                                                                        "I hope that you can",
                                                                                        "find a way to get Frank",
                                                                                        "Franklin to help us..."
                                                                                    ],
                                                                                )?;
                                                                                ctx.close_window()?;
                                                                                ctx.call(
                                                                                    Function::Cutin,
                                                                                    vec![Val::from("mets_alpha"), Val::from(255)],
                                                                                )?;
                                                                                return Err(Stop::End);
                                                                            }
                                                                            _ => {}
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                if ctx.var("sign_q").get()?.number()? < 80 {
                                                                    ctx.lines(args![
                                                                        "Frank Franklin has lived",
                                                                        "in seclusion and only a few",
                                                                        "people are aware of his skill.",
                                                                        "But I can assure you that his",
                                                                        "knowledge of ancient languages",
                                                                        "is unrivaled by any mortal."
                                                                    ])?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        "Metz",
                                                                        args![
                                                                            "I suppose his interest",
                                                                            "in history is what drives",
                                                                            "him in work. Still, I've",
                                                                            "heard that he doesn't like",
                                                                            "meeting people. I hope you",
                                                                            "can convince him to help us..."
                                                                        ],
                                                                    )?;
                                                                } else {
                                                                    if ctx.var("sign_q").get()?.number()? < 82 {
                                                                        ctx.lines(args![
                                                                            "So far, I've learned",
                                                                            "from my research that",
                                                                            "the gem's message has",
                                                                            "details about a certain",
                                                                            "location and an item that",
                                                                            "serves as some kind of key..."
                                                                        ])?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as(
                                                                            "Metz",
                                                                            args![
                                                                                "While I investigate,",
                                                                                "please try to have Frank",
                                                                                "Franklin translate the",
                                                                                "message in the Record of",
                                                                                "Ancient Language. Keep up",
                                                                                ((Val::from("the good work, ")
                                                                                    + ctx.call(
                                                                                        Function::StrCharInfo,
                                                                                        vec![Val::from(0)]
                                                                                    )?)
                                                                                    + Val::from("."))
                                                                            ],
                                                                        )?;
                                                                    } else {
                                                                        if ctx.var("sign_q").get()? == 82 {
                                                                            ctx.lines(args![
                                                                                "I get it now!",
                                                                                "The 'skyscraper'",
                                                                                "in this text refers to",
                                                                                "the Geffen Tower!",
                                                                                "Alright, let's see..."
                                                                            ])?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as(
                                                                                "Metz",
                                                                                args![
                                                                                    "Hmm...",
                                                                                    "''The town where the",
                                                                                    "rejected are left behind.''",
                                                                                    "What does that mean",
                                                                                    "and where could it be...?"
                                                                                ],
                                                                            )?;
                                                                        } else {
                                                                            if ctx.var("sign_q").get()?.number()? < 98 {
                                                                                ctx.lines(args![
                                                                                    "I'm sure the town",
                                                                                    "where the rejected",
                                                                                    "live means something,",
                                                                                    "but I'm unable to figure",
                                                                                    "out the answer to this",
                                                                                    "little riddle..."
                                                                                ])?;
                                                                            } else {
                                                                                if (ctx.var("sign_q").get()?.number()? > 100
                                                                                    && ctx.var("sign_q").get()?.number()? < 105)
                                                                                {
                                                                                    ctx.lines(args![
                                                                                        "Angrboda...?!",
                                                                                        "According to legend,",
                                                                                        "her soul was split into",
                                                                                        "pieces and placed behind",
                                                                                        "seals created by the gods!"
                                                                                    ])?;
                                                                                    ctx.next()?;
                                                                                    ctx.lines_as(
                                                                                        "Metz",
                                                                                        args![
                                                                                            "You'll need a stout,",
                                                                                            "heavy or really sharp",
                                                                                            "weapon. Of course, I'm",
                                                                                            "told that such weapons",
                                                                                            "are truly rare..."
                                                                                        ],
                                                                                    )?;
                                                                                    ctx.next()?;
                                                                                    ctx.lines_as(
                                                                                        "Metz",
                                                                                        args![
                                                                                            "Supposedly, normal",
                                                                                            "Blacksmiths can't even",
                                                                                            "forge those kinds of rare",
                                                                                            "weapons. But if you manage",
                                                                                            "to get one, you might have a",
                                                                                            "chance of breaking the seals."
                                                                                        ],
                                                                                    )?;
                                                                                    ctx.next()?;
                                                                                    ctx.lines_as(
                                                                                        "Metz",
                                                                                        args![
                                                                                            "Of course, I'm worried",
                                                                                            "that may be violating the",
                                                                                            "will of the gods by releasing",
                                                                                            "Angrboda, but we've already",
                                                                                            "gone this far..."
                                                                                        ],
                                                                                    )?;
                                                                                } else {
                                                                                    if ctx.var("sign_q").get()?.number()? < 137 {
                                                                                        ctx.lines(args![
                                                                                            "I think you're",
                                                                                            "almost there. All",
                                                                                            "of our efforts will",
                                                                                            "soon come to fruition!"
                                                                                        ])?;
                                                                                    } else {
                                                                                        if ctx.var("sign_q").get()? == 137 {
                                                                                            ctx.lines(args![
                                                                                                "This is it...!",
                                                                                                "You've brought me,",
                                                                                                "'The Sign!' I've finally",
                                                                                                "proven its existence!",
                                                                                                "Please let me handle",
                                                                                                "this and come back later~"
                                                                                            ])?;
                                                                                            ctx.call(
                                                                                                Function::DelItem,
                                                                                                vec![Val::from(7314), Val::from(1)],
                                                                                            )?;
                                                                                            ctx.var("sign_q").set(Val::from(138))?;
                                                                                            l_stime_s = ctx.call(
                                                                                                Function::GetTime,
                                                                                                vec![ctx.constant("DT_HOUR")?],
                                                                                            )?;
                                                                                            if l_stime_s.clone().number()? < 1 {
                                                                                                ctx.var("sign_sq").set(Val::from(1))?;
                                                                                            } else {
                                                                                                if l_stime_s.clone().number()? < 3 {
                                                                                                    ctx.var("sign_sq").set(Val::from(2))?;
                                                                                                } else {
                                                                                                    if l_stime_s.clone().number()? < 5 {
                                                                                                        ctx.var("sign_sq")
                                                                                                            .set(Val::from(3))?;
                                                                                                    } else {
                                                                                                        if l_stime_s.clone().number()? < 7 {
                                                                                                            ctx.var("sign_sq")
                                                                                                                .set(Val::from(4))?;
                                                                                                        } else {
                                                                                                            if l_stime_s.clone().number()?
                                                                                                                < 9
                                                                                                            {
                                                                                                                ctx.var("sign_sq")
                                                                                                                    .set(Val::from(5))?;
                                                                                                            } else {
                                                                                                                if l_stime_s
                                                                                                                    .clone()
                                                                                                                    .number()?
                                                                                                                    < 11
                                                                                                                {
                                                                                                                    ctx.var("sign_sq")
                                                                                                                        .set(Val::from(
                                                                                                                            6,
                                                                                                                        ))?;
                                                                                                                } else {
                                                                                                                    if l_stime_s
                                                                                                                        .clone()
                                                                                                                        .number()?
                                                                                                                        < 13
                                                                                                                    {
                                                                                                                        ctx.var("sign_sq")
                                                                                                                            .set(
                                                                                                                                Val::from(
                                                                                                                                    7,
                                                                                                                                ),
                                                                                                                            )?;
                                                                                                                    } else if l_stime_s
                                                                                                                        .clone()
                                                                                                                        .number()?
                                                                                                                        < 15
                                                                                                                    {
                                                                                                                        ctx.var("sign_sq")
                                                                                                                            .set(
                                                                                                                                Val::from(
                                                                                                                                    8,
                                                                                                                                ),
                                                                                                                            )?;
                                                                                                                    } else if l_stime_s
                                                                                                                        .clone()
                                                                                                                        .number()?
                                                                                                                        < 17
                                                                                                                    {
                                                                                                                        ctx.var("sign_sq")
                                                                                                                            .set(
                                                                                                                                Val::from(
                                                                                                                                    9,
                                                                                                                                ),
                                                                                                                            )?;
                                                                                                                    } else if l_stime_s
                                                                                                                        .clone()
                                                                                                                        .number()?
                                                                                                                        < 19
                                                                                                                    {
                                                                                                                        ctx.var("sign_sq")
                                                                                                                            .set(
                                                                                                                                Val::from(
                                                                                                                                    10,
                                                                                                                                ),
                                                                                                                            )?;
                                                                                                                    } else if l_stime_s
                                                                                                                        .clone()
                                                                                                                        .number()?
                                                                                                                        < 21
                                                                                                                    {
                                                                                                                        ctx.var("sign_sq")
                                                                                                                            .set(
                                                                                                                                Val::from(
                                                                                                                                    11,
                                                                                                                                ),
                                                                                                                            )?;
                                                                                                                    } else {
                                                                                                                        ctx.var("sign_sq")
                                                                                                                            .set(
                                                                                                                                Val::from(
                                                                                                                                    12,
                                                                                                                                ),
                                                                                                                            )?;
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        } else {
                                                                                            if ctx.var("sign_q").get()? == 138 {
                                                                                                l_stime_s1 = ctx.call(
                                                                                                    Function::GetTime,
                                                                                                    vec![ctx.constant("DT_HOUR")?],
                                                                                                )?;
                                                                                                if l_stime_s1.clone().number()? < 1 {
                                                                                                    if ctx.var("sign_sq").get()? == 11 {
                                                                                                        l_pass_s = Val::from(1);
                                                                                                    }
                                                                                                } else {
                                                                                                    if l_stime_s1.clone().number()? < 3 {
                                                                                                        if ctx.var("sign_sq").get()? == 12 {
                                                                                                            l_pass_s = Val::from(1);
                                                                                                        }
                                                                                                    } else {
                                                                                                        if l_stime_s1.clone().number()? < 5
                                                                                                        {
                                                                                                            if ctx.var("sign_sq").get()?
                                                                                                                == 1
                                                                                                            {
                                                                                                                l_pass_s = Val::from(1);
                                                                                                            }
                                                                                                        } else {
                                                                                                            if l_stime_s1
                                                                                                                .clone()
                                                                                                                .number()?
                                                                                                                < 7
                                                                                                            {
                                                                                                                if ctx
                                                                                                                    .var("sign_sq")
                                                                                                                    .get()?
                                                                                                                    == 2
                                                                                                                {
                                                                                                                    l_pass_s = Val::from(1);
                                                                                                                }
                                                                                                            } else {
                                                                                                                if l_stime_s1
                                                                                                                    .clone()
                                                                                                                    .number()?
                                                                                                                    < 9
                                                                                                                {
                                                                                                                    if ctx
                                                                                                                        .var("sign_sq")
                                                                                                                        .get()?
                                                                                                                        == 3
                                                                                                                    {
                                                                                                                        l_pass_s =
                                                                                                                            Val::from(1);
                                                                                                                    }
                                                                                                                } else {
                                                                                                                    if l_stime_s1
                                                                                                                        .clone()
                                                                                                                        .number()?
                                                                                                                        < 11
                                                                                                                    {
                                                                                                                        if ctx
                                                                                                                            .var("sign_sq")
                                                                                                                            .get()?
                                                                                                                            == 4
                                                                                                                        {
                                                                                                                            l_pass_s =
                                                                                                                                Val::from(
                                                                                                                                    1,
                                                                                                                                );
                                                                                                                        }
                                                                                                                    } else {
                                                                                                                        if l_stime_s1
                                                                                                                            .clone()
                                                                                                                            .number()?
                                                                                                                            < 13
                                                                                                                        {
                                                                                                                            if ctx.var("sign_sq").get()? == 5 {
                                                                                                                                l_pass_s = Val::from(1);
                                                                                                                            }
                                                                                                                        } else {
                                                                                                                            if l_stime_s1
                                                                                                                                .clone()
                                                                                                                                .number()?
                                                                                                                                < 15
                                                                                                                            {
                                                                                                                                if ctx.var("sign_sq").get()? == 6 {
                                                                                                                                    l_pass_s = Val::from(1);
                                                                                                                                }
                                                                                                                            } else if l_stime_s1.clone().number()? < 17 {
                                                                                                                                if ctx.var("sign_sq").get()? == 7 {
                                                                                                                                    l_pass_s = Val::from(1);
                                                                                                                                }
                                                                                                                            } else if l_stime_s1.clone().number()? < 19 {
                                                                                                                                if ctx.var("sign_sq").get()? == 8 {
                                                                                                                                    l_pass_s = Val::from(1);
                                                                                                                                }
                                                                                                                            } else if l_stime_s1.clone().number()? < 21 {
                                                                                                                                if ctx.var("sign_sq").get()? == 9 {
                                                                                                                                    l_pass_s = Val::from(1);
                                                                                                                                }
                                                                                                                            } else if ctx.var("sign_sq").get()? == 10 {
                                                                                                                                l_pass_s = Val::from(1);
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                                if l_pass_s.clone() == 1 {
                                                                                                    ctx.lines(args![
                                                                                                        "Fascinating...",
                                                                                                        "This was made with",
                                                                                                        "a material outside of",
                                                                                                        "Midgard! See this",
                                                                                                        "blue lens? That must be",
                                                                                                        "its incredible power source!"
                                                                                                    ])?;
                                                                                                    ctx.next()?;
                                                                                                    ctx.lines_as("Metz", args!["I've found that the runes", "around the lens control some", "kind of seals placed at the Geffen Tower and Geffen Fountain. But with this in your hands, the seals should be broken..."])?;
                                                                                                    ctx.next()?;
                                                                                                    ctx.lines_as("Metz", args!["I... I think", "you can even use this", "to enter Valhalla. There", "may be even other applications", "using the power of this item..."])?;
                                                                                                    ctx.next()?;
                                                                                                    ctx.lines_as(
                                                                                                        "Metz",
                                                                                                        args![
                                                                                                            "I can only think of",
                                                                                                            "one person who can",
                                                                                                            "handle working with",
                                                                                                            "this: Engel Howard,",
                                                                                                            "Midgard's best",
                                                                                                            "Blacksmith."
                                                                                                        ],
                                                                                                    )?;
                                                                                                    ctx.var("sign_q")
                                                                                                        .set(Val::from(139))?;
                                                                                                    ctx.var("sign_sq").set(Val::from(0))?;
                                                                                                    ctx.call(
                                                                                                        Function::GetItem,
                                                                                                        vec![Val::from(7314), Val::from(1)],
                                                                                                    )?;
                                                                                                } else {
                                                                                                    ctx.lines(args![
                                                                                                        "I'm sorry, but I'm",
                                                                                                        "still examining the",
                                                                                                        "artifact you've lent to",
                                                                                                        "me. Would you please",
                                                                                                        "give me some more time?"
                                                                                                    ])?;
                                                                                                }
                                                                                            } else if ctx.var("sign_q").get()?.number()?
                                                                                                < 141
                                                                                            {
                                                                                                ctx.lines(args![
                                                                                                    "Have you visited",
                                                                                                    "Engel Howard yet?",
                                                                                                    "He's the only one",
                                                                                                    "who can unlock the",
                                                                                                    "Sign's power for you..."
                                                                                                ])?;
                                                                                            } else if ctx.var("sign_q").get()? == 141 {
                                                                                                if ctx.call(
                                                                                                    Function::CountItem,
                                                                                                    vec![Val::from(2644)],
                                                                                                )? == 1
                                                                                                {
                                                                                                    ctx.lines(args![
                                                                                                        "Ah, you're finally",
                                                                                                        "here. I've been waiting",
                                                                                                        "to speak with you. First,",
                                                                                                        "let me thank you again",
                                                                                                        "for all of your help."
                                                                                                    ])?;
                                                                                                    ctx.next()?;
                                                                                                    ctx.lines_as(
                                                                                                        "Metz",
                                                                                                        args![
                                                                                                            "Thanks to you, my wish",
                                                                                                            "of proving the existence",
                                                                                                            "of the Sign has finally been",
                                                                                                            "fulfilled. Its power is now",
                                                                                                            "yours to do as you wish."
                                                                                                        ],
                                                                                                    )?;
                                                                                                    ctx.next()?;
                                                                                                    ctx.lines_as("Metz", args!["You may have had your doubts,", "but I'm happy enough to confirm that the Sign really exists. Thanks again, and I hope you stop by and chat from time to time."])?;
                                                                                                    ctx.var("sign_q")
                                                                                                        .set(Val::from(142))?;
                                                                                                    ctx.call(
                                                                                                        Function::GetExperience,
                                                                                                        vec![
                                                                                                            Val::from(2000000),
                                                                                                            Val::from(0),
                                                                                                        ],
                                                                                                    )?;
                                                                                                } else {
                                                                                                    ctx.lines(args![
                                                                                                        "Have you visited",
                                                                                                        "Engel Howard yet?",
                                                                                                        "He's the only one",
                                                                                                        "who can unlock the",
                                                                                                        "the Sign's power for you..."
                                                                                                    ])?;
                                                                                                }
                                                                                            } else {
                                                                                                if ctx.var("sign_q").get()? == 201 {
                                                                                                    ctx.lines(args![
                                                                                                        "You failed?",
                                                                                                        "It's disappointing,",
                                                                                                        "but I know that you",
                                                                                                        "did your best. Let",
                                                                                                        "me thank you for",
                                                                                                        "all your work."
                                                                                                    ])?;
                                                                                                    ctx.next()?;
                                                                                                    ctx.lines_as(
                                                                                                        "Metz",
                                                                                                        args![
                                                                                                            "Although I need to",
                                                                                                            "take back the Sobbing",
                                                                                                            "Starlight, I hope you",
                                                                                                            "accept this as a token",
                                                                                                            "of my gratitude. Good luck",
                                                                                                            "on your travels, adventurer."
                                                                                                        ],
                                                                                                    )?;
                                                                                                    if ctx.call(
                                                                                                        Function::CheckWeight,
                                                                                                        vec![Val::from(7178), Val::from(1)],
                                                                                                    )? == 0
                                                                                                    {
                                                                                                        ctx.next()?;
                                                                                                        ctx.lines(args!["^3355FFWait a second! Right now,", "you have too many items in your inventory. Please come back after you've freed up more inventory space.^000000"])?;
                                                                                                        ctx.close_window()?;
                                                                                                        return Err(Stop::End);
                                                                                                    }
                                                                                                    ctx.call(
                                                                                                        Function::DelItem,
                                                                                                        vec![Val::from(7178), Val::from(1)],
                                                                                                    )?;
                                                                                                    ctx.var("sign_q")
                                                                                                        .set(Val::from(202))?;
                                                                                                    ctx.call(
                                                                                                        Function::GetExperience,
                                                                                                        vec![
                                                                                                            Val::from(100000),
                                                                                                            Val::from(0),
                                                                                                        ],
                                                                                                    )?;
                                                                                                    ctx.call(
                                                                                                        Function::GetItem,
                                                                                                        vec![Val::from(617), Val::from(1)],
                                                                                                    )?;
                                                                                                } else if ctx
                                                                                                    .var("sign_q")
                                                                                                    .get()?
                                                                                                    .number()?
                                                                                                    > 201
                                                                                                {
                                                                                                    if ctx.call(
                                                                                                        Function::CountItem,
                                                                                                        vec![Val::from(7178)],
                                                                                                    )? == 1
                                                                                                    {
                                                                                                        ctx.lines(args![
                                                                                                            "I'm sorry...",
                                                                                                            "But I'm taking",
                                                                                                            "back the Sobbing",
                                                                                                            "Starlight from you.",
                                                                                                            "Apparently, you weren't",
                                                                                                            "worthy of the task..."
                                                                                                        ])?;
                                                                                                        ctx.next()?;
                                                                                                        ctx.lines_as(
                                                                                                            "Metz",
                                                                                                            args!["......", "Farewell."],
                                                                                                        )?;
                                                                                                        ctx.call(
                                                                                                            Function::DelItem,
                                                                                                            vec![
                                                                                                                Val::from(7178),
                                                                                                                Val::from(1),
                                                                                                            ],
                                                                                                        )?;
                                                                                                    } else {
                                                                                                        ctx.lines(args![
                                                                                                            "I guess I can't",
                                                                                                            "really blame you",
                                                                                                            "since I didn't provide",
                                                                                                            "you with that much help..."
                                                                                                        ])?;
                                                                                                    }
                                                                                                } else {
                                                                                                    ctx.lines(args![
                                                                                                        "Thank you so much",
                                                                                                        "for helping me make",
                                                                                                        "my dream come true.",
                                                                                                        "can work on another",
                                                                                                        "project together."
                                                                                                    ])?;
                                                                                                    ctx.next()?;
                                                                                                    ctx.lines_as(
                                                                                                        "Metz",
                                                                                                        args![
                                                                                                            "Well then...",
                                                                                                            "Good luck on",
                                                                                                            "your journeys,",
                                                                                                            ((Val::from("brave ")
                                                                                                                + ctx.call(
                                                                                                                    Function::StrCharInfo,
                                                                                                                    vec![Val::from(0)]
                                                                                                                )?)
                                                                                                                + Val::from("."))
                                                                                                        ],
                                                                                                    )?;
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
                }
            }
        }
    }
    ctx.close_window()?;
    ctx.call(Function::Cutin, vec![Val::from("mets_alpha"), Val::from(255)])?;
    return Err(Stop::End);
}

pub fn archeologist_sign(ctx: &Ctx) -> Script {
    archeologist_sign_body(ctx, Vec::new()).map(|_| ())
}

fn steward_sign_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    shared::quests_the_sign_quest::f_updatesignvars(ctx, vec![])?;
    ctx.mes("[Vandez]")?;
    if ctx.var("sign_q").get()?.number()? < 3 {
        ctx.lines(args!["Welcome to the", "Brayde Estate. How", "may I be of service?"])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Nothing.:I'm here to see Metz.:Gimmie your cash!")])? {
            1 => {
                ctx.lines_as(
                    "Vandez",
                    args![
                        "If you do not have",
                        "any business to conduct",
                        "with Master Metz, please",
                        "leave immediately."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Vandez",
                    args![
                        (Val::from("Very good, ")
                            + (if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                                Val::from("sir.")
                            } else {
                                Val::from("madam.")
                            })),
                        "Please wait a moment",
                        "while I consult with",
                        "the master in his study."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            3 => {
                ctx.lines_as(
                    "Vandez",
                    args![
                        "My apologies,",
                        "but I insist that",
                        "you leave the premises",
                        "^FF0000immediately^000000."
                    ],
                )?;
                ctx.close_window()?;
                ctx.call(Function::PercentHeal, vec![Val::from(-30), Val::from(0)])?;
                ctx.call(Function::Warp, vec![Val::from("prontera"), Val::from(150), Val::from(150)])?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else {
        if ctx.var("sign_q").get()?.number()? < 14 {
            ctx.lines(args![
                ((Val::from("Ah, Master ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(".")),
                "Welcome. How may I be",
                "of service today?"
            ])?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Who is Arian?:What is Metz doing?:How is Elle?")])? {
                1 => {
                    ctx.lines_as(
                        "Vandez",
                        args![
                            "Ah yes, Arian.",
                            "I consider him to be",
                            "a man of few words. The",
                            "words he does choose to",
                            "use are rather harsh and",
                            "brutish, you might say."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines_as(
                        "Vandez",
                        args![
                            "Although I've served",
                            "the Brayde family for years,",
                            "I'm unfamiliar with Master",
                            "Metz's work. My apologies, but",
                            "I simply cannot even begin to fathom his research..."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                3 => {
                    ctx.lines_as(
                        "Vandez",
                        args![
                            "Ah, Mistress Elle",
                            "has been working here",
                            "since she was a very young",
                            "girl. She is almost like",
                            "a granddaughter to me.",
                            "Ha ha-^333333*Ahem*^000000"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        } else {
            if ctx.var("sign_q").get()?.number()? < 13 {
                ctx.lines(args![
                    "I recall that",
                    "Master Arian visited",
                    "the master a while ago.",
                    "They spent all night",
                    "discussing something",
                    "related to the master's work."
                ])?;
                ctx.next()?;
                ctx.lines_as(
                    "Vandez",
                    args![
                        "Goodness, it sounded",
                        "dreadfully serious. The",
                        "term, 'ancient power' is",
                        "usually not mentioned",
                        "quite often in conversation."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("sign_q").get()?.number()? < 15 {
                    ctx.lines(args![
                        "Sometimes I worry",
                        "about Mistress Elle.",
                        "She's one of the most",
                        "beautiful girls in Prontera,",
                        "but does not have a boyfriend."
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Vandez",
                        args![
                            "Many men have given her",
                            "a great deal of attention, but",
                            "she always manages to reject",
                            "every single suitor..."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("sign_q").get()?.number()? < 20 {
                        ctx.lines(args![
                            "Many travellers have",
                            "begun to visit the master",
                            "lastly. Apparently, they've",
                            "been led here by the sign",
                            "boards we've posted."
                        ])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Vandez",
                            args![
                                "Take heed and",
                                "do not let anyone",
                                "else take the opportunity",
                                "the master has presented",
                                "away from you."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if ctx.var("sign_q").get()?.number()? < 25 {
                            ctx.lines(args![
                                "It's most curious,",
                                "but Elle seems to",
                                "consider that unrefined",
                                "Jesqurienne as her rival.",
                                "Women are truly difficult",
                                "for me to understand..."
                            ])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.var("sign_q").get()?.number()? < 35 {
                            ctx.lines(args![
                                "Avarice knows no bounds.",
                                "The pastime of gambling",
                                "seems to best represent",
                                "mankind's greed."
                            ])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Vandez",
                                args![
                                    "Winning or losing, some",
                                    "gamblers always seem to find",
                                    "some reason to continue when",
                                    "it's smarter to stop. Then again, the mind usually doesn't take precedence over the heart..."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.var("sign_q").get()?.number()? < 54 {
                            ctx.lines(args![
                                "Bakerlan is a young,",
                                "brilliant man. Although",
                                "he inherited his company,",
                                "it takes talent to continue",
                                "his father's successes."
                            ])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Vandez",
                                args![
                                    "It's amazing that Bakerlan",
                                    "is also managing to expand",
                                    "his business even further.",
                                    "Of course, Master Metz has",
                                    "had a role in Bakerlan's",
                                    "success..."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.var("sign_q").get()? == 54 {
                            ctx.lines(args![
                                "Oh, congratulations~",
                                "You finally passed all",
                                "of the tests. I know they",
                                "must have been incredibly",
                                "taxing and challenging."
                            ])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Vandez",
                                args![
                                    "Master Metz has informed",
                                    "me that he wishes for you",
                                    "to restore find someone",
                                    "to restore the Sobbing",
                                    "Starlight. I wish you luck",
                                    "in that endeavor."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if (ctx.var("sign_q").get()? == 97 || ctx.var("sign_q").get()? == 98) {
                            ctx.lines(args![
                                "Although humans make",
                                "mistakes, there are those",
                                "times when failing cannot",
                                "be an option."
                            ])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Vandez",
                                args![
                                    "There will always",
                                    "be situations where",
                                    "you won't be getting",
                                    "any second chances.",
                                    "Anyways remember that."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            ctx.lines(args![
                                "There will always",
                                "be situations where",
                                "you won't be getting",
                                "any second chances.",
                                "Anyways remember that."
                            ])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                }
            }
        }
    }
    Ok(Val::from(0))
}

pub fn steward_sign(ctx: &Ctx) -> Script {
    steward_sign_body(ctx, Vec::new()).map(|_| ())
}

fn maid_sign_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn maid_sign(ctx: &Ctx) -> Script {
    maid_sign_body(ctx, Vec::new()).map(|_| ())
}

fn maid_sign2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    shared::quests_the_sign_quest::f_updatesignvars(ctx, vec![])?;
    ctx.mes("[Elle]")?;
    if ctx.var("sign_q").get()?.number()? < 3 {
        ctx.lines(args![
            "Wh-what are you",
            "doing in Master Metz's",
            "room? Please leave",
            "immediately!"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("sign_q").get()?.number()? < 14 {
            ctx.lines(args![
                "Oh...?",
                ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("?")),
                "Is there anything that",
                "I can help you with?"
            ])?;
            ctx.next()?;
            if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                match runtime::select_values(ctx, &[Val::from("You wanna have coffee sometime?:Tell me about Metz.")])? {
                    1 => {
                        ctx.lines_as(
                            "Elle",
                            args!["...?", "*Blush~*", "Oh~ A-are you", "asking me out", "on a date?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Elle",
                            args![
                                "Weeeell~",
                                "Do I have the time",
                                "to go out with you or",
                                "not? Hm, maybe if",
                                "you're really nice",
                                ((Val::from("to me, , ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("."))
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as("Elle", args!["Umm....", "I don't know", "what to say?"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Elle",
                            args![
                                "He's my employer and",
                                "I have an opinion of him,",
                                "but if I told you, it might",
                                "look unprofessional. But",
                                "if you have time later, I just",
                                "might tell you. Ho ho~"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            } else {
                match runtime::select_values(ctx, &[Val::from("You have nice skin, Miss Elle.:Tell me about Metz.")])? {
                    1 => {
                        ctx.lines_as(
                            "Elle",
                            args![
                                "Eh...?",
                                "Why, thank you!",
                                "Maybe it's because",
                                "I don't go out too",
                                "often. You have nice",
                                ((Val::from("I don't go out too, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                    + Val::from("~"))
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as("Elle", args!["Umm....", "I don't know", "what to say?"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Elle",
                            args![
                                "He's my employer and",
                                "I have an opinion of him,",
                                "but if I told you, it might",
                                "look unprofessional. But",
                                "if you have time later, I just",
                                "might tell you."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            }
        } else {
            if ctx.var("sign_q").get()?.number()? < 13 {
                ctx.lines(args![
                    "I'm not so busy",
                    "nowadays, but I was",
                    "working hard for a while",
                    "so my body aches. Oh, what",
                    "am I saying? Please ignore",
                    "that--! Ho ho ho~"
                ])?;
                ctx.call(Function::Emotion, vec![ctx.constant("ET_PROFUSELY_SWEAT")?])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("sign_q").get()?.number()? < 15 {
                    ctx.lines(args![
                        "Guys never understand",
                        "what a girl really wants!",
                        "They're always too afraid",
                        "of showing their interest~"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Elle",
                        args![
                            "Well, even when they",
                            "do ask me on dates, I never",
                            "accept their proposals that",
                            "easily. Hmph, there's not",
                            "too many boys I like out",
                            "there, anyway."
                        ],
                    )?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_HNG")?])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("sign_q").get()?.number()? < 20 {
                        ctx.lines(args![
                            "^333333*Phew....*^000000",
                            "I'm so tired, even",
                            "though I didn't work",
                            "that much today. Some",
                            "days I feel exhausted",
                            "for no reason at all..."
                        ])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Elle",
                            args![
                                "I should take",
                                "the day off and",
                                "rest. If I had some",
                                "refreshment, that",
                                "would be perfect..."
                            ],
                        )?;
                        ctx.close_window()?;
                        if ctx.call(Function::CountItem, vec![Val::from(504)])?.number()? > 0 {
                            let choice = runtime::select_values(ctx, &[Val::from("Why don't you take this?")])?;
                            ctx.var("@menu").set(choice)?;
                            ctx.lines_as(
                                "Elle",
                                args!["Oh, you didn't", "need to do this,", "but that you soooo", "much! Ahhhhhhh~"],
                            )?;
                            ctx.call(Function::DelItem, vec![Val::from(504), Val::from(1)])?;
                            ctx.close_window()?;
                            ctx.call(Function::Emotion, vec![ctx.constant("ET_CHUPCHUP")?])?;
                            return Err(Stop::End);
                        }
                        return Err(Stop::End);
                    } else {
                        if ctx.var("sign_q").get()?.number()? < 25 {
                            ctx.lines(args![
                                "Jesqurienne?",
                                "Well, she's beautiful",
                                "and talented. She's actually",
                                "really great. That's why..."
                            ])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Elle",
                                args![
                                    "I consider her",
                                    "my rival! She might",
                                    "be smarter than me,",
                                    "but maybe I can become",
                                    "more beautiful than her..."
                                ],
                            )?;
                            ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.var("sign_q").get()?.number()? < 35 {
                            ctx.lines(args![
                                "Have you been",
                                "in Comodo before?",
                                "It's such a beautiful",
                                "place with so many",
                                "things to do!"
                            ])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Elle",
                                args![
                                    "I like gambling in",
                                    "Comodo, but I think",
                                    "it's a little addictive.",
                                    "I've even seen people",
                                    "who seem to live for it..."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.var("sign_q").get()?.number()? < 54 {
                            ctx.lines(args![
                                "I remember that",
                                "there was a lady",
                                "working for Master",
                                "Bakerlan named Seylin.",
                                "I hear she's from some",
                                "distant country..."
                            ])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Elle",
                                args![
                                    "I've seen her",
                                    "once and I think",
                                    "she has a unique",
                                    "style of beauty. Well,",
                                    "different than mine or",
                                    "Jesquienne's anyway..."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.var("sign_q").get()? == 54 {
                            ctx.call(Function::Emotion, vec![ctx.constant("ET_DELIGHT")?])?;
                            ctx.lines(args!["La la la~~", "Oh, hello.", "Long time no see.", "La la la~~"])?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("What's up?:What happened?")])? {
                                1 => {
                                    ctx.lines_as("Elle", args!["Oh...?", "Oops...!", "Do I really", "look that excited?"])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Elle",
                                        args![
                                            "I just started",
                                            "going out with someone!",
                                            "He's a tall handsome boy",
                                            "and I'm so in love with",
                                            "him and he's so...."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines(args![
                                        "^3355FFElle continued",
                                        "talking about how",
                                        "much she loves her",
                                        "new boyfriend, even",
                                        "after you leave the room.^000000"
                                    ])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                2 => {
                                    ctx.lines_as(
                                        "Elle",
                                        args![
                                            "Umm....",
                                            "Yes, something",
                                            "very good happened",
                                            "to me. Ho ho ho ho!",
                                            "But it's a secret~"
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                _ => {}
                            }
                        } else if ctx.var("sign_q").get()? == 97 {
                            ctx.lines(args!["Ah~ Hello!", "Nice to see", "you again~"])?;
                            ctx.next()?;
                            ctx.lines_as("Elle", args!["Er, I'm sorry, but I heard that something bad happened to you. But don't worry, maybe you'll have better luck next time~"])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            ctx.lines(args![
                                "^333333*Sigh...*^000000",
                                "Lately, there've",
                                "been too many guests",
                                "visiting Master Metz.",
                                "Why can't they leave their",
                                "dirt outside of the manor?"
                            ])?;
                            ctx.call(Function::Emotion, vec![ctx.constant("ET_HNG")?])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                }
            }
        }
    }
    Ok(Val::from(0))
}

pub fn maid_sign2(ctx: &Ctx) -> Script {
    maid_sign2_body(ctx, Vec::new()).map(|_| ())
}

use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn prime_minister_dmitree_m_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("mos_nowinter").get()?.number()? > 11 && ctx.var("mos_nowinter").get()?.number()? < 14) {
        ctx.lines_as(
            "Prime Minister Dmitree",
            args![
                "You are in trouble if",
                "you conspire against the Csar,",
                "so, make your actions carefully,",
                "and if you want to clean yourself",
                "from suspicion... do everything you",
                "can to prove your innocence."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("mos_nowinter").get()? == 14 {
        ctx.lines_as(
            "Prime Minister Dmitree",
            args!["You have just cleared your name,", "but... we don't trust you wholly yet."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Prime Minister Dmitree",
            args![
                "I can't trust you, but...",
                "if you succeed in taking hold of",
                "the summer... I might be able to."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("mos_nowinter").get()? == 20 {
        ctx.lines_as(
            "Prime Minister Dmitree",
            args![
                "We know that you mean well.",
                "But we can't fully trust you,",
                "so we kept an eye on your movements."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Prime Minister Dmitree",
            args![
                "I heard that you performed",
                "witchcraft at the palace square...",
                "so, I ordered scholars to",
                "investigate the weather."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Prime Minister Dmitree",
            args![
                "The results from taking observation",
                "of the weather, are the reducing",
                "change of temperature, and that the",
                "highest temperature is now stabilized."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Prime Minister Dmitree",
            args![
                "I don't know what this may mean, by",
                "way of principle, but I've decided",
                "to admit the facts."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Prime Minister Dmitree",
            args![
                "Already, our dear Csar knows.",
                "I finished the report.",
                "Go and announce it to him."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("mos_whale_edq").get()?.number()? < 16 {
        ctx.lines_as("Prime Minister Dmitree", args!["You are a foreign traveler. This is the Moscovia Palace, home of Csar Aleksay the Third. Extending your every courtesy here... is not bad manners."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("mos_whale_edq").get()? == 16 {
            ctx.lines_as(
                "Prime Minister Dmitree",
                args!["Traveler, why have you come to the Csar's Palace?"],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Just to look around.:To see the dear Csar.")],
            )?) == 1
            {
                ctx.lines_as(
                    "Prime Minister Dmitree",
                    args!["If so... look around with caution. Do not bother the Csar."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Prime Minister Dmitree",
                    args!["Moscovia welcomes all travelers such as yourself!"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Prime Minister Dmitree",
                    args!["Marvelously, our dear Csar likes it very much when interesting stories are often told to him."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Prime Minister Dmitree",
                args!["Did you come to see our dear Csar? If so, tell me your business with him in advance."],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("I'd like to say hello.:I have an adventure story for him.")],
            )?) == 1
            {
                ctx.lines_as(
                    "Prime Minister Dmitree",
                    args!["I will tell him directly about your business. You don't need to worry yourself with the Csar."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Prime Minister Dmitree",
                args!["I wonder what sort of things have happened to you... The Csar will be very pleased to hear your stories."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Prime Minister Dmitree",
                args!["I will admit you. If our dear Csar is satisfied with your story, he will give you a big prize."],
            )?;
            ctx.var("mos_whale_edq").set(Val::from(17))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("mos_whale_edq").get()? == 17 {
                ctx.lines_as(
                    "Prime Minister Dmitree",
                    args!["I already announced you,", "so go see him and speak to him directly."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if (ctx.var("mos_whale_edq").get()?.number()? > 17 && ctx.var("mos_whale_edq").get()?.number()? < 35) {
                    ctx.lines_as(
                        "Prime Minister Dmitree",
                        args![
                            "Now, our dear Csar needs a rest.",
                            "You should do your duty and go find the whale island."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if (ctx.var("mos_whale_edq").get()? == 35 || ctx.var("mos_whale_edq").get()? == 38) {
                    ctx.lines_as(
                        "Prime Minister Dmitree",
                        args!["Ah! You've come back.", "I will request for you to see him immediately."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("mos_whale_edq").get()? == 39 {
                    ctx.lines_as(
                        "Prime Minister Dmitree",
                        args!["I don't want to hear your terrible ", "performance anymore so, go out away."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("mos_whale_edq").get()? == 40 {
                    ctx.lines_as(
                        "Prime Minister Dmitree",
                        args!["What's up? Dear Chare is taking a rest.", "If you have nothing special, go out."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("mos_whale_edq").get()? == 41 {
                    ctx.lines_as(
                        "Prime Minister Dmitree",
                        args![
                            "Your Gusli performance was so touching.",
                            "Unforgettable! Whenever you are here, please allow us to hear that tune again."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        }
    }
    ctx.lines_as(
        "Prime Minister Dmitree",
        args![
            "You are a foreign traveler. This is a palace",
            "which lives Alexei the Third.",
            "Extend you every courtesy don't not bad manner."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn prime_minister_dmitree_m(ctx: &Ctx) -> Script {
    prime_minister_dmitree_m_body(ctx, Vec::new()).map(|_| ())
}

fn find_ship_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn find_ship(ctx: &Ctx) -> Script {
    find_ship_body(ctx, Vec::new()).map(|_| ())
}

fn find_ship_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("mos_whale_edq").get()?.number()? < 35 {
        return Err(Stop::End);
    }
    ctx.lines(args![
        "-Watching the sea from the docks,",
        "it suddenly dawns upon you that you",
        "have memories from Whale Island.-"
    ])?;
    ctx.next()?;
    ctx.lines_as(
        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
        args!["Stop by the whale island?"],
    )?;
    ctx.next()?;
    if Val::from(runtime::select_values(ctx, &[Val::from("Go to Whale Island.:Stay put.")])?) == 1 {
        if ctx.call(Function::IsEquipped, vec![Val::from(2707)])?.is_true() {
            ctx.call(Function::SoundEffect, vec![Val::from("mos_gusli2.wav"), Val::from(0)])?;
            ctx.lines(args![
                "-Slowly, your hands are on the",
                "Gusli, and the playing starts...",
                "reminding you of the melody which",
                "the aged stranger had played...-"
            ])?;
            ctx.next()?;
            ctx.lines_as("Village resident", args!["So...something is rising from the sea!!!"])?;
            ctx.next()?;
            ctx.lines_as("Village Youth", args!["What...What is that...??"])?;
            ctx.next()?;
            ctx.lines_as(
                "Mr. Ibanoff",
                args!["Ohohoh!... That's the whale", "island...!!! Someday, I hope to go there! Hahaha."],
            )?;
            ctx.next()?;
            ctx.call(Function::Warp, vec![Val::from("mosk_fild01"), Val::from(95), Val::from(93)])?;
        } else {
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "Oh my goodness... Slipped right out",
                    "of my mind... to forget equipping the Gusli."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "The aged stranger said that when I",
                    "want to go to Whale Island again, I",
                    "should play the Gusli from this place..."
                ],
            )?;
        }
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
        args!["I can go some other time.", "I will do other work now."],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn find_ship_ontouch(ctx: &Ctx) -> Script {
    find_ship_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn findship_run(ctx: &Ctx, mut step: FindshipStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            FindshipStep::Start => {
                step = FindshipStep::OnEnable;
                continue 'machine;
            }
            FindshipStep::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("#findship")])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            FindshipStep::OnTimer300000 => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                step = FindshipStep::OnInit;
                continue 'machine;
            }
            FindshipStep::OnInit => {
                ctx.var("$@mos1_edq").set(Val::from(0))?;
                ctx.call(Function::DisableNpc, vec![Val::from("#findship")])?;
                return Err(Stop::End);
            }
            FindshipStep::OnTouch => {
                if ctx.var("mos_whale_edq").get()? == 12 {
                    ctx.mes("seeewaaaaaaaaaaa")?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mr. Ibanoff",
                        args![
                            ((Val::from("Hey ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("! Look!")),
                            "It's dangerous! Hide! Hurry!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["That... that is..."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Something... something is rising..."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mr. Ibanoff",
                        args![
                            ((Val::from("Watch out! ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("!")),
                            "Ah... That... That is... What..."
                        ],
                    )?;
                    ctx.var("mos_whale_edq").set(Val::from(13))?;
                    if ctx.call(Function::IsBeginQuest, vec![Val::from(18108)])?.is_true() {
                        ctx.call(Function::ChangeQuest, vec![Val::from(18108), Val::from(18109)])?;
                    }
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("mosk_fild01"), Val::from(95), Val::from(93)])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn findship(ctx: &Ctx) -> Script {
    findship_run(ctx, FindshipStep::Start, Vec::new()).map(|_| ())
}

pub fn findship_onenable(ctx: &Ctx) -> Script {
    findship_run(ctx, FindshipStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn findship_ontimer300000(ctx: &Ctx) -> Script {
    findship_run(ctx, FindshipStep::OnTimer300000, Vec::new()).map(|_| ())
}

pub fn findship_oninit(ctx: &Ctx) -> Script {
    findship_run(ctx, FindshipStep::OnInit, Vec::new()).map(|_| ())
}

pub fn findship_ontouch(ctx: &Ctx) -> Script {
    findship_run(ctx, FindshipStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn baehideun_main(ctx: &Ctx) -> Script {
    baehideun_main_run(ctx, BaehideunMainStep::Start, Vec::new()).map(|_| ())
}

pub fn baehideun_main_oninit(ctx: &Ctx) -> Script {
    baehideun_main_run(ctx, BaehideunMainStep::OnInit, Vec::new()).map(|_| ())
}

pub fn baehideun_main_onenable(ctx: &Ctx) -> Script {
    baehideun_main_run(ctx, BaehideunMainStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn baehideun_main_ontimer300000(ctx: &Ctx) -> Script {
    baehideun_main_run(ctx, BaehideunMainStep::OnTimer300000, Vec::new()).map(|_| ())
}

pub fn baehideun_main_ondisable(ctx: &Ctx) -> Script {
    baehideun_main_run(ctx, BaehideunMainStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn baehideun_main_onmymobdead(ctx: &Ctx) -> Script {
    baehideun_main_run(ctx, BaehideunMainStep::OnMyMobDead, Vec::new()).map(|_| ())
}

fn gallina_mos_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if !(ctx.var("mos_swan").get()?.is_true()) {
        shared::quests_quests_moscovia::f_mos_1(ctx, vec![])?;
    } else if ctx.var("mos_swan").get()? == 1 {
        ctx.lines_as(
            "Gallina",
            args![
                "Mikhail, my timid son must still be in this village.",
                "I'm sorry if he's shy and timid like his father."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("mos_swan").get()?.number()? < 25 {
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Mikhail hasn't come yet?"],
        )?;
        ctx.next()?;
        ctx.lines_as("Gallina", args!["No, where on earth is he?"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("mos_swan").get()? == 25 {
        ctx.lines_as(
            "Gallina",
            args!["He came back with my Matrushka roughly pasted together and just left..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Gallina",
            args!["I'm sorry that I treated the little boy badly.", "He did his best in his own way."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["That's ok. You will be a sweet mom to your little boy. haha"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Gallina",
            args![
                "Yes. I'm sure I will. hoho..",
                "Anna, my daughter made a mistake. She laid the blame upon her brother alone. I'll punish her too."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Gallina",
            args!["I appreciate your effort. You went to a dangerous place to find my son"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["It often happens to me, hehe."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Gallina",
            args!["I can't afford to reward you with much but... I'll let you know how to make a delicious hotcake!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Gallina",
            args![
                "I have to work now but Larissa will tell you about that. She's our maid..",
                "She's really a nice cook."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Gallina",
            args![
                "Hotcakes of Moscovia are so delicious!",
                "Once you make it, I bet you that you'll love it."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Gallina",
            args![
                "Well, now I have to get to work!",
                "I'll make you my hotcakes someday. Please visit me later"
            ],
        )?;
        ctx.var("mos_swan").set(Val::from(100))?;
        ctx.call(Function::CompleteQuest, vec![Val::from(18069)])?;
        ctx.call(Function::GetExperience, vec![Val::from(1000000), Val::from(0)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Gallina",
        args![
            "I'm always trying to prepare a new dish.",
            "What do you think of 'the most spicy chili hotcake in the world'?",
            "I think that will be great!"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn gallina_mos(ctx: &Ctx) -> Script {
    gallina_mos_body(ctx, Vec::new()).map(|_| ())
}

fn anna_mos_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_annainfo_s = Val::from("");
    if !(ctx.var("mos_swan").get()?.is_true()) {
        shared::quests_quests_moscovia::f_mos_1(ctx, vec![])?;
    } else {
        if ctx.var("mos_swan").get()? == 1 {
            ctx.lines_as(
                "Anna",
                args!["Mikhail, he's a coward, a crybaby.", "Mikhail, he's a coward, a crybaby."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("mos_swan").get()? == 2 {
                ctx.lines_as("Anna", args!["Why are you here?"])?;
                ctx.next()?;
                'l1: loop {
                    if !(true) {
                        break 'l1;
                    }
                    'b1: {
                        match runtime::select_values(
                            ctx,
                            &[Val::from(
                                "About where Mikhail might be hiding:About their relationship:About the situation:Ask her about other things:End the conversation",
                            )],
                        )? {
                            1 => {
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args!["Where is Mikhail?", "Do you know where he is?"],
                                )?;
                                ctx.next()?;
                                ctx.call(Function::Emotion, vec![ctx.constant("ET_MERONG")?])?;
                                ctx.lines_as("Anna", args!["If I had known that, I would have already found him, you fool."])?;
                                ctx.next()?;
                                ctx.call(
                                    Function::Emotion,
                                    vec![
                                        ctx.constant("ET_SWEAT")?,
                                        Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                                    ],
                                )?;
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args!["Hahaha, you're right."],
                                )?;
                            }
                            2 => {
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args!["How's your relationship with your brother?"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Anna",
                                    args![
                                        "Mikhail always stays behind me and asks me to read books!",
                                        "And he cries too much.",
                                        "It annoys me.",
                                        "And he only wants to play with me. That's why he has no friends."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args!["You seem very courageous.", "I can understand why your mother worries about him."],
                                )?;
                            }
                            3 => {
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args!["What were you doing when Mikhail broke your mother's Matrushka?"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Anna", args!["I was there."])?;
                                ctx.next()?;
                                ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Oh were you?"])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Anna",
                                    args![
                                        "When my parents were out, they asked me to take care of Mikhail.",
                                        "I didn't want to do that but I'm his sister so I was reading a book next to him."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Anna",
                                    args!["He began to tease me to read a book for him and later, he annoyed me saying that he's hungry!"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Anna", args!["I was so bothered with him and I pretended not to hear him and kept reading. I guess it made him mad and so he broke the Matrushka."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Anna",
                                    args!["He made trouble and began to cry!", "He's such a timid boy...", "(giggling)"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Anna", args!["And then I told him that we should glue the pieces together before Mom came back, so Mikhail went to get ^3131FFpaste^000000 but he hasn't come back yet."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args!["Where can you get it?", "Why didn't you go with him?"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Anna",
                                    args![
                                        "Do you think we kids know that?",
                                        "He broke it and should get the thing by himself!",
                                        "And Mom said that he should do his work for himself to become a great general."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args!["Don't you have any idea of where he might be?"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Anna", args!["No."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args!["Is there anyone who is close to him?"],
                                )?;
                                ctx.next()?;
                                ctx.call(Function::Emotion, vec![ctx.constant("ET_HNG")?])?;
                                ctx.lines_as(
                                    "Anna",
                                    args![
                                        "(giggle) He's a fool and has no friends.",
                                        "But among our villagers, the lady of ^3131FFInn 'Sticky Herb Tree'^000000 has held Mikhail dear."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Anna", args!["I have no idea anymore."])?;
                                ctx.next()?;
                                ctx.mes("- Anna sticks her tongue out. -")?;
                                ctx.next()?;
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args!["It's the only clue, I guess I'll go to ^3131FFInn 'Sticky Herb Tree'^000000?"],
                                )?;
                                if ctx.var("mos_swan").get()? == 2 {
                                    ctx.var("mos_swan").set(Val::from(3))?;
                                    ctx.call(Function::ChangeQuest, vec![Val::from(18060), Val::from(18061)])?;
                                }
                            }
                            4 => {
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args!["What am I going to ask her?"],
                                )?;
                                ctx.next()?;
                                let (input, status) = runtime::input_text(ctx, None, None)?;
                                l_annainfo_s = input;
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args![
                                        "Anna,",
                                        ((Val::from("") + l_annainfo_s.clone()) + Val::from("?")),
                                        "Do you know what this is?"
                                    ],
                                )?;
                                ctx.next()?;
                                if l_annainfo_s.clone() == "Gravity" {
                                    ctx.lines_as("Anna", args!["I know that they have a game called RAGNAROK Online 2... and they like... what am I saying now... right?"])?;
                                } else if l_annainfo_s.clone() == "Gallina" {
                                    ctx.lines_as(
                                        "Anna",
                                        args![
                                            "Mom's hotcakes are so delicious! Oh, it's sweet flavor!",
                                            "Everyone in this village likes Mom's hotcakes. Hehe."
                                        ],
                                    )?;
                                } else if l_annainfo_s.clone() == "Mikhail" {
                                    ctx.lines_as("Anna", args!["Mikhail is a timid fool!"])?;
                                } else {
                                    ctx.lines_as("Anna", args!["I have no idea about that thing."])?;
                                }
                            }
                            5 => {
                                ctx.lines_as("Anna", args!["Hmm ~ ~ ~ ~ "])?;
                                ctx.next()?;
                                ctx.lines(args![
                                    "- Anna frowns at me and sticks out her tongue.",
                                    "She doesn't want to talk to me -"
                                ])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                        ctx.next()?;
                    }
                }
            } else if ctx.var("mos_swan").get()? == 3 {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["Ok, I got a clue from Anna and one thing that I have to do is go to ^3131FFInn 'Sticky Herb Tree'^000000."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("mos_swan").get()?.number()? < 12 {
                ctx.lines_as("Anna", args!["Mikhail is foolish, timid and a coward!"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("mos_swan").get()? == 25 {
                ctx.lines_as("Anna", args!["Mik..ha..il", "He tattled on me to mom..."])?;
                ctx.next()?;
                ctx.lines_as("Gallina", args!["Anna!"])?;
                ctx.next()?;
                ctx.lines_as("Anna", args!["....I'm sorry."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("mos_swan").get()? == 100 {
                ctx.lines_as("Anna", args!["Mik..ha..il", "He tattled on me to mom..."])?;
                ctx.next()?;
                ctx.lines_as("Gallina", args!["Anna!"])?;
                ctx.next()?;
                ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                ctx.lines_as("Anna", args!["....I'm sorry."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
    ctx.lines_as("Anna", args!["....I'm bored."])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn anna_mos(ctx: &Ctx) -> Script {
    anna_mos_body(ctx, Vec::new()).map(|_| ())
}

fn bed_mos1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.mes("- There are sheets and a pillow which seem so neat and soft that I'll probably fall asleep as soon as I lie down on them. -")?;
    if ctx.var("mos_swan").get()? != 11 {
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.next()?;
    if Val::from(runtime::select_values(ctx, &[Val::from("Check other things:Look it over ")])?) == 1 {
        ctx.mes("- You'll examine the bed later -")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.mes("- You come near the bed to look it over. -")?;
    ctx.next()?;
    ctx.lines(args![
        " .............. ",
        " .............. ",
        " .............. ",
        " .............. "
    ])?;
    ctx.next()?;
    ctx.lines(args![
        " .............. ",
        " .............. ",
        " .............. ",
        " .............. ",
        " .............. "
    ])?;
    ctx.next()?;
    ctx.mes("- You didn't find anything -")?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn bed_mos1(ctx: &Ctx) -> Script {
    bed_mos1_body(ctx, Vec::new()).map(|_| ())
}

fn fire_pot_mos_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if !(ctx.var("mos_swan").get()?.is_true()) {
        ctx.mes("- It's a fire pot to heat the room or bake something -")?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("mos_swan").get()? == 1 {
            ctx.lines(args![
                "- It's a fire pot that is used when Gallina bakes hotcakes.",
                "It seems that this has not been used for a long time.",
                "I think I should ask his family where he might have fun off to -"
            ])?;
            ctx.var("mos_swan").set(Val::from(2))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("mos_swan").get()?.number()? < 11 {
            ctx.lines(args![
                "- It's a fire pot that is used when Gallina bakes hotcakes.",
                "It seems that it was used a long time ago. -"
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("mos_swan").get()? == 11 {
            ctx.lines(args![
                "- It's a fire pot that is used when Gallina bakes hotcakes.",
                "It seems that it was used a long time ago. -"
            ])?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("Check other things:Look it over")])?) == 1 {
                ctx.mes("- You decide to check out other things -")?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.mes("- You come near the pot to look it over. -")?;
            ctx.next()?;
            ctx.lines(args![
                " .............. ",
                " .............. ",
                " .............. ",
                " .............. "
            ])?;
            ctx.next()?;
            ctx.lines(args![
                " .............. ",
                " .............. ",
                " .............. ",
                " .............. ",
                " .............. "
            ])?;
            ctx.next()?;
            ctx.mes("- As you look it over very carefully, you find some pieces of bread on the floor around the fire pot! -")?;
            ctx.next()?;
            match runtime::select_values(
                ctx,
                &[Val::from(
                    "Mikhail is in the fire pot.:Mikhail is around the fire pot.:There's nothing between pieces of bread and Mikhail.",
                )],
            )? {
                1 => {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["I finally found him."],
                    )?;
                    ctx.next()?;
                    ctx.mes("- You put your arm into the hole of the fire pot. -")?;
                    ctx.next()?;
                    ctx.mes("- Something makes a rustling sound. -")?;
                    ctx.next()?;
                    ctx.mes("- You call Mikhail with a low voice. -")?;
                    ctx.next()?;
                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Mik-ha-il-."])?;
                    ctx.next()?;
                    ctx.mes("- .......................... -")?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Mikhail.", "I know you're there, please come out."],
                    )?;
                    ctx.next()?;
                    ctx.mes("- .......................... -")?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Your mom and sister are worried about you."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("???", args![".......hey."])?;
                    ctx.next()?;
                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["?????"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mikhail",
                        args![".......No, I can't............", "I'll.... be punished........"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "No, you won't, Mikhail.",
                            "Your mom is worried about you so much.",
                            "Your sister, too."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "You broke your mom's Matrushka by mistake, didn't you?",
                            "I'll tell her about your mistake. Please come out."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mikhail",
                        args!["Oh.. ma.. matrushka..", "I didn't break..it.. it..wasn't.. just me!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["And?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mikhail",
                        args![
                            "Anna pushed me and bumped me......",
                            "But she told Mom that it was just me who broke it...."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Oh, dear! You and Anna did that but she put all the blame on you?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Mikhail", args!["Yes....."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["You should come out and tell your mom the truth! Let's go Mikhail."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Mikhail", args!["No! No, I can't!!"])?;
                    ctx.next()?;
                    ctx.mes("- A small-white hand comes out of the fire pot and grabs your ankle -")?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mikhail",
                        args![
                            "No...I'm afraid that Mom will punish me...",
                            "Because Grandma's Matrushka is broken......"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Oh, my. Please don't cry little boy.", "There's no way. I'll get a paste to bond all the pieces together. And then you can bring it back to your mom and apologize to her."])?;
                    ctx.next()?;
                    ctx.lines_as("Mikhail", args!["Can you... get... a paste?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["You don't believe me?", "Ok, stay here. I'll be right back with the paste."],
                    )?;
                    ctx.next()?;
                    ctx.lines(args![
                        "- I need to know what's required for the paste.",
                        "I'll ask that guy ^3131FFMr. Victor^000000 about them. -"
                    ])?;
                    ctx.var("mos_swan").set(Val::from(12))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(18064), Val::from(18065)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.mes("- I guess he's near the fire pot. Where can I find him? -")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                3 => {
                    ctx.mes("- I guess pieces of bread have nothing to do with Mikhail. -")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        } else if ctx.var("mos_swan").get()? == 12 {
            ctx.mes("- Ok, what I have to do first is ask Mr. Victor what I need for the paste. -")?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("mos_swan").get()? == 24 {
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Hey, Mikhail. I've got the paste!."],
            )?;
            ctx.next()?;
            ctx.lines_as("Mikhail", args!["......Really? are you serious?"])?;
            ctx.next()?;
            ctx.call(Function::DoNpcEvent, vec![Val::from("Mikhail#mos::OnEnable")])?;
            ctx.mes("- You hand over the paste from Victor to Mikhail. -")?;
            ctx.next()?;
            ctx.lines_as("Mikhail", args!["Wow! Great!"])?;
            ctx.next()?;
            ctx.mes("- He begins working on the broken Matrushka as if he was piecing together a puzzle. -")?;
            ctx.next()?;
            ctx.lines_as("Mikhail", args!["I've done it!!!!!!!", "Thank you so much!!!"])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["You're welcome, haha.", "Anyway, can you promise me one thing?"],
            )?;
            ctx.next()?;
            ctx.lines_as("Mikhail", args!["What?"])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Bring back the Matrushka to your mother, and promise me that you will be a brave general in the future."],
            )?;
            ctx.next()?;
            ctx.lines_as("Mikhail", args!["............."])?;
            ctx.next()?;
            ctx.lines_as("Mikhail", args!["OK, I will!"])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Hoohoo, you're good boy."],
            )?;
            ctx.next()?;
            ctx.lines_as("Mikhail", args!["Now I'm gonna give this back to Mom."])?;
            ctx.call(Function::DoNpcEvent, vec![Val::from("Mikhail#mos::OnInit")])?;
            ctx.var("mos_swan").set(Val::from(25))?;
            ctx.call(Function::DelItem, vec![Val::from(7764), Val::from(1)])?;
            ctx.call(Function::ChangeQuest, vec![Val::from(18068), Val::from(18069)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    ctx.mes("- It's a fire pot to heat the room or bake something -")?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn fire_pot_mos(ctx: &Ctx) -> Script {
    fire_pot_mos_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum MikhailMosStep {
    Start,
    OnInit,
    OnEnable,
}

fn mikhail_mos_run(ctx: &Ctx, mut step: MikhailMosStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MikhailMosStep::Start => {
                return Err(Stop::End);
            }
            MikhailMosStep::OnInit => {
                step = MikhailMosStep::OnEnable;
                continue 'machine;
            }
            MikhailMosStep::OnEnable => {
                ctx.call(Function::DisableNpc, vec![Val::from("Mikhail#mos")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn mikhail_mos(ctx: &Ctx) -> Script {
    mikhail_mos_run(ctx, MikhailMosStep::Start, Vec::new()).map(|_| ())
}

pub fn mikhail_mos_oninit(ctx: &Ctx) -> Script {
    mikhail_mos_run(ctx, MikhailMosStep::OnInit, Vec::new()).map(|_| ())
}

pub fn mikhail_mos_onenable(ctx: &Ctx) -> Script {
    mikhail_mos_run(ctx, MikhailMosStep::OnEnable, Vec::new()).map(|_| ())
}

fn landlord_mos_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Landlord",
        args![
            "Oh, welcome to the Inn 'Sticky Herb Tree'.",
            "It is the most comfortable and calmest place in all of Moscovia."
        ],
    )?;
    ctx.next()?;
    if (ctx.var("mos_swan").get()? == 4 || ctx.var("mos_swan").get()? == 5) {
        ctx.lines_as(
            "Landlord",
            args![
                "You're up already?",
                "Well since you're young you've probably already recovered all of your strength.",
                "Hohoho."
            ],
        )?;
        ctx.next()?;
        'l1: loop {
            if !(true) {
                break 'l1;
            }
            'b1: {
                match runtime::select_values(
                    ctx,
                    &[Val::from("Ask her about the inn:Ask her about Mikhail.:End the conversation")],
                )? {
                    1 => {
                        ctx.lines_as("Landlord", args!["The name of our inn came from a big apple tree which is outside of the village.", "I hope our inn will be the most famous place in Moscovia like the apple tree that distinguishes itself in the thick wood."])?;
                        ctx.next()?;
                    }
                    2 => {
                        ctx.lines_as(
                            "Landlord",
                            args![
                                "Mikhail?",
                                "Ah! You're talking about a cute boy, Mr. Ibanoff's son?",
                                "Yes I saw him. He was here a few hours ago."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Landlord",
                            args!["He looked pale and asked me for a high-strength adhesive. He might've broken something important."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Landlord",
                            args![
                                "I felt sorry for him because he was so tense. He's such a sweet boy. I'd like to make him my son.",
                                "Hohoho"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["So what happened?", "Did Mikhail get it?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Landlord",
                            args![
                                "Hoho, I don't really know. Maybe my husband does...",
                                "He's not much to look at, but he's good at repairing and making stuff! hoho"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Landlord",
                            args!["But it's been a long time since he was out of Moscovia so I can't ask him how to make adhesives now..."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["......And then?"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Landlord",
                            args![
                                "I told him to go to the pub over there.",
                                "I also told him not to drink what those guys offer to him!",
                                "Cause he's such a cute boy!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["So, did he go there?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Landlord",
                            args!["Yes he did-.", "As he's a good boy, he would go there.", "He's really gentle."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["OK. Thank you for your answers!"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Landlord",
                            args![
                                "I was unkind to make you stay here but wasn't it easier for you? hoho.",
                                "When it gets dark, promise me to come here again. Hoho"
                            ],
                        )?;
                        ctx.var("mos_swan").set(Val::from(7))?;
                        ctx.call(Function::ChangeQuest, vec![Val::from(18061), Val::from(18062)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    3 => {
                        ctx.lines_as("Landlord", args!["When you need to take a rest, where will you go?"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Landlord",
                            args![
                                "You should come back here to Inn 'Sticky Herb Tree'. Ok?",
                                "I'll offer you better service next time."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            }
        }
    }
    'b3: {
        match runtime::select_values(
            ctx,
            &[
                Val::from("Save"),
                Val::from("Stay the night - 5000z"),
                (if ctx.var("mos_swan").get()? == 3 {
                    Val::from("Ask her about Mikhail.")
                } else {
                    Val::from("")
                }),
            ],
        )? {
            1 => {
                ctx.lines_as(
                    "Landlord",
                    args!["Your respawn point has been saved.", "Hope we can see you again next time hoho."],
                )?;
                ctx.call(
                    Function::SavePoint,
                    vec![Val::from("mosk_in"), Val::from(142), Val::from(189), Val::from(1), Val::from(1)],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                if ctx.var("Zeny").get()?.number()? > 4999 {
                    ctx.mes("[Landlord]")?;
                    if ctx.var("mos_swan").get()? == 3 {
                        ctx.lines(args![
                            "Ok, I'll bring you the best room.",
                            "Please have a rest, young adventurer."
                        ])?;
                    } else {
                        ctx.mes("Please be comfortable.")?;
                    }
                    break 'b3;
                }
                ctx.lines_as(
                    "Landlord",
                    args![
                        "The service charge is 5000z.",
                        "Please make sure you have enough money for the service."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            3 => {
                ctx.mes("[Landlord]")?;
                ctx.call(Function::Emotion, vec![ctx.constant("ET_THROB")?])?;
                ctx.lines(args![
                    ((Val::from("Oh~! You look great! Look at the ")
                        + (if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                            Val::from("solid muscle")
                        } else {
                            Val::from("fair skin")
                        }))
                        + Val::from("!")),
                    "But you look tired. Is it because of a long journey?"
                ])?;
                ctx.next()?;
                ctx.lines_as(
                    "Landlord",
                    args![
                        (Val::from("We've got a room available just for you. ")
                            + (if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                                Val::from("It's the best in town. How about staying the night?")
                            } else {
                                Val::from("It's like a princess' room.")
                            })),
                        (if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                            Val::from("I'll charge you at reasonable price for you, handsome guy.")
                        } else {
                            Val::from("How about staying the night? I'll mark down the price for you, beautiful lady.")
                        })
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "Oh, I'm sorry but I didn't come to stay here.",
                        "I'm looking for a kid and I've got something to ask you..."
                    ],
                )?;
                ctx.next()?;
                ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
                ctx.lines_as("Landlord", args!["What!?!?"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Landlord",
                    args![
                        "Hey~ You've heard about 'give and take'?",
                        "Haven't you?",
                        "If you're my customer, I could offer you what you want but you're not!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Landlord",
                    args!["If you promise me to stay overnight, I'll tell you about what you want to know. Deal?"],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("Refuse.:Stay overnight and get the info. - 5000z")],
                )?) == 1
                {
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_ANGER")?])?;
                    ctx.lines_as("Landlord", args!["Well, get the info by yourself then."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ctx.var("Zeny").get()?.number()? < 5000 {
                    ctx.lines_as(
                        "Landlord",
                        args!["No way. You don't have enough money.", "Go away! I can't offer you a room."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Landlord",
                    args!["Oh, God! You're as great as you look~", "Ok, I'll bring you to the room in a bit!!"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Landlord",
                    args![
                        "Come on, please follow me with your luggage.",
                        "Oh, let me carry them. Hohoho, you must be exhausted.",
                        "I wish you a good night's rest. Hohoho."
                    ],
                )?;
                ctx.var("mos_swan").set(Val::from(4))?;
            }
            _ => {}
        }
    }
    ctx.close_window()?;
    ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(5000))?))?;
    ctx.call(Function::PercentHeal, vec![Val::from(100), Val::from(100)])?;
    ctx.call(Function::Warp, vec![Val::from("mosk_in"), Val::from(215), Val::from(181)])?;
    return Err(Stop::End);
}

pub fn landlord_mos(ctx: &Ctx) -> Script {
    landlord_mos_body(ctx, Vec::new()).map(|_| ())
}

fn pub_owner_mos_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_j = Val::from(0);
    let mut l_s = Val::from(0);
    if (ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 2000 {
        ctx.lines(args![
            "- Please stop here!! -",
            "- You're carrying too many items -",
            "- Please try again -",
            "- after using the kafra service -"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("mos_swan").get()? == 7 {
        ctx.lines_as(
            "Pub Owner",
            args![
                "We've got another wanderer here",
                "Welcome to our pub.",
                "I'm Alexandre of ^3131FF'Pub Stream'^000000. You can call me 'Sasha'."
            ],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from("Ask him about Mikhail.:Order a drink.")],
        )?) == 1
        {
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Sasha, has a little boy called 'Mikhail' came here?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Pub Owner",
                args![
                    "Are you talking about the cute little boy? Yes he has came here.",
                    "He looked pale and needed a high-strength adhesive ."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["So what did you say ?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Pub Owner",
                args![
                    "I don't know well about that thing.",
                    "However, I know a person who may know about that so I introduced him to Mikhail."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Who's that person?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Pub Owner",
                args![
                    "It's Victor over there.",
                    "He's very careful about details so we call him ^3131FF'hedgehog Victor'^000000.",
                    "No one knows about it well except him."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Sasha, thanks a lot!"],
            )?;
            ctx.next()?;
            ctx.lines_as("Pub Owner", args!["You're welcome."])?;
            ctx.var("mos_swan").set(Val::from(8))?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Pub Owner",
            args![
                "Although you may be an experienced adventurer, you look so young.",
                "I recommend you these drinks. Which one will you take?"
            ],
        )?;
        ctx.next()?;
        l_j = (Val::from(runtime::select_values(
            ctx,
            &[Val::from("Milk - 1000z:Sticky_Herb juice - 1000z:They are all expensive!")],
        )?)
        .try_sub(Val::from(1))?);
        if l_j.clone() == 2 {
            ctx.lines_as("Pub Owner", args!["Hahaha, too much for you eh cheapskate."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        if ctx.var("Zeny").get()?.number()? > 999 {
            ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(1000))?))?;
            ctx.var("mos_swan").set(Val::from(9))?;
            if l_j.clone() == 0 {
                ctx.call(Function::GetItem, vec![Val::from(519), Val::from(1)])?;
            } else {
                ctx.call(Function::GetItem, vec![Val::from(531), Val::from(1)])?;
            }
            ctx.lines_as(
                "Pub Owner",
                args!["Here you are. This is what you ordered.", "How do you like Moscovia?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "Yes, it's beautiful and gorgeous.",
                    "Sasha, most of all, can you answer one question?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Pub Owner", args!["About what?"])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Has Mikhail come here?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Pub Owner",
                args!["Yes he has.", "He looked pale and needed a high-strength adhesive."],
            )?;
            ctx.next()?;
            ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["So what happened?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Pub Owner",
                args![
                    "I don't know much about adhesives.",
                    "But, I know a person who may know about them so I introduced Mikhail to him."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Who's this person?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Pub Owner",
                args![
                    "It's Victor over there.",
                    "He's very careful about details so we call him ^3131FF'Hedgehog Victor'^000000.",
                    "No one knows adhesives as well as he does."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Sasha, thanks a lot!"],
            )?;
            ctx.next()?;
            ctx.lines_as("Pub Owner", args!["You're welcome."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Pub Owner",
            args![
                "You'll need a lot of money while you're traveling.",
                "If you don't have enough money, I'll offer you a glass of water."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("mos_swan").get()? == 8 {
        ctx.lines_as(
            "Pub Owner",
            args![
                "While I work in this pub, I can hear stories from all around the world.",
                "They are all heroes of their life."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Pub Owner",
            args!["Each story interests me so a day passes quick.", "That's why I love my job."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Pub Owner",
            args![
                "Well, do you have some stories for me? or you've got something to ask me.",
                "I recommend you those drinks."
            ],
        )?;
        ctx.next()?;
        l_s = Val::from(runtime::select_values(
            ctx,
            &[Val::from("Milk - 1000z:Apple juice - 1000z:They are all expensive!")],
        )?);
        if l_s.clone() == 3 {
            ctx.lines_as("Pub Owner", args!["Hahaha, Here is the sightseeing place."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        if ctx.var("Zeny").get()?.number()? > 999 {
            ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(1000))?))?;
            ctx.var("mos_swan").set(Val::from(9))?;
            if l_s.clone() == 1 {
                ctx.call(Function::GetItem, vec![Val::from(519), Val::from(1)])?;
            } else {
                ctx.call(Function::GetItem, vec![Val::from(531), Val::from(1)])?;
            }
            ctx.lines_as("Pub Owner", args!["Here you are. This is what you ordered.", "Enjoy yourself."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Pub Owner",
            args![
                "You'll need a lot of money while you're traveling.",
                "If you don't have enough money, I'll offer you a glass of water."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("mos_swan").get()? == 9 {
        ctx.lines_as("Pub Owner", args!["Have you spoken to Victor?"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Pub Owner",
        args![
            "Welcome to our pub.",
            "I'm Alexandre of ^3131FF'Stream Pub'^000000. But, everyone calls me 'Sasha'.",
            "It's a fine day today. I feel like going out."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn pub_owner_mos(ctx: &Ctx) -> Script {
    pub_owner_mos_body(ctx, Vec::new()).map(|_| ())
}

fn victor_mos_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    if (ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 2000 {
        ctx.lines(args![
            "- Please stop here!! -",
            "- You're carrying too many items -",
            "- Please try again -",
            "- after using the kafra service -"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("mos_swan").get()? == 8 {
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Excuse me.", "Could you spare a few minutes for me?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Victor",
            args![
                "Who are you?",
                "What gives you the right to say such a thing?",
                "I'm busy appreciating wine in this glass."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("mos_swan").get()? == 9 {
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Excuse me.", "Could you spare a few minutes for me?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Victor",
                args![
                    "Who are you?",
                    "What gives you the right to say such a thing?",
                    "I'm busy appreciating wine in this glass."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Victor",
                args![
                    "Pardon? You're drinking too.",
                    "Ok, it's lonely to drink alone. I but I've gotten used to it."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Victor", args!["Well, tell me what you have on your mind. I'm listening."])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Have you seen a little boy who was looking for paste?"],
            )?;
            ctx.next()?;
            ctx.lines_as("Victor", args![".................."])?;
            ctx.next()?;
            ctx.lines_as(
                "Victor",
                args![
                    "You're talking about Mikhail?",
                    "He's such a sentimental boy. I was troubled by him"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Victor",
                args!["By the way, how did you know that he came to see me? Did he say anything to you?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Mikhail hasn't come back yet since he left to get the paste. He broke his mother's Matrushka.."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Victor",
                args!["............What?", "Hmm.. Oh dear.....", "I doubt he went there."],
            )?;
            ctx.next()?;
            ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["There?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Victor",
                args!["Yes. Baba Yaga 'The Terrible' and other scary animals live there."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Victor",
                args![
                    "The little boy came here crying a few hours ago.",
                    "He's got something that needs to be stuck together, so he needed a high-strength adhesive."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Victor",
                args!["But I don't carry such a thing. So I told him to bring me some stuff for it."],
            )?;
            ctx.next()?;
            ctx.lines_as("Victor", args!["To get the stuff for a high-strength adhesive, you need to go to an isolated swamp. You can reach it by boat and it takes a long time."])?;
            ctx.next()?;
            ctx.lines_as(
                "Victor",
                args!["But the way to the swamp is very dangerous. There are horrible monsters around it, all hunting with eager eyes."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Victor",
                args![
                    "Especially, the old lady Baba Yaga who attacks anyone she sees. She especially likes youngsters like you. haha!",
                    "heeheehee!!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Victor", args!["...You turned pale! hahaha!!"])?;
            ctx.next()?;
            ctx.lines_as(
                "Victor",
                args![
                    "But...",
                    "I'm exaggerating a little, it's true that there are scary things.",
                    "But, I don't believe that a little and timid boy like Mikhail would go there. It's impossible!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Victor",
                args![
                    "I said to him, 'you can never, ever go there' and told him to go back home.",
                    "He asked for the impossible so I calmed him down with some bread."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Victor",
                args!["He was already white with fear that he broke his mother's matrushka. Do you think that he would go there?"],
            )?;
            ctx.next()?;
            ctx.lines_as("Victor", args!["He might've just hidden himself somewhere, haha!"])?;
            ctx.var("mos_swan").set(Val::from(10))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(18062), Val::from(18063)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if (ctx.var("mos_swan").get()? == 10 || ctx.var("mos_swan").get()? == 11) {
            ctx.lines_as(
                "Victor",
                args![
                    "There's a large island near Moscovia. It takes so much time to reach there by a boat. So nobody wants to go there. "
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Victor",
                args!["If you go deep into its forest, you can find a swamp where sticky weeds are growing."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Victor",
                args!["The way to go there is full of danger. Do think that a timid boy like Mikhail can go there?"],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("mos_swan").get()? == 12 {
            ctx.lines_as(
                "Victor",
                args!["You came back. Hmm, are you about to make me responsible that he's gone?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["No, I found him. You don't have to worry."],
            )?;
            ctx.next()?;
            ctx.lines_as("Victor", args!["So, what do you want with me?"])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["I promised him I'd make the high-strength adhesive. What do I have to do for that?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Victor",
                args![
                    "Why are you so eager to make it? How important is the stuff?",
                    "Well, I don't care..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Victor",
                args!["There's a large island near Moscovia. It takes so much time to reach there by a boat. So nobody wants to go there."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Victor",
                args!["If you go deep into its forest, you can find a swamp where sticky weeds are growing."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Victor",
                args!["You should bring ^3131FF'10 sticky herbs'^000000 and ^3131FF 1 medicine bowl^000000 to me."],
            )?;
            ctx.var("mos_swan").set(Val::from(13))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(18065), Val::from(18066)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if (ctx.var("mos_swan").get()?.number()? > 12 && ctx.var("mos_swan").get()?.number()? < 23) {
            ctx.lines_as(
                "Victor",
                args![
                    "If you want to make adhesives",
                    "get 10 sticky herbs and",
                    "1 medicine bowl for me.",
                    "Those sticky herbs are growing in the swamp. It may be dangerous but that shouldn't be a problem for you?"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("mos_swan").get()? == 23 {
            ctx.lines_as(
                "Victor",
                args![
                    "What do you want with me this time??",
                    "Oh, you said you wanted an adhesive...",
                    "You're bothering me so much!"
                ],
            )?;
            ctx.next()?;
            if (ctx.call(Function::CountItem, vec![Val::from(7763)])?.number()? > 9
                && ctx.call(Function::CountItem, vec![Val::from(7134)])?.is_true())
            {
                ctx.lines_as("Victor", args!["Give me the stuff! I'll make it quickly. You're annoying me!"])?;
                ctx.next()?;
                l_i = Val::from(0);
                'l1: loop {
                    if !(l_i.clone().number()? < 3) {
                        break 'l1;
                    }
                    'b1: {
                        ctx.mes("- He's making adhesive with a crunching sound -")?;
                    }
                    l_i = (l_i.clone() + Val::from(1));
                }
                ctx.next()?;
                l_i = Val::from(0);
                'l2: loop {
                    if !(l_i.clone().number()? < 4) {
                        break 'l2;
                    }
                    'b2: {
                        ctx.mes("- And he may be rubbing something -")?;
                    }
                    l_i = (l_i.clone() + Val::from(1));
                }
                ctx.next()?;
                ctx.lines_as(
                    "Victor",
                    args!["Ok, done!", "I don't want you to bother me any longer!", "Please leave!"],
                )?;
                ctx.call(Function::DelItem, vec![Val::from(7763), Val::from(10)])?;
                ctx.call(Function::DelItem, vec![Val::from(7134), Val::from(1)])?;
                ctx.var("mos_swan").set(Val::from(24))?;
                ctx.call(Function::GetItem, vec![Val::from(7764), Val::from(1)])?;
                ctx.call(Function::ChangeQuest, vec![Val::from(18067), Val::from(18068)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Victor",
                args![
                    "He's annoying me so much. Give me the materials!",
                    "Oh dear? What are those things? I said 10 sticky herbs and 1 medicine bowl!!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Victor",
                args!["Even though I'm always in the pub, I have my own business... !"],
            )?;
            ctx.next()?;
            ctx.lines_as("Victor", args![".............!!!!"])?;
            ctx.next()?;
            ctx.lines_as(
                "Victor",
                args!["..................", "What are you staring at? Bring them to me right now!"],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    ctx.lines_as("Victor", args!["What did you say? I just want to drink peacefully!"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn victor_mos(ctx: &Ctx) -> Script {
    victor_mos_body(ctx, Vec::new()).map(|_| ())
}

fn swan_inn_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn swan_inn(ctx: &Ctx) -> Script {
    swan_inn_body(ctx, Vec::new()).map(|_| ())
}

fn swan_inn_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("mos_swan").get()? != 4 {
        return Err(Stop::End);
    }
    ctx.lines(args!["..................", ".................."])?;
    ctx.next()?;
    ctx.lines(args!["..................", "..................", ".................."])?;
    ctx.next()?;
    ctx.lines(args![
        "..................",
        "..................",
        "..................",
        ".................."
    ])?;
    ctx.next()?;
    ctx.lines_as(
        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
        args!["Oh, I got a good night's sleep. How long did I sleep?"],
    )?;
    ctx.next()?;
    ctx.lines_as(
        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
        args![
            "Although I was forced to stay here, it's true that it's got great facilities.",
            "I feel refreshed."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
        args!["Ok, I guess I will go to the landlord to get some info."],
    )?;
    ctx.var("mos_swan").set(Val::from(5))?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn swan_inn_ontouch(ctx: &Ctx) -> Script {
    swan_inn_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn bubbling_swamp_mos1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 2000 {
        ctx.mes("- Your bag is very heavy today -")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("mos_swan").get()? == 10 {
        ctx.mes("- You feel sticky just looking at this swamp. -")?;
        ctx.next()?;
        ctx.mes("- Gas bubbles are rising. The atmosphere here is pretty scary -")?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["I don't see any traces of Mikhail. I think I should go back and check his house one more time."],
        )?;
        ctx.var("mos_swan").set(Val::from(11))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(18063), Val::from(18064)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("mos_swan").get()? == 11 && ctx.var("mos_swan").get()? == 12) {
        ctx.mes("- You can see a small muddy swamp -")?;
        ctx.next()?;
        ctx.mes("- Gas bubbles are rising. The atmosphere here is pretty scary -")?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("mos_swan").get()?.number()? > 12 && ctx.var("mos_swan").get()?.number()? < 23) {
        ctx.mes("- You feel sticky just looking at the swamp. -")?;
        ctx.next()?;
        ctx.mes("- Gas bubbles are rising. The atmosphere here is pretty scary -")?;
        ctx.next()?;
        ctx.mes("- You stretch out to find sticky herbs. -")?;
        ctx.next()?;
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?.number()? < 6 {
            ctx.mes("- You have pricked your finger on a Sticky Herb. -")?;
            ctx.call(Function::GetItem, vec![Val::from(7763), Val::from(1)])?;
            ctx.var("mos_swan").set((ctx.var("mos_swan").get()? + Val::from(1)))?;
            if ctx.var("mos_swan").get()? == 23 {
                ctx.call(Function::ChangeQuest, vec![Val::from(18066), Val::from(18067)])?;
            }
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.mes("- You have pricked your finger on a Green Herb. -")?;
        ctx.call(Function::GetItem, vec![Val::from(511), Val::from(1)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.mes("- You can see a small swamp which seems very muddy. -")?;
    ctx.next()?;
    ctx.mes("- Gas bubbles are rising. The atmosphere here is pretty scary -")?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn bubbling_swamp_mos1(ctx: &Ctx) -> Script {
    bubbling_swamp_mos1_body(ctx, Vec::new()).map(|_| ())
}

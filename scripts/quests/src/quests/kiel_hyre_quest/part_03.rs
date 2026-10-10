use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn sword_hilt_kh_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_khswords = Val::from(0);
    if (ctx.var("khcottagepoem1").get()?.number()? < 3
        || (ctx.var("khcottagepoem2").get()?.number()? < 2 && ctx.var("kielhyrequest").get()?.number()? < 30))
    {
        ctx.lines(args![
            "^3355FFThere are four elaborately",
            "designed swords positioned",
            "next to four creepy looking",
            "snake sculptures.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("khcottagepoem1").get()?.number()? <= 4
        || (ctx.var("khcottagepoem2").get()?.number()? <= 2 && ctx.var("kielhyrequest").get()?.number()? < 30))
    {
        ctx.lines(args![
            "^3355FFThere are four elaborately",
            "designed swords positioned",
            "next to four creepy looking",
            "snake sculptures. Wait!",
            "Perhaps they're related to",
            "that poem you read earlier..."
        ])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("No way!:Of course!")])? {
            1 => {
                ctx.lines(args![
                    "^3355FFImpossible...",
                    "It must be some",
                    "kind of coincidence...^000000"
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines(args![
                    "^3355FFOf course! And look!",
                    "There's a hole on top",
                    "of the head of each snake",
                    "sculpture. These holes seem",
                    "big enough to insert each of",
                    "the ornamental swords nearby...^000000"
                ])?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Ignore:Insert Ornamental Swords")])? {
                    1 => {
                        ctx.lines(args![
                            "^3355FFImposible...^000000",
                            "^3355FFIt must be some^000000",
                            "^3355FFkind of coincidence...^000000"
                        ])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        if ctx.var("khcottagepoem1").get()? != 4 {
                            ctx.lines(args![
                                "^3355FFCan't... Pull out...",
                                "Sword! It must be",
                                "locked into place",
                                "somehow, or sealed",
                                "by some strange force!^000000"
                            ])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            ctx.lines(args![
                                "^3355FFYou should probably",
                                "try to insert each sword",
                                "into the correct snake.",
                                "First, please select the",
                                "snake in which you will",
                                "insert the first sword.^000000"
                            ])?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("First Snake:Second Snake:Third Snake:Fourth Snake")])? {
                                2 => {
                                    l_khswords = (l_khswords.clone() + Val::from(1));
                                }
                                _ => {}
                            }
                            ctx.next()?;
                            ctx.lines(args![
                                "^3355FFNow, please choose",
                                "the snake in which you",
                                "will insert the second sword.^000000"
                            ])?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("First Snake:Second Snake:Third Snake:Fourth Snake")])? {
                                4 => {
                                    l_khswords = (l_khswords.clone() + Val::from(1));
                                }
                                _ => {}
                            }
                            ctx.next()?;
                            ctx.lines(args![
                                "^3355FFNext, please select",
                                "the snake in which you",
                                "will insert the third sword.^000000"
                            ])?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("First Snake:Second Snake:Third Snake:Fourth Snake")])? {
                                1 => {
                                    l_khswords = (l_khswords.clone() + Val::from(1));
                                }
                                _ => {}
                            }
                            ctx.next()?;
                            ctx.lines(args![
                                "^3355FFLastly, please select",
                                "the snake in which you",
                                "will insert the fourth sword.^000000"
                            ])?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("First Snake:Second Snake:Third Snake:Fourth Snake")])? {
                                3 => {
                                    l_khswords = (l_khswords.clone() + Val::from(1));
                                }
                                _ => {}
                            }
                            if l_khswords.clone() == 4 {
                                ctx.lines(args![
                                    "^3355FFYour ears are suddenly",
                                    "filled with a low buzzing",
                                    "noise, and your mind",
                                    "and body feel as if they",
                                    "are being swept away...^000000"
                                ])?;
                                ctx.var("khcottagepoem1").set(Val::from(5))?;
                                ctx.var("khcottagepoem2").set(Val::from(3))?;
                                ctx.close_window()?;
                                ctx.call(Function::Warp, vec![Val::from("kh_vila"), Val::from(178), Val::from(72)])?;
                                return Err(Stop::End);
                            } else {
                                ctx.lines(args![
                                    "^3355FFNothing happened...",
                                    "You probably didn't",
                                    "insert the swords into",
                                    "the correct snakes. For now,",
                                    "you should return the swords,",
                                    "and then try this again later.^000000"
                                ])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    } else if ((ctx.var("khcottagepoem1").get()? == 5 && ctx.var("khcottagepoem2").get()? == 3)
        || ctx.var("kielhyrequest").get()?.number()? >= 30)
    {
        ctx.lines(args![
            "^3355FFHere is a hidden",
            "path that leads to",
            "the secret laboratory.^000000"
        ])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Enter:Cancel")])? {
            1 => {
                ctx.call(Function::Warp, vec![Val::from("kh_vila"), Val::from(178), Val::from(72)])?;
                return Err(Stop::End);
            }
            2 => {
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    Ok(Val::from(0))
}

pub fn sword_hilt_kh(ctx: &Ctx) -> Script {
    sword_hilt_kh_body(ctx, Vec::new()).map(|_| ())
}

fn test_tube_khp2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args![
        "^3355FFThis test tube contains",
        "a young man dressed in",
        "a Kiel Hyre Academy",
        "uniform. Somehow, you",
        "get the feeling that you've",
        "seen him somewhere before.^000000"
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn test_tube_khp2(ctx: &Ctx) -> Script {
    test_tube_khp2_body(ctx, Vec::new()).map(|_| ())
}

fn test_tube_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_khinput_s = Val::from("");
    if ctx.var("kielhyrequest").get()?.number()? > 28 {
        ctx.lines(args![
            "^3355FFA strange looking,",
            "wizened old man is",
            "held within this test tube.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("kielhyrequest").get()? == 28 {
        ctx.lines(args![
            "^3355FFA strange looking,",
            "wizened old man is",
            "held within this test tube.",
            "There is a small red button",
            "right underneath the test tube.^000000"
        ])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Press Button:Investigate Further")])? {
            1 => {
                ctx.lines_as(
                    "???",
                    args![
                        "H-hello...? C-can you",
                        "hear me? I don't recognize",
                        "you... But... Maybe I forgot?",
                        "Wait, wait. If you're my friend,",
                        "then you know what to call me,",
                        "right? Do you know what I am?"
                    ],
                )?;
                ctx.next()?;
                let (input, status) = runtime::input_text(ctx, None, None)?;
                l_khinput_s = input;
                if l_khinput_s.clone() == "little lost devil" {
                    ctx.lines_as("???", args!["Heh... Heh heh...", "He knows... Hey, you", "have to remember these", "numbers, okay? D-don't", "forget, they'll be important...", "^FF00004^000000, ^FF00007^000000, ^FF00007^000000, ^FF00002^000000, ^FF00009^000000, ^FF00006^000000, ^FF00001^000000. That's all..."])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FF4772961^000000",
                        "^3355FFWhat could^000000",
                        "^3355FFthose numbers^000000",
                        "^3355FFpossibly mean?^000000"
                    ])?;
                    ctx.var("khcottagepoem1").set(Val::from(0))?;
                    ctx.var("khcottagepoem2").set(Val::from(0))?;
                    ctx.var("kielhyrequest").set(Val::from(30))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "???",
                        args!["No... No...", "That's not right.", "I don't think we", "were friends. No..."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            2 => {
                ctx.lines(args!["^3355FFLet's investigate", "this area a little", "more first.^000000"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if ctx.var("kielhyrequest").get()?.number()? >= 30 {
        if (ctx.call(Function::CountItem, vec![Val::from(7491)])?.number()? > 0
            && ctx.call(Function::CountItem, vec![Val::from(7492)])?.number()? > 0)
        {
            ctx.lines(args![
                "^3355FFWhat could the",
                "number 4772961",
                "mean? For now, you've",
                "found everything that",
                "you need from this place,",
                "so you should return to Elly.^000000"
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines(args![
                "^3355FFWhat could the",
                "number 4772961",
                "mean? For now, you'd",
                "better search this cottage",
                "for any clues you can find...^000000"
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    return Err(Stop::End);
}

pub fn test_tube(ctx: &Ctx) -> Script {
    test_tube_body(ctx, Vec::new()).map(|_| ())
}

fn heavy_door_kh1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_khinput_s = Val::from("");
    if ctx.var("kielhyrequest").get()?.number()? < 38 {
        ctx.lines(args!["^3355FFThere is a large, heavy", "door infront of you.^000000"])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Open Door:Cancel")])? {
            1 => {
                let (input, status) = runtime::input_text(ctx, None, None)?;
                l_khinput_s = input;
                ctx.lines(args![
                    "^3355FFYou try to push the",
                    "door open with all",
                    "your might, but fail",
                    "to make it budge.^000000"
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines(args![
                    "^3355FFyou decided to leave",
                    "this door alone until",
                    "you can figure out",
                    "how to open it^000000"
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if (ctx.var("kielhyrequest").get()?.number()? >= 38 && ctx.var("kielhyrequest").get()?.number()? < 46) {
        ctx.lines(args!["^3355FFThere is a large, heavy", "door in front of you.^000000"])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Open Door:Cancel")])? {
            1 => {
                let (input, status) = runtime::input_text(ctx, None, None)?;
                l_khinput_s = input;
                if l_khinput_s.clone() == "Blue Keycard" {
                    ctx.lines(args!["^3355FFYou've successfully", "opened the door."])?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("kh_school"), Val::from(119), Val::from(144)])?;
                    return Err(Stop::End);
                } else {
                    ctx.lines(args![
                        "^3355FFYou try to push the",
                        "door open with all",
                        "your might, but fail",
                        "to make it budge.^000000"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            2 => {
                ctx.lines(args![
                    "^3355FFYou decided to leave",
                    "this door alone until",
                    "you can figure out",
                    "how you can open it.^000000"
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else {
        ctx.lines(args![
            "^3355FFThis is an incredibly",
            "heavy door that is tightly",
            "closed. You won't be able to",
            "open it through brute force.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn heavy_door_kh1(ctx: &Ctx) -> Script {
    heavy_door_kh1_body(ctx, Vec::new()).map(|_| ())
}

fn heavy_door_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_khinput_s = Val::from("");
    if (ctx.var("kielhyrequest").get()?.number()? >= 38 && ctx.var("kielhyrequest").get()?.number()? < 46) {
        ctx.lines(args!["^3355FFThere is a large, heavy", "door infront of you.^000000"])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Open Door:Cancel")])? {
            1 => {
                let (input, status) = runtime::input_text(ctx, None, None)?;
                l_khinput_s = input;
                ctx.lines(args![
                    "^3355FFYou try to push the",
                    "door open with all",
                    "your might, but fail",
                    "to make it budge.^000000"
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines(args![
                    "^3355FFyou decided to leave",
                    "this door alone until",
                    "you can figure out",
                    "how to open it^000000"
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else {
        ctx.lines(args!["^3355FFThere is a large, heavy", "door infront of you^000000"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn heavy_door(ctx: &Ctx) -> Script {
    heavy_door_body(ctx, Vec::new()).map(|_| ())
}

fn beautiful_lady_kh1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::Cutin, vec![Val::from("kh_ellisia"), Val::from(2)])?;
    if ctx.var("kielhyrequest").get()?.number()? < 40 {
        ctx.lines_as(
            "Allysia",
            args![
                "Hm? I don't think",
                "I know you. Kiel Hyre",
                "has only authorized Elly,",
                "and a trusted friend that",
                "may be helping her, to",
                "be admitted to this area."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Allysia",
            args![
                "I cannot see ^FF0000Elly^000000.",
                "Has she been attacked as well?",
                "Are you ^FF0000Elly's friend^000000?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Allysia",
            args![
                "I don't see Elly anywhere",
                "around here. I can only let",
                "you enter if you can identify",
                "yourself as Elly's friend.",
                "Is there anything that you",
                "can present to me as proof?"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Yellow Keycard:Blue Keycard:Golden Key:Carved Button:...?")])? {
            3 => {
                ctx.lines_as(
                    "Allysia",
                    args![
                        "Ah, isn't this the",
                        "key that Kiel Hyre gave",
                        "to Elly? Yes, I'm convinced",
                        "that you've been helping her.",
                        "I've been waiting for you, so",
                        "let me guide you to my room."
                    ],
                )?;
                ctx.close_window()?;
                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                ctx.call(Function::Warp, vec![Val::from("kh_school"), Val::from(120), Val::from(180)])?;
                return Err(Stop::End);
            }
            4 => {
                ctx.lines_as(
                    "Allysia",
                    args![
                        "Th-that's Kiehl's",
                        "seal! Did he send you",
                        "here to get me?! I'm",
                        "not taking any chances!"
                    ],
                )?;
                ctx.close_window()?;
                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("kh_school"),
                        Val::from(117),
                        Val::from(144),
                        Val::from("Bomb"),
                        Val::from(1745),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("kh_school"),
                        Val::from(117),
                        Val::from(144),
                        Val::from("Bomb"),
                        Val::from(1745),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("kh_school"),
                        Val::from(117),
                        Val::from(144),
                        Val::from("Bomb"),
                        Val::from(1745),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("kh_school"),
                        Val::from(117),
                        Val::from(144),
                        Val::from("Bomb"),
                        Val::from(1745),
                        Val::from(1),
                    ],
                )?;
                return Err(Stop::End);
            }
            5 => {
                ctx.lines_as("Allysia", args!["......", "........", "..........."])?;
                ctx.close_window()?;
                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                return Err(Stop::End);
            }
            _ => {}
        }
        ctx.lines_as("Allysia", args!["Hm? This doesn't prove", "that Elly really trusts you..."])?;
        ctx.close_window()?;
        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
        return Err(Stop::End);
    } else if ctx.var("kielhyrequest").get()?.number()? >= 40 {
        ctx.lines_as("Allysia", args!["Please follow me."])?;
        ctx.close_window()?;
        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
        ctx.call(Function::Warp, vec![Val::from("kh_school"), Val::from(120), Val::from(180)])?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn beautiful_lady_kh1(ctx: &Ctx) -> Script {
    beautiful_lady_kh1_body(ctx, Vec::new()).map(|_| ())
}

fn beautiful_lady_kh2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn beautiful_lady_kh2(ctx: &Ctx) -> Script {
    beautiful_lady_kh2_body(ctx, Vec::new()).map(|_| ())
}

fn beautiful_lady_kh6_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(7496), Val::from(1)])? == 0 {
        ctx.lines(args![
            "^3355FFJust a second...",
            "You're carrying too",
            "many items with you",
            "right now, so you'll",
            "need to free up more",
            "Inventory space first...^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.call(Function::Cutin, vec![Val::from("kh_ellisia"), Val::from(2)])?;
    if ctx.var("kielhyrequest").get()?.number()? < 38 {
        ctx.lines_as("??????", args!["This is private property.", "Please leave immediately!"])?;
        ctx.close_window()?;
        ctx.call(Function::Cutin, vec![Val::from("kh_ellisia"), Val::from(255)])?;
        ctx.call(Function::Warp, vec![Val::from("yuno_fild08"), Val::from(73), Val::from(172)])?;
        return Err(Stop::End);
    } else {
        if ctx.var("kielhyrequest").get()?.number()? <= 38 {
            ctx.lines_as(
                "Allysia",
                args![
                    "Friend of prototype Elly,",
                    "I welcome you. As you may",
                    "have figured out, I need your",
                    "help. Kiel Hyre is being held",
                    "somewhere inside this factory."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Allysia",
                args![
                    "It is imperative that",
                    "you search for Kiel Hyre",
                    "and rescue him as soon",
                    "as you possibly can!"
                ],
            )?;
            ctx.next()?;
            match runtime::select_values(
                ctx,
                &[Val::from("What is this factory?:What happened to ^FF0000Kiel Hyre^000000?")],
            )? {
                1 => {
                    ctx.lines_as(
                        "Allysia",
                        args![
                            "This factory is part of",
                            "the Kiel Hyre Foundation's",
                            "secret business where humanoid",
                            "robots are manufactured. Myself,",
                            "and all of the academy's students",
                            "are actually robots, not humans."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Allysia",
                        args![
                            "The Kiel Hyre Foundation's",
                            "ultimate goal is to develop",
                            "superior robots that will help",
                            "human society. As robots, we",
                            "can handle tasks that are too",
                            "dangerous or difficult for humans."
                        ],
                    )?;
                }
                2 => {
                    ctx.lines_as(
                        "Allysia",
                        args![
                            "^3355FFKiehl^000000 has proceeded to",
                            "perform abnormal modifications",
                            "to the humanoid robots. This",
                            "has been reported to Kiel Hyre,",
                            "who has left to stop Kiehl.",
                            "However, Hyre hasn't returned."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Allysia",
                        args![
                            "Fearing that Kiehl would",
                            "break me, Kiel Hyre brought",
                            "a copy of me instead. I can still",
                            "detect Kiel Hyre's heartbeat,",
                            "so he should be alright, but his",
                            "own son might harm him soon."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Allysia",
                        args![
                            "Take this Keycard",
                            "which will enable you",
                            "to enter the secret areas",
                            "inside this factory. If you",
                            "locate Kiel Hyre, please",
                            "let me know right away."
                        ],
                    )?;
                    ctx.call(Function::GetItem, vec![Val::from(7496), Val::from(1)])?;
                    ctx.var("kielhyrequest").set(Val::from(40))?;
                }
                _ => {}
            }
            ctx.close_window()?;
            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
            return Err(Stop::End);
        } else if ctx.var("kielhyrequest").get()? == 40 {
            ctx.lines_as(
                "Allysia",
                args![
                    "I can still detect",
                    "Kiel Hyre's heartbeat,",
                    "but his sone Kiehl might",
                    "do something desperate to",
                    "him soon. Please locate Kiel",
                    "Hyre before that can happen!"
                ],
            )?;
            ctx.close_window()?;
            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
            ctx.call(Function::Warp, vec![Val::from("kh_dun01"), Val::from(22), Val::from(216)])?;
            return Err(Stop::End);
        } else if ctx.var("kielhyrequest").get()? == 42 {
            ctx.lines_as("Allysia", args!["Have you already", "located Kiel Hyre?"])?;
            ctx.next()?;
            ctx.lines(args![
                "^3355FFYou tell Allysia where^000000",
                "^3355FFKiel Hyre has been locked^000000",
                "^3355FFup, and give her the metal^000000",
                "^3355FFfragment that Kiel Hyre handed^000000",
                "^3355FFto you. She took fragment^000000",
                "^3355FFand wore it around her wrist.^000000"
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Allysia",
                args![
                    "...Kiel Hyre's secret code",
                    "confirmed. B_2_3 area.",
                    "Vital signs are normal.",
                    "Envelope received. Sending",
                    "modified Puppet Designs..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Allysia",
                args!["Alright, I've received", "Kiel Hyre's orders, and", "must carry them out..."],
            )?;
            ctx.call(Function::DelItem, vec![Val::from(7497), Val::from(1)])?;
            ctx.var("kielhyrequest").set(Val::from(44))?;
            ctx.close_window()?;
            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
            return Err(Stop::End);
        } else if ctx.var("kielhyrequest").get()? == 44 {
            ctx.lines_as(
                "Allysia",
                args![
                    "Alright, I have a mission",
                    "to carry out for Kiel Hyre,",
                    "and I don't have much time.",
                    "Let me give you some specific",
                    "instructions very quickly."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Allysia",
                args![
                    "Firstly, I need to take your",
                    "keycards for security reasons.",
                    "Secondly, meet me at the Kiel",
                    "Hyre Mansion in Lighthalzen.",
                    "Present the Golden Key to",
                    "be admitted to the premises."
                ],
            )?;
            ctx.call(Function::DelItem, vec![Val::from(7492), Val::from(1)])?;
            ctx.call(Function::DelItem, vec![Val::from(7495), Val::from(1)])?;
            ctx.call(Function::DelItem, vec![Val::from(7496), Val::from(1)])?;
            ctx.var("kielhyrequest").set(Val::from(46))?;
            ctx.next()?;
            ctx.lines_as(
                "Allysia",
                args![
                    "I understand that this",
                    "is sudden, and I'm not",
                    "giving you a thorough",
                    "explanation, but something",
                    "horrible will happen if I don't",
                    "hurry as quickly as possible."
                ],
            )?;
            ctx.close_window()?;
            ctx.call(Function::Warp, vec![Val::from("yuno_fild08"), Val::from(73), Val::from(172)])?;
            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
            return Err(Stop::End);
        } else if ctx.var("kielhyrequest").get()?.number()? >= 44 {
            ctx.lines_as(
                "Allysia",
                args![
                    "I understand that this",
                    "is sudden, and I'm not",
                    "giving you a thorough",
                    "explanation, but something",
                    "horrible will happen if i don't",
                    "hurry as quickly as possible."
                ],
            )?;
            ctx.close_window()?;
            ctx.call(Function::Warp, vec![Val::from("yuno_fild08"), Val::from(73), Val::from(172)])?;
            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
            return Err(Stop::End);
        }
    }
    Ok(Val::from(0))
}

pub fn beautiful_lady_kh6(ctx: &Ctx) -> Script {
    beautiful_lady_kh6_body(ctx, Vec::new()).map(|_| ())
}

fn signboard_kh_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Notice",
        args![
            "Cute Pets are prohibited",
            "in this area. (Cute Pets that",
            "provide special assistance",
            "to the visually impaired are",
            "exempt from this rule.)"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn signboard_kh(ctx: &Ctx) -> Script {
    signboard_kh_body(ctx, Vec::new()).map(|_| ())
}

fn mechanical_device_kh_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_khinput_s = Val::from("");
    if (ctx.var("kielhyrequest").get()?.number()? <= 38 || ctx.var("kielhyrequest").get()?.number()? >= 44) {
        ctx.lines(args![
            "^3355FFYou encounter",
            "a mechanical device.",
            "It looks like it can be",
            "operated by inserting the",
            "correct keycard into the slot.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("kielhyrequest").get()? == 40 && ctx.call(Function::CountItem, vec![Val::from(7496)])?.number()? >= 1) {
        ctx.lines(args![
            "^3355FFYou encounter",
            "a mechanical device.",
            "It looks like it can be",
            "operated by inserting the",
            "correct keycard into the slot.^000000"
        ])?;
        ctx.next()?;
        let (input, status) = runtime::input_text(ctx, None, None)?;
        l_khinput_s = input;
        if l_khinput_s.clone() == "Red Keycard" {
            ctx.lines(args!["^3355FFThe door opens once", "you insert the Red Keycard.^000000"])?;
            ctx.close_window()?;
            ctx.call(Function::Warp, vec![Val::from("kh_dun01"), Val::from(170), Val::from(227)])?;
            return Err(Stop::End);
        } else {
            ctx.mes("^3355FFNothing happened.^000000")?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    Ok(Val::from(0))
}

pub fn mechanical_device_kh(ctx: &Ctx) -> Script {
    mechanical_device_kh_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum FactoryBAreaDoorStep {
    Start,
    OnTouch,
}

fn factory_b_area_door_run(ctx: &Ctx, mut step: FactoryBAreaDoorStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            FactoryBAreaDoorStep::Start => {
                step = FactoryBAreaDoorStep::OnTouch;
                continue 'machine;
            }
            FactoryBAreaDoorStep::OnTouch => {
                if ctx.var("kielhyrequest").get()? == 40 {
                    if ctx.call(Function::CheckWeight, vec![Val::from(7497), Val::from(1)])? == 0 {
                        ctx.lines(args![
                            "^3355FFJust a second...",
                            "You're carrying too",
                            "many items with you",
                            "right now, so you'll",
                            "need to free up more",
                            "Inventory space first...^000000"
                        ])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines(args![
                        "^3355FFYou can faintly hear",
                        "a voice from the other",
                        "side of this door.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as("????", args!["^333333...Kiehl...", "...How dare you...!^000000"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Mister Kiel Hyre?", "Is that you in there?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Kiel Hyre", args!["Wh-who's there?", "Identify yourself!"])?;
                    ctx.next()?;
                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["I'm... I'm..."])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFYou explain your story",
                        "to the voice behind the",
                        "door, and tell him what",
                        "happened to Elly and Allysia.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kiel Hyre",
                        args![
                            "^333333...Yes... It's me.",
                            "I'm Kiel Hyre, trapped",
                            "in here. He went so far",
                            "as to modify Elly, eh?",
                            "...............................^000000"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kiel Hyre",
                        args![
                            "^333333Hurry, take this",
                            "module to Allysia!",
                            "She'll know what to do",
                            "with it. If you're really",
                            "helping us, then she'll have",
                            "some instructions for you too.^000000 "
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFKiel Hyre slid",
                        "a strange metal",
                        "fragment through",
                        "the gap between the",
                        "door and the floor.^000000"
                    ])?;
                    ctx.call(Function::GetItem, vec![Val::from(7497), Val::from(1)])?;
                    ctx.var("kielhyrequest").set(Val::from(42))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("kielhyrequest").get()?.number()? >= 42 {
                    ctx.lines_as("Kiel Hyre", args!["......"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn factory_b_area_door(ctx: &Ctx) -> Script {
    factory_b_area_door_run(ctx, FactoryBAreaDoorStep::Start, Vec::new()).map(|_| ())
}

pub fn factory_b_area_door_ontouch(ctx: &Ctx) -> Script {
    factory_b_area_door_run(ctx, FactoryBAreaDoorStep::OnTouch, Vec::new()).map(|_| ())
}

fn entrance_device_khd2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_khinput_s = Val::from("");
    if ctx.call(Function::CountItem, vec![Val::from(7509)])?.number()? < 1 {
        ctx.lines(args![
            "^3355FFYou encounter",
            "a mechanical device.",
            "It looks like it can be",
            "operated by inserting the",
            "correct keycard into the slot.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines(args![
            "^3355FFYou encounter",
            "a mechanical device.",
            "It looks like it can be",
            "operated by inserting the",
            "correct keycard into the slot.^000000"
        ])?;
        ctx.next()?;
        let (input, status) = runtime::input_text(ctx, None, None)?;
        l_khinput_s = input;
        if l_khinput_s.clone() == "Luxurious Keycard" {
            ctx.lines(args![
                "^3355FFAs you insert the",
                "Luxurious Keycard",
                "into the keycard slot,",
                "the door swings open",
                "to reveal a long flight",
                "of descending stairs.^000000"
            ])?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Descend Stairs:Cancel")])? {
                1 => {
                    ctx.call(Function::Warp, vec![Val::from("kh_dun02"), Val::from(41), Val::from(198)])?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines(args![
                        "^3355FFIt looks like this door",
                        "has automatically closed",
                        "after the preprogrammed",
                        "time limit has elapsed.^000000"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        } else {
            ctx.lines(args![
                "^3355FFYou try to push the",
                "door open with all",
                "your might, but fail",
                "to make it budge.^000000"
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    Ok(Val::from(0))
}

pub fn entrance_device_khd2(ctx: &Ctx) -> Script {
    entrance_device_khd2_body(ctx, Vec::new()).map(|_| ())
}

fn steward_kh_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("kielhyrequest").get()?.number()? < 46 {
        ctx.lines_as("Steward", args!["This is a private residence,", "please leave."])?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("lighthalzen"), Val::from(188), Val::from(201)])?;
        return Err(Stop::End);
    }
    if (ctx.var("kielhyrequest").get()?.number()? >= 46 && ctx.var("kielhyrequest").get()?.number()? < 50) {
        ctx.lines_as(
            "Steward",
            args!["Greetings.", "Have you been invited", "by the master of", "this mansion?"],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Present Golden Key:????")])? {
            1 => {
                ctx.lines_as(
                    "Steward",
                    args![
                        ((Val::from("Ah, Master ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(".")),
                        "I've been expecting your",
                        "arrival. Please, this way."
                    ],
                )?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("kh_mansion"), Val::from(21), Val::from(14)])?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Steward",
                    args![
                        "If you have not been",
                        "invited by the master of",
                        "this mansion, then I'm",
                        "afraid that I must insist on",
                        "your immediate departure!"
                    ],
                )?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("lighthalzen"), Val::from(188), Val::from(201)])?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    if ((ctx.var("kielhyrequest").get()?.number()? >= 50 && ctx.var("kielhyrequest").get()?.number()? < 64)
        || ctx.var("kielhyrequest").get()?.number()? >= 70)
    {
        ctx.lines_as(
            "Steward",
            args![
                ((Val::from("Ah, Master ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(".")),
                "It is a pleasure to",
                "receive your company",
                "once again. Would you",
                "like to see my master?"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Yes:No")])? {
            1 => {
                ctx.lines_as("Steward", args!["Very well.", "Right this way~"])?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("kh_mansion"), Val::from(21), Val::from(14)])?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as("Steward", args!["Very well.", "Please make", "yourself at home."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    if ((ctx.var("kielhyrequest").get()? == 52 || ctx.var("kielhyrequest").get()? == 54)
        || (ctx.var("kielhyrequest").get()?.number()? >= 64 && ctx.var("kielhyrequest").get()?.number()? <= 70))
    {
        ctx.lines_as(
            "Steward",
            args![
                ((Val::from("Ah, Master ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(".")),
                "It is a pleasure to",
                "receive your company",
                "once again. Would you",
                "like to see my master, or...?"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("^FF0000Kiel Hyre^000000:^3355FFMitchell^000000")])? {
            1 => {
                ctx.lines_as("Steward", args!["Very well.", "Right this way~"])?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("kh_mansion"), Val::from(21), Val::from(14)])?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Steward",
                    args!["Oh...? You must.", "be here to deliver", "good news. Excuse me..."],
                )?;
                ctx.next()?;
                ctx.lines(args![
                    "^3355FFThe Steward furtively^000000",
                    "^3355FFlooked around to check^000000",
                    "^3355FFif anyone is watching him.^000000"
                ])?;
                ctx.next()?;
                ctx.lines_as(
                    "Steward",
                    args![
                        "I believe it should",
                        "be safe enough to let",
                        "you go see him now...",
                        "Please, hurry this way."
                    ],
                )?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("kh_mansion"), Val::from(20), Val::from(87)])?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    Ok(Val::from(0))
}

pub fn steward_kh(ctx: &Ctx) -> Script {
    steward_kh_body(ctx, Vec::new()).map(|_| ())
}

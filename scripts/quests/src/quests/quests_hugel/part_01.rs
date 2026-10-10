use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum AllenSchuwellStep {
    Start,
    OnTouch,
}

fn allen_schuwell_run(ctx: &Ctx, mut step: AllenSchuwellStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            AllenSchuwellStep::Start => {
                if ctx.var("BaseLevel").get()?.number()? > 49 {
                    if ctx.var("hg_herb").get()? == 0 {
                        ctx.lines_as(
                            "Allen",
                            args![
                                "*Groooooan~*",
                                "Arrrgh, my back hurts...",
                                "And my stomach... Why do",
                                "I have to suffer like this",
                                "while other people get to",
                                "live happy, luxurious lives?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Allen",
                            args![
                                "My son is the only family",
                                "I've got. He used to be such",
                                "a good obedient kid, but now",
                                "I doubt whether he cares about",
                                "his father! I told that lazy kid to hurry and bring me my meds..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Allen",
                            args![
                                "But does he worry about",
                                "his father's suffering?! No!",
                                "I bet you he's hanging out with",
                                "his no good hoodlum friends!",
                                "Argh! The pain is getting worse! I... I need my medication!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Allen",
                            args![
                                "Oh! I can definitely feel",
                                "it getting worse! Please,",
                                "adventurer, would you help",
                                "a sick man? I need you to",
                                "find my son Postell and to",
                                "tell him to bring my medicine!"
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Sure.:No, I'm sorry.")])? {
                            1 => {
                                ctx.lines_as(
                                    "Allen",
                                    args![
                                        "Th-thank you so much!",
                                        "Please head east, go",
                                        "outside of town, and",
                                        "you should be able to",
                                        "find Postell. Tell him that",
                                        "I need my ''Kolbun A'' now!"
                                    ],
                                )?;
                                ctx.var("hg_herb").set(Val::from(1))?;
                                ctx.call(Function::SetQuest, vec![Val::from(8053)])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Allen",
                                    args![
                                        "And let him know that",
                                        "if he doesn't shape up...",
                                        "Well, I just might have",
                                        "to disown my own son!",
                                        "He's dead to me if he",
                                        "doesn't love his father!"
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    "Allen",
                                    args![
                                        "I... I don't understand...!",
                                        "How can you leave a sick",
                                        "man alone with his suffering?",
                                        "I-I desperately need help!",
                                        "No! Please, d-don't go away!"
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    } else {
                        if ctx.var("hg_herb").get()? == 2 {
                            ctx.lines_as(
                                "Allen",
                                args![
                                    "Postell... My son...",
                                    "Why have you forsaken me?",
                                    "I'm lying here at the brink",
                                    "of death, and the boy won't",
                                    "even come to see his father!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Allen",
                                args![
                                    "But you... I can count on you.",
                                    "Even in Lighthalzen, I'd never",
                                    "find anyone as dependable as",
                                    "you. This time, I've decided to",
                                    "discipline my degenerate son.",
                                    "Will you help me once more?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["Wait, wait...", "Lighthalzen?", "Did you used to", "live there or something?"],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Allen",
                                args![
                                    "Uh? Er, well, that's just",
                                    "an expression I use. You",
                                    "know, like an example.",
                                    "I-I--^333333*Cough!*^000000 Oh, oh no.",
                                    "My ^333333*Hack!*^000000 pancreas! Or liver!",
                                    "I'm having another attack!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["......", ".........", "............"],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Allen",
                                args![
                                    "Oh, everything's getting",
                                    "so dark! P-please, if you",
                                    "have any mercy at all, any",
                                    "shred of human decency,",
                                    "would you please let me",
                                    "have a White P-Potion?"
                                ],
                            )?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("Give White Potion:Ignore")])? {
                                1 => {
                                    if !(ctx.call(Function::CountItem, vec![Val::from(504)])?.is_true()) {
                                        ctx.lines_as(
                                            "Allen",
                                            args![
                                                "Oh... Oh no...! You",
                                                "don't have any White",
                                                "Potions either?! Wh-what",
                                                "what am I gonna--?! Help me,",
                                                "please! Bring me a White Potion",
                                                "as soon as you can! ^333333*Cough!*^000000"
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else {
                                        ctx.call(Function::DelItem, vec![Val::from(504), Val::from(1)])?;
                                        ctx.lines_as(
                                            "Allen",
                                            args![
                                                "Oh, thank you!",
                                                "I-It hurts so much...",
                                                "A White Potion just has",
                                                "to help. I-It has to work!"
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                            args!["......", ".........", "............"],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Allen",
                                            args![
                                                "Ah, that... *Whew*",
                                                "Thank you so much. Now,",
                                                "all I need to do is wait for",
                                                "my son to bring my medicine.",
                                                "Wait. What if he takes too long",
                                                "to get here. No, no, what if..."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Allen",
                                            args![
                                                "Wh-what if I have another",
                                                "ulcer attack before Postell",
                                                "can bring my meds?! I might",
                                                "not survive! A-adventurer, would you please stay by my side until",
                                                "my son arrives? P-please?"
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.var("hg_herb").set(Val::from(3))?;
                                        ctx.call(Function::ChangeQuest, vec![Val::from(8054), Val::from(8055)])?;
                                        ctx.lines_as(
                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                            args!["......", ".........", "............"],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                }
                                2 => {
                                    ctx.lines_as(
                                        "Allen",
                                        args![
                                            "W-wait! A-are you just",
                                            "going to let me d-die?",
                                            "Noo! ^333333*Cough*^000000 Arrgh! A plague!",
                                            "A plague on both your houses!"
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                _ => {}
                            }
                        } else {
                            if ctx.var("hg_herb").get()? == 3 {
                                ctx.lines_as(
                                    "Allen",
                                    args![
                                        "*Gasp* I... I can",
                                        "feel it! I think I'm",
                                        "gonna have another",
                                        "neutron attack! Argh, it's",
                                        "already effecting all of the",
                                        "neutrons in my nervous system!"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Allen",
                                    args![
                                        "C-can't... Lift...",
                                        "Legs and fingers...",
                                        "But... Must... Get...",
                                        "''Withstander'' medicine...",
                                        "from... Drawer... H-heeelp me!",
                                        "Adventurer! H-Heeeeelp~!"
                                    ],
                                )?;
                                ctx.var("hg_herb").set(Val::from(4))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                if ctx.var("hg_herb").get()? == 4 {
                                    ctx.lines_as(
                                        "Allen",
                                        args![
                                            "No...!",
                                            "There's a light at the",
                                            "the end of the tunnel!",
                                            "Please, adventurer! C-come",
                                            "a little closer to me! Don't",
                                            "let me step into the light!"
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    if ctx.var("hg_herb").get()? == 6 {
                                        ctx.lines(args![
                                            "^3355FFYou have given the",
                                            "Withstander medicine",
                                            "to Allen who guzzles",
                                            "it down very quickly.^000000"
                                        ])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Allen",
                                            args![
                                                "Blood... circulating...",
                                                "through limbs... Clarity...",
                                                "of thought... returning..."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Allen",
                                            args![
                                                "Ah, wh-what a relief. It looks",
                                                "like I-I live to see another",
                                                "day. Th-thank you so much..."
                                            ],
                                        )?;
                                        ctx.var("hg_herb").set(Val::from(7))?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else {
                                        if ctx.var("hg_herb").get()? == 7 {
                                            ctx.lines_as(
                                                "Allen",
                                                args![
                                                    "I... I can't...!",
                                                    "I can barely breathe!",
                                                    "A-adventurer, I need you",
                                                    "again! Please, c-come closer",
                                                    "to me! I-I think I need CPR!"
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        } else {
                                            if ctx.var("hg_herb").get()? == 9 {
                                                ctx.lines(args!["^3355FFYou give another dose", "of Withstander to Allen.^000000"])?;
                                                ctx.next()?;
                                                if ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])? == 3 {
                                                    ctx.lines_as(
                                                        "Allen",
                                                        args![
                                                            "Oh... Thank you...",
                                                            "I feel much b-better now.",
                                                            "My son Postell, he should",
                                                            "be coming soon. Thank you",
                                                            "for being so patient with me..."
                                                        ],
                                                    )?;
                                                    ctx.var("hg_herb").set(Val::from(10))?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                }
                                                ctx.lines_as(
                                                    "Allen",
                                                    args![
                                                        "Oh... Thank you...",
                                                        "Y-you're an angel, a regular",
                                                        "Florence Nightingale. Please",
                                                        "stay with m-me until my son",
                                                        "arrives. You wouldn't abandon",
                                                        "this sick man now, would you?"
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                    args!["(^333333Your son Postell", "better get here soon!^000000)"],
                                                )?;
                                                ctx.var("hg_herb").set(Val::from(7))?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            } else if ctx.var("hg_herb").get()? == 10 {
                                                ctx.lines_as(
                                                    "Allen",
                                                    args![
                                                        "Arrrgh! My ligaments!",
                                                        "They're in such pain!",
                                                        "Oh, oh no! I think my",
                                                        "bones are infected!",
                                                        "Help me, oh help me!"
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.call(Function::DoNpcEvent, vec![Val::from("Postell Schuwell#D::OnEnable")])?;
                                                ctx.lines_as("Allen", args!["Oh...", "My son...!", "P-Postell!"])?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Postell",
                                                    args![
                                                        "Uh... Hey, dad.",
                                                        "I brought that",
                                                        "medicine you're",
                                                        "supposed to",
                                                        "need so badly."
                                                    ],
                                                )?;
                                                ctx.call(
                                                    Function::NpcSpecialEffect,
                                                    vec![
                                                        ctx.constant("EF_CHANGECOLD")?,
                                                        ctx.constant("AREA")?,
                                                        Val::from("Postell Schuwell#D"),
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Allen",
                                                    args!["Thank goodness...", "G-give it to me!", "^333333*Gulp gulp gulp*^000000"],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Allen",
                                                    args!["W-wait...", "Something's not", "right. Bunkoll A", "doesn't taste like this..."],
                                                )?;
                                                ctx.call(
                                                    Function::Emotion,
                                                    vec![
                                                        ctx.constant("ET_SWEAT")?,
                                                        ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Postell Schuwell#D")])?,
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Postell",
                                                    args!["Bunkoll A...?", "I thougt you said that", "you wanted Kolbun A?"],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Allen",
                                                    args![
                                                        "My own son, lying to",
                                                        "me! Are you trying to",
                                                        "trick me, Postell?!",
                                                        "I told you get Bunkoll A!"
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                    args!["(^333333Wait, wait...", "Allen did say that", "he wanted Kolbun A...^000000)"],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Postell",
                                                    args![
                                                        "Right. Okay, I guess",
                                                        "I made a mistake? Um,",
                                                        "Dad, I'll be right back with",
                                                        "the medicine that you want.",
                                                        "It won't take long, so don't",
                                                        "stress yourself about it."
                                                    ],
                                                )?;
                                                ctx.close_window()?;
                                                ctx.var("hg_herb").set(Val::from(11))?;
                                                ctx.call(Function::DisableNpc, vec![Val::from("Postell Schuwell#D")])?;
                                                ctx.call(
                                                    Function::Emotion,
                                                    vec![
                                                        ctx.constant("ET_SWEAT")?,
                                                        Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                                                    ],
                                                )?;
                                                ctx.call(Function::Emotion, vec![ctx.constant("ET_SWEAT")?])?;
                                                return Err(Stop::End);
                                            } else if ctx.var("hg_herb").get()? == 11 {
                                                ctx.lines_as(
                                                    "Allen",
                                                    args![
                                                        "*Sniff* I should have known",
                                                        "that Postell really does care",
                                                        "about me deep down inside.",
                                                        "I wish I had never gotten",
                                                        "myself sick in Lighthalzen...",
                                                        "Then we both wouldn't suffer..."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Allen",
                                                    args![
                                                        "Don't worry, adventurer.",
                                                        "I haven't forgotten about",
                                                        "you. Please take a look inside",
                                                        "the medicine drawer, and help",
                                                        "yourself to whatever you like~"
                                                    ],
                                                )?;
                                                ctx.var("hg_herb").set(Val::from(12))?;
                                                ctx.call(Function::ChangeQuest, vec![Val::from(8055), Val::from(8056)])?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            } else if ctx.var("hg_herb").get()? == 12 {
                                                ctx.lines_as(
                                                    "Allen",
                                                    args![
                                                        "Don't worry, adventurer.",
                                                        "I haven't forgotten about",
                                                        "you. Please take a look inside",
                                                        "the medicine drawer, and help",
                                                        "yourself to whatever you like~"
                                                    ],
                                                )?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            } else if ctx.var("hg_herb").get()? == 13 {
                                                ctx.lines_as(
                                                    "Allen",
                                                    args![
                                                        "So did you find the",
                                                        "Old Blue Box I put in",
                                                        "my medicine drawer? It's",
                                                        "not much, but I hope you",
                                                        "like it. Consider my way",
                                                        "of thanking you for your help~"
                                                    ],
                                                )?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            } else {
                                                ctx.lines_as(
                                                    "Allen",
                                                    args![
                                                        "*Groooooan~*",
                                                        "Arrrgh, my back hurts...",
                                                        "And my stomach... Why do",
                                                        "I have to suffer like this",
                                                        "while other people get to",
                                                        "live happy, luxurious lives?"
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
                } else {
                    ctx.lines_as(
                        "Allen",
                        args![
                            "*Groooooan~*",
                            "Arrrgh, my back hurts...",
                            "And my stomach... Why do",
                            "I have to suffer like this",
                            "while other people get to",
                            "live happy, luxurious lives?"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                step = AllenSchuwellStep::OnTouch;
                continue 'machine;
            }
            AllenSchuwellStep::OnTouch => {
                if ctx.var("hg_herb").get()? == 4 {
                    ctx.lines_as(
                        "Allen",
                        args![
                            "^333333*Cough cough!*^000000",
                            "W-Withstander...!",
                            "I need m-my Withstander!",
                            "P-please! ^333333*Cough cough!*^000000"
                        ],
                    )?;
                    ctx.var("hg_herb").set(Val::from(5))?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("#DrawerOpener::OnEnable")])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("hg_herb").get()? == 7 {
                    ctx.lines_as(
                        "Allen",
                        args![
                            "^333333*Whew*^000000",
                            "I... I can breathe again!",
                            "B-but what if I have another",
                            "attack?! Please, g-get me",
                            "another dose of Withstander,",
                            "just to be on the safe side!"
                        ],
                    )?;
                    ctx.var("hg_herb").set(Val::from(8))?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("#DrawerOpener::OnEnable")])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn allen_schuwell(ctx: &Ctx) -> Script {
    allen_schuwell_run(ctx, AllenSchuwellStep::Start, Vec::new()).map(|_| ())
}

pub fn allen_schuwell_ontouch(ctx: &Ctx) -> Script {
    allen_schuwell_run(ctx, AllenSchuwellStep::OnTouch, Vec::new()).map(|_| ())
}

fn postell_schuwell_d_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Postell", args!["Hey, uh...", "I'm in the middle", "of something here."])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn postell_schuwell_d(ctx: &Ctx) -> Script {
    postell_schuwell_d_body(ctx, Vec::new()).map(|_| ())
}

fn postell_schuwell_d_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Postell Schuwell#D")])?;
    return Err(Stop::End);
}

pub fn postell_schuwell_d_oninit(ctx: &Ctx) -> Script {
    postell_schuwell_d_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn postell_schuwell_d_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("Postell Schuwell#D")])?;
    ctx.call(Function::InitNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn postell_schuwell_d_onenable(ctx: &Ctx) -> Script {
    postell_schuwell_d_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn postell_schuwell_d_ontimer100000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Postell Schuwell#D")])?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn postell_schuwell_d_ontimer100000(ctx: &Ctx) -> Script {
    postell_schuwell_d_ontimer100000_body(ctx, Vec::new()).map(|_| ())
}

fn postell_schuwell_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("hg_herb").get()? == 1 {
        ctx.lines_as(
            "Postell",
            args![
                "I really like living",
                "in Hugel. Fresh air,",
                "clean water, everyone's",
                "friendly and nice. What",
                "more could I want?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Postell",
            args![
                "Oh, what's that?",
                "My dad wants me to",
                "bring him some medicine?",
                "Again? Well, he was really",
                "sick a long time ago, but",
                "nowadays, I'm not so sure."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Postell",
            args![
                "I mean, I'm pretty sure",
                "his illnesses are all, you",
                "know, psychosomatic.",
                "Besides, I'm worried that",
                "all the medicines he's taking",
                "are habit forming, you know?"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("But your father needs you!:Habit forming?")])? {
            1 => {
                ctx.lines_as(
                    "Allen's Voice",
                    args![
                        "Yeah, I guess you're right.",
                        "Even if I think he's nuts,",
                        "and I don't agree with him,",
                        "I should at least show that",
                        "I care about my dad. What",
                        "medicine did he say he needed?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Postell",
                    args![
                        "''Kolbun A?'' Alright,",
                        "I better make sure that",
                        "it's safe for him to take.",
                        "Who knows, that stuff might",
                        "make him worse. Anyway,",
                        "I'll try to get it to him soon."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Postell",
                    args![
                        "Hey, would you do",
                        "me a favor and make",
                        "sure that my dad doesn't",
                        "get himself into any",
                        "more trouble? I'd really",
                        "appreciate it if you do."
                    ],
                )?;
                ctx.var("hg_herb").set(Val::from(2))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(8053), Val::from(8054)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Postell",
                    args![
                        "Yeah, like he's addicted",
                        "to the medicine. That's why",
                        "I'm trying to make sure he",
                        "gets as little of it as possible! Honestly, I think he drinks",
                        "too much medicine already..."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if ctx.var("hg_herb").get()?.number()? > 1 {
        ctx.lines_as(
            "Postell",
            args![
                "Kolbun A, Kolbun A...",
                "Let's see, is it really",
                "safe for my dad to take?",
                "If it isn't, I better come up",
                "with some replacement..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Postell",
            args![
                "I really like living",
                "in Hugel. Fresh air,",
                "clean water, everyone's",
                "friendly and nice. What",
                "more could I want?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn postell_schuwell(ctx: &Ctx) -> Script {
    postell_schuwell_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum UpperDrawerFirstStep {
    Start,
    OnTouch,
}

fn upper_drawer_first_run(ctx: &Ctx, mut step: UpperDrawerFirstStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_rand = Val::from(0);
    'machine: loop {
        match step {
            UpperDrawerFirstStep::Start => {
                step = UpperDrawerFirstStep::OnTouch;
                continue 'machine;
            }
            UpperDrawerFirstStep::OnTouch => {
                l_rand = ctx.call(Function::Rand, vec![Val::from(1), Val::from(7)])?;
                if ctx.var("hg_herb").get()?.number()? < 5 {
                    ctx.lines(args![
                        "^3355FFThere are several",
                        "books and a few",
                        "liquid medicines",
                        "inside this drawer.^000000"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("hg_herb").get()? == 5 {
                        ctx.lines_as(
                            "Allen",
                            args![
                                "Oh... Oh...",
                                "Oooh... The pain",
                                "is gone! It's a miracle!",
                                "Maybe I'll just take the",
                                "Withstander later..."
                            ],
                        )?;
                        ctx.var("hg_herb").set(Val::from(4))?;
                        ctx.next()?;
                        l_rand = ctx.call(Function::Rand, vec![Val::from(1), Val::from(7)])?;
                        if l_rand.clone() == 4 {
                            ctx.lines(args!["^3355FFYou find an old book", "inside the drawer.^000000"])?;
                            ctx.next()?;
                            ctx.lines_as("Welcome to Lighthalzen!", args!["Author: Tupetso Iltekka"])?;
                            ctx.next()?;
                            ctx.lines(args!["......", ".........", "............"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "(^333333I guess Allen used",
                                    "to live in Lighthalzen.",
                                    "I guess that's why he'd",
                                    "keep this old PR brochure...^000000)"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if l_rand.clone() == 5 {
                            ctx.lines(args![
                                "^3355FFYou find Allen's",
                                "journal inside the",
                                "drawer and quickly",
                                "leaf through its pages.",
                                "It's difficult to read since",
                                "the pages are old and worn.^000000"
                            ])?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "Let's see, let's see...",
                                    "Rekenber Corporation...",
                                    "Something about the Slums....",
                                    "Enormous funds... Experiments.",
                                    "Lab... Escape... Hugel... Um...",
                                    "Dr. Morriphen... Side effects?"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["Wh-what...?", "Do you really", "need that medicine?!"],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    } else {
                        if ctx.var("hg_herb").get()? == 6 {
                            return Err(Stop::End);
                        } else if ctx.var("hg_herb").get()? == 8 {
                            ctx.lines_as(
                                "Allen",
                                args![
                                    "Oh... Oh...",
                                    "Oooh... The pain",
                                    "is gone! It's a miracle!",
                                    "Maybe I'll just take the",
                                    "Withstander later..."
                                ],
                            )?;
                            ctx.var("hg_herb").set(Val::from(7))?;
                            ctx.next()?;
                            if l_rand.clone() == 4 {
                                ctx.lines(args!["^3355FFYou find an old book", "inside the drawer.^000000"])?;
                                ctx.next()?;
                                ctx.lines_as("Welcome to Lighthalzen!", args!["Author: Tupetso Iltekka"])?;
                                ctx.next()?;
                                ctx.lines(args!["......", ".........", "............"])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args![
                                        "(^333333I guess Allen used",
                                        "to live in Lighthalzen.",
                                        "I guess that's why he'd",
                                        "keep this old PR brochure...^000000)"
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if l_rand.clone() == 5 {
                                ctx.lines(args![
                                    "^3355FFYou find Allen's",
                                    "journal inside the",
                                    "drawer and quickly",
                                    "leaf through its pages.",
                                    "It's difficult to read since",
                                    "the pages are old and worn.^000000"
                                ])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args![
                                        "Let's see, let's see...",
                                        "Rekenber Corporation...",
                                        "Something about the Slums....",
                                        "Enormous funds... Experiments.",
                                        "Lab... Escape... Hugel... Um...",
                                        "Dr. Morriphen... Side effects?"
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args!["Wh-what...?", "Do you really", "need that medicine?!"],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        } else if ctx.var("hg_herb").get()? == 9 {
                            return Err(Stop::End);
                        } else if ctx.var("hg_herb").get()? == 12 {
                            ctx.lines(args!["^3355FFYou find an", "Old Blue Box deep", "inside the drawer.^000000"])?;
                            ctx.close_window()?;
                            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_CONE")?])?;
                            ctx.var("hg_herb").set(Val::from(13))?;
                            ctx.call(Function::GetItem, vec![Val::from(603), Val::from(1)])?;
                            ctx.call(Function::GetExperience, vec![Val::from(50000), Val::from(0)])?;
                            ctx.call(Function::CompleteQuest, vec![Val::from(8056)])?;
                        } else if l_rand.clone().number()? > 3 {
                            ctx.lines(args!["^3355FFYou find an old book", "inside the drawer.^000000"])?;
                            ctx.next()?;
                            ctx.lines_as("Welcome to Lighthalzen!", args!["Author: Tupetso Iltekka"])?;
                            ctx.next()?;
                            ctx.lines(args!["......", ".........", "............"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "(^333333I guess Allen used",
                                    "to live in Lighthalzen.",
                                    "I guess that's why he'd",
                                    "keep this old PR brochure...^000000)"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            ctx.lines(args![
                                "^3355FFYou find Allen's",
                                "journal inside the",
                                "drawer and quickly",
                                "leaf through its pages.",
                                "It's difficult to read since",
                                "the pages are old and worn.^000000"
                            ])?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "Let's see, let's see...",
                                    "Rekenber Corporation...",
                                    "Something about the Slums....",
                                    "Enormous funds... Experiments.",
                                    "Lab... Escape... Hugel... Um...",
                                    "Dr. Morriphen... Side effects?"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn upper_drawer_first(ctx: &Ctx) -> Script {
    upper_drawer_first_run(ctx, UpperDrawerFirstStep::Start, Vec::new()).map(|_| ())
}

pub fn upper_drawer_first_ontouch(ctx: &Ctx) -> Script {
    upper_drawer_first_run(ctx, UpperDrawerFirstStep::OnTouch, Vec::new()).map(|_| ())
}

fn upper_drawer_second_run(ctx: &Ctx, mut step: UpperDrawerSecondStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            UpperDrawerSecondStep::Start => {
                step = UpperDrawerSecondStep::OnInit;
                continue 'machine;
            }
            UpperDrawerSecondStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Upper Drawer#Second")])?;
                return Err(Stop::End);
            }
            UpperDrawerSecondStep::OnTouch => {
                if ctx.var("hg_herb").get()? == 5 {
                    ctx.lines(args!["^3355FFYou take out one dose of", "Withstander from the drawer.^000000"])?;
                    ctx.var("hg_herb").set(Val::from(6))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("hg_herb").get()? == 8 {
                    ctx.lines(args!["^3355FFYou take out one dose of", "Withstander from the drawer.^000000"])?;
                    ctx.var("hg_herb").set(Val::from(9))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines(args!["^3355FFThere are several books", "kept inside this drawer.^000000"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        }
    }
}

pub fn upper_drawer_second(ctx: &Ctx) -> Script {
    upper_drawer_second_run(ctx, UpperDrawerSecondStep::Start, Vec::new()).map(|_| ())
}

pub fn upper_drawer_second_oninit(ctx: &Ctx) -> Script {
    upper_drawer_second_run(ctx, UpperDrawerSecondStep::OnInit, Vec::new()).map(|_| ())
}

pub fn upper_drawer_second_ontouch(ctx: &Ctx) -> Script {
    upper_drawer_second_run(ctx, UpperDrawerSecondStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn draweropener(ctx: &Ctx) -> Script {
    draweropener_run(ctx, DraweropenerStep::Start, Vec::new()).map(|_| ())
}

pub fn draweropener_oninit(ctx: &Ctx) -> Script {
    draweropener_run(ctx, DraweropenerStep::OnInit, Vec::new()).map(|_| ())
}

pub fn draweropener_onenable(ctx: &Ctx) -> Script {
    draweropener_run(ctx, DraweropenerStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn draweropener_ontimer1000(ctx: &Ctx) -> Script {
    draweropener_run(ctx, DraweropenerStep::OnTimer1000, Vec::new()).map(|_| ())
}

pub fn draweropener_ontimer4500(ctx: &Ctx) -> Script {
    draweropener_run(ctx, DraweropenerStep::OnTimer4500, Vec::new()).map(|_| ())
}

fn morriphen_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("hg_bio").get()? == 0 {
        if ctx.var("hg_herb").get()? == 13 {
            ctx.lines_as(
                "Morriphen",
                args![
                    "Wh-who are you, and what are",
                    "you looking for? If you're here",
                    "for the stuff, then you came",
                    "too early. As you can see,",
                    "I don't feel well enough to",
                    "speak to you right now."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Morriphen", args!["^333333*Cough Cough*", "*Cough Cough*^000000"])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "Holy--! Are you alright?",
                    "Y-you're coughing up blood!",
                    "Relax, I was just passing by!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Morriphen",
                args![
                    "R-really? Well then, I'm",
                    "sorry to have bothered you,",
                    "then. I guess I'm just a little",
                    "jumpy is all. ^333333*Cough, cough~*^000000"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "Hey... You don't sound",
                    "too good. Do you want me",
                    "to bring you to a hospital?",
                    "That's a really nasty cough",
                    "you've got there, so maybe",
                    "you should get it checked."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Morriphen",
                args![
                    "No, thanks, I'm fine.",
                    "I just haven't gone home",
                    "for a few months and wasn't",
                    "able to get my prescription",
                    "refilled, that's all. ^333333*Haaak*",
                    "*Cough-cough-cooooooough!*^000000"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Hmm...", "What should I do?"],
            )?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Leave him:Help him")])? {
                1 => {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "You know, I'm a little",
                            "worried about your coughing,",
                            "but I don't think there's any",
                            "way that I can help you..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Morriphen",
                        args![
                            "^333333*Cough cough*^000000",
                            "Don't worry, I'll be",
                            "just fine. You take care",
                            "of yourself, you hear?"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Hey...", "Are you sure that", "you'll be alright?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Morriphen",
                        args![
                            "^333333*Cough cough*^000000",
                            "Don't worry, I'll be",
                            "just fine. You take care",
                            "of yourself, you hear?",
                            "^333333*Haaaak* *Reeetch*^000000"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["...", "......", "........."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Morriphen",
                        args![
                            "^333333*Reeeeetch*",
                            "*Cooooough Cough*",
                            "*Hhhh, Hhhhh, Hhhh*",
                            "*Haaaaack* *Retch*^000000"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Please, mister.",
                            "Let me help you.",
                            "I can't just leave",
                            "you alone if you",
                            "sound like that."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Morriphen",
                        args![
                            "...............................",
                            "I didn't, I didn't want to",
                            "bother you, b-but if you insist",
                            "oh h-helping me, then maybe",
                            "I'll ask you to do me a favor."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Morriphen",
                        args![
                            "Would you visit my home in",
                            "Hugel and ask my wife, Siria,",
                            "for my medication? She knows",
                            "where to find it, and will give",
                            "it to you. Oh, and please don't",
                            "tell her anything else, okay?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Morriphen",
                        args![
                            "She's always worried about",
                            "me getting worse, and I don't",
                            "want to give her any reason to",
                            "get stressed. Please just tell",
                            "her that I ran out of medicine."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Alright.", "I'll come back here", "as soon as I can."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Morriphen",
                        args!["Thanks so much.", "^333333*Cough Cough*", "*Cough Cough*^000000"],
                    )?;
                    ctx.var("hg_bio").set(Val::from(1))?;
                    ctx.call(Function::SetQuest, vec![Val::from(11009)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        } else {
            ctx.lines_as(
                "Morriphen",
                args!["Ugh... I don't feel", "very well. I guess", "I better call it a day..."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        if ctx.var("hg_bio").get()? == 1 {
            ctx.lines_as(
                "Morriphen",
                args![
                    "Please ask my ^333333*Cough*^000000",
                    "wife Siria in my home in",
                    "^333333*Reeetch*^000000 Hugel for m-my",
                    "medicine. Th-thank you...",
                    "^333333*Haaaaack* *Hhh, haaaack*^000000"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("hg_bio").get()? == 2 {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "Oh no... Morriphen",
                        "doesn't look good at all!",
                        "I better hurry and bring",
                        "him medicine from Siria",
                        "in Hugel as soon as I can!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("hg_bio").get()? == 3 {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Oh no... Morriphen",
                            "doesn't look good at all!",
                            "I better hurry and bring",
                            "him medicine from Dono",
                            "in Hugel as soon as I can!"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("hg_bio").get()? == 4 {
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![
                                "Oh no... Morriphen",
                                "doesn't look good at all!",
                                "I better hurry and bring",
                                "him medicine from Dono",
                                "in Hugel as soon as I can!"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if ctx.var("hg_bio").get()? == 5 {
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "Oh no... Morriphen",
                                    "doesn't look good at all!",
                                    "I better hurry and bring",
                                    "him medicine from Dono",
                                    "in Hugel as soon as I can!"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.var("hg_bio").get()? == 6 {
                            ctx.lines(args![
                                "^3355FFYou administer the",
                                "medicine to Morriphen,",
                                "who slowly regains",
                                "consciousness.^000000"
                            ])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Morriphen",
                                args![
                                    "^333333*Cough, cough*^000000",
                                    "Wh-where am I?",
                                    "I'm... not dead yet?",
                                    "Oh, you! It seems I was",
                                    "right to trust you. So did my",
                                    "wife give you my medicine?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "Actually, Siria also ran",
                                    "out of your medicine when",
                                    "I arrived at your house, so",
                                    "she told me to go get some",
                                    "from your friend, Dono."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Morriphen",
                                args![
                                    "Oh, no, what have you done?",
                                    "My wife's life depends on that",
                                    "medicine too! Oh no, if she",
                                    "doesn't get any, she'll die",
                                    "from my stupidity! Please!",
                                    "I beg you, bring this to her!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Morriphen",
                                args![
                                    "That woman is everything",
                                    "to me. If she's gone, then",
                                    "I don't know what I'll do.",
                                    "Please take this medicine",
                                    "to her as soon as you can!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "Don't worry, I'm on my",
                                    "way. I'll hurry as fast as",
                                    "I can back to Hugel to",
                                    "give Siria this medicine."
                                ],
                            )?;
                            ctx.var("hg_bio").set(Val::from(7))?;
                            ctx.call(Function::ChangeQuest, vec![Val::from(11014), Val::from(11015)])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.var("hg_bio").get()? == 7 {
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "What am I doing...?",
                                    "I need to return to",
                                    "Hugel and deliver this",
                                    "medicine to Siria as",
                                    "soon as possible!"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.var("hg_bio").get()? == 8 {
                            ctx.lines_as(
                                "Morriphen",
                                args![
                                    "You're back! So, have you",
                                    "seen my wife? She's okay,",
                                    "isn't she? Why aren't you",
                                    "saying--oh no. Oh dear God,",
                                    "no. D-don't tell me she's dead!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Morriphen",
                                args![
                                    "She has to be fine...!",
                                    "If anything happened",
                                    "to her, I... I won't forgive",
                                    "you! No one would find your",
                                    "body! No, wait, wait... No, I'm",
                                    "sorry, I'm just so anxious...!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Morriphen",
                                args![
                                    "^333333*Sob*^000000 Please forgive me!",
                                    "Please tell me my wife is",
                                    "okay. I'm sorry, everything",
                                    "is just so overwhelming right",
                                    "now. ^333333*Sob*^000000 Please, please..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "Siria is fine.",
                                    "Now are you satisfied?",
                                    "If you're that worried about",
                                    "her, you ought to see her",
                                    "more often. Geez..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Morriphen",
                                args![
                                    "Hallelujah! I'm so",
                                    "relieved! She's alive!",
                                    "I'm sorry, I'd be home",
                                    "more often, but I'm always",
                                    "struggling here to make ends",
                                    "meet. Thank you so much!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["Don't sweat it.", "It was just the right", "thing to do, that's all."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Morriphen",
                                args![
                                    "You're such a kind person...",
                                    "You save my life and my wife's",
                                    "life. I'll do anything you ask me to do to repay you! Consider me",
                                    "your humble servant from now on. What is your command, oh master?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "That's...",
                                    "That's going too far.",
                                    "Becoming my servant for",
                                    "life isn't necessary, you know."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Morriphen",
                                args![
                                    "Wait, I know! If you won't",
                                    "accept my life, then I can",
                                    "give you someone else's. Do",
                                    "you have any enemies! I'll go",
                                    "out and kill someone for you!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "...That's fine.",
                                    "Actually, I'm more interested",
                                    "in hearing how you, Siria, and",
                                    "Dono are all connected. When",
                                    "I spoke to Siria, she told me",
                                    "that you two weren't married..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "Siria did mention",
                                    "that she's greatly",
                                    "indebted to you for",
                                    "some reason, and she'll",
                                    "never be able to fully repay",
                                    "you, not even with her life."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Morriphen",
                                args![
                                    "She said that? ^333333*Sigh*^000000",
                                    "Poor girl, I've asked so",
                                    "many times not to think",
                                    "that way. Alright, if that's",
                                    "what you really want, then",
                                    "I will tell you our story."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Morriphen",
                                args![
                                    "Dono and I have been friends",
                                    "since childhood. We did almost",
                                    "everything together. Even when",
                                    "we started our professional",
                                    "careers, we both decided to work for the Rekenber Corporation."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Morriphen",
                                args![
                                    "Dono eventually became in",
                                    "charge of Rekenber's Medical",
                                    "Experimental Research, while",
                                    "I spearheaded projects for",
                                    "Rekenber's weapon research."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Morriphen",
                                args![
                                    "Although our areas of expertise",
                                    "are different, Dono and I found",
                                    "that our collaborations were",
                                    "productive for both of us. But",
                                    "then, one day, I dropped by",
                                    "Dono's office and saw her."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Morriphen",
                                args![
                                    "She was floating in a giant",
                                    "test tube, and I couldn't take",
                                    "my eyes off her: she was the",
                                    "most beautiful thing I'd ever",
                                    "seen. But Dono said she was",
                                    "nothing but an experiment..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Morriphen",
                                args![
                                    "He told me not to be so",
                                    "curious, but I didn't listen.",
                                    "Ever since then, I went to his office as often as I could, making",
                                    "stupid excuses to see her. I just couldn't help it. I was in love."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Morriphen",
                                args![
                                    "Then, I heard news that Dono's",
                                    "research wasn't producing the",
                                    "expected results, and that he",
                                    "would be shutting everything",
                                    "down. All experiments would",
                                    "be efficiently disposed."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Morriphen",
                                args![
                                    "There was a snowstorm",
                                    "that night, but I braved the",
                                    "freezing cold to sneak into the",
                                    "Rekenber Laboratory. I broke her out of her test tube, set the lab",
                                    "on fire, and got the hell out."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Morriphen",
                                args![
                                    "I couldn't know it at the",
                                    "time, but I was exposed to",
                                    "some chemicals during the fire.",
                                    "They damaged my tissues badly,",
                                    "so that's why I'm so dependent",
                                    "on these medicines now."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Morriphen",
                                args![
                                    "Anyway, we ran from the lab,",
                                    "escaping to the mountains.",
                                    "I just carried her on my back,",
                                    "as far as I could go. Finally,",
                                    "I couldn't fight the dizziness,",
                                    "and I suddenly blacked out."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Morriphen",
                                args![
                                    "I remember seeing a tall, robed",
                                    "man right before I fainted. He",
                                    "said something about bringing",
                                    "us to Hugel, a resting place",
                                    "for the lost, those born from Odin's shadow. It was strange..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Morriphen",
                                args![
                                    "I couldn't see his face.",
                                    "His hood was covering it,",
                                    "but I think his name was, um...",
                                    "Mawon... Mawong? Anyway, when",
                                    "we woke up, we were lying in",
                                    "warm beds inside Hugel Village."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Morriphen",
                                args![
                                    "The people we met were very",
                                    "accomdating, welcome, and",
                                    "friendly. They also didn't",
                                    "ask us any questions: we were",
                                    "just accepted. The girl I saved didn't know how to speak then..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Morriphen",
                                args![
                                    "As soon as I got better,",
                                    "I taught her our language,",
                                    "and even gave her a name,",
                                    "Siria. She was no longer an",
                                    "experiment, but my beloved. Still, we didn't live happily ever after."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Morriphen",
                                args![
                                    "I had to make living for the",
                                    "two of us to survive, so I found work in Einbroch and Lighthalzen.",
                                    "Eventually, I bumped into Dono",
                                    "inside a shabby herb shop in Lighthalzen. I felt so ashamed..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Morriphen",
                                args![
                                    "I burned up his lab, which",
                                    "must've hurt his reputation",
                                    "in the company. Still, when",
                                    "I told him what happened, he",
                                    "seemed to understand. He",
                                    "even started to help me..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Morriphen",
                                args![
                                    "You know, Siria was supposed",
                                    "to become a mass murder weapon.",
                                    "The reason why Dono's experiment failed was became she resisted",
                                    "their plans. She's more than just an experiment or weapon, you know."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Morriphen",
                                args![
                                    "If everything had turned out",
                                    "as Dono planned, Siria would",
                                    "become a soldier, and I would",
                                    "be supplying her armaments,",
                                    "cutting edge weapons at the pinnacle of military development."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Morriphen",
                                args![
                                    "While I was working for Rekenber, I was working on mass producing",
                                    "imitations of the legendary sword, ''Executioner.'' They wouldn't be",
                                    "perfect copies, but imagine what would've happened if I succeeded."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Morriphen",
                                args![
                                    "When I think about it, we all",
                                    "could have been rich. I was in",
                                    "charge of weapon manufacturing",
                                    "after all, and our biggest",
                                    "customer, Arunafeltz, would",
                                    "have given us good business."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Morriphen",
                                args![
                                    "I made my choice, and I have",
                                    "no regrets. In fact, I wouldn't",
                                    "change my life or give away",
                                    "Siria for the world. However,",
                                    "Dono would probably think",
                                    "differently: work was his life."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Morriphen",
                                args![
                                    "I get the feeling that Dono",
                                    "would never tell Rekenber about",
                                    "me and Siria. I can never repay",
                                    "him for the helping us now, but",
                                    "I'm sure that he must hate me",
                                    "deep down inside."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "Don't think like that.",
                                    "I'm sure Dono still thinks",
                                    "of you as his friend. He's",
                                    "bitter about what happened,",
                                    "but he won't abandon you.",
                                    "Why don't you go see him?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "Besides, you have an",
                                    "excuse to go see him now,",
                                    "your medicine, right? I'm",
                                    "sure you can repay him by",
                                    "visiting him, and living",
                                    "happily with your Seria."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Morriphen",
                                args![
                                    "^333333*Sob*^000000 You're right...",
                                    "I'm so sorry Dono! ^333333*Sniff*^000000",
                                    "Thank you so much. If you",
                                    "ever visit Hugel again, come",
                                    "by and see me. You've really",
                                    "helped me and Seria so much."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "Don't worry about it.",
                                    "Just get better so that",
                                    "I won't have to worry",
                                    "about you. Take care~"
                                ],
                            )?;
                            ctx.var("hg_bio").set(Val::from(9))?;
                            ctx.call(Function::GetExperience, vec![Val::from(700000), Val::from(0)])?;
                            ctx.call(Function::CompleteQuest, vec![Val::from(11016)])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.var("hg_bio").get()? == 9 {
                            ctx.lines_as(
                                "Morriphen",
                                args![
                                    "Oh, long time no see~",
                                    "Yes, my wife and I can't",
                                    "possibly be any happier.",
                                    "We've been taking it easy,",
                                    "and managed to make full",
                                    "recoveries. Amazing, isn't it?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Morriphen",
                                args![
                                    "Oh, and we finally got",
                                    "married in a small ceremony",
                                    "here in Hugel. Why don't you",
                                    "come visit our house sometime?"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            ctx.lines_as(
                                "Morriphen",
                                args![
                                    "Oh, long time no see~",
                                    "Yes, my wife and I can't",
                                    "possibly be any happier.",
                                    "We've been taking it easy,",
                                    "and managed to make full",
                                    "recoveries. Amazing, isn't it?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Morriphen",
                                args![
                                    "Oh, and we finally got",
                                    "married in a small ceremony",
                                    "here in Hugel. Why don't you",
                                    "come visit our house sometime?"
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
    Ok(Val::from(0))
}

pub fn morriphen(ctx: &Ctx) -> Script {
    morriphen_body(ctx, Vec::new()).map(|_| ())
}

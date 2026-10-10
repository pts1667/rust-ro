use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn man_king_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("aru_monas").get()?.number()? < 22 {
        ctx.lines(args![
            "^3355FFYou find a man lying on",
            "the floor, wearing torn",
            "yet luxurious clothing.",
            "He doesn't seem to be",
            "breathing at all...^000000."
        ])?;
        ctx.next()?;
        ctx.mes("^3355FFHe's dead.^000000")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("aru_monas").get()? == 22 || ctx.var("aru_monas").get()? == 23) {
        ctx.lines(args![
            "^3355FFYou find a man lying on",
            "the floor, wearing torn",
            "yet luxurious clothing.",
            "He doesn't seem to be",
            "breathing at all...^000000."
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFYou bring your ears more",
            "closely to his mouth: it",
            "seems that he really is",
            "still barely alive.^000000"
        ])?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Touch Him:Ignore Him")])?) == 1 {
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["This man...", "He's so familiar", "for some reason."],
            )?;
            ctx.next()?;
            ctx.lines(args![
                "^3355FFAs you touch him, the",
                "man suddenly jumps",
                "up with a crazed yowl.^000000"
            ])?;
            ctx.var("aru_monas").set(Val::from(23))?;
            ctx.call(
                Function::Monster,
                vec![
                    Val::from("abbey03"),
                    Val::from(232),
                    Val::from(232),
                    Val::from("Dead King"),
                    Val::from(1875),
                    Val::from(1),
                    Val::from("Man#King::OnMyMobDead"),
                ],
            )?;
            ctx.call(Function::InitNpcTimer, vec![])?;
            ctx.call(Function::DisableNpc, vec![Val::from("Man#King")])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines(args![
            "^3355FFYou don't feel",
            "comfortable enough",
            "to touch this man.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.mes("^3355FFHe's dead... Now...^000000")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn man_king(ctx: &Ctx) -> Script {
    man_king_body(ctx, Vec::new()).map(|_| ())
}

fn man_king_onmymobdead_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("Dead Man#King::OnEnable")])?;
    return Err(Stop::End);
}

pub fn man_king_onmymobdead(ctx: &Ctx) -> Script {
    man_king_onmymobdead_body(ctx, Vec::new()).map(|_| ())
}

fn man_king_ontimer300000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::DisableNpc, vec![Val::from("Dead Man#King")])?;
    ctx.call(Function::EnableNpc, vec![Val::from("Man#King")])?;
    return Err(Stop::End);
}

pub fn man_king_ontimer300000(ctx: &Ctx) -> Script {
    man_king_ontimer300000_body(ctx, Vec::new()).map(|_| ())
}

fn dead_man_king_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(7766), Val::from(1)])? == 0 {
        ctx.lines(args![
            "^3355FFWait a second!",
            "Right now, you're carrying",
            "too many things with you.",
            "Please come back after",
            "using the Kafra Service",
            "to store some of your items.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("aru_monas").get()? == 23 {
        ctx.lines(args![
            "^3355FFYou have no idea why",
            "this dead man is moving",
            "around, so you decided to",
            "examine him. In his jacket,",
            "you find a shining medal...^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["This medal...", "This means that", "this man is...!"],
        )?;
        ctx.var("aru_monas").set(Val::from(24))?;
        ctx.call(Function::GetItem, vec![Val::from(7726), Val::from(1)])?;
        ctx.call(Function::DisableNpc, vec![Val::from("Dead Man#King")])?;
        ctx.call(Function::EnableNpc, vec![Val::from("Man#King")])?;
        ctx.call(Function::StopNpcTimer, vec![])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    return Err(Stop::End);
}

pub fn dead_man_king(ctx: &Ctx) -> Script {
    dead_man_king_body(ctx, Vec::new()).map(|_| ())
}

fn dead_man_king_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::InitNpcTimer, vec![])?;
    ctx.call(Function::EnableNpc, vec![Val::from("Dead Man#King")])?;
    return Err(Stop::End);
}

pub fn dead_man_king_onenable(ctx: &Ctx) -> Script {
    dead_man_king_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn dead_man_king_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Dead Man#King")])?;
    return Err(Stop::End);
}

pub fn dead_man_king_oninit(ctx: &Ctx) -> Script {
    dead_man_king_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn dead_man_king_ontimer150000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Dead Man#King")])?;
    ctx.call(Function::EnableNpc, vec![Val::from("Man#King")])?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn dead_man_king_ontimer150000(ctx: &Ctx) -> Script {
    dead_man_king_ontimer150000_body(ctx, Vec::new()).map(|_| ())
}

fn aideami_aru_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn aideami_aru(ctx: &Ctx) -> Script {
    aideami_aru_body(ctx, Vec::new()).map(|_| ())
}

fn aideami_aru_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("aru_monas").get()? == 26 {
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["There is a low wall here against the other wall, if I climb it I could reach the resting place of Tristan III..."],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Stay here:Climb the wall")])? {
            1 => {
                ctx.call(Function::Warp, vec![Val::from("nameless_n"), Val::from(158), Val::from(169)])?;
                return Err(Stop::End);
            }
            2 => {
                ctx.call(Function::Warp, vec![Val::from("abbey01"), Val::from(51), Val::from(15)])?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else {
        ctx.call(Function::Warp, vec![Val::from("nameless_n"), Val::from(158), Val::from(169)])?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn aideami_aru_ontouch(ctx: &Ctx) -> Script {
    aideami_aru_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn boss_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input = Val::from(0);
    shared::other_gm_npcs::f_gm_npc(ctx, vec![])?;
    ctx.lines_as("Patch", args!["Input."])?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("Now:How much?:P- How much?:Others")])? {
        1 => {
            ctx.lines(args![" ", (Val::from("") + ctx.var("aru_monas").get()?)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        2 => {
            let (input, status) = runtime::input_number(ctx, Some(0), Some(1000))?;
            l_input = input;
            ctx.var("aru_monas").set(l_input.clone())?;
            ctx.lines(args![" ", (Val::from("") + ctx.var("aru_monas").get()?)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        3 => {
            let (input, status) = runtime::input_number(ctx, Some(0), Some(1000))?;
            l_input = input;
            ctx.var("prt_curse").set(l_input.clone())?;
            ctx.lines(args![" ", (Val::from("") + ctx.var("prt_curse").get()?)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        4 => {
            ctx.var("prt_curse").set(Val::from(61))?;
            ctx.var("ra_tem_q").set(Val::from(12))?;
            ctx.var("rachel_camel").set(Val::from(25))?;
            ctx.var("lost_boy").set(Val::from(0))?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn boss(ctx: &Ctx) -> Script {
    boss_body(ctx, Vec::new()).map(|_| ())
}

fn niren_ss_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(907), Val::from(200)])? == 0 {
        ctx.lines(args![
            "^3355FFWait a second!",
            "Right now, you're carrying",
            "too many things with you.",
            "Please come back after",
            "using the Kafra Service",
            "to store some of your items.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.call(Function::Cutin, vec![Val::from("ra_gwoman"), Val::from(2)])?;
    ctx.lines_as(
        "High Priestess Niren",
        args![
            "So you wanted to talk to me?",
            "I'm pretty tired right now, but",
            "I can spare a moment or two.",
            "What would you like to ask?"
        ],
    )?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("Ask About Veins Smugglers:Ask About Zhed")])? {
        1 => {
            if ctx.var("aru_monas").get()? == 12 {
                ctx.lines_as(
                    "Niren",
                    args![
                        "Veins smugglers...",
                        "Would you explain",
                        "exactly what you mean",
                        "by Veins smugglers?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Niren",
                    args![
                        "Oh, them. Yes, I know",
                        "a little bit about them.",
                        "I'm afraid that I can't really",
                        "disclose certain information",
                        "about them since they're",
                        "a classified case..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Niren",
                    args![
                        "I'm afraid that I cannot give you information about them easily",
                        "as they mean a lot to us."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["Actually, I was curious", "about who they kidnapped."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Niren",
                    args![
                        "Oh, well, I suppose",
                        "I can tell you more",
                        "about their captive.",
                        "But first, I'd like to",
                        "ask you for a favor."
                    ],
                )?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("Accept:Do Not Refuse")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as(
                    "Niren",
                    args![
                        "When we realized the",
                        "kind of hostage the",
                        "smugglers captured, we",
                        "didn't know what to do with",
                        "him. We've had too many",
                        "internal problems to handle..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Niren",
                    args![
                        "Still, we felt that we",
                        "could use him in our plan,",
                        "so we held him in a secret",
                        "place whose location is only",
                        "known to a few people. But",
                        "then... We lost all contact."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Niren",
                    args![
                        "We sent investigators, but",
                        "we still have no idea what",
                        "happened. All they found was",
                        "a message left by one survivor:",
                        "''They're all demons.'' The",
                        "hostage is probably dead."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Niren",
                    args![
                        "He may have been a high",
                        "ranking official, but we",
                        "were too preoccupied",
                        "with other matters. In fact,",
                        "I haven't gotten the chance",
                        "to think of what to do with him..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Niren",
                    args![
                        "I want to ask you to",
                        "go to that place, and",
                        "figure out what happened",
                        "there, and if that high ranking",
                        "official from Rune-Midgarts is",
                        "still alive. I doubt it, but..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Niren",
                    args![
                        "The investigators we employ",
                        "in our temple usually pale",
                        "in comparison to you foreign",
                        "adventurers, so I have faith",
                        "that you can do it. Still,",
                        "it'll be quite dangerous."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Niren",
                    args![
                        "If you can do this, I'll",
                        "tell you the location of",
                        "that place: we usually call",
                        "it the monastery. It used to",
                        "be an actual monastery, but we",
                        "use it to hold captives now."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Niren",
                    args![
                        "I'm sorry, but I can't tell",
                        "you anything else about that",
                        "old place. It's located to the",
                        "southwest of Veins across the",
                        "sea. You should be able to find",
                        "a boat at South Veins Beach."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Niren",
                    args![
                        "However... Never mind.",
                        "I'd like to help you more,",
                        "but I'd be compromising my",
                        "position. I'm sorry. Please",
                        "come back and tell me if",
                        "you learn anything there."
                    ],
                )?;
                ctx.var("aru_monas").set(Val::from(13))?;
                ctx.next()?;
                ctx.lines_as(
                    "Niren",
                    args![
                        "I'll send a message to the",
                        "magistrate of Veins. If you",
                        "talk to him, he'll provide you",
                        "with some useful information",
                        "for your journey. Good luck."
                    ],
                )?;
                ctx.close_window()?;
            } else if ctx.var("aru_monas").get()?.number()? < 18 {
                ctx.lines_as(
                    "Niren",
                    args![
                        "Going to the monastery",
                        "isn't the problem: coming",
                        "back here alive will probably",
                        "be the real challenge for you."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Niren",
                    args![
                        "We can't send any troops",
                        "there until we understand",
                        "what they'd be confronting",
                        "there. That's why we need",
                        "you to investigate the area."
                    ],
                )?;
                ctx.close_window()?;
            } else if ctx.var("aru_monas").get()?.number()? < 25 {
                ctx.lines_as(
                    "Niren",
                    args![
                        "Still investigating the",
                        "monastery? Perhaps if",
                        "you found any records",
                        "left behind by one of",
                        "the residents over there..."
                    ],
                )?;
                ctx.close_window()?;
            } else if ctx.var("aru_monas").get()? == 25 {
                if ctx.call(Function::CountItem, vec![Val::from(7755)])?.number()? < 1 {
                    ctx.lines_as(
                        "Niren",
                        args![
                            "Still investigating the",
                            "monastery? Perhaps if",
                            "you found any records",
                            "left behind by one of",
                            "the residents over there..."
                        ],
                    )?;
                    ctx.close_window()?;
                } else {
                    ctx.lines_as(
                        "Niren",
                        args![
                            "You found this journal",
                            "in the monastery? Perfect.",
                            "Let me read it, and see",
                            "what we can learn..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Niren",
                        args![
                            "Hmm, this sounds really",
                            "bad. This is tragic, especially",
                            "we kept so many important",
                            "people over there. I'll request",
                            "to dispatch troops there right",
                            "away. Something must be done."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Niren",
                        args![
                            "There are too many problems",
                            "we have to confront now, but",
                            "this monastery issue needs",
                            "top priority. We won't be able",
                            "to handle it if another threat",
                            "comes from that place."
                        ],
                    )?;
                    ctx.call(Function::DelItem, vec![Val::from(7755), Val::from(1)])?;
                    ctx.var("aru_monas").set(Val::from(26))?;
                    ctx.call(Function::GetExperience, vec![Val::from(500000), Val::from(0)])?;
                    ctx.close_window()?;
                }
            } else {
                ctx.lines_as(
                    "Niren",
                    args![
                        "I suppose that Arunafeltz",
                        "and Rune-Midgarts will be too",
                        "busy with their own internal",
                        "issues to commit to war.",
                        "High Priest Zhed will be",
                        "glad to know about this."
                    ],
                )?;
                ctx.close_window()?;
            }
        }
        2 => {
            if ctx.var("aru_em").get()?.number()? < 10 {
                ctx.lines_as(
                    "High Priestess Niren",
                    args![
                        "I'm sorry...",
                        "I really don't want",
                        "to think about Zhed",
                        "right now. He's just..."
                    ],
                )?;
                ctx.close_window()?;
            } else {
                if ctx.var("aru_em").get()? == 10 {
                    ctx.lines_as(
                        "High Priestess Niren",
                        args![
                            "You wanted to ask me about",
                            "Beken--High Priest Zhed?",
                            "I can't do anything for him.",
                            "He dug his own grave by",
                            "allowing you into the",
                            "holy ground, didn't he?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "High Priestess Niren",
                        args!["I thought I warned you", "before to stay out of trouble."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "I'm not here to discuss",
                            "that. But I happened to",
                            "hear that you're the only",
                            "person that can help him now."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "High Priestess Niren",
                        args![
                            "...............................",
                            "...............................",
                            "..............................."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "High Priestess Niren",
                        args![
                            "It's true that Zhed and",
                            "I used to work well together,",
                            "but we no longer share the",
                            "same dream. You'd better find",
                            "someone else. Unlike him,",
                            "I actually want a war."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Weren't you guys friends?",
                            "Besides, you don't seem as",
                            "aggressive as those other",
                            "hard liner priests."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "High Priestess Niren",
                        args![
                            "Aggressive? Hmpf. If you",
                            "take a good look at those",
                            "moderates, you'll see that",
                            "they oppose war, but they're",
                            "also totally corrupt. In fact, they committed worse atrocities!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "High Priestess Niren",
                        args![
                            "The moderates conducted",
                            "inhumane testing on living",
                            "creatures, even humans!",
                            "And Zhed and I... Could do",
                            "nothing but pray to Freya.",
                            "Only war can this this chaos!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "High Priestess Niren",
                        args!["We need to start all", "over again. I can't help", "Zhed. Please leave..."],
                    )?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFIt doesn't look like",
                        "you can change High",
                        "Priestess Niren's mind.",
                        "For now, you'd better talk",
                        "to High Priest Zhed.^000000"
                    ])?;
                    ctx.var("aru_em").set(Val::from(11))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(2132), Val::from(2133)])?;
                    ctx.close_window()?;
                } else {
                    if (ctx.var("aru_em").get()? == 11 || ctx.var("aru_em").get()? == 12) {
                        ctx.lines_as(
                            "High Priestess Niren",
                            args![
                                "I already told you",
                                "how I feel about Zhed.",
                                "We were wrong all along...",
                                "This time, war is the answer."
                            ],
                        )?;
                        ctx.close_window()?;
                    } else {
                        if ctx.var("aru_em").get()? == 13 {
                            ctx.lines_as(
                                "High Priestess Niren",
                                args![
                                    "I already told you",
                                    "how I feel about Zhed.",
                                    "We were wrong all along...",
                                    "This time, war is the answer."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "Fine. If you're so sure",
                                    "of yourself, then you won't",
                                    "mind reading this letter",
                                    "from High Priest Zhed."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^3355FFNiren angrily grabbed the",
                                "letter, ripped the envelope",
                                "open, and began to read his",
                                "message. Her stern face slowly",
                                "softened as she slowly scanned",
                                "what Zhed had to tell her.^000000"
                            ])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "High Priestess Niren",
                                args![".................", ".................", "................."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "High Priestess Niren",
                                args![
                                    "That Zhed... He always",
                                    "did have a way with words.",
                                    "I suppose the old saying is",
                                    "true: the pen truly is mightier",
                                    "than the sword. *Sigh...*"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Sippie", args!["High Priestess", "Niren? Are you okay?"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "High Priestess Niren",
                                args!["Oh, I'm fine,", "thank you. I guess", "I'm just a little tired."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "High Priestess Niren",
                                args![
                                    "...............................",
                                    "I have to admit, there's truth",
                                    "in what Zhed's saying. We",
                                    "have a duty to protect our",
                                    "people, the pope, our holy",
                                    "ground. But I need to think..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "High Priestess Niren",
                                args![
                                    "Would you give me some",
                                    "time to myself? I'll let you",
                                    "know once I've made a decision."
                                ],
                            )?;
                            ctx.var("aru_em").set(Val::from(14))?;
                            ctx.close_window()?;
                        } else {
                            if ctx.var("aru_em").get()? == 14 {
                                if ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])? == 1 {
                                    ctx.lines_as(
                                        "High Priestess Niren",
                                        args!["...................", "...................", "..................."],
                                    )?;
                                    ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Sippie",
                                        args![
                                            "High Priestess Niren,",
                                            "you must be really tired.",
                                            "You look pale for some reason."
                                        ],
                                    )?;
                                    ctx.call(Function::Emotion, vec![ctx.constant("ET_PROFUSELY_SWEAT")?])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "High Priestess Niren",
                                        args![
                                            "I do feel rather drained.",
                                            "I don't know why I've been",
                                            "holding on to this hatred for",
                                            "so long. I guess I can finally",
                                            "just cast it to the winds..."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("High Priestess Niren", args!["Adventurer...", "What was your name?"])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args![
                                            ((Val::from("My name is ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                + Val::from("."))
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "High Priestess Niren",
                                        args![
                                            "Adventurer...",
                                            "You had the chance to talk",
                                            "with our pope once, but I doubt",
                                            "you'll be able to do so again",
                                            "^3311FFin private^000000. You'll definitely",
                                            "need our help for that end."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args!["Does this mean that", "you've decided to work", "with Zhed again?"],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines(args![
                                        "^3355FFNiren quietly nodded",
                                        "her head, and then",
                                        "resumed talking.^000000"
                                    ])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "High Priestess Niren",
                                        args![
                                            "I'm still not sure if",
                                            "I can join him in the long",
                                            "run. I mean, talking to the",
                                            "pope might not result in",
                                            "anything, actually. But",
                                            "it's still worth a try."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "High Priestess Niren",
                                        args![
                                            "However, I've never",
                                            "doubted Zhed's dedication",
                                            "to Arunafeltz. He deserves",
                                            "another chance from me."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "High Priestess Niren",
                                        args![
                                            "Our pope is always surrounded ",
                                            "by priests and followers sent",
                                            "by High Priest Vildt. He always",
                                            "manages to keep his eyes and",
                                            "ears on her. We need to get",
                                            "those spies away somehow..."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args!["What can we do", "about them, then?"],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "High Priestess Niren",
                                        args![
                                            "For now, just approach",
                                            "the pope as if you just",
                                            "wanted to leisurely chat.",
                                            "While you're doing that,",
                                            "try to gather information",
                                            "from the priests around her."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args!["Sure thing. You", "can count on me!"],
                                    )?;
                                    ctx.call(
                                        Function::Emotion,
                                        vec![
                                            ctx.constant("ET_BEST")?,
                                            Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                                        ],
                                    )?;
                                    ctx.var("aru_em").set(Val::from(15))?;
                                    ctx.call(Function::ChangeQuest, vec![Val::from(2134), Val::from(2135)])?;
                                    ctx.close_window()?;
                                } else {
                                    ctx.lines_as(
                                        "High Priestess Niren",
                                        args![
                                            "Ugh, this is giving me",
                                            "a headache. There's",
                                            "too much to consider...",
                                            "Too much at stake here..."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                }
                            } else {
                                if ctx.var("aru_em").get()? == 15 {
                                    ctx.lines_as(
                                        "High Priestess Niren",
                                        args![
                                            "Just approach the pope",
                                            "for a chat, and see what",
                                            "you can learn from the",
                                            "other priests around her."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                } else {
                                    if ctx.var("aru_em").get()? == 16 {
                                        ctx.lines_as(
                                            "High Priestess Niren",
                                            args!["How is the pope?", "I haven't had the chance", "to meet her recently, so..."],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                            args![
                                                "Well, she doesn't seem",
                                                "very well, just like the",
                                                "last time I saw her. I didn't",
                                                "learn anything about all those",
                                                "priests. Just that they've been",
                                                "working in the temple non-stop."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "High Priestess Niren",
                                            args![
                                                "Yes, they've been by her",
                                                "side twenty-fours a day.",
                                                "It's been like that for years."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.call(Function::Emotion, vec![ctx.constant("ET_KEK")?])?;
                                        ctx.lines_as(
                                            "Sippie",
                                            args![
                                                "Oh, High Priestess Niren,",
                                                "I love working for you, but",
                                                "sometimes I wish I could leave",
                                                "just to see my family. Even",
                                                "just once a year would be nice."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.call(Function::Emotion, vec![ctx.constant("ET_AHA")?])?;
                                        ctx.lines_as(
                                            "High Priestess Niren",
                                            args![
                                                "...............................",
                                                "You're absolutely right.",
                                                "That gives me an idea."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.call(Function::Emotion, vec![ctx.constant("ET_DELIGHT")?])?;
                                        ctx.lines_as("Sippie", args!["Huh? What is it?"])?;
                                        ctx.next()?;
                                        ctx.lines_as("High Priestess Niren", args!["You all need a vacation."])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "High Priestess Niren",
                                            args![
                                                ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("."))
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Yes?"])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "High Priestess Niren",
                                            args![
                                                "I should reward everyone",
                                                "in the Sky Garden for their",
                                                "contribution. However,",
                                                "they were hired by Vildt,",
                                                "so I can't directly give them",
                                                "leave for vacation. Hmm..."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("High Priestess Niren", args!["Sippie...", "I'll need a pen", "and some paper."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Sippie", args!["Yes, ma'am.", "There you go!"])?;
                                        ctx.call(Function::Cutin, vec![Val::from("ra_gwoman"), Val::from(255)])?;
                                        ctx.next()?;
                                        ctx.lines(args![
                                            "^3355FF*Scribble Scribble*^000000",
                                            "^3355FF*Scribble Scribble*^000000",
                                            "^3355FF*Scribble Scribble*^000000",
                                            "^3355FF*Scribble Scribble*^000000"
                                        ])?;
                                        ctx.next()?;
                                        ctx.call(Function::Cutin, vec![Val::from("ra_gwoman"), Val::from(2)])?;
                                        ctx.lines_as(
                                            "High Priestess Niren",
                                            args![
                                                "I administered a blessing",
                                                "for a child from a native",
                                                "family, and they were very",
                                                "grateful since most of the",
                                                "priests shun the natives."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "High Priestess Niren",
                                            args![
                                                "It turns out that this",
                                                "child is gifted at forgery.",
                                                "She doesn't like being too",
                                                "close to other people, but",
                                                "that's beside the point.",
                                                "Here, take this letter."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "High Priestess Niren",
                                            args![
                                                "Ah, and take this file that",
                                                "contains a sample of Vildt's",
                                                "handwriting. Bring them to",
                                                "a girl named Ishmael in a",
                                                "village in North Rachel.",
                                                "I hope she'll help us..."
                                            ],
                                        )?;
                                        ctx.var("aru_em").set(Val::from(17))?;
                                        ctx.call(Function::GetItem, vec![Val::from(7343), Val::from(1)])?;
                                        ctx.call(Function::ChangeQuest, vec![Val::from(2136), Val::from(2137)])?;
                                        ctx.close_window()?;
                                    } else if ctx.var("aru_em").get()? == 17 {
                                        ctx.lines_as(
                                            "High Priestess Niren",
                                            args![
                                                "Please deliver my letter",
                                                "and that file to Ishmael in",
                                                "North Rachel. Hopefully,",
                                                "she can use her talents",
                                                "to help us out."
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                    } else if ctx.var("aru_em").get()? == 19 {
                                        if ctx.call(Function::CountItem, vec![Val::from(7343)])?.number()? > 0 {
                                            ctx.lines_as("High Priestess Niren", args!["Have you met with Ishmael?"])?;
                                            ctx.next()?;
                                            ctx.lines(args!["^3355FFYou handed Niren the", "file forged by Ishmael.^000000"])?;
                                            ctx.next()?;
                                            ctx.call(Function::Emotion, vec![ctx.constant("ET_STARE")?])?;
                                            ctx.lines_as(
                                                "High Priestess Niren",
                                                args![
                                                    "Perfect! This forgery is",
                                                    "so well done, I'm sure",
                                                    "that even Vildt won't be",
                                                    "know if he wrote it or not."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "High Priestess Niren",
                                                args![
                                                    ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                        + Val::from("...")),
                                                    "I'm going to use this to",
                                                    "send the priests and followers",
                                                    "on vacation. Then me and Zhed",
                                                    "can talk to the pope without",
                                                    "any unecessary interruptions."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "High Priestess Niren",
                                                args![
                                                    "I really don't want to burden",
                                                    "our pope with what Zhed and I",
                                                    "have to say... But that's much",
                                                    "preferable to doing nothing",
                                                    "and experiencing the downfall",
                                                    "of our beloved Arunafeltz."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "High Priestess Niren",
                                                args![
                                                    "Sippie, please",
                                                    "deliver this vacation",
                                                    "approval to the priests and",
                                                    "followers in the Sky Garden."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as("Sippie", args!["Yes, ma'am!"])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "High Priestess Niren",
                                                args![
                                                    ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                        + Val::from(",")),
                                                    "please go back to Zhed,",
                                                    "and let him know everything",
                                                    "is ready. Then I want you to",
                                                    "meet us in the ^3131FFSky Garden^000000."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Yes!"])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "High Priestess Niren",
                                                args![
                                                    "We need to hurry and",
                                                    "do this before High",
                                                    "Priest Vildt realizes",
                                                    "what we are doing."
                                                ],
                                            )?;
                                            ctx.call(Function::DelItem, vec![Val::from(7343), Val::from(1)])?;
                                            ctx.var("aru_em").set(Val::from(20))?;
                                            ctx.call(Function::ChangeQuest, vec![Val::from(2139), Val::from(2140)])?;
                                            ctx.close_window()?;
                                        } else {
                                            ctx.lines(args!["^3131FFYou seem to have", "misplaced Ishmael's file.^000000"])?;
                                            ctx.close_window()?;
                                        }
                                    } else {
                                        if (ctx.var("aru_em").get()? == 20 || ctx.var("aru_em").get()? == 21) {
                                            ctx.lines_as("High Priestess Niren", args!["Please hurry back", "to High Priest Zhed."])?;
                                            ctx.close_window()?;
                                        } else if ctx.var("aru_em").get()? == 22 {
                                            ctx.lines_as(
                                                "High Priestess Niren",
                                                args![
                                                    "I'm so proud of our pope!",
                                                    "Did you hear her wonderful",
                                                    "speech? Well, I should say",
                                                    "those are the words of Freya,",
                                                    "but... Well, you know. Hohoho!"
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "High Priestess Niren",
                                                args![
                                                    "Our priests may have acted",
                                                    "selfishly, but they are still",
                                                    "committed to Arunafeltz's",
                                                    "welfare. Things should",
                                                    "change for the better now."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "High Priestess Niren",
                                                args![
                                                    "I'm ashamed that I strayed",
                                                    "from the path of peace and",
                                                    "almost endangered my country.",
                                                    "I'm glad Zhed brought me back",
                                                    "my senses. From now on, I'll",
                                                    "repent for my sins with Zhed."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "High Priestess Niren",
                                                args![
                                                    "I hope the goddess can",
                                                    "forgive me. Ah, and you",
                                                    "should know that Zhed is",
                                                    "waiting for you. Why don't",
                                                    "you go see him now?"
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines(args![
                                                "^3355FFNiren softly smiles",
                                                "at you, and you realize",
                                                "that it's the first time",
                                                "that you've seen that",
                                                "expression on her face.^000000"
                                            ])?;
                                            ctx.var("aru_em").set(Val::from(23))?;
                                            ctx.call(Function::GetExperience, vec![Val::from(1000000), Val::from(0)])?;
                                            ctx.call(Function::ChangeQuest, vec![Val::from(2141), Val::from(2142)])?;
                                            ctx.close_window()?;
                                        } else if ctx.var("aru_em").get()?.number()? > 21 {
                                            ctx.lines_as(
                                                "High Priestess Niren",
                                                args!["Thanks to you and", "Bekento, I think I've", "finally found my way."],
                                            )?;
                                            ctx.close_window()?;
                                        } else {
                                            ctx.lines_as(
                                                "High Priestess Niren",
                                                args![
                                                    "I'm sorry...",
                                                    "I really don't want",
                                                    "to think about Zhed",
                                                    "right now. He's just..."
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    ctx.call(Function::Cutin, vec![Val::from("ra_gwoman"), Val::from(255)])?;
    return Err(Stop::End);
}

pub fn niren_ss(ctx: &Ctx) -> Script {
    niren_ss_body(ctx, Vec::new()).map(|_| ())
}

fn sippie_ss_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Sippie",
        args![
            "Now I finally have some",
            "time to take a break.",
            "Too many followers are",
            "crowding High Priestess",
            "Niren, and they're giving",
            "me so much work to do!"
        ],
    )?;
    if ctx.var("aru_em").get()?.number()? < 11 {
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("aru_em").get()? == 11 {
        ctx.next()?;
        ctx.lines_as(
            "Sippie",
            args![
                "Excuse me, but are you",
                "a friend of our pope?",
                "I think I saw you talk",
                "to her a while ago..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sippie",
            args![
                "I don't understand why",
                "High Priestess Niren wants",
                "to go to war. I actually think",
                "that in her heart, she doesn't",
                "want it. She's sweet and kind",
                "once you get to know her."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sippie",
            args![
                "She loves children...",
                "She's like a mother to",
                "all the kids in Arunafeltz.",
                "She even brought our pope to",
                "Rachel when she was a baby,",
                "raising her like her own child."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sippie",
            args![
                "If you didn't know she",
                "was our pope, it'd be easy",
                "to mistake her for High",
                "Priestess Niren's daughter."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn sippie_ss(ctx: &Ctx) -> Script {
    sippie_ss_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ForeignMerchantAru1Step {
    Start,
    OnTouch,
}

fn foreign_merchant_aru1_run(ctx: &Ctx, mut step: ForeignMerchantAru1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ForeignMerchantAru1Step::Start => {
                if ctx.var("aru_vol").get()? == 27 {
                    if ctx.var("aru_em").get()? == 0 {
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
                        ctx.lines_as(
                            "Foreign Merchant",
                            args![
                                "I never thought the desert",
                                "would be this hot! They built",
                                "a town here?! How can they call",
                                "this place inhabitable! What's",
                                "more, the crowds make things",
                                "around here so much hotter!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
                        ctx.lines_as(
                            "Foreign Merchant",
                            args![
                                "I heard that I could make",
                                "a killing in Morocc, which is",
                                "why I moved my business here.",
                                "But it looks like the weather",
                                "is going to kill me first!"
                            ],
                        )?;
                        ctx.next()?;
                        let choice = runtime::select_values(ctx, &[Val::from("How did you hear of Morocc?")])?;
                        ctx.var("@menu").set(choice)?;
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
                        ctx.lines_as(
                            "Foreign Merchant",
                            args![
                                "I... I just told you.",
                                "I heard that Morocc is",
                                "full customers willing to",
                                "buy things at premium prices.",
                                "Ugh, but it's a burning hell!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_SWEAT")?])?;
                        ctx.lines_as(
                            "Foreign Merchant",
                            args![
                                "I don't even care about making",
                                "money anymore! I just want to",
                                "be somewhere cool, drinking",
                                "a nice, frosty drink! You know,",
                                "I might just leave this town",
                                "tomorrow. Forget this desert!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                        ctx.call(
                            Function::Emotion,
                            vec![
                                ctx.constant("ET_SWEAT")?,
                                Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                            ],
                        )?;
                        ctx.lines_as(
                            "Foreign Merchant",
                            args![
                                "Ah, that's right. Don't they",
                                "sell some sweet, heavenly",
                                "drink around here? You know,",
                                "near the pond in the middle",
                                "of Morocc? Before I leave,",
                                "I want to taste that drink..."
                            ],
                        )?;
                        ctx.var("aru_em").set(Val::from(1))?;
                        ctx.call(Function::SetQuest, vec![Val::from(2129)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if (ctx.var("aru_em").get()?.number()? > 0 && ctx.var("aru_em").get()?.number()? < 7) {
                        ctx.lines_as(
                            "Foreign Merchant",
                            args![
                                "Say, isn't there a wandering",
                                "drink merchant near the pond",
                                "in the middle of Morocc. Oh,",
                                "what I would do to have that",
                                "delicious drink he's selling..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Foreign Merchant",
                            args![
                                "Oh, the heat's getting to me...",
                                "And I'm running out of sweat",
                                "to sweat... I think I'm going",
                                "to faint soon! I need a drink!"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("aru_em").get()? == 7 {
                        ctx.lines_as("Foreign Merchant", args!["*Pant Pant*", "*Sweat*"])?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Give Fruit Wine:Quietly Watch Him")])? {
                            1 => {
                                ctx.lines_as(
                                    "Foreign Merchant",
                                    args![
                                        "Oh, thank you! This",
                                        "smells so heavenly...",
                                        "Is this that famous drink",
                                        "that I keep hearing about?"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines(args![
                                    "^3355FF*Gulp Gulp Gulp Gulp*^000000",
                                    "^3355FF*Gulp Gulp Gulp Gulp*^000000",
                                    "^3355FF*Gulp Gulp Gulp Gulp*^000000",
                                    "^3355FF*Gulp Gulp Gulp Gulp*^000000",
                                    "^3355FF*Gulp Gulp Gulp Gulp*^000000"
                                ])?;
                                ctx.next()?;
                                ctx.call(Function::Emotion, vec![ctx.constant("ET_THROB")?])?;
                                ctx.lines_as(
                                    "Foreign Merchant",
                                    args![
                                        "Ahhh! That hit the spot!",
                                        "It's so refreshing and so",
                                        "exotic, like... Like... Those",
                                        "dancers in Comodo! Ahhhh!",
                                        "Thank you so much, friend!"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Foreign Merchant",
                                    args![
                                        "There must be some",
                                        "way I can repay you.",
                                        "Ah, here we go. Take it.",
                                        ((Val::from("Isn't your name, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                            + Val::from("?"))
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args!["What th--?", "How did you know?"],
                                )?;
                                ctx.next()?;
                                ctx.lines(args![
                                    "^3355FFThe merchant giggled,",
                                    "note from under his sleeve.",
                                    "You open the note, and",
                                    "read its message.^000000"
                                ])?;
                                ctx.next()?;
                                ctx.lines(args![
                                    ((Val::from("^666666") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(".")),
                                    " ",
                                    "The time for us to act",
                                    "has come once again.",
                                    "Please come see me",
                                    "once you get the chance.",
                                    " ",
                                    " - Bekento^000000"
                                ])?;
                                ctx.next()?;
                                ctx.call(
                                    Function::Emotion,
                                    vec![
                                        ctx.constant("ET_HUK")?,
                                        Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                                    ],
                                )?;
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args!["So... This means...", "You're not really a", "merchant, are you?"],
                                )?;
                                ctx.next()?;
                                ctx.call(Function::Emotion, vec![ctx.constant("ET_MERONG")?])?;
                                ctx.lines(args![
                                    "^3355FFThe merchant quietly",
                                    "looks at you with a",
                                    "mischievous grin.^000000"
                                ])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args!["So you asked me to get", "you a drink as an excuse", "just to talk to you?!"],
                                )?;
                                ctx.call(
                                    Function::Emotion,
                                    vec![
                                        ctx.constant("ET_CRY")?,
                                        Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args![
                                        "Oh well, thanks for",
                                        "bringing me the note.",
                                        "But I'm not buying you",
                                        "any more drinks, got it?"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Foreign Merchant",
                                    args!["Heh, understood.", "Now why don't you", "speak to him now?", "Good luck!"],
                                )?;
                                ctx.var("aru_em").set(Val::from(8))?;
                                ctx.call(Function::ChangeQuest, vec![Val::from(2130), Val::from(2131)])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    "Foreign Merchant",
                                    args!["Wha-- Do you just", "enjoy watching me", "slowly sweat to death?"],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    } else {
                        ctx.lines_as(
                            "Foreign Merchant",
                            args![
                                "Well, I'm already here.",
                                "I guess I may as well go",
                                "sightseeing around Morocc.",
                                "*Pant Pant* But it's so hot!"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                } else {
                    ctx.lines_as(
                        "Foreign Merchant",
                        args![
                            "I never thought the desert",
                            "would be this hot! They built",
                            "a town here?! How can they call",
                            "this place inhabitable! What's",
                            "more, the crowds make things",
                            "around here so much hotter!"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                step = ForeignMerchantAru1Step::OnTouch;
                continue 'machine;
            }
            ForeignMerchantAru1Step::OnTouch => {
                if ctx.var("aru_vol").get()? == 27 && ctx.var("aru_em").get()? == 0 {
                    ctx.lines_as(
                        "Foreign Merchant",
                        args!["*Pant Pant*", "Man, I think I'm going", "to die from all this heat!"],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn foreign_merchant_aru1(ctx: &Ctx) -> Script {
    foreign_merchant_aru1_run(ctx, ForeignMerchantAru1Step::Start, Vec::new()).map(|_| ())
}

pub fn foreign_merchant_aru1_ontouch(ctx: &Ctx) -> Script {
    foreign_merchant_aru1_run(ctx, ForeignMerchantAru1Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Foreignmerchant1Step {
    Start,
    OnTouch,
}

fn foreignmerchant1_run(ctx: &Ctx, mut step: Foreignmerchant1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Foreignmerchant1Step::Start => {
                step = Foreignmerchant1Step::OnTouch;
                continue 'machine;
            }
            Foreignmerchant1Step::OnTouch => {
                if ctx.var("aru_em").get()? == 4 {
                    ctx.var("aru_em").set(Val::from(5))?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn foreignmerchant1(ctx: &Ctx) -> Script {
    foreignmerchant1_run(ctx, Foreignmerchant1Step::Start, Vec::new()).map(|_| ())
}

pub fn foreignmerchant1_ontouch(ctx: &Ctx) -> Script {
    foreignmerchant1_run(ctx, Foreignmerchant1Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Foreignmerchant2Step {
    Start,
    OnTouch,
}

fn foreignmerchant2_run(ctx: &Ctx, mut step: Foreignmerchant2Step, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_nawara = Val::from(0);
    'machine: loop {
        match step {
            Foreignmerchant2Step::Start => {
                step = Foreignmerchant2Step::OnTouch;
                continue 'machine;
            }
            Foreignmerchant2Step::OnTouch => {
                if ctx.var("aru_em").get()? == 1 {
                    ctx.var("aru_em").set(Val::from(2))?;
                } else if ctx.var("aru_em").get()? == 5 {
                    l_nawara = ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?;
                    if l_nawara.clone().number()? < 5 {
                        ctx.call(Function::EnableNpc, vec![Val::from("Foreign Merchant#aru2")])?;
                    } else {
                        ctx.var("aru_em").set(Val::from(2))?;
                    }
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn foreignmerchant2(ctx: &Ctx) -> Script {
    foreignmerchant2_run(ctx, Foreignmerchant2Step::Start, Vec::new()).map(|_| ())
}

pub fn foreignmerchant2_ontouch(ctx: &Ctx) -> Script {
    foreignmerchant2_run(ctx, Foreignmerchant2Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Foreignmerchant3Step {
    Start,
    OnTouch,
}

fn foreignmerchant3_run(ctx: &Ctx, mut step: Foreignmerchant3Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Foreignmerchant3Step::Start => {
                step = Foreignmerchant3Step::OnTouch;
                continue 'machine;
            }
            Foreignmerchant3Step::OnTouch => {
                if ctx.var("aru_em").get()? == 2 {
                    ctx.var("aru_em").set(Val::from(3))?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn foreignmerchant3(ctx: &Ctx) -> Script {
    foreignmerchant3_run(ctx, Foreignmerchant3Step::Start, Vec::new()).map(|_| ())
}

pub fn foreignmerchant3_ontouch(ctx: &Ctx) -> Script {
    foreignmerchant3_run(ctx, Foreignmerchant3Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Foreignmerchant4Step {
    Start,
    OnTouch,
}

fn foreignmerchant4_run(ctx: &Ctx, mut step: Foreignmerchant4Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Foreignmerchant4Step::Start => {
                step = Foreignmerchant4Step::OnTouch;
                continue 'machine;
            }
            Foreignmerchant4Step::OnTouch => {
                if ctx.var("aru_em").get()? == 3 {
                    ctx.var("aru_em").set(Val::from(4))?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn foreignmerchant4(ctx: &Ctx) -> Script {
    foreignmerchant4_run(ctx, Foreignmerchant4Step::Start, Vec::new()).map(|_| ())
}

pub fn foreignmerchant4_ontouch(ctx: &Ctx) -> Script {
    foreignmerchant4_run(ctx, Foreignmerchant4Step::OnTouch, Vec::new()).map(|_| ())
}

fn foreign_merchant_aru2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("aru_em").get()?.number()? > 1 && ctx.var("aru_em").get()?.number()? < 7) {
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Excuse me, but I heard",
                "that I could buy some",
                "good drinks around here.",
                "Would you happen to be",
                "the one selling them?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Foreign Merchant",
            args![
                "Oh, you mean my fruit wine?",
                "I'm afraid it's not as good",
                "as people chalk it up to be.",
                "It's just a little something",
                "I brew at home, and share",
                "with my friends after dinner."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Well, I ran into this",
                "merchant that really wants",
                "to have a taste of that wine.",
                "Do you think that you'd be",
                "able to sell me some?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Foreign Merchant", args!["Oh, I'm sorry, but I'm", "all sold out for today."])?;
        ctx.next()?;
        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Oh, no..."])?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_OTL")?,
                Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Foreign Merchant",
            args![
                "...............................",
                "...............................",
                "..............................."
            ],
        )?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_PROFUSELY_SWEAT")?])?;
        ctx.next()?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_HUK")?,
                Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
            ],
        )?;
        ctx.lines_as(
            "Foreign Merchant",
            args![
                "You know, I save a small",
                "bottle of wine for myself",
                "so I'll just give that to you.",
                "I can always brew more, and",
                "I'm flattered that your friend",
                "is so interested in my wine."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Really? Thank you!", "I'm sure that he's really", "appreciate your generosity."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Foreign Merchant",
            args![
                "Oh, don't mention it.",
                "Well, I should head back",
                "home now. I'll be sure to",
                "keep an extra bottle in case",
                "you want more next time. Take",
                "care, and I'll see you later!"
            ],
        )?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_FLAG")?])?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFYou received a bottle",
            "of the famous fruit wine",
            "from the old man. That",
            "guy's pretty nice!^000000"
        ])?;
        ctx.var("aru_em").set(Val::from(7))?;
        ctx.call(Function::DisableNpc, vec![Val::from("Foreign Merchant#aru2")])?;
        ctx.call(Function::ChangeQuest, vec![Val::from(2129), Val::from(2130)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Foreign Merchant",
            args![
                "I'm sorry, but I'm all",
                "sold out of fruit wine.",
                "I hope that's not too",
                "much of an inconvenience."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn foreign_merchant_aru2(ctx: &Ctx) -> Script {
    foreign_merchant_aru2_body(ctx, Vec::new()).map(|_| ())
}

fn foreign_merchant_aru2_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Foreign Merchant#aru2")])?;
    return Err(Stop::End);
}

pub fn foreign_merchant_aru2_oninit(ctx: &Ctx) -> Script {
    foreign_merchant_aru2_oninit_body(ctx, Vec::new()).map(|_| ())
}

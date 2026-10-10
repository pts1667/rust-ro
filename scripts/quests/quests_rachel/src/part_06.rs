use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum PannoRachelStep {
    Start,
    LLottery,
    LEnd,
}

fn panno_rachel_run(ctx: &Ctx, mut step: PannoRachelStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_bonus_donate = Val::from(0);
    let mut l_bonus_donate2 = Val::from(0);
    'machine: loop {
        match step {
            PannoRachelStep::Start => {
                ctx.call(Function::Cutin, vec![Val::from("ra_fano03"), Val::from(2)])?;
                if (ctx.var("ra_tem_q").get()?.number()? >= 12
                    || runtime::op(&ctx.var("misc_quest").get()?, "&", &Val::from(8192))?.is_true())
                {
                    ctx.call(Function::Cutin, vec![Val::from("ra_fano03"), Val::from(2)])?;
                    ctx.lines_as(
                        "Priestess Panno",
                        args!["Good day.", "Have you come to", "redeem Lottery Tickets?"],
                    )?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("Yes:No")])?) == 1 {
                        step = PannoRachelStep::LLottery;
                        continue 'machine;
                    }
                    if ctx.var("ra_tem_q").get()? == 12 {
                        ctx.call(Function::Cutin, vec![Val::from("ra_fano01"), Val::from(2)])?;
                        ctx.lines_as(
                            "Priestess Panno",
                            args![
                                "I already told you",
                                "everything I know.",
                                "Remember to keep",
                                "quiet about what I said."
                            ],
                        )?;
                        ctx.close_window()?;
                        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                        return Err(Stop::End);
                    }
                } else if ctx.var("ra_tem_q").get()?.number()? < 2 {
                    ctx.lines_as(
                        "Priestess Panno",
                        args!["Greetings.", "May Freya fill", "your days with joy.", "Laughter. And prosperity."],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(
                        ctx,
                        &[Val::from("Redeem Lottery Tickets:Temple Information:Hey, what's happening?")],
                    )? {
                        1 => {
                            ctx.call(Function::Cutin, vec![Val::from("ra_fano03"), Val::from(2)])?;
                            ctx.lines_as(
                                "Priestess Panno",
                                args![
                                    "Fine. You really",
                                    "want to exchange your",
                                    "Lottery Tickets? It's my job",
                                    "to ask and make sure, you",
                                    "know, in case you were saving",
                                    "them for some weird reason."
                                ],
                            )?;
                            ctx.next()?;
                            if Val::from(runtime::select_values(ctx, &[Val::from("Yes:No")])?) == 1 {
                                step = PannoRachelStep::LLottery;
                                continue 'machine;
                            }
                            ctx.call(Function::Cutin, vec![Val::from("ra_fano03"), Val::from(2)])?;
                            ctx.lines_as(
                                "Priestess Panno",
                                args![
                                    "You can redeem your",
                                    "Lottery Tickets at any",
                                    "time, so please visit",
                                    "me at your leisure.",
                                    "Go with Freya."
                                ],
                            )?;
                            ctx.close_window()?;
                            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.call(Function::Cutin, vec![Val::from("ra_fano01"), Val::from(2)])?;
                            ctx.lines_as(
                                "Priestess Panno",
                                args![
                                    "The Chapel is located",
                                    "over the wall behind me.",
                                    "Our pope's office and chambers",
                                    "are upstairs. You can only go",
                                    "there if you have special",
                                    "authorization."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["......", "........."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Priestess Panno", args!["............", ".........", "......"])?;
                            ctx.next()?;
                            ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["...?"])?;
                            ctx.next()?;
                            ctx.call(Function::Cutin, vec![Val::from("ra_fano02"), Val::from(2)])?;
                            ctx.lines_as(
                                "Priestess Panno",
                                args![
                                    "You come to a temple",
                                    "to pray, not to look",
                                    "around. Or chit-chat",
                                    "with the priestesses",
                                    "like me. Don't forget it."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.call(Function::Cutin, vec![Val::from("ra_fano03"), Val::from(2)])?;
                            ctx.lines_as(
                                "Priestess Panno",
                                args![
                                    "The offices for the High",
                                    "Priests are located on each",
                                    "side of the hallway. However,",
                                    "you can only enter if you've",
                                    "been permitted beforehand."
                                ],
                            )?;
                            ctx.close_window()?;
                            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                            return Err(Stop::End);
                        }
                        3 => {
                            ctx.lines_as(
                                "Priestess Panno",
                                args!["......................", "No chit-chat", "inside the temple."],
                            )?;
                            ctx.close_window()?;
                            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else if ctx.var("ra_tem_q").get()? == 10 {
                    ctx.lines_as("Priestess Panno", args!["Good day."])?;
                    ctx.next()?;
                    match runtime::select_values(
                        ctx,
                        &[Val::from("Redeem Lottery Tickets:Temple Information:Hey, did you open the gate?")],
                    )? {
                        1 => {
                            ctx.call(Function::Cutin, vec![Val::from("ra_fano01"), Val::from(2)])?;
                            ctx.lines_as(
                                "Priestess Panno",
                                args![
                                    "You... still have",
                                    "Lottery Tickets from",
                                    "making donations for",
                                    "the festival? Look up the",
                                    "word ''punctuality'' in the",
                                    "dictionary. It might help you."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Priestess Panno",
                                args![
                                    "Fine. You really",
                                    "want to exchange your",
                                    "Lottery Tickets? It's my job",
                                    "to ask and make sure, you",
                                    "know, in case you were saving",
                                    "them for some weird reason."
                                ],
                            )?;
                            ctx.next()?;
                            if Val::from(runtime::select_values(ctx, &[Val::from("Yes:No")])?) == 1 {
                                step = PannoRachelStep::LLottery;
                                continue 'machine;
                            }
                            ctx.lines_as(
                                "Priestess Panno",
                                args![
                                    "You can redeem your",
                                    "Lottery Tickets at any",
                                    "time, so please visit",
                                    "me at your leisure.",
                                    "Go with Freya."
                                ],
                            )?;
                            ctx.close_window()?;
                            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.call(Function::Cutin, vec![Val::from("ra_fano01"), Val::from(2)])?;
                            ctx.lines_as(
                                "Priestess Panno",
                                args![
                                    "The Chapel is located",
                                    "over the wall behind me",
                                    "Our pope's office and chambers",
                                    "are upstairs. You can only go",
                                    "there if you have special",
                                    "authorization."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["......", "........."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Priestess Panno", args!["............", ".........", "......"])?;
                            ctx.next()?;
                            ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["...?"])?;
                            ctx.next()?;
                            ctx.call(Function::Cutin, vec![Val::from("ra_fano02"), Val::from(2)])?;
                            ctx.lines_as(
                                "Priestess Panno",
                                args![
                                    "You come to a temple",
                                    "to pray, not to look",
                                    "around. Or chit-chat",
                                    "with the priestesses",
                                    "like me. Don't forget it."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.call(Function::Cutin, vec![Val::from("ra_fano02"), Val::from(2)])?;
                            ctx.lines_as(
                                "Priestess Panno",
                                args![
                                    "The offices for the High",
                                    "Priests are located on each",
                                    "side of the hallway. However,",
                                    "you can only enter if you've",
                                    "been permitted beforehand."
                                ],
                            )?;
                            ctx.close_window()?;
                            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                            return Err(Stop::End);
                        }
                        3 => {
                            ctx.call(Function::Cutin, vec![Val::from("ra_fano02"), Val::from(2)])?;
                            ctx.lines_as("Priestess Panno", args!["Gate...?"])?;
                            ctx.next()?;
                            let choice = runtime::select_values(ctx, &[Val::from("The locked gate to the temple!")])?;
                            ctx.var("@menu").set(choice)?;
                            ctx.lines_as("Priestess Panno", args!["I don't know", "what you mean."])?;
                            ctx.next()?;
                            let choice = runtime::select_values(ctx, &[Val::from("Explain")])?;
                            ctx.var("@menu").set(choice)?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "I think that Priestess Nemma",
                                    "has been worried about the",
                                    "gate being broken, though",
                                    "it hasn't been like that since",
                                    "you've started your position",
                                    "here at the temple."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Priestess Panno",
                                args![
                                    "Worried...?",
                                    "Yes, my twin sister",
                                    "has been known to",
                                    "do a lot of that, and cry",
                                    "for my help. Am I right?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "It's weird",
                                    "...She said that the gate",
                                    "should automatically fix",
                                    "itself, but it hasn't been",
                                    "doing it lately. She was",
                                    "acting really funny.."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "She was talking about",
                                    "the temple's history and",
                                    "how the security system was",
                                    "built. Then she mentioned you,",
                                    "so I get the feeling you might",
                                    "know something about this."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Priestess Panno", args!["........."])?;
                            ctx.var("ra_tem_q").set(Val::from(11))?;
                            ctx.close_window()?;
                            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else if ctx.var("ra_tem_q").get()? == 11 {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "I remember now...!",
                            "I was there when the gate",
                            "closed, and I heard someone",
                            "with your exact voice whisper",
                            "something to magically open",
                            "the gate! What's that all about?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Priestess Panno", args!["......", ".........", "............"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Priestess Panno",
                        args!["......", ".........", "............", "We... I mean...."],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("ra_fano01"), Val::from(2)])?;
                    ctx.lines_as(
                        "Priestess Panno",
                        args![
                            "Nemma and I are descended",
                            "from a family of great alchemists.",
                            "They went on a long, religious",
                            "pilgrimage and ended up here.",
                            "They are the ones that built",
                            "this temple and security system."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Priestess Panno",
                        args![
                            "In a sense, this temple",
                            "is the tomb of our ancestors,",
                            "who left Rune-Midgarts and",
                            "built this city of Rachel about",
                            "a thousand years ago."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Priestess Panno",
                        args![
                            "Over the generations, many",
                            "Alchemists left Rune-Midgarts",
                            "and settled here, converting",
                            "the desert into green fields.",
                            "This temple was supposed",
                            "be a shelter from intruders."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Priestess Panno",
                        args![
                            "My ancestor designed the",
                            "temple security system to",
                            "require a special permit in",
                            "order to access the temple",
                            "But the invaders never came,",
                            "and we didn't need the permits."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Priestess Panno",
                        args![
                            "The special permits were",
                            "disposed, but the automatic",
                            "security system was left on",
                            "the gate, just in case. Now,",
                            "I've heard that the security",
                            "system has been malfunctioning."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("ra_fano03"), Val::from(2)])?;
                    ctx.lines_as(
                        "Priestess Panno",
                        args![
                            "However, that's not possible.",
                            "Even by today's standards, the",
                            "system is perfectly designed.",
                            "While I was praying in the",
                            "Chapel some time ago, I found",
                            "what is causing the gate problems."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Priestess Panno",
                        args![
                            "Suddenly, the lights went",
                            "out, and I could only barely",
                            "see distinguish my surroundings.",
                            "I was about to go outside when",
                            "I heard a noise from the stairs."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("ra_fano02"), Val::from(2)])?;
                    ctx.lines_as(
                        "Priestess Panno",
                        args![
                            "From what I can tell,",
                            "someone is transporting",
                            "something out of the Holy",
                            "Grounds. I don't know what",
                            "they keep there, though.",
                            "I'm not a high rank Priestess."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Priestess Panno",
                        args![
                            "I'm not sure if I want",
                            "to find out what's there",
                            "I heard something that",
                            "sounded like a horrible",
                            "monster's howl. It was",
                            "incredibly frightening."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Priestess Panno",
                        args![
                            "I panicked, ran to",
                            "my desk at the front,",
                            "and just recited the secret",
                            "password passed down from my",
                            "ancestors to open the gate."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("ra_fano01"), Val::from(2)])?;
                    ctx.lines_as(
                        "Priestess Panno",
                        args![
                            "You see, my ancestors",
                            "included the secret password",
                            "and voice recognition feature",
                            "into the gate in case something",
                            "after they disposed of the",
                            "temple permits--just in case."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Priestess Panno",
                        args![
                            "The real reason why the gate",
                            "is acting strangely is not",
                            "because the gate is broken,",
                            "but because someone is sneaking",
                            "in and out of the Holy Ground."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Priestess Panno",
                        args![
                            "Whoever they are, they",
                            "must be using one of the",
                            "old temple permits in order",
                            "to cover their tracks. Now,",
                            "you know everything."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("ra_fano01"), Val::from(2)])?;
                    ctx.lines_as(
                        "Priestess Panno",
                        args![
                            "You'd better not",
                            "reveal what you learned",
                            "from me. Otherwise, I'm",
                            "sure we'll end up dead.",
                            "Excuse me now, I need",
                            "to get back to work."
                        ],
                    )?;
                    ctx.var("ra_tem_q").set(Val::from(12))?;
                    ctx.close_window()?;
                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                    return Err(Stop::End);
                }
                ctx.call(Function::Cutin, vec![Val::from("ra_fano02"), Val::from(2)])?;
                ctx.lines_as(
                    "Priestess Panno",
                    args![
                        "This is a holy place",
                        "Behave yourself, and",
                        "respect those who have",
                        "come here just to worship."
                    ],
                )?;
                ctx.close_window()?;
                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                return Err(Stop::End);
            }
            PannoRachelStep::LLottery => {
                if ctx.call(Function::CheckWeight, vec![Val::from(607), Val::from(1)])? == 0 {
                    ctx.call(Function::Cutin, vec![Val::from("ra_fano02"), Val::from(2)])?;
                    ctx.lines_as(
                        "Priestess Panno",
                        args![
                            "You're carrying too much",
                            "right now. What are you",
                            "going to do if I give you",
                            "something large, unwieldy",
                            "and heavy? Put your junk",
                            "away in Storage first."
                        ],
                    )?;
                    ctx.close_window()?;
                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Priestess Panno",
                    args![
                        "Here we go",
                        "Excited? Mm",
                        "I can redeem only",
                        "1 Lottery Ticket at",
                        "a time. Your reward",
                        "for this ticket is..."
                    ],
                )?;
                ctx.next()?;
                ctx.call(Function::Cutin, vec![Val::from("ra_fano02"), Val::from(2)])?;
                if ctx.call(Function::CountItem, vec![Val::from(7570)])?.number()? > 0 {
                    ctx.lines_as(
                        "Priestess Panno",
                        args![
                            "^FF0000This^000000. It's been in our",
                            "storage for a while, but",
                            "I hope this is acceptable",
                            "as a token of the temple's",
                            "gratitude. May Freya smile",
                            "upon you for your generosity..."
                        ],
                    )?;
                    ctx.call(Function::DelItem, vec![Val::from(7570), Val::from(1)])?;
                    l_bonus_donate = ctx.call(Function::Rand, vec![Val::from(100)])?;
                    if l_bonus_donate.clone() == 99 {
                        l_bonus_donate2 = ctx.call(Function::Rand, vec![Val::from(100)])?;
                        if (l_bonus_donate2.clone().number()? > 0 && l_bonus_donate2.clone().number()? < 11) {
                            ctx.call(Function::GetItem, vec![Val::from(616), Val::from(1)])?;
                        } else if (l_bonus_donate2.clone().number()? > 10 && l_bonus_donate2.clone().number()? < 31) {
                            ctx.call(Function::GetItem, vec![Val::from(617), Val::from(1)])?;
                        } else if (l_bonus_donate2.clone().number()? > 30 && l_bonus_donate2.clone().number()? < 61) {
                            ctx.call(Function::GetItem, vec![Val::from(603), Val::from(1)])?;
                        } else {
                            ctx.call(Function::GetItem, vec![Val::from(607), Val::from(1)])?;
                        }
                    } else {
                        if (l_bonus_donate.clone().number()? > 88 && l_bonus_donate.clone().number()? < 96) {
                            ctx.call(Function::GetItem, vec![Val::from(644), Val::from(1)])?;
                        } else {
                            if (l_bonus_donate.clone().number()? > 76 && l_bonus_donate.clone().number()? < 89) {
                                ctx.call(Function::GetItem, vec![Val::from(607), Val::from(1)])?;
                            } else {
                                if (l_bonus_donate.clone().number()? > 65 && l_bonus_donate.clone().number()? < 77) {
                                    ctx.call(Function::GetItem, vec![Val::from(505), Val::from(1)])?;
                                } else if (l_bonus_donate.clone().number()? > 57 && l_bonus_donate.clone().number()? < 66) {
                                    ctx.call(Function::GetItem, vec![Val::from(604), Val::from(1)])?;
                                } else if (l_bonus_donate.clone().number()? > 45 && l_bonus_donate.clone().number()? < 58) {
                                    ctx.call(Function::GetItem, vec![Val::from(608), Val::from(1)])?;
                                } else if (l_bonus_donate.clone().number()? > 5 && l_bonus_donate.clone().number()? < 11) {
                                    ctx.call(Function::GetItem, vec![Val::from(518), Val::from(1)])?;
                                } else if (l_bonus_donate.clone().number()? > 0 && l_bonus_donate.clone().number()? < 6) {
                                    ctx.call(Function::GetItem, vec![Val::from(526), Val::from(1)])?;
                                } else {
                                    ctx.call(Function::GetItem, vec![Val::from(547), Val::from(1)])?;
                                }
                            }
                        }
                    }
                    ctx.close_window()?;
                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Priestess Panno",
                    args!["You...", "Don't have any Lottery", "Tickets to redeem."],
                )?;
                if ctx.var("$rachel_donate").get()?.number()? > 9999 {
                    ctx.lines(args![
                        "We're not distributing",
                        "them now, but maybe you",
                        "can ask your friends for one."
                    ])?;
                } else {
                    ctx.lines(args![
                        "You can obtain them from",
                        "Priestess Nemma at the temple",
                        "entrance after you donate zeny."
                    ])?;
                }
                ctx.next()?;
                ctx.call(Function::Cutin, vec![Val::from("ra_fano03"), Val::from(2)])?;
                ctx.lines_as("Priestess Panno", args!["May Freya be with you."])?;
                ctx.close_window()?;
                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                return Err(Stop::End);
            }
            PannoRachelStep::LEnd => {
                ctx.close_window()?;
                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn panno_rachel(ctx: &Ctx) -> Script {
    panno_rachel_run(ctx, PannoRachelStep::Start, Vec::new()).map(|_| ())
}

fn pope_s_office_guard_rac_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("ra_tem_q").get()? == 15 && ctx.call(Function::CountItem, vec![Val::from(7561)])?.number()? > 39) {
        ctx.lines_as(
            "Pope's Office Guard",
            args![
                "Hm? What's that you have",
                "there? Oh, you've gathered",
                "40 Glacial Hearts and brought",
                "a recommendation letter from",
                "a High Priest? Most impressive."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Pope's Office Guard",
            args![
                "I think I know why you're",
                "here. Thanks for bringing",
                "those to me, and I'll let you",
                "enter and see the pope",
                ((Val::from("Welcome, brave ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("."))
            ],
        )?;
        ctx.close_window()?;
        ctx.var("ra_tem_q").set(Val::from(16))?;
        ctx.call(Function::DelItem, vec![Val::from(7561), Val::from(40)])?;
        return Err(Stop::End);
    } else if ctx.var("ra_tem_q").get()? == 16 {
        ctx.lines_as(
            "Pope's Office Guard",
            args!["The pope is inside", "expecting you, so please", "don't keep her waiting long."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if runtime::op(&ctx.var("misc_quest").get()?, "&", &Val::from(8192))?.is_true() {
        if (ctx.var("aru_em").get()? == 15 || ctx.var("aru_em").get()? == 21) {
            ctx.mes("- Niren told me to pay a visit to the Pope in her office... -")?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Pope's Office Guard",
                args![
                    "I'm sorry, but outsiders",
                    "typically aren't allowed",
                    "to visit the pope without",
                    "special authorization."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        ctx.lines_as(
            "Pope's Office Guard",
            args![
                "I'm sorry, but the pope",
                "is officiating a service",
                "now. Please come back",
                "after the service is ended."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn pope_s_office_guard_rac(ctx: &Ctx) -> Script {
    pope_s_office_guard_rac_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Gyoin1RachelStep {
    Start,
    OnTouch,
}

fn gyoin1_rachel_run(ctx: &Ctx, mut step: Gyoin1RachelStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Gyoin1RachelStep::Start => {
                step = Gyoin1RachelStep::OnTouch;
                continue 'machine;
            }
            Gyoin1RachelStep::OnTouch => {
                if ctx.var("ra_tem_q").get()? == 16 {
                    ctx.call(Function::Warp, vec![Val::from("ra_temin"), Val::from(276), Val::from(239)])?;
                } else if (ctx.var("ra_tem_q").get()?.number()? > 16
                    || runtime::op(&ctx.var("misc_quest").get()?, "&", &Val::from(8192))?.is_true())
                {
                    if (ctx.var("aru_em").get()? == 15 || ctx.var("aru_em").get()? == 21) {
                        ctx.call(Function::Warp, vec![Val::from("ra_temin"), Val::from(276), Val::from(239)])?;
                    } else {
                        ctx.lines_as(
                            "Pope's Office Guard",
                            args![
                                "I'm sorry, but the pope",
                                "is officiating a service",
                                "now. Please come back",
                                "after the service is ended."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                if ctx.var("ra_tem_q").get()?.number()? < 16 {
                    ctx.mes("^3355FFThe door is locked.^000000")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn gyoin1_rachel(ctx: &Ctx) -> Script {
    gyoin1_rachel_run(ctx, Gyoin1RachelStep::Start, Vec::new()).map(|_| ())
}

pub fn gyoin1_rachel_ontouch(ctx: &Ctx) -> Script {
    gyoin1_rachel_run(ctx, Gyoin1RachelStep::OnTouch, Vec::new()).map(|_| ())
}

fn rachel33_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn rachel33(ctx: &Ctx) -> Script {
    rachel33_body(ctx, Vec::new()).map(|_| ())
}

fn rachel33_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("aru_em").get()? == 21 {
        ctx.call(Function::Warp, vec![Val::from("que_temsky"), Val::from(99), Val::from(11)])?;
    } else {
        ctx.call(Function::Warp, vec![Val::from("ra_temsky"), Val::from(99), Val::from(11)])?;
    }
    return Err(Stop::End);
}

pub fn rachel33_ontouch(ctx: &Ctx) -> Script {
    rachel33_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn priest_1rachel_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Priest", args!["May Freya be with you."])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn priest_1rachel(ctx: &Ctx) -> Script {
    priest_1rachel_body(ctx, Vec::new()).map(|_| ())
}

fn male_follower_1rachel_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Male Follower", args!["May Freya be with you."])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn male_follower_1rachel(ctx: &Ctx) -> Script {
    male_follower_1rachel_body(ctx, Vec::new()).map(|_| ())
}

fn priestess_1rachel_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Priestess", args!["May Freya be with you."])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn priestess_1rachel(ctx: &Ctx) -> Script {
    priestess_1rachel_body(ctx, Vec::new()).map(|_| ())
}

fn female_follower_1rachel_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Female Follower", args!["May Freya be with you."])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn female_follower_1rachel(ctx: &Ctx) -> Script {
    female_follower_1rachel_body(ctx, Vec::new()).map(|_| ())
}

fn pope_rachel_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input_s = Val::from("");
    if ctx.var("ra_tem_q").get()? == 16 {
        ctx.call(Function::Cutin, vec![Val::from("ra_bishop"), Val::from(2)])?;
        ctx.lines(args!["^3355FFTh-this little", "girl is the pope...?!^000000"])?;
        ctx.next()?;
        ctx.lines_as(
            "Pope",
            args![
                "...............................",
                "..........................Um...",
                "W-welcome. Is something",
                "wrong? Oh, I know, it's my",
                "eyes. I'm sorry if they",
                "scare you a little bit."
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("No, not at all...!")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as("Pope", args!["......", "......", "......You know, my appearance..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Pope",
            args![
                "It's alright. Actually,",
                "I'm used to it. People",
                "are usually a little shocked",
                "the first time they see me.",
                "So... Um... Where do you",
                ((Val::from("come from, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("?"))
            ],
        )?;
        ctx.next()?;
        let (input, status) = runtime::input_text(ctx, None, None)?;
        ctx.var("@input$").set(input)?;
        ctx.lines_as(
            "Pope",
            args![
                (ctx.var("@input$").get()? + Val::from("...?")),
                "I think I might have heard",
                "about that place before.",
                "Wow... What did you do",
                "when you lived there?"
            ],
        )?;
        ctx.next()?;
        let (input, status) = runtime::input_text(ctx, None, None)?;
        ctx.var("@input$").set(input)?;
        ctx.lines_as(
            "Pope",
            args![
                (ctx.var("@input$").get()? + Val::from("...?")),
                "Wow, that sounds quite",
                "extraordinary. Ooh, ooh!",
                "Tell me, how have you come",
                "here to Rachel? I'm interested",
                "in knowing more about you~"
            ],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFYou and the pope",
            "continued to comfortably",
            "converse. As she spoke, she",
            "seemed more like a young,",
            "lonely girl than a solemn",
            "religious figure for a nation.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Pope",
            args![
                "...Wow, sometimes, I really",
                "wish I could live like you.",
                "You'd think being pope would",
                "be great, but they make me work",
                "all the time. And there are all",
                "sorts of things I can't do."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Pope",
            args![
                "You know what's weird?",
                "I think I'm the head of this",
                "religion, but there's a sacred",
                "place that even I'm not allowed",
                "to visit. Isn't that so weird?"
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("R-really...?")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Pope",
            args![
                "...Yeah. I don't know",
                "much about, and I'm not",
                "really supposed to talk",
                "about it. We just call this",
                "place the ''Holy Ground...''"
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Holy Ground?")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Pope",
            args![
                "I'm sorry... I want",
                "to tell you more, but...",
                "That's all I know. I wonder...",
                "I wonder what they could",
                "be hiding over there..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Priest",
            args![
                "I'm sorry to interrupt",
                "your Excellency, but it's",
                "time to officiate services.",
                "Pardon me."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Pope",
            args![
                "Already?! Hmpf...",
                "Oh, before I forget,",
                "I needed to tell you that",
                "High Priest Zhed wanted",
                ((Val::from("to talk to you, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("."))
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Pope",
            args![
                "I really want to thank",
                "you for coming to speak",
                "to me. I had a great time",
                "learning. Now I have to",
                "go officiate services, so...",
                "This is goodbye for now..."
            ],
        )?;
        ctx.close_window()?;
        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
        ctx.var("ra_tem_q").set(Val::from(17))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(8100), Val::from(8101)])?;
        return Err(Stop::End);
    } else if ctx.var("aru_em").get()?.number()? < 15 {
        ctx.lines_as(
            "Pope",
            args!["Oh, that's right...", "Zhed... Er, High Priest", "Zhed wanted to speak to you!"],
        )?;
        ctx.close_window()?;
        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
        return Err(Stop::End);
    } else if ctx.var("aru_em").get()? == 15 {
        ctx.lines_as(
            "Pope",
            args![
                "Oh, hello! It's you again!",
                "Have you come to tell me",
                "interesting stories about",
                "the outside world again?"
            ],
        )?;
        ctx.next()?;
        let (input, status) = runtime::input_text(ctx, None, None)?;
        l_input_s = input;
        ctx.lines_as(
            "Pope",
            args!["Mm. That sounds pretty", "strange to me. Well, it's", "good to hear it from you."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "(^333333I can't just leave",
                "now. I should try to see",
                "if I can learn anything",
                "useful from the pope...^000000)"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Aren't you lonely?:Do you ever take a break?")])? {
            1 => {
                ctx.lines_as(
                    "Pope",
                    args![
                        "Well, there are always",
                        "some priests with me all",
                        "the time. Some of them",
                        "even watch me when I sleep.",
                        "I don't think I have time",
                        "to feel lonely, actually."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Pope",
                    args![
                        "Thank you for asking.",
                        "Sometimes... Sometimes",
                        "I wonder if I have a real",
                        "family. I don't even know",
                        "who'd they be or if they",
                        "exist. I don't know..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Pope",
                    args![
                        "I... I'm sorry.",
                        "I guess you didn't",
                        "expect to hear something",
                        "like that. ^666666*Sigh*^000000 I'm feeling",
                        "pretty exhausted. I better...",
                        "I better get some rest..."
                    ],
                )?;
                ctx.var("aru_em").set(Val::from(16))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(2135), Val::from(2136)])?;
                ctx.close_window()?;
                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Pope",
                    args![
                        "Well, even if I wanted",
                        "to, I can't just go out",
                        "and play. As pope... Well...",
                        "Everyone and everything",
                        "sort of needs my attention."
                    ],
                )?;
                ctx.close_window()?;
                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if ctx.var("aru_em").get()? == 16 {
        ctx.lines_as(
            "Follower",
            args![
                "The pope said that she's",
                "exhausted. Please come",
                "back when she's feeling",
                "better, and she will have",
                "an audience with you."
            ],
        )?;
    }
    ctx.close_window()?;
    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
    return Err(Stop::End);
}

pub fn pope_rachel(ctx: &Ctx) -> Script {
    pope_rachel_body(ctx, Vec::new()).map(|_| ())
}

fn high_priestess_niren_ra_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn high_priestess_niren_ra(ctx: &Ctx) -> Script {
    high_priestess_niren_ra_body(ctx, Vec::new()).map(|_| ())
}

fn high_priestess_niren_ra_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("High Priestess Niren#ra")])?;
    return Err(Stop::End);
}

pub fn high_priestess_niren_ra_oninit(ctx: &Ctx) -> Script {
    high_priestess_niren_ra_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn high_priestess_niren_ra_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::InitNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn high_priestess_niren_ra_onenable(ctx: &Ctx) -> Script {
    high_priestess_niren_ra_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn high_priestess_niren_ra_ontimer120000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::DisableNpc, vec![Val::from("High Priestess Niren#ra")])?;
    return Err(Stop::End);
}

pub fn high_priestess_niren_ra_ontimer120000(ctx: &Ctx) -> Script {
    high_priestess_niren_ra_ontimer120000_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Out1RachelStep {
    Start,
    OnTouch,
}

fn out1_rachel_run(ctx: &Ctx, mut step: Out1RachelStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Out1RachelStep::Start => {
                step = Out1RachelStep::OnTouch;
                continue 'machine;
            }
            Out1RachelStep::OnTouch => {
                if ctx.var("ra_tem_q").get()? == 18 {
                    ctx.lines(args![
                        "^3355FFIt sounds like they're",
                        "still talking. It'd be",
                        "best to patient and wait",
                        "just a little while longer.",
                        "Still... It's awfully boring.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFA little eavesdropping",
                        "never hurt anyone. Maybe",
                        "you can hear them a little",
                        "better if you listen from",
                        "the left side of the wall.^000000"
                    ])?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("ra_temin"), Val::from(300), Val::from(153)])?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn out1_rachel(ctx: &Ctx) -> Script {
    out1_rachel_run(ctx, Out1RachelStep::Start, Vec::new()).map(|_| ())
}

pub fn out1_rachel_ontouch(ctx: &Ctx) -> Script {
    out1_rachel_run(ctx, Out1RachelStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Hidden1RachelStep {
    Start,
    OnTouch,
}

fn hidden1_rachel_run(ctx: &Ctx, mut step: Hidden1RachelStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Hidden1RachelStep::Start => {
                step = Hidden1RachelStep::OnTouch;
                continue 'machine;
            }
            Hidden1RachelStep::OnTouch => {
                if ctx.var("ra_tem_q").get()? == 18 {
                    ctx.lines(args![
                        "^3355FFAt this distance you",
                        "can cleartly hear the",
                        "voices from the other",
                        "side of the wall...^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "????",
                        args![
                            "...Was it really",
                            "necessary to involve",
                            "an outsider in this",
                            "situation? Answer me!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "High Priest Zhed",
                        args![
                            "Is this about the",
                            "accident? You know that",
                            "I've disagreed with the",
                            "actions that have led to",
                            "the mana leakage."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "High Priest Zhed",
                        args![
                            "Even though I thought",
                            "it was a reckless decision,",
                            "I've closed the shrine and",
                            "did my best to cover up this",
                            "unfortunate accident."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "????",
                        args![
                            "I'm not blaming you. In",
                            "fact, I'll admit it: that was",
                            "our mistake. We caused",
                            "the monster infestation by",
                            "letting the mana leak."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "????",
                        args![
                            "But that's not related",
                            "to what I'm asking you!",
                            "Do you really think it's",
                            "alright to allow an outsider",
                            "to speak to our pope during",
                            "such a tumultuous time?!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "High Priest Zhed",
                        args![
                            "Why not? I thought it'd",
                            "bring her great comfort.",
                            "She is very confused as to",
                            "why the Holy Ground has been",
                            "closed, and she cannot know",
                            "what is happening there."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "High Priest Zhed",
                        args![
                            "Again... I strongly",
                            "disagree with what is",
                            "secretly going on over",
                            "there. I know that I can't",
                            "officially oppose this",
                            "course of action, but..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "????",
                        args![
                            "There's nothing to worry",
                            "about! It'll all be over before",
                            "you know it. Besides, it's",
                            "too late to turn back now,",
                            "so I warn you: don't you",
                            "dare do anything foolish."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFSomeone's coming!",
                        "You'd better step away",
                        "from the wall and feign",
                        "ignorance as best you can!^000000"
                    ])?;
                    ctx.next()?;
                    ctx.call(Function::EnableNpc, vec![Val::from("High Priestess Niren#ra")])?;
                    ctx.call(Function::Cutin, vec![Val::from("ra_gwoman"), Val::from(2)])?;
                    ctx.lines_as(
                        "High Priestess Niren",
                        args![
                            "Oh! You must be the one",
                            "High Priest Be- er, Zhed,",
                            "told me about. It's very nice",
                            "to meet you. High Priestess",
                            "Niren, at your service."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "High Priestess Niren",
                        args![
                            "I hear that you had the",
                            "privilege of speaking with",
                            "our pope, and that she had",
                            "a wonderful time. What did",
                            "you think of her, hm?"
                        ],
                    )?;
                    ctx.next()?;
                    let choice = runtime::select_values(ctx, &[Val::from("She's different than what I expected...")])?;
                    ctx.var("@menu").set(choice)?;
                    ctx.call(Function::Cutin, vec![Val::from("ra_gwoman2"), Val::from(2)])?;
                    ctx.lines_as(
                        "High Priestess Niren",
                        args![
                            "Ah, I know what you mean...",
                            "Usually, most outsiders tell",
                            "me that they expect our pope",
                            "to be a wizened old man.",
                            "But that's not the case",
                            "here in Arunafeltz."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("ra_gwoman"), Val::from(2)])?;
                    ctx.lines_as(
                        "High Priestess Niren",
                        args![
                            "Usually, our pope is chosen",
                            "by her resemblance to our",
                            "goddess Freya. She must",
                            "have silvery blond hair, snowy",
                            "skin, and divinely colored",
                            "eyes. You saw, didn't you?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "High Priestess Niren",
                        args![
                            "I'm curious: what kinds",
                            "of things did you and the",
                            "pope talk about? As you know,",
                            "we priests are forbidden from",
                            "leaving Arunafeltz, so..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFYou relate the details of",
                        "your talk with the pope, but",
                        "you make sure not to mention",
                        "anything about the Holy Ground",
                        "as High Priest Zhed had asked.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "High Priestess Niren",
                        args![
                            "Ah, that's fairly",
                            "interesting, your land.",
                            "Tell me, how did you come",
                            "to know Bekento, er, I mean,",
                            "High Priest Zhed?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["(^666666Bekento?^000000)"],
                    )?;
                    ctx.next()?;
                    let choice = runtime::select_values(ctx, &[Val::from("Explain how You Met High Priest Zhed")])?;
                    ctx.var("@menu").set(choice)?;
                    ctx.call(Function::Cutin, vec![Val::from("ra_gwoman2"), Val::from(2)])?;
                    ctx.lines_as(
                        "High Priestess Niren",
                        args![
                            ".........................",
                            "That's all? Hm? That's",
                            "pretty funny. ^333333*Sigh*^000000 I believe",
                            "he trusts people too much...",
                            "But that's only my opinion."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("ra_gwoman"), Val::from(2)])?;
                    ctx.lines_as(
                        "High Priestess Niren",
                        args![
                            "If you don't mind...",
                            "I want to give you a bit of",
                            "advice. Don't get too close",
                            "to High Priest Zhed. If you",
                            "do, you might end up getting",
                            "yourself in trouble."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                    ctx.call(Function::DisableNpc, vec![Val::from("High Priestess Niren#ra")])?;
                    ctx.lines(args![
                        "^3355FFHigh Priestess Niren left",
                        "as soon as she said those words.",
                        "What could she possibly mean?",
                        "For now, you may as well",
                        "talk to High Priest Zhed.^000000"
                    ])?;
                    ctx.var("ra_tem_q").set(Val::from(19))?;
                    if ctx.call(Function::IsBeginQuest, vec![Val::from(8102)])? == 1 {
                        ctx.call(Function::ChangeQuest, vec![Val::from(8102), Val::from(8103)])?;
                    } else if ctx.call(Function::IsBeginQuest, vec![Val::from(8101)])? == 1 {
                        ctx.call(Function::ChangeQuest, vec![Val::from(8101), Val::from(8103)])?;
                    }
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn hidden1_rachel(ctx: &Ctx) -> Script {
    hidden1_rachel_run(ctx, Hidden1RachelStep::Start, Vec::new()).map(|_| ())
}

pub fn hidden1_rachel_ontouch(ctx: &Ctx) -> Script {
    hidden1_rachel_run(ctx, Hidden1RachelStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Key1RachelStep {
    Start,
    OnTouch,
}

fn key1_rachel_run(ctx: &Ctx, mut step: Key1RachelStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Key1RachelStep::Start => {
                step = Key1RachelStep::OnTouch;
                continue 'machine;
            }
            Key1RachelStep::OnTouch => {
                if ctx.var("ra_tem_q").get()? == 20 {
                    ctx.lines(args!["^3355FFYou find a small", "shining object laid", "on the floor.^000000"])?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("Ignore:Pick It Up")])?) == 1 {
                        ctx.lines(args![
                            "^3355FFYou decided to ignore",
                            "the small shining object,",
                            "no matter how important it",
                            "may be to you in the future.^000000"
                        ])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines(args![
                        "^3355FFUpon picking up the",
                        "object, you are able to",
                        "identify it as a small key.",
                        "Perhaps you can use it to",
                        "open some kind of lock."
                    ])?;
                    ctx.var("ra_tem_q").set(Val::from(21))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn key1_rachel(ctx: &Ctx) -> Script {
    key1_rachel_run(ctx, Key1RachelStep::Start, Vec::new()).map(|_| ())
}

pub fn key1_rachel_ontouch(ctx: &Ctx) -> Script {
    key1_rachel_run(ctx, Key1RachelStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Saint1RachelStep {
    Start,
    OnTouch,
}

fn saint1_rachel_run(ctx: &Ctx, mut step: Saint1RachelStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Saint1RachelStep::Start => {
                step = Saint1RachelStep::OnTouch;
                continue 'machine;
            }
            Saint1RachelStep::OnTouch => {
                if (ctx.var("ra_tem_q").get()?.number()? > 21
                    || runtime::op(&ctx.var("misc_quest").get()?, "&", &Val::from(8192))?.is_true())
                {
                    ctx.call(Function::Warp, vec![Val::from("ra_san01"), Val::from(140), Val::from(135)])?;
                    return Err(Stop::End);
                }
                if ctx.var("ra_tem_q").get()? == 21 {
                    ctx.lines(args![
                        "^3355FFThis must be the",
                        "entrance to the Holy",
                        "Ground. However, the",
                        "door in your way is locked.^000000."
                    ])?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("Quit:Use Small Key")])?) == 2 {
                        ctx.lines(args![
                            "^3355FFYou insert the Small",
                            "Key that you found in",
                            "High Priest Zhed's room,",
                            "and find that it is able",
                            "to unlock this door.^000000"
                        ])?;
                        ctx.close_window()?;
                        ctx.call(Function::Warp, vec![Val::from("ra_san01"), Val::from(140), Val::from(135)])?;
                        return Err(Stop::End);
                    }
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.mes("^3355FFThe door is locked.^000000")?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn saint1_rachel(ctx: &Ctx) -> Script {
    saint1_rachel_run(ctx, Saint1RachelStep::Start, Vec::new()).map(|_| ())
}

pub fn saint1_rachel_ontouch(ctx: &Ctx) -> Script {
    saint1_rachel_run(ctx, Saint1RachelStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Imir1RachelStep {
    Start,
    OnTouch,
}

fn imir1_rachel_run(ctx: &Ctx, mut step: Imir1RachelStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Imir1RachelStep::Start => {
                step = Imir1RachelStep::OnTouch;
                continue 'machine;
            }
            Imir1RachelStep::OnTouch => {
                if ctx.var("ra_tem_q").get()? == 21 {
                    ctx.lines(args![
                        "^3355FFThis large area looks",
                        "like it was artificially build",
                        "for some reason-- it doesn't",
                        "seem like an area naturally",
                        "infested by monsters.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFPerhaps if you explore",
                        "this place, you'll be able",
                        "to find something interesting.^000000"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn imir1_rachel(ctx: &Ctx) -> Script {
    imir1_rachel_run(ctx, Imir1RachelStep::Start, Vec::new()).map(|_| ())
}

pub fn imir1_rachel_ontouch(ctx: &Ctx) -> Script {
    imir1_rachel_run(ctx, Imir1RachelStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Imir2RachelStep {
    Start,
    OnTouch,
}

fn imir2_rachel_run(ctx: &Ctx, mut step: Imir2RachelStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Imir2RachelStep::Start => {
                step = Imir2RachelStep::OnTouch;
                continue 'machine;
            }
            Imir2RachelStep::OnTouch => {
                if ctx.var("ra_tem_q").get()? == 21 {
                    ctx.lines(args![
                        "^3355FFThere's something here",
                        "beneath the water. This",
                        "may warrant a closer look.^000000"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn imir2_rachel(ctx: &Ctx) -> Script {
    imir2_rachel_run(ctx, Imir2RachelStep::Start, Vec::new()).map(|_| ())
}

pub fn imir2_rachel_ontouch(ctx: &Ctx) -> Script {
    imir2_rachel_run(ctx, Imir2RachelStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Imir3RachelStep {
    Start,
    OnTouch,
}

fn imir3_rachel_run(ctx: &Ctx, mut step: Imir3RachelStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Imir3RachelStep::Start => {
                step = Imir3RachelStep::OnTouch;
                continue 'machine;
            }
            Imir3RachelStep::OnTouch => {
                if ctx.var("ra_tem_q").get()? == 21 {
                    ctx.lines(args![
                        "^3355FFNo wonder this object",
                        "in the water seems so",
                        "familiar: it's a Ymir's",
                        "Heart Piece! In fact, there's",
                        "dozens of them just lying here.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFWhy would so many of these",
                        "powerful artifacts be here",
                        "in the Holy Ground? Perhaps",
                        "this is the secret that the",
                        "priests are trying to keep.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.call(Function::EnableNpc, vec![Val::from("High Priestess Niren#r2")])?;
                    ctx.call(Function::Cutin, vec![Val::from("ra_gwoman"), Val::from(2)])?;
                    ctx.lines_as(
                        "High Priestess Niren",
                        args![
                            "How dare you intrude",
                            "the Holy Ground! Identify",
                            "yourself! I'll have you tried!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("ra_gwoman2"), Val::from(2)])?;
                    ctx.lines_as(
                        "High Priestess Niren",
                        args![
                            "O-oh! It's you, the",
                            "adventurer recommended",
                            "by Bekento. Hm. I apologize",
                            "for snapping at you like",
                            "that. Didn't you know that",
                            "no one's allowed here?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("ra_gwoman"), Val::from(2)])?;
                    ctx.lines_as(
                        "High Priestess Niren",
                        args![
                            "Hmm. Well, you Rune-Midgarts",
                            "adventurers are famous for your",
                            "skills and abilities, but this",
                            "was not expected. I mean, we",
                            "made sure this place was",
                            "absolutely secure."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("ra_gwoman2"), Val::from(2)])?;
                    ctx.lines_as(
                        "High Priestess Niren",
                        args![
                            "What's done is done...",
                            "I warned Bekento that he",
                            "might cause trouble for you,",
                            "but it seems that you've",
                            "caused trouble for him."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "High Priestess Niren",
                        args![
                            "Understand this:",
                            "most intruders are",
                            "severely punished, but",
                            "because of my friendship",
                            "with Bekento, I'm letting",
                            "you off easy. Remember that."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(
                        Function::StartStatus,
                        vec![ctx.constant("SC_BLIND")?, Val::from(600000), Val::from(0)],
                    )?;
                    ctx.lines(args![
                        "^3355FFNiren began to chant",
                        "in a low voice, and your",
                        "eyelids grow heavier as you",
                        "grow drowsier and sleepier...^000000"
                    ])?;
                    ctx.close_window()?;
                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                    ctx.call(Function::DisableNpc, vec![Val::from("High Priestess Niren#r2")])?;
                    ctx.var("ra_tem_q").set(Val::from(22))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(8104), Val::from(8105)])?;
                    ctx.call(Function::Warp, vec![Val::from("rachel"), Val::from(163), Val::from(152)])?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn imir3_rachel(ctx: &Ctx) -> Script {
    imir3_rachel_run(ctx, Imir3RachelStep::Start, Vec::new()).map(|_| ())
}

pub fn imir3_rachel_ontouch(ctx: &Ctx) -> Script {
    imir3_rachel_run(ctx, Imir3RachelStep::OnTouch, Vec::new()).map(|_| ())
}

fn high_priestess_niren_r2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn high_priestess_niren_r2(ctx: &Ctx) -> Script {
    high_priestess_niren_r2_body(ctx, Vec::new()).map(|_| ())
}

fn high_priestess_niren_r2_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("High Priestess Niren#r2")])?;
    return Err(Stop::End);
}

pub fn high_priestess_niren_r2_oninit(ctx: &Ctx) -> Script {
    high_priestess_niren_r2_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn high_priestess_niren_r2_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::InitNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn high_priestess_niren_r2_onenable(ctx: &Ctx) -> Script {
    high_priestess_niren_r2_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn high_priestess_niren_r2_ontimer120000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::DisableNpc, vec![Val::from("High Priestess Niren#r2")])?;
    return Err(Stop::End);
}

pub fn high_priestess_niren_r2_ontimer120000(ctx: &Ctx) -> Script {
    high_priestess_niren_r2_ontimer120000_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Imir3Rachel2Step {
    Start,
    OnTouch,
}

fn imir3_rachel2_run(ctx: &Ctx, mut step: Imir3Rachel2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Imir3Rachel2Step::Start => {
                step = Imir3Rachel2Step::OnTouch;
                continue 'machine;
            }
            Imir3Rachel2Step::OnTouch => {
                if ctx.var("ra_tem_q").get()? == 22 {
                    ctx.lines(args![
                        "^3355FFYou feel a slight headache",
                        "after you recollect your senses.",
                        "Somehow, you're been brought",
                        "back to Rachel Town. How long",
                        "have you been unconscious?^000000"
                    ])?;
                    ctx.next()?;
                    ctx.call(Function::EndStatus, vec![ctx.constant("SC_BLIND")?])?;
                    ctx.lines(args![
                        "^3355FFIt would be best to",
                        "ask High Priest Zhed",
                        "about what had happened.^000000"
                    ])?;
                    ctx.var("ra_tem_q").set(Val::from(23))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn imir3_rachel2(ctx: &Ctx) -> Script {
    imir3_rachel2_run(ctx, Imir3Rachel2Step::Start, Vec::new()).map(|_| ())
}

pub fn imir3_rachel2_ontouch(ctx: &Ctx) -> Script {
    imir3_rachel2_run(ctx, Imir3Rachel2Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum SeekingFollowerRachelStep {
    Start,
    LMes,
    HoistEnd1,
    OnTouch,
}

fn seeking_follower_rachel_run(ctx: &Ctx, mut step: SeekingFollowerRachelStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            SeekingFollowerRachelStep::Start => {
                if (ctx.var("lost_boy").get()? == 13 && ctx.var("ra_tem_q").get()? == 20) {
                    step = SeekingFollowerRachelStep::LMes;
                    continue 'machine;
                } else {
                    {
                        ctx.lines_as("Arunafeltz Follower", args!["May Freya bless you", "on your journeys..."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                step = SeekingFollowerRachelStep::HoistEnd1;
                continue 'machine;
            }
            SeekingFollowerRachelStep::LMes => {
                ctx.lines_as(
                    "Arunafeltz Follower",
                    args![
                        "Excuse me, but",
                        ((Val::from("are you ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("?"))
                    ],
                )?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("Yes.")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as(
                    "Arunafeltz Follower",
                    args![
                        "High Priest Zhed",
                        "would like to see",
                        ((Val::from("you right away, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("."))
                    ],
                )?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("May ask why?")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as(
                    "Arunafeltz Follower",
                    args![
                        "Well, I actually have no",
                        "idea. I'm only supposed to",
                        "inform you that he's looking",
                        "for you. Please visit High",
                        "Priest Zhed in the second",
                        "right room of the temple."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
                step = SeekingFollowerRachelStep::HoistEnd1;
                continue 'machine;
            }
            SeekingFollowerRachelStep::HoistEnd1 => {
                step = SeekingFollowerRachelStep::OnTouch;
                continue 'machine;
            }
            SeekingFollowerRachelStep::OnTouch => {
                if (ctx.var("lost_boy").get()? == 13 && ctx.var("ra_tem_q").get()? == 20) {
                    step = SeekingFollowerRachelStep::LMes;
                    continue 'machine;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn seeking_follower_rachel(ctx: &Ctx) -> Script {
    seeking_follower_rachel_run(ctx, SeekingFollowerRachelStep::Start, Vec::new()).map(|_| ())
}

pub fn seeking_follower_rachel_ontouch(ctx: &Ctx) -> Script {
    seeking_follower_rachel_run(ctx, SeekingFollowerRachelStep::OnTouch, Vec::new()).map(|_| ())
}

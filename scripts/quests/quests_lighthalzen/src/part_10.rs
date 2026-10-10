use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn scamp_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("lhz_rekenber").get()?.number()? > 21 {
        ctx.lines_as(
            "Ahman",
            args![
                "Oh, hello. I've heard",
                "that you had to quit.",
                "It's quite a pity, really.",
                "If it weren't for you, some",
                "of my packages would have",
                "been destroyed by those thugs."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("lhz_rekenber").get()? == 21 {
            ctx.lines_as(
                "Ahman",
                args![
                    "Shouldn't you be",
                    "taking a break? Besides,",
                    "Lyozien is still waiting for",
                    "you on the Airship, isn't he?"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("lhz_rekenber").get()? == 20 {
                ctx.lines_as(
                    "Ahman",
                    args![
                        "Oh, have my packages",
                        "arrived? Good, good.",
                        "I appreciate all of your",
                        "hard work. I'm surprised",
                        "they haven't hired you",
                        "full time by now."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Ahman",
                    args![
                        "Is something the matter?",
                        "You seem really pale. Oh",
                        "well, you'll have plenty of",
                        "time to relax on the Airship.",
                        "Oh, and don't worry, I'll",
                        "take care of the packages."
                    ],
                )?;
                ctx.var("lhz_rekenber").set(Val::from(21))?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if (ctx.var("lhz_rekenber").get()?.number()? > 15 && ctx.var("lhz_rekenber").get()?.number()? < 20) {
                    ctx.lines_as(
                        "Ahman",
                        args![
                            "Oh, hello. I'm not",
                            "expecting any packages",
                            "at this moment, although",
                            "I'm aware that there are a",
                            "few deliveries in queue, but shouldn't you be in Lighthalzen?"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("lhz_rekenber").get()? == 15 {
                        ctx.lines_as(
                            "Ahman",
                            args![
                                "Shouldn't you be on",
                                "your way and report to",
                                "Lyozien? You should hurry",
                                "before the Airship takes off."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("lhz_rekenber").get()? == 14 {
                        ctx.lines_as(
                            "Ahman",
                            args![
                                "Ah, it's you again.",
                                "I assume that means that",
                                "my packages have arrived",
                                "safely. Is that right?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![
                                "Yes, that's right.",
                                "Actually, this time we ",
                                "were attacked by a group",
                                "of thugs, so I was wondering",
                                "if you knew anything about it... "
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ahman",
                            args![
                                "They attacked again?",
                                "Oh, that isn't good.",
                                "Well, I have no idea",
                                "what's going on. I wish",
                                "I had some idea of what",
                                "they were up to, really."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.var("lhz_rekenber").set(Val::from(15))?;
                        ctx.lines_as(
                            "Ahman",
                            args![
                                "For now, you should",
                                "go and report to Lyozien.",
                                "I assume that you protected",
                                "my packages, so thank you",
                                "for your diligent work. Now, I shall pick up what I ordered..."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if (ctx.var("lhz_rekenber").get()?.number()? > 10 && ctx.var("lhz_rekenber").get()?.number()? < 14) {
                        ctx.lines_as(
                            "Ahman",
                            args![
                                "Oh, it's you again.",
                                "Shouldn't you be getting",
                                "on the Airship and heading",
                                "back to the Schwarzwald",
                                "Republic? There are more",
                                "deliveries in queue, you know."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("lhz_rekenber").get()? == 10 {
                        ctx.lines_as(
                            "Ahman",
                            args![
                                "Thank you for letting me",
                                "know that my order has arrived.",
                                "You should go back to Lyozien",
                                "now so you can finish your job.",
                                "Perhaps I'll see you again",
                                "sometime, adventurer."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("lhz_rekenber").get()? == 9 {
                        ctx.lines_as(
                            "Man",
                            args![
                                "Hmm, can you really",
                                "call this place an Airport?",
                                "It's far too small, wouldn't",
                                "you agree? Still, I kind of",
                                "enjoy sitting around here."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![
                                "Excuse me, but do",
                                "you know where I can",
                                "find a man named Ahman?",
                                "I have a message for him."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ahman",
                            args![
                                "I'm Ahman, how can--",
                                "Oh! You must be here to",
                                "tell me that my packages",
                                "have arrived. Am I correct?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![
                                "Y-yes. That's right.",
                                "Your packages have",
                                "arrived and they're",
                                "being guarded until",
                                "you come to pick them up."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ahman",
                            args![
                                "Ah, that's very good to",
                                "know. Say, are you a new",
                                "worker for Lyozien and Kazien?",
                                "I don't believe I've seen you",
                                "around before. Have they finally started hiring part timers?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![
                                "Yes, that's right.",
                                "Actually, I'm working for",
                                "them part time. I heard",
                                "they were really busy, so",
                                "I sort of volunteered my time."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ahman",
                            args![
                                "Alright, alright.",
                                "I suppose that you",
                                "also don't know what's",
                                "being delivered in these",
                                "packages, just like Lyozien."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ahman",
                            args![
                                "Well, it's all confidential",
                                "information anyway, so don't",
                                "worry about it. Thank you for",
                                "notifying me about the delivery.^FFFFFF  ^000000 Now, you should go back and ",
                                "tell Lyozien. Take care now~"
                            ],
                        )?;
                        ctx.var("lhz_rekenber").set(Val::from(10))?;
                        ctx.call(Function::ChangeQuest, vec![Val::from(12012), Val::from(12013)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
            }
        }
    }
    ctx.lines_as(
        "Man",
        args![
            "Hmm, can you really",
            "call this place an Airport?",
            "It's far too small, wouldn't",
            "you agree? Still, I kind of",
            "enjoy sitting around here."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn scamp(ctx: &Ctx) -> Script {
    scamp_body(ctx, Vec::new()).map(|_| ())
}

pub fn bully1(ctx: &Ctx) -> Script {
    bully1_run(ctx, Bully1Step::Start, Vec::new()).map(|_| ())
}

pub fn bully1_oninit(ctx: &Ctx) -> Script {
    bully1_run(ctx, Bully1Step::OnInit, Vec::new()).map(|_| ())
}

pub fn bully1_onenter(ctx: &Ctx) -> Script {
    bully1_run(ctx, Bully1Step::OnEnter, Vec::new()).map(|_| ())
}

pub fn bully1_onreset(ctx: &Ctx) -> Script {
    bully1_run(ctx, Bully1Step::OnReset, Vec::new()).map(|_| ())
}

pub fn bully1_onmymobdead(ctx: &Ctx) -> Script {
    bully1_run(ctx, Bully1Step::OnMyMobDead, Vec::new()).map(|_| ())
}

pub fn bully1_ontimer120000(ctx: &Ctx) -> Script {
    bully1_run(ctx, Bully1Step::OnTimer120000, Vec::new()).map(|_| ())
}

pub fn bully2(ctx: &Ctx) -> Script {
    bully2_run(ctx, Bully2Step::Start, Vec::new()).map(|_| ())
}

pub fn bully2_oninit(ctx: &Ctx) -> Script {
    bully2_run(ctx, Bully2Step::OnInit, Vec::new()).map(|_| ())
}

pub fn bully2_onenter(ctx: &Ctx) -> Script {
    bully2_run(ctx, Bully2Step::OnEnter, Vec::new()).map(|_| ())
}

pub fn bully2_onreset(ctx: &Ctx) -> Script {
    bully2_run(ctx, Bully2Step::OnReset, Vec::new()).map(|_| ())
}

pub fn bully2_onmymobdead(ctx: &Ctx) -> Script {
    bully2_run(ctx, Bully2Step::OnMyMobDead, Vec::new()).map(|_| ())
}

pub fn bully2_ontimer120000(ctx: &Ctx) -> Script {
    bully2_run(ctx, Bully2Step::OnTimer120000, Vec::new()).map(|_| ())
}

pub fn packidentity(ctx: &Ctx) -> Script {
    packidentity_run(ctx, PackidentityStep::Start, Vec::new()).map(|_| ())
}

pub fn packidentity_oninit(ctx: &Ctx) -> Script {
    packidentity_run(ctx, PackidentityStep::OnInit, Vec::new()).map(|_| ())
}

pub fn packidentity_onenter(ctx: &Ctx) -> Script {
    packidentity_run(ctx, PackidentityStep::OnEnter, Vec::new()).map(|_| ())
}

pub fn packidentity_ontouch(ctx: &Ctx) -> Script {
    packidentity_run(ctx, PackidentityStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn packidentity_ontimer120000(ctx: &Ctx) -> Script {
    packidentity_run(ctx, PackidentityStep::OnTimer120000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Flashback1Step {
    Start,
    OnTouch,
}

fn flashback1_run(ctx: &Ctx, mut step: Flashback1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Flashback1Step::Start => {
                step = Flashback1Step::OnTouch;
                continue 'machine;
            }
            Flashback1Step::OnTouch => {
                if ctx.var("lhz_rekenber").get()? == 22 {
                    ctx.call(Function::Cutin, vec![Val::from("lhz_kaz11"), Val::from(2)])?;
                    ctx.lines_as(
                        "Kazien",
                        args![
                            "-Don't you have anything to protect, huh?- ",
                            "-Are you sure that you're always doing the right thing?-"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Kazien", args!["Answer me! Answer me! Answer meee!"])?;
                    ctx.next()?;
                    ctx.var("lhz_rekenber").set(Val::from(23))?;
                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![".............Damn it."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn flashback1(ctx: &Ctx) -> Script {
    flashback1_run(ctx, Flashback1Step::Start, Vec::new()).map(|_| ())
}

pub fn flashback1_ontouch(ctx: &Ctx) -> Script {
    flashback1_run(ctx, Flashback1Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Flashback2Step {
    Start,
    OnTouch,
}

fn flashback2_run(ctx: &Ctx, mut step: Flashback2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Flashback2Step::Start => {
                step = Flashback2Step::OnTouch;
                continue 'machine;
            }
            Flashback2Step::OnTouch => {
                if ctx.var("lhz_rekenber").get()? == 22 {
                    ctx.call(Function::Cutin, vec![Val::from("lhz_kaz11"), Val::from(2)])?;
                    ctx.lines_as(
                        "Kazien",
                        args![
                            "Look man, this is what",
                            "I decided. I don't care",
                            "what other people'll think.",
                            "I might go to hell when",
                            "I die, but that's my problem."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kazien",
                        args![
                            "Besides, you adventurers",
                            "are always running around",
                            "with your swords and magic spells... Isn't that just as bad?",
                            "It's not the weapons or the power that's bad: it's how they're used."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kazien",
                        args![
                            "There'd be days when my",
                            "brother and I'd have nothing",
                            "to eat. So when I heard about",
                            "this job, I took it. What good",
                            "is world peace if I'm not even",
                            "alive to enjoy it, huh?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.var("lhz_rekenber").set(Val::from(23))?;
                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["......", ".........", "Damn it!"],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn flashback2(ctx: &Ctx) -> Script {
    flashback2_run(ctx, Flashback2Step::Start, Vec::new()).map(|_| ())
}

pub fn flashback2_ontouch(ctx: &Ctx) -> Script {
    flashback2_run(ctx, Flashback2Step::OnTouch, Vec::new()).map(|_| ())
}

fn lyozienswitch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    let mut l_str_s = Val::from("");
    shared::other_gm_npcs::f_gm_npc(ctx, vec![])?;
    ctx.lines_as("Lyozien Switch", args!["Input password.", "Enter 0 to cancel."])?;
    ctx.next()?;
    l_i = shared::other_gm_npcs::f_gm_npc(ctx, vec![Val::from(1028), Val::from(0), Val::from(0), Val::from(4000)])?;
    if l_i.clone() == -2 {
        ctx.lines_as("Lyozien Switch", args!["Incorrect."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if l_i.clone() == -1 {
        ctx.lines_as("Lyozien Switch", args!["Canceled."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if l_i.clone() == 0 {
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as("Lyozien Switch", args!["Do you want to", "turn the Lyozien", "NPC ON or OFF?"])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("On:OFF")])? {
            1 => {
                l_str_s = Val::from("activated");
                ctx.call(Function::DoNpcEvent, vec![Val::from("Man#Lyozien::OnEnable")])?;
            }
            2 => {
                l_str_s = Val::from("deactivated");
                ctx.call(Function::DoNpcEvent, vec![Val::from("Man#Lyozien::OnDisable")])?;
            }
            _ => {}
        }
        ctx.lines_as(
            "Lyozien Switch",
            args![
                "Lyozien NPC is",
                ((Val::from("now ") + l_str_s.clone()) + Val::from(".")),
                " ",
                "/mm airplane_01.gat 96 48"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn lyozienswitch(ctx: &Ctx) -> Script {
    lyozienswitch_body(ctx, Vec::new()).map(|_| ())
}

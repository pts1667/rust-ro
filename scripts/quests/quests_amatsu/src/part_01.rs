use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn publisher_ama_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_event_momo = Val::from(0);
    let mut l_gift_1 = Val::from(0);
    let mut l_gift_2 = Val::from(0);
    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0 {
        ctx.lines(args![
            "- Wait a moment!! -",
            "- Currently you are carrying -",
            "- too many items with you. -",
            "- Please store some items into your Kafra storage -",
            "- and try again. -"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("event_momo").get()?.number()? < 2 {
        ctx.lines_as(
            "Publisher",
            args![
                "Hello~!!",
                "Our ^009CFFScroll Publishing Company^000000",
                "is professionally publishing",
                "tales.",
                " "
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Publisher",
            args![
                "We established",
                "^3163FFMomotaro Field Trip^000000",
                "to celebrate ^009CFF<Momotaro Story>^000000 selling over a million copies."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Publisher",
            args![
                "Become Momotaro and eliminate",
                "Dokebis, just like in the story!",
                "There will also be rewards."
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(
            ctx,
            &[Val::from("What is Momotaro Story?:I want to try!:I'm busy right now...")],
        )? {
            1 => {
                ctx.lines_as(
                    "Publisher",
                    args![
                        "Eh~!? How can you not know about",
                        "Momotaro? Don't they have this",
                        "tale in Midgard?? Well...",
                        "Let me tell you the story."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Publisher",
                    args!["Long ago, there lived an old married couple. They were happy, but didn't have any children."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Publisher",
                    args![
                        "One day, while old lady was doing the laundy, she found a humongous",
                        "peach. She cut it in half and...",
                        "'Poof!' There was a baby inside!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Publisher",
                    args![
                        "The old man and lady were shocked.",
                        "They decided to adopt that baby,",
                        "and give him their love..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Publisher", args!["That baby grew so fast, like that beanstalk from 'Jack and Beanstalk,' and became a strong boy in only a few days.", " "])?;
                ctx.next()?;
                ctx.lines_as(
                    "Publisher",
                    args![
                        "That boy's name was...",
                        "<Momotaro>!!!",
                        "Momotaro traveled to eliminate",
                        "dokebis that were harassing",
                        "the towners."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Publisher",
                    args![
                        "He became friends with",
                        "^3163FFYoyo, Picky, and Desert Wolf^000000",
                        "and eliminated all of the Dokebis. Then he lived happily ever after with the old couple."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Publisher",
                    args!["How was it?", "Great, right?", "I mean, this story sold a million copies!"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                if ctx.var("BaseLevel").get()?.number()? > 29 {
                    ctx.lines_as(
                        "Publisher",
                        args![
                            "Yay-! You look excited!",
                            "You'll be ready to go soon.",
                            "Please fill out this registration card."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Publisher",
                        args![
                            "....................",
                            "......Mm.....Let's see.....",
                            ".............Good!",
                            "Alright, then!",
                            "Let it begin~~~!!!!!!!!!",
                            "<Momotaro Field Trip>~!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Warp, vec![Val::from("ama_test"), Val::from(52), Val::from(35)])?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Publisher",
                    args!["Hmm~", "Why don't you train yourself more", "and come back?"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            3 => {
                ctx.lines_as("Publisher", args!["Eh?", "...But there are sweet rewards waiting..."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if ctx.var("event_momo").get()? == 4 {
        ctx.lines_as(
            "Publisher",
            args!["Ah~ What nice weather!", "This is the perfect weather to do some reading."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("event_momo").get()?.number()? > 1 {
        ctx.lines_as(
            "Publisher",
            args![
                "How was it? Did you have fun?",
                "I hope you had a good time",
                "during the field trip.",
                " ",
                " "
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Publisher",
            args!["This is your reward for you.", "Please continue to enjoy stories~", " "],
        )?;
        l_event_momo = ctx.var("event_momo").get()?;
        ctx.var("event_momo").set(Val::from(4))?;
        ctx.call(Function::CompleteQuest, vec![Val::from(8128)])?;
        ctx.call(Function::CompleteQuest, vec![Val::from(8129)])?;
        ctx.call(Function::CompleteQuest, vec![Val::from(8130)])?;
        if l_event_momo.clone() == 3 {
            l_gift_1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?;
            if l_gift_1.clone() == 1 {
                ctx.call(Function::GetItem, vec![Val::from(659), Val::from(1)])?;
            }
            if l_gift_1.clone() == 2 {
                ctx.call(Function::GetItem, vec![Val::from(633), Val::from(1)])?;
            }
            if l_gift_1.clone() == 3 {
                ctx.call(Function::GetItem, vec![Val::from(634), Val::from(1)])?;
            }
            if l_gift_1.clone() == 4 {
                ctx.call(Function::GetItem, vec![Val::from(639), Val::from(1)])?;
            }
            if l_gift_1.clone() == 5 {
                ctx.call(Function::GetItem, vec![Val::from(636), Val::from(1)])?;
            }
            if l_gift_1.clone() == 6 {
                ctx.call(Function::GetItem, vec![Val::from(628), Val::from(1)])?;
            }
            if l_gift_1.clone() == 7 {
                ctx.call(Function::GetItem, vec![Val::from(637), Val::from(1)])?;
            }
            if l_gift_1.clone() == 8 {
                ctx.call(Function::GetItem, vec![Val::from(635), Val::from(1)])?;
            }
            if l_gift_1.clone() == 9 {
                ctx.call(Function::GetItem, vec![Val::from(626), Val::from(1)])?;
            }
            if l_gift_1.clone() == 10 {
                ctx.call(Function::GetItem, vec![Val::from(641), Val::from(1)])?;
            }
        } else {
            l_gift_2 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?;
            if l_gift_2.clone() == 1 {
                ctx.call(Function::GetItem, vec![Val::from(622), Val::from(1)])?;
            }
            if l_gift_2.clone() == 2 {
                ctx.call(Function::GetItem, vec![Val::from(627), Val::from(1)])?;
            }
            if l_gift_2.clone() == 3 {
                ctx.call(Function::GetItem, vec![Val::from(629), Val::from(1)])?;
            }
            if l_gift_2.clone() == 4 {
                ctx.call(Function::GetItem, vec![Val::from(632), Val::from(1)])?;
            }
            if l_gift_2.clone() == 5 {
                ctx.call(Function::GetItem, vec![Val::from(623), Val::from(1)])?;
            }
            if l_gift_2.clone() == 6 {
                ctx.call(Function::GetItem, vec![Val::from(619), Val::from(1)])?;
            }
            if l_gift_2.clone() == 7 {
                ctx.call(Function::GetItem, vec![Val::from(621), Val::from(1)])?;
            }
            if l_gift_2.clone() == 8 {
                ctx.call(Function::GetItem, vec![Val::from(620), Val::from(1)])?;
            }
            if l_gift_2.clone() == 9 {
                ctx.call(Function::GetItem, vec![Val::from(625), Val::from(1)])?;
            }
            if l_gift_2.clone() == 10 {
                ctx.call(Function::GetItem, vec![Val::from(624), Val::from(1)])?;
            }
        }
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn publisher_ama(ctx: &Ctx) -> Script {
    publisher_ama_body(ctx, Vec::new()).map(|_| ())
}

fn assistant_ama_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("event_momo").get()? == 0 {
        ctx.lines_as("Satoshi", args!["Welcome to <Momotaro Field Trip>", " "])?;
        ctx.next()?;
        ctx.lines_as(
            "Satoshi",
            args!["I'm Satoshi who is in charge of the waiting room in <Momotaro Field Trip>."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Satoshi",
            args![
                "Originally, I was working for the publishing company, but we lacked workers for the Field Trip...",
                "But working here is much better!",
                "Now I just watch people trying to accomplish the mission. Heh hehe~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Satoshi", args!["................", "Just kidding", "*Wipes away sweat*"])?;
        ctx.next()?;
        ctx.lines_as(
            "Satoshi",
            args!["Well, let me explain", "about the rules in the Field Trip.", " "],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Satoshi",
            args!["First of all...Did you hear", "about the story from the publisher?"],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Yes:No")])?) == 1 {
            ctx.lines_as(
                "Satoshi",
                args![
                    "Hehe...Good.",
                    "Momotaro Story is our",
                    "pride. It's the best story EVER.",
                    "Hahahah!!!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Satoshi", args!["...Hmmhmm."])?;
            ctx.next()?;
            ctx.lines_as(
                "Satoshi",
                args![
                    "The Field Trip is simple.",
                    "Go inside and eliminate",
                    "Dokebis bravely!!!",
                    "Just like Momotaro!!!",
                    "Understand? Bravely!!!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Satoshi", args!["In addition, there are three", "things you need to know."])?;
            ctx.next()?;
            ctx.lines_as(
                "Satoshi",
                args!["First....", "You can't come back once you clear the field trip. Keep that in mind."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Satoshi",
                args![
                    "Secondly....",
                    "Q-pet is prohibited",
                    "inside of the field trip.",
                    "If you are with a pet,",
                    "please change it to egg status."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Satoshi",
                args!["And finally...", "One person can be on the", "field trip for 6 minutes."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Satoshi",
                args![
                    "Well then, please",
                    "wait your turn",
                    "in the waiting room.",
                    "Good luck in fighting!!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Assistant",
            args![
                "Eh? You can't just",
                "skip the story",
                "when you enter field trip!!",
                "I can't allow you to do that",
                "as Momotaro's fan!!!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Assistant",
            args!["Please listen to the story", "from the publisher.", "Thank you.", "Bye~~"],
        )?;
        ctx.next()?;
        ctx.call(Function::Warp, vec![Val::from("amatsu"), Val::from(223), Val::from(230)])?;
        return Err(Stop::End);
    } else if ctx.var("event_momo").get()? == 1 {
        ctx.lines_as(
            "Satoshi",
            args![
                "Oh my...Are you alright?",
                "I thought it was entertaining but maybe it was too hard for you?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Satoshi", args!["Now what are you going to do?", "Do you want to go in again?"])?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Yes:No, I want to go back")])?) == 1 {
            ctx.lines_as(
                "Satoshi",
                args![
                    "The more effort you put into this, the sweeter victory will taste.",
                    "Good luck in fighting!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Satoshi",
            args![
                "Well, I guess the reality of the situation is that you just can't fulfill the role of Momotaro...",
                "Still, don't be depressed."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Satoshi", args!["I think you've gained enough experience from the field trip."])?;
        ctx.next()?;
        ctx.lines_as(
            "Assistant",
            args!["If you hear a good story,", "Please, contact our", "publishing company.", "Bye-"],
        )?;
        ctx.close_window()?;
        ctx.call(Function::PercentHeal, vec![Val::from(100), Val::from(0)])?;
        ctx.var("event_momo").set(Val::from(0))?;
        ctx.call(Function::Warp, vec![Val::from("amatsu"), Val::from(223), Val::from(230)])?;
        return Err(Stop::End);
    } else if ctx.var("event_momo").get()? == 2 {
        ctx.lines_as("Satoshi", args!["Woohoo~ Congratulations!!", "You were so great!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Satoshi",
            args![
                "It is time to say good bye.",
                "If you hear a good story,",
                "Please, contact our",
                "publishing company.",
                "Bye-"
            ],
        )?;
        ctx.close_window()?;
        ctx.call(Function::PercentHeal, vec![Val::from(100), Val::from(0)])?;
        ctx.call(Function::Warp, vec![Val::from("amatsu"), Val::from(223), Val::from(230)])?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Satoshi",
        args![
            "Woohoo~ Congratulations!!",
            "You were so great!!",
            "Even though your finish was kind of weak..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Satoshi",
        args![
            "It is time to say good bye.",
            "If you hear a good story,",
            "Please, contact our",
            "publishing company.",
            "Bye-"
        ],
    )?;
    ctx.close_window()?;
    ctx.call(Function::Warp, vec![Val::from("amatsu"), Val::from(223), Val::from(230)])?;
    return Err(Stop::End);
}

pub fn assistant_ama(ctx: &Ctx) -> Script {
    assistant_ama_body(ctx, Vec::new()).map(|_| ())
}

fn assistant_ama_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::WaitingRoom,
        vec![
            Val::from("Waiting Exhibit."),
            Val::from(10),
            Val::from("Assistant#ama::OnStartArena"),
            Val::from(1),
        ],
    )?;
    ctx.call(Function::EnableWaitingRoomEvent, vec![Val::from("Assistant#ama")])?;
    return Err(Stop::End);
}

pub fn assistant_ama_oninit(ctx: &Ctx) -> Script {
    assistant_ama_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn assistant_ama_onstartarena_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Coach#ama")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("Am Mut#ama::OnReset")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("Dokebi#ez::OnReset")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("Dokebi#hd::OnReset")])?;
    ctx.call(Function::EnableNpc, vec![Val::from("Grandma#ama1")])?;
    ctx.call(Function::EnableNpc, vec![Val::from("Grandpa#ama")])?;
    ctx.call(
        Function::WarpWaitingPc,
        vec![Val::from("ama_test"), Val::from(50), Val::from(83)],
    )?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("Timer#ama::OnEnable")])?;
    ctx.call(Function::DisableWaitingRoomEvent, vec![Val::from("Assistant#ama")])?;
    return Err(Stop::End);
}

pub fn assistant_ama_onstartarena(ctx: &Ctx) -> Script {
    assistant_ama_onstartarena_body(ctx, Vec::new()).map(|_| ())
}

fn assistant_ama_onreset_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableWaitingRoomEvent, vec![Val::from("Assistant#ama")])?;
    return Err(Stop::End);
}

pub fn assistant_ama_onreset(ctx: &Ctx) -> Script {
    assistant_ama_onreset_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum HanakoChanAmaStep {
    Start,
    OnTouch,
}

fn hanako_chan_ama_run(ctx: &Ctx, mut step: HanakoChanAmaStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            HanakoChanAmaStep::Start => {
                step = HanakoChanAmaStep::OnTouch;
                continue 'machine;
            }
            HanakoChanAmaStep::OnTouch => {
                ctx.lines_as("Hanako chan", args![".......Eeeeheeheehee", "....................."])?;
                ctx.next()?;
                ctx.lines_as("Hanako chan", args!["..Eeeeheeheeheeheeheehee", ".........................."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Hanako chan",
                    args![
                        "......................",
                        "...Want red toilet paper....",
                        ".....or bl-ue toilet paper.... "
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn hanako_chan_ama(ctx: &Ctx) -> Script {
    hanako_chan_ama_run(ctx, HanakoChanAmaStep::Start, Vec::new()).map(|_| ())
}

pub fn hanako_chan_ama_ontouch(ctx: &Ctx) -> Script {
    hanako_chan_ama_run(ctx, HanakoChanAmaStep::OnTouch, Vec::new()).map(|_| ())
}

fn grandpa_ama_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn grandpa_ama(ctx: &Ctx) -> Script {
    grandpa_ama_body(ctx, Vec::new()).map(|_| ())
}

fn grandpa_ama_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Grandpa#ama")])?;
    return Err(Stop::End);
}

pub fn grandpa_ama_oninit(ctx: &Ctx) -> Script {
    grandpa_ama_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn grandma_ama1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn grandma_ama1(ctx: &Ctx) -> Script {
    grandma_ama1_body(ctx, Vec::new()).map(|_| ())
}

fn grandma_ama1_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Grandma#ama1")])?;
    return Err(Stop::End);
}

pub fn grandma_ama1_oninit(ctx: &Ctx) -> Script {
    grandma_ama1_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn grandma_ama1_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Grandma", args!["Arrrk!!!"])?;
    ctx.next()?;
    ctx.lines_as("Grandma", args!["Honey!! Someone hacked my account and stole my equipment!! "])?;
    ctx.next()?;
    ctx.lines_as(
        "Grandpa",
        args!["Hmm. The Chief said Dokebi earned 20 million zeny by hacking others' accounts..."],
    )?;
    ctx.next()?;
    ctx.call(Function::SetQuest, vec![Val::from(8127)])?;
    ctx.lines_as("Grandma", args!["Dohhhhhhh!!!! "])?;
    ctx.next()?;
    ctx.lines_as("Grandpa", args!["....Huh I've heard that somewhere.. ..... "])?;
    ctx.next()?;
    ctx.lines_as("Grandpa", args!["Anyway, Momotaro..."])?;
    ctx.next()?;
    ctx.lines_as(
        "Grandpa",
        args![
            "As you know, since we towners spend our time sitting and chatting,",
            "we can't kill Dokebis because we're low level, so...Please do it for us. "
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Grandma",
        args![
            "Momotaro...",
            "I really wanted to give you",
            "a handmade ^3163FFYummiest Red Potion in the whole world^000000 ...but...",
            ".....I failed to make them ...."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Grandma", args!["...I'm sorry. I should have been leveling up before...*Sob*"])?;
    ctx.next()?;
    ctx.lines_as("Grandma & Grandpa", args!["So...Do me this favor, kid~"])?;
    ctx.close_window()?;
    ctx.var("event_momo").set(Val::from(1))?;
    if ((ctx.call(Function::CountItem, vec![Val::from(9010)])?.number()? > 0
        || ctx.call(Function::CountItem, vec![Val::from(9005)])?.number()? > 0)
        || ctx.call(Function::CountItem, vec![Val::from(9016)])?.number()? > 0)
    {
        ctx.call(Function::DisableNpc, vec![Val::from("Grandpa#ama")])?;
        ctx.call(Function::DisableNpc, vec![Val::from("Grandma#ama1")])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("Dokebi#ez::OnEnable")])?;
    } else {
        ctx.call(Function::DisableNpc, vec![Val::from("Grandpa#ama")])?;
        ctx.call(Function::DisableNpc, vec![Val::from("Grandma#ama1")])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("Dokebi#hd::OnEnable")])?;
    }
    return Err(Stop::End);
}

pub fn grandma_ama1_ontouch(ctx: &Ctx) -> Script {
    grandma_ama1_ontouch_body(ctx, Vec::new()).map(|_| ())
}

pub fn dokebi_ez(ctx: &Ctx) -> Script {
    dokebi_ez_run(ctx, DokebiEzStep::Start, Vec::new()).map(|_| ())
}

pub fn dokebi_ez_oninit(ctx: &Ctx) -> Script {
    dokebi_ez_run(ctx, DokebiEzStep::OnInit, Vec::new()).map(|_| ())
}

pub fn dokebi_ez_onenable(ctx: &Ctx) -> Script {
    dokebi_ez_run(ctx, DokebiEzStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn dokebi_ez_onreset(ctx: &Ctx) -> Script {
    dokebi_ez_run(ctx, DokebiEzStep::OnReset, Vec::new()).map(|_| ())
}

pub fn dokebi_ez_onmymobdead(ctx: &Ctx) -> Script {
    dokebi_ez_run(ctx, DokebiEzStep::OnMyMobDead, Vec::new()).map(|_| ())
}

pub fn dokebi_hd(ctx: &Ctx) -> Script {
    dokebi_hd_run(ctx, DokebiHdStep::Start, Vec::new()).map(|_| ())
}

pub fn dokebi_hd_oninit(ctx: &Ctx) -> Script {
    dokebi_hd_run(ctx, DokebiHdStep::OnInit, Vec::new()).map(|_| ())
}

pub fn dokebi_hd_onenable(ctx: &Ctx) -> Script {
    dokebi_hd_run(ctx, DokebiHdStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn dokebi_hd_onreset(ctx: &Ctx) -> Script {
    dokebi_hd_run(ctx, DokebiHdStep::OnReset, Vec::new()).map(|_| ())
}

pub fn dokebi_hd_onmymobdead(ctx: &Ctx) -> Script {
    dokebi_hd_run(ctx, DokebiHdStep::OnMyMobDead, Vec::new()).map(|_| ())
}

fn coach_ama_run(ctx: &Ctx, mut step: CoachAmaStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            CoachAmaStep::Start => {
                ctx.lines_as(
                    "Coach",
                    args![
                        "Hoho~ Good.",
                        "I've been watching you in the VIP room. You were really something.",
                        "I was truly amazed."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Coach",
                    args!["I thought you were really", "Momotaro in the story!", "Hohohoho..."],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Well, well. It is nothing~:It was boring.")])? {
                    1 => {
                        ctx.lines_as(
                            "Coach",
                            args![
                                "...Hohoho..",
                                "Don't be so proud of yourself.",
                                "When I was young like you,",
                                "I squashed Dokebi with",
                                "my little finger. Hohoho~"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Coach",
                            args!["..............", "...Don't give me that look.", "I was just joking. Hmmhmm."],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::ChangeQuest, vec![Val::from(8127), Val::from(8128)])?;
                        ctx.lines_as(
                            "Coach",
                            args![
                                "Now, the Momotaro story is over.",
                                "You can get your reward when you talk to the Publisher lady you saw first."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Coach", args!["Don't lose your high self-esteem", "in the future. Farewell."])?;
                        ctx.close_window()?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("Timer#ama::OnDisable")])?;
                        ctx.call(Function::Warp, vec![Val::from("amatsu"), Val::from(223), Val::from(230)])?;
                        ctx.call(Function::DisableNpc, vec![Val::from("Coach#ama")])?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("Assistant#ama::OnReset")])?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as(
                            "Coach",
                            args![
                                ".....!!!!!!",
                                "....Hohohoho..",
                                "You're pretty funny.",
                                "Alright, hot stuff.",
                                "Do you want to listen to my proposal?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::ChangeQuest, vec![Val::from(8127), Val::from(8129)])?;
                        ctx.lines_as(
                            "Coach",
                            args![
                                "At this point I'm supposed to send you back...BUT!",
                                "I really want to know",
                                "what you are capable of. Hehe~",
                                "Hohoho~"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Coach",
                            args![
                                "So, I will give you a chance.",
                                "But if you fail,",
                                "^3163FFyou can't take this challenge.^000000",
                                "Also, this mission will be pretty hard."
                            ],
                        )?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(ctx, &[Val::from("Nah.. I'm good.:Bring it on, foo'!")])?) == 1 {
                            ctx.call(Function::EraseQuest, vec![Val::from(8129)])?;
                            ctx.lines_as(
                                "Coach",
                                args![
                                    "Hoho~I understand...",
                                    "You must be exhausted from the previous battle. It would have been a grand battle, though..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Coach",
                                args![
                                    "Now, the Momotaro story is over.",
                                    "You can get your reward",
                                    "when you talk to",
                                    "the Publisher lady you first spoke to."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Coach", args!["Don't lose your high self-esteem", "in the future. Farewell."])?;
                            ctx.next()?;
                            ctx.call(Function::SetQuest, vec![Val::from(8128)])?;
                            ctx.call(Function::Warp, vec![Val::from("amatsu"), Val::from(223), Val::from(230)])?;
                            ctx.call(Function::DisableNpc, vec![Val::from("Coach#ama")])?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from("Timer#ama::OnDisable")])?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from("Assistant#ama::OnReset")])?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as(
                            "Coach",
                            args![
                                "Hohoho~ I like your attitude.",
                                "Now, this is the last battle!",
                                "Show me what you got.",
                                "I'm looking forward to this."
                            ],
                        )?;
                        ctx.call(Function::PercentHeal, vec![Val::from(70), Val::from(0)])?;
                        ctx.call(Function::DisableNpc, vec![Val::from("Coach#ama")])?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("Am Mut#ama::OnEnable")])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
                step = CoachAmaStep::OnInit;
                continue 'machine;
            }
            CoachAmaStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Coach#ama")])?;
                return Err(Stop::End);
            }
            CoachAmaStep::OnTouch => {
                ctx.lines_as("Coach", args!["Boom bam Boooom!!!", "Tada~~~ !"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn coach_ama(ctx: &Ctx) -> Script {
    coach_ama_run(ctx, CoachAmaStep::Start, Vec::new()).map(|_| ())
}

pub fn coach_ama_oninit(ctx: &Ctx) -> Script {
    coach_ama_run(ctx, CoachAmaStep::OnInit, Vec::new()).map(|_| ())
}

pub fn coach_ama_ontouch(ctx: &Ctx) -> Script {
    coach_ama_run(ctx, CoachAmaStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn am_mut_ama(ctx: &Ctx) -> Script {
    am_mut_ama_run(ctx, AmMutAmaStep::Start, Vec::new()).map(|_| ())
}

pub fn am_mut_ama_oninit(ctx: &Ctx) -> Script {
    am_mut_ama_run(ctx, AmMutAmaStep::OnInit, Vec::new()).map(|_| ())
}

pub fn am_mut_ama_onenable(ctx: &Ctx) -> Script {
    am_mut_ama_run(ctx, AmMutAmaStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn am_mut_ama_onreset(ctx: &Ctx) -> Script {
    am_mut_ama_run(ctx, AmMutAmaStep::OnReset, Vec::new()).map(|_| ())
}

pub fn am_mut_ama_onmymobdead(ctx: &Ctx) -> Script {
    am_mut_ama_run(ctx, AmMutAmaStep::OnMyMobDead, Vec::new()).map(|_| ())
}

fn coach_after_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn coach_after(ctx: &Ctx) -> Script {
    coach_after_body(ctx, Vec::new()).map(|_| ())
}

fn coach_after_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Coach#after")])?;
    return Err(Stop::End);
}

pub fn coach_after_oninit(ctx: &Ctx) -> Script {
    coach_after_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn coach_after_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Coach", args!["( Clap- Clap- Clap- )"])?;
    ctx.next()?;
    ctx.lines_as(
        "Coach",
        args![
            "Hohoho~ You are really something.",
            "You've got the moves, kiddo.",
            "I'll give you that."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Coach",
        args![
            "Well, time's up so...",
            "Let me show you the exit.",
            "I had a great time...",
            "Don't forget to get your reward~"
        ],
    )?;
    ctx.close_window()?;
    ctx.var("event_momo").set(Val::from(3))?;
    ctx.call(Function::ChangeQuest, vec![Val::from(8129), Val::from(8130)])?;
    ctx.call(Function::Warp, vec![Val::from("amatsu"), Val::from(223), Val::from(230)])?;
    ctx.call(Function::DisableNpc, vec![Val::from("Coach#after")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("Timer#ama::OnDisable")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("Assistant#ama::OnReset")])?;
    return Err(Stop::End);
}

pub fn coach_after_ontouch(ctx: &Ctx) -> Script {
    coach_after_ontouch_body(ctx, Vec::new()).map(|_| ())
}

pub fn timer_ama(ctx: &Ctx) -> Script {
    timer_ama_run(ctx, TimerAmaStep::Start, Vec::new()).map(|_| ())
}

pub fn timer_ama_oninit(ctx: &Ctx) -> Script {
    timer_ama_run(ctx, TimerAmaStep::OnInit, Vec::new()).map(|_| ())
}

pub fn timer_ama_onenable(ctx: &Ctx) -> Script {
    timer_ama_run(ctx, TimerAmaStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn timer_ama_ondisable(ctx: &Ctx) -> Script {
    timer_ama_run(ctx, TimerAmaStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn timer_ama_ontimer1000(ctx: &Ctx) -> Script {
    timer_ama_run(ctx, TimerAmaStep::OnTimer1000, Vec::new()).map(|_| ())
}

pub fn timer_ama_ontimer181000(ctx: &Ctx) -> Script {
    timer_ama_run(ctx, TimerAmaStep::OnTimer181000, Vec::new()).map(|_| ())
}

pub fn timer_ama_ontimer301000(ctx: &Ctx) -> Script {
    timer_ama_run(ctx, TimerAmaStep::OnTimer301000, Vec::new()).map(|_| ())
}

pub fn timer_ama_ontimer361000(ctx: &Ctx) -> Script {
    timer_ama_run(ctx, TimerAmaStep::OnTimer361000, Vec::new()).map(|_| ())
}

pub fn timer_ama_ontimer361500(ctx: &Ctx) -> Script {
    timer_ama_run(ctx, TimerAmaStep::OnTimer361500, Vec::new()).map(|_| ())
}

pub fn timer_ama_ontimer362000(ctx: &Ctx) -> Script {
    timer_ama_run(ctx, TimerAmaStep::OnTimer362000, Vec::new()).map(|_| ())
}

pub fn timer_ama_ontimer362500(ctx: &Ctx) -> Script {
    timer_ama_run(ctx, TimerAmaStep::OnTimer362500, Vec::new()).map(|_| ())
}

fn backwarp_ama_run(ctx: &Ctx, mut step: BackwarpAmaStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            BackwarpAmaStep::Start => {
                step = BackwarpAmaStep::OnInit;
                continue 'machine;
            }
            BackwarpAmaStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("backwarp#ama")])?;
                return Err(Stop::End);
            }
            BackwarpAmaStep::OnTouch => {
                ctx.call(Function::Warp, vec![Val::from("amatsu"), Val::from(115), Val::from(95)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn backwarp_ama(ctx: &Ctx) -> Script {
    backwarp_ama_run(ctx, BackwarpAmaStep::Start, Vec::new()).map(|_| ())
}

pub fn backwarp_ama_oninit(ctx: &Ctx) -> Script {
    backwarp_ama_run(ctx, BackwarpAmaStep::OnInit, Vec::new()).map(|_| ())
}

pub fn backwarp_ama_ontouch(ctx: &Ctx) -> Script {
    backwarp_ama_run(ctx, BackwarpAmaStep::OnTouch, Vec::new()).map(|_| ())
}

fn sushi_master_ama_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_fish_m1 = Val::from(0);
    if (ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 5000 {
        ctx.lines_as(
            "Magumagu",
            args![
                "Hey. You look really heavy.",
                "Don't you have trouble walking?",
                "I'm sorry, but there is no space",
                "to put down your stuff in my shop."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Magumagu",
            args![
                "Put some of your stuff away somewhere.",
                "Why are you carrying so much...?",
                "Huhuhu..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("ama_sushi").get()? == 4 {
            ctx.lines_as(
                "Magumagu",
                args![
                    "Training to slice raw fish",
                    "daily will make you a master sushi chef.",
                    "So don't waste your time in here, and learn what you are good at.",
                    " "
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("fish_r").get()? == 1 {
                if ctx.call(Function::CountItem, vec![Val::from(529)])?.number()? > 9 {
                    ctx.lines_as(
                        "Magumagu",
                        args![
                            "Oh! I really appreciate it.",
                            "Right on time!",
                            "I really need them to make a dessert."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Magumagu",
                        args![
                            "Thanks. It's nothing, but",
                            "I will give you my shop's special cuisine, Fish Slice, in return."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.var("fish_r").set(Val::from(0))?;
                    ctx.call(Function::EraseQuest, vec![Val::from(10036)])?;
                    ctx.call(Function::DelItem, vec![Val::from(529), Val::from(10)])?;
                    ctx.call(Function::GetItem, vec![Val::from(544), Val::from(15)])?;
                    ctx.lines_as(
                        "Magumagu",
                        args![
                            "Here's 15 fish slices.",
                            "Please enjoy this food with your friends.",
                            "And come back whenever you miss the flavor of Amatsu cuisine."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Magumagu",
                    args!["Oh man, you didn't prepare", "what I asked for...", "Do not forget what I asked."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Magumagu",
                    args![
                        "10 ^0000FFCandy^000000",
                        "You didn't forget it, right?",
                        "Bring me these supplies, please."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("fish_r").get()? == 2 {
                    if ctx.call(Function::CountItem, vec![Val::from(964)])?.number()? > 9 {
                        ctx.lines_as(
                            "Magumagu",
                            args!["Oh! I really appreciate it.", "Right on time!", "I need them to make a sauce."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Magumagu",
                            args!["Thanks. It is nothing, but", "I will give you two sets of Sushi in return."],
                        )?;
                        ctx.next()?;
                        ctx.var("fish_r").set(Val::from(0))?;
                        ctx.call(Function::EraseQuest, vec![Val::from(10037)])?;
                        ctx.call(Function::DelItem, vec![Val::from(964), Val::from(10)])?;
                        ctx.call(Function::GetItem, vec![Val::from(551), Val::from(20)])?;
                        ctx.lines_as(
                            "Magumagu",
                            args!["Share it with your friends", "and family members.", "Please come again."],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        "Magumagu",
                        args!["Oh man, you didn't prepare", "what I asked for...", "Do not forget what I asked."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Magumagu",
                        args![
                            "10 ^0000FFCrab Shells^000000...",
                            "You didn't forget it, right?",
                            "Bring me these supplies, please."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("fish_r").get()? == 3 {
                    if ctx.call(Function::CountItem, vec![Val::from(961)])?.number()? > 9 {
                        ctx.lines_as(
                            "Magumagu",
                            args!["Oh! I really appreciate it.", "Right on time!", "I was preparing appetizers..."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Magumagu",
                            args!["Thanks. It is nothing but", "I will give you several sets of Sushi in return."],
                        )?;
                        ctx.next()?;
                        ctx.var("fish_r").set(Val::from(0))?;
                        ctx.call(Function::EraseQuest, vec![Val::from(10038)])?;
                        ctx.call(Function::DelItem, vec![Val::from(961), Val::from(10)])?;
                        ctx.call(Function::GetItem, vec![Val::from(551), Val::from(30)])?;
                        ctx.lines_as(
                            "Magumagu",
                            args![
                                "These are for three people,",
                                "so share them with your friends.",
                                "Come back again if you feel like helping out some more."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        "Magumagu",
                        args!["Oh man, you didn't prepare", "what I asked for...", "Do not forget what I asked."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Magumagu",
                        args![
                            "10 ^0000FFConches^000000",
                            "You didn't forget it, right?",
                            "Bring me these supplies, please."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("fish_r").get()? == 4 {
                    if ctx.call(Function::CountItem, vec![Val::from(1023)])?.number()? > 9 {
                        ctx.lines_as(
                            "Magumagu",
                            args!["Oh! I really appreciate it.", "Right on time!", "I was decorating a platter."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Magumagu",
                            args!["Thanks. It is nothing but", "I will give you a large amount of food in return."],
                        )?;
                        ctx.next()?;
                        ctx.var("fish_r").set(Val::from(0))?;
                        ctx.call(Function::EraseQuest, vec![Val::from(10039)])?;
                        ctx.call(Function::DelItem, vec![Val::from(1023), Val::from(10)])?;
                        ctx.call(Function::GetItem, vec![Val::from(544), Val::from(20)])?;
                        ctx.call(Function::GetItem, vec![Val::from(551), Val::from(30)])?;
                        ctx.lines_as(
                            "Magumagu",
                            args!["You can feed your guild", "with this food.", "Come back with a friend."],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        "Magumagu",
                        args!["Oh man, you didn't prepare", "what I asked for...", "Do not forget what I asked."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Magumagu",
                        args![
                            "10 ^0000FFFish Tails^000000",
                            "You didn't forget it, right?",
                            "Bring me these supplies, please."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("fish_r").get()? == 5 {
                    if ctx.call(Function::CountItem, vec![Val::from(736)])?.number()? > 0 {
                        ctx.lines_as(
                            "Magumagu",
                            args!["Oh! I really appreciate it.", "Right on time!", "I didn't have a white platter."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Magumagu",
                            args!["Thanks. It is nothing but", "I will give you two sets of Sushi in return."],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::DelItem, vec![Val::from(736), Val::from(1)])?;
                        ctx.var("fish_r").set(Val::from(0))?;
                        ctx.call(Function::EraseQuest, vec![Val::from(10040)])?;
                        ctx.call(Function::GetItem, vec![Val::from(551), Val::from(20)])?;
                        ctx.lines_as(
                            "Magumagu",
                            args!["Share it with your friends", "and family members.", "Please come again."],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        "Magumagu",
                        args!["Oh man, you didn't prepare", "what I asked for...", "Do not forget what I asked."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Magumagu",
                        args![
                            "I need a white platter:",
                            "1 ^0000FFChina^000000",
                            "You didn't forget it, right?",
                            "Bring it to me, okay?"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("fish_r").get()? == 6 {
                    if ctx.call(Function::CountItem, vec![Val::from(950)])?.number()? > 99 {
                        if ctx.var("ama_sushi").get()? == 2 {
                            ctx.lines_as("Magumagu", args![".............................."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Magumagu",
                                args![
                                    "You are a really kind person.",
                                    "You have brought everything",
                                    "that I've asked you..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Magumagu",
                                args![
                                    "You are not the first foreigner",
                                    "I have encountered. Amatsu is getting more and more tourists.",
                                    "I just wanted to test you.",
                                    " "
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Magumagu",
                                args![
                                    "Your job was just errands but",
                                    "I wanted to know...",
                                    "how you devote yourself,",
                                    "how serious you are about completing your job and not giving up."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Magumagu",
                                args![
                                    "I was thinking,",
                                    "'If there is a person like that,",
                                    "I would give that person everything about cooking that I have mastered"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Magumagu", args!["And.....", "You...", "You are the one."])?;
                            ctx.next()?;
                            ctx.lines_as("Magumagu", args!["I'm old now. No one knows how long I can hold this knife. You have been chosen to inherit my culinary art...", " "])?;
                            ctx.next()?;
                            ctx.call(Function::DelItem, vec![Val::from(950), Val::from(100)])?;
                            ctx.var("ama_sushi").set(Val::from(4))?;
                            ctx.call(Function::CompleteQuest, vec![Val::from(10041)])?;
                            ctx.call(Function::GetItem, vec![Val::from(1144), Val::from(1)])?;
                            ctx.lines_as(
                                "Magumagu",
                                args![
                                    "Here, take my knife.",
                                    "From now on, make fine cuisine with that knife.",
                                    "...Learn how to slice a fish.",
                                    " ",
                                    " "
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Magumagu",
                                args![
                                    "From now on, I will no longer ask you favors and give you fish slices in return....",
                                    "Practice your skill with that knife and teach mainlanders the pleasure of fine cuisine..."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            ctx.lines_as(
                                "Magumagu",
                                args![
                                    "Wow...these are real hearts of",
                                    "mermaid. The legends saying",
                                    "that these could be found in another continent were true...",
                                    " "
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Magumagu",
                                args![
                                    "Gathering all these must have been hard...Well, then.",
                                    "Today, I will use all of my",
                                    "ingredients to make a special cuisine for you!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Magumagu",
                                args!["Give me a moment...", "I shall show you my ^0000FFtrue culinary skill^000000."],
                            )?;
                            ctx.next()?;
                            ctx.call(Function::DelItem, vec![Val::from(950), Val::from(20)])?;
                            ctx.call(Function::GetItem, vec![Val::from(551), Val::from(20)])?;
                            ctx.lines_as("Magumagu", args!["Try these first.", "Made from the freshest ingredients."])?;
                            ctx.next()?;
                            ctx.call(Function::DelItem, vec![Val::from(950), Val::from(20)])?;
                            ctx.call(Function::GetItem, vec![Val::from(544), Val::from(20)])?;
                            ctx.lines_as(
                                "Magumagu",
                                args!["Try these too.", "My shop's fish slices are the best of the best!"],
                            )?;
                            ctx.next()?;
                            ctx.call(Function::DelItem, vec![Val::from(950), Val::from(20)])?;
                            ctx.call(Function::GetItem, vec![Val::from(551), Val::from(20)])?;
                            ctx.lines_as(
                                "Magumagu",
                                args!["Have some more. Don't say no...!", "Isn't it good? Huh? Isn't it good?"],
                            )?;
                            ctx.next()?;
                            ctx.call(Function::DelItem, vec![Val::from(950), Val::from(20)])?;
                            ctx.call(Function::GetItem, vec![Val::from(544), Val::from(20)])?;
                            ctx.lines_as("Magumagu", args!["Haha~! Not done yet!", "It's okay, have some more!"])?;
                            ctx.next()?;
                            ctx.call(Function::DelItem, vec![Val::from(950), Val::from(20)])?;
                            ctx.call(Function::GetItem, vec![Val::from(544), Val::from(10)])?;
                            ctx.call(Function::GetItem, vec![Val::from(551), Val::from(10)])?;
                            ctx.lines_as(
                                "Magumagu",
                                args![
                                    "Take the leftovers.",
                                    "You look stuffed...",
                                    "Share the rest with your friends and family."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.var("ama_sushi").set((ctx.var("ama_sushi").get()? + Val::from(1)))?;
                            ctx.var("fish_r").set(Val::from(0))?;
                            ctx.call(Function::EraseQuest, vec![Val::from(10041)])?;
                            ctx.lines_as(
                                "Magumagu",
                                args![
                                    "Thanks a lot for today! Haha!",
                                    "Come back again when you feel like helping. Take care...!!!"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                    ctx.lines_as(
                        "Magumagu",
                        args!["Oh man, you didn't prepare", "what I asked for...", "Do not forget what I asked."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Magumagu",
                        args![
                            "100 ^0000FFHearts of Mermaid^000000, okay?",
                            "You didn't forget it, right?",
                            "Bring me the supplies, please."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        }
    }
    ctx.lines_as(
        "Magumagu",
        args![
            "Holy cow~",
            "This is a problem.....",
            "There are so many customers, but I can't get all the ingredients that I need..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Magumagu",
        args![
            "This is horrible...",
            "Some of my customers will not get",
            "the chance to eat fine Amatsu cuisine...",
            " "
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Magumagu",
        args![
            "Well, well. Are you a customer?",
            "Welcome. As always, my shop",
            "highly values the freshness of",
            "fish slices. What brings you down here...?"
        ],
    )?;
    ctx.next()?;
    'b1: {
        let subject1 = Val::from(runtime::select_values(
            ctx,
            &[Val::from(
                "I would like to buy Sushi:I would like to buy fish slice:Do you need assistance?:Keep up the good work",
            )],
        )?);
        let mut matched1 = false;
        let no_case1 = !subject1.loosely_equals(&Val::from(1))
            && !subject1.loosely_equals(&Val::from(2))
            && !subject1.loosely_equals(&Val::from(3))
            && !subject1.loosely_equals(&Val::from(4));
        if !matched1 && subject1.loosely_equals(&Val::from(1)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Magumagu",
                args![
                    "Alright...! How many?",
                    "A set of Sushi is 700z. If you",
                    "want just 1 Sushi. It is 74z.",
                    "If you want more, tell me."
                ],
            )?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("700z A set!:74z A piece!:I will try it later")])? {
                1 => {
                    if ctx.var("Zeny").get()?.number()? < 700 {
                        ctx.lines_as(
                            "Magumagu",
                            args![
                                "Oh man, you don't have enough money.",
                                "If you want to eat delicious fish slices, you better bring more money."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(700))?))?;
                    ctx.call(Function::GetItem, vec![Val::from(551), Val::from(10)])?;
                    ctx.lines_as(
                        "Magumagu",
                        args!["There you go. If you like the taste, please order some more."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    if ctx.var("Zeny").get()?.number()? < 74 {
                        ctx.lines_as(
                            "Magumagu",
                            args![
                                "Oh man, you don't have enough money.",
                                "If you want to eat delicious Sushi,",
                                "you better bring more money."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(74))?))?;
                    ctx.call(Function::GetItem, vec![Val::from(551), Val::from(1)])?;
                    ctx.lines_as(
                        "Magumagu",
                        args!["There you go. If you like the taste, please order some more."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                3 => {
                    ctx.lines_as(
                        "Magumagu",
                        args![
                            "Up to you. My Sushi is",
                            "the best of the best! The taste",
                            "and freshness are the best in the",
                            "world. If you have time, try my Sushi."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        }
        if !matched1 && subject1.loosely_equals(&Val::from(2)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Magumagu",
                args![
                    "Alright...! How many?",
                    "A set of fish slices is 350z.",
                    "1 fish slice is 37z.",
                    "If you want more, tell me."
                ],
            )?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("350z A set!:37z A piece!:I will try it later")])? {
                1 => {
                    if ctx.var("Zeny").get()?.number()? < 350 {
                        ctx.lines_as(
                            "Magumagu",
                            args![
                                "Oh man, you don't have enough money.",
                                "If you want to eat delicious fish slices, you better bring more money."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(350))?))?;
                    ctx.call(Function::GetItem, vec![Val::from(544), Val::from(10)])?;
                    ctx.lines_as(
                        "Magumagu",
                        args!["There you go. If you like the taste, please order some more."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    if ctx.var("Zeny").get()?.number()? < 37 {
                        ctx.lines_as(
                            "Magumagu",
                            args![
                                "Oh man, you don't have enough money.",
                                "If you want to eat delicious fish slice, you better bring more money."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(37))?))?;
                    ctx.call(Function::GetItem, vec![Val::from(544), Val::from(1)])?;
                    ctx.lines_as(
                        "Magumagu",
                        args!["There you go. If you like the taste, please order some more."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                3 => {
                    ctx.lines_as(
                        "Magumagu",
                        args![
                            "Up to you. My fish slice is",
                            "the best of the best! The taste",
                            "and freshness are the best in the",
                            "world. If you have time, try my fish slices."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        }
        if !matched1 && subject1.loosely_equals(&Val::from(3)) {
            matched1 = true;
        }
        if matched1 {
            if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_NOVICE")?) {
                ctx.lines_as(
                    "Magumagu",
                    args![
                        "Haha~ YOU are the one who needs",
                        "assistance! Help others",
                        "when you can take care of yourself.",
                        " "
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Magumagu",
                    args![
                        "Some help would be great~!!",
                        "I was worried because we always",
                        "run out of ingredients...",
                        "Will you do me a favor?",
                        "I will reward you."
                    ],
                )?;
                ctx.next()?;
                l_fish_m1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?;
                if (l_fish_m1.clone() == 1 || l_fish_m1.clone() == 2) {
                    ctx.var("fish_r").set(Val::from(1))?;
                    ctx.call(Function::SetQuest, vec![Val::from(10036)])?;
                    ctx.lines_as(
                        "Magumagu",
                        args![
                            "I need some candies to make",
                            "a dessert for the customers.",
                            "Bring me ^0000FF10 Candy^000000.",
                            "It's not hard, right?",
                            " "
                        ],
                    )?;
                } else {
                    if (l_fish_m1.clone() == 3 || l_fish_m1.clone() == 4) {
                        ctx.var("fish_r").set(Val::from(2))?;
                        ctx.call(Function::SetQuest, vec![Val::from(10037)])?;
                        ctx.lines_as(
                            "Magumagu",
                            args![
                                "I need some crab shells to make",
                                "a sauce for my cuisine...",
                                "If you bring me ^0000FF10 Crab Shells^000000,",
                                "I will reward you.",
                                "Okay?"
                            ],
                        )?;
                    } else if (l_fish_m1.clone() == 5 || l_fish_m1.clone() == 6) {
                        ctx.var("fish_r").set(Val::from(3))?;
                        ctx.call(Function::SetQuest, vec![Val::from(10038)])?;
                        ctx.lines_as(
                            "Magumagu",
                            args![
                                "I have used all of my conches.",
                                "I need it to make an appetizer...",
                                "Please bring me ^0000FF10 Conches^000000.",
                                "It is hard to find conches around here...",
                                "Please do me this favor."
                            ],
                        )?;
                    } else if (l_fish_m1.clone() == 7 || l_fish_m1.clone() == 8) {
                        ctx.var("fish_r").set(Val::from(4))?;
                        ctx.call(Function::SetQuest, vec![Val::from(10039)])?;
                        ctx.lines_as(
                            "Magumagu",
                            args![
                                "Decorative fish tails are out of",
                                "stock. This is urgent...",
                                "Please bring me ^0000FF10 fish tails^000000.",
                                "They are always missing when I need them badly...",
                                "Please do me this favor."
                            ],
                        )?;
                    } else if l_fish_m1.clone() == 9 {
                        ctx.var("fish_r").set(Val::from(5))?;
                        ctx.call(Function::SetQuest, vec![Val::from(10040)])?;
                        ctx.lines_as(
                            "Magumagu",
                            args![
                                "We are missing a platter to serve",
                                "fish slices to customers.",
                                "This is horrible...",
                                "Will you buy me some fine porcelain ^0000FFChina^000000?",
                                "I can't just put food anywhere..."
                            ],
                        )?;
                    } else if ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])? == 1 {
                        ctx.var("fish_r").set(Val::from(6))?;
                        ctx.call(Function::SetQuest, vec![Val::from(10041)])?;
                        ctx.lines_as(
                            "Magumagu",
                            args![
                                "Don't ask me why...",
                                "But, I need something special...",
                                "^0000FF100 Hearts of Mermaid^000000.....",
                                "I know that it sounds impossible, but it is really important to me..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Magumagu",
                            args!["Don't ask any questions", "about my request...", "Just keep what I want in mind."],
                        )?;
                    } else {
                        ctx.var("fish_r").set(Val::from(5))?;
                        ctx.call(Function::SetQuest, vec![Val::from(10040)])?;
                        ctx.lines_as(
                            "Magumagu",
                            args![
                                "We are missing a platter to serve",
                                "fish slices to customers.",
                                "This is horrible...",
                                "Will you buy me some fine porcelain ^0000FFChina^000000? I can't just put food anywhere..."
                            ],
                        )?;
                    }
                }
                ctx.next()?;
                ctx.lines_as(
                    "Magumagu",
                    args![
                        "Well, good luck to you...",
                        "I will be waiting for you.",
                        "Don't forget what I asked..."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
        if !matched1 && subject1.loosely_equals(&Val::from(4)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Magumagu",
                args![
                    "Haha, you won't forget the flavor of my fish slice once you've",
                    "tasted it. If you have time, try my fish slices.",
                    " "
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    Ok(Val::from(0))
}

pub fn sushi_master_ama(ctx: &Ctx) -> Script {
    sushi_master_ama_body(ctx, Vec::new()).map(|_| ())
}

fn gate_soldier_ama1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Gate Soldier",
        args![
            "This is the great palace,",
            "Toukoujyo.",
            "Locals are prohibited",
            "from entering..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Gate Soldier",
        args![
            "Your attire looks unfamiliar.",
            "Are you from another continent?",
            "The lord has granted entrance to",
            "tourists, so you can go in.",
            " "
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn gate_soldier_ama1(ctx: &Ctx) -> Script {
    gate_soldier_ama1_body(ctx, Vec::new()).map(|_| ())
}

fn gate_soldier_ama2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Gate Soldier",
        args![
            "Are you from Midgard?",
            "Welcome to Amatsu.",
            "Visit our lord in Chun-Su-Gak",
            "when you go in.",
            " "
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Gate Soldier",
        args![
            "He is really kind.",
            "He invested in various fields for",
            "Amatsu, and intercontinental trade was his idea."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn gate_soldier_ama2(ctx: &Ctx) -> Script {
    gate_soldier_ama2_body(ctx, Vec::new()).map(|_| ())
}

fn gate_soldier_ama3_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Gate Soldier",
        args![
            "Welcome. The lord of the palace",
            "has specially allowed guests",
            "from other continents."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Gate Soldier",
        args!["Look around and take your time.", "Please enjoy your stay."],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn gate_soldier_ama3(ctx: &Ctx) -> Script {
    gate_soldier_ama3_body(ctx, Vec::new()).map(|_| ())
}

fn gate_soldier_ama4_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Gate Soldier",
        args![
            "The lord is really nice guy.",
            "Who would know that our town was",
            "once a small village?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Gate Soldier",
        args![
            "I thank him for hiring me.",
            "These days, he seems to have",
            "troubles on his mind.",
            "Lately, he's been looking pretty gloomy.",
            " "
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn gate_soldier_ama4(ctx: &Ctx) -> Script {
    gate_soldier_ama4_body(ctx, Vec::new()).map(|_| ())
}

fn gate_soldier_ama5_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.mes("[Gate Soldier]")?;
    if ctx.var("event_amatsu").get()? == 0 {
        ctx.lines(args![
            "You can't enter here.",
            "The mother of our lord is resting in here. She needs her rest."
        ])?;
        ctx.next()?;
        ctx.var("event_amatsu").set(Val::from(1))?;
        ctx.call(Function::SetQuest, vec![Val::from(8131)])?;
        ctx.lines_as(
            "Gate Soldier",
            args![
                "She has been sick for months.",
                "That is why she is staying in here.",
                "It's a pretty fearsome sickness..."
            ],
        )?;
    } else if ctx.var("event_amatsu").get()? == 1 {
        ctx.lines(args![
            "She should regain her health...",
            "But still, my lord is worrying so much.",
            " "
        ])?;
    } else if ctx.var("event_amatsu").get()? == 5 {
        ctx.lines(args!["Augh! I was suprised by that loud sound.", "What happened? Huh?", " "])?;
    } else if ctx.var("event_amatsu").get()? == 6 {
        ctx.lines(args![
            "The mother of our lord hasn't",
            "fully recovered her health.",
            "Still, she is better than before.",
            " "
        ])?;
    } else {
        ctx.lines(args![
            "If you are sent by my lord,",
            "it is okay to enter...",
            "But you wouldn't be able to cure",
            "her. Many others tried and failed."
        ])?;
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn gate_soldier_ama5(ctx: &Ctx) -> Script {
    gate_soldier_ama5_body(ctx, Vec::new()).map(|_| ())
}

fn gate_soldier_ama6_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.mes("[Gate Soldier]")?;
    if ctx.var("event_amatsu").get()? == 0 {
        ctx.lines(args![
            "You can't enter here.",
            "The mother of our lord is resting in here.",
            " "
        ])?;
        ctx.next()?;
        ctx.call(Function::SetQuest, vec![Val::from(8131)])?;
        ctx.var("event_amatsu").set(Val::from(1))?;
        ctx.lines_as(
            "Gate Soldier",
            args!["She has been sick for months.", "That is why she is staying in here.", "..."],
        )?;
    } else if ctx.var("event_amatsu").get()? == 1 {
        ctx.lines(args![
            "Please, be quiet.",
            "The mother of the lord is staying in here. She needs to relax."
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Gate Soldier",
            args!["Why is this happening", "to my kind lord...", "How can it be? *Sob*..."],
        )?;
    } else if ctx.var("event_amatsu").get()? == 5 {
        ctx.lines(args![
            "Eh? Something was flying",
            "in the sky... You didn't see? Ugh.",
            "That sound suprised me."
        ])?;
    } else if ctx.var("event_amatsu").get()? == 6 {
        ctx.lines(args![
            "Now our lord is relieved.",
            "We were so worrying about it so much.",
            "Now, it is okay... *sob*."
        ])?;
    } else {
        ctx.lines(args![
            "Oh...man. People from the other",
            "continents are all doctors!",
            "How many doctors have come to visit?! I can't even count anymore!"
        ])?;
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn gate_soldier_ama6(ctx: &Ctx) -> Script {
    gate_soldier_ama6_body(ctx, Vec::new()).map(|_| ())
}

fn soldier_ama1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Ichiro",
        args!["Welcome.", "Our lord prepared guest rooms", "for travelers like you."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Ichiro",
        args![
            "If you are having any problems,",
            "tell me. I would appreciate it if you talk to our lord of the",
            "palace. This is all provided by him."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn soldier_ama1(ctx: &Ctx) -> Script {
    soldier_ama1_body(ctx, Vec::new()).map(|_| ())
}

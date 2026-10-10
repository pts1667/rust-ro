use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum Room21WarpStep {
    Start,
    OnInit,
    OnDisable,
    OnEnable,
    OnTouch,
}

fn room2_1_warp_run(ctx: &Ctx, mut step: Room21WarpStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Room21WarpStep::Start => {
                step = Room21WarpStep::OnInit;
                continue 'machine;
            }
            Room21WarpStep::OnInit => {
                step = Room21WarpStep::OnDisable;
                continue 'machine;
            }
            Room21WarpStep::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("#room2_1_warp")])?;
                return Err(Stop::End);
            }
            Room21WarpStep::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("#room2_1_warp")])?;
                return Err(Stop::End);
            }
            Room21WarpStep::OnTouch => {
                ctx.call(Function::Warp, vec![Val::from("que_job01"), Val::from(52), Val::from(50)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn room2_1_warp(ctx: &Ctx) -> Script {
    room2_1_warp_run(ctx, Room21WarpStep::Start, Vec::new()).map(|_| ())
}

pub fn room2_1_warp_oninit(ctx: &Ctx) -> Script {
    room2_1_warp_run(ctx, Room21WarpStep::OnInit, Vec::new()).map(|_| ())
}

pub fn room2_1_warp_ondisable(ctx: &Ctx) -> Script {
    room2_1_warp_run(ctx, Room21WarpStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn room2_1_warp_onenable(ctx: &Ctx) -> Script {
    room2_1_warp_run(ctx, Room21WarpStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn room2_1_warp_ontouch(ctx: &Ctx) -> Script {
    room2_1_warp_run(ctx, Room21WarpStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Room22WarpStep {
    Start,
    OnInit,
    OnDisable,
    OnEnable,
    OnTouch,
}

fn room2_2_warp_run(ctx: &Ctx, mut step: Room22WarpStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Room22WarpStep::Start => {
                step = Room22WarpStep::OnInit;
                continue 'machine;
            }
            Room22WarpStep::OnInit => {
                step = Room22WarpStep::OnDisable;
                continue 'machine;
            }
            Room22WarpStep::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("#room2_2_warp")])?;
                return Err(Stop::End);
            }
            Room22WarpStep::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("#room2_2_warp")])?;
                return Err(Stop::End);
            }
            Room22WarpStep::OnTouch => {
                ctx.call(Function::Warp, vec![Val::from("que_job01"), Val::from(52), Val::from(50)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn room2_2_warp(ctx: &Ctx) -> Script {
    room2_2_warp_run(ctx, Room22WarpStep::Start, Vec::new()).map(|_| ())
}

pub fn room2_2_warp_oninit(ctx: &Ctx) -> Script {
    room2_2_warp_run(ctx, Room22WarpStep::OnInit, Vec::new()).map(|_| ())
}

pub fn room2_2_warp_ondisable(ctx: &Ctx) -> Script {
    room2_2_warp_run(ctx, Room22WarpStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn room2_2_warp_onenable(ctx: &Ctx) -> Script {
    room2_2_warp_run(ctx, Room22WarpStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn room2_2_warp_ontouch(ctx: &Ctx) -> Script {
    room2_2_warp_run(ctx, Room22WarpStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum QueJob01Room21OutStep {
    Start,
    OnTouch,
}

fn que_job01_room2_1_out_run(ctx: &Ctx, mut step: QueJob01Room21OutStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            QueJob01Room21OutStep::Start => {
                step = QueJob01Room21OutStep::OnTouch;
                continue 'machine;
            }
            QueJob01Room21OutStep::OnTouch => {
                ctx.call(Function::Warp, vec![Val::from("que_job01"), Val::from(52), Val::from(50)])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from(" #room2timer::OnStop")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn que_job01_room2_1_out(ctx: &Ctx) -> Script {
    que_job01_room2_1_out_run(ctx, QueJob01Room21OutStep::Start, Vec::new()).map(|_| ())
}

pub fn que_job01_room2_1_out_ontouch(ctx: &Ctx) -> Script {
    que_job01_room2_1_out_run(ctx, QueJob01Room21OutStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum QueJob01Room22OutStep {
    Start,
    OnTouch,
}

fn que_job01_room2_2_out_run(ctx: &Ctx, mut step: QueJob01Room22OutStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            QueJob01Room22OutStep::Start => {
                step = QueJob01Room22OutStep::OnTouch;
                continue 'machine;
            }
            QueJob01Room22OutStep::OnTouch => {
                ctx.call(Function::Warp, vec![Val::from("que_job01"), Val::from(52), Val::from(50)])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from(" #room2timer::OnStop")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn que_job01_room2_2_out(ctx: &Ctx) -> Script {
    que_job01_room2_2_out_run(ctx, QueJob01Room22OutStep::Start, Vec::new()).map(|_| ())
}

pub fn que_job01_room2_2_out_ontouch(ctx: &Ctx) -> Script {
    que_job01_room2_2_out_run(ctx, QueJob01Room22OutStep::OnTouch, Vec::new()).map(|_| ())
}

fn rin_moc_room2_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(714), Val::from(1)])? == 0 {
        ctx.mes("- You cannot proceed with the quest when you're carrying too many items with you. -")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("mao_morocc2").get()? == 5 {
        if ctx.var("mao_request").get()?.number()? > 103 {
            ctx.lines_as(
                "Rin",
                args![
                    ((Val::from("...") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("...?")),
                    "It's been a long time. What's going on?"
                ],
            )?;
            ctx.call(Function::Cutin, vec![Val::from("moc2_rin02"), Val::from(2)])?;
        } else {
            ctx.lines_as("Rin", args!["Who are you? Only authorized personnel can enter this area."])?;
            ctx.call(Function::Cutin, vec![Val::from("moc2_rin01"), Val::from(2)])?;
        }
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Mr. Kidd sent me.:Just passing by.")])? {
            1 => {
                ctx.call(Function::Cutin, vec![Val::from("moc2_rin02"), Val::from(2)])?;
                ctx.lines_as(
                    "Rin",
                    args![
                        "...?",
                        "...Ah.. Ahh, I see.",
                        "I haven't contacted him for a while, and so he's sent you to me...",
                        "*Sigh*.. *Cough*."
                    ],
                )?;
                ctx.next()?;
                ctx.call(Function::Cutin, vec![Val::from("moc2_rin01"), Val::from(2)])?;
                ctx.lines_as(
                    "Rin",
                    args![
                        "As you see, I've been feeling under the weather. I'm in no condition to contact him.",
                        "...Let me think. Where I should start..?"
                    ],
                )?;
                ctx.next()?;
                if ((ctx.var("mao_request").get()?.number()? > 25 && ctx.var("mao_request").get()?.number()? < 31)
                    || (ctx.var("mao_request").get()?.number()? > 125 && ctx.var("mao_request").get()?.number()? < 129))
                {
                    ctx.mes("I don't know if you know this, but something catastrophic happened in this town before Satan Morocc resurrected himself.")?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Rin",
                        args!["R, otherwise known as 'Rayan,' performed a ritual to break Satan Morocc's seal."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Rin",
                        args![
                            "We were able to gather a group of people and stop him from completing the ritual,",
                            "and although the seal wasn't broken, he succeeded in weakening it."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Rin",
                        args![
                            "Soon after we captured him, an army of monsters attacked this town",
                            "as Satan Morocc's angry roars echoed in the air."
                        ],
                    )?;
                    ctx.call(Function::Cutin, vec![Val::from("moc2_rin04"), Val::from(2)])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Rin",
                        args![
                            "R, otherwise known as Rayan, is responsible for these series of events.",
                            "He caused everything that's happening now."
                        ],
                    )?;
                    ctx.next()?;
                } else {
                    ctx.lines_as("Rin", args!["I don't know if you know this, but something catastrophic happened in this town before Satan Morocc resurrected himself."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Rin",
                        args!["R, otherwise known as 'Rayan,' performed a ritual to break Satan Morocc's seal."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Rin",
                        args![
                            "We were able to gather a group of people and stop him from completing the ritual,",
                            "and although the seal wasn't broken, he succeeded in weakening it."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Rin",
                        args![
                            "Soon after we captured him, an army of monsters attacked this town",
                            "as Satan Morocc's angry roars echoed in the air."
                        ],
                    )?;
                    ctx.call(Function::Cutin, vec![Val::from("moc2_rin04"), Val::from(2)])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Rin",
                        args![
                            "R, otherwise known as Rayan, is responsible for these series of events.",
                            "He caused everything that's happening now."
                        ],
                    )?;
                    ctx.next()?;
                }
                ctx.lines_as(
                    "Rin",
                    args![
                        "Our job is primarily investigating Satan Morocc's whereabouts",
                        "as well as the relation between the space gap and Satan Morocc..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Rin",
                    args![
                        "We, however, couldn't let Rayan get away with what he's done",
                        "I've been focusing on pursuing Rayan."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Rin",
                    args![
                        "If we capture him, we might also find out some information about Satan Morocc:",
                        "we might be able to learn where he tried to summon Satan Morocc, and where we should send Satan Morocc back to..."
                    ],
                )?;
                ctx.next()?;
                ctx.call(Function::Cutin, vec![Val::from("moc2_rin03"), Val::from(2)])?;
                ctx.lines_as(
                    "Rin",
                    args![
                        "I really want to know how the space gap is linked to Satan Morocc...",
                        "*Pant Pant*... ..."
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("And then what?:Don't strain yourself.")])? {
                    1 => {
                        ctx.call(Function::Cutin, vec![Val::from("moc2_rin01"), Val::from(2)])?;
                        ctx.lines_as(
                            "Rin",
                            args!["I'm sorry, but I'm not yet fully recovered... I need to speak slowly. Please bear with me."],
                        )?;
                        ctx.next()?;
                    }
                    2 => {
                        ctx.call(Function::Cutin, vec![Val::from("moc2_rin01"), Val::from(2)])?;
                        ctx.lines_as("Rin", args!["Thank you for your kindness.", "...*Pant Pant*..."])?;
                        ctx.next()?;
                    }
                    _ => {}
                }
                ctx.lines_as(
                    "Rin",
                    args![
                        "I received these injuries from Rayan and his gang. I finally located them after a long stakeout and pursuit...",
                        "......... ..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Rin", args!["Yes... I need to write this down..."])?;
                ctx.next()?;
                ctx.mes("- Mumbling something, she took down notes in an old notebook as she spoke. -")?;
                ctx.next()?;
                ctx.lines_as(
                    "Rin",
                    args![
                        "The Dandelions.. They're Rayan's puppets.",
                        "I was ambushed by them in a mountain far from a village..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Rin",
                    args![
                        "When I woke up, I was here.",
                        "Since then, my vision's been blurry. I guess that must have been caused by my injuries..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Rin",
                    args![
                        "How'd you like to conduct an investigation around that area? We might find something..",
                        "Like important clues or where they've headed..."
                    ],
                )?;
                ctx.var("mao_morocc2").set(Val::from(6))?;
                ctx.next()?;
                ctx.lines_as(
                    "Rin",
                    args!["^4d4dffIf you're interested in helping me, I'll continue talking.^000000"],
                )?;
                ctx.close_window()?;
            }
            2 => {
                ctx.call(Function::Cutin, vec![Val::from("moc2_rin03"), Val::from(2)])?;
                ctx.lines_as(
                    "Rin",
                    args![
                        "Did you just come by for no reason?",
                        "...God, I can't believe just let you come here!",
                        "Grr.. I need to teach them a lesson. Why can't they let me rest for God's sake!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Rin",
                    args![
                        "I'm sorry, but I'm extremely tired and cranky.",
                        "If you don't have anything important to tell me, then I'd like you to leave."
                    ],
                )?;
                ctx.close_window()?;
            }
            _ => {}
        }
    } else if ctx.var("mao_morocc2").get()? == 6 {
        ctx.call(Function::Cutin, vec![Val::from("moc2_rin01"), Val::from(2)])?;
        ctx.lines_as(
            "Rin",
            args![
                "So, are you interested in helping me?",
                "I remember the location was... Southwest from a village called Hugel.",
                "Do you know where to find Abyss Lake, where the Dragons reside?."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Rin",
            args![
                "^4d4dffWhen you pass Abyss Lake and head west,^000000 you'll arrive at stiff mountains.",
                "I was ambushed somewhere ^4d4dffsouth of the second hill from the top of the mountain^000000"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Rin",
            args![
                "They ganged up on me, but I put up a good fight...",
                "I'm sure I at least made them drop something..",
                "Please try to find something there that might serve as a clue."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Rin",
            args![
                "While you're gone, I'll be preparing a document to send to Mr. Kidd.",
                "Be safe, and look out for yourself, okay?"
            ],
        )?;
        ctx.var("mao_morocc2").set(Val::from(7))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(7015), Val::from(7016)])?;
        ctx.close_window()?;
    } else if ctx.var("mao_morocc2").get()? == 7 {
        ctx.call(Function::Cutin, vec![Val::from("moc2_rin01"), Val::from(2)])?;
        ctx.lines_as(
            "Rin",
            args![
                "I remember the location was...",
                "somewhere south of the second hill of a mountain west from Abyss Lake."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Rin",
            args![
                "What are you looking at?",
                "How do I remember the location so clearly?",
                "It's the last location where I searched for Rayan. How can I not remember?"
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("moc2_rin02"), Val::from(2)])?;
        ctx.lines_as("Rin", args!["Don't underestimate me; I'm an elite member of my guild."])?;
        ctx.close_window()?;
    } else if ctx.var("mao_morocc2").get()? == 8 {
        ctx.call(Function::Cutin, vec![Val::from("moc2_rin02"), Val::from(2)])?;
        ctx.lines_as("Rin", args!["Have you found anything?"])?;
        ctx.next()?;
        ctx.lines(args![
            "- You told her you couldn't find anything regarding their whereabouts,",
            "but you found the ''Bloody Crystal of Darkness'' in the bushes."
        ])?;
        ctx.next()?;
        if ctx.call(Function::CountItem, vec![Val::from(6027)])?.number()? > 0 {
            ctx.call(Function::Cutin, vec![Val::from("moc2_rin01"), Val::from(2)])?;
            ctx.lines_as(
                "Rin",
                args![
                    "..This is...it.",
                    "You can feel the evil spirit just by looking at it.",
                    "This...there's no doubt. It's the same kind of crystal as the ones found on Satan Morocc and his minions."
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::Cutin, vec![Val::from("moc2_rin04"), Val::from(2)])?;
            ctx.lines_as(
                "Rin",
                args![
                    "But...this crystal...left there..",
                    "..Was it them who dropped it?",
                    "So it seems Rayan and Satan Morocc...there really is something going on between those two."
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::Cutin, vec![Val::from("moc2_rin01"), Val::from(2)])?;
            ctx.lines_as(
                "Rin",
                args![
                    "You've worked hard. Thank you.",
                    "Ha...",
                    "Just lying here, not being able to do anything...it's so frustrating."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Rin",
                args![
                    "Here, take this journal to Mr. Kidd.",
                    "It should've been done a while ago.",
                    "Oh, bring this crystal to him as well."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Rin",
                args!["Ah..the journal..Yes, I should record this incident onto the journal."],
            )?;
            ctx.next()?;
            ctx.mes("......")?;
            ctx.next()?;
            ctx.lines(args!["......", "......"])?;
            ctx.next()?;
            ctx.call(Function::Cutin, vec![Val::from("moc2_rin02"), Val::from(2)])?;
            ctx.lines_as(
                "Rin",
                args![
                    "All done. Here you go.",
                    "Here's the journal and the crystal.",
                    "Take them to Mr. Kidd for me."
                ],
            )?;
            ctx.next()?;
            ctx.lines(args![
                "- You received the journal from Rin.",
                "Instead of a journal, it's more like a bunch of documents being exchanged. -"
            ])?;
            ctx.var("mao_morocc2").set(Val::from(9))?;
            ctx.call(Function::GetItem, vec![Val::from(6029), Val::from(1)])?;
            ctx.call(Function::ChangeQuest, vec![Val::from(7017), Val::from(7018)])?;
            ctx.close_window()?;
        } else {
            ctx.call(Function::Cutin, vec![Val::from("moc2_rin01"), Val::from(2)])?;
            ctx.lines_as("Rin", args!["..A bloody Crystal of the Darkness?", "Can you show it to me?"])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "Oh, I don't have it with me. I'll go bring it right away.",
                    "^4d4dff(Wait, what did I do with it? I didn't sell it or anything, did I?)^000000"
                ],
            )?;
            ctx.close_window()?;
        }
    } else if ctx.var("mao_morocc2").get()? == 9 {
        ctx.call(Function::Cutin, vec![Val::from("moc2_rin03"), Val::from(2)])?;
        ctx.lines_as(
            "Rin",
            args![
                "Hurry up; deliver the journal and crystal to Mr. Kidd.",
                "That's your job, isn't it?",
                "*Pant* I need to rest now.."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Rin", args!["My eyes are blurry...", "*Pant Pant*..."])?;
        ctx.close_window()?;
    } else {
        ctx.call(Function::Cutin, vec![Val::from("moc2_rin03"), Val::from(2)])?;
        ctx.lines_as("Rin", args!["..Argh.. I'm still recovering .. I'm not supposed to move~"])?;
        ctx.close_window()?;
    }
    ctx.call(Function::Cutin, vec![Val::from("moc2_rin03"), Val::from(255)])?;
    return Err(Stop::End);
}

pub fn rin_moc_room2_1(ctx: &Ctx) -> Script {
    rin_moc_room2_1_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum RinMocRoom22Step {
    Start,
    OnInit,
    OnDisable,
    OnEnable,
}

fn rin_moc_room2_2_run(ctx: &Ctx, mut step: RinMocRoom22Step, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_pattern_s = Val::from("");
    let mut l_rotto = Val::from(0);
    'machine: loop {
        match step {
            RinMocRoom22Step::Start => {
                if ctx.call(Function::CheckWeight, vec![Val::from(714), Val::from(1)])? == 0 {
                    ctx.mes("- You cannot proceed with the quest when you're carrying too many items with you. -")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ctx.var("mao_morocc2").get()? == 21 {
                    if (ctx.call(Function::CountItem, vec![Val::from(6029)])?.number()? > 0
                        && ctx.call(Function::CountItem, vec![Val::from(6027)])?.number()? > 0)
                    {
                        ctx.call(Function::Cutin, vec![Val::from("moc2_rin03"), Val::from(2)])?;
                        ctx.lines_as(
                            "Rin",
                            args!["Hey, you've come back at just the right time.", "Look at him; this is ridiculous."],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::Cutin, vec![Val::from("moc2_dan02"), Val::from(0)])?;
                        ctx.lines_as(
                            "Rayan",
                            args![
                                "I know what I did is unforgivable, no matter how good my reason might be.",
                                "..But... *Sigh*... It's too late to regret it now, isn't it?"
                            ],
                        )?;
                        ctx.next()?;
                        let choice = runtime::select_values(ctx, &[Val::from("Who is this?")])?;
                        ctx.var("@menu").set(choice)?;
                        ctx.call(Function::Cutin, vec![Val::from("moc2_rin01"), Val::from(2)])?;
                        ctx.lines_as("Rin", args!["Don't you know? He's Rayan Moore who dared to release Satan Morocc, deceive everyone, and bring havoc to this world.", "And you know what?"])?;
                        ctx.next()?;
                        ctx.lines_as("Rin", args!["Not only that, he slaughtered his own comrades without mercy!"])?;
                        ctx.next()?;
                        ctx.call(Function::Cutin, vec![Val::from("moc2_dan02"), Val::from(0)])?;
                        ctx.lines_as(
                            "Rayan",
                            args![
                                "...Waaah!",
                                "..I.. I.. I killed them with my own hands!",
                                "*Sob*..",
                                "I'm.. I'm sorry.."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines(args![
                            "- Rayan is shivering in fear.",
                            "He still hasn't gotten over the fact that he killed all of his comrades.-"
                        ])?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["What is going on?", "He doesn't look like the Rayan I know."],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::Cutin, vec![Val::from("moc2_rin03"), Val::from(2)])?;
                        ctx.lines_as(
                            "Rin",
                            args![
                                "I don't know. This Rayan seems to be a completely different individual than the one we know.",
                                "I guess this is what they call Multiple Personality Disorder."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Rin",
                            args![
                                "This Rayan isn't the Rayan Moore that plotted the conspiracy,",
                                "but another personality hidden inside of him."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::Cutin, vec![Val::from("moc2_rin01"), Val::from(2)])?;
                        ctx.lines_as("Rin", args!["Hey, calm down. Why don't you speak slowly?"])?;
                        ctx.next()?;
                        ctx.call(Function::Cutin, vec![Val::from("moc2_dan02"), Val::from(0)])?;
                        ctx.lines_as(
                            "Rayan",
                            args![
                                "..It was me.. I was only going to pretend to break the seal of Satan Morocc...",
                                "Don't you see? I meant for it to just be an act! I was going to throw the town in panic..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Rayan",
                            args![
                                "..That would be my chance to spread the teachings of Freya, our messiah.",
                                "Then something went wrong..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::Cutin, vec![Val::from("moc2_rin04"), Val::from(2)])?;
                        ctx.lines_as("Rin", args!["What?", "Man, how can you be so stupid?!", "*Sigh* Continue."])?;
                        ctx.next()?;
                        ctx.call(Function::Cutin, vec![Val::from("moc2_dan02"), Val::from(0)])?;
                        ctx.lines_as(
                            "Rayan",
                            args![
                                "I was studying some documents for my plan, and then I found something by accident.",
                                "That was when I became a completely different person.. *Sob*"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Rayan",
                            args![
                                "..My plan.. It was like I wasn't myself anymore. I wanted to release Satan Morocc for real.",
                                "I kidnapped children.. Threatened my comrades."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Rayan",
                            args![
                                "..I held the true ritual to break the seal and release Satan Morocc...",
                                "Wah.. My.. My head hurts.. This.. No, you can't.. Waaah!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Rayan",
                            args!["..Grr.. You.. You foolish weaklings..", "..I shall kill you.. No... Nooo!"],
                        )?;
                        ctx.next()?;
                        ctx.lines(args![
                            "- Suddenly Rayan started talking nonsense and then pulled his hair out in anguish.",
                            "It seems as if he is trying to resist the evil personality within him. -"
                        ])?;
                        ctx.next()?;
                        ctx.call(Function::Cutin, vec![Val::from("moc2_dan02"), Val::from(255)])?;
                        match runtime::select_values(ctx, &[Val::from("Smack his head.:Punch his stomach.:Slap his face.")])? {
                            1 => {
                                ctx.call(
                                    Function::NpcSpecialEffect,
                                    vec![ctx.constant("EF_HIT2")?, ctx.constant("AREA")?, Val::from("Rayan#moc_room2_2")],
                                )?;
                                ctx.mes("- Wanting to stop Rayan from going berserk, you smacked the back of his head with the journal. Rayan staggered and then fell to the ground. Nice job! -")?;
                                ctx.next()?;
                            }
                            2 => {
                                ctx.call(
                                    Function::NpcSpecialEffect,
                                    vec![ctx.constant("EF_HIT4")?, ctx.constant("AREA")?, Val::from("Rayan#moc_room2_2")],
                                )?;
                                ctx.mes("- Wanting to stop Rayan from going berserk, you sucker punched him in the 'ole bread basket. Rayan staggered and then fell to the ground...! -")?;
                                ctx.next()?;
                            }
                            3 => {
                                ctx.call(
                                    Function::NpcSpecialEffect,
                                    vec![ctx.constant("EF_HIT1")?, ctx.constant("AREA")?, Val::from("Rayan#moc_room2_2")],
                                )?;
                                ctx.mes("- Wanting to stop Rayan from going berserk, you slapped his face with the journal. Rayan staggered and then fell onto the ground. Wah! -")?;
                                ctx.next()?;
                            }
                            _ => {}
                        }
                        ctx.mes("- By your sudden violent move, the Bloody Crystal of Darkness flew out of your jacket, and rolled on the ground toward Rayan. -")?;
                        ctx.next()?;
                        ctx.call(Function::Cutin, vec![Val::from("moc2_dan02"), Val::from(0)])?;
                        ctx.lines_as("Rayan", args!["Ahh....", "....? Isn't.. Isn't this?!...."])?;
                        ctx.call(
                            Function::NpcSpecialEffect,
                            vec![ctx.constant("EF_POISON")?, ctx.constant("AREA")?, Val::from("Rayan#moc_room2_2")],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Rin", args!["Wah! What's going on?", "Why is it suddenly..?!"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["Is this a time-space phenomenon?!", "Why..? There's no gap here..?"],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::Cutin, vec![Val::from("moc2_dan01"), Val::from(1)])?;
                        ctx.lines_as(
                            "Rayan",
                            args![
                                "Haha.. Muhahaha!",
                                "Muhahahahaha!!",
                                "He's come.. My master has arrived!!",
                                "As you command, master. I'm here to serve you!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Voice from Rayan",
                            args![
                                "^FF0000You insignificant humans!",
                                "I'll be back at full strength,",
                                "and I'll pay you back for locking me in the darkness for hundreds of years!^000000"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::Cutin, vec![Val::from("moc2_rin04"), Val::from(2)])?;
                        ctx.lines_as(
                            "Rin",
                            args!["What... What just happened?", "Are you Rayan? Or are you someone else now?!"],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::Cutin, vec![Val::from("moc2_dan01"), Val::from(0)])?;
                        ctx.lines_as(
                            "Rayan",
                            args![
                                "You're too late. My master has already departed!",
                                "I'm his loyal servant.",
                                "You're too foolish to understand his great will."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Rayan", args!["Haha... Muhahaha!", "He's calling me!", "He needs me now!"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Rayan",
                            args![
                                "You're Rin, right?",
                                "This is the last time you'll ever see me.",
                                "Haha... Muhahahahaha!!"
                            ],
                        )?;
                        ctx.call(
                            Function::NpcSpecialEffect,
                            vec![ctx.constant("EF_ENTRY")?, ctx.constant("AREA")?, Val::from("Rayan#moc_room2_2")],
                        )?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("Rayan#moc_room2_2::OnDisable")])?;
                        ctx.next()?;
                        ctx.call(Function::Cutin, vec![Val::from("moc2_rin03"), Val::from(2)])?;
                        ctx.lines_as("Rin", args!["............", "....Huh...?", "What just happened?"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["...I don't know...", "...I think he's gone...?"],
                        )?;
                        ctx.call(Function::DelItem, vec![Val::from(6029), Val::from(1)])?;
                        ctx.call(Function::DelItem, vec![Val::from(6027), Val::from(1)])?;
                        ctx.var("mao_morocc2").set(Val::from(22))?;
                        ctx.close_window()?;
                    } else {
                        ctx.call(Function::Cutin, vec![Val::from("moc2_rin01"), Val::from(2)])?;
                        ctx.lines_as("Rin", args!["What's up?", "I'm kind of busy right now."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![
                                "..I've brought back the journal and something else..",
                                "..Er, sorry. I forgot to bring the journal! I'll be right back."
                            ],
                        )?;
                        ctx.close_window()?;
                    }
                } else {
                    if ctx.var("mao_morocc2").get()? == 22 {
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![
                                "What should we do now?",
                                "You know, that voice.. Do you think it was Satan Morocc...?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::Cutin, vec![Val::from("moc2_rin01"), Val::from(2)])?;
                        ctx.lines_as(
                            "Rin",
                            args![
                                "..Maybe.",
                                "B.ased on what the more normal Rayan has told us,",
                                "it seems that another personality within him was forcibly created by Satan Morocc."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Rin",
                            args![
                                "Satan Morocc must have chosen Rayan to break his seal.",
                                "He's been controlling Rayan from deep inside his mind.."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Rin",
                            args!["And... That Mad Rayan seems to have a connection with your Bloody Crystal of Darkness."],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::Cutin, vec![Val::from("moc2_rin02"), Val::from(2)])?;
                        ctx.lines_as(
                            "Rin",
                            args!["I didn't see that side of Rayan until you came; he was very cooperative and feeble up until then."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Rin", args!["The original Rayan seemed to remember everything that happened while he was controlled by that mad personality.."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Rin",
                            args![
                                "He was in shock after learning that he's killed all his comrades with his own hand.",
                                "He started acting differently once you approached him. I must record all of this in the journal."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Rin",
                            args!["Right. He completely changed once he touched the crystal, as if he himself became Satan Morocc."],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::Cutin, vec![Val::from("moc2_rin03"), Val::from(2)])?;
                        ctx.lines_as("Rin", args!["..Damn it."])?;
                        ctx.next()?;
                        let choice = runtime::select_values(ctx, &[Val::from("What should we do now?")])?;
                        ctx.var("@menu").set(choice)?;
                        ctx.call(Function::Cutin, vec![Val::from("moc2_rin02"), Val::from(2)])?;
                        ctx.lines_as(
                            "Rin",
                            args![
                                "Don't worry.",
                                "I've marked Rayan's body in case he ran away.",
                                "We can trace him down."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Rin",
                            args![
                                "I'll give you the pattern number of his magic wavelength. Can you give the number to Echinacea?",
                                "She'll take care of the rest."
                            ],
                        )?;
                        ctx.next()?;
                        l_rotto = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                        if l_rotto.clone() == 1 {
                            l_pattern_s = Val::from("SDHF92F-SDF");
                        } else if l_rotto.clone() == 2 {
                            l_pattern_s = Val::from("VWNM94GVWN90");
                        } else {
                            l_pattern_s = Val::from("CM3-TRDFGHE0");
                        }
                        ctx.lines_as(
                            "Rin",
                            args![
                                "The pattern number is...",
                                ((Val::from("^4d4dff[") + l_pattern_s.clone()) + Val::from("]^000000")),
                                "Don't leave out even one character. Okay?"
                            ],
                        )?;
                        ctx.var("mao_morocc2").set((Val::from(22) + l_rotto.clone()))?;
                        ctx.call(Function::GetItem, vec![Val::from(6029), Val::from(1)])?;
                        ctx.call(
                            Function::ChangeQuest,
                            vec![Val::from(7030), (Val::from(7030) + l_rotto.clone())],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Rin",
                            args![
                                "Here, bring this journal with you.",
                                "Please don't forget the pattern number. Alright? Make sure that Echinacea gets it."
                            ],
                        )?;
                        ctx.close_window()?;
                    } else {
                        if ctx.var("mao_morocc2").get()? == 23 {
                            ctx.call(Function::Cutin, vec![Val::from("moc2_rin01"), Val::from(2)])?;
                            ctx.lines_as(
                                "Rin",
                                args![
                                    "Report to Echinacea in the Ash Vacuum, and give her the pattern number.",
                                    "The pattern number is ^4d4dff[SDHF92F-SDF]^000000. Don't leave out even one character. Okay?"
                                ],
                            )?;
                            ctx.close_window()?;
                        } else if ctx.var("mao_morocc2").get()? == 24 {
                            ctx.call(Function::Cutin, vec![Val::from("moc2_rin01"), Val::from(2)])?;
                            ctx.lines_as(
                                "Rin",
                                args![
                                    "Report to Echinacea in the Ash Vacuum, and give her the pattern number.",
                                    "The pattern number is ^4d4dff[VWNM94GVWN90]^000000. Don't leave out even one character. Okay?"
                                ],
                            )?;
                            ctx.close_window()?;
                        } else if ctx.var("mao_morocc2").get()? == 25 {
                            ctx.call(Function::Cutin, vec![Val::from("moc2_rin01"), Val::from(2)])?;
                            ctx.lines_as(
                                "Rin",
                                args![
                                    "Report to Echinacea in the Ash Vacuum, and give her the pattern number.",
                                    "The pattern number is ^4d4dff[CM3-TRDFGHE0]^000000. Don't leave out even one character. Okay?"
                                ],
                            )?;
                            ctx.close_window()?;
                        } else if (ctx.var("mao_morocc2").get()?.number()? > 25 && ctx.var("mao_morocc2").get()?.number()? < 29) {
                            ctx.call(Function::Cutin, vec![Val::from("moc2_rin01"), Val::from(2)])?;
                            ctx.lines_as(
                                "Rin",
                                args![
                                    "Ouch, my head...",
                                    "Yes? Why are you back again?",
                                    "What...you forgot the pattern number?",
                                    "Sigh...Oh well."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Rin", args!["Make sure you don't forget it again. The pattern number is..."])?;
                            if ctx.var("mao_morocc2").get()? == 26 {
                                l_pattern_s = Val::from("SDHF92F-SDF");
                            } else if ctx.var("mao_morocc2").get()? == 27 {
                                l_pattern_s = Val::from("VWNM94GVWN90");
                            } else {
                                l_pattern_s = Val::from("CM3-TRDFGHE0");
                            }
                            ctx.lines(args![
                                ((Val::from("^4d4dff[") + l_pattern_s.clone())
                                    + Val::from("]^000000. Don't leave out even one character. Okay?")),
                                "Got it?"
                            ])?;
                            ctx.next()?;
                            ctx.lines_as("Rin", args!["Sigh. Get going, then...", "I need to take a break..."])?;
                            ctx.close_window()?;
                        } else {
                            ctx.call(Function::Cutin, vec![Val::from("moc2_rin01"), Val::from(2)])?;
                            ctx.lines_as(
                                "Rin",
                                args![
                                    "Ahhh~ I'm a sick person. I need to rest!",
                                    "I don't want to do anything else~",
                                    "Hurry up and go, don't bother me~"
                                ],
                            )?;
                            ctx.close_window()?;
                        }
                    }
                }
                ctx.call(Function::Cutin, vec![Val::from("moc2_rin01"), Val::from(255)])?;
                return Err(Stop::End);
            }
            RinMocRoom22Step::OnInit => {
                step = RinMocRoom22Step::OnDisable;
                continue 'machine;
            }
            RinMocRoom22Step::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("Rin#moc_room2_2")])?;
                return Err(Stop::End);
            }
            RinMocRoom22Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Rin#moc_room2_2")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn rin_moc_room2_2(ctx: &Ctx) -> Script {
    rin_moc_room2_2_run(ctx, RinMocRoom22Step::Start, Vec::new()).map(|_| ())
}

pub fn rin_moc_room2_2_oninit(ctx: &Ctx) -> Script {
    rin_moc_room2_2_run(ctx, RinMocRoom22Step::OnInit, Vec::new()).map(|_| ())
}

pub fn rin_moc_room2_2_ondisable(ctx: &Ctx) -> Script {
    rin_moc_room2_2_run(ctx, RinMocRoom22Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn rin_moc_room2_2_onenable(ctx: &Ctx) -> Script {
    rin_moc_room2_2_run(ctx, RinMocRoom22Step::OnEnable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum RayanMocRoom22Step {
    Start,
    OnInit,
    OnDisable,
    OnEnable,
}

fn rayan_moc_room2_2_run(ctx: &Ctx, mut step: RayanMocRoom22Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            RayanMocRoom22Step::Start => {
                ctx.mes("- For some reason, Rayan is shivering in fear. -")?;
                ctx.next()?;
                ctx.call(Function::Cutin, vec![Val::from("moc2_rin02"), Val::from(2)])?;
                ctx.lines_as(
                    "Rin",
                    args![
                        "He's not being himself for some reason.",
                        "I'm just glad that he's sane enough to speak to me.",
                        "I don't know what's going on with him."
                    ],
                )?;
                ctx.close_window()?;
                ctx.call(Function::Cutin, vec![Val::from("moc2_rin01"), Val::from(255)])?;
                return Err(Stop::End);
            }
            RayanMocRoom22Step::OnInit => {
                step = RayanMocRoom22Step::OnDisable;
                continue 'machine;
            }
            RayanMocRoom22Step::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("Rayan#moc_room2_2")])?;
                return Err(Stop::End);
            }
            RayanMocRoom22Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Rayan#moc_room2_2")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn rayan_moc_room2_2(ctx: &Ctx) -> Script {
    rayan_moc_room2_2_run(ctx, RayanMocRoom22Step::Start, Vec::new()).map(|_| ())
}

pub fn rayan_moc_room2_2_oninit(ctx: &Ctx) -> Script {
    rayan_moc_room2_2_run(ctx, RayanMocRoom22Step::OnInit, Vec::new()).map(|_| ())
}

pub fn rayan_moc_room2_2_ondisable(ctx: &Ctx) -> Script {
    rayan_moc_room2_2_run(ctx, RayanMocRoom22Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn rayan_moc_room2_2_onenable(ctx: &Ctx) -> Script {
    rayan_moc_room2_2_run(ctx, RayanMocRoom22Step::OnEnable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum HeapOfEarthMao201Step {
    Start,
    OnTouch,
}

fn heap_of_earth_mao2_01_run(ctx: &Ctx, mut step: HeapOfEarthMao201Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            HeapOfEarthMao201Step::Start => {
                if ctx.call(Function::CheckWeight, vec![Val::from(714), Val::from(1)])? == 0 {
                    ctx.mes("- You cannot proceed with the quest when you're carrying too many items with you. -")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ctx.var("mao_morocc2").get()? == 7 {
                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_LEVEL99_4")?])?;
                    ctx.lines(args![
                        "You have found a pile of dirt and deep tracks under thick bushes.",
                        "There are deep footprints and spilled blood around the area."
                    ])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "Some desperate fighting definitely happened here.",
                        "There are some broken branches to the side of a set of footprints that look like they're skidding backwards...",
                        "Somebody must have been fallen down on the ground, or..."
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "I've been having a strange feeling around this area...",
                            "It feels like faint mana.. ?!... Yes, I can barely see an aura.",
                            "...What is this?"
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Ignore it.:Take a careful look.")])? {
                        1 => {
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "Hmm.. This place must be where Rin fought with Rayan's gang.",
                                    "I don't see anything that looks important."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines(args![
                                "As you took a careful look,",
                                "you notice something shining in the ground."
                            ])?;
                            ctx.next()?;
                            ctx.mes("^4d4dffYou have found a 'Bloody Crystal of Darkness'.^000000")?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "I don't think I'll find anything else here...",
                                    "I should go back now.",
                                    "I hope Rin will understand."
                                ],
                            )?;
                            ctx.var("mao_morocc2").set(Val::from(8))?;
                            ctx.call(Function::GetItem, vec![Val::from(6027), Val::from(1)])?;
                            ctx.call(Function::ChangeQuest, vec![Val::from(7016), Val::from(7017)])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else if ctx.var("mao_morocc2").get()? == 8 {
                    ctx.lines(args![
                        "There are traces of a battle here, including spilled blood.",
                        "There's also a pile of dirt that seems out of place."
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.mes("There are traces of a battle here, and spilled blood.")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                step = HeapOfEarthMao201Step::OnTouch;
                continue 'machine;
            }
            HeapOfEarthMao201Step::OnTouch => {
                if ctx.var("mao_morocc2").get()? == 7 {
                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_LEVEL99_4")?])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn heap_of_earth_mao2_01(ctx: &Ctx) -> Script {
    heap_of_earth_mao2_01_run(ctx, HeapOfEarthMao201Step::Start, Vec::new()).map(|_| ())
}

pub fn heap_of_earth_mao2_01_ontouch(ctx: &Ctx) -> Script {
    heap_of_earth_mao2_01_run(ctx, HeapOfEarthMao201Step::OnTouch, Vec::new()).map(|_| ())
}

fn traces_mao2_object02_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("mao_morocc2").get()? == 7 || ctx.var("mao_morocc2").get()? == 8) {
        ctx.lines(args![
            "You find many footprints on the ground.",
            "Considering all the blood on the ground, a battle must have occurred here fairly recently.",
            "The footprints continue over the hill."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.mes("You found some footprints.")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn traces_mao2_object02(ctx: &Ctx) -> Script {
    traces_mao2_object02_body(ctx, Vec::new()).map(|_| ())
}

fn traces_mao2_object03_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("mao_morocc2").get()? == 7 || ctx.var("mao_morocc2").get()? == 8) {
        ctx.lines(args![
            "Somebody left a mark under this small tree.",
            "The mark is weathered and faded,",
            "but it is an arrow pointing southwest."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines(args![
            "There's some scribbling on the tree,",
            "but you can't decipher what it means."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn traces_mao2_object03(ctx: &Ctx) -> Script {
    traces_mao2_object03_body(ctx, Vec::new()).map(|_| ())
}

pub fn moc2_event_on(ctx: &Ctx) -> Script {
    moc2_event_on_run(ctx, Moc2EventOnStep::Start, Vec::new()).map(|_| ())
}

pub fn moc2_event_on_onenable(ctx: &Ctx) -> Script {
    moc2_event_on_run(ctx, Moc2EventOnStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn moc2_event_on_ondisable(ctx: &Ctx) -> Script {
    moc2_event_on_run(ctx, Moc2EventOnStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn moc2_event_on_onstop(ctx: &Ctx) -> Script {
    moc2_event_on_run(ctx, Moc2EventOnStep::OnStop, Vec::new()).map(|_| ())
}

pub fn moc2_event_on_ontouch(ctx: &Ctx) -> Script {
    moc2_event_on_run(ctx, Moc2EventOnStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn moc2_event_on_ontimer300000(ctx: &Ctx) -> Script {
    moc2_event_on_run(ctx, Moc2EventOnStep::OnTimer300000, Vec::new()).map(|_| ())
}

pub fn moc2_event_on_ontimer303000(ctx: &Ctx) -> Script {
    moc2_event_on_run(ctx, Moc2EventOnStep::OnTimer303000, Vec::new()).map(|_| ())
}

pub fn moc2_event_on_ontimer306000(ctx: &Ctx) -> Script {
    moc2_event_on_run(ctx, Moc2EventOnStep::OnTimer306000, Vec::new()).map(|_| ())
}

pub fn moc2_event_on_ontimer307000(ctx: &Ctx) -> Script {
    moc2_event_on_run(ctx, Moc2EventOnStep::OnTimer307000, Vec::new()).map(|_| ())
}

pub fn moc2_event_on_ontimer308000(ctx: &Ctx) -> Script {
    moc2_event_on_run(ctx, Moc2EventOnStep::OnTimer308000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Moc2Event01Step {
    Start,
    OnInit,
    OnDisable,
    OnEnable,
    OnTouch,
}

fn moc2_event01_run(ctx: &Ctx, mut step: Moc2Event01Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Moc2Event01Step::Start => {
                step = Moc2Event01Step::OnInit;
                continue 'machine;
            }
            Moc2Event01Step::OnInit => {
                step = Moc2Event01Step::OnDisable;
                continue 'machine;
            }
            Moc2Event01Step::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("#moc2_event01")])?;
                return Err(Stop::End);
            }
            Moc2Event01Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("#moc2_event01")])?;
                return Err(Stop::End);
            }
            Moc2Event01Step::OnTouch => {
                ctx.call(Function::SoundEffect, vec![Val::from("wander_man_move.wav"), Val::from(1)])?;
                ctx.call(
                    Function::StartStatus,
                    vec![ctx.constant("SC_BLIND")?, Val::from(600000), Val::from(0)],
                )?;
                ctx.lines(args![
                    "As soon as you touched the crystal,",
                    "you feel your body being pulled into the space gap.",
                    "You remember Kidd's confused voice, screaming your name."
                ])?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["By the way, where am I?", "Argh.. My head hurts..."],
                )?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("que_dan01"),
                        Val::from("Female Voice: Checkmate! You can't run away any longer!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x7b68ee"),
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        ".. ? Wh... What's going on?",
                        "I can't see things.. I'm getting dizzy...",
                        "What just happened?"
                    ],
                )?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("que_dan01"),
                        Val::from("Male Voice: *Giggle Giggle* If you thought I was alone, you're mistaken! Argh!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xA8A8A8"),
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "Huh? Are they fighting?",
                        "I can hear a crowd of people coming...",
                        "Argh! It's too dark to see...!"
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Rin#moc2_bt_rin01::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Rayan#moc2_bt_r01::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion Member#moc2_1::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion Member#moc2_2::OnEnable")])?;
                ctx.next()?;
                ctx.lines_as(
                    "Familiar Female Voice",
                    args![
                        "Shut up! It's creepy that you all look alike. Bring it on! I'm not afraid of you!",
                        "I'm not going to let you run away again!"
                    ],
                )?;
                ctx.next()?;
                ctx.call(Function::SoundEffect, vec![Val::from("wander_man_move.wav"), Val::from(1)])?;
                ctx.call(Function::EndStatus, vec![ctx.constant("SC_BLIND")?])?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "... !!!!",
                        "Rin!!!!",
                        "Why? Why is Rin here? Wait, where am I?",
                        "Argh.. I can't move at all..."
                    ],
                )?;
                ctx.next()?;
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![
                        ctx.constant("EF_BEGINSPELL2")?,
                        ctx.constant("AREA")?,
                        Val::from("Dandelion Member#moc2_1"),
                    ],
                )?;
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![
                        ctx.constant("EF_BEGINSPELL2")?,
                        ctx.constant("AREA")?,
                        Val::from("Dandelion Member#moc2_2"),
                    ],
                )?;
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_SURPRISE")?,
                        ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Rin#moc2_bt_rin01")])?,
                    ],
                )?;
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_SURPRISE")?,
                        Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                    ],
                )?;
                ctx.lines_as("Rin", args!["You're not alone. So what?", "You're still coming with me!"])?;
                ctx.next()?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Rin#moc2_bt_rin01::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Rin#moc2_bt_rin02::OnEnable")])?;
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![ctx.constant("EF_ICECRASH")?, ctx.constant("AREA")?, Val::from("Rin#moc2_bt_rin01")],
                )?;
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![
                        ctx.constant("EF_METEORSTORM")?,
                        ctx.constant("AREA")?,
                        Val::from("Rin#moc2_bt_rin01"),
                    ],
                )?;
                ctx.lines_as(
                    "Rayan",
                    args![
                        "Hah, how impressive! You run like a rabbit!",
                        "I should have you killed you when I had the chance.",
                        "I didn't expect you to be so persistent..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Rin",
                    args![
                        "That's what I wanted to say!",
                        "You played me like a chess piece,",
                        "and now I want to tear you apart in five pieces. Why don't you just shut up and surrender before I kill you?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Rayan",
                    args!["Hah.. You still have no idea, huh? You already lost this game, lady."],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion Member#moc2_3::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion Member#moc2_4::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion Member#moc2_5::OnEnable")])?;
                ctx.next()?;
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![ctx.constant("EF_FREEZED")?, ctx.constant("AREA")?, Val::from("Rin#moc2_bt_rin02")],
                )?;
                ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("que_dan01"),
                        Val::from("Rin: When.. When did you...!?!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x7b68ee"),
                    ],
                )?;
                ctx.lines_as("Rayan", args!["I'm sorry that I have to do this.", "...", "Kill her."])?;
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![
                        ctx.constant("EF_BEGINSPELL2")?,
                        ctx.constant("AREA")?,
                        Val::from("Dandelion Member#moc2_3"),
                    ],
                )?;
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![
                        ctx.constant("EF_BEGINSPELL2")?,
                        ctx.constant("AREA")?,
                        Val::from("Dandelion Member#moc2_4"),
                    ],
                )?;
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![
                        ctx.constant("EF_BEGINSPELL2")?,
                        ctx.constant("AREA")?,
                        Val::from("Dandelion Member#moc2_5"),
                    ],
                )?;
                ctx.next()?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("que_dan01"),
                        Val::from(
                            "Rayan: Follow me as soon as you take care of her. Our next meeting place is the usual place. I'm leaving.",
                        ),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xA8A8A8"),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Rayan#moc2_bt_r01::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion Member#moc2_1::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion Member#moc2_2::OnDisable")])?;
                ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Rin-!!!"])?;
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![ctx.constant("EF_LIGHTBOLT")?, ctx.constant("AREA")?, Val::from("Rin#moc2_bt_rin02")],
                )?;
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![
                        ctx.constant("EF_FIREPILLARBOMB")?,
                        ctx.constant("AREA")?,
                        Val::from("Rin#moc2_bt_rin02"),
                    ],
                )?;
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![
                        ctx.constant("EF_METEORSTORM")?,
                        ctx.constant("AREA")?,
                        Val::from("Rin#moc2_bt_rin02"),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Rin#moc2_bt_rin02::OnDisable")])?;
                ctx.next()?;
                ctx.lines(args![
                    "You feel helpless: you can do nothing but watch her slowly get overpowered..",
                    "Then, the pressure on your body suddenly dissipates."
                ])?;
                ctx.call(Function::ChangeQuest, vec![Val::from(7019), Val::from(7020)])?;
                ctx.close_window()?;
                ctx.call(Function::DisableNpc, vec![Val::from("#moc2_event01")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion Member#moc2_3::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion Member#moc2_4::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion Member#moc2_5::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Corpse#moc2_dead01::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Corpse#moc2_dead01::OnCall")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn moc2_event01(ctx: &Ctx) -> Script {
    moc2_event01_run(ctx, Moc2Event01Step::Start, Vec::new()).map(|_| ())
}

pub fn moc2_event01_oninit(ctx: &Ctx) -> Script {
    moc2_event01_run(ctx, Moc2Event01Step::OnInit, Vec::new()).map(|_| ())
}

pub fn moc2_event01_ondisable(ctx: &Ctx) -> Script {
    moc2_event01_run(ctx, Moc2Event01Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn moc2_event01_onenable(ctx: &Ctx) -> Script {
    moc2_event01_run(ctx, Moc2Event01Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn moc2_event01_ontouch(ctx: &Ctx) -> Script {
    moc2_event01_run(ctx, Moc2Event01Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn corpse_moc2_dead01(ctx: &Ctx) -> Script {
    corpse_moc2_dead01_run(ctx, CorpseMoc2Dead01Step::Start, Vec::new()).map(|_| ())
}

pub fn corpse_moc2_dead01_oninit(ctx: &Ctx) -> Script {
    corpse_moc2_dead01_run(ctx, CorpseMoc2Dead01Step::OnInit, Vec::new()).map(|_| ())
}

pub fn corpse_moc2_dead01_ondisable(ctx: &Ctx) -> Script {
    corpse_moc2_dead01_run(ctx, CorpseMoc2Dead01Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn corpse_moc2_dead01_onenable(ctx: &Ctx) -> Script {
    corpse_moc2_dead01_run(ctx, CorpseMoc2Dead01Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn corpse_moc2_dead01_oncall(ctx: &Ctx) -> Script {
    corpse_moc2_dead01_run(ctx, CorpseMoc2Dead01Step::OnCall, Vec::new()).map(|_| ())
}

pub fn corpse_moc2_dead01_onmymobdead(ctx: &Ctx) -> Script {
    corpse_moc2_dead01_run(ctx, CorpseMoc2Dead01Step::OnMyMobDead, Vec::new()).map(|_| ())
}

pub fn corpse_moc2_dead01_onreset(ctx: &Ctx) -> Script {
    corpse_moc2_dead01_run(ctx, CorpseMoc2Dead01Step::OnReset, Vec::new()).map(|_| ())
}

pub fn corpse_moc2_dead01_ontimer4000(ctx: &Ctx) -> Script {
    corpse_moc2_dead01_run(ctx, CorpseMoc2Dead01Step::OnTimer4000, Vec::new()).map(|_| ())
}

pub fn corpse_moc2_dead01_ontimer7000(ctx: &Ctx) -> Script {
    corpse_moc2_dead01_run(ctx, CorpseMoc2Dead01Step::OnTimer7000, Vec::new()).map(|_| ())
}

pub fn corpse_moc2_dead01_ontimer10000(ctx: &Ctx) -> Script {
    corpse_moc2_dead01_run(ctx, CorpseMoc2Dead01Step::OnTimer10000, Vec::new()).map(|_| ())
}

pub fn corpse_moc2_dead01_ontimer11000(ctx: &Ctx) -> Script {
    corpse_moc2_dead01_run(ctx, CorpseMoc2Dead01Step::OnTimer11000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum DandelionDuplicatesStep {
    Start,
    OnInit,
    OnDisable,
    OnEnable,
}

fn dandelion_duplicates_run(ctx: &Ctx, mut step: DandelionDuplicatesStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            DandelionDuplicatesStep::Start => {
                return Err(Stop::End);
            }
            DandelionDuplicatesStep::OnInit => {
                step = DandelionDuplicatesStep::OnDisable;
                continue 'machine;
            }
            DandelionDuplicatesStep::OnDisable => {
                ctx.call(Function::DisableNpc, vec![])?;
                return Err(Stop::End);
            }
            DandelionDuplicatesStep::OnEnable => {
                ctx.call(Function::EnableNpc, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn dandelion_duplicates(ctx: &Ctx) -> Script {
    dandelion_duplicates_run(ctx, DandelionDuplicatesStep::Start, Vec::new()).map(|_| ())
}

pub fn dandelion_duplicates_oninit(ctx: &Ctx) -> Script {
    dandelion_duplicates_run(ctx, DandelionDuplicatesStep::OnInit, Vec::new()).map(|_| ())
}

pub fn dandelion_duplicates_ondisable(ctx: &Ctx) -> Script {
    dandelion_duplicates_run(ctx, DandelionDuplicatesStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn dandelion_duplicates_onenable(ctx: &Ctx) -> Script {
    dandelion_duplicates_run(ctx, DandelionDuplicatesStep::OnEnable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Moc2B1GateStep {
    Start,
    OnTouch,
}

fn moc2_b1_gate_run(ctx: &Ctx, mut step: Moc2B1GateStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Moc2B1GateStep::Start => {
                step = Moc2B1GateStep::OnTouch;
                continue 'machine;
            }
            Moc2B1GateStep::OnTouch => {
                if ctx.var("mao_morocc2").get()? == 17 {
                    ctx.lines(args![
                        "- You find a side door that leads to the basement.",
                        "A faint light is shining through the hinges.",
                        "There are people inside, talking loudly. -"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Male Voice",
                        args!["Hey Rayan, say something.", "We did everything you asked,", "but..."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Male Voice",
                        args![
                            "...This is totally wrong.",
                            "We did something that we shouldn't have.",
                            "Come on, reconsider this."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Male Voice",
                        args![
                            "Rayan, Rayan!",
                            "All we wanted was just to shake up the town just a little bit,",
                            "not summon Satan Morocc! The ritual failed like we first planned, but..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Voices",
                        args![
                            "That's right. It failed, but what happened? Satan Morocc resurrected himself.",
                            "He'll bring the end of this world.",
                            "What have we done?!",
                            "We can't even go back to our country now!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Anxious Voice",
                        args![
                            "...We all knew that we would be disposed after we stopped being useful.. Just like pawns in a chess match.",
                            "But I'm curious.",
                            "Rayan..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Anxious Voice",
                        args![
                            "Rayan, say something!",
                            "What the hell's wrong with you?",
                            "Why, can't you still forget the Assassin?",
                            "...Hey, tell us. What should we do now?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Anxious Voice",
                        args![
                            "Let me ask you one question.",
                            "...Did you... Did you really summon Satan Morocc?",
                            "Did you?",
                            "Did you really ask us to help you resurrect Satan Morocc?!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Anxious Voice",
                        args![
                            "If your silence means yes, I'm out!",
                            "I didn't know then, but I know now. I'm leaving!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Rayan",
                        args![
                            "Incompetent weaklings.",
                            "I don't need you anymore.",
                            "Thank you for following my lead so far.",
                            "But you're nothing but puppets of the goddess.. *Giggle Giggle*"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Male Voice",
                        args!["Hey, hey! What are you doing?!", "Why are you doing this?!"],
                    )?;
                    ctx.next()?;
                    ctx.lines(args![
                        "- You hear loud thuds and thumping in the midst of their yelling and screaming.",
                        "It sounds like they're having a serious dispute.-"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "...Rayan? ...Morocc? ...Resurrect...?",
                            "I have no idea what they're talking about. Why are they fighting?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Rayan",
                        args![
                            "You ignorant puppets..",
                            "You're too stupid to understand my master's great will...",
                            "...Oh, right. I completely forgot about 'it.' It's time to take it back.",
                            "What do you say? *Giggle Giggle*"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["What the?! Does he know I'm here?!", "I'd better run!"],
                    )?;
                    ctx.next()?;
                    ctx.mes("- You used a Butterfly Wing.-")?;
                    ctx.var("mao_morocc2").set(Val::from(18))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(7026), Val::from(7027)])?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("SavePoint"), Val::from(0), Val::from(0)])?;
                    return Err(Stop::End);
                } else if ctx.var("mao_morocc2").get()? == 19 {
                    ctx.lines(args!["- The side to the basement is open.", "You can enter if you want. -"])?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Enter.:Do not enter.")])? {
                        1 => {
                            ctx.call(Function::Warp, vec![Val::from("que_dan02"), Val::from(91), Val::from(11)])?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.mes("You have decided to not enter the basement.")?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else {
                    ctx.lines(args![
                        "- This door seems to lead to the basement,",
                        "but it's locked pretty securely. -"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn moc2_b1_gate(ctx: &Ctx) -> Script {
    moc2_b1_gate_run(ctx, Moc2B1GateStep::Start, Vec::new()).map(|_| ())
}

pub fn moc2_b1_gate_ontouch(ctx: &Ctx) -> Script {
    moc2_b1_gate_run(ctx, Moc2B1GateStep::OnTouch, Vec::new()).map(|_| ())
}

fn man_moc2_crazyr01_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("mao_morocc2").get()? == 19 {
        ctx.mes("A man covered in blood is standing at a corner with a mysterious grin.")?;
        ctx.next()?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("Mr. Kidd#moc_extra01::OnEnable")])?;
        ctx.lines_as(
            "Mr. Kidd",
            args![
                "I guess it was the right timing.",
                "By the way, who is this guy?",
                "He looks kind of.. He's insane, isn't he?"
            ],
        )?;
        ctx.call(Function::Cutin, vec![Val::from("mocseal_kid01"), Val::from(0)])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Woah! Where did you come from?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Mr. Kidd",
            args!["...From somewhere.", "Anyways...", "What happened?", "Who are you?"],
        )?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("moc2_dan01"), Val::from(2)])?;
        ctx.lines_as(
            "Man",
            args![
                "...Who am I...?",
                "...I think I forgot...",
                "They're all dead.. I've killed them.",
                "Hehehe, I just followed the will of my master.",
                "Muhahahahaha!!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["No... No way!"])?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Rayan?!")])?;
        ctx.var("@menu").set(choice)?;
        ctx.call(Function::Cutin, vec![Val::from("moc2_kid02"), Val::from(0)])?;
        ctx.lines_as(
            "Mr. Kidd",
            args![
                "What? Is this really the Rayan we've been chasing?",
                "..... !!!",
                "Right, you said Rayan was having a dispute with his buddies.."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("moc2_dan02"), Val::from(2)])?;
        ctx.lines_as(
            "Rayan",
            args![
                "...Ray... Rayan? That's my name? That's right...",
                "I'm Rayan. Rayan Moore.",
                "Who are you..?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Rayan",
            args![
                "Where am I? What's all this blood?!",
                "...Huh? Where am I? What happened to my friends?"
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("mocseal_kid01"), Val::from(0)])?;
        ctx.lines_as(
            "Mr. Kidd",
            args![
                "...I guess I was right. He really is Rayan Moore.",
                "What's wrong with him?",
                "He seems... He seems really confused. Maybe even unstable."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Mr. Kidd",
            args![
                "Well, it's the perfect time to capture this guy before he can kill us.",
                "Rin will know if this is the real Rayan because I've never seen him without his mask.."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Mr. Kidd",
            args![
                "I'll bring this guy to Rin so she can interrogate him.",
                "I'll see you at the camp."
            ],
        )?;
        ctx.call(Function::Cutin, vec![Val::from("mocseal_kid01"), Val::from(255)])?;
        ctx.next()?;
        ctx.mes("- You watched Mr. Kidd arrest the unconscious Rayan and take out a teleportation item. You decide to return to town. -")?;
        ctx.var("mao_morocc2").set(Val::from(20))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(7028), Val::from(7029)])?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("SavePoint"), Val::from(0), Val::from(0)])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("Mr. Kidd#moc_extra01::OnDisable")])?;
    } else {
        ctx.lines_as("Man", args!["I live to serve my master!", "Are you his enemy?", "DIE!"])?;
        ctx.close_window()?;
        ctx.call(Function::PercentHeal, vec![Val::from(-100), Val::from(0)])?;
        return Err(Stop::End);
    }
    ctx.call(Function::Cutin, vec![Val::from("mocseal_kid01"), Val::from(255)])?;
    return Err(Stop::End);
}

pub fn man_moc2_crazyr01(ctx: &Ctx) -> Script {
    man_moc2_crazyr01_body(ctx, Vec::new()).map(|_| ())
}

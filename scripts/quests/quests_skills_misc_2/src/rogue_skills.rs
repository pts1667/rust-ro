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

#[derive(Clone, Copy, Debug)]
enum Killershow01Step {
    Start,
    OnInit,
    OnEnable,
    OnDisable,
    OnTimer1000,
    OnTimer120000,
    OnTimer150000,
}

fn killershow01_run(ctx: &Ctx, mut step: Killershow01Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Killershow01Step::Start => {
                step = Killershow01Step::OnInit;
                continue 'machine;
            }
            Killershow01Step::OnInit => {
                ctx.set_npc_visible("#killershow01", false)?;
                return Err(Stop::End);
            }
            Killershow01Step::OnEnable => {
                ctx.set_npc_visible("#killershow01", true)?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Killershow01Step::OnDisable => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.set_npc_visible("#killershow01", false)?;
                return Err(Stop::End);
            }
            Killershow01Step::OnTimer1000 => {
                ctx.set_npc_visible("Killer#Rogueguild", true)?;
                return Err(Stop::End);
            }
            Killershow01Step::OnTimer120000 => {
                ctx.set_npc_visible("Killer#Rogueguild", false)?;
                return Err(Stop::End);
            }
            Killershow01Step::OnTimer150000 => {
                ctx.set_npc_visible("Killer#Rogueguild", false)?;
                ctx.set_npc_visible("#killershow01", false)?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn killershow01(ctx: &Ctx) -> Script {
    killershow01_run(ctx, Killershow01Step::Start, Vec::new()).map(|_| ())
}

pub fn killershow01_oninit(ctx: &Ctx) -> Script {
    killershow01_run(ctx, Killershow01Step::OnInit, Vec::new()).map(|_| ())
}

pub fn killershow01_onenable(ctx: &Ctx) -> Script {
    killershow01_run(ctx, Killershow01Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn killershow01_ondisable(ctx: &Ctx) -> Script {
    killershow01_run(ctx, Killershow01Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn killershow01_ontimer1000(ctx: &Ctx) -> Script {
    killershow01_run(ctx, Killershow01Step::OnTimer1000, Vec::new()).map(|_| ())
}

pub fn killershow01_ontimer120000(ctx: &Ctx) -> Script {
    killershow01_run(ctx, Killershow01Step::OnTimer120000, Vec::new()).map(|_| ())
}

pub fn killershow01_ontimer150000(ctx: &Ctx) -> Script {
    killershow01_run(ctx, Killershow01Step::OnTimer150000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum KillerRogueguildStep {
    Start,
    OnInit,
    OnTouch,
}

fn killer_rogueguild_run(ctx: &Ctx, mut step: KillerRogueguildStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_effects: Vec<Val> = Vec::new();
    let mut l_move_1 = Val::from(0);
    'machine: loop {
        match step {
            KillerRogueguildStep::Start => {
                step = KillerRogueguildStep::OnInit;
                continue 'machine;
            }
            KillerRogueguildStep::OnInit => {
                ctx.set_npc_visible("Killer#Rogueguild", false)?;
                return Err(Stop::End);
            }
            KillerRogueguildStep::OnTouch => {
                if ctx.var("rog_sk").get()? == 10 {
                    ctx.call(Function::SpecialEffect, args![constants::EF_CHANGECOLD])?;
                    ctx.call(Function::NpcSpecialEffect, args![constants::EF_CHANGEWIND])?;
                    ctx.call(Function::SpecialEffect, args![constants::EF_LIGHTSPHERE])?;
                    ctx.var("rog_sk").set(Val::from(11))?;
                    ctx.lines_as("Killer", args!["Wh-what have", "you done to me?!", "C-can't... move!"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Haijara Greg",
                        args![
                            "Oh? I didn't expect",
                            "you to be able to cast",
                            "Close Confine so soon!",
                            "Amazing, just amazing!",
                            "Now, this is a good chance",
                            "for you to master the skill..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Haijara Greg",
                        args![
                            "Now pay attention.",
                            "I want you to practice",
                            "predicting your opponent's",
                            "movement intent on this killer.",
                            "You should be able to see which way he plans to move by his aura."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Haijara Greg",
                        args![
                            "If he plans to move to left,",
                            "his aura will be white. If he",
                            "moves to the right, it will be",
                            "yellow. If he intends to go",
                            "backward, it will be pale red."
                        ],
                    )?;
                    ctx.next()?;
                    runtime::local_set(&mut l_effects, &Val::from(1), Val::from(constants::EF_CHANGECOLD), false);
                    runtime::local_set(&mut l_effects, &Val::from(2), Val::from(constants::EF_CHANGEWIND), false);
                    runtime::local_set(&mut l_effects, &Val::from(3), Val::from(constants::EF_CHANGEEARTH), false);
                    let mut l_lim_1: i32 = 0;
                    while l_lim_1 < 10 {
                        'b1: {
                            l_move_1 = ctx.call(Function::Rand, args![1, 3])?;
                            ctx.call(
                                Function::NpcSpecialEffect,
                                args![runtime::local_get(&l_effects, &l_move_1.clone(), false)],
                            )?;
                            if Val::from(runtime::select_values(
                                ctx,
                                &[Val::from("Block him to the Left:Block him to the Right:Block his Retreat")],
                            )?)
                            .loosely_equals(&l_move_1.clone())
                            {
                                ctx.lines(args![
                                    "^3355FFThe killer remains",
                                    "unable to move and looks",
                                    "incredibly confused! Right",
                                    "now, you're using the Close",
                                    "Confine skill perfectly!^000000"
                                ])?;
                                ctx.call(Function::NpcSpecialEffect, args![constants::EF_POTION1])?;
                                ctx.call(Function::SpecialEffect, args![constants::EF_POTION7])?;
                            } else {
                                ctx.lines(args![
                                    "^3355FFWait--!",
                                    "For some reason,",
                                    "you sense that's not",
                                    "the direction the killer",
                                    "is moving at this moment.",
                                    "You naturally correct yourself.^000000"
                                ])?;
                            }
                            ctx.next()?;
                        }
                        l_lim_1 += 1;
                    }
                    ctx.lines(args![
                        "^3355FFYou successfully",
                        "retrieved the priceless",
                        "skill book written by the",
                        "legendary Chae Takbae.^000000"
                    ])?;
                    ctx.close_window()?;
                    ctx.set_npc_visible("Killer#Rogueguild", false)?;
                    ctx.npc().do_event("#killershow01::OnDisable")?;
                    return Err(Stop::End);
                } else if ctx.var("rog_sk").get()? == 11 {
                    ctx.lines_as("Killer", args!["Grrrrr...", "S-still...", "C-can't... Move!"])?;
                    ctx.close_window()?;
                    ctx.set_npc_visible("Killer#Rogueguild", false)?;
                    ctx.npc().do_event("#killershow01::OnDisable")?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn killer_rogueguild(ctx: &Ctx) -> Script {
    killer_rogueguild_run(ctx, KillerRogueguildStep::Start, Vec::new()).map(|_| ())
}

pub fn killer_rogueguild_oninit(ctx: &Ctx) -> Script {
    killer_rogueguild_run(ctx, KillerRogueguildStep::OnInit, Vec::new()).map(|_| ())
}

pub fn killer_rogueguild_ontouch(ctx: &Ctx) -> Script {
    killer_rogueguild_run(ctx, KillerRogueguildStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn haijara_greg_rogueguild(ctx: &Ctx) -> Script {
    if ctx.var("BaseJob").get()? == constants::JOB_ROGUE {
        if ctx.var("rog_sk").get()? == 12 && ctx.var("Upper").get()? == 1 {
            ctx.lines_as(
                "Haijara Greg",
                args![
                    "Hm? Ah, amnesia as",
                    "resulting from transcending,",
                    "eh? Then I will teach you the",
                    "Close Confine skill once again."
                ],
            )?;
            ctx.call(Function::SpecialEffect, args![constants::EF_LIGHTSPHERE])?;
            ctx.call(Function::Skill, args!["RG_CLOSECONFINE", 1, constants::SKILL_PERM])?;
            ctx.var("rog_sk").set(Val::from(13))?;
            return ctx.close();
        } else if ctx.var("rog_sk").get()? == 13 && ctx.var("Upper").get()? == 1 {
            ctx.lines_as(
                "Haijara Greg",
                args![
                    "A Stalker, eh?",
                    "Make sure that you",
                    "use your abilities to",
                    "malign foes that deserve",
                    "to be stalked. Best of",
                    "luck to you, adventurer."
                ],
            )?;
            return ctx.close();
        } else {
            if ctx.var("rog_sk").get()?.number()? < 1 {
                ctx.lines_as(
                    "Haijara Greg",
                    args![
                        "H-how did you find this",
                        "place? I thought this panic",
                        "room was supposed to be",
                        "impenetrable, even by Rogues!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Haijara Greg",
                    args!["This can't be good!", "It will only be a matter of", "time before they find me..."],
                )?;
                return ctx.close();
            } else if ctx.var("rog_sk").get()? == 1 {
                ctx.lines_as(
                    "Haijara Greg",
                    args!["Wh-who are you,", "and how did you get", "in here? Identify yourself!"],
                )?;
                ctx.next()?;
                if ctx.menu(&["I don't mean you any harm!", "Give us what we want!"])? == 0 {
                    ctx.lines_as(
                        "Haijara Greg",
                        args![
                            "No...?",
                            "Then... Then",
                            "why have you come?",
                            "It is no accident that",
                            "you have found me."
                        ],
                    )?;
                    ctx.next()?;
                    if ctx.menu(&["I came to help you.", "Oh, actually, I wasn't looking for you."])? == 0 {
                        ctx.lines_as("Haijara Greg", args!["Hmm... Well, I suppose"])?;
                        if ctx.var("Upper").get()? == 1 {
                            ctx.mes("I can trust a fellow Stalker")?;
                        } else {
                            ctx.mes("I can trust a fellow Rogue")?;
                        }
                        ctx.lines(args![
                            "with my predictament. Honor",
                            "among thieves and all that.",
                            "Alright. Have you ever heard",
                            "the legend of Chae Takbae?"
                        ])?;
                        ctx.next()?;
                        if ctx.menu(&["No", "Yes"])? == 0 {
                            ctx.lines_as(
                                "Haijara Greg",
                                args![
                                    "100 years ago, Chae Takbae",
                                    "was the very first person to",
                                    "transcend his limits. He was",
                                    "also the very person to choose",
                                    "a different path, rather than the job order he previously had."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Haijara Greg",
                                args![
                                    "He was originally a Monk,",
                                    "but after transcending, he",
                                    "somehow became a Stalker.",
                                    "Retaining his knowledge of",
                                    "the Monk's Root skill, he adapted it for the purposes of the Rogues."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Haijara Greg",
                                args![
                                    "Chae Takbae recorded the",
                                    "fundamentals for this new",
                                    "Rogue skill in a book that",
                                    "I was fortunate enough to",
                                    "obtain. But as soon as I got",
                                    "it, the blackmailing begain..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Haijara Greg",
                                args![
                                    "There are unscrupulous",
                                    "parties that will use any",
                                    "means to take the book away",
                                    "from me. I can understand, as",
                                    "my sons and I have learned new",
                                    "skills from Takbae's writings."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Haijara Greg",
                                args![
                                    "However, I am running",
                                    "out of time and those men",
                                    "will inevitably find me.",
                                    "Will you help me save",
                                    "myself and my sons?"
                                ],
                            )?;
                            ctx.next()?;
                            if ctx.menu(&["Sure!", "Sorry, but I'm busy."])? == 0 {
                                ctx.lines_as(
                                    "Haijara Greg",
                                    args![
                                        "Oh, thank you so much!",
                                        "Listen, I can't risk being",
                                        "found, so would you take",
                                        "this letter to my youngest",
                                        "son, ^FF0000Louis Greg^000000? Hurry,",
                                        "there's not much time left!"
                                    ],
                                )?;
                                ctx.var("rog_sk").set(Val::from(2))?;
                                return ctx.close();
                            }
                            ctx.lines_as(
                                "Haijara Greg",
                                args![
                                    "I... I see.",
                                    "But please realize",
                                    "that, if not you, who",
                                    "can I trust to help me?"
                                ],
                            )?;
                            return ctx.close();
                        }
                        ctx.lines_as(
                            "Haijara Greg",
                            args![
                                "Then you would know the",
                                "value of the skill book he has",
                                "written that I now possess.",
                                "However, I'm hounded by men",
                                "who will do anything to get it.",
                                "Would you please help me?"
                            ],
                        )?;
                        ctx.next()?;
                        if ctx.menu(&["Sure!", "Sorry, but I'm busy."])? == 0 {
                            ctx.lines_as(
                                "Haijara Greg",
                                args![
                                    "Oh, thank you so much!",
                                    "Listen, I can't risk being",
                                    "found, so would you take",
                                    "this letter to my youngest",
                                    "son, ^FF0000Louis Greg^000000? Hurry,",
                                    "there's not much time left!"
                                ],
                            )?;
                            ctx.var("rog_sk").set(Val::from(2))?;
                            return ctx.close();
                        }
                        ctx.lines_as(
                            "Haijara Greg",
                            args![
                                "I... I see.",
                                "But please realize",
                                "that, if not you, who",
                                "can I trust to help me?"
                            ],
                        )?;
                        return ctx.close();
                    }
                    ctx.lines_as(
                        "Haijara Greg",
                        args![
                            "Hm...?",
                            "That seems unlikely, but",
                            "I suppose I better give you",
                            "the benefit of the doubt.",
                            "Well then, I hope that you",
                            "can find your way out of here."
                        ],
                    )?;
                    return ctx.close();
                }
                ctx.lines_as(
                    "Haijara Greg",
                    args![
                        "I see. So be it.",
                        "I'll show you the skill",
                        "that you covet so much...",
                        "^FF0000Close Confine^000000!"
                    ],
                )?;
                ctx.call(Function::NpcSpecialEffect, args![constants::EF_CHANGECOLD])?;
                ctx.call(Function::SpecialEffect, args![constants::EF_CHANGEWIND])?;
                ctx.next()?;
                ctx.lines_as(
                    "Haijara Greg",
                    args!["Hmpf. Now you are", "helpless, allowing", "me to do this: ^FF0000Back Stab^000000!"],
                )?;
                ctx.call(Function::SpecialEffect, args![constants::EF_COMBOATTACK5])?;
                ctx.call(Function::PercentHeal, args![-95, 0])?;
                ctx.next()?;
                ctx.lines_as(
                    "Haijara Greg",
                    args![
                        "Now get out of here.",
                        "Never show your greedy",
                        "face in front of me again.",
                        "And just be happy that",
                        "I haven't killed you!"
                    ],
                )?;
                ctx.var("rog_sk").set(Val::from(1))?;
                return ctx.close();
            } else if ctx.var("rog_sk").get()? == 2 {
                ctx.lines_as(
                    "Haijara Greg",
                    args![
                        "We're running out of",
                        "time... Please bring this",
                        "letter to my youngest son,",
                        "^FF0000Louis^000000, as soon as you can!",
                        "You can find him in the Rogue",
                        "Guild near ^FF0000Hollgrehenn Junior^000000."
                    ],
                )?;
                return ctx.close();
            } else if ctx.var("rog_sk").get()? == 3 {
                ctx.lines_as(
                    "Haijara Greg",
                    args![
                        "Louis sent you to find",
                        "^FF0000Thor^000000? He's in the Rogue",
                        "Guild near ^FF0000Hermanthorn Jr.^000000,",
                        "isn't he? I'm sorry that you",
                        "have to visit my sons one by",
                        "one... I know it's impractical."
                    ],
                )?;
                return ctx.close();
            } else if ctx.var("rog_sk").get()? == 4 {
                ctx.lines_as(
                    "Haijara Greg",
                    args![
                        "Ah, looking for ^FF0000Jay^000000, eh?",
                        "He's near ^FF0000Antonio Jr.^000000 here",
                        "in the Rogue Guild. Or at",
                        "least, he's usually there."
                    ],
                )?;
                return ctx.close();
            } else if ctx.var("rog_sk").get()? == 5 {
                ctx.lines_as(
                    "Haijara Greg",
                    args![
                        "Oh, you're back!",
                        "And you've brought",
                        "a letter from Jay.",
                        "Good, good, let me",
                        "read what he has to say..."
                    ],
                )?;
                ctx.var("rog_sk").set(Val::from(6))?;
                ctx.next()?;
                ctx.lines_as(
                    "Haijara Greg",
                    args![
                        "Ah, great news! He's",
                        "contacted the Rogue Guild",
                        "to request extra protection",
                        "and to alert their guard. I can",
                        "finally relax just a little now. No one messes with Rogues."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Haijara Greg",
                    args![
                        "Thank you very much for",
                        "your help. I would be in",
                        "hiding forever if it weren't",
                        "for you. In return, let me",
                        "offer you the chance to learn",
                        "Chae Takbae's secret skill."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Haijara Greg",
                    args![
                        "Please speak to ^FF0000Thor^000000",
                        "and tell him that I've",
                        "permitted you to learn",
                        "the skill I have taught",
                        "all of my sons. He'll",
                        "comply, I'm sure of it."
                    ],
                )?;
                return ctx.close();
            } else if ctx.var("rog_sk").get()? == 6 {
                ctx.lines_as(
                    "Haijara Greg",
                    args![
                        "Please ask Thor to teach",
                        "you Chae Takbae's secret",
                        "Rogue skill. Understand that",
                        "I can't teach you this skill",
                        "with the equipment here in",
                        "this sloven panic room."
                    ],
                )?;
                return ctx.close();
            } else if ctx.var("rog_sk").get()? == 7 {
                ctx.lines_as(
                    "Haijara Greg",
                    args![
                        "Hm...? Aren't you",
                        "supposed to be in the",
                        "middle of training to learn",
                        "that skill? Please speak to",
                        "Thor and complete your training."
                    ],
                )?;
                return ctx.close();
            } else if ctx.var("rog_sk").get()? == 8 {
                ctx.lines_as(
                    "Haijara Greg",
                    args![
                        "You've completed the",
                        "training? Ah, that's quite",
                        "exceptional. However, you",
                        "should speak to Thor first..."
                    ],
                )?;
                return ctx.close();
            } else if ctx.var("rog_sk").get()? == 9 {
                ctx.lines_as(
                    "Haijara Greg",
                    args![
                        "Oh, you've come back!",
                        "I hear that you've completed",
                        "the training and are ready",
                        "to hear about the applications",
                        "for the Close Confine skill."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Haijara Greg",
                    args![
                        "Wah! What th--?!",
                        "Stop! P-please, stop",
                        "that man! We can't let",
                        "him steal that book!"
                    ],
                )?;
                ctx.call(Function::NpcSpecialEffect, args![constants::EF_COMBOATTACK5])?;
                ctx.npc().do_event("Killer#Rogueguild::OnEnable")?;
                ctx.var("rog_sk").set(Val::from(10))?;
                return ctx.close();
            } else if ctx.var("rog_sk").get()? == 10 {
                ctx.lines_as(
                    "Haijara Greg",
                    args![
                        "That man just stole the",
                        "Close Confine skill book!",
                        "Please! Don't let him get",
                        "away! That book is priceless!"
                    ],
                )?;
                ctx.npc().do_event("#killershow01::OnEnable")?;
                return ctx.close();
            } else if ctx.var("rog_sk").get()? == 11 {
                ctx.npc().do_event("Killer#Rogueguild::OnDisable")?;
                ctx.npc().do_event("#killershow01::OnDisable")?;
                ctx.lines(args![
                    "^3355FFYou returned the",
                    "skill book written by",
                    "Chae Takbae to Haijara.^000000"
                ])?;
                ctx.next()?;
                ctx.lines_as(
                    "Haijara Greg",
                    args![
                        "Thanks so much for your",
                        "assistance. Now, if you'd",
                        "like to know some detailed",
                        "information about Close",
                        "Confine, I can tell you more",
                        "about the skill if you like."
                    ],
                )?;
                ctx.next()?;
                if ctx.menu(&["Yes, please.", "No, thanks."])? == 0 {
                    ctx.lines_as(
                        "Haijara Greg",
                        args![
                            "As you must know,",
                            "Close Confine immobilizes",
                            "an enemy that is very close",
                            "to you. However, there are",
                            "a few nuances regarding its",
                            "use on players or monsters."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Haijara Greg",
                        args![
                            "Now, Close Confine is",
                            "similar to the Monk's skill,",
                            "Root, but it only inhibits the",
                            "enemy's movement, not",
                            "its attack capabilities."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Haijara Greg",
                        args![
                            "With the exception of",
                            "Back Stab, which we can",
                            "only use once during Close",
                            "Confine's duration, we can",
                            "use any skill during the",
                            "Close Confine status."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Haijara Greg",
                        args![
                            "Enemies affected by",
                            "Close Confine can escape",
                            "by using Fly Wing, Butterfly",
                            "Wings, or the Teleport or",
                            "Hiding skills. However, only",
                            "Hiding is active during WoE."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Haijara Greg",
                        args![
                            "Therefore, Close Confine",
                            "can be a very useful skill",
                            "during Guild War sieges, given",
                            "that the target doesn't use the",
                            "the Hiding skill to get away."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Haijara Greg",
                        args![
                            "With the exception of Boss",
                            "monsters, Close Confine",
                            "will immobilize monsters for",
                            "10 seconds, giving Rogues",
                            "new possibilities when",
                            "hunting in a party."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Haijara Greg",
                        args![
                            "Well, that's all I can tell",
                            "you about Close Confine.",
                            "I hope that you can learn",
                            "more about this skill through",
                            "practice, and that you become as great a legend as Chae Takbae."
                        ],
                    )?;
                    ctx.var("rog_sk").set(Val::from(12))?;
                    ctx.call(Function::Skill, args!["RG_CLOSECONFINE", 1, constants::SKILL_PERM])?;
                    ctx.call(Function::SpecialEffect, args![constants::EF_LIGHTSPHERE])?;
                    return ctx.close();
                }
                ctx.lines_as(
                    "Haijara Greg",
                    args![
                        "All right then...",
                        "I hope that you can",
                        "learn more about Close",
                        "Confine through diligent",
                        "practice. Good luck, and",
                        "thanks again for your help."
                    ],
                )?;
                ctx.var("rog_sk").set(Val::from(12))?;
                ctx.call(Function::Skill, args!["RG_CLOSECONFINE", 1, constants::SKILL_PERM])?;
                ctx.call(Function::SpecialEffect, args![constants::EF_LIGHTSPHERE])?;
                return ctx.close();
            }
            ctx.lines_as(
                "Haijara Greg",
                args![
                    "Thank you for helping",
                    "me protect this priceless",
                    "skill book. I hope that the",
                    "next time we meet, we'll be",
                    "comrades on the battlefield..."
                ],
            )?;
            return ctx.close();
        }
    } else {
        ctx.lines_as(
            "Haijara Greg",
            args![
                "H-how did you find this",
                "place? I thought this panic",
                "room was supposed to be",
                "impenetrable, even by Rogues!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Haijara Greg",
            args!["This can't be good!", "It will only be a matter of", "time before they find me..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Haijara Greg",
            args![
                "Please...",
                "Find me a Stalker",
                "or a Rogue that I can",
                "trust and send him to",
                "help me! I don't have ",
                "much time left..."
            ],
        )?;
        return ctx.close();
    }
}

pub fn louis_greg_rogueguild(ctx: &Ctx) -> Script {
    if ctx.var("BaseJob").get()? == constants::JOB_ROGUE || ctx.var("Class").get()? == constants::JOB_THIEF_HIGH {
        if ctx.var("rog_sk").get()?.number()? < 1 {
            ctx.lines_as(
                "Louis Greg",
                args![
                    "My father just...",
                    "He just vanished!",
                    "He was teaching me and",
                    "my brothers a new skill,",
                    "but lately he began acting",
                    "paranoid for some reason..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Louis Greg",
                args![
                    "Maybe he went into hiding?",
                    "I know there's a hidden panic",
                    "room in the Rogue Guild, but",
                    "even I don't know where to find",
                    "it. I hope everything's okay..."
                ],
            )?;
            ctx.var("rog_sk").set(Val::from(1))?;
            return ctx.close();
        } else if ctx.var("rog_sk").get()? == 1 {
            ctx.lines_as(
                "Louis Greg",
                args![
                    "Father did warn that",
                    "people might come after us",
                    "if they're learned about the",
                    "new skill we were learning.",
                    "Maybe he went into hiding in",
                    "the Rogue Guild's panic room..."
                ],
            )?;
            return ctx.close();
        } else if ctx.var("rog_sk").get()? == 2 {
            ctx.lines_as(
                "Louis Greg",
                args![
                    "What's this...?",
                    "A letter from my",
                    "father? Oh, he must",
                    "be alright! Quick, let",
                    "me read it right away!"
                ],
            )?;
            ctx.var("rog_sk").set(Val::from(3))?;
            ctx.next()?;
            ctx.lines_as(
                "Louis Greg",
                args![
                    "Oh no, he may be safe for",
                    "now, but father is being hunted",
                    "by some dangerous people? My",
                    "brother Thor will want to know",
                    "about this. Let me write him",
                    "a letter really quickly..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Louis Greg",
                args![
                    "I know that I'm in no",
                    "position to ask any favors,",
                    "but I guess my father must",
                    "trust you. Please, would you",
                    "take my letter and deliver it",
                    "to my older brother, Thor?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Louis Greg",
                args![
                    "You can find Thor",
                    "next to Hermanthorn Jr.",
                    "inside the Rogue Guild.",
                    "I'd really appreciate it if",
                    "you could help my family."
                ],
            )?;
            return ctx.close();
        } else if ctx.var("rog_sk").get()? == 3 {
            ctx.lines_as(
                "Louis Greg",
                args![
                    "My elder brother, ^FF0000Thor^000000,",
                    "must know about this right",
                    "away! Please bring him this",
                    "letter for me. He should be",
                    "near ^FF0000Hermanthorn Jr.^000000 here",
                    "inside the Rogue Guild."
                ],
            )?;
            return ctx.close();
        } else if ctx.var("rog_sk").get()? == 4 {
            ctx.lines_as(
                "Louis Greg",
                args![
                    "You're looking for my",
                    "brother, Jay? He's usually",
                    "hanging out here in the",
                    "Rogue Guild with Antonio Jr."
                ],
            )?;
            return ctx.close();
        } else if ctx.var("rog_sk").get()? == 5 {
            ctx.lines_as(
                "Louis Greg",
                args![
                    "If you need to speak",
                    "to my father, he's still",
                    "probably in the hidden panic",
                    "room inside the Rogue Guild.",
                    "I still don't know where that",
                    "place could possibly be..."
                ],
            )?;
            return ctx.close();
        } else if ctx.var("rog_sk").get()? == 6 {
            ctx.lines_as(
                "Louis Greg",
                args![
                    "You want to learn the",
                    "secret Rogue skill? Oh,",
                    "you should probably talk",
                    "to ^FF0000Thor^000000 about that. I...",
                    "I'm really bad at explaining",
                    "things to people. Really bad."
                ],
            )?;
            return ctx.close();
        } else if ctx.var("rog_sk").get()? == 7 {
            ctx.lines_as(
                "Louis Greg",
                args![
                    "Hm? You're in the middle",
                    "of learning the secret Rogue",
                    "skill, aren't you? Ooh, then",
                    "you're not supposed to be here",
                    "just yet. Please go back and",
                    "talk to ^FF0000Thor^000000 again, okay?"
                ],
            )?;
            return ctx.close();
        } else if ctx.var("rog_sk").get()? == 8 {
            ctx.lines_as(
                "Louis Greg",
                args![
                    "Oh, oh!",
                    "You're done with",
                    "the training? Ah,",
                    "then you need to",
                    "talk to Thor again!"
                ],
            )?;
            return ctx.close();
        } else if ctx.var("rog_sk").get()? == 9 {
            ctx.lines_as(
                "Louis Greg",
                args![
                    "You wanted to learn",
                    "more about Close Confine?",
                    "Ugh, then you better speak",
                    "to my father. I just learned",
                    "that skill myself, you know."
                ],
            )?;
            return ctx.close();
        } else if ctx.var("rog_sk").get()? == 11 {
            ctx.lines_as(
                "Louis Greg",
                args![
                    "I just heard from my",
                    "brothers that Chae Takbae",
                    "developed Close Confine",
                    "as a way to brutally beat his",
                    "enemies, keeping them from",
                    "running away. Is that true?"
                ],
            )?;
            return ctx.close();
        }
        ctx.lines_as(
            "Louis Greg",
            args![
                "Wow, you're really",
                "great! I wish I were",
                "as powerful as you.",
                "I hate being a kid!"
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as(
        "Louis Greg",
        args![
            "Why'd I become a Rogue?",
            "I guess I just like being",
            "sneaky. That, and being",
            "moral and law abiding is",
            "just too tough, you know?"
        ],
    )?;
    return ctx.close();
}

pub fn thor_greg_rogueguild(ctx: &Ctx) -> Script {
    if ctx.var("BaseJob").get()? == constants::JOB_ROGUE || ctx.var("Class").get()? == constants::JOB_THIEF_HIGH {
        if ctx.var("rog_sk").get()?.number()? < 1 {
            ctx.lines_as(
                "Thor Greg",
                args![
                    "Where did father go?",
                    "I hope those weird men",
                    "didn't get to him. With",
                    "any luck, he's hidden in",
                    "the panic room, but still..."
                ],
            )?;
            ctx.var("rog_sk").set(Val::from(1))?;
            return ctx.close();
        } else if ctx.var("rog_sk").get()? == 1 {
            ctx.lines_as(
                "Thor Greg",
                args![
                    "Hm... My father might",
                    "be hiding in the panic",
                    "room. I've never been able",
                    "to find it, but Markie says",
                    "that the entrance is cleverly",
                    "hidden to her left. Hmmm..."
                ],
            )?;
            return ctx.close();
        } else if ctx.var("rog_sk").get()? == 2 {
            ctx.lines_as(
                "Thor Greg",
                args![
                    "Are you looking for",
                    "my little brother, ^FF0000Louis^000000?",
                    "Oh, he's always hanging",
                    "around that ^FF0000Hollgrehenn Jr.^000000",
                    "here in the Rogue Guild.",
                    "What did you need him for?"
                ],
            )?;
            return ctx.close();
        } else if ctx.var("rog_sk").get()? == 3 {
            ctx.lines_as(
                "Thor Greg",
                args!["What's this you're", "giving me? A letter", "from Louis? Let's see..."],
            )?;
            ctx.var("rog_sk").set(Val::from(4))?;
            ctx.next()?;
            ctx.lines_as(
                "Thor Greg",
                args![
                    "Oh God, father's in",
                    "serious trouble! I better",
                    "tell my older brother, Jay,",
                    "right away! Quick, find ^FF0000Jay^000000",
                    "next to ^FF0000Antonio Jr.^000000 here in",
                    "the guild! P-please hurry!"
                ],
            )?;
            return ctx.close();
        } else if ctx.var("rog_sk").get()? == 4 {
            ctx.lines_as(
                "Thor Greg",
                args![
                    "You can find Jay hanging",
                    "out with Antonio Jr. here",
                    "inside the Rogue Guild. ",
                    "Please bring him the letter",
                    "I've written as soon as possible!"
                ],
            )?;
            return ctx.close();
        } else if ctx.var("rog_sk").get()? == 5 {
            ctx.lines_as(
                "Thor Greg",
                args![
                    "Hopefully, father hasn't",
                    "gone outside of the Rogue",
                    "Guild's panic room. That",
                    "may be the only place",
                    "where he's safe..."
                ],
            )?;
            return ctx.close();
        } else if ctx.var("rog_sk").get()? == 6 {
            ctx.lines_as(
                "Thor Greg",
                args![
                    "What's that? Father",
                    "wanted me to teach you",
                    "the secret Rogue skill?",
                    "Alright, I'll train you in the",
                    "same way father did. Would",
                    "you like an explanation first?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Thor Greg",
                args![
                    "If you don't want an",
                    "explanation, I'll just",
                    "send you to the training",
                    "ground right away so that",
                    "you can learn ^FF0000Close Confine^000000."
                ],
            )?;
            ctx.next()?;
            if ctx.menu(&["Listen to Explanation", "Go to Training Ground"])? == 0 {
                ctx.lines_as(
                    "Thor Greg",
                    args![
                        "Alright, the very first step",
                        "to learning ^FF0000Close Confine^000000",
                        "is to master blocking your",
                        "enemy's movement. I'll send",
                        "you to a special training",
                        "ground so you can practice."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Thor Greg",
                    args![
                        "There, you'll encounter our",
                        "training partner. Approach her",
                        "closely and make sure that you",
                        "predict and block her movements",
                        "to the left, right or backward. Get ready, I'm sending you now..."
                    ],
                )?;
                ctx.close_window()?;
                ctx.warp("in_rogue", 89, 114)?;
                return ctx.end();
            }
            ctx.warp("in_rogue", 89, 114)?;
            return ctx.end();
        } else if ctx.var("rog_sk").get()? == 7 {
            ctx.lines_as(
                "Thor Greg",
                args![
                    "Alright, I'm going to break",
                    "the fourth wall here and assume",
                    "you were disconnected from the",
                    "game. Would you like to return",
                    "to the training ground in order",
                    "to learn ^FF0000Close Confine^000000?"
                ],
            )?;
            ctx.next()?;
            if ctx.menu(&["Yes, please.", "No, thanks."])? == 0 {
                ctx.lines_as(
                    "Thor Greg",
                    args![
                        "Alright, the very first step",
                        "to learning ^FF0000Close Confine^000000",
                        "is to master blocking your",
                        "enemy's movement. I'll send",
                        "you to a special training",
                        "ground so you can practice."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Thor Greg",
                    args![
                        "There, you'll encounter our",
                        "training partner. Approach her",
                        "closely and make sure that you",
                        "predict and block her movements",
                        "to the left, right or backward. Get ready, I'm sending you now..."
                    ],
                )?;
                ctx.var("rog_sk").set(Val::from(6))?;
                ctx.close_window()?;
                ctx.warp("in_rogue", 89, 114)?;
                return ctx.end();
            }
            ctx.lines_as(
                "Thor Greg",
                args![
                    "Alright, alright.",
                    "When you feel ready",
                    "to resume training,",
                    "just let me know."
                ],
            )?;
            return ctx.close();
        } else if ctx.var("rog_sk").get()? == 8 {
            ctx.lines_as(
                "Thor Greg",
                args![
                    "Ah, I hear from Kienna",
                    "that you've completed your",
                    "training. Congratulations!",
                    "Now, please speak to my",
                    "father so that he can explain the Close Confine skill in detail."
                ],
            )?;
            ctx.var("rog_sk").set(Val::from(9))?;
            return ctx.close();
        } else if ctx.var("rog_sk").get()? == 9 {
            ctx.lines_as(
                "Thor Greg",
                args![
                    "Please talk to my father",
                    "so that he can explain the",
                    "nuances of the Close Confine",
                    "skill to you. He should still be in the Rogue Guild's panic room."
                ],
            )?;
            return ctx.close();
        } else if ctx.var("rog_sk").get()? == 11 {
            ctx.lines_as(
                "Thor Greg",
                args![
                    "You know, Chae Takbae",
                    "would say, ''I'm Chae",
                    "Takbae. And you are...?''",
                    "to opponents, and right",
                    "before they could answer,",
                    "he'd beat them to a pulp."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Thor Greg", args!["He really is", "a legendary hero", "amongst Rogues...!"])?;
            return ctx.close();
        }
        ctx.lines_as(
            "Thor Greg",
            args![
                "Hm...?",
                "It seems that",
                "you're much stronger",
                "than even me. There's",
                "probably not too many",
                "people who'd mess with you..."
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as(
        "Thor Greg",
        args![
            "Hmm, gank this, gank",
            "that. *Sigh* It's my most",
            "shameful fault: I spend zeny",
            "almost as quickly as I can",
            "steal it. It's irresponsible..."
        ],
    )?;
    return ctx.close();
}

pub fn jay_greg_rogueguild(ctx: &Ctx) -> Script {
    if ctx.var("BaseJob").get()? == constants::JOB_ROGUE || ctx.var("Class").get()? == constants::JOB_THIEF_HIGH {
        if ctx.var("rog_sk").get()?.number()? < 1 {
            ctx.lines_as(
                "Jay Greg",
                args![
                    "My father must be hidden",
                    "in the Rogue Guild's panic",
                    "room. Strangely, my brothers",
                    "and I can never find it and",
                    "figure out whether he's safe..."
                ],
            )?;
            ctx.var("rog_sk").set(Val::from(1))?;
            ctx.next()?;
            ctx.lines_as(
                "Jay Greg",
                args![
                    "Recently, he's been pursued",
                    "by these strange men who've",
                    "been threatening our family.",
                    "Well, he did just teach us",
                    "a new skill, so he can use",
                    "that to protect himself..."
                ],
            )?;
            return ctx.close();
        } else if ctx.var("rog_sk").get()? == 1 {
            ctx.lines_as(
                "Jay Greg",
                args![
                    "I'm guessing my father",
                    "hid himself in the Rogue",
                    "Guild's panic room. We can",
                    "never find it, but supposedly",
                    "the entrance is hidden close",
                    "to Markie somewhere."
                ],
            )?;
            return ctx.close();
        } else if ctx.var("rog_sk").get()? == 2 {
            ctx.lines_as(
                "Jay Greg",
                args![
                    "You're looking for",
                    "Louis, my little brother?",
                    "He's here in the Rouge Guild,",
                    "so it shouldn't be too hard to",
                    "find him. He's probably just",
                    "standing near Hollgrehen Jr."
                ],
            )?;
            return ctx.close();
        } else if ctx.var("rog_sk").get()? == 3 {
            ctx.lines_as(
                "Jay Greg",
                args![
                    "Thor? He should be",
                    "around the Rogue Guild",
                    "somewhere. Have you tried",
                    "looking around Hermathorn Jr.?"
                ],
            )?;
            return ctx.close();
        } else if ctx.var("rog_sk").get()? == 4 {
            ctx.lines_as(
                "Jay Greg",
                args![
                    "A letter for me...?",
                    "Ah, it's from Thor, so",
                    "I guess I better read",
                    "it right away. Hmmm..."
                ],
            )?;
            ctx.var("rog_sk").set(Val::from(5))?;
            ctx.next()?;
            ctx.lines_as(
                "Jay Greg",
                args![
                    "I see... I must report",
                    "this to the Rogue Guild",
                    "right away, and send a reply",
                    "to my father. Hmm. Let me",
                    "write him a letter right now.",
                    "Please give me a moment..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Jay Greg",
                args![
                    "There, it's done. Please",
                    "give this to my father with",
                    "all the haste you can muster.",
                    "I know it's much to ask, but",
                    "I cannot find the way to the",
                    "hidden panic room myself..."
                ],
            )?;
            return ctx.close();
        } else if ctx.var("rog_sk").get()? == 5 {
            ctx.lines_as(
                "Jay Greg",
                args![
                    "Please give my reply",
                    "to my father as soon as",
                    "you can. He's still hidden",
                    "in the Rogue Guild's panic",
                    "room. That is, if our enemies",
                    "still haven't found him yet."
                ],
            )?;
            return ctx.close();
        } else if ctx.var("rog_sk").get()? == 6 {
            ctx.lines_as(
                "Jay Greg",
                args![
                    "You need to train for",
                    "the Close Confine skill?",
                    "I think Thor is the only",
                    "one with access to the",
                    "training ground, so",
                    "talk to him first."
                ],
            )?;
            return ctx.close();
        } else if ctx.var("rog_sk").get()? == 7 {
            ctx.lines_as(
                "Jay Greg",
                args![
                    "You need to train for",
                    "the Close Confine skill?",
                    "I think Thor is the only",
                    "one with access to the",
                    "training ground, so",
                    "talk to him first."
                ],
            )?;
            return ctx.close();
        } else if ctx.var("rog_sk").get()? == 8 {
            ctx.lines_as(
                "Jay Greg",
                args![
                    "Ah, you finished the",
                    "training for Close Confine,",
                    "did you? Great, now go and",
                    "tell my brother, Thor."
                ],
            )?;
            return ctx.close();
        } else if ctx.var("rog_sk").get()? == 9 {
            ctx.lines_as(
                "Jay Greg",
                args![
                    "Hm. You should probably",
                    "talk to my father to learn",
                    "more of the nuances about",
                    "the Close Confine skill.",
                    "Have you seen him in the",
                    "Rogue Guild's panic room?"
                ],
            )?;
            return ctx.close();
        } else if ctx.var("rog_sk").get()? == 12 {
            ctx.lines_as(
                "Jay Greg",
                args![
                    "Chae Takbae sure",
                    "seemed like a stubborn,",
                    "thuggish guy. But he must",
                    "have been pretty smart to",
                    "invent some of his own skills."
                ],
            )?;
            return ctx.close();
        }
        ctx.lines_as(
            "Jay Greg",
            args![
                "I get the feeling",
                "that you're going to",
                "be one of the best Rogues",
                "around, if you already aren't."
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as(
        "Jay Greg",
        args![
            "Ever since I learned",
            "Intimdate, I've gotten",
            "real punchy, maybe even",
            "masochistic. I mean, if they",
            "hit you with a skill, you can",
            "hit them back with it!"
        ],
    )?;
    return ctx.close();
}

#[derive(Clone, Copy, Debug)]
enum S1strecogStep {
    Start,
    OnTouch,
}

fn s_1strecog_run(ctx: &Ctx, mut step: S1strecogStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            S1strecogStep::Start => {
                step = S1strecogStep::OnTouch;
                continue 'machine;
            }
            S1strecogStep::OnTouch => {
                ctx.npc().do_event("#1st5min::OnEnable")?;
                ctx.npc().do_event("#1stmove::OnEnable")?;
                ctx.set_npc_visible("#1strecog", false)?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn s_1strecog(ctx: &Ctx) -> Script {
    s_1strecog_run(ctx, S1strecogStep::Start, Vec::new()).map(|_| ())
}

pub fn s_1strecog_ontouch(ctx: &Ctx) -> Script {
    s_1strecog_run(ctx, S1strecogStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum S1st5minStep {
    Start,
    OnInit,
    OnEnable,
    OnDisable,
    OnTimer1000,
    OnTimer290000,
    OnTimer310000,
    OnTimer315000,
}

fn s_1st5min_run(ctx: &Ctx, mut step: S1st5minStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            S1st5minStep::Start => {
                step = S1st5minStep::OnInit;
                continue 'machine;
            }
            S1st5minStep::OnInit => {
                ctx.set_npc_visible("#1st5min", false)?;
                return Err(Stop::End);
            }
            S1st5minStep::OnEnable => {
                ctx.set_npc_visible("#1st5min", true)?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            S1st5minStep::OnDisable => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.set_npc_visible("#1st5min", false)?;
                return Err(Stop::End);
            }
            S1st5minStep::OnTimer1000 => {
                ctx.call(
                    Function::MapAnnounce,
                    args![
                        "prt_are01",
                        "Welcome to the Close Confine Training Ground. You will be automatically teleported outside in 5 minutes.",
                        constants::BC_MAP,
                        "0x00ff00"
                    ],
                )?;
                return Err(Stop::End);
            }
            S1st5minStep::OnTimer290000 => {
                ctx.call(
                    Function::MapAnnounce,
                    args![
                        "prt_are01",
                        "You will be teleported outside in 20 seconds.",
                        constants::BC_MAP,
                        "0x00ff00"
                    ],
                )?;
                return Err(Stop::End);
            }
            S1st5minStep::OnTimer310000 => {
                ctx.call(
                    Function::MapAnnounce,
                    args![
                        "prt_are01",
                        "You will be teleported outside in 5 seconds.",
                        constants::BC_MAP,
                        "0x00ff00"
                    ],
                )?;
                return Err(Stop::End);
            }
            S1st5minStep::OnTimer315000 => {
                ctx.call(
                    Function::MapAnnounce,
                    args!["prt_are01", "You are now being teleported outside.", constants::BC_MAP, "0x00ff00"],
                )?;
                ctx.set_npc_visible("Kienna#1st", false)?;
                ctx.set_npc_visible("Kienna#2nd", false)?;
                ctx.set_npc_visible("Kienna#3rd", false)?;
                ctx.set_npc_visible("Kienna#4th", false)?;
                ctx.set_npc_visible("Kienna#5th", false)?;
                ctx.set_npc_visible("Kienna#6th", false)?;
                ctx.set_npc_visible("Kienna#7th", false)?;
                ctx.set_npc_visible("Kienna#8th", false)?;
                ctx.npc().do_event("#1stmove::OnDisable")?;
                ctx.set_npc_visible("#1strecog", true)?;
                ctx.npc().do_event("Waiting Room#rogue10::OnEnable")?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::MapWarp, args!["prt_are01", "in_rogue", 264, 124])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn s_1st5min(ctx: &Ctx) -> Script {
    s_1st5min_run(ctx, S1st5minStep::Start, Vec::new()).map(|_| ())
}

pub fn s_1st5min_oninit(ctx: &Ctx) -> Script {
    s_1st5min_run(ctx, S1st5minStep::OnInit, Vec::new()).map(|_| ())
}

pub fn s_1st5min_onenable(ctx: &Ctx) -> Script {
    s_1st5min_run(ctx, S1st5minStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn s_1st5min_ondisable(ctx: &Ctx) -> Script {
    s_1st5min_run(ctx, S1st5minStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn s_1st5min_ontimer1000(ctx: &Ctx) -> Script {
    s_1st5min_run(ctx, S1st5minStep::OnTimer1000, Vec::new()).map(|_| ())
}

pub fn s_1st5min_ontimer290000(ctx: &Ctx) -> Script {
    s_1st5min_run(ctx, S1st5minStep::OnTimer290000, Vec::new()).map(|_| ())
}

pub fn s_1st5min_ontimer310000(ctx: &Ctx) -> Script {
    s_1st5min_run(ctx, S1st5minStep::OnTimer310000, Vec::new()).map(|_| ())
}

pub fn s_1st5min_ontimer315000(ctx: &Ctx) -> Script {
    s_1st5min_run(ctx, S1st5minStep::OnTimer315000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum S1stmoveStep {
    Start,
    OnInit,
    OnEnable,
    OnTimer3000,
    OnTimer5000,
    OnTimer8000,
    OnTimer9000,
    OnDisable,
}

fn s_1stmove_run(ctx: &Ctx, mut step: S1stmoveStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            S1stmoveStep::Start => {
                step = S1stmoveStep::OnInit;
                continue 'machine;
            }
            S1stmoveStep::OnInit => {
                ctx.set_npc_visible("#1stmove", false)?;
                return Err(Stop::End);
            }
            S1stmoveStep::OnEnable => {
                ctx.set_npc_visible("#1stmove", true)?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            S1stmoveStep::OnTimer3000 => {
                ctx.call(
                    Function::MapAnnounce,
                    args![
                        "prt_are01",
                        "Kienna will appear in 1 second. Please approach her as closely as possible.",
                        constants::BC_MAP,
                        "0x00ff00"
                    ],
                )?;
                return Err(Stop::End);
            }
            S1stmoveStep::OnTimer5000 => {
                match ctx.rand_range(1, 8)? {
                    1 => ctx.set_npc_visible("Kienna#1st", true)?,
                    2 => ctx.set_npc_visible("Kienna#2nd", true)?,
                    3 => ctx.set_npc_visible("Kienna#3rd", true)?,
                    4 => ctx.set_npc_visible("Kienna#4th", true)?,
                    5 => ctx.set_npc_visible("Kienna#5th", true)?,
                    6 => ctx.set_npc_visible("Kienna#6th", true)?,
                    7 => ctx.set_npc_visible("Kienna#7th", true)?,
                    8 => ctx.set_npc_visible("Kienna#8th", true)?,
                    _ => {}
                }
                return Err(Stop::End);
            }
            S1stmoveStep::OnTimer8000 => {
                ctx.set_npc_visible("Kienna#1st", false)?;
                ctx.set_npc_visible("Kienna#2nd", false)?;
                ctx.set_npc_visible("Kienna#3rd", false)?;
                ctx.set_npc_visible("Kienna#4th", false)?;
                ctx.set_npc_visible("Kienna#5th", false)?;
                ctx.set_npc_visible("Kienna#6th", false)?;
                ctx.set_npc_visible("Kienna#7th", false)?;
                ctx.set_npc_visible("Kienna#8th", false)?;
                return Err(Stop::End);
            }
            S1stmoveStep::OnTimer9000 => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.npc().do_event("#1stmove::OnEnable")?;
                ctx.call(Function::MapWarp, args!["prt_are01", "prt_are01", 150, 150])?;
                return Err(Stop::End);
            }
            S1stmoveStep::OnDisable => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.set_npc_visible("#1stmove", false)?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn s_1stmove(ctx: &Ctx) -> Script {
    s_1stmove_run(ctx, S1stmoveStep::Start, Vec::new()).map(|_| ())
}

pub fn s_1stmove_oninit(ctx: &Ctx) -> Script {
    s_1stmove_run(ctx, S1stmoveStep::OnInit, Vec::new()).map(|_| ())
}

pub fn s_1stmove_onenable(ctx: &Ctx) -> Script {
    s_1stmove_run(ctx, S1stmoveStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn s_1stmove_ontimer3000(ctx: &Ctx) -> Script {
    s_1stmove_run(ctx, S1stmoveStep::OnTimer3000, Vec::new()).map(|_| ())
}

pub fn s_1stmove_ontimer5000(ctx: &Ctx) -> Script {
    s_1stmove_run(ctx, S1stmoveStep::OnTimer5000, Vec::new()).map(|_| ())
}

pub fn s_1stmove_ontimer8000(ctx: &Ctx) -> Script {
    s_1stmove_run(ctx, S1stmoveStep::OnTimer8000, Vec::new()).map(|_| ())
}

pub fn s_1stmove_ontimer9000(ctx: &Ctx) -> Script {
    s_1stmove_run(ctx, S1stmoveStep::OnTimer9000, Vec::new()).map(|_| ())
}

pub fn s_1stmove_ondisable(ctx: &Ctx) -> Script {
    s_1stmove_run(ctx, S1stmoveStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn kienna_1st(ctx: &Ctx) -> Script {
    return ctx.end();
}

pub fn kienna_1st_ontouch(ctx: &Ctx) -> Script {
    shared::quests_skills_rogue_skills::f_kienna(ctx, args![ctx.call(Function::StrNpcInfo, args![2])?])?;
    return ctx.end();
}

#[derive(Clone, Copy, Debug)]
enum WaitingRoomRogue10Step {
    Start,
    OnEnable,
    OnInit,
    OnStartArena,
}

fn waiting_room_rogue10_run(ctx: &Ctx, mut step: WaitingRoomRogue10Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            WaitingRoomRogue10Step::Start => {
                step = WaitingRoomRogue10Step::OnEnable;
                continue 'machine;
            }
            WaitingRoomRogue10Step::OnEnable => {
                ctx.set_npc_visible("Waiting Room#rogue10", true)?;
                ctx.call(Function::EnableWaitingRoomEvent, args!["Waiting Room#rogue10"])?;
                return Err(Stop::End);
            }
            WaitingRoomRogue10Step::OnInit => {
                ctx.call(
                    Function::WaitingRoom,
                    args!["Training Ground", 10, "Waiting Room#rogue10::OnStartArena", 1],
                )?;
                ctx.call(Function::EnableWaitingRoomEvent, args!["Waiting Room#rogue10"])?;
                return Err(Stop::End);
            }
            WaitingRoomRogue10Step::OnStartArena => {
                ctx.call(Function::WarpWaitingPc, args!["prt_are01", 150, 150])?;
                ctx.call(Function::DisableWaitingRoomEvent, args!["Waiting Room#rogue10"])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn waiting_room_rogue10(ctx: &Ctx) -> Script {
    waiting_room_rogue10_run(ctx, WaitingRoomRogue10Step::Start, Vec::new()).map(|_| ())
}

pub fn waiting_room_rogue10_onenable(ctx: &Ctx) -> Script {
    waiting_room_rogue10_run(ctx, WaitingRoomRogue10Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn waiting_room_rogue10_oninit(ctx: &Ctx) -> Script {
    waiting_room_rogue10_run(ctx, WaitingRoomRogue10Step::OnInit, Vec::new()).map(|_| ())
}

pub fn waiting_room_rogue10_onstartarena(ctx: &Ctx) -> Script {
    waiting_room_rogue10_run(ctx, WaitingRoomRogue10Step::OnStartArena, Vec::new()).map(|_| ())
}

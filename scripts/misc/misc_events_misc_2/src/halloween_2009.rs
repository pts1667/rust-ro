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

pub fn pumpkin_hat_researcher(ctx: &Ctx) -> Script {
    if ctx.player().base_level()? < 45 {
        ctx.lines_as(
            "Pumpkin Hat Researcher",
            args!["Shoo, I don't need a child. Shoo! I don't talk to novices."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Pumpkin Hat Researcher",
            args!["Go reach a level that can fight with stronger monsters and come back."],
        )?;
        return ctx.close();
    }
    ctx.lines_as("Pumpkin Hat Researcher", args!["Say do you like Pumpkin Pies?"])?;
    ctx.next()?;
    ctx.lines_as(
        "Pumpkin Hat Researcher",
        args!["I'm a Pumpkin Hat researcher, Why don't you listen to my story?"],
    )?;
    ctx.next()?;
    loop {
        match ctx.menu(&[
            "Listen to the story.",
            "Ask about Pumpkin Hat.",
            "Get a Pumpkin Hat.",
            "Stop the conversation.",
        ])? {
            0 => {
                ctx.lines_as(
                    "Pumpkin Hat Researcher",
                    args![
                        "I've been studying about an upgraded Pumpkin Hat.",
                        "I have discovered that it is a very simple process."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Pumpkin Hat Researcher",
                    args![
                        "The process is quite simple.",
                        "If you bring me ^4a4aff20 Jack o' Pumpkin^000000 I can show you.",
                        "Isn't that a tempting proposal?"
                    ],
                )?;
                ctx.next()?;
            }
            1 => {
                ctx.lines_as("Pumpkin Hat Researcher", args!["This upgraded pumpkin hat is powerful stuff!"])?;
                ctx.next()?;
                ctx.lines_as("Pumpkin Hat Researcher", args!["It can make a Pumpkin Pie that restores a large percentage of HP & SP using condensed energy to the person who wears it."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Pumpkin Hat Researcher",
                    args!["All you need is ^4a4aff20 Jack o' Pumpkin^000000s."],
                )?;
                ctx.next()?;
            }
            2 => {
                ctx.lines_as(
                    "Pumpkin Hat Researcher",
                    args!["Do you want to get Pumpkin Pies? Okay, let me count the Jack o' Pumpkins you've brought."],
                )?;
                ctx.next()?;
                if ctx.items().count(1062)? < 20 {
                    ctx.lines_as(
                        "Pumpkin Hat Researcher",
                        args![
                            "I need ^4a4aff20 Jack o' Pumpkin^000000.",
                            "I'm not an alchemist or a wizard to create something from nothing."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Pumpkin Hat Researcher",
                        args![
                            "Okay, go hunting monsters and come back.",
                            "I'm going to stay here for a while so take your time."
                        ],
                    )?;
                    ctx.next()?;
                } else {
                    ctx.lines_as(
                        "Pumpkin Hat Researcher",
                        args!["I hope this will be useful to you. Don't forget to wear it while fighting to get your Pumpkin Pies."],
                    )?;
                    ctx.items().take(1062, 20)?;
                    ctx.items().give(5668, 1)?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Pumpkin Hat Researcher",
                        args!["I guess that I should get back to my research."],
                    )?;
                    return ctx.close();
                }
            }
            3 => {
                ctx.lines_as(
                    "Pumpkin Hat Researcher",
                    args!["Bye, until we'll see each other again.", "I wish you well..."],
                )?;
                return ctx.close();
            }
            _ => {}
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum TrickOrTreaterStep {
    Start,
    OnTouch,
    OnInit,
    OnEnableTreat,
    OnTimer15000,
    OnTimer300000,
}

fn trick_or_treater_run(ctx: &Ctx, mut step: TrickOrTreaterStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            TrickOrTreaterStep::Start => {
                step = TrickOrTreaterStep::OnTouch;
                continue 'machine;
            }
            TrickOrTreaterStep::OnTouch => {
                ctx.call(Function::EnableNpc, args![ctx.call(Function::StrNpcInfo, args![3])?])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.lines_as("Trick or Treater", args!["Hooray! hooray! Hooray!", "Trick or Treat?"])?;
                ctx.next()?;
                if ctx.menu(&["Trick.", "Treat."])? == 0 {
                    ctx.lines_as(
                        "Trick or Treater",
                        args!["!!!!", "Fine. I have no choice but to trick you back!"],
                    )?;
                    ctx.call(Function::StartStatus, args![ctx.constant("SC_STUN")?, 5000, 0])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as("Trick or Treater", args!["Oh yay! What kind of treat do you have?"])?;
                ctx.next()?;
                let treat_item = match ctx.menu(&["Candy", "Candy Cane", "Well-baked Cookie", "Nothing"])? {
                    0 => Some(529),
                    1 => Some(530),
                    2 => Some(538),
                    _ => None,
                };
                if let Some(item) = treat_item {
                    if ctx.items().count(item)? > 0 {
                        ctx.lines_as(
                            "Trick or Treater",
                            args!["Yay thank you!", "Here, take this for being so nice!"],
                        )?;
                        for food in ["SC_STRFOOD", "SC_INTFOOD", "SC_VITFOOD", "SC_AGIFOOD", "SC_DEXFOOD", "SC_LUKFOOD"] {
                            ctx.call(Function::StartStatus, args![ctx.constant(food)?, 1800000, 5])?;
                        }
                        ctx.call(Function::StartStatus, args![ctx.constant("SC_FLEEFOOD")?, 1800000, 15])?;
                        ctx.items().take(item, 1)?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                ctx.lines_as(
                    "Trick or Treater",
                    args![
                        "At least a Candy, a Candy Cane or a Well-baked Cookie is all I ask for a treat.",
                        "Fine. I have no choice but to trick you!"
                    ],
                )?;
                ctx.call(Function::StartStatus, args![ctx.constant("SC_STUN")?, 5000, 0])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            TrickOrTreaterStep::OnInit => {
                for npc in [
                    "Trick or Treater#iRO1",
                    "Trick or Treater#iRO2",
                    "Trick or Treater#iRO3",
                    "Trick or Treater#iRO4",
                    "Trick or Treater#iRO5",
                    "Trick or Treater#iRO6",
                    "Trick or Treater#iRO7",
                    "Trick or Treater#iRO8",
                ] {
                    ctx.set_npc_visible(npc, false)?;
                }
                return Err(Stop::End);
            }
            TrickOrTreaterStep::OnEnableTreat => {
                ctx.call(Function::EnableNpc, args![ctx.call(Function::StrNpcInfo, args![3])?])?;
                ctx.call(Function::DisableNpc, args![ctx.call(Function::StrNpcInfo, args![3])?])?;
                return Err(Stop::End);
            }
            TrickOrTreaterStep::OnTimer15000 => {
                ctx.call(Function::DisableNpc, args![ctx.call(Function::StrNpcInfo, args![3])?])?;
                ctx.call(Function::DisableNpc, args![ctx.call(Function::StrNpcInfo, args![3])?])?;
                return Err(Stop::End);
            }
            TrickOrTreaterStep::OnTimer300000 => {
                ctx.call(
                    Function::DoNpcEvent,
                    args![ctx.call(Function::StrNpcInfo, args![3])? + Val::from("::OnEnableTreat")],
                )?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn trick_or_treater(ctx: &Ctx) -> Script {
    trick_or_treater_run(ctx, TrickOrTreaterStep::Start, Vec::new()).map(|_| ())
}

pub fn trick_or_treater_ontouch(ctx: &Ctx) -> Script {
    trick_or_treater_run(ctx, TrickOrTreaterStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn trick_or_treater_oninit(ctx: &Ctx) -> Script {
    trick_or_treater_run(ctx, TrickOrTreaterStep::OnInit, Vec::new()).map(|_| ())
}

pub fn trick_or_treater_onenabletreat(ctx: &Ctx) -> Script {
    trick_or_treater_run(ctx, TrickOrTreaterStep::OnEnableTreat, Vec::new()).map(|_| ())
}

pub fn trick_or_treater_ontimer15000(ctx: &Ctx) -> Script {
    trick_or_treater_run(ctx, TrickOrTreaterStep::OnTimer15000, Vec::new()).map(|_| ())
}

pub fn trick_or_treater_ontimer300000(ctx: &Ctx) -> Script {
    trick_or_treater_run(ctx, TrickOrTreaterStep::OnTimer300000, Vec::new()).map(|_| ())
}

pub fn halloween_wizard_iro09(ctx: &Ctx) -> Script {
    let mut l_hallowtown = Val::from(0);
    let mut l_hallowtowns_s: Vec<Val> = Vec::new();
    let mut l_mapname_s = Val::from("");
    ctx.lines_as("Halloween Wizard", args!["...", "Do you want to play a trick on someone?"])?;
    ctx.next()?;
    loop {
        match ctx.menu(&["What trick?", "Sure", "No."])? {
            0 => {
                ctx.lines_as(
                    "Halloween Wizard",
                    args!["I can summon monsters in other parts of the world with just a few materials."],
                )?;
                ctx.next()?;
                ctx.lines_as("Halloween Wizard", args!["Sounds interesting huh?"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Halloween Wizard",
                    args!["If you bring me Fabric, Jack o' Pumpkins, Worn Fabric, or Crushed Pumpkins I can summon the monsters."],
                )?;
                ctx.next()?;
            }
            1 => {
                ctx.lines_as("Halloween Wizard", args!["Which town do you want to play a trick on?"])?;
                ctx.next()?;
                let position = ctx
                    .call(Function::GetMapXy, args![constants::BL_PC, ctx.player().name()?])?
                    .into_array()
                    .ok_or_else(|| Stop::Error("Invalid position".into()))?;
                l_mapname_s = position[0].clone();
                if l_mapname_s == "prontera" {
                    l_hallowtown = match ctx.menu(&["Geffen", "Payon", "Alberta", "Aldebaran"])? {
                        0 => Val::from(3),
                        1 => Val::from(2),
                        2 => Val::from(4),
                        _ => Val::from(5),
                    };
                } else if l_mapname_s == "payon" {
                    l_hallowtown = match ctx.menu(&["Prontera", "Geffen", "Alberta", "Aldebaran"])? {
                        0 => Val::from(1),
                        1 => Val::from(3),
                        2 => Val::from(4),
                        _ => Val::from(5),
                    };
                } else if l_mapname_s == "geffen" {
                    l_hallowtown = match ctx.menu(&["Prontera", "Payon", "Alberta", "Aldebaran"])? {
                        0 => Val::from(1),
                        1 => Val::from(2),
                        2 => Val::from(4),
                        _ => Val::from(5),
                    };
                } else if l_mapname_s == "alberta" {
                    l_hallowtown = match ctx.menu(&["Prontera", "Geffen", "Payon", "Aldebaran"])? {
                        0 => Val::from(1),
                        1 => Val::from(3),
                        2 => Val::from(2),
                        _ => Val::from(5),
                    };
                } else if l_mapname_s == "aldebaran" {
                    l_hallowtown = match ctx.menu(&["Prontera", "Geffen", "Payon", "Alberta"])? {
                        0 => Val::from(1),
                        1 => Val::from(3),
                        2 => Val::from(2),
                        _ => Val::from(4),
                    };
                }
                runtime::local_set(&mut l_hallowtowns_s, &Val::from(1), Val::from("prontera"), true);
                runtime::local_set(&mut l_hallowtowns_s, &Val::from(2), Val::from("payon"), true);
                runtime::local_set(&mut l_hallowtowns_s, &Val::from(3), Val::from("geffen"), true);
                runtime::local_set(&mut l_hallowtowns_s, &Val::from(4), Val::from("alberta"), true);
                runtime::local_set(&mut l_hallowtowns_s, &Val::from(5), Val::from("aldebaran"), true);
                ctx.lines_as("Halloween Wizard", args!["Ok then let's go to the next step."])?;
                ctx.next()?;
                ctx.lines_as("Halloween Wizard", args!["How many Fabrics or Jack o' Pumpkins do you want to use? Don't go over 100 because that is the max amount that I can use."])?;
                ctx.next()?;
                let (input, _) = runtime::input_number(ctx, None, None)?;
                let mut l_input = input.number()?;
                if l_input == 0 {
                    ctx.lines_as(
                        "Halloween Wizard",
                        args!["You have no definite idea.", "It's not a big deal.", "Let me know."],
                    )?;
                    ctx.next()?;
                    continue;
                }
                if l_input > 100 {
                    ctx.lines_as(
                        "Halloween Wizard",
                        args!["I told you that it must be between 1 to 100!", "You didn't pay attention!"],
                    )?;
                    ctx.next()?;
                    continue;
                }
                let l_fabric = ctx.items().count(1059)?;
                let l_jack = ctx.items().count(1062)?;
                let l_worn = ctx.items().count(6299)?;
                let l_crushed = ctx.items().count(6298)?;
                let mut l_whispers = 0;
                let mut l_darklords = 0;
                if l_fabric + l_jack + l_worn + l_crushed < l_input {
                    ctx.lines_as(
                        "Halloween Wizard",
                        args!["Recount the number of items you have and tell me the total.", "Huhuhuhuhuhu..."],
                    )?;
                    ctx.next()?;
                    continue;
                }
                if l_fabric > 0 {
                    if l_fabric >= l_input {
                        ctx.call(Function::DelItem, args![1059, l_input])?;
                        l_whispers += l_input;
                        l_input = 0;
                    } else {
                        ctx.call(Function::DelItem, args![1059, l_fabric])?;
                        l_input -= l_fabric;
                        l_whispers += l_fabric;
                    }
                }
                if l_worn > 0 && l_input != 0 {
                    if l_worn >= l_input {
                        ctx.call(Function::DelItem, args![6299, l_input])?;
                        l_whispers += l_input;
                        l_input = 0;
                    } else {
                        ctx.call(Function::DelItem, args![6299, l_worn])?;
                        l_input -= l_worn;
                        l_whispers += l_worn;
                    }
                }
                if l_jack > 0 && l_input != 0 {
                    if l_jack >= l_input {
                        ctx.call(Function::DelItem, args![1062, l_input])?;
                        l_darklords += l_input;
                        l_input = 0;
                    } else {
                        ctx.call(Function::DelItem, args![1062, l_jack])?;
                        l_input -= l_jack;
                        l_darklords += l_jack;
                    }
                }
                if l_crushed > 0 && l_input != 0 {
                    if l_crushed >= l_input {
                        ctx.call(Function::DelItem, args![6298, l_input])?;
                        l_darklords += l_input;
                        l_input = 0;
                    } else {
                        ctx.call(Function::DelItem, args![6298, l_crushed])?;
                        l_input -= l_crushed;
                        l_darklords += l_crushed;
                    }
                }
                if l_input > 0 {
                    ctx.mes("Theres a problem.")?;
                    return ctx.close();
                }
                ctx.call(
                    Function::Monster,
                    args![
                        Val::from("") + runtime::local_get(&l_hallowtowns_s, &l_hallowtown, true) + Val::from(""),
                        0,
                        0,
                        "Halloween Whisper",
                        3014,
                        l_whispers,
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    args![
                        Val::from("") + runtime::local_get(&l_hallowtowns_s, &l_hallowtown, true) + Val::from(""),
                        0,
                        0,
                        "Halloween Dark Lord",
                        3015,
                        l_darklords,
                    ],
                )?;
                ctx.lines_as(
                    "Halloween Wizard",
                    args![
                        "Here's what you wanted.",
                        "Imagine what the people must be thinking in the other villages?",
                        "Muahahaha"
                    ],
                )?;
                return ctx.close();
            }
            2 => {
                ctx.lines_as(
                    "Halloween Wizard",
                    args![
                        "If you change your mind, come back here...",
                        "I'll stay here for a while...",
                        "Kkkk..."
                    ],
                )?;
                return ctx.close();
            }
            _ => {}
        }
    }
}

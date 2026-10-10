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
enum DimensionalGorgePieceStep {
    Start,
    LEnter,
}

fn dimensional_gorge_piece_run(ctx: &Ctx, mut step: DimensionalGorgePieceStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_md_name_s = Val::from("");
    let mut l_orctime = Val::from(0);
    let mut l_party_id = Val::from(0);
    'machine: loop {
        match step {
            DimensionalGorgePieceStep::Start => {
                l_party_id = ctx.call(Function::GetCharacterId, args![1])?;
                l_md_name_s = Val::from("Orc's Memory");
                if !(ctx
                    .call(Function::InstanceCheckParty, args![l_party_id.clone(), 2, 30, 80])?
                    .is_true())
                {
                    ctx.mes("Only users between Levels ^ff000030 ~ 80^000000 can enter this Dungeon.")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                l_orctime = ctx.call(Function::CheckQuest, args![12059, constants::PLAYTIME])?;
                if l_orctime == -1 {
                    if ctx.call(Function::IsPartyLeader, args![ctx.call(Function::GetCharacterId, args![1])?])? == 1 {
                        ctx.lines(args![
                            ((Val::from("Party status confirmed. Would you like to book entrance to the ") + l_md_name_s.clone())
                                + Val::from("?"))
                        ])?;
                        ctx.next()?;
                        'b1: {
                            let subject1 = Val::from(runtime::select_values(
                                ctx,
                                &[((Val::from("Reserve the ") + l_md_name_s.clone()) + Val::from(":Enter the Dungeon:Cancel"))],
                            )?);
                            let mut matched1 = false;
                            if !matched1 && subject1 == 1 {
                                matched1 = true;
                            }
                            if matched1 {
                                if ctx.call(Function::InstanceCreate, args![l_md_name_s.clone()])?.number()? < 0 {
                                    ctx.lines(args![
                                        (Val::from("Party Name: ") + ctx.call(Function::GetPartyName, args![l_party_id.clone()])?),
                                        (Val::from("Party Leader: ") + ctx.player().name()?),
                                        ((Val::from("^0000ff") + l_md_name_s.clone()) + Val::from(" ^000000 - Reservation Failed."))
                                    ])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                ctx.lines(args![((Val::from("^0000ff") + l_md_name_s.clone()) + Val::from("^000000- Attempting to book an entrance")), ((Val::from("After making a reservation, you have to select 'Enter the Dungeon' from the menu if you wish to enter the ") + l_md_name_s.clone()) + Val::from("."))])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            if !matched1 && subject1 == 2 {
                                matched1 = true;
                            }
                            if matched1 {
                                dimensional_gorge_piece_run(ctx, DimensionalGorgePieceStep::LEnter, args![0])?;
                            }
                            if !matched1 && subject1 == 3 {
                                matched1 = true;
                            }
                            if matched1 {
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                    }
                    if Val::from(runtime::select_values(
                        ctx,
                        &[(l_md_name_s.clone() + Val::from(" Enter the Memorial Dungeon:Cancel"))],
                    )?) == 2
                    {
                        return Err(Stop::End);
                    }
                    dimensional_gorge_piece_run(ctx, DimensionalGorgePieceStep::LEnter, args![1])?;
                } else if l_orctime == 0 || l_orctime == 1 {
                    ctx.mes("You can enter the Dungeon if it has been generated.")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(
                        ctx,
                        &[((Val::from("Enter the Dungeon ") + l_md_name_s.clone()) + Val::from(":Cancel"))],
                    )?) == 2
                    {
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    dimensional_gorge_piece_run(ctx, DimensionalGorgePieceStep::LEnter, args![0])?;
                } else if l_orctime == 2 {
                    ctx.mes("^0000ffAll records and after-effects related to the Orc's Memory Dungeon are deleted. You can now regenerate or re-enter the dungeon.^000000")?;
                    ctx.quests().erase(12059)?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.mes("In order to generate a dungeon you must be the Party Leader and have at least 2 members in the party.")?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            DimensionalGorgePieceStep::LEnter => {
                let subject2 = ctx.call(Function::InstanceEnter, args!["Orc's Memory"])?;
                if subject2 == constants::IE_OTHER {
                    ctx.mes("An unknown error has occurred.")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if subject2 == constants::IE_NOINSTANCE {
                    ctx.lines(args!["Memorial Dungeon Orc's Memory does not exist.", "Memorial Dungeon has been destroyed by the Party Leader, or because of the time limit. Please try again after 2 hours."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if subject2 == constants::IE_NOMEMBER {
                    ctx.mes("Only a member of the party can enter the Memorial Dungeon.")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if subject2 == constants::IE_OK {
                    ctx.call(
                        Function::MapAnnounce,
                        args![
                            "gef_fild10",
                            (((ctx.call(Function::GetPartyName, args![ctx.call(Function::GetCharacterId, args![1])?],)?
                                + Val::from(" party's member "))
                                + ctx.player().name()?)
                                + Val::from(" has entered the Orc's Memory.")),
                            constants::BC_MAP,
                            "0x00ff99"
                        ],
                    )?;
                    if ctx.call(Function::CheckQuest, args![12059])? == -1 {
                        ctx.call(Function::SetQuest, args![12059])?;
                    }
                    if runtime::arg(&args, 0, Val::from(0)) == 0 {
                        ctx.close_window()?;
                    }
                    return Err(Stop::End);
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn dimensional_gorge_piece(ctx: &Ctx) -> Script {
    dimensional_gorge_piece_run(ctx, DimensionalGorgePieceStep::Start, Vec::new()).map(|_| ())
}

pub fn mad_scientist_orc(ctx: &Ctx) -> Script {
    if ctx.player().base_level()? < 50 {
        ctx.mes("The Mad Scientist doesn't seem to notice you and keeps mumbling to himself...")?;
        return ctx.close();
    }
    if ctx.var("mad").get()? == 0 {
        ctx.lines_as(
            "Mad Scientist",
            args![
                "Haha, no Orcs are coming near me!",
                "The power of the Dimensional Gorge is undefeatable!!"
            ],
        )?;
        ctx.next()?;
        if ctx.menu(&["Orcs don't attack you?", "You wish!"])? == 0 {
            ctx.lines_as(
                "Mad Scientist",
                args![
                    "Yeah, yeah.. I know it's hard to belive. I don't blame ya.",
                    "I used to study the Dimensional Gorge discovered near Morocc.",
                    "According to my research, the minerals found there have enormous power inside them!"
                ],
            )?;
            ctx.next()?;
        } else {
            ctx.lines_as("Mad Scientist", args!["You are a pretty distrustful person, huh?"])?;
            ctx.next()?;
        }
        let choice = runtime::select_values(ctx, &[Val::from("Oh? Like what, travel to alternate space?")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Mad Scientist",
            args!["Very smart! I have actually just done that! See that statue over there? It's not your normal Monolith!"],
        )?;
        ctx.next()?;
        ctx.lines_as("Mad Scientist", args!["I have created it with a combination of our technology and the unlimited energy I discovered from the Gorge! Want to try it?"])?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("...Um, that doesn't sound legal?")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Mad Scientist",
            args!["Hey, since when did something so interesting have to be legal!? How about it! You can talk to the Orcs! C'mon!"],
        )?;
        ctx.next()?;
        ctx.lines_as("Mad Scientist", args!["Chances are they'll just give you lots of instructions once they see you, so just wing it! They're always saying the same thing, to the point where I memorized them!"])?;
        ctx.next()?;
        if ctx.menu(&["Alright... What do I do?", "Umm, maybe not."])? == 0 {
            ctx.lines_as(
                "Mad Scientist",
                args!["Very good! Just sit in the hands of this statue! And try to act natural when you're in there!"],
            )?;
            ctx.var("mad").set(Val::from(1))?;
            return ctx.close();
        }
        ctx.lines_as("Mad Scientist", args!["Oh fine, be that way!"])?;
        return ctx.close();
    } else if ctx.var("mad").get()? == 1 {
        ctx.lines_as(
            "Mad Scientist",
            args!["Hmm? Put your hand on that Monemus Statue if you want to experience traveling through dimensions!"],
        )?;
        return ctx.close();
    } else {
        ctx.var("mad").set(Val::from(1))?;
        return ctx.end();
    }
    Ok(())
}

pub fn resurrect_monsters1(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn resurrect_monsters1_oninstanceinit(ctx: &Ctx) -> Script {
    ctx.call(
        Function::DisableNpc,
        args![ctx.call(Function::InstanceNpcName, args!["#Resurrect Monsters1"])?],
    )?;
    ctx.end()
}

pub fn resurrect_monsters1_ondisable(ctx: &Ctx) -> Script {
    ctx.call(Function::StopNpcTimer, args![])?;
    ctx.call(
        Function::KillMonster,
        args![
            ctx.call(Function::InstanceMapName, args!["1@orcs"])?,
            (ctx.call(Function::InstanceNpcName, args!["#Resurrect Monsters1"])? + Val::from("::OnMyMobDead"))
        ],
    )?;
    ctx.end()
}

pub fn resurrect_monsters1_onenable(ctx: &Ctx) -> Script {
    ctx.call(
        Function::EnableNpc,
        args![ctx.call(Function::InstanceNpcName, args!["#Resurrect Monsters1"])?],
    )?;
    ctx.call(
        Function::Monster,
        args![
            ctx.call(Function::InstanceMapName, args!["1@orcs"])?,
            0,
            0,
            "Orc Warrior",
            1023,
            30,
            (ctx.call(Function::InstanceNpcName, args!["#Resurrect Monsters1"])? + Val::from("::OnMyMobDead"))
        ],
    )?;
    ctx.end()
}

pub fn resurrect_monsters1_onmymobdead(ctx: &Ctx) -> Script {
    let l_map_s = ctx.call(Function::InstanceMapName, args!["1@orcs"])?;
    let l_mob_dead_num = Val::from(30).try_sub(ctx.call(
        Function::MobCount,
        args![
            l_map_s.clone(),
            (ctx.call(Function::InstanceNpcName, args!["#Resurrect Monsters1"])? + Val::from("::OnMyMobDead"))
        ],
    )?)?;
    let mut l_mob_ran = Val::from(0);
    if l_mob_dead_num.number()? > 0 {
        l_mob_ran = ctx.call(Function::Rand, args![1, 30])?;
        if l_mob_ran.number()? > 29 {
            ctx.call(
                Function::Monster,
                args![
                    l_map_s.clone(),
                    0,
                    0,
                    "Orc Warrior",
                    1023,
                    l_mob_dead_num.clone(),
                    (ctx.call(Function::InstanceNpcName, args!["#Resurrect Monsters1"])? + Val::from("::OnMyMobDead"))
                ],
            )?;
        } else if l_mob_ran.number()? > 28 && l_mob_ran.number()? < 30 {
            ctx.call(
                Function::Monster,
                args![
                    l_map_s.clone(),
                    0,
                    0,
                    "High Orc",
                    1213,
                    l_mob_dead_num.clone(),
                    (ctx.call(Function::InstanceNpcName, args!["#Resurrect Monsters1"])? + Val::from("::OnMyMobDead"))
                ],
            )?;
            if ctx.call(Function::Rand, args![1, 10])? == 9 {
                ctx.call(
                    Function::MapAnnounce,
                    args![
                        l_map_s.clone(),
                        "High Orc: We need more defenses! Get more people here!",
                        constants::BC_MAP,
                        "0xff4444"
                    ],
                )?;
            }
        } else if l_mob_ran.number()? > 26 && l_mob_ran.number()? < 29 {
            ctx.call(
                Function::AreaMonster,
                args![
                    l_map_s.clone(),
                    41,
                    91,
                    51,
                    81,
                    "High Orc",
                    1213,
                    l_mob_dead_num.clone(),
                    (ctx.call(Function::InstanceNpcName, args!["#Resurrect Monsters1"])? + Val::from("::OnMyMobDead"))
                ],
            )?;
            if ctx.call(Function::Rand, args![1, 10])? == 9 {
                ctx.call(
                    Function::MapAnnounce,
                    args![
                        l_map_s.clone(),
                        "Where are the High Orcs!? Get them to stop the enemies!",
                        constants::BC_MAP,
                        "0xff4444"
                    ],
                )?;
            }
        } else {
            ctx.call(
                Function::AreaMonster,
                args![
                    l_map_s.clone(),
                    17,
                    187,
                    27,
                    177,
                    "High Orc",
                    1213,
                    l_mob_dead_num.clone(),
                    (ctx.call(Function::InstanceNpcName, args!["#Resurrect Monsters1"])? + Val::from("::OnMyMobDead"))
                ],
            )?;
            if ctx.call(Function::Rand, args![1, 5])? == 3 {
                ctx.call(
                    Function::MapAnnounce,
                    args![
                        l_map_s.clone(),
                        "Caution: The army's starting to concentrate at Zone No. 4.",
                        constants::BC_MAP,
                        "0x77ff77"
                    ],
                )?;
            }
            if ctx.call(Function::Rand, args![1, 100])? == 50 {
                ctx.call(Function::InitNpcTimer, args![])?;
            }
        }
    }
    ctx.end()
}

pub fn resurrect_monsters1_ontimer10(ctx: &Ctx) -> Script {
    ctx.call(
        Function::MapAnnounce,
        args![
            ctx.call(Function::InstanceMapName, args!["1@orcs"])?,
            "Shouts of the Chief Orc of Safeguards: Looks like this will take longer than expected. Summon the Stalactic Golems!",
            constants::BC_MAP,
            "0xff4444"
        ],
    )?;
    ctx.end()
}

pub fn resurrect_monsters1_ontimer4010(ctx: &Ctx) -> Script {
    let l_map_s = ctx.call(Function::InstanceMapName, args!["1@orcs"])?;
    ctx.call(
        Function::MapAnnounce,
        args![
            l_map_s.clone(),
            "Stalactic Golems are digging out of the deep underground.",
            constants::BC_MAP,
            "0x77ff77"
        ],
    )?;
    ctx.call(
        Function::AreaMonster,
        args![
            l_map_s.clone(),
            17,
            187,
            27,
            177,
            "Stalactic Golem",
            1278,
            20,
            (ctx.call(Function::InstanceNpcName, args!["#Resurrect Monsters1"])? + Val::from("::OnMyMobDead"))
        ],
    )?;
    ctx.call(Function::StopNpcTimer, args![])?;
    ctx.end()
}

pub fn resurrect_monsters2(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn resurrect_monsters2_oninstanceinit(ctx: &Ctx) -> Script {
    ctx.call(
        Function::DisableNpc,
        args![ctx.call(Function::InstanceNpcName, args!["#Resurrect Monsters2"])?],
    )?;
    ctx.end()
}

pub fn resurrect_monsters2_ondisable(ctx: &Ctx) -> Script {
    ctx.call(
        Function::KillMonster,
        args![
            ctx.call(Function::InstanceMapName, args!["1@orcs"])?,
            (ctx.call(Function::InstanceNpcName, args!["#Resurrect Monsters2"])? + Val::from("::OnMyMobDead"))
        ],
    )?;
    ctx.end()
}

pub fn resurrect_monsters2_onenable(ctx: &Ctx) -> Script {
    ctx.call(
        Function::EnableNpc,
        args![ctx.call(Function::InstanceNpcName, args!["#Resurrect Monsters2"])?],
    )?;
    ctx.call(
        Function::Monster,
        args![
            ctx.call(Function::InstanceMapName, args!["1@orcs"])?,
            0,
            0,
            "Trained Wolf",
            1106,
            15,
            (ctx.call(Function::InstanceNpcName, args!["#Resurrect Monsters2"])? + Val::from("::OnMyMobDead"))
        ],
    )?;
    ctx.end()
}

pub fn resurrect_monsters2_onmymobdead(ctx: &Ctx) -> Script {
    let l_map_s = ctx.call(Function::InstanceMapName, args!["1@orcs"])?;
    let l_mob_dead_num = Val::from(15).try_sub(ctx.call(
        Function::MobCount,
        args![
            l_map_s.clone(),
            (ctx.call(Function::InstanceNpcName, args!["#Resurrect Monsters2"])? + Val::from("::OnMyMobDead"))
        ],
    )?)?;
    if ctx.rand_range(1, 30)? > 15 {
        if l_mob_dead_num.number()? > 0 {
            ctx.call(
                Function::Monster,
                args![
                    l_map_s.clone(),
                    0,
                    0,
                    "Trained Wolf",
                    1106,
                    l_mob_dead_num.clone(),
                    (ctx.call(Function::InstanceNpcName, args!["#Resurrect Monsters2"])? + Val::from("::OnMyMobDead"))
                ],
            )?;
        }
    } else if l_mob_dead_num.number()? > 0 {
        ctx.call(
            Function::AreaMonster,
            args![
                l_map_s.clone(),
                17,
                187,
                27,
                177,
                "Trained Wolf",
                1106,
                l_mob_dead_num.clone(),
                (ctx.call(Function::InstanceNpcName, args!["#Resurrect Monsters2"])? + Val::from("::OnMyMobDead"))
            ],
        )?;
    }
    ctx.end()
}

pub fn resurrect_monsters3(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn resurrect_monsters3_oninstanceinit(ctx: &Ctx) -> Script {
    ctx.call(
        Function::DisableNpc,
        args![ctx.call(Function::InstanceNpcName, args!["#Resurrect Monsters3"])?],
    )?;
    ctx.end()
}

pub fn resurrect_monsters3_ondisable(ctx: &Ctx) -> Script {
    ctx.call(
        Function::KillMonster,
        args![
            ctx.call(Function::InstanceMapName, args!["1@orcs"])?,
            (ctx.call(Function::InstanceNpcName, args!["#Resurrect Monsters3"])? + Val::from("::OnMyMobDead"))
        ],
    )?;
    ctx.end()
}

pub fn resurrect_monsters3_onenable(ctx: &Ctx) -> Script {
    ctx.call(
        Function::EnableNpc,
        args![ctx.call(Function::InstanceNpcName, args!["#Resurrect Monsters3"])?],
    )?;
    ctx.call(
        Function::Monster,
        args![
            ctx.call(Function::InstanceMapName, args!["1@orcs"])?,
            0,
            0,
            "Orc Archer",
            1189,
            15,
            (ctx.call(Function::InstanceNpcName, args!["#Resurrect Monsters3"])? + Val::from("::OnMyMobDead"))
        ],
    )?;
    ctx.end()
}

pub fn resurrect_monsters3_onmymobdead(ctx: &Ctx) -> Script {
    let l_map_s = ctx.call(Function::InstanceMapName, args!["1@orcs"])?;
    let l_mob_dead_num = Val::from(15).try_sub(ctx.call(
        Function::MobCount,
        args![
            l_map_s.clone(),
            (ctx.call(Function::InstanceNpcName, args!["#Resurrect Monsters3"])? + Val::from("::OnMyMobDead"))
        ],
    )?)?;
    let mut l_mob_ran = Val::from(0);
    l_mob_ran = ctx.call(Function::Rand, args![1, 30])?;
    if l_mob_ran.number()? > 29 {
        if l_mob_dead_num.number()? > 0 {
            ctx.call(
                Function::Monster,
                args![
                    l_map_s.clone(),
                    0,
                    0,
                    "Orc Archer",
                    1189,
                    l_mob_dead_num.clone(),
                    (ctx.call(Function::InstanceNpcName, args!["#Resurrect Monsters3"])? + Val::from("::OnMyMobDead"))
                ],
            )?;
        }
    } else if l_mob_ran.number()? > 26 && l_mob_ran.number()? < 30 {
        if l_mob_dead_num.number()? > 0 {
            ctx.call(
                Function::AreaMonster,
                args![
                    l_map_s.clone(),
                    43,
                    155,
                    47,
                    159,
                    "Orc Archer",
                    1189,
                    l_mob_dead_num.clone(),
                    (ctx.call(Function::InstanceNpcName, args!["#Resurrect Monsters3"])? + Val::from("::OnMyMobDead"))
                ],
            )?;
            if ctx.call(Function::Rand, args![1, 3])? == 3 {
                ctx.call(
                    Function::MapAnnounce,
                    args![
                        l_map_s.clone(),
                        "High Orc: Attack them from behind! Cut off their support!",
                        constants::BC_MAP,
                        "0xff4444"
                    ],
                )?;
            }
        }
    } else if l_mob_dead_num.number()? > 0 {
        ctx.call(
            Function::AreaMonster,
            args![
                l_map_s.clone(),
                17,
                187,
                27,
                177,
                "Orc Archer",
                1189,
                l_mob_dead_num.clone(),
                (ctx.call(Function::InstanceNpcName, args!["#Resurrect Monsters3"])? + Val::from("::OnMyMobDead"))
            ],
        )?;
    }
    ctx.end()
}

pub fn resurrect_monsters4(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn resurrect_monsters4_oninstanceinit(ctx: &Ctx) -> Script {
    ctx.call(
        Function::EnableNpc,
        args![ctx.call(Function::InstanceNpcName, args!["#Resurrect Monsters4"])?],
    )?;
    ctx.call(
        Function::AreaMonster,
        args![
            ctx.call(Function::InstanceMapName, args!["1@orcs"])?,
            98,
            35,
            178,
            115,
            "Anopheles",
            1627,
            10,
            (ctx.call(Function::InstanceNpcName, args!["#Resurrect Monsters4"])? + Val::from("::OnMyMobDead"))
        ],
    )?;
    ctx.end()
}

pub fn resurrect_monsters4_onmymobdead(ctx: &Ctx) -> Script {
    let l_map_s = ctx.call(Function::InstanceMapName, args!["1@orcs"])?;
    let l_mob_dead_num = Val::from(10).try_sub(ctx.call(
        Function::MobCount,
        args![
            l_map_s.clone(),
            (ctx.call(Function::InstanceNpcName, args!["#Resurrect Monsters4"])? + Val::from("::OnMyMobDead"))
        ],
    )?)?;
    if l_mob_dead_num.number()? > 0 {
        ctx.call(
            Function::Monster,
            args![
                l_map_s.clone(),
                0,
                0,
                "Anopheles",
                1627,
                l_mob_dead_num.clone(),
                (ctx.call(Function::InstanceNpcName, args!["#Resurrect Monsters4"])? + Val::from("::OnMyMobDead"))
            ],
        )?;
    }
    ctx.end()
}

pub fn resurrect_monsters4_ondisable(ctx: &Ctx) -> Script {
    ctx.call(
        Function::KillMonster,
        args![
            ctx.call(Function::InstanceMapName, args!["1@orcs"])?,
            (ctx.call(Function::InstanceNpcName, args!["#Resurrect Monsters4"])? + Val::from("::OnMyMobDead"))
        ],
    )?;
    ctx.end()
}

pub fn kruger_1_1(ctx: &Ctx) -> Script {
    ctx.call(
        Function::DoNpcEvent,
        args![(ctx.call(Function::InstanceNpcName, args!["Kruger#1-2"])? + Val::from("::OnEnable"))],
    )?;
    ctx.end()
}

pub fn kruger_1_2(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn kruger_1_2_oninstanceinit(ctx: &Ctx) -> Script {
    ctx.call(
        Function::DisableNpc,
        args![ctx.call(Function::InstanceNpcName, args!["Kruger#1-2"])?],
    )?;
    ctx.end()
}

pub fn kruger_1_2_onenable(ctx: &Ctx) -> Script {
    ctx.call(
        Function::DisableNpc,
        args![ctx.call(Function::InstanceNpcName, args!["Kruger#1-1"])?],
    )?;
    ctx.call(
        Function::EnableNpc,
        args![ctx.call(Function::InstanceNpcName, args!["Kruger#1-2"])?],
    )?;
    ctx.call(Function::InitNpcTimer, args![])?;
    ctx.end()
}

pub fn kruger_1_2_ontimer10(ctx: &Ctx) -> Script {
    ctx.call(
        Function::MapAnnounce,
        args![
            ctx.call(Function::InstanceMapName, args!["1@orcs"])?,
            "Kruger: Damn... What took you so long!! I don't have all day!!",
            constants::BC_MAP,
            "0xffff00"
        ],
    )?;
    ctx.end()
}

pub fn kruger_1_2_ontimer5710(ctx: &Ctx) -> Script {
    ctx.call(
        Function::MapAnnounce,
        args![
            ctx.call(Function::InstanceMapName, args!["1@orcs"])?,
            "Kruger: My plan was to let our comrades open the gate, but it's all ruined since I got busted by the Orc Shaman.",
            constants::BC_MAP,
            "0xffff00"
        ],
    )?;
    ctx.end()
}

pub fn kruger_1_2_ontimer14610(ctx: &Ctx) -> Script {
    ctx.call(
        Function::MapAnnounce,
        args![
            ctx.call(Function::InstanceMapName, args!["1@orcs"])?,
            "Shouts of the Chief Orc of Safeguards: I smell a rat.. Send some patrols to the entrance!!",
            constants::BC_MAP,
            "0xff4444"
        ],
    )?;
    ctx.end()
}

pub fn kruger_1_2_ontimer20210(ctx: &Ctx) -> Script {
    ctx.call(
        Function::MapAnnounce,
        args![
            ctx.call(Function::InstanceMapName, args!["1@orcs"])?,
            "Kruger: Darn it.. They'll be here any minute. Ok. Listen to me now.",
            constants::BC_MAP,
            "0xffff00"
        ],
    )?;
    ctx.end()
}

pub fn kruger_1_2_ontimer24910(ctx: &Ctx) -> Script {
    ctx.call(Function::MapAnnounce, args![ctx.call(Function::InstanceMapName, args!["1@orcs"])?, "Kruger: The Orc Shaman has sealed the 1st basement by dividing it into 4 zones. Each zone has one Enchanted Orc who has the power to unseal the next zone.", constants::BC_MAP, "0xffff00"])?;
    ctx.end()
}

pub fn kruger_1_2_ontimer34310(ctx: &Ctx) -> Script {
    ctx.call(
        Function::MapAnnounce,
        args![
            ctx.call(Function::InstanceMapName, args!["1@orcs"])?,
            "Kruger: Find those Enchanted Orcs and get rid of them to move to the next zone.",
            constants::BC_MAP,
            "0xffff00"
        ],
    )?;
    ctx.end()
}

pub fn kruger_1_2_ontimer39710(ctx: &Ctx) -> Script {
    ctx.call(Function::MapAnnounce, args![ctx.call(Function::InstanceMapName, args!["1@orcs"])?, "Kruger: Try to avoid encountering Orcs other then the Enchanted ones. Everytime you kill a normal Orc, High Orcs will gather at the last path to the 2nd floor.", constants::BC_MAP, "0xffff00"])?;
    ctx.end()
}

pub fn kruger_1_2_ontimer49210(ctx: &Ctx) -> Script {
    ctx.call(Function::MapAnnounce, args![ctx.call(Function::InstanceMapName, args!["1@orcs"])?, "Kruger: In the worst case, the path to the 2nd floor could be completely blocked. For your own sake, you should be as sneaky as possible.", constants::BC_MAP, "0xffff00"])?;
    ctx.end()
}

pub fn kruger_1_2_ontimer56310(ctx: &Ctx) -> Script {
    ctx.call(Function::MapAnnounce, args![ctx.call(Function::InstanceMapName, args!["1@orcs"])?, "Mission: Sneak in and get rid of the 'Enchanted Orcs'. Avoiding battles with other Orcs is the best way of getting into the 2nd floor.", constants::BC_MAP, "0x44ffff"])?;
    ctx.call(
        Function::DoNpcEvent,
        args![(ctx.call(Function::InstanceNpcName, args!["#Resurrect Monsters1"])? + Val::from("::OnEnable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        args![(ctx.call(Function::InstanceNpcName, args!["#Resurrect Monsters2"])? + Val::from("::OnEnable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        args![(ctx.call(Function::InstanceNpcName, args!["#Resurrect Monsters3"])? + Val::from("::OnEnable"))],
    )?;
    ctx.call(
        Function::DisableNpc,
        args![ctx.call(Function::InstanceNpcName, args!["Kruger#1-2"])?],
    )?;
    ctx.end()
}

pub fn kruger_1_2_ontimer60000(ctx: &Ctx) -> Script {
    ctx.call(
        Function::AreaMonster,
        args![
            ctx.call(Function::InstanceMapName, args!["1@orcs"])?,
            137,
            83,
            143,
            89,
            "Enchanted Orc",
            1023,
            1,
            (ctx.call(Function::InstanceNpcName, args!["B1 Area Mobs"])? + Val::from("::OnMyMobDead"))
        ],
    )?;
    ctx.call(Function::StopNpcTimer, args![])?;
    ctx.end()
}

#[derive(Clone, Copy, Debug)]
enum B1Area1Step {
    Start,
    OnInstanceInit,
    OnEnable,
    OnTouch,
    OnContinue,
    OnTimer10300,
    OnTimer18700,
}

fn b1_area_1_run(ctx: &Ctx, mut step: B1Area1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            B1Area1Step::Start => {
                step = B1Area1Step::OnInstanceInit;
                continue 'machine;
            }
            B1Area1Step::OnInstanceInit => {
                ctx.call(
                    Function::DisableNpc,
                    args![ctx.call(Function::InstanceNpcName, args!["B1 Area 1"])?],
                )?;
                return Err(Stop::End);
            }
            B1Area1Step::OnEnable => {
                ctx.call(
                    Function::EnableNpc,
                    args![ctx.call(Function::InstanceNpcName, args!["B1 Area 1"])?],
                )?;
                ctx.call(
                    Function::AreaMonster,
                    args![
                        ctx.call(Function::InstanceMapName, args!["1@orcs"])?,
                        103,
                        105,
                        109,
                        111,
                        "Enchanted Orc",
                        1023,
                        1,
                        (ctx.call(Function::InstanceNpcName, args!["B1 Area Mobs"])? + Val::from("::OnMyMobDead1"))
                    ],
                )?;
                return Err(Stop::End);
            }
            B1Area1Step::OnTouch => {
                ctx.call(
                    Function::Warp,
                    args![ctx.call(Function::InstanceMapName, args!["1@orcs"])?, 168, 130],
                )?;
                return Err(Stop::End);
            }
            B1Area1Step::OnContinue => {
                ctx.call(
                    Function::DoNpcEvent,
                    args![(ctx.call(Function::InstanceNpcName, args!["B1 Area 2"])? + Val::from("::OnEnable"))],
                )?;
                ctx.call(Function::InitNpcTimer, args![])?;
                return Err(Stop::End);
            }
            B1Area1Step::OnTimer10300 => {
                ctx.call(Function::MapAnnounce, args![ctx.call(Function::InstanceMapName, args!["1@orcs"])?, "Kruger's Whisper: The Orcs here used to be my companions. They just lost their will ever since the Orc Shaman started to control them with her magic.", constants::BC_MAP, "0xff4499"])?;
                return Err(Stop::End);
            }
            B1Area1Step::OnTimer18700 => {
                ctx.call(
                    Function::MapAnnounce,
                    args![
                        ctx.call(Function::InstanceMapName, args!["1@orcs"])?,
                        "Kruger's Whisper: There's nothing we can do but to defeat the Orc Shaman if we want to save the remaining tribes.",
                        constants::BC_MAP,
                        "0xff4499"
                    ],
                )?;
                ctx.call(Function::StopNpcTimer, args![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn b1_area_1(ctx: &Ctx) -> Script {
    b1_area_1_run(ctx, B1Area1Step::Start, Vec::new()).map(|_| ())
}

pub fn b1_area_1_oninstanceinit(ctx: &Ctx) -> Script {
    b1_area_1_run(ctx, B1Area1Step::OnInstanceInit, Vec::new()).map(|_| ())
}

pub fn b1_area_1_onenable(ctx: &Ctx) -> Script {
    b1_area_1_run(ctx, B1Area1Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn b1_area_1_ontouch(ctx: &Ctx) -> Script {
    b1_area_1_run(ctx, B1Area1Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn b1_area_1_oncontinue(ctx: &Ctx) -> Script {
    b1_area_1_run(ctx, B1Area1Step::OnContinue, Vec::new()).map(|_| ())
}

pub fn b1_area_1_ontimer10300(ctx: &Ctx) -> Script {
    b1_area_1_run(ctx, B1Area1Step::OnTimer10300, Vec::new()).map(|_| ())
}

pub fn b1_area_1_ontimer18700(ctx: &Ctx) -> Script {
    b1_area_1_run(ctx, B1Area1Step::OnTimer18700, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum B1Area2Step {
    Start,
    OnInstanceInit,
    OnEnable,
    OnTouch,
    OnContinue,
    OnTimer30300,
    OnTimer37600,
}

fn b1_area_2_run(ctx: &Ctx, mut step: B1Area2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            B1Area2Step::Start => {
                step = B1Area2Step::OnInstanceInit;
                continue 'machine;
            }
            B1Area2Step::OnInstanceInit => {
                ctx.call(
                    Function::DisableNpc,
                    args![ctx.call(Function::InstanceNpcName, args!["B1 Area 2"])?],
                )?;
                return Err(Stop::End);
            }
            B1Area2Step::OnEnable => {
                ctx.call(
                    Function::EnableNpc,
                    args![ctx.call(Function::InstanceNpcName, args!["B1 Area 2"])?],
                )?;
                ctx.call(
                    Function::AreaMonster,
                    args![
                        ctx.call(Function::InstanceMapName, args!["1@orcs"])?,
                        32,
                        40,
                        38,
                        46,
                        "Enchanted Orc",
                        1023,
                        1,
                        (ctx.call(Function::InstanceNpcName, args!["B1 Area Mobs"])? + Val::from("::OnMyMobDead2"))
                    ],
                )?;
                return Err(Stop::End);
            }
            B1Area2Step::OnTouch => {
                ctx.call(
                    Function::Warp,
                    args![ctx.call(Function::InstanceMapName, args!["1@orcs"])?, 85, 85],
                )?;
                return Err(Stop::End);
            }
            B1Area2Step::OnContinue => {
                ctx.call(
                    Function::DoNpcEvent,
                    args![(ctx.call(Function::InstanceNpcName, args!["B1 Area 3"])? + Val::from("::OnEnable"))],
                )?;
                ctx.call(Function::InitNpcTimer, args![])?;
                return Err(Stop::End);
            }
            B1Area2Step::OnTimer30300 => {
                ctx.call(
                    Function::MapAnnounce,
                    args![
                        ctx.call(Function::InstanceMapName, args!["1@orcs"])?,
                        "Kruger's Whisper: I saw the bodies of our tribe. It seems that the Orc Shaman used those Orcs for her rituals.",
                        constants::BC_MAP,
                        "0xff4499"
                    ],
                )?;
                return Err(Stop::End);
            }
            B1Area2Step::OnTimer37600 => {
                ctx.call(
                    Function::MapAnnounce,
                    args![
                        ctx.call(Function::InstanceMapName, args!["1@orcs"])?,
                        "Kruger's Whisper: ... It all has to do with me. I am responsible for this evil.",
                        constants::BC_MAP,
                        "0xff4499"
                    ],
                )?;
                ctx.call(Function::StopNpcTimer, args![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn b1_area_2(ctx: &Ctx) -> Script {
    b1_area_2_run(ctx, B1Area2Step::Start, Vec::new()).map(|_| ())
}

pub fn b1_area_2_oninstanceinit(ctx: &Ctx) -> Script {
    b1_area_2_run(ctx, B1Area2Step::OnInstanceInit, Vec::new()).map(|_| ())
}

pub fn b1_area_2_onenable(ctx: &Ctx) -> Script {
    b1_area_2_run(ctx, B1Area2Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn b1_area_2_ontouch(ctx: &Ctx) -> Script {
    b1_area_2_run(ctx, B1Area2Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn b1_area_2_oncontinue(ctx: &Ctx) -> Script {
    b1_area_2_run(ctx, B1Area2Step::OnContinue, Vec::new()).map(|_| ())
}

pub fn b1_area_2_ontimer30300(ctx: &Ctx) -> Script {
    b1_area_2_run(ctx, B1Area2Step::OnTimer30300, Vec::new()).map(|_| ())
}

pub fn b1_area_2_ontimer37600(ctx: &Ctx) -> Script {
    b1_area_2_run(ctx, B1Area2Step::OnTimer37600, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum B1Area3Step {
    Start,
    OnInstanceInit,
    OnEnable,
    OnTouch,
    OnContinue,
    OnTimer30300,
    OnTimer32700,
}

fn b1_area_3_run(ctx: &Ctx, mut step: B1Area3Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            B1Area3Step::Start => {
                step = B1Area3Step::OnInstanceInit;
                continue 'machine;
            }
            B1Area3Step::OnInstanceInit => {
                ctx.call(
                    Function::DisableNpc,
                    args![ctx.call(Function::InstanceNpcName, args!["B1 Area 3"])?],
                )?;
                return Err(Stop::End);
            }
            B1Area3Step::OnEnable => {
                ctx.call(
                    Function::EnableNpc,
                    args![ctx.call(Function::InstanceNpcName, args!["B1 Area 3"])?],
                )?;
                ctx.call(
                    Function::AreaMonster,
                    args![
                        ctx.call(Function::InstanceMapName, args!["1@orcs"])?,
                        19,
                        177,
                        25,
                        183,
                        "Enchanted Orc",
                        1023,
                        1,
                        (ctx.call(Function::InstanceNpcName, args!["B1 Area Mobs"])? + Val::from("::OnMyMobDead3"))
                    ],
                )?;
                return Err(Stop::End);
            }
            B1Area3Step::OnTouch => {
                ctx.call(
                    Function::Warp,
                    args![ctx.call(Function::InstanceMapName, args!["1@orcs"])?, 38, 110],
                )?;
                return Err(Stop::End);
            }
            B1Area3Step::OnContinue => {
                ctx.call(
                    Function::DoNpcEvent,
                    args![(ctx.call(Function::InstanceNpcName, args!["B1 Area 4"])? + Val::from("::OnEnable"))],
                )?;
                ctx.call(Function::InitNpcTimer, args![])?;
                return Err(Stop::End);
            }
            B1Area3Step::OnTimer30300 => {
                ctx.call(
                    Function::MapAnnounce,
                    args![
                        ctx.call(Function::InstanceMapName, args!["1@orcs"])?,
                        "Please, hang in there!",
                        constants::BC_MAP,
                        "0xff4499"
                    ],
                )?;
                return Err(Stop::End);
            }
            B1Area3Step::OnTimer32700 => {
                ctx.call(
                    Function::MapAnnounce,
                    args![
                        ctx.call(Function::InstanceMapName, args!["1@orcs"])?,
                        "We'll get some rest when we get to the 2nd basement after passing through here.",
                        constants::BC_MAP,
                        "0xff4499"
                    ],
                )?;
                ctx.call(Function::StopNpcTimer, args![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn b1_area_3(ctx: &Ctx) -> Script {
    b1_area_3_run(ctx, B1Area3Step::Start, Vec::new()).map(|_| ())
}

pub fn b1_area_3_oninstanceinit(ctx: &Ctx) -> Script {
    b1_area_3_run(ctx, B1Area3Step::OnInstanceInit, Vec::new()).map(|_| ())
}

pub fn b1_area_3_onenable(ctx: &Ctx) -> Script {
    b1_area_3_run(ctx, B1Area3Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn b1_area_3_ontouch(ctx: &Ctx) -> Script {
    b1_area_3_run(ctx, B1Area3Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn b1_area_3_oncontinue(ctx: &Ctx) -> Script {
    b1_area_3_run(ctx, B1Area3Step::OnContinue, Vec::new()).map(|_| ())
}

pub fn b1_area_3_ontimer30300(ctx: &Ctx) -> Script {
    b1_area_3_run(ctx, B1Area3Step::OnTimer30300, Vec::new()).map(|_| ())
}

pub fn b1_area_3_ontimer32700(ctx: &Ctx) -> Script {
    b1_area_3_run(ctx, B1Area3Step::OnTimer32700, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum B1Area4Step {
    Start,
    OnInstanceInit,
    OnEnable,
    OnTouch,
}

fn b1_area_4_run(ctx: &Ctx, mut step: B1Area4Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            B1Area4Step::Start => {
                step = B1Area4Step::OnInstanceInit;
                continue 'machine;
            }
            B1Area4Step::OnInstanceInit => {
                ctx.call(
                    Function::DisableNpc,
                    args![ctx.call(Function::InstanceNpcName, args!["B1 Area 4"])?],
                )?;
                return Err(Stop::End);
            }
            B1Area4Step::OnEnable => {
                ctx.call(
                    Function::EnableNpc,
                    args![ctx.call(Function::InstanceNpcName, args!["B1 Area 4"])?],
                )?;
                ctx.call(
                    Function::DoNpcEvent,
                    args![(ctx.call(Function::InstanceNpcName, args!["#Resurrect Monsters1"])? + Val::from("::OnDisable"))],
                )?;
                ctx.call(
                    Function::DoNpcEvent,
                    args![(ctx.call(Function::InstanceNpcName, args!["#Resurrect Monsters2"])? + Val::from("::OnDisable"))],
                )?;
                ctx.call(
                    Function::DoNpcEvent,
                    args![(ctx.call(Function::InstanceNpcName, args!["#Resurrect Monsters3"])? + Val::from("::OnDisable"))],
                )?;
                ctx.call(
                    Function::DoNpcEvent,
                    args![(ctx.call(Function::InstanceNpcName, args!["#Resurrect Monsters4"])? + Val::from("::OnDisable"))],
                )?;
                return Err(Stop::End);
            }
            B1Area4Step::OnTouch => {
                ctx.call(
                    Function::Warp,
                    args![ctx.call(Function::InstanceMapName, args!["2@orcs"])?, 32, 171],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn b1_area_4(ctx: &Ctx) -> Script {
    b1_area_4_run(ctx, B1Area4Step::Start, Vec::new()).map(|_| ())
}

pub fn b1_area_4_oninstanceinit(ctx: &Ctx) -> Script {
    b1_area_4_run(ctx, B1Area4Step::OnInstanceInit, Vec::new()).map(|_| ())
}

pub fn b1_area_4_onenable(ctx: &Ctx) -> Script {
    b1_area_4_run(ctx, B1Area4Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn b1_area_4_ontouch(ctx: &Ctx) -> Script {
    b1_area_4_run(ctx, B1Area4Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn b1_area_mobs(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn b1_area_mobs_onmymobdead(ctx: &Ctx) -> Script {
    ctx.call(
        Function::DoNpcEvent,
        args![(ctx.call(Function::InstanceNpcName, args!["B1 Area 1"])? + Val::from("::OnEnable"))],
    )?;
    ctx.end()
}

pub fn b1_area_mobs_onmymobdead1(ctx: &Ctx) -> Script {
    ctx.call(
        Function::DoNpcEvent,
        args![(ctx.call(Function::InstanceNpcName, args!["B1 Area 1"])? + Val::from("::OnContinue"))],
    )?;
    ctx.end()
}

pub fn b1_area_mobs_onmymobdead2(ctx: &Ctx) -> Script {
    ctx.call(
        Function::DoNpcEvent,
        args![(ctx.call(Function::InstanceNpcName, args!["B1 Area 2"])? + Val::from("::OnContinue"))],
    )?;
    ctx.end()
}

pub fn b1_area_mobs_onmymobdead3(ctx: &Ctx) -> Script {
    ctx.call(
        Function::DoNpcEvent,
        args![(ctx.call(Function::InstanceNpcName, args!["B1 Area 3"])? + Val::from("::OnContinue"))],
    )?;
    ctx.end()
}

pub fn s_2resurrect_monsters1(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn s_2resurrect_monsters1_oninstanceinit(ctx: &Ctx) -> Script {
    ctx.call(
        Function::DisableNpc,
        args![ctx.call(Function::InstanceNpcName, args!["#2Resurrect Monsters1"])?],
    )?;
    ctx.end()
}

pub fn s_2resurrect_monsters1_onenable(ctx: &Ctx) -> Script {
    ctx.call(
        Function::EnableNpc,
        args![ctx.call(Function::InstanceNpcName, args!["#2Resurrect Monsters1"])?],
    )?;
    ctx.call(
        Function::Monster,
        args![
            ctx.call(Function::InstanceMapName, args!["2@orcs"])?,
            0,
            0,
            "Vengeful Orc",
            1152,
            30,
            (ctx.call(Function::InstanceNpcName, args!["#2Resurrect Monsters1"])? + Val::from("::OnMyMobDead"))
        ],
    )?;
    ctx.end()
}

pub fn s_2resurrect_monsters1_ondisable(ctx: &Ctx) -> Script {
    ctx.call(Function::StopNpcTimer, args![])?;
    ctx.call(
        Function::KillMonster,
        args![
            ctx.call(Function::InstanceMapName, args!["2@orcs"])?,
            (ctx.call(Function::InstanceNpcName, args!["#2Resurrect Monsters1"])? + Val::from("::OnMyMobDead"))
        ],
    )?;
    ctx.end()
}

pub fn s_2resurrect_monsters1_onmymobdead(ctx: &Ctx) -> Script {
    let l_map_s = ctx.call(Function::InstanceMapName, args!["2@orcs"])?;
    let l_mob_dead_num = Val::from(30).try_sub(ctx.call(
        Function::MobCount,
        args![
            l_map_s.clone(),
            (ctx.call(Function::InstanceNpcName, args!["#2Resurrect Monsters1"])? + Val::from("::OnMyMobDead"))
        ],
    )?)?;
    let mut l_mob_ran = Val::from(0);
    if l_mob_dead_num.number()? > 0 {
        l_mob_ran = ctx.call(Function::Rand, args![1, 30])?;
        if l_mob_ran.number()? > 29 {
            ctx.call(
                Function::Monster,
                args![
                    l_map_s.clone(),
                    0,
                    0,
                    "Vengeful Orc",
                    1152,
                    l_mob_dead_num.clone(),
                    (ctx.call(Function::InstanceNpcName, args!["#2Resurrect Monsters1"])? + Val::from("::OnMyMobDead"))
                ],
            )?;
        } else if l_mob_ran.number()? > 28 && l_mob_ran.number()? < 30 {
            ctx.call(
                Function::Monster,
                args![
                    l_map_s.clone(),
                    0,
                    0,
                    "High Orc",
                    1213,
                    l_mob_dead_num.clone(),
                    (ctx.call(Function::InstanceNpcName, args!["#2Resurrect Monsters1"])? + Val::from("::OnMyMobDead"))
                ],
            )?;
        } else if l_mob_ran.number()? > 26 && l_mob_ran.number()? < 29 {
            ctx.call(
                Function::AreaMonster,
                args![
                    l_map_s.clone(),
                    157,
                    112,
                    167,
                    122,
                    "High Orc",
                    1213,
                    l_mob_dead_num.clone(),
                    (ctx.call(Function::InstanceNpcName, args!["#2Resurrect Monsters1"])? + Val::from("::OnMyMobDead"))
                ],
            )?;
            if ctx.call(Function::Rand, args![1, 10])? == 9 {
                ctx.call(
                    Function::MapAnnounce,
                    args![
                        l_map_s.clone(),
                        "Warning: High Orcs are gathering near area 3.",
                        constants::BC_MAP,
                        "0xff4444"
                    ],
                )?;
            }
        } else {
            ctx.call(
                Function::AreaMonster,
                args![
                    l_map_s.clone(),
                    173,
                    13,
                    183,
                    23,
                    "High Orc",
                    1213,
                    l_mob_dead_num.clone(),
                    (ctx.call(Function::InstanceNpcName, args!["#2Resurrect Monsters1"])? + Val::from("::OnMyMobDead"))
                ],
            )?;
            if ctx.call(Function::Rand, args![1, 5])? == 3 {
                ctx.call(
                    Function::MapAnnounce,
                    args![
                        l_map_s.clone(),
                        "Caution: The Forces have started to concentrate at the Shaman's Altar.",
                        constants::BC_MAP,
                        "0x77ff77"
                    ],
                )?;
            }
            if ctx.call(Function::Rand, args![1, 70])? == 50 {
                ctx.call(Function::InitNpcTimer, args![])?;
            }
        }
    }
    ctx.end()
}

pub fn s_2resurrect_monsters1_ontimer10(ctx: &Ctx) -> Script {
    ctx.call(
        Function::MapAnnounce,
        args![
            ctx.call(Function::InstanceMapName, args!["2@orcs"])?,
            "Voice from somewhere: Foolish... Do you really think the altar would fall like that?",
            constants::BC_MAP,
            "0xff4444"
        ],
    )?;
    ctx.end()
}

pub fn s_2resurrect_monsters1_ontimer4010(ctx: &Ctx) -> Script {
    let l_map_s = ctx.call(Function::InstanceMapName, args!["2@orcs"])?;
    ctx.call(
        Function::MapAnnounce,
        args![
            l_map_s.clone(),
            "[ Wraiths were summoned by an unknown power ]",
            constants::BC_MAP,
            "0x77ff77"
        ],
    )?;
    ctx.call(
        Function::AreaMonster,
        args![
            l_map_s.clone(),
            167,
            25,
            177,
            35,
            "Wraith",
            1475,
            30,
            (ctx.call(Function::InstanceNpcName, args!["#2Resurrect Monsters1"])? + Val::from("::OnMyMobDead"))
        ],
    )?;
    ctx.call(Function::StopNpcTimer, args![])?;
    ctx.end()
}

pub fn s_2resurrect_monsters3(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn s_2resurrect_monsters3_oninstanceinit(ctx: &Ctx) -> Script {
    ctx.call(
        Function::DisableNpc,
        args![ctx.call(Function::InstanceNpcName, args!["#2Resurrect Monsters3"])?],
    )?;
    ctx.end()
}

pub fn s_2resurrect_monsters3_onenable(ctx: &Ctx) -> Script {
    ctx.call(
        Function::EnableNpc,
        args![ctx.call(Function::InstanceNpcName, args!["#2Resurrect Monsters3"])?],
    )?;
    ctx.call(
        Function::Monster,
        args![
            ctx.call(Function::InstanceMapName, args!["2@orcs"])?,
            0,
            0,
            "Orc Zombie",
            1153,
            15,
            (ctx.call(Function::InstanceNpcName, args!["#2Resurrect Monsters3"])? + Val::from("::OnMyMobDead"))
        ],
    )?;
    ctx.end()
}

pub fn s_2resurrect_monsters3_onmymobdead(ctx: &Ctx) -> Script {
    let l_map_s = ctx.call(Function::InstanceMapName, args!["2@orcs"])?;
    let l_mob_dead_num = Val::from(15).try_sub(ctx.call(
        Function::MobCount,
        args![
            l_map_s.clone(),
            (ctx.call(Function::InstanceNpcName, args!["#2Resurrect Monsters3"])? + Val::from("::OnMyMobDead"))
        ],
    )?)?;
    let mut l_mob_ran = Val::from(0);
    l_mob_ran = ctx.call(Function::Rand, args![1, 30])?;
    if l_mob_ran.number()? > 29 {
        if l_mob_dead_num.number()? > 0 {
            ctx.call(
                Function::Monster,
                args![
                    l_map_s.clone(),
                    0,
                    0,
                    "Orc Archer",
                    1189,
                    l_mob_dead_num.clone(),
                    (ctx.call(Function::InstanceNpcName, args!["#2Resurrect Monsters3"])? + Val::from("::OnMyMobDead"))
                ],
            )?;
        }
    } else if l_mob_ran.number()? > 6 && l_mob_ran.number()? < 30 {
        if l_mob_dead_num.number()? > 0 {
            ctx.call(
                Function::AreaMonster,
                args![
                    l_map_s.clone(),
                    168,
                    10,
                    184,
                    26,
                    "Orc Archer",
                    1189,
                    l_mob_dead_num.clone(),
                    (ctx.call(Function::InstanceNpcName, args!["#2Resurrect Monsters3"])? + Val::from("::OnMyMobDead"))
                ],
            )?;
            if ctx.call(Function::Rand, args![1, 15])? == 3 {
                ctx.call(
                    Function::MapAnnounce,
                    args![
                        l_map_s.clone(),
                        "Warning: Orc Archer teams are gathering near the altar.",
                        constants::BC_MAP,
                        "0xff4444"
                    ],
                )?;
            }
        }
    } else if l_mob_dead_num.number()? > 0 {
        ctx.call(
            Function::AreaMonster,
            args![
                l_map_s.clone(),
                168,
                21,
                184,
                21,
                "Orc Archer",
                1189,
                l_mob_dead_num.clone(),
                (ctx.call(Function::InstanceNpcName, args!["#2Resurrect Monsters3"])? + Val::from("::OnMyMobDead"))
            ],
        )?;
    }
    ctx.end()
}

pub fn s_2resurrect_monsters3_ondisable(ctx: &Ctx) -> Script {
    ctx.call(
        Function::KillMonster,
        args![
            ctx.call(Function::InstanceMapName, args!["2@orcs"])?,
            (ctx.call(Function::InstanceNpcName, args!["#2Resurrect Monsters3"])? + Val::from("::OnMyMobDead"))
        ],
    )?;
    ctx.end()
}

pub fn s_2resurrect_monsters4(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn s_2resurrect_monsters4_oninstanceinit(ctx: &Ctx) -> Script {
    ctx.call(
        Function::EnableNpc,
        args![ctx.call(Function::InstanceNpcName, args!["#2Resurrect Monsters4"])?],
    )?;
    ctx.call(
        Function::Monster,
        args![
            ctx.call(Function::InstanceMapName, args!["2@orcs"])?,
            0,
            0,
            "Anopheles",
            1627,
            10,
            (ctx.call(Function::InstanceNpcName, args!["#2Resurrect Monsters4"])? + Val::from("::OnMyMobDead"))
        ],
    )?;
    ctx.end()
}

pub fn s_2resurrect_monsters4_onmymobdead(ctx: &Ctx) -> Script {
    let l_map_s = ctx.call(Function::InstanceMapName, args!["2@orcs"])?;
    let l_mob_dead_num = Val::from(10).try_sub(ctx.call(
        Function::MobCount,
        args![
            l_map_s.clone(),
            (ctx.call(Function::InstanceNpcName, args!["#2Resurrect Monsters4"])? + Val::from("::OnMyMobDead"))
        ],
    )?)?;
    if l_mob_dead_num.number()? > 0 {
        ctx.call(
            Function::Monster,
            args![
                l_map_s.clone(),
                0,
                0,
                "Anopheles",
                1627,
                1,
                (ctx.call(Function::InstanceNpcName, args!["#2Resurrect Monsters4"])? + Val::from("::OnMyMobDead"))
            ],
        )?;
    }
    ctx.end()
}

pub fn kruger_2_1(ctx: &Ctx) -> Script {
    ctx.call(
        Function::DoNpcEvent,
        args![(ctx.call(Function::InstanceNpcName, args!["Kruger#2-2"])? + Val::from("::OnEnable"))],
    )?;
    ctx.end()
}

pub fn kruger_2_2(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn kruger_2_2_oninstanceinit(ctx: &Ctx) -> Script {
    ctx.call(
        Function::DisableNpc,
        args![ctx.call(Function::InstanceNpcName, args!["Kruger#2-2"])?],
    )?;
    ctx.end()
}

pub fn kruger_2_2_onenable(ctx: &Ctx) -> Script {
    ctx.call(
        Function::DisableNpc,
        args![ctx.call(Function::InstanceNpcName, args!["Kruger#2-1"])?],
    )?;
    ctx.call(
        Function::EnableNpc,
        args![ctx.call(Function::InstanceNpcName, args!["Kruger#2-2"])?],
    )?;
    ctx.call(Function::InitNpcTimer, args![])?;
    ctx.end()
}

pub fn kruger_2_2_ontimer10(ctx: &Ctx) -> Script {
    ctx.call(
        Function::MapAnnounce,
        args![
            ctx.call(Function::InstanceMapName, args!["2@orcs"])?,
            "Kruger's Whisper: I'll tell you how to get to the Shaman's altar.",
            constants::BC_MAP,
            "0xffff00"
        ],
    )?;
    ctx.end()
}

pub fn kruger_2_2_ontimer3510(ctx: &Ctx) -> Script {
    ctx.call(
        Function::MapAnnounce,
        args![
            ctx.call(Function::InstanceMapName, args!["2@orcs"])?,
            "Kruger's Whisper: Do you see the braziers that light the path? Unseal the next zone by strengthening their flames.",
            constants::BC_MAP,
            "0xffff00"
        ],
    )?;
    ctx.end()
}

pub fn kruger_2_2_ontimer10710(ctx: &Ctx) -> Script {
    ctx.call(
        Function::MapAnnounce,
        args![
            ctx.call(Function::InstanceMapName, args!["2@orcs"])?,
            "Kruger's Whisper: Of course those monsters won't let you touch the braziers that easily.",
            constants::BC_MAP,
            "0xffff00"
        ],
    )?;
    ctx.end()
}

pub fn kruger_2_2_ontimer16310(ctx: &Ctx) -> Script {
    ctx.call(
        Function::MapAnnounce,
        args![
            ctx.call(Function::InstanceMapName, args!["2@orcs"])?,
            "Kruger's Whisper: But still, try keep the battles not too noticable so the Shaman won't guard the altar with her army squad.",
            constants::BC_MAP,
            "0xffff00"
        ],
    )?;
    ctx.end()
}

pub fn kruger_2_2_ontimer21910(ctx: &Ctx) -> Script {
    ctx.call(
        Function::MapAnnounce,
        args![
            ctx.call(Function::InstanceMapName, args!["2@orcs"])?,
            "Kruger's Whisper: Only the Party Leader can strengthen the flames, so protect your leader.",
            constants::BC_MAP,
            "0xffff00"
        ],
    )?;
    ctx.end()
}

pub fn kruger_2_2_ontimer23910(ctx: &Ctx) -> Script {
    ctx.call(
        Function::MapAnnounce,
        args![
            ctx.call(Function::InstanceMapName, args!["2@orcs"])?,
            "Mission: Unseal the zone by lighting the braziers. They can only be lit in a certain order, so be careful.",
            constants::BC_MAP,
            "0x4444ff"
        ],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        args![(ctx.call(Function::InstanceNpcName, args!["#2Resurrect Monsters1"])? + Val::from("::OnEnable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        args![(ctx.call(Function::InstanceNpcName, args!["#2Resurrect Monsters3"])? + Val::from("::OnEnable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        args![(ctx.call(Function::InstanceNpcName, args!["Torch#1-1"])? + Val::from("::OnEnable"))],
    )?;
    ctx.call(
        Function::DisableNpc,
        args![ctx.call(Function::InstanceNpcName, args!["Kruger#2-2"])?],
    )?;
    ctx.end()
}

pub fn torch_1_1(ctx: &Ctx) -> Script {
    let mut l_id: Vec<Val> = Vec::new();
    if ctx.call(Function::IsPartyLeader, args![ctx.call(Function::GetCharacterId, args![1])?])? == 0 {
        return ctx.end();
    }
    ctx.call(Function::ProgressBar, args!["ffff00", 5])?;
    runtime::local_set(
        &mut l_id,
        &Val::from(0),
        runtime::atoi(&runtime::charat(&ctx.call(Function::StrNpcInfo, args![2])?, &Val::from(0))?),
        false,
    );
    runtime::local_set(
        &mut l_id,
        &Val::from(1),
        runtime::atoi(&runtime::charat(&ctx.call(Function::StrNpcInfo, args![2])?, &Val::from(2))?),
        false,
    );
    if runtime::local_get(&l_id, &Val::from(1), false) == 4 {
        ctx.call(
            Function::DoNpcEvent,
            args![
                (ctx.call(
                    Function::InstanceNpcName,
                    args![(Val::from("#Warp2-") + runtime::local_get(&l_id, &Val::from(0), false))],
                )? + Val::from("::OnEnable"))
            ],
        )?;
    } else {
        ctx.call(
            Function::DoNpcEvent,
            args![
                (ctx.call(
                    Function::InstanceNpcName,
                    args![
                        (((Val::from("Torch#") + runtime::local_get(&l_id, &Val::from(0), false)) + Val::from("-"))
                            + (runtime::local_get(&l_id, &Val::from(1), false) + Val::from(1)))
                    ],
                )? + Val::from("::OnEnable"))
            ],
        )?;
    }
    ctx.call(Function::InitNpcTimer, args![])?;
    ctx.call(Function::DisableNpc, args![])?;
    ctx.end()
}

pub fn torch_1_1_oninstanceinit(ctx: &Ctx) -> Script {
    if (ctx.call(Function::StrNpcInfo, args![0])? != "Torch#2-1" && ctx.call(Function::StrNpcInfo, args![0])? != "Torch#3-1") {
        ctx.call(Function::DisableNpc, args![])?;
    }
    ctx.end()
}

pub fn torch_1_1_onenable(ctx: &Ctx) -> Script {
    ctx.call(Function::EnableNpc, args![])?;
    ctx.end()
}

pub fn torch_1_1_ontimer100(ctx: &Ctx) -> Script {
    ctx.call(Function::NpcSpecialEffect, args![constants::EF_FIREPILLAR])?;
    ctx.end()
}

pub fn torch_1_1_ontimer2000(ctx: &Ctx) -> Script {
    ctx.call(Function::NpcSpecialEffect, args![constants::EF_FIREPILLARBOMB])?;
    ctx.call(Function::StopNpcTimer, args![])?;
    ctx.end()
}

#[derive(Clone, Copy, Debug)]
enum Warp21Step {
    Start,
    OnInstanceInit,
    OnEnable,
    OnContinue,
    OnTimer10000,
    OnTouch,
}

fn warp2_1_run(ctx: &Ctx, mut step: Warp21Step, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_map_s = Val::from("");
    'machine: loop {
        match step {
            Warp21Step::Start => {
                step = Warp21Step::OnInstanceInit;
                continue 'machine;
            }
            Warp21Step::OnInstanceInit => {
                ctx.call(
                    Function::DisableNpc,
                    args![ctx.call(Function::InstanceNpcName, args!["#Warp2-1"])?],
                )?;
                return Err(Stop::End);
            }
            Warp21Step::OnEnable => {
                l_map_s = ctx.call(Function::InstanceMapName, args!["2@orcs"])?;
                ctx.call(
                    Function::Monster,
                    args![
                        l_map_s.clone(),
                        109,
                        156,
                        "Safeguard Chief",
                        1981,
                        1,
                        (ctx.call(Function::InstanceNpcName, args!["#Mobs Control"])? + Val::from("::OnMyMobDead1"))
                    ],
                )?;
                ctx.call(
                    Function::MapAnnounce,
                    args![
                        l_map_s.clone(),
                        "The Chief Orc of Safeguards: Oh!! Looks like I have company. Defeat me if you can!!",
                        constants::BC_MAP,
                        "0xff8888"
                    ],
                )?;
                return Err(Stop::End);
            }
            Warp21Step::OnContinue => {
                ctx.call(
                    Function::EnableNpc,
                    args![ctx.call(Function::InstanceNpcName, args!["#Warp2-1"])?],
                )?;
                ctx.call(Function::InitNpcTimer, args![])?;
                return Err(Stop::End);
            }
            Warp21Step::OnTimer10000 => {
                ctx.call(
                    Function::AreaMonster,
                    args![
                        ctx.call(Function::InstanceMapName, args!["2@orcs"])?,
                        28,
                        158,
                        40,
                        170,
                        "Safeguard Chief",
                        1981,
                        1,
                        (ctx.call(Function::InstanceNpcName, args!["#Mobs Control"])? + Val::from("::OnMyMobDead1"))
                    ],
                )?;
                ctx.call(Function::StopNpcTimer, args![])?;
                return Err(Stop::End);
            }
            Warp21Step::OnTouch => {
                ctx.call(
                    Function::Warp,
                    args![ctx.call(Function::InstanceMapName, args!["2@orcs"])?, 47, 93],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn warp2_1(ctx: &Ctx) -> Script {
    warp2_1_run(ctx, Warp21Step::Start, Vec::new()).map(|_| ())
}

pub fn warp2_1_oninstanceinit(ctx: &Ctx) -> Script {
    warp2_1_run(ctx, Warp21Step::OnInstanceInit, Vec::new()).map(|_| ())
}

pub fn warp2_1_onenable(ctx: &Ctx) -> Script {
    warp2_1_run(ctx, Warp21Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn warp2_1_oncontinue(ctx: &Ctx) -> Script {
    warp2_1_run(ctx, Warp21Step::OnContinue, Vec::new()).map(|_| ())
}

pub fn warp2_1_ontimer10000(ctx: &Ctx) -> Script {
    warp2_1_run(ctx, Warp21Step::OnTimer10000, Vec::new()).map(|_| ())
}

pub fn warp2_1_ontouch(ctx: &Ctx) -> Script {
    warp2_1_run(ctx, Warp21Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Warp22Step {
    Start,
    OnInstanceInit,
    OnEnable,
    OnContinue,
    OnTimer10000,
    OnTouch,
}

fn warp2_2_run(ctx: &Ctx, mut step: Warp22Step, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_map_s = Val::from("");
    'machine: loop {
        match step {
            Warp22Step::Start => {
                step = Warp22Step::OnInstanceInit;
                continue 'machine;
            }
            Warp22Step::OnInstanceInit => {
                ctx.call(
                    Function::DisableNpc,
                    args![ctx.call(Function::InstanceNpcName, args!["#Warp2-2"])?],
                )?;
                return Err(Stop::End);
            }
            Warp22Step::OnEnable => {
                l_map_s = ctx.call(Function::InstanceMapName, args!["2@orcs"])?;
                ctx.call(
                    Function::Monster,
                    args![
                        l_map_s.clone(),
                        67,
                        64,
                        "Orc Sniper",
                        1982,
                        1,
                        (ctx.call(Function::InstanceNpcName, args!["#Mobs Control"])? + Val::from("::OnMyMobDead2"))
                    ],
                )?;
                ctx.call(
                    Function::MapAnnounce,
                    args![
                        l_map_s.clone(),
                        "Orc Sniper: Hah! Pretty impressive that you made it this far, but your foolish little trip ends here...",
                        constants::BC_MAP,
                        "0xff8888"
                    ],
                )?;
                return Err(Stop::End);
            }
            Warp22Step::OnContinue => {
                ctx.call(
                    Function::EnableNpc,
                    args![ctx.call(Function::InstanceNpcName, args!["#Warp2-2"])?],
                )?;
                ctx.call(Function::InitNpcTimer, args![])?;
                return Err(Stop::End);
            }
            Warp22Step::OnTimer10000 => {
                ctx.call(
                    Function::AreaMonster,
                    args![
                        ctx.call(Function::InstanceMapName, args!["2@orcs"])?,
                        40,
                        91,
                        52,
                        103,
                        "Orc Sniper",
                        1982,
                        1,
                        (ctx.call(Function::InstanceNpcName, args!["#Mobs Control"])? + Val::from("::OnMyMobDead2"))
                    ],
                )?;
                ctx.call(Function::StopNpcTimer, args![])?;
                return Err(Stop::End);
            }
            Warp22Step::OnTouch => {
                ctx.call(
                    Function::Warp,
                    args![ctx.call(Function::InstanceMapName, args!["2@orcs"])?, 107, 55],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn warp2_2(ctx: &Ctx) -> Script {
    warp2_2_run(ctx, Warp22Step::Start, Vec::new()).map(|_| ())
}

pub fn warp2_2_oninstanceinit(ctx: &Ctx) -> Script {
    warp2_2_run(ctx, Warp22Step::OnInstanceInit, Vec::new()).map(|_| ())
}

pub fn warp2_2_onenable(ctx: &Ctx) -> Script {
    warp2_2_run(ctx, Warp22Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn warp2_2_oncontinue(ctx: &Ctx) -> Script {
    warp2_2_run(ctx, Warp22Step::OnContinue, Vec::new()).map(|_| ())
}

pub fn warp2_2_ontimer10000(ctx: &Ctx) -> Script {
    warp2_2_run(ctx, Warp22Step::OnTimer10000, Vec::new()).map(|_| ())
}

pub fn warp2_2_ontouch(ctx: &Ctx) -> Script {
    warp2_2_run(ctx, Warp22Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Warp23Step {
    Start,
    OnInstanceInit,
    OnEnable,
    OnContinue,
    OnTimer10,
    OnTimer6810,
    OnTimer10310,
    OnTimer13110,
    OnTouch,
}

fn warp2_3_run(ctx: &Ctx, mut step: Warp23Step, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_map_s = Val::from("");
    'machine: loop {
        match step {
            Warp23Step::Start => {
                step = Warp23Step::OnInstanceInit;
                continue 'machine;
            }
            Warp23Step::OnInstanceInit => {
                ctx.call(
                    Function::DisableNpc,
                    args![ctx.call(Function::InstanceNpcName, args!["#Warp2-3"])?],
                )?;
                return Err(Stop::End);
            }
            Warp23Step::OnEnable => {
                l_map_s = ctx.call(Function::InstanceMapName, args!["2@orcs"])?;
                ctx.call(
                    Function::Monster,
                    args![
                        l_map_s.clone(),
                        152,
                        147,
                        "Depraved Orc Spirit",
                        1983,
                        1,
                        (ctx.call(Function::InstanceNpcName, args!["#Mobs Control"])? + Val::from("::OnMyMobDead3"))
                    ],
                )?;
                ctx.call(
                    Function::MapAnnounce,
                    args![
                        l_map_s.clone(),
                        "Depraved Orc Spirit: I smell flesh! Hungry! Wanna try some human meat!!",
                        constants::BC_MAP,
                        "0xff8888"
                    ],
                )?;
                return Err(Stop::End);
            }
            Warp23Step::OnContinue => {
                ctx.call(
                    Function::AreaMonster,
                    args![
                        ctx.call(Function::InstanceMapName, args!["2@orcs"])?,
                        117,
                        61,
                        129,
                        73,
                        "Depraved Orc Spirit",
                        1983,
                        1
                    ],
                )?;
                ctx.call(
                    Function::DoNpcEvent,
                    args![(ctx.call(Function::InstanceNpcName, args!["#Boss Control"])? + Val::from("::OnEnable"))],
                )?;
                ctx.call(
                    Function::EnableNpc,
                    args![ctx.call(Function::InstanceNpcName, args!["#Warp2-3"])?],
                )?;
                ctx.call(Function::InitNpcTimer, args![])?;
                return Err(Stop::End);
            }
            Warp23Step::OnTimer10 => {
                ctx.call(Function::MapAnnounce, args![ctx.call(Function::InstanceMapName, args!["2@orcs"])?, "Shaman Cargalache: Hahaha!! So, you finally made it here. The assassin you sent was just terrible. That stupid Orc is getting cold under my feet.", constants::BC_MAP, "0xffff00"])?;
                return Err(Stop::End);
            }
            Warp23Step::OnTimer6810 => {
                ctx.call(
                    Function::MapAnnounce,
                    args![
                        ctx.call(Function::InstanceMapName, args!["2@orcs"])?,
                        "Shaman Cargalache: My loyal slave, go get those intruders!",
                        constants::BC_MAP,
                        "0xffff00"
                    ],
                )?;
                return Err(Stop::End);
            }
            Warp23Step::OnTimer10310 => {
                ctx.call(
                    Function::MapAnnounce,
                    args![
                        ctx.call(Function::InstanceMapName, args!["2@orcs"])?,
                        "Depraved Orc Hero: Whatever you say, my lord.",
                        constants::BC_MAP,
                        "0xff7777"
                    ],
                )?;
                return Err(Stop::End);
            }
            Warp23Step::OnTimer13110 => {
                ctx.call(Function::MapAnnounce, args![ctx.call(Function::InstanceMapName, args!["2@orcs"])?, "Caution: You have been discovered by Shaman Cargalache. Kruger's plan to assassinate the Shaman has failed. You must defeat Cargalache and find traces of Kruger.", constants::BC_MAP, "0x8888ff"])?;
                ctx.call(Function::StopNpcTimer, args![])?;
                return Err(Stop::End);
            }
            Warp23Step::OnTouch => {
                ctx.call(
                    Function::Warp,
                    args![ctx.call(Function::InstanceMapName, args!["2@orcs"])?, 167, 95],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn warp2_3(ctx: &Ctx) -> Script {
    warp2_3_run(ctx, Warp23Step::Start, Vec::new()).map(|_| ())
}

pub fn warp2_3_oninstanceinit(ctx: &Ctx) -> Script {
    warp2_3_run(ctx, Warp23Step::OnInstanceInit, Vec::new()).map(|_| ())
}

pub fn warp2_3_onenable(ctx: &Ctx) -> Script {
    warp2_3_run(ctx, Warp23Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn warp2_3_oncontinue(ctx: &Ctx) -> Script {
    warp2_3_run(ctx, Warp23Step::OnContinue, Vec::new()).map(|_| ())
}

pub fn warp2_3_ontimer10(ctx: &Ctx) -> Script {
    warp2_3_run(ctx, Warp23Step::OnTimer10, Vec::new()).map(|_| ())
}

pub fn warp2_3_ontimer6810(ctx: &Ctx) -> Script {
    warp2_3_run(ctx, Warp23Step::OnTimer6810, Vec::new()).map(|_| ())
}

pub fn warp2_3_ontimer10310(ctx: &Ctx) -> Script {
    warp2_3_run(ctx, Warp23Step::OnTimer10310, Vec::new()).map(|_| ())
}

pub fn warp2_3_ontimer13110(ctx: &Ctx) -> Script {
    warp2_3_run(ctx, Warp23Step::OnTimer13110, Vec::new()).map(|_| ())
}

pub fn warp2_3_ontouch(ctx: &Ctx) -> Script {
    warp2_3_run(ctx, Warp23Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn boss_control(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn boss_control_oninstanceinit(ctx: &Ctx) -> Script {
    ctx.call(
        Function::DisableNpc,
        args![ctx.call(Function::InstanceNpcName, args!["#Boss Control"])?],
    )?;
    ctx.end()
}

pub fn boss_control_onenable(ctx: &Ctx) -> Script {
    let l_map_s = ctx.call(Function::InstanceMapName, args!["2@orcs"])?;
    ctx.call(
        Function::Monster,
        args![
            l_map_s.clone(),
            185,
            8,
            "Shaman Cargalache",
            1984,
            1,
            (ctx.call(Function::InstanceNpcName, args!["#Boss Control"])? + Val::from("::OnMyMobDead"))
        ],
    )?;
    ctx.call(Function::Monster, args![l_map_s.clone(), 179, 15, "Depraved Orc Hero", 1087, 1])?;
    ctx.call(
        Function::EnableNpc,
        args![ctx.call(Function::InstanceNpcName, args!["#Boss Control"])?],
    )?;
    ctx.end()
}

pub fn boss_control_onmymobdead(ctx: &Ctx) -> Script {
    ctx.call(
        Function::DoNpcEvent,
        args![(ctx.call(Function::InstanceNpcName, args!["Kruger#"])? + Val::from("::OnEnable"))],
    )?;
    let l_map_s = ctx.call(Function::InstanceMapName, args!["2@orcs"])?;
    let l_mob_ran = ctx.call(Function::Rand, args![1, 5])?;
    if l_mob_ran == 1 {
        ctx.call(
            Function::MapAnnounce,
            args![
                l_map_s.clone(),
                "Shaman Cargalache: How... How could this be... How could someone like you...!!",
                constants::BC_MAP,
                "0xffff00"
            ],
        )?;
    } else if l_mob_ran == 2 {
        ctx.call(
            Function::MapAnnounce,
            args![
                l_map_s.clone(),
                "Shaman Cargalache: How is it that I've been overpowered by mere humans!",
                constants::BC_MAP,
                "0xffff00"
            ],
        )?;
    } else if l_mob_ran == 3 {
        ctx.call(
            Function::MapAnnounce,
            args![
                l_map_s.clone(),
                "Shaman Cargalache: This... This can't be the end...",
                constants::BC_MAP,
                "0xffff00"
            ],
        )?;
    } else if l_mob_ran == 4 {
        ctx.call(
            Function::MapAnnounce,
            args![
                l_map_s.clone(),
                "Shaman Cargalache: I... Can't die... Yet...!",
                constants::BC_MAP,
                "0xffff00"
            ],
        )?;
    } else {
        ctx.call(
            Function::MapAnnounce,
            args![
                l_map_s.clone(),
                "Shaman Cargalache: Defeated by these fools... It can't be happening...!",
                constants::BC_MAP,
                "0xffff00"
            ],
        )?;
    }
    ctx.call(
        Function::DoNpcEvent,
        args![(ctx.call(Function::InstanceNpcName, args!["#2Resurrect Monsters1"])? + Val::from("::OnDisable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        args![(ctx.call(Function::InstanceNpcName, args!["#2Resurrect Monsters3"])? + Val::from("::OnDisable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        args![(ctx.call(Function::InstanceNpcName, args!["#Warp Outside Orc Dun"])? + Val::from("::OnEnable"))],
    )?;
    ctx.end()
}

#[derive(Clone, Copy, Debug)]
enum WarpOutsideOrcDunStep {
    Start,
    OnInstanceInit,
    OnEnable,
    OnTouch,
}

fn warp_outside_orc_dun_run(ctx: &Ctx, mut step: WarpOutsideOrcDunStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            WarpOutsideOrcDunStep::Start => {
                step = WarpOutsideOrcDunStep::OnInstanceInit;
                continue 'machine;
            }
            WarpOutsideOrcDunStep::OnInstanceInit => {
                ctx.call(
                    Function::DisableNpc,
                    args![ctx.call(Function::InstanceNpcName, args!["#Warp Outside Orc Dun"])?],
                )?;
                return Err(Stop::End);
            }
            WarpOutsideOrcDunStep::OnEnable => {
                ctx.call(
                    Function::EnableNpc,
                    args![ctx.call(Function::InstanceNpcName, args!["#Warp Outside Orc Dun"])?],
                )?;
                return Err(Stop::End);
            }
            WarpOutsideOrcDunStep::OnTouch => {
                ctx.warp("gef_fild10", 240, 197)?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn warp_outside_orc_dun(ctx: &Ctx) -> Script {
    warp_outside_orc_dun_run(ctx, WarpOutsideOrcDunStep::Start, Vec::new()).map(|_| ())
}

pub fn warp_outside_orc_dun_oninstanceinit(ctx: &Ctx) -> Script {
    warp_outside_orc_dun_run(ctx, WarpOutsideOrcDunStep::OnInstanceInit, Vec::new()).map(|_| ())
}

pub fn warp_outside_orc_dun_onenable(ctx: &Ctx) -> Script {
    warp_outside_orc_dun_run(ctx, WarpOutsideOrcDunStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn warp_outside_orc_dun_ontouch(ctx: &Ctx) -> Script {
    warp_outside_orc_dun_run(ctx, WarpOutsideOrcDunStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn kruger(ctx: &Ctx) -> Script {
    if ctx.var("yong_odun").get()?.number()? >= 2 {
        ctx.mes("You can see the dead body of Kruger, peacefully lying on the ground.")?;
        return ctx.close();
    }
    ctx.lines_as("Kruger", args!["*Coughing*", ctx.player().name()? + ", it's you..."])?;
    ctx.next()?;
    ctx.lines(args![format!("[{}] ", ctx.player().name()?), "Don't move! You are wounded!"])?;
    ctx.next()?;
    ctx.lines_as(
        "Kruger",
        args!["It's... all right.. I'm dying...", "....", "The Shaman? What about the Shaman?"],
    )?;
    ctx.next()?;
    ctx.lines(args![
        format!("[{}] ", ctx.player().name()?),
        "The Shaman's dead now. Who was that Shaman really?"
    ])?;
    ctx.next()?;
    ctx.mes("Kruger seemed to be relieved as he hears of the death of the Shaman, but you notice the bitter expression on his face.")?;
    ctx.next()?;
    ctx.lines_as(
        "Kruger",
        args![
            "I.. I just couldn't kill my own daughter...",
            "Thank you, I'm sure she's finally free from the nightmare that used to choke her soul."
        ],
    )?;
    ctx.next()?;
    ctx.mes("Kruger was about to say something more, but he breathed his last breath before he could...")?;
    ctx.var("yong_odun").set(Val::from(2))?;
    ctx.close()
}

pub fn kruger_oninstanceinit(ctx: &Ctx) -> Script {
    ctx.call(
        Function::DisableNpc,
        args![ctx.call(Function::InstanceNpcName, args!["Kruger#"])?],
    )?;
    ctx.end()
}

pub fn kruger_onenable(ctx: &Ctx) -> Script {
    ctx.call(
        Function::EnableNpc,
        args![ctx.call(Function::InstanceNpcName, args!["Kruger#"])?],
    )?;
    ctx.end()
}

pub fn mobs_control(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn mobs_control_onmymobdead1(ctx: &Ctx) -> Script {
    ctx.call(
        Function::DoNpcEvent,
        args![(ctx.call(Function::InstanceNpcName, args!["#Warp2-1"])? + Val::from("::OnContinue"))],
    )?;
    ctx.end()
}

pub fn mobs_control_onmymobdead2(ctx: &Ctx) -> Script {
    ctx.call(
        Function::DoNpcEvent,
        args![(ctx.call(Function::InstanceNpcName, args!["#Warp2-2"])? + Val::from("::OnContinue"))],
    )?;
    ctx.end()
}

pub fn mobs_control_onmymobdead3(ctx: &Ctx) -> Script {
    ctx.call(
        Function::DoNpcEvent,
        args![(ctx.call(Function::InstanceNpcName, args!["#Warp2-3"])? + Val::from("::OnContinue"))],
    )?;
    ctx.end()
}

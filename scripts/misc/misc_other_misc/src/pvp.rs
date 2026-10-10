#![allow(
    unused_imports,
    unused_labels,
    unused_assignments,
    unused_mut,
    unused_parens,
    unused_variables,
    unreachable_code
)]

use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

const SAVE_POINTS: [(&str, i32, i32); 5] = [
    ("morocc_in", 141, 139),
    ("alberta_in", 22, 148),
    ("prt_in", 54, 137),
    ("geffen_in", 70, 59),
    ("payon_in01", 142, 46),
];

const WARP_X: [i32; 4] = [40, 59, 20, 40];
const WARP_Y: [i32; 4] = [59, 40, 40, 20];

const EXIT_WARPS: [(&str, i32, i32); 5] = [
    ("prontera", 107, 60),
    ("morocc", 157, 96),
    ("geffen", 120, 36),
    ("payon", 96, 100),
    ("alberta", 41, 243),
];

pub fn pvp_narrator(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "PVP Narrator",
        args![
            "Hello and welcome!",
            "I am in charge of",
            "explaining the PVP Modes.",
            "I am the PVP Narrator!"
        ],
    )?;
    ctx.next()?;
    match ctx.menu(&[
        "What is PVP?",
        "What are the PVP Modes?",
        "What are the rules for PVP?",
        "Save Position.",
        "End Dialog.",
    ])? {
        0 => {
            ctx.lines_as(
                "PVP Narrator",
                args![
                    "In short, PVP means",
                    "' Player VS Player Mode '",
                    "It's a unique place for people",
                    "to duel with each other."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "PVP Narrator",
                args![
                    "Just tell the",
                    "^3355FFGate Keeper^000000",
                    "that you want to try. He will",
                    "let you enter the PVP square."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "PVP Narrator",
                args![
                    "But, you need to be at",
                    "least level 31. And you",
                    "must pay 500 zeny entrance fee in order",
                    "to enter a PVP fight square."
                ],
            )?;
        }
        1 => {
            ctx.lines_as(
                "PVP Narrator",
                args![
                    "When you are qualified, you",
                    "can choose one of the two modes.",
                    "Yoyo Mode or Nightmare Mode."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "PVP Narrator",
                args![
                    "Yoyo Mode is risk free.",
                    "You can experience PVP",
                    "without any restriction or",
                    "punishment. It is recommended",
                    "that you practice your skills",
                    "here before you move on."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "PVP Narrator",
                args![
                    "Nightmare Mode is very",
                    "dangerous! Please be cautious,",
                    "you will lose some of your",
                    "EXP when you are defeated. And",
                    "there is a small chance that",
                    "you will drop some equipment."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "PVP Narrator",
                args![
                    "However, if you win, the",
                    "rewards can be great!",
                    "To avoid misunderstanding,",
                    "you should think twice",
                    "before you go there...",
                    "Good Luck!"
                ],
            )?;
        }
        2 => {
            ctx.lines_as(
                "PVP Narrator",
                args![
                    "Each of the fight squares",
                    "have a row of Narrators and",
                    "choose them based on your",
                    "qualifications."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "PVP Narrator",
                args![
                    "Each Narrator will ask",
                    "which of the five PVP maps",
                    "you wish to go to.",
                    "Choose, and go in!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "PVP Narrator",
                args![
                    "Each map has a limitation on",
                    "the number of people who can",
                    "participate. So you will see",
                    "figures in the corner showing",
                    "'Attendee/Total'."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "PVP Narrator",
                args![
                    "Also, there is a hidden EXP",
                    "value in PVP mode. This EXP",
                    "score will only apply inside",
                    "of the PVP zone, so do not",
                    "worry."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "PVP Narrator",
                args![
                    "Every player's EXP at the",
                    "beginning is usually 5 points.",
                    "If you win, it will",
                    "increase by 1 point"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "PVP Narrator",
                args![
                    "In the same way, when you",
                    "lose... Your EXP will",
                    "drop by 5 points.",
                    "So be careful!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "PVP Narrator",
                args![
                    "When you are defeated",
                    "And your EXP is equal",
                    "to or less than 0,",
                    "You will be removed from PVP",
                    "and your duel is finished!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "PVP Narrator",
                args![
                    "However, if your EXP is more",
                    "than 0. You can still get help",
                    "through other players healing...",
                    "Do you get it?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "PVP Narrator",
                args![
                    "The fighting commands inside of",
                    "PVP are the same as the normal.",
                    "All the basic controls are the",
                    "same."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "PVP Narrator",
                args![
                    "However, there is one thing...",
                    "Within the PVP fight square",
                    "and PVP fighting zones,",
                    "you cannot save your position.",
                    "Remember well... These rules",
                    "can help to ensure your victory."
                ],
            )?;
        }
        3 => {
            ctx.lines_as(
                "PVP Narrator",
                args![
                    "Position successfully saved...",
                    "Thank you very much!",
                    "We will see you again soon."
                ],
            )?;
            for (npc, x, y) in SAVE_POINTS {
                if ctx.call(Function::StrNpcInfo, args![4])? == npc {
                    ctx.call(Function::SavePoint, args![npc, x, y, 1, 1])?;
                }
            }
        }
        4 => {
            ctx.lines_as(
                "PVP Narrator",
                args![
                    "With war raging between monsters",
                    "and humans, this competition",
                    "among people - PVP -",
                    "encourages us all to get",
                    "stronger. Come again,",
                    "we welcome your challenge!"
                ],
            )?;
        }
        _ => {}
    }
    ctx.close()
}

#[derive(Clone, Copy, Debug)]
enum GateKeeperStep {
    Start,
    LWarp,
}

fn gate_keeper_run(ctx: &Ctx, mut step: GateKeeperStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            GateKeeperStep::Start => {
                ctx.lines_as(
                    "Gate Keeper",
                    args![
                        "Glad to be of service.",
                        "I will open the PVP fight",
                        "square for you! If you have",
                        "any questions about the PVP",
                        "modes or rules, Please ask",
                        "the Narrator..."
                    ],
                )?;
                ctx.next()?;
                'b1: {
                    let subject1 = Val::from(runtime::select_values(
                        ctx,
                        &[Val::from(
                            "^FF5533' PvP Nightmare Mode'^000000:^3355FF' PvP Yoyo Mode'^000000:^3355FF' PvP Event Mode'^000000:Quit",
                        )],
                    )?);
                    let mut matched1 = false;
                    if !matched1 && subject1 == 1 {
                        matched1 = true;
                    }
                    if matched1 {
                        ctx.lines_as(
                            "Gate Keeper",
                            args![
                                "I am sorry, but currently the Nightmare mode service is not available.",
                                "Please use the Yoyo Mode instead. We apologize for the inconvenience."
                            ],
                        )?;
                        break 'b1;
                    }
                    if !matched1 && subject1 == 2 {
                        matched1 = true;
                    }
                    if matched1 {
                        ctx.lines_as(
                            "Gate Keeper",
                            args![
                                "The admission fee is 500 Zeny.",
                                "Do you want to move",
                                "to the Yoyo Mode",
                                "fight square?"
                            ],
                        )?;
                        ctx.next()?;
                        let subject2 = Val::from(runtime::select_values(ctx, &[Val::from("Move:Cancel")])?);
                        if subject2 == 1 {
                            if ctx.player().zeny()? > 499 && ctx.player().base_level()? > 30 {
                                ctx.player().set_zeny(ctx.player().zeny()? - 500)?;
                                gate_keeper_run(ctx, GateKeeperStep::LWarp, args!["pvp_y_room"])?;
                            } else {
                                ctx.lines_as(
                                    "Gate Keeper",
                                    args![
                                        "Excuse me, but",
                                        "did you not come prepared?",
                                        "Double check that you have the",
                                        "500 Zeny entrance fee, and",
                                        "that you are at least level 31!"
                                    ],
                                )?;
                            }
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        if subject2 == 2 {
                            ctx.lines_as(
                                "Gate Keeper",
                                args![
                                    "With war raging between monsters",
                                    "and humans, this competition",
                                    "among people - PVP -",
                                    "encourages us all to get",
                                    "stronger. Come again,",
                                    "we welcome your challenge!"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                    if !matched1 && subject1 == 3 {
                        matched1 = true;
                    }
                    if matched1 {
                        ctx.lines_as(
                            "Gate Keeper",
                            args!["Welcome!", "Please double check", "that you have the admission or viewing ticket."],
                        )?;
                        ctx.next()?;
                        if ctx.call(Function::CountItem, args![7028])? == 0 && ctx.call(Function::CountItem, args![7029])? == 0 {
                            ctx.lines_as(
                                "Gate Keeper",
                                args![
                                    "Eh? You don't have it? Then I",
                                    "am sorry, this fight square is",
                                    "only for people who have",
                                    "admission or viewing tickets.",
                                    "You cannot come in without it."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as("Gate Keeper", args!["Yes, thank you for participating. Have fun!"])?;
                        if ctx.player().zeny()? >= 500 {
                            ctx.player().set_zeny(ctx.player().zeny()? - 500)?;
                        }
                        gate_keeper_run(ctx, GateKeeperStep::LWarp, args!["pvp_room"])?;
                        break 'b1;
                    }
                    if !matched1 && subject1 == 4 {
                        matched1 = true;
                    }
                    if matched1 {
                        ctx.lines_as(
                            "Gate Keeper",
                            args![
                                "With war raging between monsters",
                                "and humans, This competition",
                                "among people - PVP -",
                                "encourages us all to get",
                                "stronger. Come again,",
                                "we welcome your challenge!"
                            ],
                        )?;
                        break 'b1;
                    }
                }
                ctx.close_window()?;
                return Err(Stop::End);
            }
            GateKeeperStep::LWarp => {
                let map = runtime::arg(&args, 0, Val::from(0));
                ctx.call(Function::Warp, args![map, 51, 23])?;
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn gate_keeper(ctx: &Ctx) -> Script {
    gate_keeper_run(ctx, GateKeeperStep::Start, Vec::new()).map(|_| ())
}

pub fn registration_staff_1(ctx: &Ctx) -> Script {
    if ctx.call(Function::CountItem, args![7028])? == 0 {
        ctx.lines_as(
            "PVP Combat Square Register Staff",
            args![
                "Eh? How did you get in here?",
                "This is the entrance for players only.",
                "For details about viewers please proceed to the Register Staff on your right."
            ],
        )?;
        return ctx.close();
    }
    let l_size = runtime::array_size(ctx, ".warp_x")?;
    if l_size == 0 {
        for (index, x) in WARP_X.iter().enumerate() {
            ctx.var(".warp_x")
                .set_at(runtime::index(&Val::from(index as i32))?, Val::from(*x))?;
        }
        for (index, y) in WARP_Y.iter().enumerate() {
            ctx.var(".warp_y")
                .set_at(runtime::index(&Val::from(index as i32))?, Val::from(*y))?;
        }
    }
    if ctx.menu(&["Combat Square one", "Cancel"])? == 0 {
        ctx.lines_as(
            "PVP Combat Square Register Staff",
            args![Val::from("'") + ctx.player().name()? + Val::from("'"), "Are you ready?!"],
        )?;
        ctx.next()?;
        if ctx.menu(&["Yes!", "No!"])? == 0 {
            ctx.lines_as(
                "PVP Combat Square Register Staff",
                args!["OK! I will send you inside", "Good luck!"],
            )?;
            ctx.close_window()?;
            ctx.items().take(7028, 1)?;
            let l_rand = ctx.call(Function::Rand, args![l_size])?;
            ctx.call(
                Function::Warp,
                args![
                    "pvp_2vs2",
                    ctx.var(".warp_x").get_at(runtime::index(&l_rand)?)?,
                    ctx.var(".warp_y").get_at(runtime::index(&l_rand)?)?
                ],
            )?;
            runtime::array_delete(ctx, ".warp_x", &l_rand, Some(&Val::from(1)))?;
            runtime::array_delete(ctx, ".warp_y", &l_rand, Some(&Val::from(1)))?;
            return ctx.end();
        }
        ctx.lines_as("PVP Combat Square Register Staff", args!["Come back anytime you are ready."])?;
        return ctx.close();
    }
    ctx.close()
}

pub fn registration_staff_1_oninit(ctx: &Ctx) -> Script {
    ctx.call(Function::WaitingRoom, args!["Combat Square players entrance only", 0])?;
    ctx.end()
}

pub fn spectator_s_entrance_dum(ctx: &Ctx) -> Script {
    if ctx.call(Function::CountItem, args![7029])? == 0 {
        ctx.lines_as(
            "PVP Compete Square Register Staff",
            args![
                "This is the entrance for viewers.",
                "For details about players entrance please proceed to the Register Staff on your left."
            ],
        )?;
        return ctx.close();
    }
    if ctx.menu(&["Compete Square one", "Cancel"])? == 0 {
        ctx.lines_as(
            "PVP Combat Square Register Staff",
            args!["You got it, thanks for participating. Have fun!"],
        )?;
        ctx.close_window()?;
        ctx.items().take(7029, 1)?;
        match ctx.rand_range(1, 4)? {
            1 => {
                ctx.warp("pvp_2vs2", 39, 7)?;
                return ctx.end();
            }
            2 => {
                ctx.warp("pvp_2vs2", 39, 73)?;
                return ctx.end();
            }
            3 => {
                ctx.warp("pvp_2vs2", 7, 39)?;
                return ctx.end();
            }
            4 => {
                ctx.warp("pvp_2vs2", 73, 39)?;
                return ctx.end();
            }
            _ => {}
        }
    }
    ctx.close()
}

pub fn spectator_s_entrance_dum_oninit(ctx: &Ctx) -> Script {
    ctx.call(Function::WaitingRoom, args!["Compete Square viewer's entrance", 0])?;
    ctx.end()
}

#[derive(Clone, Copy, Debug)]
enum CombatSquareStaffDumStep {
    Start,
    OnTouch,
}

fn combat_square_staff_dum_run(ctx: &Ctx, mut step: CombatSquareStaffDumStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            CombatSquareStaffDumStep::Start => {
                step = CombatSquareStaffDumStep::OnTouch;
                continue 'machine;
            }
            CombatSquareStaffDumStep::OnTouch => {
                ctx.lines_as("Combat Square Staff", args!["May I help you?"])?;
                if ctx.menu(&["To the center viewer seat.", "Leave Combat Square."])? == 0 {
                    ctx.warp("pvp_2vs2", 38, 38)?;
                    return Err(Stop::End);
                }
                ctx.warp("pvp_room", 84, 39)?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn combat_square_staff_dum(ctx: &Ctx) -> Script {
    combat_square_staff_dum_run(ctx, CombatSquareStaffDumStep::Start, Vec::new()).map(|_| ())
}

pub fn combat_square_staff_dum_ontouch(ctx: &Ctx) -> Script {
    combat_square_staff_dum_run(ctx, CombatSquareStaffDumStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn combat_square_staff_5(ctx: &Ctx) -> Script {
    ctx.lines_as("Combat Square Staff", args!["May I help you?"])?;
    'b1: {
        let subject1 = Val::from(runtime::select_values(
            ctx,
            &[Val::from("To the side viewer seat.:Leave Combat Square.")],
        )?);
        let mut matched1 = false;
        if !matched1 && subject1 == 1 {
            matched1 = true;
        }
        if matched1 {
            match ctx.rand_range(1, 4)? {
                1 => {
                    ctx.warp("pvp_2vs2", 39, 7)?;
                    return ctx.end();
                }
                2 => {
                    ctx.warp("pvp_2vs2", 39, 73)?;
                    return ctx.end();
                }
                3 => {
                    ctx.warp("pvp_2vs2", 7, 39)?;
                    return ctx.end();
                }
                4 => {
                    ctx.warp("pvp_2vs2", 73, 39)?;
                    return ctx.end();
                }
                _ => {}
            }
        }
        if !matched1 && subject1 == 2 {
            matched1 = true;
        }
        if matched1 {
            ctx.warp("pvp_c_room", 84, 39)?;
            return ctx.end();
        }
    }
    Ok(())
}

#[derive(Clone, Copy, Debug)]
enum OutEventpvpStep {
    Start,
    OnTouch,
}

fn out_eventpvp_run(ctx: &Ctx, mut step: OutEventpvpStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            OutEventpvpStep::Start => {
                step = OutEventpvpStep::OnTouch;
                continue 'machine;
            }
            OutEventpvpStep::OnTouch => {
                ctx.lines_as(
                    "Combat Square Staff",
                    args!["Did you have fun in Combat Square?", "May I ask where you want to go?"],
                )?;
                ctx.next()?;
                let choice = ctx.menu(&["Prontera.", "Morocc.", "Geffen.", "Payon.", "Alberta.", "Cancel."])?;
                if choice == 5 {
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                let (map, x, y) = EXIT_WARPS[choice];
                ctx.warp(map, x, y)?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn out_eventpvp(ctx: &Ctx) -> Script {
    out_eventpvp_run(ctx, OutEventpvpStep::Start, Vec::new()).map(|_| ())
}

pub fn out_eventpvp_ontouch(ctx: &Ctx) -> Script {
    out_eventpvp_run(ctx, OutEventpvpStep::OnTouch, Vec::new()).map(|_| ())
}

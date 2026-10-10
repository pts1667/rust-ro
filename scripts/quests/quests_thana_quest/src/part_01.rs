use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum TowerKeeperStep {
    Start,
    OnTouch,
}

fn tower_keeper_run(ctx: &Ctx, mut step: TowerKeeperStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            TowerKeeperStep::Start => {
                ctx.lines_as(
                    "Gatei",
                    args![
                        "Greetings, adventurer.",
                        "I am Gatei Knumm, keeper",
                        "of this Thanatos Tower.",
                        "How may I help you?"
                    ],
                )?;
                ctx.next()?;
                'b1: {
                    let subject1 = Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("Thanatos Tower?:Entrance Fee?:Enter the tower:Cancel")],
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
                            "Gatei",
                            args![
                                "Yes, the tower in front",
                                "of you was an ancient ruins",
                                "that got its name from a word",
                                "written on a stone plate found",
                                "inside. Today, this place is",
                                "a popular tourist attraction."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Gatei",
                            args![
                                "This tower grew in popularity",
                                "for adventurers when the",
                                "Rekenber Corporation began",
                                "tower reconstruction efforts.",
                                "The 1st and 2nd floors are now",
                                "repaired and free of monsters."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Gatei",
                            args![
                                "However, reconstruction of",
                                "the 3rd and higher floors is",
                                "incomplete. Those levels are",
                                "still infested with monsters, so we're contracting temp workers",
                                "to exterminate all of them."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Gatei",
                            args![
                                "If you're interested in",
                                "temporary contract work,",
                                "or would like to know about",
                                "this place in detail, please",
                                "ask one of the guides inside.",
                                "Thank you and enjoy your visit~"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                        matched1 = true;
                    }
                    if matched1 {
                        ctx.lines_as(
                            "Gatei",
                            args![
                                "Everyone is welcome to",
                                "enjoy Thanatos Tower.",
                                "Only the 1st and 2nd floors",
                                "are open to the public at",
                                "this time. The entrance fee",
                                "is 5,000 zeny per person."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Gatei",
                            args![
                                "5,000 zeny may seem a little",
                                "steep for a tourist attraction,",
                                "but trust me, this tower provides a very unique experience. Also,",
                                "our loyal customers and contract workers get a special discount~"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    if !matched1 && subject1.loosely_equals(&Val::from(3)) {
                        matched1 = true;
                    }
                    if matched1 {
                        ctx.lines_as(
                            "Gatei",
                            args![
                                "Would you like to",
                                "enter Thanatos Tower?",
                                "The entry fee is ^FF00005,000 zeny^000000."
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Enter:Maybe next time.")])? {
                            1 => {
                                if ctx.var("thana_tower").get()?.number()? > 0 {
                                    ctx.lines_as(
                                        "Gatei",
                                        args![
                                            ((Val::from("Oh, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("!")),
                                            "Welcome back! Since you've",
                                            "got a temporary Rekenber",
                                            "work contract, your entrance",
                                            "fee is only 3,000 zeny."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    match runtime::select_values(ctx, &[Val::from("Enter:No, thanks.")])? {
                                        1 => {
                                            if ctx.var("Zeny").get()?.number()? > 2999 {
                                                ctx.lines_as(
                                                    "Gatei",
                                                    args![
                                                        "Thank you, and",
                                                        "please keep up the",
                                                        "good work. Ah, and don't",
                                                        "forget: safety first when",
                                                        "you fight those monsters!"
                                                    ],
                                                )?;
                                                ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(3000))?))?;
                                                ctx.close_window()?;
                                                ctx.call(Function::Warp, vec![Val::from("tha_scene01"), Val::from(131), Val::from(220)])?;
                                                return Err(Stop::End);
                                            }
                                            ctx.lines_as(
                                                "Gatei",
                                                args![
                                                    "Oh, I'm sorry...",
                                                    "But you don't seem to",
                                                    "have enough zeny. Oh well,",
                                                    "just come back again later~"
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        2 => {
                                            ctx.lines_as(
                                                "Gatei",
                                                args!["You must be busy, then.", "Well, not to worry, we'll", "always be here. Farewell~"],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        _ => {}
                                    }
                                }
                                if ctx.var("Zeny").get()?.number()? > 4999 {
                                    ctx.lines_as(
                                        "Gatei",
                                        args![
                                            "Ah, I've received your",
                                            "entrance fee. Thank you",
                                            "very much. Now, I hope",
                                            "you enjoy your visit",
                                            "to Thanatos Tower~"
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(5000))?))?;
                                    ctx.call(Function::Warp, vec![Val::from("tha_scene01"), Val::from(131), Val::from(220)])?;
                                    return Err(Stop::End);
                                }
                                ctx.lines_as(
                                    "Gatei",
                                    args![
                                        "Well...",
                                        "Hm. I'm sorry, but you",
                                        "don't seem to have enough",
                                        "zeny for the entrance fee.",
                                        "Please come back later..."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    "Gatei",
                                    args!["Very well. Please come", "and visit us again here", "in Thanatos Tower."],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                    if !matched1 && subject1.loosely_equals(&Val::from(4)) {
                        matched1 = true;
                    }
                    if matched1 {
                        ctx.lines_as(
                            "Gatei",
                            args![
                                "Hmm? Very well.",
                                "But if you wish to visit",
                                "this Thanatos Tower, please",
                                "do not hesitate to ask me.",
                                "Thank you and have a nice day."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                step = TowerKeeperStep::OnTouch;
                continue 'machine;
            }
            TowerKeeperStep::OnTouch => {
                ctx.lines_as(
                    "Tower Keeper",
                    args![
                        "Excuse me, but you cannot",
                        "enter. This place is under",
                        "the Rekenber Corporation's",
                        "administration, and this area",
                        "is restricted to all those",
                        "without authorization."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn tower_keeper(ctx: &Ctx) -> Script {
    tower_keeper_run(ctx, TowerKeeperStep::Start, Vec::new()).map(|_| ())
}

pub fn tower_keeper_ontouch(ctx: &Ctx) -> Script {
    tower_keeper_run(ctx, TowerKeeperStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum GuideStep {
    Start,
    LContract,
}

fn guide_run(ctx: &Ctx, mut step: GuideStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            GuideStep::Start => {
                if ctx.var("thana_tower").get()? == 0 {
                    ctx.lines_as(
                        "Ditze",
                        args![
                            "Welcome to Thanatos Tower.",
                            "The tower's reconstruction is",
                            "a Rekenber Corporation project,",
                            "and the 1st and 2nd floors are",
                            "now open to the public. So",
                            "how may I help you today?"
                        ],
                    )?;
                    ctx.next()?;
                    'b1: {
                        let subject1 = Val::from(runtime::select_values(
                            ctx,
                            &[Val::from("Tower Information:Temporary Work Contract:Cancel")],
                        )?);
                        let mut matched1 = false;
                        let no_case1 = !subject1.loosely_equals(&Val::from(1))
                            && !subject1.loosely_equals(&Val::from(2))
                            && !subject1.loosely_equals(&Val::from(3));
                        if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                            matched1 = true;
                        }
                        if matched1 {
                            ctx.lines_as(
                                "Ditze",
                                args![
                                    "When this tower was built,",
                                    "who built it, and its purpose",
                                    "are all mysteries. There are",
                                    "many rumors about it being a",
                                    "Mage lab, a hero's momunent,",
                                    "or a demon fortress..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Ditze",
                                args![
                                    "The Rekenber Corporation has",
                                    "been researching the origin of",
                                    "this tower, but has not yet been able to confirm anything. Although",
                                    "we've lost many researchers to the tower monsters, we won't give up!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Ditze",
                                args![
                                    "Rekenber is convinced that this",
                                    "tower holds many secrets, and",
                                    "successfully reconstructed the",
                                    "first 2 floors of this tower in",
                                    "the pursuit of this knowledge."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Ditze",
                                args![
                                    "Currently, we are focused on",
                                    "reconstructing the tower's 3rd",
                                    "and 4th floors. Luckily, many",
                                    "adventurers are exterminating",
                                    "the monsters on those floors, working under temporary contracts."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Ditze",
                                args![
                                    "At this rate, we expect",
                                    "to complete reconstruction",
                                    "of the 3rd and 4th floors in",
                                    "the near future, bringing us",
                                    "closer to our goal of opening all 12 floors of Thanatos Tower."
                                ],
                            )?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("Tower Monsters?:Temp Contract Work?:......")])? {
                                1 => {
                                    ctx.lines_as(
                                        "Ditze",
                                        args![
                                            "Yes, when we began",
                                            "reconstruction of the",
                                            "Thanatos Tower, these",
                                            "monsters that look like angels",
                                            "just appeared out of nowhere",
                                            "to attack all the workers."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Ditze",
                                        args![
                                            "At first, witnesses thought",
                                            "that these monsters might",
                                            "be, you know, actual angels.",
                                            "But if they were, why were they",
                                            "attacking for no reason?"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Ditze",
                                        args![
                                            "Anyway, we asked the Juno ",
                                            "Sage Academy to investigate",
                                            "them, and they confirmed that",
                                            "these angelic creatures are true monsters--their resemblance to",
                                            "angels is merely coincidence."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                2 => {
                                    ctx.lines_as(
                                        "Ditze",
                                        args![
                                            "You may have already heard",
                                            "from the Tower Keeper, but",
                                            "we're contracting adventurers to exterminate the tower monsters",
                                            "in the 3rd and higher levels on",
                                            "a temporary employee basis."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Ditze",
                                        args![
                                            "I'm in charge of hiring",
                                            "temp contract workers, so",
                                            "talk to me if you're interested. Only temp workers are allowed",
                                            "to access the higher levels",
                                            "here in Thanatos Tower."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    step = GuideStep::LContract;
                                    continue 'machine;
                                }
                                3 => {
                                    ctx.lines_as(
                                        "Ditze",
                                        args![
                                            "To develop the floors above",
                                            "the 2nd floor, we're going to",
                                            "need as many temp contract",
                                            "workers as we can hire. Why",
                                            "don't you consider working",
                                            "for us under a temp contract?"
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
                                "Ditze",
                                args![
                                    "You may have already heard",
                                    "from the Tower Keeper, but",
                                    "we're contracting adventurers to exterminate the tower monsters",
                                    "in the 3rd and higher levels on",
                                    "a temporary employee basis."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Ditze",
                                args![
                                    "I'm in charge of hiring",
                                    "temp contract workers, so",
                                    "talk to me if you're interested. Only temp workers are allowed",
                                    "to access the higher levels",
                                    "here in Thanatos Tower."
                                ],
                            )?;
                            ctx.next()?;
                            step = GuideStep::LContract;
                            continue 'machine;
                        }
                        if !matched1 && subject1.loosely_equals(&Val::from(3)) {
                            matched1 = true;
                        }
                        if matched1 {
                            ctx.lines_as(
                                "Ditze",
                                args![
                                    "Well, if you have any",
                                    "questions, feel free to",
                                    "ask me later. My name is",
                                    "Ditze Lappa. Have a good day!"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                }
                ctx.lines_as(
                    "Ditze",
                    args![
                        "For more detailed information",
                        "about monster exterminations,",
                        "please ask the 2nd Floor Guide",
                        "and the Guide next to me. Well,",
                        "we hope you enjoy your experience working with Rekenber Corporation~"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            GuideStep::LContract => {
                match runtime::select_values(ctx, &[Val::from("Maybe next time:Sure, I'd like to work for you.")])? {
                    1 => {
                        ctx.lines_as(
                            "Ditze",
                            args![
                                "Well, alright.",
                                "But come and talk to",
                                "me as soon as you decide",
                                "that you want to help with",
                                "the tower reconstruction by",
                                "working for Rekenber."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as(
                            "Ditze",
                            args![
                                "That's great!",
                                "Would you please fill",
                                "out this employment",
                                "agreement first? Let's",
                                "see. Your name is...",
                                (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(", right?"))
                            ],
                        )?;
                        ctx.next()?;
                        let choice = runtime::select_values(ctx, &[Val::from("Yes")])?;
                        ctx.var("@menu").set(choice)?;
                        ctx.lines_as(
                            "Ditze",
                            args![
                                "Alright, please read",
                                "this agreement carefully.",
                                "If you agree with all of",
                                "the conditions, go ahead",
                                "and just sign the bottom."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^3355FFDitze hands you the",
                            "Employment Agreement",
                            "document for you to read.^000000"
                        ])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "- Employment Agreement -",
                            " ",
                            "1. This employment agreement",
                            "is effective between Rekenber",
                            ((Val::from("Corporation (''Employer'') and ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                + Val::from("(''Employee'').")),
                            "1-1. This employment agreement",
                            "is classified as a Mercernary",
                            "contract between both parties.",
                            "2. The terms of this contract",
                            "immediately take effect once",
                            "it is signed by both parties",
                            "(Employer and Employee).",
                            "3. When the Employee performs",
                            "a mission, the mission results",
                            "must be verified with acceptible physical proof that must be",
                            "presented to the Employer.",
                            "3-1. Please refer to Section",
                            "7A for examples of acceptible",
                            "proof of monster extermination.",
                            "3-2. Acceptible proof is",
                            "determined and defined by a",
                            "representative of the Employer.",
                            "4. Employer will allot rewards",
                            "to Employees after receiving",
                            "extermination proof.",
                            "4-1. Possible rewards for",
                            "monster exterminations may",
                            "include the following.",
                            ".................................................",
                            ".................................................",
                            ".................................................",
                            ".................................................",
                            ".................................................",
                            "13. This contract's terms are",
                            "only applicable witin Thanatos",
                            "Tower.",
                            "14. Employees will receive",
                            "a discount on the Entry Fee",
                            "to the Thanatos Tower.",
                            " ",
                            " ",
                            " ",
                            "Employer Signature____________",
                            "Employee Signature____________"
                        ])?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Sign:Don't Sign")])? {
                            1 => {
                                ctx.lines(args!["^3355FFYou sign two copies of the", "Employment Agreement.^000000"])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Ditze",
                                    args![
                                        "Thank you. Now, you are",
                                        "an official employee of the",
                                        "Rekenber Corporation! Well,",
                                        "within the limits of this tower",
                                        "anyway. This contract doesn't",
                                        "apply outside of this place."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Ditze",
                                    args![
                                        "From now on, you can talk",
                                        "to the 2nd Floor Guide to",
                                        "enter the 3rd Floor. If you",
                                        "have any mission reward",
                                        "questions, please ask ^3355FFLiei^000000."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Ditze",
                                    args![
                                        "If you want to check",
                                        "more information about",
                                        "your work, please talk",
                                        "to the 2nd Floor Guide.",
                                        "Thank you, and welcome",
                                        "to the Rekenber Corporation~"
                                    ],
                                )?;
                                ctx.var("thana_tower").set(Val::from(1))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    "Ditze",
                                    args![
                                        "Oh? Was there an article",
                                        "within the contract that you",
                                        "disagreed with? Hm. Well,",
                                        "that's fine. But if you change",
                                        "your mind, please come back",
                                        "and ask me anytime. Thank you~"
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn guide(ctx: &Ctx) -> Script {
    guide_run(ctx, GuideStep::Start, Vec::new()).map(|_| ())
}

fn guide_reward_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    let mut l_items: Vec<Val> = Vec::new();
    let mut l_zeny_tt = Val::from(0);
    ctx.mes("[Liei]")?;
    if ctx.var("thana_tower").get()? == 0 {
        ctx.lines(args![
            "Good day, I'm",
            "Liei Kuniziet of the",
            "Employee Mission",
            "Reward Department",
            "here in Thanatos Tower."
        ])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Employee's mission reward?:Keep up the good work.")])? {
            1 => {
                ctx.lines_as(
                    "Liei",
                    args![
                        "Currently, Rekenber Corporation",
                        "is contracting temp employees",
                        "to develop the higher levels",
                        "of Thanatos Tower. If you'd",
                        "like to apply, please ask",
                        "Ditze right next to me."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Liei",
                    args!["Thank you. Ah, and I hope", "that you enjoy your visit", "here to Thanatos Tower."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    ctx.lines(args!["Ah, hello~", "How may I help you?"])?;
    ctx.next()?;
    if Val::from(runtime::select_values(ctx, &[Val::from("Reward:Nothing")])?) == 2 {
        ctx.lines_as(
            "Liei",
            args![
                "Alright, then.",
                "Please do your best",
                "to exterminate the",
                "monsters that infest",
                "the higher floors of",
                "the Thanatos Tower~"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Liei",
        args![
            ((Val::from("You're ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(", yes?")),
            "Let me check our temp",
            "employee records for--ah.",
            "Here it is. Alright, so would",
            "you please tell me what kind",
            "of mission proof you brought?"
        ],
    )?;
    ctx.next()?;
    let base = Val::from(1).number()?;
    runtime::local_set(&mut l_items, &Val::from(base + 0), Val::from(7435), false);
    runtime::local_set(&mut l_items, &Val::from(base + 1), Val::from(7440), false);
    runtime::local_set(&mut l_items, &Val::from(base + 2), Val::from(7441), false);
    runtime::local_set(&mut l_items, &Val::from(base + 3), Val::from(7442), false);
    l_i = Val::from(runtime::select_values(
        ctx,
        &[Val::from("Golden Ornament:Red Feather:Blue Feather:Cursed Seal")],
    )?);
    ctx.mes("[Liei]")?;
    if !(ctx
        .call(Function::CountItem, vec![runtime::local_get(&l_items, &l_i.clone(), false)])?
        .is_true())
    {
        ctx.lines(args![
            "I'm sorry, but you are not",
            ((Val::from("carrying any ")
                + shared::other_global_functions::f_getplural(
                    ctx,
                    vec![ctx.call(Function::GetItemName, vec![runtime::local_get(&l_items, &l_i.clone(), false)])?]
                )?)
                + Val::from(".")),
            "Please check your inventory",
            "one more time, and then come",
            "to me to redeem your items",
            "for a reward later, alright?"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines(args![
        "The reward for each",
        (ctx.call(Function::GetItemName, vec![runtime::local_get(&l_items, &l_i.clone(), false)])? + Val::from(" is...")),
        " ",
        "1,000 zeny"
    ])?;
    ctx.next()?;
    l_zeny_tt = (ctx
        .call(Function::CountItem, vec![runtime::local_get(&l_items, &l_i.clone(), false)])?
        .try_mul(Val::from(1000))?);
    ctx.lines_as(
        "Liei",
        args![
            (shared::other_global_functions::f_insertplural(
                ctx,
                vec![
                    ctx.call(Function::CountItem, vec![runtime::local_get(&l_items, &l_i.clone(), false)])?,
                    ctx.call(Function::GetItemName, vec![runtime::local_get(&l_items, &l_i.clone(), false)])?
                ]
            )? + Val::from(", then")),
            "you will receive a total of...",
            " ",
            ((Val::from("") + l_zeny_tt.clone()) + Val::from(" zeny"))
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Liei",
        args![
            "Would you like to exchange",
            (Val::from("all of your ")
                + shared::other_global_functions::f_getplural(
                    ctx,
                    vec![ctx.call(Function::GetItemName, vec![runtime::local_get(&l_items, &l_i.clone(), false)])?]
                )?),
            "for your reward right now?"
        ],
    )?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("Yes:No")])? {
        1 => {
            ctx.lines_as(
                "Liei",
                args![
                    "Great! Here is your",
                    (l_zeny_tt.clone() + Val::from(" zeny. Thank you,")),
                    "and please keep up",
                    "the good work~"
                ],
            )?;
            ctx.call(
                Function::DelItem,
                vec![
                    runtime::local_get(&l_items, &l_i.clone(), false),
                    ctx.call(Function::CountItem, vec![runtime::local_get(&l_items, &l_i.clone(), false)])?,
                ],
            )?;
            ctx.var("Zeny").set((ctx.var("Zeny").get()? + l_zeny_tt.clone()))?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        2 => {
            ctx.lines_as(
                "Liei",
                args![
                    "Sure, no problem.",
                    "Just come back and",
                    "talk to me whenever",
                    "you want to receive",
                    "your reward, alright?"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn guide_reward(ctx: &Ctx) -> Script {
    guide_reward_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum EntranceGuideStep {
    Start,
    LRequest,
}

fn entrance_guide_run(ctx: &Ctx, mut step: EntranceGuideStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            EntranceGuideStep::Start => {
                ctx.mes("[Burled]")?;
                if ctx.var("thana_tower").get()? == 0 {
                    ctx.lines(args![
                        "You are in front of the entrance to the 3rd Floor. Only contracted",
                        "temp employees are authorized",
                        "to enter that area. For Rekenber temp contract information, please",
                        "speak to the 2nd Floor Guide."
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines(args![
                    "This is the path to the 3rd floor.",
                    "Only the contracted staff are allowed to enter.",
                    "How can I help you?"
                ])?;
                ctx.next()?;
                match runtime::select_values(
                    ctx,
                    &[Val::from("Let me go to 3rd floor.:Tower Information.:Start a conversation.")],
                )? {
                    1 => {
                        ctx.lines_as(
                            "Burled",
                            args![
                                "Oh, alright. Let me",
                                "check and see if you're",
                                "on our temp list. Hmmm...",
                                ((Val::from("Ah, you're ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(", right?"))
                            ],
                        )?;
                        ctx.next()?;
                        ctx.mes("[Burled]")?;
                        if ctx
                            .call(
                                Function::GetAreaUsers,
                                vec![Val::from("tha_t02"), Val::from(226), Val::from(156), Val::from(236), Val::from(166)],
                            )?
                            .number()?
                            < 5
                        {
                            ctx.lines(args![
                                "First, we need to wait until",
                                "at least 5 temps are gathered",
                                "to form a work group. Right",
                                "now, there are a total of",
                                (ctx.call(
                                    Function::GetAreaUsers,
                                    vec![Val::from("tha_t02"), Val::from(226), Val::from(156), Val::from(236), Val::from(166)]
                                )? + Val::from(" temp workers waiting to")),
                                "enter the 3rd Floor."
                            ])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Burled",
                                args![
                                    "If you can, try to get",
                                    "your friends to help you",
                                    "by coming near me. Please",
                                    "understand that we have this",
                                    "temp worker group requirement",
                                    "for various safety reasons."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines(args![
                            "Great, you're just",
                            "in time. We just met",
                            "the minimum 5 temp group",
                            "requirement, so we'll open",
                            "the gate to the 3rd Floor soon."
                        ])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Burled",
                            args![
                                "The gate to the 3rd Floor",
                                "will close shortly, so enter it",
                                "as soon as you can. We have",
                                "to close it quickly because we",
                                "can't have the tower monsters",
                                "entering the lower floors..."
                            ],
                        )?;
                        if ctx.var("thana_tower").get()?.number()? > 3 {
                            ctx.next()?;
                            ctx.lines(args![
                                "^4d4dffBurled is smiling lightly so that only I notice.",
                                "I nod, then watch him open the gate.^000000"
                            ])?;
                        }
                        ctx.close_window()?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("3rdf_warp#tt::OnEnable")])?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as(
                            "Burled",
                            args![
                                "This gate is the only passage",
                                "that connects to the 3rd Floor.",
                                "After the 4th Floor, passages",
                                "between floors only travel one",
                                "way, meaning you can't exit the",
                                "same way you that you entered."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Burled",
                            args![
                                "You see, there's a strange",
                                "power that affects the 5th",
                                "Floor, and all floors above,",
                                "which doesn't allow people to",
                                "backtrack through the passage in which they entered the floor."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Burled",
                            args![
                                "So if you ascend past the",
                                "4th Floor, please be careful",
                                "and make sure that you find",
                                "a way to get back."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Burled",
                            args![
                                "The top is the 12th floor,",
                                "and the monsters grow more powerful",
                                "as you ascend the tower.",
                                "The geographical features also change considerably."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Burled",
                            args![
                                "Because the higher levels are",
                                "too dangerous, we only open the",
                                "3rd Floor Gate when 5 or more",
                                "are gathered here. Therefore,",
                                "you may need to wait if less",
                                "than 5 temps have gathered."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    3 => {
                        if ctx.var("thana_tower").get()?.number()? < 3 {
                            ctx.lines_as("Burled", args!["...?", "Do you have any questions?"])?;
                            ctx.next()?;
                            if Val::from(runtime::select_values(
                                ctx,
                                &[Val::from(
                                    "About the development.:The relationship of Cool Event Corporation and Rekenber Corporation.:Nope.",
                                )],
                            )?) == 3
                            {
                                ctx.lines_as("Burled", args!["Take care."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.lines_as(
                                "Burled",
                                args![
                                    "...um, you're quite curious.",
                                    "If you want to know more, would you do me a favor?",
                                    "Since you asked for our confidential info..."
                                ],
                            )?;
                            ctx.next()?;
                            if Val::from(runtime::select_values(
                                ctx,
                                &[Val::from(
                                    "Really? Don't make me afraid of it.:Just tell me about it.:I'll listen about it later.",
                                )],
                            )?) == 3
                            {
                                ctx.lines_as("Burled", args!["As you wish...", "Take care!"])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.lines_as(
                                "Burled",
                                args![
                                    "Haha. That's not a big deal.",
                                    "We, Cool Event, are the corporation that works in many areas of business.",
                                    "From simple events, guides for dungeons, and trading items."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Burled", args!["Though our business started as a small one, we've been getting bigger with assistance from the Rekenber Corporation.", "They suggested to develop this tower together."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Burled",
                                args![
                                    "The Rekenber Corporation is only digging out the remains for studies.",
                                    "The rest would be made into sightseeing place, which Cool Event Corporation can manage well."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Burled",
                                args![
                                    "It's the biggest business ever.",
                                    "Also quite profitable as tourist attractions.",
                                    "But..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Burled",
                                args![
                                    "Considerably strong monsters spawn inside the tower.",
                                    "The monsters weren't spawned there before.",
                                    "The day after that happened..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Burled",
                                args![
                                    "...anyway, there were some accidently spawned monsters and some troubles...",
                                    "We and the Rekenber Corporation are in shock.",
                                    "We were influenced economically, but the more serious thing is..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Burled",
                                args![
                                    "Many staff involved in development are",
                                    "sacrificing themselves by coming to 3rd floor."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Burled",
                                args![
                                    "While we've pumped our all into the business, that accident happened...",
                                    "We eventually merged with the Rekenber Corporation."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Burled",
                                args![
                                    "The development is on hold temporarily.",
                                    "Recruiting adventurers to defeat monsters...",
                                    "Isn't this funny?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Burled",
                                args![
                                    "No matter what happened there, they wouldn't be hurt.",
                                    "Even if they don't get at the root of the accident...."
                                ],
                            )?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("Oh...:What accident are you referring to?")])? {
                                1 => {
                                    ctx.lines_as(
                                        "Burled",
                                        args![
                                            "Yes, it's not that big of a deal. Haha....",
                                            "If we are working here like this....",
                                            "we would know about the death of animals."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Burled",
                                        args![
                                            "Cool Event Corporation is considered the follower...",
                                            "They don't clear up how many people",
                                            "died or any reasons for it."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Burled",
                                        args!["Now you're curious about this case.", "What do they sacrifice for?"],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Burled",
                                        args![
                                            "I talked a lot.",
                                            "Sorry, I am not supposed to talk about this case like this...",
                                            "I've been quite concerned about it, that's all."
                                        ],
                                    )?;
                                    ctx.var("thana_tower").set(Val::from(3))?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                2 => {
                                    ctx.lines_as("Burled", args!["If you promise me that", "you will do me a favor..."])?;
                                    ctx.next()?;
                                    match runtime::select_values(ctx, &[Val::from("Ok! I will.:No, I won't.")])? {
                                        1 => {
                                            entrance_guide_run(ctx, EntranceGuideStep::LRequest, vec![])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                args!["(What's the accident...?", "He might let me know later...)"],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        2 => {
                                            ctx.lines_as(
                                                "Burled",
                                                args![
                                                    "...there is no way...",
                                                    "They request you to explore the internal part.",
                                                    "You're employed for this exploration."
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
                        } else if ctx.var("thana_tower").get()? == 3 {
                            entrance_guide_run(ctx, EntranceGuideStep::LRequest, vec![])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.var("thana_tower").get()? == 4 {
                            ctx.lines_as(
                                "Burled",
                                args![
                                    "Rekenber Corporation must have another intention,",
                                    "since they don't seem interested in the tower much.",
                                    "I want to know their intention."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Burled",
                                args![
                                    "This tower is sealed and separated from the outside.",
                                    "We need to break those seals, explore inside and develop it."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Burled",
                                args![
                                    "We are developing the 3rd and 4th floors now...",
                                    "There are still seals there.",
                                    "You can find out some clues when you search around them."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if (ctx.var("thana_tower").get()?.number()? > 4 && ctx.var("thana_tower").get()?.number()? < 9) {
                            ctx.lines_as(
                                "Burled",
                                args![
                                    "The magical key and messages?",
                                    "...we absolutely haven't learned all the tower's secrets..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Burled",
                                args![
                                    "Who made the seals?",
                                    "...can you investigate more?",
                                    "There must be definitive evidence..."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.var("thana_tower").get()? == 9 {
                            ctx.lines_as("Burled", args!["Any new progress?"])?;
                            ctx.next()?;
                            let choice = runtime::select_values(ctx, &[Val::from("I found Varmunt's Journal...")])?;
                            ctx.var("@menu").set(choice)?;
                            ctx.lines(args!["- You show him Varmunt's Journal.", "Burled reads it. -"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Burled",
                                args![
                                    "...I recognize this.",
                                    "This is the hologragh of the wise man Varmunt.",
                                    "Regenschirm from Rekenber's huge research institute has..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Burled",
                                args![
                                    "...isn't this the one they are looking for now?!",
                                    "Isn't their mission to find out Varmunt's research materials...?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Burled",
                                args![
                                    "For assisting uncompleted research...",
                                    "Varmunt's research materials are needed...",
                                    "...so their mission isn't just an investigation of the tower."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Burled",
                                args![
                                    "Something to get as an advantage...",
                                    "Even they don't know where it is...",
                                    "...anyways, this is the one of their main objects."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Burled",
                                args![
                                    "It's absolutely great. The wise man Varmunt's belongings. How unbelievable!",
                                    "Now, what do we do with this?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Burled",
                                args![
                                    "Originally everything we found out in this tower was under control of the Rekenber Corporation.",
                                    "This is supposed to theirs, but..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Burled",
                                args![
                                    ((Val::from("How would you like to keep this, ")
                                        + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                        + Val::from("?")),
                                    "Definitely this should be a secret.",
                                    "...um, this is a little revenge for them."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Burled",
                                args![
                                    "I can guess that there are several secrets in this tower through this note.",
                                    ((Val::from("This secret should be disclosed by you, ")
                                        + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                        + Val::from(", before Rekenber does."))
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Burled",
                                args![
                                    "No matter how much power they obtain... they won't get to know about this tower...",
                                    "Haha..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Burled",
                                args![
                                    "...haha...",
                                    "I feel a bit relieved.",
                                    "Thanks.",
                                    "This isn't much, but here's a return present."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Burled",
                                args![
                                    ((Val::from("I hope that these will be useful for you, ")
                                        + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                        + Val::from("."))
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines(args![
                                "- Burled gives you the note and an Old Violet Box. -",
                                " ",
                                "^4d4dffYou acquire one Old Violet Box,",
                                "as well as a little EXP.^000000"
                            ])?;
                            ctx.var("thana_tower").set(Val::from(10))?;
                            ctx.call(Function::CompleteQuest, vec![Val::from(7053)])?;
                            ctx.call(Function::GetExperience, vec![Val::from(120000), Val::from(10000)])?;
                            ctx.call(Function::GetItem, vec![Val::from(617), Val::from(1)])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            ctx.lines_as(
                                "Burled",
                                args!["Are you keeping the secrets well?", "Is there a Phantom on the top of this tower?"],
                            )?;
                            if ctx.call(Function::Rand, vec![Val::from(3)])? == 1 {
                                ctx.mes("You look tired. This isn't a big deal, but it's for you.")?;
                                runtime::npc_skill(ctx, &Val::from("AL_HEAL"), &Val::from(11), &Val::from(50), &Val::from(70))?;
                            }
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                    _ => {}
                }
                step = EntranceGuideStep::LRequest;
                continue 'machine;
            }
            EntranceGuideStep::LRequest => {
                ctx.lines_as(
                    "Burled",
                    args![
                        "Ah, it's not that difficult.",
                        "After you explore the tower, let me know in detail anything you learn."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Burled",
                    args![
                        "...I can't believe the Rekenber Corporation. Many colleagues and friends died for them.",
                        "But the Rekenber Corporation doesn't expose the reason they are dead."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Burled",
                    args![
                        "The conditions were bad for us from beginning...",
                        "All of the developed places except research materials would be turned into tourist attractions...",
                        "Honestly, all the personnel are from Cool Event Corp."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Burled",
                    args![
                        "They know about it as well.",
                        "This business needs much sacrifice as well as personnel who are up for it."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Burled",
                    args![
                        "Now I don't feel victimized anymore, but I just want to know what drives these accidents...",
                        "^4d4dffWhat makes us victimized...?^000000",
                        "That's what I want to know."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Burled",
                    args![
                        "What's this tower?",
                        "Why did they develop this dangerous place?",
                        "And for what? Should my colleagues be dead for it?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Burled",
                    args![
                        "I seriously want to know about it.",
                        "Why they must sacrifice themselves...",
                        "Let me know..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines(args![
                    "He seriously wants to know about it.",
                    "His hands are trembling now...",
                    "I nodd without answering."
                ])?;
                ctx.var("thana_tower").set(Val::from(4))?;
                ctx.call(Function::SetQuest, vec![Val::from(7048)])?;
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn entrance_guide(ctx: &Ctx) -> Script {
    entrance_guide_run(ctx, EntranceGuideStep::Start, Vec::new()).map(|_| ())
}

fn s_3rdf_warp_tt_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn s_3rdf_warp_tt(ctx: &Ctx) -> Script {
    s_3rdf_warp_tt_body(ctx, Vec::new()).map(|_| ())
}

fn s_3rdf_warp_tt_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("3rdf_warp#tt")])?;
    return Err(Stop::End);
}

pub fn s_3rdf_warp_tt_oninit(ctx: &Ctx) -> Script {
    s_3rdf_warp_tt_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn s_3rdf_warp_tt_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("thana_tower").get()? == 0 {
        ctx.call(Function::Warp, vec![Val::from("tha_t02"), Val::from(227), Val::from(158)])?;
    } else {
        ctx.call(Function::Warp, vec![Val::from("tha_t03"), Val::from(219), Val::from(159)])?;
    }
    return Err(Stop::End);
}

pub fn s_3rdf_warp_tt_ontouch(ctx: &Ctx) -> Script {
    s_3rdf_warp_tt_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn s_3rdf_warp_tt_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("3rdf_warp#tt")])?;
    ctx.call(Function::InitNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn s_3rdf_warp_tt_onenable(ctx: &Ctx) -> Script {
    s_3rdf_warp_tt_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn s_3rdf_warp_tt_ontimer30000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::DisableNpc, vec![Val::from("3rdf_warp#tt")])?;
    return Err(Stop::End);
}

pub fn s_3rdf_warp_tt_ontimer30000(ctx: &Ctx) -> Script {
    s_3rdf_warp_tt_ontimer30000_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum RuneDeviceTt1Step {
    Start,
    OnTouch,
    LKey,
}

fn rune_device_tt1_run(ctx: &Ctx, mut step: RuneDeviceTt1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            RuneDeviceTt1Step::Start => {
                if (ctx.call(Function::CountItem, vec![Val::from(7421)])? == 0
                    && ctx.call(Function::CountItem, vec![Val::from(7426)])? == 0)
                {
                    ctx.lines(args![
                        "^3355FFA mysterious field of",
                        "energy seems to surround",
                        "the mechanical device and",
                        "its power prevents you from",
                        "approaching the machine.^000000"
                    ])?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Investigate it.:I don't care about it.")])? {
                        1 => {
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["This seems mysterious...", "Let me investigate."],
                            )?;
                            ctx.next()?;
                            ctx.mes("^3355FFAs you follow the magical power, there's something wrapped inside the rune device.^000000")?;
                            ctx.next()?;
                            if ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_TAEKWON")?) {
                                ctx.lines(args![
                                    "^3355FFYou kick the energy",
                                    "field with all of your strength. After absorbing the impact, the",
                                    "field fizzles out with a soft,",
                                    "gentle ''pzzzzzh'' sound.^000000"
                                ])?;
                                ctx.next()?;
                                step = RuneDeviceTt1Step::LKey;
                                continue 'machine;
                            } else if ctx.call(Function::GetEquipWeaponLevel, vec![ctx.constant("EQI_HAND_R")?])? == 4 {
                                ctx.lines(args![
                                    ((Val::from("^3355FFWith your ")
                                        + ctx.call(Function::GetEquipName, vec![ctx.constant("EQI_HAND_R")?])?)
                                        + Val::from(" in")),
                                    "hand, you smash the energy",
                                    "field with all of your strength. After absorbing the impact, the",
                                    "field fizzles out with a soft,",
                                    "gentle ''pzzzzzh'' sound.^000000"
                                ])?;
                                ctx.next()?;
                                step = RuneDeviceTt1Step::LKey;
                                continue 'machine;
                            } else {
                                ctx.lines(args![
                                    "^3355FFYou smash the energy",
                                    "field with your weapon",
                                    "using all of your strength,",
                                    "but you weren't able to",
                                    "break down the barrier.",
                                    "You probably need a more",
                                    "powerful weapon...^000000"
                                ])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                        2 => {
                            ctx.lines(args!["^3355FFYou decide to leave", "the machine alone.^000000"])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                }
                ctx.mes("You've acquired everything you need from this rune device.")?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            RuneDeviceTt1Step::OnTouch => {
                if (ctx.call(Function::CountItem, vec![Val::from(7421)])? == 0
                    && ctx.call(Function::CountItem, vec![Val::from(7426)])? == 0)
                {
                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_LEVEL99_4")?])?;
                }
                return Err(Stop::End);
            }
            RuneDeviceTt1Step::LKey => {
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BRANDISH2")?])?;
                ctx.lines(args![
                    "After breaking the device, the exterior shatters.",
                    "The energy field begins to disappear,",
                    "and you see that a red object inside was the origin of the magical power."
                ])?;
                ctx.next()?;
                ctx.mes("- You acquired the powerful Red Key. -")?;
                ctx.call(Function::GetItem, vec![Val::from(7421), Val::from(1)])?;
                if ctx.var("thana_tower").get()? != 4 {
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines(args![
                    "^4d4dffOnce you hold the key, a shocking feeling passes through your head.",
                    "You see an illusion of light...^000000"
                ])?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Ignore it.:Concentrate on it.")])? {
                    1 => {
                        ctx.lines(args!["^3355FFYou decide to leave", "the machine alone.^000000"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines(args!["You focus on the light.", "Some letters begin to appear..."])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^b22222I've used the Gate Seal",
                            "technology to seal the gate",
                            "and the charm stones. Although",
                            "the seals are in place, I can't",
                            "stop worrying that they might",
                            "break in the future."
                        ])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^b22222I can't relax when a, shall",
                            "I say, particular group covets",
                            "the charm stones and can easily",
                            "break the seals. Since they are",
                            "broken now, are many people hurt?"
                        ])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^b22222This tower contains strong",
                            "magical powers and much evil. It is",
                            "dangerous by itself, but I sealed it",
                            "because of one man's strong desires..."
                        ])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^b22222I wouldn't recommend to go futher.",
                            "You would need to challenge that poor",
                            "being, who I've named this tower after.",
                            "His soul still rests here..."
                        ])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^b22222Nobody believed me, the crazy scientist.",
                            "Though I wanted to keep this a secret, somebody",
                            "must get to know about it. That's way I created",
                            "this rune device, and hid this message."
                        ])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^b22222Please show that my experience",
                            "wasn't just an illusion.^000000"
                        ])?;
                        ctx.next()?;
                        ctx.lines(args!["That's all.", "The letters fly away in the form of a red key..."])?;
                        ctx.var("thana_tower").set(Val::from(5))?;
                        ctx.call(Function::ChangeQuest, vec![Val::from(7048), Val::from(7049)])?;
                        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_COMBOATTACK1")?])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn rune_device_tt1(ctx: &Ctx) -> Script {
    rune_device_tt1_run(ctx, RuneDeviceTt1Step::Start, Vec::new()).map(|_| ())
}

pub fn rune_device_tt1_ontouch(ctx: &Ctx) -> Script {
    rune_device_tt1_run(ctx, RuneDeviceTt1Step::OnTouch, Vec::new()).map(|_| ())
}

fn rune_device_tt2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_ball = Val::from(0);
    let mut l_input = Val::from(0);
    let mut l_input10 = Val::from(0);
    let mut l_input100 = Val::from(0);
    let mut l_retry = Val::from(0);
    let mut l_strike = Val::from(0);
    let mut l_yagu1 = Val::from(0);
    let mut l_yagu10 = Val::from(0);
    let mut l_yagu100 = Val::from(0);
    if (ctx.call(Function::CountItem, vec![Val::from(7422)])? == 0 && ctx.call(Function::CountItem, vec![Val::from(7427)])? == 0) {
        ctx.lines(args![
            "^3355FFYou find a screen",
            "with three tiny panels and",
            "a numeric keypad underneath.",
            "As you press one of the",
            "number keys, you hear a",
            "beep as the screen activates.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Screen",
            args![
                "Please enter a 3 digit",
                "number. Do not use a",
                "single number more than",
                "once or use the number 0."
            ],
        )?;
        ctx.next()?;
        'l1: loop {
            if !(true) {
                break 'l1;
            }
            'b1: {
                l_yagu100 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(9)])?;
                l_yagu10 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(9)])?;
                l_yagu1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(9)])?;
                if ((!l_yagu100.clone().loosely_equals(&l_yagu10.clone()) && !l_yagu100.clone().loosely_equals(&l_yagu1.clone()))
                    && !l_yagu10.clone().loosely_equals(&l_yagu1.clone()))
                {
                    break 'l1;
                }
            }
        }
        'l2: loop {
            if !(true) {
                break 'l2;
            }
            'b2: {
                'l3: loop {
                    if !(true) {
                        break 'l3;
                    }
                    'b3: {
                        let (input, status) = runtime::input_number(ctx, None, None)?;
                        l_input = input;
                        if (l_input.clone().number()? < 100 || l_input.clone().number()? > 999) {
                            ctx.lines_as(
                                "Screen",
                                args![
                                    "Number input",
                                    "requirement has",
                                    "not been fulfilled.",
                                    "Please enter a",
                                    "3 digit number."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        l_input100 = (l_input.clone().try_div(Val::from(100))?);
                        l_input10 = ((l_input.clone().try_rem(Val::from(100))?).try_div(Val::from(10))?);
                        if ((l_input100.clone().number()? > 0 && l_input10.clone().number()? > 0)
                            && (l_input.clone().try_rem(Val::from(10))?).number()? > 0)
                        {
                            if ((!l_input100.clone().loosely_equals(&l_input10.clone())
                                && !l_input100.clone().loosely_equals(&(l_input.clone().try_rem(Val::from(10))?)))
                                && !l_input10.clone().loosely_equals(&(l_input.clone().try_rem(Val::from(10))?)))
                            {
                                break 'l3;
                            }
                            ctx.lines_as(
                                "Screen",
                                args![
                                    "Violation of number",
                                    "input parameter. The",
                                    "number 0 has been input,",
                                    "or a number has been",
                                    "input more than once."
                                ],
                            )?;
                            ctx.next()?;
                        }
                    }
                }
                l_retry = (l_retry.clone() + Val::from(1));
                ctx.lines_as(
                    "Screen",
                    args![
                        "You have input...",
                        ((((((Val::from("^0000ff") + l_input100.clone()) + Val::from("^000000, ^0000ff")) + l_input10.clone())
                            + Val::from("^000000, ^0000ff"))
                            + (l_input.clone().try_rem(Val::from(10))?))
                            + Val::from("^000000")),
                        " ",
                        "Calculating Results...",
                        "Please wait a moment..."
                    ],
                )?;
                ctx.next()?;
                l_strike = Val::from(0);
                l_ball = Val::from(0);
                if l_yagu100.clone().loosely_equals(&l_input100.clone()) {
                    l_strike = (l_strike.clone() + Val::from(1));
                }
                if l_yagu10.clone().loosely_equals(&l_input10.clone()) {
                    l_strike = (l_strike.clone() + Val::from(1));
                }
                if l_yagu1.clone().loosely_equals(&(l_input.clone().try_rem(Val::from(10))?)) {
                    l_strike = (l_strike.clone() + Val::from(1));
                }
                if (l_yagu100.clone().loosely_equals(&l_input10.clone())
                    || l_yagu100.clone().loosely_equals(&(l_input.clone().try_rem(Val::from(10))?)))
                {
                    l_ball = (l_ball.clone() + Val::from(1));
                }
                if (l_yagu10.clone().loosely_equals(&l_input100.clone())
                    || l_yagu10.clone().loosely_equals(&(l_input.clone().try_rem(Val::from(10))?)))
                {
                    l_ball = (l_ball.clone() + Val::from(1));
                }
                if (l_yagu1.clone().loosely_equals(&l_input100.clone()) || l_yagu1.clone().loosely_equals(&l_input10.clone())) {
                    l_ball = (l_ball.clone() + Val::from(1));
                }
                if l_strike.clone() == 3 {
                    ctx.lines_as("Screen", args!["Input number accepted.", "Access authorized."])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFAfter the screen displays",
                        "the access authorization",
                        "notice, magical power condenses",
                        "and appears on the screen's",
                        "surface. An object forms...^000000"
                    ])?;
                    ctx.next()?;
                    ctx.mes("^4d4dffThe powerful Yellow Key appears.^000000")?;
                    ctx.call(Function::GetItem, vec![Val::from(7422), Val::from(1)])?;
                    if ctx.var("thana_tower").get()? != 5 {
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines(args![
                        "^4d4dffA fierce feeling passes through your head.",
                        "You seen an illusion of light, like when you acquired the first key.^000000"
                    ])?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Ignore it.:Concentrate on it.")])? {
                        1 => {
                            ctx.mes("You decide to ignore it.")?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines(args!["You focus on the light.", "Some letters begin to appear..."])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^b22222Have you found the second key?",
                                "I wish to tell you where the seal is, but I won't unveil it so easily.",
                                "Here's a hint. Go to the eagle on the 5th floor."
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^b22222The reason I came here is...",
                                "to find someone.",
                                "We human beings can't understand this..."
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^b22222When I found her in Juperos, I couldn't relax.",
                                "The giant men staying with her in deep side of cave",
                                "made me rush through my investigation..."
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^b22222After confirming her existence, I became blindly obsessed with finding traces of her.",
                                "'Her', you ask? She is shaped like a woman..."
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^b22222I followed her traces without knowing that she existed 10 years earlier.",
                                "Then I came upon this place."
                            ])?;
                            ctx.next()?;
                            ctx.mes("^b22222This tower set up for...^000000")?;
                            ctx.next()?;
                            ctx.mes("It suddenly shakes, then disappears.")?;
                            ctx.var("thana_tower").set(Val::from(6))?;
                            ctx.call(Function::ChangeQuest, vec![Val::from(7049), Val::from(7050)])?;
                            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_COMBOATTACK1")?])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else {
                    ctx.lines_as("Screen", args!["*Beeeeep*", "Unauthorized", "numerical sequence."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Screen",
                        args![
                            "Correct number",
                            "in correct place",
                            ((Val::from("in sequence total: ^FF0000") + l_strike.clone()) + Val::from("^000000")),
                            " ",
                            ((Val::from("Correct number total: ^FF0000") + l_ball.clone()) + Val::from("^000000"))
                        ],
                    )?;
                    ctx.next()?;
                    if l_retry.clone().number()? > 4 {
                        ctx.lines_as(
                            "Screen",
                            args![
                                "Correct number",
                                "authorization",
                                "sequence was...",
                                ((((((Val::from("^ff0000") + l_yagu100.clone()) + Val::from("^000000, ^ff0000")) + l_yagu10.clone())
                                    + Val::from("^000000, ^ff0000"))
                                    + l_yagu1.clone())
                                    + Val::from("^000000")),
                                "Authorization number",
                                "will change upon retry."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
            }
        }
    }
    ctx.mes("You've acquired everything you need from this rune device.")?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn rune_device_tt2(ctx: &Ctx) -> Script {
    rune_device_tt2_body(ctx, Vec::new()).map(|_| ())
}

fn rune_device_tt2_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.call(Function::CountItem, vec![Val::from(7422)])? == 0 && ctx.call(Function::CountItem, vec![Val::from(7427)])? == 0) {
        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_LEVEL99_4")?])?;
    }
    return Err(Stop::End);
}

pub fn rune_device_tt2_ontouch(ctx: &Ctx) -> Script {
    rune_device_tt2_ontouch_body(ctx, Vec::new()).map(|_| ())
}

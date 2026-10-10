use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn poring_war_recruiter_wop_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Poring",
        args![
            "!!!!!",
            "Whoa-! Humans, ring~!!",
            "Gotta hide, hide, right~!",
            "They're tempting us with Jellopy! Don't be fooled!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Poring",
        args![
            "Hwak!!",
            "Ring, Ring~ What's wrong with you people..?",
            "Hey.. Hey, there. Hu.. Humans...",
            "Poring.."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Poring",
        args![
            "I.. I've got some interesting work for ya.. Would you be interested?",
            "We.. we porings need lots and lots of brave human worriers, ring~."
        ],
    )?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("Alright, I'm with you!:What's that?:Ignore")])? {
        1 => {
            if ctx.var("Zeny").get()?.number()? > 499 {
                ctx.lines_as(
                    "Poring",
                    args!["Oh, and there's an entrance fee of 500 zeny, ring.", "Have a good time, ring."],
                )?;
                ctx.call(
                    Function::DelItem,
                    vec![Val::from(7773), ctx.call(Function::CountItem, vec![Val::from(7773)])?],
                )?;
                ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(500))?))?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("poring_w01"), Val::from(112), Val::from(138)])?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Poring",
                    args![
                        "Oh, and there's an entrance fee of 500 zeny, ring.",
                        "...........",
                        "Hey, that's life, ring. We need zeny too you know~!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
        2 => {
            ctx.lines_as(
                "Poring",
                args![
                    "That's.. because there's been a.. slight confliction in our.. Ring Society..",
                    "So we've got to.............have a battle to settle this problem..",
                    "You'll see when you get there!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        3 => {
            ctx.lines_as("Poring", args!["Huhhhh! Hu.. Humans are so cold and cruel!!!"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn poring_war_recruiter_wop(ctx: &Ctx) -> Script {
    poring_war_recruiter_wop_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum PoringVendingMachineWStep {
    Start,
    SPoringVending,
}

fn poring_vending_machine_w_run(ctx: &Ctx, mut step: PoringVendingMachineWStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_random_figure = Val::from(0);
    'machine: loop {
        match step {
            PoringVendingMachineWStep::Start => {
                if ctx.call(Function::CheckWeight, vec![Val::from(714), Val::from(3)])? == 0 {
                    ctx.mes("- You are carrying too much items in order to use the Vending Machine. -")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.mes("It's a vending machine. You can use Poring Coints to purchase.")?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("Purchase.:Read the descriptions of goods.")],
                )?) == 1
                {
                    ctx.lines(args![
                        "You need Poring Coins to purchase items.",
                        "You cannot use any zeny.",
                        "Item name - Price Poring Coin(P.Co)"
                    ])?;
                    ctx.next()?;
                    'b1: {
                        let subject1 = Val::from(runtime::select_values(
                            ctx,
                            &[Val::from(
                                "Marvelous Medal - 4 P.Co:Union of Tribe - 20 P.Co:Poring Box - 30 P.Co:Next",
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
                            poring_vending_machine_w_run(
                                ctx,
                                PoringVendingMachineWStep::SPoringVending,
                                vec![Val::from(7515), Val::from(4)],
                            )?;
                        }
                        if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                            matched1 = true;
                        }
                        if matched1 {
                            poring_vending_machine_w_run(
                                ctx,
                                PoringVendingMachineWStep::SPoringVending,
                                vec![Val::from(658), Val::from(20)],
                            )?;
                        }
                        if !matched1 && subject1.loosely_equals(&Val::from(3)) {
                            matched1 = true;
                        }
                        if matched1 {
                            poring_vending_machine_w_run(
                                ctx,
                                PoringVendingMachineWStep::SPoringVending,
                                vec![Val::from(12109), Val::from(30)],
                            )?;
                        }
                        if !matched1 && subject1.loosely_equals(&Val::from(4)) {
                            matched1 = true;
                        }
                        if matched1 {
                            ctx.lines(args!["This is a special item.", "Item name - Poring Coin(P.Co)"])?;
                            ctx.next()?;
                            'b2: {
                                let subject2 = Val::from(runtime::select_values(
                                    ctx,
                                    &[Val::from(
                                        "Wild Rose - 15 P.Co:Doppelganger - 20 P.Co:Egnigem Cenia - 20 P.Co:Collection Item",
                                    )],
                                )?);
                                let mut matched2 = false;
                                let no_case2 = !subject2.loosely_equals(&Val::from(1))
                                    && !subject2.loosely_equals(&Val::from(2))
                                    && !subject2.loosely_equals(&Val::from(3))
                                    && !subject2.loosely_equals(&Val::from(4));
                                if !matched2 && subject2.loosely_equals(&Val::from(1)) {
                                    matched2 = true;
                                }
                                if matched2 {
                                    poring_vending_machine_w_run(
                                        ctx,
                                        PoringVendingMachineWStep::SPoringVending,
                                        vec![Val::from(12300), Val::from(15)],
                                    )?;
                                }
                                if !matched2 && subject2.loosely_equals(&Val::from(2)) {
                                    matched2 = true;
                                }
                                if matched2 {
                                    poring_vending_machine_w_run(
                                        ctx,
                                        PoringVendingMachineWStep::SPoringVending,
                                        vec![Val::from(12301), Val::from(20)],
                                    )?;
                                }
                                if !matched2 && subject2.loosely_equals(&Val::from(3)) {
                                    matched2 = true;
                                }
                                if matched2 {
                                    poring_vending_machine_w_run(
                                        ctx,
                                        PoringVendingMachineWStep::SPoringVending,
                                        vec![Val::from(12302), Val::from(20)],
                                    )?;
                                }
                                if !matched2 && subject2.loosely_equals(&Val::from(4)) {
                                    matched2 = true;
                                }
                                if matched2 {
                                    ctx.lines(args![
                                        "Figures of 1st Job Class Characters including Novice are finally on sale!",
                                        "Figures except for Novice are all ^4d4dffCharacter bound items^000000.",
                                        "Please be aware before you make a purchase~",
                                        "Item name - Poring Coin(P.Co)"
                                    ])?;
                                    ctx.next()?;
                                    'b3: {
                                        let subject3 = Val::from(runtime::select_values(
                                            ctx,
                                            &[Val::from(
                                                "Novice Figure - 50 P.Co:Swordman Figure - 100 P.Co:Thief Figure - 100 P.Co:Merchant Figure - 100 P.Co:Acolyte Figure - 100 P.Co:Mage Figure - 100 P.Co:Archer Figure - 100 P.Co:Random Draw - 50 P.Co:Cancel",
                                            )],
                                        )?);
                                        let mut matched3 = false;
                                        let no_case3 = !subject3.loosely_equals(&Val::from(1))
                                            && !subject3.loosely_equals(&Val::from(2))
                                            && !subject3.loosely_equals(&Val::from(3))
                                            && !subject3.loosely_equals(&Val::from(4))
                                            && !subject3.loosely_equals(&Val::from(5))
                                            && !subject3.loosely_equals(&Val::from(6))
                                            && !subject3.loosely_equals(&Val::from(7))
                                            && !subject3.loosely_equals(&Val::from(8))
                                            && !subject3.loosely_equals(&Val::from(9));
                                        if !matched3 && subject3.loosely_equals(&Val::from(1)) {
                                            matched3 = true;
                                        }
                                        if matched3 {
                                            poring_vending_machine_w_run(
                                                ctx,
                                                PoringVendingMachineWStep::SPoringVending,
                                                vec![Val::from(2765), Val::from(50)],
                                            )?;
                                        }
                                        if !matched3 && subject3.loosely_equals(&Val::from(2)) {
                                            matched3 = true;
                                        }
                                        if matched3 {
                                            poring_vending_machine_w_run(
                                                ctx,
                                                PoringVendingMachineWStep::SPoringVending,
                                                vec![Val::from(2766), Val::from(100)],
                                            )?;
                                        }
                                        if !matched3 && subject3.loosely_equals(&Val::from(3)) {
                                            matched3 = true;
                                        }
                                        if matched3 {
                                            poring_vending_machine_w_run(
                                                ctx,
                                                PoringVendingMachineWStep::SPoringVending,
                                                vec![Val::from(2770), Val::from(100)],
                                            )?;
                                        }
                                        if !matched3 && subject3.loosely_equals(&Val::from(4)) {
                                            matched3 = true;
                                        }
                                        if matched3 {
                                            poring_vending_machine_w_run(
                                                ctx,
                                                PoringVendingMachineWStep::SPoringVending,
                                                vec![Val::from(2771), Val::from(100)],
                                            )?;
                                        }
                                        if !matched3 && subject3.loosely_equals(&Val::from(5)) {
                                            matched3 = true;
                                        }
                                        if matched3 {
                                            poring_vending_machine_w_run(
                                                ctx,
                                                PoringVendingMachineWStep::SPoringVending,
                                                vec![Val::from(2767), Val::from(100)],
                                            )?;
                                        }
                                        if !matched3 && subject3.loosely_equals(&Val::from(6)) {
                                            matched3 = true;
                                        }
                                        if matched3 {
                                            poring_vending_machine_w_run(
                                                ctx,
                                                PoringVendingMachineWStep::SPoringVending,
                                                vec![Val::from(2768), Val::from(100)],
                                            )?;
                                        }
                                        if !matched3 && subject3.loosely_equals(&Val::from(7)) {
                                            matched3 = true;
                                        }
                                        if matched3 {
                                            poring_vending_machine_w_run(
                                                ctx,
                                                PoringVendingMachineWStep::SPoringVending,
                                                vec![Val::from(2769), Val::from(100)],
                                            )?;
                                        }
                                        if !matched3 && subject3.loosely_equals(&Val::from(8)) {
                                            matched3 = true;
                                        }
                                        if matched3 {
                                            ctx.lines(args![
                                                "You have chosen Random Draw.",
                                                "1 of 7 diffeent kinds of figures will be selected."
                                            ])?;
                                            ctx.next()?;
                                            if Val::from(runtime::select_values(ctx, &[Val::from("Draw:Cancel")])?) == 1 {
                                                if ctx.call(Function::CountItem, vec![Val::from(7539)])?.number()? >= 50 {
                                                    ctx.lines(args![
                                                        "Insert the Poring coin and pull the lever.",
                                                        "Click~ The item came out of the mouth of the Poring with a rumbling sound.",
                                                        "What could it be?"
                                                    ])?;
                                                    ctx.next()?;
                                                    let subject4 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(17)])?;
                                                    if subject4 == 5 {
                                                        l_random_figure = Val::from(2766);
                                                    } else if subject4 == 6 {
                                                        l_random_figure = Val::from(2767);
                                                    } else if subject4 == 8 {
                                                        l_random_figure = Val::from(2770);
                                                    } else if subject4 == 11 {
                                                        l_random_figure = Val::from(2771);
                                                    } else if subject4 == 13 {
                                                        l_random_figure = Val::from(2769);
                                                    } else if subject4 == 14 {
                                                        l_random_figure = Val::from(2768);
                                                    } else {
                                                        l_random_figure = Val::from(2765);
                                                    }
                                                    ctx.lines(args![
                                                        ((Val::from("A nice ")
                                                            + ctx.call(Function::GetItemName, vec![l_random_figure.clone()])?)
                                                            + Val::from("."))
                                                    ])?;
                                                    ctx.call(Function::DelItem, vec![Val::from(7539), Val::from(50)])?;
                                                    ctx.call(Function::GetItem, vec![l_random_figure.clone(), Val::from(1)])?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                } else {
                                                    ctx.mes("Not enough coins.")?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                }
                                            }
                                            ctx.lines_as(
                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                args!["... Maybe next time..."],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        if !matched3 && subject3.loosely_equals(&Val::from(9)) {
                                            matched3 = true;
                                        }
                                        if matched3 {
                                            ctx.lines_as(
                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                args!["... Maybe next time..."],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                ctx.lines(args!["Selling Item List", "====================", "[Marvelous Medal]", " : A medal made of special metal only produced in Hugel.", " ", "[Union of Tribe]", " : A statue with the image of a strong union of Tribes. People believe watching this statue actually helps strengthen the relationships between Tribes.", " ", "[Poring Box]", " : A box wrapped with Poring patterned wrapping paper. Something's inside.", " ", "[Wild Rose]", " : Your friend Wild Rose will come and help you.", " ", "[Mr. Doppel]", " : A young nobe, Doppelganger will come and help you.", " ", "[Egnigem Cenia]", " : A beautiful girl, Egnigem Cenia from Somatology Laboratory, is going to come and help you.", " ", "[Novice Figure]", " : A fine figure of a Novice. Can be equiped as an '^4d4dffaccessory^000000'.", " HP + 70, extra effect of HP + 30 when equipped by a Novices.", " ", "[Swordman Figure]", " : A nice figure of a Swordman. Can be equipped as an '^4d4dffaccessory^000000'.", " VIT + 1, extra effect of DEF + 2 when equipped by Swordman classes.", " ", "[Merchant Figure]", " : A fine figure of a Merchant. Can be equipped as an '^4d4dffaccessory^000000'.", " STR + 1, extra effect of CRI + 5 when equipped by Merchant classes.", " ", "[Thief Figure]", " : A fine Figure of a Thief. Can be equipped as an '^4d4dffaccessory^000000'.", " AGI + 1, extra effectASPD + 3% when equipped by Thief classes.", " ", "[Mage Figure]", " : A fine figure of a Mage. Can be equipped as an '^4d4dffaccessory^000000'.", " INT + 1, an extra SP Recovery increase by 5% when equipped by Mage classes.", " ", "[Acolyte Figure]", " : A fine figure of an Acolyte. Can be equipped as an '^4d4dffaccessory^000000'.", " INT + 1, extra effct of SP + 50 when equipped by Acolyte classes.", " ", "[Archer Figure]", " : A fine figure of an Archer. Can be equipped as an '^4d4dffaccessory^000000'.", " DEX + 1, extra effct of ATK + 10 when equipped by Archer classes."])?;
                ctx.close_window()?;
                return Err(Stop::End);
                return Err(Stop::End);
            }
            PoringVendingMachineWStep::SPoringVending => {
                if runtime::op(
                    &ctx.call(Function::CountItem, vec![Val::from(7539)])?,
                    ">=",
                    &runtime::arg(&args, 1, Val::from(0)),
                )?
                .is_true()
                {
                    ctx.mes("Click~ The item came out of the mouth of the Poring with a rumbling sound.")?;
                    ctx.call(Function::DelItem, vec![Val::from(7539), runtime::arg(&args, 1, Val::from(0))])?;
                    ctx.call(Function::GetItem, vec![runtime::arg(&args, 0, Val::from(0)), Val::from(1)])?;
                } else {
                    ctx.mes("Not enough coins.")?;
                }
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn poring_vending_machine_w(ctx: &Ctx) -> Script {
    poring_vending_machine_w_run(ctx, PoringVendingMachineWStep::Start, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum SweetDeviWopStep {
    Start,
    OnPCLogoutEvent,
    OnPCDieEvent,
    OnPCKillEvent,
    OnInit,
}

fn sweet_devi_wop_run(ctx: &Ctx, mut step: SweetDeviWopStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_map_s = Val::from("");
    let mut l_x = Val::from(0);
    let mut l_y = Val::from(0);
    'machine: loop {
        match step {
            SweetDeviWopStep::Start => {
                if ((ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 2000
                    || ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0)
                {
                    ctx.lines(args![
                        "- Wait a minute !! -",
                        "- Currently you're carrying -",
                        "- too many items with you. -",
                        "- Please try again -",
                        "- after you lose some weight. -"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Deviruchi",
                    args![
                        "Oh, Another Human Warrior!",
                        "How come so many humans want to join our Poring War these days?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Deviruchi",
                    args![
                        "Well, whatever, as long as I make money out of it~",
                        "Ok, Warriors-! Hahaha How badly have I wanted to shout it out~!!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Deviruchi",
                    args!["You, brave warrior, are you ready to join the holy battle of Angeling and Deviling?"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Deviruchi",
                    args![
                        "Hehehee, I feel kinda shy now.",
                        "Anyway, human. Do you want to join our Poring War?"
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(
                    ctx,
                    &[Val::from(
                        "Am I qualified to join?:How do I join the war?:Cancel:Let me out of here, please!",
                    )],
                )? {
                    1 => {
                        ctx.lines_as(
                            "Deviruchi",
                            args![
                                "No racial discrimination! Anyone can join if they're willing to fight.",
                                "But the weird thing is that you humans who used to be our greatest enemies are now our participants."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Deviruchi",
                            args![
                                "If you really want to fight in the war or whatever.. the Team recruiter's right there.",
                                "Also, it's totally up to you which team you want to fight for~"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as(
                            "Deviruchi",
                            args![
                                "It's simple. You see that Team recruiting room?",
                                "Each team is composed of 5 people. As soon as 5 members are collected, the battle starts.",
                                "You win if you kill the other team's Porings."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Deviruchi",
                            args!["This also means that the battle needs the total of 10 members."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Deviruchi", args!["When all 10 members are collected, those participants get to choose a team. Each team then should have 5 members who are in the SAME party."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Deviruchi",
                            args![
                                "So, finally, half of the members joins the Angeling Team",
                                "and the other half joins the Deviling Team."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Deviruchi", args!["If you're a member of the Angeling Team, your goal is to kill the Devilings at the other team's base and vice versa."])?;
                        ctx.next()?;
                        ctx.lines_as("Deviruchi", args!["Each team should try killing both Porings in the other team's base. After killing one Poring, you have a limited time to kill the other Poring. Otherwise, the one you killed will come back alive."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Deviruchi",
                            args![
                                "It's sort of like a capture the flag game but with Porings instead. Understood?",
                                "And you need to make sure you know who's in which party."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    3 => {
                        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["hmm, I see."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    4 => {
                        ctx.lines_as(
                            "Deviruchi",
                            args!["Oh, Alright. I can help.", "I'll send you back to your savepoint."],
                        )?;
                        ctx.close_window()?;
                        if ctx.var("wop_savemap$").get()? != "" {
                            ctx.call(
                                Function::SavePoint,
                                vec![
                                    ctx.var("wop_savemap$").get()?,
                                    ctx.var("wop_savemap_x").get()?,
                                    ctx.var("wop_savemap_y").get()?,
                                    Val::from(1),
                                    Val::from(1),
                                ],
                            )?;
                            ctx.var("wop_savemap$").set(Val::from(""))?;
                            ctx.var("wop_savemap_x").set(Val::from(0))?;
                            ctx.var("wop_savemap_y").set(Val::from(0))?;
                        }
                        ctx.call(
                            Function::Warp,
                            vec![
                                ctx.call(Function::GetSavePoint, vec![Val::from(0)])?,
                                ctx.call(Function::GetSavePoint, vec![Val::from(1)])?,
                                ctx.call(Function::GetSavePoint, vec![Val::from(2)])?,
                            ],
                        )?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
                step = SweetDeviWopStep::OnPCLogoutEvent;
                continue 'machine;
            }
            SweetDeviWopStep::OnPCLogoutEvent => {
                let position = ctx
                    .call(Function::GetMapXy, vec![Val::from(0)])?
                    .into_array()
                    .ok_or_else(|| Stop::Error("Invalid position".into()))?;
                l_map_s = position[0].clone();
                l_x = position[1].clone();
                l_y = position[2].clone();
                if l_map_s.clone() == "poring_w02" {
                    if ctx.var("wop_savemap$").get()? != "" {
                        ctx.call(
                            Function::SavePoint,
                            vec![
                                ctx.var("wop_savemap$").get()?,
                                ctx.var("wop_savemap_x").get()?,
                                ctx.var("wop_savemap_y").get()?,
                                Val::from(1),
                                Val::from(1),
                            ],
                        )?;
                        ctx.var("wop_savemap$").set(Val::from(""))?;
                        ctx.var("wop_savemap_x").set(Val::from(0))?;
                        ctx.var("wop_savemap_y").set(Val::from(0))?;
                    }
                    ctx.call(
                        Function::DelItem,
                        vec![Val::from(7773), ctx.call(Function::CountItem, vec![Val::from(7773)])?],
                    )?;
                    ctx.var("wop_team").set(Val::from(0))?;
                }
                return Err(Stop::End);
            }
            SweetDeviWopStep::OnPCDieEvent => {
                let position = ctx
                    .call(Function::GetMapXy, vec![Val::from(0)])?
                    .into_array()
                    .ok_or_else(|| Stop::Error("Invalid position".into()))?;
                l_map_s = position[0].clone();
                l_x = position[1].clone();
                l_y = position[2].clone();
                if (l_map_s.clone() == "poring_w02" && ctx.var("wop_team").get()?.is_true()) {
                    if (ctx.call(Function::GetSavePoint, vec![Val::from(0)])? != "poring_w02" && ctx.var("wop_savemap$").get()? == "") {
                        ctx.var("wop_savemap$").set(ctx.call(Function::GetSavePoint, vec![Val::from(0)])?)?;
                        ctx.var("wop_savemap_x")
                            .set(ctx.call(Function::GetSavePoint, vec![Val::from(1)])?)?;
                        ctx.var("wop_savemap_y")
                            .set(ctx.call(Function::GetSavePoint, vec![Val::from(2)])?)?;
                    }
                    if ctx.var("wop_team").get()? == 1 {
                        ctx.call(
                            Function::SavePoint,
                            vec![
                                Val::from("poring_w02"),
                                ctx.call(Function::Rand, vec![Val::from(44), Val::from(51)])?,
                                ctx.call(Function::Rand, vec![Val::from(76), Val::from(87)])?,
                            ],
                        )?;
                    }
                    if ctx.var("wop_team").get()? == 2 {
                        ctx.call(
                            Function::SavePoint,
                            vec![
                                Val::from("poring_w02"),
                                ctx.call(Function::Rand, vec![Val::from(146), Val::from(153)])?,
                                ctx.call(Function::Rand, vec![Val::from(76), Val::from(87)])?,
                            ],
                        )?;
                    }
                }
                return Err(Stop::End);
            }
            SweetDeviWopStep::OnPCKillEvent => {
                let position = ctx
                    .call(Function::GetMapXy, vec![Val::from(0)])?
                    .into_array()
                    .ok_or_else(|| Stop::Error("Invalid position".into()))?;
                l_map_s = position[0].clone();
                l_x = position[1].clone();
                l_y = position[2].clone();
                if (l_map_s.clone() == "poring_w02" && ctx.var("wop_team").get()?.is_true()) {
                    ctx.call(
                        Function::GetNamedItem,
                        vec![
                            Val::from(7773),
                            ctx.call(Function::ConvertPcInfo, vec![ctx.var("@killedrid").get()?, Val::from(0)])?,
                        ],
                    )?;
                }
                return Err(Stop::End);
            }
            SweetDeviWopStep::OnInit => {
                ctx.call(
                    Function::RemoveMapFlag,
                    vec![Val::from("poring_w02"), ctx.constant("MF_PARTYLOCK")?],
                )?;
                ctx.call(Function::RemoveMapFlag, vec![Val::from("poring_w02"), ctx.constant("MF_PVP")?])?;
                ctx.call(
                    Function::RemoveMapFlag,
                    vec![Val::from("poring_w02"), ctx.constant("MF_PVP_NOGUILD")?],
                )?;
                ctx.call(
                    Function::RemoveMapFlag,
                    vec![Val::from("poring_w02"), ctx.constant("MF_PVP_NOCALCRANK")?],
                )?;
                ctx.var("$@wop_teamcount").set(Val::from(0))?;
                ctx.var("$@wop_deadcount_a").set(Val::from(0))?;
                ctx.var("$@wop_deadcount_d").set(Val::from(0))?;
                ctx.var("$@wop_team_a").set(Val::from(0))?;
                ctx.var("$@wop_team_d").set(Val::from(0))?;
                ctx.var("$@wop_doorcount_a").set(Val::from(0))?;
                ctx.var("$@wop_doorcount_d").set(Val::from(0))?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn sweet_devi_wop(ctx: &Ctx) -> Script {
    sweet_devi_wop_run(ctx, SweetDeviWopStep::Start, Vec::new()).map(|_| ())
}

pub fn sweet_devi_wop_onpclogoutevent(ctx: &Ctx) -> Script {
    sweet_devi_wop_run(ctx, SweetDeviWopStep::OnPCLogoutEvent, Vec::new()).map(|_| ())
}

pub fn sweet_devi_wop_onpcdieevent(ctx: &Ctx) -> Script {
    sweet_devi_wop_run(ctx, SweetDeviWopStep::OnPCDieEvent, Vec::new()).map(|_| ())
}

pub fn sweet_devi_wop_onpckillevent(ctx: &Ctx) -> Script {
    sweet_devi_wop_run(ctx, SweetDeviWopStep::OnPCKillEvent, Vec::new()).map(|_| ())
}

pub fn sweet_devi_wop_oninit(ctx: &Ctx) -> Script {
    sweet_devi_wop_run(ctx, SweetDeviWopStep::OnInit, Vec::new()).map(|_| ())
}

fn poring_wop_door_all_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn poring_wop_door_all(ctx: &Ctx) -> Script {
    poring_wop_door_all_body(ctx, Vec::new()).map(|_| ())
}

fn poring_wop_door_all_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::WaitingRoom,
        vec![
            Val::from("[Recruiting 10 Battle Participants]"),
            Val::from(11),
            Val::from("Poring#wop_door_all::OnStartArena"),
            Val::from(10),
            Val::from(500),
            Val::from(9),
        ],
    )?;
    ctx.call(Function::EnableWaitingRoomEvent, vec![])?;
    return Err(Stop::End);
}

pub fn poring_wop_door_all_oninit(ctx: &Ctx) -> Script {
    poring_wop_door_all_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn poring_wop_door_all_onstartarena_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::WarpWaitingPc,
        vec![Val::from("poring_w01"), Val::from(101), Val::from(70)],
    )?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_warp_rtry::OnReady")])?;
    ctx.call(Function::DisableWaitingRoomEvent, vec![])?;
    return Err(Stop::End);
}

pub fn poring_wop_door_all_onstartarena(ctx: &Ctx) -> Script {
    poring_wop_door_all_onstartarena_body(ctx, Vec::new()).map(|_| ())
}

fn poring_wop_door_all_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableWaitingRoomEvent, vec![])?;
    return Err(Stop::End);
}

pub fn poring_wop_door_all_onenable(ctx: &Ctx) -> Script {
    poring_wop_door_all_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn poring_wop_door_all_ondisable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableWaitingRoomEvent, vec![])?;
    return Err(Stop::End);
}

pub fn poring_wop_door_all_ondisable(ctx: &Ctx) -> Script {
    poring_wop_door_all_ondisable_body(ctx, Vec::new()).map(|_| ())
}

fn poring_wop_door_a_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn poring_wop_door_a(ctx: &Ctx) -> Script {
    poring_wop_door_a_body(ctx, Vec::new()).map(|_| ())
}

fn poring_wop_door_a_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::WaitingRoom,
        vec![
            Val::from("[Angeling Team Recruiter]"),
            Val::from(6),
            Val::from("Poring#wop_door_a::OnStartArena"),
            Val::from(5),
            Val::from(0),
            Val::from(9),
        ],
    )?;
    ctx.call(Function::EnableWaitingRoomEvent, vec![])?;
    return Err(Stop::End);
}

pub fn poring_wop_door_a_oninit(ctx: &Ctx) -> Script {
    poring_wop_door_a_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn poring_wop_door_a_onstartarena_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::WarpWaitingPc,
        vec![Val::from("poring_w02"), Val::from(26), Val::from(175)],
    )?;
    ctx.call(Function::DisableWaitingRoomEvent, vec![])?;
    if ctx.var("$@wop_teamcount").get()? == 0 {
        ctx.var("$@wop_teamcount").set(Val::from(1))?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("Poring#wop_door_d::OnDevilingStart")])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_warp_rtry::OnStop")])?;
    } else if ctx.var("$@wop_teamcount").get()? == 1 {
        ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_master::OnStart")])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("Poring#wop_door_d::OnStop")])?;
        ctx.call(Function::StopNpcTimer, vec![])?;
    }
    return Err(Stop::End);
}

pub fn poring_wop_door_a_onstartarena(ctx: &Ctx) -> Script {
    poring_wop_door_a_onstartarena_body(ctx, Vec::new()).map(|_| ())
}

fn poring_wop_door_a_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableWaitingRoomEvent, vec![])?;
    return Err(Stop::End);
}

pub fn poring_wop_door_a_onenable(ctx: &Ctx) -> Script {
    poring_wop_door_a_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn poring_wop_door_a_ondisable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableWaitingRoomEvent, vec![])?;
    return Err(Stop::End);
}

pub fn poring_wop_door_a_ondisable(ctx: &Ctx) -> Script {
    poring_wop_door_a_ondisable_body(ctx, Vec::new()).map(|_| ())
}

fn poring_wop_door_a_onangelingstart_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::InitNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn poring_wop_door_a_onangelingstart(ctx: &Ctx) -> Script {
    poring_wop_door_a_onangelingstart_body(ctx, Vec::new()).map(|_| ())
}

fn poring_wop_door_a_onstop_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn poring_wop_door_a_onstop(ctx: &Ctx) -> Script {
    poring_wop_door_a_onstop_body(ctx, Vec::new()).map(|_| ())
}

fn poring_wop_door_a_ontimer1000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("poring_w01"),
            Val::from("The greatest battle of all time, the recruitment for the Deviling Team is over, ring!"),
            Val::from(0),
            Val::from(15761536),
        ],
    )?;
    return Err(Stop::End);
}

pub fn poring_wop_door_a_ontimer1000(ctx: &Ctx) -> Script {
    poring_wop_door_a_ontimer1000_body(ctx, Vec::new()).map(|_| ())
}

fn poring_wop_door_a_ontimer4000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("poring_w01"),
            Val::from("Join the proud Angeling Team with angel wings!!"),
            Val::from(0),
            Val::from(15761536),
        ],
    )?;
    return Err(Stop::End);
}

pub fn poring_wop_door_a_ontimer4000(ctx: &Ctx) -> Script {
    poring_wop_door_a_ontimer4000_body(ctx, Vec::new()).map(|_| ())
}

fn poring_wop_door_a_ontimer8000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("poring_w01"),
            Val::from("You got one minute to join the Angeling Team. The battle will be cancelled in 1 minute if not ready!!"),
            Val::from(0),
            Val::from(15761536),
        ],
    )?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_master::OnAngelingWarn")])?;
    return Err(Stop::End);
}

pub fn poring_wop_door_a_ontimer8000(ctx: &Ctx) -> Script {
    poring_wop_door_a_ontimer8000_body(ctx, Vec::new()).map(|_| ())
}

fn poring_wop_door_a_ontimer13000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("poring_w01"),
            Val::from("This is the time to join the great Angeling Team, ring!"),
            Val::from(0),
            Val::from(15761536),
        ],
    )?;
    return Err(Stop::End);
}

pub fn poring_wop_door_a_ontimer13000(ctx: &Ctx) -> Script {
    poring_wop_door_a_ontimer13000_body(ctx, Vec::new()).map(|_| ())
}

fn poring_wop_door_a_ontimer20000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("poring_w01"),
            Val::from("This battle is the proud of the porings! Ring! Join the Angeling Team!!"),
            Val::from(0),
            Val::from(15761536),
        ],
    )?;
    return Err(Stop::End);
}

pub fn poring_wop_door_a_ontimer20000(ctx: &Ctx) -> Script {
    poring_wop_door_a_ontimer20000_body(ctx, Vec::new()).map(|_| ())
}

fn poring_wop_door_a_ontimer30000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("poring_w01"),
            Val::from("We don't have much time, ring. Don't let the Devilings contaminate you!"),
            Val::from(0),
            Val::from(15761536),
        ],
    )?;
    return Err(Stop::End);
}

pub fn poring_wop_door_a_ontimer30000(ctx: &Ctx) -> Script {
    poring_wop_door_a_ontimer30000_body(ctx, Vec::new()).map(|_| ())
}

fn poring_wop_door_a_ontimer40000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("poring_w01"),
            Val::from("Come! Join us!"),
            Val::from(0),
            Val::from(15761536),
        ],
    )?;
    return Err(Stop::End);
}

pub fn poring_wop_door_a_ontimer40000(ctx: &Ctx) -> Script {
    poring_wop_door_a_ontimer40000_body(ctx, Vec::new()).map(|_| ())
}

fn poring_wop_door_a_ontimer50000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("poring_w01"),
            Val::from("What a pitty! I can't believe that brave warriors are missing!"),
            Val::from(0),
            Val::from(15761536),
        ],
    )?;
    return Err(Stop::End);
}

pub fn poring_wop_door_a_ontimer50000(ctx: &Ctx) -> Script {
    poring_wop_door_a_ontimer50000_body(ctx, Vec::new()).map(|_| ())
}

fn poring_wop_door_a_ontimer55000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("poring_w01"),
            Val::from("Deviling: You can't leave us waiting for ever!! We're going to cancel the battle, ring!"),
            Val::from(0),
            Val::from(3407718),
        ],
    )?;
    return Err(Stop::End);
}

pub fn poring_wop_door_a_ontimer55000(ctx: &Ctx) -> Script {
    poring_wop_door_a_ontimer55000_body(ctx, Vec::new()).map(|_| ())
}

fn poring_wop_door_a_ontimer55100_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DoNpcEvent, vec![Val::from("Poring#wop_door_a::OnDisable")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("Poring#wop_door_d::OnDisable")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("Poring#wop_door_all::OnDisable")])?;
    ctx.var("$@wop_teamcount").set(Val::from(0))?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_master::OnAngelingEnd")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_warp_rtry::OnEnable")])?;
    return Err(Stop::End);
}

pub fn poring_wop_door_a_ontimer55100(ctx: &Ctx) -> Script {
    poring_wop_door_a_ontimer55100_body(ctx, Vec::new()).map(|_| ())
}

fn poring_wop_door_a_ontimer58000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("poring_w01"),
            Val::from("...There is nothing we can do, ring... Lets cheer for the next one, ring."),
            Val::from(0),
            Val::from(15761536),
        ],
    )?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_master::OnReset")])?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn poring_wop_door_a_ontimer58000(ctx: &Ctx) -> Script {
    poring_wop_door_a_ontimer58000_body(ctx, Vec::new()).map(|_| ())
}

fn poring_wop_door_d_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn poring_wop_door_d(ctx: &Ctx) -> Script {
    poring_wop_door_d_body(ctx, Vec::new()).map(|_| ())
}

fn poring_wop_door_d_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::WaitingRoom,
        vec![
            Val::from("[Deviling Team Recruiter]"),
            Val::from(6),
            Val::from("Poring#wop_door_d::OnStartArena"),
            Val::from(5),
            Val::from(0),
            Val::from(9),
        ],
    )?;
    ctx.call(Function::EnableWaitingRoomEvent, vec![])?;
    return Err(Stop::End);
}

pub fn poring_wop_door_d_oninit(ctx: &Ctx) -> Script {
    poring_wop_door_d_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn poring_wop_door_d_onstartarena_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::WarpWaitingPc,
        vec![Val::from("poring_w02"), Val::from(170), Val::from(175)],
    )?;
    ctx.call(Function::DisableWaitingRoomEvent, vec![])?;
    if ctx.var("$@wop_teamcount").get()? == 0 {
        ctx.var("$@wop_teamcount").set(Val::from(1))?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("Poring#wop_door_a::OnAngelingStart")])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_warp_rtry::OnStop")])?;
    } else if ctx.var("$@wop_teamcount").get()? == 1 {
        ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_master::OnStart")])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("Poring#wop_door_a::OnStop")])?;
        ctx.call(Function::StopNpcTimer, vec![])?;
    }
    return Err(Stop::End);
}

pub fn poring_wop_door_d_onstartarena(ctx: &Ctx) -> Script {
    poring_wop_door_d_onstartarena_body(ctx, Vec::new()).map(|_| ())
}

fn poring_wop_door_d_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableWaitingRoomEvent, vec![])?;
    return Err(Stop::End);
}

pub fn poring_wop_door_d_onenable(ctx: &Ctx) -> Script {
    poring_wop_door_d_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn poring_wop_door_d_ondisable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableWaitingRoomEvent, vec![])?;
    return Err(Stop::End);
}

pub fn poring_wop_door_d_ondisable(ctx: &Ctx) -> Script {
    poring_wop_door_d_ondisable_body(ctx, Vec::new()).map(|_| ())
}

fn poring_wop_door_d_ondevilingstart_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::InitNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn poring_wop_door_d_ondevilingstart(ctx: &Ctx) -> Script {
    poring_wop_door_d_ondevilingstart_body(ctx, Vec::new()).map(|_| ())
}

fn poring_wop_door_d_onstop_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn poring_wop_door_d_onstop(ctx: &Ctx) -> Script {
    poring_wop_door_d_onstop_body(ctx, Vec::new()).map(|_| ())
}

fn poring_wop_door_d_ontimer1000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("poring_w01"),
            Val::from("No more good people, the recruitment for the Angeling Team is over, ring!"),
            Val::from(0),
            Val::from(3407718),
        ],
    )?;
    return Err(Stop::End);
}

pub fn poring_wop_door_d_ontimer1000(ctx: &Ctx) -> Script {
    poring_wop_door_d_ontimer1000_body(ctx, Vec::new()).map(|_| ())
}

fn poring_wop_door_d_ontimer4000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("poring_w01"),
            Val::from("Nice members of the Deviling Team! Lets gather, ring!!"),
            Val::from(0),
            Val::from(3407718),
        ],
    )?;
    return Err(Stop::End);
}

pub fn poring_wop_door_d_ontimer4000(ctx: &Ctx) -> Script {
    poring_wop_door_d_ontimer4000_body(ctx, Vec::new()).map(|_| ())
}

fn poring_wop_door_d_ontimer8000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("poring_w01"),
            Val::from("The battle will be cancelled if the members aren't recruited in one minute!!"),
            Val::from(0),
            Val::from(3407718),
        ],
    )?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_master::OnDevilingWarn")])?;
    return Err(Stop::End);
}

pub fn poring_wop_door_d_ontimer8000(ctx: &Ctx) -> Script {
    poring_wop_door_d_ontimer8000_body(ctx, Vec::new()).map(|_| ())
}

fn poring_wop_door_d_ontimer13000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("poring_w01"),
            Val::from("This is the time to join the brave Deviling Team, ring!"),
            Val::from(0),
            Val::from(3407718),
        ],
    )?;
    return Err(Stop::End);
}

pub fn poring_wop_door_d_ontimer13000(ctx: &Ctx) -> Script {
    poring_wop_door_d_ontimer13000_body(ctx, Vec::new()).map(|_| ())
}

fn poring_wop_door_d_ontimer20000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("poring_w01"),
            Val::from("D,E,V,I,L,I,N,G! Deviling Team! Come and join us!"),
            Val::from(0),
            Val::from(3407718),
        ],
    )?;
    return Err(Stop::End);
}

pub fn poring_wop_door_d_ontimer20000(ctx: &Ctx) -> Script {
    poring_wop_door_d_ontimer20000_body(ctx, Vec::new()).map(|_| ())
}

fn poring_wop_door_d_ontimer30000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("poring_w01"),
            Val::from("There ain't much time left, ring! If you wish to became a member of Deviling Team, Come and Join!"),
            Val::from(0),
            Val::from(3407718),
        ],
    )?;
    return Err(Stop::End);
}

pub fn poring_wop_door_d_ontimer30000(ctx: &Ctx) -> Script {
    poring_wop_door_d_ontimer30000_body(ctx, Vec::new()).map(|_| ())
}

fn poring_wop_door_d_ontimer40000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("poring_w01"),
            Val::from("D,E,V,I,L,I,N,G! Deviling Team! Come and Join us!!"),
            Val::from(0),
            Val::from(3407718),
        ],
    )?;
    return Err(Stop::End);
}

pub fn poring_wop_door_d_ontimer40000(ctx: &Ctx) -> Script {
    poring_wop_door_d_ontimer40000_body(ctx, Vec::new()).map(|_| ())
}

fn poring_wop_door_d_ontimer50000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("poring_w01"),
            Val::from("What a pitty! I can't believe there aren't enough players!"),
            Val::from(0),
            Val::from(3407718),
        ],
    )?;
    return Err(Stop::End);
}

pub fn poring_wop_door_d_ontimer50000(ctx: &Ctx) -> Script {
    poring_wop_door_d_ontimer50000_body(ctx, Vec::new()).map(|_| ())
}

fn poring_wop_door_d_ontimer55000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("poring_w01"),
            Val::from("Angeling: We got no time to wait, stupid Deviling! The battle has been cancelled, ring!"),
            Val::from(0),
            Val::from(15761536),
        ],
    )?;
    return Err(Stop::End);
}

pub fn poring_wop_door_d_ontimer55000(ctx: &Ctx) -> Script {
    poring_wop_door_d_ontimer55000_body(ctx, Vec::new()).map(|_| ())
}

fn poring_wop_door_d_ontimer55100_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DoNpcEvent, vec![Val::from("Poring#wop_door_a::OnDisable")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("Poring#wop_door_d::OnDisable")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("Poring#wop_door_all::OnDisable")])?;
    ctx.var("$@wop_teamcount").set(Val::from(0))?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_master::OnDevilingEnd")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_warp_rtry::OnEnable")])?;
    return Err(Stop::End);
}

pub fn poring_wop_door_d_ontimer55100(ctx: &Ctx) -> Script {
    poring_wop_door_d_ontimer55100_body(ctx, Vec::new()).map(|_| ())
}

fn poring_wop_door_d_ontimer58000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("poring_w01"),
            Val::from("...Ughhhhhh... Tell me that isn't happening, ring! Right, Be ready for the next one, ring!!"),
            Val::from(0),
            Val::from(3407718),
        ],
    )?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_master::OnReset")])?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn poring_wop_door_d_ontimer58000(ctx: &Ctx) -> Script {
    poring_wop_door_d_ontimer58000_body(ctx, Vec::new()).map(|_| ())
}

fn wop_warp_rtry_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn wop_warp_rtry(ctx: &Ctx) -> Script {
    wop_warp_rtry_body(ctx, Vec::new()).map(|_| ())
}

fn wop_warp_rtry_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("#wop_warp_rtry")])?;
    return Err(Stop::End);
}

pub fn wop_warp_rtry_oninit(ctx: &Ctx) -> Script {
    wop_warp_rtry_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn wop_warp_rtry_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("#wop_warp_rtry")])?;
    return Err(Stop::End);
}

pub fn wop_warp_rtry_onenable(ctx: &Ctx) -> Script {
    wop_warp_rtry_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn wop_warp_rtry_ondisable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("#wop_warp_rtry")])?;
    return Err(Stop::End);
}

pub fn wop_warp_rtry_ondisable(ctx: &Ctx) -> Script {
    wop_warp_rtry_ondisable_body(ctx, Vec::new()).map(|_| ())
}

fn wop_warp_rtry_onready_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::InitNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn wop_warp_rtry_onready(ctx: &Ctx) -> Script {
    wop_warp_rtry_onready_body(ctx, Vec::new()).map(|_| ())
}

fn wop_warp_rtry_onstop_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn wop_warp_rtry_onstop(ctx: &Ctx) -> Script {
    wop_warp_rtry_onstop_body(ctx, Vec::new()).map(|_| ())
}

fn wop_warp_rtry_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::Warp, vec![Val::from("poring_w01"), Val::from(112), Val::from(138)])?;
    return Err(Stop::End);
}

pub fn wop_warp_rtry_ontouch(ctx: &Ctx) -> Script {
    wop_warp_rtry_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn wop_warp_rtry_ontimer3000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("poring_w01"),
            Val::from("Porings: I am giving you 1 minute. Choose your team, ring!"),
            Val::from(0),
            Val::from(15761536),
        ],
    )?;
    return Err(Stop::End);
}

pub fn wop_warp_rtry_ontimer3000(ctx: &Ctx) -> Script {
    wop_warp_rtry_ontimer3000_body(ctx, Vec::new()).map(|_| ())
}

fn wop_warp_rtry_ontimer33000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("poring_w01"),
            Val::from("Porings: 30 seconds left! Come on, Choose a team now, ring?!!"),
            Val::from(0),
            Val::from(15761536),
        ],
    )?;
    return Err(Stop::End);
}

pub fn wop_warp_rtry_ontimer33000(ctx: &Ctx) -> Script {
    wop_warp_rtry_ontimer33000_body(ctx, Vec::new()).map(|_| ())
}

fn wop_warp_rtry_ontimer58000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("poring_w01"),
            Val::from("Porings: You sure you're a warrior?!!! I'm disappointed, ring!"),
            Val::from(0),
            Val::from(15761536),
        ],
    )?;
    return Err(Stop::End);
}

pub fn wop_warp_rtry_ontimer58000(ctx: &Ctx) -> Script {
    wop_warp_rtry_ontimer58000_body(ctx, Vec::new()).map(|_| ())
}

fn wop_warp_rtry_ontimer60000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("poring_w01"),
            Val::from("The battle has been canceled since not all teams are full!!"),
            Val::from(0),
            Val::from(15761536),
        ],
    )?;
    return Err(Stop::End);
}

pub fn wop_warp_rtry_ontimer60000(ctx: &Ctx) -> Script {
    wop_warp_rtry_ontimer60000_body(ctx, Vec::new()).map(|_| ())
}

fn wop_warp_rtry_ontimer61000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("#wop_warp_rtry")])?;
    return Err(Stop::End);
}

pub fn wop_warp_rtry_ontimer61000(ctx: &Ctx) -> Script {
    wop_warp_rtry_ontimer61000_body(ctx, Vec::new()).map(|_| ())
}

fn wop_warp_rtry_ontimer65000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("#wop_warp_rtry")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_master::OnReset")])?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn wop_warp_rtry_ontimer65000(ctx: &Ctx) -> Script {
    wop_warp_rtry_ontimer65000_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum MrDoppelWopTeamAStep {
    Start,
    OnInit,
    OnEnable,
    OnDisable,
}

fn mr_doppel_wop_team_a_run(ctx: &Ctx, mut step: MrDoppelWopTeamAStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_a_tname_s = Val::from("");
    let mut l_pname_s = Val::from("");
    'machine: loop {
        match step {
            MrDoppelWopTeamAStep::Start => {
                if ctx.call(Function::CheckWeight, vec![Val::from(714), Val::from(3)])? == 0 {
                    ctx.lines(args![
                        "- Wait a minute !! -",
                        "- Currently you're carrying -",
                        "- too many items with you. -",
                        "- Please try again -",
                        "- after you lose some weight. -"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                l_a_tname_s = ctx.call(Function::GetPartyName, vec![ctx.var("$@wop_team_a").get()?])?;
                l_pname_s = ctx.call(
                    Function::GetPartyName,
                    vec![ctx.call(Function::GetCharacterId, vec![Val::from(1)])?],
                )?;
                if (ctx.var("$@wop_team_a").get()? != 0
                    && ctx
                        .var("$@wop_team_a")
                        .get()?
                        .loosely_equals(&ctx.call(Function::GetCharacterId, vec![Val::from(1)])?))
                {
                    ctx.lines_as(
                        "Mr. Doppel",
                        args![
                            "So, everyone joined the party?",
                            ((Val::from("The name of the party is... ") + l_pname_s.clone()) + Val::from(", right?")),
                            "I'll transfer you to the battle staging area."
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("No! Wait!:Go to the staging area.")])? {
                        1 => {
                            ctx.lines_as(
                                "Mr. Doppel",
                                args![
                                    "What is it now?",
                                    "Can't you have a little more consideration?",
                                    "You don't have much time. Decide now!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Mr. Doppel",
                                args!["If you don't get there in time, you won't make it to the battle."],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            if ctx
                                .call(
                                    Function::IsPartyLeader,
                                    vec![ctx.call(Function::GetCharacterId, vec![Val::from(1)])?],
                                )?
                                .loosely_equals(&Val::from(1))
                            {
                                ctx.lines_as(
                                    "Mr. Doppel",
                                    args!["So, you are the leader. Before going to the battlefield, you should check all your members."],
                                )?;
                                ctx.next()?;
                                match runtime::select_values(
                                    ctx,
                                    &[Val::from("Very well. I'll be the last.:I am the last. Send me to the battlefield.")],
                                )? {
                                    1 => {
                                        ctx.lines_as("Mr. Doppel", args!["First, make sure all the members are in your party."])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    2 => {
                                        ctx.lines_as("Mr. Doppel", args!["Very well. Nice you have done everything on time."])?;
                                        ctx.call(
                                            Function::DelItem,
                                            vec![Val::from(7773), ctx.call(Function::CountItem, vec![Val::from(7773)])?],
                                        )?;
                                        ctx.close_window()?;
                                        ctx.var("wop_team").set(Val::from(0))?;
                                        ctx.call(Function::Warp, vec![Val::from("poring_w02"), Val::from(44), Val::from(82)])?;
                                        return Err(Stop::End);
                                    }
                                    _ => {}
                                }
                            } else {
                                ctx.lines_as(
                                    "Mr. Doppel",
                                    args![
                                        "Right. I'm gonna send you to the battle staging area.",
                                        "Wait for all the other party members there."
                                    ],
                                )?;
                                ctx.call(
                                    Function::DelItem,
                                    vec![Val::from(7773), ctx.call(Function::CountItem, vec![Val::from(7773)])?],
                                )?;
                                ctx.close_window()?;
                                ctx.var("wop_team").set(Val::from(0))?;
                                ctx.call(Function::Warp, vec![Val::from("poring_w02"), Val::from(44), Val::from(82)])?;
                                return Err(Stop::End);
                            }
                        }
                        _ => {}
                    }
                } else if ctx
                    .call(
                        Function::IsPartyLeader,
                        vec![ctx.call(Function::GetCharacterId, vec![Val::from(1)])?],
                    )?
                    .loosely_equals(&Val::from(1))
                {
                    if ctx.var("$@wop_team_a").get()? == 0 {
                        ctx.lines_as(
                            "Mr. Doppel",
                            args![
                                "So, you are the party leader of Angeling Team.",
                                "Are you sure all the members are in your party?",
                                "First, lets register your party name, after, we'll check the members."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Mr. Doppel",
                            args![
                                "Lets see... The name of the party is...",
                                " ",
                                ((Val::from("^4d4dff ") + l_pname_s.clone()) + Val::from(" ^000000")),
                                " ",
                                "Right? That is the name you wish?"
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(
                            ctx,
                            &[Val::from("No! You're wrong.:Yes. I would like to register that name.:Cancel")],
                        )? {
                            1 => {
                                ctx.lines_as("Mr. Doppel", args!["Hey, I don't have all day! Make your mind and register as fast as you can.", "Don't forget to let all the members join the party. Only the members of a registered party can join the battle."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    "Mr. Doppel",
                                    args![
                                        ((Val::from("So, I'll register your party name as - ") + l_pname_s.clone()) + Val::from(" -.")),
                                        "Now, Tell your ^4d4dffmembers to confirm your party^000000.",
                                        "I'll send you to the battlefield as soon as I confirm your party."
                                    ],
                                )?;
                                ctx.var("$@wop_team_a")
                                    .set(ctx.call(Function::GetCharacterId, vec![Val::from(1)])?)?;
                                ctx.call(Function::MapAnnounce, vec![Val::from("poring_w02"), Val::from("The registration of the Angeling Team has been confirmed. The party members must confirm their team with Mr. Doppel."), Val::from(0), Val::from(3407718)])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            3 => {
                                ctx.lines_as(
                                    "Mr. Doppel",
                                    args!["The clock is ticking. Make up your mind and register as soon as you can."],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    } else {
                        ctx.lines_as(
                            "Mr. Doppel",
                            args![
                                "So, you are the leader of the party.",
                                "Haven't you finished the party registration yet?",
                                "You must stay on the one that has been registered!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Mr. Doppel",
                            args![
                                "The name of the party is ",
                                ((Val::from(" ") + l_a_tname_s.clone()) + Val::from(" ")),
                                "Please, confirm."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                } else {
                    ctx.lines_as(
                        "Mr. Doppel",
                        args!["If you aren't a registered member of the party, you can't join the battle."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mr. Doppel",
                        args![
                            "I'll check again and, after the registeration of the party name, you'll be sent to the battlefield.",
                            "We must stay together as a party, since this is a team game. Otherwise, we'll have problems."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                step = MrDoppelWopTeamAStep::OnInit;
                continue 'machine;
            }
            MrDoppelWopTeamAStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Mr. Doppel#wop_team_a")])?;
                return Err(Stop::End);
            }
            MrDoppelWopTeamAStep::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Mr. Doppel#wop_team_a")])?;
                return Err(Stop::End);
            }
            MrDoppelWopTeamAStep::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("Mr. Doppel#wop_team_a")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn mr_doppel_wop_team_a(ctx: &Ctx) -> Script {
    mr_doppel_wop_team_a_run(ctx, MrDoppelWopTeamAStep::Start, Vec::new()).map(|_| ())
}

pub fn mr_doppel_wop_team_a_oninit(ctx: &Ctx) -> Script {
    mr_doppel_wop_team_a_run(ctx, MrDoppelWopTeamAStep::OnInit, Vec::new()).map(|_| ())
}

pub fn mr_doppel_wop_team_a_onenable(ctx: &Ctx) -> Script {
    mr_doppel_wop_team_a_run(ctx, MrDoppelWopTeamAStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn mr_doppel_wop_team_a_ondisable(ctx: &Ctx) -> Script {
    mr_doppel_wop_team_a_run(ctx, MrDoppelWopTeamAStep::OnDisable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum MrDoppelWopTeamDStep {
    Start,
    OnInit,
    OnEnable,
    OnDisable,
}

fn mr_doppel_wop_team_d_run(ctx: &Ctx, mut step: MrDoppelWopTeamDStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_d_tname_s = Val::from("");
    let mut l_pname_s = Val::from("");
    'machine: loop {
        match step {
            MrDoppelWopTeamDStep::Start => {
                if ctx.call(Function::CheckWeight, vec![Val::from(714), Val::from(3)])? == 0 {
                    ctx.lines(args![
                        "- Wait a minute !! -",
                        "- Currently you're carrying -",
                        "- too many items with you. -",
                        "- Please try again -",
                        "- after you lose some weight. -"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                l_d_tname_s = ctx.call(Function::GetPartyName, vec![ctx.var("$@wop_team_d").get()?])?;
                l_pname_s = ctx.call(
                    Function::GetPartyName,
                    vec![ctx.call(Function::GetCharacterId, vec![Val::from(1)])?],
                )?;
                if (ctx.var("$@wop_team_d").get()? != 0
                    && ctx
                        .var("$@wop_team_d")
                        .get()?
                        .loosely_equals(&ctx.call(Function::GetCharacterId, vec![Val::from(1)])?))
                {
                    ctx.lines_as(
                        "Mr. Doppel",
                        args![
                            "So, everyone joined the party?",
                            ((Val::from("The name of the party is... ") + l_pname_s.clone()) + Val::from(", right?")),
                            "I'll transfer you to the battle staging area."
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("No! Wait!:Go to the staging area.")])? {
                        1 => {
                            ctx.lines_as(
                                "Mr. Doppel",
                                args![
                                    "What is it now?",
                                    "Can't you have a little more consideration?",
                                    "You don't have much time. Decide now!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Mr. Doppel",
                                args!["If you don't get there in time, you won't make it to the battle."],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            if ctx
                                .call(
                                    Function::IsPartyLeader,
                                    vec![ctx.call(Function::GetCharacterId, vec![Val::from(1)])?],
                                )?
                                .loosely_equals(&Val::from(1))
                            {
                                ctx.lines_as(
                                    "Mr. Doppel",
                                    args!["So, you are the leader. Before going to the battlefield, you should check all your members."],
                                )?;
                                ctx.next()?;
                                match runtime::select_values(
                                    ctx,
                                    &[Val::from("Very well. I'll be the last.:I am the last. Send me to the battlefield.")],
                                )? {
                                    1 => {
                                        ctx.lines_as("Mr. Doppel", args!["First, make sure all the members are in your party."])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    2 => {
                                        ctx.lines_as("Mr. Doppel", args!["Very well. Nice you have done everything on time."])?;
                                        ctx.call(
                                            Function::DelItem,
                                            vec![Val::from(7773), ctx.call(Function::CountItem, vec![Val::from(7773)])?],
                                        )?;
                                        ctx.close_window()?;
                                        ctx.var("wop_team").set(Val::from(0))?;
                                        ctx.call(Function::Warp, vec![Val::from("poring_w02"), Val::from(153), Val::from(82)])?;
                                        return Err(Stop::End);
                                    }
                                    _ => {}
                                }
                            } else {
                                ctx.lines_as(
                                    "Mr. Doppel",
                                    args![
                                        "Right. I'm gonna send you to the battle staging area.",
                                        "Wait for all the other party members there."
                                    ],
                                )?;
                                ctx.call(
                                    Function::DelItem,
                                    vec![Val::from(7773), ctx.call(Function::CountItem, vec![Val::from(7773)])?],
                                )?;
                                ctx.close_window()?;
                                ctx.var("wop_team").set(Val::from(0))?;
                                ctx.call(Function::Warp, vec![Val::from("poring_w02"), Val::from(153), Val::from(82)])?;
                                return Err(Stop::End);
                            }
                        }
                        _ => {}
                    }
                } else if ctx
                    .call(
                        Function::IsPartyLeader,
                        vec![ctx.call(Function::GetCharacterId, vec![Val::from(1)])?],
                    )?
                    .loosely_equals(&Val::from(1))
                {
                    if ctx.var("$@wop_team_d").get()? == 0 {
                        ctx.lines_as(
                            "Mr. Doppel",
                            args![
                                "So, you are the party leader of Deviling Team.",
                                "Are you sure all the members are in your party?",
                                "First, lets register your party name, after, we'll check the members."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Mr. Doppel",
                            args![
                                "Lets see... The name of the party is...",
                                " ",
                                ((Val::from("^4d4dff ") + l_pname_s.clone()) + Val::from(" ^000000")),
                                " ",
                                "Right? That is the name you wish?"
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(
                            ctx,
                            &[Val::from("No! You're wrong.:Yes. I would like to register that name.:Cancel")],
                        )? {
                            1 => {
                                ctx.lines_as("Mr. Doppel", args!["Hey, I don't have all day! Make your mind and register as fast as you can.", "Don't forget to let all the members join the party. Only the members of a registered party can join the battle."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    "Mr. Doppel",
                                    args![
                                        ((Val::from("So, I'll register your party name as - ") + l_pname_s.clone()) + Val::from(" -.")),
                                        "Now, Tell your ^4d4dffmembers to confirm your party^000000.",
                                        "I'll send you to the battlefield as soon as I confirm your party."
                                    ],
                                )?;
                                ctx.var("$@wop_team_d")
                                    .set(ctx.call(Function::GetCharacterId, vec![Val::from(1)])?)?;
                                ctx.call(Function::MapAnnounce, vec![Val::from("poring_w02"), Val::from("The registration of the Deviling Team has been confirmed. The party members must confirm their team with Mr. Doppel."), Val::from(0), Val::from(3407718)])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            3 => {
                                ctx.lines_as(
                                    "Mr. Doppel",
                                    args!["The clock is ticking. Make up your mind and register as soon as you can."],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    } else {
                        ctx.lines_as(
                            "Mr. Doppel",
                            args![
                                "So, you are the leader of the party.",
                                "Haven't you finished the party registration yet?",
                                "You must stay on the one that has been registered!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Mr. Doppel",
                            args![
                                "The name of the party is ",
                                ((Val::from(" ") + l_d_tname_s.clone()) + Val::from(" ")),
                                "Please, confirm."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                } else {
                    ctx.lines_as(
                        "Mr. Doppel",
                        args!["If you aren't a registered member of the party, you can't join the battle."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mr. Doppel",
                        args![
                            "I'll check again and, after the registeration of the party name, you'll be sent to the battlefield.",
                            "We must stay together as a party, since this is a team game. Otherwise, we'll have problems."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                step = MrDoppelWopTeamDStep::OnInit;
                continue 'machine;
            }
            MrDoppelWopTeamDStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Mr. Doppel#wop_team_d")])?;
                return Err(Stop::End);
            }
            MrDoppelWopTeamDStep::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Mr. Doppel#wop_team_d")])?;
                return Err(Stop::End);
            }
            MrDoppelWopTeamDStep::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("Mr. Doppel#wop_team_d")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn mr_doppel_wop_team_d(ctx: &Ctx) -> Script {
    mr_doppel_wop_team_d_run(ctx, MrDoppelWopTeamDStep::Start, Vec::new()).map(|_| ())
}

pub fn mr_doppel_wop_team_d_oninit(ctx: &Ctx) -> Script {
    mr_doppel_wop_team_d_run(ctx, MrDoppelWopTeamDStep::OnInit, Vec::new()).map(|_| ())
}

pub fn mr_doppel_wop_team_d_onenable(ctx: &Ctx) -> Script {
    mr_doppel_wop_team_d_run(ctx, MrDoppelWopTeamDStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn mr_doppel_wop_team_d_ondisable(ctx: &Ctx) -> Script {
    mr_doppel_wop_team_d_run(ctx, MrDoppelWopTeamDStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn wop_master(ctx: &Ctx) -> Script {
    wop_master_run(ctx, WopMasterStep::Start, Vec::new()).map(|_| ())
}

pub fn wop_master_onreset(ctx: &Ctx) -> Script {
    wop_master_run(ctx, WopMasterStep::OnReset, Vec::new()).map(|_| ())
}

pub fn wop_master_onstart(ctx: &Ctx) -> Script {
    wop_master_run(ctx, WopMasterStep::OnStart, Vec::new()).map(|_| ())
}

pub fn wop_master_onangelingwarn(ctx: &Ctx) -> Script {
    wop_master_run(ctx, WopMasterStep::OnAngelingWarn, Vec::new()).map(|_| ())
}

pub fn wop_master_ondevilingwarn(ctx: &Ctx) -> Script {
    wop_master_run(ctx, WopMasterStep::OnDevilingWarn, Vec::new()).map(|_| ())
}

pub fn wop_master_ondevilingend(ctx: &Ctx) -> Script {
    wop_master_run(ctx, WopMasterStep::OnDevilingEnd, Vec::new()).map(|_| ())
}

pub fn wop_master_onangelingend(ctx: &Ctx) -> Script {
    wop_master_run(ctx, WopMasterStep::OnAngelingEnd, Vec::new()).map(|_| ())
}

pub fn wop_master_onstop(ctx: &Ctx) -> Script {
    wop_master_run(ctx, WopMasterStep::OnStop, Vec::new()).map(|_| ())
}

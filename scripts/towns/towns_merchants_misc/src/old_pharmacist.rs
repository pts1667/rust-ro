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

#[derive(Clone, Copy, Debug)]
enum PharmacistStep {
    Start,
    LMaking,
}

fn pharmacist_run(ctx: &Ctx, mut step: PharmacistStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_amount = 0;
    let mut l_item_req = Val::from(0);
    let mut l_max = 0;
    let mut l_req_amount = Val::from(0);
    'machine: loop {
        match step {
            PharmacistStep::Start => {
                if ctx.call(Function::CheckWeight, args![1201, 1])? == 0 {
                    ctx.lines(args![
                        "- Wait a moment! -",
                        "- Currently you're carrying -",
                        "- too many items with you. -",
                        "- Please come back later -",
                        "- after you put some items into kafra storage. -"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as("Old Pharmacist", args!["Ummmm...", "What brings you here...?"])?;
                ctx.next()?;
                let subject1 = Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("Make Potion:Talk.:Mixing Information:Cancel")],
                )?);
                let mut matched1 = false;
                if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                    matched1 = true;
                }
                if matched1 {
                    if ctx.var("MaxWeight").get()?.number()? - ctx.var("Weight").get()?.number()? < 5000 {
                        ctx.lines_as(
                            "Old Pharmacist",
                            args![
                                "Why are you carrying these so many!",
                                "Don't be greedy, carry only as much you need!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Old Pharmacist",
                            args![
                                "You are too heavy to receive potions from me...",
                                "Go store some items in your storage first!"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Old Pharmacist",
                            args!["You have all the stuff ready, right? Which one would you like?"],
                        )?;
                        ctx.next()?;
                        let subject2 = Val::from(runtime::select_values(
                            ctx,
                            &[Val::from(
                                "Red Potion.:Orange Potion.:Yellow Potion.:White Potion.:Blue Potion.:Green Potion.:Actually, I don't want anything.",
                            )],
                        )?);
                        let mut matched2 = false;
                        if !matched2 && subject2.loosely_equals(&Val::from(1)) {
                            matched2 = true;
                        }
                        if matched2 {
                            pharmacist_run(ctx, PharmacistStep::LMaking, args![507, 3, 501])?;
                        }
                        if !matched2 && subject2.loosely_equals(&Val::from(2)) {
                            matched2 = true;
                        }
                        if matched2 {
                            ctx.mes("[Old Pharmacist]")?;
                            if ctx.items().count(507)? < ctx.items().count(713)? {
                                l_max = ctx.items().count(507)?;
                            } else if ctx.items().count(508)? < ctx.items().count(713)? {
                                l_max = ctx.items().count(508)?;
                            } else {
                                l_max = ctx.items().count(713)?;
                            }
                            if ctx.items().count(507)? < 1 || ctx.items().count(508)? < 1 || ctx.call(Function::CountItem, args![713])? == 0
                            {
                                ctx.lines(args![
                                    "You rascal! What did you expect?! Coming here with nothing. Tsk!",
                                    "Get lost!"
                                ])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            if ctx.player().zeny()? < 3 {
                                ctx.lines(args![
                                    "You rascal! What did you expect?! Coming here with nothing. Tsk!",
                                    "Get lost!"
                                ])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.mes("How many?")?;
                            ctx.next()?;
                            let subject3 = runtime::select_values(
                                ctx,
                                &[Val::from(
                                    "Make as many as I can.:I want to choose an amount.:Actually, I don't want anything.",
                                )],
                            )?;
                            if subject3 == 1 {
                                if ctx.items().count(507)? < l_max
                                    || ctx.items().count(508)? < l_max
                                    || ctx.items().count(713)? < l_max
                                    || ctx.player().zeny()? < l_max * 3
                                {
                                    ctx.lines_as(
                                        "Old Pharmacist",
                                        args!["You rascal! You don't even have all the materials and you want me to make you potions?!"],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                ctx.player().set_zeny(ctx.player().zeny()? - l_max * 5)?;
                                ctx.call(Function::DelItem, args![507, l_max])?;
                                ctx.call(Function::DelItem, args![508, l_max])?;
                                ctx.call(Function::DelItem, args![713, l_max])?;
                                ctx.call(Function::GetItem, args![502, l_max])?;
                            } else if subject3 == 2 {
                                ctx.lines_as("Old Pharmacist", args![Val::from("Then pick a number below 100. If you don't want any, just enter '0'. With the materials you have, you can make about ") + l_max + Val::from(" potions.")])?;
                                ctx.next()?;
                                let (input, _) = runtime::input_number(ctx, Some(0), Some(101))?;
                                l_amount = input.number()?;
                                if l_amount == 0 {
                                    ctx.lines_as("Old Pharmacist", args!["Make up your mind, will you?!"])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                if l_amount > 100 {
                                    ctx.lines_as("Old Pharmacist", args!["Are you deaf? I said less than 100!"])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                if ctx.items().count(507)? < l_amount
                                    || ctx.items().count(508)? < l_amount
                                    || ctx.items().count(713)? < l_amount
                                    || ctx.player().zeny()? < l_amount * 3
                                {
                                    ctx.lines_as(
                                        "Old Pharmacist",
                                        args!["You rascal! You don't even have all the materials and you want me to make you potions?!"],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                ctx.next()?;
                                ctx.player().set_zeny(ctx.player().zeny()? - l_amount * 5)?;
                                ctx.call(Function::DelItem, args![507, l_amount])?;
                                ctx.call(Function::DelItem, args![508, l_amount])?;
                                ctx.call(Function::DelItem, args![713, l_amount])?;
                                ctx.call(Function::GetItem, args![502, l_amount])?;
                            } else if subject3 == 3 {
                                ctx.lines_as("Old Pharmacist", args!["What?!", "Grrr...", "Bleh!", "Get lost!"])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.lines_as(
                                "Old Pharmacist",
                                args!["Here you go. It's all done so you can take it. But remember! Abusing medicine is not good."],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        if !matched2 && subject2.loosely_equals(&Val::from(3)) {
                            matched2 = true;
                        }
                        if matched2 {
                            pharmacist_run(ctx, PharmacistStep::LMaking, args![508, 10, 503])?;
                        }
                        if !matched2 && subject2.loosely_equals(&Val::from(4)) {
                            matched2 = true;
                        }
                        if matched2 {
                            pharmacist_run(ctx, PharmacistStep::LMaking, args![509, 20, 504])?;
                        }
                        if !matched2 && subject2.loosely_equals(&Val::from(5)) {
                            matched2 = true;
                        }
                        if matched2 {
                            pharmacist_run(ctx, PharmacistStep::LMaking, args![510, 30, 505])?;
                        }
                        if !matched2 && subject2.loosely_equals(&Val::from(6)) {
                            matched2 = true;
                        }
                        if matched2 {
                            pharmacist_run(ctx, PharmacistStep::LMaking, args![511, 3, 506])?;
                        }
                        if !matched2 && subject2.loosely_equals(&Val::from(7)) {
                            matched2 = true;
                        }
                        if matched2 {
                            ctx.lines_as("Old Pharmacist", args!["What?!", "Grrr...", "Bleh!", "Get lost!"])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                }
                if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                    matched1 = true;
                }
                if matched1 {
                    ctx.lines_as("Old Pharmacist", args!["With medicine, you can increase a person's ability to regenerate. But, they're only good up to a point. *Sigh* I'm starting to think of the days when I was young. I must be getting old."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Old Pharmacist",
                        args!["Anyways, a potion is merely a potion. Nothing more and nothing less."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if !matched1 && subject1.loosely_equals(&Val::from(3)) {
                    matched1 = true;
                }
                if matched1 {
                    ctx.lines_as(
                        "Old Pharmacist",
                        args![
                            "Hrrrmm...",
                            "You young ones can be quite annoying. But, since you asked, I'll explain."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Old Pharmacist", args!["Herbs work well by themselves, but if you use my special techniques and skills to make potions out of them, the effect is much much greater."])?;
                    ctx.next()?;
                    ctx.lines_as("Old Pharmacist", args!["If you ask eagerly and politely, I will make them for you. But, not for free... Don't worry though, I only charge a small fee, so it's not that expensive."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Old Pharmacist",
                        args![
                            "Red Potion - ^0098E52 Red Herbs, 1 Empty Bottle, 2 zeny fee.^000000",
                            "Orange Potion - ^0098E51 Red Herb, 1 Yellow Herb, 1 Empty Bottle, 5 zeny fee.^000000",
                            "Yellow Potion - ^0098E52 Yellow Herbs, 1 Empty Bottle, 10 zeny.^000000"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Old Pharmacist",
                        args![
                            "White Potion - ^0098E52 White Herbs, 1 Empty Bottle, 20 zeny fee.^000000",
                            "Blue Potion - ^0098E52 Blue Herbs, 1 Empty Bottle, 30 zeny fee.^000000",
                            "Green Potion - ^0098E52 Green Herbs, 1 Empty Bottle, 3 zeny fee.^000000"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if !matched1 && subject1.loosely_equals(&Val::from(4)) {
                    matched1 = true;
                }
                if matched1 {
                    ctx.lines_as(
                        "Old Pharmacist",
                        args!["What a boring person. If you have something to say, why don't you say it?!"],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                step = PharmacistStep::LMaking;
                continue 'machine;
            }
            PharmacistStep::LMaking => {
                l_item_req = runtime::arg(&args, 0, Val::from(0));
                l_req_amount = runtime::arg(&args, 1, Val::from(0));
                ctx.mes("[Old Pharmacist]")?;
                if ctx.call(Function::CountItem, args![l_item_req.clone()])?.number()? / 2 < ctx.items().count(713)? {
                    l_max = ctx.call(Function::CountItem, args![l_item_req.clone()])?.number()? / 2;
                } else {
                    l_max = ctx.items().count(713)?;
                }
                if ctx.call(Function::CountItem, args![l_item_req.clone()])?.number()? < 2
                    || ctx.call(Function::CountItem, args![713])? == 0
                {
                    ctx.lines(args![
                        "You rascal! What did you expect?! Coming here with nothing. Tsk!",
                        "Get lost!"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ctx.player().zeny()? < 3 {
                    ctx.lines(args![
                        "You rascal! What did you expect?! Coming here with nothing. Tsk!",
                        "Get lost!"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.mes("How many?")?;
                ctx.next()?;
                let subject4 = runtime::select_values(
                    ctx,
                    &[Val::from(
                        "Make as many as I can.:I want to choose an amount.:Actually, I don't want anything.",
                    )],
                )?;
                if subject4 == 1 {
                    if ctx.call(Function::CountItem, args![l_item_req.clone()])?.number()? < l_max * 2
                        || ctx.items().count(713)? < l_max
                        || ctx.player().zeny()? < l_max * l_req_amount.number()?
                    {
                        ctx.lines_as(
                            "Old Pharmacist",
                            args!["You rascal! You don't even have all the materials and you want me to make you potions?!"],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.player().set_zeny(ctx.player().zeny()? - l_max * l_req_amount.number()?)?;
                    ctx.call(Function::DelItem, args![l_item_req.clone(), l_max * 2])?;
                    ctx.call(Function::DelItem, args![713, l_max])?;
                    ctx.call(Function::GetItem, args![runtime::arg(&args, 2, Val::from(0)), l_max])?;
                } else if subject4 == 2 {
                    ctx.lines_as("Old Pharmacist", args![Val::from("Then pick a number below 100. If you don't want any, just enter '0'. With the materials you have, you can make about ") + l_max + Val::from(" potions.")])?;
                    ctx.next()?;
                    let (input, _) = runtime::input_number(ctx, None, None)?;
                    l_amount = input.number()?;
                    if l_amount == 0 {
                        ctx.lines_as("Old Pharmacist", args!["Make up your mind, will you?!"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    if l_amount > 100 {
                        ctx.lines_as("Old Pharmacist", args!["Are you deaf? I said less than 100!"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    if ctx.call(Function::CountItem, args![l_item_req.clone()])?.number()? < l_amount * 2
                        || ctx.items().count(713)? < l_amount
                        || ctx.player().zeny()? < l_amount * l_req_amount.number()?
                    {
                        ctx.lines_as(
                            "Old Pharmacist",
                            args!["You rascal! You don't even have all the materials and you want me to make you potions?!"],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.player().set_zeny(ctx.player().zeny()? - l_max * l_req_amount.number()?)?;
                    ctx.call(Function::DelItem, args![l_item_req.clone(), l_amount * 2])?;
                    ctx.call(Function::DelItem, args![713, l_amount])?;
                    ctx.call(Function::GetItem, args![runtime::arg(&args, 2, Val::from(0)), l_amount])?;
                } else if subject4 == 3 {
                    ctx.lines_as("Old Pharmacist", args!["What?!", "Grrr...", "Bleh!", "Get lost!"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Old Pharmacist",
                    args!["Here you go. It's all done so you can take it. But remember! Abusing medicine is not good."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn pharmacist(ctx: &Ctx) -> Script {
    pharmacist_run(ctx, PharmacistStep::Start, Vec::new()).map(|_| ())
}

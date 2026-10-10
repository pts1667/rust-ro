use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn bowl_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_bat = Val::from(0);
    let mut l_implode2_s = Val::from("");
    let mut l_implode3_s = Val::from("");
    let mut l_implode_s = Val::from("");
    let mut l_input = Val::from(0);
    let mut l_locker = Val::from(0);
    let mut l_m = Val::from(0);
    let mut l_maho = Val::from(0);
    let mut l_menu2_s: Vec<Val> = Vec::new();
    let mut l_menu3_s: Vec<Val> = Vec::new();
    let mut l_menu_s: Vec<Val> = Vec::new();
    let mut l_mush = Val::from(0);
    let mut l_nankai = Val::from(0);
    let mut l_sand = Val::from(0);
    let mut l_star = Val::from(0);
    let mut l_w = Val::from(0);
    if ctx.var("mos_nowinter").get()? != 16 {
        return Err(Stop::End);
    }
    ctx.lines(args![
        "-It is a very dirty pot.",
        "Something is boiling",
        "Baba Yaga might have done something",
        "with it.",
        "Well, let's get it started.-"
    ])?;
    ctx.next()?;
    let base = Val::from(0).number()?;
    runtime::local_set(&mut l_menu_s, &Val::from(base + 0), Val::from("Powder Of Wing Of Bat"), true);
    runtime::local_set(&mut l_menu_s, &Val::from(base + 1), Val::from("Liquid Of Spawn"), true);
    runtime::local_set(&mut l_menu_s, &Val::from(base + 2), Val::from("Grasshopper's Leg"), true);
    runtime::local_set(&mut l_menu_s, &Val::from(base + 3), Val::from("Starsand Of Witch"), true);
    runtime::local_set(&mut l_menu_s, &Val::from(base + 4), Val::from("Fine Grit"), true);
    l_implode_s = runtime::implode(&l_menu_s, &Val::from(":"))?;
    'l1: loop {
        if !(l_nankai.clone().number()? < 3) {
            break 'l1;
        }
        'b1: {
            ctx.lines(args!["-Something is still being boiled in the pot.", "What am I going to do?-"])?;
            ctx.next()?;
            match runtime::select_values(
                ctx,
                &[Val::from("Put the materials in it.:Pour water in it.:Stir it up.:It is over!")],
            )? {
                1 => {
                    l_m = (Val::from(runtime::select_values(ctx, &[l_implode_s.clone()])?).try_sub(Val::from(1))?);
                    if (((!(l_m.clone().is_true()) && l_bat.clone().is_true()) || (l_m.clone() == 1 && l_mush.clone().is_true()))
                        || (l_m.clone() == 2 && l_locker.clone().is_true()))
                    {
                        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_POISONHIT")?])?;
                    } else {
                        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_SMOKE")?])?;
                        if l_m.clone() == 0 {
                            l_bat = (l_bat.clone() + Val::from(1));
                        }
                        if l_m.clone() == 1 {
                            l_mush = (l_mush.clone() + Val::from(1));
                        }
                        if l_m.clone() == 2 {
                            ctx.var(".locker").set((ctx.var(".locker").get()? + Val::from(1)))?;
                        }
                        l_maho = (l_maho.clone() + Val::from(1));
                    }
                    if l_m.clone().number()? >= 3 {
                        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_POISONHIT")?])?;
                    }
                    l_nankai = (l_nankai.clone() + Val::from(1));
                    ctx.lines(args![
                        ((Val::from("-I put the ") + runtime::local_get(&l_menu_s, &l_m.clone(), true)) + Val::from(" in the pot.")),
                        "Its smell slightly changes.-"
                    ])?;
                }
                2 => {
                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_POISONHIT")?])?;
                    l_nankai = (l_nankai.clone() + Val::from(1));
                    ctx.lines(args![
                        "-I pour water in the pot a little.",
                        "The liquid has become thin.",
                        "No other remarkable changes",
                        "have happened.-"
                    ])?;
                }
                3 => {
                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_POISONHIT")?])?;
                    l_nankai = (l_nankai.clone() + Val::from(1));
                    ctx.lines(args![
                        "-I stir it up",
                        "with a stick several times.",
                        "No other remarkable changes",
                        "have happened.-"
                    ])?;
                }
                4 => {
                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_POISONATTACK")?])?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["I will tell Baba Yaga that", "the work has been done."],
                    )?;
                    ctx.var("mos_nowinter").set(Val::from(17))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
            ctx.next()?;
        }
    }
    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BUBBLE")?])?;
    ctx.lines(args![
        "-The liquid has been changed",
        "and is now bubbling.",
        "It seems to have shrunk,",
        "but not by much.-"
    ])?;
    ctx.next()?;
    ctx.lines(args!["-Anyway, the first step is done", "let's go on the next stage.-"])?;
    ctx.next()?;
    'l3: loop {
        if !(l_nankai.clone().number()? < 7) {
            break 'l3;
        }
        'b3: {
            ctx.mes("-Well, What am I going to do?-")?;
            ctx.next()?;
            match runtime::select_values(
                ctx,
                &[Val::from("Put the materials in it.:Pour water in it.:Stir it up.:It is over!")],
            )? {
                1 => {
                    l_w = Val::from(0);
                    l_m = (Val::from(runtime::select_values(ctx, &[runtime::implode(&l_menu_s, &Val::from(":"))?])?)
                        .try_sub(Val::from(1))?);
                    if l_m.clone().number()? >= 3 {
                        if (l_nankai.clone() == 5 || l_nankai.clone() == 6) {
                            if ((l_m.clone() == 3 && l_star.clone().is_true()) || (l_m.clone() == 4 && l_sand.clone().is_true())) {
                                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_POISONHIT")?])?;
                            } else {
                                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_SMOKE")?])?;
                                l_maho = (l_maho.clone() + Val::from(1));
                                if l_m.clone() == 3 {
                                    l_star = (l_star.clone() + Val::from(1));
                                }
                                if l_m.clone() == 4 {
                                    l_sand = (l_sand.clone() + Val::from(1));
                                }
                                l_w = Val::from(1);
                            }
                        } else {
                            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_POISONHIT")?])?;
                            if l_m.clone() == 3 {
                                l_star = (l_star.clone() + Val::from(1));
                            }
                            if l_m.clone() == 4 {
                                l_sand = (l_sand.clone() + Val::from(1));
                            }
                        }
                        ctx.next()?;
                    } else {
                        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_POISONHIT")?])?;
                    }
                    l_nankai = (l_nankai.clone() + Val::from(1));
                    ctx.lines(args![
                        ((Val::from("-I put the ") + runtime::local_get(&l_menu_s, &l_m.clone(), true)) + Val::from(" in the pot."))
                    ])?;
                    if l_m.clone().number()? <= 2 {
                        ctx.mes("It's smell drastically changes.-")?;
                    } else {
                        ctx.lines(args![
                            ((Val::from("Its ")
                                + (if l_w.clone().is_true() {
                                    Val::from("smell")
                                } else {
                                    Val::from("color")
                                }))
                                + Val::from(" slightly changes.-"))
                        ])?;
                    }
                }
                2 => {
                    if l_nankai.clone() == 3 {
                        ctx.lines(args![
                            "-I pour water in the pot a little.",
                            "The boiling sounds stronger as if",
                            "something in the cloudy liquid",
                            "has changed.-"
                        ])?;
                        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_DRAGONSMOKE")?])?;
                        l_maho = (l_maho.clone() + Val::from(1));
                    } else {
                        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_POISONHIT")?])?;
                        ctx.mes("-I pour water in the pot and, the liquid gets thin.-")?;
                    }
                    l_nankai = (l_nankai.clone() + Val::from(1));
                }
                3 => {
                    if l_nankai.clone() == 4 {
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["Ok, I will stir it up this time.", "How many times should I..?"],
                        )?;
                        ctx.next()?;
                        let (input, status) = runtime::input_number(ctx, None, None)?;
                        l_input = input;
                        if l_input.clone() == 20 {
                            ctx.mes("-It must be 20 times.-")?;
                            ctx.next()?;
                            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_WIND")?])?;
                            ctx.lines(args![
                                "-Stirring up makes it",
                                "brighter and",
                                "its smell gets",
                                "more bearable."
                            ])?;
                            l_maho = (l_maho.clone() + Val::from(1));
                            l_nankai = (l_nankai.clone() + Val::from(1));
                        } else if !(l_input.clone().is_true()) {
                            ctx.mes("-I won't stir it up.-")?;
                        } else if l_input.clone().number()? > 100 {
                            ctx.lines(args!["-It won't be able", "to stir so many times.", "Let me think again.-"])?;
                        } else {
                            ctx.lines(args![
                                ((Val::from("-Yes, it must be ") + l_input.clone()) + Val::from(" times.-"))
                            ])?;
                            ctx.next()?;
                            ctx.lines(args!["-I stir it up really hard.", "It is boiled.-"])?;
                            l_nankai = (l_nankai.clone() + Val::from(1));
                        }
                    } else {
                        ctx.mes("-Bubble, bubble-")?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["Something changed?"],
                        )?;
                        l_nankai = (l_nankai.clone() + Val::from(1));
                    }
                }
                4 => {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["I will tell Baba Yaga that", "it has been done."],
                    )?;
                    ctx.var("mos_nowinter").set(Val::from(17))?;
                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_POISONATTACK")?])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
            ctx.next()?;
        }
    }
    ctx.lines(args!["-I am sure that the book told me", "to wait for some time.-"])?;
    ctx.next()?;
    ctx.mes("............")?;
    ctx.next()?;
    ctx.mes("............")?;
    ctx.next()?;
    ctx.lines(args![
        "-The smell of the liquid boiling in",
        "the pot has changed enough.",
        "Let's go on to the next stage.-"
    ])?;
    ctx.next()?;
    let base = Val::from(0).number()?;
    runtime::local_set(&mut l_menu2_s, &Val::from(base + 0), Val::from("Witched Starsand"), true);
    runtime::local_set(&mut l_menu2_s, &Val::from(base + 1), Val::from("Fine Grit"), true);
    runtime::local_set(&mut l_menu2_s, &Val::from(base + 2), Val::from("Detonator"), true);
    runtime::local_set(&mut l_menu2_s, &Val::from(base + 3), Val::from("Red Blood"), true);
    runtime::local_set(&mut l_menu2_s, &Val::from(base + 4), Val::from("Burning Heart"), true);
    let base = Val::from(0).number()?;
    runtime::local_set(&mut l_menu3_s, &Val::from(base + 0), Val::from("Witched Starsand"), true);
    runtime::local_set(&mut l_menu3_s, &Val::from(base + 1), Val::from("Fine Grit"), true);
    runtime::local_set(&mut l_menu3_s, &Val::from(base + 2), Val::from("Detonator"), true);
    runtime::local_set(&mut l_menu3_s, &Val::from(base + 3), Val::from("Red Blood"), true);
    runtime::local_set(&mut l_menu3_s, &Val::from(base + 4), Val::from("Burning Heart"), true);
    runtime::local_set(&mut l_menu3_s, &Val::from(base + 5), Val::from("Piece Of Diamond"), true);
    l_implode2_s = runtime::implode(&l_menu2_s, &Val::from(":"))?;
    l_implode3_s = runtime::implode(&l_menu3_s, &Val::from(":"))?;
    'l5: loop {
        if !(true) {
            break 'l5;
        }
        'b5: {
            ctx.mes("-Well, What am I going to do?-")?;
            ctx.next()?;
            match runtime::select_values(
                ctx,
                &[Val::from("Put the materials in it.:Pour water in it.:Stir it up.:It is over!")],
            )? {
                1 => {
                    l_w = Val::from(0);
                    if l_nankai.clone() != 11 {
                        l_m = (Val::from(runtime::select_values(ctx, &[l_implode2_s.clone()])?).try_sub(Val::from(1))?);
                        if (((l_m.clone() == 2 && l_nankai.clone() == 9) || (l_m.clone() == 3 && l_nankai.clone() == 8))
                            || (l_m.clone() == 4 && l_nankai.clone() == 10))
                        {
                            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_SMOKE")?])?;
                            l_maho = (l_maho.clone() + Val::from(1));
                        } else {
                            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_POISONHIT")?])?;
                            l_w = Val::from(1);
                        }
                        if l_m.clone().number()? <= 1 {
                            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_POISONHIT")?])?;
                        }
                        l_nankai = (l_nankai.clone() + Val::from(1));
                        ctx.lines(args![
                            ((Val::from("-I put the ") + runtime::local_get(&l_menu2_s, &l_m.clone(), true)) + Val::from(" in the pot."))
                        ])?;
                        if l_m.clone().number()? <= 1 {
                            ctx.mes("Its smell slightly changes.-")?;
                        } else {
                            ctx.lines(args![
                                (if l_w.clone().is_true() {
                                    Val::from("The smell gets worse.-")
                                } else {
                                    Val::from("The smell has been changed a little.-")
                                })
                            ])?;
                        }
                    } else {
                        l_m = (Val::from(runtime::select_values(ctx, &[l_implode3_s.clone()])?).try_sub(Val::from(1))?);
                        if l_m.clone() != 5 {
                            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_POISONHIT")?])?;
                        } else {
                            l_maho = (l_maho.clone() + Val::from(1));
                            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_SMOKE")?])?;
                        }
                        l_nankai = (l_nankai.clone() + Val::from(1));
                        ctx.lines(args![
                            ((((Val::from("-I put ") + (if l_m.clone() == 5 { Val::from("a") } else { Val::from("the") }))
                                + Val::from(" "))
                                + runtime::local_get(&l_menu3_s, &l_m.clone(), true))
                                + Val::from(" in the pot."))
                        ])?;
                        if l_m.clone() == 5 {
                            ctx.mes("The solution alters in color.-")?;
                        } else {
                            ctx.mes("The smell gets worse.-")?;
                        }
                    }
                }
                2 => {
                    l_nankai = (l_nankai.clone() + Val::from(1));
                    ctx.lines(args!["-I pour water in the pot a little.", "The smell gets better.-"])?;
                }
                3 => {
                    if l_nankai.clone() == 7 {
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["Ok, I will stir it up this time.", "How many times should I...?"],
                        )?;
                        ctx.next()?;
                        let (input, status) = runtime::input_number(ctx, None, None)?;
                        l_input = input;
                        if l_input.clone() == 15 {
                            ctx.mes("-It must be 15 times.-")?;
                            ctx.next()?;
                            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_WIND")?])?;
                            ctx.lines(args!["-Stirring up makes it", "brighter.-"])?;
                            l_maho = (l_maho.clone() + Val::from(1));
                            l_nankai = (l_nankai.clone() + Val::from(1));
                        } else if l_input.clone() == 0 {
                            ctx.mes("-I won't stir it up.-")?;
                        } else if l_input.clone().number()? > 100 {
                            ctx.lines(args!["-It won't be to", "stir so many times.", "Let me think again.-"])?;
                        } else {
                            ctx.lines(args![
                                ((Val::from("-Yes, it must be ") + l_input.clone()) + Val::from(" times.-"))
                            ])?;
                            ctx.next()?;
                            ctx.lines(args!["-I stir it up really hard.", "It is boiled.-"])?;
                            l_nankai = (l_nankai.clone() + Val::from(1));
                        }
                    } else {
                        ctx.mes("-Bubble, bubble-")?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["Something changed?"],
                        )?;
                        l_nankai = (l_nankai.clone() + Val::from(1));
                    }
                }
                4 => {
                    ctx.lines(args!["-It seems that the work is over.", "I need to show this to Baba Yaga.-"])?;
                    if l_maho.clone() == 12 {
                        ctx.var("mos_nowinter").set(Val::from(18))?;
                        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_GASPUSH")?])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_POISONATTACK")?])?;
                    ctx.var("mos_nowinter").set(Val::from(17))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
            ctx.next()?;
        }
    }
    Ok(Val::from(0))
}

pub fn bowl(ctx: &Ctx) -> Script {
    bowl_body(ctx, Vec::new()).map(|_| ())
}

fn nowinterplz_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("mos_nowinter").get()? != 19 {
        return Err(Stop::End);
    }
    if ctx.call(Function::CountItem, vec![Val::from(7765)])?.is_true() {
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["This place must be", "the center of town..."],
        )?;
        ctx.next()?;
        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Ok, let's do it!"])?;
        ctx.next()?;
        ctx.lines(args!["-I drop the magic bottle", "containing Baba Yaga's", "Secret Medicine.-"])?;
        ctx.call(Function::DelItem, vec![Val::from(7765), Val::from(1)])?;
        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BARRIER")?])?;
        ctx.var("mos_nowinter").set(Val::from(20))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(18078), Val::from(18079)])?;
        ctx.next()?;
        ctx.lines(args![
            "-You feel like your",
            "body is heating up.",
            "It's much hotter",
            "than before.-"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["I'm sure that the season has changed", "But how can I confirm this?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["First I'm going to see the Csar.", "He would like to hear about this."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
        args!["Ah, where did I place", "the magic bottle that Baba Yaga gave to me..."],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn nowinterplz(ctx: &Ctx) -> Script {
    nowinterplz_body(ctx, Vec::new()).map(|_| ())
}

fn a_little_girl_mos1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_kid = Val::from(0);
    if ctx.var("mos_nowinter").get()? == 10 && ctx.var("mos_kid").get()? == 0 {
        l_kid = ctx.call(Function::Rand, vec![Val::from(1), Val::from(100)])?;
        let choice = runtime::select_values(ctx, &[Val::from("Do you like the warm weather, little girl?")])?;
        ctx.var("@menu").set(choice)?;
        ctx.mes("[Yosefina]")?;
        if l_kid.clone().number()? > 70 {
            ctx.lines(args![
                "Yes, I like it very much~",
                "I never want it to leave. I don't want winter to come."
            ])?;
            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_SUMMONSLAVE")?])?;
            ctx.var("mos_kid").set(Val::from(1))?;
            if (ctx.var("mos_middle").get()? == 1 && ctx.var("mos_elder").get()? == 1) {
                ctx.var("mos_nowinter").set(Val::from(11))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(18075), Val::from(18076)])?;
            }
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines(args![
            "Yes, I like it very much~",
            "But, as soon as the sun sets I must go back home."
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Yosefina",
            args!["Baba Yaga kidnaps", "bad kids wandering", "in the night, I heard."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Yosefina",
        args!["Baba Yaga, the Horrible Cannibal", "is living outside the town."],
    )?;
    ctx.next()?;
    ctx.lines_as("Yosefina", args!["My mom told me.", "that she is real."])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn a_little_girl_mos1(ctx: &Ctx) -> Script {
    a_little_girl_mos1_body(ctx, Vec::new()).map(|_| ())
}

fn a_young_man_mos2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_kid = Val::from(0);
    if ctx.var("mos_nowinter").get()? == 10 && ctx.var("mos_middle").get()? == 0 {
        let choice = runtime::select_values(ctx, &[Val::from("You don't like winter, do you?")])?;
        ctx.var("@menu").set(choice)?;
        l_kid = ctx.call(Function::Rand, vec![Val::from(1), Val::from(100)])?;
        ctx.mes("[Fedor]")?;
        if l_kid.clone().number()? > 70 {
            ctx.lines(args![
                "No, I don't like the cold winter.",
                "And I get angry when",
                "seeing couples."
            ])?;
            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_SUMMONSLAVE")?])?;
            ctx.var("mos_middle").set(Val::from(1))?;
            if (ctx.var("mos_kid").get()? == 1 && ctx.var("mos_elder").get()? == 1) {
                ctx.var("mos_nowinter").set(Val::from(11))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(18075), Val::from(18076)])?;
            }
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines(args![
            "Huuuu, I wish I could have a girlfriend",
            "I would be able to enjoy the warm sunlight better..."
        ])?;
        ctx.next()?;
        ctx.lines_as("Fedor", args!["Who are you?", "Don't make matters worse!"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as("Fedor", args!["Finally, winter is over.", "Spring is coming", "to my mind..."])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn a_young_man_mos2(ctx: &Ctx) -> Script {
    a_young_man_mos2_body(ctx, Vec::new()).map(|_| ())
}

fn a_middle_aged_man_mos3_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("mos_nowinter").get()? == 10 && ctx.var("mos_elder").get()? == 0 {
        let choice = runtime::select_values(ctx, &[Val::from("Whoever likes the winter?")])?;
        ctx.var("@menu").set(choice)?;
        ctx.mes("[Viktor]")?;
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(100)])?.number()? > 70 {
            ctx.lines(args![
                "Tell me about it!",
                "Everyone will be sad",
                "when it comes again.",
                "I don't want winter to come."
            ])?;
            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_SUMMONSLAVE")?])?;
            ctx.var("mos_elder").set(Val::from(1))?;
            if (ctx.var("mos_kid").get()? == 1 && ctx.var("mos_middle").get()? == 1) {
                ctx.var("mos_nowinter").set(Val::from(11))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(18075), Val::from(18076)])?;
            }
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines(args![
            "Hmm, do you think so...",
            "I am too familiar with both",
            "summer and winter to care much for",
            "one over the other."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Viktor",
        args!["Our people are very, very", "proud of their strength and", "invincible spirits."],
    )?;
    ctx.next()?;
    ctx.lines_as("Viktor", args!["It doesn't matter to us", "how cold the winter is!"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn a_middle_aged_man_mos3(ctx: &Ctx) -> Script {
    a_middle_aged_man_mos3_body(ctx, Vec::new()).map(|_| ())
}

fn a_little_boy_mos4_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Vasili", args!["Wow, he is", "an adventurer, an adventurer!!"])?;
    ctx.next()?;
    if ctx.var("mos_nowinter").get()? == 10 && ctx.var("mos_kid").get()? == 0 {
        ctx.lines_as(
            "Vasili",
            args![
                "Why did you come here?",
                "Don't you have any colleague?",
                "I heard that the heroes in epics were tall,",
                "but, why are you so small?",
                "Where is your gold-shining sword?",
                "I heard that the armor glittered white,",
                "but, why don't you have on the armor?",
                "Won't you go for another adventure?",
                "How can you do it in the cold and snowy winter?"
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("I will go if it gets warm.")])?;
        ctx.var("@menu").set(choice)?;
        ctx.mes("[Vasili]")?;
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(100)])?.number()? > 70 {
            ctx.lines(args![
                "Will you? I will also",
                "go out for adventures",
                "with shining armor and",
                "a long and heavy sword",
                "some day."
            ])?;
            ctx.next()?;
            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_SUMMONSLAVE")?])?;
            ctx.lines_as("Vasili", args!["If winter is gone,", "it is much easier to go on adventures."])?;
            ctx.var("mos_kid").set(Val::from(1))?;
            if (ctx.var("mos_middle").get()? == 1 && ctx.var("mos_elder").get()? == 1) {
                ctx.var("mos_nowinter").set(Val::from(11))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(18075), Val::from(18076)])?;
            }
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines(args!["Ah, when are you leaving?", "Where are you going this time?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Vasili",
            args![
                "Will you go to slay the dragon,",
                "that breathes fire and can",
                "exterminates 10 men by flapping",
                "its wings once?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Vasili",
        args![
            "Tell me your exciting story.",
            "Have you fought a dragon?",
            "Where is your gold-shining sword and shield?",
            "Where?"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn a_little_boy_mos4(ctx: &Ctx) -> Script {
    a_little_boy_mos4_body(ctx, Vec::new()).map(|_| ())
}

fn a_lady_mos5_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("mos_nowinter").get()? == 10 && ctx.var("mos_middle").get()? == 0 {
        let choice = runtime::select_values(ctx, &[Val::from("You look good today.")])?;
        ctx.var("@menu").set(choice)?;
        ctx.mes("[Katya]")?;
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(100)])?.number()? > 70 {
            ctx.lines(args![
                "Yes, I feel good.",
                "When it gets warm, sunflowers bloom.",
                "The sunflower is the symbol",
                "of this province."
            ])?;
            ctx.next()?;
            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_SUMMONSLAVE")?])?;
            ctx.lines_as(
                "Katya",
                args!["I wish that winter never comes back and", "I could see sunflowers everyday."],
            )?;
            ctx.var("mos_middle").set(Val::from(1))?;
            if (ctx.var("mos_kid").get()? == 1 && ctx.var("mos_elder").get()? == 1) {
                ctx.var("mos_nowinter").set(Val::from(11))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(18075), Val::from(18076)])?;
            }
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines(args![
            "Yes, I feel good.",
            "When it gets warm, sunflowers bloom.",
            "The sunflower is the symbol",
            "of this province."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as("Katya", args!["The spring has come~"])?;
    ctx.next()?;
    ctx.lines_as("Katya", args!["A million sunflowers are blooming~"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn a_lady_mos5(ctx: &Ctx) -> Script {
    a_lady_mos5_body(ctx, Vec::new()).map(|_| ())
}

fn a_lady_mos6_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("mos_nowinter").get()? == 10 && ctx.var("mos_elder").get()? == 0 {
        let choice = runtime::select_values(ctx, &[Val::from("The weather is getting warmer.")])?;
        ctx.var("@menu").set(choice)?;
        ctx.mes("[Roza]")?;
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(100)])?.number()? > 70 {
            ctx.lines(args!["Yes, kids like it and", "flowers are blooming."])?;
            ctx.next()?;
            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_SUMMONSLAVE")?])?;
            ctx.lines_as(
                "Roza",
                args!["Whoever likes the cold and dark winter?", "I hope this weather last forever."],
            )?;
            ctx.var("mos_elder").set(Val::from(1))?;
            if (ctx.var("mos_kid").get()? == 1 && ctx.var("mos_middle").get()? == 1) {
                ctx.var("mos_nowinter").set(Val::from(11))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(18075), Val::from(18076)])?;
            }
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines(args![
            "In the warm days,",
            "I'm in trouble.",
            "It's too hard to control",
            "the children."
        ])?;
        ctx.next()?;
        ctx.lines_as("Roza", args!["I think", "I need winter again."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as("Roza", args!["Naughty children get too", "excited in the warm days."])?;
    ctx.next()?;
    ctx.lines_as("Roza", args!["Where is the sun?", "Where is it hiding?"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn a_lady_mos6(ctx: &Ctx) -> Script {
    a_lady_mos6_body(ctx, Vec::new()).map(|_| ())
}

fn a_little_boy_mos7_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("mos_nowinter").get()? == 10 && ctx.var("mos_kid").get()? == 0 {
        let choice = runtime::select_values(ctx, &[Val::from("Do you like mysterious stories?")])?;
        ctx.var("@menu").set(choice)?;
        ctx.mes("[Feliks]")?;
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(100)])?.number()? > 70 {
            ctx.lines(args![
                "Yes, I love them.",
                "But, in winter,",
                "nobody comes out of their homes.",
                "So it's not very fun."
            ])?;
            ctx.next()?;
            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_SUMMONSLAVE")?])?;
            ctx.lines_as("Feliks", args!["If winter never comes", "I can have", "fun all the time..."])?;
            ctx.var("mos_kid").set(Val::from(1))?;
            if (ctx.var("mos_middle").get()? == 1 && ctx.var("mos_elder").get()? == 1) {
                ctx.var("mos_nowinter").set(Val::from(11))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(18075), Val::from(18076)])?;
            }
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines(args![
            "Yes, my Grandma told me",
            "fairy-tales.",
            "The winter is cold, but",
            "it is fun to hear them."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as("Feliks", args!["This is the story", "about a terrible dragon."])?;
    ctx.next()?;
    ctx.lines_as(
        "Feliks",
        args![
            "It is sleeping",
            "in its lair,",
            "but, it destroys everything",
            "around it when it awakes."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Feliks",
        args![
            "This came from my grandma's",
            "grandma's grandma's",
            "grandma's grandma's",
            "grandma's grandma."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn a_little_boy_mos7(ctx: &Ctx) -> Script {
    a_little_boy_mos7_body(ctx, Vec::new()).map(|_| ())
}

fn a_young_man_mos8_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Ilyav",
        args!["I am going to adventure", "to experience new worlds", "as you do."],
    )?;
    if ctx.var("mos_nowinter").get()? == 10 && ctx.var("mos_middle").get()? == 0 {
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("When do you feel good?")])?;
        ctx.var("@menu").set(choice)?;
        ctx.mes("[Ilyav]")?;
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(100)])?.number()? > 70 {
            ctx.lines(args![
                "When the weather is as good as recently",
                "I feel an impulse to adventure",
                "many times a day."
            ])?;
            ctx.next()?;
            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_SUMMONSLAVE")?])?;
            ctx.lines_as(
                "Ilyav",
                args![
                    "But, I don't want to in the cold winter.",
                    "Hu, if the weather everday was as good as lately,",
                    "I would like to go out a lot more.",
                    "I hate winter..."
                ],
            )?;
            ctx.var("mos_middle").set(Val::from(1))?;
            if (ctx.var("mos_kid").get()? == 1 && ctx.var("mos_elder").get()? == 1) {
                ctx.var("mos_nowinter").set(Val::from(11))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(18075), Val::from(18076)])?;
            }
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines(args!["Well...", "I don't know.", "I just am sometimes."])?;
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn a_young_man_mos8(ctx: &Ctx) -> Script {
    a_young_man_mos8_body(ctx, Vec::new()).map(|_| ())
}

fn a_man_mos9_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("mos_nowinter").get()? == 10 && ctx.var("mos_elder").get()? == 0 {
        let choice = runtime::select_values(ctx, &[Val::from("What do you usually do in winter?")])?;
        ctx.var("@menu").set(choice)?;
        ctx.mes("[Orek]")?;
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(100)])?.number()? > 70 {
            ctx.lines(args![
                "It is too cold in winter so I don't go outside.",
                "There's so much snow that it makes it hard to go",
                "around here and there."
            ])?;
            ctx.next()?;
            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_SUMMONSLAVE")?])?;
            ctx.lines_as(
                "Orek",
                args![
                    "In winter, fishing is more difficult,",
                    "anyway, it is bad for us.",
                    "I think it would be ok if we never had winter again."
                ],
            )?;
            ctx.var("mos_elder").set(Val::from(1))?;
            if (ctx.var("mos_kid").get()? == 1 && ctx.var("mos_middle").get()? == 1) {
                ctx.var("mos_nowinter").set(Val::from(11))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(18075), Val::from(18076)])?;
            }
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines(args![
            "It snows a lot in winter and",
            "gets too cold.",
            "So, we don't go outside",
            "without a particular reason."
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Orek",
            args![
                "My family just sits beside the pechka and talks,",
                "hoping that winter passes by soon."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Orek",
        args![
            "The present Csar is a bit strict and",
            "terrible, but",
            "he actually loves",
            "his people."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn a_man_mos9(ctx: &Ctx) -> Script {
    a_man_mos9_body(ctx, Vec::new()).map(|_| ())
}

fn a_lady_mos10_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Kyra", args!["I've seen many people", "from other provinces recently."])?;
    ctx.next()?;
    ctx.lines_as("Kyra", args!["This used to not", "be a tourist town.", "What happened...?"])?;
    if ctx.var("mos_nowinter").get()? == 10 && ctx.var("mos_elder").get()? == 0 {
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Here it is warm and good.")])?;
        ctx.var("@menu").set(choice)?;
        ctx.mes("[Kyra]")?;
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(100)])?.number()? > 70 {
            ctx.lines(args![
                "Hohoho, that may be true now.",
                "But, you don't even know how cold it gets here in winter.",
                "You wouldn't even want to go outside."
            ])?;
            ctx.next()?;
            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_SUMMONSLAVE")?])?;
            ctx.lines_as(
                "Kyra",
                args!["It is good to stay with my family, but", "nobody likes the cold winter."],
            )?;
            ctx.var("mos_elder").set(Val::from(1))?;
            if (ctx.var("mos_kid").get()? == 1 && ctx.var("mos_middle").get()? == 1) {
                ctx.var("mos_nowinter").set(Val::from(11))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(18075), Val::from(18076)])?;
            }
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines(args![
            "Hohoho, that may be true now.",
            "But, you don't even know how cold it gets here in winter.",
            "You wouldn't even want to go outside."
        ])?;
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn a_lady_mos10(ctx: &Ctx) -> Script {
    a_lady_mos10_body(ctx, Vec::new()).map(|_| ())
}

fn a_lady_mos11_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Sabina",
        args![
            "Sunflowers are squeezed for oil and",
            "their bodies are used for medicinal purposes.",
            "They are very useful."
        ],
    )?;
    if ctx.var("mos_nowinter").get()? == 10 && ctx.var("mos_middle").get()? == 0 {
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("When do sunflowers bloom?")])?;
        ctx.var("@menu").set(choice)?;
        ctx.mes("[Sabina]")?;
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(100)])?.number()? > 70 {
            ctx.lines(args![
                "They start to bloom",
                "from the late summer.",
                "You cannot help being",
                "attracted to sunflowers,",
                "if you see the spectacular scene",
                "of a field filled up with them."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Sabina",
                args!["If the summer continues to last,", "I can see them all the time..."],
            )?;
            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_SUMMONSLAVE")?])?;
            ctx.var("mos_middle").set(Val::from(1))?;
            if (ctx.var("mos_kid").get()? == 1 && ctx.var("mos_elder").get()? == 1) {
                ctx.var("mos_nowinter").set(Val::from(11))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(18075), Val::from(18076)])?;
            }
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines(args![
            "They start to bloom",
            "from the late summer.",
            "You cannot help being",
            "attracted to sunflowers,",
            "if you see the spectacular scene",
            "of a field filled up with them."
        ])?;
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn a_lady_mos11(ctx: &Ctx) -> Script {
    a_lady_mos11_body(ctx, Vec::new()).map(|_| ())
}

fn a_young_man_mos12_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Izlof", args!["There is a old saying,", "'an opportunity is a chance.'"])?;
    ctx.next()?;
    ctx.lines_as(
        "Izlof",
        args![
            "It is best to confess",
            "to ladies in the warm weather,",
            "when their minds wander",
            "right now!"
        ],
    )?;
    if ctx.var("mos_nowinter").get()? == 10 && ctx.var("mos_middle").get()? == 0 {
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("It's good to have warm weather.")])?;
        ctx.var("@menu").set(choice)?;
        ctx.mes("[Izlof]")?;
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(100)])?.number()? > 70 {
            ctx.lines(args!["Of course.", "From now on, it is my golden age of opportunity!"])?;
            ctx.next()?;
            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_SUMMONSLAVE")?])?;
            ctx.lines_as(
                "Izlof",
                args![
                    "If it is the summer all the time,",
                    "my life will be in an amorous mood.",
                    "Hahaha!",
                    "I don't want winter to come."
                ],
            )?;
            ctx.var("mos_middle").set(Val::from(1))?;
            if (ctx.var("mos_kid").get()? == 1 && ctx.var("mos_elder").get()? == 1) {
                ctx.var("mos_nowinter").set(Val::from(11))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(18075), Val::from(18076)])?;
            }
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines(args![
            "Of course.",
            "The cold winter has gone.",
            "From now on, it is my golden age!"
        ])?;
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn a_young_man_mos12(ctx: &Ctx) -> Script {
    a_young_man_mos12_body(ctx, Vec::new()).map(|_| ())
}

fn a_man_mos13_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Lev",
        args![
            "I was once like you,",
            "with a hot heart and cool reason,",
            "adventuring everywhere and",
            "coping with all the troubles..."
        ],
    )?;
    if ctx.var("mos_nowinter").get()? == 10 && ctx.var("mos_elder").get()? == 0 {
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("You need to take a rest.")])?;
        ctx.var("@menu").set(choice)?;
        ctx.mes("[Lev]")?;
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(100)])?.number()? > 70 {
            ctx.lines(args!["Yes, under the warm sunlight,", "I like to rest."])?;
            ctx.next()?;
            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_SUMMONSLAVE")?])?;
            ctx.lines_as(
                "Lev",
                args!["I hope that this warm weather", "will last forever.", "I hate the cold winter."],
            )?;
            ctx.var("mos_elder").set(Val::from(1))?;
            if (ctx.var("mos_kid").get()? == 1 && ctx.var("mos_middle").get()? == 1) {
                ctx.var("mos_nowinter").set(Val::from(11))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(18075), Val::from(18076)])?;
            }
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines(args!["Yes, under the warm sunlight,", "I like to rest."])?;
        ctx.next()?;
        ctx.lines_as(
            "Lev",
            args!["But, I am still alive!", "I don't want to hear talk", "like that from you."],
        )?;
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn a_man_mos13(ctx: &Ctx) -> Script {
    a_man_mos13_body(ctx, Vec::new()).map(|_| ())
}

fn a_young_man_mos14_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Fredek",
        args!["The men here grow up", "after going through tough waves in", "the vast sea"],
    )?;
    if ctx.var("mos_nowinter").get()? == 10 && ctx.var("mos_middle").get()? == 0 {
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("However, if winter comes...")])?;
        ctx.var("@menu").set(choice)?;
        ctx.mes("[Fredek]")?;
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(100)])?.number()? > 70 {
            ctx.lines(args!["Yes, in winter the", "sea is frozen, so it", "is impossible to sail."])?;
            ctx.next()?;
            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_SUMMONSLAVE")?])?;
            ctx.lines_as("Fredek", args!["I hope that winter never comes.", "It is my dream."])?;
            ctx.var("mos_middle").set(Val::from(1))?;
            if (ctx.var("mos_kid").get()? == 1 && ctx.var("mos_elder").get()? == 1) {
                ctx.var("mos_nowinter").set(Val::from(11))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(18075), Val::from(18076)])?;
            }
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines(args![
            "Although I won't be able",
            "to sail, my passion is not",
            "cooled down from winter."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.next()?;
    ctx.lines_as("Fredek", args!["Do you like", "sailing?"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn a_young_man_mos14(ctx: &Ctx) -> Script {
    a_young_man_mos14_body(ctx, Vec::new()).map(|_| ())
}

fn a_man_mos15_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Gavrel",
        args![
            "Don't you think that the castle is magnificent?",
            "It was built by my great great grandfather."
        ],
    )?;
    if ctx.var("mos_nowinter").get()? == 10 && ctx.var("mos_elder").get()? == 0 {
        ctx.next()?;
        ctx.lines_as(
            "Gavrel",
            args!["It is very strong and", "it is warm inside of it.", "I am very proud of it."],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Is it cold much in winter?")])?;
        ctx.var("@menu").set(choice)?;
        ctx.mes("[Gavrel]")?;
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(100)])?.number()? > 70 {
            ctx.lines(args![
                "Yes, it is very cold.",
                "If you didn't prepare,",
                "it would be hard for you."
            ])?;
            ctx.next()?;
            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_SUMMONSLAVE")?])?;
            ctx.lines_as(
                "Gavrel",
                args![
                    "It would be a lot better",
                    "if winter never came again.",
                    "But, design of coldness is",
                    "winter itself, isn't it?"
                ],
            )?;
            ctx.var("mos_elder").set(Val::from(1))?;
            if (ctx.var("mos_kid").get()? == 1 && ctx.var("mos_middle").get()? == 1) {
                ctx.var("mos_nowinter").set(Val::from(11))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(18075), Val::from(18076)])?;
            }
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines(args![
            "Yes, it is very cold.",
            "If you didn't prepare,",
            "it would be hard for you."
        ])?;
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn a_man_mos15(ctx: &Ctx) -> Script {
    a_man_mos15_body(ctx, Vec::new()).map(|_| ())
}

fn a_little_boy_mos16_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("mos_nowinter").get()? == 10 && ctx.var("mos_kid").get()? == 0 {
        let choice = runtime::select_values(ctx, &[Val::from("Do you like summer?")])?;
        ctx.var("@menu").set(choice)?;
        ctx.mes("[Rurik]")?;
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(100)])?.number()? > 70 {
            ctx.lines(args!["Ah, move a bit to the side.", "a bit more... Yes.", "Ah, stop there."])?;
            ctx.next()?;
            ctx.lines_as(
                "Rurik",
                args![
                    "I certainly like the summer.",
                    "Unless the sunlight is too strong,",
                    "it is much better than the cold winter."
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_SUMMONSLAVE")?])?;
            ctx.lines_as(
                "Rurik",
                args![
                    "By the way, what is this for?",
                    "It is natural to like the summer.",
                    "Do you think",
                    "the winter should come again?"
                ],
            )?;
            ctx.var("mos_kid").set(Val::from(1))?;
            if (ctx.var("mos_middle").get()? == 1 && ctx.var("mos_elder").get()? == 1) {
                ctx.var("mos_nowinter").set(Val::from(11))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(18075), Val::from(18076)])?;
            }
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines(args![
            "Sure, I like the summer.",
            "Why are you asking",
            "such a question so suddenly?"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Rurik",
        args!["It's hard to look up at you.", "Come lower so I can see your eyes."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Rurik",
        args![
            "Hmm, that's better.",
            "Children are the future.",
            "I won't have a future if I fall",
            "back and hurt my neck",
            "while looking up at you."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn a_little_boy_mos16(ctx: &Ctx) -> Script {
    a_little_boy_mos16_body(ctx, Vec::new()).map(|_| ())
}

fn pile_of_skeletons_magic_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("mos_nowinter").get()? == 8 {
        ctx.lines(args![
            "-There are ugly skulls",
            "all over here but, ",
            "this is the only place that I can stay.-"
        ])?;
        ctx.next()?;
        ctx.lines(args!["-Get closer to see", "more carefully.-"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.mes("-The ugly skulls are gathered.-")?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn pile_of_skeletons_magic(ctx: &Ctx) -> Script {
    pile_of_skeletons_magic_body(ctx, Vec::new()).map(|_| ())
}

fn pile_of_skeletons_magic_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("mos_nowinter").get()? == 8 {
        ctx.lines(args![
            "-She said that the Magic Gourd Bottle that",
            "can hold people's speech",
            "is around here.-"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "This is probably the only place",
                "where the bottle is hidden,",
                "but there are bones all over."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Anyway, why am I here?", "It would be trouble if", "the dragon came back..."],
        )?;
        ctx.next()?;
        'l1: loop {
            if !(true) {
                break 'l1;
            }
            'b1: {
                if Val::from(runtime::select_values(ctx, &[Val::from("Look for it.:I will try next time.")])?) == 1 {
                    if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_THIEF")?) {
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["Here it is.", "That was easy to find."],
                        )?;
                        ctx.var("mos_nowinter").set(Val::from(9))?;
                        ctx.call(Function::GetItem, vec![Val::from(7761), Val::from(1)])?;
                        ctx.call(Function::ChangeQuest, vec![Val::from(18073), Val::from(18074)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    if ctx.call(Function::Rand, vec![Val::from(1), Val::from(20)])?.number()? > 14 {
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["Ah, I got it!", "I better get out of here quickly."],
                        )?;
                        ctx.var("mos_nowinter").set(Val::from(9))?;
                        ctx.call(Function::GetItem, vec![Val::from(7761), Val::from(1)])?;
                        ctx.call(Function::ChangeQuest, vec![Val::from(18073), Val::from(18074)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.mes("............")?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Where the hell is it?!", "It has to be in here somewhere!"],
                    )?;
                    ctx.next()?;
                }
                ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["I will try later."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
    Ok(Val::from(0))
}

pub fn pile_of_skeletons_magic_ontouch(ctx: &Ctx) -> Script {
    pile_of_skeletons_magic_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn irina_edq_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if !(ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])?.is_true()) {
        ctx.lines(args![
            "Wait a moment!!",
            "You have too many items.",
            "You can't receive this.",
            "Lighten your weight and",
            "try again."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Irina",
        args!["Hello, foreign traveler!", "Have you had good day", "in Moscovia?"],
    )?;
    ctx.next()?;
    ctx.lines_as("Irina", args!["A special souvenir for visiting Moscovia...?"])?;
    ctx.next()?;
    ctx.lines_as(
        "Irina",
        args![
            "For you, let's make a Shafka hat.",
            "The Shafka has practicality and",
            "it's spruce! Do you want one?"
        ],
    )?;
    ctx.next()?;
    if Val::from(runtime::select_values(
        ctx,
        &[Val::from("Learn about the Shafka.:Make a Shafka hat.")],
    )?) == 1
    {
        ctx.lines_as(
            "Irina",
            args![
                "When I look at you, you seem to",
                "want to put on something of a thick",
                "fur hat on your head, out in this",
                "temperature. A Shafka!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Irina",
            args![
                "If you've come here for the first",
                "time, you might not know... that",
                "winter here is famous for being so",
                "long and cold in Moscovia."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Irina",
            args![
                "A Shafka hat is especially",
                "necessary to live here during the",
                "cold seasons. Without this hat, you",
                "may not endure a winter!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Irina",
            args![
                "Now, the long, long winter has",
                "ended and the sun is shining... But",
                "someday the winter will come again.",
                "So, for preparation, you should",
                "keep a Shafka hat handy."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Irina",
            args![
                "Don't worry about keeping warm if",
                "you're wearing a Shafka hat. Even",
                "during the coldest weather... You",
                "can roll around in the snow and the",
                "Shafka still keeps you warm!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Irina",
        args![
            "Do you want to make a Shafka hat?",
            "Heheh. Good idea!",
            "That's a good task.",
            "You will be grateful forever."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Irina",
        args![
            "If you bring the materials,",
            "we can make one immediately!",
            "The materials are: ^0000FFNine Tails 20, Yarn 10, Soft Silk 10, Sea-otter Fur 20, Spool 1^000000."
        ],
    )?;
    if ctx.call(Function::IsBeginQuest, vec![Val::from(18121)])? == 0 {
        ctx.call(Function::SetQuest, vec![Val::from(18121)])?;
    }
    ctx.next()?;
    if ((((ctx.call(Function::CountItem, vec![Val::from(1022)])?.number()? > 19
        && ctx.call(Function::CountItem, vec![Val::from(7038)])?.number()? > 9)
        && ctx.call(Function::CountItem, vec![Val::from(7166)])?.number()? > 9)
        && ctx.call(Function::CountItem, vec![Val::from(7065)])?.number()? > 19)
        && ctx.call(Function::CountItem, vec![Val::from(7217)])?.is_true())
    {
        ctx.lines_as(
            "Irina",
            args!["You did well.", "Give me the materials. I will make the Shafka."],
        )?;
        ctx.next()?;
        ctx.call(Function::DelItem, vec![Val::from(1022), Val::from(20)])?;
        ctx.call(Function::DelItem, vec![Val::from(7038), Val::from(10)])?;
        ctx.call(Function::DelItem, vec![Val::from(7166), Val::from(10)])?;
        ctx.call(Function::DelItem, vec![Val::from(7065), Val::from(20)])?;
        ctx.call(Function::DelItem, vec![Val::from(7217), Val::from(1)])?;
        ctx.call(Function::GetItem, vec![Val::from(5243), Val::from(1)])?;
        ctx.call(Function::CompleteQuest, vec![Val::from(18121)])?;
        ctx.lines_as("Irina", args!["Good, I made it. So, how about it?", "Do you like it?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Irina",
            args![
                "If you need a Shafka hat,",
                "come to me whenever,",
                "with the materials.",
                "I will make it."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Irina", args!["Okay?", "Okay, so... Bye-bye!~"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Irina",
        args![
            "Ah... You lack some materials. We",
            "can't make the Shafka hat with just",
            "these materials."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Irina",
        args!["If you bring the right materials, I", "will make a Shafka immediately."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Irina",
        args!["The materials are: ^0000FFNine Tails 20, Yarn 10, Soft Silk 10, Sea-otter Fur 20, Spool 1^000000."],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn irina_edq(ctx: &Ctx) -> Script {
    irina_edq_body(ctx, Vec::new()).map(|_| ())
}

fn sage_rus01_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if !(ctx.var("rhea_rus_main").get()?.is_true()) {
        ctx.lines_as(
            "Sage",
            args![
                "Ah, a foreigner,",
                "you're not from",
                "around here are you?",
                "It's nice to meet you!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Sage", args!["I've heard stories of the many adventurers from your lands. But, I haven't been able to talk with them much because they were always busy."])?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from("Sorry, but I'm busy.....:Ok, let's talk.")],
        )?) == 1
        {
            ctx.lines_as(
                "Sage",
                args!["Ah, you are...", "I just wanted to hear about news from other lands..."],
            )?;
            ctx.next()?;
            ctx.lines_as("Sage", args!["Sorry to bother you. You have a good trip."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Sage",
            args!["I've heard that interesting creatures inhabit your lands. Could you please tell me about at least one of them?"],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Show him a Poring Card")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["This is a card of one of the monsters that inhabit the Midgard continent."],
        )?;
        ctx.next()?;
        if !(ctx.call(Function::CountItem, vec![Val::from(4001)])?.is_true()) {
            ctx.lines_as("Sage", args!["...?!", "What are you talking about?"])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["I mean, this card.............", "..................Ehhh?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Ah.. haha.. where is it?!", "W, wait.. I've got one somewhere!"],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Sage",
            args!["Hooh, it is.. I can't believe this is real.", "Thank you for showing me this."],
        )?;
        ctx.next()?;
        ctx.lines_as("Sage", args!["By the way, do you like this place?"])?;
        ctx.next()?;
        ctx.lines_as("Sage", args!["Now, that winter has passed and summer is coming the weather is great. And I know that winter comes every year, but I always pray that it won't return again."])?;
        ctx.next()?;
        ctx.lines_as("Sage", args!["After the passing of winter, the summer weather has made the forest dense and the air so warm. My favorite season will always be summer."])?;
        ctx.next()?;
        ctx.lines_as(
            "Sage",
            args!["But, don't forget that you should still be careful. Lots of adventurers were lost in the forst in front of us."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Have any of the adventurers returned?"],
        )?;
        ctx.next()?;
        ctx.lines_as("Sage", args!["Only one has come back and told me of a mysterious stone, where the road splits into three, that curses those who touch it so I've been keeping away from it."])?;
        ctx.next()?;
        ctx.lines_as(
            "Sage",
            args!["But, don't worry about me. I know my way around here and am not afraid of monsters, haha."],
        )?;
        ctx.var("rhea_rus_main").set(Val::from(1))?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as("Sage", args!["Now, that winter has passed and summer is coming the weather is great. And I know that winter comes every year, but I always pray that it won't return again."])?;
    ctx.next()?;
    ctx.lines_as("Sage", args!["After the passing of winter, the summer weather has made the forest dense and the air so warm. My favorite season will always be summer."])?;
    ctx.next()?;
    ctx.lines_as(
        "Sage",
        args!["But, don't forget that you should still be careful. Lots of adventurers were lost in the forst in front of us."],
    )?;
    ctx.next()?;
    ctx.lines_as("Sage", args!["Only one has come back and told me of a mysterious stone, where the road splits into three, that curses those who touch it so I've been keeping away from it."])?;
    ctx.next()?;
    ctx.lines_as(
        "Sage",
        args!["Me? Haha, don't worry about me. I know my way around here and am not afraid of monsters, haha."],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn sage_rus01(ctx: &Ctx) -> Script {
    sage_rus01_body(ctx, Vec::new()).map(|_| ())
}

fn mysterious_stone_rus02_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if !(ctx.var("rhea_rus_main").get()?.is_true()) {
        ctx.mes("- The road forks here -")?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("rhea_rus_main").get()?.number()? < 3 {
        ctx.lines(args!["- Words are engraved -", "- on the stone -"])?;
        ctx.next()?;
        ctx.lines(args!["- Right path -> Drain -", "- Left path -> Curse -", "- Middle -> ? -"])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["...Where should I go?"],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Right:Left:Middle")])? {
            1 => {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["...What about the right..."],
                )?;
                ctx.next()?;
                ctx.mes("- You decide to go to the right -")?;
                ctx.next()?;
                ctx.mes("- !!!!!! -")?;
                ctx.call(Function::PercentHeal, vec![Val::from(-50), Val::from(-50)])?;
                ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_DARKBREATH")?])?;
                ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_HIT2")?])?;
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_HUK")?,
                        Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                    ],
                )?;
                ctx.next()?;
                ctx.lines(args!["- An unknown force has -", "- drained your health -"])?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("mosk_dun01"), Val::from(190), Val::from(47)])?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines(args![
                    ((Val::from("[") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("]"))
                ])?;
                if !(ctx.call(Function::CheckRiding, vec![])?.is_true()) {
                    ctx.mes("...What about the left...?")?;
                    ctx.next()?;
                    ctx.mes("- You decide to go to the left -")?;
                    ctx.next()?;
                    ctx.mes("- !!!!!! -")?;
                    ctx.call(
                        Function::Emotion,
                        vec![
                            ctx.constant("ET_HUK")?,
                            Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines(args!["- An unknown force has -", "- cursed your body -"])?;
                } else {
                    ctx.mes("...What about the left...?")?;
                    ctx.next()?;
                    ctx.mes("- You decide to go to the left -")?;
                    ctx.next()?;
                    ctx.mes("- !!!!!! -")?;
                    ctx.call(
                        Function::Emotion,
                        vec![
                            ctx.constant("ET_HUK")?,
                            Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines(args![
                        "- Your trusty Pecopeco senses -",
                        "- an unknown force and tries -",
                        "- to run away!! -"
                    ])?;
                    ctx.call(Function::SetRiding, vec![Val::from(0)])?;
                }
                ctx.call(
                    Function::StartStatus,
                    vec![ctx.constant("SC_CURSE")?, Val::from(60000), Val::from(0)],
                )?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("mosk_dun01"), Val::from(190), Val::from(47)])?;
                return Err(Stop::End);
            }
            3 => {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["...What about the middle...?"],
                )?;
                ctx.next()?;
                ctx.lines(args!["- You move forward -", "- toward the middle -"])?;
                ctx.next()?;
                ctx.mes("- !!!!!! -")?;
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_HUK")?,
                        Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Gray Wolf#rus03::OnEnable")])?;
                ctx.var("rhea_rus_main").set(Val::from(2))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if (ctx.var("rhea_rus_main").get()? == 3 || ctx.var("rhea_rus_main").get()? == 4) {
        ctx.lines(args!["- Words are engraved -", "- on the stone -"])?;
        ctx.next()?;
        ctx.lines(args!["- Right path -> Drain -", "- Left path -> Curse -", "- Middle -> ? -"])?;
        ctx.next()?;
        ctx.lines(args!["- You carefully read the stone -", "- and decide to wait -"])?;
        if ctx.var("rhea_rus_main").get()? == 3 {
            ctx.var("rhea_rus_main").set(Val::from(4))?;
        }
        ctx.call(Function::DoNpcEvent, vec![Val::from("Gray Wolf#rus03::OnEnable")])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn mysterious_stone_rus02(ctx: &Ctx) -> Script {
    mysterious_stone_rus02_body(ctx, Vec::new()).map(|_| ())
}

fn gray_wolf_rus03_run(ctx: &Ctx, mut step: GrayWolfRus03Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            GrayWolfRus03Step::Start => {
                if ctx.var("rhea_rus_main").get()?.number()? < 2 {
                    ctx.lines_as(
                        "Gray Wolf",
                        args!["... Turn back, adventurer...", "This is not where you belong."],
                    )?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("mosk_dun01"), Val::from(190), Val::from(47)])?;
                    return Err(Stop::End);
                } else if ctx.var("rhea_rus_main").get()? == 2 {
                    ctx.lines_as(
                        "Gray Wolf",
                        args!["Halt, adventurer.", "Why did you disregard the warnings on the stone?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Are you the reason adventurers are getting lost around this forest?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Gray Wolf",
                        args!["...lost around this forest.", "He doesn't want anyone to wander around the forest."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["He? Who is he?!"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Gray Wolf",
                        args![
                            "His name is Koshei, the Immortal...",
                            "and he has been freed from his captivity... You'd better leave before he finds you."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Gray Wolf",
                        args!["I'm warning you again!", "Unless you want to be killed, leave now."],
                    )?;
                    ctx.var("rhea_rus_main").set(Val::from(3))?;
                    ctx.call(
                        Function::Emotion,
                        vec![
                            ctx.constant("ET_HUK")?,
                            Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                        ],
                    )?;
                    ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_HIT2")?])?;
                    ctx.call(Function::PercentHeal, vec![Val::from(-50), Val::from(0)])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "- The Gray Wolf attacks you -",
                        "- with it's claws and jumps -",
                        "- into the bushes behind you!! -"
                    ])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Gray Wolf#rus03::OnDisable")])?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("mosk_dun01"), Val::from(190), Val::from(47)])?;
                    return Err(Stop::End);
                } else if ctx.var("rhea_rus_main").get()? == 4 {
                    ctx.lines_as("Gray Wolf", args!["...Why did you come back?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "I am afraid of nothing!",
                            "I'm not afraid of your warning 'cuz I'm not as weak as you think I am!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Your attack from before was a cheapshot! If you have something to do with the missing people around here, you won't be forgiven!", "Come on!!!"])?;
                    ctx.next()?;
                    ctx.lines(args!["- You stand firm and -", "- aim your weapon -", "- at the Gray Wolf -"])?;
                    ctx.next()?;
                    ctx.lines_as("Gray Wolf", args!["Stop it. You cannot beat me."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Let's see who's stronger?!", "Take your position. I won't take you by surprise!!!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Gray Wolf", args!["......You...... Haha.....", "Hahahahaha!!"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Gray Wolf",
                        args![
                            "Yes, you are very brave.",
                            "... Yes, I think that I've been waiting for someone like you."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Gray Wolf", args!["Please accept my apologies."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Gray Wolf",
                        args!["I need brave adventurers, like you, to help me. Can you help me?"],
                    )?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("No, how can I trust you?:Ok, I'll help")],
                    )?) == 1
                    {
                        ctx.lines_as("Gray Wolf", args!["Hu.. you won't. But, I ask you again. Can you help me?"])?;
                        ctx.next()?;
                    }
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "... Ok, ok. There's no way I can refuse this type of request.",
                            "So, what do you want me to do?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Gray Wolf",
                        args!["This isn't a good place to talk. Get on my back, I will show you something."],
                    )?;
                    ctx.next()?;
                    ctx.lines(args!["- He turns his back to me -", "- I jump up and suddenly -"])?;
                    ctx.var("rhea_rus_main").set(Val::from(5))?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Gray Wolf#rus03::OnDisable")])?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("mosk_dun01"), Val::from(45), Val::from(257)])?;
                    return Err(Stop::End);
                }
                step = GrayWolfRus03Step::OnEnable;
                continue 'machine;
            }
            GrayWolfRus03Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Gray Wolf#rus03")])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            GrayWolfRus03Step::OnTimer120000 => {
                step = GrayWolfRus03Step::OnDisable;
                continue 'machine;
            }
            GrayWolfRus03Step::OnDisable => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                step = GrayWolfRus03Step::OnInit;
                continue 'machine;
            }
            GrayWolfRus03Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Gray Wolf#rus03")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn gray_wolf_rus03(ctx: &Ctx) -> Script {
    gray_wolf_rus03_run(ctx, GrayWolfRus03Step::Start, Vec::new()).map(|_| ())
}

pub fn gray_wolf_rus03_onenable(ctx: &Ctx) -> Script {
    gray_wolf_rus03_run(ctx, GrayWolfRus03Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn gray_wolf_rus03_ontimer120000(ctx: &Ctx) -> Script {
    gray_wolf_rus03_run(ctx, GrayWolfRus03Step::OnTimer120000, Vec::new()).map(|_| ())
}

pub fn gray_wolf_rus03_ondisable(ctx: &Ctx) -> Script {
    gray_wolf_rus03_run(ctx, GrayWolfRus03Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn gray_wolf_rus03_oninit(ctx: &Ctx) -> Script {
    gray_wolf_rus03_run(ctx, GrayWolfRus03Step::OnInit, Vec::new()).map(|_| ())
}

use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn dwarf_blacksmith_south_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_count = Val::from(0);
    let mut l_damage = Val::from(0);
    let mut l_n_atk = Val::from(0);
    let mut l_n_def = Val::from(0);
    let mut l_n_vit = Val::from(0);
    let mut l_p_atk = Val::from(0);
    let mut l_p_def = Val::from(0);
    let mut l_p_vit = Val::from(0);
    if runtime::op(&ctx.var("$god3").get()?, "<", &ctx.var("$@god_check1").get()?)?.is_true() {
        ctx.lines_as(
            "Sudri",
            args!["I want to compete", "and fight with stronger and stronger opponents!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sudri",
            args!["One day, I'll return to Svartalfaheimr and defeat Ivaldi! Mwahahaha!"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if runtime::op(&ctx.var("$god4").get()?, ">=", &ctx.var("$@god_check2").get()?)?.is_true() {
            ctx.lines_as("Sudri", args!["Wait, this is", "not the right time", "for fighting..."])?;
            ctx.next()?;
            ctx.lines_as("Sudri", args!["Something that has been hidden is about to be born out of the sweat of determination and tears of sacrifice. We must wait until then."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("god_mjo_0").get()? == 11 {
                ctx.lines_as(
                    "Sudri",
                    args![
                        "I want to compete with a stronger one!",
                        "I will be stronger and stronger,",
                        "one day when I get back to Svartalfaheimr",
                        "I shall defeat Ivaldi!",
                        "Muhahahaha...!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("god_mjo_10").get()? == 10 {
                    ctx.lines_as("Sudri", args!["I must admit that you're a really strong human. Let us fight again when we have a chance. The next time, you may not be so lucky!"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("god_mjo_0").get()? == 1 {
                        if (((ctx.var("god_mjo_1").get()? == 3 || ctx.var("god_mjo_2").get()? == 3) || ctx.var("god_mjo_3").get()? == 3)
                            || ctx.var("god_mjo_4").get()? == 3)
                        {
                            ctx.lines_as("Sudri", args!["Go back to where you belong before I beat you to death!"])?;
                            ctx.next()?;
                            ctx.lines_as("Sudri", args!["All you can gain here are a few herbs."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            if ctx.var("god_mjo_2").get()? == 2 {
                                ctx.lines_as("Sudri", args!["That was a great fight!", "Mwahahaha! I'm satisfied with the results. I may have lost, but we fought honorably with everything we had."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                if (((ctx.var("god_mjo_1").get()? == 0 || ctx.var("god_mjo_1").get()? == 1)
                                    || ctx.var("god_mjo_3").get()? != 0)
                                    || ctx.var("god_mjo_4").get()? != 0)
                                {
                                    ctx.lines_as("Sudri", args!["So...", "What brings", "you here?"])?;
                                    ctx.next()?;
                                    match runtime::select_values(ctx, &[Val::from("Nothing.:Excuse me, sir.")])? {
                                        1 => {
                                            ctx.lines_as("Sudri", args!["You have too much time on your hands. Why don't you log out and hang out with your friends instead?"])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        2 => {
                                            ctx.var("god_mjo_2").set(Val::from(3))?;
                                            ctx.lines_as("Sudri", args!["Why should", "I excuse you?"])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Sudri",
                                                args![
                                                    "If there's",
                                                    "anything I hate,",
                                                    "it's insincerity and",
                                                    "sarcasm. What, you",
                                                    "wanna fight?!"
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        _ => {}
                                    }
                                } else {
                                    if ctx.var("god_mjo_2").get()? == 1 {
                                        l_n_vit = Val::from(200);
                                        l_p_vit = Val::from(100);
                                        'l2: loop {
                                            if !(true) {
                                                break 'l2;
                                            }
                                            'b2: {
                                                ctx.lines(args![
                                                    ((Val::from("Sudri : ") + l_n_vit.clone()) + Val::from(" HP")),
                                                    ((((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                        + Val::from(" : "))
                                                        + l_p_vit.clone())
                                                        + Val::from(" HP")),
                                                    "--------------------",
                                                    ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                        + Val::from("")),
                                                    "initiated",
                                                    "an attack!"
                                                ])?;
                                                ctx.next()?;
                                                match runtime::select_values(
                                                    ctx,
                                                    &[Val::from("...?!:Strike Head!:Strike Chest!:Strike Legs!:Take a break.")],
                                                )? {
                                                    1 => {
                                                        l_p_atk = Val::from(0);
                                                    }
                                                    2 => {
                                                        l_p_atk = Val::from(1);
                                                    }
                                                    3 => {
                                                        l_p_atk = Val::from(2);
                                                    }
                                                    4 => {
                                                        l_p_atk = Val::from(3);
                                                    }
                                                    5 => {
                                                        l_p_atk = Val::from(4);
                                                    }
                                                    _ => {}
                                                }
                                                l_n_def = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                                                l_damage = ctx.call(Function::Rand, vec![Val::from(15), Val::from(25)])?;
                                                if l_p_atk.clone() == 1 {
                                                    ctx.lines(args![
                                                        ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                            + Val::from("")),
                                                        "attacks Sudri's head!"
                                                    ])?;
                                                } else if l_p_atk.clone() == 2 {
                                                    ctx.lines(args![
                                                        ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                            + Val::from("")),
                                                        "strikes Sudri's chest!"
                                                    ])?;
                                                } else if l_p_atk.clone() == 3 {
                                                    ctx.lines(args![
                                                        ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                            + Val::from("")),
                                                        "aims for Sudri's legs!"
                                                    ])?;
                                                } else if l_p_atk.clone() == 4 {
                                                    ctx.lines(args![
                                                        ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                            + Val::from("")),
                                                        "requests a break!"
                                                    ])?;
                                                } else {
                                                    ctx.lines(args![
                                                        ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                            + Val::from("'s")),
                                                        "weak point revealed!"
                                                    ])?;
                                                }
                                                if l_p_atk.clone().loosely_equals(&l_n_def.clone()) {
                                                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_GUARD")?])?;
                                                    if l_n_def.clone() == 1 {
                                                        ctx.lines(args![
                                                            "--------------------",
                                                            "Sudri easily dodges",
                                                            "your attack by twisting",
                                                            "his small, yet svelte, body.",
                                                            "--------------------",
                                                            ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                + Val::from(" misses!"))
                                                        ])?;
                                                    } else if l_n_def.clone() == 2 {
                                                        ctx.lines(args![
                                                            "--------------------",
                                                            "Sudri blocks your",
                                                            "attack by crossing",
                                                            "his stout arms.",
                                                            "--------------------",
                                                            ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                + Val::from("'s attack is blocked!"))
                                                        ])?;
                                                    } else if l_n_def.clone() == 3 {
                                                        ctx.lines(args![
                                                            "--------------------",
                                                            "Sudri dodges your",
                                                            "attack with a graceful",
                                                            "leap to the heavens.",
                                                            "--------------------",
                                                            ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                + Val::from(" misses!"))
                                                        ])?;
                                                    }
                                                } else {
                                                    if l_p_atk.clone() == 4 {
                                                        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_HEAL")?])?;
                                                        l_p_vit = (l_p_vit.clone() + Val::from(10));
                                                        ctx.lines(args![
                                                            "--------------------",
                                                            ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                + Val::from("")),
                                                            "has gained 10 HP!"
                                                        ])?;
                                                    } else if l_p_atk.clone() == 1 {
                                                        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_HIT5")?])?;
                                                        l_n_vit = (l_n_vit.clone().try_sub(l_damage.clone())?);
                                                        ctx.lines(args![
                                                            "--------------------",
                                                            "You successfully hit",
                                                            "Sudri on the head!",
                                                            "--------------------",
                                                            ((Val::from("Sudri has lost ") + l_damage.clone()) + Val::from(" HP!"))
                                                        ])?;
                                                    } else if l_p_atk.clone() == 2 {
                                                        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_HIT2")?])?;
                                                        l_n_vit = (l_n_vit.clone().try_sub(l_damage.clone())?);
                                                        ctx.lines(args![
                                                            "--------------------",
                                                            "You successfully hit",
                                                            "Sudri on the chest!",
                                                            "--------------------",
                                                            "Sudri has",
                                                            ((Val::from("lost ") + l_damage.clone()) + Val::from(" HP!"))
                                                        ])?;
                                                    } else if l_p_atk.clone() == 3 {
                                                        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_HIT4")?])?;
                                                        l_n_vit = (l_n_vit.clone().try_sub(l_damage.clone())?);
                                                        ctx.lines(args![
                                                            "--------------------",
                                                            "You successfully hit",
                                                            "Sudri on the legs!",
                                                            "--------------------",
                                                            "Sudri has",
                                                            ((Val::from("lost ") + l_damage.clone()) + Val::from(" HP!"))
                                                        ])?;
                                                    } else if l_p_atk.clone() == 0 {
                                                        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_HIT5")?])?;
                                                        l_p_vit = (l_p_vit.clone().try_sub(Val::from(10))?);
                                                        ctx.lines(args![
                                                            "--------------------",
                                                            "You were hit by",
                                                            "Sudri's counter attack!",
                                                            "--------------------",
                                                            ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                + Val::from(" has lost 10 HP!"))
                                                        ])?;
                                                        if l_p_vit.clone().number()? < 1 {
                                                            ctx.mes("Defeated...")?;
                                                            ctx.next()?;
                                                            break 'l2;
                                                        }
                                                    } else {
                                                        ctx.lines(args![
                                                            "--------------------",
                                                            "Something happened",
                                                            "and the fight has stopped!"
                                                        ])?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    }
                                                }
                                                if l_n_vit.clone().number()? < 1 {
                                                    ctx.lines(args![
                                                        ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                            + Val::from(" wins!"))
                                                    ])?;
                                                    ctx.next()?;
                                                    break 'l2;
                                                }
                                                ctx.next()?;
                                                ctx.lines(args![
                                                    ((Val::from("Sudri : ") + l_n_vit.clone()) + Val::from(" HP")),
                                                    ((((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                        + Val::from(" : "))
                                                        + l_p_vit.clone())
                                                        + Val::from(" HP")),
                                                    "--------------------",
                                                    "Sudri attacks...!"
                                                ])?;
                                                ctx.next()?;
                                                match runtime::select_values(ctx, &[Val::from("...?!:Dodge!:Block!:Jump!:Counter back!")])?
                                                {
                                                    1 => {
                                                        l_p_def = Val::from(0);
                                                    }
                                                    2 => {
                                                        l_p_def = Val::from(1);
                                                    }
                                                    3 => {
                                                        l_p_def = Val::from(2);
                                                    }
                                                    4 => {
                                                        l_p_def = Val::from(3);
                                                    }
                                                    5 => {
                                                        l_p_def = Val::from(4);
                                                    }
                                                    _ => {}
                                                }
                                                l_n_atk = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                                                l_damage = ctx.call(Function::Rand, vec![Val::from(20), Val::from(25)])?;
                                                if l_n_atk.clone() == 1 {
                                                    ctx.mes("Sudri aims for the head!")?;
                                                } else if l_n_atk.clone() == 2 {
                                                    ctx.mes("Sudri strikes the chest!")?;
                                                } else {
                                                    ctx.mes("Sudri attacks the legs!")?;
                                                }
                                                if l_n_atk.clone().loosely_equals(&l_p_def.clone()) {
                                                    ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_GUARD")?])?;
                                                    if l_p_def.clone() == 1 {
                                                        ctx.lines(args![
                                                            "--------------------",
                                                            ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                + Val::from("")),
                                                            "quickly dodged!",
                                                            "Sudri's arms were",
                                                            "too short and missed!",
                                                            "--------------------",
                                                            "Sudri has failed to attack."
                                                        ])?;
                                                    } else if l_p_def.clone() == 2 {
                                                        ctx.lines(args![
                                                            "--------------------",
                                                            ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                + Val::from("")),
                                                            "barely blocked",
                                                            "Sudri's attack.",
                                                            "--------------------",
                                                            "Sudri has failed to attack."
                                                        ])?;
                                                    } else if l_p_def.clone() == 3 {
                                                        ctx.lines(args![
                                                            "--------------------",
                                                            ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                + Val::from(" jumped,")),
                                                            "dodged Sudri's attack at ease.",
                                                            "--------------------",
                                                            "Sudri has failed to attack."
                                                        ])?;
                                                    }
                                                } else {
                                                    if l_p_def.clone() == 4 {
                                                        l_count = ctx.call(Function::Rand, vec![Val::from(1), Val::from(4)])?;
                                                        ctx.lines(args![
                                                            "--------------------",
                                                            ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                + Val::from("")),
                                                            "counters!"
                                                        ])?;
                                                        if l_count.clone() == 1 {
                                                            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_AUTOCOUNTER")?])?;
                                                            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_MAGNUMBREAK")?])?;
                                                            l_n_vit = (l_n_vit.clone().try_sub(Val::from(20))?);
                                                            ctx.lines(args![
                                                                "You successfully",
                                                                "counter attacked!",
                                                                "--------------------",
                                                                "Sudri has lost 20 HP!"
                                                            ])?;
                                                            if l_n_vit.clone().number()? < 1 {
                                                                ctx.lines(args![
                                                                    ((Val::from("")
                                                                        + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                        + Val::from(" won!"))
                                                                ])?;
                                                                ctx.next()?;
                                                                break 'l2;
                                                            }
                                                        } else {
                                                            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_CRASHEARTH")?])?;
                                                            l_p_vit = (l_p_vit.clone().try_sub(Val::from(30))?);
                                                            ctx.lines(args![
                                                                "You've taken",
                                                                "critical damage",
                                                                "on your weak spot!",
                                                                "--------------------",
                                                                ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                    + Val::from(" has lost 30 HP!"))
                                                            ])?;
                                                        }
                                                    } else if l_n_atk.clone() == 1 {
                                                        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_HIT5")?])?;
                                                        l_p_vit = (l_p_vit.clone().try_sub(l_damage.clone())?);
                                                        ctx.lines(args![
                                                            "--------------------",
                                                            "Sudri successfully",
                                                            ((Val::from("hit ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                + Val::from("")),
                                                            "on the head!",
                                                            "--------------------",
                                                            ((((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                + Val::from(" has lost "))
                                                                + l_damage.clone())
                                                                + Val::from(" HP!"))
                                                        ])?;
                                                    } else if l_n_atk.clone() == 2 {
                                                        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_HIT2")?])?;
                                                        l_p_vit = (l_p_vit.clone().try_sub(l_damage.clone())?);
                                                        ctx.lines(args![
                                                            "--------------------",
                                                            "Sudri successfully",
                                                            ((Val::from("hit ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                + Val::from("")),
                                                            "on the chest!",
                                                            "--------------------",
                                                            ((((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                + Val::from(" has lost "))
                                                                + l_damage.clone())
                                                                + Val::from(" HP!"))
                                                        ])?;
                                                    } else if l_n_atk.clone() == 3 {
                                                        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_HIT4")?])?;
                                                        l_p_vit = (l_p_vit.clone().try_sub(l_damage.clone())?);
                                                        ctx.lines(args![
                                                            "--------------------",
                                                            "Sudri successfully",
                                                            ((Val::from("hit ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                + Val::from("")),
                                                            "on the legs!",
                                                            "--------------------",
                                                            ((((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                + Val::from(" has lost "))
                                                                + l_damage.clone())
                                                                + Val::from(" HP!"))
                                                        ])?;
                                                    } else if l_n_atk.clone() == 0 {
                                                        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_HIT5")?])?;
                                                        l_p_vit = (l_p_vit.clone().try_sub(l_damage.clone())?);
                                                        ctx.lines(args![
                                                            "--------------------",
                                                            "Sudri successfully",
                                                            ((Val::from("hits ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                + Val::from("")),
                                                            "during a moment of",
                                                            "absent-mindedness!",
                                                            "--------------------",
                                                            ((((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                + Val::from(" has lost "))
                                                                + l_damage.clone())
                                                                + Val::from(" HP!"))
                                                        ])?;
                                                    } else {
                                                        ctx.lines(args![
                                                            "--------------------",
                                                            "Something happened",
                                                            "and the fight has stopped!"
                                                        ])?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    }
                                                }
                                                if l_p_vit.clone().number()? < 1 {
                                                    ctx.mes("Sudri won!")?;
                                                    ctx.next()?;
                                                    break 'l2;
                                                }
                                                ctx.next()?;
                                            }
                                        }
                                        if runtime::op(&l_p_vit.clone(), "<", &ctx.var("n_vit").get()?)?.is_true() {
                                            ctx.lines_as(
                                                "Sudri",
                                                args![
                                                    "Muhahahaha!",
                                                    "You're not strong enough to beat me! I want someone who can offer",
                                                    "me a challenge!"
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Sudri",
                                                args!["Go out, train some and get stronger before you even think about coming back!"],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        } else if runtime::op(&l_n_vit.clone(), "<", &ctx.var("p_vit").get()?)?.is_true() {
                                            ctx.var("god_mjo_2").set(Val::from(2))?;
                                            ctx.lines_as(
                                                "Sudri",
                                                args!["You're stronger than me. I never thought I'd meet a human as strong as you."],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as("Sudri", args!["I'm impressed! Alright, I'll tell my friends good things about you. Hopefully, my brothers will give you the help you're looking for."])?;
                                            ctx.next()?;
                                            ctx.lines_as("Sudri", args!["Okay then,", "be safe on", "your travels!"])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                    } else if ctx.var("god_mjo_2").get()? == 0 {
                                        ctx.lines_as("Sudri", args!["So...", "What brings", "you here, human?"])?;
                                        ctx.next()?;
                                        match runtime::select_values(ctx, &[Val::from("Nothing.:Excuse me, sir.")])? {
                                            1 => {
                                                ctx.lines_as("Sudri", args!["You have too much time on your hands. Why don't you log out and hang out with your buddies for a while?"])?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                            2 => {
                                                ctx.lines_as("Sudri", args!["Huh. You're different than other humans. But still, trusting you because you know how to speak respectfully isn't very wise."])?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Sudri",
                                                    args![
                                                        "If there's anything I love,",
                                                        "it's bare knuckle brawling,",
                                                        "old school style."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Sudri",
                                                    args![
                                                        "Words can be deceptive,",
                                                        "but if you can beat me in",
                                                        "a fight, I think I might",
                                                        "just talk to you.",
                                                        "How about it?"
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                match runtime::select_values(
                                                    ctx,
                                                    &[Val::from("...:Yes, I accept your challenge.:No, I'm scared!")],
                                                )? {
                                                    1 => {
                                                        ctx.var("god_mjo_2").set(Val::from(3))?;
                                                        ctx.lines_as("Sudri", args!["You didn't", "even answer me!", "Fine! Whatever!"])?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    }
                                                    2 => {
                                                        ctx.var("god_mjo_2").set(Val::from(1))?;
                                                        ctx.lines_as("Sudri", args!["Ah, I like you already, human! Now why don't you go do some warm ups, and we'll fight when you're ready."])?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    }
                                                    3 => {
                                                        ctx.var("god_mjo_2").set(Val::from(1))?;
                                                        ctx.lines_as(
                                                            "Sudri",
                                                            args![
                                                                "Eh...?",
                                                                "Why are you being",
                                                                "such a coward?",
                                                                "Are you afraid of",
                                                                "this old and tiny Dwarf?"
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Sudri", args!["Come on, I'll even let you have the first hit. Just come to me when you're ready to fight!"])?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    }
                                                    _ => {}
                                                }
                                            }
                                            _ => {}
                                        }
                                    } else {
                                        ctx.lines_as("Sudri", args!["Zzzz Zzzz..."])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                }
                            }
                        }
                    } else {
                        if ctx.var("god_mjo_0").get()? == 2 {
                            if (((ctx.var("god_mjo_1").get()? == 3 || ctx.var("god_mjo_2").get()? == 3)
                                || ctx.var("god_mjo_3").get()? == 3)
                                || ctx.var("god_mjo_4").get()? == 3)
                            {
                                ctx.lines_as(
                                    "Sudri",
                                    args![
                                        "[Sudri]",
                                        "Get out of here before I beat you to death! All you can gain over here is a few herbs, anyway!"
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                if ctx.var("god_mjo_3").get()? == 2 {
                                    ctx.lines_as(
                                        "Sudri",
                                        args![
                                            "That was a great fight!",
                                            "Hahahahahahahahahaah!",
                                            "I regret nothing.",
                                            "I may have lost, but",
                                            "I gave it my all!"
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    if ((((ctx.var("god_mjo_1").get()? == 0 || ctx.var("god_mjo_1").get()? == 1)
                                        || ctx.var("god_mjo_2").get()? == 0)
                                        || ctx.var("god_mjo_2").get()? == 1)
                                        || ctx.var("god_mjo_4").get()? != 0)
                                    {
                                        ctx.lines_as("Sudri", args!["What made you come to me?"])?;
                                        ctx.next()?;
                                        match runtime::select_values(ctx, &[Val::from("Nothing.:Excuse me, sir.")])? {
                                            1 => {
                                                ctx.lines_as(
                                                    "Sudri",
                                                    args![
                                                        "You have too much",
                                                        "time on your hands.",
                                                        "Why don't you log out and go out with your friends instead?"
                                                    ],
                                                )?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                            2 => {
                                                ctx.var("god_mjo_3").set(Val::from(3))?;
                                                ctx.lines_as(
                                                    "Sudri",
                                                    args!["Why should", "I excuse you?", "Do you want to", "fight with me, huh?"],
                                                )?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                            _ => {}
                                        }
                                    } else {
                                        if ctx.var("god_mjo_3").get()? == 1 {
                                            ctx.lines_as("Sudri", args!["Cool, let's fight!"])?;
                                            ctx.next()?;
                                            l_n_vit = Val::from(200);
                                            l_p_vit = Val::from(100);
                                            'l8: loop {
                                                if !(true) {
                                                    break 'l8;
                                                }
                                                'b8: {
                                                    ctx.lines(args![
                                                        ((Val::from("Sudri : ") + l_n_vit.clone()) + Val::from(" HP")),
                                                        ((((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                            + Val::from(" : "))
                                                            + l_p_vit.clone())
                                                            + Val::from(" HP")),
                                                        "--------------------",
                                                        ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                            + Val::from("")),
                                                        "initiated",
                                                        "an attack!"
                                                    ])?;
                                                    ctx.next()?;
                                                    match runtime::select_values(
                                                        ctx,
                                                        &[Val::from("...?!:Strike Head!:Strike Chest!:Strike Legs!:Take a break.")],
                                                    )? {
                                                        1 => {
                                                            l_p_atk = Val::from(0);
                                                        }
                                                        2 => {
                                                            l_p_atk = Val::from(1);
                                                        }
                                                        3 => {
                                                            l_p_atk = Val::from(2);
                                                        }
                                                        4 => {
                                                            l_p_atk = Val::from(3);
                                                        }
                                                        5 => {
                                                            l_p_atk = Val::from(4);
                                                        }
                                                        _ => {}
                                                    }
                                                    l_n_def = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                                                    l_damage = ctx.call(Function::Rand, vec![Val::from(15), Val::from(25)])?;
                                                    if l_p_atk.clone() == 1 {
                                                        ctx.lines(args![
                                                            ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                + Val::from("")),
                                                            "attacks Sudri's head!"
                                                        ])?;
                                                    } else if l_p_atk.clone() == 2 {
                                                        ctx.lines(args![
                                                            ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                + Val::from("")),
                                                            "strikes Sudri's chest!"
                                                        ])?;
                                                    } else if l_p_atk.clone() == 3 {
                                                        ctx.lines(args![
                                                            ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                + Val::from("")),
                                                            "aims for Sudri's legs!"
                                                        ])?;
                                                    } else if l_p_atk.clone() == 4 {
                                                        ctx.lines(args![
                                                            ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                + Val::from("")),
                                                            "requests a break!"
                                                        ])?;
                                                    } else {
                                                        ctx.lines(args![
                                                            ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                + Val::from("'s")),
                                                            "weak point revealed!"
                                                        ])?;
                                                    }
                                                    if l_p_atk.clone().loosely_equals(&l_n_def.clone()) {
                                                        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_GUARD")?])?;
                                                        if l_n_def.clone() == 1 {
                                                            ctx.lines(args![
                                                                "--------------------",
                                                                "Sudri easily dodges",
                                                                "your attack by twisting",
                                                                "his small, yet svelte, body.",
                                                                "--------------------",
                                                                ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                    + Val::from(" misses!"))
                                                            ])?;
                                                        } else if l_n_def.clone() == 2 {
                                                            ctx.lines(args![
                                                                "--------------------",
                                                                "Sudri blocks your",
                                                                "attack by crossing",
                                                                "his stout arms.",
                                                                "--------------------",
                                                                ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                    + Val::from("'s attack is blocked!"))
                                                            ])?;
                                                        } else if l_n_def.clone() == 3 {
                                                            ctx.lines(args![
                                                                "--------------------",
                                                                "Sudri dodges your",
                                                                "attack with a graceful",
                                                                "leap to the heavens.",
                                                                "--------------------",
                                                                ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                    + Val::from(" misses!"))
                                                            ])?;
                                                        }
                                                    } else {
                                                        if l_p_atk.clone() == 4 {
                                                            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_HEAL")?])?;
                                                            l_p_vit = (l_p_vit.clone() + Val::from(10));
                                                            ctx.lines(args![
                                                                "--------------------",
                                                                ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                    + Val::from("")),
                                                                "has gained 10 HP!"
                                                            ])?;
                                                        } else if l_p_atk.clone() == 1 {
                                                            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_HIT5")?])?;
                                                            l_n_vit = (l_n_vit.clone().try_sub(l_damage.clone())?);
                                                            ctx.lines(args![
                                                                "--------------------",
                                                                "You successfully hit",
                                                                "Sudri on the head!",
                                                                "--------------------",
                                                                ((Val::from("Sudri has lost ") + l_damage.clone()) + Val::from(" HP!"))
                                                            ])?;
                                                        } else if l_p_atk.clone() == 2 {
                                                            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_HIT2")?])?;
                                                            l_n_vit = (l_n_vit.clone().try_sub(l_damage.clone())?);
                                                            ctx.lines(args![
                                                                "--------------------",
                                                                "You successfully hit",
                                                                "Sudri on the chest!",
                                                                "--------------------",
                                                                "Sudri has",
                                                                ((Val::from("lost ") + l_damage.clone()) + Val::from(" HP!"))
                                                            ])?;
                                                        } else if l_p_atk.clone() == 3 {
                                                            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_HIT4")?])?;
                                                            l_n_vit = (l_n_vit.clone().try_sub(l_damage.clone())?);
                                                            ctx.lines(args![
                                                                "--------------------",
                                                                "You successfully hit",
                                                                "Sudri on the legs!",
                                                                "--------------------",
                                                                "Sudri has",
                                                                ((Val::from("lost ") + l_damage.clone()) + Val::from(" HP!"))
                                                            ])?;
                                                        } else if l_p_atk.clone() == 0 {
                                                            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_HIT5")?])?;
                                                            l_p_vit = (l_p_vit.clone().try_sub(Val::from(10))?);
                                                            ctx.lines(args![
                                                                "--------------------",
                                                                "You were hit by",
                                                                "Sudri's counter attack!",
                                                                "--------------------",
                                                                ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                    + Val::from(" has lost 10 HP!"))
                                                            ])?;
                                                            if l_p_vit.clone().number()? < 1 {
                                                                ctx.mes("Defeated...")?;
                                                                ctx.next()?;
                                                                break 'l8;
                                                            }
                                                        } else {
                                                            ctx.lines(args![
                                                                "--------------------",
                                                                "Something happened",
                                                                "and the fight has stopped!"
                                                            ])?;
                                                            ctx.close_window()?;
                                                            return Err(Stop::End);
                                                        }
                                                    }
                                                    if l_n_vit.clone().number()? < 1 {
                                                        ctx.lines(args![
                                                            ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                + Val::from(" wins!"))
                                                        ])?;
                                                        ctx.next()?;
                                                        break 'l8;
                                                    }
                                                    ctx.next()?;
                                                    ctx.lines(args![
                                                        ((Val::from("Sudri : ") + l_n_vit.clone()) + Val::from(" HP")),
                                                        ((((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                            + Val::from(" : "))
                                                            + l_p_vit.clone())
                                                            + Val::from(" HP")),
                                                        "--------------------",
                                                        "Sudri attacks...!"
                                                    ])?;
                                                    ctx.next()?;
                                                    match runtime::select_values(
                                                        ctx,
                                                        &[Val::from("...?!:Dodge!:Block!:Jump!:Counter back!")],
                                                    )? {
                                                        1 => {
                                                            l_p_def = Val::from(0);
                                                        }
                                                        2 => {
                                                            l_p_def = Val::from(1);
                                                        }
                                                        3 => {
                                                            l_p_def = Val::from(2);
                                                        }
                                                        4 => {
                                                            l_p_def = Val::from(3);
                                                        }
                                                        5 => {
                                                            l_p_def = Val::from(4);
                                                        }
                                                        _ => {}
                                                    }
                                                    l_n_atk = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                                                    l_damage = ctx.call(Function::Rand, vec![Val::from(20), Val::from(25)])?;
                                                    if l_n_atk.clone() == 1 {
                                                        ctx.mes("Sudri aims for the head!")?;
                                                    } else if l_n_atk.clone() == 2 {
                                                        ctx.mes("Sudri strikes the chest!")?;
                                                    } else {
                                                        ctx.mes("Sudri attacks the legs!")?;
                                                    }
                                                    if l_n_atk.clone().loosely_equals(&l_p_def.clone()) {
                                                        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_GUARD")?])?;
                                                        if l_p_def.clone() == 1 {
                                                            ctx.lines(args![
                                                                "--------------------",
                                                                ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                    + Val::from(" quickly dodged,")),
                                                                "Sudri's arms were too",
                                                                "short to reach at you.",
                                                                "--------------------",
                                                                "Sudri has failed to attack."
                                                            ])?;
                                                        } else if l_p_def.clone() == 2 {
                                                            ctx.lines(args![
                                                                "--------------------",
                                                                ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                    + Val::from("")),
                                                                "barely blocked",
                                                                "Sudri's attack.",
                                                                "--------------------",
                                                                "Sudri has failed to attack."
                                                            ])?;
                                                        } else if l_p_def.clone() == 3 {
                                                            ctx.lines(args![
                                                                "--------------------",
                                                                ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                    + Val::from(" jumped,")),
                                                                "dodged Sudri's attack at ease.",
                                                                "--------------------",
                                                                "Sudri has failed to attack."
                                                            ])?;
                                                        }
                                                    } else {
                                                        if l_p_def.clone() == 4 {
                                                            l_count = ctx.call(Function::Rand, vec![Val::from(1), Val::from(4)])?;
                                                            ctx.lines(args![
                                                                "--------------------",
                                                                ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                    + Val::from("")),
                                                                "counters!"
                                                            ])?;
                                                            if l_count.clone() == 1 {
                                                                ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_AUTOCOUNTER")?])?;
                                                                ctx.call(
                                                                    Function::NpcSpecialEffect,
                                                                    vec![ctx.constant("EF_MAGNUMBREAK")?],
                                                                )?;
                                                                l_n_vit = (l_n_vit.clone().try_sub(Val::from(20))?);
                                                                ctx.lines(args![
                                                                    "You successfully",
                                                                    "counter attacked!",
                                                                    "--------------------",
                                                                    "Sudri has lost 20 HP!"
                                                                ])?;
                                                                if l_n_vit.clone().number()? < 1 {
                                                                    ctx.lines(args![
                                                                        ((Val::from("")
                                                                            + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                            + Val::from(" won!"))
                                                                    ])?;
                                                                    ctx.next()?;
                                                                    break 'l8;
                                                                }
                                                            } else {
                                                                ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_CRASHEARTH")?])?;
                                                                l_p_vit = (l_p_vit.clone().try_sub(Val::from(30))?);
                                                                ctx.lines(args![
                                                                    "You've taken",
                                                                    "critical damage",
                                                                    "on your weak spot!",
                                                                    "--------------------",
                                                                    ((Val::from("")
                                                                        + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                        + Val::from(" has lost 30 HP!"))
                                                                ])?;
                                                            }
                                                        } else if l_n_atk.clone() == 1 {
                                                            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_HIT5")?])?;
                                                            l_p_vit = (l_p_vit.clone().try_sub(l_damage.clone())?);
                                                            ctx.lines(args![
                                                                "--------------------",
                                                                "Sudri successfully",
                                                                ((Val::from("hit ")
                                                                    + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                    + Val::from("")),
                                                                "on the head!",
                                                                "--------------------",
                                                                ((((Val::from("")
                                                                    + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                    + Val::from(" has lost "))
                                                                    + l_damage.clone())
                                                                    + Val::from(" HP!"))
                                                            ])?;
                                                        } else if l_n_atk.clone() == 2 {
                                                            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_HIT2")?])?;
                                                            l_p_vit = (l_p_vit.clone().try_sub(l_damage.clone())?);
                                                            ctx.lines(args![
                                                                "--------------------",
                                                                "Sudri successfully",
                                                                ((Val::from("hit ")
                                                                    + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                    + Val::from("")),
                                                                "on the chest!",
                                                                "--------------------",
                                                                ((((Val::from("")
                                                                    + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                    + Val::from(" has lost "))
                                                                    + l_damage.clone())
                                                                    + Val::from(" HP!"))
                                                            ])?;
                                                        } else if l_n_atk.clone() == 3 {
                                                            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_HIT4")?])?;
                                                            l_p_vit = (l_p_vit.clone().try_sub(l_damage.clone())?);
                                                            ctx.lines(args![
                                                                "--------------------",
                                                                "Sudri successfully",
                                                                ((Val::from("hit ")
                                                                    + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                    + Val::from("")),
                                                                "on the legs!",
                                                                "--------------------",
                                                                ((((Val::from("")
                                                                    + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                    + Val::from(" has lost "))
                                                                    + l_damage.clone())
                                                                    + Val::from(" HP!"))
                                                            ])?;
                                                        } else if l_n_atk.clone() == 0 {
                                                            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_HIT5")?])?;
                                                            l_p_vit = (l_p_vit.clone().try_sub(l_damage.clone())?);
                                                            ctx.lines(args![
                                                                "--------------------",
                                                                "Sudri successfully",
                                                                ((Val::from("hits ")
                                                                    + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                    + Val::from("")),
                                                                "during a moment of",
                                                                "absent-mindedness!",
                                                                "--------------------",
                                                                ((((Val::from("")
                                                                    + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                    + Val::from(" has lost "))
                                                                    + l_damage.clone())
                                                                    + Val::from(" HP!"))
                                                            ])?;
                                                        } else {
                                                            ctx.lines(args![
                                                                "--------------------",
                                                                "Something happened",
                                                                "and the fight has stopped!"
                                                            ])?;
                                                            ctx.close_window()?;
                                                            return Err(Stop::End);
                                                        }
                                                    }
                                                    if l_p_vit.clone().number()? < 1 {
                                                        ctx.mes("Sudri won!")?;
                                                        ctx.next()?;
                                                        break 'l8;
                                                    }
                                                    ctx.next()?;
                                                }
                                            }
                                            if runtime::op(&l_p_vit.clone(), "<", &l_n_vit.clone())?.is_true() {
                                                ctx.lines_as(
                                                    "Sudri",
                                                    args![
                                                        "Muhahahaha!",
                                                        "You're not strong enough to beat me! I want someone who can offer",
                                                        "me a challenge!"
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Sudri",
                                                    args!["Go out, train some and get stronger before you even think about coming back!"],
                                                )?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            } else if runtime::op(&l_n_vit.clone(), "<", &l_p_vit.clone())?.is_true() {
                                                ctx.var("god_mjo_3").set(Val::from(2))?;
                                                ctx.lines_as(
                                                    "Sudri",
                                                    args!["You're stronger than me. I never thought I'd meet a human as strong as you."],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as("Sudri", args!["I'm impressed! Alright, I'll tell my friends good things about you. Hopefully, my brothers will give you the help you're looking for."])?;
                                                ctx.next()?;
                                                ctx.lines_as("Sudri", args!["Okay then,", "be safe on", "your travels!"])?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                        } else if ctx.var("god_mjo_3").get()? == 0 {
                                            ctx.lines_as("Sudri", args!["What made you come to me?"])?;
                                            ctx.next()?;
                                            'b11: {
                                                let subject11 =
                                                    Val::from(runtime::select_values(ctx, &[Val::from("Nothing.:Excuse me, sir.")])?);
                                                let mut matched11 = false;
                                                let no_case11 =
                                                    !subject11.loosely_equals(&Val::from(1)) && !subject11.loosely_equals(&Val::from(2));
                                                if !matched11 && subject11.loosely_equals(&Val::from(1)) {
                                                    matched11 = true;
                                                }
                                                if matched11 {
                                                    ctx.lines_as(
                                                        "Sudri",
                                                        args![
                                                            "You have too much",
                                                            "time on your hands.",
                                                            "Why don't you log out and go out with your friends instead?"
                                                        ],
                                                    )?;
                                                    ctx.close_window()?;
                                                }
                                                if !matched11 && subject11.loosely_equals(&Val::from(2)) {
                                                    matched11 = true;
                                                }
                                                if matched11 {
                                                    ctx.lines_as("Sudri", args!["Huh. You're different than other humans. But still, trusting you because you know how to speak respectfully isn't very wise."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Sudri",
                                                        args![
                                                            "If there's anything I love,",
                                                            "it's bare knuckle brawling,",
                                                            "old school style."
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Sudri",
                                                        args![
                                                            "Words can be deceptive,",
                                                            "but if you can beat me in",
                                                            "a fight, I think I might",
                                                            "just talk to you.",
                                                            "How about it?"
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    match runtime::select_values(
                                                        ctx,
                                                        &[Val::from("...:Yes, I accept your challenge.:No, I'm scared!")],
                                                    )? {
                                                        1 => {
                                                            ctx.var("god_mjo_3").set(Val::from(3))?;
                                                            ctx.lines_as(
                                                                "Sudri",
                                                                args!["You didn't", "even answer me!", "Fine! Whatever!"],
                                                            )?;
                                                            ctx.close_window()?;
                                                            return Err(Stop::End);
                                                        }
                                                        2 => {
                                                            ctx.var("god_mjo_3").set(Val::from(1))?;
                                                            ctx.lines_as("Sudri", args!["Ah, I like you already, human! Now why don't you go do some warm ups, and we'll fight when you're ready."])?;
                                                            ctx.close_window()?;
                                                            return Err(Stop::End);
                                                        }
                                                        3 => {
                                                            ctx.var("god_mjo_3").set(Val::from(1))?;
                                                            ctx.lines_as(
                                                                "Sudri",
                                                                args![
                                                                    "Eh...?",
                                                                    "Why are you being",
                                                                    "such a coward?",
                                                                    "Are you afraid of",
                                                                    "this old and tiny Dwarf?"
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines_as("Sudri", args!["Come on, I'll even let you have the first hit. Just come to me when you're ready to fight!"])?;
                                                            ctx.close_window()?;
                                                            return Err(Stop::End);
                                                        }
                                                        _ => {}
                                                    }
                                                }
                                            }
                                        } else {
                                            ctx.lines_as("Sudri", args!["Zzzz Zzzz..."])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                    }
                                }
                            }
                        } else {
                            if ctx.var("god_mjo_0").get()? == 0 {
                                ctx.lines_as(
                                    "Sudri",
                                    args!["In a one on one fight, you put everything on the line to show your might to your opponent."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Sudri", args!["Have you ever felt the same way I do, human? Win or lose, just giving your all is the most satisfying accomplishment."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                ctx.lines_as("Sudri", args!["Zzzz Zzzz..."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(Val::from(0))
}

pub fn dwarf_blacksmith_south(ctx: &Ctx) -> Script {
    dwarf_blacksmith_south_body(ctx, Vec::new()).map(|_| ())
}

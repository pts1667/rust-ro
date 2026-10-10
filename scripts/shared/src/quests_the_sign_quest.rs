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
enum FSignsealStep {
    Start,
    FSealFail,
    AfterFSealFail,
}

fn any_item_held(ctx: &Ctx, items: &[i32]) -> Result<bool, Stop> {
    for &item in items {
        if ctx.call(Function::CountItem, args![item])?.is_true() {
            return Ok(true);
        }
    }
    Ok(false)
}

fn f_signseal_run(ctx: &Ctx, mut step: FSignsealStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            FSignsealStep::Start => {
                step = FSignsealStep::AfterFSealFail;
                continue 'machine;
            }
            FSignsealStep::FSealFail => {
                ctx.lines(args!["^3355FFYou hit the seal as hard", "as you can with the weapon in your hand. However, the seal is merely shaken by the force of your blow. Perhaps you need something", "more powerful to break the seal...^000000"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            FSignsealStep::AfterFSealFail => {
                let l_r = runtime::arg(&args, 0, Val::from(0));
                if any_item_held(ctx, &[1558, 1963])? {
                    if l_r.number()? < 980 {
                        f_signseal_run(ctx, FSignsealStep::FSealFail, vec![])?;
                    }
                } else if any_item_held(ctx, &[1227, 1228, 1240, 1241, 1962, 1813])? {
                    if l_r.number()? < 960 {
                        f_signseal_run(ctx, FSignsealStep::FSealFail, vec![])?;
                    }
                } else if any_item_held(ctx, &[1719, 1130, 1133, 1223, 1229, 1231, 1413, 1814, 1242])? {
                    if l_r.number()? < 940 {
                        f_signseal_run(ctx, FSignsealStep::FSealFail, vec![])?;
                    }
                } else if any_item_held(ctx, &[1131, 1230, 1232])? {
                    if l_r.number()? < 920 {
                        f_signseal_run(ctx, FSignsealStep::FSealFail, vec![])?;
                    }
                } else if any_item_held(ctx, &[1132, 1134, 1233, 1234, 1235, 1414, 1523, 1236])? {
                    if l_r.number()? < 900 {
                        f_signseal_run(ctx, FSignsealStep::FSealFail, vec![])?;
                    }
                } else if any_item_held(ctx, &[1237, 1524, 1525, 1557, 1415, 1964])? {
                    if l_r.number()? < 880 {
                        f_signseal_run(ctx, FSignsealStep::FSealFail, vec![])?;
                    }
                } else if any_item_held(ctx, &[1135, 1140, 1141, 1527])? {
                    if l_r.number()? < 860 {
                        f_signseal_run(ctx, FSignsealStep::FSealFail, vec![])?;
                    }
                } else if any_item_held(ctx, &[1164, 1165, 1467, 1138, 1139, 1224, 1225, 1416, 1526])? {
                    if l_r.number()? < 840 {
                        f_signseal_run(ctx, FSignsealStep::FSealFail, vec![])?;
                    }
                } else if any_item_held(ctx, &[1305, 1720, 1136, 1137, 1166])? {
                    if l_r.number()? < 820 {
                        f_signseal_run(ctx, FSignsealStep::FSealFail, vec![])?;
                    }
                } else if any_item_held(ctx, &[1261, 1528, 1167])? {
                    if l_r.number()? < 800 {
                        f_signseal_run(ctx, FSignsealStep::FSealFail, vec![])?;
                    }
                } else if any_item_held(ctx, &[1364, 1913])? {
                    if l_r.number()? < 780 {
                        f_signseal_run(ctx, FSignsealStep::FSealFail, vec![])?;
                    }
                } else if any_item_held(ctx, &[1170, 1468, 1168, 1169])? {
                    if l_r.number()? < 760 {
                        f_signseal_run(ctx, FSignsealStep::FSealFail, vec![])?;
                    }
                } else if any_item_held(ctx, &[1365, 1366, 1473])? {
                    if l_r.number()? < 740 {
                        f_signseal_run(ctx, FSignsealStep::FSealFail, vec![])?;
                    }
                } else if any_item_held(ctx, &[1367, 1368, 1466, 1469])? {
                    if l_r.number()? < 720 {
                        f_signseal_run(ctx, FSignsealStep::FSealFail, vec![])?;
                    }
                } else if any_item_held(ctx, &[1369, 1470])? {
                    if l_r.number()? < 700 {
                        f_signseal_run(ctx, FSignsealStep::FSealFail, vec![])?;
                    }
                } else if any_item_held(ctx, &[1722, 1471])? {
                    if l_r.number()? < 680 {
                        f_signseal_run(ctx, FSignsealStep::FSealFail, vec![])?;
                    }
                } else if any_item_held(ctx, &[1363])? {
                    if l_r.number()? < 660 {
                        f_signseal_run(ctx, FSignsealStep::FSealFail, vec![])?;
                    }
                } else if any_item_held(ctx, &[1530])? {
                    if l_r.number()? < 500 {
                        f_signseal_run(ctx, FSignsealStep::FSealFail, vec![])?;
                    }
                } else {
                    ctx.lines(args![
                        "^3355FFThe weapon you're holding",
                        "right now doesn't look like it has any chance of breaking this seal. You'll definitely need something",
                        "more powerful...^000000"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if runtime::arg(&args, 1, Val::from(0)).is_true() {
                    ctx.lines(args![
                        "^3355FFUpon obtaining the last piece",
                        "of Agrboda's soul, all four soul pieces emitted a strange light, rose to the air and combined into",
                        "a single transparent jewel.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFThe jewel floated",
                        "down to your waiting",
                        "hands, and you hear its",
                        "voice speak directly into",
                        "the depths of your heart...^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Agrboda's Soul",
                        args![
                            "^333333I'm...",
                            "I'm leaving my soul",
                            "with you. Please guide",
                            "me to the queen of the dead...^000000"
                        ],
                    )?;
                    ctx.items().take(7306, 3)?;
                    ctx.var("sign_q").set(Val::from(117))?;
                    ctx.items().give(7307, 1)?;
                } else {
                    ctx.lines(args!["^3355FFOnce you strike the seal,", "it cracks open and a flash of mysterious light floods out of it. Inside of the seal, you find a very peculiar object...^000000"])?;
                    ctx.next()?;
                    ctx.lines(args!["^3355FFYou have", "obtained a^6E7B8B", "Piece of Spirit^3355FF.^000000"])?;
                    ctx.items().give(7306, 1)?;
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn f_signseal(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    f_signseal_run(ctx, FSignsealStep::Start, args)
}

pub fn f_updatesignvars(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if !(ctx.var("sign_q").get()?.is_true()) {
        if ctx.var("signquest").get()?.number()? <= 4 {
            ctx.var("sign_q").set(ctx.var("signquest").get()?)?;
        } else if ctx.var("signquest").get()? == 5 {
            ctx.var("sign_q").set(Val::from(14))?;
        } else if ctx.var("signquest").get()? == 6 {
            ctx.var("sign_q").set(Val::from(15))?;
        } else if ctx.var("signquest").get()? == 7 {
            ctx.var("sign_q").set(Val::from(17))?;
        } else if ctx.var("signquest").get()? == 8 {
            ctx.var("sign_q").set(Val::from(19))?;
        } else if ctx.var("signquest").get()? == 9 {
            ctx.var("sign_q").set(Val::from(20))?;
        } else if ctx.var("signquest").get()? == 10 {
            ctx.var("sign_q").set(Val::from(25))?;
        } else if ctx.var("signquest").get()? == 11 {
            ctx.var("sign_q").set(Val::from(27))?;
        } else if ctx.var("signquest").get()? == 12 {
            ctx.var("sign_q").set(Val::from(35))?;
        } else if ctx.var("signquest").get()? == 13 {
            ctx.var("sign_q").set(Val::from(53))?;
        } else if ctx.var("signquest").get()?.number()? >= 14 && ctx.var("signquest").get()?.number()? <= 15 {
            ctx.var("sign_q").set(Val::from(54))?;
        } else if ctx.var("signquest").get()?.number()? >= 16 && ctx.var("signquest").get()?.number()? <= 18 {
            ctx.var("sign_q").set(ctx.var("signquest").get()? + Val::from(39))?;
        } else if ctx.var("signquest").get()? == 19 && ctx.call(Function::CountItem, args![7278])?.is_true() {
            ctx.var("sign_q").set(Val::from(61))?;
        } else if ctx.var("signquest").get()? == 20 {
            ctx.var("sign_q").set(Val::from(65))?;
        } else if ctx.var("signquest").get()? == 21 {
            ctx.var("sign_q").set(Val::from(69))?;
        } else if ctx.var("signquest").get()?.number()? >= 22 && ctx.var("signquest").get()?.number()? <= 28 {
            ctx.var("sign_q").set(ctx.var("signquest").get()? + Val::from(48))?;
        } else if ctx.var("signquest").get()?.number()? >= 29 && ctx.var("signquest").get()?.number()? <= 34 {
            ctx.var("sign_q").set(ctx.var("signquest").get()? + Val::from(49))?;
        } else if ctx.var("signquest").get()? == 35 {
            ctx.var("sign_q").set(Val::from(83))?;
            ctx.var("sign_sq").set(Val::from(1))?;
        } else if ctx.var("signquest").get()? == 36 {
            ctx.var("sign_q").set(Val::from(83))?;
            ctx.var("sign_sq").set(Val::from(1))?;
        } else if ctx.var("signquest").get()? == 37 {
            ctx.var("sign_q").set(Val::from(83))?;
            ctx.var("sign_sq").set(Val::from(2))?;
        } else if ctx.var("signquest").get()? == 38 {
            ctx.var("sign_q").set(Val::from(83))?;
            ctx.var("sign_sq").set(Val::from(3))?;
        } else if ctx.var("signquest").get()? == 39 {
            ctx.var("sign_q").set(Val::from(83))?;
            ctx.var("sign_sq").set(Val::from(5))?;
        } else if ctx.var("signquest").get()? == 40 {
            ctx.var("sign_q").set(Val::from(83))?;
            ctx.var("sign_sq").set(Val::from(6))?;
        } else if ctx.var("signquest").get()? == 41 {
            ctx.var("sign_q").set(Val::from(83))?;
            ctx.var("sign_sq").set(Val::from(7))?;
        } else if ctx.var("signquest").get()? == 42 {
            ctx.var("sign_q").set(Val::from(83))?;
            ctx.var("sign_sq").set(Val::from(8))?;
        } else if ctx.var("signquest").get()?.number()? >= 43 && ctx.var("signquest").get()?.number()? <= 48 {
            ctx.var("sign_q").set(ctx.var("signquest").get()? + Val::from(42))?;
        } else if ctx.var("signquest").get()?.number()? >= 49 && ctx.var("signquest").get()?.number()? <= 52 {
            ctx.var("sign_q").set(ctx.var("signquest").get()? + Val::from(44))?;
        } else if ctx.var("signquest").get()? == 53 {
            ctx.var("sign_q").set(Val::from(100))?;
        } else if ctx.var("signquest").get()? == 54 {
            ctx.var("sign_q").set(Val::from(117))?;
        } else if ctx.var("signquest").get()? == 55 {
            ctx.var("sign_q").set(Val::from(118))?;
        } else if ctx.var("signquest").get()? == 56 {
            ctx.var("sign_q").set(Val::from(127))?;
        } else if ctx.var("signquest").get()? == 57 {
            ctx.var("sign_q").set(Val::from(129))?;
        } else if ctx.var("signquest").get()? == 58 {
            ctx.var("sign_q").set(Val::from(130))?;
        } else if ctx.var("signquest").get()?.number()? >= 59 && ctx.var("signquest").get()?.number()? <= 66 {
            ctx.var("sign_q").set(ctx.var("signquest").get()? + Val::from(78))?;
        }
        if ctx.var("sign_fail").get()? == 1 {
            ctx.var("sign_q").set(Val::from(200))?;
        } else if ctx.var("sign_fail").get()?.number()? >= 2 {
            ctx.var("sign_q").set(Val::from(201))?;
        }
        if ctx.var("ariantest").get()?.number()? >= 2 {
            ctx.var("sign_q").set(ctx.var("ariantest").get()? + Val::from(3))?;
        }
        if ctx.var("gaanantest").get()? == 1 {
            ctx.var("sign_q").set(Val::from(12))?;
        }
        if ctx.var("signjore").get()? == 1 {
            ctx.var("sign_q").set(Val::from(16))?;
        }
        if ctx.var("scarealchsign").get()? == 1 {
            ctx.var("sign_q").set(Val::from(18))?;
        }
        if ctx.var("dearles_test").get()? == 1 {
            ctx.var("sign_q").set(Val::from(28))?;
        } else if ctx.var("dearles_test").get()? == 2 {
            ctx.var("sign_q").set(Val::from(29))?;
        } else if ctx.var("dearles_test").get()? == 3 {
            ctx.var("sign_q").set(Val::from(30))?;
        }
        if ctx.var("signdance").get()? == 1 {
            ctx.var("sign_q").set(Val::from(32))?;
        } else if ctx.var("signdance").get()? == 2 {
            ctx.var("sign_q").set(Val::from(33))?;
        }
        if ctx.var("bakerlan_test").get()?.number()? >= 1 && ctx.var("bakerlan_test").get()?.number()? <= 3 {
            ctx.var("sign_q").set(ctx.var("bakerlan_test").get()? + Val::from(35))?;
        } else if ctx.var("bakerlan_test").get()?.number()? >= 4 && ctx.var("bakerlan_test").get()?.number()? <= 6 {
            ctx.var("sign_q").set(ctx.var("bakerlan_test").get()? + Val::from(36))?;
        } else if ctx.var("bakerlan_test").get()?.number()? >= 7 && ctx.var("bakerlan_test").get()?.number()? <= 13 {
            ctx.var("sign_q").set(ctx.var("bakerlan_test").get()? + Val::from(39))?;
        }
        if ctx.var("signanvil").get()?.number()? >= 1 && ctx.var("signanvil").get()?.number()? <= 4 {
            ctx.var("sign_sq").set(ctx.var("signanvil").get()? + Val::from(57))?;
        }
        if ctx.var("signlaichin").get()?.number()? >= 1 {
            ctx.mes("^FF0000You stumble and drop the your^000000")?;
            if ctx.call(Function::CountItem, args![7306])?.is_true() {
                ctx.lines(args![
                    Val::from("^FF0000") + ctx.call(Function::GetItemName, args![7306])? + Val::from("^000000")
                ])?;
            }
            if ctx.call(Function::CountItem, args![7306])?.is_true() && ctx.call(Function::CountItem, args![7307])?.is_true() {
                ctx.mes("^FF0000and^000000")?;
            }
            if ctx.call(Function::CountItem, args![7307])?.is_true() {
                ctx.lines(args![
                    Val::from("^FF0000") + ctx.call(Function::GetItemName, args![7307])? + Val::from("^000000")
                ])?;
            }
            ctx.mes("^FF0000pieces on the ground and they vanish! Perhaps you should talk to Lachin.^000000")?;
            ctx.call(Function::DelItem, args![7306, ctx.call(Function::CountItem, args![7306])?])?;
            ctx.call(Function::DelItem, args![7307, ctx.call(Function::CountItem, args![7307])?])?;
            ctx.var("sign_q").set(ctx.call(Function::Rand, args![97, 100])?)?;
        }
        if ctx.var("signengelhour").get()?.is_true() {
            ctx.mes("^FF0000You see that Engel has forgotten to look at 'The Sign', how you got it back is a mystery too you, but you should remind Engel that he needs to look at it.^000000")?;
            ctx.items().give(7314, 1)?;
            ctx.var("sign_q").set(Val::from(139))?;
        }
        if ctx.var("sign_branch8a").get()? == 1 {
            ctx.var("sign_q").set(Val::from(119))?;
        } else if ctx.var("sign_branch8a").get()? == 2 {
            ctx.var("sign_q").set(Val::from(120))?;
        } else if ctx.var("sign_branch8a").get()? == 3 {
            ctx.var("sign_q").set(Val::from(122))?;
        } else if ctx.var("sign_branch8a").get()? == 4 {
            ctx.var("sign_q").set(Val::from(124))?;
        } else if ctx.var("sign_branch8a").get()? == 5 {
            ctx.var("sign_q").set(Val::from(134))?;
        } else if ctx.var("sign_branch8a").get()? == 7 || ctx.var("sign_branch8a").get()? == 8 {
            ctx.var("sign_q").set(Val::from(126))?;
        }
        if ctx.var("sign_branch8b").get()?.number()? > 0 {
            ctx.mes("^FF0000Something is wrong, perhaps you should go talk to Serin again.^000000")?;
            ctx.var("sign_q").set(Val::from(132))?;
        }
        if ctx.var("sign_branch2b").get()?.is_true() {
            ctx.var("sign_q").set(Val::from(91))?;
        }
        if ctx.var("signmetzhour").get()?.is_true() {
            ctx.mes("^FF0000You see that Metz has forgotten to look at 'The Sign', how you got it back is a mystery too you, but you should remind Metz that he needs to look at it.^000000")?;
            ctx.items().give(7314, 1)?;
            ctx.var("sign_q").set(Val::from(138))?;
        }
        ctx.var("signquest").set(Val::from(0))?;
        ctx.var("sign_fail").set(Val::from(0))?;
        ctx.var("gaananpoint").set(Val::from(0))?;
        ctx.var("gaanantest").set(Val::from(0))?;
        ctx.var("ariantest").set(Val::from(0))?;
        ctx.var("signjore").set(Val::from(0))?;
        ctx.var("scarealchsign").set(Val::from(0))?;
        ctx.var("dearles_test").set(Val::from(0))?;
        ctx.var("signdance").set(Val::from(0))?;
        ctx.var("bakerlan_test").set(Val::from(0))?;
        ctx.var("signanvil").set(Val::from(0))?;
        ctx.var("signengelhour").set(Val::from(0))?;
        ctx.var("signlaichin").set(Val::from(0))?;
        ctx.var("sign_seal1").set(Val::from(0))?;
        ctx.var("sign_seal2").set(Val::from(0))?;
        ctx.var("sign_seal3").set(Val::from(0))?;
        ctx.var("sign_seal4").set(Val::from(0))?;
        ctx.var("sign_seal1$").set(Val::from(""))?;
        ctx.var("sign_seal2$").set(Val::from(""))?;
        ctx.var("sign_seal3$").set(Val::from(""))?;
        ctx.var("sign_seal4$").set(Val::from(""))?;
        ctx.var("brokenseal").set(Val::from(0))?;
        ctx.var("sign_branch8a").set(Val::from(0))?;
        ctx.var("sign_branch8b").set(Val::from(0))?;
        ctx.var("sign_branch2b").set(Val::from(0))?;
        ctx.var("serinring").set(Val::from(0))?;
        ctx.var("signmetzhour").set(Val::from(0))?;
    }
    return Ok(Val::from(0));
}

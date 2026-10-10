use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum CatPawAgentSplStep {
    Start,
    Catwarp,
    AfterCatwarp,
}

fn cat_paw_agent_spl_run(ctx: &Ctx, mut step: CatPawAgentSplStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            CatPawAgentSplStep::Start => {
                if ctx.var("ep13_yong1").get()?.number()? < 3 {
                    ctx.lines_as(
                        "Cat Paw Agent",
                        args!["Welcome to Cat Trading.", "I guess you're a first-time customer, huh?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Cat Paw Agent",
                        args!["For more details about our contract, you need to talk to our staff first."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if (ctx.var("ep13_yong1").get()?.number()? > 2 && ctx.var("ep13_yong1").get()?.number()? < 20) {
                        ctx.lines_as(
                            "Cat Paw Agent",
                            args![
                                "Cat Trading's available services are as followed.",
                                "For additional services, please consult Agent Gyaruk."
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Save your location:Cancel")])? {
                            1 => {
                                ctx.call(
                                    Function::SavePoint,
                                    vec![Val::from("mid_camp"), Val::from(62), Val::from(127), Val::from(1), Val::from(1)],
                                )?;
                                ctx.lines_as("Cat Paw Agent", args!["Actually, residents of this village continued to offer resistance. So I shall save your location at Midgards Allied Forces Post for your safety."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as("Cat Paw Agent", args!["Thank you for using our service."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    } else {
                        if (ctx.var("ep13_yong1").get()?.number()? > 19 && ctx.var("ep13_yong1").get()?.number()? < 40) {
                            ctx.lines_as(
                                "Cat Paw Agent",
                                args![
                                    "Cat Trading's available services are as followed.",
                                    "For additional services, please consult Agent Gyaruk."
                                ],
                            )?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("Save your location:Use Storage:Cancel")])? {
                                1 => {
                                    ctx.call(
                                        Function::SavePoint,
                                        vec![Val::from("mid_camp"), Val::from(62), Val::from(127), Val::from(1), Val::from(1)],
                                    )?;
                                    ctx.lines_as("Cat Paw Agent", args!["Actually, residents of this village continued to offer resistance. So I shall save your location at Midgards Allied Forces Post for your safety."])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                2 => {
                                    if !(shared::other_global_functions::f_canopenstorage(ctx, vec![])?.is_true()) {
                                        ctx.lines_as(
                                            "Cat Paw Agent",
                                            args![
                                                "I'm sorry, but you",
                                                "need the Novice's",
                                                "Basic Skill Level 6 to",
                                                "use the Storage Service."
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if ctx.var("Zeny").get()?.number()? >= 60 {
                                        ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(60))?))?;
                                        ctx.lines_as("Cat Paw Agent", args!["Thank you.", "Your storage will be opened shortly."])?;
                                        ctx.close_window()?;
                                        ctx.call(Function::OpenStorage, vec![])?;
                                        return Err(Stop::End);
                                    } else {
                                        ctx.lines_as(
                                            "Cat Paw Agent",
                                            args![
                                                "I'm sorry, but you don't",
                                                "have enough money?",
                                                "Cat Trading's storage",
                                                "service is 60 zeny.",
                                                "It's cheap, isn't it?"
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                }
                                3 => {
                                    ctx.lines_as("Cat Paw Agent", args!["Thank you for using our service."])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                _ => {}
                            }
                        } else {
                            if (ctx.var("ep13_yong1").get()?.number()? > 39 && ctx.var("ep13_yong1").get()?.number()? < 100) {
                                ctx.lines_as(
                                    "Cat Paw Agent",
                                    args![
                                        "Cat Trading's available services are as followed.",
                                        "For additional services, please consult Agent Gyaruk."
                                    ],
                                )?;
                                ctx.next()?;
                                'b3: {
                                    let subject3 = Val::from(runtime::select_values(
                                        ctx,
                                        &[Val::from("Save your location:Use Storage:Use Cat Warp (Midgard):Cancel")],
                                    )?);
                                    let mut matched3 = false;
                                    let no_case3 = !subject3.loosely_equals(&Val::from(1))
                                        && !subject3.loosely_equals(&Val::from(2))
                                        && !subject3.loosely_equals(&Val::from(3))
                                        && !subject3.loosely_equals(&Val::from(4));
                                    if !matched3 && subject3.loosely_equals(&Val::from(1)) {
                                        matched3 = true;
                                    }
                                    if matched3 {
                                        ctx.call(
                                            Function::SavePoint,
                                            vec![Val::from("mid_camp"), Val::from(62), Val::from(127), Val::from(1), Val::from(1)],
                                        )?;
                                        ctx.lines_as("Cat Paw Agent", args!["Actually, residents of this village continued to offer resistance. So I shall save your location at Midgards Allied Forces Post for your safety."])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    if !matched3 && subject3.loosely_equals(&Val::from(2)) {
                                        matched3 = true;
                                    }
                                    if matched3 {
                                        if !(shared::other_global_functions::f_canopenstorage(ctx, vec![])?.is_true()) {
                                            ctx.lines_as(
                                                "Cat Paw Agent",
                                                args![
                                                    "I'm sorry, but you",
                                                    "need the Novice's",
                                                    "Basic Skill Level 6 to",
                                                    "use the Storage Service."
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        } else if ctx.var("Zeny").get()?.number()? >= 60 {
                                            ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(60))?))?;
                                            ctx.lines_as("Cat Paw Agent", args!["Thank you.", "Your storage will be opened shortly."])?;
                                            ctx.close_window()?;
                                            ctx.call(Function::OpenStorage, vec![])?;
                                            return Err(Stop::End);
                                        } else {
                                            ctx.lines_as(
                                                "Cat Paw Agent",
                                                args![
                                                    "I'm sorry, but you don't",
                                                    "have enough money?",
                                                    "Cat Trading's storage",
                                                    "service is 60 zeny.",
                                                    "It's cheap, isn't it?"
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                    }
                                    if !matched3 && subject3.loosely_equals(&Val::from(3)) {
                                        matched3 = true;
                                    }
                                    if matched3 {
                                        ctx.lines_as(
                                            "Cat Paw Agent",
                                            args![
                                                "The warp service is only",
                                                "available for customers with",
                                                (ctx.var("ep13_yong1").get()? + Val::from(" or more Cat Trading Points.")),
                                                "Please remember, you can't come back easily once you move to Midgard."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        if (ctx.var("ep13_yong1").get()?.number()? > 39 && ctx.var("ep13_yong1").get()?.number()? < 50) {
                                            'b4: {
                                                let subject4 =
                                                    Val::from(runtime::select_values(ctx, &[Val::from("Prontera -> 5500z:Cancel")])?);
                                                let mut matched4 = false;
                                                let no_case4 =
                                                    !subject4.loosely_equals(&Val::from(1)) && !subject4.loosely_equals(&Val::from(2));
                                                if !matched4 && subject4.loosely_equals(&Val::from(1)) {
                                                    matched4 = true;
                                                }
                                                if matched4 {
                                                    cat_paw_agent_spl_run(
                                                        ctx,
                                                        CatPawAgentSplStep::Catwarp,
                                                        vec![Val::from(5500), Val::from(2)],
                                                    )?;
                                                }
                                                if !matched4 && subject4.loosely_equals(&Val::from(2)) {
                                                    matched4 = true;
                                                }
                                                if matched4 {
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                }
                                            }
                                        } else {
                                            if (ctx.var("ep13_yong1").get()?.number()? > 49 && ctx.var("ep13_yong1").get()?.number()? < 60)
                                            {
                                                'b5: {
                                                    let subject5 = Val::from(runtime::select_values(
                                                        ctx,
                                                        &[Val::from("Alberta -> 5500z:Prontera -> 5500z:Cancel")],
                                                    )?);
                                                    let mut matched5 = false;
                                                    let no_case5 = !subject5.loosely_equals(&Val::from(1))
                                                        && !subject5.loosely_equals(&Val::from(2))
                                                        && !subject5.loosely_equals(&Val::from(3));
                                                    if !matched5 && subject5.loosely_equals(&Val::from(1)) {
                                                        matched5 = true;
                                                    }
                                                    if matched5 {
                                                        cat_paw_agent_spl_run(
                                                            ctx,
                                                            CatPawAgentSplStep::Catwarp,
                                                            vec![Val::from(5500), Val::from(1)],
                                                        )?;
                                                    }
                                                    if !matched5 && subject5.loosely_equals(&Val::from(2)) {
                                                        matched5 = true;
                                                    }
                                                    if matched5 {
                                                        cat_paw_agent_spl_run(
                                                            ctx,
                                                            CatPawAgentSplStep::Catwarp,
                                                            vec![Val::from(5500), Val::from(2)],
                                                        )?;
                                                    }
                                                    if !matched5 && subject5.loosely_equals(&Val::from(3)) {
                                                        matched5 = true;
                                                    }
                                                    if matched5 {
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    }
                                                }
                                            } else {
                                                if (ctx.var("ep13_yong1").get()?.number()? > 59
                                                    && ctx.var("ep13_yong1").get()?.number()? < 70)
                                                {
                                                    'b6: {
                                                        let subject6 = Val::from(runtime::select_values(
                                                            ctx,
                                                            &[Val::from("Alberta -> 5025z:Prontera -> 5025z:Izlude -> 5025z:Cancel")],
                                                        )?);
                                                        let mut matched6 = false;
                                                        let no_case6 = !subject6.loosely_equals(&Val::from(1))
                                                            && !subject6.loosely_equals(&Val::from(2))
                                                            && !subject6.loosely_equals(&Val::from(3))
                                                            && !subject6.loosely_equals(&Val::from(4));
                                                        if !matched6 && subject6.loosely_equals(&Val::from(1)) {
                                                            matched6 = true;
                                                        }
                                                        if matched6 {
                                                            cat_paw_agent_spl_run(
                                                                ctx,
                                                                CatPawAgentSplStep::Catwarp,
                                                                vec![Val::from(5025), Val::from(1)],
                                                            )?;
                                                        }
                                                        if !matched6 && subject6.loosely_equals(&Val::from(2)) {
                                                            matched6 = true;
                                                        }
                                                        if matched6 {
                                                            cat_paw_agent_spl_run(
                                                                ctx,
                                                                CatPawAgentSplStep::Catwarp,
                                                                vec![Val::from(5025), Val::from(2)],
                                                            )?;
                                                        }
                                                        if !matched6 && subject6.loosely_equals(&Val::from(3)) {
                                                            matched6 = true;
                                                        }
                                                        if matched6 {
                                                            cat_paw_agent_spl_run(
                                                                ctx,
                                                                CatPawAgentSplStep::Catwarp,
                                                                vec![Val::from(5025), Val::from(3)],
                                                            )?;
                                                        }
                                                        if !matched6 && subject6.loosely_equals(&Val::from(4)) {
                                                            matched6 = true;
                                                        }
                                                        if matched6 {
                                                            ctx.close_window()?;
                                                            return Err(Stop::End);
                                                        }
                                                    }
                                                } else if (ctx.var("ep13_yong1").get()?.number()? > 69
                                                    && ctx.var("ep13_yong1").get()?.number()? < 80)
                                                {
                                                    'b7: {
                                                        let subject7 = Val::from(runtime::select_values(
                                                            ctx,
                                                            &[Val::from(
                                                                "Alberta -> 5025z:Prontera -> 5025z:Izlude -> 5025z:Geffen -> 5025z:Cancel",
                                                            )],
                                                        )?);
                                                        let mut matched7 = false;
                                                        let no_case7 = !subject7.loosely_equals(&Val::from(1))
                                                            && !subject7.loosely_equals(&Val::from(2))
                                                            && !subject7.loosely_equals(&Val::from(3))
                                                            && !subject7.loosely_equals(&Val::from(4))
                                                            && !subject7.loosely_equals(&Val::from(5));
                                                        if !matched7 && subject7.loosely_equals(&Val::from(1)) {
                                                            matched7 = true;
                                                        }
                                                        if matched7 {
                                                            cat_paw_agent_spl_run(
                                                                ctx,
                                                                CatPawAgentSplStep::Catwarp,
                                                                vec![Val::from(5025), Val::from(1)],
                                                            )?;
                                                        }
                                                        if !matched7 && subject7.loosely_equals(&Val::from(2)) {
                                                            matched7 = true;
                                                        }
                                                        if matched7 {
                                                            cat_paw_agent_spl_run(
                                                                ctx,
                                                                CatPawAgentSplStep::Catwarp,
                                                                vec![Val::from(5025), Val::from(2)],
                                                            )?;
                                                        }
                                                        if !matched7 && subject7.loosely_equals(&Val::from(3)) {
                                                            matched7 = true;
                                                        }
                                                        if matched7 {
                                                            cat_paw_agent_spl_run(
                                                                ctx,
                                                                CatPawAgentSplStep::Catwarp,
                                                                vec![Val::from(5025), Val::from(3)],
                                                            )?;
                                                        }
                                                        if !matched7 && subject7.loosely_equals(&Val::from(4)) {
                                                            matched7 = true;
                                                        }
                                                        if matched7 {
                                                            cat_paw_agent_spl_run(
                                                                ctx,
                                                                CatPawAgentSplStep::Catwarp,
                                                                vec![Val::from(5025), Val::from(4)],
                                                            )?;
                                                        }
                                                        if !matched7 && subject7.loosely_equals(&Val::from(5)) {
                                                            matched7 = true;
                                                        }
                                                        if matched7 {
                                                            ctx.close_window()?;
                                                            return Err(Stop::End);
                                                        }
                                                    }
                                                } else if (ctx.var("ep13_yong1").get()?.number()? > 79
                                                    && ctx.var("ep13_yong1").get()?.number()? < 90)
                                                {
                                                    'b8: {
                                                        let subject8 = Val::from(runtime::select_values(
                                                            ctx,
                                                            &[Val::from(
                                                                "Alberta -> 4765z:Prontera -> 4765z:Izlude -> 4765z:Geffen -> 4765z:Payon -> 4765z:Cancel",
                                                            )],
                                                        )?);
                                                        let mut matched8 = false;
                                                        let no_case8 = !subject8.loosely_equals(&Val::from(1))
                                                            && !subject8.loosely_equals(&Val::from(2))
                                                            && !subject8.loosely_equals(&Val::from(3))
                                                            && !subject8.loosely_equals(&Val::from(4))
                                                            && !subject8.loosely_equals(&Val::from(5))
                                                            && !subject8.loosely_equals(&Val::from(6));
                                                        if !matched8 && subject8.loosely_equals(&Val::from(1)) {
                                                            matched8 = true;
                                                        }
                                                        if matched8 {
                                                            cat_paw_agent_spl_run(
                                                                ctx,
                                                                CatPawAgentSplStep::Catwarp,
                                                                vec![Val::from(4765), Val::from(1)],
                                                            )?;
                                                        }
                                                        if !matched8 && subject8.loosely_equals(&Val::from(2)) {
                                                            matched8 = true;
                                                        }
                                                        if matched8 {
                                                            cat_paw_agent_spl_run(
                                                                ctx,
                                                                CatPawAgentSplStep::Catwarp,
                                                                vec![Val::from(4765), Val::from(2)],
                                                            )?;
                                                        }
                                                        if !matched8 && subject8.loosely_equals(&Val::from(3)) {
                                                            matched8 = true;
                                                        }
                                                        if matched8 {
                                                            cat_paw_agent_spl_run(
                                                                ctx,
                                                                CatPawAgentSplStep::Catwarp,
                                                                vec![Val::from(4765), Val::from(3)],
                                                            )?;
                                                        }
                                                        if !matched8 && subject8.loosely_equals(&Val::from(4)) {
                                                            matched8 = true;
                                                        }
                                                        if matched8 {
                                                            cat_paw_agent_spl_run(
                                                                ctx,
                                                                CatPawAgentSplStep::Catwarp,
                                                                vec![Val::from(4765), Val::from(4)],
                                                            )?;
                                                        }
                                                        if !matched8 && subject8.loosely_equals(&Val::from(5)) {
                                                            matched8 = true;
                                                        }
                                                        if matched8 {
                                                            cat_paw_agent_spl_run(
                                                                ctx,
                                                                CatPawAgentSplStep::Catwarp,
                                                                vec![Val::from(4765), Val::from(5)],
                                                            )?;
                                                        }
                                                        if !matched8 && subject8.loosely_equals(&Val::from(6)) {
                                                            matched8 = true;
                                                        }
                                                        if matched8 {
                                                            ctx.close_window()?;
                                                            return Err(Stop::End);
                                                        }
                                                    }
                                                } else if (ctx.var("ep13_yong1").get()?.number()? > 89
                                                    && ctx.var("ep13_yong1").get()?.number()? < 100)
                                                {
                                                    'b9: {
                                                        let subject9 = Val::from(runtime::select_values(
                                                            ctx,
                                                            &[Val::from(
                                                                "Alberta -> 4765z:Prontera -> 4765z:Izlude -> 4765z:Geffen -> 4765z:Payon -> 4765z:Morocc -> 4765z:Cancel",
                                                            )],
                                                        )?);
                                                        let mut matched9 = false;
                                                        let no_case9 = !subject9.loosely_equals(&Val::from(1))
                                                            && !subject9.loosely_equals(&Val::from(2))
                                                            && !subject9.loosely_equals(&Val::from(3))
                                                            && !subject9.loosely_equals(&Val::from(4))
                                                            && !subject9.loosely_equals(&Val::from(5))
                                                            && !subject9.loosely_equals(&Val::from(6))
                                                            && !subject9.loosely_equals(&Val::from(7));
                                                        if !matched9 && subject9.loosely_equals(&Val::from(1)) {
                                                            matched9 = true;
                                                        }
                                                        if matched9 {
                                                            cat_paw_agent_spl_run(
                                                                ctx,
                                                                CatPawAgentSplStep::Catwarp,
                                                                vec![Val::from(4765), Val::from(1)],
                                                            )?;
                                                        }
                                                        if !matched9 && subject9.loosely_equals(&Val::from(2)) {
                                                            matched9 = true;
                                                        }
                                                        if matched9 {
                                                            cat_paw_agent_spl_run(
                                                                ctx,
                                                                CatPawAgentSplStep::Catwarp,
                                                                vec![Val::from(4765), Val::from(2)],
                                                            )?;
                                                        }
                                                        if !matched9 && subject9.loosely_equals(&Val::from(3)) {
                                                            matched9 = true;
                                                        }
                                                        if matched9 {
                                                            cat_paw_agent_spl_run(
                                                                ctx,
                                                                CatPawAgentSplStep::Catwarp,
                                                                vec![Val::from(4765), Val::from(3)],
                                                            )?;
                                                        }
                                                        if !matched9 && subject9.loosely_equals(&Val::from(4)) {
                                                            matched9 = true;
                                                        }
                                                        if matched9 {
                                                            cat_paw_agent_spl_run(
                                                                ctx,
                                                                CatPawAgentSplStep::Catwarp,
                                                                vec![Val::from(4765), Val::from(4)],
                                                            )?;
                                                        }
                                                        if !matched9 && subject9.loosely_equals(&Val::from(5)) {
                                                            matched9 = true;
                                                        }
                                                        if matched9 {
                                                            cat_paw_agent_spl_run(
                                                                ctx,
                                                                CatPawAgentSplStep::Catwarp,
                                                                vec![Val::from(4765), Val::from(5)],
                                                            )?;
                                                        }
                                                        if !matched9 && subject9.loosely_equals(&Val::from(6)) {
                                                            matched9 = true;
                                                        }
                                                        if matched9 {
                                                            cat_paw_agent_spl_run(
                                                                ctx,
                                                                CatPawAgentSplStep::Catwarp,
                                                                vec![Val::from(4765), Val::from(6)],
                                                            )?;
                                                        }
                                                        if !matched9 && subject9.loosely_equals(&Val::from(7)) {
                                                            matched9 = true;
                                                        }
                                                        if matched9 {
                                                            ctx.close_window()?;
                                                            return Err(Stop::End);
                                                        }
                                                    }
                                                } else if ctx.var("ep13_yong1").get()?.number()? > 99 {
                                                    'b10: {
                                                        let subject10 = Val::from(runtime::select_values(
                                                            ctx,
                                                            &[Val::from(
                                                                "Alberta -> 4590z:Prontera -> 4590z:Izlude -> 4590z:Geffen -> 4590z:Payon -> 4590z:Morocc -> 4590z:Al De Baran -> 4590z:Cancel",
                                                            )],
                                                        )?);
                                                        let mut matched10 = false;
                                                        let no_case10 = !subject10.loosely_equals(&Val::from(1))
                                                            && !subject10.loosely_equals(&Val::from(2))
                                                            && !subject10.loosely_equals(&Val::from(3))
                                                            && !subject10.loosely_equals(&Val::from(4))
                                                            && !subject10.loosely_equals(&Val::from(5))
                                                            && !subject10.loosely_equals(&Val::from(6))
                                                            && !subject10.loosely_equals(&Val::from(7))
                                                            && !subject10.loosely_equals(&Val::from(8));
                                                        if !matched10 && subject10.loosely_equals(&Val::from(1)) {
                                                            matched10 = true;
                                                        }
                                                        if matched10 {
                                                            cat_paw_agent_spl_run(
                                                                ctx,
                                                                CatPawAgentSplStep::Catwarp,
                                                                vec![Val::from(4590), Val::from(1)],
                                                            )?;
                                                        }
                                                        if !matched10 && subject10.loosely_equals(&Val::from(2)) {
                                                            matched10 = true;
                                                        }
                                                        if matched10 {
                                                            cat_paw_agent_spl_run(
                                                                ctx,
                                                                CatPawAgentSplStep::Catwarp,
                                                                vec![Val::from(4590), Val::from(2)],
                                                            )?;
                                                        }
                                                        if !matched10 && subject10.loosely_equals(&Val::from(3)) {
                                                            matched10 = true;
                                                        }
                                                        if matched10 {
                                                            cat_paw_agent_spl_run(
                                                                ctx,
                                                                CatPawAgentSplStep::Catwarp,
                                                                vec![Val::from(4590), Val::from(3)],
                                                            )?;
                                                        }
                                                        if !matched10 && subject10.loosely_equals(&Val::from(4)) {
                                                            matched10 = true;
                                                        }
                                                        if matched10 {
                                                            cat_paw_agent_spl_run(
                                                                ctx,
                                                                CatPawAgentSplStep::Catwarp,
                                                                vec![Val::from(4590), Val::from(4)],
                                                            )?;
                                                        }
                                                        if !matched10 && subject10.loosely_equals(&Val::from(5)) {
                                                            matched10 = true;
                                                        }
                                                        if matched10 {
                                                            cat_paw_agent_spl_run(
                                                                ctx,
                                                                CatPawAgentSplStep::Catwarp,
                                                                vec![Val::from(4590), Val::from(5)],
                                                            )?;
                                                        }
                                                        if !matched10 && subject10.loosely_equals(&Val::from(6)) {
                                                            matched10 = true;
                                                        }
                                                        if matched10 {
                                                            cat_paw_agent_spl_run(
                                                                ctx,
                                                                CatPawAgentSplStep::Catwarp,
                                                                vec![Val::from(4590), Val::from(6)],
                                                            )?;
                                                        }
                                                        if !matched10 && subject10.loosely_equals(&Val::from(7)) {
                                                            matched10 = true;
                                                        }
                                                        if matched10 {
                                                            cat_paw_agent_spl_run(
                                                                ctx,
                                                                CatPawAgentSplStep::Catwarp,
                                                                vec![Val::from(4590), Val::from(7)],
                                                            )?;
                                                        }
                                                        if !matched10 && subject10.loosely_equals(&Val::from(8)) {
                                                            matched10 = true;
                                                        }
                                                        if matched10 {
                                                            ctx.close_window()?;
                                                            return Err(Stop::End);
                                                        }
                                                    }
                                                } else {
                                                    ctx.lines_as(
                                                        "Cat Paw Agent",
                                                        args![
                                                            "I'm sorry, but you're not",
                                                            "eligible to use the warp service.",
                                                            "Please check your points,",
                                                            "and then come back."
                                                        ],
                                                    )?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                }
                                            }
                                        }
                                    }
                                    if !matched3 && subject3.loosely_equals(&Val::from(4)) {
                                        matched3 = true;
                                    }
                                    if matched3 {
                                        ctx.lines_as("Cat Paw Agent", args!["Thank you for using our service."])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                }
                            } else if ctx.var("ep13_yong1").get()?.number()? > 99 {
                                ctx.lines_as(
                                    "Cat Paw Agent",
                                    args![
                                        "Cat Trading's available services are as followed.",
                                        "For additional services, please consult Agent Gyaruk."
                                    ],
                                )?;
                                ctx.next()?;
                                'b11: {
                                    let subject11 = Val::from(runtime::select_values(
                                        ctx,
                                        &[Val::from(
                                            "Save your location:Use Storage:Use Cat Warp (Midgard):Use Cat Warp (Jottunheim):Cancel",
                                        )],
                                    )?);
                                    let mut matched11 = false;
                                    let no_case11 = !subject11.loosely_equals(&Val::from(1))
                                        && !subject11.loosely_equals(&Val::from(2))
                                        && !subject11.loosely_equals(&Val::from(3))
                                        && !subject11.loosely_equals(&Val::from(4))
                                        && !subject11.loosely_equals(&Val::from(5));
                                    if !matched11 && subject11.loosely_equals(&Val::from(1)) {
                                        matched11 = true;
                                    }
                                    if matched11 {
                                        ctx.call(
                                            Function::SavePoint,
                                            vec![Val::from("mid_camp"), Val::from(62), Val::from(127), Val::from(1), Val::from(1)],
                                        )?;
                                        ctx.lines_as("Cat Paw Agent", args!["Actually, residents of this village continued to offer resistance. So I shall save your location at Midgards Allied Forces Post for your safety."])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    if !matched11 && subject11.loosely_equals(&Val::from(2)) {
                                        matched11 = true;
                                    }
                                    if matched11 {
                                        if !(shared::other_global_functions::f_canopenstorage(ctx, vec![])?.is_true()) {
                                            ctx.lines_as(
                                                "Cat Paw Agent",
                                                args![
                                                    "I'm sorry, but you",
                                                    "need the Novice's",
                                                    "Basic Skill Level 6 to",
                                                    "use the Storage Service."
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        } else if ctx.var("Zeny").get()?.number()? >= 60 {
                                            ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(60))?))?;
                                            ctx.lines_as("Cat Paw Agent", args!["Thank you.", "Your storage will be opened shortly."])?;
                                            ctx.close_window()?;
                                            ctx.call(Function::OpenStorage, vec![])?;
                                            return Err(Stop::End);
                                        } else {
                                            ctx.lines_as(
                                                "Cat Paw Agent",
                                                args![
                                                    "I'm sorry, but you don't",
                                                    "have enough money?",
                                                    "Cat Trading's storage",
                                                    "service is 60 zeny.",
                                                    "It's cheap, isn't it?"
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                    }
                                    if !matched11 && subject11.loosely_equals(&Val::from(3)) {
                                        matched11 = true;
                                    }
                                    if matched11 {
                                        ctx.lines_as(
                                            "Cat Paw Agent",
                                            args![
                                                "The warp service is only",
                                                "available for customers with",
                                                (ctx.var("ep13_yong1").get()? + Val::from(" or more Cat Trading Points.")),
                                                "Please remember, you can't come back easily once you move to Midgard."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        if (ctx.var("ep13_yong1").get()?.number()? > 99 && ctx.var("ep13_yong1").get()?.number()? < 200) {
                                            'b12: {
                                                let subject12 = Val::from(runtime::select_values(
                                                    ctx,
                                                    &[Val::from(
                                                        "Alberta -> 4590z:Prontera -> 4590z:Izlude -> 4590z:Geffen -> 4590z:Payon -> 4590z:Morocc -> 4590z:Al De Baran -> 4590z:Cancel",
                                                    )],
                                                )?);
                                                let mut matched12 = false;
                                                let no_case12 = !subject12.loosely_equals(&Val::from(1))
                                                    && !subject12.loosely_equals(&Val::from(2))
                                                    && !subject12.loosely_equals(&Val::from(3))
                                                    && !subject12.loosely_equals(&Val::from(4))
                                                    && !subject12.loosely_equals(&Val::from(5))
                                                    && !subject12.loosely_equals(&Val::from(6))
                                                    && !subject12.loosely_equals(&Val::from(7))
                                                    && !subject12.loosely_equals(&Val::from(8));
                                                if !matched12 && subject12.loosely_equals(&Val::from(1)) {
                                                    matched12 = true;
                                                }
                                                if matched12 {
                                                    cat_paw_agent_spl_run(
                                                        ctx,
                                                        CatPawAgentSplStep::Catwarp,
                                                        vec![Val::from(4590), Val::from(1)],
                                                    )?;
                                                }
                                                if !matched12 && subject12.loosely_equals(&Val::from(2)) {
                                                    matched12 = true;
                                                }
                                                if matched12 {
                                                    cat_paw_agent_spl_run(
                                                        ctx,
                                                        CatPawAgentSplStep::Catwarp,
                                                        vec![Val::from(4590), Val::from(2)],
                                                    )?;
                                                }
                                                if !matched12 && subject12.loosely_equals(&Val::from(3)) {
                                                    matched12 = true;
                                                }
                                                if matched12 {
                                                    cat_paw_agent_spl_run(
                                                        ctx,
                                                        CatPawAgentSplStep::Catwarp,
                                                        vec![Val::from(4590), Val::from(3)],
                                                    )?;
                                                }
                                                if !matched12 && subject12.loosely_equals(&Val::from(4)) {
                                                    matched12 = true;
                                                }
                                                if matched12 {
                                                    cat_paw_agent_spl_run(
                                                        ctx,
                                                        CatPawAgentSplStep::Catwarp,
                                                        vec![Val::from(4590), Val::from(4)],
                                                    )?;
                                                }
                                                if !matched12 && subject12.loosely_equals(&Val::from(5)) {
                                                    matched12 = true;
                                                }
                                                if matched12 {
                                                    cat_paw_agent_spl_run(
                                                        ctx,
                                                        CatPawAgentSplStep::Catwarp,
                                                        vec![Val::from(4590), Val::from(5)],
                                                    )?;
                                                }
                                                if !matched12 && subject12.loosely_equals(&Val::from(6)) {
                                                    matched12 = true;
                                                }
                                                if matched12 {
                                                    cat_paw_agent_spl_run(
                                                        ctx,
                                                        CatPawAgentSplStep::Catwarp,
                                                        vec![Val::from(4590), Val::from(6)],
                                                    )?;
                                                }
                                                if !matched12 && subject12.loosely_equals(&Val::from(7)) {
                                                    matched12 = true;
                                                }
                                                if matched12 {
                                                    cat_paw_agent_spl_run(
                                                        ctx,
                                                        CatPawAgentSplStep::Catwarp,
                                                        vec![Val::from(4590), Val::from(7)],
                                                    )?;
                                                }
                                                if !matched12 && subject12.loosely_equals(&Val::from(8)) {
                                                    matched12 = true;
                                                }
                                                if matched12 {
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                }
                                            }
                                        } else if (ctx.var("ep13_yong1").get()?.number()? > 199
                                            && ctx.var("ep13_yong1").get()?.number()? < 250)
                                        {
                                            'b13: {
                                                let subject13 = Val::from(runtime::select_values(
                                                    ctx,
                                                    &[Val::from(
                                                        "Alberta -> 4170z:Prontera -> 4170z:Izlude -> 4170z:Geffen -> 4170z:Payon -> 4170z:Morocc -> 4170z:Al De Baran -> 4170z:Juno -> 4170z:Cancel",
                                                    )],
                                                )?);
                                                let mut matched13 = false;
                                                let no_case13 = !subject13.loosely_equals(&Val::from(1))
                                                    && !subject13.loosely_equals(&Val::from(2))
                                                    && !subject13.loosely_equals(&Val::from(3))
                                                    && !subject13.loosely_equals(&Val::from(4))
                                                    && !subject13.loosely_equals(&Val::from(5))
                                                    && !subject13.loosely_equals(&Val::from(6))
                                                    && !subject13.loosely_equals(&Val::from(7))
                                                    && !subject13.loosely_equals(&Val::from(8))
                                                    && !subject13.loosely_equals(&Val::from(9));
                                                if !matched13 && subject13.loosely_equals(&Val::from(1)) {
                                                    matched13 = true;
                                                }
                                                if matched13 {
                                                    cat_paw_agent_spl_run(
                                                        ctx,
                                                        CatPawAgentSplStep::Catwarp,
                                                        vec![Val::from(4170), Val::from(1)],
                                                    )?;
                                                }
                                                if !matched13 && subject13.loosely_equals(&Val::from(2)) {
                                                    matched13 = true;
                                                }
                                                if matched13 {
                                                    cat_paw_agent_spl_run(
                                                        ctx,
                                                        CatPawAgentSplStep::Catwarp,
                                                        vec![Val::from(4170), Val::from(2)],
                                                    )?;
                                                }
                                                if !matched13 && subject13.loosely_equals(&Val::from(3)) {
                                                    matched13 = true;
                                                }
                                                if matched13 {
                                                    cat_paw_agent_spl_run(
                                                        ctx,
                                                        CatPawAgentSplStep::Catwarp,
                                                        vec![Val::from(4170), Val::from(3)],
                                                    )?;
                                                }
                                                if !matched13 && subject13.loosely_equals(&Val::from(4)) {
                                                    matched13 = true;
                                                }
                                                if matched13 {
                                                    cat_paw_agent_spl_run(
                                                        ctx,
                                                        CatPawAgentSplStep::Catwarp,
                                                        vec![Val::from(4170), Val::from(4)],
                                                    )?;
                                                }
                                                if !matched13 && subject13.loosely_equals(&Val::from(5)) {
                                                    matched13 = true;
                                                }
                                                if matched13 {
                                                    cat_paw_agent_spl_run(
                                                        ctx,
                                                        CatPawAgentSplStep::Catwarp,
                                                        vec![Val::from(4170), Val::from(5)],
                                                    )?;
                                                }
                                                if !matched13 && subject13.loosely_equals(&Val::from(6)) {
                                                    matched13 = true;
                                                }
                                                if matched13 {
                                                    cat_paw_agent_spl_run(
                                                        ctx,
                                                        CatPawAgentSplStep::Catwarp,
                                                        vec![Val::from(4170), Val::from(6)],
                                                    )?;
                                                }
                                                if !matched13 && subject13.loosely_equals(&Val::from(7)) {
                                                    matched13 = true;
                                                }
                                                if matched13 {
                                                    cat_paw_agent_spl_run(
                                                        ctx,
                                                        CatPawAgentSplStep::Catwarp,
                                                        vec![Val::from(4170), Val::from(7)],
                                                    )?;
                                                }
                                                if !matched13 && subject13.loosely_equals(&Val::from(8)) {
                                                    matched13 = true;
                                                }
                                                if matched13 {
                                                    cat_paw_agent_spl_run(
                                                        ctx,
                                                        CatPawAgentSplStep::Catwarp,
                                                        vec![Val::from(4170), Val::from(8)],
                                                    )?;
                                                }
                                                if !matched13 && subject13.loosely_equals(&Val::from(9)) {
                                                    matched13 = true;
                                                }
                                                if matched13 {
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                }
                                            }
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        } else if (ctx.var("ep13_yong1").get()?.number()? > 249
                                            && ctx.var("ep13_yong1").get()?.number()? < 300)
                                        {
                                            'b14: {
                                                let subject14 = Val::from(runtime::select_values(
                                                    ctx,
                                                    &[Val::from(
                                                        "Alberta -> 4025z:Prontera -> 4025z:Izlude -> 4025z:Geffen -> 4025z:Payon -> 4025z:Morocc -> 4025z:Al De Baran -> 4025z:Juno -> 4025z:Einbroch -> 4025z:Cancel",
                                                    )],
                                                )?);
                                                let mut matched14 = false;
                                                let no_case14 = !subject14.loosely_equals(&Val::from(1))
                                                    && !subject14.loosely_equals(&Val::from(2))
                                                    && !subject14.loosely_equals(&Val::from(3))
                                                    && !subject14.loosely_equals(&Val::from(4))
                                                    && !subject14.loosely_equals(&Val::from(5))
                                                    && !subject14.loosely_equals(&Val::from(6))
                                                    && !subject14.loosely_equals(&Val::from(7))
                                                    && !subject14.loosely_equals(&Val::from(8))
                                                    && !subject14.loosely_equals(&Val::from(9))
                                                    && !subject14.loosely_equals(&Val::from(10));
                                                if !matched14 && subject14.loosely_equals(&Val::from(1)) {
                                                    matched14 = true;
                                                }
                                                if matched14 {
                                                    cat_paw_agent_spl_run(
                                                        ctx,
                                                        CatPawAgentSplStep::Catwarp,
                                                        vec![Val::from(4025), Val::from(1)],
                                                    )?;
                                                }
                                                if !matched14 && subject14.loosely_equals(&Val::from(2)) {
                                                    matched14 = true;
                                                }
                                                if matched14 {
                                                    cat_paw_agent_spl_run(
                                                        ctx,
                                                        CatPawAgentSplStep::Catwarp,
                                                        vec![Val::from(4025), Val::from(2)],
                                                    )?;
                                                }
                                                if !matched14 && subject14.loosely_equals(&Val::from(3)) {
                                                    matched14 = true;
                                                }
                                                if matched14 {
                                                    cat_paw_agent_spl_run(
                                                        ctx,
                                                        CatPawAgentSplStep::Catwarp,
                                                        vec![Val::from(4025), Val::from(3)],
                                                    )?;
                                                }
                                                if !matched14 && subject14.loosely_equals(&Val::from(4)) {
                                                    matched14 = true;
                                                }
                                                if matched14 {
                                                    cat_paw_agent_spl_run(
                                                        ctx,
                                                        CatPawAgentSplStep::Catwarp,
                                                        vec![Val::from(4025), Val::from(4)],
                                                    )?;
                                                }
                                                if !matched14 && subject14.loosely_equals(&Val::from(5)) {
                                                    matched14 = true;
                                                }
                                                if matched14 {
                                                    cat_paw_agent_spl_run(
                                                        ctx,
                                                        CatPawAgentSplStep::Catwarp,
                                                        vec![Val::from(4025), Val::from(5)],
                                                    )?;
                                                }
                                                if !matched14 && subject14.loosely_equals(&Val::from(6)) {
                                                    matched14 = true;
                                                }
                                                if matched14 {
                                                    cat_paw_agent_spl_run(
                                                        ctx,
                                                        CatPawAgentSplStep::Catwarp,
                                                        vec![Val::from(4025), Val::from(6)],
                                                    )?;
                                                }
                                                if !matched14 && subject14.loosely_equals(&Val::from(7)) {
                                                    matched14 = true;
                                                }
                                                if matched14 {
                                                    cat_paw_agent_spl_run(
                                                        ctx,
                                                        CatPawAgentSplStep::Catwarp,
                                                        vec![Val::from(4025), Val::from(7)],
                                                    )?;
                                                }
                                                if !matched14 && subject14.loosely_equals(&Val::from(8)) {
                                                    matched14 = true;
                                                }
                                                if matched14 {
                                                    cat_paw_agent_spl_run(
                                                        ctx,
                                                        CatPawAgentSplStep::Catwarp,
                                                        vec![Val::from(4025), Val::from(8)],
                                                    )?;
                                                }
                                                if !matched14 && subject14.loosely_equals(&Val::from(9)) {
                                                    matched14 = true;
                                                }
                                                if matched14 {
                                                    cat_paw_agent_spl_run(
                                                        ctx,
                                                        CatPawAgentSplStep::Catwarp,
                                                        vec![Val::from(4025), Val::from(9)],
                                                    )?;
                                                }
                                                if !matched14 && subject14.loosely_equals(&Val::from(10)) {
                                                    matched14 = true;
                                                }
                                                if matched14 {
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                }
                                            }
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        } else if ctx.var("ep13_yong1").get()?.number()? > 299 {
                                            'b15: {
                                                let subject15 = Val::from(runtime::select_values(
                                                    ctx,
                                                    &[Val::from(
                                                        "Alberta -> 3970z:Prontera -> 3970z:Izlude -> 3970z:Geffen -> 3970z:Payon -> 3970z:Morocc -> 3970z:Al De Baran -> 3970z:Juno -> 3970z:Einbroch -> 3970z:Lighthalzen -> 3970z:Cancel",
                                                    )],
                                                )?);
                                                let mut matched15 = false;
                                                let no_case15 = !subject15.loosely_equals(&Val::from(1))
                                                    && !subject15.loosely_equals(&Val::from(2))
                                                    && !subject15.loosely_equals(&Val::from(3))
                                                    && !subject15.loosely_equals(&Val::from(4))
                                                    && !subject15.loosely_equals(&Val::from(5))
                                                    && !subject15.loosely_equals(&Val::from(6))
                                                    && !subject15.loosely_equals(&Val::from(7))
                                                    && !subject15.loosely_equals(&Val::from(8))
                                                    && !subject15.loosely_equals(&Val::from(9))
                                                    && !subject15.loosely_equals(&Val::from(10))
                                                    && !subject15.loosely_equals(&Val::from(11));
                                                if !matched15 && subject15.loosely_equals(&Val::from(1)) {
                                                    matched15 = true;
                                                }
                                                if matched15 {
                                                    cat_paw_agent_spl_run(
                                                        ctx,
                                                        CatPawAgentSplStep::Catwarp,
                                                        vec![Val::from(3970), Val::from(1)],
                                                    )?;
                                                }
                                                if !matched15 && subject15.loosely_equals(&Val::from(2)) {
                                                    matched15 = true;
                                                }
                                                if matched15 {
                                                    cat_paw_agent_spl_run(
                                                        ctx,
                                                        CatPawAgentSplStep::Catwarp,
                                                        vec![Val::from(3970), Val::from(2)],
                                                    )?;
                                                }
                                                if !matched15 && subject15.loosely_equals(&Val::from(3)) {
                                                    matched15 = true;
                                                }
                                                if matched15 {
                                                    cat_paw_agent_spl_run(
                                                        ctx,
                                                        CatPawAgentSplStep::Catwarp,
                                                        vec![Val::from(3970), Val::from(3)],
                                                    )?;
                                                }
                                                if !matched15 && subject15.loosely_equals(&Val::from(4)) {
                                                    matched15 = true;
                                                }
                                                if matched15 {
                                                    cat_paw_agent_spl_run(
                                                        ctx,
                                                        CatPawAgentSplStep::Catwarp,
                                                        vec![Val::from(3970), Val::from(4)],
                                                    )?;
                                                }
                                                if !matched15 && subject15.loosely_equals(&Val::from(5)) {
                                                    matched15 = true;
                                                }
                                                if matched15 {
                                                    cat_paw_agent_spl_run(
                                                        ctx,
                                                        CatPawAgentSplStep::Catwarp,
                                                        vec![Val::from(3970), Val::from(5)],
                                                    )?;
                                                }
                                                if !matched15 && subject15.loosely_equals(&Val::from(6)) {
                                                    matched15 = true;
                                                }
                                                if matched15 {
                                                    cat_paw_agent_spl_run(
                                                        ctx,
                                                        CatPawAgentSplStep::Catwarp,
                                                        vec![Val::from(3970), Val::from(6)],
                                                    )?;
                                                }
                                                if !matched15 && subject15.loosely_equals(&Val::from(7)) {
                                                    matched15 = true;
                                                }
                                                if matched15 {
                                                    cat_paw_agent_spl_run(
                                                        ctx,
                                                        CatPawAgentSplStep::Catwarp,
                                                        vec![Val::from(3970), Val::from(7)],
                                                    )?;
                                                }
                                                if !matched15 && subject15.loosely_equals(&Val::from(8)) {
                                                    matched15 = true;
                                                }
                                                if matched15 {
                                                    cat_paw_agent_spl_run(
                                                        ctx,
                                                        CatPawAgentSplStep::Catwarp,
                                                        vec![Val::from(3970), Val::from(8)],
                                                    )?;
                                                }
                                                if !matched15 && subject15.loosely_equals(&Val::from(9)) {
                                                    matched15 = true;
                                                }
                                                if matched15 {
                                                    cat_paw_agent_spl_run(
                                                        ctx,
                                                        CatPawAgentSplStep::Catwarp,
                                                        vec![Val::from(3970), Val::from(9)],
                                                    )?;
                                                }
                                                if !matched15 && subject15.loosely_equals(&Val::from(10)) {
                                                    matched15 = true;
                                                }
                                                if matched15 {
                                                    cat_paw_agent_spl_run(
                                                        ctx,
                                                        CatPawAgentSplStep::Catwarp,
                                                        vec![Val::from(3970), Val::from(10)],
                                                    )?;
                                                }
                                                if !matched15 && subject15.loosely_equals(&Val::from(11)) {
                                                    matched15 = true;
                                                }
                                                if matched15 {
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                }
                                            }
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        } else {
                                            ctx.lines_as(
                                                "Cat Paw Agent",
                                                args![
                                                    "I'm sorry, but you're not",
                                                    "eligible to use the warp service.",
                                                    "Please check your points,",
                                                    "and then come back."
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                    }
                                    if !matched11 && subject11.loosely_equals(&Val::from(4)) {
                                        matched11 = true;
                                    }
                                    if matched11 {
                                        if ctx.call(Function::StrNpcInfo, vec![Val::from(2)])? == "spl" {
                                            'b16: {
                                                let subject16 = Val::from(runtime::select_values(
                                                    ctx,
                                                    &[Val::from("Alliance Forces Post -> 5500z:Manuk Camp -> 7500z:Cancel")],
                                                )?);
                                                let mut matched16 = false;
                                                let no_case16 = !subject16.loosely_equals(&Val::from(1))
                                                    && !subject16.loosely_equals(&Val::from(2))
                                                    && !subject16.loosely_equals(&Val::from(3));
                                                if !matched16 && subject16.loosely_equals(&Val::from(1)) {
                                                    matched16 = true;
                                                }
                                                if matched16 {
                                                    cat_paw_agent_spl_run(
                                                        ctx,
                                                        CatPawAgentSplStep::Catwarp,
                                                        vec![Val::from(5500), Val::from(13)],
                                                    )?;
                                                }
                                                if !matched16 && subject16.loosely_equals(&Val::from(2)) {
                                                    matched16 = true;
                                                }
                                                if matched16 {
                                                    cat_paw_agent_spl_run(
                                                        ctx,
                                                        CatPawAgentSplStep::Catwarp,
                                                        vec![Val::from(5500), Val::from(12)],
                                                    )?;
                                                }
                                                if !matched16 && subject16.loosely_equals(&Val::from(3)) {
                                                    matched16 = true;
                                                }
                                                if matched16 {
                                                    ctx.lines_as("Cat Paw Agent", args!["Thank you for using our service."])?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                }
                                            }
                                        } else {
                                            'b17: {
                                                let subject17 = Val::from(runtime::select_values(
                                                    ctx,
                                                    &[Val::from("Alliance Forces Post -> 5500z:Splendide Camp -> 7500z:Cancel")],
                                                )?);
                                                let mut matched17 = false;
                                                let no_case17 = !subject17.loosely_equals(&Val::from(1))
                                                    && !subject17.loosely_equals(&Val::from(2))
                                                    && !subject17.loosely_equals(&Val::from(3));
                                                if !matched17 && subject17.loosely_equals(&Val::from(1)) {
                                                    matched17 = true;
                                                }
                                                if matched17 {
                                                    cat_paw_agent_spl_run(
                                                        ctx,
                                                        CatPawAgentSplStep::Catwarp,
                                                        vec![Val::from(5500), Val::from(13)],
                                                    )?;
                                                }
                                                if !matched17 && subject17.loosely_equals(&Val::from(2)) {
                                                    matched17 = true;
                                                }
                                                if matched17 {
                                                    cat_paw_agent_spl_run(
                                                        ctx,
                                                        CatPawAgentSplStep::Catwarp,
                                                        vec![Val::from(5500), Val::from(11)],
                                                    )?;
                                                }
                                                if !matched17 && subject17.loosely_equals(&Val::from(3)) {
                                                    matched17 = true;
                                                }
                                                if matched17 {
                                                    ctx.lines_as("Cat Paw Agent", args!["Thank you for using our service."])?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                }
                                            }
                                        }
                                    }
                                    if !matched11 && subject11.loosely_equals(&Val::from(5)) {
                                        matched11 = true;
                                    }
                                    if matched11 {
                                        ctx.lines_as("Cat Paw Agent", args!["Thank you for using our service."])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                }
                            } else {
                                ctx.lines_as("Cat Paw Agent", args!["... ... ... ...", "Please give me some Piece of Fish."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                    }
                }
                step = CatPawAgentSplStep::AfterCatwarp;
                continue 'machine;
            }
            CatPawAgentSplStep::Catwarp => {
                if runtime::op(&ctx.var("Zeny").get()?, "<", &runtime::arg(&args, 0, Val::from(0)))?.is_true() {
                    ctx.lines_as("Cat Paw Agent", args!["Don't play with money."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.close_window()?;
                ctx.var("Zeny")
                    .set((ctx.var("Zeny").get()?.try_sub(runtime::arg(&args, 0, Val::from(0)))?))?;
                let subject18 = runtime::arg(&args, 1, Val::from(0));
                if subject18 == 1 {
                    ctx.call(Function::Warp, vec![Val::from("alberta"), Val::from(117), Val::from(56)])?;
                    return Err(Stop::End);
                } else if subject18 == 2 {
                    ctx.call(Function::Warp, vec![Val::from("prontera"), Val::from(116), Val::from(72)])?;
                    return Err(Stop::End);
                } else if subject18 == 3 {
                    ctx.call(Function::Warp, vec![Val::from("izlude"), Val::from(91), Val::from(105)])?;
                    return Err(Stop::End);
                } else if subject18 == 4 {
                    ctx.call(Function::Warp, vec![Val::from("geffen"), Val::from(120), Val::from(39)])?;
                    return Err(Stop::End);
                } else if subject18 == 5 {
                    ctx.call(Function::Warp, vec![Val::from("payon"), Val::from(161), Val::from(58)])?;
                    return Err(Stop::End);
                } else if subject18 == 6 {
                    ctx.call(Function::Warp, vec![Val::from("morocc"), Val::from(156), Val::from(46)])?;
                    return Err(Stop::End);
                } else if subject18 == 7 {
                    ctx.call(Function::Warp, vec![Val::from("aldebaran"), Val::from(168), Val::from(112)])?;
                    return Err(Stop::End);
                } else if subject18 == 8 {
                    ctx.call(Function::Warp, vec![Val::from("yuno"), Val::from(158), Val::from(125)])?;
                    return Err(Stop::End);
                } else if subject18 == 9 {
                    ctx.call(Function::Warp, vec![Val::from("einbroch"), Val::from(158), Val::from(301)])?;
                    return Err(Stop::End);
                } else if subject18 == 10 {
                    ctx.call(Function::Warp, vec![Val::from("lighthalzen"), Val::from(163), Val::from(64)])?;
                    return Err(Stop::End);
                } else if subject18 == 11 {
                    ctx.call(Function::Warp, vec![Val::from("spl_fild02"), Val::from(32), Val::from(225)])?;
                    return Err(Stop::End);
                } else if subject18 == 12 {
                    ctx.call(Function::Warp, vec![Val::from("man_fild02"), Val::from(129), Val::from(61)])?;
                    return Err(Stop::End);
                } else if subject18 == 13 {
                    ctx.call(Function::Warp, vec![Val::from("mid_camp"), Val::from(62), Val::from(127)])?;
                    return Err(Stop::End);
                }
                return Ok(Val::from(0));
            }
            CatPawAgentSplStep::AfterCatwarp => {
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn cat_paw_agent_spl(ctx: &Ctx) -> Script {
    cat_paw_agent_spl_run(ctx, CatPawAgentSplStep::Start, Vec::new()).map(|_| ())
}

fn mysterious_rock_30_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_rhea_ran = Val::from(0);
    if (ctx.call(Function::CountItem, vec![Val::from(6048)])?.number()? < 3
        && ctx.call(Function::CheckQuest, vec![Val::from(12062), ctx.constant("PLAYTIME")?])? == -1)
    {
        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_REPAIRWEAPON")?])?;
        ctx.call(Function::ProgressBar, vec![Val::from("ffff00"), Val::from(10)])?;
        l_rhea_ran = ctx.call(Function::Rand, vec![Val::from(1), Val::from(20)])?;
        if l_rhea_ran.clone().number()? < 13 {
            ctx.call(Function::GetItem, vec![Val::from(7049), Val::from(1)])?;
        } else {
            if l_rhea_ran.clone() == 13 {
                ctx.call(Function::GetItem, vec![Val::from(990), Val::from(1)])?;
            } else if l_rhea_ran.clone() == 14 {
                ctx.call(Function::GetItem, vec![Val::from(991), Val::from(1)])?;
            } else if l_rhea_ran.clone() == 15 {
                ctx.call(Function::GetItem, vec![Val::from(992), Val::from(1)])?;
            } else if l_rhea_ran.clone() == 16 {
                ctx.call(Function::GetItem, vec![Val::from(993), Val::from(1)])?;
            } else if l_rhea_ran.clone() == 17 {
                ctx.call(Function::GetItem, vec![Val::from(6080), Val::from(1)])?;
            } else {
                ctx.call(Function::GetItem, vec![Val::from(6048), Val::from(1)])?;
            }
        }
        ctx.call(Function::InitNpcTimer, vec![])?;
        ctx.call(Function::DisableNpc, vec![])?;
        return Err(Stop::End);
    } else {
        ctx.lines(args![
            "This rock contains unidentified minerals.",
            "It's not possible to mine more than the limit."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    return Err(Stop::End);
}

pub fn mysterious_rock_30(ctx: &Ctx) -> Script {
    mysterious_rock_30_body(ctx, Vec::new()).map(|_| ())
}

fn mysterious_rock_30_ontimer120000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![])?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn mysterious_rock_30_ontimer120000(ctx: &Ctx) -> Script {
    mysterious_rock_30_ontimer120000_body(ctx, Vec::new()).map(|_| ())
}

fn school_of_fish_5_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_fcast = Val::from(0);
    let mut l_rhea_ran = Val::from(0);
    let mut l_rhea_ran3 = Val::from(0);
    let mut l_rhea_ran4 = Val::from(0);
    let mut l_rhea_ran5 = Val::from(0);
    if (ctx.call(Function::CheckQuest, vec![Val::from(12060), ctx.constant("PLAYTIME")?])? == -1
        && ctx.call(Function::CountItem, vec![Val::from(6039)])?.number()? < 20)
    {
        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_BUBBLE")?])?;
        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_INVENOM")?])?;
        l_fcast = Val::from(15);
        if ctx.call(Function::IsEquipped, vec![Val::from(2550)])?.is_true() {
            l_fcast = (l_fcast.clone().try_sub(Val::from(2))?);
        }
        if ctx.call(Function::IsEquipped, vec![Val::from(2443)])?.is_true() {
            l_fcast = (l_fcast.clone().try_sub(Val::from(2))?);
        }
        if ctx.call(Function::IsEquipped, vec![Val::from(2764)])?.is_true() {
            l_fcast = (l_fcast.clone().try_sub(Val::from(3))?);
        }
        if ctx.call(Function::IsEquipped, vec![Val::from(2775)])?.is_true() {
            l_fcast = (l_fcast.clone().try_sub(Val::from(1))?);
        }
        if ctx.call(Function::IsEquipped, vec![Val::from(1599)])?.is_true() {
            l_fcast = (l_fcast.clone().try_sub(Val::from(3))?);
        }
        if ctx.call(Function::IsEquipped, vec![Val::from(2199)])?.is_true() {
            l_fcast = (l_fcast.clone().try_sub(Val::from(4))?);
        }
        ctx.call(Function::ProgressBar, vec![Val::from("ffff00"), l_fcast.clone()])?;
        if ctx.var("ep13_1_rhea").get()? == 13 && ctx.call(Function::Rand, vec![Val::from(1), Val::from(20)])? == 2 {
            ctx.call(Function::GetItem, vec![Val::from(6037), Val::from(1)])?;
            ctx.var("ep13_1_rhea").set(Val::from(14))?;
            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_BUBBLE")?])?;
            ctx.call(
                Function::MapAnnounce,
                vec![
                    ctx.call(Function::StrCharInfo, vec![Val::from(3)])?,
                    (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(" has caught a Loose File.")),
                    ctx.constant("BC_MAP")?,
                    Val::from("0xff77ff"),
                ],
            )?;
        }
        l_rhea_ran = ctx.call(Function::Rand, vec![Val::from(1), Val::from(70)])?;
        if l_rhea_ran.clone().number()? < 20 {
            ctx.call(Function::GetItem, vec![Val::from(6039), Val::from(1)])?;
        } else {
            if l_rhea_ran.clone() == 20 {
                ctx.call(Function::GetItem, vec![Val::from(908), Val::from(1)])?;
            } else {
                if l_rhea_ran.clone() == 21 {
                    ctx.call(Function::GetItem, vec![Val::from(909), Val::from(1)])?;
                } else {
                    if l_rhea_ran.clone() == 22 {
                        ctx.call(Function::GetItem, vec![Val::from(963), Val::from(1)])?;
                    } else {
                        if l_rhea_ran.clone() == 23 {
                            ctx.call(Function::GetItem, vec![Val::from(956), Val::from(1)])?;
                        } else {
                            if l_rhea_ran.clone() == 24 {
                                ctx.call(Function::GetItem, vec![Val::from(6049), Val::from(1)])?;
                            } else {
                                if l_rhea_ran.clone() == 25 {
                                    ctx.call(Function::GetItem, vec![Val::from(918), Val::from(1)])?;
                                } else if l_rhea_ran.clone() == 26 {
                                    ctx.call(Function::GetItem, vec![Val::from(960), Val::from(1)])?;
                                } else if l_rhea_ran.clone() == 27 {
                                    ctx.call(Function::GetItem, vec![Val::from(910), Val::from(1)])?;
                                } else if l_rhea_ran.clone() == 28 {
                                    ctx.call(Function::GetItem, vec![Val::from(6081), Val::from(1)])?;
                                } else if (l_rhea_ran.clone().number()? > 28 && l_rhea_ran.clone().number()? < 40) {
                                    ctx.call(Function::GetItem, vec![Val::from(7049), Val::from(1)])?;
                                } else {
                                    ctx.mes("Nothing was caught.")?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                            }
                        }
                    }
                }
            }
        }
        l_rhea_ran5 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(200)])?;
        if l_rhea_ran5.clone() == 3 {
            ctx.call(Function::GetItem, vec![Val::from(644), Val::from(1)])?;
            ctx.call(
                Function::MapAnnounce,
                vec![
                    ctx.call(Function::StrCharInfo, vec![Val::from(3)])?,
                    (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(" has caught a Gift Box.")),
                    ctx.constant("BC_MAP")?,
                    Val::from("0x00ffff"),
                ],
            )?;
        }
        l_rhea_ran3 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(500)])?;
        if l_rhea_ran3.clone() == 3 {
            ctx.call(Function::GetItem, vec![Val::from(603), Val::from(1)])?;
            ctx.call(
                Function::MapAnnounce,
                vec![
                    ctx.call(Function::StrCharInfo, vec![Val::from(3)])?,
                    (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(" has caught an Old Blue Box.")),
                    ctx.constant("BC_MAP")?,
                    Val::from("0x00ffff"),
                ],
            )?;
        }
        l_rhea_ran4 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3000)])?;
        if l_rhea_ran4.clone() == 3 {
            ctx.call(Function::GetItem, vec![Val::from(617), Val::from(1)])?;
            ctx.call(
                Function::MapAnnounce,
                vec![
                    ctx.call(Function::StrCharInfo, vec![Val::from(3)])?,
                    (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(" has caught an Old Purple Box.")),
                    ctx.constant("BC_MAP")?,
                    Val::from("0x44ff44"),
                ],
            )?;
        }
    } else {
        ctx.mes("Fish are swimming in the water.")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    return Err(Stop::End);
}

pub fn school_of_fish_5(ctx: &Ctx) -> Script {
    school_of_fish_5_body(ctx, Vec::new()).map(|_| ())
}

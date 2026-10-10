use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum GuardingMonkMkStep {
    Start,
    OnTouch,
}

fn guarding_monk_mk_run(ctx: &Ctx, mut step: GuardingMonkMkStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            GuardingMonkMkStep::Start => {
                if ctx.var("Upper").get()? == 1 {
                    ctx.lines_as(
                        "Tohobu",
                        args![
                            "Hmm? What business do you have here?",
                            "If you wish to enter this sacred area,",
                            "you must give me your name and job level!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Tohobu",
                        args![
                            "....Eh?",
                            "Oh!^FF0000gosh^000000! I am sorry, I think I misunderstood you from someone I know."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Tohobu", args![".......", "........"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Tohobu",
                        args!["It is odd...I never misunderstand people...oh, well. Have a good day."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?) && ctx.var("monk_q").get()? == 0) {
                    ctx.lines_as(
                        "Tohobu",
                        args![
                            "Hmm? What business do you have here?",
                            "If you wish to enter this sacred area,",
                            "you must give me your name and job level!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Tohobu", args!["Now, please tell me your name and job level."])?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("Ignore him.:Tell him.")])?) == 1 {
                        ctx.lines_as("Tohobu", args!["To ignore another is disrespectful, get out!"])?;
                        ctx.close_window()?;
                        ctx.call(Function::Warp, vec![Val::from("prt_fild03"), Val::from(357), Val::from(256)])?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        "Tohobu",
                        args![
                            ((Val::from("Hmm... ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(" is your name?")),
                            "...did I say it right?",
                            ((Val::from("Okay, and your job level is ") + ctx.var("JobLevel").get()?) + Val::from(" correct?"))
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Tohobu",
                        args![
                            "Very well... why have you come here",
                            ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("?"))
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(
                        ctx,
                        &[Val::from(
                            "To visit and learn about monks.:I wish to become a monk...:I'm tired and need to rest...",
                        )],
                    )? {
                        1 => {
                            ctx.lines_as(
                                "Tohobu",
                                args![
                                    "I see...",
                                    "We monks live our lives for spiritual enlightenment.",
                                    "We improve our bodies as well as our minds to reach true inner peace.",
                                    "May you find your inner peace as well."
                                ],
                            )?;
                            ctx.var("monk_q").set(Val::from(1))?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?)
                                && ctx.var("JobLevel").get()?.number()? > 39)
                            {
                                ctx.lines_as(
                                    "Tohobu",
                                    args![
                                        "Hmm you seem as though you have been training for this...",
                                        "That is good. Go see our sensei Moohae. Speak with him.",
                                        "He will help you start your training."
                                    ],
                                )?;
                                ctx.var("monk_q").set(Val::from(2))?;
                                ctx.call(Function::SetQuest, vec![Val::from(3016)])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?)
                                && ctx.var("JobLevel").get()?.number()? < 40)
                            {
                                ctx.lines_as(
                                    "Tohobu",
                                    args![
                                        "Hmm, you do not seem ready to become a monk.",
                                        "To become a monk you must be,",
                                        "at least a job level 40 Acolyte.",
                                        "If not, you are not yet ready to become a monk."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Tohobu",
                                    args![
                                        "Come back to me when you have trained more",
                                        "and I will let you know if you are ready."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Tohobu",
                                    args![
                                        "I hope that you will soon join us on our",
                                        "path of inner peace and enlightenment.",
                                        "I'll be waiting here for you."
                                    ],
                                )?;
                                ctx.var("monk_q").set(Val::from(1))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                ctx.lines_as("Tohobu", args!["Hahahha that was a good joke!"])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                        3 => {
                            ctx.lines_as(
                                "Tohobu",
                                args![
                                    "Yes, we all need to take a rest once in a while...",
                                    "It is a good idea not to stress your self.",
                                    "Come in and make yourself comfortable.",
                                    "Rest as long as you need to."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Tohobu",
                                args![
                                    "I hope that you become energized",
                                    "when observing our brothers in their",
                                    "pursuit of spiritual enlightenment.",
                                    "I hope you reach it too."
                                ],
                            )?;
                            ctx.var("monk_q").set(Val::from(1))?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else {
                    if (ctx.var("monk_q").get()? == 1 && ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?)) {
                        ctx.lines_as(
                            "Tohobu",
                            args!["What do you think? Did your visit reveal anything to your spirit?"],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("No...:I wish to become a monk.:I need to rest...")])? {
                            1 => {
                                ctx.lines_as(
                                    "Tohobu",
                                    args![
                                        "I see, there is no shame in that.",
                                        "I hope that your experience here with",
                                        "our brothers has helped you become one",
                                        "step closer to true enlightenment."
                                    ],
                                )?;
                                ctx.var("monk_q").set(Val::from(1))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?)
                                    && ctx.var("JobLevel").get()?.number()? > 39)
                                {
                                    ctx.lines_as(
                                        "Tohobu",
                                        args![
                                            "Hmm you seem as though you have been training for this...",
                                            "That is good. Go see our sensei Moohae. Speak with him.",
                                            "He will help you start your training."
                                        ],
                                    )?;
                                    ctx.var("monk_q").set(Val::from(2))?;
                                    ctx.call(Function::SetQuest, vec![Val::from(3016)])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?)
                                    && ctx.var("JobLevel").get()?.number()? < 40)
                                {
                                    ctx.lines_as(
                                        "Tohobu",
                                        args![
                                            "Hmm, you do not seem ready to become a monk.",
                                            "To become a monk you must be,",
                                            "at least a job level 40 Acolyte.",
                                            "If not, you are not yet ready to become a monk."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Tohobu",
                                        args![
                                            "Come back to me when you have trained more on your own",
                                            "and I will let you know if you are ready."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Tohobu",
                                        args![
                                            "I hope that you will soon join us in our",
                                            "path to inner peace and enlightenment.",
                                            "I'll be waiting here for you."
                                        ],
                                    )?;
                                    ctx.var("monk_q").set(Val::from(1))?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    ctx.lines_as("Tohobu", args!["Hahahha that was a good joke!"])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                            }
                            3 => {
                                ctx.lines_as(
                                    "Tohobu",
                                    args![
                                        "Yes, we all need to take a rest once in a while...",
                                        "It is a good idea not to stress your self.",
                                        "Come in and make yourself comfortable.",
                                        "Rest as long as you need to."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Tohobu",
                                    args![
                                        "I hope that you become energized",
                                        "when observing our brothers in their",
                                        "pursuit of spiritual enlightenment.",
                                        "I hope you reach it too."
                                    ],
                                )?;
                                ctx.var("monk_q").set(Val::from(1))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                    if ctx.var("monk_q").get()? == 0 {
                        ctx.lines_as(
                            "Tohobu",
                            args![
                                "Hmm? What business do you have here?",
                                "If you wish to enter this sacred area,",
                                "You must give me your name, job level, and level!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Tohobu", args!["Now, please tell me your name as well as your job level!"])?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(ctx, &[Val::from("Ignore.:Tell him.")])?) == 1 {
                            ctx.lines_as("Tohobu", args!["To ignore another is disrespectful, get out!"])?;
                            ctx.close_window()?;
                            ctx.call(Function::Warp, vec![Val::from("prt_fild03"), Val::from(357), Val::from(256)])?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as(
                            "Tohobu",
                            args![
                                ((Val::from("Hmm... ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                    + Val::from(" is your name?")),
                                "...did I say it right?",
                                ((Val::from("Okay, and your job level is ") + ctx.var("JobLevel").get()?) + Val::from(" correct?"))
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Tohobu",
                            args![
                                "Okay, Now, why have you come here",
                                ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("?"))
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(
                            ctx,
                            &[Val::from(
                                "To visit and learn about monks.:I wish to become a monk...:I'm tired and need to rest...",
                            )],
                        )? {
                            1 => {
                                ctx.lines_as(
                                    "Tohobu",
                                    args![
                                        "I see...",
                                        "We monks live our lives for God and spiritual enlightenment.",
                                        "We improve our bodies as well as our minds to reach true inner peace.",
                                        "May you find your inner peace as well."
                                    ],
                                )?;
                                ctx.var("monk_q").set(Val::from(1))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?)
                                    && ctx.var("JobLevel").get()?.number()? > 39)
                                {
                                    ctx.lines_as(
                                        "Tohobu",
                                        args![
                                            "Hmm you seem as though you have been training for this...",
                                            "That is good. Go see our sensei Moohae, speak with him",
                                            "and he will help you start new training."
                                        ],
                                    )?;
                                    ctx.var("monk_q").set(Val::from(2))?;
                                    ctx.call(Function::SetQuest, vec![Val::from(3016)])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?)
                                    && ctx.var("JobLevel").get()?.number()? < 40)
                                {
                                    ctx.lines_as(
                                        "Tohobu",
                                        args![
                                            "Hmm, you do not seem ready to become a monk.",
                                            "To become a monk you must be,",
                                            "at least a job level 40 Acolyte.",
                                            "If not, you are not yet ready to become a monk."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Tohobu",
                                        args![
                                            "Come back to me when you have trained more on your own",
                                            "and I will let you know if you are ready."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Tohobu",
                                        args![
                                            "I hope that you will soon join us in our",
                                            "path to inner peace and enlightenment.",
                                            "I'll be waiting here for you."
                                        ],
                                    )?;
                                    ctx.var("monk_q").set(Val::from(1))?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    ctx.lines_as("Tohobu", args!["Hahahha that was a good joke!"])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                            }
                            3 => {
                                ctx.lines_as(
                                    "Tohobu",
                                    args![
                                        "Yes, we all need to take a rest once in a while...",
                                        "It is a good idea not to stress your self.",
                                        "Come in and make yourself comfortable.",
                                        "Rest as long as you need to."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Tohobu",
                                    args![
                                        "I hope that you become energized",
                                        "when observing our brothers in their",
                                        "pursuit of spiritual enlightenment.",
                                        "I hope you reach it too."
                                    ],
                                )?;
                                ctx.var("monk_q").set(Val::from(1))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    } else if ctx.var("monk_q").get()? == 1 {
                        ctx.lines_as("Tohobu", args!["Listen carefully on your journey.", "There is much to learn."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?) && ctx.var("monk_q").get()? == 2) {
                        ctx.lines_as(
                            "Tohobu",
                            args![
                                "Hmm... would you like to meet sensei Moohae?",
                                "He is in the south east area in 'The Hall of Monks'."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?)
                        && ctx.var("monk_q").get()?.number()? > 2)
                    {
                        ctx.lines_as(
                            "Tohobu",
                            args!["I hope you do well in your training and I look forward to seeing you again."],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Tohobu",
                            args![
                                "Welcome to the central chamber of our Church.",
                                "Please, try not to disturb the other monks.",
                                "Even if you are a monk yourself."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                step = GuardingMonkMkStep::OnTouch;
                continue 'machine;
            }
            GuardingMonkMkStep::OnTouch => {
                if ctx.var("monk_q").get()? == 0 {
                    ctx.lines_as(
                        "Tohobu",
                        args!["How dare you set foot in", "this holy building! ! !", "Where is your respect?!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Tohobu", args!["Leave this place ! ! !"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ctx.var("monk_q").get()? == 1 {
                    ctx.lines_as("Tohobu", args!["Hmmm... come in.", "You may learn something..."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?) && ctx.var("monk_q").get()? == 2) {
                    ctx.lines_as(
                        "Tohobu",
                        args![
                            "Hmm.....you wish to see our sensei Moohae?",
                            "He is in the south east section of this building."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?) && ctx.var("monk_q").get()?.number()? > 2) {
                    ctx.lines_as("Tohobu", args!["I look forward to seeing you become a monk and joining us."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn guarding_monk_mk(ctx: &Ctx) -> Script {
    guarding_monk_mk_run(ctx, GuardingMonkMkStep::Start, Vec::new()).map(|_| ())
}

pub fn guarding_monk_mk_ontouch(ctx: &Ctx) -> Script {
    guarding_monk_mk_run(ctx, GuardingMonkMkStep::OnTouch, Vec::new()).map(|_| ())
}

fn sensei_moohae_mk_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_items: Vec<Val> = Vec::new();
    let mut l_jlevel = Val::from(0);
    ctx.lines_as(
        "Sensei Moohae",
        args![
            "Greetings, you seem to be on a pure path.",
            "Come in, come in, what can I do for you today?"
        ],
    )?;
    ctx.next()?;
    if ctx.var("SkillPoint").get()?.is_true() {
        ctx.lines_as(
            "Sensei Moohae",
            args![
                "If you have free skill points, you will lose them during a job change.",
                "Make sure to use any skill points you have."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ((ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?) && ctx.var("monk_q").get()? == 2)
        && ctx.var("JobLevel").get()?.number()? > 39)
    {
        ctx.lines_as(
            "Sensei Moohae",
            args!["I sense a fighting spirit, do you wish to become a monk? "],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Yes.:No.")])?) == 2 {
            ctx.lines_as(
                "Sensei Moohae",
                args![
                    "My apologies... It has been some time since",
                    "I have sensed someone with your strength.",
                    "I hope you find your path young one."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as("Sensei Moohae", args!["There are still those who wish to follow the old ways."])?;
        if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
            ctx.mes("A strong young man. I am pleased of your will to join us.")?;
        } else {
            ctx.mes("Such a delicate flower. I am pleased to see your will to join us.")?;
        }
        ctx.next()?;
        ctx.lines_as(
            "Sensei Moohae",
            args![
                "Oh, you are the new pupil that wishes to join us...",
                "Well there are a few things that you should know prior to beginning your training."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sensei Moohae",
            args![
                "We monks are on a path of inner peace and enlightenment.",
                "We strive to bring such peace to all others with great care."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sensei Moohae",
            args![
                "We monks achieve this from mental and physical training.",
                "We search for enlightenment in our surroundings and in nature."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sensei Moohae",
            args!["It is, of course, important to always keep our original faith in God."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sensei Moohae",
            args![
                "This is not an easy life and the true test of becoming a monk is having the ability to endure all of which I said...",
                "The life of a monk is not for everybody, only those strong enough can become a monk."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sensei Moohae",
            args![
                "Now, that you understand all of this,",
                "prepare yourself to train",
                "your strength and spirit."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Sensei Moohae", args!["Let us start with a simple task."])?;
        ctx.next()?;
        let subject1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(7)])?;
        if subject1 == 1 {
            let base = Val::from(0).number()?;
            runtime::local_set(&mut l_items, &Val::from(base + 0), Val::from(938), false);
            runtime::local_set(&mut l_items, &Val::from(base + 1), Val::from(5), false);
            runtime::local_set(&mut l_items, &Val::from(base + 2), Val::from(1055), false);
            runtime::local_set(&mut l_items, &Val::from(base + 3), Val::from(10), false);
            runtime::local_set(&mut l_items, &Val::from(base + 4), Val::from(511), false);
            runtime::local_set(&mut l_items, &Val::from(base + 5), Val::from(20), false);
            runtime::local_set(&mut l_items, &Val::from(base + 6), Val::from(3), false);
            ctx.call(Function::ChangeQuest, vec![Val::from(3016), Val::from(3017)])?;
        } else if subject1 == 2 {
            let base = Val::from(0).number()?;
            runtime::local_set(&mut l_items, &Val::from(base + 0), Val::from(942), false);
            runtime::local_set(&mut l_items, &Val::from(base + 1), Val::from(20), false);
            runtime::local_set(&mut l_items, &Val::from(base + 2), Val::from(1002), false);
            runtime::local_set(&mut l_items, &Val::from(base + 3), Val::from(5), false);
            runtime::local_set(&mut l_items, &Val::from(base + 4), Val::from(510), false);
            runtime::local_set(&mut l_items, &Val::from(base + 5), Val::from(3), false);
            runtime::local_set(&mut l_items, &Val::from(base + 6), Val::from(4), false);
            ctx.call(Function::ChangeQuest, vec![Val::from(3016), Val::from(3018)])?;
        } else if subject1 == 3 {
            let base = Val::from(0).number()?;
            runtime::local_set(&mut l_items, &Val::from(base + 0), Val::from(905), false);
            runtime::local_set(&mut l_items, &Val::from(base + 1), Val::from(30), false);
            runtime::local_set(&mut l_items, &Val::from(base + 2), Val::from(909), false);
            runtime::local_set(&mut l_items, &Val::from(base + 3), Val::from(5), false);
            runtime::local_set(&mut l_items, &Val::from(base + 4), Val::from(955), false);
            runtime::local_set(&mut l_items, &Val::from(base + 5), Val::from(10), false);
            runtime::local_set(&mut l_items, &Val::from(base + 6), Val::from(5), false);
            ctx.call(Function::ChangeQuest, vec![Val::from(3016), Val::from(3019)])?;
        } else if subject1 == 4 {
            let base = Val::from(0).number()?;
            runtime::local_set(&mut l_items, &Val::from(base + 0), Val::from(943), false);
            runtime::local_set(&mut l_items, &Val::from(base + 1), Val::from(5), false);
            runtime::local_set(&mut l_items, &Val::from(base + 2), Val::from(935), false);
            runtime::local_set(&mut l_items, &Val::from(base + 3), Val::from(20), false);
            runtime::local_set(&mut l_items, &Val::from(base + 4), Val::from(912), false);
            runtime::local_set(&mut l_items, &Val::from(base + 5), Val::from(5), false);
            runtime::local_set(&mut l_items, &Val::from(base + 6), Val::from(6), false);
            ctx.call(Function::ChangeQuest, vec![Val::from(3016), Val::from(3020)])?;
        } else if subject1 == 5 {
            let base = Val::from(0).number()?;
            runtime::local_set(&mut l_items, &Val::from(base + 0), Val::from(7053), false);
            runtime::local_set(&mut l_items, &Val::from(base + 1), Val::from(5), false);
            runtime::local_set(&mut l_items, &Val::from(base + 2), Val::from(509), false);
            runtime::local_set(&mut l_items, &Val::from(base + 3), Val::from(10), false);
            runtime::local_set(&mut l_items, &Val::from(base + 4), Val::from(508), false);
            runtime::local_set(&mut l_items, &Val::from(base + 5), Val::from(10), false);
            runtime::local_set(&mut l_items, &Val::from(base + 6), Val::from(7), false);
            ctx.call(Function::ChangeQuest, vec![Val::from(3016), Val::from(3021)])?;
        } else if subject1 == 6 {
            let base = Val::from(0).number()?;
            runtime::local_set(&mut l_items, &Val::from(base + 0), Val::from(913), false);
            runtime::local_set(&mut l_items, &Val::from(base + 1), Val::from(10), false);
            runtime::local_set(&mut l_items, &Val::from(base + 2), Val::from(948), false);
            runtime::local_set(&mut l_items, &Val::from(base + 3), Val::from(5), false);
            runtime::local_set(&mut l_items, &Val::from(base + 4), Val::from(7033), false);
            runtime::local_set(&mut l_items, &Val::from(base + 5), Val::from(20), false);
            runtime::local_set(&mut l_items, &Val::from(base + 6), Val::from(8), false);
            ctx.call(Function::ChangeQuest, vec![Val::from(3016), Val::from(3022)])?;
        } else if subject1 == 7 {
            let base = Val::from(0).number()?;
            runtime::local_set(&mut l_items, &Val::from(base + 0), Val::from(1027), false);
            runtime::local_set(&mut l_items, &Val::from(base + 1), Val::from(5), false);
            runtime::local_set(&mut l_items, &Val::from(base + 2), Val::from(1025), false);
            runtime::local_set(&mut l_items, &Val::from(base + 3), Val::from(20), false);
            runtime::local_set(&mut l_items, &Val::from(base + 4), Val::from(1042), false);
            runtime::local_set(&mut l_items, &Val::from(base + 5), Val::from(10), false);
            runtime::local_set(&mut l_items, &Val::from(base + 6), Val::from(9), false);
            ctx.call(Function::ChangeQuest, vec![Val::from(3016), Val::from(3023)])?;
        }
        ctx.lines_as(
            "Sensei Moohae",
            args![
                (((runtime::local_get(&l_items, &Val::from(1), false) + Val::from(" "))
                    + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_items, &Val::from(0), false)])?)
                    + Val::from(",")),
                (((runtime::local_get(&l_items, &Val::from(3), false) + Val::from(" "))
                    + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_items, &Val::from(2), false)])?)
                    + Val::from(",")),
                (((runtime::local_get(&l_items, &Val::from(5), false) + Val::from(" "))
                    + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_items, &Val::from(4), false)])?)
                    + Val::from(".")),
                "Find these items and return to me."
            ],
        )?;
        ctx.var("monk_q").set(runtime::local_get(&l_items, &Val::from(6), false))?;
        ctx.next()?;
        ctx.mes("[Sensei Moohae]")?;
        let subject2 = runtime::local_get(&l_items, &Val::from(6), false);
        if subject2 == 3 {
            ctx.mes("Why the face? This is a test of your abilities.")?;
        } else if subject2 == 4 {
            ctx.mes("What's wrong? This is a test of your abilities.")?;
        } else if subject2 == 5 {
            ctx.mes("You do understand don't you? This is a test of your abilities.")?;
        } else if subject2 == 6 {
            ctx.mes("Don't look at me like that. This is a test of your abilities.")?;
        } else if subject2 == 7 {
            ctx.mes("You don't seem concerned, this is a test of your abilities You should take this seriously.")?;
        } else if subject2 == 8 {
            ctx.mes("It is a test of your abilities so make sure you acquire these on your own.")?;
        } else if subject2 == 9 {
            ctx.mes("Don't be concerned, I believe you can do it. This is only to test your abilities.")?;
        }
        ctx.next()?;
        ctx.lines_as(
            "Sensei Moohae",
            args![
                "If you are unable to return with these items, you are not yet ready to become a monk.",
                "Be sure to collect all the items I listed.",
                "May God be with you."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("monk_q").get()? == 3 {
            ctx.lines_as("Sensei Moohae", args!["You are back, did you bring what I asked?"])?;
            ctx.next()?;
            if ((ctx.call(Function::CountItem, vec![Val::from(938)])?.number()? > 4
                && ctx.call(Function::CountItem, vec![Val::from(1055)])?.number()? > 9)
                && ctx.call(Function::CountItem, vec![Val::from(511)])?.number()? > 19)
            {
                ctx.lines_as(
                    "Sensei Moohae",
                    args!["Well done, you found all the items.", "I will tell this to the elders."],
                )?;
                ctx.var("monk_q").set(Val::from(10))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(3017), Val::from(3024)])?;
                ctx.call(Function::DelItem, vec![Val::from(938), Val::from(5)])?;
                ctx.call(Function::DelItem, vec![Val::from(1055), Val::from(10)])?;
                ctx.call(Function::DelItem, vec![Val::from(511), Val::from(20)])?;
                ctx.next()?;
                ctx.lines_as(
                    "Sensei Moohae",
                    args![
                        "Let's see who is to see you next..",
                        "Ah... go find elder Touha.",
                        "He is in the north west."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Sensei Moohae",
                    args![
                        "How can you think to be done?",
                        "You do not have what I asked for!",
                        "5 Sticky Mucus,",
                        "10 Earthworm Peeling,",
                        "20 Green Herb.",
                        "These are the items I require, go find them all."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else {
            if ctx.var("monk_q").get()? == 4 {
                ctx.lines_as("Sensei Moohae", args!["...eh?"])?;
                ctx.next()?;
                if ((ctx.call(Function::CountItem, vec![Val::from(942)])?.number()? > 19
                    && ctx.call(Function::CountItem, vec![Val::from(1002)])?.number()? > 4)
                    && ctx.call(Function::CountItem, vec![Val::from(510)])?.number()? > 2)
                {
                    ctx.lines_as(
                        "Sensei Moohae",
                        args!["Very good, you found all the items.", "I will tell this to the elders."],
                    )?;
                    ctx.var("monk_q").set(Val::from(10))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(3018), Val::from(3024)])?;
                    ctx.call(Function::DelItem, vec![Val::from(942), Val::from(20)])?;
                    ctx.call(Function::DelItem, vec![Val::from(1002), Val::from(5)])?;
                    ctx.call(Function::DelItem, vec![Val::from(510), Val::from(3)])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Sensei Moohae",
                        args![
                            "Let's see who is to see you next..",
                            "Ah... go find elder Touha.",
                            "He is in the north west."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Sensei Moohae",
                        args![
                            "Why did you return?",
                            "You do not have what I asked for!",
                            "20 Yoyo Tail,",
                            "5 Iron Ore,",
                            "3 Blue Herb.",
                            "These are the items I require, go find them all."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            } else {
                if ctx.var("monk_q").get()? == 5 {
                    ctx.lines_as("Sensei Moohae", args!["Hmm?"])?;
                    ctx.next()?;
                    if ((ctx.call(Function::CountItem, vec![Val::from(905)])?.number()? > 29
                        && ctx.call(Function::CountItem, vec![Val::from(909)])?.number()? > 4)
                        && ctx.call(Function::CountItem, vec![Val::from(955)])?.number()? > 9)
                    {
                        ctx.lines_as(
                            "Sensei Moohae",
                            args![
                                "See, that wasn't so bad you real found all the items.",
                                "I will tell this to the elders."
                            ],
                        )?;
                        ctx.var("monk_q").set(Val::from(10))?;
                        ctx.call(Function::ChangeQuest, vec![Val::from(3019), Val::from(3024)])?;
                        ctx.call(Function::DelItem, vec![Val::from(905), Val::from(30)])?;
                        ctx.call(Function::DelItem, vec![Val::from(909), Val::from(5)])?;
                        ctx.call(Function::DelItem, vec![Val::from(955), Val::from(10)])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Sensei Moohae",
                            args!["The next step will be given", "to you by Touha.", "He is in the north west."],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Sensei Moohae",
                            args![
                                "How can you think to be done?",
                                "You do not have what I asked for!",
                                "30 Stem,",
                                "5 Jellopy",
                                "10 Worm Peelings",
                                "These are the items I require, go find them all."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                } else {
                    if ctx.var("monk_q").get()? == 6 {
                        ctx.lines_as("Sensei Moohae", args!["I have been waiting for you."])?;
                        ctx.next()?;
                        if ((ctx.call(Function::CountItem, vec![Val::from(943)])?.number()? > 4
                            && ctx.call(Function::CountItem, vec![Val::from(935)])?.number()? > 19)
                            && ctx.call(Function::CountItem, vec![Val::from(912)])?.number()? > 4)
                        {
                            ctx.lines_as(
                                "Sensei Moohae",
                                args!["Impressive, you really found all the items.", "I will tell this to the elders."],
                            )?;
                            ctx.var("monk_q").set(Val::from(10))?;
                            ctx.call(Function::ChangeQuest, vec![Val::from(3020), Val::from(3024)])?;
                            ctx.call(Function::DelItem, vec![Val::from(943), Val::from(5)])?;
                            ctx.call(Function::DelItem, vec![Val::from(935), Val::from(20)])?;
                            ctx.call(Function::DelItem, vec![Val::from(912), Val::from(5)])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Sensei Moohae",
                                args![
                                    "Your next step will be with..",
                                    "elder Touha. Go find him.",
                                    "He is in the north west."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            ctx.lines_as(
                                "Sensei Moohae",
                                args![
                                    "How can you think to be done?",
                                    "You do not have what I asked for!",
                                    "5 Solid Shell,",
                                    "20 Shell,",
                                    "5 Zargon.",
                                    "These are the items I require, go find them all."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    } else {
                        if ctx.var("monk_q").get()? == 7 {
                            ctx.lines_as("Sensei Moohae", args!["Hello again. Back so soon?"])?;
                            ctx.next()?;
                            if ((ctx.call(Function::CountItem, vec![Val::from(7053)])?.number()? > 4
                                && ctx.call(Function::CountItem, vec![Val::from(509)])?.number()? > 9)
                                && ctx.call(Function::CountItem, vec![Val::from(508)])?.number()? > 9)
                            {
                                ctx.lines_as(
                                    "Sensei Moohae",
                                    args!["Very nice, you found all the items.", "I will tell this to the elders."],
                                )?;
                                ctx.var("monk_q").set(Val::from(10))?;
                                ctx.call(Function::ChangeQuest, vec![Val::from(3021), Val::from(3024)])?;
                                ctx.call(Function::DelItem, vec![Val::from(7053), Val::from(5)])?;
                                ctx.call(Function::DelItem, vec![Val::from(509), Val::from(10)])?;
                                ctx.call(Function::DelItem, vec![Val::from(508), Val::from(10)])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Sensei Moohae",
                                    args![
                                        "Let's see who is to see you next..",
                                        "Ah... go find elder Touha.",
                                        "He is in the north west."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                ctx.lines_as(
                                    "Sensei Moohae",
                                    args![
                                        "Where are the items...?",
                                        "You do not have what I asked for!",
                                        "5 Cyfar,",
                                        "10 White Herb,",
                                        "10 Yellow Herb.",
                                        "These are the items I require, go find them all."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        } else {
                            if ctx.var("monk_q").get()? == 8 {
                                ctx.lines_as("Sensei Moohae", args!["Hmm?"])?;
                                ctx.next()?;
                                if ((ctx.call(Function::CountItem, vec![Val::from(913)])?.number()? > 9
                                    && ctx.call(Function::CountItem, vec![Val::from(948)])?.number()? > 4)
                                    && ctx.call(Function::CountItem, vec![Val::from(7033)])?.number()? > 19)
                                {
                                    ctx.lines_as(
                                        "Sensei Moohae",
                                        args!["Excellent, all the items I asked for.", "I will tell this to the elders."],
                                    )?;
                                    ctx.var("monk_q").set(Val::from(10))?;
                                    ctx.call(Function::ChangeQuest, vec![Val::from(3022), Val::from(3024)])?;
                                    ctx.call(Function::DelItem, vec![Val::from(913), Val::from(10)])?;
                                    ctx.call(Function::DelItem, vec![Val::from(948), Val::from(5)])?;
                                    ctx.call(Function::DelItem, vec![Val::from(7033), Val::from(20)])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Sensei Moohae",
                                        args![
                                            "Let's see who is to see you next..",
                                            "Ah... go find elder Touha.",
                                            "He is in the north west."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    ctx.lines_as(
                                        "Sensei Moohae",
                                        args![
                                            "How can you think to be done?",
                                            "You do not have what I asked for!",
                                            "10 Tooth of Bat,",
                                            "5 Bear's Foot skin",
                                            "20 Poison Spore",
                                            "These are the items I require, go find them all."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                            } else {
                                if ctx.var("monk_q").get()? == 9 {
                                    ctx.lines_as("Sensei Moohae", args!["Welcome back."])?;
                                    ctx.next()?;
                                    if ((ctx.call(Function::CountItem, vec![Val::from(1027)])?.number()? > 4
                                        && ctx.call(Function::CountItem, vec![Val::from(1025)])?.number()? > 19)
                                        && ctx.call(Function::CountItem, vec![Val::from(1042)])?.number()? > 9)
                                    {
                                        ctx.lines_as(
                                            "Sensei Moohae",
                                            args!["Wow, you found all the items!!", "I will tell this to the elders."],
                                        )?;
                                        ctx.var("monk_q").set(Val::from(10))?;
                                        ctx.call(Function::ChangeQuest, vec![Val::from(3023), Val::from(3024)])?;
                                        ctx.call(Function::DelItem, vec![Val::from(1027), Val::from(5)])?;
                                        ctx.call(Function::DelItem, vec![Val::from(1025), Val::from(20)])?;
                                        ctx.call(Function::DelItem, vec![Val::from(1042), Val::from(10)])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Sensei Moohae",
                                            args![
                                                "Let's see who is to see you next..",
                                                "Ah... go find elder Touha.",
                                                "He is in the north west."
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else {
                                        ctx.lines_as(
                                            "Sensei Moohae",
                                            args![
                                                "How can you think to be done?",
                                                "You do not have what I asked for!",
                                                "5 Porcupine Quill,",
                                                "20 Cobweb,",
                                                "10 Bug Leg.",
                                                "These are the items I require, go find them all."
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                } else {
                                    if (ctx.var("monk_q").get()?.number()? > 9 && ctx.var("monk_q").get()?.number()? < 14) {
                                        ctx.lines_as(
                                            "Sensei Moohae",
                                            args![
                                                "I told you already.",
                                                "Go find ^CC0000Touha^000000.",
                                                "He is a little north west of here."
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else {
                                        if (ctx.var("monk_q").get()?.number()? > 13 && ctx.var("monk_q").get()?.number()? < 26) {
                                            ctx.lines_as(
                                                "Sensei Moohae",
                                                args!["Oh, are you still in the process of training?", "Hurry and finish!"],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        } else {
                                            if (ctx.var("monk_q").get()?.number()? > 25 && ctx.var("monk_q").get()?.number()? < 27) {
                                                ctx.lines_as(
                                                    "Sensei Moohae",
                                                    args![
                                                        "I hear good things coming from your training.",
                                                        "Good luck and work hard. You will do great things as a monk."
                                                    ],
                                                )?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            } else if (ctx.var("monk_q").get()? == 27
                                                && ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?))
                                            {
                                                ctx.lines_as("Sensei Moohae", args![".......Hmmm.....", "Go to Tomoon, get a special potion from him. It will look like a green potion, but it isn't. Bring it to me..."])?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            } else if (ctx.var("monk_q").get()? == 28
                                                && ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?))
                                            {
                                                if ctx.call(Function::CountItem, vec![Val::from(506)])?.number()? > 0 {
                                                    ctx.lines_as(
                                                        "Sensei Moohae",
                                                        args![
                                                            "Do you still have the medicine you were supposed to bring?",
                                                            "You must drink that green potion to strengthen yourself for becoming a monk."
                                                        ],
                                                    )?;
                                                } else if ctx.call(Function::CountItem, vec![Val::from(506)])? == 0 {
                                                    ctx.lines_as("Sensei Moohae", args!["Have you finished the task? Good, so you do have what it takes to become a monk.", "You didn't throw away the precious potion did you?"])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Sensei Moohae", args!["The potion you drank earlier must be taking its effect by now.", "Now that you drank the potion your training to become a monk will begin shortly..."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Sensei Moohae",
                                                        args![
                                                            "But first, answer me these questions.",
                                                            "Do you dedicate the remainder of your life to the pursuit of purity?"
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    if Val::from(runtime::select_values(ctx, &[Val::from("Yes.:No.")])?) == 2 {
                                                        ctx.lines_as(
                                                            "Sensei Moohae",
                                                            args![
                                                                "....with that kind of reply...",
                                                                "Have you not enough heart to become a monk?",
                                                                "Do you feel you have not suffered enough?"
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Sensei Moohae",
                                                            args![
                                                                "Think about it a little more and return!",
                                                                "We cannot accept a monk who is tainted with doubt..."
                                                            ],
                                                        )?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    }
                                                    ctx.lines_as("Sensei Moohae", args!["Will you take advantage of the abilities gained through our training to use for personal benefit?"])?;
                                                    ctx.next()?;
                                                    if Val::from(runtime::select_values(ctx, &[Val::from("Yes.:No.")])?) == 1 {
                                                        ctx.lines_as("Sensei Moohae", args!["...then we cannot accept you as a monk. We, monks do not practice for personal benefit.", "We lead our lives honorably and as holy executioners to the damned."])?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Sensei Moohae", args!["Go back where you're from and reconsider what it means to be a monk...", "How you stand before me now, you will never last as a monk and will be tainted by that which is evil..."])?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    }
                                                    ctx.lines_as(
                                                        "Sensei Moohae",
                                                        args![
                                                            "Will you punishing those who are against",
                                                            "veritas and aequitas? ^CCCCCC(Truth and Justice)^000000"
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    if Val::from(runtime::select_values(ctx, &[Val::from("Yes.:No.")])?) == 2 {
                                                        ctx.lines_as("Sensei Moohae", args!["Who do you think we, the monks are for!", "Any creature that is against the will of such spawns from the dregs of the world!", "They are not worthy to exist!"])?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Sensei Moohae",
                                                            args![
                                                                "Return when you are ready to face and eliminate that which is evil.",
                                                                "Then you will know what you have to do next without my instructions."
                                                            ],
                                                        )?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    }
                                                    ctx.lines_as("Sensei Moohae", args!["Will you cooperate with others who have the same goal as yours and sacrifice yourself as a means to an end?"])?;
                                                    ctx.next()?;
                                                    if Val::from(runtime::select_values(ctx, &[Val::from("Yes.:No.")])?) == 2 {
                                                        ctx.lines_as("Sensei Moohae", args!["Did you say no...? This is unacceptable...", "If you can help your comrades by sacrificing yourself that is a true display of purity."])?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Sensei Moohae", args!["Go back and contemplate upon what it means to sacrifice yourself for those you care for.", "Sacrificing yourself for others may seem easy, but it's the most difficult thing to do as a human being."])?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    }
                                                    ctx.lines_as(
                                                        "Sensei Moohae",
                                                        args!["Will you assist your comrades by gathering monsters to follow you?"],
                                                    )?;
                                                    ctx.next()?;
                                                    if Val::from(runtime::select_values(ctx, &[Val::from("Yes.:No.")])?) == 1 {
                                                        ctx.lines_as("Sensei Moohae", args!["That is not acceptable. Purposely taunting monsters to follow you can be very dangerous and harmful to others. This is not the way of a monk.", "... that behavior is regarded as careless and is not tolerated."])?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Sensei Moohae", args!["Even though you may be nearly invincible when hardening your body that skill is meant to be used for emergency situation not to be used for such disrespectful use!"])?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Sensei Moohae",
                                                            args![
                                                                "You might feel that's helping others, but it's not true.",
                                                                "Consider what it is you must do as a monk for others again."
                                                            ],
                                                        )?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    }
                                                    ctx.lines_as("Sensei Moohae", args!["Will you yell and shout the same things over and over again in towns or in fields?"])?;
                                                    ctx.next()?;
                                                    if Val::from(runtime::select_values(ctx, &[Val::from("Yes.:No.")])?) == 1 {
                                                        ctx.lines_as("Sensei Moohae", args!["You are not allowed to do so. This doesn't apply only to monks but to everyone.", "Nobody wants their peace disturbed!", "Even if you mean well by it, it is disrespectful and not allowed."])?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    }
                                                    ctx.lines_as("Sensei Moohae", args!["Are you willing to die for others on your monk's path of being a holy executioner?"])?;
                                                    ctx.next()?;
                                                    if Val::from(runtime::select_values(ctx, &[Val::from("Yes.:No.")])?) == 2 {
                                                        ctx.lines_as("Sensei Moohae", args!["You cannot become a monk with such an attitude!!!", "If we can eliminate at least one more enemy of ours by sacrificing ourselves, that's what is expected of you as a holy executioner in whom we are trained to be."])?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Sensei Moohae",
                                                            args![
                                                                "If you are unwilling to sacrifice yourself for those you care about,",
                                                                "how can you expect to reach true enlightenment?",
                                                                "Ponder upon the real meaning of life and death!!"
                                                            ],
                                                        )?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    }
                                                    ctx.mes("Lastly, make your oath that you will keep these vows.")?;
                                                    ctx.next()?;
                                                    if Val::from(runtime::select_values(
                                                        ctx,
                                                        &[Val::from(" I vow to keep these oaths.:...eh...no...")],
                                                    )?) == 2
                                                    {
                                                        ctx.lines_as("Sensei Moohae", args![".............."])?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Sensei Moohae", args!["Then your training isn't completed."])?;
                                                        if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                                                            ctx.mes("You will not be accepted as a monk my boy.")?;
                                                        } else {
                                                            ctx.mes("You will not be accepted as a monk little girl.")?;
                                                        }
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Sensei Moohae",
                                                            args![
                                                                "In light of this, your training will start again from the beginning...."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines(args!["Calm down yourself... I reconsidered... perhaps you are simply not ready for the commitment yet.", "Come back later when you're ready..."])?;
                                                        ctx.next()?;
                                                        ctx.mes("[Sensei Moohae]")?;
                                                        if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                                                            ctx.mes(
                                                                "I hope that you are able to realize what you are to become soon my boy...",
                                                            )?;
                                                        } else {
                                                            ctx.mes("I hope that you are able to realize what you are to become soon my girl...")?;
                                                        }
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    }
                                                    ctx.lines_as(
                                                        "Sensei Moohae",
                                                        args!["Then your training is complete...", "Please come closer."],
                                                    )?;
                                                    if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                                                        ctx.mes("We welcome you brother, in our holy battle against evil!")?;
                                                    } else {
                                                        ctx.mes("We welcome you sister, in our holy battle against evil!")?;
                                                    }
                                                    ctx.next()?;
                                                    ctx.mes("[Sensei Moohae]")?;
                                                    if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                                                        ctx.mes("My brother, your oath has been heard by all around us.")?;
                                                    } else {
                                                        ctx.mes("My sister, your oath has been heard by all around us.")?;
                                                    }
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Sensei Moohae",
                                                        args!["I will now perform the ultimate techniques upon your body..."],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Sensei Moohae", args!["I will use these ancient techniques to amplify your strength through the use of pressure points on your body."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Sensei Moohae", args!["Close your eyes........."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Sensei Moohae", args!["And relax your body......."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                        args!["^00CCCC- You breathe in deeply -^000000"],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["^CC0000- You feel fingers poking you all over your body with swiftness -^000000"])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Sensei Moohae", args!["Kiiii~~~Yahahhhhhhh!!!"])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Sensei Moohae", args!["Ooooohaaa!!!"])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Sensei Moohae", args!["Kiii~~~Yahahhhhhhh!!!"])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Sensei Moohae", args!["Haa~ Haa~ Haa~!!!!!"])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Sensei Moohae", args![".... now open your eyes......"])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Sensei Moohae", args!["....and see life through the eyes of a monk."])?;
                                                    ctx.next()?;
                                                    l_jlevel = ctx.var("JobLevel").get()?;
                                                    ctx.call(Function::CompleteQuest, vec![Val::from(3032)])?;
                                                    shared::other_global_functions::job_change(ctx, vec![ctx.constant("JOB_MONK")?])?;
                                                    shared::other_global_functions::f_clearjobvar(ctx, vec![])?;
                                                    ctx.lines_as("Sensei Moohae", args!["....You are a monk."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Sensei Moohae", args!["...heh."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Sensei Moohae", args!["Well...I guess I am too old to do that anymore...I was better when I was younger..."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Sensei Moohae", args!["...anyways, you are a monk now.", "Welcome!"])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Sensei Moohae", args!["I hope you will keep your vow.."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Sensei Moohae",
                                                        args!["continue your training on your path and practice harder."],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Sensei Moohae",
                                                        args![
                                                            "Now...you may leave where the wind may take you.",
                                                            "Oh and I have a gift for you before you leave."
                                                        ],
                                                    )?;
                                                    if l_jlevel.clone() == 50 {
                                                        ctx.call(Function::GetItem, vec![Val::from(1804), Val::from(1)])?;
                                                    } else {
                                                        ctx.call(Function::GetItem, vec![Val::from(1801), Val::from(1)])?;
                                                    }
                                                }
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            } else if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?) {
                                                ctx.lines_as("Sensei Moohae", args!["You are...an acolyte..?", "If you seek consultation, go to the Sanctuary in Prontera. This place is for Monks, not for you.", "Unless you intend to become a monk....please leave."])?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            } else if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_MONK")?) {
                                                ctx.lines_as(
                                                    "Sensei Moohae",
                                                    args![
                                                        "How's your practice going?",
                                                        "I hope you are still training and keeping your vows."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Sensei Moohae",
                                                    args![
                                                        "We must always continue our training in life and stay true to our path.",
                                                        "Otherwise evil will come and taint our mind with impurities."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Sensei Moohae",
                                                    args![
                                                        "Don't forget your vows, stay on your path and",
                                                        "do not let any evil taint your pure heart."
                                                    ],
                                                )?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            } else {
                                                ctx.lines_as("Sensei Moohae", args!["If you seek consultation, go to the Sanctuary in Prontera.", "We do not have anything of interest to you here, please leave and do not disturb the other monks."])?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn sensei_moohae_mk(ctx: &Ctx) -> Script {
    sensei_moohae_mk_body(ctx, Vec::new()).map(|_| ())
}

use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn high_bishop_prst_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_allowhpassist = Val::from(0);
    let mut l_joblvl = Val::from(0);
    if (ctx.var("Upper").get()? == 1 && l_allowhpassist.clone() != 1) {
        ctx.lines_as(
            "Bishop Paul",
            args![
                "Hm...?",
                "Ah, I sense that you are a warrior that has been to Valhalla. You who have been reborn... We are here to look after you."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Bishop Paul", args!["Do not let evil conquer your soul. You have enough courage and power to overcome the hardest situation. May God bless you..."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if !ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?) {
        if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_PRIEST")?) {
            ctx.lines_as("Bishop Paul", args!["Ah..."])?;
            if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                ctx.lines(args![
                    ((Val::from("It is good to see you again, Brother ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                        + Val::from(". Once again, God's grace has caused our paths to cross."))
                ])?;
            } else {
                ctx.lines(args![
                    ((Val::from("It is good to see you once again, Sister ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                        + Val::from(". The grace of God has brought you once more before me."))
                ])?;
            }
            ctx.next()?;
            ctx.lines_as("Bishop Paul", args!["I'm pleased to see that you are continuing to lead the children of God on the right path. Is there anything I can help you with today?"])?;
            ctx.next()?;
            match runtime::select_values(
                ctx,
                &[Val::from(
                    "How is your health?:I want to help this Acolyte.:Father, I need your help.",
                )],
            )? {
                1 => {
                    ctx.lines_as("Bishop Paul", args!["Thank you for your concern. I'm doing fine and am in good health. Please give my regards to your brothers and sisters."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Bishop Paul",
                        args!["Keep in mind that we are God's messengers on this Earth. Always remember that we must always help others."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines_as(
                        "Bishop Paul",
                        args!["Ah, that's a good idea. Helping young Acolytes should also be one of a Priest's priorities."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Bishop Paul", args!["However, there are certain things that an Acolyte must do alone. All Acolytes must complete their divine test by themselves."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Bishop Paul",
                        args!["I hope you will assist your Acolyte friend in the second test, the spiritual training."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Bishop Paul", args!["You need to bring ^0000FF1 Rosary^000000 in order to accompany an Acolyte in spiritual training. If you have one of those, I can send you to the test area now."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Bishop Paul",
                        args!["Do you wish to help him out during the spiritual training?"],
                    )?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("Yes, I do.:Give me a second.")])?) == 1 {
                        if ctx.call(Function::CountItem, vec![Val::from(2608)])?.number()? > 0 {
                            ctx.lines_as(
                                "Bishop Paul",
                                args!["I will now send you to the training place for Acolytes. Please send my regards to Brother Peter..."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Bishop Paul",
                                args!["I hope you will assist this Acolyte in becoming a Priest."],
                            )?;
                            ctx.close_window()?;
                            ctx.call(Function::Warp, vec![Val::from("job_prist"), Val::from(26), Val::from(178)])?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as("Bishop Paul", args!["Unfortunately you didn't bring a ^0000FFRosary^000000. You need one of those in order to be in the testing area."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        "Bishop Paul",
                        args!["I see, take your time. Don't forget to bring a ^0000FFRosary^000000..."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                3 => {
                    ctx.lines_as(
                        "Bishop Paul",
                        args!["You must be strong. Have faith, as you are loved by God. I pray the wounds of the body are healed soon..."],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::PercentHeal, vec![Val::from(90), Val::from(0)])?;
                    ctx.lines_as("Bishop Paul", args!["God, please look after your poor children. Help them overcome their hardships and difficulties. Refresh their spirits..."])?;
                    ctx.next()?;
                    ctx.call(Function::PercentHeal, vec![Val::from(0), Val::from(90)])?;
                    ctx.lines_as(
                        "Bishop Paul",
                        args![
                            "I hope my invocation has eased your pain. Now please go forth and spread God's message. May God be with you..."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        } else if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_NOVICE")?) {
            ctx.lines_as("Bishop Paul", args!["May God be"])?;
            if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                ctx.mes("with you, brother.")?;
            } else {
                ctx.mes("with you, sister.")?;
            }
            ctx.next()?;
            ctx.lines_as("Bishop Paul", args!["You are in", "the Sanctuary.", "What brings you here?"])?;
            ctx.next()?;
            match runtime::select_values(
                ctx,
                &[Val::from("I want to be an Acolyte.:I want to be a Priest.:Nothing, really.")],
            )? {
                1 => {
                    ctx.lines_as(
                        "Bishop Paul",
                        args!["Oh I see...", "If you wish to become an Acolyte, please visit the other room."],
                    )?;
                }
                2 => {
                    ctx.lines_as(
                        "Bishop Paul",
                        args!["Oh I see. However, you must first become an Acolyte before becoming a Priest. Please visit the other room."],
                    )?;
                }
                3 => {
                    ctx.lines_as(
                        "Bishop Paul",
                        args!["Please make yourself at home. On Earth, nowhere is safer than this Sanctuary."],
                    )?;
                }
                _ => {}
            }
            ctx.next()?;
            ctx.lines_as("Bishop Paul", args!["May God bless you."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as("Bishop Paul", args!["May God be"])?;
            if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                ctx.mes("with you, brother.")?;
            } else {
                ctx.mes("with you, sister.")?;
            }
            ctx.next()?;
            ctx.lines_as("Bishop Paul", args!["What brings you here", "to Prontera Sanctuary?"])?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Information about Priests.:Nothing.")])? {
                1 => {
                    ctx.lines_as(
                        "Bishop Paul",
                        args!["Priests have the authority to perform and administer religious rites."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Bishop Paul",
                        args![
                            "You must first be thoroughly disciplined as an Acolyte before you can be promoted to the position of Priest."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Bishop Paul",
                        args!["When you reach Acolyte Job level 40, you will be able to apply for the Priest test."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Bishop Paul", args!["If you pass the test, you will be able to use more powerful skills that will be effective against Demon and Undead creatures..."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Bishop Paul",
                        args!["With all of your ability, you will play an important role in towns and dungeons."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Bishop Paul",
                        args!["Our duty and obligation as Priests is to devote ourselves to helping others without expecting reward."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Bishop Paul", args!["As we help others, we must not expect to treat us in a similar fashion. To be a great Priest is your choice and responsibility, not anyone else's."])?;
                    ctx.next()?;
                    ctx.lines_as("Bishop Paul", args!["However, those who receive should be polite. You should give an outstanding example, but you should also have your limits as a human."])?;
                    ctx.next()?;
                    ctx.lines_as("Bishop Paul", args!["I hope I explained enough of the class. Why don't you go outside and talk to some of the other Priests if you want to learn more about our way of life?"])?;
                    ctx.next()?;
                }
                2 => {
                    ctx.lines_as(
                        "Bishop Paul",
                        args!["Please make yourself at home. Nowhere on Earth is safer than the Prontera Sanctuary."],
                    )?;
                    ctx.next()?;
                }
                _ => {}
            }
            ctx.lines_as("Bishop Paul", args!["Well...", "May God", "bless you."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if ctx.var("priest_q").get()? == 0 {
        ctx.lines_as("Bishop Paul", args!["May God bless"])?;
        if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
            ctx.mes("you, Brother.")?;
        } else {
            ctx.mes("you, Sister.")?;
        }
        ctx.lines(args!["What brings", "you to me?"])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("I want to be a Priest.:How are you, Father?")])? {
            1 => {
                ctx.lines_as(
                    "Bishop Paul",
                    args!["I see. So you wish to be a Priest. God will be delighted by your decision and will bless you."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Bishop Paul",
                    args![
                        "I am Bishop Paul Cervantes, and am in charge of the Prontera Parish.",
                        "I am glad to meet a person as eager and devoted to God such as yourself."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Bishop Paul", args!["If you set your mind on becoming a Priest, you must undergo several tests. Only Acolytes who reach job level 40 are qualified for testing."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Bishop Paul",
                    args!["If you satisfy the requirements, I suggest that you apply for the Priest job first. Do you wish to apply now?"],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("Yes, I do.:I need some time to think about it...")],
                )?) == 1
                {
                    if ctx.var("JobLevel").get()?.number()? < 40 {
                        ctx.lines_as(
                            "Bishop Paul",
                            args!["You are not yet qualified to be a Priest. Please go out into the world and broaden your experiences."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Bishop Paul", args!["There are still things that you must learn as an Acolyte. However, I look forward to meeting you again very soon."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    if ctx.var("SkillPoint").get()?.is_true() {
                        ctx.lines_as("Bishop Paul", args!["You have skill points left.", "I strongly recommend that you use all of these skill points before you apply for the Priest job change test."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.var("priest_q").set(Val::from(1))?;
                    ctx.call(Function::SetQuest, vec![Val::from(8009)])?;
                    ctx.mes("[Bishop Paul]")?;
                    if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                        ctx.lines(args![((Val::from("Now I will explain the Three Trials of Priesthood. These tribulations will bring you much suffering, but I hope you can complete them, Brother ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("."))])?;
                    } else {
                        ctx.lines(args![((Val::from("Now I will explain the Three Trials of Priesthood. These tribulations will bring you much suffering, but I hope you can complete them, Sister ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("."))])?;
                    }
                    ctx.next()?;
                    ctx.lines_as(
                        "Bishop Paul",
                        args!["For the First Trial, you will make a pilgrimage, and visit three acscetic Priests in a specific order."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Bishop Paul",
                        args![
                            "The Second Trial will consist of spiritual training. You must resist the temptations of Demons and the Undead."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Bishop Paul", args!["In the Final Trial, you will promise your devotion to God. Your willingness to sacrifice yourself will also be questioned."])?;
                    ctx.next()?;
                    ctx.lines_as("Bishop Paul", args!["Acolytes that have reached Job Level 50 will be exempt from the First Trial, the pilgrimage, as they have already demonstrated their enthusiasm and devotion."])?;
                    ctx.next()?;
                    if ctx.var("JobLevel").get()? == 50 {
                        ctx.lines_as(
                            "Bishop Paul",
                            args![
                                "I can see the great effort you have exerted to reach job level 50. You have been a loyal servant to God."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Bishop Paul", args!["Now, you may go directly go to the Second Trial: Spiritual Training. For this training, you may bring a Priest with you."])?;
                        ctx.next()?;
                        ctx.lines_as("Bishop Paul", args!["I have no doubt that you will do a good job by yourself. However, it will be easier with the aid of a Brother or Sister that has already become a Priest."])?;
                        ctx.next()?;
                        ctx.var("priest_q").set(Val::from(5))?;
                        ctx.call(Function::ChangeQuest, vec![Val::from(8009), Val::from(8011)])?;
                        ctx.lines_as("Bishop Paul", args!["Well, are you ready for the Spiritual Training?"])?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(ctx, &[Val::from("I am ready.:Give me a minute.")])?) == 1 {
                            ctx.lines_as("Bishop Paul", args!["Good. I will send you to the training ground. When you get there, please speak to Brother Peter who is in charge of the training."])?;
                            ctx.next()?;
                            ctx.call(Function::Warp, vec![Val::from("job_prist"), Val::from(24), Val::from(180)])?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as(
                            "Bishop Paul",
                            args![
                                "No problem, take your time.",
                                "May God give you the strength to overcome your fears..."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        "Bishop Paul",
                        args!["Well, let me tell you the order of the ascetic Priests that you must visit for your pilgrimage."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Bishop Paul",
                        args![
                            "First, please visit Father",
                            "Rubalkabara who is Northeast",
                            "of the Prontera Ruins."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Bishop Paul",
                        args![
                            "Second, please visit Sister Mathilda. She is located in",
                            "an area near Morocc, Southwest of Prontera."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Bishop Paul",
                        args!["The third Priest you must visit is Father Yosuke. He is in a field Northwest of Prontera."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Bishop Paul",
                        args!["Well then, I wish you a safe journey. If you have any questions, please ask Sister Cecilia."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Bishop Paul",
                        args!["When you return from your pilgrimage, I will let you know", "of the next test."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Bishop Paul", args!["May God", "bless you..."])?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(8009), Val::from(8010)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Bishop Paul",
                    args!["Please take your time.", "You are always welcomed.", "May God bless you..."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Bishop Paul",
                    args!["I see...", "I am doing fine", "and am in good health.", "Thank you for asking."],
                )?;
                ctx.next()?;
                ctx.mes("[Bishop Paul]")?;
                if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                    ctx.mes("I hope you will continue to go on your mission as God's servant, brother.")?;
                } else {
                    ctx.mes("I hope you will continue to go on your mission as God's servant, sister.")?;
                }
                ctx.next()?;
                ctx.lines_as(
                    "Bishop Paul",
                    args!["Hopefully, our paths", "will cross again.", "May God bless you..."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else {
        if ctx.var("priest_q").get()? == 1 {
            ctx.lines_as(
                "Bishop Paul",
                args!["May I ask you the reason you're still here? You didn't forget your pilgrimage, did you?"],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Sorry father, I need to check the order.:No no no, not at all.")],
            )?) == 1
            {
                ctx.lines_as(
                    "Bishop Paul",
                    args!["Ah, I see. I will let you know the order of pilgrimage again, and hope that you will have a safe journey."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Bishop Paul",
                    args!["First, meet Father Rubalkabara. He's at the Northeast of the Prontera ruins."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Bishop Paul",
                    args!["Then, remember to meet Sister Mathilda. She's somewhere near the town of Morocc, Southwest of Prontera."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Bishop Paul",
                    args!["And lastly, please seek out Father Yosuke. He is in the a field Northwest of Prontera."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Bishop Paul",
                    args!["Well then, I shall pray for your safe journey. If you want more information, please ask Sister Cecilia."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Bishop Paul",
                    args!["When you get back from the pilgrimage, I will let you know the next test."],
                )?;
                ctx.next()?;
                ctx.lines_as("Bishop Paul", args!["May God bless you..."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as("Bishop Paul", args!["I see. But still, if you have any questions, you may wish to ask Sister Cecilia. She will address any of your concerns."])?;
            ctx.next()?;
            ctx.lines_as(
                "Bishop Paul",
                args!["Well then, I shall pray for your safe journey. May God bless you..."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("priest_q").get()? == 2 {
                ctx.lines_as("Bishop Paul", args!["I see you have returned from your meeting with Father Rubalkabara. How is he doing? I am worried about his health, since he's been there all alone... "])?;
                ctx.next()?;
                ctx.lines_as(
                    "Bishop Paul",
                    args!["For your next quest, you should meet Sister Mathilda. I shall be awaiting your safe return."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("priest_q").get()? == 3 {
                    ctx.lines_as("Bishop Paul", args!["I see that you have returned from your journey to meet Sister Mathilda. She has been meditating in the hot, dry desert for a long time."])?;
                    ctx.next()?;
                    ctx.lines_as("Bishop Paul", args!["Finally, it is now time for you to meet Father Yosuke. He is doing penance somewhere around a field Northwest of Prontera. Please seek him out, and then return here to me."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("priest_q").get()? == 4 {
                        ctx.var("priest_q").set(Val::from(5))?;
                        ctx.call(Function::ChangeQuest, vec![Val::from(8010), Val::from(8011)])?;
                        ctx.lines_as(
                            "Bishop Paul",
                            args!["You've accomplished", "your pilgrimage.", "Congratulations."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Bishop Paul", args!["Now it is time to begin your spiritual training. As I mentioned before, you may bring a Priest to help you during this training."])?;
                        ctx.next()?;
                        ctx.lines_as("Bishop Paul", args!["Although you cannot receive their help throughout all of the testing, they can at least help you during the spiritual training."])?;
                        ctx.next()?;
                        ctx.lines_as("Bishop Paul", args!["Well, are you ready for", "the spiritual training?"])?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(ctx, &[Val::from("I'm ready.:Give me a minute.")])?) == 1 {
                            ctx.lines_as("Bishop Paul", args!["Good. I will send you to the training ground. When you get there, please speak to Brother Peter who is in charge of the training."])?;
                            ctx.next()?;
                            ctx.call(Function::Warp, vec![Val::from("job_prist"), Val::from(24), Val::from(180)])?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as(
                            "Bishop Paul",
                            args![
                                "No problem,",
                                "take your time.",
                                "May God grant you",
                                "the strength to",
                                "overcome your fears..."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if ctx.var("priest_q").get()? == 5 {
                            ctx.lines_as(
                                "Bishop Paul",
                                args!["You seem confident about the spiritual training. Shall we begin?"],
                            )?;
                            ctx.next()?;
                            if Val::from(runtime::select_values(ctx, &[Val::from("I'm ready.:Give me a minute.")])?) == 1 {
                                ctx.lines_as("Bishop Paul", args!["Good. I will send you to the training ground. When you get there, please speak to Brother Peter who is in charge of the training."])?;
                                ctx.next()?;
                                ctx.call(Function::Warp, vec![Val::from("job_prist"), Val::from(24), Val::from(180)])?;
                                return Err(Stop::End);
                            }
                            ctx.lines_as(
                                "Bishop Paul",
                                args![
                                    "No problem,",
                                    "take your time.",
                                    "May God grant you",
                                    "the strength to",
                                    "overcome your fears..."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.var("priest_q").get()? == 6 {
                            ctx.lines_as(
                                "Bishop Paul",
                                args![
                                    "You look tired and exhausted. However, you must endure even more suffering once you become a Priest."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Bishop Paul", args!["Please endure these trials for the sake of your dream. Why don't you challenge the spiritual training again?"])?;
                            ctx.next()?;
                            if Val::from(runtime::select_values(ctx, &[Val::from("I'll try again.:Give me a minute.")])?) == 1 {
                                ctx.lines_as(
                                    "Bishop Paul",
                                    args!["Good. I will send you to the training ground. Please ask for help from Brother Peter."],
                                )?;
                                ctx.next()?;
                                ctx.call(Function::Warp, vec![Val::from("job_prist"), Val::from(24), Val::from(180)])?;
                                return Err(Stop::End);
                            }
                            ctx.lines_as(
                                "Bishop Paul",
                                args![
                                    "No problem,",
                                    "take your time.",
                                    "May God grant you",
                                    "the strength to",
                                    "overcome your fears..."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.var("priest_q").get()? == 7 {
                            ctx.lines_as("Bishop Paul", args!["I am glad that you've done well with the spiritual training. Congratulations. You are now qualified to be called a Priest."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Bishop Paul",
                                args!["Now, you must go and swear your devotion to God with Sister Cecilia. Don't be nervous..."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Bishop Paul",
                                args!["Just answer honestly, and listen to the voice of God that speaks quietly in your heart."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Bishop Paul", args!["Well then...", "I will be here", "waiting for you."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.var("priest_q").get()? == 8 {
                            ctx.lines_as("Bishop Paul", args!["Hmm? You haven't made your oath yet...? Without the conviction of an oath to God, you may be tempted by evil at anytime."])?;
                            ctx.next()?;
                            ctx.lines_as("Bishop Paul", args!["You should go to sister Cecilia and promise your devotion to God. Return here with honor, and listen to the voice of God that speaks quietly in your heart."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.var("priest_q").get()? == 9 {
                            if ctx.var("SkillPoint").get()?.is_true() {
                                ctx.lines_as("Bishop Paul", args!["You have remaining skills points. Please use these skill points to upgrade your skills, and then return to me."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.lines_as("Bishop Paul", args!["Congratulations, you have completed the trials required of all Priests. Let me promote you to the position of Priest right away."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Bishop Paul",
                                args!["God, grant your power to your servant standing before you."],
                            )?;
                            ctx.call(Function::ChangeQuest, vec![Val::from(8015), Val::from(8016)])?;
                            if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                                ctx.mes("Let him send your message throughout the ends of the earth.")?;
                            } else {
                                ctx.mes("Let her send your message throughout the ends of the earth.")?;
                            }
                            ctx.next()?;
                            ctx.lines_as(
                                "Bishop Paul",
                                args!["Make this servant of yours an instrument of your miraculous works..."],
                            )?;
                            ctx.next()?;
                            l_joblvl = ctx.var("JobLevel").get()?;
                            ctx.call(Function::CompleteQuest, vec![Val::from(8016)])?;
                            shared::other_global_functions::job_change(ctx, vec![ctx.constant("JOB_PRIEST")?])?;
                            shared::other_global_functions::f_clearjobvar(ctx, vec![])?;
                            ctx.lines_as("Bishop Paul", args!["Now you are born again as a Priest. I congratulate you, and hope you will greatly help other people for the rest of your life."])?;
                            ctx.next()?;
                            ctx.mes("[Bishop Paul]")?;
                            if l_joblvl.clone().number()? < 50 {
                                ctx.call(Function::GetItem, vec![Val::from(1550), Val::from(1)])?;
                                ctx.mes("This book is for you. I hope it will aid you in spreading God's message on earth.")?;
                            } else {
                                ctx.call(Function::GetItem, vec![Val::from(1551), Val::from(1)])?;
                                ctx.mes("In commemoration of your job change, I am giving you a bible. This will lighten your way to the path of righteousness.")?;
                            }
                            ctx.next()?;
                            ctx.lines_as("Bishop Paul", args!["You've shown great effort, and have made admirable progress in your personal quest for holiness. Please lead your life as a sincere Priest..."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                }
            }
        }
    }
    Ok(Val::from(0))
}

pub fn high_bishop_prst(ctx: &Ctx) -> Script {
    high_bishop_prst_body(ctx, Vec::new()).map(|_| ())
}

fn sister_cecilia_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.mes("[Sister Cecilia]")?;
    if !ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?) {
        if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_PRIEST")?) {
            ctx.lines(args![
                ((Val::from("May god bless you, ")
                    + (if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                        Val::from("brother")
                    } else {
                        Val::from("sister")
                    }))
                    + Val::from(". It brings my heart joy to see that you working hard to carry out the will of God."))
            ])?;
        } else if ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_NOVICE")?) {
            ctx.lines(args![
                ((Val::from("May god bless you, ")
                    + (if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                        Val::from("brother")
                    } else {
                        Val::from("sister")
                    }))
                    + Val::from(".")),
                "Prontera parish welcomes you."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Sister Cecilia",
                args!["Oh, you haven't chosen a job yet? Why don't you consider devoting your life to God?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sister Cecilia",
                args!["You can lead a fulfilling life as an Acolyte, helping out other people in need."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sister Cecilia",
                args![
                    "If you're interested, please ask the Priest in the other room. You won't ever regret the choice to become an Acolyte."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sister Cecilia",
                args!["When you reach Job level 40 as an Acolyte, you can be promoted to a Priest."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sister Cecilia",
                args!["But please...", "Take your time, and decide what job will be the best for you."],
            )?;
        } else {
            ctx.lines(args![
                ((Val::from("May god bless you, ")
                    + (if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                        Val::from("brother")
                    } else {
                        Val::from("sister")
                    }))
                    + Val::from(".")),
                "Welcome to Prontera parish. How may I help you?"
            ])?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Tell me more about Priests.:Nothing.")],
            )?) == 1
            {
                ctx.lines_as("Sister Cecilia", args!["Messengers of God are usually known as Priests. After becoming an Acolyte, you can train with the goal of becoming a Priest."])?;
                ctx.next()?;
                ctx.lines_as("Sister Cecilia", args!["Servants of God are prohibited to use weapons based on blades. For us, the meaning of battle with monsters is not in the killing, but in the enlightening of their souls."])?;
            } else {
                ctx.lines_as(
                    "Sister Cecilia",
                    args![
                        "I see. Well, feel free to relax and make yourself at home. Nowhere on earth is safer than the Prontera Sanctuary."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Sister Cecilia", args!["May God bless you..."])?;
            }
        }
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("priest_q").get()? == 0 {
        if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
            ctx.mes("May God bless you, brother.")?;
        } else {
            ctx.mes("May God bless you, sister.")?;
        }
        ctx.mes("May I ask what brings you here?")?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("I wish to become a Priest.:Nothing.")])? {
            1 => {
                ctx.lines_as("Sister Cecilia", args!["I see. You've devoted yourself to God. Many Acolytes wish to become Priests to continue on their personal journey towards holiness."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Sister Cecilia",
                    args!["Let me introduce myself. I am Cecilia Margarita, and I am in charge of part of the Priest job change process."],
                )?;
                ctx.next()?;
                ctx.lines_as("Sister Cecilia", args!["I've been supporting many people in becoming Priests ever since I joined the Prontera Parish. That is one of my main responsibilities."])?;
                ctx.next()?;
                ctx.lines_as("Sister Cecilia", args!["In order to become a Priest, you must complete 3 trials. A pilgrimage, a session of spiritual training, and an oath of devotion to God."])?;
                ctx.next()?;
                ctx.lines_as("Sister Cecilia", args!["If you wish to become a servant of God, please apply for the Priest job with Bishop Paul, and complete all 3 trials."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Sister Cecilia",
                    args![
                        "If you experience a problem during any of your trials, feel free to visit me. I will help you as much as I can."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Sister Cecilia",
                    args!["Make yourself at home. I insist that you recover and take a rest in this Sanctuary. May God bless you..."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else {
        if ctx.var("priest_q").get()? == 1 {
            ctx.mes("Ah, you've started your pilgrimage. Please do your best to accomplish this first trial.")?;
            ctx.next()?;
            ctx.lines_as(
                "Sister Cecilia",
                args!["The first Priest you must meet is Father Rubalkabara. He is in the ruins Northeast of Prontera."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sister Cecilia",
                args!["Travel one field North from Prontera, and then three fields East, and you will arrive at the ruins."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sister Cecilia",
                args!["Of course, you can also head 1 field East from Prontera, then go 1 field north, and then go 2 fields East..."],
            )?;
            ctx.next()?;
            ctx.lines_as("Sister Cecilia", args!["Father Rubalkabara will be at the entrance of the Prontera Ruins. Be careful. That place is a habitat for aggressive Chocos."])?;
            ctx.next()?;
            ctx.lines_as("Sister Cecilia", args!["After meeting Father Rubalkabara, please visit Sister Mathilda and Father Yosuke. You can check your quest progress with me if you have any questions later."])?;
            ctx.next()?;
            ctx.lines_as("Sister Cecilia", args!["Well then, have a good journey. Please don't give up to short lived tribulations, and I hope that you accomplish your goals."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("priest_q").get()? == 2 {
                ctx.mes(
                    "Oh, you've met Father Rubalkabara. Now it's time for you to visit Sister Mathilda. She is near a town named Morocc.",
                )?;
                ctx.next()?;
                ctx.lines_as("Sister Cecilia", args!["She has been training her religious discipline somewhere in a field North of Morocc. If you look around that field, you will be able to find her."])?;
                ctx.next()?;
                ctx.lines_as("Sister Cecilia", args!["Of course, sometimes I want to devote myself to training like those other Priests, but I have my duty to assist those Acolytes applying for the Priest job."])?;
                ctx.next()?;
                ctx.lines_as("Sister Cecilia", args!["But I believe this is God's will, and that this is the work he has intended me to do as his servant. Have a safe journey, and come back safely."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("priest_q").get()? == 3 {
                    ctx.mes("Now, the final Priest that you must meet is Father Yosuke. I've heard that he is training near a lake located Northwest of Prontera.")?;
                    ctx.next()?;
                    ctx.lines_as("Sister Cecilia", args!["From Prontera, travel one field North, and then two fields towards the West. You may also travel two fields West first, and then travel one field to the North."])?;
                    ctx.next()?;
                    ctx.lines_as("Sister Cecilia", args!["Although there are still two trials awaiting you, I have faith that you will be able to accomplish your goal of becoming a Priest..."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("priest_q").get()? == 4 {
                        ctx.mes("Welcome. You demonstrated great effort to accomplish your first trial. Now, speak to Bishop Paul so that you can begin your next trial on your path to Priesthood.")?;
                        ctx.next()?;
                        ctx.mes("[Sister Cecilia]")?;
                        if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                            ctx.lines(args![
                                ((Val::from("Brother ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("..."))
                            ])?;
                        } else {
                            ctx.lines(args![
                                ((Val::from("Sister ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("..."))
                            ])?;
                        }
                        ctx.mes("The spiritual training is much more difficult than the pilgrimage, but I believe in you.")?;
                        ctx.next()?;
                        ctx.lines_as("Sister Cecilia", args!["I hope that you find someone who has already become a Priest to help during the spiritual training. Good luck, and have faith."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("priest_q").get()? == 5 {
                        ctx.mes("Oh, you haven't finished the spiritual training yet?")?;
                        ctx.next()?;
                        ctx.lines_as("Sister Cecilia", args!["I cannot let you know the specific details, but as long as you believe in yourself and have faith in all that is good, you will succeed."])?;
                        ctx.next()?;
                        ctx.lines_as("Sister Cecilia", args!["Please speak to Father Peter in the test hall for more details. He is a close friend of Bishop Paul and may give you some useful tips for the spiritual training."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("priest_q").get()? == 6 {
                        ctx.mes("Yes, I understand that you've been through a really difficult situation. However, do not give up and succumb to temptation. You must be able to resist evil to become a Priest.")?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Sister Cecilia",
                            args![
                                "If you know somebody who has already become a Priest, ask them to help you during your spiritual training."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Sister Cecilia",
                            args!["May God give you guidance and protection. When you complete your training, please come back to me."],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if (ctx.var("priest_q").get()? == 7 || ctx.var("priest_q").get()? == 8) {
                        if ctx.var("priest_q").get()? == 7 {
                            if ctx.call(Function::CheckQuest, vec![Val::from(8014)])? == -1 {
                                ctx.call(Function::ChangeQuest, vec![Val::from(8013), Val::from(8014)])?;
                            }
                            ctx.mes(
                                "Welcome! I'm so glad to see you've come back! Now, there is one last trial left for you to complete.",
                            )?;
                        } else if ctx.var("priest_q").get()? == 8 {
                            ctx.mes("...")?;
                            ctx.next()?;
                            ctx.lines_as("Sister Cecilia", args!["Welcome back.", "I hope that you've reflected on what you've said last time, and that you now have the attitude to become a Priest."])?;
                        }
                        ctx.next()?;
                        ctx.mes("[Sister Cecilia]")?;
                        if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                            ctx.lines(args![
                                ((Val::from("Brother ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("..."))
                            ])?;
                        } else {
                            ctx.lines(args![
                                ((Val::from("Sister ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("..."))
                            ])?;
                        }
                        ctx.mes("We will now begin your formal oath for the Priesthood. Make yourself comfortable, and just answer with your heart.")?;
                        ctx.next()?;
                        ctx.mes("[Sister Cecilia]")?;
                        if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                            ctx.lines(args![
                                ((Val::from("Brother ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(","))
                            ])?;
                        } else {
                            ctx.lines(args![
                                ((Val::from("Sister ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(","))
                            ])?;
                        }
                        ctx.lines(args!["Are you willing", "to give your life to God?"])?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(ctx, &[Val::from("Yes.:No!")])?) == 2 {
                            ctx.lines_as(
                                "Sister Cecilia",
                                args!["Aw...? How could you give me that kind of answer? I assume you're not ready to be a Priest yet..."],
                            )?;
                            ctx.next()?;
                            ctx.var("priest_q").set(Val::from(8))?;
                            ctx.lines_as("Sister Cecilia", args!["You should reflect a little more on the teachings of holiness and come back later. You can't be a Priest if your spirit is weak."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as(
                            "Sister Cecilia",
                            args!["Will you take advantage of the holy abilities given by God for selfish, destructive or greedy ends?"],
                        )?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(ctx, &[Val::from("Yes.:No.")])?) == 1 {
                            ctx.lines_as("Sister Cecilia", args!["Aw...? God won't grant you the power of holiness if your goals aren't just and pure. Meditate on your motivations for a while, and then come back to me."])?;
                            ctx.next()?;
                            ctx.var("priest_q").set(Val::from(8))?;
                            ctx.lines_as("Sister Cecilia", args!["Think about the qualities that make Priests people of respect. You can't be a Priest if your spirit is not in accordance with God."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as(
                            "Sister Cecilia",
                            args!["Will you help aid others, even complete strangers, in battles by easing their suffering?"],
                        )?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(ctx, &[Val::from("Yes.:No.")])?) == 2 {
                            ctx.lines_as("Sister Cecilia", args!["No, no. You've got the wrong idea. God authorizes us to use his power to support his children. You must help people in danger: it is your obligation."])?;
                            ctx.next()?;
                            ctx.var("priest_q").set(Val::from(8))?;
                            ctx.lines_as("Sister Cecilia", args!["Go and observe the adventurers that are fighting for peace in this world. They will teach you what you must do in order to help them."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as(
                            "Sister Cecilia",
                            args!["Are you willing to sacrifice yourself for the sake of others?"],
                        )?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(ctx, &[Val::from("Yes.:No.")])?) == 2 {
                            ctx.lines_as("Sister Cecilia", args!["How can you say no...? That's one of the basic principles of Priesthood. You must value the welfare of others over your own safety."])?;
                            ctx.next()?;
                            ctx.var("priest_q").set(Val::from(8))?;
                            ctx.lines_as("Sister Cecilia", args!["Go and think about the value of suffering and the meaning of sacrifice. When you think you understand more about helping those in need, come back to me."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as(
                            "Sister Cecilia",
                            args!["Will you repeatly say the same phrase in public in order to send God's message to his children?"],
                        )?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(ctx, &[Val::from("Yes.:No.")])?) == 1 {
                            ctx.lines_as("Sister Cecilia", args!["No no no... You've got it wrong. Even though your purpose is to spread God's message, no one will eagerly accept what you say when you spam text."])?;
                            ctx.next()?;
                            ctx.var("priest_q").set(Val::from(8))?;
                            ctx.lines_as("Sister Cecilia", args!["Remember...", "You must be a moral person, and display maturity and respect to other players. This kind of attitude applies for all classes,", "I believe."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as(
                            "Sister Cecilia",
                            args!["Will you lure many monsters to help your party members level up?"],
                        )?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(ctx, &[Val::from("Yes.:No.")])?) == 1 {
                            ctx.lines_as("Sister Cecilia", args!["No, you won't. Luring many monsters does more harm than good. There is no exception. That behavior is totally unacceptable."])?;
                            ctx.next()?;
                            ctx.var("priest_q").set(Val::from(8))?;
                            ctx.lines_as("Sister Cecilia", args!["Even if it looks like you are aiding your party members, such action results in bad karma. Please reflect on that for a while."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as(
                            "Sister Cecilia",
                            args!["Will you follow God, no matter what it takes, even if he demands you to kill yourself?"],
                        )?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(ctx, &[Val::from("Yes.:No.")])?) == 2 {
                            ctx.lines_as("Sister Cecilia", args!["With that spirit, you can't be a Priest. If it is God's will to sacrifice yourself for a good purpose, you must carry out God's will as his servant."])?;
                            ctx.next()?;
                            ctx.var("priest_q").set(Val::from(8))?;
                            ctx.lines_as("Sister Cecilia", args!["Besides, God has also given Priests the resurrection power. Think about the meaning of life and death again, and then come back to me."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.mes("[Sister Cecilia]")?;
                        if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                            ctx.lines(args![
                                ((Val::from("Brother ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("..."))
                            ])?;
                        } else {
                            ctx.lines(args![
                                ((Val::from("Sister ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("..."))
                            ])?;
                        }
                        ctx.mes("You have demonstrated your devotion to God. Will you swear to adhere to his teachings for the rest of your days?")?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(ctx, &[Val::from("I do.:No.")])?) == 1 {
                            ctx.var("priest_q").set(Val::from(9))?;
                            ctx.call(Function::ChangeQuest, vec![Val::from(8014), Val::from(8015)])?;
                            ctx.lines_as("Sister Cecilia", args!["Now, you have completed your oath of Priesthood and accomplished all three trials required to become a Priest."])?;
                            ctx.next()?;
                            ctx.lines_as("Sister Cecilia", args!["Now go to Bishop Paul. And remember, we are all brothers and sisters in the eyes of God. Peace be with you..."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as("Sister Cecilia", args!["..."])?;
                        ctx.next()?;
                        ctx.lines_as("Sister Cecilia", args!["...", "......"])?;
                        ctx.next()?;
                        ctx.var("priest_q").set(Val::from(8))?;
                        ctx.lines_as(
                            "Sister Cecilia",
                            args!["You've come so far...", "Why would you want", "to throw this all away...?"],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("priest_q").get()? == 9 {
                        ctx.lines(args![
                            "Congratulations.",
                            "You've completed all three trials required for the Priesthood. Bishop Paul is now waiting for you."
                        ])?;
                        ctx.next()?;
                        ctx.lines_as("Sister Cecilia", args!["Peace be with you..."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
            }
        }
    }
    Ok(Val::from(0))
}

pub fn sister_cecilia(ctx: &Ctx) -> Script {
    sister_cecilia_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum PeterSAlbertoStep {
    Start,
    OnEnable,
    OnDisable,
}

fn peter_s_alberto_run(ctx: &Ctx, mut step: PeterSAlbertoStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            PeterSAlbertoStep::Start => {
                ctx.mes("[Father Peter]")?;
                if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_PRIEST")?) {
                    ctx.mes("Welcome!")?;
                    if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                        ctx.lines(args![
                            ((Val::from("Brother ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("!"))
                        ])?;
                    } else {
                        ctx.lines(args![
                            ((Val::from("Sister ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("!"))
                        ])?;
                    }
                    ctx.mes("So good to see you again!")?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Father Peter",
                        args![
                            "Are you here to help an Acolyte friend for the spiritual training? That's great~ I think you'll do a good job."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Father Peter",
                        args!["Remember, no matter how much you want to help this Acolyte, this is not your quest."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Father Peter",
                        args!["You may assist and lighten your friend's burden, but you take upon this task for yourself."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Father Peter", args!["So...", "Are you gonna help him right now?"])?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Yes, I am.:Give me a minute.:I changed my mind.")])? {
                        1 => {
                            ctx.lines_as(
                                "Father Peter",
                                args!["Go for it! As your Acolyte enters, the test will begin. Now, I will send you to the testing area."],
                            )?;
                            ctx.close_window()?;
                            ctx.call(Function::Warp, vec![Val::from("job_prist"), Val::from(24), Val::from(44)])?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as(
                                "Father Peter",
                                args![
                                    "Hm...?",
                                    "What for?",
                                    "Well, so long as you arrive in time to help your friend, it will be okay."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        3 => {
                            ctx.lines_as(
                                "Father Peter",
                                args!["Oh...?", "Then please,", "go ahead. God bless", "you, and take care!"],
                            )?;
                            ctx.close_window()?;
                            ctx.call(Function::Warp, vec![Val::from("prontera"), Val::from(234), Val::from(318)])?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                }
                if ctx.var("priest_q").get()? == 5 {
                    ctx.lines(args!["Welcome~!", "I congratulate you", "for passing the first trail."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Father Peter",
                        args![
                            "My name is",
                            "Peter S. Alberto.",
                            "How is my buddy Paul?",
                            "Is he doing alright",
                            "these days?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Father Peter", args!["Oh, I keep forgetting that he was promoted to Bishop. I think I'm supposed to call him Bishop Paul, or 'His Excellency.' Haha~"])?;
                    ctx.next()?;
                    ctx.lines_as("Father Peter", args!["Anyway, let me give you a brief explanation of the spiritual training. Are you familiar with what the spiritual training is for Priests?"])?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Yes, I do.:Sorry...")])? {
                        1 => {
                            ctx.lines_as("Father Peter", args!["Haha, I like you! But it never hurts to have too much information. The more well informed you are, the more easily you'll pass the test!"])?;
                            ctx.next()?;
                        }
                        2 => {
                            ctx.lines_as(
                                "Father Peter",
                                args!["Oh, no need to be sorry. I'm here to give you the information you need anyway. So, don't worry."],
                            )?;
                            ctx.next()?;
                        }
                        _ => {}
                    }
                    ctx.lines_as("Father Peter", args!["In spiritual training, you will be defeating evil creatures. Creatures of the Undead and Demons are all evil. In choosing to serve darkness, they are our enemies!"])?;
                    ctx.next()?;
                    ctx.lines_as("Father Peter", args!["There are too many evil creatures that roam this world against the will of God. Innocents suffer as a result of their malignance."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Father Peter",
                        args!["We, as Priests, are obligated to exterminate all those creatures, thus spreading love and peace."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Father Peter", args!["This training will test your ability to eliminate evil. Since this trial is pretty difficult to be accomplished by yourself,", "I recommend getting help from a Priest if you can."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Father Peter",
                        args!["If you are close to a Priest, you'd better ask him to assist you during this trial. Now, are you ready?"],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("I'm ready.:Please hold on.:I want to go back.")])? {
                        1 => {
                            ctx.lines_as("Father Peter", args!["Now, let the spiritual training begin. It's simple. Just kill them all. Show no mercy to the creatures of darkness!"])?;
                            ctx.next()?;
                            ctx.lines_as("Father Peter", args!["Now...", "Go for it!"])?;
                            ctx.close_window()?;
                            if ctx.call(Function::CheckQuest, vec![Val::from(8012)])? == -1 {
                                ctx.call(Function::ChangeQuest, vec![Val::from(8011), Val::from(8012)])?;
                            }
                            ctx.call(Function::Warp, vec![Val::from("job_prist"), Val::from(24), Val::from(44)])?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from("Zombie_Generator#prst::OnEnable")])?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from("Peter S. Alberto::OnDisable")])?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from("Peter S. Alberto#2::OnEnable")])?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.var("priest_q").set(Val::from(6))?;
                            ctx.lines_as(
                                "Father Peter",
                                args!["Hm? What is it you need?", "Well, no problem. You can", "afford to take your time."],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        3 => {
                            ctx.var("priest_q").set(Val::from(6))?;
                            ctx.lines_as("Father Peter", args!["What...?", "You wanna go back??"])?;
                            ctx.next()?;
                            ctx.lines_as("Father Peter", args!["I understand. I suppose you have some important reason or business that you must attend to. Come back whenever you can."])?;
                            ctx.close_window()?;
                            ctx.call(Function::Warp, vec![Val::from("prontera"), Val::from(234), Val::from(318)])?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else if ctx.var("priest_q").get()? == 6 {
                    ctx.lines(args![
                        "Are you ready this time?",
                        "Complete this trial quickly,",
                        "and become a Priest!"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as("Father Peter", args!["Are you ready then?"])?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("I'm ready.:Please hold on.:I want to go back.")])? {
                        1 => {
                            ctx.lines_as("Father Peter", args!["Now, let the spiritual training begin. For the glory of God, for peace on earth, and goodwill towards all men..."])?;
                            ctx.next()?;
                            ctx.lines_as("Father Peter", args!["Go...", "Kill those", "misbegotten creatures!"])?;
                            ctx.close_window()?;
                            if ctx.call(Function::CheckQuest, vec![Val::from(8012)])? == -1 {
                                ctx.call(Function::ChangeQuest, vec![Val::from(8011), Val::from(8012)])?;
                            }
                            ctx.call(Function::Warp, vec![Val::from("job_prist"), Val::from(24), Val::from(44)])?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from("Zombie_Generator#prst::OnEnable")])?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from("Peter S. Alberto::OnDisable")])?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from("Peter S. Alberto#2::OnEnable")])?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as(
                                "Father Peter",
                                args!["Hm? What is it you need?", "Well, no problem. You can", "afford to take your time."],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        3 => {
                            ctx.lines_as("Father Peter", args!["What...?", "You wanna go back??"])?;
                            ctx.next()?;
                            ctx.lines_as("Father Peter", args!["I understand. I suppose you have some important reason or business that you must attend to. Come back whenever you can."])?;
                            ctx.close_window()?;
                            ctx.call(Function::Warp, vec![Val::from("prontera"), Val::from(234), Val::from(318)])?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else {
                    ctx.mes("Go back!")?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("prontera"), Val::from(234), Val::from(318)])?;
                    return Err(Stop::End);
                }
                step = PeterSAlbertoStep::OnEnable;
                continue 'machine;
            }
            PeterSAlbertoStep::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Peter S. Alberto")])?;
                return Err(Stop::End);
            }
            PeterSAlbertoStep::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("Peter S. Alberto")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn peter_s_alberto(ctx: &Ctx) -> Script {
    peter_s_alberto_run(ctx, PeterSAlbertoStep::Start, Vec::new()).map(|_| ())
}

pub fn peter_s_alberto_onenable(ctx: &Ctx) -> Script {
    peter_s_alberto_run(ctx, PeterSAlbertoStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn peter_s_alberto_ondisable(ctx: &Ctx) -> Script {
    peter_s_alberto_run(ctx, PeterSAlbertoStep::OnDisable, Vec::new()).map(|_| ())
}

fn peter_s_alberto_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.mes("[Father Peter]")?;
    if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_PRIEST")?) {
        ctx.mes("Welcome!")?;
        if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
            ctx.lines(args![
                ((Val::from("Brother ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("!"))
            ])?;
        } else {
            ctx.lines(args![
                ((Val::from("Sister ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("!"))
            ])?;
        }
        ctx.mes("So good to see you!")?;
        ctx.next()?;
        ctx.lines_as(
            "Father Peter",
            args!["Are you here to help an Acolyte friend for the spiritual training? That's great~ I think you'll do a good job."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Father Peter",
            args!["Well, another Acolyte is in the training ground right now. You'll need to wait a little bit longer..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Father Peter",
            args!["Please come back a little later. If this acolyte's done with the training, I will send you to the training area."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("priest_q").get()? == 5 {
        ctx.mes("Please hold on for a while. Another acolyte is in the training ground right now.")?;
        ctx.next()?;
        ctx.lines_as(
            "Father Peter",
            args!["If you want to take the test, please wait a while and talk to me again."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("priest_q").get()? == 6 {
        ctx.mes("Please hold on for a while. Another acolyte is in the training ground right now.")?;
        ctx.next()?;
        ctx.lines_as(
            "Father Peter",
            args!["If you want to take the test, please wait a while and talk to me again."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines(args!["Peace...", "Be with you."])?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("prontera"), Val::from(234), Val::from(318)])?;
        return Err(Stop::End);
    }
}

pub fn peter_s_alberto_2(ctx: &Ctx) -> Script {
    peter_s_alberto_2_body(ctx, Vec::new()).map(|_| ())
}

fn peter_s_alberto_2_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Peter S. Alberto#2")])?;
    return Err(Stop::End);
}

pub fn peter_s_alberto_2_oninit(ctx: &Ctx) -> Script {
    peter_s_alberto_2_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn peter_s_alberto_2_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("Peter S. Alberto#2")])?;
    return Err(Stop::End);
}

pub fn peter_s_alberto_2_onenable(ctx: &Ctx) -> Script {
    peter_s_alberto_2_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn peter_s_alberto_2_ondisable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Peter S. Alberto#2")])?;
    return Err(Stop::End);
}

pub fn peter_s_alberto_2_ondisable(ctx: &Ctx) -> Script {
    peter_s_alberto_2_ondisable_body(ctx, Vec::new()).map(|_| ())
}

pub fn zombie_generator_prst(ctx: &Ctx) -> Script {
    zombie_generator_prst_run(ctx, ZombieGeneratorPrstStep::Start, Vec::new()).map(|_| ())
}

pub fn zombie_generator_prst_oninit(ctx: &Ctx) -> Script {
    zombie_generator_prst_run(ctx, ZombieGeneratorPrstStep::OnInit, Vec::new()).map(|_| ())
}

pub fn zombie_generator_prst_onenable(ctx: &Ctx) -> Script {
    zombie_generator_prst_run(ctx, ZombieGeneratorPrstStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn zombie_generator_prst_onm1(ctx: &Ctx) -> Script {
    zombie_generator_prst_run(ctx, ZombieGeneratorPrstStep::Onm1, Vec::new()).map(|_| ())
}

pub fn zombie_generator_prst_onm2(ctx: &Ctx) -> Script {
    zombie_generator_prst_run(ctx, ZombieGeneratorPrstStep::Onm2, Vec::new()).map(|_| ())
}

pub fn zombie_generator_prst_onm3(ctx: &Ctx) -> Script {
    zombie_generator_prst_run(ctx, ZombieGeneratorPrstStep::Onm3, Vec::new()).map(|_| ())
}

pub fn zombie_generator_prst_onm4(ctx: &Ctx) -> Script {
    zombie_generator_prst_run(ctx, ZombieGeneratorPrstStep::Onm4, Vec::new()).map(|_| ())
}

pub fn zombie_generator_prst_onm5(ctx: &Ctx) -> Script {
    zombie_generator_prst_run(ctx, ZombieGeneratorPrstStep::Onm5, Vec::new()).map(|_| ())
}

pub fn zombie_generator_prst_ondisable(ctx: &Ctx) -> Script {
    zombie_generator_prst_run(ctx, ZombieGeneratorPrstStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn zombie_generator_prst_ontimer300000(ctx: &Ctx) -> Script {
    zombie_generator_prst_run(ctx, ZombieGeneratorPrstStep::OnTimer300000, Vec::new()).map(|_| ())
}

pub fn zombie_generator_prst_ontimer300500(ctx: &Ctx) -> Script {
    zombie_generator_prst_run(ctx, ZombieGeneratorPrstStep::OnTimer300500, Vec::new()).map(|_| ())
}

pub fn zombie_generator_prst_ontimer301500(ctx: &Ctx) -> Script {
    zombie_generator_prst_run(ctx, ZombieGeneratorPrstStep::OnTimer301500, Vec::new()).map(|_| ())
}

pub fn zombie_generator_prst_ontimer302000(ctx: &Ctx) -> Script {
    zombie_generator_prst_run(ctx, ZombieGeneratorPrstStep::OnTimer302000, Vec::new()).map(|_| ())
}

fn z_c_prst_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn z_c_prst(ctx: &Ctx) -> Script {
    z_c_prst_body(ctx, Vec::new()).map(|_| ())
}

fn z_c_prst_onmymobdead_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::SetVariableOfNpc,
        vec![
            Val::from(".mymobs"),
            Val::from("Zombie_Generator#prst"),
            Val::from(0),
            (ctx.call(
                Function::GetVariableOfNpc,
                vec![Val::from(".mymobs"), Val::from("Zombie_Generator#prst"), Val::from(0)],
            )?
            .try_sub(Val::from(1))?),
        ],
    )?;
    return Err(Stop::End);
}

pub fn z_c_prst_onmymobdead(ctx: &Ctx) -> Script {
    z_c_prst_onmymobdead_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ZombieInfoStep {
    Start,
    OnTouch,
}

fn zombie_info_run(ctx: &Ctx, mut step: ZombieInfoStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ZombieInfoStep::Start => {
                step = ZombieInfoStep::OnTouch;
                continue 'machine;
            }
            ZombieInfoStep::OnTouch => {
                if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_PRIEST")?) {
                    ctx.lines_as("Father Peter", args!["When the Priest applicant enters, 5 minutes will be given to complete this trial. Proceed slowly and help your Acolyte."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Father Peter",
                        args!["Enter through the warp at the end of the hall, where you will be lead to the next test hall."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Father Peter",
                        args!["Remember...", "This trial must be", "completed within", "5 minutes. Best of luck~"],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?) {
                    ctx.lines_as(
                        "Father Peter",
                        args!["I will give you exactly 5 minutes! You must proceed slowly and eliminate the Zombies."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Father Peter",
                        args!["Slay all the zombies and go through the warp at the end of the hall. Make sure that you kill them all."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn zombie_info(ctx: &Ctx) -> Script {
    zombie_info_run(ctx, ZombieInfoStep::Start, Vec::new()).map(|_| ())
}

pub fn zombie_info_ontouch(ctx: &Ctx) -> Script {
    zombie_info_run(ctx, ZombieInfoStep::OnTouch, Vec::new()).map(|_| ())
}

fn zombie1_1_run(ctx: &Ctx, mut step: Zombie11Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Zombie11Step::Start => {
                step = Zombie11Step::OnInit;
                continue 'machine;
            }
            Zombie11Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Zombie1_1")])?;
                return Err(Stop::End);
            }
            Zombie11Step::OnTouch => {
                if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?) {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Zombie_Generator#prst::Onm1")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Zombie1_1::OnDisable")])?;
                }
                return Err(Stop::End);
            }
            Zombie11Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Zombie1_1")])?;
                return Err(Stop::End);
            }
            Zombie11Step::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("Zombie1_1")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn zombie1_1(ctx: &Ctx) -> Script {
    zombie1_1_run(ctx, Zombie11Step::Start, Vec::new()).map(|_| ())
}

pub fn zombie1_1_oninit(ctx: &Ctx) -> Script {
    zombie1_1_run(ctx, Zombie11Step::OnInit, Vec::new()).map(|_| ())
}

pub fn zombie1_1_ontouch(ctx: &Ctx) -> Script {
    zombie1_1_run(ctx, Zombie11Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn zombie1_1_onenable(ctx: &Ctx) -> Script {
    zombie1_1_run(ctx, Zombie11Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn zombie1_1_ondisable(ctx: &Ctx) -> Script {
    zombie1_1_run(ctx, Zombie11Step::OnDisable, Vec::new()).map(|_| ())
}

fn zombie2_1_run(ctx: &Ctx, mut step: Zombie21Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Zombie21Step::Start => {
                step = Zombie21Step::OnInit;
                continue 'machine;
            }
            Zombie21Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Zombie2_1")])?;
                return Err(Stop::End);
            }
            Zombie21Step::OnTouch => {
                if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?) {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Zombie_Generator#prst::Onm2")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Zombie2_1::OnDisable")])?;
                }
                return Err(Stop::End);
            }
            Zombie21Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Zombie2_1")])?;
                return Err(Stop::End);
            }
            Zombie21Step::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("Zombie2_1")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn zombie2_1(ctx: &Ctx) -> Script {
    zombie2_1_run(ctx, Zombie21Step::Start, Vec::new()).map(|_| ())
}

pub fn zombie2_1_oninit(ctx: &Ctx) -> Script {
    zombie2_1_run(ctx, Zombie21Step::OnInit, Vec::new()).map(|_| ())
}

pub fn zombie2_1_ontouch(ctx: &Ctx) -> Script {
    zombie2_1_run(ctx, Zombie21Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn zombie2_1_onenable(ctx: &Ctx) -> Script {
    zombie2_1_run(ctx, Zombie21Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn zombie2_1_ondisable(ctx: &Ctx) -> Script {
    zombie2_1_run(ctx, Zombie21Step::OnDisable, Vec::new()).map(|_| ())
}

fn zombie3_1_run(ctx: &Ctx, mut step: Zombie31Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Zombie31Step::Start => {
                step = Zombie31Step::OnInit;
                continue 'machine;
            }
            Zombie31Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Zombie3_1")])?;
                return Err(Stop::End);
            }
            Zombie31Step::OnTouch => {
                if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?) {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Zombie_Generator#prst::Onm3")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Zombie3_1::OnDisable")])?;
                }
                return Err(Stop::End);
            }
            Zombie31Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Zombie3_1")])?;
                return Err(Stop::End);
            }
            Zombie31Step::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("Zombie3_1")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn zombie3_1(ctx: &Ctx) -> Script {
    zombie3_1_run(ctx, Zombie31Step::Start, Vec::new()).map(|_| ())
}

pub fn zombie3_1_oninit(ctx: &Ctx) -> Script {
    zombie3_1_run(ctx, Zombie31Step::OnInit, Vec::new()).map(|_| ())
}

pub fn zombie3_1_ontouch(ctx: &Ctx) -> Script {
    zombie3_1_run(ctx, Zombie31Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn zombie3_1_onenable(ctx: &Ctx) -> Script {
    zombie3_1_run(ctx, Zombie31Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn zombie3_1_ondisable(ctx: &Ctx) -> Script {
    zombie3_1_run(ctx, Zombie31Step::OnDisable, Vec::new()).map(|_| ())
}

fn zombie4_1_run(ctx: &Ctx, mut step: Zombie41Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Zombie41Step::Start => {
                step = Zombie41Step::OnInit;
                continue 'machine;
            }
            Zombie41Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Zombie4_1")])?;
                return Err(Stop::End);
            }
            Zombie41Step::OnTouch => {
                if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?) {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Zombie_Generator#prst::Onm4")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Zombie4_1::OnDisable")])?;
                }
                return Err(Stop::End);
            }
            Zombie41Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Zombie4_1")])?;
                return Err(Stop::End);
            }
            Zombie41Step::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("Zombie4_1")])?;
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn zombie4_1(ctx: &Ctx) -> Script {
    zombie4_1_run(ctx, Zombie41Step::Start, Vec::new()).map(|_| ())
}

pub fn zombie4_1_oninit(ctx: &Ctx) -> Script {
    zombie4_1_run(ctx, Zombie41Step::OnInit, Vec::new()).map(|_| ())
}

pub fn zombie4_1_ontouch(ctx: &Ctx) -> Script {
    zombie4_1_run(ctx, Zombie41Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn zombie4_1_onenable(ctx: &Ctx) -> Script {
    zombie4_1_run(ctx, Zombie41Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn zombie4_1_ondisable(ctx: &Ctx) -> Script {
    zombie4_1_run(ctx, Zombie41Step::OnDisable, Vec::new()).map(|_| ())
}

fn zombie5_1_run(ctx: &Ctx, mut step: Zombie51Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Zombie51Step::Start => {
                step = Zombie51Step::OnInit;
                continue 'machine;
            }
            Zombie51Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Zombie5_1")])?;
                return Err(Stop::End);
            }
            Zombie51Step::OnTouch => {
                if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?) {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Zombie_Generator#prst::Onm5")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Zombie5_1::OnDisable")])?;
                }
                return Err(Stop::End);
            }
            Zombie51Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Zombie5_1")])?;
                return Err(Stop::End);
            }
            Zombie51Step::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("Zombie5_1")])?;
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn zombie5_1(ctx: &Ctx) -> Script {
    zombie5_1_run(ctx, Zombie51Step::Start, Vec::new()).map(|_| ())
}

pub fn zombie5_1_oninit(ctx: &Ctx) -> Script {
    zombie5_1_run(ctx, Zombie51Step::OnInit, Vec::new()).map(|_| ())
}

pub fn zombie5_1_ontouch(ctx: &Ctx) -> Script {
    zombie5_1_run(ctx, Zombie51Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn zombie5_1_onenable(ctx: &Ctx) -> Script {
    zombie5_1_run(ctx, Zombie51Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn zombie5_1_ondisable(ctx: &Ctx) -> Script {
    zombie5_1_run(ctx, Zombie51Step::OnDisable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Prst11Step {
    Start,
    OnTouch,
}

fn prst1_1_run(ctx: &Ctx, mut step: Prst11Step, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_mobs = Val::from(0);
    'machine: loop {
        match step {
            Prst11Step::Start => {
                step = Prst11Step::OnTouch;
                continue 'machine;
            }
            Prst11Step::OnTouch => {
                l_mobs = ctx.call(
                    Function::GetVariableOfNpc,
                    vec![Val::from(".mymobs"), Val::from("Zombie_Generator#prst"), Val::from(0)],
                )?;
                if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_PRIEST")?) {
                    ctx.call(Function::Warp, vec![Val::from("job_prist"), Val::from(168), Val::from(17)])?;
                } else if (ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?) && l_mobs.clone().number()? < 1) {
                    ctx.call(Function::Warp, vec![Val::from("job_prist"), Val::from(168), Val::from(17)])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Zombie_Generator#prst::OnDisable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Peter S. Alberto#2::OnDisable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Peter S. Alberto::OnEnable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Zombie_Generator#prst::OnDisable")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn prst1_1(ctx: &Ctx) -> Script {
    prst1_1_run(ctx, Prst11Step::Start, Vec::new()).map(|_| ())
}

pub fn prst1_1_ontouch(ctx: &Ctx) -> Script {
    prst1_1_run(ctx, Prst11Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum DeviruchiPrstStep {
    Start,
    OnTouch,
}

fn deviruchi_prst_run(ctx: &Ctx, mut step: DeviruchiPrstStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            DeviruchiPrstStep::Start => {
                step = DeviruchiPrstStep::OnTouch;
                continue 'machine;
            }
            DeviruchiPrstStep::OnTouch => {
                if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_PRIEST")?) {
                    ctx.lines_as("Deviruchi", args!["Whaaaaat...?", "What are you", "doing back here?"])?;
                    ctx.next()?;
                    ctx.lines_as("Deviruchi", args!["Well, look who's the ^660000BMOC^000000 now. That's '^660000B^000000ig ^660000M^000000an ^660000O^000000n ^660000C^000000ampus,' if you didn't know. By the way, I was being sarcastic. You know, if you didn't notice."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Deviruchi",
                        args!["Are you really", "happy being a Priest?", "There's no possible way."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Deviruchi", args!["Alright, alright, for old time's sake, I'll let you pass me. But only this once. But I better not catch you again! This is evil turf, you hear?!"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?) {
                    ctx.lines_as("Deviruchi", args!["Why...", "Hello little Aco."])?;
                    ctx.next()?;
                    ctx.lines_as("Deviruchi", args!["You must be here training hard to be a Priest. Funny, I know a lot of God's servants, actually. They tell me that it's really tough serving that God guy all the time. So... So ^666666tiring^000000 and ^666666unrewarding^000000."])?;
                    ctx.next()?;
                    ctx.lines_as("Deviruchi", args!["I mean, people are always crying to Priests for help no matter where they are. And Priests never get anything in return..."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Deviruchi",
                        args![
                            "It's tragic really, how unappreciated Priests are.",
                            "It's so clear that any job is better. Anything at all..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Deviruchi", args!["Wouldn't life be so much easier if you weren't a Priest? And it'd be so easy. All you'd have to do is quit right now..."])?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("You're right, I quit!:Out of my sight, demon!")],
                    )?) == 1
                    {
                        ctx.lines_as("Deviruchi", args!["^660000YES~!^000000 I mean...", "Good for you!"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Deviruchi",
                            args!["Oh...?", "Look at the ^660000time^000000.", "You better get going."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Deviruchi", args!["BWAHAHAHAHAHAH!", "GET THE JOKE!?"])?;
                        ctx.close_window()?;
                        ctx.call(Function::Warp, vec![Val::from("c_tower2"), Val::from(168), Val::from(33)])?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        "Deviruchi",
                        args![
                            "Out of your sight?",
                            "I guess I'm not the",
                            "cutest thing you've",
                            "ever seen, huh?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Deviruchi", args!["But how about this...? Now, isn't this an attractive sight? A nice, shiny new card. Mint condition. Not too many people have this you know. But I happen to have soooo many, my pockets hurt."])?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("�̽�Ʈ����ī��.bmp"), Val::from(4)])?;
                    ctx.lines_as(
                        "Deviruchi",
                        args![
                            "Isn't it everyone's dream to have one of these? Think about it, being a Priest can only bring you suffering..."
                        ],
                    )?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("You're right, I'll take it!:Silence!")],
                    )?) == 1
                    {
                        ctx.lines_as("Deviruchi", args!["Good choice!", "This card can", "can be yours..."])?;
                        ctx.next()?;
                        ctx.call(Function::Cutin, vec![Val::from("�̽�Ʈ����ī��.bmp"), Val::from(255)])?;
                        ctx.lines_as(
                            "Deviruchi",
                            args!["Theoretically!", "BWAHAHAHAHAHAHAHA!", "Go and earn it yourself!"],
                        )?;
                        ctx.close_window()?;
                        ctx.call(Function::Warp, vec![Val::from("mjolnir_05"), Val::from(200), Val::from(200)])?;
                        return Err(Stop::End);
                    }
                    ctx.call(Function::Cutin, vec![Val::from("�̽�Ʈ����ī��.bmp"), Val::from(255)])?;
                    ctx.lines_as(
                        "Deviruchi",
                        args!["Did...", "Did you just tell", "me to shut up?", "Oh my God..."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Deviruchi",
                        args![
                            "Sorry...",
                            "Oh ^660000your^000000 God.",
                            "Fine, get going.",
                            "But you'll regret",
                            "your decision later!"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn deviruchi_prst(ctx: &Ctx) -> Script {
    deviruchi_prst_run(ctx, DeviruchiPrstStep::Start, Vec::new()).map(|_| ())
}

pub fn deviruchi_prst_ontouch(ctx: &Ctx) -> Script {
    deviruchi_prst_run(ctx, DeviruchiPrstStep::OnTouch, Vec::new()).map(|_| ())
}

use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn senior_crusader_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_cru_m1 = Val::from(0);
    let mut l_item1 = Val::from(0);
    let mut l_item2 = Val::from(0);
    let mut l_item3 = Val::from(0);
    let mut l_item4 = Val::from(0);
    let mut l_joblevel = Val::from(0);
    ctx.mes("[Michael Halig]")?;
    if ctx.var("Upper").get()? == 1 {
        ctx.mes("Go and train yourself in preparation for the holy war that is coming. Victory will be in the hands of those who are most ready to receive it.")?;
        ctx.next()?;
        ctx.lines_as(
            "Michael Halig",
            args!["You don't belong here, my friend.", "Be advised to continue practicing yourself."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if !ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_SWORDMAN")?) {
            if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_CRUSADER")?) {
                ctx.mes("Go and train yourself in preparation for the holy war that is coming. Victory will be in the hands of those who are most ready to receive it.")?;
                ctx.next()?;
                ctx.lines_as("Michael Halig", args!["Chaos will one day arise to challenge our principles of peace, justice and order. Until we have eliminated evil and created our ideal world, we must not cease training."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_NOVICE")?) {
                ctx.lines(args!["We are Crusaders,", "warriors preparing", "to fight in the Holy War."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Michael Halig",
                    args!["If you wish to join us, you must first learn the Swordsman discipline and train yourself thoroughly..."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines(args!["We are Crusaders,", "warriors preparing", "to fight in the Holy War."])?;
            ctx.next()?;
            ctx.lines_as("Michael Halig", args!["As it happened one thousand years ago, evil forces will one day attack in droves in an attempt to take over the world once again."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ((ctx.var("crus_q").get()?.number()? <= 3 && ctx.call(Function::CountItem, vec![Val::from(1004)])?.is_true())
            && ctx.call(Function::CountItem, vec![Val::from(1009)])?.is_true())
        {
            ctx.lines(args![
                "Ah...",
                "I see that you have been called to become a Crusader. We are assured of your will, but now we must test your capabilities."
            ])?;
            ctx.next()?;
            ctx.lines_as("Michael Halig", args!["Meet with Moorenak Miyol who is training in the underground dungeon of the Prontera Castle. Go, and speak with him first."])?;
            ctx.next()?;
            ctx.call(Function::DelItem, vec![Val::from(1004), Val::from(1)])?;
            ctx.call(Function::DelItem, vec![Val::from(1009), Val::from(1)])?;
            ctx.var("crus_q").set(Val::from(4))?;
            ctx.lines_as("Michael Halig", args!["Moorenak and others like him will test the limits of your capabilities and help you find your path. Return to me after you have completed their tests..."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("crus_q").get()? == 0 {
            ctx.lines(args![
                "We are Crusaders, warriors preparing for the Holy War.",
                "What brings you",
                "to this place?"
            ])?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("I want to prepare for the Holy War!:Nothing in particular.")],
            )?) == 1
            {
                ctx.lines_as(
                    "Michael Halig",
                    args!["You wish to become", "a Crusader...?", "Joining us is not as", "easy as it sounds."],
                )?;
                ctx.next()?;
                ctx.lines_as("Michael Halig", args!["I am Michael Halig. I am one of but many Crusaders preparing for the Holy War. We continuously train ourselves with sincere faith and endless loyalty."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Michael Halig",
                    args![
                        "We recruit Swordsmen that express exceptional faith, or those who were born as warriors",
                        "by Odin's will."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Michael Halig", args!["Possessing the ^3355FFChivalry Emblem^000000 and ^3355FFHand of God^000000 is seen as a sign that you have been called to become a Crusader."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Michael Halig",
                    args!["After obtaining those items and passing our tests, you too, can become a Crusader."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Michael Halig",
                    args![
                        "I don't know what type of person you are right now though. But",
                        "I shall test you if you desire. Are you willing to endure these tests in preparation for the Holy War?"
                    ],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("Yes, I do.:I'd like to think about it.")],
                )?) == 1
                {
                    if ctx.var("JobLevel").get()?.number()? < 40 {
                        ctx.lines_as(
                            "Michael Halig",
                            args![
                                "Wait...",
                                "You're not ready yet.",
                                "You need to be at least",
                                "Job Level 40 to become",
                                "a Crusader."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Michael Halig", args!["Train yourself more as a Swordsman and wait for your calling. I understand your intent, but as of now, you cannot join us."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    if ctx.var("SkillPoint").get()?.is_true() {
                        ctx.lines_as("Michael Halig", args!["You haven't finished learning everything as a Swordsman. Use all of your remaining skill points, and then return to me."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        "Michael Halig",
                        args!["Then...", "I shall test you to see if you are fit to become a Crusader."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Michael Halig",
                        args![
                            "Your name is",
                            ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("...?")),
                            "Let's see..."
                        ],
                    )?;
                    ctx.next()?;
                    if (ctx.call(Function::CountItem, vec![Val::from(1004)])?.is_true()
                        && ctx.call(Function::CountItem, vec![Val::from(1009)])?.is_true())
                    {
                        ctx.lines_as("Michael Halig", args!["Ah...", "I see that you have been called to become a Crusader. We are assured of your will, but now we must test your capabilities."])?;
                        ctx.next()?;
                        ctx.lines_as("Michael Halig", args!["Meet with Moorenak Miyol who is training in the underground dungeon of the Prontera Castle. Go, and speak with him first."])?;
                        ctx.next()?;
                        ctx.call(Function::DelItem, vec![Val::from(1004), Val::from(1)])?;
                        ctx.call(Function::DelItem, vec![Val::from(1009), Val::from(1)])?;
                        ctx.var("crus_q").set(Val::from(4))?;
                        ctx.call(Function::SetQuest, vec![Val::from(3009)])?;
                        ctx.lines_as("Michael Halig", args!["Moorenak and others like him will test the limits of your capabilities and help you find your path. Return to me after you have completed their tests..."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as("Michael Halig", args!["Mmm. I can't discern whether or not you have the heart to become a Crusader. However, if you have the will and put forth the effort, you may have what it takes."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Michael Halig",
                        args![
                            "Now...",
                            "This is my test for you. Bring me the following items and prove your determination to me."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.mes("[Michael Halig]")?;
                    l_cru_m1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                    if l_cru_m1.clone() == 1 {
                        ctx.var("crus_q").set(Val::from(1))?;
                        ctx.call(Function::SetQuest, vec![Val::from(3006)])?;
                        ctx.lines(args![
                            ((Val::from("10 ^3355FF") + ctx.call(Function::GetItemName, vec![Val::from(957)])?) + Val::from("^000000")),
                            ((Val::from("10 ^3355FF") + ctx.call(Function::GetItemName, vec![Val::from(959)])?) + Val::from("^000000")),
                            ((Val::from("10 ^3355FF") + ctx.call(Function::GetItemName, vec![Val::from(1099)])?) + Val::from("^000000")),
                            ((Val::from("10 ^3355FF") + ctx.call(Function::GetItemName, vec![Val::from(901)])?) + Val::from("^000000"))
                        ])?;
                        ctx.next()?;
                    } else if l_cru_m1.clone() == 2 {
                        ctx.var("crus_q").set(Val::from(2))?;
                        ctx.call(Function::SetQuest, vec![Val::from(3007)])?;
                        ctx.lines(args![
                            ((Val::from("10 ^3355FF") + ctx.call(Function::GetItemName, vec![Val::from(932)])?) + Val::from("^000000")),
                            ((Val::from("10 ^3355FF") + ctx.call(Function::GetItemName, vec![Val::from(1043)])?) + Val::from("^000000")),
                            ((Val::from("10 ^3355FF") + ctx.call(Function::GetItemName, vec![Val::from(1098)])?) + Val::from("^000000")),
                            ((Val::from("10 ^3355FF") + ctx.call(Function::GetItemName, vec![Val::from(1094)])?) + Val::from("^000000"))
                        ])?;
                        ctx.next()?;
                    } else {
                        ctx.var("crus_q").set(Val::from(3))?;
                        ctx.call(Function::SetQuest, vec![Val::from(3008)])?;
                        ctx.lines(args![
                            ((Val::from("10 ^3355FF") + ctx.call(Function::GetItemName, vec![Val::from(958)])?) + Val::from("^000000")),
                            ((Val::from("10 ^3355FF") + ctx.call(Function::GetItemName, vec![Val::from(930)])?) + Val::from("^000000")),
                            ((Val::from("10 ^3355FF") + ctx.call(Function::GetItemName, vec![Val::from(1041)])?) + Val::from("^000000")),
                            ((Val::from("10 ^3355FF") + ctx.call(Function::GetItemName, vec![Val::from(1062)])?) + Val::from("^000000"))
                        ])?;
                        ctx.next()?;
                    }
                    ctx.lines_as("Michael Halig", args!["Show me the strength of your will by gathering these items. If you prove successful, you will progress to the next test. May Odin protect you."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Michael Halig",
                    args![
                        "Nobody knows when the Holy War will come. We must prepare in advance and cannot afford to slacken",
                        "our training."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Michael Halig", args!["If you feel that participating in the Holy War is your calling, please come back right away and take the test to become", "a Crusader."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.mes("[Michael Halig]")?;
            if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                ctx.mes(
                    "If you, too, are a man of the sword, constantly train and prepare yourself. No one knows when the Holy War may begin.",
                )?;
            } else {
                ctx.mes("As a woman of the sword, you must train diligently and constantly. Prepare yourself, for no one knows when the Holy War may be coming.")?;
            }
            ctx.close_window()?;
            return Err(Stop::End);
        } else if (ctx.var("crus_q").get()?.number()? >= 1 && ctx.var("crus_q").get()?.number()? <= 3) {
            ctx.mes("Have you proven your determination with the task I have given you, or do you possess the items proving that you have received your calling?")?;
            ctx.next()?;
            let subject1 = ctx.var("crus_q").get()?;
            if subject1 == 1 {
                l_item1 = Val::from(957);
                l_item2 = Val::from(959);
                l_item3 = Val::from(1099);
                l_item4 = Val::from(901);
            } else if subject1 == 2 {
                l_item1 = Val::from(932);
                l_item2 = Val::from(1043);
                l_item3 = Val::from(1098);
                l_item4 = Val::from(1094);
            } else if subject1 == 3 {
                l_item1 = Val::from(958);
                l_item2 = Val::from(930);
                l_item3 = Val::from(1041);
                l_item4 = Val::from(1062);
            }
            if (((ctx.call(Function::CountItem, vec![l_item1.clone()])?.number()? > 9
                && ctx.call(Function::CountItem, vec![l_item2.clone()])?.number()? > 9)
                && ctx.call(Function::CountItem, vec![l_item3.clone()])?.number()? > 9)
                && ctx.call(Function::CountItem, vec![l_item4.clone()])?.number()? > 9)
            {
                ctx.lines_as(
                    "Michael Halig",
                    args![
                        "Ah, well done.",
                        "I must acknowledge your efforts and determination. You may now proceed to take the next test."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Michael Halig", args!["Meet with Moorenak Miyol who is training in the underground dungeon of the Prontera Castle. Go, and speak with him first."])?;
                ctx.next()?;
                ctx.call(Function::DelItem, vec![l_item1.clone(), Val::from(10)])?;
                ctx.call(Function::DelItem, vec![l_item2.clone(), Val::from(10)])?;
                ctx.call(Function::DelItem, vec![l_item3.clone(), Val::from(10)])?;
                ctx.call(Function::DelItem, vec![l_item4.clone(), Val::from(10)])?;
                ctx.var("crus_q").set(Val::from(4))?;
                if ctx.call(Function::CheckQuest, vec![Val::from(3006)])? != -1 {
                    ctx.call(Function::ChangeQuest, vec![Val::from(3006), Val::from(3009)])?;
                } else if ctx.call(Function::CheckQuest, vec![Val::from(3007)])? != -1 {
                    ctx.call(Function::ChangeQuest, vec![Val::from(3007), Val::from(3009)])?;
                } else {
                    ctx.call(Function::ChangeQuest, vec![Val::from(3008), Val::from(3009)])?;
                }
                ctx.lines_as("Michael Halig", args!["Moorenak and others like him will test the limits of your capabilities and help you find your path. Return to me after you have completed their tests..."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as("Michael Halig", args!["Ah, you still have not completed the task I have given to you. Bring me the following items, and prove your will to become a Crusader to me..."])?;
            ctx.next()?;
            ctx.lines_as(
                "Michael Halig",
                args![
                    ((Val::from("10 ^3355FF") + ctx.call(Function::GetItemName, vec![l_item1.clone()])?) + Val::from("^000000")),
                    ((Val::from("10 ^3355FF") + ctx.call(Function::GetItemName, vec![l_item2.clone()])?) + Val::from("^000000")),
                    ((Val::from("10 ^3355FF") + ctx.call(Function::GetItemName, vec![l_item3.clone()])?) + Val::from("^000000")),
                    ((Val::from("10 ^3355FF") + ctx.call(Function::GetItemName, vec![l_item4.clone()])?) + Val::from("^000000"))
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Michael Halig",
                args!["If you put forth the effort, you'll be able to accomplish this task. May Odin protect you on your journeys."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("crus_q").get()? == 10 {
            if ctx.var("SkillPoint").get()?.is_true() {
                ctx.mes("You haven't finished learning everything as a Swordsman. Use all of your remaining skill points, and then return to me.")?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.mes("Congratulations on completing all of your tests. You are truly worthy of fighting alongside side us in the Holy War as a Crusader.")?;
            ctx.next()?;
            ctx.lines_as(
                "Michael Halig",
                args!["Together, let us ready ourselves and be victorious over evil and tyranny!"],
            )?;
            ctx.next()?;
            l_joblevel = ctx.var("JobLevel").get()?;
            ctx.call(Function::CompleteQuest, vec![Val::from(3015)])?;
            shared::other_global_functions::job_change(ctx, vec![ctx.constant("JOB_CRUSADER")?])?;
            shared::other_global_functions::f_clearjobvar(ctx, vec![])?;
            ctx.lines_as(
                "Michael Halig",
                args![
                    "Behold...!",
                    "You are now a Crusader!",
                    "When the Holy War comes, we shall fight side by side against the forces of evil."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Michael Halig", args!["Now you are", "one of us!"])?;
            if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                ctx.mes("...Brother.")?;
            } else {
                ctx.mes("...Comrade.")?;
            }
            ctx.next()?;
            if l_joblevel.clone() != 50 {
                ctx.call(Function::GetItem, vec![Val::from(504), Val::from(6)])?;
            } else {
                ctx.call(Function::GetItem, vec![Val::from(504), Val::from(12)])?;
            }
            ctx.lines_as(
                "Michael Halig",
                args!["And use this in times of dire peril. It will give you strength when your wounds are most grievous."],
            )?;
            ctx.next()?;
            ctx.lines_as("Michael Halig", args!["Never forget that the Holy War is approaching. We must be prepare for the inevitable tide of evil that will sweep this world. Now, go forth and fight for the principles of freedom and justice!"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    ctx.mes("It seems that you have not yet completed all of the testing. You will not be ready to become a Crusader until you have completed the tests set before you.")?;
    ctx.next()?;
    ctx.lines_as("Michael Halig", args!["Return to me when you have completed all of your tests. When you finally prove eligible, you will join the proud ranks of the mighty Crusaders."])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn senior_crusader(ctx: &Ctx) -> Script {
    senior_crusader_body(ctx, Vec::new()).map(|_| ())
}

fn man_in_anguish_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.mes("[Murnak Mijoul]")?;
    if !ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_SWORDMAN")?) {
        if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_CRUSADER")?) {
            ctx.mes("Don't linger around in a place like this and forge your own path towards discovering your own strengths. The day that we will join hands in battle will come soon.")?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_NOVICE")?) {
            ctx.lines(args!["A Novice...?", "So green, and yet,", "so much potential."])?;
            ctx.next()?;
            ctx.lines_as("Murnak Mijoul", args!["Let me assure you that I'm no criminal. I'm here merely to train myself. Perhaps as you become stronger, you will understand."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.mes("Hm. You have no business in a place like this. Please leave, and do not interrupt my training.")?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("crus_q").get()?.number()? >= 0 && ctx.var("crus_q").get()?.number()? <= 3) {
        ctx.lines(args![
            "What do you want...?",
            "If you have no business here,",
            "then please leave. A tranquil state of mind is essential in self training..."
        ])?;
        ctx.next()?;
        ctx.lines_as("Murnak Mijoul", args!["I wish to improve the sense of serenity in my heart in preparation for the Holy War that is to come. So please, do not disturb me."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("crus_q").get()? == 4 {
        ctx.lines(args!["What is it...?", "Do you have business"])?;
        if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
            ctx.mes("with me, man of the sword?")?;
        } else {
            ctx.mes("with me, woman of the sword?")?;
        }
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from("I'd like to take the Crusader test.:Nothing.")],
        )?) == 1
        {
            ctx.lines_as(
                "Murnak Mijoul",
                args!["You wish to become a Crusader...? Hm, fighting in the Holy War is an admirable goal we may both share."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Murnak Mijoul",
                args![
                    "Your name is",
                    ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("...?")),
                    "Let me take",
                    "a look at your face."
                ],
            )?;
            ctx.next()?;
            if (ctx.call(Function::GetEquipId, vec![ctx.constant("EQI_ACC_L")?])? != 2608
                && ctx.call(Function::GetEquipId, vec![ctx.constant("EQI_ACC_R")?])? != 2608)
            {
                ctx.lines_as("Murnak Mijoul", args!["Hmm. You wish to become a Crusader, but do not wear a Rosary? I have no business with you if you cannot uphold our customs."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Murnak Mijoul",
                args![
                    "Hmm...",
                    "You seem so-so, but also young and ambitious. Ambition may work against you if it is not tempered with patience."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Murnak Mijoul", args!["Don't give me a reason to doubt you, and show me your patience. You must endure my test with your patience if you wish to become a Crusader."])?;
            ctx.next()?;
            ctx.lines_as(
                "Murnak Mijoul",
                args!["You may take my test right away. It's actually quite simple. All you must do is walk to the end of the corridor."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Murnak Mijoul",
                args![
                    "But...",
                    "You must keep one thing in mind. Under no condition are you allowed to kill the monsters."
                ],
            )?;
            ctx.next()?;
            ctx.var("crus_q").set(Val::from(5))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(3009), Val::from(3010)])?;
            ctx.lines_as("Murnak Mijoul", args!["Well then...", "Good luck."])?;
            ctx.close_window()?;
            ctx.call(Function::Warp, vec![Val::from("job_cru"), Val::from(98), Val::from(40)])?;
            return Err(Stop::End);
        }
        ctx.lines_as("Murnak Mijoul", args!["You seem to have a lot of time on your hands. Why don't you do something more productive, like pick Jellopy off the streets or something?"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("crus_q").get()? == 5 {
        ctx.lines(args![
            "What is it...?",
            "You're the Swordman from before. What happened, did you fail?"
        ])?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from("Let me retake the test.:What kind of test was that?!")],
        )?) == 1
        {
            ctx.lines_as(
                "Murnak Mijoul",
                args![
                    "I'll let you retake the test as much as you like. But if you continuously fail, there's an inherent problem with your",
                    "state of mind."
                ],
            )?;
            ctx.next()?;
            if (ctx.call(Function::GetEquipId, vec![ctx.constant("EQI_ACC_L")?])? != 2608
                && ctx.call(Function::GetEquipId, vec![ctx.constant("EQI_ACC_R")?])? != 2608)
            {
                ctx.lines_as("Murnak Mijoul", args!["Wait...", "Where have you left your Rosary? You can't let that lie around just anywhere. We are supposed to be warriors of holiness."])?;
                ctx.next()?;
                ctx.lines_as("Murnak Mijoul", args!["Carelessly losing things is an attitude for mere Swordsmen. If you really want to retake the test, you must respect the Crusader traditions."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Murnak Mijoul",
                args![
                    "Your problem is your habit of hitting monsters the moment you see one. You must develop your patience and endurance."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Murnak Mijoul",
                args![
                    "Just focus...",
                    "Your only goal is to walk from one end of the corridor to the other. It's simple when you think",
                    "about it."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Murnak Mijoul",
                args!["No matter what you do, do not kill any of the monsters. This time, think carefully before you draw your sword."],
            )?;
            ctx.next()?;
            ctx.var("crus_q").set(Val::from(5))?;
            ctx.lines_as("Murnak Mijoul", args!["Well then...", "Good luck."])?;
            ctx.close_window()?;
            ctx.call(Function::Warp, vec![Val::from("job_cru"), Val::from(98), Val::from(40)])?;
        }
        ctx.lines_as(
            "Murnak Mijoul",
            args!["Who are you to say that you don't like my test? With that kind of attitude, you'll never become a Crusader."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Murnak Mijoul",
            args!["I can understand if you are easily frustrated, but you must overcome your frustration to pass this test."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("crus_q").get()? == 6 {
        ctx.lines(args![
            "Hmm, seems like you did well. It shouldn't have been too hard. You no longer have any business",
            "with me."
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Murnak Mijoul",
            args!["For your next test, go look for Gabriel Valentine in the Prontera Sanctuary. Well then, I'll see you around."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines(args![
        "Hmmm...",
        "You still have tests to complete if you want to become a Crusader, don't you?"
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn man_in_anguish(ctx: &Ctx) -> Script {
    man_in_anguish_body(ctx, Vec::new()).map(|_| ())
}

fn crusader_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_cru_m = Val::from(0);
    let mut l_cru_t = Val::from(0);
    ctx.mes("[Gabriel Valentine]")?;
    if !ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_SWORDMAN")?) {
        if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_CRUSADER")?) {
            ctx.lines(args!["Welcome, fellow Crusader.", "How is your training", "coming along?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Gabriel Valentine",
                args!["You must not forget to train everyday, and prepare for the day the Holy War will come upon us."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_NOVICE")?) {
            ctx.lines(args![
                "Welcome, I am a Crusader.",
                "I am preparing for the",
                "foretold Holy War",
                "that is to come."
            ])?;
            ctx.next()?;
            ctx.lines_as("Gabriel Valentine", args!["If you are interested in becoming a Crusader, you must train first as a Swordman. Come and visit us again when you believe that you have learned enough as a Swordman..."])?;
            ctx.next()?;
            ctx.lines_as(
                "Gabriel Valentine",
                args!["We are located in the Prontera Central Palace, so if you have time, it wouldn't hurt to stop by."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines(args![
            "Welcome, we are Crusaders.",
            "We are preparing for the",
            "foretold Holy War",
            "that is to come."
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Gabriel Valentine",
            args!["I hope you will train yourself in preparation for the future as well."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("crus_q").get()? == 0 {
        ctx.lines(args![
            "Welcome. We are Crusaders.",
            "We are preparing for the",
            "foretold Holy War",
            "that is to come."
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Gabriel Valentine",
            args!["If you would like to become a Crusader, please speak with our leader in the Prontera Central Palace."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("crus_q").get()? == 6 || ctx.var("crus_q").get()? == 7) {
        if ctx.var("crus_q").get()? == 6 {
            ctx.lines(args![
                "Welcome.",
                "Did you do well",
                "on those painful tests?",
                "I will be conducting your next test."
            ])?;
            ctx.next()?;
            ctx.lines_as("Gabriel Valentine", args!["My name is Gabriel Valentine. I, too, am preparing for the Holy War. For the time being, I act as guard for this church."])?;
            ctx.next()?;
            ctx.lines_as(
                "Gabriel Valentine",
                args![
                    "I will test to see if you have acquired the knowledge that is necessary to become a Crusader.",
                    "We can't very well win the Holy War just by swinging a sword."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Gabriel Valentine",
                args!["I will give", "you 10 questions.", "Answer them correctly."],
            )?;
            ctx.next()?;
        } else if ctx.var("crus_q").get()? == 7 {
            ctx.lines(args![
                "Welcome back~",
                "Did you prepare",
                "well for this test?",
                "Let's try again,",
                "shall we...?"
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Gabriel Valentine",
                args![
                    "Once again, I'm going",
                    "to give you 10 questions",
                    "Listen carefully, and",
                    "choose the correct answer."
                ],
            )?;
            ctx.next()?;
        }
        l_cru_m = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
        if l_cru_m.clone() == 1 {
            ctx.lines_as(
                "Gabriel Valentine",
                args!["1. Which attribute is the most effective in atttacking the Undead?"],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("Neutral:Earth:Undead:Holy")])?) == 4 {
                l_cru_t = (l_cru_t.clone() + Val::from(10));
            }
            ctx.lines_as(
                "Gabriel Valentine",
                args!["2. If the monster is a Level 2 Undead, how much more damage does a Holy attack do compared to Fire?"],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("25 %:50 %:75 %:100 %")])?) == 1 {
                l_cru_t = (l_cru_t.clone() + Val::from(10));
            }
            ctx.lines_as("Gabriel Valentine", args!["3. What item can you not get from an Evil Druid?"])?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Monk Hat:Yggdrasil leaf:White Herb:Amulet ")],
            )?) == 1
            {
                l_cru_t = (l_cru_t.clone() + Val::from(10));
            }
            ctx.lines_as("Gabriel Valentine", args!["4. Which Undead monster", "has the highest HP?"])?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Ghoul:Skeleton Prisoner:Wraith:Zombie Prisoner")],
            )?) == 4
            {
                l_cru_t = (l_cru_t.clone() + Val::from(10));
            }
            ctx.lines_as(
                "Gabriel Valentine",
                args!["5. Which of the following monsters is a different size than the others?"],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Wraith:Khalitzburg:Drake:Evil Druid")],
            )?) == 3
            {
                l_cru_t = (l_cru_t.clone() + Val::from(10));
            }
            ctx.lines_as(
                "Gabriel Valentine",
                args!["6. Which card grants you tolerance to Undead property attacks?"],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Orc Skeleton Card:Orc Zombie Card:Ghoul Card:Skel Worker Card")],
            )?) == 2
            {
                l_cru_t = (l_cru_t.clone() + Val::from(10));
            }
            ctx.lines_as(
                "Gabriel Valentine",
                args!["7. What was the relationship between Munak and Bongun before they passed away?"],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from(
                    "Big Brother and Little Sister:Childhood friends in the same village:Stepbrother and sister:Complete strangers",
                )],
            )?) == 2
            {
                l_cru_t = (l_cru_t.clone() + Val::from(10));
            }
            ctx.lines_as(
                "Gabriel Valentine",
                args!["8. Which of the following monsters is not aggressive?"],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Soldier Skeleton:Orc Skeleton:Skeleton:Skel Worker")],
            )?) == 3
            {
                l_cru_t = (l_cru_t.clone() + Val::from(10));
            }
            ctx.lines_as(
                "Gabriel Valentine",
                args!["9. What is the name of the shield in which a Munak Card has been inserted?"],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Atomic Shield:Amulet Shield:Hypnotic Shield:Homeroth Shield")],
            )?) == 2
            {
                l_cru_t = (l_cru_t.clone() + Val::from(10));
            }
            ctx.lines_as(
                "Gabriel Valentine",
                args!["10. Which of the following monsters does not drop Memento?"],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("Munak:Ghoul:Mummy:Soldier Skeleton")])?) == 1 {
                l_cru_t = (l_cru_t.clone() + Val::from(10));
            }
        } else if l_cru_m.clone() == 2 {
            ctx.lines_as(
                "Gabriel Valentine",
                args!["1. Which of the following monsters is a different attribute than the others?"],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("Carat:Wind Ghost:Isis:Wanderer")])?) == 3 {
                l_cru_t = (l_cru_t.clone() + Val::from(10));
            }
            ctx.lines_as(
                "Gabriel Valentine",
                args!["2. Which sword is effective in attacking Demon monsters?"],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Decussate Tsurugi:Hollowed Tsurugi:Damned Tsurugi:Drowsy Tsurugi")],
            )?) == 1
            {
                l_cru_t = (l_cru_t.clone() + Val::from(10));
            }
            ctx.lines_as("Gabriel Valentine", args!["3. Which item is NOT dropped by Dokebi?"])?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Rough Elunium:Golden Hammer:Sword Mace:Mighty Staff")],
            )?) == 2
            {
                l_cru_t = (l_cru_t.clone() + Val::from(10));
            }
            ctx.lines_as("Gabriel Valentine", args!["4. Which Demon monster has the most HP?"])?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("Giearth:Magnolia:Dokebi:Marionette")])?) == 4 {
                l_cru_t = (l_cru_t.clone() + Val::from(10));
            }
            ctx.lines_as(
                "Gabriel Valentine",
                args!["5. Which Demon monster is a different size than the others?"],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Ghostring:Whisper:Deviruchi:Baphomet Junior")],
            )?) == 1
            {
                l_cru_t = (l_cru_t.clone() + Val::from(10));
            }
            ctx.lines_as(
                "Gabriel Valentine",
                args!["6. Which shield reduces damage inflicted by Demon monsters?"],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Satanic Shield:Shield from Hell:Amulet Shield:Excellent Shield")],
            )?) == 2
            {
                l_cru_t = (l_cru_t.clone() + Val::from(10));
            }
            ctx.lines_as(
                "Gabriel Valentine",
                args!["7. Which attribute is the most effective on the Wind Ghost?"],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("Water:Earth:Fire:Wind")])?) == 2 {
                l_cru_t = (l_cru_t.clone() + Val::from(10));
            }
            ctx.lines_as(
                "Gabriel Valentine",
                args!["8. Which monster is different from the other Demon monsters?"],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("Sohee:Isis:Dokebi:Whisper")])?) == 4 {
                l_cru_t = (l_cru_t.clone() + Val::from(10));
            }
            ctx.lines_as("Gabriel Valentine", args!["9. What effect does the Marionette Card have?"])?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from(
                    "Increase defense against Shadow attacks by 30 %:Increase defense against poison attacks by 30 %:Increase defense against Ghost attacks by 30 %:Increase defense against Neutral attacks by 30 %",
                )],
            )?) == 3
            {
                l_cru_t = (l_cru_t.clone() + Val::from(10));
            }
            ctx.lines_as(
                "Gabriel Valentine",
                args!["10. Which of the following is an effective way to react when encountering a demon monster?"],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from(
                    "Scream, 'Evil one, go away!':Offer your soul and get a deal.:Put Holy Water on a weapon and attack.:Put on a Deviruchi hat.",
                )],
            )?) == 3
            {
                l_cru_t = (l_cru_t.clone() + Val::from(10));
            }
        } else {
            ctx.lines_as(
                "Gabriel Valentine",
                args!["1. What level of 'Divine Protection' do you need to learn 'Demon Bane?'"],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("Level 1:Level 2:Level 3:Level 4")])?) == 3 {
                l_cru_t = (l_cru_t.clone() + Val::from(10));
            }
            ctx.lines_as(
                "Gabriel Valentine",
                args!["2. If your INT is 30, including INT bonuses from quipment, at level 55, how much HP does Level 5 Heal recover?"],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("396:440:484:528")])?) == 2 {
                l_cru_t = (l_cru_t.clone() + Val::from(10));
            }
            ctx.lines_as(
                "Gabriel Valentine",
                args!["3. With Level 7 Divine Protection, by how much is your defense against the Undead increased?"],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("21:22:23:24")])?) == 1 {
                l_cru_t = (l_cru_t.clone() + Val::from(10));
            }
            ctx.lines_as(
                "Gabriel Valentine",
                args!["4. Which of the following spears can attack Nightmare, which is endowed with the Ghost attribute?"],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Lance:Bill Guisarme:Cresent scythe:Zephyrus")],
            )?) == 4
            {
                l_cru_t = (l_cru_t.clone() + Val::from(10));
            }
            ctx.lines_as(
                "Gabriel Valentine",
                args!["5. What level of 'Heal' do you need to learn 'Cure?'"],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("Level 1:Level 2:Level 3:Level 4")])?) == 2 {
                l_cru_t = (l_cru_t.clone() + Val::from(10));
            }
            ctx.lines_as(
                "Gabriel Valentine",
                args!["6. What is the attack speed when Level 3 Cavalier Mastery is learned?"],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from(
                    "70 % of normal speed:80 % of normal speed:90 % of normal speed:100 % of normal speed",
                )],
            )?) == 2
            {
                l_cru_t = (l_cru_t.clone() + Val::from(10));
            }
            ctx.lines_as(
                "Gabriel Valentine",
                args!["7. Which of the following is not correct of the Demon Bane skill?"],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from(
                    "Increase attack on Undead:Only Acolytes can learn the skill:When mastered, + 30 increase:Passive Skill",
                )],
            )?) == 2
            {
                l_cru_t = (l_cru_t.clone() + Val::from(10));
            }
            ctx.lines_as("Gabriel Valentine", args!["8. How much SP does Level 7 Heal use?"])?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("30:31:33:35")])?) == 2 {
                l_cru_t = (l_cru_t.clone() + Val::from(10));
            }
            ctx.lines_as(
                "Gabriel Valentine",
                args!["9. What status cannot be", "cured with the Cure skill?"],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("Curse:Silence:Chaos:Blind")])?) == 1 {
                l_cru_t = (l_cru_t.clone() + Val::from(10));
            }
            ctx.lines_as("Gabriel Valentine", args!["10. What best describes a Crusader?"])?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from(
                    "One preparing for matrimony.:One preparing for the Holy War.:One preparing consummation.:One preparing potions.",
                )],
            )?) == 2
            {
                l_cru_t = (l_cru_t.clone() + Val::from(10));
            }
        }
        ctx.lines_as(
            "Gabriel Valentine",
            args!["Good work~", "Well, first let me", "look at your results."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Gabriel Valentine",
            args![
                ((Val::from(" ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("'s score")),
                ((Val::from("is ") + l_cru_t.clone()) + Val::from(" points..."))
            ],
        )?;
        if l_cru_t.clone() == 100 {
            ctx.var("crus_q").set(Val::from(8))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(3011), Val::from(3013)])?;
            ctx.lines(args!["Superb! Now, it's time for", "you to take the next test."])?;
            ctx.next()?;
            ctx.lines_as(
                "Gabriel Valentine",
                args![
                    "Go to Prontera Castle",
                    "and meet Bliant Piyord.",
                    "I will inform him that",
                    "he will be testing you next."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if l_cru_t.clone() == 90 {
            ctx.var("crus_q").set(Val::from(8))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(3011), Val::from(3013)])?;
            ctx.lines(args!["Well done~ Now, it's time for", "you to take the next test."])?;
            ctx.next()?;
            ctx.lines_as(
                "Gabriel Valentine",
                args![
                    "Go to Prontera Castle",
                    "and meet Bliant Piyord.",
                    "I will inform him that",
                    "he will be testing you next."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if (l_cru_t.clone() == 80 && ctx.var("crus_q").get()? == 7) {
            ctx.var("crus_q").set(Val::from(8))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(3011), Val::from(3013)])?;
            ctx.lines(args![
                "Seems like you prepared a lot so I'll let you pass this time.",
                "Hurry now and go take the next test."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Gabriel Valentine",
                args![
                    "Go to the Prontera Castle and meet Bliant Piyord.",
                    "I will inform him to prepare the next test."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.var("crus_q").set(Val::from(7))?;
        if ctx.call(Function::CheckQuest, vec![Val::from(3011)])? != -1 {
            ctx.call(Function::ChangeQuest, vec![Val::from(3011), Val::from(3012)])?;
        }
        ctx.lines(args![
            "Hmmm... What a pity.",
            "Go study some more and",
            "take this test again, okay?"
        ])?;
        ctx.next()?;
        ctx.lines_as("Gabriel Valentine", args!["Don't stress, you need to know a lot in order to pass this test. In any case, I'll be waiting right here. When you think you're ready, come back, alright?"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("crus_q").get()? == 8 || ctx.var("crus_q").get()? == 9) {
        ctx.mes("Like I mentioned before, you should go to Prontera Castle and meet with Bliant Piyord to take your next test. Good luck, and become a Crusder soon, alright?")?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("crus_q").get()? == 10 {
        ctx.mes("What are you still doing here? You've already completed all the tests. Go talk to our leader, you're pretty much ready to become a Crusader now.")?;
        ctx.next()?;
        ctx.lines_as(
            "Gabriel Valentine",
            args!["You will soon join us in our preparations for the Holy War. Continue to live with faith after becoming a Crusader."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines(args![
        "Mmm...?",
        "It seems that you're",
        "an aspiring Crusader...",
        "But, it's not my turn",
        "to test you quite yet."
    ])?;
    ctx.next()?;
    ctx.lines_as(
        "Gabriel Valentine",
        args![
            "Finish those other tests,",
            "and come to me once you're",
            "instructed. Until then,",
            "I'll see you later~"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn crusader(ctx: &Ctx) -> Script {
    crusader_body(ctx, Vec::new()).map(|_| ())
}

fn patron_knight_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Bliant Piyord", args!["Welcome."])?;
    if !ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_SWORDMAN")?) {
        if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_CRUSADER")?) {
            ctx.lines(args!["How goes", "your training?"])?;
            ctx.next()?;
            ctx.lines_as("Bliant Piyord", args!["Develop your faith. From faith springs strength and discipline. Day after day, train yourself and become a great Crusader."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_NOVICE")?) {
            ctx.lines(args![
                "We are Crusaders,",
                "warriors of holiness preparing for the great Holy War that is to come."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Bliant Piyord",
                args![
                    "Are you interested",
                    "in becoming a Crusader?",
                    "We are always waiting",
                    "for more capable men and",
                    "women to join our ranks."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Bliant Piyord", args!["Train as a Swordsman and come to us when you think you are ready. If you have been called by Odin to become a Crusader, that would be even better."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines(args![
            "We are Crusaders,",
            "warriors of holiness preparing for the great Holy War that is to come."
        ])?;
        ctx.next()?;
        ctx.lines_as("Bliant Piyord", args!["Even in these relatively peaceful times, our training is ceaseless. We must be ready for the day with the tides of darkness shall rush against mankind..."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("crus_q").get()? == 0 {
        ctx.lines(args![
            "We are Crusaders,",
            "warriors of holiness preparing for the great Holy War that is to come."
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Bliant Piyord",
            args![
                "Are you interested",
                "in becoming a Crusader?",
                "We are always waiting",
                "for more capable men and",
                "women to join our ranks."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Bliant Piyord",
            args![
                "To become a Crusader, you must train until you are Job Level 40. For more details, please speak with our leader inside."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Bliant Piyord", args!["May Odin", "be with you..."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("crus_q").get()? == 8 || ctx.var("crus_q").get()? == 9) {
        if ctx.var("crus_q").get()? == 8 {
            ctx.lines(args!["It's nice", "to meet you.", "It is now time", "for your final test."])?;
            ctx.next()?;
            ctx.lines_as("Bliant Piyord", args!["My name is Bliant Piyord and I too, am preparing for the Holy War. It must've been quite a hassle to come all this way."])?;
            ctx.next()?;
            ctx.lines_as(
                "Bliant Piyord",
                args!["This test will gauge your skills in battle. Only those with great fighting ability can become Crusaders."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Bliant Piyord",
                args![
                    "To take the battle test, bring",
                    "1 ^3355FFHoly Water^000000. This will be used to purify you prior to taking the test."
                ],
            )?;
            if ctx.call(Function::CheckQuest, vec![Val::from(3013)])? != -1 {
                ctx.call(Function::ChangeQuest, vec![Val::from(3013), Val::from(3014)])?;
            }
            ctx.next()?;
            ctx.lines_as(
                "Bliant Piyord",
                args![
                    "Well then...",
                    "Shall we",
                    "begin right away?",
                    "Or do you need time",
                    "to prepare yourself?"
                ],
            )?;
        } else {
            ctx.lines(args!["Are you prepared", "for the test now?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Bliant Piyord",
                args!["Would you like to begin now, or do you still need time to make preparations?"],
            )?;
        }
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from("I would like to begin.:Give me some time to prepare.")],
        )?) == 1
        {
            if (ctx.call(Function::GetEquipId, vec![ctx.constant("EQI_ACC_L")?])? != 2608
                && ctx.call(Function::GetEquipId, vec![ctx.constant("EQI_ACC_R")?])? != 2608)
            {
                ctx.lines_as(
                    "Bliant Piyord",
                    args!["Just a second, you do not have a Rosary equipped. As a Crusader, you must always have a Rosary on your person."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Bliant Piyord",
                    args!["Come back and take", "the test after you", "have a Rosary."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.call(Function::CountItem, vec![Val::from(523)])?.number()? < 1 {
                ctx.lines_as(
                    "Bliant Piyord",
                    args![
                        "For the battle test, please prepare one ^3355FFHoly water^000000.",
                        "I told you just to purify the candidates."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Bliant Piyord",
                    args!["Get prepares and come back to here.", "I will wait for you."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.call(Function::DelItem, vec![Val::from(523), Val::from(1)])?;
            ctx.lines_as(
                "Bliant Piyord",
                args![
                    "Then, I'll start",
                    "the test. You will",
                    "be purified with the",
                    "Holy Water you prepared."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Bliant Piyord", args!["..............."])?;
            ctx.next()?;
            ctx.lines_as(
                "Bliant Piyord",
                args![
                    "Let's begin.",
                    "Go and enter",
                    "the waiting room.",
                    "Defeat the monsters",
                    "that appear in",
                    "the 4 stages."
                ],
            )?;
            ctx.close_window()?;
            ctx.call(Function::Warp, vec![Val::from("job_cru"), Val::from(24), Val::from(169)])?;
            return Err(Stop::End);
        }
        ctx.lines_as("Bliant Piyord", args!["Prepare 1 ^3355FFHoly Water^000000 to take the battle test. As I've said, it will be used to purify you prior to taking the test."])?;
        ctx.next()?;
        ctx.lines_as(
            "Bliant Piyord",
            args![
                "Come back and take",
                "the test after you",
                "have prepared",
                "1 Holy Water.",
                "I shall be",
                "waiting."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("crus_q").get()? == 10 {
        ctx.lines(args![
            "Congratulations.",
            "You have completed",
            "all the tests to",
            "become a Crusader."
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Bliant Piyord",
            args![
                "Go talk to our",
                "leader inside.",
                "We welcome you into",
                "the ranks of those",
                "preparing for the",
                "coming Holy War."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.mes("Are you not one of the ones in the process of becoming a Crusader?")?;
    ctx.next()?;
    ctx.lines_as(
        "Bliant Piyord",
        args![
            "It's not your turn",
            "to take my test yet.",
            "Come back after taking",
            "all the other tests."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Bliant Piyord",
        args!["I'll see you soon.", "May Odin's blessings", "be with you."],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn patron_knight(ctx: &Ctx) -> Script {
    patron_knight_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum SummonerCr1Step {
    Start,
    OnTimer300000,
    OnTimer300500,
    OnTimer301000,
    OnInit,
    OnStart,
    OnReset,
    OnEnd,
    OnDead,
}

fn summoner_cr1_run(ctx: &Ctx, mut step: SummonerCr1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            SummonerCr1Step::Start => {
                step = SummonerCr1Step::OnTimer300000;
                continue 'machine;
            }
            SummonerCr1Step::OnTimer300000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Summoner#cr1::OnReset")])?;
                return Err(Stop::End);
            }
            SummonerCr1Step::OnTimer300500 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Summoner#cr1::OnEnd")])?;
                return Err(Stop::End);
            }
            SummonerCr1Step::OnTimer301000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Summoner#cr1::OnStart")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            SummonerCr1Step::OnInit => {
                ctx.call(Function::EnableNpc, vec![Val::from("Summoner#cr1")])?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(94),
                        Val::from(45),
                        Val::from("Mandragora"),
                        Val::from(1020),
                        Val::from(1),
                        Val::from("Summoner#cr1::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(94),
                        Val::from(55),
                        Val::from("Mandragora"),
                        Val::from(1020),
                        Val::from(1),
                        Val::from("Summoner#cr1::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(94),
                        Val::from(65),
                        Val::from("Mandragora"),
                        Val::from(1020),
                        Val::from(1),
                        Val::from("Summoner#cr1::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(94),
                        Val::from(75),
                        Val::from("Mandragora"),
                        Val::from(1020),
                        Val::from(1),
                        Val::from("Summoner#cr1::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(94),
                        Val::from(85),
                        Val::from("Mandragora"),
                        Val::from(1020),
                        Val::from(1),
                        Val::from("Summoner#cr1::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(94),
                        Val::from(95),
                        Val::from("Mandragora"),
                        Val::from(1020),
                        Val::from(1),
                        Val::from("Summoner#cr1::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(101),
                        Val::from(45),
                        Val::from("Mandragora"),
                        Val::from(1020),
                        Val::from(1),
                        Val::from("Summoner#cr1::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(101),
                        Val::from(55),
                        Val::from("Mandragora"),
                        Val::from(1020),
                        Val::from(1),
                        Val::from("Summoner#cr1::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(101),
                        Val::from(65),
                        Val::from("Mandragora"),
                        Val::from(1020),
                        Val::from(1),
                        Val::from("Summoner#cr1::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(101),
                        Val::from(75),
                        Val::from("Mandragora"),
                        Val::from(1020),
                        Val::from(1),
                        Val::from("Summoner#cr1::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(101),
                        Val::from(85),
                        Val::from("Mandragora"),
                        Val::from(1020),
                        Val::from(1),
                        Val::from("Summoner#cr1::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(101),
                        Val::from(95),
                        Val::from("Mandragora"),
                        Val::from(1020),
                        Val::from(1),
                        Val::from("Summoner#cr1::OnDead"),
                    ],
                )?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            SummonerCr1Step::OnStart => {
                ctx.call(Function::EnableNpc, vec![Val::from("Summoner#cr1")])?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(94),
                        Val::from(45),
                        Val::from("Mandragora"),
                        Val::from(1020),
                        Val::from(1),
                        Val::from("Summoner#cr1::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(94),
                        Val::from(55),
                        Val::from("Mandragora"),
                        Val::from(1020),
                        Val::from(1),
                        Val::from("Summoner#cr1::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(94),
                        Val::from(65),
                        Val::from("Mandragora"),
                        Val::from(1020),
                        Val::from(1),
                        Val::from("Summoner#cr1::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(94),
                        Val::from(75),
                        Val::from("Mandragora"),
                        Val::from(1020),
                        Val::from(1),
                        Val::from("Summoner#cr1::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(94),
                        Val::from(85),
                        Val::from("Mandragora"),
                        Val::from(1020),
                        Val::from(1),
                        Val::from("Summoner#cr1::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(94),
                        Val::from(95),
                        Val::from("Mandragora"),
                        Val::from(1020),
                        Val::from(1),
                        Val::from("Summoner#cr1::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(101),
                        Val::from(45),
                        Val::from("Mandragora"),
                        Val::from(1020),
                        Val::from(1),
                        Val::from("Summoner#cr1::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(101),
                        Val::from(55),
                        Val::from("Mandragora"),
                        Val::from(1020),
                        Val::from(1),
                        Val::from("Summoner#cr1::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(101),
                        Val::from(65),
                        Val::from("Mandragora"),
                        Val::from(1020),
                        Val::from(1),
                        Val::from("Summoner#cr1::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(101),
                        Val::from(75),
                        Val::from("Mandragora"),
                        Val::from(1020),
                        Val::from(1),
                        Val::from("Summoner#cr1::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(101),
                        Val::from(85),
                        Val::from("Mandragora"),
                        Val::from(1020),
                        Val::from(1),
                        Val::from("Summoner#cr1::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(101),
                        Val::from(95),
                        Val::from("Mandragora"),
                        Val::from(1020),
                        Val::from(1),
                        Val::from("Summoner#cr1::OnDead"),
                    ],
                )?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            SummonerCr1Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("job_cru"), Val::from("Summoner#cr1::OnDead")],
                )?;
                return Err(Stop::End);
            }
            SummonerCr1Step::OnEnd => {
                ctx.call(Function::DisableNpc, vec![Val::from("Summoner#cr1")])?;
                return Err(Stop::End);
            }
            SummonerCr1Step::OnDead => {
                ctx.call(Function::Warp, vec![Val::from("prt_fild05"), Val::from(353), Val::from(251)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn summoner_cr1(ctx: &Ctx) -> Script {
    summoner_cr1_run(ctx, SummonerCr1Step::Start, Vec::new()).map(|_| ())
}

pub fn summoner_cr1_ontimer300000(ctx: &Ctx) -> Script {
    summoner_cr1_run(ctx, SummonerCr1Step::OnTimer300000, Vec::new()).map(|_| ())
}

pub fn summoner_cr1_ontimer300500(ctx: &Ctx) -> Script {
    summoner_cr1_run(ctx, SummonerCr1Step::OnTimer300500, Vec::new()).map(|_| ())
}

pub fn summoner_cr1_ontimer301000(ctx: &Ctx) -> Script {
    summoner_cr1_run(ctx, SummonerCr1Step::OnTimer301000, Vec::new()).map(|_| ())
}

pub fn summoner_cr1_oninit(ctx: &Ctx) -> Script {
    summoner_cr1_run(ctx, SummonerCr1Step::OnInit, Vec::new()).map(|_| ())
}

pub fn summoner_cr1_onstart(ctx: &Ctx) -> Script {
    summoner_cr1_run(ctx, SummonerCr1Step::OnStart, Vec::new()).map(|_| ())
}

pub fn summoner_cr1_onreset(ctx: &Ctx) -> Script {
    summoner_cr1_run(ctx, SummonerCr1Step::OnReset, Vec::new()).map(|_| ())
}

pub fn summoner_cr1_onend(ctx: &Ctx) -> Script {
    summoner_cr1_run(ctx, SummonerCr1Step::OnEnd, Vec::new()).map(|_| ())
}

pub fn summoner_cr1_ondead(ctx: &Ctx) -> Script {
    summoner_cr1_run(ctx, SummonerCr1Step::OnDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum SummonerCr2Step {
    Start,
    OnTimer345000,
    OnTimer345500,
    OnTimer346000,
    OnInit,
    OnStart,
    OnReset,
    OnEnd,
    OnDead,
}

fn summoner_cr2_run(ctx: &Ctx, mut step: SummonerCr2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            SummonerCr2Step::Start => {
                step = SummonerCr2Step::OnTimer345000;
                continue 'machine;
            }
            SummonerCr2Step::OnTimer345000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Summoner#cr2::OnReset")])?;
                return Err(Stop::End);
            }
            SummonerCr2Step::OnTimer345500 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Summoner#cr2::OnEnd")])?;
                return Err(Stop::End);
            }
            SummonerCr2Step::OnTimer346000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Summoner#cr2::OnStart")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            SummonerCr2Step::OnInit => {
                ctx.call(Function::EnableNpc, vec![Val::from("Summoner#cr2")])?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(98),
                        Val::from(50),
                        Val::from("Flora"),
                        Val::from(1118),
                        Val::from(1),
                        Val::from("Summoner#cr2::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(92),
                        Val::from(60),
                        Val::from("Flora"),
                        Val::from(1118),
                        Val::from(1),
                        Val::from("Summoner#cr2::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(104),
                        Val::from(60),
                        Val::from("Flora"),
                        Val::from(1118),
                        Val::from(1),
                        Val::from("Summoner#cr2::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(98),
                        Val::from(70),
                        Val::from("Flora"),
                        Val::from(1118),
                        Val::from(1),
                        Val::from("Summoner#cr2::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(92),
                        Val::from(80),
                        Val::from("Flora"),
                        Val::from(1118),
                        Val::from(1),
                        Val::from("Summoner#cr2::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(104),
                        Val::from(90),
                        Val::from("Flora"),
                        Val::from(1118),
                        Val::from(1),
                        Val::from("Summoner#cr2::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(98),
                        Val::from(90),
                        Val::from("Flora"),
                        Val::from(1118),
                        Val::from(1),
                        Val::from("Summoner#cr2::OnDead"),
                    ],
                )?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            SummonerCr2Step::OnStart => {
                ctx.call(Function::EnableNpc, vec![Val::from("Summoner#cr2")])?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(98),
                        Val::from(50),
                        Val::from("Flora"),
                        Val::from(1118),
                        Val::from(1),
                        Val::from("Summoner#cr2::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(92),
                        Val::from(60),
                        Val::from("Flora"),
                        Val::from(1118),
                        Val::from(1),
                        Val::from("Summoner#cr2::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(104),
                        Val::from(60),
                        Val::from("Flora"),
                        Val::from(1118),
                        Val::from(1),
                        Val::from("Summoner#cr2::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(98),
                        Val::from(70),
                        Val::from("Flora"),
                        Val::from(1118),
                        Val::from(1),
                        Val::from("Summoner#cr2::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(92),
                        Val::from(80),
                        Val::from("Flora"),
                        Val::from(1118),
                        Val::from(1),
                        Val::from("Summoner#cr2::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(104),
                        Val::from(90),
                        Val::from("Flora"),
                        Val::from(1118),
                        Val::from(1),
                        Val::from("Summoner#cr2::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(98),
                        Val::from(90),
                        Val::from("Flora"),
                        Val::from(1118),
                        Val::from(1),
                        Val::from("Summoner#cr2::OnDead"),
                    ],
                )?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            SummonerCr2Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("job_cru"), Val::from("Summoner#cr2::OnDead")],
                )?;
                return Err(Stop::End);
            }
            SummonerCr2Step::OnEnd => {
                ctx.call(Function::DisableNpc, vec![Val::from("Summoner#cr2")])?;
                return Err(Stop::End);
            }
            SummonerCr2Step::OnDead => {
                ctx.call(Function::Warp, vec![Val::from("prt_fild05"), Val::from(353), Val::from(251)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn summoner_cr2(ctx: &Ctx) -> Script {
    summoner_cr2_run(ctx, SummonerCr2Step::Start, Vec::new()).map(|_| ())
}

pub fn summoner_cr2_ontimer345000(ctx: &Ctx) -> Script {
    summoner_cr2_run(ctx, SummonerCr2Step::OnTimer345000, Vec::new()).map(|_| ())
}

pub fn summoner_cr2_ontimer345500(ctx: &Ctx) -> Script {
    summoner_cr2_run(ctx, SummonerCr2Step::OnTimer345500, Vec::new()).map(|_| ())
}

pub fn summoner_cr2_ontimer346000(ctx: &Ctx) -> Script {
    summoner_cr2_run(ctx, SummonerCr2Step::OnTimer346000, Vec::new()).map(|_| ())
}

pub fn summoner_cr2_oninit(ctx: &Ctx) -> Script {
    summoner_cr2_run(ctx, SummonerCr2Step::OnInit, Vec::new()).map(|_| ())
}

pub fn summoner_cr2_onstart(ctx: &Ctx) -> Script {
    summoner_cr2_run(ctx, SummonerCr2Step::OnStart, Vec::new()).map(|_| ())
}

pub fn summoner_cr2_onreset(ctx: &Ctx) -> Script {
    summoner_cr2_run(ctx, SummonerCr2Step::OnReset, Vec::new()).map(|_| ())
}

pub fn summoner_cr2_onend(ctx: &Ctx) -> Script {
    summoner_cr2_run(ctx, SummonerCr2Step::OnEnd, Vec::new()).map(|_| ())
}

pub fn summoner_cr2_ondead(ctx: &Ctx) -> Script {
    summoner_cr2_run(ctx, SummonerCr2Step::OnDead, Vec::new()).map(|_| ())
}

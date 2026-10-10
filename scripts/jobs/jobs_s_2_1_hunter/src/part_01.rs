use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn hunter_info_hnt_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args![
        "============ Notice ============",
        "We would like to inform that the Hunter Job Change Location",
        "has been moved to ^ff0000Hugel^000000 in the Schwarzwald Republic."
    ])?;
    ctx.next()?;
    ctx.mes("You can now use the Hugel airline, so please use the airship to visit Hugel.")?;
    ctx.next()?;
    ctx.mes("You will find the new Job Change Location at ^ff0000 Hugel 208 222 ^000000.")?;
    ctx.next()?;
    ctx.lines(args![
        "^804000(You found a tiny line written at the end of the notice.)^000000",
        " ",
        " ",
        " ",
        "I, the Falcon breeder have moved out as well."
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn hunter_info_hnt(ctx: &Ctx) -> Script {
    hunter_info_hnt_body(ctx, Vec::new()).map(|_| ())
}

fn hunter_guildsman_hnt_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_hunter_t = Val::from(0);
    let mut l_joblvl = Val::from(0);
    if ctx.var("Upper").get()? == 1 {
        ctx.lines_as(
            "Hunter Sherin",
            args!["Oh, how have you been? It's been a long time, hasn't it?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Hunter Sherin",
            args![
                "...Wait.",
                "Oops! I'm sorry, I could have sworn that we've met before. Huh. How weird."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ARCHER")?) && ctx.var("JobLevel").get()?.number()? < 40) {
        ctx.lines_as("Hunter Guildsman", args!["Eh? You haven't had enough training as an Archer yet. To become a Hunter, you must gain a certain level of experience as an Archer first."])?;
        ctx.next()?;
        ctx.lines_as("Hunter Guildsman", args!["Go out and train yourself a bit more. You'll need to be at least Job Level 40 before you'll be ready to become a Hunter. Of course, you can train more than that if you want."])?;
        ctx.next()?;
        ctx.lines_as("Hunter Guildsman", args!["See you next time~"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("SkillPoint").get()?.is_true() {
        ctx.lines_as("Hunter Sherin", args!["You can't change jobs without using all your skill points. Please use all of your skill points before applying to change jobs~"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_NOVICE")?) {
        ctx.lines_as(
            "Hunter Guildsman",
            args![
                "You must be...",
                "a Novice? Wow, you've got a long way ahead of you. I don't think there's much you can do here."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_SWORDMAN")?) {
            ctx.lines_as("Hunter Guildsman", args!["You have chosen the path of the sword. I can respect the strength of your blade. Of course, from far away, you can't really afford to throw your sword at enemies, can you?"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_MAGE")?) {
                ctx.lines_as(
                    "Hunter Guildsman",
                    args!["You deal with magic? It must feel great to be able to wield mystic power."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?) {
                    ctx.lines_as(
                        "Hunter Guildsman",
                        args![
                            "Oh, a person who serves God! Nice to meet you. There aren't many people like you that visit this place~ Hehe."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_MERCHANT")?) {
                        ctx.lines_as("Hunter Guildsman", args!["Oh...", "How's your business coming along?"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_THIEF")?) {
                            ctx.lines_as(
                                "Hunter Guildsman",
                                args!["Agh?!", "This place doesn't have anything worth stealing or anyone worth killing!!"],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_DANCER")?)
                                || ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_BARD")?))
                            {
                                ctx.lines_as(
                                    "Hunter Guildsman",
                                    args![
                                        "Phew...sometimes it is really hard for being a hunter, you know...",
                                        "So how do you like your bohemian life? Yeah, I am envious of you for having so much freedom..."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_HUNTER")?) {
                                    ctx.lines_as(
                                        "Hunter Sherin",
                                        args![
                                            ((Val::from("Oh~ ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("!!")),
                                            "Long time no see~ What brings you here? Did your Falcon run away or something?"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Hunter Sherin", args!["I don't really have any official notices from the guild at the moment, so I hope you didn't come here just for that..."])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ARCHER")?) {
                                        if (ctx.var("hntr_q").get()? == 17 && ctx.call(Function::CountItem, vec![Val::from(1007)])? == 0) {
                                            ctx.lines_as("Hunter Sherin", args!["Hmm... I've been informed that you passed the test. But you don't have the necessary 'Necklace of Wisdom (Penetration)' that proves your accomplishment."])?;
                                            ctx.next()?;
                                            ctx.lines_as("Hunter Sherin", args!["I'll let you change your job, but you must bring back a 'Necklace of Wisdom (Penetration),' no matter what it takes."])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        } else if (ctx.var("hntr_q").get()? == 17
                                            && ctx.call(Function::CountItem, vec![Val::from(1007)])?.number()? > 0)
                                        {
                                            ctx.lines_as(
                                                "Hunter Sherin",
                                                args!["Oh...?", "You passed", "the job test?!", "Congratulations~!!"],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Hunter Sherin",
                                                args!["Well then,", "I will now change", "your job to a Hunter~"],
                                            )?;
                                            ctx.next()?;
                                            ctx.call(Function::DelItem, vec![Val::from(1007), Val::from(1)])?;
                                            ctx.lines_as("Hunter Sherin", args!["Tada~ Congratulations!", "You look great as a Hunter!!"])?;
                                            l_joblvl = ctx.var("JobLevel").get()?;
                                            ctx.call(Function::CompleteQuest, vec![Val::from(4013)])?;
                                            shared::other_global_functions::job_change(ctx, vec![ctx.constant("JOB_HUNTER")?])?;
                                            shared::other_global_functions::f_clearjobvar(ctx, vec![])?;
                                            ctx.next()?;
                                            ctx.lines_as("Hunter Sherin", args!["Become a noble person and become a worthy representative of our Hunter Guild. Show your love of nature as a Hunter~"])?;
                                            ctx.next()?;
                                            ctx.lines_as("Hunter Sherin", args!["And also, here is a little reward for all the effort you put in. It's from me, of course~"])?;
                                            if l_joblvl.clone().number()? > 49 {
                                                ctx.call(Function::GetItem, vec![Val::from(1718), Val::from(1)])?;
                                            } else {
                                                ctx.call(Function::GetItem, vec![Val::from(1710), Val::from(1)])?;
                                            }
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        } else if (ctx.var("hntr_q").get()? == 0 || ctx.var("hntr_q").get()? == 1) {
                                            if ctx.var("hntr_q").get()? == 0 {
                                                ctx.lines_as("Hunter Guildsman", args!["Oh, you're an Archer! It seems as if you've trained enough as an Archer... You came here to become a Hunter, right?"])?;
                                                ctx.next()?;
                                                match runtime::select_values(
                                                    ctx,
                                                    &[
                                                        Val::from("Yes. That's what I'm here for."),
                                                        Val::from("What are the requirements to change jobs?"),
                                                        Val::from("....I don't want to change yet."),
                                                    ],
                                                )? {
                                                    1 => {
                                                        ctx.lines_as(
                                                            "Hunter Guildsman",
                                                            args![
                                                                "Hehe~",
                                                                "I was right! Let me put you on the candidate list. Let's see~*"
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Hunter Guildsman", args!["Hmm... Let's start with the interview. Wait and relax a moment. No need to worry, I'll prepare everything~"])?;
                                                        ctx.next()?;
                                                        ctx.mes("^3355FF*Gathers and flips through papers*^000000")?;
                                                        ctx.next()?;
                                                        ctx.mes("^3355FF*Rummage rummage*^000000")?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Hunter Guildsman", args!["Ah!", "Here they are: the interview questions~ First of all, my name is 'Sherin.' Nice to meet you!"])?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Hunter Sherin", args!["Well then,", "shall we begin?"])?;
                                                        ctx.next()?;
                                                        if Val::from(runtime::select_values(
                                                            ctx,
                                                            &[Val::from("Yes~ Let's start now."), Val::from("No, I'll be back later.")],
                                                        )?) == 2
                                                        {
                                                            ctx.lines_as(
                                                                "Hunter Sherin",
                                                                args!["Okay...", "Come back", "when you're ready~"],
                                                            )?;
                                                            ctx.close_window()?;
                                                            return Err(Stop::End);
                                                        }
                                                        if ctx.call(Function::IsBeginQuest, vec![Val::from(4000)])? == 0 {
                                                            ctx.call(Function::SetQuest, vec![Val::from(4000)])?;
                                                        }
                                                        ctx.lines_as("Hunter Sherin", args!["Listen carefully to the scenarios I describe. When I ask a question, you choose an answer. Pretty simple, don't you think?"])?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Hunter Sherin", args!["I just want to know how you think about life, and why you want to become a Hunter, so there's no need to be nervous."])?;
                                                    }
                                                    2 => {
                                                        ctx.lines_as(
                                                            "Hunter Guildsman",
                                                            args!["Job change", "requirements?", "First...", "You must be an Archer."],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Hunter Guildsman",
                                                            args!["Second...", "You must be", "at least Job Level 40."],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Hunter Guildsman", args!["Third...", "You must bring all of the items that will be requested by the guild. You can worry about that later."])?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Hunter Guildsman", args!["Fourth...", "You've gotta pass the test administered by the guild. If you have had enough training as an Archer, you should be able to pass the test~"])?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    }
                                                    3 => {
                                                        ctx.lines_as("Hunter Guildsman", args!["Okay then,", "see you next time~"])?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    }
                                                    _ => {}
                                                }
                                            } else if ctx.var("hntr_q").get()? == 1 {
                                                ctx.lines_as("Hunter Sherin", args!["Welcome back...!", "Well, let's start with the interview. This time, carefully think about the answers to each question."])?;
                                            }
                                            ctx.next()?;
                                            ctx.lines_as("Hunter Sherin", args!["Well then,", "let's begin."])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Hunter Sherin",
                                                args!["You are an Archer, and you don't know where you should go to hunt. What do you do?"],
                                            )?;
                                            ctx.next()?;
                                            'b2: {
                                                let subject2 = Val::from(runtime::select_values(
                                                    ctx,
                                                    &[
                                                        Val::from("Scream out loud asking where you should go."),
                                                        Val::from("Quietly ask a person passing by."),
                                                        Val::from("Wander around alone and search for a place."),
                                                    ],
                                                )?);
                                                let mut matched2 = false;
                                                let no_case2 =
                                                    !subject2.loosely_equals(&Val::from(2)) && !subject2.loosely_equals(&Val::from(3));
                                                if !matched2 && subject2.loosely_equals(&Val::from(2)) {
                                                    matched2 = true;
                                                }
                                                if !matched2 && subject2.loosely_equals(&Val::from(3)) {
                                                    matched2 = true;
                                                }
                                                if matched2 {
                                                    l_hunter_t = (l_hunter_t.clone() + Val::from(10));
                                                }
                                                if !matched2 && no_case2 {
                                                    matched2 = true;
                                                }
                                                if matched2 {
                                                    break 'b2;
                                                }
                                            }
                                            ctx.lines_as("Hunter Sherin", args!["So you've decided on a place to hunt. You're going to hunt the monsters known as Hodes in the Sograt Desert."])?;
                                            ctx.next()?;
                                            ctx.lines_as("Hunter Sherin", args!["But you are in Payon!!", "How do you go to the desert?"])?;
                                            ctx.next()?;
                                            'b3: {
                                                let subject3 = Val::from(runtime::select_values(
                                                    ctx,
                                                    &[
                                                        Val::from("Ask a Priest to open a free warp portal."),
                                                        Val::from("Use the Kafra service."),
                                                        Val::from("Walk with a friend."),
                                                    ],
                                                )?);
                                                let mut matched3 = false;
                                                let no_case3 =
                                                    !subject3.loosely_equals(&Val::from(2)) && !subject3.loosely_equals(&Val::from(3));
                                                if !matched3 && subject3.loosely_equals(&Val::from(2)) {
                                                    matched3 = true;
                                                }
                                                if !matched3 && subject3.loosely_equals(&Val::from(3)) {
                                                    matched3 = true;
                                                }
                                                if matched3 {
                                                    l_hunter_t = (l_hunter_t.clone() + Val::from(10));
                                                }
                                                if !matched3 && no_case3 {
                                                    matched3 = true;
                                                }
                                                if matched3 {
                                                    break 'b3;
                                                }
                                            }
                                            ctx.lines_as("Hunter Sherin", args!["There is no Priest to ask for a warp, and no friend is around to walk with you. You must use the Kafra service, but you have no Zeny!"])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Hunter Sherin",
                                                args!["How would you go", "about to make the", "Zeny that you need?"],
                                            )?;
                                            ctx.next()?;
                                            'b4: {
                                                let subject4 = Val::from(runtime::select_values(
                                                    ctx,
                                                    &[
                                                        Val::from("Beg here and there."),
                                                        Val::from("Sell items I do not need."),
                                                        Val::from("Hunt at a nearby field."),
                                                    ],
                                                )?);
                                                let mut matched4 = false;
                                                let no_case4 =
                                                    !subject4.loosely_equals(&Val::from(2)) && !subject4.loosely_equals(&Val::from(3));
                                                if !matched4 && subject4.loosely_equals(&Val::from(2)) {
                                                    matched4 = true;
                                                }
                                                if !matched4 && subject4.loosely_equals(&Val::from(3)) {
                                                    matched4 = true;
                                                }
                                                if matched4 {
                                                    l_hunter_t = (l_hunter_t.clone() + Val::from(10));
                                                }
                                                if !matched4 && no_case4 {
                                                    matched4 = true;
                                                }
                                                if matched4 {
                                                    break 'b4;
                                                }
                                            }
                                            ctx.lines_as("Hunter Sherin", args!["So you finally arrive at the Sograt Desert. But you realize that Hodes are a bit too strong for you to hunt alone."])?;
                                            ctx.next()?;
                                            ctx.lines_as("Hunter Sherin", args!["What is your", "solution to this", "situation?"])?;
                                            ctx.next()?;
                                            if Val::from(runtime::select_values(
                                                ctx,
                                                &[
                                                    Val::from("Attack a Hode from the top of a hill."),
                                                    Val::from("Go back to town."),
                                                    Val::from("Attack someone else's Hode."),
                                                ],
                                            )?) == 2
                                            {
                                                l_hunter_t = (l_hunter_t.clone() + Val::from(10));
                                            }
                                            ctx.lines_as("Hunter Sherin", args!["Let's say you were having too much trouble hunting Hodes and returned to town. Now you are out of HP and a Priest happens to be around. How would you ask for a Heal?"])?;
                                            ctx.next()?;
                                            if Val::from(runtime::select_values(
                                                ctx,
                                                &[
                                                    Val::from("Would it be possible to get a Heal, please?"),
                                                    Val::from("Heal, please."),
                                                    Val::from("Heal me."),
                                                ],
                                            )?) == 1
                                            {
                                                l_hunter_t = (l_hunter_t.clone() + Val::from(10));
                                            }
                                            ctx.lines_as("Hunter Sherin", args!["This time, you found a rare item while you were going through your inventory. You go out to sell the item, and there are many people with stores and chatrooms open."])?;
                                            ctx.next()?;
                                            ctx.lines_as("Hunter Sherin", args!["What is the", "best way to", "sell your item?"])?;
                                            ctx.next()?;
                                            'b5: {
                                                let subject5 = Val::from(runtime::select_values(
                                                    ctx,
                                                    &[
                                                        Val::from("Scream out loud to everyone in sight."),
                                                        Val::from("Open a chatroom and wait."),
                                                        Val::from("Look to see if anyone already wants it."),
                                                    ],
                                                )?);
                                                let mut matched5 = false;
                                                let no_case5 =
                                                    !subject5.loosely_equals(&Val::from(2)) && !subject5.loosely_equals(&Val::from(3));
                                                if !matched5 && subject5.loosely_equals(&Val::from(2)) {
                                                    matched5 = true;
                                                }
                                                if !matched5 && subject5.loosely_equals(&Val::from(3)) {
                                                    matched5 = true;
                                                }
                                                if matched5 {
                                                    l_hunter_t = (l_hunter_t.clone() + Val::from(10));
                                                }
                                                if !matched5 && no_case5 {
                                                    matched5 = true;
                                                }
                                                if matched5 {
                                                    break 'b5;
                                                }
                                            }
                                            ctx.lines_as(
                                                "Hunter Sherin",
                                                args!["While you are waiting, someone is begging for items and Zeny. What should you do?"],
                                            )?;
                                            ctx.next()?;
                                            if Val::from(runtime::select_values(
                                                ctx,
                                                &[
                                                    Val::from("Give some of my items and Zeny."),
                                                    Val::from("Ignore and walk away."),
                                                    Val::from("Tell the person about a good place to hunt."),
                                                ],
                                            )?) == 1
                                            {
                                                l_hunter_t = (l_hunter_t.clone() + Val::from(10));
                                            }
                                            ctx.lines_as("Hunter Sherin", args!["By now, you decide to go to the Maze by yourself."])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Hunter Sherin",
                                                args!["But on your way, you run", "into someone that is lost.", "What should you do?"],
                                            )?;
                                            ctx.next()?;
                                            'b6: {
                                                let subject6 = Val::from(runtime::select_values(
                                                    ctx,
                                                    &[
                                                        Val::from("Tell them which way to go."),
                                                        Val::from("Guide them to their destination."),
                                                        Val::from("Ignore."),
                                                    ],
                                                )?);
                                                let mut matched6 = false;
                                                let no_case6 =
                                                    !subject6.loosely_equals(&Val::from(1)) && !subject6.loosely_equals(&Val::from(2));
                                                if !matched6 && subject6.loosely_equals(&Val::from(1)) {
                                                    matched6 = true;
                                                }
                                                if !matched6 && subject6.loosely_equals(&Val::from(2)) {
                                                    matched6 = true;
                                                }
                                                if matched6 {
                                                    l_hunter_t = (l_hunter_t.clone() + Val::from(10));
                                                }
                                                if !matched6 && no_case6 {
                                                    matched6 = true;
                                                }
                                                if matched6 {
                                                    break 'b6;
                                                }
                                            }
                                            ctx.lines_as("Hunter Sherin", args!["After meeting this lost person, you decide to get back to hunting. Just then, you find that someone is attacking a boss!"])?;
                                            ctx.next()?;
                                            ctx.lines_as("Hunter Sherin", args!["What should you do?"])?;
                                            ctx.next()?;
                                            if Val::from(runtime::select_values(
                                                ctx,
                                                &[
                                                    Val::from("Watch, then attack when asked for help."),
                                                    Val::from("Attack and see what happens."),
                                                    Val::from("Just go back to town."),
                                                ],
                                            )?) == 1
                                            {
                                                l_hunter_t = (l_hunter_t.clone() + Val::from(10));
                                            }
                                            ctx.lines_as(
                                                "Hunter Sherin",
                                                args![
                                                    "You are now very exhausted after your day of hunting. It's time to go back to town."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as("Hunter Sherin", args!["But what's this!? You find an expensive item lying on the floor! What should you do with it?"])?;
                                            ctx.next()?;
                                            'b7: {
                                                let subject7 = Val::from(runtime::select_values(
                                                    ctx,
                                                    &[
                                                        Val::from("Pick it up and keep it."),
                                                        Val::from("Try to find the owner."),
                                                        Val::from("Just walk by."),
                                                    ],
                                                )?);
                                                let mut matched7 = false;
                                                let no_case7 =
                                                    !subject7.loosely_equals(&Val::from(2)) && !subject7.loosely_equals(&Val::from(3));
                                                if !matched7 && subject7.loosely_equals(&Val::from(2)) {
                                                    matched7 = true;
                                                }
                                                if !matched7 && subject7.loosely_equals(&Val::from(3)) {
                                                    matched7 = true;
                                                }
                                                if matched7 {
                                                    l_hunter_t = (l_hunter_t.clone() + Val::from(10));
                                                }
                                                if !matched7 && no_case7 {
                                                    matched7 = true;
                                                }
                                                if matched7 {
                                                    break 'b7;
                                                }
                                            }
                                            ctx.lines_as("Hunter Sherin", args!["Okay, this is the end of the test!"])?;
                                            ctx.next()?;
                                            if l_hunter_t.clone() == 100 {
                                                ctx.var("hntr_q").set(Val::from(2))?;
                                                ctx.call(Function::ChangeQuest, vec![Val::from(4000), Val::from(4001)])?;
                                                ctx.lines_as("Hunter Sherin", args!["Well done! Your answers show you've got the right outlook on life. You definitely have the right qualities to become a Hunter~"])?;
                                                ctx.next()?;
                                                ctx.lines_as("Hunter Sherin", args!["I'm glad to say you've passed the interview~ Now, go to that person in the corner and confirm which item is necessary for your job change test~"])?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            } else if l_hunter_t.clone() == 90 {
                                                ctx.var("hntr_q").set(Val::from(2))?;
                                                ctx.call(Function::ChangeQuest, vec![Val::from(4000), Val::from(4001)])?;
                                                ctx.lines_as("Hunter Sherin", args!["Well, I'm looking at your answers and your score isn't perfect. But I'll let you pass anyway. I don't know what our Guildmaster will think, though."])?;
                                                ctx.next()?;
                                                ctx.lines_as("Hunter Sherin", args!["Just remember to always keep basic etiquette in mind. Try harder in the following tests and make me happy, okay?"])?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                            ctx.var("hntr_q").set(Val::from(1))?;
                                            ctx.lines_as("Hunter Sherin", args!["Hmm... I don't think this'll work out. You can't become a Hunter without knowing basic etiquette."])?;
                                            ctx.next()?;
                                            ctx.lines_as("Hunter Sherin", args!["Think about how you answered the questions one more time. In order to value nature, you must first value your relationship with others."])?;
                                            ctx.next()?;
                                            ctx.lines_as("Hunter Sherin", args!["Then, you'll be more in tune with nature and with other people. It's this kind of harmony that makes the best Hunters."])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        } else if (ctx.var("hntr_q").get()?.number()? > 2 && ctx.var("hntr_q").get()?.number()? < 10) {
                                            ctx.lines_as("Hunter Sherin", args!["Just give the item to the Demon Hunter, the guy who's all the way to the left in this area~"])?;
                                            ctx.next()?;
                                            ctx.mes("[Hunter Sherin]")?;
                                            if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                                                ctx.mes("If you decide to become a Hunter, promise to come visit me. I want to see you as a Hunter. You would look great!!")?;
                                            } else {
                                                ctx.mes("If you decide to become a Hunter, come and visit me, okay? You're pretty, but... You'd be even prettier as a Hunter. Hehe~")?;
                                            }
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        } else if ctx.var("hntr_q").get()? == 2 {
                                            ctx.lines_as("Hunter Sherin", args!["????"])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Hunter Sherin",
                                                args!["You weren't able to find the Demon Hunter? That guy is just to the left! Hehe~"],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as("Hunter Sherin", args!["I want to see you become a Hunter...! Waaah~ I think you'd look great, so please hurry!"])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        ctx.lines_as("Hunter Sherin", args!["Hmm, I think you're supposed to visit someone else, not me. Like our Guildmaster who's out for business, you know?"])?;
                                        ctx.next()?;
                                        ctx.lines_as("Hunter Sherin", args!["I heard he's at the Payon Central Palace... Or in the Archer Village, one of those two places. Try visiting those places and find him, okay?"])?;
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
    ctx.lines_as(
        "Hunter Guildsman",
        args!["Eh? You haven't trained enough as an Archer. To become a Hunter, you need to gain more experience as an Archer first."],
    )?;
    ctx.next()?;
    ctx.lines_as("Hunter Guildsman", args!["Well then, go train some more. You must be at least Job Level 40 to be exact. Of course, you can always go higher. See you next time~"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn hunter_guildsman_hnt(ctx: &Ctx) -> Script {
    hunter_guildsman_hnt_body(ctx, Vec::new()).map(|_| ())
}

fn guild_receptionist_hnt_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    let mut l_items: Vec<Val> = Vec::new();
    if ctx.var("hntr_q").get()? == 2 {
        ctx.lines_as("Guild Receptionist", args![((Val::from("Greetings. They call me... ^660000The Demon Hunter^000000. I am the one in charge of processing applications. Your name is ... ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(", correct?"))])?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from("Yes, that is correct."), Val::from("Nope~~(heeheehee)")],
        )?) == 2
        {
            ctx.lines_as(
                "Demon Hunter",
                args![
                    "Hey, stop messing around.",
                    ((Val::from("Your name is ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(", right?"))
                ],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Yes..."), Val::from("Hehehe. I keep telling you, it's not~~")],
            )?) == 2
            {
                ctx.lines_as("Demon Hunter", args!["Leave if you plan to trifle me your petty tricks. Do you not realize that you are toying with ^660000The Demon Hunter^000000?!'"])?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("hugel"), Val::from(208), Val::from(223)])?;
                return Err(Stop::End);
            }
        }
        ctx.lines_as("Demon Hunter", args!["Okay. These are the items you need for the test. Since we provide all the arrows you will be using for this test, we need you to get the materials to make them."])?;
        ctx.next()?;
        ctx.lines_as(
            "Demon Hunter",
            args!["You see, we're having some financial problems. Let's see, we're short on these items..."],
        )?;
        ctx.next()?;
        if Val::from(0).loosely_equals(&Val::from(1)) {
            l_i = Val::from(1);
        }
        let subject1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(7)])?;
        if subject1 == 1 {
            let base = Val::from(0).number()?;
            runtime::local_set(
                &mut l_items,
                &Val::from(base + 0),
                (if l_i.clone().is_true() { Val::from(928) } else { Val::from(7030) }),
                false,
            );
            runtime::local_set(&mut l_items, &Val::from(base + 1), Val::from(5), false);
            runtime::local_set(&mut l_items, &Val::from(base + 2), Val::from(1019), false);
            runtime::local_set(&mut l_items, &Val::from(base + 3), Val::from(5), false);
            runtime::local_set(&mut l_items, &Val::from(base + 4), Val::from(509), false);
            runtime::local_set(&mut l_items, &Val::from(base + 5), Val::from(3), false);
            runtime::local_set(&mut l_items, &Val::from(base + 6), Val::from(3), false);
            ctx.call(Function::ChangeQuest, vec![Val::from(4001), Val::from(4002)])?;
        } else if subject1 == 2 {
            let base = Val::from(0).number()?;
            runtime::local_set(&mut l_items, &Val::from(base + 0), Val::from(925), false);
            runtime::local_set(&mut l_items, &Val::from(base + 1), Val::from(3), false);
            runtime::local_set(&mut l_items, &Val::from(base + 2), Val::from(932), false);
            runtime::local_set(&mut l_items, &Val::from(base + 3), Val::from(5), false);
            runtime::local_set(&mut l_items, &Val::from(base + 4), Val::from(511), false);
            runtime::local_set(&mut l_items, &Val::from(base + 5), Val::from(3), false);
            runtime::local_set(&mut l_items, &Val::from(base + 6), Val::from(4), false);
            ctx.call(Function::ChangeQuest, vec![Val::from(4001), Val::from(4003)])?;
        } else if subject1 == 3 {
            let base = Val::from(0).number()?;
            runtime::local_set(
                &mut l_items,
                &Val::from(base + 0),
                (if l_i.clone().is_true() { Val::from(1013) } else { Val::from(937) }),
                false,
            );
            runtime::local_set(&mut l_items, &Val::from(base + 1), Val::from(3), false);
            runtime::local_set(&mut l_items, &Val::from(base + 2), Val::from(919), false);
            runtime::local_set(&mut l_items, &Val::from(base + 3), Val::from(3), false);
            runtime::local_set(&mut l_items, &Val::from(base + 4), Val::from(507), false);
            runtime::local_set(&mut l_items, &Val::from(base + 5), Val::from(5), false);
            runtime::local_set(&mut l_items, &Val::from(base + 6), Val::from(5), false);
            ctx.call(Function::ChangeQuest, vec![Val::from(4001), Val::from(4004)])?;
        } else if subject1 == 4 {
            let base = Val::from(0).number()?;
            runtime::local_set(
                &mut l_items,
                &Val::from(base + 0),
                (if l_i.clone().is_true() { Val::from(947) } else { Val::from(1021) }),
                false,
            );
            runtime::local_set(&mut l_items, &Val::from(base + 1), Val::from(3), false);
            runtime::local_set(
                &mut l_items,
                &Val::from(base + 2),
                (if l_i.clone().is_true() { Val::from(7033) } else { Val::from(7032) }),
                false,
            );
            runtime::local_set(&mut l_items, &Val::from(base + 3), Val::from(3), false);
            runtime::local_set(&mut l_items, &Val::from(base + 4), Val::from(914), false);
            runtime::local_set(&mut l_items, &Val::from(base + 5), Val::from(10), false);
            runtime::local_set(&mut l_items, &Val::from(base + 6), Val::from(6), false);
            ctx.call(Function::ChangeQuest, vec![Val::from(4001), Val::from(4005)])?;
        } else if subject1 == 5 {
            let base = Val::from(0).number()?;
            runtime::local_set(&mut l_items, &Val::from(base + 0), Val::from(935), false);
            runtime::local_set(&mut l_items, &Val::from(base + 1), Val::from(9), false);
            runtime::local_set(&mut l_items, &Val::from(base + 2), Val::from(955), false);
            runtime::local_set(&mut l_items, &Val::from(base + 3), Val::from(9), false);
            runtime::local_set(&mut l_items, &Val::from(base + 4), Val::from(508), false);
            runtime::local_set(&mut l_items, &Val::from(base + 5), Val::from(9), false);
            runtime::local_set(&mut l_items, &Val::from(base + 6), Val::from(7), false);
            ctx.call(Function::ChangeQuest, vec![Val::from(4001), Val::from(4006)])?;
        } else if subject1 == 6 {
            let base = Val::from(0).number()?;
            runtime::local_set(&mut l_items, &Val::from(base + 0), Val::from(913), false);
            runtime::local_set(&mut l_items, &Val::from(base + 1), Val::from(3), false);
            runtime::local_set(&mut l_items, &Val::from(base + 2), Val::from(938), false);
            runtime::local_set(&mut l_items, &Val::from(base + 3), Val::from(1), false);
            runtime::local_set(&mut l_items, &Val::from(base + 4), Val::from(948), false);
            runtime::local_set(&mut l_items, &Val::from(base + 5), Val::from(1), false);
            runtime::local_set(&mut l_items, &Val::from(base + 6), Val::from(8), false);
            ctx.call(Function::ChangeQuest, vec![Val::from(4001), Val::from(4007)])?;
        } else if subject1 == 7 {
            let base = Val::from(0).number()?;
            runtime::local_set(&mut l_items, &Val::from(base + 0), Val::from(1027), false);
            runtime::local_set(&mut l_items, &Val::from(base + 1), Val::from(2), false);
            runtime::local_set(&mut l_items, &Val::from(base + 2), Val::from(942), false);
            runtime::local_set(&mut l_items, &Val::from(base + 3), Val::from(1), false);
            runtime::local_set(&mut l_items, &Val::from(base + 4), Val::from(1026), false);
            runtime::local_set(&mut l_items, &Val::from(base + 5), Val::from(1), false);
            runtime::local_set(&mut l_items, &Val::from(base + 6), Val::from(9), false);
            ctx.call(Function::ChangeQuest, vec![Val::from(4001), Val::from(4008)])?;
        }
        ctx.lines_as(
            "Demon Hunter",
            args![
                ((((((((((((Val::from("Hmm. ^660000") + runtime::local_get(&l_items, &Val::from(1), false)) + Val::from(" "))
                    + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_items, &Val::from(0), false)])?)
                    + Val::from("^000000 to use for arrow tips. ^660000"))
                    + runtime::local_get(&l_items, &Val::from(3), false))
                    + Val::from(" "))
                    + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_items, &Val::from(2), false)])?)
                    + Val::from("^000000 to use here and there. And ^660000"))
                    + runtime::local_get(&l_items, &Val::from(5), false))
                    + Val::from(" "))
                    + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_items, &Val::from(4), false)])?)
                    + Val::from("^000000 please."))
            ],
        )?;
        ctx.var("hntr_q").set(runtime::local_get(&l_items, &Val::from(6), false))?;
        ctx.next()?;
        ctx.mes("[Demon Hunter]")?;
        if ctx.var("hntr_q").get()?.number()? >= 5 {
            ctx.mes("By the way, a member of our Hunter Guild is at the Archer Guild on official business. I believe you have to go visit this person since he is in charge of the testing.")?;
        } else {
            ctx.lines(args!["Oh right. Our Guildmaster has gone on an official trip to the Payon Central Palace. You have to go visit him because the one that administers the test.", "You can find him in a building east of the Payon Central Palace."])?;
        }
        ctx.next()?;
        ctx.lines_as(
            "Demon Hunter",
            args!["Alright then, come back to me when you have everything ready~"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if (ctx.var("hntr_q").get()?.number()? >= 3 && ctx.var("hntr_q").get()?.number()? <= 9) {
            if Val::from(0).loosely_equals(&Val::from(1)) {
                l_i = Val::from(1);
            }
            let subject2 = ctx.var("hntr_q").get()?;
            if subject2 == 3 {
                let base = Val::from(0).number()?;
                runtime::local_set(
                    &mut l_items,
                    &Val::from(base + 0),
                    (if l_i.clone().is_true() { Val::from(928) } else { Val::from(7030) }),
                    false,
                );
                runtime::local_set(&mut l_items, &Val::from(base + 1), Val::from(5), false);
                runtime::local_set(&mut l_items, &Val::from(base + 2), Val::from(1019), false);
                runtime::local_set(&mut l_items, &Val::from(base + 3), Val::from(5), false);
                runtime::local_set(&mut l_items, &Val::from(base + 4), Val::from(509), false);
                runtime::local_set(&mut l_items, &Val::from(base + 5), Val::from(3), false);
                runtime::local_set(&mut l_items, &Val::from(base + 6), Val::from(10), false);
            } else if subject2 == 4 {
                let base = Val::from(0).number()?;
                runtime::local_set(&mut l_items, &Val::from(base + 0), Val::from(925), false);
                runtime::local_set(&mut l_items, &Val::from(base + 1), Val::from(3), false);
                runtime::local_set(&mut l_items, &Val::from(base + 2), Val::from(932), false);
                runtime::local_set(&mut l_items, &Val::from(base + 3), Val::from(5), false);
                runtime::local_set(&mut l_items, &Val::from(base + 4), Val::from(511), false);
                runtime::local_set(&mut l_items, &Val::from(base + 5), Val::from(3), false);
                runtime::local_set(&mut l_items, &Val::from(base + 6), Val::from(10), false);
            } else if subject2 == 5 {
                let base = Val::from(0).number()?;
                runtime::local_set(
                    &mut l_items,
                    &Val::from(base + 0),
                    (if l_i.clone().is_true() { Val::from(1013) } else { Val::from(937) }),
                    false,
                );
                runtime::local_set(&mut l_items, &Val::from(base + 1), Val::from(3), false);
                runtime::local_set(&mut l_items, &Val::from(base + 2), Val::from(919), false);
                runtime::local_set(&mut l_items, &Val::from(base + 3), Val::from(3), false);
                runtime::local_set(&mut l_items, &Val::from(base + 4), Val::from(507), false);
                runtime::local_set(&mut l_items, &Val::from(base + 5), Val::from(5), false);
                runtime::local_set(&mut l_items, &Val::from(base + 6), Val::from(10), false);
            } else if subject2 == 6 {
                let base = Val::from(0).number()?;
                runtime::local_set(
                    &mut l_items,
                    &Val::from(base + 0),
                    (if l_i.clone().is_true() { Val::from(947) } else { Val::from(1021) }),
                    false,
                );
                runtime::local_set(&mut l_items, &Val::from(base + 1), Val::from(3), false);
                runtime::local_set(
                    &mut l_items,
                    &Val::from(base + 2),
                    (if l_i.clone().is_true() { Val::from(7033) } else { Val::from(7032) }),
                    false,
                );
                runtime::local_set(&mut l_items, &Val::from(base + 3), Val::from(3), false);
                runtime::local_set(&mut l_items, &Val::from(base + 4), Val::from(914), false);
                runtime::local_set(&mut l_items, &Val::from(base + 5), Val::from(10), false);
                runtime::local_set(&mut l_items, &Val::from(base + 6), Val::from(10), false);
            } else if subject2 == 7 {
                let base = Val::from(0).number()?;
                runtime::local_set(&mut l_items, &Val::from(base + 0), Val::from(935), false);
                runtime::local_set(&mut l_items, &Val::from(base + 1), Val::from(9), false);
                runtime::local_set(&mut l_items, &Val::from(base + 2), Val::from(955), false);
                runtime::local_set(&mut l_items, &Val::from(base + 3), Val::from(9), false);
                runtime::local_set(&mut l_items, &Val::from(base + 4), Val::from(508), false);
                runtime::local_set(&mut l_items, &Val::from(base + 5), Val::from(9), false);
                runtime::local_set(&mut l_items, &Val::from(base + 6), Val::from(11), false);
            } else if subject2 == 8 {
                let base = Val::from(0).number()?;
                runtime::local_set(&mut l_items, &Val::from(base + 0), Val::from(913), false);
                runtime::local_set(&mut l_items, &Val::from(base + 1), Val::from(3), false);
                runtime::local_set(&mut l_items, &Val::from(base + 2), Val::from(938), false);
                runtime::local_set(&mut l_items, &Val::from(base + 3), Val::from(1), false);
                runtime::local_set(&mut l_items, &Val::from(base + 4), Val::from(948), false);
                runtime::local_set(&mut l_items, &Val::from(base + 5), Val::from(1), false);
                runtime::local_set(&mut l_items, &Val::from(base + 6), Val::from(11), false);
            } else if subject2 == 9 {
                let base = Val::from(0).number()?;
                runtime::local_set(&mut l_items, &Val::from(base + 0), Val::from(1027), false);
                runtime::local_set(&mut l_items, &Val::from(base + 1), Val::from(2), false);
                runtime::local_set(&mut l_items, &Val::from(base + 2), Val::from(942), false);
                runtime::local_set(&mut l_items, &Val::from(base + 3), Val::from(1), false);
                runtime::local_set(&mut l_items, &Val::from(base + 4), Val::from(1026), false);
                runtime::local_set(&mut l_items, &Val::from(base + 5), Val::from(1), false);
                runtime::local_set(&mut l_items, &Val::from(base + 6), Val::from(11), false);
            }
            ctx.lines_as("Demon Hunter", args!["Hmm?"])?;
            ctx.next()?;
            if ((runtime::op(
                &ctx.call(Function::CountItem, vec![runtime::local_get(&l_items, &Val::from(0), false)])?,
                ">=",
                &runtime::local_get(&l_items, &Val::from(1), false),
            )?
            .is_true()
                && runtime::op(
                    &ctx.call(Function::CountItem, vec![runtime::local_get(&l_items, &Val::from(2), false)])?,
                    ">=",
                    &runtime::local_get(&l_items, &Val::from(3), false),
                )?
                .is_true())
                && runtime::op(
                    &ctx.call(Function::CountItem, vec![runtime::local_get(&l_items, &Val::from(4), false)])?,
                    ">=",
                    &runtime::local_get(&l_items, &Val::from(5), false),
                )?
                .is_true())
            {
                ctx.var("hntr_q").set(runtime::local_get(&l_items, &Val::from(6), false))?;
                if ctx.call(Function::IsBeginQuest, vec![Val::from(4002)])? == 1 {
                    ctx.call(Function::ChangeQuest, vec![Val::from(4002), Val::from(4009)])?;
                } else {
                    if ctx.call(Function::IsBeginQuest, vec![Val::from(4003)])? == 1 {
                        ctx.call(Function::ChangeQuest, vec![Val::from(4003), Val::from(4009)])?;
                    } else if ctx.call(Function::IsBeginQuest, vec![Val::from(4004)])? == 1 {
                        ctx.call(Function::ChangeQuest, vec![Val::from(4004), Val::from(4009)])?;
                    } else if ctx.call(Function::IsBeginQuest, vec![Val::from(4005)])? == 1 {
                        ctx.call(Function::ChangeQuest, vec![Val::from(4005), Val::from(4009)])?;
                    } else if ctx.call(Function::IsBeginQuest, vec![Val::from(4006)])? == 1 {
                        ctx.call(Function::ChangeQuest, vec![Val::from(4006), Val::from(4010)])?;
                    } else if ctx.call(Function::IsBeginQuest, vec![Val::from(4007)])? == 1 {
                        ctx.call(Function::ChangeQuest, vec![Val::from(4007), Val::from(4010)])?;
                    } else {
                        ctx.call(Function::ChangeQuest, vec![Val::from(4008), Val::from(4010)])?;
                    }
                }
                ctx.call(
                    Function::DelItem,
                    vec![
                        runtime::local_get(&l_items, &Val::from(0), false),
                        runtime::local_get(&l_items, &Val::from(1), false),
                    ],
                )?;
                ctx.call(
                    Function::DelItem,
                    vec![
                        runtime::local_get(&l_items, &Val::from(2), false),
                        runtime::local_get(&l_items, &Val::from(3), false),
                    ],
                )?;
                ctx.call(
                    Function::DelItem,
                    vec![
                        runtime::local_get(&l_items, &Val::from(4), false),
                        runtime::local_get(&l_items, &Val::from(5), false),
                    ],
                )?;
                ctx.mes("[Demon Hunter]")?;
                if ctx.var("hntr_q").get()? == 10 {
                    ctx.mes("You brought all of the necessary materials... You can get directions to the testing area from our Guildmaster who is currently in the Payon Central Palace.")?;
                } else {
                    ctx.mes("You brought all of the necessary materials... To get directions to the testing area, go talk to our Guildmaster who is at the Archer Guild.")?;
                }
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as("Demon Hunter", args!["You don't have all", "of the required materials..."])?;
            ctx.next()?;
            ctx.lines_as(
                "Demon Hunter",
                args![
                    "The items you need are",
                    ((((Val::from("^660000") + runtime::local_get(&l_items, &Val::from(1), false)) + Val::from(" "))
                        + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_items, &Val::from(0), false)])?)
                        + Val::from("^000000,")),
                    ((((Val::from("^660000") + runtime::local_get(&l_items, &Val::from(3), false)) + Val::from(" "))
                        + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_items, &Val::from(2), false)])?)
                        + Val::from("^000000 and")),
                    ((((Val::from("^660000") + runtime::local_get(&l_items, &Val::from(5), false)) + Val::from(" "))
                        + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_items, &Val::from(4), false)])?)
                        + Val::from("^000000.")),
                    "Come back once you have",
                    "gathered all the items."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if (ctx.var("hntr_q").get()?.number()? > 9 && ctx.var("hntr_q").get()?.number()? < 17) {
            ctx.lines_as("Demon Hunter", args!["Hmm? You didn't go to the Guildmaster either in Payon Central Palace or at the Archer Guild? He should be at one of the two places, so go look for him."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("hntr_q").get()? == 17 {
            ctx.lines_as(
                "Demon Hunter",
                args!["Ooh. You passed the test. Congratulations~ You should go talk to Sherin now."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Guild Receptionist",
                args!["If you wish to change your job to a Hunter, you must register first."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
}

pub fn guild_receptionist_hnt(ctx: &Ctx) -> Script {
    guild_receptionist_hnt_body(ctx, Vec::new()).map(|_| ())
}

fn hunter_htngm_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("hntr_q").get()? == 10 {
        ctx.lines_as(
            "Hunter Guildmaster",
            args![
                "Hmpf. You must be here for the Hunter job test. Let me tell you about the testing process. What would you like to know?"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(
            ctx,
            &[
                Val::from("What is the test?"),
                Val::from("What are the passing requirements?"),
                Val::from("Any warnings?"),
                Val::from("Begin test."),
            ],
        )? {
            1 => {
                ctx.lines_as("Hunter Guildmaster", args!["You have to hunt down certain monsters with a particular name. But you must avoid all the traps while you're at it."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Hunter Guildmaster",
                    args!["This is to test your ability to move swiftly and locate targets in various situations."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as("Hunter Guildmaster", args!["Within the time limit, starting in the 6 o'clock direction of the map, you must hunt the target monsters and then hit the escape switch that will appear in the center of the map."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Hunter Guildmaster",
                    args!["You will pass if you are able to escape in the 12 o'clock direction of the map after you hit the switch."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            3 => {
                ctx.lines_as("Hunter Guildmaster", args!["Hmm. Warnings... Well, if you fall into a trap, you have to start from the beginning. Also, only one person can take the test at a time."])?;
                ctx.next()?;
                ctx.lines_as("Hunter Guildmaster", args!["I will send you to the testing place. There will be a waiting room, but if someone else is taking the test, you must wait in the chatroom."])?;
                ctx.next()?;
                ctx.lines_as("Hunter Guildmaster", args!["If the person in front succeeds, resigns or fails, the next person waiting in the chatroom is sent to the testing area. If nobody is waiting, the test will begin as soon as you enter the chatroom."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            4 => {
                ctx.lines_as("Hunter Guildmaster", args!["Okay. I'll send you to the testing area right now. Don't blame me if you get lost or confused because you didn't listen to my explanation."])?;
                ctx.next()?;
            }
            _ => {}
        }
        ctx.lines_as(
            "Hunter Guildmaster",
            args!["Well, then. Your arrows are probably still being made, so you can use mine to take the test."],
        )?;
        ctx.var("hntr_q").set(Val::from(12))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(4009), Val::from(4011)])?;
        ctx.call(Function::GetItem, vec![Val::from(1751), Val::from(200)])?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("job_hunte"), Val::from(176), Val::from(22)])?;
        return Err(Stop::End);
    } else {
        if (ctx.var("hntr_q").get()?.number()? > 1 && ctx.var("hntr_q").get()?.number()? < 10) {
            ctx.lines_as("Hunter Guildmaster", args!["Mmm...?", "What is an Archer", "visiting me for?"])?;
            ctx.next()?;
            ctx.lines_as("Hunter Guildmaster", args!["I wasn't notified about anything in particular from the Hunter Guild. You're not trying to skip the middle part of the Hunter test, are you?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Hunter Guildmaster",
                args!["Go gather the items for the test, and come back after you've visited the Hunter Guild."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("hntr_q").get()? == 11 {
            ctx.lines_as(
                "Hunter",
                args!["Hmm? Can I help you? If you wish to change jobs, you should visit the person at the Archer Guild, not me."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if (ctx.var("hntr_q").get()?.number()? > 11 && ctx.var("hntr_q").get()?.number()? < 16) {
            ctx.lines_as("Hunter Guildmaster", args!["Hmm. You're the Archer that almost gave up on changing jobs. You do have everything ready, right? Then I'll send you to take the test right away."])?;
            ctx.next()?;
            ctx.lines_as("Hunter Guildmaster", args!["If you have any", "questions, ask now."])?;
            ctx.next()?;
            match runtime::select_values(
                ctx,
                &[
                    Val::from("What is the test?"),
                    Val::from("What are the passing requirements?"),
                    Val::from("Any warnings?"),
                    Val::from("Begin test."),
                ],
            )? {
                1 => {
                    ctx.lines_as("Hunter Guildmaster", args!["You have to hunt down certain monsters with a particular name, but you also avoid all the traps at the same time. This is to test your ability to move swiftly and locate targets in various situations."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines_as("Hunter Guildmaster", args!["Within the given time, you will start at the 6 o'clock direction of the map, hunt the target monsters, and then hit the escape switch that will appear in the center of the map."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Hunter Guildmaster",
                        args!["You will pass if you are able to escape in the 12 o'clock direction of the map after you hit the switch."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                3 => {
                    ctx.lines_as("Hunter Guildmaster", args!["Hmm. Warnings... Well, if you fall into a trap, you have to start from the beginning. Also, only one person can take the test at a time."])?;
                    ctx.next()?;
                    ctx.lines_as("Hunter Guildmaster", args!["I'll send you to the testing place. There is a waiting room, but if someone else is taking the test, you must wait in the chatroom."])?;
                    ctx.next()?;
                    ctx.lines_as("Hunter Guildmaster", args!["If the person in front succeeds, resigns or fails, the next person waiting in the chatroom is sent to the testing area. If nobody is waiting, the test will begin as soon as you enter the chatroom."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                4 => {
                    ctx.lines_as("Hunter Guildmaster", args!["Okay. Good luck.", "I'll send you right now."])?;
                    ctx.next()?;
                }
                _ => {}
            }
            ctx.lines_as(
                "Hunter Guildmaster",
                args!["Eh? Why won't I give you Silver Arrows? Don't expect anything when you don't bring any materials."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Hunter Guildmaster",
                args!["Anyway, I believe you've prepared it yourself. Let's begin now."],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Okay. Let's start..."), Val::from("Ah, wait a sec.")],
            )?) == 1
            {
                ctx.lines_as("Hunter Guildmaster", args!["Okay!! I hope", "you will pass this time!"])?;
                ctx.close_window()?;
                ctx.var("hntr_q").set(Val::from(12))?;
                ctx.call(Function::Warp, vec![Val::from("job_hunte"), Val::from(176), Val::from(22)])?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Hunter Guildmaster",
                args!["Then hurry and finish", "all of your preparations."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("hntr_q").get()? == 16 {
            ctx.lines_as(
                "Hunter Guildmaster",
                args![
                    "Wow, you came back in one piece!",
                    "I mean, good job. I'll give you the item which proves that you have passed the test."
                ],
            )?;
            ctx.var("hntr_q").set(Val::from(17))?;
            ctx.call(
                Function::SavePoint,
                vec![Val::from("payon"), Val::from(104), Val::from(99), Val::from(1), Val::from(1)],
            )?;
            ctx.call(Function::GetItem, vec![Val::from(1007), Val::from(1)])?;
            ctx.call(Function::ChangeQuest, vec![Val::from(4012), Val::from(4013)])?;
            ctx.next()?;
            ctx.lines_as("Hunter Guildmaster", args!["Okay, here it is. Now, go back to the Hunter Guild. I have some more business left to do here, but I hope you can become a Hunter soon."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("hntr_q").get()? == 17 {
            ctx.lines_as("Hunter Guildmaster", args!["I see you aren't in a hurry to become a Hunter? When I first became a Hunter, I ran around for a month wildly brandishing my bow because I was so happy. Hehe~"])?;
            ctx.next()?;
            ctx.lines_as("Hunter Guildmaster", args!["Now, you should go back to the Hunter Guild~"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Hunter",
                args![
                    "...Can I help you?",
                    "I'm here for official business and am busy at the moment. If you'll excuse me..."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
}

pub fn hunter_htngm(ctx: &Ctx) -> Script {
    hunter_htngm_body(ctx, Vec::new()).map(|_| ())
}

fn hunter_htngm2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::Cutin, vec![Val::from("job_huntermaster"), Val::from(2)])?;
    if ctx.var("hntr_q").get()? == 11 {
        ctx.lines_as(
            "Hunter Guildmaster",
            args!["Mmm. I see you're here for the Hunter job test. Let me explain the testing process. What would you like to know?"],
        )?;
        ctx.next()?;
        match runtime::select_values(
            ctx,
            &[
                Val::from("What is the test?"),
                Val::from("What are the passing requirements?"),
                Val::from("Any warnings?"),
                Val::from("Begin test."),
            ],
        )? {
            1 => {
                ctx.lines_as("Hunter Guildmaster", args!["You have to hunt down certain monsters with a particular name, but you must avoid all of the traps here at the same time."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Hunter Guildmaster",
                    args!["This is to test your ability to move swiftly and locate targets in various situations."],
                )?;
                ctx.close_window()?;
                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as("Hunter Guildmaster", args!["Within the given time, you will start in the 6 o'clock direction of the map, hunt the target monsters, and hit the escape switch that will appear in the center of the map."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Hunter Guildmaster",
                    args!["You will pass if you are able to escape in the 12 o'clock direction of the map when the switch is activated."],
                )?;
                ctx.close_window()?;
                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                return Err(Stop::End);
            }
            3 => {
                ctx.lines_as("Hunter Guildmaster", args!["Mmm. Warnings... Well, if you fall into a trap, you have to start the test over from the beginning. And only one person can take the test at a time."])?;
                ctx.next()?;
                ctx.lines_as("Hunter Guildmaster", args!["I'll send you to the testing place. There is a waiting room, but if someone else is taking the test, you must wait in the chatroom."])?;
                ctx.next()?;
                ctx.lines_as("Hunter Guildmaster", args!["If the person currently taking the test passes or fails, the next person waiting in the chatroom is sent to the testing area. If nobody is waiting, the test will begin as soon as you enter the chatroom."])?;
                ctx.close_window()?;
                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                return Err(Stop::End);
            }
            4 => {
                ctx.lines_as("Hunter Guildmaster", args!["...Okay. I'll send you to the testing area right now. Don't blame me if you get lost or confused because you didn't listen to the explanation."])?;
                ctx.next()?;
            }
            _ => {}
        }
        ctx.lines_as(
            "Hunter Guildmaster",
            args!["Well, your arrows are probably still being made, so you can use mine to take the test."],
        )?;
        ctx.call(Function::GetItem, vec![Val::from(1751), Val::from(200)])?;
        ctx.next()?;
        ctx.lines_as("Hunter Guildmaster", args!["Good luck."])?;
        ctx.var("hntr_q").set(Val::from(12))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(4010), Val::from(4011)])?;
        ctx.call(Function::Warp, vec![Val::from("job_hunte"), Val::from(176), Val::from(22)])?;
        return Err(Stop::End);
    } else {
        if (ctx.var("hntr_q").get()?.number()? > 1 && ctx.var("hntr_q").get()?.number()? < 10) {
            ctx.lines_as("Hunter Guildmaster", args!["Mmm...?", "Why is an Archer", "visiting me?"])?;
            ctx.next()?;
            ctx.lines_as("Hunter Guildmaster", args!["I wasn't notified by the Hunter Guild about anything. You're not trying to skip the middle part of the Hunter test, are you?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Hunter Guildmaster",
                args!["Stop by once you have gathered all the items needed for the test."],
            )?;
            ctx.close_window()?;
            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
            return Err(Stop::End);
        } else if ctx.var("hntr_q").get()? == 10 {
            ctx.lines_as(
                "Hunter",
                args![
                    "Mmm?",
                    "Can I help you",
                    "with something?",
                    "Oh, you must be",
                    "a Hunter applicant."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Hunter",
                args!["If you wish to change jobs, I think you need to go visit the person at Payon Central Palace."],
            )?;
            ctx.close_window()?;
            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
            return Err(Stop::End);
        } else if (ctx.var("hntr_q").get()?.number()? > 11 && ctx.var("hntr_q").get()?.number()? < 16) {
            ctx.lines_as("Hunter Guildmaster", args!["Mmm? Aren't you the Archer who gave up last time? Now you're ready, right? Then I'll send you to the job change area right now."])?;
            ctx.next()?;
            ctx.lines_as(
                "Hunter Guildmaster",
                args!["If you still", "have questions,", "feel free to ask."],
            )?;
            ctx.next()?;
            match runtime::select_values(
                ctx,
                &[
                    Val::from("What is the test?"),
                    Val::from("What are the requirements to pass the test?"),
                    Val::from("Any warnings?"),
                    Val::from("Begin test."),
                ],
            )? {
                1 => {
                    ctx.lines_as("Hunter Guildmaster", args!["You have to hunt down certain monsters with a particular name, but you must avoid all of the traps here at the same time."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Hunter Guildmaster",
                        args!["This is to test your ability to move swiftly and locate targets in various situations."],
                    )?;
                    ctx.close_window()?;
                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines_as("Hunter Guildmaster", args!["Within the given time, you will start in the 6 o'clock direction of the map, hunt the target monsters, and hit the escape switch that will appear in the center of the map."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Hunter Guildmaster",
                        args![
                            "You will pass if you are able to escape in the 12 o'clock direction of the map when the switch is activated."
                        ],
                    )?;
                    ctx.close_window()?;
                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                    return Err(Stop::End);
                }
                3 => {
                    ctx.lines_as("Hunter Guildmaster", args!["Mmm. Warnings... Well, if you fall into a trap, you have to start the test over from the beginning. And only one person can take the test at a time."])?;
                    ctx.next()?;
                    ctx.lines_as("Hunter Guildmaster", args!["I'll send you to the testing place. There is a waiting room, but if someone else is taking the test, you must wait in the chatroom."])?;
                    ctx.next()?;
                    ctx.lines_as("Hunter Guildmaster", args!["If the person currently taking the test passes or fails, the next person waiting in the chatroom is sent to the testing area. If nobody is waiting, the test will begin as soon as you enter the chatroom."])?;
                    ctx.close_window()?;
                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                    return Err(Stop::End);
                }
                4 => {
                    ctx.lines_as("Hunter Guildmaster", args!["Okay, good luck.", "I'll send you right now..."])?;
                    ctx.next()?;
                }
                _ => {}
            }
            ctx.lines_as("Hunter Guildmaster", args!["Eh? Why don't I give you any Silver Arrows? Well, you shouldn't expect to get anything when you didn't even bring the materials."])?;
            ctx.next()?;
            ctx.lines_as(
                "Hunter Guildmaster",
                args!["Well...", "I believe", "you're ready.", "Let's begin."],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Yes, let's start."), Val::from("Ah, wait a moment.")],
            )?) == 1
            {
                ctx.lines_as("Hunter Guildmaster", args!["Okay!! Now...", "Pass this time!"])?;
                ctx.close_window()?;
                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                ctx.var("hntr_q").set(Val::from(12))?;
                ctx.call(Function::Warp, vec![Val::from("job_hunte"), Val::from(176), Val::from(22)])?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Hunter Guildmaster",
                args!["*Sigh...*", "Come back when", "you're done with", "your preparations."],
            )?;
            ctx.close_window()?;
            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
            return Err(Stop::End);
        } else if ctx.var("hntr_q").get()? == 16 {
            ctx.lines_as(
                "Hunter Guildmaster",
                args![
                    "Wow. You're back in one piece!",
                    "I mean, good job. Well then, I'll give you the item which serves as proof that you passed the test."
                ],
            )?;
            ctx.var("hntr_q").set(Val::from(17))?;
            ctx.call(
                Function::SavePoint,
                vec![Val::from("payon"), Val::from(104), Val::from(99), Val::from(1), Val::from(1)],
            )?;
            ctx.call(Function::GetItem, vec![Val::from(1007), Val::from(1)])?;
            ctx.call(Function::ChangeQuest, vec![Val::from(4012), Val::from(4013)])?;
            ctx.next()?;
            ctx.lines_as(
                "Hunter Guildmaster",
                args!["Well...", "There you go.", "Now hurry back to the Hunter Guild, and join us~"],
            )?;
            ctx.close_window()?;
            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
            return Err(Stop::End);
        } else if ctx.var("hntr_q").get()? == 17 {
            ctx.lines_as("Hunter Guildmaster", args!["I see you're not in a hurry to become a Hunter. I was running around with joy, wildly brandishing my bow the first month I became a Hunter. Hehe~"])?;
            ctx.next()?;
            ctx.lines_as(
                "Hunter Guildmaster",
                args!["Now hurry~", "They're waiting", "for you back", "at the Hunter Guild."],
            )?;
            ctx.close_window()?;
            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Hunter",
                args!["What do you want? I'm on an official trip and am busy at the moment. Now if you'll excuse me."],
            )?;
            ctx.close_window()?;
            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
            return Err(Stop::End);
        }
    }
}

pub fn hunter_htngm2(ctx: &Ctx) -> Script {
    hunter_htngm2_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum GuideHntStep {
    Start,
    OnTouch,
}

fn guide_hnt_run(ctx: &Ctx, mut step: GuideHntStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            GuideHntStep::Start => {
                step = GuideHntStep::OnTouch;
                continue 'machine;
            }
            GuideHntStep::OnTouch => {
                if ctx.var("hntr_q").get()? == 12 {
                    ctx.lines_as(
                        "Guide",
                        args!["Good day. Welcome to the Hunter testing site. The test begins when you enter the next room."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Guide", args!["As explained before, hunt 4 or more of the monsters named ^3355FFJob change monster^000000. Upon doing so, a switch in the center of the map will then appear."])?;
                    ctx.next()?;
                    ctx.lines_as("Guide", args!["When you destroy the switch, the exit will appear in the 12 o'clock direction of the map. Complete everything within the given time and escape."])?;
                    ctx.next()?;
                    ctx.lines_as("Guide", args!["If you faint in the middle, fall into a trap, or go over the time limit, you'll fail. Then you must then retake the test."])?;
                    ctx.next()?;
                    ctx.lines_as("Guide", args!["We will provide the arrows, so just make sure that you bring a bow. Well, then. Please enter when you are ready."])?;
                    ctx.call(
                        Function::SavePoint,
                        vec![Val::from("job_hunte"), Val::from(176), Val::from(22), Val::from(1), Val::from(1)],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if (ctx.var("hntr_q").get()?.number()? > 12 && ctx.var("hntr_q").get()?.number()? < 16) {
                    ctx.lines_as(
                        "Guide",
                        args!["Hmm...", "Did you mess up?", "I'll recover some", "of your HP and SP for now."],
                    )?;
                    ctx.call(Function::PercentHeal, vec![Val::from(100), Val::from(100)])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Guide",
                        args!["If it's too hard, it wouldn't hurt to try again next time. Would you like to resign for now?"],
                    )?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("Keep trying."), Val::from("Resign.")])?) == 1 {
                        ctx.lines_as("Guide", args!["Okay. Do your best and become a great Hunter. Please enter the waiting room. If someone is already taking the test, you must wait until that person is done."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(" has resigned. Next person, please enter.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                    ctx.lines_as(
                        "Guide",
                        args!["Very well. I'll send you to Payon. Hope to see you next time. Don't forget to save when you leave."],
                    )?;
                    ctx.close_window()?;
                    ctx.var("hntr_q").set(Val::from(13))?;
                    ctx.call(
                        Function::SavePoint,
                        vec![Val::from("payon"), Val::from(104), Val::from(99), Val::from(1), Val::from(1)],
                    )?;
                    ctx.call(Function::Warp, vec![Val::from("payon_in02"), Val::from(21), Val::from(27)])?;
                } else if ctx.var("hntr_q").get()?.number()? > 15 {
                    ctx.lines_as(
                        "Guide",
                        args!["You shouldn't be here. How about finding the required job change item first?"],
                    )?;
                    ctx.close_window()?;
                    ctx.call(
                        Function::SavePoint,
                        vec![Val::from("payon"), Val::from(104), Val::from(99), Val::from(1), Val::from(1)],
                    )?;
                    ctx.call(Function::Warp, vec![Val::from("payon_in02"), Val::from(21), Val::from(27)])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn guide_hnt(ctx: &Ctx) -> Script {
    guide_hnt_run(ctx, GuideHntStep::Start, Vec::new()).map(|_| ())
}

pub fn guide_hnt_ontouch(ctx: &Ctx) -> Script {
    guide_hnt_run(ctx, GuideHntStep::OnTouch, Vec::new()).map(|_| ())
}

fn waiting_room_hnt_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn waiting_room_hnt(ctx: &Ctx) -> Script {
    waiting_room_hnt_body(ctx, Vec::new()).map(|_| ())
}

fn waiting_room_hnt_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::WaitingRoom,
        vec![
            Val::from("Waiting Room"),
            Val::from(10),
            Val::from("Waiting Room#hnt::OnStartArena"),
            Val::from(1),
        ],
    )?;
    ctx.call(Function::EnableWaitingRoomEvent, vec![])?;
    return Err(Stop::End);
}

pub fn waiting_room_hnt_oninit(ctx: &Ctx) -> Script {
    waiting_room_hnt_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn waiting_room_hnt_onstartarena_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::WarpWaitingPc,
        vec![Val::from("job_hunte"), Val::from(90), Val::from(67)],
    )?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("Manager#hnt::OnEnable")])?;
    ctx.call(Function::DisableWaitingRoomEvent, vec![])?;
    return Err(Stop::End);
}

pub fn waiting_room_hnt_onstartarena(ctx: &Ctx) -> Script {
    waiting_room_hnt_onstartarena_body(ctx, Vec::new()).map(|_| ())
}

fn waiting_room_hnt_onstart_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableWaitingRoomEvent, vec![])?;
    return Err(Stop::End);
}

pub fn waiting_room_hnt_onstart(ctx: &Ctx) -> Script {
    waiting_room_hnt_onstart_body(ctx, Vec::new()).map(|_| ())
}

pub fn manager_hnt(ctx: &Ctx) -> Script {
    manager_hnt_run(ctx, ManagerHntStep::Start, Vec::new()).map(|_| ())
}

pub fn manager_hnt_oninit(ctx: &Ctx) -> Script {
    manager_hnt_run(ctx, ManagerHntStep::OnInit, Vec::new()).map(|_| ())
}

pub fn manager_hnt_onenable(ctx: &Ctx) -> Script {
    manager_hnt_run(ctx, ManagerHntStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn manager_hnt_onmymobdead(ctx: &Ctx) -> Script {
    manager_hnt_run(ctx, ManagerHntStep::OnMyMobDead, Vec::new()).map(|_| ())
}

pub fn manager_hnt_onmymobdead2(ctx: &Ctx) -> Script {
    manager_hnt_run(ctx, ManagerHntStep::OnMyMobDead2, Vec::new()).map(|_| ())
}

pub fn manager_hnt_onreset(ctx: &Ctx) -> Script {
    manager_hnt_run(ctx, ManagerHntStep::OnReset, Vec::new()).map(|_| ())
}

pub fn manager_hnt_ondisable(ctx: &Ctx) -> Script {
    manager_hnt_run(ctx, ManagerHntStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn manager_hnt_ontimer1000(ctx: &Ctx) -> Script {
    manager_hnt_run(ctx, ManagerHntStep::OnTimer1000, Vec::new()).map(|_| ())
}

pub fn manager_hnt_ontimer3000(ctx: &Ctx) -> Script {
    manager_hnt_run(ctx, ManagerHntStep::OnTimer3000, Vec::new()).map(|_| ())
}

pub fn manager_hnt_ontimer5000(ctx: &Ctx) -> Script {
    manager_hnt_run(ctx, ManagerHntStep::OnTimer5000, Vec::new()).map(|_| ())
}

pub fn manager_hnt_ontimer7000(ctx: &Ctx) -> Script {
    manager_hnt_run(ctx, ManagerHntStep::OnTimer7000, Vec::new()).map(|_| ())
}

pub fn manager_hnt_ontimer9000(ctx: &Ctx) -> Script {
    manager_hnt_run(ctx, ManagerHntStep::OnTimer9000, Vec::new()).map(|_| ())
}

pub fn manager_hnt_ontimer11000(ctx: &Ctx) -> Script {
    manager_hnt_run(ctx, ManagerHntStep::OnTimer11000, Vec::new()).map(|_| ())
}

pub fn manager_hnt_ontimer13000(ctx: &Ctx) -> Script {
    manager_hnt_run(ctx, ManagerHntStep::OnTimer13000, Vec::new()).map(|_| ())
}

pub fn manager_hnt_ontimer14000(ctx: &Ctx) -> Script {
    manager_hnt_run(ctx, ManagerHntStep::OnTimer14000, Vec::new()).map(|_| ())
}

pub fn manager_hnt_ontimer74000(ctx: &Ctx) -> Script {
    manager_hnt_run(ctx, ManagerHntStep::OnTimer74000, Vec::new()).map(|_| ())
}

pub fn manager_hnt_ontimer134000(ctx: &Ctx) -> Script {
    manager_hnt_run(ctx, ManagerHntStep::OnTimer134000, Vec::new()).map(|_| ())
}

pub fn manager_hnt_ontimer164000(ctx: &Ctx) -> Script {
    manager_hnt_run(ctx, ManagerHntStep::OnTimer164000, Vec::new()).map(|_| ())
}

pub fn manager_hnt_ontimer187000(ctx: &Ctx) -> Script {
    manager_hnt_run(ctx, ManagerHntStep::OnTimer187000, Vec::new()).map(|_| ())
}

pub fn manager_hnt_ontimer188000(ctx: &Ctx) -> Script {
    manager_hnt_run(ctx, ManagerHntStep::OnTimer188000, Vec::new()).map(|_| ())
}

pub fn manager_hnt_ontimer189000(ctx: &Ctx) -> Script {
    manager_hnt_run(ctx, ManagerHntStep::OnTimer189000, Vec::new()).map(|_| ())
}

pub fn manager_hnt_ontimer191000(ctx: &Ctx) -> Script {
    manager_hnt_run(ctx, ManagerHntStep::OnTimer191000, Vec::new()).map(|_| ())
}

pub fn manager_hnt_ontimer192000(ctx: &Ctx) -> Script {
    manager_hnt_run(ctx, ManagerHntStep::OnTimer192000, Vec::new()).map(|_| ())
}

pub fn manager_hnt_ontimer193000(ctx: &Ctx) -> Script {
    manager_hnt_run(ctx, ManagerHntStep::OnTimer193000, Vec::new()).map(|_| ())
}

pub fn manager_hnt_ontimer194000(ctx: &Ctx) -> Script {
    manager_hnt_run(ctx, ManagerHntStep::OnTimer194000, Vec::new()).map(|_| ())
}

pub fn manager_hnt_ontimer195000(ctx: &Ctx) -> Script {
    manager_hnt_run(ctx, ManagerHntStep::OnTimer195000, Vec::new()).map(|_| ())
}

pub fn manager_hnt_ontimer197000(ctx: &Ctx) -> Script {
    manager_hnt_run(ctx, ManagerHntStep::OnTimer197000, Vec::new()).map(|_| ())
}

pub fn switch_hnt(ctx: &Ctx) -> Script {
    switch_hnt_run(ctx, SwitchHntStep::Start, Vec::new()).map(|_| ())
}

pub fn switch_hnt_ontouch(ctx: &Ctx) -> Script {
    switch_hnt_run(ctx, SwitchHntStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn switch_hnt_ondisable(ctx: &Ctx) -> Script {
    switch_hnt_run(ctx, SwitchHntStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn switch_hnt_onenable(ctx: &Ctx) -> Script {
    switch_hnt_run(ctx, SwitchHntStep::OnEnable, Vec::new()).map(|_| ())
}

fn exit_hnttest_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn exit_hnttest(ctx: &Ctx) -> Script {
    exit_hnttest_body(ctx, Vec::new()).map(|_| ())
}

fn exit_hnttest_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("exit#hnttest")])?;
    return Err(Stop::End);
}

pub fn exit_hnttest_oninit(ctx: &Ctx) -> Script {
    exit_hnttest_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn exit_hnttest_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DoNpcEvent, vec![Val::from("Manager#hnt::OnReset")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("Waiting Room#hnt::OnStart")])?;
    ctx.var("hntr_q").set(Val::from(16))?;
    ctx.call(Function::ChangeQuest, vec![Val::from(4011), Val::from(4012)])?;
    ctx.call(
        Function::SavePoint,
        vec![Val::from("payon"), Val::from(104), Val::from(99), Val::from(1), Val::from(1)],
    )?;
    if ctx.call(Function::Rand, vec![Val::from(2)])?.is_true() {
        ctx.call(Function::Warp, vec![Val::from("payon_in02"), Val::from(21), Val::from(27)])?;
    } else {
        ctx.call(Function::Warp, vec![Val::from("payon_in03"), Val::from(128), Val::from(7)])?;
    }
    return Err(Stop::End);
}

pub fn exit_hnttest_ontouch(ctx: &Ctx) -> Script {
    exit_hnttest_ontouch_body(ctx, Vec::new()).map(|_| ())
}

use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum HansonNvStep {
    Start,
    SUserJobchoice,
}

fn hanson_nv_run(ctx: &Ctx, mut step: HansonNvStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_acolyte_p = Val::from(0);
    let mut l_archer_p = Val::from(0);
    let mut l_job_c = Val::from(0);
    let mut l_jobs_s: Vec<Val> = Vec::new();
    let mut l_magician_p = Val::from(0);
    let mut l_merchant_p = Val::from(0);
    let mut l_startmap = Val::from(0);
    let mut l_swordman_p = Val::from(0);
    let mut l_thief_p = Val::from(0);
    'machine: loop {
        match step {
            HansonNvStep::Start => {
                if ctx.call(Function::CheckWeight, vec![Val::from(909), Val::from(400)])? == 0 {
                    ctx.lines_as("Hanson", args!["All of the items you are carrying must be quite a burden. Where did you get so much things? Please lighten your weight by getting rid of things you don't need."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ctx.var("nov_3_swordman").get()? == 20 {
                    ctx.lines_as(
                        "Hanson",
                        args![
                            "Good day,",
                            ((Val::from("^A62A2A") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("'^000000.")),
                            "You've made quite",
                            "an effort to come here."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Hanson",
                        args![
                            "This final test in the Training Grounds is a personality test,",
                            "but it's not a mandatory course."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Hanson", args!["However, there are some benefits", "to taking this test. When you take this test, you'll receive many health items which will help you when you join the Ragnarok Online community."])?;
                    ctx.next()?;
                    ctx.lines_as("Hanson", args!["Secondly, after you finish the course we will suggest the job class that seems best suited to your personality and teleport you to a town where you can change into the job we suggested."])?;
                    ctx.next()?;
                    ctx.lines_as("Hanson", args!["Now...", "What would", "you like to do?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Hanson",
                        args!["Do you wish to start Ragnarok Online immediately, or take this personality test course first?"],
                    )?;
                    ctx.next()?;
                    'b1: {
                        let subject1 = Val::from(runtime::select_values(
                            ctx,
                            &[Val::from("I'll take the course.:Let me start Ragnarok Online please.")],
                        )?);
                        let mut matched1 = false;
                        let no_case1 = !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2));
                        if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                            matched1 = true;
                        }
                        if matched1 {
                            ctx.lines_as(
                                "Hanson",
                                args![
                                    "Excellent choice!",
                                    "You're supposed to take every training course if you really want to be a well-prepared player!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Hanson",
                                args!["Alright, let me start the 1st personality test. Please relax", "and take it easy."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Hanson", args!["Remember, this test is only to check your personality, there is no set standard for right and wrong. Now! Let's begin the test!"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Hanson",
                                args!["Please choose the word", "that best matches you from", "among the following."],
                            )?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("Study:Exercise:Public service:Violence")])? {
                                1 => {
                                    l_magician_p = (l_magician_p.clone() + Val::from(1));
                                }
                                2 => {
                                    l_thief_p = (l_thief_p.clone() + Val::from(1));
                                    l_swordman_p = (l_swordman_p.clone() + Val::from(1));
                                }
                                3 => {
                                    l_acolyte_p = (l_acolyte_p.clone() + Val::from(1));
                                }
                                4 => {
                                    l_thief_p = (l_thief_p.clone() + Val::from(1));
                                }
                                _ => {}
                            }
                            match runtime::select_values(ctx, &[Val::from("Change:Conserve")])? {
                                1 => {
                                    l_magician_p = (l_magician_p.clone() + Val::from(1));
                                }
                                2 => {
                                    l_acolyte_p = (l_acolyte_p.clone() + Val::from(1));
                                }
                                _ => {}
                            }
                            match runtime::select_values(ctx, &[Val::from("Consumer:Seller:Producer")])? {
                                1 => {
                                    l_swordman_p = (l_swordman_p.clone() + Val::from(1));
                                    l_thief_p = (l_thief_p.clone() + Val::from(1));
                                    l_acolyte_p = (l_acolyte_p.clone() + Val::from(1));
                                }
                                2 => {
                                    l_merchant_p = (l_merchant_p.clone() + Val::from(1));
                                }
                                3 => {
                                    l_magician_p = (l_magician_p.clone() + Val::from(1));
                                }
                                _ => {}
                            }
                            match runtime::select_values(ctx, &[Val::from("Celebrity:Prudence")])? {
                                1 => {
                                    l_thief_p = (l_thief_p.clone() + Val::from(1));
                                }
                                2 => {
                                    l_archer_p = (l_archer_p.clone() + Val::from(1));
                                }
                                _ => {}
                            }
                            match runtime::select_values(ctx, &[Val::from("Theory:Experience")])? {
                                1 => {
                                    l_magician_p = (l_magician_p.clone() + Val::from(1));
                                }
                                2 => {
                                    l_swordman_p = (l_swordman_p.clone() + Val::from(1));
                                }
                                _ => {}
                            }
                            match runtime::select_values(ctx, &[Val::from("The Past:Reality:The Future")])? {
                                1 => {
                                    l_archer_p = (l_archer_p.clone() + Val::from(1));
                                }
                                2 => {
                                    l_merchant_p = (l_merchant_p.clone() + Val::from(1));
                                    l_thief_p = (l_thief_p.clone() + Val::from(1));
                                }
                                3 => {
                                    l_magician_p = (l_magician_p.clone() + Val::from(1));
                                }
                                _ => {}
                            }
                            ctx.lines_as("Hanson", args!["Please answer", "'Yes' or 'No' to", "the following questions."])?;
                            ctx.next()?;
                            ctx.lines_as("Hanson", args!["I'd rather die", "than live submissively."])?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("Yes.:No.")])? {
                                1 => {
                                    l_swordman_p = (l_swordman_p.clone() + Val::from(1));
                                }
                                2 => {
                                    l_thief_p = (l_thief_p.clone() + Val::from(1));
                                    l_merchant_p = (l_merchant_p.clone() + Val::from(1));
                                }
                                _ => {}
                            }
                            ctx.lines_as("Hanson", args!["You are often upset", "to see someone better", "than you."])?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("Yes.:No.")])? {
                                1 => {
                                    l_merchant_p = (l_merchant_p.clone() + Val::from(1));
                                }
                                2 => {
                                    l_acolyte_p = (l_acolyte_p.clone() + Val::from(1));
                                }
                                _ => {}
                            }
                            ctx.lines_as("Hanson", args!["You don't mind", "exploring dangerous", "places."])?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("Yes.:No.")])? {
                                1 => {
                                    l_swordman_p = (l_swordman_p.clone() + Val::from(1));
                                }
                                2 => {
                                    l_magician_p = (l_magician_p.clone() + Val::from(1));
                                }
                                _ => {}
                            }
                            ctx.lines_as("Hanson", args!["You are", "a leader-type", "person."])?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("Yes.:No.")])? {
                                1 => {
                                    l_swordman_p = (l_swordman_p.clone() + Val::from(1));
                                }
                                2 => {
                                    l_archer_p = (l_archer_p.clone() + Val::from(1));
                                }
                                _ => {}
                            }
                            ctx.lines_as("Hanson", args!["While exploring", "a dungeon, you run", "into a dead end."])?;
                            ctx.next()?;
                            ctx.lines_as("Hanson", args!["However, there is a sign that reads 'Do Not Push' next to a stone that looks strangely like a button on the wall next to you."])?;
                            ctx.next()?;
                            ctx.lines_as("Hanson", args!["Do you give in", "to the urge to push", "this button?"])?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("Yes.:No.")])? {
                                1 => {
                                    l_thief_p = (l_thief_p.clone() + Val::from(1));
                                }
                                2 => {
                                    l_swordman_p = (l_swordman_p.clone() + Val::from(1));
                                }
                                _ => {}
                            }
                            ctx.lines_as("Hanson", args!["You often see", "things that don't exist."])?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("Yes.:No.")])? {
                                1 => {
                                    l_acolyte_p = (l_acolyte_p.clone() + Val::from(1));
                                }
                                2 => {
                                    l_magician_p = (l_magician_p.clone() + Val::from(1));
                                }
                                _ => {}
                            }
                            ctx.lines_as(
                                "Hanson",
                                args!["If you fell off", "a cliff, you'd feel", "like you were flying."],
                            )?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("Yes.:No.")])? {
                                1 => {
                                    l_acolyte_p = (l_acolyte_p.clone() + Val::from(1));
                                }
                                2 => {
                                    l_magician_p = (l_magician_p.clone() + Val::from(1));
                                }
                                _ => {}
                            }
                            ctx.lines_as("Hanson", args!["Money talks."])?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("Yes.:No.")])? {
                                1 => {
                                    l_merchant_p = (l_merchant_p.clone() + Val::from(1));
                                }
                                2 => {
                                    l_archer_p = (l_archer_p.clone() + Val::from(1));
                                }
                                _ => {}
                            }
                            ctx.lines_as("Hanson", args!["Now, let me give you some different questions. Please relax and take it easy, and choose the answer that suits you best."])?;
                            ctx.next()?;
                            ctx.lines_as("Hanson", args!["As you check", "your tight schedule...."])?;
                            ctx.next()?;
                            match runtime::select_values(
                                ctx,
                                &[Val::from(
                                    "You feel like a robot.:You are proud and satisfied.:Schedule? What schedule?",
                                )],
                            )? {
                                1 => {
                                    l_swordman_p = (l_swordman_p.clone() + Val::from(1));
                                    l_thief_p = (l_thief_p.clone() + Val::from(1));
                                }
                                2 => {
                                    l_acolyte_p = (l_acolyte_p.clone() + Val::from(1));
                                    l_magician_p = (l_magician_p.clone() + Val::from(1));
                                }
                                3 => {
                                    l_archer_p = (l_archer_p.clone() + Val::from(1));
                                    l_merchant_p = (l_merchant_p.clone() + Val::from(1));
                                }
                                _ => {}
                            }
                            ctx.lines_as(
                                "Hanson",
                                args![
                                    "As you go window shopping,",
                                    "you find a really interesting item in a store, debating whether or not to buy it. Before making",
                                    "a purchase, the first thing",
                                    "you do is..."
                                ],
                            )?;
                            ctx.next()?;
                            match runtime::select_values(
                                ctx,
                                &[Val::from(
                                    "Consider if you need it.:Check the price.:Don't think twice, just buy it!",
                                )],
                            )? {
                                1 => {
                                    l_archer_p = (l_archer_p.clone() + Val::from(1));
                                }
                                2 => {
                                    l_merchant_p = (l_merchant_p.clone() + Val::from(1));
                                }
                                3 => {
                                    l_thief_p = (l_thief_p.clone() + Val::from(1));
                                }
                                _ => {}
                            }
                            ctx.lines_as(
                                "Hanson",
                                args!["Fill in the blank:", "You ^3355FF_____^000000", "competing with other people..."],
                            )?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("don't mind...:don't like...:don't care about...")])? {
                                1 => {
                                    l_merchant_p = (l_merchant_p.clone() + Val::from(1));
                                }
                                2 => {
                                    l_thief_p = (l_thief_p.clone() + Val::from(1));
                                }
                                3 => {
                                    l_acolyte_p = (l_acolyte_p.clone() + Val::from(1));
                                    l_swordman_p = (l_swordman_p.clone() + Val::from(1));
                                }
                                _ => {}
                            }
                            ctx.lines_as("Hanson", args!["You're responsible for a task that requires you to cooperate with many people. If you handle it alone, it will take a lot of effort and time."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Hanson",
                                args!["But if you cooperate with others, it will be simple and an enjoyable task. You would... "],
                            )?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("Handle it myself, even if it's hard.:Ask friends to help.")])? {
                                1 => {
                                    l_magician_p = (l_magician_p.clone() + Val::from(1));
                                }
                                2 => {
                                    l_merchant_p = (l_merchant_p.clone() + Val::from(1));
                                }
                                _ => {}
                            }
                            ctx.lines_as(
                                "Hanson",
                                args!["You happen to", "find a girl who", "fainted on the street.", "What would you do?"],
                            )?;
                            ctx.next()?;
                            match runtime::select_values(
                                ctx,
                                &[Val::from(
                                    "Carry her to a hospital.:Assess the situation before taking action.:Just ignore it.",
                                )],
                            )? {
                                1 => {
                                    l_acolyte_p = (l_acolyte_p.clone() + Val::from(1));
                                }
                                2 => {
                                    l_swordman_p = (l_swordman_p.clone() + Val::from(1));
                                    l_archer_p = (l_archer_p.clone() + Val::from(1));
                                }
                                3 => {
                                    l_magician_p = (l_magician_p.clone() + Val::from(1));
                                    l_thief_p = (l_thief_p.clone() + Val::from(1));
                                    l_merchant_p = (l_merchant_p.clone() + Val::from(1));
                                }
                                _ => {}
                            }
                            ctx.lines_as("Hanson", args!["You happen to", "pick up 'Clothing.'", "What would you do?"])?;
                            ctx.next()?;
                            match runtime::select_values(
                                ctx,
                                &[Val::from(
                                    "Check the brand.:Wonder who lost it.:Finder's keepers!:Leave it where it was.",
                                )],
                            )? {
                                1 => {
                                    l_merchant_p = (l_merchant_p.clone() + Val::from(1));
                                }
                                2 => {
                                    l_acolyte_p = (l_acolyte_p.clone() + Val::from(1));
                                }
                                3 => {
                                    l_merchant_p = (l_merchant_p.clone() + Val::from(1));
                                    l_thief_p = (l_thief_p.clone() + Val::from(1));
                                }
                                4 => {
                                    l_magician_p = (l_magician_p.clone() + Val::from(1));
                                }
                                _ => {}
                            }
                            ctx.lines_as("Hanson", args!["You happened to accidentally slip your tongue in the middle of a conversation. How do you cope with this situation?"])?;
                            ctx.next()?;
                            match runtime::select_values(
                                ctx,
                                &[Val::from(
                                    "Pretend it's a joke.:Change the subject.:Analyze it.:Apologize honestly.",
                                )],
                            )? {
                                1 => {
                                    l_thief_p = (l_thief_p.clone() + Val::from(1));
                                }
                                2 => {
                                    l_swordman_p = (l_swordman_p.clone() + Val::from(1));
                                }
                                3 => {
                                    l_magician_p = (l_magician_p.clone() + Val::from(1));
                                }
                                4 => {
                                    l_acolyte_p = (l_acolyte_p.clone() + Val::from(1));
                                }
                                _ => {}
                            }
                            ctx.lines_as("Hanson", args!["You're on a trip with your beloved. Your significant other then asks you to buy a souvenir that's not particularly good. What do you do?"])?;
                            ctx.next()?;
                            match runtime::select_values(
                                ctx,
                                &[Val::from("Buy the item for her/him.:Say 'no.':Promise it for next time.")],
                            )? {
                                1 => {
                                    l_swordman_p = (l_swordman_p.clone() + Val::from(1));
                                }
                                2 => {
                                    l_merchant_p = (l_merchant_p.clone() + Val::from(1));
                                }
                                3 => {
                                    l_thief_p = (l_thief_p.clone() + Val::from(1));
                                }
                                _ => {}
                            }
                            ctx.lines_as(
                                "Hanson",
                                args!["Okay~! That's all for the test. You've finished all the Training Grounds courses. Congratulations!"],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Hanson", args!["I've prepared some items for you since you passed the personality test. Please take them, you've earned it."])?;
                            ctx.next()?;
                            ctx.var("nov_3_swordman").set(Val::from(40))?;
                            ctx.call(Function::GetItem, vec![Val::from(501), Val::from(4)])?;
                            ctx.call(Function::GetItem, vec![Val::from(503), Val::from(2)])?;
                            ctx.call(Function::GetItem, vec![Val::from(506), Val::from(2)])?;
                            ctx.next()?;
                            ctx.lines_as("Hanson", args!["Now, we will recommend a suitable job for you after analyzing the results of your personality test. Please wait a moment."])?;
                            ctx.next()?;
                            ctx.lines_as("Hanson", args!["..."])?;
                            ctx.next()?;
                            ctx.lines_as("Hanson", args!["...", "......"])?;
                            ctx.next()?;
                            ctx.lines_as("Hanson", args!["Here's the", "final result", "of your test."])?;
                            ctx.next()?;
                            if runtime::op(&l_swordman_p.clone(), ">", &l_magician_p.clone())?.is_true() {
                                if runtime::op(&l_swordman_p.clone(), ">", &l_merchant_p.clone())?.is_true() {
                                    if runtime::op(&l_swordman_p.clone(), ">", &l_thief_p.clone())?.is_true() {
                                        if runtime::op(&l_swordman_p.clone(), ">", &l_archer_p.clone())?.is_true() {
                                            if runtime::op(&l_swordman_p.clone(), ">", &l_acolyte_p.clone())?.is_true() {
                                                l_job_c = Val::from(1);
                                            } else {
                                                l_job_c = Val::from(6);
                                            }
                                        } else {
                                            if runtime::op(&l_archer_p.clone(), ">", &l_acolyte_p.clone())?.is_true() {
                                                l_job_c = Val::from(5);
                                            } else {
                                                l_job_c = Val::from(6);
                                            }
                                        }
                                    } else {
                                        if runtime::op(&l_thief_p.clone(), ">", &l_archer_p.clone())?.is_true() {
                                            if runtime::op(&l_thief_p.clone(), ">", &l_acolyte_p.clone())?.is_true() {
                                                l_job_c = Val::from(4);
                                            } else {
                                                l_job_c = Val::from(6);
                                            }
                                        } else {
                                            if runtime::op(&l_archer_p.clone(), ">", &l_acolyte_p.clone())?.is_true() {
                                                l_job_c = Val::from(5);
                                            } else {
                                                l_job_c = Val::from(6);
                                            }
                                        }
                                    }
                                } else {
                                    if runtime::op(&l_merchant_p.clone(), ">", &l_thief_p.clone())?.is_true() {
                                        if runtime::op(&l_merchant_p.clone(), ">", &l_archer_p.clone())?.is_true() {
                                            if runtime::op(&l_merchant_p.clone(), ">", &l_acolyte_p.clone())?.is_true() {
                                                l_job_c = Val::from(3);
                                            } else {
                                                l_job_c = Val::from(6);
                                            }
                                        } else {
                                            if runtime::op(&l_archer_p.clone(), ">", &l_acolyte_p.clone())?.is_true() {
                                                l_job_c = Val::from(5);
                                            } else {
                                                l_job_c = Val::from(6);
                                            }
                                        }
                                    } else {
                                        if runtime::op(&l_thief_p.clone(), ">", &l_archer_p.clone())?.is_true() {
                                            if runtime::op(&l_thief_p.clone(), ">", &l_acolyte_p.clone())?.is_true() {
                                                l_job_c = Val::from(4);
                                            } else {
                                                l_job_c = Val::from(6);
                                            }
                                        } else {
                                            if runtime::op(&l_archer_p.clone(), ">", &l_acolyte_p.clone())?.is_true() {
                                                l_job_c = Val::from(5);
                                            } else {
                                                l_job_c = Val::from(6);
                                            }
                                        }
                                    }
                                }
                            } else {
                                if runtime::op(&l_magician_p.clone(), ">", &l_merchant_p.clone())?.is_true() {
                                    if runtime::op(&l_magician_p.clone(), ">", &l_thief_p.clone())?.is_true() {
                                        if runtime::op(&l_magician_p.clone(), ">", &l_archer_p.clone())?.is_true() {
                                            if runtime::op(&l_magician_p.clone(), ">", &l_acolyte_p.clone())?.is_true() {
                                                l_job_c = Val::from(2);
                                            } else {
                                                l_job_c = Val::from(6);
                                            }
                                        } else {
                                            if runtime::op(&l_archer_p.clone(), ">", &l_acolyte_p.clone())?.is_true() {
                                                l_job_c = Val::from(5);
                                            } else {
                                                l_job_c = Val::from(6);
                                            }
                                        }
                                    } else {
                                        if runtime::op(&l_thief_p.clone(), ">", &l_archer_p.clone())?.is_true() {
                                            if runtime::op(&l_thief_p.clone(), ">", &l_acolyte_p.clone())?.is_true() {
                                                l_job_c = Val::from(4);
                                            } else {
                                                l_job_c = Val::from(6);
                                            }
                                        } else {
                                            if runtime::op(&l_archer_p.clone(), ">", &l_acolyte_p.clone())?.is_true() {
                                                l_job_c = Val::from(5);
                                            } else {
                                                l_job_c = Val::from(6);
                                            }
                                        }
                                    }
                                } else {
                                    if runtime::op(&l_merchant_p.clone(), ">", &l_thief_p.clone())?.is_true() {
                                        if runtime::op(&l_merchant_p.clone(), ">", &l_archer_p.clone())?.is_true() {
                                            if runtime::op(&l_merchant_p.clone(), ">", &l_acolyte_p.clone())?.is_true() {
                                                l_job_c = Val::from(3);
                                            } else {
                                                l_job_c = Val::from(6);
                                            }
                                        } else {
                                            if runtime::op(&l_archer_p.clone(), ">", &l_acolyte_p.clone())?.is_true() {
                                                l_job_c = Val::from(5);
                                            } else {
                                                l_job_c = Val::from(6);
                                            }
                                        }
                                    } else {
                                        if runtime::op(&l_thief_p.clone(), ">", &l_archer_p.clone())?.is_true() {
                                            if runtime::op(&l_thief_p.clone(), ">", &l_acolyte_p.clone())?.is_true() {
                                                l_job_c = Val::from(4);
                                            } else {
                                                l_job_c = Val::from(6);
                                            }
                                        } else {
                                            if runtime::op(&l_archer_p.clone(), ">", &l_acolyte_p.clone())?.is_true() {
                                                l_job_c = Val::from(5);
                                            } else {
                                                l_job_c = Val::from(6);
                                            }
                                        }
                                    }
                                }
                            }
                            if l_job_c.clone() == 1 {
                                ctx.lines_as(
                                    "Hanson",
                                    args![
                                        "Although you're very straight forward and simple minded, you",
                                        "have a strong will and want to be an important person for this world."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Hanson", args!["You're also always", "trying to protect the weak."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Hanson",
                                    args!["For you, who has your own will, ^696969Swordman^000000 class is the most suitable job."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Hanson",
                                    args!["So would you like to accept our recommendation, or would you like to choose a job on your own?"],
                                )?;
                                ctx.next()?;
                                match runtime::select_values(ctx, &[Val::from("Swordman!:My own choice!")])? {
                                    1 => {
                                        ctx.lines_as(
                                            "Hanson",
                                            args![
                                                "That's a great choice!",
                                                "After you receive all the supplies, I will teleport you to the Swordman Association."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines(args![
                                            "^660000List of Supplies^000000",
                                            "^0000335 Free Ticket for Kafra Storage^000000",
                                            "^0000335 Free Ticket for Kafra Transportation^000000",
                                            "^0000331 Falchion^000000",
                                            "^0000337 Phracon^000000"
                                        ])?;
                                        ctx.next()?;
                                        ctx.var("nov_3_swordman").set(Val::from(40))?;
                                        ctx.call(Function::GetItem, vec![Val::from(7059), Val::from(5)])?;
                                        ctx.call(Function::GetItem, vec![Val::from(7060), Val::from(5)])?;
                                        ctx.call(Function::GetItem, vec![Val::from(1104), Val::from(1)])?;
                                        ctx.call(Function::GetItem, vec![Val::from(1010), Val::from(7)])?;
                                        ctx.lines_as("Hanson", args!["Please check your inventory to see if you have received all the supplies listed. Let me briefly inform you about the items you've received."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Hanson", args!["Free tickets for Kafra storage and transportation can be used for Kafra storage and teleport services."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Hanson", args!["'Zeny' is the currency of Midgard. 'Falchion' is a weapon that will be very useful once you become a Swordman."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Hanson", args!["'Phracon' is an ore which can be used to upgrade lvl 1 weapons. To strengthen your Falchion with this Phracon, please visit a forge in one of the towns."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Hanson", args!["The town you will be sent to is called Izlude which is a satellite of Prontera. The Swordman Association is located in the West of town. Please remember this."])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Hanson",
                                            args![
                                                "You will now",
                                                "be teleported.",
                                                "Good luck,",
                                                ((Val::from("^A62A2A") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                    + Val::from("^000000")),
                                                "and farewell."
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        ctx.var("nov_1st_cos").set(Val::from(0))?;
                                        ctx.var("nov_2nd_cos").set(Val::from(0))?;
                                        ctx.var("nov_3_swordman").set(Val::from(0))?;
                                        ctx.var("nov_3_archer").set(Val::from(0))?;
                                        ctx.var("nov_3_thief").set(Val::from(0))?;
                                        ctx.var("nov_3_magician").set(Val::from(0))?;
                                        ctx.var("nov_3_acolyte").set(Val::from(0))?;
                                        ctx.var("nov_3_merchant").set(Val::from(0))?;
                                        ctx.call(
                                            Function::SavePoint,
                                            vec![Val::from("izlude"), Val::from(93), Val::from(104), Val::from(1), Val::from(1)],
                                        )?;
                                        ctx.call(Function::Warp, vec![Val::from("izlude_in"), Val::from(74), Val::from(167)])?;
                                        return Err(Stop::End);
                                    }
                                    2 => {
                                        hanson_nv_run(ctx, HansonNvStep::SUserJobchoice, vec![Val::from("Swordsman")])?;
                                        return Err(Stop::End);
                                    }
                                    _ => {}
                                }
                            } else {
                                if l_job_c.clone() == 2 {
                                    ctx.lines_as("Hanson", args!["You enjoy analyzing things around you, and you're very independent. You have use insightful judgment and you can be very shy and logical."])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Hanson",
                                        args!["For you, the observative intellectual, ^696969Mage^000000 is the most suitable job."],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Hanson",
                                        args![
                                            "So, would you like to accept our recommendation or would you like to choose a job on your own?"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    match runtime::select_values(ctx, &[Val::from("Mage!:My own choice!")])? {
                                        1 => {
                                            ctx.lines_as(
                                                "Hanson",
                                                args![
                                                    "That's a great choice!",
                                                    "After you receive all the supplies, I'll teleport you to the Mage town."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines(args![
                                                "^660000List of Supplies^000000",
                                                "^0000335 Free Ticket for Kafra Storage^000000",
                                                "^0000335 Free Ticket for Kafra Transportation^000000",
                                                "^0000331 Rod^000000",
                                                "^0000331 Cutter^000000",
                                                "^0000337 Phracon^000000"
                                            ])?;
                                            ctx.next()?;
                                            ctx.var("nov_3_swordman").set(Val::from(40))?;
                                            ctx.call(Function::GetItem, vec![Val::from(7059), Val::from(5)])?;
                                            ctx.call(Function::GetItem, vec![Val::from(7060), Val::from(5)])?;
                                            ctx.call(Function::GetItem, vec![Val::from(1601), Val::from(1)])?;
                                            ctx.call(Function::GetItem, vec![Val::from(1204), Val::from(1)])?;
                                            ctx.call(Function::GetItem, vec![Val::from(1010), Val::from(7)])?;
                                            ctx.lines_as("Hanson", args!["Please check your inventory to see if you have received all the supplies listed. Let me briefly inform you about the items you've received."])?;
                                            ctx.next()?;
                                            ctx.lines_as("Hanson", args!["Free tickets for Kafra storage and transportation can be used for Kafra storage and teleport services."])?;
                                            ctx.next()?;
                                            ctx.lines_as("Hanson", args!["'Zeny' is the currency of Midgard. That 'Cutter' has been given to you so that you can fight monsters before you become a Mage."])?;
                                            ctx.next()?;
                                            ctx.lines_as("Hanson", args!["Once you become a Mage, you can use the 'Rod' that has been given to you. It will be very useful during your early days as a Mage."])?;
                                            ctx.next()?;
                                            ctx.lines_as("Hanson", args!["'Phracon' is an ore which can be used to upgrade lvl 1 weapons. To strengthen your Level 1 weapons with this Phracon, please visit a forge in one of the towns."])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Hanson",
                                                args![
                                                    "The town you will arrive is named 'Geffen'.",
                                                    "The mage academy is located in the Northwest part in town. Please remember this."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Hanson",
                                                args![
                                                    "You'll now be teleported.",
                                                    ((Val::from("Good luck, ^A62A2A")
                                                        + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                        + Val::from("^000000 and farewell."))
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            ctx.var("nov_1st_cos").set(Val::from(0))?;
                                            ctx.var("nov_2nd_cos").set(Val::from(0))?;
                                            ctx.var("nov_3_swordman").set(Val::from(0))?;
                                            ctx.var("nov_3_archer").set(Val::from(0))?;
                                            ctx.var("nov_3_thief").set(Val::from(0))?;
                                            ctx.var("nov_3_magician").set(Val::from(0))?;
                                            ctx.var("nov_3_acolyte").set(Val::from(0))?;
                                            ctx.var("nov_3_merchant").set(Val::from(0))?;
                                            ctx.call(
                                                Function::SavePoint,
                                                vec![Val::from("geffen"), Val::from(119), Val::from(37), Val::from(1), Val::from(1)],
                                            )?;
                                            ctx.call(Function::Warp, vec![Val::from("geffen_in"), Val::from(163), Val::from(98)])?;
                                            return Err(Stop::End);
                                        }
                                        2 => {
                                            hanson_nv_run(ctx, HansonNvStep::SUserJobchoice, vec![Val::from("Mage")])?;
                                            return Err(Stop::End);
                                        }
                                        _ => {}
                                    }
                                } else if l_job_c.clone() == 3 {
                                    ctx.lines_as("Hanson", args!["You're very willful and very well organized. You've already set a goal in life and have become very responsible for your actions."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Hanson", args!["Because of your drive and desire to succeed, ^696969Merchant^000000 is the most suitable job for you."])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Hanson",
                                        args![
                                            "So, would you like to accept our recommendation or would you like to choose a job on your own?"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    match runtime::select_values(ctx, &[Val::from("Merchant!:My own choice!")])? {
                                        1 => {
                                            ctx.lines_as(
                                                "Hanson",
                                                args![
                                                    "That's a great choice!",
                                                    "After you receive all the supplies, I will teleport you to the merchant town."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines(args![
                                                "^660000List of Supplies^000000",
                                                "^0000334 Free Ticket for Kafra Storage^000000",
                                                "^0000334 Free Ticket for Kafra Transportation^000000",
                                                "^0000334 Free Ticket for the Cart Service^000000",
                                                "^0000331 Battle Axe^000000",
                                                "^0000337 Phracon^000000"
                                            ])?;
                                            ctx.next()?;
                                            ctx.var("nov_3_swordman").set(Val::from(40))?;
                                            ctx.call(Function::GetItem, vec![Val::from(7059), Val::from(4)])?;
                                            ctx.call(Function::GetItem, vec![Val::from(7060), Val::from(4)])?;
                                            ctx.call(Function::GetItem, vec![Val::from(7061), Val::from(4)])?;
                                            ctx.call(Function::GetItem, vec![Val::from(1351), Val::from(1)])?;
                                            ctx.call(Function::GetItem, vec![Val::from(1010), Val::from(7)])?;
                                            ctx.lines_as("Hanson", args!["Please check your inventory to see if you have received all the supplies listed. Let me briefly inform you about the items you've received."])?;
                                            ctx.next()?;
                                            ctx.lines_as("Hanson", args!["Free tickets for Kafra storage and transportation can be used for Kafra storage and teleport services."])?;
                                            ctx.next()?;
                                            ctx.lines_as("Hanson", args!["'Zeny' is the currency of Midgard. 'Battle Axe' will come in handy once you become a Merchant."])?;
                                            ctx.next()?;
                                            ctx.lines_as("Hanson", args!["'Phracon' is an ore which can be used to upgrade lvl 1 weapons. To strengthen your Battle Axe with this Phracon, please visit a forge in one of the towns."])?;
                                            ctx.next()?;
                                            ctx.lines_as("Hanson", args!["The town you will be sent to is named Alberta. The Merchant Guild is located to the SouthWest within Alberta. Please remember this."])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Hanson",
                                                args![
                                                    "You will now",
                                                    "be teleported.",
                                                    "Good luck,",
                                                    ((Val::from("^A62A2A") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                        + Val::from("^000000")),
                                                    "and farewell."
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            ctx.var("nov_1st_cos").set(Val::from(0))?;
                                            ctx.var("nov_2nd_cos").set(Val::from(0))?;
                                            ctx.var("nov_3_swordman").set(Val::from(0))?;
                                            ctx.var("nov_3_archer").set(Val::from(0))?;
                                            ctx.var("nov_3_thief").set(Val::from(0))?;
                                            ctx.var("nov_3_magician").set(Val::from(0))?;
                                            ctx.var("nov_3_acolyte").set(Val::from(0))?;
                                            ctx.var("nov_3_merchant").set(Val::from(0))?;
                                            ctx.call(
                                                Function::SavePoint,
                                                vec![Val::from("alberta"), Val::from(30), Val::from(232), Val::from(1), Val::from(1)],
                                            )?;
                                            ctx.call(Function::Warp, vec![Val::from("alberta_in"), Val::from(62), Val::from(44)])?;
                                            return Err(Stop::End);
                                        }
                                        2 => {
                                            hanson_nv_run(ctx, HansonNvStep::SUserJobchoice, vec![Val::from("Merchant")])?;
                                            return Err(Stop::End);
                                        }
                                        _ => {}
                                    }
                                } else if l_job_c.clone() == 4 {
                                    ctx.lines_as("Hanson", args!["Carpe diem:", "Seize the day.", "That's how you live."])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Hanson",
                                        args![
                                            "From your natural curiosity",
                                            "comes a happy-go-lucky sense of adventure, and a desire to explore."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Hanson",
                                        args!["For someone like you,", "^696969Thief^000000 is the most suitable job.'"],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Hanson",
                                        args![
                                            "So, would you like to accept our recommendation or would you like to choose a job on your own?"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    match runtime::select_values(ctx, &[Val::from("Thief!:My own choice!")])? {
                                        1 => {
                                            ctx.lines_as(
                                                "Hanson",
                                                args![
                                                    "That's a great choice!",
                                                    "After you receive all the supplies, I'll teleport you to the Thief town."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines(args![
                                                "^660000List of Supplies^000000",
                                                "^0000335 Free Ticket for Kafra Storage^000000",
                                                "^0000335 Free Ticket for Kafra Transportation^000000",
                                                "^0000331 Main Gauche^000000",
                                                "^0000337 Phracon^000000"
                                            ])?;
                                            ctx.next()?;
                                            ctx.var("nov_3_swordman").set(Val::from(40))?;
                                            ctx.call(Function::GetItem, vec![Val::from(7059), Val::from(5)])?;
                                            ctx.call(Function::GetItem, vec![Val::from(7060), Val::from(5)])?;
                                            ctx.call(Function::GetItem, vec![Val::from(1207), Val::from(1)])?;
                                            ctx.call(Function::GetItem, vec![Val::from(1010), Val::from(7)])?;
                                            ctx.lines_as("Hanson", args!["Please check your inventory to see if you have received all the supplies listed. Let me briefly inform you about the items you've received."])?;
                                            ctx.next()?;
                                            ctx.lines_as("Hanson", args!["Free tickets for Kafra storage and transportation can be used for Kafra storage and teleport services."])?;
                                            ctx.next()?;
                                            ctx.lines_as("Hanson", args!["'Zeny' is the currency of Midgard. 'Main Gauche' is a weapon that will be very useful once you become a Thief."])?;
                                            ctx.next()?;
                                            ctx.lines_as("Hanson", args!["'Phracon' is an ore which can be used to upgrade lvl 1 weapons. To strengthen your Main Gauche with this Phracon, please visit a forge in one of the towns."])?;
                                            ctx.next()?;
                                            ctx.lines_as("Hanson", args!["The town you will be sent to is named Morocc. The Thief Guild is in the first underground floor of the pyramid NorthWest of Morocc. Remember this."])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Hanson",
                                                args![
                                                    "You will now",
                                                    "be teleported.",
                                                    "Good luck,",
                                                    ((Val::from("^A62A2A") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                        + Val::from("^000000")),
                                                    "and farewell."
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            ctx.call(
                                                Function::SavePoint,
                                                vec![Val::from("morocc"), Val::from(150), Val::from(99), Val::from(1), Val::from(1)],
                                            )?;
                                            ctx.call(Function::Warp, vec![Val::from("moc_ruins"), Val::from(155), Val::from(44)])?;
                                            return Err(Stop::End);
                                        }
                                        2 => {
                                            hanson_nv_run(ctx, HansonNvStep::SUserJobchoice, vec![Val::from("Thief")])?;
                                            return Err(Stop::End);
                                        }
                                        _ => {}
                                    }
                                } else if l_job_c.clone() == 5 {
                                    ctx.lines_as("Hanson", args!["You always try to understand other people, even though they are strange. You expect others to try to understand you."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Hanson", args!["You refuse to be a ordinary person as you persue your dream. As a person sensitive to nature, ^696969Archer^000000 is the most suitable job for you."])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Hanson",
                                        args![
                                            "So, would you like to accept our recommendation or would you like to choose a job on your own?"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    match runtime::select_values(ctx, &[Val::from("Archer!:My own choice!")])? {
                                        1 => {
                                            ctx.lines_as(
                                                "Hanson",
                                                args![
                                                    "That's a great choice!",
                                                    "After you receive all the supplies, I'll teleport you to the Archer town."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines(args![
                                                "^660000List of Supplies^000000",
                                                "^0000335 Free Ticket for Kafra Storage^000000",
                                                "^0000335 Free Ticket for Kafra Transportation^000000",
                                                "^0000331 Composite Bow^000000",
                                                "^0000337 Phracon^000000"
                                            ])?;
                                            ctx.next()?;
                                            ctx.var("nov_3_swordman").set(Val::from(40))?;
                                            ctx.call(Function::GetItem, vec![Val::from(7059), Val::from(5)])?;
                                            ctx.call(Function::GetItem, vec![Val::from(7060), Val::from(5)])?;
                                            ctx.call(Function::GetItem, vec![Val::from(1704), Val::from(1)])?;
                                            ctx.call(Function::GetItem, vec![Val::from(1010), Val::from(7)])?;
                                            ctx.lines_as("Hanson", args!["Please check your inventory to see if you have received all the supplies listed. Let me briefly inform you about the items you've received."])?;
                                            ctx.next()?;
                                            ctx.lines_as("Hanson", args!["Free tickets for Kafra storage and transportation can be used for Kafra storage and teleport services."])?;
                                            ctx.next()?;
                                            ctx.lines_as("Hanson", args!["'Zeny' is the currency of Midgard. 'Composite Bow' is a weapon that will be very useful once you become an Archer."])?;
                                            ctx.next()?;
                                            ctx.lines_as("Hanson", args!["'Phracon' is an ore which can be used to upgrade lvl 1 weapons. To strengthen your Composite Bow with this Phracon, please visit a forge in one of the towns."])?;
                                            ctx.next()?;
                                            ctx.lines_as("Hanson", args!["The town you will be sent to is named Payon. The Archer Guild is located to the NorthWest in town. Please remember this."])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Hanson",
                                                args![
                                                    "You will now",
                                                    "be teleported.",
                                                    "Good luck,",
                                                    ((Val::from("^A62A2A") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                        + Val::from("^000000")),
                                                    "and farewell."
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            ctx.var("nov_1st_cos").set(Val::from(0))?;
                                            ctx.var("nov_2nd_cos").set(Val::from(0))?;
                                            ctx.var("nov_3_swordman").set(Val::from(0))?;
                                            ctx.var("nov_3_archer").set(Val::from(0))?;
                                            ctx.var("nov_3_thief").set(Val::from(0))?;
                                            ctx.var("nov_3_magician").set(Val::from(0))?;
                                            ctx.var("nov_3_acolyte").set(Val::from(0))?;
                                            ctx.var("nov_3_merchant").set(Val::from(0))?;
                                            ctx.call(
                                                Function::SavePoint,
                                                vec![Val::from("payon"), Val::from(70), Val::from(100), Val::from(1), Val::from(1)],
                                            )?;
                                            ctx.call(Function::Warp, vec![Val::from("payon_in02"), Val::from(64), Val::from(65)])?;
                                            return Err(Stop::End);
                                        }
                                        2 => {
                                            hanson_nv_run(ctx, HansonNvStep::SUserJobchoice, vec![Val::from("Archer")])?;
                                            return Err(Stop::End);
                                        }
                                        _ => {}
                                    }
                                } else if l_job_c.clone() == 6 {
                                    ctx.lines_as("Hanson", args!["You are very warm hearted and considerate, and you're willing to sacrifice your well being for the sake of others."])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Hanson",
                                        args!["You're always eager to help others, which is why you're so well liked."],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Hanson",
                                        args!["For you who are kind of heart, ^696969Acolyte^000000 is the most suitable job."],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Hanson",
                                        args![
                                            "So, would you like to accept our recommendation or would you like to choose a job on your own?"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    match runtime::select_values(ctx, &[Val::from("Acolyte!:My own choice!")])? {
                                        1 => {
                                            ctx.lines_as(
                                                "Hanson",
                                                args![
                                                    "That's a great choice!",
                                                    "After you receive all the supplies, I'll teleport you behind the Sanctuary."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines(args![
                                                "^660000List of Supplies^000000",
                                                "^0000335 Free Ticket for Kafra Storage^000000",
                                                "^0000335 Free Ticket for Kafra Transportation^000000",
                                                "^0000331 Mace^000000",
                                                "^0000337 Phracon^000000"
                                            ])?;
                                            ctx.next()?;
                                            ctx.var("nov_3_swordman").set(Val::from(40))?;
                                            ctx.call(Function::GetItem, vec![Val::from(7059), Val::from(5)])?;
                                            ctx.call(Function::GetItem, vec![Val::from(7060), Val::from(5)])?;
                                            ctx.call(Function::GetItem, vec![Val::from(1504), Val::from(1)])?;
                                            ctx.call(Function::GetItem, vec![Val::from(1010), Val::from(7)])?;
                                            ctx.lines_as("Hanson", args!["Please check your inventory to see if you have received all the supplies listed. Let me briefly inform you about the items you've received."])?;
                                            ctx.next()?;
                                            ctx.lines_as("Hanson", args!["Free tickets for Kafra storage and transportation can be used for Kafra storage and teleport services."])?;
                                            ctx.next()?;
                                            ctx.lines_as("Hanson", args!["'Zeny' is the currency of Midgard. 'Mace' is a weapon that will be very useful once you become an Acolyte."])?;
                                            ctx.next()?;
                                            ctx.lines_as("Hanson", args!["'Phracon' is an ore which can be used to upgrade lvl 1 weapons. To strengthen your Mace with this Phracon, please visit a forge in one of the towns."])?;
                                            ctx.next()?;
                                            ctx.lines_as("Hanson", args!["You have chosen to be an Acolyte. The town you will be sent to is named Prontera. The Sanctuary is NorthEast in Prontera. Please remember this."])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Hanson",
                                                args![
                                                    "You will now",
                                                    "be teleported.",
                                                    "Good luck,",
                                                    ((Val::from("^A62A2A") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                        + Val::from("^000000")),
                                                    "and farewell."
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            ctx.var("nov_1st_cos").set(Val::from(0))?;
                                            ctx.var("nov_2nd_cos").set(Val::from(0))?;
                                            ctx.var("nov_3_swordman").set(Val::from(0))?;
                                            ctx.var("nov_3_archer").set(Val::from(0))?;
                                            ctx.var("nov_3_thief").set(Val::from(0))?;
                                            ctx.var("nov_3_magician").set(Val::from(0))?;
                                            ctx.var("nov_3_acolyte").set(Val::from(0))?;
                                            ctx.var("nov_3_merchant").set(Val::from(0))?;
                                            ctx.call(
                                                Function::SavePoint,
                                                vec![Val::from("prontera"), Val::from(117), Val::from(72), Val::from(1), Val::from(1)],
                                            )?;
                                            ctx.call(Function::Warp, vec![Val::from("prt_church"), Val::from(172), Val::from(19)])?;
                                            return Err(Stop::End);
                                        }
                                        2 => {
                                            hanson_nv_run(ctx, HansonNvStep::SUserJobchoice, vec![Val::from("Acolyte")])?;
                                            return Err(Stop::End);
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                        if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                            matched1 = true;
                        }
                        if matched1 {
                            ctx.lines_as(
                                "Hanson",
                                args![
                                    "I understand.",
                                    "Let me transport",
                                    "you to the world of",
                                    "Ragnarok Online",
                                    "immediately."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Hanson",
                                args!["For more information and knowledge, I hope you will obtain your own experiences in Midgard."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Hanson",
                                args![
                                    "Lastly...",
                                    "I hope you will",
                                    "become a nice player,",
                                    ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(".")),
                                    "Fare well."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.var("nov_3_swordman").set(Val::from(40))?;
                            ctx.var("nov_1st_cos").set(Val::from(0))?;
                            ctx.var("nov_2nd_cos").set(Val::from(0))?;
                            ctx.var("nov_3_swordman").set(Val::from(0))?;
                            ctx.var("nov_3_archer").set(Val::from(0))?;
                            ctx.var("nov_3_thief").set(Val::from(0))?;
                            ctx.var("nov_3_magician").set(Val::from(0))?;
                            ctx.var("nov_3_acolyte").set(Val::from(0))?;
                            ctx.var("nov_3_merchant").set(Val::from(0))?;
                            l_startmap = ctx.call(Function::Rand, vec![Val::from(1), Val::from(6)])?;
                            if (l_startmap.clone().number()? > 0 && l_startmap.clone().number()? < 2) {
                                ctx.call(
                                    Function::SavePoint,
                                    vec![Val::from("prontera"), Val::from(117), Val::from(72), Val::from(1), Val::from(1)],
                                )?;
                                ctx.call(Function::Warp, vec![Val::from("prt_fild08"), Val::from(170), Val::from(371)])?;
                            } else {
                                if (l_startmap.clone().number()? > 1 && l_startmap.clone().number()? < 3) {
                                    ctx.call(
                                        Function::SavePoint,
                                        vec![Val::from("geffen"), Val::from(119), Val::from(37), Val::from(1), Val::from(1)],
                                    )?;
                                    ctx.call(Function::Warp, vec![Val::from("gef_fild07"), Val::from(327), Val::from(188)])?;
                                } else if (l_startmap.clone().number()? > 2 && l_startmap.clone().number()? < 4) {
                                    ctx.call(
                                        Function::SavePoint,
                                        vec![Val::from("alberta"), Val::from(30), Val::from(232), Val::from(1), Val::from(1)],
                                    )?;
                                    ctx.call(Function::Warp, vec![Val::from("pay_fild03"), Val::from(388), Val::from(70)])?;
                                } else if (l_startmap.clone().number()? > 3 && l_startmap.clone().number()? < 5) {
                                    ctx.call(
                                        Function::SavePoint,
                                        vec![Val::from("morocc"), Val::from(150), Val::from(99), Val::from(1), Val::from(1)],
                                    )?;
                                    ctx.call(Function::Warp, vec![Val::from("moc_fild07"), Val::from(198), Val::from(39)])?;
                                } else if (l_startmap.clone().number()? > 4 && l_startmap.clone().number()? < 6) {
                                    ctx.call(
                                        Function::SavePoint,
                                        vec![Val::from("payon"), Val::from(256), Val::from(242), Val::from(1), Val::from(1)],
                                    )?;
                                    ctx.call(Function::Warp, vec![Val::from("pay_fild01"), Val::from(334), Val::from(354)])?;
                                } else if (l_startmap.clone().number()? > 5 && l_startmap.clone().number()? < 7) {
                                    ctx.call(
                                        Function::SavePoint,
                                        vec![Val::from("izlude"), Val::from(93), Val::from(104), Val::from(1), Val::from(1)],
                                    )?;
                                    ctx.call(Function::Warp, vec![Val::from("prt_fild08"), Val::from(357), Val::from(212)])?;
                                }
                            }
                            return Err(Stop::End);
                        }
                    }
                } else {
                    if ctx.var("nov_3_swordman").get()? == 40 {
                        ctx.lines_as("Hanson", args!["Hmmm...?", "Why are you", "still here?"])?;
                        ctx.next()?;
                        ctx.lines_as("Hanson", args!["You didn't say anything, so", "I assumed you were already gone. Since you have already finished the final test and I gave you all the supplies..."])?;
                        ctx.next()?;
                        ctx.lines_as("Hanson", args!["The only thing", "left to do is to lead", "you to Midgard~"])?;
                        ctx.next()?;
                        ctx.var("nov_1st_cos").set(Val::from(0))?;
                        ctx.var("nov_2nd_cos").set(Val::from(0))?;
                        ctx.var("nov_3_swordman").set(Val::from(0))?;
                        ctx.var("nov_3_archer").set(Val::from(0))?;
                        ctx.var("nov_3_thief").set(Val::from(0))?;
                        ctx.var("nov_3_magician").set(Val::from(0))?;
                        ctx.var("nov_3_acolyte").set(Val::from(0))?;
                        ctx.var("nov_3_merchant").set(Val::from(0))?;
                        l_startmap = ctx.call(Function::Rand, vec![Val::from(1), Val::from(6)])?;
                        if (l_startmap.clone().number()? > 0 && l_startmap.clone().number()? < 2) {
                            ctx.call(
                                Function::SavePoint,
                                vec![Val::from("prontera"), Val::from(117), Val::from(72), Val::from(1), Val::from(1)],
                            )?;
                            ctx.call(Function::Warp, vec![Val::from("prt_fild08"), Val::from(170), Val::from(371)])?;
                        } else {
                            if (l_startmap.clone().number()? > 1 && l_startmap.clone().number()? < 3) {
                                ctx.call(
                                    Function::SavePoint,
                                    vec![Val::from("geffen"), Val::from(119), Val::from(37), Val::from(1), Val::from(1)],
                                )?;
                                ctx.call(Function::Warp, vec![Val::from("gef_fild07"), Val::from(327), Val::from(188)])?;
                            } else if (l_startmap.clone().number()? > 2 && l_startmap.clone().number()? < 4) {
                                ctx.call(
                                    Function::SavePoint,
                                    vec![Val::from("alberta"), Val::from(30), Val::from(232), Val::from(1), Val::from(1)],
                                )?;
                                ctx.call(Function::Warp, vec![Val::from("pay_fild03"), Val::from(388), Val::from(70)])?;
                            } else if (l_startmap.clone().number()? > 3 && l_startmap.clone().number()? < 5) {
                                ctx.call(
                                    Function::SavePoint,
                                    vec![Val::from("morocc"), Val::from(150), Val::from(99), Val::from(1), Val::from(1)],
                                )?;
                                ctx.call(Function::Warp, vec![Val::from("moc_fild07"), Val::from(198), Val::from(39)])?;
                            } else if (l_startmap.clone().number()? > 4 && l_startmap.clone().number()? < 6) {
                                ctx.call(
                                    Function::SavePoint,
                                    vec![Val::from("payon"), Val::from(70), Val::from(100), Val::from(1), Val::from(1)],
                                )?;
                                ctx.call(Function::Warp, vec![Val::from("pay_fild01"), Val::from(334), Val::from(354)])?;
                            } else if (l_startmap.clone().number()? > 5 && l_startmap.clone().number()? < 7) {
                                ctx.call(
                                    Function::SavePoint,
                                    vec![Val::from("izlude"), Val::from(93), Val::from(104), Val::from(1), Val::from(1)],
                                )?;
                                ctx.call(Function::Warp, vec![Val::from("prt_fild08"), Val::from(357), Val::from(212)])?;
                            }
                        }
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Hanson",
                            args![
                                "Hello, you",
                                ((Val::from("must be ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("."))
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Hanson",
                            args!["I am Hanson,", "the person in", "charge of the", "personality test."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Hanson",
                            args![
                                "Please speak",
                                "to Bruce for",
                                "'Class Explanation'",
                                "before we begin your",
                                "test. Thank you."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                step = HansonNvStep::SUserJobchoice;
                continue 'machine;
            }
            HansonNvStep::SUserJobchoice => {
                ctx.lines_as(
                    "Hanson",
                    args![
                        "I see. It's your choice.",
                        "There is no obligation to change to the job we recommend. Please choose the job you wish to become."
                    ],
                )?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("Swordsman:Mage:Merchant:Thief:Archer:Acolyte")])?;
                ctx.var("@menu").set(choice)?;
                let base = Val::from(1).number()?;
                runtime::local_set(&mut l_jobs_s, &Val::from(base + 0), Val::from("Swordsman"), true);
                runtime::local_set(&mut l_jobs_s, &Val::from(base + 1), Val::from("Mage"), true);
                runtime::local_set(&mut l_jobs_s, &Val::from(base + 2), Val::from("Merchant"), true);
                runtime::local_set(&mut l_jobs_s, &Val::from(base + 3), Val::from("Thief"), true);
                runtime::local_set(&mut l_jobs_s, &Val::from(base + 4), Val::from("Archer"), true);
                runtime::local_set(&mut l_jobs_s, &Val::from(base + 5), Val::from("Acolyte"), true);
                ctx.lines_as("Hanson", args!["You have chosen"])?;
                if ctx.var("@menu").get()? == 1 {
                    ctx.lines(args!["to become a Swordsman.", "You will be sent to", "the town of Izlude."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Hanson",
                        args!["The Swordman Association is located to the Northwest in Izlude. Please remember this."],
                    )?;
                } else if ctx.var("@menu").get()? == 2 {
                    ctx.lines(args!["to become a Mage.", "You will be sent to", "the town of Geffen."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Hanson",
                        args!["The Mage Academy is located in the NorthWest in town. Please remember this."],
                    )?;
                } else if ctx.var("@menu").get()? == 3 {
                    ctx.lines(args!["to become a Merchant.", "You will be sent to", "the town of Alberta."])?;
                } else if ctx.var("@menu").get()? == 4 {
                    ctx.lines(args!["to become a Thief.", "You will be sent to", "the town of Morocc."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Hanson",
                        args![
                            "The Thief guild is in the underground 1st floor of a pyramid which is NorthWest of town. Please remember this."
                        ],
                    )?;
                } else if ctx.var("@menu").get()? == 5 {
                    ctx.lines(args!["to become an Archer.", "You will be sent to", "the town of Payon."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Hanson",
                        args!["The Archer Guild is located to the NorthWest in Payon. Please remember this."],
                    )?;
                } else {
                    ctx.lines(args!["to become an Acolyte.", "You will be sent to", "the town of Prontera."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Hanson",
                        args!["The Prontera Sanctuary is located to the NorthEast in Prontera. Please remember this."],
                    )?;
                }
                ctx.next()?;
                ctx.lines_as(
                    "Hanson",
                    args![
                        "Let me give you",
                        "some supplies. Then",
                        "you will transported",
                        "to the chosen town."
                    ],
                )?;
                ctx.next()?;
                ctx.lines(args![
                    "^660000List of Supplies^000000",
                    "^0000335 Free Ticket for Kafra Storage^000000",
                    "^0000335 Free Ticket for Kafra Transportation^000000",
                    "^0000331 Adventurer's Suit^000000"
                ])?;
                ctx.next()?;
                ctx.var("nov_3_swordman").set(Val::from(40))?;
                ctx.call(Function::GetItem, vec![Val::from(7059), Val::from(5)])?;
                ctx.call(Function::GetItem, vec![Val::from(7060), Val::from(5)])?;
                ctx.call(Function::GetItem, vec![Val::from(2305), Val::from(1)])?;
                ctx.lines_as(
                    "Hanson",
                    args![
                        "Please check your inventory",
                        "to see if you have received all the supplies listed. Let me briefly inform you about the items you've received."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Hanson",
                    args!["Free tickets for Kafra storage and transportation can be used for Kafra storage and teleport services."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Hanson",
                    args![
                        ((Val::from("'Zeny' is the currency of Midgard. The 'Adventurer's Suit' will come in handy once you become a ")
                            + runtime::local_get(&l_jobs_s, &ctx.var("@menu").get()?, true))
                            + Val::from("."))
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Hanson",
                    args![
                        "You will now",
                        "be teleported.",
                        "Good luck,",
                        ((Val::from("^A62A2A") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("^000000")),
                        "and farewell."
                    ],
                )?;
                ctx.next()?;
                ctx.var("nov_1st_cos").set(Val::from(0))?;
                ctx.var("nov_2nd_cos").set(Val::from(0))?;
                ctx.var("nov_3_swordman").set(Val::from(0))?;
                ctx.var("nov_3_archer").set(Val::from(0))?;
                ctx.var("nov_3_thief").set(Val::from(0))?;
                ctx.var("nov_3_magician").set(Val::from(0))?;
                ctx.var("nov_3_acolyte").set(Val::from(0))?;
                ctx.var("nov_3_merchant").set(Val::from(0))?;
                if ctx.var("@menu").get()? == 1 {
                    ctx.call(
                        Function::SavePoint,
                        vec![Val::from("izlude"), Val::from(93), Val::from(104), Val::from(1), Val::from(1)],
                    )?;
                    ctx.call(Function::Warp, vec![Val::from("izlude_in"), Val::from(74), Val::from(167)])?;
                } else if ctx.var("@menu").get()? == 2 {
                    ctx.call(
                        Function::SavePoint,
                        vec![Val::from("geffen"), Val::from(119), Val::from(37), Val::from(1), Val::from(1)],
                    )?;
                    ctx.call(Function::Warp, vec![Val::from("geffen_in"), Val::from(163), Val::from(98)])?;
                } else if ctx.var("@menu").get()? == 3 {
                    ctx.call(
                        Function::SavePoint,
                        vec![Val::from("alberta"), Val::from(30), Val::from(232), Val::from(1), Val::from(1)],
                    )?;
                    ctx.call(Function::Warp, vec![Val::from("alberta_in"), Val::from(62), Val::from(44)])?;
                } else if ctx.var("@menu").get()? == 4 {
                    ctx.call(
                        Function::SavePoint,
                        vec![Val::from("morocc"), Val::from(150), Val::from(99), Val::from(1), Val::from(1)],
                    )?;
                    ctx.call(Function::Warp, vec![Val::from("moc_ruins"), Val::from(155), Val::from(44)])?;
                } else if ctx.var("@menu").get()? == 5 {
                    ctx.call(
                        Function::SavePoint,
                        vec![Val::from("payon"), Val::from(70), Val::from(100), Val::from(1), Val::from(1)],
                    )?;
                    ctx.call(Function::Warp, vec![Val::from("payon_in02"), Val::from(64), Val::from(65)])?;
                } else {
                    ctx.call(
                        Function::SavePoint,
                        vec![Val::from("prontera"), Val::from(117), Val::from(72), Val::from(1), Val::from(1)],
                    )?;
                    ctx.call(Function::Warp, vec![Val::from("prt_church"), Val::from(172), Val::from(19)])?;
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn hanson_nv(ctx: &Ctx) -> Script {
    hanson_nv_run(ctx, HansonNvStep::Start, Vec::new()).map(|_| ())
}

fn bruce_nv_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_NOVICE")?) {
        if ctx.var("nov_3_swordman").get()? == 20 {
            ctx.lines_as(
                "Bruce",
                args![
                    "Let me explain the",
                    "First Job Classes",
                    "to you once again.",
                    "Which job did you",
                    "have in mind?"
                ],
            )?;
            ctx.next()?;
            'l1: loop {
                if !(true) {
                    break 'l1;
                }
                'b1: {
                    match runtime::select_values(
                        ctx,
                        &[Val::from("Swordman:Mage:Archer:Merchant:Thief:Acolyte:End conversation.")],
                    )? {
                        1 => {
                            ctx.lines_as("Bruce", args!["As the name implies, the", "Swordman is an expert in wielding Swords. They can also use Spear weapons, but typically you don't see Spear wielding Swordmen very often."])?;
                            ctx.next()?;
                            ctx.lines_as("Bruce", args!["Swordman possess strong physical strength, allowing them to equip heavy armor and weapons. Most weapon classes, except for bows and rods, can be equipped by the Swordman class."])?;
                            ctx.next()?;
                            ctx.lines_as("Bruce", args!["The only weakness of the Swordman class is that they cannot use magic spells. However, this can be compensated by using weapons with an elemental attribute."])?;
                            ctx.next()?;
                            ctx.lines_as("Bruce", args!["One of the greatest benefits of being a Swordman is having an enormous amount of HP, meaning they can more easily withstand damage from their enemies."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Bruce",
                                args!["After learning some strong attack skills, the Swordman is almost unbeatable in a melee fight."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Bruce", args!["In Ragnarok Online, Swordman generally takes the position of tanker, protecting characters of other classes from being attacked or hurt."])?;
                            ctx.next()?;
                            ctx.lines_as("Bruce", args!["A Swordman is the ideal character to take the position of party leader. When advancing to the Second Job Class, Swordmen can change their jobs to ^8E2323Knights^000000 or ^8E2323Crusaders^000000."])?;
                            ctx.var("nov_3_swordman").set(Val::from(20))?;
                            ctx.next()?;
                        }
                        2 => {
                            ctx.lines_as("Bruce", args!["The Mage class specializes in using the forces of Fire, Water, Earth and Lightning to attack their enemies."])?;
                            ctx.next()?;
                            ctx.lines_as("Bruce", args!["However, due to their weak physical strength, they are only allowed to equip Rods and Knives as weapons, and wear light armor for defense."])?;
                            ctx.next()?;
                            ctx.lines_as("Bruce", args!["Despite their physical weakness, they are able to do massive damage with their powerful spells. This fact alone attracts many people to join this class."])?;
                            ctx.next()?;
                            ctx.lines_as("Bruce", args!["In Ragnarok Online, the Mage takes a heavily offensive role in parties and is depended upon to deal great damage to enemies."])?;
                            ctx.next()?;
                            ctx.lines_as("Bruce", args!["When advancing to the Second Job Class, Mages can change their jobs to ^8E2323Wizards^000000 or ^8E2323Sages^000000."])?;
                            ctx.var("nov_3_swordman").set(Val::from(20))?;
                            ctx.next()?;
                        }
                        3 => {
                            ctx.lines_as("Bruce", args!["The Archer class are experts in using Bow weapons, and are useful in parties for their long range attacks."])?;
                            ctx.next()?;
                            ctx.lines_as("Bruce", args!["Despite being physically weaker, Archers possess high accuracy with powerful long range bows. This allows them to attack and kill monsters from a safe distance."])?;
                            ctx.next()?;
                            ctx.lines_as("Bruce", args!["In Ragnarok Online, Archers have relatively little HP, but their long range attacks allow them to easily dispatch enemies before the enemy gets close enough to hurt them."])?;
                            ctx.next()?;
                            ctx.lines_as("Bruce", args!["When advancing to the Second Job Class, every Archer may advance to the ^8E2323Hunter^000000 class. Alternatively, male Archers may advance to become ^8E2323Bards^000000, and female Archers may become ^8E2323Dancers^000000."])?;
                            ctx.var("nov_3_swordman").set(Val::from(20))?;
                            ctx.next()?;
                        }
                        4 => {
                            ctx.lines_as("Bruce", args!["The Merchant class specializes in commerce. Due to the strong influence of the Merchant Guild, the Merchant class is attractive to those who wish to focus on earning Zeny."])?;
                            ctx.next()?;
                            ctx.lines_as("Bruce", args!["In Ragnarok Online, the Merchant class possesses various economic abilities. Merchants can learn to sell items to NPCs for higher prices, as well as receive discounts from NPCs."])?;
                            ctx.next()?;
                            ctx.lines_as("Bruce", args!["In addition, Merchants may rent", "a Cart that greatly expands their carrying capacity and allows them to open shops with their own items and prices."])?;
                            ctx.next()?;
                            ctx.lines_as("Bruce", args!["When advancing to the Second Job Class, Merchants can change their jobs to ^8E2323Blacksmiths^000000 or ^8E2323Alchemists^000000."])?;
                            ctx.var("nov_3_swordman").set(Val::from(20))?;
                            ctx.next()?;
                        }
                        5 => {
                            ctx.lines_as("Bruce", args!["Thieves are experts at using Dagger class weapons. They strike quickly and easily evade attacks from their enemies."])?;
                            ctx.next()?;
                            ctx.lines_as("Bruce", args!["Thieves can learn skills that allow them to hide from their enemies, or steal items from monsters. They are also feared for their use of poison, which slowly weakens", "their enemies."])?;
                            ctx.next()?;
                            ctx.lines_as("Bruce", args!["When advancing to the Second Job Class, Thieves can change their jobs to ^8E2323Assassins^000000 or ^8E2323Rogues^000000."])?;
                            ctx.var("nov_3_swordman").set(Val::from(20))?;
                            ctx.next()?;
                        }
                        6 => {
                            ctx.lines_as("Bruce", args!["In Ragnarok Online, Acolytes act as messengers of God in Rune-Midgarts. They possess skills that support their allies, as well as the life saving Heal ability."])?;
                            ctx.next()?;
                            ctx.lines_as("Bruce", args!["The Acolyte's support abilities make them a welcome addition to any party. In difficult situations, the Acolyte's skills will ensure the survival of the party, allowing other members to focus on offense."])?;
                            ctx.next()?;
                            ctx.lines_as("Bruce", args!["When advancing to the Second Job Class, Acolytes can change their jobs to ^8E2323Priests^000000 or ^8E2323Monks^000000."])?;
                            ctx.var("nov_3_swordman").set(Val::from(20))?;
                            ctx.next()?;
                        }
                        7 => {
                            ctx.lines_as(
                                "Bruce",
                                args![
                                    "For more information,",
                                    "please visit the official",
                                    "Ragnarok Online website:",
                                    " ",
                                    "^0000FFiro.ragnarokonline.com^000000."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Bruce",
                                args!["Hanson is waiting", "for you now. Good luck", "out there, young Novice."],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                }
            }
        } else if ctx.var("nov_3_swordman").get()? == 40 {
            ctx.lines_as("Bruce", args!["I'm sorry, but", "there's nothing", "more I can teach you."])?;
            ctx.next()?;
            ctx.lines_as(
                "Bruce",
                args!["Hanson is waiting", "for you now. Good luck", "out there, young Novice."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Bruce",
                args![
                    "You've gone",
                    "through quite",
                    "a bit of trouble",
                    "to finish all the",
                    "training courses."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Bruce",
                args![
                    "Hello there,",
                    ((Val::from("^A62A2A") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("'^000000,")),
                    "pleased to meet you.",
                    "I am Bruce of the",
                    "Rune-Midgarts Kingdom."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Bruce", args!["My duty is to assist you by teaching information about each First Job Class, so that you can decide which job you want to be."])?;
            ctx.next()?;
            ctx.lines_as(
                "Bruce",
                args![
                    "The First Job Classes are",
                    "^0000FFSwordman, Mage, Archer, Merchant, Thief and Acolyte^000000."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Bruce", args!["So...", "Which job did", "you have in mind?"])?;
            ctx.next()?;
            'l3: loop {
                if !(true) {
                    break 'l3;
                }
                'b3: {
                    match runtime::select_values(
                        ctx,
                        &[Val::from("Swordman:Mage:Archer:Merchant:Thief:Acolyte:End conversation.")],
                    )? {
                        1 => {
                            ctx.lines_as("Bruce", args!["As the name implies, the", "Swordman is an expert in wielding Swords. They can also use Spear weapons, but typically you don't see Spear wielding Swordmen", "very often."])?;
                            ctx.next()?;
                            ctx.lines_as("Bruce", args!["Swordmen possess strong physical strength, allowing them to equip heavy armor and weapons. Most weapon classes, except for bows and rods, can be equipped by the Swordman class."])?;
                            ctx.next()?;
                            ctx.lines_as("Bruce", args!["The only weakness of the Swordman class is that they cannot use magic spells. However, this can be compensated by using weapons with an elemental attribute."])?;
                            ctx.next()?;
                            ctx.lines_as("Bruce", args!["One of the greatest benefits of being a Swordman is having an enormous amount of HP, meaning they can more easily withstand damage from their enemies."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Bruce",
                                args!["After learning some strong attack skills, the Swordman is almost unbeatable in a melee fight."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Bruce", args!["In Ragnarok Online, Swordman generally takes the position of tanker, protecting characters of other classes from being attacked or hurt."])?;
                            ctx.next()?;
                            ctx.lines_as("Bruce", args!["A Swordman is the ideal character to take the position of party leader. When advancing to the Second Job Class, Swordmen can change their jobs to ^8E2323Knights^000000 or ^8E2323Crusaders^000000."])?;
                            ctx.var("nov_3_swordman").set(Val::from(20))?;
                            ctx.next()?;
                        }
                        2 => {
                            ctx.lines_as("Bruce", args!["The Mage class specializes in using the forces of Fire, Water, Earth and Lightning to attack their enemies."])?;
                            ctx.next()?;
                            ctx.lines_as("Bruce", args!["However, due to their weak physical strength, they are only allowed to equip Rods and Knives as weapons, and wear light armor for defense."])?;
                            ctx.next()?;
                            ctx.lines_as("Bruce", args!["Despite their physical weakness, they are able to do massive damage with their powerful spells. This fact alone attracts many people to join this class."])?;
                            ctx.next()?;
                            ctx.lines_as("Bruce", args!["In Ragnarok Online, the Mage takes a heavily offensive role in parties and is depended upon to deal great damage to enemies."])?;
                            ctx.next()?;
                            ctx.lines_as("Bruce", args!["When advancing to the Second Job Class, Mages can change their jobs to ^8E2323Wizards^000000 or ^8E2323Sages^000000."])?;
                            ctx.var("nov_3_swordman").set(Val::from(20))?;
                            ctx.next()?;
                        }
                        3 => {
                            ctx.lines_as("Bruce", args!["The Archer class are experts in using Bow weapons, and are useful in parties for their long range attacks."])?;
                            ctx.next()?;
                            ctx.lines_as("Bruce", args!["Despite being physically weaker, Archers possess high accuracy with powerful long range bows. This allows them to attack and kill monsters from a safe distance."])?;
                            ctx.next()?;
                            ctx.lines_as("Bruce", args!["In Ragnarok Online, Archers have relatively little HP, but their long range attacks allow them to easily dispatch enemies before the enemy gets close enough to hurt them."])?;
                            ctx.next()?;
                            ctx.lines_as("Bruce", args!["When advancing to the Second Job Class, every Archer may advance to the ^8E2323Hunter^000000 class. Alternatively, male Archers may advance to become ^8E2323Bards^000000, and female Archers may become ^8E2323Dancers^000000."])?;
                            ctx.var("nov_3_swordman").set(Val::from(20))?;
                            ctx.next()?;
                        }
                        4 => {
                            ctx.lines_as("Bruce", args!["The Merchant class specializes in commerce. Due to the strong influence of the Merchant Guild, the Merchant class is attractive to those who wish to focus on earning Zeny."])?;
                            ctx.next()?;
                            ctx.lines_as("Bruce", args!["In Ragnarok Online, the Merchant class possesses various economic abilities. Merchants can learn to sell items to NPCs for higher prices, as well as receive discounts from NPCs."])?;
                            ctx.next()?;
                            ctx.lines_as("Bruce", args!["In addition, Merchants may rent", "a Cart that greatly expands their carrying capacity and allows them to open shops with their own items and prices."])?;
                            ctx.next()?;
                            ctx.lines_as("Bruce", args!["When advancing to the Second Job Class, Merchants can change their jobs to ^8E2323Blacksmiths^000000 or ^8E2323Alchemists^000000."])?;
                            ctx.var("nov_3_swordman").set(Val::from(20))?;
                            ctx.next()?;
                        }
                        5 => {
                            ctx.lines_as("Bruce", args!["Thieves are experts at using Dagger class weapons. They strike quickly and easily evade attacks from their enemies."])?;
                            ctx.next()?;
                            ctx.lines_as("Bruce", args!["Thieves can learn skills that allow them to hide from their enemies, or steal items from monsters. They are also feared for their use of poison, which slowly weakens", "their enemies."])?;
                            ctx.next()?;
                            ctx.lines_as("Bruce", args!["When advancing to the Second Job Class, Thieves can change their jobs to ^8E2323Assassins^000000 or ^8E2323Rogues^000000."])?;
                            ctx.var("nov_3_swordman").set(Val::from(20))?;
                            ctx.next()?;
                        }
                        6 => {
                            ctx.lines_as("Bruce", args!["In Ragnarok Online, Acolytes act as messengers of God in Rune-Midgarts. They possess skills that support their allies, as well as the life saving Heal ability."])?;
                            ctx.next()?;
                            ctx.lines_as("Bruce", args!["The Acolyte's support abilities make them a welcome addition to any party. In difficult situations, the Acolyte's skills will ensure the survival of the party, allowing other members to focus on offense."])?;
                            ctx.next()?;
                            ctx.lines_as("Bruce", args!["When advancing to the Second Job Class, Acolytes can change their jobs to ^8E2323Priests^000000 or ^8E2323Monks^000000."])?;
                            ctx.var("nov_3_swordman").set(Val::from(20))?;
                            ctx.next()?;
                        }
                        7 => {
                            ctx.lines_as(
                                "Bruce",
                                args![
                                    "For more information,",
                                    "please visit the official",
                                    "Ragnarok Online website:",
                                    " ",
                                    "^0000FFiro.ragnarokonline.com^000000."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Bruce",
                                args!["Hanson is waiting", "for you now. Good luck", "out there, young Novice."],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    Ok(Val::from(0))
}

pub fn bruce_nv(ctx: &Ctx) -> Script {
    bruce_nv_body(ctx, Vec::new()).map(|_| ())
}

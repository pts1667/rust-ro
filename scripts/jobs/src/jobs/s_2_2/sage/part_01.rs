use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn dean_of_the_academy_sa_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::Cutin, vec![Val::from("job_sage_kayron"), Val::from(2)])?;
    ctx.mes("[Kayron Grik]")?;
    if ctx.var("Upper").get()? == 1 {
        ctx.lines(args![
            "Haha, I have seen many people",
            "but it seems you possess special power and abilities."
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Kayron Grik",
            args![
                "You'd better leave and increase your reputation.",
                "Never forget that once you also used to be a novice."
            ],
        )?;
        ctx.close_window()?;
        ctx.call(Function::Cutin, vec![Val::from("job_sage_kayron"), Val::from(255)])?;
        return Err(Stop::End);
    }
    if !ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_MAGE")?) {
        if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_SAGE")?) {
            ctx.lines(args![
                "What brings you here? Is there anything bothering you recently?",
                "Although you're already a Sage, that doesn't mean you can stop studying."
            ])?;
            ctx.next()?;
            ctx.lines_as("Kayron Grik", args!["Our knowledge is the mainspring of activity which helps the kingdom to be developed faster.", "Please keep this in mind: you must study and record everything you've discovered so that all in the kingdom may benefit."])?;
        } else if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_NOVICE")?) {
            ctx.lines(args![
                "Hahah, so my little Novice, what brings you this way? ...I guess you're on a sightseeing trip?",
                "You must have had a really hard time to reach this place. I must say, you seem interested in the Sage class."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Kayron Grik",
                args![
                    "If you aspire to become a Sage, you must first live life as a Mage. Only then shall you have a chance.",
                    "I am looking forward seeing you again."
                ],
            )?;
        } else {
            ctx.lines(args!["*Chuckle* Although we've been studying this world for a long time, I know that studying in itself will fulfill all the needs of the people.", "Other classes are just as important to the welfare of Rune-Midgarts..."])?;
            ctx.next()?;
            ctx.lines_as(
                "Kayron Grik",
                args![
                    "However, if you happen to meet a Sage down the road, I hope you will lend him your assistance.",
                    "And if you do, he shall repay you in kind..."
                ],
            )?;
        }
        ctx.close_window()?;
        ctx.call(Function::Cutin, vec![Val::from("job_sage_kayron"), Val::from(255)])?;
        return Err(Stop::End);
    }
    if ctx.var("sage_q").get()? == 0 {
        ctx.call(Function::Cutin, vec![Val::from("job_sage_kayron"), Val::from(2)])?;
        ctx.lines(args![
            "Welcome, young one. I can see that you're intrigued by the wonders of magic.",
            "So what kind of business brings you to me?"
        ])?;
        ctx.next()?;
        match runtime::select_values(
            ctx,
            &[Val::from(
                "I would like to be a Sage.:Let me know about the Sage job change.:Nothing.",
            )],
        )? {
            1 => {
                ctx.lines_as(
                    "Kayron Grik",
                    args![
                        "Hm? Do you wish to become a Sage?",
                        "Well then, I would like to suggest a few things that are required of a Sage."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Kayron Grik", args!["Sages are those who promote the development of this continent through the endless studying and recording of all knowledge related to this world.", "Becoming a Sage is more than just a costume change: It's an important job where you must be always aware of your duty and responsibility to the people."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Kayron Grik",
                    args![
                        "You may want to enter an enrollment application to the Schweicherbil Magic Academy.",
                        "I believe they can explain in detail what you need to become a Sage."
                    ],
                )?;
            }
            2 => {
                ctx.lines_as(
                    "Kayron Grik",
                    args![
                        "Sage job change...hmm...an interesting turn of phrase, I must say.",
                        "Although that is what we say, it is very inappropriate to think of becoming a Sage as merely changing a job."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Kayron Grik",
                    args![
                        "In order to be a Sage, you must enter an application to the Schweicherbil Magic Academy.",
                        "You will then take the entrance examination."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Kayron Grik",
                    args![
                        "After the examination, you will be assigned to study a specific subject...",
                        "and finally, you will submit your dissertation to the university."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Kayron Grik",
                    args![
                        "I am the one who evaluates your dissertation.",
                        "When you pass all the courses, you will become a Sage."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Kayron Grik",
                    args![
                        "Please visit the Schweicherbil Magic Academy.",
                        "A young Sage named Metheus Sylphe will accept your application."
                    ],
                )?;
            }
            3 => {
                ctx.lines_as(
                    "Kayron Grik",
                    args![
                        "I see...Well, seeing as you have leisure time to spare, I encourage you to peruse as many books as you can.",
                        "You can find and research every worldly matter within their pages."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Kayron Grik",
                    args![
                        "Ah yes, you can not become the most intelligent person merely by reading all the books around you...",
                        "However, for a Sage such as myself, the knowledge found in books is most important."
                    ],
                )?;
            }
            _ => {}
        }
        ctx.close_window()?;
        ctx.call(Function::Cutin, vec![Val::from("job_sage_kayron"), Val::from(255)])?;
        return Err(Stop::End);
    } else if ctx.var("sage_q").get()? == 15 {
        if ctx.call(Function::CountItem, vec![Val::from(1550)])?.number()? > 0 {
            if ctx.var("JobLevel").get()?.number()? < 40 {
                ctx.var("sage_q").set(Val::from(0))?;
                ctx.lines(args![
                    "You don't seem to be qualified yet.",
                    "Remember, you must reach at least job level 40 to become a Sage."
                ])?;
                ctx.close_window()?;
                ctx.call(Function::Cutin, vec![Val::from("job_sage_kayron"), Val::from(255)])?;
                return Err(Stop::End);
            }
            if ctx.var("SkillPoint").get()?.is_true() {
                ctx.lines(args![
                    "You possess remaining skill points...",
                    "Before you submit your dissertation, please take care this matter first."
                ])?;
                ctx.close_window()?;
                ctx.call(Function::Cutin, vec![Val::from("job_sage_kayron"), Val::from(255)])?;
                return Err(Stop::End);
            }
            ctx.lines(args![
                "Ho~ So? Did you finally complete your dissertation? Well done.",
                "Let me see."
            ])?;
            ctx.next()?;
            ctx.lines_as("Kayron Grik", args!["Hmm..."])?;
            ctx.next()?;
            ctx.lines_as("Kayron Grik", args!["Huh..."])?;
            ctx.next()?;
            ctx.lines_as("Kayron Grik", args!["Interesting..."])?;
            ctx.next()?;
            ctx.lines_as(
                "Kayron Grik",
                args![
                    "Although it's roughly written, it's well done for a beginner.",
                    "Ah yes...you seem to be proficient in studying."
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::CompleteQuest, vec![Val::from(2052)])?;
            shared::other_global_functions::job_change(ctx, vec![ctx.constant("JOB_SAGE")?])?;
            shared::other_global_functions::f_clearjobvar(ctx, vec![])?;
            ctx.lines_as(
                "Kayron Grik",
                args![
                    "Congratulations! You have now become a Sage.",
                    "Always remember to keep a studious and analytical mindset."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Kayron Grik",
                args![
                    "Also, keep this dissertation and treat it with care, since it is one and only book you have written.",
                    "You may have need of it one of these days. And it shall forever remind of this grandiose moment."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Kayron Grik",
                args!["Well then...May God fill your path with knowledge.", "Study with diligence!!"],
            )?;
        } else {
            ctx.lines(args![
                "Hmm? What has happened to you? Where did you leave your dissertation?",
                "Please bring it to me so that you may pass the test."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Kayron Grik",
                args![
                    "Wait...you didn't lose it, did you?",
                    "Well...that's your business. It is regrettable that you won't be able to get a chance to write a book anymore."
                ],
            )?;
        }
    } else {
        ctx.lines(args![
            "*Chuckle* Becoming a Sage isn't as simple as you may have assumed.",
            "You cannot become a Sage because your magic skills are inadequate..."
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Kayron Grik",
            args![
                "Study diligently, and return when you finish your dissertation.",
                "Until then, farewell!"
            ],
        )?;
    }
    ctx.close_window()?;
    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
    return Err(Stop::End);
}

pub fn dean_of_the_academy_sa(ctx: &Ctx) -> Script {
    dean_of_the_academy_sa_body(ctx, Vec::new()).map(|_| ())
}

fn staff_of_the_academy_a_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_count: Vec<Val> = Vec::new();
    let mut l_i = Val::from(0);
    let mut l_item: Vec<Val> = Vec::new();
    let mut l_sage_q_t = Val::from(0);
    let mut l_size = Val::from(0);
    ctx.mes("[Metheus Sylphe]")?;
    if ctx.var("Upper").get()? == 1 {
        ctx.lines(args![
            "Welcome to the",
            "Schweicherbil Magic",
            "Academy. W-wait a second...",
            "Do I know you from somewhere?"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Metheus Sylphe",
            args![
                "We've met before, haven't",
                "we? Oh gosh, I must sound",
                "pretty crazy. I'm sorry, I guess it's because I haven't been",
                "sleeping too well? Oh well,",
                "have a good day, adventurer~"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if !ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_MAGE")?) {
        if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_SAGE")?) {
            ctx.lines(args![
                "Oh nice to meet you again, long time no see.",
                "So how's it going with the studying?"
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Metheus Sylphe",
                args![
                    "It's okay to study books and magic scrolls all day, ",
                    "but you must go outside and fight with monsters as much as you can in order to be a well experienced Sage."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Metheus Sylphe",
                args![
                    "If you know any Sage candidates, please give them some advice...",
                    "Also, please give my regards to your colleagues as well."
                ],
            )?;
        } else if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_NOVICE")?) {
            ctx.mes("Welcome to the Schweicherbil Magic Academy.")?;
            ctx.next()?;
            ctx.lines_as(
                "Metheus Sylphe",
                args![
                    "This place is specialized in Sage class training. Mostly, what we do is study about monsters and magic spells.",
                    "We always welcome new students."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Metheus Sylphe",
                args![
                    "People who are at job jevel 40 as Mage class are qualified to apply for enrollment.",
                    "By passing selected courses, we will then approve them as Sages."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Metheus Sylphe",
                args!["If you're interested in the Sage class, please come again.", "And have a good day."],
            )?;
        } else {
            ctx.mes("Welcome to the Schweicherbil Magic Academy.")?;
            ctx.next()?;
            ctx.lines_as(
                "Metheus Sylphe",
                args![
                    "This place is specialized in Sage class training. What we do is study about monsters and magic spells.",
                    "People who are at job jevel 40 as Mage class are qualified to apply for enrollment."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Metheus Sylphe",
                args![
                    "If you have any Mage friends, please let them know about this academy.",
                    "Have a good day."
                ],
            )?;
        }
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("sage_q").get()? == 0 {
        ctx.lines(args![
            "Welcome to the Schweicherbil Magic Academy.",
            "Oh, You're a Mage. How may I assist you?"
        ])?;
        ctx.next()?;
        'b1: {
            let subject1 = Val::from(runtime::select_values(
                ctx,
                &[Val::from(
                    "Let me know about the Sage job change.:I want to enroll in the school.:Nothing.",
                )],
            )?);
            let mut matched1 = false;
            let no_case1 = !subject1.loosely_equals(&Val::from(1))
                && !subject1.loosely_equals(&Val::from(2))
                && !subject1.loosely_equals(&Val::from(3));
            if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as(
                    "Metheus Sylphe",
                    args![
                        "I see. Do you wish to become a Sage?",
                        "Unfortunately, we are not in charge of changing your job to the Sage class."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Metheus Sylphe",
                    args![
                        "After you enter this academy and pass certain courses...",
                        "you will receive official approval to conduct studies as a Sage."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Metheus Sylphe", args!["For this reason, we do not speak of this proccess as a job change, but as graduation.", "Anyway, if you enter your application for this academy, I will inform you about the registration fee and will let you take the test."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Metheus Sylphe",
                    args![
                        "For your information, if you bring ^3355FFOld Magicbook^000000 and ^3355FFNecklace of Wisdom^000000, ",
                        "you don't have to pay for the Registration Fee to enroll in the school."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Metheus Sylphe",
                    args![
                        "After you register, you will be able to take the entrance test.",
                        "If you pass the test, you will write a thesis for a given subject."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Metheus Sylphe",
                    args![
                        "The Dean of the academy will decide whether or not you are qualified.",
                        "If you're granted admission, you will be able to join in study and research activities as a Sage."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Metheus Sylphe", args!["You're always welcome to join us.", "Have a good day."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as(
                    "Metheus Sylphe",
                    args!["I see, you want to join the academy. Once again, welcome to the Schweicherbil Magic Academy."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Metheus Sylphe",
                    args![
                        "People who have already met the basic requirement by reaching at Mage job level 40 are qualified for enrollment.",
                        "A small registration fee will also be required."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Metheus Sylphe", args!["The registration fee is 70,000 zeny.", "However, if you bring ^3355FFOld Magicbook^000000 and ^3355FFNecklace of Wisdom^000000, you will be exempt from this fee."])?;
                ctx.next()?;
                ctx.lines_as("Metheus Sylphe", args!["So, do you wish to apply immediately?"])?;
                ctx.next()?;
                match runtime::select_values(
                    ctx,
                    &[Val::from("Yes, I do.:The fee is much too expensive.:I will come back later.")],
                )? {
                    1 => {
                        if ctx.var("JobLevel").get()?.number()? < 40 {
                            ctx.lines_as(
                                "Metheus Sylphe",
                                args![
                                    "I'm sorry, but you haven't met the basic requirements yet.",
                                    "Please go study more and reach Mage job level 40 first."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        if ctx.var("SkillPoint").get()?.is_true() {
                            ctx.lines_as(
                                "Metheus Sylphe",
                                args![
                                    "You have unused skill points left. Please go learn all those skills you've been planning to learn.",
                                    "We do not accept any ambiguous candidates."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as(
                            "Metheus Sylphe",
                            args![
                                "Very well. Let's complete your application form.",
                                "Please put your signature here."
                            ],
                        )?;
                        ctx.next()?;
                        let choice = runtime::select_values(ctx, &[ctx.call(Function::StrCharInfo, vec![Val::from(0)])?])?;
                        ctx.var("@menu").set(choice)?;
                        ctx.lines_as(
                            "Metheus Sylphe",
                            args![
                                ((Val::from("Your name is ... ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                    + Val::from(". It's a very nice name."))
                            ],
                        )?;
                        ctx.next()?;
                        if ctx.var("JobLevel").get()? == 50 {
                            ctx.lines_as(
                                "Metheus Sylphe",
                                args![
                                    "Oh, you've mastered the Mage job! You're great!! *Clap Clap Clap*",
                                    "In reward for your great effort, you will be exempt from the registration fee!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Metheus Sylphe",
                                args!["Yes, everything's ready.", "Next, you will take an entrance test."],
                            )?;
                            ctx.var("sage_q").set(Val::from(4))?;
                            ctx.call(Function::SetQuest, vec![Val::from(2041)])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Metheus Sylphe",
                                args!["Please visit Professor Claytos.", "He's in the left room."],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as(
                            "Metheus Sylphe",
                            args![
                                "Will you pay the registration fee with 70,000 zeny?",
                                "Or will you give me ^3355FFOld Magicbook^000000 and ^3355FFNecklace of Wisdom^000000?"
                            ],
                        )?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(
                            ctx,
                            &[Val::from("Pay 70,000 zeny.:Give him Old Magicbook and Necklace of Wisdom.")],
                        )?) == 1
                        {
                            if ctx.var("Zeny").get()?.number()? > 69999 {
                                ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(70000))?))?;
                                ctx.lines_as(
                                    "Metheus Sylphe",
                                    args![
                                        "Thank you, your application has been accepted.",
                                        "Next, you will take an entrance test."
                                    ],
                                )?;
                                ctx.var("sage_q").set(Val::from(4))?;
                                ctx.call(Function::SetQuest, vec![Val::from(2041)])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Metheus Sylphe",
                                    args!["Please visit Professor Claytos.", "He's in the left room."],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.lines_as(
                                "Metheus Sylphe",
                                args![
                                    "What a shame! It seems you didn't bring enough money for tuition.",
                                    "Please make sure you have at least 70,000 zeny to enroll in classes."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        if (ctx.call(Function::CountItem, vec![Val::from(1006)])?.number()? > 0
                            && ctx.call(Function::CountItem, vec![Val::from(1007)])?.number()? > 0)
                        {
                            ctx.call(Function::DelItem, vec![Val::from(1006), Val::from(1)])?;
                            ctx.call(Function::DelItem, vec![Val::from(1007), Val::from(1)])?;
                            ctx.lines_as(
                                "Metheus Sylphe",
                                args![
                                    "Thank you, your application has been accepted.",
                                    "Next, you will take the entrance test."
                                ],
                            )?;
                            ctx.var("sage_q").set(Val::from(4))?;
                            ctx.call(Function::SetQuest, vec![Val::from(2041)])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Metheus Sylphe",
                                args!["Please visit Professor Claytos.", "He's in the left room."],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as(
                            "Metheus Sylphe",
                            args![
                                "Umm...It seems you didn't bring any of those?",
                                "I suppose you left them somewhere behind. Please go get them, and then come back."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        if ctx.var("JobLevel").get()?.number()? < 40 {
                            ctx.lines_as("Metheus Sylphe", args!["Before we talk about the registration fee, it seems you haven't met the basic requirement yet, Mage job level 40.", "Please go study more, and then come back to enroll."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        if ctx.var("JobLevel").get()? == 50 {
                            ctx.lines_as("Metheus Sylphe", args!["Well, I can't help you with that issue. If you don't have the fee, you are not allowed to enter the academy.", "Even if you might think it's absurdly expensive, it's a justifiable price to pay in order to become a Sage."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Metheus Sylphe",
                                args![
                                    "Anyway... oh! You mastered the Mage job! You're truly exemplary!! *Clap Clap Clap*.",
                                    "As a reward for your great effort, you will be exempt from the registration fee!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Metheus Sylphe",
                                args!["Okay, let's complete the application form.", "Please put your signature here."],
                            )?;
                            ctx.next()?;
                            let choice = runtime::select_values(ctx, &[ctx.call(Function::StrCharInfo, vec![Val::from(0)])?])?;
                            ctx.var("@menu").set(choice)?;
                            ctx.lines_as(
                                "Metheus Sylphe",
                                args![
                                    ((Val::from("Your name is ... ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                        + Val::from(". It's a very nice name."))
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Metheus Sylphe",
                                args!["Yes, everything's ready.", "Next, you will take the entrance test."],
                            )?;
                            ctx.var("sage_q").set(Val::from(4))?;
                            ctx.call(Function::SetQuest, vec![Val::from(2041)])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Metheus Sylphe",
                                args!["Please visit professor Claytos.", "He's in the left room."],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        if ctx.var("Zeny").get()?.number()? > 43210 {
                            ctx.lines_as("Metheus Sylphe", args!["Well, I can't help you with that issue. If you don't have the fee, you are not allowed to enter the academy.", "Even if you may think it's absurdly expensive, it's a justifiable price to pay in order to become a Sage."])?;
                            ctx.next()?;
                            ctx.lines_as("Metheus Sylphe", args!["Alternatively, you could try to find ^3355FFOld Magicbook^000000 and ^3355FFNecklace of Wisdom^000000.", "If you don't wish to do that, you must save some money for the registration fee."])?;
                            ctx.next()?;
                            ctx.lines_as("Metheus Sylphe", args!["Goodbye, and have a good day."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            ctx.lines_as(
                                "Metheus Sylphe",
                                args![
                                    "Oh, I guess you don't have enough money?",
                                    "Under the existing provisions, you must pay 70,000 zeny for the application..."
                                ],
                            )?;
                            ctx.next()?;
                            if Val::from(runtime::select_values(
                                ctx,
                                &[Val::from("Please...is there any way?:Ok, I will come back later.")],
                            )?) == 1
                            {
                                ctx.lines_as(
                                    "Metheus Sylphe",
                                    args![
                                        "Hmmm...then I shall offer a special option!",
                                        "You will pay 30,000 zeny and bring some items as compensation for the tuition discount."
                                    ],
                                )?;
                                ctx.next()?;
                                let subject3 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                                if subject3 == 1 {
                                    ctx.var("sage_q").set(Val::from(1))?;
                                    ctx.call(Function::SetQuest, vec![Val::from(2043)])?;
                                    ctx.lines_as(
                                        "Metheus Sylphe",
                                        args![
                                            "Please gather the following items.",
                                            "50 ^3355FFFeather of Birds^000000",
                                            "50 ^3355FFFluff^000000",
                                            "25 ^3355FFIron Ore^000000"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Metheus Sylphe", args!["If you bring those items, your tuition will be 30,000 zeny, in lieu of the original 70,000 zeny fee."])?;
                                } else if subject3 == 2 {
                                    ctx.var("sage_q").set(Val::from(2))?;
                                    ctx.call(Function::SetQuest, vec![Val::from(2044)])?;
                                    ctx.lines_as(
                                        "Metheus Sylphe",
                                        args![
                                            "Please gather the following items.",
                                            "50 ^3355FFClover^000000",
                                            "50 ^3355FFFeather^000000",
                                            "25 ^3355FFSquid Ink^000000"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Metheus Sylphe", args!["If you bring the aforementioned items, the tuition fee will be 30,000 zeny, rather than the original 70,000 zeny fee."])?;
                                } else if subject3 == 3 {
                                    ctx.var("sage_q").set(Val::from(3))?;
                                    ctx.call(Function::SetQuest, vec![Val::from(2045)])?;
                                    ctx.lines_as(
                                        "Metheus Sylphe",
                                        args![
                                            "Please gather the following items.",
                                            "50 ^3355FFFeather of Birds^000000",
                                            "50 ^3355FFFluff^000000",
                                            "50 ^3355FFClover^000000",
                                            "50 ^3355FFFeather^000000"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Metheus Sylphe", args!["If you bring those items, your tuition will only be 30,000 zeny, instead of the original 70,000 zeny fee."])?;
                                }
                                ctx.mes("I am sure it's a very reasonable option for you.")?;
                                ctx.next()?;
                                ctx.lines_as("Metheus Sylphe", args!["Ah yes, before gathering all of those items, if you happen to have 70,000 zeny, I will be more than happy to receive the full payment.", "That is, after all, our original policy."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Metheus Sylphe",
                                    args![
                                        "Alternatively, you can bring me ^3355FFOld Magicbook^000000 and ^3355FFNecklace of Wisdom^000000.",
                                        "Goodbye, and have a good day."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.lines_as(
                                "Metheus Sylphe",
                                args!["Ah yes, take your time...", "Goodbye, and have a good day."],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                    3 => {
                        ctx.lines_as(
                            "Metheus Sylphe",
                            args!["Ah yes, take your time...", "Goodbye, and have a good day."],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            }
            if !matched1 && subject1.loosely_equals(&Val::from(3)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as(
                    "Metheus Sylphe",
                    args![
                        "I see, take your time. You can also take a look around.",
                        "Goodbye, and have a good day."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    } else if (ctx.var("sage_q").get()?.number()? >= 1 && ctx.var("sage_q").get()?.number()? <= 3) {
        ctx.mes("Welcome, once again.")?;
        ctx.next()?;
        if (ctx.call(Function::CountItem, vec![Val::from(1006)])?.number()? > 0
            && ctx.call(Function::CountItem, vec![Val::from(1007)])?.number()? > 0)
        {
            ctx.call(Function::DelItem, vec![Val::from(1006), Val::from(1)])?;
            ctx.call(Function::DelItem, vec![Val::from(1007), Val::from(1)])?;
            ctx.lines_as(
                "Metheus Sylphe",
                args!["Well done. Let me proceed with your application request."],
            )?;
            ctx.var("sage_q").set(Val::from(4))?;
            ctx.next()?;
        } else if ctx.var("Zeny").get()?.number()? > 69999 {
            ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(70000))?))?;
            ctx.lines_as(
                "Metheus Sylphe",
                args!["Well done. Let me proceed with your application request."],
            )?;
            ctx.var("sage_q").set(Val::from(4))?;
            ctx.next()?;
        } else {
            let subject4 = ctx.var("sage_q").get()?;
            if subject4 == 1 {
                let base = Val::from(0).number()?;
                runtime::local_set(&mut l_item, &Val::from(base + 0), Val::from(916), false);
                runtime::local_set(&mut l_item, &Val::from(base + 1), Val::from(914), false);
                runtime::local_set(&mut l_item, &Val::from(base + 2), Val::from(1002), false);
                let base = Val::from(0).number()?;
                runtime::local_set(&mut l_count, &Val::from(base + 0), Val::from(50), false);
                runtime::local_set(&mut l_count, &Val::from(base + 1), Val::from(50), false);
                runtime::local_set(&mut l_count, &Val::from(base + 2), Val::from(25), false);
            } else if subject4 == 2 {
                let base = Val::from(0).number()?;
                runtime::local_set(&mut l_item, &Val::from(base + 0), Val::from(705), false);
                runtime::local_set(&mut l_item, &Val::from(base + 1), Val::from(949), false);
                runtime::local_set(&mut l_item, &Val::from(base + 2), Val::from(1024), false);
                let base = Val::from(0).number()?;
                runtime::local_set(&mut l_count, &Val::from(base + 0), Val::from(50), false);
                runtime::local_set(&mut l_count, &Val::from(base + 1), Val::from(50), false);
                runtime::local_set(&mut l_count, &Val::from(base + 2), Val::from(25), false);
            } else if subject4 == 3 {
                let base = Val::from(0).number()?;
                runtime::local_set(&mut l_item, &Val::from(base + 0), Val::from(916), false);
                runtime::local_set(&mut l_item, &Val::from(base + 1), Val::from(914), false);
                runtime::local_set(&mut l_item, &Val::from(base + 2), Val::from(705), false);
                runtime::local_set(&mut l_item, &Val::from(base + 3), Val::from(949), false);
                let base = Val::from(0).number()?;
                runtime::local_set(&mut l_count, &Val::from(base + 0), Val::from(50), false);
                runtime::local_set(&mut l_count, &Val::from(base + 1), Val::from(50), false);
                runtime::local_set(&mut l_count, &Val::from(base + 2), Val::from(50), false);
                runtime::local_set(&mut l_count, &Val::from(base + 3), Val::from(50), false);
            }
            l_size = (Val::from(l_item.len() as i32).try_sub(Val::from(1))?);
            l_i = Val::from(0);
            'l5: loop {
                if !(runtime::op(&l_i.clone(), "<", &l_size.clone())?.is_true()
                    && runtime::op(
                        &ctx.call(Function::CountItem, vec![runtime::local_get(&l_item, &l_i.clone(), false)])?,
                        ">=",
                        &runtime::local_get(&l_count, &l_i.clone(), false),
                    )?
                    .is_true())
                {
                    break 'l5;
                }
                'b5: {}
                l_i = (l_i.clone() + Val::from(1));
            }
            if l_i.clone().loosely_equals(&l_size.clone()) {
                if ctx.var("Zeny").get()?.number()? > 29999 {
                    l_i = Val::from(0);
                    'l6: loop {
                        if !(runtime::op(&l_i.clone(), "<", &l_size.clone())?.is_true()) {
                            break 'l6;
                        }
                        'b6: {
                            ctx.call(
                                Function::DelItem,
                                vec![
                                    runtime::local_get(&l_item, &l_i.clone(), false),
                                    runtime::local_get(&l_count, &l_i.clone(), false),
                                ],
                            )?;
                        }
                        l_i = (l_i.clone() + Val::from(1));
                    }
                    ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(30000))?))?;
                    ctx.lines_as(
                        "Metheus Sylphe",
                        args!["Well done. Let me proceed with your application request."],
                    )?;
                    l_sage_q_t = ctx.var("sage_q").get()?;
                    ctx.var("sage_q").set(Val::from(4))?;
                    ctx.next()?;
                }
                ctx.lines_as(
                    "Metheus Sylphe",
                    args![
                        "I am sorry to say that you are not ready yet.",
                        "Although you brought all of the items, the money you have now is less than 30,000 zeny."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Metheus Sylphe",
                    args![
                        "As I told you before, you must bring all of those items, as well as the 30,000 zeny together.",
                        "Please make sure that you have the required items and money."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Metheus Sylphe",
                    args![
                        "I am sorry to say that it seems you didn't bring all of the required items.",
                        "I shall remind you what to bring, in case you have forgotten."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Metheus Sylphe",
                    args![
                        "Please bring the following items to me.",
                        (((runtime::local_get(&l_count, &Val::from(0), false) + Val::from(" ^3355FF"))
                            + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_item, &Val::from(0), false)])?)
                            + Val::from("^000000")),
                        (((runtime::local_get(&l_count, &Val::from(1), false) + Val::from(" ^3355FF"))
                            + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_item, &Val::from(1), false)])?)
                            + Val::from("^000000")),
                        (((runtime::local_get(&l_count, &Val::from(2), false) + Val::from(" ^3355FF"))
                            + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_item, &Val::from(2), false)])?)
                            + Val::from("^000000"))
                    ],
                )?;
                if ctx.var("sage_q").get()? == 3 {
                    ctx.lines(args![
                        (((runtime::local_get(&l_count, &Val::from(3), false) + Val::from(" ^3355FF"))
                            + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_item, &Val::from(3), false)])?)
                            + Val::from("^000000"))
                    ])?;
                }
                ctx.next()?;
                ctx.lines_as(
                    "Metheus Sylphe",
                    args![
                        "If you bring all of these items, your tuition fee will be reduced from 70,000 zeny to 30,000 zeny.",
                        "Good luck."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
        ctx.lines_as(
            "Metheus Sylphe",
            args!["Let's complete the application form.", "Please put your signature here."],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[ctx.call(Function::StrCharInfo, vec![Val::from(0)])?])?;
        ctx.var("@menu").set(choice)?;
        ctx.mes("[Metheus Sylphe]")?;
        let subject7 = l_sage_q_t.clone();
        if subject7 == 1 {
            ctx.lines(args![
                ((Val::from("Your name is ... ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                    + Val::from(". It's a very nice name."))
            ])?;
        } else if subject7 == 2 {
            ctx.lines(args![
                ((Val::from("Your name is ... ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                    + Val::from(". It sounds very sagacious."))
            ])?;
        } else if subject7 == 3 {
            ctx.lines(args![
                ((Val::from("Your name is ... ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                    + Val::from(". Interesting name."))
            ])?;
        }
        ctx.next()?;
        ctx.lines_as(
            "Metheus Sylphe",
            args!["Ah yes, everything is in readiness.", "Next, you will take an entrance test."],
        )?;
        ctx.var("sage_q").set(Val::from(4))?;
        if ctx.call(Function::CheckQuest, vec![Val::from(2043)])? != -1 {
            ctx.call(Function::ChangeQuest, vec![Val::from(2043), Val::from(2041)])?;
        } else if ctx.call(Function::CheckQuest, vec![Val::from(2044)])? != -1 {
            ctx.call(Function::ChangeQuest, vec![Val::from(2044), Val::from(2041)])?;
        } else if ctx.call(Function::CheckQuest, vec![Val::from(2045)])? != -1 {
            ctx.call(Function::ChangeQuest, vec![Val::from(2045), Val::from(2041)])?;
        } else {
            ctx.call(Function::SetQuest, vec![Val::from(2041)])?;
        }
        ctx.next()?;
        ctx.lines_as(
            "Metheus Sylphe",
            args!["Please visit Professor Claytos.", "He's in the left room."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("sage_q").get()? == 4 {
        ctx.lines(args![
            "Huh? What are you doing here? You're supposed to be taking the entrance test by now.",
            "Please visit Professor Claytos in the left room."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("sage_q").get()? == 15 {
        ctx.lines(args![
            "Oh, are you done with the dissertation?",
            "Sure, you can submit it to Dean Kayron."
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Metheus Sylphe",
            args!["So long as you make the effort, you should achieve good results.", "Good luck."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines(args![
            "Oh sorry, this is a rather inconvenient time to converse.",
            "Please come back later. I apologize for troubling you."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn staff_of_the_academy_a(ctx: &Ctx) -> Script {
    staff_of_the_academy_a_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum WrittenTestProfessorSStep {
    Start,
    LAskQuestions,
    HoistEnd1,
    HoistEnd2,
    HoistEnd3,
}

fn written_test_professor_s_run(ctx: &Ctx, mut step: WrittenTestProfessorSStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sage_t = Val::from(0);
    'machine: loop {
        match step {
            WrittenTestProfessorSStep::Start => {
                ctx.mes("[Claytos Verdo]")?;
                if !ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_MAGE")?) {
                    if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_SAGE")?) {
                        ctx.lines(args![
                            "Eh? What? Why are you back here?",
                            "Do you want to enter the school again?"
                        ])?;
                        ctx.next()?;
                        ctx.lines_as("Claytos Verdo", args!["Now, I understand how you feel. Since you graduated, you have become a Sage. A Sage...until the end of your days.", "So, be strong and independent. Try to explore some places where nobody else has ventured to go."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Claytos Verdo",
                            args![
                                "Don't forget to record everything you've experienced.",
                                "You must share your knowledge with others by taking excellent notes."
                            ],
                        )?;
                    } else if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_NOVICE")?) {
                        ctx.lines(args![
                            "What are you doing here, kid?",
                            "This is a Magic Academy, not a day care center."
                        ])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Claytos Verdo",
                            args!["Go outside and play with the Porings. That's your job.", "Go out, chop chop!!"],
                        )?;
                    } else if ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_WIZARD")?) {
                        ctx.lines(args![
                            "Well...look who came crawling back. Magic addict.",
                            "Yeah yeah, so it's not so bad to be devoted to magic."
                        ])?;
                        ctx.next()?;
                        ctx.lines_as("Claytos Verdo", args!["But I hope you remember, no one can live alone.", "Although you're strong enough for solo play, you must cooperate and help other people. That's what a Wizard shoud stand for."])?;
                    } else {
                        ctx.lines(args!["Hmm... I understand that you want to enter our prestigious academy, but since you chose to live as a different class,", "I don't think you can become a Sage."])?;
                        ctx.next()?;
                        ctx.lines_as("Claytos Verdo", args!["So, don't go around regretting why you chose a job other than Sage. You'd better go out and hunt, leveling up your current job."])?;
                    }
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ctx.var("sage_q").get()? == 0 {
                    {
                        ctx.lines(args![
                            "What, do you want to be a Sage?",
                            "I can tell by your eyes, hungering for wisdom."
                        ])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Claytos Verdo",
                            args![
                                "Of course, if you want to be a Sage, you must first enter the academy.",
                                "Apply for enrollment, and then come again."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                } else {
                    if ((ctx.var("sage_q").get()? == 1 || ctx.var("sage_q").get()? == 2) || ctx.var("sage_q").get()? == 3) {
                        {
                            ctx.lines(args![
                                "Hah! You didn't even finish the application process!?",
                                "I see...did Metheus tell you something?"
                            ])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Claytos Verdo",
                                args![
                                    "Do your best. It'll be a good experience for you.",
                                    "Come again when you finish the application."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    } else {
                        if ctx.var("sage_q").get()? == 4 {
                            ctx.lines(args![
                                "Welcome to the Schweicherbil Magic Academy.",
                                "You applied for this test already, didn't you?"
                            ])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Claytos Verdo",
                                args![
                                    ((Val::from("Let's see, your name is ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                        + Val::from("...")),
                                    "Okay, let's get started!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Claytos Verdo", args!["The test that I am going to give you will test your knowledge on all of the academic subjects in the world.", "I will give you 20 questions, with each question being worth 5 points. When you earn a grade of 80 points, you will pass the test."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Claytos Verdo",
                                args![
                                    "Okay, there's no need to wait. Let's start right away",
                                    "Oh, and if you don't answer immediately, the test will be cancelled."
                                ],
                            )?;
                            step = WrittenTestProfessorSStep::LAskQuestions;
                            continue 'machine;
                        } else if ctx.var("sage_q").get()? == 5 {
                            ctx.lines(args!["Welcome back.", "So, did you study harder this time?"])?;
                            ctx.next()?;
                            ctx.lines_as("Claytos Verdo", args!["You will take the written test under the same conditions as the test you took before. I'll give you 20 questions.", "Each correct answer will give you 5 points. When your score reaches 80 points, you pass the test."])?;
                            ctx.next()?;
                            ctx.var("sage_m2")
                                .set(ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?)?;
                            ctx.lines_as(
                                "Claytos Verdo",
                                args!["Okay, there's no need to wait.", "Answer immediately, or I'll fail you again."],
                            )?;
                            ctx.var("sage_q").set(Val::from(5))?;
                            step = WrittenTestProfessorSStep::LAskQuestions;
                            continue 'machine;
                            return Err(Stop::End);
                        } else if ctx.var("sage_q").get()? == 6 {
                            ctx.lines(args![
                                "What else do you want?! Do you want to take this test again?",
                                "You've already passed!"
                            ])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Claytos Verdo",
                                args!["Go visit Professor Hermes for the practical examination.", "Move!"],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.var("sage_q").get()? == 15 {
                            ctx.lines(args![
                                "Heh heh, It seems you're done with your dissertation.",
                                "But I'm not the person handling that part of the test."
                            ])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Claytos Verdo",
                                args![
                                    "submit your thesis to Dean Kayron.",
                                    "He will decide whether you are qualified to graduate or not."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            ctx.lines(args![
                                "I'm too busy to take care of written tests.",
                                "Come back later, and I'll spare some time to talk."
                            ])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        step = WrittenTestProfessorSStep::HoistEnd1;
                        continue 'machine;
                        step = WrittenTestProfessorSStep::LAskQuestions;
                        continue 'machine;
                    }
                    step = WrittenTestProfessorSStep::HoistEnd2;
                    continue 'machine;
                    step = WrittenTestProfessorSStep::LAskQuestions;
                    continue 'machine;
                }
                step = WrittenTestProfessorSStep::HoistEnd3;
                continue 'machine;
            }
            WrittenTestProfessorSStep::LAskQuestions => {
                ctx.next()?;
                let subject1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                if subject1 == 1 {
                    ctx.mes("1. Choose an item that the Gift merchant in Prontera does not sell.")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("China:Red Frame:Bouquet:Glass Bead")])?) == 3 {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("2. Choose a city where you cannot purchase a Stiletto.")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("Prontera:Morocc:Geffen:Lutie")])?) == 1 {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("3. Choose the closest city to Turtle Island.")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("Al De Baran:Alberta:Comodo:Izlude")])?) == 2 {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("4. Choose the monster that is a different type than the others.")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("Raggler:Pest:Frilldora:Aster")])?) == 4 {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("5. Choose the monster that has a different attribute than the others.")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("Mantis:Metaller:Rocker:Horn")])?) == 2 {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("6. Choose the monster that is different sized than the others.")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("Raydric:Raydric Archer:Wanderer:Dark Frame")],
                    )?) == 1
                    {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("7. Choose the monster which doesn't drop 'Alcohol'.")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("Horong:Plankton:Poison Spore:Toad")])?) == 3 {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("8. Choose the NPC that is irrelevant to the Knight job change quest.")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("Sir Siracuse:Thomas Servantes:Sir Windsor:Lady Amy")],
                    )?) == 2
                    {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("9. Choose the NPC that is not a citizen of Prontera.")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("Tono:Pina:YuPi:Hollgrehenn")])?) == 2 {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("10. Choose the right name for the Kafra lady who wears glasses.")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("Pavianne:Roxie:Leilah:Curly Sue")])?) == 3 {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("11. How much SP is spent to use lvl 7 Thunderstorm?")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("49:59:69:74")])?) == 2 {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("12. Choose the right amount of damage reduction and SP consumption of the Energy Coat skill when the caster's remaining SP is 50%.")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("Damage -24% SP1.5%:Damage -24% SP2%:Damage -18% SP1.5%:Damage -18% SP2%")],
                    )?) == 4
                    {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("13. Choose the property that is irrelevant to 'Bolt' type skills for the Mage class.")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("Water:Earth:Fire:Wind")])?) == 2 {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("14. Choose the right chance and attack strength for lvl 7 Double Attack, the Thief skill.")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("35% / 120%:35% / 140%:40% / 120%:40% / 140%")],
                    )?) == 2
                    {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("15. Choose the skill that is irrelevant to learning Magnus Exorcismus, the Priest skill.")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("Divine Protection:Heal:Ruwach:Aqua Benedicta")],
                    )?) == 1
                    {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("16. Choose the correct defense and ability of the Bunny Band.")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("1 / LUK +2:1 / LUK +5:2 / LUK +2:2 / LUK +5")],
                    )?) == 3
                    {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("17. Choose the class that cannot equip Padded Armor.")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("Swordman:Merchant:Thief:Archer")])?) == 4 {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("18. Choose the item that cures all abnormal status and restores full HP and SP at the same time.")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("Royal Jelly:Yggdrasil Seed:Yggdrasilberry:Mastella Fruit")],
                    )?) == 3
                    {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("19. Who rules the Rune-Midgarts kingdom right now?")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("Tristun the 3rd:Tristram the 3rd:Tristar the 3rd:Trast the 3rd")],
                    )?) == 2
                    {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("20. Choose the god of Crusaders.")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("Odin:Loki:Thor:Venadin")])?) == 1 {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                } else if subject1 == 2 {
                    ctx.mes("1. Choose the jewel that the Morocc Jewel Merchant does not sell.")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("Topaz:Garnet:Diamond:Sapphire")])?) == 2 {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("2. Choose the city where users cannot purchase Monster's Feed from an NPC.")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("Prontera:Morocc:Al De Baran:Alberta")],
                    )?) == 3
                    {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("3. Choose the closest city to the Maze.")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("Prontera:Morocc:Geffen:Payon")])?) == 1 {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("4. Choose the monster that is a different type than the others.")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("Muka:Drops:Plankton:Penomena")])?) == 4 {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("5. Choose the monster with the different attribute.")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("Dokebi:Isis:Giearth:Deviruchi")])?) == 3 {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("6. Choose the monster that is different in size.")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("Thiefbug (Aggressive):Horn:Metaller:Argos")],
                    )?) == 4
                    {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("7. Choose the monster which does not drop 'Yggdrasil Leaf'.")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("Marduk:Baphomet Jr.:Angeling:Wanderer")],
                    )?) == 1
                    {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("8. Choose the NPC that is irrelevant to the Priest job change quest.")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("Paul:Sir Windsor:Peter S. Alberto:Cecilia")],
                    )?) == 2
                    {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("9. Choose the NPC that is not a citizen of Morocc.")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("Syvia:Akira:Antonio:Dmitrii")])?) == 3 {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("10. Choose the Kafra lady who has gorgeous blue hair.")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("Pavianne:Roxie:Leilah:Curly Sue")])?) == 1 {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("11. Choose the skill that is irrelevant to learning Fire Wall, the Mage skill.")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("lvl 4 Fire Bolt:lvl 4 Napalm Beat:lvl 5 Fire Ball:lvl 1 Sight")],
                    )?) == 2
                    {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("12. How much SP can be restored when learning SP recovery at lvl 6 (without being affected by INT)?")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("14:16:18:21")])?) == 3 {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("13. How many INT points does a Mage receive as a bonus at job lvl 33?")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("7:6:5:4")])?) == 4 {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes(
                        "14. Choose the correct SP consumption and the skill duration for Improve Concentration lvl 5 (Archer skill).",
                    )?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("45 / 80 sec:50 / 80 sec:45 / 90 sec:50 / 90 sec")],
                    )?) == 1
                    {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("15. Choose the skill that is irrelevant to learning Maximize Power, the Blacksmith skill.")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("Hilt Binding:Skin Tempering:Hammer Fall:Weapon Perfection")],
                    )?) == 2
                    {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("16. What is the correct defense rate and ability of Cute Ribbon?")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("0 / SP +20:0 / SP +30:1 / SP +20:1 / SP +30")],
                    )?) == 3
                    {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("17. Choose the class that cannot equip Saint Robe.")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("Swordman:Merchant:Thief:Acolyte")])?) == 3 {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("18. Choose the abnormal status that cannot be cured by Green Potion.")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("Silence:Chaos:Blind:Curse")])?) == 4 {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("19. Choose the correct name for the ancient kingdom that disappeared somewhere in Geffen.")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("Geffayon:Geffenia:Gefenn:Jaffen")])?) == 2 {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("20. Choose the correct name for the tree that has become the root of this world.")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("Yggdrasil:Iggdrassil:Mastella:Dead Branch")],
                    )?) == 1
                    {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                } else if subject1 == 3 {
                    ctx.mes("1. Choose the item that the Magical Tool merchant in Geffen does not sell.")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("Mantle:Wand:Circlet:Silver Robe")])?) == 1 {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("2. Choose the city where users cannot purchase Blade from an NPC.")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("Prontera:Izlude:Al De Baran:Payon")])?) == 3 {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("3. Choose the closest city to Glast Heim.")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("Prontera:Geffen:Morocc:Payon")])?) == 2 {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("4. Choose the monster that is a different type than the others.")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("Aster:Marc:Marse:Marin")])?) == 4 {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("5. Choose the monster that has a different attribute.")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("Baby Desert Wolf:Smokie:Picky:Choco")],
                    )?) == 2
                    {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("6. Choose the monster that is different sized.")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("Drake:Wraith:Evil Druid:Khalitzburg")],
                    )?) == 1
                    {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("7. Choose the monster that does not drop 'Phracon'.")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("Pupa:Peco Peco Egg:Savage Bebe:Baby Desert Wolf")],
                    )?) == 2
                    {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("8. Choose the NPC that is irrelevant to the Blacksmith job change quest.")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("Altiregen:Geschupenschte:Barcadi:Baisulist")],
                    )?) == 3
                    {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("9. Choose the NPC that is not a citizen of Al De Baran.")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("RS125:GOD-POING:Stromme:Chemirre")])?) == 2 {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("10. Choose the Kafra lady who is the youngest among the staff.")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("Pavianne:Roxie:Leilah:Curly Sue")])?) == 4 {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("11. Choose the correct SP consumption and the number of evasions when using Safety Wall lvl 6.")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("SP 40, 6 times:SP 35, 6 times:SP 40, 7 times:SP 35, 7 times")],
                    )?) == 3
                    {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("12. Choose the correct amount of magic attack for Napalm Beat lvl 6.")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("MATK * 1.2:MATK * 1.3:MATK * 1.4:MATK * 1.5")],
                    )?) == 2
                    {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("13. Choose the catalyst stone for Mage Solution no. 4 that is used for the Mage job change quest.")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("Blue Gemstone:Red Gemstone:Yellow Gemstone:1 carat Diamond")],
                    )?) == 4
                    {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("14. Choose the correct attack strength and SP consumption for Bash lvl 6, the Swordman skill.")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("250% / 8:280% / 8:280% / 15:310% / 15")],
                    )?) == 3
                    {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("15. Choose the skill that is irrelevant to learning Claymore Trap, the Hunter skill.")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("Remove Trap:Land Mine:Ankle Snare:Flasher")],
                    )?) == 1
                    {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("16. Choose the correct defense and ability of Wedding Veil.")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("0 / MDEF +3:0 / MDEF +5:1 / MDEF +3:1 / MDEF +5")],
                    )?) == 2
                    {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("17. Choose the class that cannot equip Coat.")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("Swordman:Merchant:Thief:Novice")])?) == 4 {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("18. Choose the item that is not an ingredient for Blue Dyestuffs.")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("Alcohol:Detrimindexta:Karvodailnirol:Blue Herb")],
                    )?) == 3
                    {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("19. When the world was created by the god Odin, what did he use for the material?")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(
                        ctx,
                        &[Val::from(
                            "The heart of Ymir:The nail of Ymir:The tooth of Ymir:The memento of Ymir",
                        )],
                    )?) == 1
                    {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                    ctx.mes("20. Choose the metal that has rumored to bring fortune and fame to a person with the destiny.")?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("Empelium Gold:Emperium:Emperor:Phracon")],
                    )?) == 2
                    {
                        l_sage_t = (l_sage_t.clone() + Val::from(5));
                    }
                }
                ctx.mes("[Claytos Verdo]")?;
                if ctx.var("sage_q").get()? == 4 {
                    ctx.lines(args![
                        "Well, you answered all 20 of the questions.",
                        "Okay, let me check your answers and add up your score."
                    ])?;
                } else {
                    ctx.lines(args![
                        "Well, we finished all 20 questions.",
                        "Now, let's check how many points you got."
                    ])?;
                }
                ctx.next()?;
                ctx.lines_as("Claytos Verdo", args!["Let's see...", "Hmm... hmm..."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Claytos Verdo",
                    args![((Val::from("You got ") + l_sage_t.clone()) + Val::from(" points."))],
                )?;
                if l_sage_t.clone() == 100 {
                    if ctx.var("sage_q").get()? == 4 {
                        ctx.mes("Excellent! You seem fully qualified to become a Sage!")?;
                    } else {
                        ctx.mes("Excellent! You must have studed really hard for this test!")?;
                    }
                    ctx.var("sage_q").set(Val::from(6))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(2041), Val::from(2046)])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Claytos Verdo",
                        args![
                            "You have passed the written test.",
                            "Go visit Professor Hermes for the practical examination."
                        ],
                    )?;
                } else if l_sage_t.clone().number()? >= 80 {
                    ctx.var("sage_q").set(Val::from(6))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(2041), Val::from(2046)])?;
                    ctx.mes("Yeah, not bad. I assume that you will at least understand what you're going to learn in class.")?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Claytos Verdo",
                        args![
                            "You passed the written test.",
                            "Go visit Professor Hermes for the practical examination."
                        ],
                    )?;
                } else if ctx.var("sage_q").get()? == 4 {
                    ctx.var("sage_q").set(Val::from(5))?;
                    ctx.mes("Oh well...what a shame: You failed.")?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Claytos Verdo",
                        args![
                            "But I'll give you another chance to take the written test,",
                            "Go study harder and come back later."
                        ],
                    )?;
                } else {
                    ctx.mes("Oh what a shame: You failed.")?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Claytos Verdo",
                        args!["But I'll give you another chance,", "Go study even harder and come back."],
                    )?;
                }
                ctx.close_window()?;
                return Err(Stop::End);
                step = WrittenTestProfessorSStep::HoistEnd1;
                continue 'machine;
            }
            WrittenTestProfessorSStep::HoistEnd1 => {
                step = WrittenTestProfessorSStep::HoistEnd2;
                continue 'machine;
            }
            WrittenTestProfessorSStep::HoistEnd2 => {
                step = WrittenTestProfessorSStep::HoistEnd3;
                continue 'machine;
            }
            WrittenTestProfessorSStep::HoistEnd3 => {
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn written_test_professor_s(ctx: &Ctx) -> Script {
    written_test_professor_s_run(ctx, WrittenTestProfessorSStep::Start, Vec::new()).map(|_| ())
}

fn practical_examination_p_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.mes("[Hermes Tris]")?;
    if !ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_MAGE")?) {
        if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_SAGE")?) {
            ctx.lines(args![
                "Welcome. How have you been?",
                "I guess you've been through a lot of hard times...I can tell by your appearance."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Hermes Tris",
                args![
                    "I know how hard it is to explore all those perilous areas, but it will help you to gain more knowledge.",
                    "Book smarts never can beat street smarts."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Hermes Tris",
                args![
                    "However, it's a very dangerous idea to go deep inside a dungeon alone. ",
                    "You'd better look for trustworthy comrades."
                ],
            )?;
        } else if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_NOVICE")?) {
            ctx.mes("Heh heh, now ain't that a cute little Novice?")?;
            ctx.next()?;
            ctx.lines_as(
                "Hermes Tris",
                args![
                    "In this continent of Rune-Midgarts, there are a lot of unknown places and objects that haven't been fully discovered.",
                    "The monsters, mysterious objects and heroes of myths..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Hermes Tris",
                args![
                    "Why don't you consider being a Sage in the future?",
                    "You will love studying the world."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Hermes Tris",
                args!["If by chance you do decide to do that, we'll meet again.", "Take care, kiddy."],
            )?;
        } else {
            ctx.mes("Welcome to the Schweicherbil Magic Academy.")?;
            ctx.next()?;
            ctx.lines_as(
                "Hermes Tris",
                args![
                    "We Sages are more like scholars than Mages.",
                    "We are very helpful and powerful as members of a party."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Hermes Tris",
                args![
                    "Try to make a party with a Sage next time.",
                    "The wisdom a Sage will bring will be more than helpful for your party..."
                ],
            )?;
        }
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("sage_q").get()?.number()? >= 0 && ctx.var("sage_q").get()?.number()? <= 3) {
        ctx.lines(args![
            "I am Professor Hermes, in charge of practical examinations.",
            "Are you a candidate for the Sage class?"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Hermes Tris",
            args!["Register your application and take the written test first."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if (ctx.var("sage_q").get()? == 4 || ctx.var("sage_q").get()? == 5) {
            ctx.lines(args![
                "I am professor Hermes, in charge of practical examinations.",
                "Are you a candidate for the Sage class?"
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Hermes Tris",
                args![
                    "Go pass the written test with Professor Claytos first.",
                    "Then I will take care of you."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("sage_q").get()? == 6 {
                ctx.lines(args![
                    "Welcome, you just passed the written test, didn't you?",
                    "Now it's time for the practical examination."
                ])?;
                ctx.next()?;
                ctx.lines_as(
                    "Hermes Tris",
                    args![
                        "There is nothing difficult or special about this test.",
                        "All you have to do is kill all the monsters within the time limit."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Hermes Tris",
                    args![
                        "It's better to experience this for yourself, rather than be told about this test 100 times.",
                        "How about it? Are you ready to take this test?"
                    ],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("Yes, I am.:Sorry, give me some time.")],
                )?) == 1
                {
                    ctx.var("sage_q").set(Val::from(7))?;
                    ctx.lines_as(
                        "Hermes Tris",
                        args!["Good, let's start immediately.", "Do your best and come back safely!"],
                    )?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("job_sage"), Val::from(50), Val::from(154)])?;
                    return Err(Stop::End);
                }
                ctx.var("sage_q").set(Val::from(7))?;
                ctx.lines_as(
                    "Hermes Tris",
                    args!["Yes, you don't need to hurry... take your time and come back."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("sage_q").get()? == 7 {
                    ctx.lines(args![
                        "Welcome again! So, did you fully prepare yourself this time?",
                        "Oh well, it's not that hard. Give it your all, okay?"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as("Hermes Tris", args!["Are you ready?"])?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("Yes, I am.:Sorry, give me some time.")],
                    )?) == 1
                    {
                        ctx.lines_as(
                            "Hermes Tris",
                            args!["Good, let's start immediately.", "Do your best and come back safely!"],
                        )?;
                        ctx.close_window()?;
                        ctx.call(Function::Warp, vec![Val::from("job_sage"), Val::from(50), Val::from(154)])?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        "Hermes Tris",
                        args!["Yes, you don't need to hurry... take your time and come back."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("sage_q").get()? == 8 {
                        ctx.lines(args![
                            "Good job~ Since you passed the practical examination as well...",
                            "I'll accept your admission."
                        ])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Hermes Tris",
                            args![
                                "Now I need to decide what subject you will learn and study...",
                                "Let's see... let me check your written test grade and the time spent on the practical examination."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Hermes Tris", args!["Hmm, hmm... I see.", "Well... I think you're okay."])?;
                        ctx.next()?;
                        let subject1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                        if subject1 == 1 {
                            ctx.var("sage_q").set(Val::from(9))?;
                            ctx.call(Function::ChangeQuest, vec![Val::from(2046), Val::from(2047)])?;
                            ctx.lines_as(
                                "Hermes Tris",
                                args![
                                    "Now, you will study Yggdrasil.",
                                    "Yggdrasil is the tree that was rumored to be the source of life for this world."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Hermes Tris", args!["That is a good subject which helps us to recognize changes in the world, as well as the direction of its improvement.", "Go ask for help from Professor Saphien. He's in the Lecture Room."])?;
                            ctx.next()?;
                            ctx.lines_as("Hermes Tris", args!["I wish you luck."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if subject1 == 2 {
                            ctx.var("sage_q").set(Val::from(11))?;
                            ctx.call(Function::ChangeQuest, vec![Val::from(2046), Val::from(2048)])?;
                            ctx.lines_as("Hermes Tris", args!["Now, you will study monsters.", "The purpose of this study is to learn and understand more about creatures existing all over the continent."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Hermes Tris",
                                args![
                                    "This is a good subject which will help you lead your life as a well-experienced Sage.",
                                    "Go ask for help from Professor Lucius. He's in the Monster Museum."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Hermes Tris", args!["I wish you luck."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if subject1 == 3 {
                            ctx.var("sage_q").set(Val::from(13))?;
                            ctx.call(Function::ChangeQuest, vec![Val::from(2046), Val::from(2049)])?;
                            ctx.lines_as(
                                "Hermes Tris",
                                args![
                                    "Now, you will study magic skills that have certain properties.",
                                    "The purpose of this study is to better understand basic magic skills that we use in everyday life."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Hermes Tris",
                                args![
                                    "That is a good subject which helps you to deeply understand of the truth of magic.",
                                    "Go ask Professor Aebecee for help...He's in the Somatology Laboratory."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Hermes Tris", args!["I wish you luck."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    } else if ctx.var("sage_q").get()? == 9 {
                        ctx.lines(args![
                            "Huh? Didn't you understand what I said?",
                            "I told you to study Yggdrasil."
                        ])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Hermes Tris",
                            args!["Go ask for help from Professor Saphien. He's in the Lecture Room."],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("sage_q").get()? == 11 {
                        ctx.lines(args![
                            "Huh? Didn't you understand what I said?",
                            "I told you to study monsters."
                        ])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Hermes Tris",
                            args!["Go ask for help from Professor Lucius. He's in the Monster Museum."],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("sage_q").get()? == 13 {
                        ctx.lines(args![
                            "Huh? Didn't you understand what I said?",
                            "I told you to study magic spells that possess certain properties."
                        ])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Hermes Tris",
                            args!["Go ask a help from Professor Aebecee. He's in the Somatology Laboratory."],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("sage_q").get()? == 15 {
                        ctx.lines(args![
                            "What are you doing here? Aren't you supposed to be with Dean Kayron?",
                            "Oh well, there's no harm in showing me your dissertation though..."
                        ])?;
                        ctx.next()?;
                        ctx.lines_as("Hermes Tris", args!["But then again, maybe there is. Go see Dean Kayron."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines(args![
                            "Oh sorry, I'm quite busy at the moment...",
                            "If you have any questions, go visit the professor I've assigned to you."
                        ])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
            }
        }
    }
    Ok(Val::from(0))
}

pub fn practical_examination_p(ctx: &Ctx) -> Script {
    practical_examination_p_body(ctx, Vec::new()).map(|_| ())
}

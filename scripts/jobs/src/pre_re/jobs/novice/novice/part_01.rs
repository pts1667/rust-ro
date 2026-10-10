use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn bulletin_board_nv_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args![
        "^FF0000=================================^000000",
        "^FF0000 ^000000 ^E40CAA[Welcome]^CC0000 to ^FF9000Novice^7FFF00 Training ^00FF00Grounds ^E40CAA[Welcome]^FF0000^000000",
        "^FF0000=================================^000000"
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn bulletin_board_nv(ctx: &Ctx) -> Script {
    bulletin_board_nv_body(ctx, Vec::new()).map(|_| ())
}

fn guard_nv1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Training Grounds Guard",
        args![
            "Welcome to the Training Grounds.",
            "You are now in the outer court yard. Please go inside the castle to begin your training."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn guard_nv1(ctx: &Ctx) -> Script {
    guard_nv1_body(ctx, Vec::new()).map(|_| ())
}

fn guard_nv2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.mes("[Training Grounds Guard]")?;
    if ctx.call(Function::Rand, vec![Val::from(2)])?.is_true() {
        ctx.lines(args!["Come in!", "I would like", "to welcome you to", "the Training Grounds!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Training Grounds Guard",
            args![
                "In here, you can prepare",
                "yourself for your future",
                "adventures throughout the",
                "Ragnarok world!"
            ],
        )?;
    } else {
        ctx.lines(args![
            "Go, Novice, go!",
            "Fight, and grow stronger! Look towards a brighter tomorrow!"
        ])?;
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn guard_nv2(ctx: &Ctx) -> Script {
    guard_nv2_body(ctx, Vec::new()).map(|_| ())
}

fn receptionist_nv1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_charname_s = Val::from("");
    ctx.lines_as(
        "Training Grounds Receptionist",
        args!["Hello, you look to be new here.", "What is your name?"],
    )?;
    ctx.next()?;
    let (input, status) = runtime::input_text(ctx, None, None)?;
    l_charname_s = input;
    if !l_charname_s
        .clone()
        .loosely_equals(&ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
    {
        ctx.lines_as(
            "Training Grounds Receptionist",
            args!["Sorry, but I don't think I heard", "you correctly"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Training Grounds Receptionist",
        args!["Welcome!", "You are at the entrance", "of the ^3355FFTraining Grounds^000000."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Training Grounds Receptionist",
        args![
            "If you're new",
            "to the Ragnarok world,",
            "please choose the",
            "^3355FFTraining Grounds Introduction^000000",
            "menu for more information."
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
                &[Val::from(
                    "Apply for training.:Direct access to Ragnarok Online.:^3355FFTraining Grounds Introduction.^000000:I need a moment to think.",
                )],
            )? {
                1 => {
                    ctx.lines_as("Training Grounds Receptionist", args!["Thank you for applying for Novice training. For detailed information of each training course, please inquire the Guides for assistance."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Training Grounds Receptionist",
                        args!["When you have questions about the training course process, please feel free to ask any of the Tutors."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Training Grounds Receptionist",
                        args!["You will now be transferred", "to the Training Grounds."],
                    )?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("new_1-2"), Val::from(100), Val::from(70)])?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines_as(
                        "Training Grounds Receptionist",
                        args!["I understand.", "Please do your", "best, and I wish you", "the best of luck!"],
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
                    let subject3 = ctx.call(Function::Rand, vec![Val::from(6)])?;
                    if subject3 == 0 {
                        ctx.call(
                            Function::SavePoint,
                            vec![Val::from("prontera"), Val::from(273), Val::from(354), Val::from(1), Val::from(1)],
                        )?;
                        ctx.call(Function::Warp, vec![Val::from("prontera"), Val::from(273), Val::from(354)])?;
                    } else if subject3 == 1 {
                        ctx.call(
                            Function::SavePoint,
                            vec![Val::from("morocc"), Val::from(160), Val::from(94), Val::from(1), Val::from(1)],
                        )?;
                        ctx.call(Function::Warp, vec![Val::from("morocc"), Val::from(160), Val::from(94)])?;
                    } else if subject3 == 2 {
                        ctx.call(
                            Function::SavePoint,
                            vec![Val::from("geffen"), Val::from(120), Val::from(100), Val::from(1), Val::from(1)],
                        )?;
                        ctx.call(Function::Warp, vec![Val::from("geffen"), Val::from(120), Val::from(100)])?;
                    } else if subject3 == 3 {
                        ctx.call(
                            Function::SavePoint,
                            vec![Val::from("payon"), Val::from(70), Val::from(100), Val::from(1), Val::from(1)],
                        )?;
                        ctx.call(Function::Warp, vec![Val::from("payon"), Val::from(70), Val::from(100)])?;
                    } else if subject3 == 4 {
                        ctx.call(
                            Function::SavePoint,
                            vec![Val::from("alberta"), Val::from(116), Val::from(57), Val::from(1), Val::from(1)],
                        )?;
                        ctx.call(Function::Warp, vec![Val::from("alberta"), Val::from(116), Val::from(57)])?;
                    } else if subject3 == 5 {
                        ctx.call(
                            Function::SavePoint,
                            vec![Val::from("izlude"), Val::from(94), Val::from(103), Val::from(1), Val::from(1)],
                        )?;
                        ctx.call(Function::Warp, vec![Val::from("izlude"), Val::from(94), Val::from(103)])?;
                    }
                    return Err(Stop::End);
                }
                3 => {
                    ctx.lines_as("Training Grounds Receptionist", args!["This training grounds was established in order to provide useful information to new players of Ragnarok Online by the Rune-Midgarts Kingdom's Board of Education."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Training Grounds Receptionist",
                        args!["The training course is organized into two parts: the Basic Knowledge classes, and Field Combat training."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Training Grounds Receptionist",
                        args!["Through the first course, players will learn the necessary knowledge for a smoother gaming experience."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Training Grounds Receptionist",
                        args![
                            "In Field Combat Training,",
                            "players will engage in actual battle with weak monsters so they can learn the basics of fighting."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Training Grounds Receptionist",
                        args![
                            "With this battle practice,",
                            "players will be able to gain more experience before they enter the real world."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Training Grounds Receptionist", args!["At the end of the training, we will provide an introduction to the 1st Job Classes. This will help players decide which job class is best for them."])?;
                    ctx.next()?;
                    ctx.lines_as("Training Grounds Receptionist", args!["If you wish to participate in the training grounds, please choose '^3355FFApply for training^000000' in the menu."])?;
                    ctx.next()?;
                    ctx.lines_as("Training Grounds Receptionist", args!["Otherwise, if you want to skip the basic training and immediately enter the world of Ragnarok Online, please choose '^3355FFDirect access to Ragnarok Online^000000.'"])?;
                    ctx.next()?;
                }
                4 => {
                    ctx.lines_as(
                        "Training Grounds Receptionist",
                        args!["I understand.", "Please, take your time."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        }
    }
    Ok(Val::from(0))
}

pub fn receptionist_nv1(ctx: &Ctx) -> Script {
    receptionist_nv1_body(ctx, Vec::new()).map(|_| ())
}

fn shion_nv1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("nov_get_item04").get()?.number()? > 9 || ctx.var("nov_get_item05").get()?.number()? > 9) {
        ctx.lines_as(
            "Shion",
            args![
                "Hm...?",
                "What are you",
                "still doing here?",
                "Oh, you used a ^3355FFButterfly Wing^000000, didn't you?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Shion", args!["No, no, no~", "You're supposed to use the Butterfly Wing when you want to go back to a town ^666666after^000000 completing your training here, alright?"])?;
        ctx.next()?;
        ctx.lines_as("Shion", args!["Now, let me send", "you back to the", "Training Grounds."])?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("new_1-2"), Val::from(99), Val::from(99)])?;
        return Err(Stop::End);
    } else if ctx.var("nov_1st_cos").get()?.number()? > 2 {
        ctx.lines_as(
            "Shion",
            args![
                "The Training Grounds",
                "are located just past",
                "the bridge located",
                "to the right."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Shion",
            args![
                "Although you'll",
                "be sitting through",
                "some classes, you",
                "won't regret it.",
                "Now, go for it!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("nov_1st_cos").get()? == 2 {
        ctx.lines_as("Shion", args!["Hey...", "You little rascal!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Shion",
            args![
                "Wait...",
                "Calm down Shion.",
                "You're a professional",
                "trainer! Don't get all",
                "upset at a Novice!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Shion", args!["Go and cross the bridge to the right, right now! ^666666*Ahem*^000000 You'll see and castle, and inside you can meet all sorts of tutors."])?;
        ctx.next()?;
        ctx.lines_as("Shion", args!["If you can't see the entrance, just change your in-game camera angle by holding down the ^3355FFright Mouse button^000000 and dragging your mouse. Easy, right?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Shion",
            args![
                "To reset your camera angle,",
                "just double-click the right Mouse button. Okay then, take care!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Shion",
            args![
                "Oh, and before you leave,",
                "learn how to treat a lady nice, okay? Then they might give you gifts like this!"
            ],
        )?;
        ctx.var("nov_1st_cos").set(Val::from(3))?;
        ctx.call(Function::GetExperience, vec![Val::from(9), Val::from(0)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("nov_1st_cos").get()? == 1 {
        ctx.lines_as(
            "Shion",
            args!["Huh...?", "Why are you", "still here?", "^666666*Sigh...*^000000"],
        )?;
        ctx.next()?;
        ctx.lines_as("Shion", args!["Hey, when you enter the Training Grounds, you'll learn all sorts of things that will help you play the game. You'll even have the chance to get zeny and other rewards."])?;
        ctx.next()?;
        ctx.lines_as("Shion", args!["You can even gain", "experience like this!"])?;
        ctx.var("nov_1st_cos").set(Val::from(3))?;
        ctx.call(Function::GetExperience, vec![Val::from(9), Val::from(0)])?;
        ctx.next()?;
        ctx.lines_as(
            "Shion",
            args!["Everything you'll learn here in the Training Grounds will benefit your gameplay. So just think positive, okay?"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as("Shion", args!["Hello there~", "Welcome to the", "Training Grounds!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Shion",
            args![
                "Let's see.",
                "Your name is...",
                ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("."))
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Shion",
            args!["My name is Shion.", "Yes, this is the first time we've met, of course. Hahahaha~!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Shion",
            args![
                "Now that we've met, is there anything I can help you with?",
                "I'm here for your questions~"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(
            ctx,
            &[Val::from("Where should I go?:About Basic Interfaces.:Who the crap are you?")],
        )? {
            1 => {
                ctx.lines_as(
                    "Shion",
                    args![
                        "Do you see the bridge to your",
                        "right side? Just cross the bridge and you'll arrive at a castle. All you have to do is walk inside!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Shion",
                    args![
                        "The entrance of the castle",
                        "is a ^4D4DFFspinning white light^000000. These portals are what allow you to move from one zone to another."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Shion",
                    args![
                        "Do you know how to move?",
                        "Left click on a spot, and you'll walk over to that spot. Piece of cake, huh?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Shion",
                    args![
                        "So go for it!",
                        "Basically, you must enter the castle in order to start your adventures."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Shion",
                    args![
                        "There are soldiers",
                        "at the entrance, so don't",
                        "worry about getting lost.",
                        "Take care now~!"
                    ],
                )?;
                ctx.var("nov_1st_cos").set(Val::from(1))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Shion",
                    args!["Basic Interfaces...", "Do you know what Click, Double-click and Drag mean?"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Shion",
                    args![
                        "When you press the",
                        "left Mouse button once,",
                        "that is a click. When you press the mouse button twice in a row, that's a double-click."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Shion",
                    args![
                        "Dragging is when you move your Mouse while holding down the",
                        "Mouse button after clicking on something."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Shion",
                    args![
                        "Before we start talking about",
                        "the Basic Interfaces, you should remember these terms, just because we'll be using them frequently."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Shion", args!["Inside the castle, there is a Basic Interfaces Tutor who can teach you the basics more clearly, okay? Enter the castle to start your training."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Shion",
                    args!["The entrance", "of the castle is", "a ^4D4DFFspinning white light^000000."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Shion",
                    args![
                        "There are soldiers",
                        "at the entrance, so don't",
                        "worry about getting lost.",
                        "Take care now~!"
                    ],
                )?;
                ctx.var("nov_1st_cos").set(Val::from(1))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            3 => {
                ctx.lines_as("Shion", args!["Me? I'm Shion!", "But that's a rude way of asking! I'm volunteering my time and effort here, so you've got to show me a little bit of respect at least!"])?;
                ctx.var("nov_1st_cos").set(Val::from(2))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    Ok(Val::from(0))
}

pub fn shion_nv1(ctx: &Ctx) -> Script {
    shion_nv1_body(ctx, Vec::new()).map(|_| ())
}

fn interfaces_tutor_nv1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ((ctx.var("nov_get_item02").get()?.number()? > 9 && ctx.var("nov_get_item03").get()?.number()? > 9)
        && ctx.var("nov_get_item04").get()?.number()? > 9)
    {
        ctx.lines_as("Kris", args!["You've completed all the essential courses. Have you spoken to the assistant tutors already? The field combat training will be your next course. Would you like to proceed?"])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Sure!:No, I'll come back later.:Send me to a town!")])? {
            1 => {
                ctx.lines_as("Kris", args!["Your next course is Field Combat training. Please listen carefully to your next trainer, and I hope you pass the course. Godspeed."])?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("new_1-2"), Val::from(28), Val::from(178)])?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as("Kris", args!["Alright then. In the meantime, you might want to speak to the assistant tutors, as the basic information taught in the essential courses may not be enough for new adventurers."])?;
                ctx.next()?;
                ctx.lines_as("Kris", args!["Feel free to come back any time when you need my assistance."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            3 => {
                ctx.lines_as("Kris", args!["So, would you like to be sent to a town? If you're confident that you've learned enough, head over to the right and speak to the ^3355FFKafra Employee^000000."])?;
                ctx.next()?;
                ctx.lines_as("Kris", args!["The Kafra Services are very convenient once you get out into the real world. Their Teleport Service can be used to travel from town to town, and you can keep your items safe in the Kafra Storage."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Kris",
                    args!["We may never meet again, but I hope you grow stronger and become a great adventurer. Godspeed."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if ctx.var("nov_get_item02").get()?.number()? < 10 {
        ctx.lines_as("Kris", args!["Hello, may I see your", "proof of registration?"])?;
        ctx.next()?;
        if ctx.var("new_mes_flag0").get()?.is_true() {
            ctx.lines_as("Kris", args![((Val::from("Ah, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(", you've applied for an old training course that we no longer provide for our trainees. Let me issue a new proof of registration for you."))])?;
            ctx.var("new_mes_flag0").set(Val::from(0))?;
            ctx.var("new_mes_flag1").set(Val::from(0))?;
            ctx.var("new_mes_flag2").set(Val::from(0))?;
            ctx.var("new_mes_flag3").set(Val::from(0))?;
            ctx.var("new_mes_flag4").set(Val::from(0))?;
            ctx.var("new_mes_flag5").set(Val::from(0))?;
            ctx.var("new_lvup0").set(Val::from(0))?;
            ctx.var("new_lvup1").set(Val::from(0))?;
            ctx.var("new_joblvup").set(Val::from(0))?;
            ctx.next()?;
        }
        ctx.lines_as(
            "Kris",
            args![
                "Okay, now",
                "you're ready to go.",
                "In my class, I teach the",
                "use of the most basic",
                "interfaces."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kris",
            args![
                (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(".")),
                "would you like to learn",
                "more about interface",
                "fundamentals?"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Yes.:Nah, I'm a pro~:Cancel.")])? {
            1 => {
                ctx.lines_as("Kris", args!["First, it's possible to move every interface window on your screen by dragging the window. Just click on the window, hold down the mouse button and move your mouse."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Kris",
                    args!["Now, let me explain each interface window according to their default positions on your screen."],
                )?;
                ctx.next()?;
                ctx.lines_as("Kris", args!["At the upper left side of your screen, you will see a window with your character name and level. This is the ^3355FFBasic Information Window^000000."])?;
                ctx.next()?;
                ctx.mes("[Kris]")?;
                if ctx.var("BaseLevel").get()?.number()? < 8 {
                    ctx.lines(args![
                        "Let me give you",
                        "some experience points.",
                        "Keep an eye on your Basic Info Window and observe the change in your Base Level experience gauge."
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kris",
                        args![
                            "Did you see...?",
                            "As you gain experience,",
                            "the experience gauge fills up.",
                            "Once it is 100 % full, you gain an experience level, and the gauge is reset to 0."
                        ],
                    )?;
                    ctx.var("nov_get_item02").set(Val::from(10))?;
                    let subject3 = ctx.var("BaseLevel").get()?;
                    if subject3 == 1 {
                        ctx.call(Function::GetExperience, vec![Val::from(10), Val::from(0)])?;
                    } else if subject3 == 2 {
                        ctx.call(Function::GetExperience, vec![Val::from(17), Val::from(0)])?;
                    } else if subject3 == 3 {
                        ctx.call(Function::GetExperience, vec![Val::from(26), Val::from(0)])?;
                    } else if subject3 == 4 {
                        ctx.call(Function::GetExperience, vec![Val::from(37), Val::from(0)])?;
                    } else if subject3 == 5 {
                        ctx.call(Function::GetExperience, vec![Val::from(78), Val::from(0)])?;
                    } else if subject3 == 6 {
                        ctx.call(Function::GetExperience, vec![Val::from(115), Val::from(0)])?;
                    } else if subject3 == 7 {
                        ctx.call(Function::GetExperience, vec![Val::from(155), Val::from(0)])?;
                    }
                } else {
                    ctx.lines(args![
                        "But...",
                        "I guess you're already familiar with the Base Level experience gauge."
                    ])?;
                }
                ctx.next()?;
                ctx.lines_as("Kris", args!["At the bottom of the Basic Info Window, you will see two different experience gauge bars. The top bar is for your current Base Level, and the bottom one displays experience for your current Job Level."])?;
                ctx.next()?;
                ctx.lines_as("Kris", args!["When the Job Level", "Experience bar is filled, you will earn a Job Level, and a ^3355FFSkill Point^000000. Skill Points are spent to learn skills for your character."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Kris",
                    args![
                        "On the right side",
                        "of the Basic Info window,",
                        "you will see various",
                        "Menu buttons."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Kris",
                    args![
                        "Clicking these Menu buttons will open other Interface Windows, such as the Inventory Window",
                        "or Party Window."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Kris",
                    args![
                        "Now...",
                        "The ^3355FFChat Window^000000 is",
                        "located at the bottom",
                        "of your screen."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Kris", args!["At the bottom right of the Chat Window, you should see 2 blue buttons. The left button allows you to change your chatting options."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Kris",
                    args![
                        "The '^3355FFSend to All^000000' option",
                        "allows you to chat with",
                        "everyone on your screen."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Kris", args!["The '^3355FFSend to Party^000000' and '^3355FFSend to Guild^000000' options allows you to send messages to only members of your party or guild, regardless of how far they are."])?;
                ctx.next()?;
                ctx.lines_as("Kris", args!["You can drag the Scroll Bar", "on the right side of the Chat Window to review a conversation. Since the Chat Window is always active, you won't have any problem communicating with other players."])?;
                ctx.next()?;
                ctx.lines_as("Kris", args!["Now, one of the most important interfaces is the ^3355FFMini-Map^000000, located at the upper-right of your screen."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Kris",
                    args![
                        "The red dots on the Mini-Map indicate locations of ^3355FFWarp Portals^000000 which connect to different zones."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Kris", args!["If you've joined a party or a guild, the Mini-Map will also show you the location of your party or guild members if they are on the same map."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Kris",
                    args![
                        "Please click the Menu buttons",
                        "on the right side of your Basic Info window and familiarize yourself with the other interfaces."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Kris", args!["Well, that was my brief overview on in-game interfaces. It might seem like a lot of information now, but it will soon become second nature."])?;
                if ctx.var("JobLevel").get()?.number()? < 7 {
                    ctx.next()?;
                    ctx.lines_as("Kris", args!["Let me give you a little bit of Job experience points. Open your Skill Window and distribute your Skill Points into ^3355FFBasic Skills^000000."])?;
                    ctx.var("nov_get_item02").set(Val::from(11))?;
                    let subject4 = ctx.var("JobLevel").get()?;
                    if subject4 == 1 {
                        ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(10)])?;
                    } else if subject4 == 2 {
                        ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(18)])?;
                    } else if subject4 == 3 {
                        ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(28)])?;
                    } else if subject4 == 4 {
                        ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(40)])?;
                    } else if subject4 == 5 {
                        ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(91)])?;
                    } else if subject4 == 6 {
                        ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(151)])?;
                    }
                } else {
                    ctx.lines_as(
                        "Kris",
                        args!["Your Job Level is much higher than I had expected. You must already know the basic information by now."],
                    )?;
                }
                ctx.next()?;
                ctx.lines_as("Kris", args!["Now, why don't you speak to Edwin? He will teach you more regarding the basic use of Skills. Ah, and let me give you a small present: a Tattered Novice Ninja Suit!"])?;
                ctx.var("nov_get_item02").set(Val::from(12))?;
                ctx.call(Function::GetItem, vec![Val::from(2352), Val::from(1)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Kris",
                    args![
                        "Let me guide you",
                        "to the Field Combat",
                        "Training Course.",
                        "You can come back any time if you feel that you need a review."
                    ],
                )?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("new_1-2"), Val::from(28), Val::from(178)])?;
                return Err(Stop::End);
            }
            3 => {
                ctx.call(Function::Emotion, vec![ctx.constant("ET_PROFUSELY_SWEAT")?])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if ctx.var("nov_get_item03").get()?.number()? < 10 {
        ctx.lines_as("Kris", args!["How may I help you?", "Can I see your proof of registration?"])?;
        ctx.next()?;
        ctx.lines_as("Kris", args!["It seems that you haven't attended the Skill Information class yet. Please talk to a tutor to the very left of this room to attend his class."])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Thank you!:I'm tired of classes~:Cancel")])? {
            1 => {
                ctx.lines_as(
                    "Kris",
                    args!["When you attend the Skill Information class, you'll gain a better understanding of the use of skills."],
                )?;
                ctx.next()?;
                ctx.lines_as("Kris", args!["Since the use of skills is integral to survival in Midgard, I strongly suggest that you attend the class. Come, I shall guide you there."])?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("new_1-2"), Val::from(84), Val::from(107)])?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Kris",
                    args![
                        "I see. In that case, you must be ready for the Field Combat Training Course. Shall I send you there right away?"
                    ],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(ctx, &[Val::from("No! W-wait!:Please do~!")])?) == 1 {
                    ctx.lines_as(
                        "Kris",
                        args!["...?!", "O...kay then.", "Please come back", "when you're ready."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as("Kris", args!["Godspeed,", "young Novice."])?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("new_1-2"), Val::from(28), Val::from(178)])?;
                    return Err(Stop::End);
                }
            }
            3 => {
                ctx.call(Function::Emotion, vec![ctx.constant("ET_PROFUSELY_SWEAT")?])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if ctx.var("nov_get_item04").get()?.number()? < 10 {
        ctx.lines_as("Kris", args!["How may I help you?", "Can I see your proof of registration?"])?;
        ctx.next()?;
        ctx.lines_as("Kris", args!["It looks like you still haven't attended the Item Information class yet. Please speak to the tutor to the very right of this room to attend her class."])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Thank you.:I'm tired of classes~:Cancel")])? {
            1 => {
                ctx.lines_as("Kris", args!["The Item Information class is very useful for you to learn how to use your Hot keys and Hot key bars. Come, let me guide you there."])?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("new_1-2"), Val::from(115), Val::from(107)])?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Kris",
                    args![
                        "I see. In that case, you must be ready for the Field Combat Training Course. Shall I send you there right away?"
                    ],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(ctx, &[Val::from("No! W-wait!:Please do~!")])?) == 1 {
                    ctx.lines_as(
                        "Kris",
                        args!["...?!", "O...kay then.", "Please come back", "when you're ready."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as("Kris", args!["Godspeed,", "young Novice."])?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("new_1-2"), Val::from(28), Val::from(178)])?;
                    return Err(Stop::End);
                }
            }
            3 => {
                ctx.call(Function::Emotion, vec![ctx.constant("ET_PROFUSELY_SWEAT")?])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    Ok(Val::from(0))
}

pub fn interfaces_tutor_nv1(ctx: &Ctx) -> Script {
    interfaces_tutor_nv1_body(ctx, Vec::new()).map(|_| ())
}

fn skill_tutor_nv_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ((ctx.var("nov_get_item02").get()?.number()? > 9 && ctx.var("nov_get_item03").get()?.number()? > 9)
        && ctx.var("nov_get_item04").get()?.number()? > 9)
    {
        ctx.lines_as(
            "Cecil",
            args![
                "Huh...?",
                "Did you need more help?",
                "I see that you've completed all the essential courses. Did you speak to the assistant tutors too?"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(
            ctx,
            &[Val::from("Send me to the next course!:Assistant tutors?:Take me to a town!")],
        )? {
            1 => {
                ctx.lines_as(
                    "Cecil",
                    args![
                        "Ah! Right, right.",
                        "You've got to take on the Field Combat Training Course sometime, I suppose."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Cecil", args!["Man, I'm so jealous of the instructors in the Field Combat Training Course. Teaching basic information is just soooo not as cool as beating stuff up."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Cecil",
                    args!["Ah, right.", "Field Combat.", "I'm sending you now.", "Good luck, kid!"],
                )?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("new_1-2"), Val::from(28), Val::from(178)])?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as("Cecil", args!["You know about the", "assistant tutors, don't you?"])?;
                ctx.next()?;
                ctx.lines_as("Cecil", args!["Listen. The three of tutors in this room only teach the most basic information. The courses we teach are meant to be passed quickly."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Cecil",
                    args!["But some people might benefit a little bit more if they learned some more detailed information."],
                )?;
                ctx.next()?;
                ctx.lines_as("Cecil", args!["If you're completely new to Ragnarok, it couldn't hurt to attend the classes held by the assistant tutors at least once."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Cecil",
                    args!["A guy named Leo Handerson seems to know a lot about skills, so I think his knowledge would be useful to you."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            3 => {
                ctx.lines_as(
                    "Cecil",
                    args!["A town...?", "What do I look like, your own personal Peco Peco?"],
                )?;
                ctx.next()?;
                ctx.lines_as("Cecil", args!["That's right, you might be too young to know about that. Listen, if you want to move to a town, speak to the Kafra Lady to the right, okay?"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    if ctx.var("nov_get_item03").get()?.number()? < 10 {
        ctx.lines_as(
            "Cecil",
            args![
                ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("")),
                "Heh, I like your name!"
            ],
        )?;
        if ctx.var("new_mes_flag0").get()?.is_true() {
            ctx.lines(args![
                "By the way, your proof of registration has expired, so let me give you a new one.",
                "Let me give you a new one."
            ])?;
            ctx.var("new_mes_flag0").set(Val::from(0))?;
            ctx.var("new_mes_flag1").set(Val::from(0))?;
            ctx.var("new_mes_flag2").set(Val::from(0))?;
            ctx.var("new_mes_flag3").set(Val::from(0))?;
            ctx.var("new_mes_flag4").set(Val::from(0))?;
            ctx.var("new_mes_flag5").set(Val::from(0))?;
            ctx.var("new_lvup0").set(Val::from(0))?;
            ctx.var("new_lvup1").set(Val::from(0))?;
            ctx.var("new_joblvup").set(Val::from(0))?;
        }
        ctx.mes("Then, shall we begin the class?")?;
        ctx.next()?;
        'b2: {
            let subject2 = Val::from(runtime::select_values(
                ctx,
                &[Val::from("What do you teach?:I want Field Combat Training now!:Cancel")],
            )?);
            let mut matched2 = false;
            let no_case2 = !subject2.loosely_equals(&Val::from(1))
                && !subject2.loosely_equals(&Val::from(2))
                && !subject2.loosely_equals(&Val::from(3));
            if !matched2 && subject2.loosely_equals(&Val::from(1)) {
                matched2 = true;
            }
            if matched2 {
                ctx.lines_as(
                    "Cecil",
                    args![
                        "I live for power",
                        "and die for power!",
                        "I shall teach you",
                        "my famous fatal blow!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Cecil",
                    args![
                        "I'm just pulling your chain!",
                        "I actually just teach you how to use your skills. I still live for power, though."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Cecil", args!["In your Basic Info Window, click the ^3355FFSkill^000000 button to open your Skill Window. You can also press the '^3355FFAlt^000000' and '^3355FFS^000000' keys at the same time."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Cecil",
                    args![
                        "When your",
                        "Skill Window is open,",
                        "you'll see an icon labeled",
                        "'^3355FFBasic Skill^000000.'"
                    ],
                )?;
                if ctx.var("JobLevel").get()?.number()? < 7 {
                    ctx.next()?;
                    ctx.lines_as(
                        "Cecil",
                        args!["Now, at the bottom of the Skill Window, the number of remaining Skill Points that you have is displayed."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Cecil", args!["Open your Skill Window ('Alt' + 'S') and click the '^3355FFLv Up^000000' button next to the Basic Skill icon to allocate a Skill Point to your Basic Skills."])?;
                    ctx.var("nov_get_item03").set(Val::from(10))?;
                    let subject3 = ctx.var("JobLevel").get()?;
                    if subject3 == 1 {
                        ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(10)])?;
                    } else if subject3 == 2 {
                        ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(18)])?;
                    } else if subject3 == 3 {
                        ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(28)])?;
                    } else if subject3 == 4 {
                        ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(40)])?;
                    } else if subject3 == 5 {
                        ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(91)])?;
                    } else if subject3 == 6 {
                        ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(151)])?;
                    }
                } else {
                    ctx.next()?;
                    ctx.lines_as(
                        "Cecil",
                        args![
                            "Huh. Actually, your Job Level is higher that I thought. I guess you already know the basics about skills then."
                        ],
                    )?;
                }
                ctx.next()?;
                ctx.lines_as("Cecil", args!["So did you distribute the skill points to your Basic Skills? You'll need to master the Basic Skills eventually, so it's a good idea."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Cecil",
                    args!["For more detailed information on skills, go speak to Leo Handerson, one of the assistant tutors."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Cecil",
                    args![
                        "Oh, I almost forgot!",
                        "Let me teach you the ^3355FFFirst Aid^000000 skill. This skill will help you out a lot when you're in danger."
                    ],
                )?;
                ctx.next()?;
                ctx.lines(args!["^3355FFYou have learned", "the ^4A708BFirst Aid^3355FF skill.^000000"])?;
                ctx.call(
                    Function::Skill,
                    vec![Val::from("NV_FIRSTAID"), Val::from(1), ctx.constant("SKILL_PERM")?],
                )?;
                ctx.var("nov_sk").set(Val::from(3))?;
                ctx.var("nov_get_item03").set(Val::from(11))?;
                ctx.next()?;
                if ctx.var("JobLevel").get()?.number()? < 7 {
                    ctx.lines(args!["^3355FFYou have gained a small", "amount of Job experience.^000000"])?;
                    ctx.var("nov_get_item03").set(Val::from(12))?;
                    let subject4 = ctx.var("JobLevel").get()?;
                    if subject4 == 1 {
                        ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(10)])?;
                    } else if subject4 == 2 {
                        ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(18)])?;
                    } else if subject4 == 3 {
                        ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(28)])?;
                    } else if subject4 == 4 {
                        ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(40)])?;
                    } else if subject4 == 5 {
                        ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(91)])?;
                    } else if subject4 == 6 {
                        ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(151)])?;
                    }
                }
                ctx.next()?;
                ctx.lines_as("Cecil", args!["Now, open your Skill Window", "and check if you have the ^3355FFFirst Aid^000000 skill icon. To use it, you need to double-click that skill icon.", "Now, try it!"])?;
                ctx.call(Function::PercentHeal, vec![Val::from(-50), Val::from(0)])?;
                ctx.next()?;
                ctx.lines_as("Cecil", args!["Active skills, like First Aid, require a certain amount of SP to use them. The First Aid skill is useful for Novices, since it refills a little bit of HP."])?;
                ctx.next()?;
                ctx.lines_as("Cecil", args!["You've been", "a good student,", "so let me reward you!"])?;
                if ctx.var("BaseLevel").get()?.number()? < 8 {
                    ctx.mes("Behold: bonus experience!")?;
                    ctx.var("nov_get_item03").set(Val::from(13))?;
                    let subject5 = ctx.var("BaseLevel").get()?;
                    if subject5 == 1 {
                        ctx.call(Function::GetExperience, vec![Val::from(10), Val::from(0)])?;
                    } else if subject5 == 2 {
                        ctx.call(Function::GetExperience, vec![Val::from(17), Val::from(0)])?;
                    } else if subject5 == 3 {
                        ctx.call(Function::GetExperience, vec![Val::from(26), Val::from(0)])?;
                    } else if subject5 == 4 {
                        ctx.call(Function::GetExperience, vec![Val::from(37), Val::from(0)])?;
                    } else if subject5 == 5 {
                        ctx.call(Function::GetExperience, vec![Val::from(78), Val::from(0)])?;
                    } else if subject5 == 6 {
                        ctx.call(Function::GetExperience, vec![Val::from(115), Val::from(0)])?;
                    } else if subject5 == 7 {
                        ctx.call(Function::GetExperience, vec![Val::from(155), Val::from(0)])?;
                    }
                } else {
                    ctx.next()?;
                    ctx.lines_as(
                        "Cecil",
                        args!["Oh wait. You're much higher in level than I thought. Still, I'm proud of you!"],
                    )?;
                }
                ctx.next()?;
                ctx.lines_as("Cecil", args!["Well, that's it for the essential fundamentals. If you want a more comprehensive lesson, you gotta speak to the assistant tutors."])?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Okay.:Send me to Field Combat Training, now!:Cancel")])? {
                    1 => {
                        ctx.lines_as(
                            "Cecil",
                            args![
                                "Everyone in the",
                                "Training Grounds is",
                                "more than willing to",
                                "help you. Good luck!"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as(
                            "Cecil",
                            args![
                                "Heh heh~!",
                                "Alright, practice makes perfect! Let me send you to the guys at Field Combat Training. Take care!"
                            ],
                        )?;
                        ctx.close_window()?;
                        ctx.call(Function::Warp, vec![Val::from("new_1-2"), Val::from(28), Val::from(178)])?;
                        return Err(Stop::End);
                    }
                    3 => {
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            }
            if !matched2 && subject2.loosely_equals(&Val::from(2)) {
                matched2 = true;
            }
            if matched2 {
                ctx.lines_as(
                    "Cecil",
                    args![
                        "Heh heh~!",
                        "Alright, practice makes perfect! Let me send you to the guys at Field Combat Training. Take care!"
                    ],
                )?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("new_1-2"), Val::from(28), Val::from(178)])?;
                return Err(Stop::End);
            }
            if !matched2 && subject2.loosely_equals(&Val::from(3)) {
                matched2 = true;
            }
            if matched2 {
                ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    } else if ctx.var("nov_get_item02").get()?.number()? < 10 {
        ctx.lines_as(
            "Cecil",
            args![
                "So how may",
                "I help you?",
                "Whoa, you haven't attended the Basic Interface class yet? Oh well, I know that class is kinda boring~"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(
            ctx,
            &[Val::from("Oh, I better take that class.:Send me to Field Combat Training.:Cancel")],
        )? {
            1 => {
                ctx.lines_as("Cecil", args!["Yeah, that's a good idea. After all, you'll gain experience and items while you take that class. Alright then, the Interfaces Tutor is in the center of this room. Go for it~"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Cecil",
                    args![
                        "Heh heh~!",
                        "Alright, practice makes perfect! Let me send you to the guys at Field Combat Training. Take care!"
                    ],
                )?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("new_1-2"), Val::from(28), Val::from(178)])?;
                return Err(Stop::End);
            }
            3 => {
                ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if ctx.var("nov_get_item04").get()?.number()? < 10 {
        ctx.lines_as(
            "Cecil",
            args![
                "So how may",
                "I help you?",
                "Whoa, you haven't attended the Item Information class yet? Oh well, I know that class is kinda boring~"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(
            ctx,
            &[Val::from("Oh, I better take that class.:Send me to Field Combat Training.:Cancel")],
        )? {
            1 => {
                ctx.lines_as("Cecil", args!["Yeah, that's a good idea. After all, you'll gain experience and items while you take that class. Alright then, the Item Tutor is on the far right side of this room. Go for it~"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Cecil",
                    args![
                        "Heh heh~!",
                        "Alright, practice makes perfect! Let me send you to the guys at Field Combat Training. Take care!"
                    ],
                )?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("new_1-2"), Val::from(28), Val::from(178)])?;
                return Err(Stop::End);
            }
            3 => {
                ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    Ok(Val::from(0))
}

pub fn skill_tutor_nv(ctx: &Ctx) -> Script {
    skill_tutor_nv_body(ctx, Vec::new()).map(|_| ())
}

fn item_tutor_nv_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ((ctx.var("nov_get_item02").get()?.number()? > 9 && ctx.var("nov_get_item03").get()?.number()? > 9)
        && ctx.var("nov_get_item04").get()?.number()? > 9)
    {
        ctx.lines_as(
            "Alice",
            args![
                "Huh...?",
                "Do you need help looking for someone? You seem to have completed all of the essential courses. Where do you need to go?"
            ],
        )?;
        ctx.next()?;
        'b1: {
            let subject1 = Val::from(runtime::select_values(
                ctx,
                &[Val::from("I'm not sure~!:Send me to a town.:Cancel")],
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
                    "Alice",
                    args![
                        "Hmm...",
                        "You've learned everything else,",
                        "so I guess the only thing left is Field Combat Training. Did you want to attend that class now?"
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Yes.:Oh, w-wait.")])? {
                    1 => {
                        ctx.lines_as("Alice", args!["Make sure you keep the items I've given you handy, and that you equip all of your armor, alright? Now, take care."])?;
                        ctx.close_window()?;
                        ctx.call(Function::Warp, vec![Val::from("new_1-2"), Val::from(28), Val::from(178)])?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as("Alice", args!["Okay, no problem.", "Come to me when you", "need any help."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            }
            if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as(
                    "Alice",
                    args![
                        "Ah, I see.",
                        "If you want to move to a town, the Kafra Lady to the right of me will teleport you. Take care now~"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if !matched1 && subject1.loosely_equals(&Val::from(3)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as("Alice", args!["Hmpf...!"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    } else if ctx.var("nov_get_item04").get()?.number()? < 10 {
        ctx.lines_as(
            "Alice",
            args!["^666666*Yawn~*^000000", "This is so boring.", "Oh! Hello, you're new here."],
        )?;
        if ctx.var("new_mes_flag0").get()?.is_true() {
            ctx.mes("Ooh, your proof of registration was expired. But that's okay, I'll just give you a new one! There you go.")?;
            ctx.var("new_mes_flag0").set(Val::from(0))?;
            ctx.var("new_mes_flag1").set(Val::from(0))?;
            ctx.var("new_mes_flag2").set(Val::from(0))?;
            ctx.var("new_mes_flag3").set(Val::from(0))?;
            ctx.var("new_mes_flag4").set(Val::from(0))?;
            ctx.var("new_mes_flag5").set(Val::from(0))?;
            ctx.var("new_lvup0").set(Val::from(0))?;
            ctx.var("new_lvup1").set(Val::from(0))?;
            ctx.var("new_joblvup").set(Val::from(0))?;
        }
        ctx.lines(args!["So, have you come to attend", "my Item Information class?"])?;
        ctx.next()?;
        'b3: {
            let subject3 = Val::from(runtime::select_values(
                ctx,
                &[Val::from("Yes!:No, thanks.:How do I get to a town?")],
            )?);
            let mut matched3 = false;
            let no_case3 = !subject3.loosely_equals(&Val::from(1))
                && !subject3.loosely_equals(&Val::from(2))
                && !subject3.loosely_equals(&Val::from(3));
            if !matched3 && subject3.loosely_equals(&Val::from(1)) {
                matched3 = true;
            }
            if matched3 {
                ctx.lines_as("Alice", args!["Don't worry, it'll be short.", "Open your Inventory Window", "through either the '^3355FFitems^000000' button in the Basic Info window, or by pressing the '^3355FFAlt^000000' and '^3355FFE^000000' keys at the same time."])?;
                ctx.next()?;
                ctx.lines_as("Alice", args!["In the Inventory Window, you'll see 3 tabs labeled ^3355FFitem^000000, ^3355FFequip^000000 and ^3355FFetc^000000. Items that can be consumed are under the ^4A708Bitem^000000 tab."])?;
                ctx.next()?;
                ctx.lines_as("Alice", args!["Now, would you click the ^4A708Bitem^000000 tab in the Inventory Window? I just gave you a Novice Potion. You can drink it by double-clicking it. Go ahead, try it!"])?;
                ctx.var("nov_get_item04").set(Val::from(10))?;
                ctx.call(Function::GetItem, vec![Val::from(569), Val::from(1)])?;
                ctx.call(Function::PercentHeal, vec![Val::from(-50), Val::from(0)])?;
                ctx.next()?;
                ctx.mes("[Alice]")?;
                if ctx.call(Function::CountItem, vec![Val::from(569)])?.number()? < 1 {
                    if ctx.var("BaseLevel").get()?.number()? < 8 {
                        ctx.lines(args!["Nice~!", "And here's", "a little reward", "just for listening."])?;
                        ctx.var("nov_get_item04").set(Val::from(11))?;
                        let subject4 = ctx.var("BaseLevel").get()?;
                        if subject4 == 1 {
                            ctx.call(Function::GetExperience, vec![Val::from(10), Val::from(0)])?;
                        } else if subject4 == 2 {
                            ctx.call(Function::GetExperience, vec![Val::from(17), Val::from(0)])?;
                        } else if subject4 == 3 {
                            ctx.call(Function::GetExperience, vec![Val::from(26), Val::from(0)])?;
                        } else if subject4 == 4 {
                            ctx.call(Function::GetExperience, vec![Val::from(37), Val::from(0)])?;
                        } else if subject4 == 5 {
                            ctx.call(Function::GetExperience, vec![Val::from(78), Val::from(0)])?;
                        } else if subject4 == 6 {
                            ctx.call(Function::GetExperience, vec![Val::from(115), Val::from(0)])?;
                        } else if subject4 == 7 {
                            ctx.call(Function::GetExperience, vec![Val::from(155), Val::from(0)])?;
                        }
                    } else {
                        ctx.mes("Good job! I'd reward you with some more experience if your level weren't already this high.")?;
                    }
                } else {
                    ctx.lines(args!["Um...", "Well, you can drink", "it later I guess."])?;
                }
                ctx.next()?;
                ctx.lines_as(
                    "Alice",
                    args![
                        "Let me explain about",
                        "items in the ^4A708Bequip^000000 tab",
                        "of the Inventory Window."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Alice", args!["When you click on the ^4A708Bequip^000000 tab, you can view every item in your inventory that you can equip. Let me give you some equipment so that you can try them on."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Alice",
                    args![
                        "Got them? Good.",
                        "Now, double-click",
                        "on the Novice Slippers",
                        "I just gave you to",
                        "put them on."
                    ],
                )?;
                ctx.var("nov_get_item04").set(Val::from(12))?;
                ctx.call(Function::GetItem, vec![Val::from(2510), Val::from(1)])?;
                ctx.call(Function::GetItem, vec![Val::from(2414), Val::from(1)])?;
                ctx.call(Function::GetItem, vec![Val::from(5055), Val::from(1)])?;
                ctx.next()?;
                ctx.mes("[Alice]")?;
                if ctx.call(Function::IsEquipped, vec![Val::from(2414)])?.is_true() {
                    if ctx.var("BaseLevel").get()?.number()? < 8 {
                        ctx.lines(args!["Hooray~!", "You did it!", "You deserve a reward!"])?;
                        ctx.var("nov_get_item04").set(Val::from(13))?;
                        let subject5 = ctx.var("BaseLevel").get()?;
                        if subject5 == 1 {
                            ctx.call(Function::GetExperience, vec![Val::from(10), Val::from(0)])?;
                        } else if subject5 == 2 {
                            ctx.call(Function::GetExperience, vec![Val::from(17), Val::from(0)])?;
                        } else if subject5 == 3 {
                            ctx.call(Function::GetExperience, vec![Val::from(26), Val::from(0)])?;
                        } else if subject5 == 4 {
                            ctx.call(Function::GetExperience, vec![Val::from(37), Val::from(0)])?;
                        } else if subject5 == 5 {
                            ctx.call(Function::GetExperience, vec![Val::from(78), Val::from(0)])?;
                        } else if subject5 == 6 {
                            ctx.call(Function::GetExperience, vec![Val::from(115), Val::from(0)])?;
                        } else if subject5 == 7 {
                            ctx.call(Function::GetExperience, vec![Val::from(155), Val::from(0)])?;
                        }
                    } else {
                        ctx.mes("Good job! I'd reward you with some more experience if your level weren't already this high.")?;
                    }
                } else {
                    ctx.lines(args![
                        "Er...",
                        "You've got to",
                        "double-click",
                        "equipment to",
                        "wear it. Just",
                        "remember that, okay?"
                    ])?;
                }
                ctx.next()?;
                ctx.lines_as(
                    "Alice",
                    args![
                        "Would you",
                        "press the '^3355FFF12^000000' key?",
                        "This will summon your",
                        "Hotkey bar on your screen."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Alice", args!["You can assign hotkeys to your items, skills and equipment using the Hotkey bar. Just drag skill icons from the Skill Window or items from the Inventory Window into the Hotkey bar."])?;
                ctx.next()?;
                ctx.lines_as("Alice", args!["The Hotkeys are '^3355FFF1^000000' to '^3355FFF9^000000.'", "If you have attended the Skill Class, you must have been given the First Aid skill. Drag and drop the First Aid skill icon into the Hotkey bar."])?;
                ctx.next()?;
                ctx.lines_as("Alice", args!["For your information, only", "active skills can be assigned to a Hotkey and dragged to the Hotkey bar. Active Skills have colored, square shaped icons that can be double-clicked and used."])?;
                ctx.next()?;
                ctx.lines_as("Alice", args!["Passive Skills, such as the aptly named 'Basic Skill,' cannot be dragged into the Hotkey bar because Passive Skills are always in effect and don't need to be activated."])?;
                ctx.var("nov_get_item04").set(Val::from(14))?;
                if ctx.var("JobLevel").get()?.number()? < 7 {
                    let subject6 = ctx.var("JobLevel").get()?;
                    if subject6 == 1 {
                        ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(10)])?;
                    } else if subject6 == 2 {
                        ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(18)])?;
                    } else if subject6 == 3 {
                        ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(28)])?;
                    } else if subject6 == 4 {
                        ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(40)])?;
                    } else if subject6 == 5 {
                        ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(91)])?;
                    } else if subject6 == 6 {
                        ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(151)])?;
                    }
                }
                ctx.next()?;
                ctx.lines_as(
                    "Alice",
                    args![
                        "Well, that's it!",
                        "Let me supply you with some items",
                        "that will help you during the Field Combat Training."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Alice", args!["However, ^ff0000do not use the Fly Wing or Butterfly Wing^000000 in these Training Grounds or you could be stuck here forever. Those items are for when you graduate, okay?"])?;
                ctx.var("nov_get_item04").set(Val::from(15))?;
                ctx.call(Function::GetItem, vec![Val::from(601), Val::from(10)])?;
                ctx.call(Function::GetItem, vec![Val::from(602), Val::from(2)])?;
                ctx.call(Function::GetItem, vec![Val::from(569), Val::from(50)])?;
                ctx.next()?;
                ctx.lines_as("Alice", args!["And lastly..."])?;
                if ctx.var("JobLevel").get()?.number()? < 7 {
                    ctx.lines(args!["I will give", "you some Job experience!"])?;
                    ctx.var("nov_get_item04").set(Val::from(16))?;
                    let subject7 = ctx.var("JobLevel").get()?;
                    if subject7 == 1 {
                        ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(10)])?;
                    } else if subject7 == 2 {
                        ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(18)])?;
                    } else if subject7 == 3 {
                        ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(28)])?;
                    } else if subject7 == 4 {
                        ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(40)])?;
                    } else if subject7 == 5 {
                        ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(91)])?;
                    } else if subject7 == 6 {
                        ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(151)])?;
                    }
                } else {
                    ctx.mes("I was gonna give you some job experience points, but I think you have enough job experience for now.")?;
                }
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Now what?:Send me to the actual fighting class!:Cancel")])? {
                    1 => {
                        ctx.lines_as(
                            "Alice",
                            args!["Why don't you walk around", "and talk to other tutors if you haven't already?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Alice",
                            args![
                                "Everyone in this training grounds is more than willing to help you.",
                                "Maybe you can venture around",
                                "this area if you're bored."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Alice", args!["The assistant tutors in the room to the right possess useful knowledge. There are also a few interesting places hidden within this area. Good luck!"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as("Alice", args!["What an enthusiastic Novice you are! Okay, I'll send you to the folks in charge of Field Combat Training. Make sure that you listen carefully to the trainers."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Alice",
                            args![
                                "After all...",
                                "When you're fighting monsters, it's a matter of life and death! Alright then, take care~"
                            ],
                        )?;
                        ctx.close_window()?;
                        ctx.call(Function::Warp, vec![Val::from("new_1-2"), Val::from(28), Val::from(178)])?;
                        return Err(Stop::End);
                    }
                    3 => {
                        ctx.lines_as("Alice", args!["Hmpf!"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            }
            if !matched3 && subject3.loosely_equals(&Val::from(2)) {
                matched3 = true;
            }
            if matched3 {
                ctx.lines_as("Alice", args!["Are you sure you really want to go into Field Combat Training? Have you spoken to every tutor? You better do that beforehand."])?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("No, no! Send me to the actual fight class!:Oh, wait!")])? {
                    1 => {
                        ctx.lines_as("Alice", args!["What an enthusiastic Novice you are! Okay, I'll send you to the folks in charge of Field Combat Training. Make sure that you listen carefully to the trainers."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Alice",
                            args![
                                "After all...",
                                "When you're fighting monsters, it's a matter of life and death! Alright then, take care~"
                            ],
                        )?;
                        ctx.close_window()?;
                        ctx.call(Function::Warp, vec![Val::from("new_1-2"), Val::from(28), Val::from(178)])?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as("Alice", args!["Now, that's a good decision. You won't get any many chances to get free stuff and experience in the future. You better make the most of this opportunity while you can!"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            }
            if !matched3 && subject3.loosely_equals(&Val::from(3)) {
                matched3 = true;
            }
            if matched3 {
                ctx.lines_as(
                    "Alice",
                    args!["If you want to go to a town, ask the Kafra Employee to the right. Alright then, take care~"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    } else if ctx.var("nov_get_item02").get()?.number()? < 10 {
        ctx.lines_as(
            "Alice",
            args![
                "So how may",
                "I help you?",
                "Hmm, it seems that you haven't attended the Basic Interfaces class yet. Would you like to attend that class first?"
            ],
        )?;
        ctx.next()?;
        'b10: {
            let subject10 = Val::from(runtime::select_values(
                ctx,
                &[Val::from(
                    "I am going to attend that class.:Send me to Field Combat Training.:Cancel",
                )],
            )?);
            let mut matched10 = false;
            let no_case10 = !subject10.loosely_equals(&Val::from(1))
                && !subject10.loosely_equals(&Val::from(2))
                && !subject10.loosely_equals(&Val::from(3));
            if !matched10 && subject10.loosely_equals(&Val::from(1)) {
                matched10 = true;
            }
            if matched10 {
                ctx.lines_as("Cecil", args!["Excellent~", "You'll learn some essential stuff and gain experience and items as you take that class. The tutor for Basic Interfaces is in the center of this room. Now, go for it~"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if !matched10 && subject10.loosely_equals(&Val::from(2)) {
                matched10 = true;
            }
            if matched10 {
                ctx.lines_as("Alice", args!["Are you sure you really want to go into Field Combat Training? Have you spoken to every tutor? You better do that beforehand."])?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("I want Field Combat Training~!:Oh, wait!")])? {
                    1 => {
                        ctx.lines_as("Alice", args!["What an enthusiastic Novice you are! Okay, I'll send you to the folks in charge of Field Combat Training. Make sure that you listen carefully to the trainers."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Alice",
                            args![
                                "After all...",
                                "When you're fighting monsters, it's a matter of life and death! Alright then, take care~"
                            ],
                        )?;
                        ctx.close_window()?;
                        ctx.call(Function::Warp, vec![Val::from("new_1-2"), Val::from(28), Val::from(178)])?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as("Alice", args!["Now, that's a good decision. You won't get any many chances to get free stuff and experience in the future. You better make the most of this opportunity while you can!"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            }
            if !matched10 && subject10.loosely_equals(&Val::from(3)) {
                matched10 = true;
            }
            if matched10 {
                ctx.lines_as(
                    "Alice",
                    args!["If you want to go to a town, ask the Kafra Employee to the right. Alright then, take care~"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    } else if ctx.var("nov_get_item03").get()?.number()? < 10 {
        ctx.lines_as("Alice", args!["So how may", "I help you?", "It looks like you still haven't attended the ^4d4dffthe Skill information class^000000 yet. Would you like to attend that class first?"])?;
        ctx.next()?;
        'b12: {
            let subject12 = Val::from(runtime::select_values(
                ctx,
                &[Val::from("I'll attend that class.:Send me to Field Combat Training.:Cancel")],
            )?);
            let mut matched12 = false;
            let no_case12 = !subject12.loosely_equals(&Val::from(1))
                && !subject12.loosely_equals(&Val::from(2))
                && !subject12.loosely_equals(&Val::from(3));
            if !matched12 && subject12.loosely_equals(&Val::from(1)) {
                matched12 = true;
            }
            if matched12 {
                ctx.lines_as(
                    "Alice",
                    args!["Now, that's a good idea. Please talk to Cecil, the tutor at the far left side of this room, okay?"],
                )?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("new_1-2"), Val::from(84), Val::from(107)])?;
                return Err(Stop::End);
            }
            if !matched12 && subject12.loosely_equals(&Val::from(2)) {
                matched12 = true;
            }
            if matched12 {
                ctx.lines_as("Alice", args!["Are you sure you really want to go into Field Combat Training? Have you spoken to every tutor? You better do that beforehand."])?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("I want Field Combat Training~!:Oh, wait!")])? {
                    1 => {
                        ctx.lines_as("Alice", args!["What an enthusiastic Novice you are! Okay, I'll send you to the folks in charge of Field Combat Training. Make sure that you listen carefully to the trainers."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Alice",
                            args![
                                "After all...",
                                "When you're fighting monsters, it's a matter of life and death! Alright then, take care~"
                            ],
                        )?;
                        ctx.close_window()?;
                        ctx.call(Function::Warp, vec![Val::from("new_1-2"), Val::from(28), Val::from(178)])?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as("Alice", args!["Now, that's a good decision. You won't get any many chances to get free stuff and experience in the future. You better make the most of this opportunity while you can!"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            }
            if !matched12 && subject12.loosely_equals(&Val::from(3)) {
                matched12 = true;
            }
            if matched12 {
                ctx.lines_as(
                    "Alice",
                    args!["If you want to go to a town, ask the Kafra Employee to the right. Alright then, take care~"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
    Ok(Val::from(0))
}

pub fn item_tutor_nv(ctx: &Ctx) -> Script {
    item_tutor_nv_body(ctx, Vec::new()).map(|_| ())
}

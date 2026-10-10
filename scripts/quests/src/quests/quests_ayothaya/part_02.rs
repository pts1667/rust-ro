use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn shaman_thai_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_items: Vec<Val> = Vec::new();
    let base = Val::from(0).number()?;
    runtime::local_set(&mut l_items, &Val::from(base + 0), Val::from(1059), false);
    runtime::local_set(&mut l_items, &Val::from(base + 1), Val::from(1022), false);
    if ctx.var("ayodunquest").get()? == 1 {
        ctx.lines_as("Boonthom", args!["You...!", "Isn't it....!", "Oooooohhhhhhh!"])?;
        ctx.next()?;
        ctx.lines_as("Boonthom", args!["Oh...?"])?;
        ctx.next()?;
        ctx.lines_as("Boonthom", args!["..."])?;
        ctx.next()?;
        ctx.lines_as("Boonthom", args!["...", "......"])?;
        ctx.next()?;
        ctx.lines_as("Boonthom", args!["...", "......", "........."])?;
        ctx.next()?;
        ctx.lines_as("Boonthom", args!["Never mind.", "Hah! I've lost my", "train of thought."])?;
        ctx.next()?;
        ctx.lines_as("Boonthom", args!["So have you come to me to seek help? I can see the fear in your eyes. Let me gaze into your soul, and see what your fear is for myself. Hmmm..."])?;
        ctx.next()?;
        ctx.lines_as("Boonthom", args!["..."])?;
        ctx.next()?;
        ctx.lines_as("Boonthom", args!["...", "......"])?;
        ctx.next()?;
        ctx.lines_as("Boonthom", args!["...", "......", "........."])?;
        ctx.next()?;
        ctx.lines_as(
            "Boonthom",
            args![
                "What...!",
                "I see that you dread the Sa-mhing Tiger! I haven't heard any news of that monster for a long time..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Boonthom",
            args![
                "A long time ago,",
                "the Sa-mhing Tiger",
                "really did roam the village",
                "and ate innocent people."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Boonthom",
            args![
                "However, its terror",
                "was brought to an end when it",
                "was finally captured. I was one of those involved in capturing the Sa-mhing Tiger."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Boonthom",
            args![
                "I insisted killing on it, but the other people were too afraid to do so. Instead, the Sa-mhing Tiger",
                "was locked in a cave inside of the ancient ruins."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Boonthom",
            args![
                "I believe that was at least 10 years ago. Perhaps it's already dead by now. However, no one",
                "knows how powerful the tiger truly",
                "is. It may still be alive."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Boonthom",
            args!["I don't think we need to worry about it though. It is locked up and there it shall remain."],
        )?;
        ctx.next()?;
        match runtime::select_values(
            ctx,
            &[Val::from("I want to explore the ancient ruins.:Locked up! Good! I'm not afraid!")],
        )? {
            1 => {
                ctx.lines_as(
                    "Boonthom",
                    args![
                        "You...",
                        "You are strange.",
                        "You want to explore the ancient ruins. Do all adventurers disregard their safety like that?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Boonthom", args!["It's your choice. But remember, it is very dangerous to go through the path to the center of the building. Usually, I forbid it."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Boonthom",
                    args![
                        "But if you want to go in,",
                        "I can give you some useful tips.",
                        "Would you like to listen?"
                    ],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("If it's that dangerous, I'd rather not try.:Sure!")],
                )?) == 1
                {
                    ctx.lines_as(
                        "Boonthom",
                        args!["You made a good decision.", "You'd better be careful if", "you want to stay alive."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as("Boonthom", args!["Umm, okay.", "Since you're", "that interested..."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Boonthom",
                    args!["If you wish to enter the ancient building, you must have protection against evil spirits and malice."],
                )?;
                ctx.next()?;
                if ((ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_PRIEST")?)
                    || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?))
                    || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_CRUSADER")?))
                {
                    ctx.lines_as(
                        "Boonthom",
                        args![
                            "Even if you can",
                            "wield the power of",
                            "holiness, you will still need the spiritual security of our land."
                        ],
                    )?;
                } else {
                    ctx.lines_as(
                        "Boonthom",
                        args![
                            "There are many ghosts and",
                            "demons within the cave, and they will attack people who do not possess holy power... Like you."
                        ],
                    )?;
                }
                ctx.next()?;
                ctx.lines_as(
                    "Boonthom",
                    args![
                        "I can make you an object",
                        "containing this holy power if you wish. In order to make it, I will need some materials."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Boonthom",
                    args![
                        ((Val::from("^3366991 ")
                            + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_items, &Val::from(0), false)])?)
                            + Val::from("^000000,")),
                        ((Val::from("^3366991 ")
                            + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_items, &Val::from(1), false)])?)
                            + Val::from("^000000,")),
                        "^3366991 Solid Husk^000000 and",
                        "^3366991 Holy Water^000000...",
                        "That's all I need."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Boonthom", args!["Please come back", "when you prepare all", "the materials."])?;
                ctx.var("ayodunquest").set(Val::from(2))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(12035), Val::from(12036)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
        ctx.lines_as("Boonthom", args!["Hmmm...", "Not afraid", "of it, are you?"])?;
        ctx.next()?;
        ctx.lines_as("Boonthom", args!["............", "............"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("ayodunquest").get()? == 2 {
            if (((ctx
                .call(Function::CountItem, vec![runtime::local_get(&l_items, &Val::from(0), false)])?
                .number()?
                > 0
                && ctx
                    .call(Function::CountItem, vec![runtime::local_get(&l_items, &Val::from(1), false)])?
                    .number()?
                    > 0)
                && ctx.call(Function::CountItem, vec![Val::from(7190)])?.number()? > 0)
                && ctx.call(Function::CountItem, vec![Val::from(523)])?.number()? > 0)
            {
                ctx.lines_as(
                    "Boonthom",
                    args![
                        "Excellent!",
                        "Now you have brought",
                        "everything I need. Let me make",
                        "the thing for you as promised..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Boonthom", args!["Ooooohmmmmm..."])?;
                ctx.next()?;
                ctx.lines_as("Boonthom", args!["Hmmmmmm...", "Hmmmmmmmmm."])?;
                ctx.next()?;
                ctx.lines_as("Boonthom", args!["Pffff pffff..."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Boonthom",
                    args![
                        "Here you go.",
                        "Please take these holy threads.",
                        "With this, you will be able to enter the ruins with less worry."
                    ],
                )?;
                ctx.call(
                    Function::DelItem,
                    vec![runtime::local_get(&l_items, &Val::from(0), false), Val::from(1)],
                )?;
                ctx.call(
                    Function::DelItem,
                    vec![runtime::local_get(&l_items, &Val::from(1), false), Val::from(1)],
                )?;
                ctx.call(Function::DelItem, vec![Val::from(7190), Val::from(1)])?;
                ctx.call(Function::DelItem, vec![Val::from(523), Val::from(1)])?;
                ctx.var("ayodunquest").set(Val::from(3))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(12036), Val::from(12037)])?;
                ctx.call(Function::GetItem, vec![Val::from(7285), Val::from(1)])?;
                ctx.next()?;
                ctx.lines_as(
                    "Boonthom",
                    args![
                        "Even if you lose this, don't worry. Just bring me the materials, and",
                        "I will make you another one."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Boonthom", args!["The ancient building consists", "of 2 levels. Before you enter the 2nd underground level, please return to me so that I can tell you how to enter that place."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Boonthom",
                args![
                    "Did you forget what materials",
                    "you need to create my object of holy power? Please listen",
                    "carefully this time."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Boonthom",
                args![
                    ((Val::from("^3366991 ")
                        + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_items, &Val::from(0), false)])?)
                        + Val::from("^000000,")),
                    ((Val::from("^3366991 ")
                        + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_items, &Val::from(1), false)])?)
                        + Val::from("^000000,")),
                    "^3366991 Solid Husk^000000 and",
                    "^3366991 Holy Water^000000."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Boonthom", args!["See you later."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("ayodunquest").get()? == 3 {
                if (((ctx
                    .call(Function::CountItem, vec![runtime::local_get(&l_items, &Val::from(0), false)])?
                    .number()?
                    > 0
                    && ctx
                        .call(Function::CountItem, vec![runtime::local_get(&l_items, &Val::from(1), false)])?
                        .number()?
                        > 0)
                    && ctx.call(Function::CountItem, vec![Val::from(7190)])?.number()? > 0)
                    && ctx.call(Function::CountItem, vec![Val::from(523)])?.number()? > 0)
                {
                    ctx.lines_as(
                        "Boonthom",
                        args!["Excellent! Now you have brought everything I need, let me make the thing for you as I promised..."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Boonthom", args!["Ooooohmmmmm..."])?;
                    ctx.next()?;
                    ctx.lines_as("Boonthom", args!["Hmmmmmm...", "Hmmmmmmmmm."])?;
                    ctx.next()?;
                    ctx.lines_as("Boonthom", args!["Pffff pffff..."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Boonthom",
                        args![
                            "Here you go.",
                            "Please take these holy threads.",
                            "With this, you will be able to enter the ruins with less worry."
                        ],
                    )?;
                    ctx.call(
                        Function::DelItem,
                        vec![runtime::local_get(&l_items, &Val::from(0), false), Val::from(1)],
                    )?;
                    ctx.call(
                        Function::DelItem,
                        vec![runtime::local_get(&l_items, &Val::from(1), false), Val::from(1)],
                    )?;
                    ctx.call(Function::DelItem, vec![Val::from(7190), Val::from(1)])?;
                    ctx.call(Function::DelItem, vec![Val::from(523), Val::from(1)])?;
                    ctx.call(Function::GetItem, vec![Val::from(7285), Val::from(1)])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Boonthom",
                        args!["Even if you lose this, don't worry. Just bring me the materials, I can make you another one."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Boonthom",
                    args!["If you lose the holy threads, don't worry. I can make some more for you. Just gather the following items."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Boonthom",
                    args![
                        ((Val::from("^3366991 ")
                            + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_items, &Val::from(0), false)])?)
                            + Val::from("^000000,")),
                        ((Val::from("^3366991 ")
                            + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_items, &Val::from(1), false)])?)
                            + Val::from("^000000,")),
                        "^3366991 Solid Husk^000000 and",
                        "^3366991 Holy Water^000000."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Boonthom", args!["See you later."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if (ctx.var("ayodunquest").get()?.number()? > 3 && ctx.var("ayodunquest").get()?.number()? < 9) {
                    ctx.lines_as(
                        "Boonthom",
                        args!["You finally started seeking the tiger. I hope that you will be safe from harm."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Boonthom", args!["Feel free to ask", "me for help at anytime."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("ayodunquest").get()? == 9 {
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![
                                "I copied down this",
                                "message while exploring",
                                "the dungeon. Would you",
                                "take a look at this?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Stone Slate",
                            args![
                                "^333333Do not enter the",
                                "2nd underground level",
                                "...is danger... You wil...",
                                "...encounter... tiger...^000000"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Stone Slate",
                            args![
                                "^333333You must kill... In order to do... MUST... How to go... the 2nd underground level...",
                                "You need more... Holy power... in order to... Otherwise, it's impossible...^000000"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Boonthom",
                            args![
                                "That was written by one of my comrades who entered the ruins,",
                                "as he regretted letting the tiger live, lest it return."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Boonthom",
                            args!["If you were able to find this note, you must have strong determination to venture through the ruins."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Boonthom", args!["As you can guess from the", "note, the 2nd underground level is very very dangerous. You will need holy threads that contain even more holy protection."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Boonthom",
                            args!["Now, go and gather some materials so that I can create more powerful holy threads for you."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Boonthom",
                            args![
                                "^3366992 Holy Water^000000,",
                                "^3366991 Yggdrasil Leaf^000000,",
                                ((Val::from("^3366992 ")
                                    + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_items, &Val::from(0), false)])?)
                                    + Val::from("^000000 and")),
                                ((Val::from("^3366992 ")
                                    + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_items, &Val::from(1), false)])?)
                                    + Val::from("^000000."))
                            ],
                        )?;
                        ctx.var("ayodunquest").set(Val::from(10))?;
                        ctx.call(Function::ChangeQuest, vec![Val::from(12037), Val::from(12038)])?;
                        ctx.next()?;
                        ctx.lines_as("Boonthom", args!["Return to me", "once you have", "gathered everything."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if ctx.var("ayodunquest").get()? == 10 {
                            if (((ctx.call(Function::CountItem, vec![Val::from(523)])?.number()? > 1
                                && ctx.call(Function::CountItem, vec![Val::from(610)])?.number()? > 0)
                                && ctx
                                    .call(Function::CountItem, vec![runtime::local_get(&l_items, &Val::from(0), false)])?
                                    .number()?
                                    > 1)
                                && ctx
                                    .call(Function::CountItem, vec![runtime::local_get(&l_items, &Val::from(1), false)])?
                                    .number()?
                                    > 1)
                            {
                                ctx.lines_as(
                                    "Boonthom",
                                    args![
                                        "Hmm~",
                                        "You've brought",
                                        "everything. Quite",
                                        "the enthusiastic one,",
                                        "aren't you?"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Boonthom", args!["Ooooohmmmmm..."])?;
                                ctx.next()?;
                                ctx.lines_as("Boonthom", args!["Hmmmmmm...", "Hmmmmmmmmm."])?;
                                ctx.next()?;
                                ctx.lines_as("Boonthom", args!["Pffff pffff..."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Boonthom",
                                    args![
                                        "Here you go.",
                                        "Please take these holy threads.",
                                        "With this, you will be able to enter the ruins with less worry."
                                    ],
                                )?;
                                ctx.call(
                                    Function::DelItem,
                                    vec![runtime::local_get(&l_items, &Val::from(0), false), Val::from(2)],
                                )?;
                                ctx.call(
                                    Function::DelItem,
                                    vec![runtime::local_get(&l_items, &Val::from(1), false), Val::from(2)],
                                )?;
                                ctx.call(Function::DelItem, vec![Val::from(610), Val::from(1)])?;
                                ctx.call(Function::DelItem, vec![Val::from(523), Val::from(2)])?;
                                ctx.var("ayodunquest").set(Val::from(11))?;
                                ctx.call(Function::GetItem, vec![Val::from(7287), Val::from(1)])?;
                                ctx.call(Function::ChangeQuest, vec![Val::from(12038), Val::from(12039)])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Boonthom",
                                    args![
                                        "Even if you lose this, don't worry. Just bring me the materials, and",
                                        "I will make you another one."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.lines_as(
                                "Boonthom",
                                args![
                                    "Hmm...?",
                                    "Didn't I tell you",
                                    "what I need to make",
                                    "a more powerful",
                                    "holy thread?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Boonthom",
                                args![
                                    "^3366992 Holy Water^000000,",
                                    "^3366991 Yggdrasil Leaf^000000,",
                                    ((Val::from("^3366992 ")
                                        + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_items, &Val::from(0), false)])?)
                                        + Val::from("^000000 and")),
                                    ((Val::from("^3366992 ")
                                        + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_items, &Val::from(1), false)])?)
                                        + Val::from("^000000."))
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Boonthom",
                                args![
                                    "Return to me",
                                    "with those items",
                                    "and I will craft",
                                    "more powerful holy",
                                    "threads for you."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.var("ayodunquest").get()? == 11 {
                            if ctx.call(Function::CountItem, vec![Val::from(7287)])?.number()? > 0 {
                                ctx.lines_as(
                                    "Boonthom",
                                    args![
                                        "Those threads",
                                        "will protect you",
                                        "from the spiritual",
                                        "dangers, but it's up",
                                        "to you to defend yourself",
                                        "from monster attacks, okay?"
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if ((((ctx.call(Function::CountItem, vec![Val::from(523)])?.number()? > 1
                                && ctx.call(Function::CountItem, vec![Val::from(610)])?.number()? > 0)
                                && ctx
                                    .call(Function::CountItem, vec![runtime::local_get(&l_items, &Val::from(0), false)])?
                                    .number()?
                                    > 1)
                                && ctx
                                    .call(Function::CountItem, vec![runtime::local_get(&l_items, &Val::from(1), false)])?
                                    .number()?
                                    > 1)
                                && ctx.call(Function::CountItem, vec![Val::from(7287)])? == 0)
                            {
                                ctx.lines_as("Boonthom", args!["Ooooohmmmmm..."])?;
                                ctx.next()?;
                                ctx.lines_as("Boonthom", args!["Hmmmmmm...", "Hmmmmmmmmm."])?;
                                ctx.next()?;
                                ctx.lines_as("Boonthom", args!["Pffff pffff..."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Boonthom",
                                    args![
                                        "Here you go.",
                                        "Please take these holy threads.",
                                        "With this, you will be able to enter the ruins with less worry."
                                    ],
                                )?;
                                ctx.call(
                                    Function::DelItem,
                                    vec![runtime::local_get(&l_items, &Val::from(0), false), Val::from(2)],
                                )?;
                                ctx.call(
                                    Function::DelItem,
                                    vec![runtime::local_get(&l_items, &Val::from(1), false), Val::from(2)],
                                )?;
                                ctx.call(Function::DelItem, vec![Val::from(610), Val::from(1)])?;
                                ctx.call(Function::DelItem, vec![Val::from(523), Val::from(2)])?;
                                ctx.call(Function::GetItem, vec![Val::from(7287), Val::from(1)])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.lines_as(
                                "Boonthom",
                                args![
                                    "Hmm...?",
                                    "Didn't I tell you",
                                    "what I need to make",
                                    "a more powerful",
                                    "holy thread?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Boonthom",
                                args![
                                    "^3366992 Holy Water^000000,",
                                    "^3366991 Yggdrasil Leaf^000000,",
                                    ((Val::from("^3366992 ")
                                        + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_items, &Val::from(0), false)])?)
                                        + Val::from("^000000 and")),
                                    ((Val::from("^3366992 ")
                                        + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_items, &Val::from(1), false)])?)
                                        + Val::from("^000000."))
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.var("ayodunquest").get()? == 12 {
                            ctx.lines_as(
                                "Boonthom",
                                args![
                                    "Huh...!",
                                    "You must have seen something.",
                                    "I can tell by the gloomy look on your face, and the shadow looming over your soul."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["I found", "this old letter", "in the ruins."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Old Letter", args!["^333333Dear, Boonthom...I was a fool...There is Sa-mhing Tig'...One who ....my remains... Please take...to Boonthom..."])?;
                            ctx.next()?;
                            ctx.lines_as("Boonthom", args!["I see...", "Oh, my dear", "old friend..."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Boonthom",
                                args![
                                    "Thank you for your trouble.",
                                    "My friend left that box for me.",
                                    "But I guess he would wish to give that box to the person who found his remains."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Boonthom", args!["If you wish to explore the building again, but you don't have any holy thread, feel free to ask me at anytime."])?;
                            ctx.var("ayodunquest").set(Val::from(13))?;
                            ctx.call(Function::CompleteQuest, vec![Val::from(12039)])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.var("ayodunquest").get()?.number()? > 12 {
                            if ctx.call(Function::CountItem, vec![Val::from(7287)])?.number()? > 0 {
                                ctx.lines_as("Boonthom", args!["Best of luck on your", "expeditions, brave", "adventurer~"])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if ((((ctx.call(Function::CountItem, vec![Val::from(523)])?.number()? > 1
                                && ctx.call(Function::CountItem, vec![Val::from(610)])?.number()? > 0)
                                && ctx
                                    .call(Function::CountItem, vec![runtime::local_get(&l_items, &Val::from(0), false)])?
                                    .number()?
                                    > 1)
                                && ctx
                                    .call(Function::CountItem, vec![runtime::local_get(&l_items, &Val::from(1), false)])?
                                    .number()?
                                    > 1)
                                && ctx.call(Function::CountItem, vec![Val::from(7287)])? == 0)
                            {
                                ctx.lines_as(
                                    "Boonthom",
                                    args![
                                        "Lost the",
                                        "holy threads",
                                        "somehow, did you?",
                                        "With the items you",
                                        "have, I'll simply",
                                        "make more of them."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Boonthom", args!["Ooooohmmmmm..."])?;
                                ctx.next()?;
                                ctx.lines_as("Boonthom", args!["Hmmmmmm...", "Hmmmmmmmmm."])?;
                                ctx.next()?;
                                ctx.lines_as("Boonthom", args!["Pffff pffff..."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Boonthom",
                                    args![
                                        "Here you go.",
                                        "Please take these holy threads.",
                                        "With this, you will be able to enter the ruins with less worry."
                                    ],
                                )?;
                                ctx.call(
                                    Function::DelItem,
                                    vec![runtime::local_get(&l_items, &Val::from(0), false), Val::from(2)],
                                )?;
                                ctx.call(
                                    Function::DelItem,
                                    vec![runtime::local_get(&l_items, &Val::from(1), false), Val::from(2)],
                                )?;
                                ctx.call(Function::DelItem, vec![Val::from(610), Val::from(1)])?;
                                ctx.call(Function::DelItem, vec![Val::from(523), Val::from(2)])?;
                                ctx.call(Function::GetItem, vec![Val::from(7287), Val::from(1)])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.lines_as(
                                "Boonthom",
                                args![
                                    "Hmm...?",
                                    "Didn't I tell you",
                                    "what I need to make",
                                    "a more powerful",
                                    "holy thread?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Boonthom",
                                args![
                                    "^3366992 Holy Water^000000,",
                                    "^3366991 Yggdrasil Leaf^000000,",
                                    ((Val::from("^3366992 ")
                                        + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_items, &Val::from(0), false)])?)
                                        + Val::from("^000000 and")),
                                    ((Val::from("^3366992 ")
                                        + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_items, &Val::from(1), false)])?)
                                        + Val::from("^000000."))
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                }
            }
        }
    }
    ctx.lines_as("Boonthom", args!["Ooooohmmmmm..."])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn shaman_thai(ctx: &Ctx) -> Script {
    shaman_thai_body(ctx, Vec::new()).map(|_| ())
}

fn puraim_thai1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ((ctx.var("ayodunquest").get()?.number()? > 2 && ctx.call(Function::CountItem, vec![Val::from(7285)])?.number()? > 0)
        || (ctx.var("ayodunquest").get()?.number()? > 2 && ctx.call(Function::CountItem, vec![Val::from(7287)])?.number()? > 0))
    {
        ctx.lines_as(
            "Puraim",
            args![
                "Huh...?",
                "I feel it!",
                "I can feel the",
                "power of holiness!",
                "It's coming from you!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Puraim",
            args![
                "I guess you can go ahead",
                "into the ruins. So that's",
                "what you want? To explore",
                "this area?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Puraim",
            args![
                "I would suggest",
                "against coming inside.",
                "Even the local people",
                "fear this area."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Puraim", args!["If the Sa-mhing Tiger wasn't around, there would still be all the evil spirits in this place. Even with the holy thread, this place is still dangerous."])?;
        ctx.next()?;
        ctx.lines_as(
            "Puraim",
            args!["So what do you want?", "Do you still want to", "explore the area,", "adventurer?"],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from("Yes.:No! I'm too afraid of the ruins now.")],
        )?) == 1
        {
            ctx.lines_as(
                "Puraim",
                args![
                    "Hmmm...",
                    "It seems you adventurers are tempted by the thrill of danger,",
                    "or you've all got some death wish."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Puraim",
                args![
                    "Oh well, that's none of my business. After all, you already",
                    "have a holy spirit. Still, be careful. You might not want to go to the 2nd underground level of the ruins."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Puraim",
                args!["As you Midgardians", "say, 'curiousity killed the cat.'", "Anyway, go for it!"],
            )?;
            ctx.next()?;
            ctx.lines(args![
                "^3355FFHe shoved you off the hill",
                "and you plummet like a rock.^000000"
            ])?;
            ctx.close_window()?;
            ctx.call(Function::PercentHeal, vec![Val::from(-10), Val::from(0)])?;
            ctx.call(Function::Warp, vec![Val::from("ayo_fild02"), Val::from(30), Val::from(135)])?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Puraim",
            args![
                "Good decision!",
                "As I expected, you're",
                "smart enough not to",
                "stupidly jeopardize your life."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Puraim",
        args!["If I were you, I wouldn't", "even dare to approach", "the ruins down over there."],
    )?;
    ctx.next()?;
    ctx.lines_as("Puraim", args!["Anyone who doesn't have the protection of a holy spirit will be easy game for the evil creatures living there. You'd be killed in", "no time at all!"])?;
    ctx.next()?;
    ctx.lines_as(
        "Puraim",
        args![
            "Go back, adventurer.",
            "If you wish to explore",
            "this area, you'd better",
            "show me some holy spirit."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn puraim_thai1(ctx: &Ctx) -> Script {
    puraim_thai1_body(ctx, Vec::new()).map(|_| ())
}

fn aik_thai_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Aik",
        args![
            "So, how was your expedition?",
            "I hope that the evil spirits will not follow you outside of the ruins, and haunt your dreams."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Aik",
        args![
            "When you go back,",
            "remember to ward off",
            "the evil from your mind.",
            "Otherwise you will encounter",
            "trouble later..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Aik", args!["So, would you", "like to go back?"])?;
    ctx.next()?;
    if Val::from(runtime::select_values(
        ctx,
        &[Val::from("Yes.:No, I need to look around more...")],
    )?) == 1
    {
        ctx.lines_as("Aik", args!["Alright then...", "Here we go..."])?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFHe suddenly grabbed you",
            "and hurled you up into the air!^000000"
        ])?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("ayo_fild01"), Val::from(115), Val::from(200)])?;
        return Err(Stop::End);
    }
    ctx.lines_as("Aik", args!["I see.", "Take care."])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn aik_thai(ctx: &Ctx) -> Script {
    aik_thai_body(ctx, Vec::new()).map(|_| ())
}

fn th_dun1_1_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args![
        "^3355FFYou look down and see a pool",
        "of water in the distance. It looks like a long drop to the bottom.^000000"
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "^3355FFYou are filled",
        "with apprehension,",
        "After all, if you fall down,^000000",
        "^2F2F4Fyou might get killed.^000000"
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn th_dun1_1_1(ctx: &Ctx) -> Script {
    th_dun1_1_1_body(ctx, Vec::new()).map(|_| ())
}

fn th_dun1_1_1_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args![
        "^3355FFYou look down and see a pool",
        "of water in the distance. It looks like a long drop to the bottom.^000000"
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "^3355FFYou are filled",
        "with apprehension.",
        "After all, if you fall down,",
        "^2F2F4Fyou might get killed.^000000"
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn th_dun1_1_1_ontouch(ctx: &Ctx) -> Script {
    th_dun1_1_1_ontouch_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ThDun11Step {
    Start,
    OnTouch,
}

fn th_dun1_1_run(ctx: &Ctx, mut step: ThDun11Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ThDun11Step::Start => {
                step = ThDun11Step::OnTouch;
                continue 'machine;
            }
            ThDun11Step::OnTouch => {
                ctx.call(Function::PercentHeal, vec![Val::from(-100), Val::from(0)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn th_dun1_1(ctx: &Ctx) -> Script {
    th_dun1_1_run(ctx, ThDun11Step::Start, Vec::new()).map(|_| ())
}

pub fn th_dun1_1_ontouch(ctx: &Ctx) -> Script {
    th_dun1_1_run(ctx, ThDun11Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Hint01Step {
    Start,
    OnTouch,
}

fn hint01_run(ctx: &Ctx, mut step: Hint01Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Hint01Step::Start => {
                step = Hint01Step::OnTouch;
                continue 'machine;
            }
            Hint01Step::OnTouch => {
                if (ctx.call(Function::Rand, vec![Val::from(1), Val::from(100)])?.number()? > 60
                    && ctx.var("ayodunquest").get()?.number()? < 11)
                {
                    ctx.mes("^3355FFThe Holy Thread in your pocket suddenly began to glow. You feel that something is near you...^000000")?;
                    ctx.close_window()?;
                    ctx.call(
                        Function::ViewPoint,
                        vec![Val::from(1), Val::from(198), Val::from(164), Val::from(0), Val::from(16711680)],
                    )?;
                    ctx.call(
                        Function::ViewPoint,
                        vec![Val::from(1), Val::from(87), Val::from(16), Val::from(1), Val::from(16711680)],
                    )?;
                    ctx.call(
                        Function::ViewPoint,
                        vec![Val::from(1), Val::from(268), Val::from(214), Val::from(2), Val::from(16711680)],
                    )?;
                    ctx.call(
                        Function::ViewPoint,
                        vec![Val::from(1), Val::from(147), Val::from(274), Val::from(3), Val::from(16711680)],
                    )?;
                    ctx.call(
                        Function::ViewPoint,
                        vec![Val::from(1), Val::from(99), Val::from(118), Val::from(4), Val::from(16711680)],
                    )?;
                    ctx.call(
                        Function::ViewPoint,
                        vec![Val::from(1), Val::from(16), Val::from(188), Val::from(5), Val::from(16711680)],
                    )?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn hint01(ctx: &Ctx) -> Script {
    hint01_run(ctx, Hint01Step::Start, Vec::new()).map(|_| ())
}

pub fn hint01_ontouch(ctx: &Ctx) -> Script {
    hint01_run(ctx, Hint01Step::OnTouch, Vec::new()).map(|_| ())
}

fn hun_thai_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ayodunquest").get()? == 3 {
        ctx.var("ayodunquest").set(Val::from(4))?;
        ctx.mes("^3355FFYou find a piece of a stone slate with letters etched on it. It seems to be the 1st part of the slate.^000000")?;
        ctx.next()?;
        ctx.mes("^3355FFIt reads '^0000FFDo not enter the 2nd underground level^3355FF.' You are unable to read the rest of the message.^000000")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.mes("^3355FFYou find a piece of a stone slate with letters etched on it. It seems to be the 1st part of the slate.^000000")?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn hun_thai_1(ctx: &Ctx) -> Script {
    hun_thai_1_body(ctx, Vec::new()).map(|_| ())
}

fn hun_thai_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ayodunquest").get()? == 4 {
        ctx.var("ayodunquest").set(Val::from(5))?;
        ctx.mes("^3355FFYou find a piece of a stone slate with letters etched on it. It seems to be the 2nd part of the slate.^000000")?;
        ctx.next()?;
        ctx.mes("^3355FFIt reads '^0000FF...is danger... You wil...^3355FF.' You are unable to read the rest of the message.^000000")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.mes("^3355FFYou find a piece of a stone slate with letters etched on it. It seems to be the 2nd part of the slate.^000000")?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn hun_thai_2(ctx: &Ctx) -> Script {
    hun_thai_2_body(ctx, Vec::new()).map(|_| ())
}

fn hun_thai_3_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ayodunquest").get()? == 5 {
        ctx.var("ayodunquest").set(Val::from(6))?;
        ctx.mes("^3355FFYou find a piece of a stone slate with letters etched on it. It seems to be the 3rd part of the slate.^000000")?;
        ctx.next()?;
        ctx.mes("^3355FFIt reads '^0000FF...encounter... tiger...^3355FF.' You are unable to read the rest of the message.^000000")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.mes("^3355FFYou find a piece of a stone slate with letters etched on it. It seems to be the 3rd part of the slate.^000000")?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn hun_thai_3(ctx: &Ctx) -> Script {
    hun_thai_3_body(ctx, Vec::new()).map(|_| ())
}

fn hun_thai_4_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ayodunquest").get()? == 6 {
        ctx.var("ayodunquest").set(Val::from(7))?;
        ctx.mes("^3355FFYou find a piece of a stone slate with letters etched on it. It seems to be the 4th part of the slate.^000000")?;
        ctx.next()?;
        ctx.mes("^3355FFIt reads '^0000FFYou must kill... In order to do... MUST... How to go... the 2nd underground level...^3355FF' You are unable to read the rest of the message.^000000")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.mes("^3355FFYou find a piece of a stone slate with letters etched on it. It seems to be the 4th part of the slate.^000000")?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn hun_thai_4(ctx: &Ctx) -> Script {
    hun_thai_4_body(ctx, Vec::new()).map(|_| ())
}

fn hun_thai_5_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ayodunquest").get()? == 7 {
        ctx.var("ayodunquest").set(Val::from(8))?;
        ctx.mes("^3355FFYou find a piece of a stone slate with letters etched on it. It seems to be the 5th part of the slate.^000000")?;
        ctx.next()?;
        ctx.mes("^3355FFIt reads '^0000FFYou need more... Holy power... in order to...^3355FF' You are unable to read the rest of the message.^000000")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.mes("^3355FFYou find a piece of a stone slate with letters etched on it. It seems to be the 5th part of the slate.^000000")?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn hun_thai_5(ctx: &Ctx) -> Script {
    hun_thai_5_body(ctx, Vec::new()).map(|_| ())
}

fn hun_thai_6_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ayodunquest").get()? == 8 {
        ctx.var("ayodunquest").set(Val::from(9))?;
        ctx.mes("^3355FFYou find a piece of a stone slate with letters etched on it. It seems to be the last part of the slate.^000000")?;
        ctx.next()?;
        ctx.mes("^3355FFIt reads '^0000FFOtherwise, it's impossible...^3355FF' You are unable to read the rest of the message.^000000")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.mes("^3355FFYou find a piece of a stone slate with letters etched on it. It seems to be the last part of the slate.^000000")?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn hun_thai_6(ctx: &Ctx) -> Script {
    hun_thai_6_body(ctx, Vec::new()).map(|_| ())
}

fn s_2_dun_in_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ayodunquest").get()?.number()? > 10 {
        if ctx.call(Function::CountItem, vec![Val::from(7287)])?.number()? > 0 {
            ctx.lines(args![
                "^3355FFThe holy threads",
                "emanate a powerful sense",
                "of safety and security.",
                "With the protection of",
                "holiness, you begin your",
                "descent into the 2nd level.^000000."
            ])?;
            ctx.close_window()?;
            if ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?.number()? > 5 {
                ctx.call(Function::Warp, vec![Val::from("ayo_dun02"), Val::from(23), Val::from(26)])?;
            } else {
                ctx.call(Function::Warp, vec![Val::from("ayo_dun02"), Val::from(275), Val::from(26)])?;
            }
            return Err(Stop::End);
        }
        ctx.mes("^3355FFYou find what seems to be the entrance to the 2nd level. However, you are filled with an overwhelming sense of dread. Somehow, you can't bring yourself to go down...^000000")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.mes("^3355FFYou find what seems to be the entrance to the 2nd level. However, you are filled with an overwhelming sense of dread. Somehow, you can't bring yourself to go down...^000000")?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn s_2_dun_in(ctx: &Ctx) -> Script {
    s_2_dun_in_body(ctx, Vec::new()).map(|_| ())
}

fn reward_tiger_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("ayodunquest").get()? == 11 && ctx.call(Function::CountItem, vec![Val::from(7287)])?.number()? > 0) {
        ctx.mes("^3355FFYou find half of the skeletal remains of a person sticking out from the ground. It seems to be the remains of Boonthom's comrade.^000000")?;
        ctx.next()?;
        ctx.mes("^3355FFYou find a piece of paper beside the skeleton, as well as a box.^000000")?;
        ctx.next()?;
        ctx.mes("^3355FFYou read what is written on the old sheet of paper. The letters have faded with age, and it is difficult to read.^000000")?;
        ctx.next()?;
        ctx.lines_as(
            "Old Letter",
            args![
                "^333333Dear, Boonthom,",
                "I was a fool. There is 'Sa-mhing Tig'...One who ....my remains... Please take...to Boonthom..."
            ],
        )?;
        ctx.next()?;
        ctx.lines(args!["^3355FFYou pick", "up the box.^000000"])?;
        ctx.var("ayodunquest").set(Val::from(12))?;
        ctx.call(Function::GetItem, vec![Val::from(603), Val::from(1)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn reward_tiger(ctx: &Ctx) -> Script {
    reward_tiger_body(ctx, Vec::new()).map(|_| ())
}

fn einon_ayo_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("tomyumgoong").get()? == 0 {
        ctx.lines_as(
            "Einon",
            args!["Do you know what the", "most popular cuisine", "of this village is?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Einon",
            args![
                "Ayothaya is world famous",
                "for its regional cuisine, but",
                "out of all Ayothayan dishes,",
                "'^3131FFTom Yum Goong^000000' is the best."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Einon", args!["Tom Yum Goong is a type", "of soup that is very spicy. Try it once, and you will be amazed by its profound taste. Try it twice, and you will be enraptured by its tantalizing aroma."])?;
        ctx.next()?;
        ctx.lines_as(
            "Einon",
            args![
                "It's really one of the most delicious foods in the world!",
                "Everyone in the world would",
                "want to try this at least once in his lifetime. So, would you like",
                "to try some?"
            ],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from("Sure thing.:No, thanks. I hate spicy food.")],
        )?) == 1
        {
            ctx.lines_as(
                "Einon",
                args![
                    "Okay...!",
                    "Now who would be",
                    "best for preparing",
                    "Tom Yum Goong for you?",
                    "Hmmmmm..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Einon",
                args![
                    "Everyone in the village",
                    "knows how to make it, but",
                    "I recommend that you go",
                    "visit ^3131FFMali the Spicy^000000."
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_BEST")?])?;
            ctx.var("tomyumgoong").set(Val::from(1))?;
            ctx.call(Function::SetQuest, vec![Val::from(8123)])?;
            ctx.lines_as(
                "Einon",
                args![
                    "She is the best cook when it",
                    "comes to Tom Yum Goong!",
                    "Why don't you ask her",
                    "to cook you some?"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Einon",
            args!["Oh I see...", "However, when you", "change your mind,", "please come back."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("tomyumgoong").get()? == 1 {
        ctx.lines_as(
            "Einon",
            args![
                "Ah, you wanna know",
                "where ^3131FFMali the Spicy^000000 is?",
                "Well, there's not too many places",
                "a cook would stay other than in",
                "a restaurant, right? Hahaha~"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Einon",
        args![
            "Golden Ayothaya...",
            "Although I've grown",
            "up in this village,",
            "I still think my village",
            "is magnificient and beautiful."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Einon", args!["What do you think?", "Wouldn't you agree with me?"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn einon_ayo(ctx: &Ctx) -> Script {
    einon_ayo_body(ctx, Vec::new()).map(|_| ())
}

fn cook_ayo_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("tomyumgoong").get()?.number()? < 2 {
        ctx.lines_as(
            "Mali the Spicy",
            args![
                "Hello, there!",
                "I am called Mali the Spicy.",
                "All of my dishes always amaze",
                "my customers with their tempting scents and deep flavors."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Mali the Spicy",
            args![
                "I am especially proud of my",
                "^3131FFTom Yum Goong^000000, which is the",
                "best in the village. I suppose",
                "it's my fate to cook the greatest Tom Yum Goong ever."
            ],
        )?;
        if ctx.var("tomyumgoong").get()? == 0 {
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.next()?;
            ctx.lines_as(
                "Mali the Spicy",
                args!["Oh I see...", "You want me to cook", "Tom Yum Goong for you,", "don't you?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Mali the Spicy",
                args![
                    "Honestly, I've been really",
                    "busy ever since I opened this restaurant. If you really wish to taste my Tom Yum Goong, would",
                    "you do something for me?"
                ],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("Sure!:Eh... I dunno.")])?) == 1 {
                ctx.lines_as(
                    "Mali the Spicy",
                    args![
                        "First, would you get",
                        "the ingredients to make",
                        "Tom Yum Goong for me? I'll need some ^CE0000Shrimps^000000, ^CE0000Chiles^000000 and ^CE0000Lemons^000000."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mali the Spicy",
                    args![
                        "Remember, Shrimp is the main ingredient of this dish. So, the fresher the Shrimp, the tastier your food will be."
                    ],
                )?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("Where can I find them?")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as(
                    "Mali the Spicy",
                    args![
                        "I expected you",
                        "to ask me that.",
                        "For Shrimps, look around",
                        "the beach. You should see",
                        "a lot of them there."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Mali the Spicy", args!["However, you might have a hard time finding them since you haven't done that before. So just buy some Shrimp from the guy who sells", "the freshest Shrimp around."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Mali the Spicy",
                    args!["Hmmm...", "If you mention", "my name, that guy", "might give you a discount."],
                )?;
                ctx.var("tomyumgoong").set(Val::from(2))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(8123), Val::from(8124)])?;
                ctx.next()?;
                ctx.lines_as("Mali the Spicy", args!["His name is ^3131FFThongpool^000000.", "He will be somewhere around the village selling Shrimps. Go and ask him to give you 20 Shrimps to cook Tom Yum Goong."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Mali the Spicy",
                args![
                    "I understand this might be difficult to do, but it's worth",
                    "a try if you really want to taste the best Tom Yum Goong."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else if (ctx.var("tomyumgoong").get()?.number()? > 1 && ctx.var("tomyumgoong").get()?.number()? < 4) {
        if (ctx.var("tomyumgoong").get()? == 2 && ctx.call(Function::CountItem, vec![Val::from(567)])?.number()? > 19) {
            ctx.lines_as(
                "Mali the Spicy",
                args![
                    "Errr...",
                    "What's wrong with these Shrimp?",
                    "They all have these scratches and rips in them! I can't believe Thongpool sold such bad Shrimp",
                    "to his customers!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Mali the Spicy",
                args!["I'm so sorry, but", "I can't cook the best", "Tom Yum Goong with", "these Shrimp."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Mali the Spicy",
                args![
                    "I know it's an inconvenience, but would you go back to Thongpool",
                    "and ask him to give you the freshest Shrimp to cook the best",
                    "Tom Yum Goong?"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("tomyumgoong").get()? == 3 {
            if ctx.call(Function::CountItem, vec![Val::from(567)])?.number()? > 19 {
                ctx.lines_as(
                    "Mali the Spicy",
                    args![
                        "Ah, you came back!",
                        "Now, let me see what",
                        "you've brought. Hmm...",
                        "These are..."
                    ],
                )?;
                ctx.next()?;
                ctx.call(Function::Emotion, vec![ctx.constant("ET_BEST")?])?;
                ctx.lines_as(
                    "Mali the Spicy",
                    args![
                        "The freshest Shrimp that",
                        "I've been looking for! Well done~",
                        "With these big and fresh Shrimp,",
                        "I can cook the best Tom Yum Goong for you! Thank you so much!"
                    ],
                )?;
                ctx.call(Function::DelItem, vec![Val::from(567), Val::from(20)])?;
                ctx.next()?;
                ctx.lines_as("Mali the Spicy", args!["It will be a perfect food with the natural sweetness of the Shrimp, the sourness of Lemons and a little bit of fish sauce!"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Mali the Spicy",
                    args![
                        "Oh... Right.",
                        "But we only have Shrimp.",
                        "Now, I want you to bring",
                        "me some ^CE0000Lemons^000000."
                    ],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("Where can I find Lemons?:I hate sour food. I'd better quit!")],
                )?) == 1
                {
                    ctx.var("tomyumgoong").set(Val::from(4))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(8124), Val::from(8125)])?;
                    ctx.lines_as(
                        "Mali the Spicy",
                        args![
                            "A few days ago, I saw a good",
                            "Lemon tree while taking a walk.",
                            "But this tree belongs to an ordinary man who's not a Fruit Merchant."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mali the Spicy",
                        args![
                            "I'm not sure whether",
                            "or not he'd sell Lemons",
                            "to you, but you can try,",
                            "I guess."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.call(Function::Emotion, vec![ctx.constant("ET_ANGER")?])?;
                ctx.lines_as(
                    "Mali the Spicy",
                    args![
                        "What a stupid...!",
                        "If you quit just because you really hate sour flavors, you must not be that eager to have my Tom Yum Goong!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mali the Spicy",
                    args!["Once you taste my", "Tom Yum Goong, you'll", "regret what you just said!"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Mali the Spicy",
                args![
                    "Huh...?",
                    "What did you do",
                    "with the Shrimps?",
                    "You didn't eat them",
                    "on the way here, did you?"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Mali the Spicy",
            args!["Make sure you ask", "Mr. Thongpool to give", "you 20 Shrimps.", "Now hurry~"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("tomyumgoong").get()?.number()? > 3 && ctx.var("tomyumgoong").get()?.number()? < 6) {
        if (ctx.var("tomyumgoong").get()? == 4 && ctx.call(Function::CountItem, vec![Val::from(568)])?.number()? > 9) {
            ctx.lines_as(
                "Mali the Spicy",
                args!["You think these are", "the best quality Lemons?", "Take a closer look!"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Mali the Spicy",
                args![
                    "The skin of these",
                    "Lemons lack luster",
                    "and they don't even smell",
                    "fresh! They're just",
                    "common Lemons!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("tomyumgoong").get()? == 5 {
            if ctx.call(Function::CountItem, vec![Val::from(568)])?.number()? > 9 {
                ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
                ctx.lines_as(
                    "Mali the Spicy",
                    args![
                        "Welcome back!",
                        "Wow, you came back",
                        "really fast! Okay, let me",
                        "see the Lemons",
                        "you brought?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mali the Spicy",
                    args![
                        "Wow, great!",
                        "These are really fresh Lemons!",
                        "How could you bring all of these by yourself? Thanks for your trouble~"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mali the Spicy",
                    args![
                        "Now, you should go buy",
                        "some ^CE0000Chiles^000000 and some",
                        "other materials in the Market."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mali the Spicy",
                    args![
                        "When you talk",
                        "to the Merchants",
                        "in the Market, they",
                        "will know what you want."
                    ],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from(
                        "Okay, I'll be right back.:Ah, I'm tired now. Let me take a rest first...",
                    )],
                )?) == 1
                {
                    ctx.call(Function::DelItem, vec![Val::from(568), Val::from(10)])?;
                    ctx.var("tomyumgoong").set(Val::from(6))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(8125), Val::from(8126)])?;
                    ctx.lines_as("Mali the Spicy", args!["See you in a bit~"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Mali the Spicy",
                    args![
                        "It's your call.",
                        "But remember...",
                        "The Shrimps might go bad",
                        "while you're taking a rest."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Mali the Spicy",
                args![
                    "Huh...?",
                    "What did you",
                    "do with Lemons?",
                    "You didn't eat them",
                    "all on the way here, did you?"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Mali the Spicy",
            args![
                "Alright...",
                "I'm counting on you",
                "to get the best Lemons",
                "from the guy that owns that tree."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Mali the Spicy",
            args![
                "Somehow, you've got",
                "to get those high quality",
                "Lemons so that I can make",
                "some Tom Yum Goong!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("tomyumgoong").get()? == 6 {
        if ctx.call(Function::CountItem, vec![Val::from(7286)])?.number()? > 29 {
            ctx.lines_as(
                "Mali the Spicy",
                args![
                    "I don't understand...",
                    "Where did you get these",
                    "bad ingredients?! Didn't",
                    "I tell you that my",
                    "Tom Yum Goong",
                    "requires the best?!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as("Mali the Spicy", args!["I know you've been working hard to do me this favor. I hope you'll put forth that last little bit of effort to finish this."])?;
        ctx.next()?;
        ctx.lines_as("Malli the Spicy", args!["Please go take a look in the Market. I don't doubt that it will be a good experience. After all, my fantastic Tom Yum Goong is waiting for you!"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("tomyumgoong").get()? == 7 {
        if (ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? > 1499 {
            if ctx.call(Function::CountItem, vec![Val::from(7286)])?.number()? > 29 {
                ctx.call(Function::DelItem, vec![Val::from(7286), Val::from(30)])?;
                ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
                ctx.lines_as(
                    "Mali the Spicy",
                    args![
                        "Now...",
                        "Everything is all set!",
                        "Before we start, let me",
                        "check the materials."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mali the Spicy",
                    args![
                        "^CE0000Shrimp^000000, ^CE0000Chiles^000000, ^CE0000Lemons^000000...",
                        "Fish sauce and other little things..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Mali the Spicy", args!["Perfect!", "Shall we begin?"])?;
                ctx.next()?;
                ctx.lines(args!["^FF0000*Tuk*^000000", "^9C009C*Tup*^000000", "^009C00*Puk*^000000"])?;
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_ENCHANTPOISON")?])?;
                ctx.next()?;
                ctx.lines(args![
                    "^FF0031*Chop chop*^000000",
                    "^FF9C00*Tup tup tup*^000000",
                    "^639CFF*Pu pu^000000 -"
                ])?;
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_VENOMDUST")?])?;
                ctx.next()?;
                ctx.lines_as(
                    "Mali the Spicy",
                    args![
                        "- ^9C6300Tup tup^000000 ^006363chook^000000",
                        "^316363Chop chop^000000",
                        "^FF0000Tup tup tup^000000 ^009C00Pu pu^000000 -"
                    ],
                )?;
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_STONECURSE")?])?;
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_HASTEUP")?])?;
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL")?])?;
                ctx.next()?;
                ctx.lines_as("Mali the Spicy", args!["Wah^00009Cah^0000FFahhhh^0063FFaaaahhh^000000!"])?;
                ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_FLASHER")?])?;
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_LORD")?])?;
                ctx.next()?;
                ctx.lines_as("Mali the Spicy", args![" ...", " ......", " ........."])?;
                ctx.next()?;
                ctx.var("tomyumgoong").set(Val::from(8))?;
                ctx.call(Function::CompleteQuest, vec![Val::from(8126)])?;
                ctx.call(Function::GetItem, vec![Val::from(566), Val::from(10)])?;
                ctx.call(Function::Emotion, vec![ctx.constant("ET_BEST")?])?;
                ctx.lines_as(
                    "Mali the Spicy",
                    args![
                        "Here's your",
                        "Tom Yum Goong!",
                        "Ah~ This taste,",
                        "this scent... Yeah.",
                        "I know I did a good job."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Mali the Spicy", args!["Do you know what is the secret of Tom Yum Goong? It isn't simply just food. This has the power to heal you, and help you get into better shape."])?;
                ctx.next()?;
                ctx.lines_as("Mali the Spicy", args!["Because...", "It contains the", "power of God."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Mali the Spicy",
                    args!["Okay...!", "I'm giving you", "10 Tom Yum Goong", "free of charge~"],
                )?;
                ctx.next()?;
                ctx.lines_as("Mali the Spicy", args!["Thank you for all the trouble that you have been though to get these ingredients. It would make me happy if you shared Tom Yum Goong with your friends."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Mali the Spicy",
                    args![
                        "Share the flavor of Ayothaya, and let them know how beautiful our land is. It was good to see you, take care now~"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Mali the Spicy",
                args!["Huh? What did you do with the Chiles? You didn't eat them all on the way here, did you?"],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Mali the Spicy",
            args![
                "Hmm...",
                "You seem to be carrying too many things. Why don't you free up some space in your inventory first, and then come back?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.call(Function::Emotion, vec![ctx.constant("ET_KEK")?])?;
    ctx.lines_as(
        "Mali the Spicy",
        args![
            "Ah...!",
            "So busy, so busy!",
            "My customers never get tired of",
            "the taste of my Tom Yum Goong!",
            "Heh heh heh~!"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn cook_ayo(ctx: &Ctx) -> Script {
    cook_ayo_body(ctx, Vec::new()).map(|_| ())
}

fn thongpool_ayo_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ((ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 2000
        || ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0)
    {
        ctx.lines(args![
            "- Wait a minute! -",
            "- Currently you're carrying -",
            "- too many items with you. -",
            "- Please enlighten your weight -",
            "- and try again. -"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("tomyumgoong").get()? == 2 {
        ctx.lines_as(
            "Thongpool",
            args!["Come adventurers,", "take a look! I have", "plenty of Shrimp!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Thongpool",
            args![
                "You're looking for Shrimp,",
                "aren't you? How many Shrimp",
                "do you need? Twenty? Forty?"
            ],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from("I need Shrimp for Tom Yum Goong.:Err, they look expensive.")],
        )?) == 1
        {
            ctx.lines_as(
                "Thongpool",
                args![
                    "Ah! I guess Ms. Mali the Spicy recommended me to you. I guess",
                    "that also means you want to buy them in bulk?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Thongpool",
                args!["After all,", "you'll need a larger", "amount of Shrimp to", "cook Tom Yum Goong."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Thongpool",
                args![
                    "Oh yes, you will need 20 Shrimps for Ms. Mali's Tom Yum Goong.",
                    "I'll even give you a discount:",
                    "11,000 zeny for 20 shrimps.",
                    "What do you say?"
                ],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("I will take them.:I could get 100 Jellopies for that much!")],
            )?) == 1
            {
                if ctx.var("Zeny").get()?.number()? > 10999 {
                    if (ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? > 3999 {
                        ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(11000))?))?;
                        ctx.var("tomyumgoong").set(Val::from(3))?;
                        ctx.call(Function::GetItem, vec![Val::from(567), Val::from(20)])?;
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_BEST")?])?;
                        ctx.lines_as(
                            "Thongpool",
                            args![
                                "Good, now you have the",
                                "freshest Shrimp in this village! It's time for you to go back and ask Ms. Mali to cook them for you."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        "Thongpool",
                        args![
                            "Awww...",
                            "You're carrying too much stuff with you. I don't think you can carry more than what you have right now."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as("Thongpool", args!["Awww...", "You don't have", "enough money."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Thongpool",
                args![
                    "Well well well...",
                    "You might fill your hungry stomach with that many Jellopies but they wouldn't taste as good as my Shrimp!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Thongpool",
            args![
                "What...?",
                "Do you think",
                "it's expensive?",
                "But the fantastic taste",
                "of Tom Yum Goong",
                "is priceless!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("tomyumgoong").get()? == 3 {
        ctx.lines_as(
            "Thongpool",
            args!["Mmm...?", "Did you need", "more Shrimps?", "Would you like", "to buy more?"],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Yes!:No, thanks.")])?) == 1 {
            if ctx.var("Zeny").get()?.number()? > 10999 {
                if (ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? > 3999 {
                    ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(11000))?))?;
                    ctx.call(Function::GetItem, vec![Val::from(567), Val::from(20)])?;
                    ctx.lines_as(
                        "Thongpool",
                        args![
                            "There you go~",
                            "The freshest Shrimp you can ever get in this village, and maybe even the world!"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Thongpool",
                    args![
                        "Awww...",
                        "You're carrying too much stuff with you. I don't think you can carry more than what you have right now."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as("Thongpool", args!["Awww...", "You don't have", "enough money."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Thongpool",
            args![
                "Good, now you have gotten the freshest shrimps in this village!",
                "It's time for you to go back and ask Ms. Mali to cook for you."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Thongpool",
        args![
            "Welcome, welcome!",
            "I've got the best Shrimp caught in the cleanest waters in the world~!"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn thongpool_ayo(ctx: &Ctx) -> Script {
    thongpool_ayo_body(ctx, Vec::new()).map(|_| ())
}

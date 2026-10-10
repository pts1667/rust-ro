use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn dequ_ee_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_bang_s = Val::from("");
    let mut l_geil_s = Val::from("");
    let mut l_hans_s = Val::from("");
    let mut l_inputstr_s = Val::from("");
    let mut l_mutr_s = Val::from("");
    if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_NOVICE")?) {
        ctx.lines_as(
            "Dequ'ee",
            args![
                "Hey there.",
                "Frustrated with",
                "being a Novice?",
                "I know, I know,",
                "we've all been",
                "there once."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Dequ'ee", args!["If you want to become stronger,", "why don't you consider becoming a Swordman? It'll require discipline on your part, but that job will shape you into a tough warrior."])?;
        ctx.next()?;
        ctx.lines_as(
            "Dequ'ee",
            args![
                "It's ultimately your decision.",
                "I don't know how you want to live your life, but you should give the Swordman job some serious thought..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_SWORDMAN")?)
        && runtime::op(
            &ctx.call(Function::EaClass, vec![])?,
            "&",
            &runtime::op(&ctx.constant("EAJL_2")?, "|", &ctx.constant("EAJL_UPPER")?)?,
        )?
        .is_true())
    {
        ctx.lines_as("Dequ'ee", args!["Ah...", "A fellow sword wielder!", "How have you been doing?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Dequ'ee",
            args![
                "There must be many who must",
                "depend on the strength of your blade. It's an awesome responsibility, but we can never let our allies down in battle..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if !ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_SWORDMAN")?) {
        ctx.lines_as(
            "Dequ'ee",
            args![
                "Hello there.",
                "Not a fan of swords, are you?",
                "Well, if you were, we'd have",
                "something to talk about."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Dequ'ee",
            args!["Yeah.", "Not much", "conversation", "happening here.", "Pretty boring..."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("tu_swordman").get()?.number()? > 20 {
        ctx.lines_as(
            "Dequ'ee",
            args![
                "I hope you continue",
                "to train yourself in",
                "the ways of the sword.",
                "There's an art to",
                "wielding a blade..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("tu_swordman").get()? == 20 {
        ctx.lines_as(
            "Dequ'ee",
            args!["Huh...?", "Aren't you supposed", "to keep an eye on Bankley?", "What happened?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Dequ'ee",
            args![
                "Suicide...?",
                "Are you serious?!",
                "But there's no need to kill",
                "himself if he suspected that he's been caught. Something shady's going on here!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Dequ'ee",
            args![
                "Well, you've done a good job.",
                "As for the other three suspects, I'll make sure that they're monitored by members of the Swordman Association."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Dequ'ee",
            args![
                "Still, I don't think we'll learn anything new from those three.",
                "It looks like this case is closed for now. Once again, thanks for your help."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Dequ'ee",
            args![
                "Why don't you report back to Shurank now? I guess he still",
                "wants to show you the ropes of Swordmanship."
            ],
        )?;
        ctx.var("tu_swordman").set(Val::from(21))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(8226), Val::from(8227)])?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("izlude"), Val::from(35), Val::from(78)])?;
        return Err(Stop::End);
    }
    if ctx.var("tu_swordman").get()? == 19 {
        ctx.lines_as(
            "Dequ'ee",
            args![
                "What are still doing here?",
                "Hurry and check on Bankley!",
                "There's no telling what",
                "he might be up to!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("tu_swordman").get()? == 17 || ctx.var("tu_swordman").get()? == 18) {
        ctx.mes("[Dequ'ee]")?;
        if ctx.var("tu_swordman").get()? == 17 {
            ctx.lines(args![
                "TheisWesomeof",
                "ConBanfoevidehi",
                "victkleyundncem",
                "hekdlfiDrindkelsd"
            ])?;
            ctx.next()?;
        } else if ctx.var("tu_swordman").get()? == 18 {
            ctx.lines(args![
                "hekdlfiDrindkelsd",
                "TheisWesomeof",
                "ConBanfoevidehi",
                "victkleyundncem"
            ])?;
            ctx.next()?;
        }
        ctx.lines_as(
            "Dequ'ee",
            args![
                "I get it now!",
                "This is a complex transposition cipher in which we remove one person's code from the rest of them. So who do you think is",
                "the murderer?"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Hans:Bankley:Geil:Muetro")])? {
            1 => {
                ctx.lines_as(
                    "Dequ'ee",
                    args![
                        "Hans...?",
                        "That can't be right.",
                        "It doesn't match with",
                        "this algorithm I just",
                        "made up. You've got",
                        "to try it again!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as("Dequ'ee", args!["Bankley...?", "Hey, you might be", "right. Let me check..."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Dequ'ee",
                    args!["Yeah, when you take out Bankley's password from the rest of them and read it all in the right order..."],
                )?;
                ctx.next()?;
                ctx.lines_as("Dequ'ee", args!["Alright! It looks like Bankley is the murderer! Although we can't trust this 100%, this evidence is pretty conclusive."])?;
                ctx.next()?;
                ctx.lines_as("Dequ'ee", args!["Hurry over to Morocc and monitor Bankley in case he does anything desperate. If something happens, report to me right away!"])?;
                ctx.var("tu_swordman").set(Val::from(19))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(8224), Val::from(8225)])?;
                ctx.call(Function::GetExperience, vec![Val::from(1620), Val::from(0)])?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("moc_fild07"), Val::from(359), Val::from(201)])?;
                return Err(Stop::End);
            }
            3 => {
                ctx.lines_as(
                    "Dequ'ee",
                    args![
                        "Geil...?",
                        "That can't be right.",
                        "It doesn't match with",
                        "this algorithm I just",
                        "made up. You've got",
                        "to try it again!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            4 => {
                ctx.lines_as(
                    "Dequ'ee",
                    args![
                        "Muetro...?",
                        "That can't be right.",
                        "It doesn't match with",
                        "this algorithm I just",
                        "made up. You've got",
                        "to try it again!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    if ctx.var("tu_swordman").get()? == 16 {
        ctx.lines_as(
            "Dequ'ee",
            args!["I guess these codes need to be arranged in a certain order before we can figure out its secrets."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Dequ'ee",
            args!["Hm, you guess the order and I'll try to formulate a cryptanalysis based on your guess."],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Muetro:Hans:Geil:Bankley")])? {
            1 => {
                ctx.lines_as(
                    "Dequ'ee",
                    args![
                        "Okay, Muetro's",
                        "code is what we'll",
                        "use first. Now, whose",
                        "code do you think",
                        "should go second?"
                    ],
                )?;
                ctx.next()?;
                'b3: {
                    let subject3 = Val::from(runtime::select_values(ctx, &[Val::from("Hans:Geil:Bankley")])?);
                    let mut matched3 = false;
                    let no_case3 = !subject3.loosely_equals(&Val::from(1))
                        && !subject3.loosely_equals(&Val::from(2))
                        && !subject3.loosely_equals(&Val::from(3));
                    if !matched3 && subject3.loosely_equals(&Val::from(1)) {
                        matched3 = true;
                    }
                    if matched3 {
                        ctx.lines_as(
                            "Dequ'ee",
                            args!["Hans...?", "Alright, then", "the person with", "the third code", "would be...?"],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Geil:Bankley")])? {
                            1 => {
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args!["Muetro, Hans,", "Geil and Bankley.", "So then the full", "code would be..."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![" ", "ConBanfoevidehi", "victkleyundncem", "TheisWesomeof", "hekdlfiDrindkelsd"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![
                                        "No...",
                                        "There's no way",
                                        "I can decode it the",
                                        "way it is now. Let's",
                                        "try another combination."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args!["Muetro, Hans,", "Geil and Bankley.", "So then the full", "code would be..."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![" ", "ConBanfoevidehi", "victkleyundncem", "hekdlfiDrindkelsd", "TheisWesomeof"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![
                                        "No...",
                                        "There's no way",
                                        "I can decode it the",
                                        "way it is now. Let's",
                                        "try another combination."
                                    ],
                                )?;
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
                        ctx.lines_as(
                            "Dequ'ee",
                            args!["Geil's...?", "Alright, then", "the person with", "the third code", "would be...?"],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Hans:Bankley")])? {
                            1 => {
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args!["Muetro, Geil,", "Hans and Bankley.", "So then the full", "code would be..."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![" ", "ConBanfoevidehi", "TheisWesomeof", "victkleyundncem", "hekdlfiDrindkelsd"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![
                                        "No...",
                                        "There's no way",
                                        "I can decode it the",
                                        "way it is now. Let's",
                                        "try another combination."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args!["Muetro, Geil,", "Bankley and Hans.", "So then the full", "code would be..."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![" ", "ConBanfoevidehi", "TheisWesomeof", "hekdlfiDrindkelsd", "victkleyundncem"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![
                                        "No...",
                                        "There's no way",
                                        "I can decode it the",
                                        "way it is now. Let's",
                                        "try another combination."
                                    ],
                                )?;
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
                            "Dequ'ee",
                            args![
                                "Bankley's...?",
                                "Alright, then",
                                "the person with",
                                "the third code",
                                "would be...?"
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Hans:Geil")])? {
                            1 => {
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args!["Muetro, Bankley,", "Hans and Geil.", "So then the full", "code would be..."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![" ", "ConBanfoevidehi", "hekdlfiDrindkelsd", "victkleyundncem", "TheisWesomeof"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![
                                        "No...",
                                        "There's no way",
                                        "I can decode it the",
                                        "way it is now. Let's",
                                        "try another combination."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args!["Muetro, Bankley,", "Geil and Hans.", "So then the full", "code would be..."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![" ", "ConBanfoevidehi", "hekdlfiDrindkelsd", "TheisWesomeof", "victkleyundncem"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![
                                        "No...",
                                        "There's no way",
                                        "I can decode it the",
                                        "way it is now. Let's",
                                        "try another combination."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                }
            }
            2 => {
                ctx.lines_as(
                    "Dequ'ee",
                    args![
                        "Okay, Hans's",
                        "code is what we'll",
                        "use first. Now, whose",
                        "code do you think",
                        "should go second?"
                    ],
                )?;
                ctx.next()?;
                'b7: {
                    let subject7 = Val::from(runtime::select_values(ctx, &[Val::from("Muetro:Geil:Bankley")])?);
                    let mut matched7 = false;
                    let no_case7 = !subject7.loosely_equals(&Val::from(1))
                        && !subject7.loosely_equals(&Val::from(2))
                        && !subject7.loosely_equals(&Val::from(3));
                    if !matched7 && subject7.loosely_equals(&Val::from(1)) {
                        matched7 = true;
                    }
                    if matched7 {
                        ctx.lines_as(
                            "Dequ'ee",
                            args![
                                "Muestro's...?",
                                "Alright, then",
                                "the person with",
                                "the third code",
                                "would be...?"
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Geil:Bankley")])? {
                            1 => {
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args!["Hans, Muetro,", "Geil and Bankley.", "So then the full", "code would be..."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![" ", "victkleyundncem", "ConBanfoevidehi", "TheisWesomeof", "hekdlfiDrindkelsd"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![
                                        "No...",
                                        "There's no way",
                                        "I can decode it the",
                                        "way it is now. Let's",
                                        "try another combination."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args!["Hans, Muetro,", "Bankley and Geil.", "So then the full", "code would be..."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![" ", "victkleyundncem", "ConBanfoevidehi", "hekdlfiDrindkelsd", "TheisWesomeof"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![
                                        "No...",
                                        "There's no way",
                                        "I can decode it the",
                                        "way it is now. Let's",
                                        "try another combination."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                    if !matched7 && subject7.loosely_equals(&Val::from(2)) {
                        matched7 = true;
                    }
                    if matched7 {
                        ctx.lines_as(
                            "Dequ'ee",
                            args!["Geil's...?", "Alright, then", "the person with", "the third code", "would be...?"],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Muetro:Bankley")])? {
                            1 => {
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args!["Hans, Geil,", "Muetro and Bankley.", "So then the full", "code would be..."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![" ", "victkleyundncem", "TheisWesomeof", "ConBanfoevidehi", "hekdlfiDrindkelsd"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![
                                        "No...",
                                        "There's no way",
                                        "I can decode it the",
                                        "way it is now. Let's",
                                        "try another combination."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args!["Hans, Geil,", "Bankley and Muetro.", "So then the full", "code would be..."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![" ", "victkleyundncem", "TheisWesomeof", "hekdlfiDrindkelsd", "ConBanfoevidehi"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![
                                        "No...",
                                        "There's no way",
                                        "I can decode it the",
                                        "way it is now. Let's",
                                        "try another combination."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                    if !matched7 && subject7.loosely_equals(&Val::from(3)) {
                        matched7 = true;
                    }
                    if matched7 {
                        ctx.lines_as(
                            "Dequ'ee",
                            args![
                                "Bankley's...?",
                                "Alright, then",
                                "the person with",
                                "the third code",
                                "would be...?"
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Muetro:Geil")])? {
                            1 => {
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args!["Hans, Bankley,", "Muetro and Geil.", "So then the full", "code would be..."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![" ", "victkleyundncem", "hekdlfiDrindkelsd", "ConBanfoevidehi", "TheisWesomeof"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![
                                        "No...",
                                        "There's no way",
                                        "I can decode it the",
                                        "way it is now. Let's",
                                        "try another combination."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args!["Hans, Bankley,", "Geil and Muetro.", "So then the full", "code would be..."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![" ", "victkleyundncem", "hekdlfiDrindkelsd", "TheisWesomeof", "ConBanfoevidehi"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![
                                        "No...",
                                        "There's no way",
                                        "I can decode it the",
                                        "way it is now. Let's",
                                        "try another combination."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                }
            }
            3 => {
                ctx.lines_as(
                    "Dequ'ee",
                    args![
                        "Okay, Geil's",
                        "code is what we'll",
                        "use first. Now, whose",
                        "code do you think",
                        "should go second?"
                    ],
                )?;
                ctx.next()?;
                'b11: {
                    let subject11 = Val::from(runtime::select_values(ctx, &[Val::from("Muetro:Hans:Bankley")])?);
                    let mut matched11 = false;
                    let no_case11 = !subject11.loosely_equals(&Val::from(1))
                        && !subject11.loosely_equals(&Val::from(2))
                        && !subject11.loosely_equals(&Val::from(3));
                    if !matched11 && subject11.loosely_equals(&Val::from(1)) {
                        matched11 = true;
                    }
                    if matched11 {
                        ctx.lines_as(
                            "Dequ'ee",
                            args![
                                "Muestro's...?",
                                "Alright, then",
                                "the person with",
                                "the third code",
                                "would be...?"
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Hans:Bankley")])? {
                            1 => {
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args!["Geil, Muetro,", "Hans and Bankley.", "So then the full", "code would be..."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![" ", "TheisWesomeof", "ConBanfoevidehi", "victkleyundncem", "hekdlfiDrindkelsd"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![
                                        "No...",
                                        "There's no way",
                                        "I can decode it the",
                                        "way it is now. Let's",
                                        "try... Wait!"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![
                                        "I think that we",
                                        "can make sense of this",
                                        "thing! Give me a while",
                                        "to formulate an algorithm!"
                                    ],
                                )?;
                                ctx.var("tu_swordman").set(Val::from(17))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args!["Geil, Muetro,", "Bankley and Hans.", "So then the full", "code would be..."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![" ", "TheisWesomeof", "ConBanfoevidehi", "hekdlfiDrindkelsd", "victkleyundncem"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![
                                        "No...",
                                        "There's no way",
                                        "I can decode it the",
                                        "way it is now. Let's",
                                        "try another combination."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                    if !matched11 && subject11.loosely_equals(&Val::from(2)) {
                        matched11 = true;
                    }
                    if matched11 {
                        ctx.lines_as(
                            "Dequ'ee",
                            args!["Han's...?", "Alright, then", "the person with", "the third code", "would be...?"],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Muetro:Bankley")])? {
                            1 => {
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args!["Geil, Hans,", "Muetro and Bankley.", "So then the full", "code would be..."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![" ", "TheisWesomeof", "victkleyundncem", "ConBanfoevidehi", "hekdlfiDrindkelsd"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![
                                        "No...",
                                        "There's no way",
                                        "I can decode it the",
                                        "way it is now. Let's",
                                        "try another combination."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args!["Geil, Hans,", "Bankley and Muetro.", "So then the full", "code would be..."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![" ", "TheisWesomeof", "victkleyundncem", "hekdlfiDrindkelsd", "ConBanfoevidehi"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![
                                        "No...",
                                        "There's no way",
                                        "I can decode it the",
                                        "way it is now. Let's",
                                        "try another combination."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                    if !matched11 && subject11.loosely_equals(&Val::from(3)) {
                        matched11 = true;
                    }
                    if matched11 {
                        ctx.lines_as(
                            "Dequ'ee",
                            args![
                                "Bankley's...?",
                                "Alright, then",
                                "the person with",
                                "the third code",
                                "would be...?"
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Muetro:Hans")])? {
                            1 => {
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args!["Geil, Bankley,", "Muetro and Hans.", "So then the full", "code would be..."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![" ", "TheisWesomeof", "hekdlfiDrindkelsd", "ConBanfoevidehi", "victkleyundncem"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![
                                        "No...",
                                        "There's no way",
                                        "I can decode it the",
                                        "way it is now. Let's",
                                        "try another combination."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args!["Geil, Bankley,", "Hans and Muetro.", "So then the full", "code would be..."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![" ", "TheisWesomeof", "hekdlfiDrindkelsd", "victkleyundncem", "ConBanfoevidehi"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![
                                        "No...",
                                        "There's no way",
                                        "I can decode it the",
                                        "way it is now. Let's",
                                        "try another combination."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                }
            }
            4 => {
                ctx.lines_as(
                    "Dequ'ee",
                    args![
                        "Okay, Bankley's",
                        "code is what we'll",
                        "use first. Now, whose",
                        "code do you think",
                        "should go second?"
                    ],
                )?;
                ctx.next()?;
                'b15: {
                    let subject15 = Val::from(runtime::select_values(ctx, &[Val::from("Muetro:Hans:Geil")])?);
                    let mut matched15 = false;
                    let no_case15 = !subject15.loosely_equals(&Val::from(1))
                        && !subject15.loosely_equals(&Val::from(2))
                        && !subject15.loosely_equals(&Val::from(3));
                    if !matched15 && subject15.loosely_equals(&Val::from(1)) {
                        matched15 = true;
                    }
                    if matched15 {
                        ctx.lines_as(
                            "Dequ'ee",
                            args![
                                "Muestro's...?",
                                "Alright, then",
                                "the person with",
                                "the third code",
                                "would be...?"
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Hans:Geil")])? {
                            1 => {
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args!["Bankley, Muetro,", "Hans and Geil.", "So then the full", "code would be..."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![" ", "hekdlfiDrindkelsd", "ConBanfoevidehi", "victkleyundncem", "TheisWesomeof"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![
                                        "No...",
                                        "There's no way",
                                        "I can decode it the",
                                        "way it is now. Let's",
                                        "try another combination."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args!["Bankley, Muetro,", "Geil and Hans.", "So then the full", "code would be..."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![" ", "hekdlfiDrindkelsd", "ConBanfoevidehi", "TheisWesomeof", "victkleyundncem"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![
                                        "No...",
                                        "There's no way",
                                        "I can decode it the",
                                        "way it is now. Let's",
                                        "try another combination."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                    if !matched15 && subject15.loosely_equals(&Val::from(2)) {
                        matched15 = true;
                    }
                    if matched15 {
                        ctx.lines_as(
                            "Dequ'ee",
                            args!["Hans's...?", "Alright, then", "the person with", "the third code", "would be...?"],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Muetro:Geil")])? {
                            1 => {
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args!["Bankley, Hans,", "Muetro and Geil.", "So then the full", "code would be..."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![" ", "hekdlfiDrindkelsd", "victkleyundncem", "ConBanfoevidehi", "TheisWesomeof"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![
                                        "No...",
                                        "There's no way",
                                        "I can decode it the",
                                        "way it is now. Let's",
                                        "try another combination."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![
                                        "Bankley, Hans,",
                                        "Geil and Muetro.",
                                        "So then the full",
                                        "code would be...",
                                        " ",
                                        "hekdlfiDrindkelsd",
                                        "victkleyundncem",
                                        "TheisWesomeof",
                                        "ConBanfoevidehi"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![
                                        "No...",
                                        "There's no way",
                                        "I can decode it the",
                                        "way it is now. Let's",
                                        "try another combination."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                    if !matched15 && subject15.loosely_equals(&Val::from(3)) {
                        matched15 = true;
                    }
                    if matched15 {
                        ctx.lines_as(
                            "Dequ'ee",
                            args!["Geil's...?", "Alright, then", "the person with", "the third code", "would be...?"],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Muetro:Hans")])? {
                            1 => {
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args!["Bankley, Geil,", "Muetro, and Hans.", "So then the full", "code would be..."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![" ", "hekdlfiDrindkelsd", "TheisWesomeof", "ConBanfoevidehi", "victkleyundncem"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![
                                        "No...",
                                        "There's no way",
                                        "I can decode it the",
                                        "way it is now. Let's",
                                        "try... Wait!"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![
                                        "I think that we",
                                        "can make sense of this",
                                        "thing! Give me a while",
                                        "to formulate an algorithm!"
                                    ],
                                )?;
                                ctx.var("tu_swordman").set(Val::from(18))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args!["Bankley, Geil,", "Hans, Muetro.", "So then the full", "code would be..."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![" ", "hekdlfiDrindkelsd", "TheisWesomeof", "victkleyundncem", "ConBanfoevidehi"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![
                                        "No...",
                                        "There's no way",
                                        "I can decode it the",
                                        "way it is now. Let's",
                                        "try another combination."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                }
            }
            _ => {}
        }
    }
    if ctx.var("tu_swordman").get()? == 15 {
        l_hans_s = Val::from("victkleyundncem");
        l_bang_s = Val::from("hekdlfiDrindkelsd");
        l_mutr_s = Val::from("ConBanfoevidehi");
        l_geil_s = Val::from("TheisWesomeof");
        ctx.lines_as(
            "Dequ'ee",
            args![
                "Ah, you're back.",
                "I've just got a new",
                "lead on the possible",
                "identity of the killer."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Dequ'ee",
            args![
                "I've just heard that all our suspects have pieces of some",
                "sort of code. If we can put the code together and decipher it, we can figure out who the murderer might be."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Dequ'ee",
            args![
                "Did you get all the codes",
                "from all the suspects? First,",
                "tell me the code that Hans had."
            ],
        )?;
        ctx.next()?;
        let (input, status) = runtime::input_text(ctx, None, None)?;
        l_inputstr_s = input;
        if l_inputstr_s.clone().loosely_equals(&l_hans_s.clone()) {
            ctx.lines_as(
                "Dequ'ee",
                args![
                    "victkleyundncem?",
                    "That's certainly",
                    "strange sounding.",
                    "Now, tell me Bankley's."
                ],
            )?;
            ctx.next()?;
            let (input, status) = runtime::input_text(ctx, None, None)?;
            l_inputstr_s = input;
            if l_inputstr_s.clone().loosely_equals(&l_bang_s.clone()) {
                ctx.lines_as(
                    "Dequ'ee",
                    args![
                        "hekdlfiDrindkelsd..",
                        "What the hell is that...?",
                        "It's certainly cryptic.",
                        "What about Muetro's?"
                    ],
                )?;
                ctx.next()?;
                let (input, status) = runtime::input_text(ctx, None, None)?;
                l_inputstr_s = input;
                if l_inputstr_s.clone().loosely_equals(&l_mutr_s.clone()) {
                    ctx.lines_as(
                        "Dequ'ee",
                        args![
                            "ConBanfoevidehi.",
                            "This is going to",
                            "be tough to figure",
                            "out. Alright, now",
                            "tell me Geil's."
                        ],
                    )?;
                    ctx.next()?;
                    let (input, status) = runtime::input_text(ctx, None, None)?;
                    l_inputstr_s = input;
                    if l_inputstr_s.clone().loosely_equals(&l_geil_s.clone()) {
                        ctx.lines_as(
                            "Dequ'ee",
                            args![
                                "TheisWesomeof...",
                                "Alright, great.",
                                "Now all we have to",
                                "do is figure out what",
                                "all of this means."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Dequ'ee",
                            args![
                                "This is going to",
                                "be really difficult.",
                                "Do you have any ideas?",
                                "We have to figure this",
                                "out, it's the only clue",
                                "that we have..."
                            ],
                        )?;
                        ctx.var("tu_swordman").set(Val::from(16))?;
                        ctx.call(Function::ChangeQuest, vec![Val::from(8223), Val::from(8224)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Dequ'ee",
                            args!["...Are you sure that you heard", "him right? Why don't you go check it out again?"],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                } else {
                    ctx.lines_as(
                        "Dequ'ee",
                        args!["...Are you sure that you heard", "him right? Why don't you go check it out again?"],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            } else {
                ctx.lines_as(
                    "Dequ'ee",
                    args!["...Are you sure that you heard", "him right? Why don't you go check it out again?"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else {
            ctx.lines_as(
                "Dequ'ee",
                args![
                    "Are you sure that's right?",
                    "No, no, I don't think it is.",
                    "Would you check that code and interrogate the suspects again",
                    "if you need to?"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if ctx.var("tu_swordman").get()? == 14 {
        ctx.lines_as(
            "Dequ'ee",
            args![
                "Didn't I ask you",
                "to interrogate the",
                "suspects over in Morocc?",
                "We need to finish this",
                "investigation as soon",
                "as possible!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("tu_swordman").get()? == 13 {
        ctx.lines_as(
            "Dequ'ee",
            args![
                "Ah, you're here!",
                "Shurank must have",
                "asked you to help me.",
                "I really appreciate that."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Dequ'ee", args!["As you've probably figured out, I'm currently investigating a murder. Basically, I want you to help me figure out who the killer is."])?;
        ctx.next()?;
        ctx.lines_as(
            "Dequ'ee",
            args!["As for the victim, well, that's classified information. You understand, don't you?"],
        )?;
        ctx.next()?;
        ctx.lines_as("Dequ'ee", args!["Now, I want you to go to Morocc and interrogate four suspects. Their names are ^5D478BMuetro^000000, ^5D478BGeil^000000, ^5D478BHans^000000", "and ^5D478BBankley^000000. It shouldn't be too difficult."])?;
        ctx.next()?;
        ctx.lines_as("Dequ'ee", args!["After you speak to all of them, come back and report to me. And it's especially important that you remember or write down any important clues you might get", "from our suspects."])?;
        ctx.next()?;
        ctx.lines_as("Dequ'ee", args!["Also, keep in mind that this is a special mission for the Swordman Association. So keep everything you learn a secret. Understood?"])?;
        ctx.next()?;
        ctx.lines_as("Dequ'ee", args!["Now go and see if you can learn anything from Muetro, Geil, Hans and Bankley. We've got to figure out the killer's identity as soon as we can!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Dequ'ee",
            args![
                "Oh wait...",
                "Morocc is a little far. Alright, I'll drop you off somewhere",
                "near that city."
            ],
        )?;
        ctx.next()?;
        ctx.var("tu_swordman").set(Val::from(14))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(8222), Val::from(8223)])?;
        ctx.call(Function::Warp, vec![Val::from("moc_fild07"), Val::from(359), Val::from(201)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("tu_swordman").get()?.number()? > 7 && ctx.var("tu_swordman").get()?.number()? < 13) {
        ctx.lines_as(
            "Dequ'ee",
            args!["Hmm...", "Still receiving", "special training", "from Shurank?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Dequ'ee",
            args![
                "That's great.",
                "Swordmen nowadays are getting sloppier and sloppier. Someone has to keep the new guys in line and on their toes!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("tu_swordman").get()? == 7 {
        ctx.lines_as(
            "Dequ'ee",
            args!["What are you", "still doing here?", "Go and deliver my", "message to Shurank."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Dequ'ee",
            args![
                "Let him now that we aren't",
                "sure of the killer's identity,",
                "but we have a list of suspects.",
                "Sooner or later, we'll figure",
                "out who he is."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("tu_swordman").get()? == 6 {
        ctx.lines_as(
            "Dequ'ee",
            args!["Ah...!", "Are you the Swordman", "sent by Shurank? Good,", "I've been waiting..."],
        )?;
        ctx.next()?;
        ctx.lines_as("Dequ'ee", args!["Now tell me...", "What message does", "Shurank have for me?"])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Killer...:Murderer...")])? {
            1 => {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["What happened", "to the killer?"],
                )?;
                ctx.next()?;
                'b20: {
                    let subject20 = Val::from(runtime::select_values(ctx, &[Val::from("Who he is...:Who is behind...")])?);
                    let mut matched20 = false;
                    let no_case20 = !subject20.loosely_equals(&Val::from(1)) && !subject20.loosely_equals(&Val::from(2));
                    if !matched20 && subject20.loosely_equals(&Val::from(1)) {
                        matched20 = true;
                    }
                    if matched20 {
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["Did you find", "out who he is?", "If you did..."],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Why are we...:What are we...")])? {
                            1 => {
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args!["Why are sitting", "around, doing nothing?!"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![
                                        "So Shurank is asking,",
                                        "'What happened to the killer?",
                                        "Did you find out who he is? If you did, why are we sitting around, doing nothing?'"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![
                                        "Hmm...",
                                        "That doesn't seem to",
                                        "be the message I'm supposed to receive from Shurack. I need his exact message or I can't send",
                                        "a response..."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args!["What are we", "supposed to do now?"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![
                                        "So Shurank is asking,",
                                        "'What happened to the killer?",
                                        "Did you find out who he is? If you did, what are we supposed to do now?'"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![
                                        "Hmm...",
                                        "That doesn't seem to",
                                        "be the message I'm supposed to receive from Shurack. I need his exact message or I can't send",
                                        "a response..."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                    if !matched20 && subject20.loosely_equals(&Val::from(2)) {
                        matched20 = true;
                    }
                    if matched20 {
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["Did you figure out", "who's behind all this?", "If you did..."],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Why are we...:What are we...")])? {
                            1 => {
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args!["Why are sitting", "around, doing nothing?!"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![
                                        "So Shurank is asking,",
                                        "'What happened to the killer?",
                                        "Did you figure out who's behind all this? If you did, why are we sitting around, doing nothing?'"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![
                                        "Hmm...",
                                        "That doesn't seem to",
                                        "be the message I'm supposed to receive from Shurack. I need his exact message or I can't send",
                                        "a response..."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args!["What are we", "supposed to do now?"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![
                                        "So Shurank is asking,",
                                        "'What happened to the killer?",
                                        "Did you figure out who's behind all this? If you did, what are we supposed to do now?'"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![
                                        "Hmm...",
                                        "That doesn't seem to",
                                        "be the message I'm supposed to receive from Shurack. I need his exact message or I can't send",
                                        "a response..."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                }
            }
            2 => {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["What happened", "to the murderer?"],
                )?;
                ctx.next()?;
                'b23: {
                    let subject23 = Val::from(runtime::select_values(ctx, &[Val::from("Who he is...:Who is behind...")])?);
                    let mut matched23 = false;
                    let no_case23 = !subject23.loosely_equals(&Val::from(1)) && !subject23.loosely_equals(&Val::from(2));
                    if !matched23 && subject23.loosely_equals(&Val::from(1)) {
                        matched23 = true;
                    }
                    if matched23 {
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["Did you find", "out who he is?", "If you did..."],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Why are we...:What are we...")])? {
                            1 => {
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args!["Why are sitting", "around, doing nothing?!"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![
                                        "So Shurank is asking,",
                                        "'What happened to the",
                                        "murderer? Did you find",
                                        "out who he is? If you did,",
                                        "why are we sitting around,",
                                        "doing nothing?'"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![
                                        "Hmm...",
                                        "That doesn't seem to",
                                        "be the message I'm supposed to receive from Shurack. I need his exact message or I can't send",
                                        "a response..."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args!["What are we", "supposed to do now?"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![
                                        "So Shurank is asking,",
                                        "'What happened to the",
                                        "murderer? Did you find",
                                        "out who he is? If you did,",
                                        "what are we supposed to do now?'"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![
                                        "Hmm...",
                                        "I understand",
                                        "his concerns.",
                                        "Alright, now please",
                                        "give him this answer."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![
                                        "We haven't found",
                                        "out who the murderer",
                                        "is for sure. However,",
                                        "we have a list of suspects",
                                        "and we'll figure it out soon."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![
                                        "Thank you for taking the trouble to come this far. Let me reward you with some experience points."
                                    ],
                                )?;
                                ctx.var("tu_swordman").set(Val::from(7))?;
                                ctx.call(Function::ChangeQuest, vec![Val::from(8216), Val::from(8217)])?;
                                ctx.call(Function::GetExperience, vec![Val::from(1120), Val::from(0)])?;
                                ctx.next()?;
                                ctx.lines_as("Dequ'ee", args!["Take care", "of yourself,", "brave Swordman."])?;
                                ctx.next()?;
                                ctx.call(Function::Warp, vec![Val::from("izlude"), Val::from(35), Val::from(78)])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                    if !matched23 && subject23.loosely_equals(&Val::from(2)) {
                        matched23 = true;
                    }
                    if matched23 {
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["Did you figure out", "who's behind all this?", "If you did..."],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Why are we...:What are we...")])? {
                            1 => {
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args!["Why are sitting", "around, doing nothing?!"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![
                                        "So Shurank is asking,",
                                        "'What happened to the",
                                        "murderer? Did you figure",
                                        "out who's behind all this?",
                                        "If you did, why are we sitting around, doing nothing?'"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![
                                        "Hmm...",
                                        "That doesn't seem to",
                                        "be the message I'm supposed to receive from Shurack. I need his exact message or I can't send",
                                        "a response..."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args!["What are we", "supposed to do now?"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![
                                        "So Shurank is asking,",
                                        "'What happened to the",
                                        "murderer? Did you figure",
                                        "out who's behind all this?",
                                        "If you did, what are we",
                                        "supposed to do now?"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dequ'ee",
                                    args![
                                        "Hmm...",
                                        "That doesn't seem to",
                                        "be the message I'm supposed to receive from Shurack. I need his exact message or I can't send",
                                        "a response..."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                }
            }
            _ => {}
        }
    }
    ctx.lines(args!["Hmm...?", "Do you have any", "business with me?"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn dequ_ee(ctx: &Ctx) -> Script {
    dequ_ee_body(ctx, Vec::new()).map(|_| ())
}

fn geil_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.mes("[Geil]")?;
    if ctx.var("tu_swordman").get()? == 15 {
        ctx.mes("Were you sent by the authorities? Quit giving me such a hard time! I'm not the murderer! I don't even know who was killed. I'm not the one you're looking for!")?;
        ctx.next()?;
        ctx.lines_as(
            "Geil",
            args![
                "Sure, some weird",
                "guy told me some",
                "kind of code related to all this, but that's it! That's all I know! I'm not the killer, I'm not an accessory, nothing!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Geil",
            args![
                "^5D478BTheisWesomeof^000000",
                "That's the code. I can't believe that I'm unable to forget this.",
                "I mean, I think I was drunk",
                "when that guy told me."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Geil",
            args!["What's so important about this code, and why do you guys keep hounding me about the murder?!"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("tu_swordman").get()? == 14 {
        ctx.mes("Were you sent by the authorities? Quit giving me such a hard time! I'm not the murderer! I don't even know who was killed. I'm not the one you're looking for!")?;
        ctx.next()?;
        ctx.lines_as(
            "Geil",
            args![
                "Sure, some weird",
                "guy told me some",
                "kind of code related to all this, but that's it! That's all I know! I'm not the killer, I'm not an accessory, nothing!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Geil",
            args![
                "^5D478BTheisWesomeof^000000",
                "That's the code. I can't believe that I'm unable to forget this.",
                "I mean, I think I was drunk",
                "when that guy told me."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Geil",
            args!["What's so important about this code, and why do you guys keep hounding me about the murder?!"],
        )?;
        ctx.var("tu_swordman").set(Val::from(15))?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines(args![
        "We should all",
        "strive to live in",
        "peace and harmony",
        "with our fellow man."
    ])?;
    ctx.next()?;
    ctx.lines_as("Geil", args!["But fear, distrust and panic always seem to get in the way and mess up world events. We've got to find it within ourselves to get along with others. Don't you agree?"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn geil(ctx: &Ctx) -> Script {
    geil_body(ctx, Vec::new()).map(|_| ())
}

fn muetro_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.mes("[Muetro]")?;
    if ctx.var("tu_swordman").get()? == 15 {
        ctx.lines(args![
            "You're just",
            "like all the others.",
            "You want the code",
            "I know, right?"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Muetro",
            args![
                "^5D478BConBanfoevidehi^000000",
                "Happy now? Now",
                "leave me alone and",
                "catch the murderer."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("tu_swordman").get()? == 14 {
        ctx.lines(args![
            "You're just",
            "like all the others.",
            "You want the code",
            "I know, right?"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Muetro",
            args![
                "^5D478BConBanfoevidehi^000000",
                "Happy now? Now",
                "leave me alone and",
                "catch the murderer."
            ],
        )?;
        ctx.var("tu_swordman").set(Val::from(15))?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines(args![
        "^666666*Sigh*^000000",
        "I try and I try, but sometimes ridiculous things happen and",
        "I can't do anything about them."
    ])?;
    ctx.next()?;
    ctx.lines_as(
        "Muetro",
        args![
            "I suppose it's fate. There's no escaping what I can't possibly",
            "hope to control or understand."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn muetro(ctx: &Ctx) -> Script {
    muetro_body(ctx, Vec::new()).map(|_| ())
}

fn hans_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.mes("[Hans]")?;
    if ctx.var("tu_swordman").get()? == 15 {
        ctx.lines(args![
            "Help me!",
            "Please help me!",
            "I'm no murderer!",
            "I'm practically",
            "harmless!"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Hans",
            args![
                "^5D478Bvictkleyundncem^000000",
                "This is code I know.",
                "Some strange person",
                "told it to me earlier and said",
                "that I'd need to know it soon!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Hans",
            args![
                "He said that this code",
                "would save my life. First,",
                "I thought he was crazy, but for",
                "some reason I can't forget it."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Hans",
            args!["It's like he used magic or something to make it stick out in my brain. Please leave me alone, this is all I know!"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("tu_swordman").get()? == 14 {
        ctx.lines(args![
            "Help me!",
            "Please help me!",
            "I'm no murderer!",
            "I'm practically",
            "harmless!"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Hans",
            args![
                "^5D478Bvictkleyundncem^000000",
                "This is code I know.",
                "Some strange person",
                "told it to me earlier and said",
                "that I'd need to know it soon!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Hans",
            args![
                "He said that this code",
                "would save my life. First,",
                "I thought he was crazy, but for",
                "some reason I can't forget it."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Hans",
            args!["It's like he used magic or something to make it stick out in my brain. Please leave me alone, this is all I know!"],
        )?;
        ctx.var("tu_swordman").set(Val::from(15))?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines(args![
        "I'm sorry...",
        "But I don't think I can be of any help to you. Right now, it seems that I've got problems of my own..."
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn hans(ctx: &Ctx) -> Script {
    hans_body(ctx, Vec::new()).map(|_| ())
}

use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
pub(super) enum PrinceAnotherErnStep {
    Start,
    OnInit,
    OnEnable,
    OnDisable,
}

pub(super) fn prince_another_ern_run(ctx: &Ctx, mut step: PrinceAnotherErnStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            PrinceAnotherErnStep::Start => {
                if ctx.var("nkprince_eisen").get()? == 13 {
                    ctx.lines_as(
                        "Ahrum",
                        args!["I told you that I don't want to be king, and I don't have any intention to change my mind!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ernst",
                        args!["Brother! I don't know why you are suffering so much! Are you this weak a creature?!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ahrum",
                        args!["Suffering? Me?..Aaahhh... It looks so...? Yeahh... Yes, it does. Stressful...Huhuhu...Hahaha!!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Ernst", args!["Brother, Ahrum...?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ahrum",
                        args![
                            "...Is it only me that gets away... Anyhow, I am a disqualified person ... huhuhu. But, I've made a decision."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Ahrum", args!["Ernst."])?;
                    ctx.next()?;
                    ctx.lines_as("Ernst", args!["Yes?... Yes?"])?;
                    ctx.next()?;
                    ctx.lines_as("Ahrum", args!["Take this."])?;
                    ctx.next()?;
                    ctx.mes("- swish. - ")?;
                    ctx.next()?;
                    ctx.mes("- Ahrum casually throws a Bazerald to Ernst.-")?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ernst",
                        args!["Eh?... What's this about?... A Bazerald of Walter family?...Why?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ahrum",
                        args!["Now this is perfect timing. Even our witness is here! Ern, you remember our promise clearly, right?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Ernst", args!["What? Promise? Witness?...Ahh... Re-really..."])?;
                    ctx.next()?;
                    ctx.lines_as("Ahrum", args!["Yes, really. Now is the time."])?;
                    ctx.next()?;
                    ctx.lines_as("Ernst", args!["Nonsense! Do you think I can do that to you?!"])?;
                    ctx.next()?;
                    ctx.lines_as("Ahrum", args!["Idiot! Being King should be followed by decisive action. I have no chances to be King. Corruption, and living like that, is worse than being killed by you..."])?;
                    ctx.next()?;
                    ctx.lines_as("Ernst", args!["B-but, you can live as you are! As if nothing happened. You are just needed to return to the way you were before. Brother, let me help you. Tell me everything you hide..."])?;
                    ctx.next()?;
                    ctx.lines_as("Ahrum", args!["This is so moronic! Can you say that you are kingly?! Are you showing me sympathy now? You should be a man of sense! If not, you are not eligible as a king candidate!"])?;
                    ctx.next()?;
                    ctx.lines_as("Ernst", args!["Bu...but!"])?;
                    ctx.next()?;
                    ctx.lines_as("Ahrum", args!["Talking back and forth is of no use! I cannot help it. If you keep insisting instead of trying to be a man of sense, then such a person should not be a king."])?;
                    ctx.next()?;
                    ctx.lines_as("Ahrum", args!["Ghh?! Brother?!"])?;
                    ctx.next()?;
                    ctx.mes("(dagger thrusting sound)")?;
                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_HIT2")?])?;
                    ctx.var("nkprince_eisen").set(Val::from(14))?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Prince#another_ern::OnDisable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Prince#another_ern1::OnEnable")])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ctx.var("nkprince_eisen").get()? == 5 {
                    ctx.lines_as(
                        "Ahrum",
                        args![
                            "Whoever be the king,",
                            "let's make Rune-Midgarts...",
                            "the best country in the world, by helping each other."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ernst",
                        args![
                            "We can make it.",
                            "No, we shall make it. For Brother and I together, we shall make it."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ahrum",
                        args!["But... If I am to reign, or help you; and if I am corrupted... you kill me... by your hand."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Ahrum", args!["In the opposite case, I will kill you too, you understand? So don't ever stray, and do not ever forget your determination."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ernst",
                        args![
                            "Brother, you think about hell.",
                            "It will never happen, so...",
                            "Don't talk that way."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Ahrum", args!["It is an assumption.", "Don't take it too seriously."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ernst",
                        args![
                            "Alright, alright. Don't",
                            "take it the wrong way.",
                            "I have been acting for Brother and Kingdom, and I will be the same ever after."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
                    ctx.lines_as(
                        "Ahrum",
                        args![
                            "Ah, the appraiser has come.",
                            "How could you appear so suddenly? Ern and I were in the middle of conversation."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ernst",
                        args!["What's wrong, brother? We were not talking about anything bad. We were just making our resolution firm."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ahrum",
                        args![
                            "You came here in good timing.",
                            "We've just decided to kill",
                            "the other... if one of us is",
                            "corrupt."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Ernst", args!["Brother?!"])?;
                    ctx.next()?;
                    ctx.lines_as("Ahrum", args!["By all means, be the witness of this agreement. You cannot say no. Because it is a very good thing, in some respects, for your work, isn't it?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ernst",
                        args!["Br...Brother. You are way too suppressive. Our appraiser must have his own position and opinion."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Ahrum", args!["You still have a weak personality. I am not forcing something so difficult on him! Who cares? I take silence as a positive answer! My bid is successful! And our promise is formed too!"])?;
                    ctx.next()?;
                    ctx.lines_as("Ernst", args!["Haah...haha... Appraiser, I deeply apologize for offending you. Sorry, but there is no possibility of fulfilling this promise. So, just don't take this situation seriously... You know... When pigs fly... ~"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ernst",
                        args!["Now, let me return to my room. Keep up the good work, appraiser."],
                    )?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Prince#another_ern::OnDisable")])?;
                    ctx.var("nkprince_eisen").set(Val::from(6))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                step = PrinceAnotherErnStep::OnInit;
                continue 'machine;
            }
            PrinceAnotherErnStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Prince#another_ern")])?;
                return Err(Stop::End);
            }
            PrinceAnotherErnStep::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Prince#another_ern")])?;
                return Err(Stop::End);
            }
            PrinceAnotherErnStep::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("Prince#another_ern")])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum PrinceAnotherErn1Step {
    Start,
    OnInit,
    OnEnable,
    OnDisable,
}

pub(super) fn prince_another_ern1_run(ctx: &Ctx, mut step: PrinceAnotherErn1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            PrinceAnotherErn1Step::Start => {
                if ctx.var("nkprince_eisen").get()? == 14 {
                    ctx.lines_as("Ernst", args!["Bbb... Brother?... You... told me you would kill me... You just wanted to be killed by me??... Bbb...brother?..."])?;
                    ctx.next()?;
                    ctx.lines_as("Ahrum", args!["Huhu... Not at all. I just wanted to kill you... That's it... Good job, Ern... This is legal, self-defense; killing a villain... right?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ernst",
                        args!["B..Brother... even now it is not too late! If you go to a medic, you can be restored!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Ahrum", args!["It's too late... Once your vital organs are stabbed... I wonder how could I stay alive... or even how I could come back to life... I don't need to live."])?;
                    ctx.next()?;
                    ctx.lines_as("Ernst", args!["B..Brother....How...how could you?!"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ahrum",
                        args!["Goodbye, my brother... Be King, and change this nation. You... You can make it...guk..."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ernst",
                        args!["...Brother...Brother... Why... Why... How can your face look so satisfied? How...? Brother..."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Ahrum", args!["Huhu...You don't need to know that. By the way, you kept your promise... You should be king... But if you are lost, and stray, I will come kill you at any time... from hell!!"])?;
                    ctx.next()?;
                    ctx.lines_as("Ernst", args!["I...I don't know! I don't know what's going on!"])?;
                    ctx.next()?;
                    ctx.lines_as("Ahrum", args!["You don't need to... What you have to know about is that there was a sacrifice from me. To make my death meaningful, you should be a good king. That's my conviction... That's it..guk..! Don't lose my words... gukkuk."])?;
                    ctx.next()?;
                    ctx.lines_as("Ernst", args!["Idiot... Moron..."])?;
                    ctx.next()?;
                    ctx.lines_as("Ahrum", args!["Ahh... Appraiser, do you know what you have to do? Be mindful of your words. About what you saw, what you heard... You must not tell all of your experience to the inspector."])?;
                    ctx.next()?;
                    ctx.lines_as("Ahrum", args!["Ern killed a villain here who bullied around.. I had no chance... He showed decisive action, and that will deserve him the right to be king..."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ahrum",
                        args!["If you react wrongfully to this incident, my death will be worthless...kuk... Do you understand?"],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("... Okay I will follow your will...:......")])? {
                        1 => {
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["... Okay, I will follow your will. Don't worry..."],
                            )?;
                            ctx.next()?;
                        }
                        2 => {
                            ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["......"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Ahrum",
                                args!["Ignorance... is positive... It means... My bid... is successful..."],
                            )?;
                            ctx.next()?;
                        }
                        _ => {}
                    }
                    ctx.lines_as(
                        "Ahrum",
                        args!["Now... I can die... with peace... Thank... you very... much, both... of you."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Ernst", args!["Brother..."])?;
                    ctx.next()?;
                    ctx.lines_as("Ahrum", args!["Then, perpetually... Bye... Sorry ... now I can't go fishing."])?;
                    ctx.next()?;
                    ctx.lines_as("Ernst", args!["B...bro?"])?;
                    ctx.next()?;
                    ctx.lines_as("Ahrum", args!["......"])?;
                    ctx.next()?;
                    ctx.lines_as("Ernst", args!["B... bro, brother!"])?;
                    ctx.next()?;
                    ctx.lines_as("Ahrum", args!["......"])?;
                    ctx.next()?;
                    ctx.lines_as("Ernst", args!["Brotherrrrrrrr!!!"])?;
                    ctx.next()?;
                    ctx.var("nkprince_eisen").set(Val::from(15))?;
                    ctx.var("nk_prince").set(Val::from(7))?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Prince#another_ern1::OnDisable")])?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(10024), Val::from(10025)])?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("prt_castle"), Val::from(318), Val::from(273)])?;
                    return Err(Stop::End);
                }
                step = PrinceAnotherErn1Step::OnInit;
                continue 'machine;
            }
            PrinceAnotherErn1Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Prince#another_ern1")])?;
                return Err(Stop::End);
            }
            PrinceAnotherErn1Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Prince#another_ern1")])?;
                return Err(Stop::End);
            }
            PrinceAnotherErn1Step::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("Prince#another_ern1")])?;
                return Err(Stop::End);
            }
        }
    }
}

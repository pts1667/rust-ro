use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn herico_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_inputstr_s = Val::from("");
    if ctx.call(Function::CheckWeight, vec![Val::from(7342), Val::from(1)])? != 1 {
        ctx.lines_as(
            "Herico",
            args!["Why don't you go take some load off your shoulder first, and come back?"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.call(Function::Cutin, vec![Val::from("heri1.bmp"), Val::from(2)])?;
    if ctx.var("hg_tre").get()?.number()? < 41 {
        ctx.lines_as(
            "Herico",
            args!["The weather is beautiful today. *Sigh* But my body aches too badly to go out..."],
        )?;
        ctx.close_window()?;
        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
        return Err(Stop::End);
    } else {
        if ctx.var("hg_tre").get()? == 41 {
            ctx.lines_as(
                "Herico",
                args!["The weather is beautiful today. *Sigh* But my body aches too badly to go out..."],
            )?;
            ctx.next()?;
            ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Hello!"])?;
            ctx.next()?;
            ctx.lines_as("Herico", args!["Shede?"])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    ((Val::from("Oh, hello. Ms. Shede asked me to drop you by. Nice to meet you, I am ^3131FF")
                        + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                        + Val::from("^000000."))
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Herico",
                args![
                    "Ah, hello. I heard that recently an airport was built in this town. So, are you from a different city?",
                    "I like to see new things...although I don't like this chilly wind.",
                    "How do you like Hugel?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "It is a peaceful and beautiful village, although I can tell that it is experiencing a change.",
                    "Everyone looks happy, and they feel very different than people in the other areas.",
                    "So I feel like that I am in a different land."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Herico",
                args![
                    "Everyone has worries and happyness at the same time.",
                    "You must have travelled many cities of this country,",
                    "and have figured out many things on your own.",
                    "Am I right?"
                ],
            )?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Yes, you are right.:I am not so sure.:No.")])? {
                1 => {
                    ctx.call(Function::Cutin, vec![Val::from("heri2.bmp"), Val::from(2)])?;
                    ctx.lines_as(
                        "Herico",
                        args![
                            "I think that my senses are still working fine.",
                            "People's eyes tell many things to me."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("heri1.bmp"), Val::from(2)])?;
                    ctx.lines_as(
                        "Herico",
                        args![
                            "Your eyes are telling me your admiration and astonishment",
                            "toward what you have seen in Hugel,",
                            "which is a totally different experience for you."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Herico",
                        args![
                            "People of Hugel don't question anyone.",
                            "It is an unwritten law of this village.",
                            "That is why I could have been resting in this village without worries."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Herico",
                        args![
                            "However, as I see you, I suddenly have an urge to ruminate on my past",
                            "that I have never spoken to anyone.",
                            "May I have a moment of your time, if you don't mind?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Herico", args!["Perhaps we will learn something from each other's story."])?;
                    ctx.var("hg_tre").set(Val::from(42))?;
                    ctx.close_window()?;
                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines_as(
                        "Herico",
                        args![
                            "Then I guess that you have not been deeply impressed by this country.",
                            "Haha, alright, that's fine."
                        ],
                    )?;
                    ctx.close_window()?;
                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                    return Err(Stop::End);
                }
                3 => {
                    ctx.lines_as("Herico", args!["Oh, am I not?", "Haha, I am sorry for hazarding a conjecture."])?;
                    ctx.close_window()?;
                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        } else {
            if ctx.var("hg_tre").get()? == 42 {
                ctx.lines_as("Herico", args!["Hmm...can you tell me your name once again?"])?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        ((Val::from("Yes, my name is ^3131FF") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                            + Val::from("^000000."))
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Herico",
                    args![
                        ((Val::from("^3131FF") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                            + Val::from("^000000, I have many interesting stories for you.")),
                        "However, before I tell you my stories, I would like to ask you something. Do you mind?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Herico", args!["As you see, ever since an airport was built in this Hugel, many strangers are coming in and going out this village.", "It may be a good change for Hugel, so that more people will know about its existence.", "However, I have this bad feeling about the airport and airships.", "I do not think that this country is not extending their airlines only for the transportation purposes."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Herico",
                    args![
                        "Did you know that more and more people are using the ferry after the airport was built?",
                        "Can't you see the reason of my doubt?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Herico",
                    args![
                        "Please find out the secret about the airship and the island.",
                        "Then I will tell you my stories.",
                        "I hope that you will do your best to find out the secret."
                    ],
                )?;
                ctx.var("hg_tre").set(Val::from(43))?;
                ctx.close_window()?;
                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                return Err(Stop::End);
            } else {
                if ctx.var("hg_tre").get()? == 43 {
                    if ctx.var("hg_odin").get()? == 60 {
                        ctx.lines_as(
                            "Herico",
                            args![
                                "Welcome back. I have been waiting for you with eager anticipation to hear about the secret.",
                                "Now, please tell me what you have found out."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines(args![
                            "- You told him what you have seen and heard from the Odin Shirine, -",
                            "- such as Giantes, Ymir's Heart and Rekenber. -"
                        ])?;
                        ctx.next()?;
                        ctx.call(Function::Cutin, vec![Val::from("heri3.bmp"), Val::from(2)])?;
                        ctx.lines_as(
                            "Herico",
                            args![
                                "Rekenber...So, they have expanded",
                                "their power even to this small village, Hugel."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["Do you know about them?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Herico", args!["Of course, I know them too well."])?;
                        ctx.next()?;
                        ctx.lines_as("Herico", args!["............"])?;
                        ctx.next()?;
                        ctx.call(Function::Cutin, vec![Val::from("heri1.bmp"), Val::from(2)])?;
                        ctx.lines_as(
                            "Herico",
                            args![
                                "Thank you so much for your effort.",
                                "I am afraid that your dangerous journey has not been ended."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Herico",
                            args!["Well, this is time for me to tell you my story.", "I hope that you will enjoy it."],
                        )?;
                        ctx.var("hg_tre").set(Val::from(44))?;
                        ctx.close_window()?;
                        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Herico",
                            args![
                                "Please find out the secret about the airship and the island.",
                                "Then I will tell you my stories.",
                                "I hope that you will do your best to find out the secret."
                            ],
                        )?;
                        ctx.close_window()?;
                        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                        return Err(Stop::End);
                    }
                } else {
                    if ctx.var("hg_tre").get()? == 44 {
                        ctx.lines_as("Herico", args!["Hmm...I don't know where to start."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Herico",
                            args![
                                "As you know, we have bionic machines called",
                                "^FF0000Guardians^000000.",
                                "We are using Guardians to protect important facilities",
                                "or to avoid meaningless murders."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Herico",
                            args![
                                "Have you ever thought about how the people in old times",
                                "would protect their important facilities without guardians?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["Perhaps, they hired many men to do that?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Herico",
                            args![
                                "Yes, you are right.",
                                "For the longest time, Schwarzwald Republic had had",
                                "^3131FFMercenary Soldiers^000000 who exchanged their lives",
                                "with the good money, and they used to take care of",
                                "all the dirty jobs in the past."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Herico",
                            args![
                                "They lost their jobs when Rekenber successfully invented and publicized guardians.",
                                "Guardians were made for humans, but sadly it was the worst thing that could happen to them."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Herico",
                            args!["As they lost their jobs to guardians, they rose in rebellion called ^FF0000Mercenary Rebellion^000000."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Herico",
                            args![
                                "This Mercenary Rebellion was greatly responded to by the public.",
                                "However, they lost people's trust by choosing ^3131FFa violent method^000000",
                                "to fight, instead of appealing to the public in a convincible way.",
                                "In other words, they lost their game by making a critical mistake."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Herico",
                            args![
                                "Ever since their rebellion was failed,",
                                "survivors of the rebellion have disappeared beyond history."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::Cutin, vec![Val::from("heri3.bmp"), Val::from(2)])?;
                        ctx.lines_as(
                            "Herico",
                            args![
                                "Not only that, they held a fight with guardians in",
                                "^FF0000the Sage Varmunt's birthplace^000000",
                                "which was our proud national treasure!",
                                "So the house was completely destroyed by the battle, and",
                                "since then, no one knows where Sage Varmunt is."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["Sage Varmunt.....!"],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::Cutin, vec![Val::from("heri1.bmp"), Val::from(2)])?;
                        ctx.lines_as("Herico", args!["Oh...I must have been really upset about it."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Herico",
                            args![
                                "To be honest with you,",
                                "Sage Varmunt is partially responsible on guardians.",
                                "You know that he was the one",
                                "who created their motive power."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Herico",
                            args![
                                "It is riduculous that the mercenary soldiers were trying to",
                                "make their revenge on Sage Varmunt",
                                "for inventing guardians, you know.",
                                "But, it is understandable because they lost their jobs to guardians."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Herico",
                            args![
                                "I heard about the Mercenary Rebellion after everything was ended,",
                                "so there was nothing that I could do about Sage Varmunt.",
                                "He disappeared, and I couldn't figure out more",
                                "than what I found from his birth place...so...",
                                "I came to Hugel and settled down here."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Herico",
                            args![
                                "I tried to find at least a little bit of his documents from his laboratory...",
                                "but everything was gone...huh?",
                                "Hold on, I just remember something very important!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Herico",
                            args![
                                "Where are all of his research documents?",
                                "No...did mercenary soldiers took them or...possibly dispose them?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Herico",
                            args!["Damn it! I am so stupid! Grrr...", "How couldn't I think about this earlier?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Herico",
                            args![
                                ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                    + Val::from(", can you see what I want to tell you now?")),
                                "I have to take a long time to remember everything",
                                "about the incident, but you must understand",
                                "that this is an extremely important matter."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Herico",
                            args![
                                "Mercenary Rebellion was a huge issue back then.",
                                "Hopefully, there will be at least one news reporter",
                                "who remembers about the rebellion."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Herico",
                            args![
                                "Please go to Lighthalzen and find a news reporter",
                                "who still remembers about the rebellion.",
                                "If you find him, please ask him if he knows whereabouts of the rebellion survivors,",
                                "so that you can find out what happened to Sage Varmunt and his research documents."
                            ],
                        )?;
                        ctx.var("hg_tre").set(Val::from(45))?;
                        ctx.close_window()?;
                        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                        return Err(Stop::End);
                    } else {
                        if ctx.var("hg_tre").get()? == 45 {
                            ctx.lines_as(
                                "Herico",
                                args![
                                    "Please go to Lighthalzen and find a news reporter",
                                    "who still remembers about the rebellion.",
                                    "If you find him, please ask him if he knows whereabouts of the rebellion survivors,",
                                    "so that you can find out what happened to Sage Varmunt and his research documents."
                                ],
                            )?;
                            ctx.close_window()?;
                            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                            return Err(Stop::End);
                        } else {
                            if (ctx.var("hg_tre").get()?.number()? > 45 && ctx.var("hg_tre").get()?.number()? < 49) {
                                ctx.lines_as("Herico", args!["How have you been doing?"])?;
                                ctx.close_window()?;
                                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                return Err(Stop::End);
                            } else {
                                if ctx.var("hg_tre").get()? == 49 {
                                    ctx.lines_as("Herico", args!["Did you find Sage Varmunt's research documents?"])?;
                                    ctx.next()?;
                                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args!["Well, about the documents..."],
                                    )?;
                                    ctx.next()?;
                                    match runtime::select_values(
                                        ctx,
                                        &[Val::from("I ate them.:Rekenber has taken them into their possession.")],
                                    )? {
                                        1 => {
                                            ctx.mes("I ate them.")?;
                                            ctx.next()?;
                                            ctx.lines_as("Herico", args!["Hahaha! That's funny!"])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                args!["No, it is not funny."],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as("Herico", args!["................."])?;
                                            ctx.next()?;
                                            ctx.lines_as("Herico", args!["Then...SPIT THEM OUT!"])?;
                                            ctx.call(Function::PercentHeal, vec![Val::from(-5), Val::from(0)])?;
                                            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_HIT4")?])?;
                                            ctx.next()?;
                                            ctx.lines_as("Herico", args!["SPIT THEM OUT!"])?;
                                            ctx.call(Function::PercentHeal, vec![Val::from(-5), Val::from(0)])?;
                                            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_HIT4")?])?;
                                            ctx.next()?;
                                            ctx.lines_as("Herico", args!["I SAID, SPIT THEM OUUUUUT!"])?;
                                            ctx.call(Function::PercentHeal, vec![Val::from(-5), Val::from(0)])?;
                                            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_HIT4")?])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                args!["Err...I was just kidding..."],
                                            )?;
                                            ctx.next()?;
                                            ctx.call(Function::Cutin, vec![Val::from("heri2.bmp"), Val::from(2)])?;
                                            ctx.lines_as(
                                                "Herico",
                                                args![
                                                    "Haha, the first part was funny,",
                                                    "but the second part was kind of scary, you know?"
                                                ],
                                            )?;
                                            ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
                                            ctx.call(
                                                Function::Emotion,
                                                vec![
                                                    ctx.constant("ET_CRY")?,
                                                    Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as("Herico", args!["So, tell me, where are all of his research documents?"])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                args!["Rekenber has taken them into their possession."],
                                            )?;
                                            ctx.next()?;
                                        }
                                        2 => {
                                            ctx.mes("Rekenber has taken them into their possession.")?;
                                            ctx.next()?;
                                        }
                                        _ => {}
                                    }
                                    ctx.call(Function::Cutin, vec![Val::from("heri3.bmp"), Val::from(2)])?;
                                    ctx.lines_as("Herico", args!["..........................."])?;
                                    ctx.next()?;
                                    ctx.call(Function::Cutin, vec![Val::from("heri1.bmp"), Val::from(2)])?;
                                    ctx.lines_as("Herico", args!["It was expected."])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args!["What do you have in mind?"],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Herico",
                                        args![
                                            "First, I think that I should inform you about my true identity.",
                                            "I may now be an old man whose days are numbered,",
                                            "but I used to work at ^3131FFthe Ymir's Heart imitation research department",
                                            " in Regenschirm laboratory^000000 under Sage Varmunt's supervision."
                                        ],
                                    )?;
                                    ctx.call(
                                        Function::Emotion,
                                        vec![
                                            ctx.constant("ET_HUK")?,
                                            Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.call(Function::Cutin, vec![Val::from("heri2.bmp"), Val::from(2)])?;
                                    ctx.lines_as("Herico", args!["You look very surprised."])?;
                                    ctx.next()?;
                                    ctx.call(Function::Cutin, vec![Val::from("heri1.bmp"), Val::from(2)])?;
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args!["Does it mean that you are the one", "who created the Ymir's Heart imitation?"],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Herico",
                                        args!["Well, I was a mere assistant of Sage Varmunt,", "so I can't say that I created it."],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Herico",
                                        args![
                                            "Rekenber has been secretly gathering ^FF0000Ymir's Heart Pieces^000000.",
                                            "They are very obsessed with Ymir's Heart Pieces,",
                                            "and I guess that it is because of the enormous power that the heart pieces possess within.."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Herico",
                                        args![
                                            "However, the power of the original heart pieces",
                                            "are too strong for humans to control.",
                                            "Thus, it is not fiseable to manipulate the power",
                                            "as energy sources.",
                                            "That is why ^3131FFthe imitations of Ymir's Heart",
                                            "Pieces^000000 have been created."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Herico",
                                        args![
                                            "We, researchers including Sage Varmunt, endeavoured",
                                            "our effort in studying and analyzing the revolutionary energy source",
                                            "for a long time.",
                                            "And our effort has gained a fruitful result; the Imitations of Ymir's Heart Piece."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Herico",
                                        args![
                                            "With the invention, we could create guardians.",
                                            "We were so happy for the fact that we",
                                            "finally made our dream come true."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args![
                                            "But, Sage Varmunt was against the idea of using guardians",
                                            "for militant purposes, and I heard that he also participated",
                                            "in the Mercenary Rebellion."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.mes("- You told him what you knew about the Mercenary Rebellion. -")?;
                                    ctx.next()?;
                                    ctx.lines_as("Herico", args!["Hmm...now I see."])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Herico",
                                        args![
                                            "After we succeeded in creating the imitations of Ymir's Heart Piece,",
                                            "Sage Varmunt looked pretty depressed for some reason.",
                                            "After all, he left the laboratory and said that ^3131FFby him",
                                            "staying with Regenschirm laboratory",
                                            "will make everyone miserable^000000.",
                                            "I still don't figure out why he said that."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Herico",
                                        args![
                                            "He was a really great scholar.",
                                            "He always tried to use his knowledge in science for humankind."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Herico",
                                        args![
                                            "As expected, after Sage Varmunt left Regenschirm,",
                                            "almost every research process started stagnating.",
                                            "Even many projects were canceled or postponed."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.call(Function::Cutin, vec![Val::from("heri3.bmp"), Val::from(2)])?;
                                    ctx.lines_as(
                                        "Herico",
                                        args![
                                            "The upper management were threatened by the phenomenon,",
                                            "so they yelled at people to ^FF0000continue",
                                            "with their research by all means^000000.",
                                            "They even allowed people to experiment",
                                            "on anything that is alive."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.call(Function::Cutin, vec![Val::from("heri1.bmp"), Val::from(2)])?;
                                    ctx.lines_as(
                                        "Herico",
                                        args!["......................", "That gave me a reason to leave Regenschirm."],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Herico",
                                        args![
                                            "Rekenber will do anything to achieve their goals.",
                                            "They have become too powerful to just be considered a big corporation."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args![
                                            "That was why they founded the \"Bio Lab\"",
                                            "in order to continue with their research without Sage Varmunt."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.call(Function::Cutin, vec![Val::from("heri3.bmp"), Val::from(2)])?;
                                    ctx.call(Function::Emotion, vec![ctx.constant("ET_ANGER")?])?;
                                    ctx.lines_as(
                                        "Herico",
                                        args![
                                            "What? \"Bio Lab\"? I didn't image that they could",
                                            "even think of that! I can't express enough of my resentment toward them!"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Herico",
                                        args![
                                            "Science is to benefit humankinds,",
                                            "not to harm them. I think that something must be done.",
                                            "Can you bring me a glass of water?"
                                        ],
                                    )?;
                                    ctx.var("hg_tre").set(Val::from(50))?;
                                    ctx.close_window()?;
                                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                    return Err(Stop::End);
                                } else {
                                    if ctx.var("hg_tre").get()? == 50 {
                                        ctx.lines_as("Herico", args!["Can you guess what I am thinking right now?"])?;
                                        ctx.next()?;
                                        ctx.lines_as("Herico", args!["I am thinking about ^3131FFdestroying Regenschirm^000000."])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Herico",
                                            args![
                                                "Rekenber has become as powerful as this Schwarzwald Republic.",
                                                "I don't think that I could destroy the entire corporation.",
                                                "But even if I could, it could also lead to the fall of my country,",
                                                "because that's who big Rekenber means to this country."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Herico",
                                            args![
                                                "So, instead, I am thinking to destroy \"Regenschirm\".",
                                                "Because the laboratory may not be as big as the corporation,",
                                                "but at the same time, it has a great meaning to them.",
                                                "I want to stop their inhumane and dangerous practice in research."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Herico",
                                            args![
                                                "...*Sigh*...Unfortunately, I do not possess any power to",
                                                "proceed with my plan. Thus, I need your help, desperately."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Herico",
                                            args!["Let me hear your opinion.", "Will you join me in destroying Regenschirm?"],
                                        )?;
                                        ctx.next()?;
                                        match runtime::select_values(ctx, &[Val::from("Sorry, I won't:Yes, I will.")])? {
                                            1 => {
                                                ctx.lines_as(
                                                    "Herico",
                                                    args![
                                                        "I respect your decision since this is not something",
                                                        "that is okay for you to make a quick judgement.",
                                                        "I appreciate everything that you have done for me so far.",
                                                        "And if you change your mind, feel free to come back any time."
                                                    ],
                                                )?;
                                                ctx.close_window()?;
                                                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                                return Err(Stop::End);
                                            }
                                            2 => {
                                                ctx.call(Function::Cutin, vec![Val::from("heri2.bmp"), Val::from(2)])?;
                                                ctx.lines_as(
                                                    "Herico",
                                                    args![
                                                        "I knew that you are too righteous to",
                                                        "neglect lies and injustice.",
                                                        "I have seen that through your eyes.",
                                                        "I don't know how much we can achieve with this mission,",
                                                        "but it is very meaningful for us to at least try."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as("Herico", args!["Let's discuss further what we should do."])?;
                                                ctx.var("hg_tre").set(Val::from(51))?;
                                                ctx.close_window()?;
                                                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                                return Err(Stop::End);
                                            }
                                            _ => {}
                                        }
                                    } else {
                                        if ctx.var("hg_tre").get()? == 51 {
                                            ctx.lines_as(
                                                "Herico",
                                                args![
                                                    "We must find out how we can stop them",
                                                    "from doing more evil things, at least for a while.",
                                                    "Regenschirm is a historic laboratory, and thus",
                                                    "an enormous amount of accumulated research data",
                                                    "has been saved within the laboratory.",
                                                    "Without the research data,",
                                                    "all the researching activity within Regenschirm",
                                                    "will not be progressed."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Herico",
                                                args![
                                                    "Regenschirm is located at the underground level",
                                                    "of Rekenber's Lighthalzen headquarters.",
                                                    "It is where their filthy ambitions are reflected on.",
                                                    "Please sneak into the laboratory",
                                                    "and bring ^FF0000their research data^000000 to me."
                                                ],
                                            )?;
                                            ctx.var("hg_tre").set(Val::from(52))?;
                                            ctx.close_window()?;
                                            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                            return Err(Stop::End);
                                        } else {
                                            if ctx.var("hg_tre").get()? == 52 {
                                                ctx.lines_as(
                                                    "Herico",
                                                    args![
                                                        "Regenschirm is located at the underground level",
                                                        "of Rekenber's Laighthalzen headquarters.",
                                                        "Can you sneak into the place and bring me their research data?"
                                                    ],
                                                )?;
                                                ctx.close_window()?;
                                                ctx.call(Function::Cutin, vec![Val::from("heri1.bmp"), Val::from(255)])?;
                                                return Err(Stop::End);
                                            } else if ctx.var("hg_tre").get()? == 53 {
                                                ctx.lines_as("Herico", args!["Ah, you are back."])?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                    args![
                                                        "I had a little bit of trouble because I was caught by a guard.",
                                                        "But I took care of it."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Herico",
                                                    args![
                                                        "Thank you so much for your trouble.",
                                                        "Haha, since they lost their research data,",
                                                        "they wouldn't be able to continue with their research",
                                                        "at least for a while, I hope."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Herico",
                                                    args![
                                                        "Now, let's get into the next step, shall we?",
                                                        "I know that you have gone through many difficulties so far,",
                                                        "but this might be the most difficult thing that you've ever done."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as("Herico", args!["In my opinion, the bio lab is the most important facility in Regenschirm.", "We must not let that kind of inhumane place exist in this world any longer.", "I am so horrified by the thought that they have been doing all sorts of evil things", "under an excuse of scientific research."])?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Herico",
                                                    args![
                                                        "As I analyzed the research data which you have brought to me,",
                                                        "the bio lab is consisted of 3 levels,",
                                                        "and I suspect that the 2nd level is used to",
                                                        "perform experimental research on living bodies."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Herico",
                                                    args![
                                                        "I am pretty sure that we can find",
                                                        "a miner who knows how to make",
                                                        "Marine Sphere Bottles in ^3131FFEinbech^000000,",
                                                        "since the Marine Sphere Bottles are needed to",
                                                        "blast tunnels inside mines."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Herico",
                                                    args!["Haha...you know what I am going to ask you now, don't you?"],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as("Herico", args!["Please go meet with ^3131FFthe Marine Sphere Bottle manufacturer^000000,", "and purchase some Marine Sphere Bottles.", "And explode ^3131FFthe bio experiment equipment^000000 in the 2nd level of bio lab!"])?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Herico",
                                                    args![
                                                        "I know that this is a difficult task. But, please understand",
                                                        "that you are the only hope that I have.",
                                                        "Please stop them from performing experimental research on living bodies!"
                                                    ],
                                                )?;
                                                ctx.call(Function::DelItem, vec![Val::from(7342), Val::from(1)])?;
                                                ctx.var("hg_tre").set(Val::from(54))?;
                                                ctx.close_window()?;
                                                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                                return Err(Stop::End);
                                            } else if ctx.var("hg_tre").get()? == 54 {
                                                ctx.lines_as("Herico", args!["Please go meet with ^3131FFthe Marine Sphere Bottle manufacturer^000000,", "and purchase some Marine Sphere Bottles.", "And explode ^3131FFthe bio experiment equipment^000000 in the 2nd level of bio lab.", "You are the only hope that I have.", "Let's hope that it will work out well."])?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Herico",
                                                    args![
                                                        "If you know anyone",
                                                        "who can create Marine Sphere Bottles for you,",
                                                        "you may use his instead."
                                                    ],
                                                )?;
                                                ctx.close_window()?;
                                                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                                return Err(Stop::End);
                                            } else if ctx.var("hg_tre").get()? == 55 {
                                                ctx.lines_as(
                                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                    args!["The machine was blown up."],
                                                )?;
                                                ctx.next()?;
                                                ctx.call(Function::Cutin, vec![Val::from("heri2.bmp"), Val::from(2)])?;
                                                ctx.lines_as("Herico", args!["Excellent! Excellent!"])?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Herico",
                                                    args![
                                                        "Now, I am relieved that they won't be able to",
                                                        "proceed with their bio research at least for a while.",
                                                        "You did a great job, and I am so proud of you."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.call(Function::Cutin, vec![Val::from("heri1.bmp"), Val::from(2)])?;
                                                ctx.lines_as(
                                                    "Herico",
                                                    args![
                                                        "I assume that there are",
                                                        "some failed experimental objects remaining within the laboratory.",
                                                        "According to the research document,",
                                                        "those experimental objects are identified with ^3131FFHandcuffs^000000."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Herico",
                                                    args![
                                                        "From now on, I will reward you every time when you bring me",
                                                        "a certain amount of the Handcuffs."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Herico",
                                                    args![
                                                        "And...even though we succeeded in stopping them for now,",
                                                        "it will not last that long.",
                                                        "We need someone who has a power to stop them permanently.",
                                                        "Sadly, I am not the one."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Herico",
                                                    args!["By any chance, do you know a man who can aid his power in stopping them?"],
                                                )?;
                                                ctx.next()?;
                                                let (input, status) = runtime::input_text(ctx, None, None)?;
                                                l_inputstr_s = input;
                                                if (((l_inputstr_s.clone() == "President Karl"
                                                    || l_inputstr_s.clone() == "President Weierstrass")
                                                    || l_inputstr_s.clone() == "President")
                                                    || l_inputstr_s.clone() == "Karl Weierstrass")
                                                {
                                                    ctx.lines_as(
                                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                        args!["I think that I know one..."],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Herico", args!["Okay, tell me who he is."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                        args!["Mr. President.....!", "Mr. President must be the one who can help us."],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Herico",
                                                        args![
                                                            "You mean Karl...something something?",
                                                            "I may well say that Rekenber owns this country.",
                                                            "I don't think that he has not been influenced by them."
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                        args![
                                                            "No, Mr. President, who I know,",
                                                            "has been trying to free this country from Rekenber.",
                                                            "Even if he was frustrated by a ridiculous reason at the last time,",
                                                            "I believe that he has not given up in his hope."
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Herico",
                                                        args![
                                                            "Hmm...I see. Then I must count on your word.",
                                                            "I am giving the research data back to you,",
                                                            "so please bring it to Mr. President.",
                                                            "Let's see how well he can handle it.",
                                                            "I appreciate you for doing everything for me so far,",
                                                            "I really appreciate it."
                                                        ],
                                                    )?;
                                                    ctx.var("hg_tre").set(Val::from(56))?;
                                                    ctx.call(Function::GetItem, vec![Val::from(7342), Val::from(1)])?;
                                                    ctx.close_window()?;
                                                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                                    return Err(Stop::End);
                                                }
                                                ctx.lines_as(
                                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                    args!["Umm...no, I don't think that I know anyone."],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Herico",
                                                    args![
                                                        "Oh...okay. Hmm...we certainly need someone",
                                                        "with a power who can stop them permanently.",
                                                        "Why don't you go to different cities and find the one",
                                                        "who can help us? I am pretty sure that",
                                                        "you will find one."
                                                    ],
                                                )?;
                                                ctx.close_window()?;
                                                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                                return Err(Stop::End);
                                            } else if ctx.call(Function::CountItem, vec![Val::from(7342)])?.is_true() {
                                                ctx.lines_as(
                                                    "Herico",
                                                    args![
                                                        "Why are you still here?",
                                                        "That research data will aid a great help to Mr. President.",
                                                        "Please deliver it to him as soon as you can."
                                                    ],
                                                )?;
                                                ctx.close_window()?;
                                                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                                return Err(Stop::End);
                                            } else {
                                                ctx.lines_as("Herico", args!["Hey, welcome back."])?;
                                                ctx.next()?;
                                                match runtime::select_values(
                                                    ctx,
                                                    &[Val::from("I brought Handcuffs:Regarding Sage Varmunt's documents")],
                                                )? {
                                                    1 => {
                                                        ctx.lines_as(
                                                            "Herico",
                                                            args!["I am willing to exchange 1 of level 4 food with 100 Handcuffs."],
                                                        )?;
                                                        ctx.next()?;
                                                        if Val::from(runtime::select_values(ctx, &[Val::from("Exchange:Cancel.")])?) == 1 {
                                                            if ctx.call(Function::CountItem, vec![Val::from(7345)])?.number()? > 99 {
                                                                ctx.lines_as("Herico", args!["There you go, thank you for your trouble."])?;
                                                                ctx.call(Function::DelItem, vec![Val::from(7345), Val::from(100)])?;
                                                                let subject5 =
                                                                    ctx.call(Function::Rand, vec![Val::from(1), Val::from(6)])?;
                                                                if subject5 == 1 {
                                                                    ctx.call(Function::GetItem, vec![Val::from(12044), Val::from(1)])?;
                                                                } else if subject5 == 2 {
                                                                    ctx.call(Function::GetItem, vec![Val::from(12049), Val::from(1)])?;
                                                                } else if subject5 == 3 {
                                                                    ctx.call(Function::GetItem, vec![Val::from(12059), Val::from(1)])?;
                                                                } else if subject5 == 4 {
                                                                    ctx.call(Function::GetItem, vec![Val::from(12064), Val::from(1)])?;
                                                                } else if subject5 == 5 {
                                                                    ctx.call(Function::GetItem, vec![Val::from(12069), Val::from(1)])?;
                                                                } else if subject5 == 6 {
                                                                    ctx.call(Function::GetItem, vec![Val::from(12054), Val::from(1)])?;
                                                                }
                                                            } else {
                                                                ctx.lines_as(
                                                                    "Herico",
                                                                    args!["I don't think that you have brought me 100 Handcuffs."],
                                                                )?;
                                                            }
                                                        }
                                                        ctx.close_window()?;
                                                        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                                        return Err(Stop::End);
                                                    }
                                                    2 => {
                                                        ctx.lines_as(
                                                            "Herico",
                                                            args![
                                                                "When I checked the research data that you have brought to me,",
                                                                "it seemed that Sage Varmunt wrote his documents in cipher,",
                                                                "so they could not decode them.",
                                                                "That explains why they have been obsessed",
                                                                "with bionic researches to figure out on their own."
                                                            ],
                                                        )?;
                                                        ctx.close_window()?;
                                                        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                                        return Err(Stop::End);
                                                    }
                                                    _ => {}
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
    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
    return Err(Stop::End);
}

pub fn herico(ctx: &Ctx) -> Script {
    herico_body(ctx, Vec::new()).map(|_| ())
}

fn enquro_carson_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("hg_tre").get()?.number()? < 45 {
        ctx.lines_as(
            "Enquro Carson",
            args!["Hello, everyone. This is Enquro Carson", "from Light News."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Enquro Carson",
            args![
                "People want to hear prompt and accurate news.",
                "I feel proud of myself as a news reporter",
                "especially when people are astonished by the news",
                "which I am delivering to them."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("hg_tre").get()? == 45 {
        ctx.lines_as(
            "Enquro Carson",
            args!["Hello, everyone. This is Enquro Carson", "from Light News."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Enquro Carson",
            args![
                "People want to hear prompt and accurate news.",
                "I feel proud of myself as a news reporter",
                "especially when people are astonished by the news",
                "which I am delivering to them."
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Mercenary Rebellion:News reporter job")])? {
            1 => {
                ctx.lines_as(
                    "Enquro Carson",
                    args![
                        "Hmm...Mercenary Rebellion...?",
                        "It happened pretty long time ago.",
                        "Mercenary soldiers lost their jobs",
                        "after guardians were invented.",
                        "And they insisted their social security",
                        "through a violent method, so the government had to stop them with force."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["Do you know any survivor of Mercenary Rebellion?"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Enquro Carson",
                    args![
                        "Welll...I was not assigned to the case,",
                        "and the news reporter who kept the coverage of the case",
                        "retired long time ago...",
                        "Oh, I think that I can check",
                        "if he has left any documents about the rebellion.",
                        "Give me a moment."
                    ],
                )?;
                ctx.next()?;
                ctx.lines(args!["- *Rummage Rummage* -", "- *Rummage Rummage* -", "- *Rummage Rummage* -"])?;
                ctx.next()?;
                ctx.lines_as("Enquro Carson", args!["Oh! I found it."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Enquro Carson",
                    args![
                        "Mr. Balparan, yes, he is the news reporter,",
                        "must be greatly intrigued by the rebellion.",
                        "According to this documents, he tried to",
                        "contact with the rebellion survivors",
                        "after the rebellion ended in failure."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["Can you tell me the survivors' locations?"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Enquro Carson",
                    args![
                        "Well, I am not supposed to release",
                        "any kind of personal information to outside.",
                        "Hmm...but it happened a long time ago, and",
                        "I don't know if they are still alive or not..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["Thank you so much!"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Enquro Carson",
                    args![
                        "Wait, wait, I didn't say that I would release their information yet.",
                        "Actually my motto is that",
                        "\"^3131FFEvery bit of information must be shared^000000\"."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Enquro Carson",
                    args![
                        "I hope that you are willing to share any information",
                        "that you will find from the survivors with me.",
                        "Oh yes, do you know what? \"Commutative contract\"."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Enquro Carson",
                    args![
                        "I heard of a private pub in Morocc,",
                        "a city of Rune-Midgarts Kingdom",
                        "that is not open to the public.",
                        "People keep talking about the pub's best drink,",
                        "and I wonder how delicious the drink will be.",
                        "Can you please get me the drink? Hahaha, thanks!"
                    ],
                )?;
                ctx.var("hg_tre").set(Val::from(46))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as("Enquro Carson", args![".......Hmmm...Hmmm...Hmmm."])?;
                ctx.next()?;
                ctx.mes("- Enquro Carson looks you up and down with squinting his eyes. -")?;
                ctx.next()?;
                ctx.lines_as(
                    "Enquro Carson",
                    args!["You are not saying that you want to be a news reporter, are you?"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if ctx.var("hg_tre").get()? == 46 {
        if (!(ctx.call(Function::CountItem, vec![Val::from(12112)])?.is_true())
            || !(ctx.call(Function::CountItem, vec![Val::from(12113)])?.is_true()))
        {
            ctx.lines_as(
                "Enquro Carson",
                args![
                    "I heard of a private pub in Morocc,",
                    "a city of Rune-Midgarts Kingdom",
                    "that is not open to the public.",
                    "People keep talking about the pub's best drink,",
                    "and I wonder how delicious the drink will be.",
                    "Can you please get me the drink? Hahaha, thanks!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as("Enquro Carson", args!["Wow, this is that famous drink!", "Let's see...*Gulp*"])?;
            ctx.next()?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_BEST")?])?;
            ctx.lines_as(
                "Enquro Carson",
                args![
                    "Wow! This is awsome!",
                    "It is as delicious as I heard!",
                    "Its taste is still lingering on my tongue,",
                    "and it is like Wow!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Enquro Carson",
                args![
                    "Anyways, according to Mr. Balparan's documents,",
                    "errr....yes, this must be the name of the last mercenary soldier.",
                    "Mr. Balparan contacted with",
                    "^3131FFWintzil Trony^000000,",
                    "the last mercenary soldier for the last time in...",
                    "^FF0000Hugel^000000. Wait, Hugel?",
                    "Isn't that the small country at the corner of Schwarzwald Republic?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["What? Hugel?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Enquro Carson",
                args![
                    "Previously, it was very difficult to get to Hugel",
                    "because there was no safe route.",
                    "Basically, the village was like an island within the continent,",
                    "but ever since an airport was built there,",
                    "the village is now having its period of prosperity."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Enquro Carson",
                args![
                    "You are lucky, you know that?",
                    "Mr. Balparan gave up on meeting with the mercenary soldier",
                    "in Hugel because it was too tough to just get there.",
                    "But you, all you need to do is just getting on to the airship."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Enquro Carson",
                args![
                    "Oh, by the way, I hope that you will keep",
                    "this secret between you and me.",
                    "I released the personal information at my discretion,",
                    "and I know that I am not allowed to do that. Hehe."
                ],
            )?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_KIK")?])?;
            ctx.var("hg_tre").set(Val::from(47))?;
            ctx.call(Function::DelItem, vec![Val::from(12112), Val::from(1)])?;
            ctx.call(Function::DelItem, vec![Val::from(12113), Val::from(1)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        if ctx.var("hg_tre").get()? == 47 {
            ctx.lines_as(
                "Enquro Carson",
                args![
                    "Just get on to the airship and go to Hugel.",
                    "Hopefully you will meet with ^3131FFWintzil Trony^000000",
                    "or his son...or something.",
                    "Oh, be careful not to provoke him though.",
                    "He used to be a mercenary soldier, and they were famed for their violence."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Enquro Carson",
                args![
                    "This is very shameful for me to confess,",
                    "but I tend to make lots and lots of spelling mistakes.",
                    "Yeah, I know, it is a shame."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    Ok(Val::from(0))
}

pub fn enquro_carson(ctx: &Ctx) -> Script {
    enquro_carson_body(ctx, Vec::new()).map(|_| ())
}

fn girl_hugel_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("hg_tre").get()?.number()? < 47 {
        ctx.lines_as("Girl", args!["Bah, I am bored."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("hg_tre").get()? == 47 {
        ctx.lines_as("Girl", args!["Hehe...are you from another city?", "Heheh."])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Hey, little lady?", "By any chance, do you know a man named \"Wintzil Trony\"?"],
        )?;
        ctx.next()?;
        ctx.lines_as("Girl", args!["There is no man named Wintzil Trony."])?;
        ctx.next()?;
        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Huh?"])?;
        ctx.next()?;
        ctx.lines_as("Girl", args!["I am the Wintzil Trony.", "I am a girl, not a man."])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["..............................................................................................................."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "(^3131FFOh dear Odin, am I on a test?",
                "Or are you trying to make sure that I still have a good memory?",
                "How could...how could...how could this little child",
                "is the last mercenary soldier?! This is absurd!)"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Well...lady,"])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Are you sure that you are the last mer....no, no.",
                "Then do you have any family member who shares the same name as yours?",
                "Like your father, for instance."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Wintzil Trony",
            args![
                "I said, I am the only Wintzil Trony in my family.",
                "And I don't have a father.",
                "My mom is out of Hugel to sell Hugel's principal products."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Did...did your father pass away?"],
        )?;
        ctx.next()?;
        ctx.lines_as("Wintzil Trony", args!["I can't help it. He was too old."])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Well...do you remember what your father", "used to do when he was alive?"],
        )?;
        ctx.next()?;
        ctx.lines_as("Wintzil Trony", args!["My father sowed seeds."])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["(^3131FFGosh, she is not helping! This is not good!^000000)"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Well, lady, I think that people usually",
                "say \"cultivate a field\" instead of saying \"sowing seeds\".",
                "Did you know that?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Wintzil Trony",
            args![
                "^3131FFBut my father had only one arm,",
                "so he couldn't do such a hard work like cultivating a field.^000000",
                "But, it was alright. I could help him because I am strong enough.",
                "I cultivated fields with my mom and my father.",
                "Do you want to know how strong I am? Look at this! Yap!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![".................!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Umm...lady, can you show me your father's picture",
                "or something that used to belong to him?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Wintzil Trony",
            args![
                "Beh~ You are just like the guy who came by a long time ago.",
                "He kept asking me things over and over again,",
                "then he became upset and left."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["(He might be the news reporter from the Light News.)"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Wintzil Trony",
            args!["My father was a farmer. That's all I can say.", "Beeeeh~"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Wintzil Trony",
            args![
                "I am living in a house that is pretty empty.",
                "All my father left to us is the shelf at the upstairs."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Do you mind if I take a look at your father's shelf?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Wintzil Trony",
            args![
                "Sure!",
                "It is filled with books, and they don't look that interesting.",
                "But, can you be careful to treat the shelf? It is a keepsake from my father."
            ],
        )?;
        ctx.var("hg_tre").set(Val::from(48))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("hg_tre").get()?.number()? > 48 {
        ctx.lines_as("Wintzil Trony", args!["I am bored and bored! When is my mom coming back?"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Wintzil Trony",
            args!["If you promise me that you will not break it,", "you can take a look at the shelf."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn girl_hugel(ctx: &Ctx) -> Script {
    girl_hugel_body(ctx, Vec::new()).map(|_| ())
}

fn book_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::Cutin, vec![Val::from("hg_book.bmp"), Val::from(2)])?;
    if ctx.var("hg_tre").get()?.number()? > 47 {
        ctx.lines(args![
            "- You picked up a very thick book. -",
            "- You opened the book and found out -",
            "- that there were a small book hidden behind.-"
        ])?;
        ctx.next()?;
        'b1: {
            let subject1 = Val::from(runtime::select_values(
                ctx,
                &[Val::from("Read the small book.:Leave it alone.")],
            )?);
            let mut matched1 = false;
            let no_case1 = !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2));
            if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines(args![
                    "- You picked up the small book. -",
                    "- The book owner must want to -",
                    "- keep it secret. -"
                ])?;
                ctx.next()?;
                'l2: loop {
                    if !(true) {
                        break 'l2;
                    }
                    'b2: {
                        match runtime::select_values(
                            ctx,
                            &[Val::from(
                                "Page 1:Page 2:Page 3:Page 4:Page 5:Page 6:Page 7:Page 8:Page 9:Stop Reading.",
                            )],
                        )? {
                            1 => {
                                ctx.mes("- You started reading the 1st page of the book. -")?;
                                ctx.next()?;
                                ctx.lines(args![
                                    "Date: **month **day",
                                    "I was told that Rekenber succeeded",
                                    "in creating a type of machine called ^FF0000Guardian^000000.",
                                    "I have a bad feeling about that, so I decided to write a journal",
                                    "starting from today and keep everything in record.",
                                    " ",
                                    "\"^FF0000Guardian^000000\" is an android",
                                    "that was created based on the idea of \"^3131FFGiantes^000000\",",
                                    "the giant tribe that was said to live in an ancient age.",
                                    "It is memorized with various commands,",
                                    "so it is said to handle difficult things for humans as protecting",
                                    "important facilities and such.",
                                    " ",
                                    "However, those things are our job, the mercenary soldiers' job.",
                                    "Mercenary soldier groups in many areas",
                                    "are already agitated by the news of the Guardian creation.",
                                    "I am worried what is going to happen to us."
                                ])?;
                                ctx.next()?;
                                ctx.lines(args![
                                    "...............................",
                                    "...............................",
                                    "..............................."
                                ])?;
                                ctx.next()?;
                            }
                            2 => {
                                ctx.mes("- You started reading the 2nd page of the book. -")?;
                                ctx.next()?;
                                ctx.lines(args![
                                    "Date: **month **day",
                                    "Everybody in this place seems to be very confused and upset as well as me.",
                                    "I made a stupid mistake to slip on one of the stairs",
                                    "while absent-mindedly walking down. I really need to straighten myself out."
                                ])?;
                                ctx.next()?;
                                ctx.lines(args![
                                    "...............................",
                                    "...............................",
                                    "..............................."
                                ])?;
                                ctx.next()?;
                            }
                            3 => {
                                ctx.mes("- You started reading the 3rd page of the book. -")?;
                                ctx.next()?;
                                ctx.lines(args![
                                    "Date: **month **day",
                                    "Today, we, \"Republic Garrison\",",
                                    "were introduced to ^3131FF2 Guardians.",
                                    "Those machines joined us for their test operation^000000.",
                                    "Everyone seemed to be intimidated by their big and heavy frames.",
                                    "I thought that these machines were supposed to aid humans in doing",
                                    "hard works. But it doesn't seem to be true.",
                                    "To me, they are nothing but \"^FF0000weapons of mass destruction^000000\".",
                                    "Perhaps, they might be intended to be that way from the beginning.",
                                    " ",
                                    "What is going to be next after we finish testing the machines?",
                                    "No matter how bad other people criticize us for being greedy toward money,",
                                    "we always take pride in what we are doing, and we always do our best.",
                                    "We must try our best to do our job, otherwise we will be dead right away.",
                                    "But, now, we are losing our place to the lifeless machines.",
                                    "Many people have quit this job after the machines were introduced.",
                                    "I must admit that I am also being shaken by the thought.",
                                    " ",
                                    "Bah, god damn you, Rekenber! You just have created the worst invention in history!"
                                ])?;
                                ctx.next()?;
                                ctx.lines(args![
                                    "...............................",
                                    "...............................",
                                    "..............................."
                                ])?;
                                ctx.next()?;
                            }
                            4 => {
                                ctx.mes("- You started reading the 4th page of the book. -")?;
                                ctx.next()?;
                                ctx.lines(args![
                                    "Date: **month **day",
                                    "No one wouldn't know how boring it is to mount guard with a machine."
                                ])?;
                                ctx.next()?;
                                ctx.lines(args![
                                    "...............................",
                                    "...............................",
                                    "..............................."
                                ])?;
                                ctx.next()?;
                            }
                            5 => {
                                ctx.mes("- You started reading the 5th page of the book. -")?;
                                ctx.next()?;
                                ctx.lines(args!["Date: **month **day", "Damn it! Finally it happened today!", "Today, our troops was in total disorder.", "One of the guardians caused an error and brought havoc.", "While we were terrified and didn't know what to do, that broken guardian", "crushed the commander of 'Blade Canine Mercenary' to death.", " ", "I can't remember how we could regain control over the broken guardian.", "My mind is still baffled for the fact that that machine actually killed an innocent guy as I was kind of worried about.", "Many people were dispatched from Rekenber to investigate this matter, so hopefully they will figure out", "what the hell was wrong with that machine.", "But, it is not going to change the fact that one of our comrade was taken his life by that merciless machine,", "and no once can replace his place for his family."])?;
                                ctx.next()?;
                                ctx.lines(args![
                                    "...............................",
                                    "...............................",
                                    "..............................."
                                ])?;
                                ctx.next()?;
                            }
                            6 => {
                                ctx.mes("- You started reading the 6th page of the book. -")?;
                                ctx.next()?;
                                ctx.lines(args![
                                    "Date: **month **day",
                                    "We received a message from upper management.",
                                    "\"^3131FFKeep our mouth from spreading the story to outside.^000000\", that's the content of message.",
                                    "Rekenber must have bribed the upper management of our troops with money.",
                                    "Otherwise, why would they want us to keep our mouth sealed about a such shocking incident?",
                                    "We, angry mercenary soldiers burst into rage right after the morning session was over.",
                                    " ",
                                    "We went ahead and spread the story about the shocking incident to every mercenary troops.",
                                    "And then we destroyed every guardian which we found in hangars.",
                                    "The sounds of hammering metal and sirens were echoing everywhere, everywhere you went.",
                                    " ",
                                    "...We took refuge in a safe place out of our unit because of what happened.",
                                    "We are planning to assert our social security to the public and inform the public with",
                                    "the danger of the guardians and the negligence of the government and Rekenber toward the incident.",
                                    "Our leader told us that he has sent out letters to every mercenary troop",
                                    "to encourage them to participate in what we are trying to do.",
                                    " ",
                                    "I feel like...that I have just turn into a no-way-out.",
                                    "I am facing my future with a deep sense of gloom."
                                ])?;
                                ctx.next()?;
                                ctx.mes("- You found a folded note between the pages that you were reading. -")?;
                                ctx.next()?;
                                if Val::from(runtime::select_values(
                                    ctx,
                                    &[Val::from("Check the note.:Read the next page.")],
                                )?) == 1
                                {
                                    ctx.mes("^3131FF.........................^000000")?;
                                    ctx.next()?;
                                    ctx.lines(args![
                                        "^3131FF...I am so sorry for hearing such devastating news and for your loss.",
                                        "Every member of our troops are already aware of the shocking incident,",
                                        "and all of us are uncontrollably enraged.",
                                        "The government has been bribed ",
                                        "with Rekenber's filthy money and has become nothing but a puppet.",
                                        "We must take control over our life once again. Our future depends on us.",
                                        "We will join you, 'Blade Canine Mercenary' to fight for our rights.^000000"
                                    ])?;
                                    ctx.next()?;
                                    ctx.mes("^3131FF.........................^000000")?;
                                    ctx.next()?;
                                }
                            }
                            7 => {
                                ctx.mes("- You started reading the 7th page.-")?;
                                ctx.next()?;
                                ctx.lines(args!["Date: **month **day", "Our firm will was not enough.", "Our desperate effort was not enough!", "Many mercenary groups joined us to fight with the government and Rekenber.", "But we were too weak to compete with their invincible guardians. We were literally swept away.", "They were quicker than we expected in taking action, and lines and lines of guardians were coming after us.", "There was nothing that we could do. We met with a disastrous defeat in the battle with guardians.", " ", "Everybody is still in shock, but we can't give up now.", "I think that we are going to visit a house of a sage called \"Varmunt\" early in the morning.", "I don't know why, but it seems that the sage is against the use of guardians.", "People say that his mansion is rather like a fortress.", "I also don't know why someone would want to build his house", "as strong as a fortress. Regardless, I am pretty sure that his support will be a great help for us.", "It was a really tiring day today. It was also a very chaotic day.", "To be honest, I don't know what is going to happen now.", "I am scared if I would die helplessly. I am scared and depressed.", "Why must this happen to us, and what have we done to deserve such a horrible treatment?"])?;
                                ctx.next()?;
                                ctx.lines(args![
                                    "...............................",
                                    "...............................",
                                    "..............................."
                                ])?;
                                ctx.next()?;
                            }
                            8 => {
                                ctx.mes("- You started reading the 8th page. -")?;
                                ctx.next()?;
                                ctx.lines(args!["Date: **month **day", "Our leader became desperate, so he sent a request to Schwarzwald Republic for ^3131FFa cease-fire agreement^000000.", "I guess that he was frightened by guardians around the Varmunt mansion.", "Sadly, Schwarzwald Republic declined our request without hesitation.", "They stated through their reponse that ^FF0000they would not tolerate this kind of unacceptable rebellion by any means,", "and thus, they would take strong action against the rebellion this time.^000000.", "I guess that the government is determined to eliminate us, the mercenary soldiers through this event.", "People now started showing their resentment toward us, who brought disorder to their country.", "This is not right. Something has went wrong. We just wanted the government to secure our lives, that was all.", "Since we were declined for a cease-fire agreement, we are planning to hold the last fight tomorrow in this place."])?;
                                ctx.next()?;
                                ctx.lines(args![
                                    "...............................",
                                    "...............................",
                                    "..............................."
                                ])?;
                                ctx.next()?;
                            }
                            9 => {
                                ctx.mes("- You started reading the 9th page. -")?;
                                ctx.next()?;
                                ctx.lines(args!["Date: **month **day", "It has been a while since I wrote a journal last time.", "I am in Hugel, a small town at the border of Schwarzwald Republic.", "Because of the locational penalty, people of this town seldom see outsiders, and if they see one, they treate them warmheartedly.", "Although I didn't have a time to write a journal that time, the last fight in the Varmunt mansion turned out to be our miserable defeat...as I was worried.", "I was told that Sage Varmunt disappeared after the fight, and all of his documents were taken by Rekenber.", "I am afraid that they might create a more powerful version of the existing guardian by using his documents.", " ", "I lost one arm during the last fight, and I barely escaped to Hugel.", "I feel so lucky to be in Hugel, because when I arrived, I was a total mess with only one arm,", "but the people in this town did not ask me anything but welcomed me.", " ", "I have nowhere to go any longer. I am planning to live my life here as long as nothing happens soon.", " ", "My arm is throbbing with pain again. The history of mercenary soldiers ended, and I need to rest.", "I hope that this will be the last page of my journal."])?;
                                if ctx.var("hg_tre").get()? == 48 {
                                    ctx.var("hg_tre").set(Val::from(49))?;
                                }
                                ctx.next()?;
                                ctx.mes("- The journal was ended at the page. -")?;
                                ctx.next()?;
                            }
                            10 => {
                                ctx.mes("- You closed the book. -")?;
                                ctx.close_window()?;
                                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
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
                ctx.mes("- You closed the book. -")?;
                ctx.close_window()?;
                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                return Err(Stop::End);
            }
        }
    }
    Ok(Val::from(0))
}

pub fn book(ctx: &Ctx) -> Script {
    book_body(ctx, Vec::new()).map(|_| ())
}

fn bomb_maker_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("hg_tre").get()? == 54 {
        if ((ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 2000
            || ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0)
        {
            ctx.lines_as(
                "Boomer",
                args!["Hey, you look pretty heavy, huh?", "Do you want me to help you?"],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Boomer",
            args![
                "Hey, what's up?",
                "I am a professional Marine Sphere Bottle maker.",
                "You can call me Boomer."
            ],
        )?;
        ctx.next()?;
        'b1: {
            let subject1 = Val::from(runtime::select_values(
                ctx,
                &[Val::from("Buy Marine Sphere Bottle.:End conversation.")],
            )?);
            let mut matched1 = false;
            let no_case1 = !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2));
            if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as(
                    "Boomer",
                    args![
                        "Heh, you want my Marine Sphere Bottles, huh?",
                        "You know, my bomb never fail to blow things away.",
                        "Do you want to cut a tunnel?",
                        "You've got the right choice for the bomb here!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Boomer",
                    args![
                        "One Marine Sphere Bottle is 3,000 zeny.",
                        "You must understand that I am providing the bomb at such a low price",
                        "considering its amazing performance!",
                        "You will regret if you miss this chance. Hahaha!"
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Buy.:Cancel.")])? {
                    1 => {
                        if ctx.var("Zeny").get()?.number()? < 3000 {
                            ctx.lines_as(
                                "Boomer",
                                args!["Errr...You must have been so hurry to carry your wallet with you."],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as(
                            "Boomer",
                            args![
                                "Just make sure that you are not going to",
                                "blow yourself away with this bomb, hahah!"
                            ],
                        )?;
                        ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(3000))?))?;
                        ctx.call(Function::GetItem, vec![Val::from(7138), Val::from(1)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as("Boomer", args!["I believe that you will end up coming back to me. Heheh."])?;
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
                ctx.lines_as("Boomer", args!["Boom! Don't you need a bomb? Boom!"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    } else {
        ctx.lines_as(
            "Boomer",
            args![
                "Hey, what's up?",
                "I am a professional Marine Sphere Bottle maker.",
                "You can call me Boomer."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn bomb_maker(ctx: &Ctx) -> Script {
    bomb_maker_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum MoksMushroomsMush1Step {
    Start,
    OnTimer20000,
}

fn moks_mushrooms_mush1_run(ctx: &Ctx, mut step: MoksMushroomsMush1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MoksMushroomsMush1Step::Start => {
                if (ctx.var("hg_tre").get()?.number()? > 9 && ctx.var("hg_tre").get()?.number()? < 15) {
                    ctx.mes("- You found mushrooms that are as big as your palm. -")?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Gather them.:Pass.")])? {
                        1 => {
                            ctx.mes("- You decided to gather the mushrooms. -")?;
                            ctx.next()?;
                            ctx.mes("- *Snip Snip* -")?;
                            ctx.next()?;
                            ctx.lines(args!["- *Snip Snip* -", "- *Snip Snip* -"])?;
                            ctx.next()?;
                            ctx.lines(args!["- *Snip Snip* -", "- *Snip Snip* -", "- *Snip Snip* -"])?;
                            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_CONE")?])?;
                            ctx.next()?;
                            if ctx.call(Function::Rand, vec![Val::from(1), Val::from(5)])?.number()? > 4 {
                                ctx.mes("- You were being clumsy and broke the mushrooms. You have failed in gathering the mushrooms. -")?;
                                ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_STUNATTACK")?])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.mes("- You have successfully gathered mushrooms. -")?;
                            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_VALLENTINE2")?])?;
                            ctx.var("hg_tre").set((ctx.var("hg_tre").get()? + Val::from(1)))?;
                            if ctx.var("hg_tre").get()? == 11 {
                                ctx.mes("Collected Moks Mushroom Solution: 1ea ")?;
                            } else if ctx.var("hg_tre").get()? == 12 {
                                ctx.mes("Collected Moks Mushroom Solution: 2ea ")?;
                            } else if ctx.var("hg_tre").get()? == 13 {
                                ctx.mes("Collected Moks Mushroom Solution: 3ea ")?;
                            } else if ctx.var("hg_tre").get()? == 14 {
                                ctx.mes("Collected Moks Mushroom Solution: 4ea ")?;
                            } else if ctx.var("hg_tre").get()? == 15 {
                                ctx.mes("Collected Moks Mushroom Solution: 5ea ")?;
                            }
                            ctx.call(Function::InitNpcTimer, vec![])?;
                            ctx.call(Function::DisableNpc, vec![Val::from("Moks Mushrooms#Mush1")])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.mes("- You decided to pass by them. -")?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else if ctx.var("hg_tre").get()? == 15 {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["I think that this will do."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                step = MoksMushroomsMush1Step::OnTimer20000;
                continue 'machine;
            }
            MoksMushroomsMush1Step::OnTimer20000 => {
                ctx.call(Function::EnableNpc, vec![Val::from("Moks Mushrooms#Mush1")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn moks_mushrooms_mush1(ctx: &Ctx) -> Script {
    moks_mushrooms_mush1_run(ctx, MoksMushroomsMush1Step::Start, Vec::new()).map(|_| ())
}

pub fn moks_mushrooms_mush1_ontimer20000(ctx: &Ctx) -> Script {
    moks_mushrooms_mush1_run(ctx, MoksMushroomsMush1Step::OnTimer20000, Vec::new()).map(|_| ())
}

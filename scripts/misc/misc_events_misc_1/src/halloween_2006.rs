#![allow(
    unused_imports,
    unused_labels,
    unused_assignments,
    unused_mut,
    unused_parens,
    unused_variables,
    unreachable_code
)]

use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, constants, runtime};

pub fn gloomy_jack_06_hw(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn gloomy_jack_06_hw_ontouch(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Cool Devi",
        args![
            "If you have something to say to him,",
            "just talk to me.",
            "You won't even understand what stupid pumkin headed Jack is saying."
        ],
    )?;
    ctx.close()
}

pub fn gloomy_jack_06_hw_oneffect(ctx: &Ctx) -> Script {
    ctx.npc().special_effect(constants::EF_LEVEL99)?;
    ctx.end()
}

pub fn gloomy_jack_06_hw_oneffect2(ctx: &Ctx) -> Script {
    ctx.npc().special_effect(constants::EF_HIT2)?;
    ctx.end()
}

pub fn cool_devi_06_hw(ctx: &Ctx) -> Script {
    if ctx.var("MaxWeight").get()?.number()? - ctx.var("Weight").get()?.number()? < 2000
        || ctx.call(Function::CheckWeight, args![1201, 1])? == 0
    {
        ctx.lines(args![
            "^3355FFWait a second!",
            "Right now, you're carrying",
            "too many things with you.",
            "Please come back after",
            "using the Kafra Service",
            "to store some of your items.^000000"
        ])?;
        return ctx.close();
    }
    if ctx.items().count(7609)? > 0 {
        ctx.lines_as(
            "Gloomy Jack",
            args!["Oh wait, is that a Pumpkin Mojo you are carrying?", " ", "[Cool Devi]", "Says he."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Gloomy Jack",
            args![
                "Give it to me. I'll pay you back.",
                " ",
                "[Cool Devi]",
                "Says he...",
                "I just can guess what he will say next. So what do you say?"
            ],
        )?;
        ctx.next()?;
        if ctx.menu(&["What will he say?", "I also know about it."])? == 0 {
            ctx.lines_as(ctx.player().name()?, args!["What will he say?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Cool Devi",
                args![
                    "Jack is not like others. He is halloween Jack.",
                    "but one day, he lost his Pumpkin Mojo and turned into depressed Jack..",
                    "Pumpkin Mojo was his all."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Cool Devi",
                args![
                    "Well, actually that's the main reason Jack came to town.",
                    "To find his Pumpkin Mojo. Most suspicious suspect is ^4d4dffDelightful Lude^000000, the one known as Halloween monster"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Cool Devi",
                args![
                    "Why don't you give him back the 'Pumpkin Mojo'.",
                    "Don't worry he will compensate you."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(ctx.player().name()?, args!["Compensate?With what?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Cool Devi",
                args![
                    "If you bring ^4d4dff 1 Pumpkin Mojo^000000 and a pumpkin head that we gave out to you last year,",
                    "he will make it to ^00ff00the most fantastic pumpkin hat^000000.",
                    "Or if you don't have pumpkin head, you can just bring a ^3d3dff Pumpkin Mojo, a pumpkin, and a cap ^000000."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Cool Devi",
                args![
                    "Well, that Pumpkin Mojo is useless if you just carry it. ",
                    "Give it to Jack. Don't you feel pity for him?."
                ],
            )?;
            return ctx.close();
        }
        ctx.lines_as("Cool Devi", args!["Oh~Great.", "Then, let me see what you got."])?;
        ctx.next()?;
        if ctx.items().count(7609)? > 0 && ctx.items().count(5134)? > 0 {
            ctx.lines_as(
                "Gloomy Jack",
                args![
                    "Oh, you brought the pumpkin head!",
                    "I'll make you to nicer one.",
                    " ",
                    "[Cool Devi]",
                    "says he."
                ],
            )?;
            ctx.next()?;
            if ctx.menu(&["No, I'll come back later.", "Oh, good. Make it now!"])? == 0 {
                ctx.lines_as(
                    "Gloomy Jack",
                    args![
                        "Huh?",
                        " ",
                        "[Cool Devi]",
                        "What? Look at Jack. He has got so dissapointed.",
                        "Promise me to give the Pumpkin Mojo back to Jack later someday, will you?"
                    ],
                )?;
                return ctx.close();
            }
            ctx.lines_as(
                "Gloomy Jack",
                args![
                    "Thanks for giving my thing back, I'll make your hat prettier.",
                    " ",
                    "[Cool Devi]",
                    "says he."
                ],
            )?;
            ctx.next()?;
            ctx.npc().do_event("Gloomy Jack#06_hw::OnEffect")?;
            ctx.lines_as(
                "Excited Jack",
                args!["Lalala~ lalala~", " ", "(He starts singing. On a sudden, Jack's aura appeared.)"],
            )?;
            ctx.items().take(7609, 1)?;
            ctx.items().take(5134, 1)?;
            ctx.items().give(5202, 1)?;
            ctx.next()?;
            ctx.npc().do_event("Gloomy Jack#06_hw::OnEffect2")?;
            ctx.lines_as(
                "Gloomy Jack",
                args![
                    "My aura is not like it used to be. Maybe I need more Pumpkin Mojo.",
                    " ",
                    "[Cool Devi]",
                    "says he..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Cool Devi",
                args!["Oh~poor Jack.", "Well, someday he'll get back all his Pumpkin Mojo back."],
            )?;
            return ctx.close();
        } else if ctx.items().count(7609)? > 0 && ctx.items().count(535)? > 0 && ctx.items().count(2226)? > 0 {
            ctx.lines_as(
                "Gloomy Jack",
                args!["Wow!", "I'll turn your hat to very cool one.", " ", "[Cool Devi]", "says he..."],
            )?;
            ctx.next()?;
            if ctx.menu(&["No, I'll come back later.", "Oh,good. Make it now!"])? == 0 {
                ctx.lines_as(
                    "Gloomy Jack",
                    args![
                        "Huh?",
                        " ",
                        "[Cool Devi]",
                        "What? Look at Jack. He has got so dissapointed.",
                        "Promise me to give the Pumpkin Mojo back to Jack later someday,will you?"
                    ],
                )?;
                return ctx.close();
            }
            ctx.lines_as(
                "Gloomy Jack",
                args![
                    "Thanks for giving my thing back, I'll make your hat prettier.",
                    " ",
                    "[Cool Devi]",
                    "says he."
                ],
            )?;
            ctx.next()?;
            ctx.npc().do_event("Gloomy Jack#06_hw::OnEffect")?;
            ctx.lines_as(
                "Excited Jack",
                args!["Lalala~ lalala~", " ", "(He starts singing. On a sudden, Jack's aura appeared.)"],
            )?;
            ctx.items().take(7609, 1)?;
            ctx.items().take(535, 1)?;
            ctx.items().take(2226, 1)?;
            ctx.items().give(5202, 1)?;
            ctx.next()?;
            ctx.npc().do_event("Gloomy Jack#06_hw::OnEffect2")?;
            ctx.lines_as(
                "Gloomy Jack",
                args![
                    "My aura is not like it used to be. Maybe I need more Pumpkin Mojo.",
                    " ",
                    "[Cool Devi]",
                    "says he..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Cool Devi",
                args!["Oh~poor Jack.", "Well, someday he'll get back all his Pumpkin Mojo back."],
            )?;
            return ctx.close();
        } else {
            ctx.lines_as(
                "Cool Devi",
                args!["Hey,there. You don't seem to have all materials for the fantastic hat. "],
            )?;
            return ctx.close();
        }
    } else {
        ctx.lines_as(
            "Gloomy Jack",
            args![
                "Hey,there. Why don't you come here and listen to my story.",
                " ",
                "[Cool Devi]",
                "Says he."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Gloomy Jack",
            args![
                "I used to be very famous. But now.....",
                " ",
                "[Cool Devi]",
                "Says he...",
                "Well, I'll just tell you without translating."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Cool Devi",
            args![
                "Jack is not like others. He is halloween Jack.",
                "but one day, he lost his Pumpkin Mojo and turned into depressed Jack..",
                "Pumpkin Mojo was his all."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Cool Devi",
            args![
                "Well, actually that's the main reason Jack came to town.",
                "To find his Pumpkin Mojo. Most suspicious suspect is ^4d4dffDelightful Lude^000000, the one known as Halloween monster."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Cool Devi",
            args![
                "Why don't you give him back the 'Pumpkin Mojo'.",
                "Don't worry he will compensate you."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(ctx.player().name()?, args!["Compensate? With what?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Cool Devi",
            args![
                "If you bring ^4d4dff 1 Pumpkin Mojo^000000 and a pumpkin head that we gave out to you last year,",
                "he will make it to ^00ff00the most fantastic pumpkin hat^000000.",
                "Or if you don't have pumpkin head, you can just bring a ^3d3dff Pumpkin Mojo, a pumpkin, and a cap ^000000."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Cool Devi",
            args![
                "Well, that Pumpkin Mojo is useless if you just carry it. ",
                "Give it to Jack. Don't you feel pity for him?."
            ],
        )?;
        return ctx.close();
    }
}

pub fn hoirin_06_hw(ctx: &Ctx) -> Script {
    if ctx.var("MaxWeight").get()?.number()? - ctx.var("Weight").get()?.number()? < 2000
        || ctx.call(Function::CheckWeight, args![1201, 1])? == 0
    {
        ctx.lines(args![
            "^3355FFWait a second!",
            "Right now, you're carrying",
            "too many things with you.",
            "Please come back after",
            "using the Kafra Service",
            "to store some of your items.^000000"
        ])?;
        return ctx.close();
    }
    if ctx.var("halloween").get()?.number()? < 100 {
        ctx.lines_as(
            "Hoirin",
            args![
                "Pumpkin is the gift that God had sent us.",
                "It provides good nutrition and helps you to stay healthy!",
                "Bravo~bravo~!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Hoirin",
            args![
                "I, Hoirin have always thought about a ",
                "way to eat pumpkin more deliciously.",
                "But all of a sudden, I realized."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Hoirin", args!["If I have that! I can make best the Pumpkin pie!!!!!"])?;
        ctx.next()?;
        if ctx.menu(&["What's that?", "Ignore."])? == 0 {
            ctx.lines_as(
                "Hoirin",
                args![
                    "What's the first image you see when you think of pumpkin?",
                    "I see Jack! I think Jack is the most evolved form of pumpkin!",
                    "I wonder what does Jack have something special? Can you imagine?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Hoirin",
                args![
                    "I always prefer unique pumpkin dish! Recently, I found out that there are many kinds of Jack in the world ",
                    "and the extreme class Jack has blue aura around his body!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Hoirin",
                args![
                    "Guess what?! it was Halloween Jack!",
                    "Halloween Jack has somthing that is concentrated with pumpkin.",
                    "It is called ^4d4dffPumpkin Mojo^000000. Only if i have it, i can finish making my special pie."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Hoirin",
                args![
                    "Bring me the ingredients!",
                    "I'll make you a very special pie right away!",
                    "You'll get addicted to it.",
                    "Don't be surprised after trying it."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.player().name()?,
                args![
                    "......................",
                    "You seem to be such a passionate person.",
                    "So, what's the ingredient anyway.?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Hoirin",
                args![
                    "Good question!",
                    "To make my special Pumpkin pie, I need",
                    "^4d4dff 1 Pumpkin Mojo",
                    "2 Pumpkin",
                    "2 Egg^000000"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Hoirin",
                args![
                    "Just Bring Pumpkin Mojo, Pumpkin, and a Egg.",
                    "Leave the rest for me..You'll get to try the best pumkin pie!"
                ],
            )?;
            ctx.var("halloween").set(Val::from(100))?;
            return ctx.close();
        }
        ctx.lines_as(
            "Hoirin",
            args!["Hm..You'll regret it!", "Hoirin's pumpkin pie is just so delicious."],
        )?;
        return ctx.close();
    } else {
        ctx.lines_as(
            "Hoirin",
            args!["So, did you bring all materials?", "Were there anything hard to find?"],
        )?;
        ctx.next()?;
        match ctx.menu(&["I brought all ingredients.", "How can I get eggs?", "No,I'm just passing by."])? {
            0 => {
                if ctx.items().count(7609)? > 0 && ctx.items().count(535)? > 1 && ctx.items().count(574)? > 1 {
                    ctx.lines_as(
                        "Hoirin",
                        args!["You are right!", "You brought all things right. Give it to me!!"],
                    )?;
                    ctx.fx().special_effect(constants::EF_HIT1)?;
                    ctx.next()?;
                    ctx.npc().special_effect(constants::EF_PIERCESELF)?;
                    ctx.lines_as("Hoirin", args!["Abracadabra~~", "Abracadabra~~!"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Hoirin",
                        args![
                            "Ta-da(h)!",
                            "Here it is~ help yourself.",
                            "It'll be good for your health. I added extra nutrient in it."
                        ],
                    )?;
                    ctx.items().take(7609, 1)?;
                    ctx.items().take(535, 2)?;
                    ctx.items().take(574, 2)?;
                    ctx.items().give(12192, 1)?;
                    return ctx.close();
                } else {
                    ctx.lines_as(
                        "Hoirin",
                        args![
                            "No!! This is not enough. I need",
                            "^4d4dff 1 Pumpkin Mojo",
                            "2 Pumpkin",
                            "2 Egg^000000",
                            " at least."
                        ],
                    )?;
                    return ctx.close();
                }
            }
            1 => {
                if ctx.var("halloween").get()? == 100 {
                    ctx.lines_as(
                        "Hoirin",
                        args![
                            "Egg?",
                            "Well, actually I know someone who can help you.",
                            "There's a man who raise many chickens."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Hoirin",
                        args!["I can send you to where he is if you want.", "^4d4dffBut only for once^000000."],
                    )?;
                    ctx.next()?;
                    if ctx.menu(&["Send me.", "It's ok."])? == 0 {
                        ctx.lines_as("Hoirin", args!["Ok, go get the eggs."])?;
                        ctx.var("halloween").set(Val::from(101))?;
                        ctx.close_window()?;
                        ctx.warp("nif_fild01", 162, 113)?;
                        return ctx.end();
                    }
                    ctx.lines_as(
                        "Hoirin",
                        args![
                            "Do you think you can go without my help?!",
                            "Ok! go ahead.",
                            "Do you know where it is?"
                        ],
                    )?;
                    return ctx.close();
                } else {
                    ctx.lines_as("Hoirin", args!["if you go to ^4d4dfffarm of Skelington Town at Neiflheim^000000, you'll meet Chicken Masta. He raises chickens."])?;
                    return ctx.close();
                }
            }
            2 => {
                ctx.lines_as(
                    "Hoirin",
                    args!["Next time, don't forget to bring ", "all the ingredients for pumpkin pie!"],
                )?;
                return ctx.close();
            }
            _ => {}
        }
    }
    Ok(())
}

pub fn loli_ruri_06_hw(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn loli_ruri_06_hw_ontouch(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Deviruchi",
        args![
            "Hey there, if you have something to say to Loli Ruri, talk to me.",
            "She's not used to human language."
        ],
    )?;
    ctx.close()
}

pub fn deviruchi_06_hw(ctx: &Ctx) -> Script {
    if ctx.var("MaxWeight").get()?.number()? - ctx.var("Weight").get()?.number()? < 2000
        || ctx.call(Function::CheckWeight, args![1201, 1])? == 0
    {
        ctx.lines(args![
            "^3355FFWait a second!",
            "Right now, you're carrying",
            "too many things with you.",
            "Please come back after",
            "using the Kafra Service",
            "to store some of your items.^000000"
        ])?;
        return ctx.close();
    }
    ctx.lines_as(
        "Loli Ruri",
        args![
            "Devi~Where's the pumpkin pie?",
            "Humans make them, don't they?",
            "Do you have it then?",
            " ",
            "[Deviruchi]",
            "Says she."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Deviruchi",
        args![
            "If you have a pumpinkin pie, can I have one?",
            "Cause we are the victim if Loli Ruri gets mad."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Deviruchi",
        args!["Well I'm not saying that I want it for free.", "Don't worry, I won't let you down."],
    )?;
    ctx.next()?;
    match ctx.menu(&[
        "Give him the pumpkin pie.",
        "Do not give him the pumpkin pie.",
        "Huh? What pumkin pie?",
    ])? {
        0 => {
            if ctx.items().count(12192)? > 0 {
                ctx.lines_as(
                    "Loli Ruri",
                    args!["Thanks for the pumkin pie!", "Here, take this.", " ", "[Deviruchi]", "Says she."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Deviruchi",
                    args!["As I promised, I'll give you something worth the pie.", "Hang on...."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Deviruchi",
                    args!["Hmm...This would be good.", "Here take this, and thanks again~"],
                )?;
                ctx.items().take(12192, 1)?;
                ctx.var("@hw_temp").set(ctx.call(Function::Rand, args![1, 3])?)?;
                if ctx.var("@hw_temp").get()? == 2 {
                    ctx.items().give(12130, 1)?;
                } else {
                    ctx.items().give(7460, 3)?;
                }
                return ctx.close();
            } else {
                ctx.lines_as(
                    "Loli Ruri",
                    args![
                        "Are you kidding me?",
                        "Where's the pumpkin pie! Don't try to lie to me!",
                        " ",
                        "[Deviruchi]",
                        "Say she..Seems like she's very angry...You are in trouble."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Loli Ruri",
                    args![
                        "Devi!!!!!You are the one who told me that I can get the pumpkin pie if I stand here and wait.",
                        "Watch your back!",
                        " ",
                        "[Deviruchi]",
                        "Says she..Huh?!!Me?!!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.player().name()?,
                    args!["Poor Devi~", "You shouldn't have lied~", "Wish you a luck."],
                )?;
                return ctx.close();
            }
        }
        1 => {
            ctx.lines_as(
                "Loli Ruri",
                args![
                    "I want to eat pumpkin pie. Can I have one?Please~~~~",
                    " ",
                    "[Deviruchi]",
                    "Says he.....Give me if you have one."
                ],
            )?;
            return ctx.close();
        }
        2 => {
            ctx.lines_as(
                "Deviruchi",
                args![
                    "There's a man who makes a very special pumpkin pie.",
                    "No one can forget what it taste like."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Deviruchi",
                args![
                    "Lori Ruri is waiting here for someone who will get the pie for him.",
                    "Can you get her one?",
                    "I'll treat you back."
                ],
            )?;
            return ctx.close();
        }
        _ => {}
    }
    Ok(())
}

pub fn chicken_masta_06_hw(ctx: &Ctx) -> Script {
    if ctx.var("MaxWeight").get()?.number()? - ctx.var("Weight").get()?.number()? < 2000
        || ctx.call(Function::CheckWeight, args![1201, 1])? == 0
    {
        ctx.lines(args![
            "^3355FFWait a second!",
            "Right now, you're carrying",
            "too many things with you.",
            "Please come back after",
            "using the Kafra Service",
            "to store some of your items.^000000"
        ])?;
        return ctx.close();
    }
    if ctx.var("halloween").get()? == 101 {
        ctx.lines_as(
            "Chicken Masta",
            args!["Where did the chicken have gone?!", "Oh, hey stranger. How can I help you?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Chicken Masta",
            args![
                "This town is too dangerous for you to hang around.",
                "You'd better go back to where you came from...."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Chicken Masta",
            args![
                "...Are you looking for eggs?",
                "Recently, people are asking me for eggs to make some kind of pie....are you one of them?"
            ],
        )?;
        ctx.next()?;
        if ctx.menu(&["Actually, yes. I came here to get some eggs.", "Nope."])? == 0 {
            ctx.lines_as(
                "Chicken Masta",
                args![
                    "Ok. But some of my chickens ran away,",
                    "so I have no eggs much left.",
                    "1000 zeny for 2 eggs! how's that?"
                ],
            )?;
            ctx.next()?;
            match ctx.menu(&["Ok,I'll take it.", "I'll just buy one.", "Whew~it's too expensive."])? {
                0 => {
                    if ctx.player().zeny()? > 1999 {
                        ctx.lines_as("Chicken Masta", args!["Thanks.", "Here are the eggs."])?;
                        ctx.player().set_zeny(ctx.player().zeny()? - 2000)?;
                        ctx.var("halloween").set(Val::from(102))?;
                        ctx.items().give(574, 2)?;
                        return ctx.close();
                    } else {
                        ctx.lines_as(
                            "Chicken Masta",
                            args![
                                "Hey this is not enough~",
                                "1000zeny per each, so if you buy two,that means 2000zeny."
                            ],
                        )?;
                        return ctx.close();
                    }
                }
                1 => {
                    if ctx.player().zeny()? > 999 {
                        ctx.lines_as("Chicken Masta", args!["Thanks.", "Here are the eggs."])?;
                        ctx.player().set_zeny(ctx.player().zeny()? - 1000)?;
                        ctx.var("halloween").set(Val::from(102))?;
                        ctx.items().give(574, 1)?;
                        return ctx.close();
                    } else {
                        ctx.lines_as(
                            "Chicken Masta",
                            args!["You don't seem to have enough money...", "It's 1000 zeny per each."],
                        )?;
                        return ctx.close();
                    }
                }
                2 => {
                    ctx.lines_as(
                        "Chicken Masta",
                        args![
                            "Well...I know it's liitle bit expensive but as I told you, my chickens ran away.",
                            "You have to understand."
                        ],
                    )?;
                    return ctx.close();
                }
                _ => {}
            }
        }
        ctx.lines_as(
            "Chicken Masta",
            args![
                "Let me tell you just one thing!",
                "If you don't want to die,",
                "you'd better run away. It's too dangerous in here."
            ],
        )?;
        return ctx.close();
    } else if ctx.var("halloween").get()? == 102 {
        ctx.lines_as(
            "Chicken Masta",
            args![
                "Do you need eggs?...",
                "Then, help me first.",
                "8 of my chickens ran away.",
                "I just don't know where they are."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Chicken Masta",
            args![
                "I don't expect all chickens to come back home.",
                "Only if you find me one of them, I'll sell three eggs for you."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Chicken Masta",
            args!["Isn't it a great deal?", "Chickens must be around here somewhere."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Chicken Masta",
            args![
                "You don't have to bring back chickens to me if you find one,",
                "Just insert the word ^4d4dff'Return'^000000.",
                "........."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Chicken Masta",
            args![
                "It's a spell to make chickens to go back home.",
                ".........",
                "Don't forget the word 'Return'!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Chicken Masta", args!["I hope you can find my chickens."])?;
        return ctx.close();
    } else if ctx.var("halloween").get()? == 103 {
        ctx.lines_as(
            "Chicken Masta",
            args![
                "Oh, you came back. I've been waiting for you.",
                "And thanks for the chickens you sent me.",
                "They are saftly kept in the henhouse, in case of running away again."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Chicken Masta",
            args![
                "I didn't forget what we have promised.",
                "Tell me how many eggs you want.",
                "It's 1000zeny per each.",
                "And maximum 3 is all you can get."
            ],
        )?;
        ctx.next()?;
        loop {
            let (input, _) = runtime::input_number(ctx, None, None)?;
            if input == 0 {
                ctx.lines_as("Chicken Masta", args!["You don't have to buy it, if you don't need it."])?;
                return ctx.close();
            }
            if ctx.var("@input").get()?.number()? <= 3 {
                break;
            }
            ctx.lines_as(
                "Chicken Masta",
                args!["What did I tell you.", "I'm going to sell only three eggs."],
            )?;
            ctx.next()?;
        }
        if ctx.player().zeny()? < ctx.var("@hw_egg").get()?.number()? {
            ctx.lines_as("Chicken Masta", args!["You don't seem to have enough zeny."])?;
            return ctx.close();
        }
        ctx.lines_as(
            "Chicken Masta",
            args!["Here you are.", "But no more.", "If you want more eggs, find me more chickens."],
        )?;
        ctx.player().set_zeny(ctx.player().zeny()? - ctx.var("@hw_egg").get()?.number()?)?;
        ctx.call(Function::GetItem, args![574, ctx.var("@input").get()?])?;
        ctx.var("halloween").set(Val::from(102))?;
        return ctx.close();
    } else {
        ctx.lines_as(
            "Chicken Masta",
            args!["Where did the chicken have gone?!", "Oh, hey stranger. How can I help you?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Chicken Masta",
            args![
                "This town is too dangerous for you to hang around.",
                "You'd better go back to where you came from...."
            ],
        )?;
        return ctx.close();
    }
}

pub fn masta_s_chicken_06_hw01(ctx: &Ctx) -> Script {
    ctx.mes("Drowsing chicken.")?;
    ctx.close()
}

pub fn hwchicken(ctx: &Ctx) -> Script {
    if ctx.var("halloween").get()? == 102 || ctx.var("halloween").get()? == 103 {
        ctx.lines(args![
            "Oh, this must be Chicken Masta's chicken.",
            "I should insert the magic word."
        ])?;
        ctx.next()?;
        let (input, _) = runtime::input_text(ctx, None, None)?;
        if input == "Return" {
            ctx.npc().emotion(constants::ET_HUK)?;
            ctx.npc().special_effect(constants::EF_TELEPORTATION)?;
            ctx.mes("The magic spell has been casted.")?;
            ctx.var("halloween").set(Val::from(103))?;
            ctx.call(
                Function::DisableNpc,
                vec![(Val::from("Masta's chicken#") + ctx.call(Function::StrNpcInfo, args![1])?)],
            )?;
            ctx.close_window()?;
            ctx.call(Function::Sleep, args![180000])?;
            ctx.call(
                Function::EnableNpc,
                vec![(Val::from("Masta's chicken#") + ctx.call(Function::StrNpcInfo, args![1])?)],
            )?;
            return ctx.end();
        } else {
            ctx.lines_as(ctx.player().name()?, args!["Hm...I must have misspelled."])?;
            return ctx.close();
        }
    } else {
        ctx.mes("Drowsing chicken.")?;
        return ctx.close();
    }
}

pub fn hwchicken2(ctx: &Ctx) -> Script {
    ctx.var("@egg_temp").set(ctx.call(Function::Rand, args![1, 4])?)?;
    if ctx.var("@egg_temp").get()? == 3 {
        ctx.lines(args![
            "As soon as you got close to the chicken and touched it, it disappeared completely.",
            "You got an 'egg' in the place where the chicken disappeared."
        ])?;
        ctx.call(
            Function::DisableNpc,
            vec![(Val::from("Chicken#") + ctx.call(Function::StrNpcInfo, args![1])?)],
        )?;
        ctx.items().give(574, 1)?;
        return ctx.close();
    } else {
        ctx.mes("As soon as you got close to the chicken and touched it, it disappeared completely.")?;
        ctx.call(
            Function::DisableNpc,
            vec![(Val::from("Chicken#") + ctx.call(Function::StrNpcInfo, args![1])?)],
        )?;
        return ctx.close();
    }
}

pub fn hwchicken3(ctx: &Ctx) -> Script {
    ctx.var("@egg_temp").set(ctx.call(Function::Rand, args![1, 4])?)?;
    if ctx.var("@egg_temp").get()? == 3 {
        ctx.lines(args![
            "As soon as you got close to the chicken and touched it, it disappeared completely.",
            "You got an 'egg' in the place where the chicken disappeared."
        ])?;
        ctx.call(
            Function::DisableNpc,
            vec![(Val::from("Chicken#") + ctx.call(Function::StrNpcInfo, args![1])?)],
        )?;
        ctx.items().give(574, 1)?;
        return ctx.close();
    } else {
        ctx.mes("As soon as you got close to the chicken and touched it, it disappeared completely.")?;
        ctx.call(
            Function::DisableNpc,
            vec![(Val::from("Chicken#") + ctx.call(Function::StrNpcInfo, args![1])?)],
        )?;
        return ctx.close();
    }
}

pub fn hwchicken3_oninit(ctx: &Ctx) -> Script {
    ctx.call(Function::Sleep, args![180000])?;
    ctx.npc().special_effect(constants::EF_BAT2)?;
    ctx.end()
}

#[derive(Clone, Copy, Debug)]
enum S06HwTimer01Step {
    Start,
    OnInit,
    OnTimer3600000,
    OnTimer4200000,
}

fn s_06_hw_timer01_run(ctx: &Ctx, mut step: S06HwTimer01Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            S06HwTimer01Step::Start => {
                step = S06HwTimer01Step::OnInit;
                continue 'machine;
            }
            S06HwTimer01Step::OnInit => {
                for i in 1..=41 {
                    ctx.set_npc_visible(&format!("Chicken#06_hw_p{i:02}"), false)?;
                }
                ctx.call(Function::InitNpcTimer, args![])?;
                return Err(Stop::End);
            }
            S06HwTimer01Step::OnTimer3600000 => {
                for i in 1..=41 {
                    ctx.set_npc_visible(&format!("Chicken#06_hw_p{i:02}"), true)?;
                }
                return Err(Stop::End);
            }
            S06HwTimer01Step::OnTimer4200000 => {
                for i in 1..=41 {
                    ctx.set_npc_visible(&format!("Chicken#06_hw_p{i:02}"), false)?;
                }
                ctx.call(Function::StopNpcTimer, args![])?;
                ctx.call(Function::InitNpcTimer, args![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn s_06_hw_timer01(ctx: &Ctx) -> Script {
    s_06_hw_timer01_run(ctx, S06HwTimer01Step::Start, Vec::new()).map(|_| ())
}

pub fn s_06_hw_timer01_oninit(ctx: &Ctx) -> Script {
    s_06_hw_timer01_run(ctx, S06HwTimer01Step::OnInit, Vec::new()).map(|_| ())
}

pub fn s_06_hw_timer01_ontimer3600000(ctx: &Ctx) -> Script {
    s_06_hw_timer01_run(ctx, S06HwTimer01Step::OnTimer3600000, Vec::new()).map(|_| ())
}

pub fn s_06_hw_timer01_ontimer4200000(ctx: &Ctx) -> Script {
    s_06_hw_timer01_run(ctx, S06HwTimer01Step::OnTimer4200000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum S06HwTimer02Step {
    Start,
    OnInit,
    OnTimer5400000,
    OnTimer6000000,
}

fn s_06_hw_timer02_run(ctx: &Ctx, mut step: S06HwTimer02Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            S06HwTimer02Step::Start => {
                step = S06HwTimer02Step::OnInit;
                continue 'machine;
            }
            S06HwTimer02Step::OnInit => {
                for i in 1..=27 {
                    ctx.set_npc_visible(&format!("Chicken#06_hw_pf{i:02}"), false)?;
                }
                ctx.call(Function::InitNpcTimer, args![])?;
                return Err(Stop::End);
            }
            S06HwTimer02Step::OnTimer5400000 => {
                for i in 1..=27 {
                    ctx.set_npc_visible(&format!("Chicken#06_hw_pf{i:02}"), true)?;
                }
                return Err(Stop::End);
            }
            S06HwTimer02Step::OnTimer6000000 => {
                for i in 1..=27 {
                    ctx.set_npc_visible(&format!("Chicken#06_hw_pf{i:02}"), false)?;
                }
                ctx.call(Function::StopNpcTimer, args![])?;
                ctx.call(Function::InitNpcTimer, args![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn s_06_hw_timer02(ctx: &Ctx) -> Script {
    s_06_hw_timer02_run(ctx, S06HwTimer02Step::Start, Vec::new()).map(|_| ())
}

pub fn s_06_hw_timer02_oninit(ctx: &Ctx) -> Script {
    s_06_hw_timer02_run(ctx, S06HwTimer02Step::OnInit, Vec::new()).map(|_| ())
}

pub fn s_06_hw_timer02_ontimer5400000(ctx: &Ctx) -> Script {
    s_06_hw_timer02_run(ctx, S06HwTimer02Step::OnTimer5400000, Vec::new()).map(|_| ())
}

pub fn s_06_hw_timer02_ontimer6000000(ctx: &Ctx) -> Script {
    s_06_hw_timer02_run(ctx, S06HwTimer02Step::OnTimer6000000, Vec::new()).map(|_| ())
}

pub fn familiar_06_hw01(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Familiar",
        args![
            "Hello.",
            "I'm Loli Ruri's faithful and cute Familiar.",
            "Do you have an invitation from Loli Ruri?",
            "That's a kind of^4d4dffa special ticket to Niflheim^000000.."
        ],
    )?;
    ctx.next()?;
    if ctx.menu(&["Yes, I do.", "No, I don't."])? == 0 {
        ctx.lines_as(
            "Familiar",
            args!["Do you want to go to Niflheim?", "It's available during Halloween."],
        )?;
        ctx.next()?;
        if ctx.menu(&["Yes, I do", "No, I don't."])? == 0 {
            if ctx.items().count(7460)? > 0 {
                ctx.lines_as("Familiar", args!["I checked your ticket.", "You can go there now"])?;
                ctx.items().take(7460, 1)?;
                ctx.close_window()?;
                ctx.warp("nif_in", 18, 20)?;
                return ctx.end();
            } else {
                ctx.lines_as(
                    "Familiar",
                    args![
                        "You're a liar.",
                        "You don't have the ticket!",
                        "I'll suck up all your blood, you liar!"
                    ],
                )?;
                ctx.call(Function::PercentHeal, args![-20, 0])?;
                ctx.call(
                    Function::Emotion,
                    args![
                        constants::ET_HUK,
                        Val::from(ctx.call(Function::GetCharacterId, args![0])?.is_true()),
                    ],
                )?;
                return ctx.close();
            }
        }
        ctx.lines_as(
            "Familiar",
            args!["Ok.", "Actually, it's useless to have a special ticket to Niflheim."],
        )?;
        return ctx.close();
    }
    ctx.lines_as(
        "Familiar",
        args![
            "Ok.",
            "If you want to ask something, give a piece of pumpkin pie to Loli Ruri.",
            "He likes it."
        ],
    )?;
    ctx.close()
}

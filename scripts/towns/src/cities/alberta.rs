#![allow(
    unused_imports,
    unused_labels,
    unused_assignments,
    unused_mut,
    unused_parens,
    unused_variables,
    unreachable_code
)]

use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

pub fn fabian(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Fabian",
        args!["Man... When you travel all around the world, you'll hear of some crazy things."],
    )?;
    ctx.next()?;
    ctx.lines_as("Fabian", args!["Once, I heard that there are Cards which contain the power of monsters. If someone happens to get their hands on a card, they'll be able to use that monster's power."])?;
    ctx.next()?;
    ctx.lines_as("Fabian", args!["I'm guessing it's some sort of fad or scam, where they make you collect all the cards or whatever. I mean, how can a card really hold the power of a monster?!"])?;
    ctx.next()?;
    ctx.lines_as("Fabian", args!["Seriously..."])?;
    ctx.close()
}

pub fn steiner(ctx: &Ctx) -> Script {
    ctx.lines_as("Steiner", args!["Oh...!", "Welcome to Alberta,", "young adventurer!"])?;
    ctx.next()?;
    ctx.lines_as("Steiner", args!["Pardon me if I seem distracted. I'm milling about, trying to make a plan. You see, I hear that there is a store in Geffen that sells armor that is resistant to magic."])?;
    ctx.next()?;
    ctx.lines_as(
        "Steiner",
        args!["If I buy a lot of them in bulk, and then resell them here for a higher price..."],
    )?;
    ctx.close()
}

pub fn chad(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Chad",
        args!["People say the legendary weapon Gungnir never misses its target. I wonder if it's possibly true..."],
    )?;
    ctx.next()?;
    ctx.lines_as("Chad", args!["People also say that babies are assembled by the storks before delivery, girls dig guys who act like jerks, and that Santa Claus exists! But only in Lutie."])?;
    ctx.next()?;
    ctx.lines_as("Chad", args!["I wonder...", "If any of that", "is possibly", "true..."])?;
    ctx.close()
}

pub fn drunken_old_man(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Deagle",
        args!["^666666*Hiccup*^000000", "Wh-what are you", "staring at? Get lost!!"],
    )?;
    ctx.next()?;
    'b1: {
        let subject1 = runtime::select_values(ctx, &[Val::from("Say nothing."), Val::from("Leave him alone.")])?;
        let mut matched1 = false;
        if !matched1 && subject1 == 1 {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as("Deagle", args!["Hahahaha ^666666*hiccup*^000000... You've got some nerve. I may look worthless now, but I used to be a sailor on the 'Going Mary.'"])?;
            ctx.next()?;
            'b2: {
                let subject2 = runtime::select_values(ctx, &[Val::from("Never heard of it."), Val::from("Really? No kidding!")])?;
                let mut matched2 = false;
                if !matched2 && subject2 == 1 {
                    matched2 = true;
                }
                if matched2 {
                    ctx.lines_as(
                        "Deagle",
                        args!["Never heard of it?! Everybody knows th'notorious pirate ship 'Going Mary!' ^666666*Hiccup~*^000000"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Deagle",
                        args!["Ah~ The ol'days. If only... If only we hadn't run into that STORM...^666666*hiccup*^000000"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Deagle",
                        args!["AH~ Captain. I miss our cap'n more than anything... No foe survived before cap'n's sword."],
                    )?;
                    return ctx.close();
                }
                if !matched2 && subject2 == 2 {
                    matched2 = true;
                }
                if matched2 {
                    ctx.lines_as(
                        "Deagle",
                        args![
                            "That's right! NOBODY meshes with the crew of the 'Going Mary!' And nobody can beat out cap'n in a sword fight!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Deagle",
                        args!["CAPTAIN~!!! ^666666*HICCUP~*^000000 He would swing his sword like this, then... THEN!!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Deagle",
                        args!["The bastard the captain was fighting, and anyone of his friends near him, were surrounded in flame!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Deagle",
                        args!["Man, that sword must have had some sort of mysterious power, or the captain was just that good...!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Deagle",
                        args!["Phew~~ ^666666*Sob* *Sob...*^000000 God, I miss everyone! Now I'm depressed! Please, go away now."],
                    )?;
                    return ctx.close();
                }
            }
        }
        if !matched1 && subject1 == 2 {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as("Deagle", args!["That's right!", "Go AWAY~"])?;
            return ctx.close();
        }
    }
    Ok(())
}

pub fn shakir(ctx: &Ctx) -> Script {
    ctx.mes("[Shakir]")?;
    if ctx.call(Function::Rand, args![2])?.is_true() {
        ctx.mes("We Merchants have our own negotiating skill when we sell goods. This skill can get us more money than when other people sell goods.")?;
        ctx.next()?;
        ctx.lines_as(
            "Shakir",
            args!["It's more than just yelling 'You'll have to give more money please!' You need to have charisma, and master rhetoric!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Shakir",
            args!["We can get up to 24 % more zeny with this incredible skill. But remember to train hard to acquire it!!"],
        )?;
    } else {
        ctx.lines(args!["We Merchants can", "open roadside stands", "to do business."])?;
        ctx.next()?;
        ctx.lines_as(
            "Shakir",
            args!["With the Discount skill, we can buy goods really cheap from the stores in towns and load them into the cart we rent."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Shakir",
            args!["Then afterwards, we can travel anywhere, and sells our goods to make a profit!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Shakir",
            args!["This way, business is more convenient and safe. Don't fall asleep, although it's too easy to do that."],
        )?;
    }
    ctx.close()
}

pub fn sonya(ctx: &Ctx) -> Script {
    ctx.mes("[Sonya]")?;
    match ctx.call(Function::Rand, args![3])?.number()? {
        0 => {
            ctx.mes("Hey, you know, this one time I was walking through the forest and I saw this little green stem moving around.")?;
            ctx.next()?;
            ctx.lines_as(
                "Sonya",
                args!["I went to see what it was and when I went to touch it. The stem actually slapped my hand!"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sonya",
                args!["It startled me, so I jumped back a bit and then I realized it wasn't a stem, but a very small animal."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sonya",
                args!["I was lucky I didn't upset it. Even the smallest animal can be dangerous if angered."],
            )?;
            return ctx.close();
        }
        1 => {
            ctx.mes("You know those lazy looking bears that live in the forest on the way to Payon?")?;
            ctx.next()?;
            ctx.lines_as("Sonya", args!["Just for fun, I threw a rock at it and all of sudden it rushed at me! I was sooooo scared, I started to run away, then BAM!!!"])?;
            ctx.next()?;
            ctx.lines_as(
                "Sonya",
                args!["It ran into a low tree branch and knocked itself out! I swear, I'll never provoke an animal for fun again!"],
            )?;
            return ctx.close();
        }
        2 => {
            ctx.mes("I once saw a pack of wolves take on one of those huge, lazy bears!")?;
            ctx.next()?;
            ctx.lines_as("Sonya", args!["Wolves are much more cooperative than they may seem. If one of them is attacked, then any nearby wolves will run to help."])?;
            ctx.next()?;
            ctx.lines_as(
                "Sonya",
                args![
                    "I'd think twice if you ever want to fight one when others of its kind are around. Be careful: don't get ganged up on!"
                ],
            )?;
            return ctx.close();
        }
        _ => {}
    }
    Ok(())
}

pub fn grandmother_alma(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Grandmother Alma",
        args!["Some time ago,", "a derelict ship", "drifted into", "Alberta harbour."],
    )?;
    ctx.next()?;
    ctx.lines_as("Grandmother Alma", args!["Hoping to save any survivors, some of the townspeople ventured into the ship. However, they all ran out terrified, saying that corpses were walking around inside the ship."])?;
    ctx.next()?;
    ctx.lines_as(
        "Grandmother Alma",
        args!["The ship was also packed with dangerous marine organisms, and they couldn't get inside, even if they wanted to."],
    )?;
    ctx.next()?;
    ctx.lines_as("Grandmother Alma", args!["We couldn't do anything about that ominous looking ship, and just left it as it was. Nowadays, exploration teams try to enter that ship and wipe out its monsters."])?;
    ctx.next()?;
    ctx.lines_as("Grandmother Alma", args!["So it might be a good experience for a young person like yourself to be a recruit. But, it's still not worth risking your life if you're not strong enough."])?;
    ctx.close()
}

pub fn fisk(ctx: &Ctx) -> Script {
    ctx.lines_as("Fisk", args!["Ahoy mate,", "where'd ya", "wanna go?"])?;
    ctx.next()?;
    match runtime::select_values(
        ctx,
        &[
            Val::from("Sunken Ship -> 250 zeny."),
            Val::from("Izlude Marina -> 500 zeny."),
            Val::from("Never mind."),
        ],
    )? {
        1 => {
            if ctx.player().zeny()? < 250 {
                ctx.lines_as("Fisk", args!["Hey now, don't try to cheat me! I said 250 zeny!"])?;
                return ctx.close();
            }
            ctx.player().set_zeny(ctx.player().zeny()? - 250)?;
            ctx.warp("alb2trea", 43, 53)?;
            return ctx.end();
        }
        2 => {
            if ctx.player().zeny()? < 500 {
                ctx.lines_as("Fisk", args!["Ain't no way yer getting there without the 500 zeny first!"])?;
                return ctx.close();
            }
            ctx.player().set_zeny(ctx.player().zeny()? - 500)?;
            ctx.warp("izlude", 176, 182)?;
            return ctx.end();
        }
        3 => {
            ctx.lines_as("Fisk", args!["Alright...", "Landlubber."])?;
            return ctx.close();
        }
        _ => {}
    }
    Ok(())
}

pub fn fisk_a2t(ctx: &Ctx) -> Script {
    ctx.lines_as("Fisk", args!["So you wanna head back to the mainland in Alberta, eh?"])?;
    ctx.next()?;
    if runtime::select_values(ctx, &[Val::from("Yes please."), Val::from("I changed my mind.")])? == 1 {
        ctx.warp("alberta", 192, 169)?;
    }
    ctx.close()
}

pub fn paul(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Paul",
        args![
            "Good day~",
            "Would you like",
            "to join the",
            "exploration team",
            "of the Sunken Ship?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Paul",
        args!["Oh! Before you join, I must warn you. If you're not that strong, you may not want to go."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Paul",
        args!["So, want", "to sign up?", "The admission", "fee is only", "200 Zeny."],
    )?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("Sign me up!"), Val::from("Uh, no thanks.")])? {
        1 => {
            if ctx.player().zeny()? < 200 {
                ctx.lines_as(
                    "Paul",
                    args!["It seems you don't have the money, my friend. But please come back when you're able to pay."],
                )?;
                return ctx.close();
            }
            ctx.player().set_zeny(ctx.player().zeny()? - 200)?;
            ctx.warp("alb2trea", 62, 69)?;
            return ctx.close();
        }
        2 => {
            ctx.lines_as(
                "Paul",
                args!["Alright, well...", "I'll be around", "if you change", "your mind."],
            )?;
            return ctx.close();
        }
        _ => {}
    }
    Ok(())
}

pub fn phelix(ctx: &Ctx) -> Script {
    let mut l_amount = Val::from(0);
    let mut l_max = 0;
    ctx.mes("[Phelix]")?;
    if ctx.var("MaxWeight").get()?.number()? - ctx.var("Weight").get()?.number()? < 10000 {
        ctx.lines(args![
            "Wait a moment!!",
            "You have brought too many things!",
            "You cannot accept any more items!",
            "Please reduce the amount of items,",
            "then come see me again."
        ])?;
        return ctx.close();
    }
    if ctx.var("@event_zelopy").get()? == 0 {
        ctx.lines(args![
            "The hell are you doing here?",
            "There is nothing you can get for free on this ship, if you want somethin', work for it!!"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Phelix",
            args![
                "Hmm, so why don't you bring me 10 Jellopies and I will give 1 potion. How's that sound?",
                "Or if that's too hard for your pansy ass, 3 Jellopies for 1 Carrot."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Phelix",
            args!["If you're interested in my offer, get me the stuff I mentioned."],
        )?;
        ctx.var("@event_zelopy").set(Val::from(1))?;
        return ctx.close();
    }
    ctx.mes("Hmm.. you want to exchange Jellopies for Red Potions or some Carrots eh? Well.. which one?")?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("Red Potions please."), Val::from("Carrots please.")])? {
        1 => {
            ctx.lines_as("Phelix", args!["Alright...", "Let's see", "what'cha got..."])?;
            ctx.next()?;
            ctx.mes("[Phelix]")?;
            if ctx.items().count(909)? < 10 {
                ctx.mes("Hey! Weren't you listening? I said 10 Jellopies for 1 Red Potion.. are ya deaf?")?;
                return ctx.close();
            }
            l_max = ctx.call(Function::CountItem, args![909])?.number()? / 10;
            ctx.lines(args!["Hmm, not bad...", "How many potions", "do you want to get?"])?;
            ctx.next()?;
            match runtime::select_values(
                ctx,
                &[
                    Val::from("As many as I can, please."),
                    Val::from("I want this many."),
                    Val::from("Never mind, I like my jellopy."),
                ],
            )? {
                1 => {
                    ctx.call(Function::DelItem, args![909, l_max * 10])?;
                    ctx.call(Function::GetItem, args![501, l_max])?;
                }
                2 => {
                    ctx.lines_as(
                        "Phelix",
                        args![
                            "I'm not giving you more than 100 at a time so don't bother, OK? If you don't want any, just say '0'.",
                            Val::from("Right now, the most you can get is ")
                                + l_max
                                + Val::from(" but remember, 100 at most, you want to break my back?.")
                        ],
                    )?;
                    let (input, _) = runtime::input_number(ctx, None, None)?;
                    l_amount = input;
                    ctx.next()?;
                    ctx.mes("[Phelix]")?;
                    if l_amount.number()? <= 0 {
                        ctx.mes("Much obliged, come again anytime.")?;
                        return ctx.close();
                    }
                    if l_amount.number()? > 100 {
                        ctx.mes("Hey, what'd I say? 100 at a time at most, you're trying to kill me aren't you!")?;
                        return ctx.close();
                    }
                    if ctx.call(Function::CountItem, args![909])?.number()? < l_amount.number()? * 10 {
                        ctx.mes("Hmm, it looks like you don't have enough. Go get more Jellopies if you want anything else from me.")?;
                        return ctx.close();
                    }
                    ctx.call(Function::DelItem, args![909, l_amount.number()? * 10])?;
                    ctx.call(Function::GetItem, args![501, l_amount.clone()])?;
                }
                3 => {
                    ctx.lines_as("Phelix", args!["No problem,", "see you next time."])?;
                    return ctx.close();
                }
                _ => {}
            }
            ctx.lines_as(
                "Phelix",
                args!["There you go! As I promised. Don't go suckin' them all down at once."],
            )?;
            return ctx.close();
        }
        2 => {
            ctx.lines_as("Phelix", args!["Alright, let's see what ya got..."])?;
            ctx.next()?;
            ctx.mes("[Phelix]")?;
            if ctx.items().count(909)? < 3 {
                ctx.mes("Hmm, look pansy ass, I said 3 Jellopies for 1 Carrot.. got it?")?;
                return ctx.close();
            }
            l_max = ctx.call(Function::CountItem, args![909])?.number()? / 3;
            ctx.lines(args!["Not too bad pansy...", "How many do you want?"])?;
            ctx.next()?;
            match runtime::select_values(
                ctx,
                &[
                    Val::from("As many as I can get, please"),
                    Val::from("I want this many."),
                    Val::from("Never mind, I like my jellopy."),
                ],
            )? {
                1 => {
                    ctx.call(Function::DelItem, args![909, l_max * 3])?;
                    ctx.call(Function::GetItem, args![515, l_max])?;
                }
                2 => {
                    ctx.lines_as(
                        "Phelix",
                        args![
                            "Right I'm not giving you more than 100 at a time so don't bother, okay? If you don't want any, just say '0'."
                        ],
                    )?;
                    let (input, _) = runtime::input_number(ctx, None, None)?;
                    l_amount = input;
                    ctx.next()?;
                    ctx.mes("[Phelix]")?;
                    if l_amount == 0 {
                        ctx.mes("Alright then, see you next time.")?;
                        return ctx.close();
                    }
                    if l_amount.number()? > 100 {
                        ctx.mes(
                            "Hey pansy ass, I said 100 at most, no more than that! I'm not going to break my back for the likes of you!",
                        )?;
                        return ctx.close();
                    }
                    if ctx.call(Function::CountItem, args![909])?.number()? < l_amount.number()? * 3 {
                        ctx.mes("Seems you don't have enough. Go get some more if you want anything else.")?;
                        return ctx.close();
                    }
                    ctx.call(Function::DelItem, args![909, l_amount.number()? * 3])?;
                    ctx.call(Function::GetItem, args![515, l_amount.clone()])?;
                }
                3 => {
                    ctx.lines_as("Phelix", args!["Catch'ya later."])?;
                    return ctx.close();
                }
                _ => {}
            }
            ctx.lines_as("Phelix", args!["There you go~! As I promised. Try not to stuff yer face."])?;
            return ctx.close();
        }
        _ => {}
    }
    Ok(())
}

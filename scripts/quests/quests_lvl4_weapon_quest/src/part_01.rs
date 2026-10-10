use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn bazo_lv4_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_answer = Val::from(0);
    let mut l_dap = Val::from(0);
    let mut l_i = Val::from(0);
    let mut l_itemreq = Val::from(0);
    let mut l_mons: Vec<Val> = Vec::new();
    if ctx.call(Function::CheckWeight, vec![Val::from(908), Val::from(300)])? == 0 {
        ctx.mes("^3355FFWait a second! Right now, you're carrying too many items with you. Please come back after putting some of your things into Kafra Storage.^000000")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("lv4_weapon").get()? == 0 {
        ctx.lines_as("Bazo", args!["Hello...", "Not from around", "here, are you?"])?;
        ctx.next()?;
        ctx.lines_as("Bazo", args!["My name is Bazo Heburiech. I'm pleased to make your acquaintance. As you can see, I'm also not from around here either. I feel as if... We were meant to meet."])?;
        ctx.next()?;
        ctx.lines_as("Bazo", args!["If you don't mind, let me tell you a little about this place. When I first got here, I could tell there was something different about", "this village."])?;
        ctx.next()?;
        ctx.lines_as("Bazo", args!["For one thing, Umbala is close", "to Niflheim, realm of the dead. Because it's so near, some of the people here can actually draw evil power from that realm."])?;
        ctx.next()?;
        ctx.lines_as("Bazo", args!["Don't worry, the locals here only use this power to produce specialty items. During my stay here, I too have learned some of those crafting methods."])?;
        ctx.next()?;
        ctx.lines_as(
            "Bazo",
            args!["If you like, I'd be willing to show you my skills. Now how does that sound?"],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Sounds good~:No, thanks.")])? {
            1 => {
                if ctx.var("BaseLevel").get()?.number()? < 70 {
                    ctx.lines_as("Bazo", args!["Hmm. For now, you should level", "up and get stronger first. Please understand that I mean well when I say that I don't think you're quite ready for my goods."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Bazo",
                        args![
                            "Like I said before, these",
                            "goods have some evil power. If you're not strong or experienced enough, you'd be driven insane."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Bazo", args!["Once you're ready,", "I'll gladly make you", "something great~"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Bazo",
                    args![
                        "Excellent! Let me tell you what materials I'll need. There's not that many, but you might want to write them down."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Bazo",
                    args![
                        "First, I'll need",
                        "some basic materials.",
                        "10 Gold,",
                        "50 Steel and",
                        "10 Emperium."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Bazo",
                    args![
                        "Then, I'll need some rare ore to enchant this product. The product's trait will depend on the ore that you use."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Bazo", args!["Please bring me 30 of either Citrin, Turquoise or Agate. Well then, I wish you luck in finding those things and I'll be waiting for you~"])?;
                ctx.var("lv4_weapon").set(Val::from(1))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Bazo",
                    args![
                        "Oh, okay.",
                        "That's fine with me.",
                        "But I'll be right here if you",
                        "ever change your mind."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    if ctx.var("lv4_weapon").get()?.number()? > 7 {
        ctx.lines_as("Bazo", args!["Hmm...", "I sense something different about you. I can't quite describe it, but I get the distince feeling that there's nothing I can do for you."])?;
        ctx.next()?;
        ctx.lines_as("Bazo", args!["In any case, I wish you safety in your travels, adventurer."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("lv4_weapon").get()?.number()? >= 5 && ctx.var("lv4_weapon").get()?.number()? <= 7) {
        ctx.lines_as(
            "Bazo",
            args![
                "Great...!",
                "Now, let's do this!",
                "Oh, and you should know",
                "now that I can't tell what kind of thing we might get. But let's hope that it'll be good."
            ],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFYou feel a mysterious energy gathering around the materials",
            "and the air around them begins to crackle. It seems as if the materials were absorbing",
            "all of the ambient energy.^000000"
        ])?;
        ctx.next()?;
        ctx.mes("^3355FFWithout any discernable action from Bazo, the materials began to fuse into each other.^000000")?;
        ctx.next()?;
        ctx.lines_as("Bazo", args!["Can you feel that? The materials are now gathering power on their own. It's out of my hands now. All we can do now is wait and pray."])?;
        ctx.next()?;
        ctx.lines(args!["^3355FFThe materials finish melding", "and the air grows calm.^000000"])?;
        ctx.next()?;
        ctx.lines_as(
            "Bazo",
            args![
                "Ooooooh! It's done!",
                "We've succeeded! Ah,",
                "this is amazing! Our prayers",
                "must have been answered!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Bazo",
            args![
                "It looks like all those materials turned into some kind of weapon. That ought to be quite useful to you. Now let's see..."
            ],
        )?;
        ctx.next()?;
        ctx.mes("[Bazo]")?;
        if ctx.var("lv4_weapon").get()? == 5 {
            ctx.lines(args![
                "Immaterial Sword!",
                "This sword was born to be yours!",
                "Congratulations and hopefully it will be useful to you."
            ])?;
            ctx.var("lv4_weapon").set(Val::from(0))?;
            ctx.call(Function::GetItem, vec![Val::from(1141), Val::from(1)])?;
        } else if ctx.var("lv4_weapon").get()? == 6 {
            ctx.lines(args![
                "Slash!",
                "This mace was born to be yours!",
                "Congratulations and hopefully it will be useful to you."
            ])?;
            ctx.var("lv4_weapon").set(Val::from(0))?;
            ctx.call(Function::GetItem, vec![Val::from(1526), Val::from(1)])?;
        } else if ctx.var("lv4_weapon").get()? == 7 {
            ctx.lines(args![
                "Quadrille!",
                "This mace was born to be yours!",
                "Congratulations and hopefully it will be useful to you."
            ])?;
            ctx.var("lv4_weapon").set(Val::from(0))?;
            ctx.call(Function::GetItem, vec![Val::from(1527), Val::from(1)])?;
        }
        ctx.next()?;
        ctx.lines_as("Bazo", args!["Ah, I'm very satisfied with these results. If you wish to have another one, please feel free to visit me anytime. Alright then, enjoy your travels~"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (((ctx.var("lv4_weapon").get()? == 2 && ctx.call(Function::CountItem, vec![Val::from(7295)])?.number()? > 29)
        || (ctx.var("lv4_weapon").get()? == 3 && ctx.call(Function::CountItem, vec![Val::from(7294)])?.number()? > 29))
        || (ctx.var("lv4_weapon").get()? == 4 && ctx.call(Function::CountItem, vec![Val::from(7291)])?.number()? > 29))
    {
        if ctx.var("lv4_weapon").get()? == 2 {
            l_itemreq = Val::from(7295);
        } else if ctx.var("lv4_weapon").get()? == 3 {
            l_itemreq = Val::from(7294);
        } else if ctx.var("lv4_weapon").get()? == 4 {
            l_itemreq = Val::from(7291);
        }
        ctx.lines_as(
            "Bazo",
            args![
                "Okay, it seems like you're",
                "ready so let's get started. Now, my favorite monsters are Poring, Hode, Obeaune and Minorous."
            ],
        )?;
        ctx.next()?;
        l_i = Val::from(1);
        'l2: loop {
            if !(l_i.clone().number()? <= 5) {
                break 'l2;
            }
            'b2: {
                let base = l_i.clone().number()?;
                runtime::local_set(
                    &mut l_mons,
                    &Val::from(base + 0),
                    ctx.call(Function::Rand, vec![Val::from(1), Val::from(4)])?,
                    false,
                );
                ctx.mes("[Bazo]")?;
                if l_i.clone() == 1 {
                    ctx.lines(args![
                        "Here's the first question.",
                        "What monster am I thinking",
                        "of right now?"
                    ])?;
                } else if l_i.clone() == 2 {
                    ctx.lines(args![
                        "Alright...",
                        "Now guess which monster",
                        "I'm thinking about right... Now!"
                    ])?;
                } else if l_i.clone() == 3 {
                    ctx.lines(args!["Now what monster", "am I thinking about?"])?;
                } else if l_i.clone() == 4 {
                    ctx.lines(args!["Which monster am", "I thinking about right now?"])?;
                } else if l_i.clone() == 5 {
                    ctx.lines(args![
                        "Okay, one last time.",
                        "Guess which monster I'm",
                        "thinking about right now."
                    ])?;
                }
                ctx.next()?;
                l_answer = Val::from(runtime::select_values(ctx, &[Val::from("Poring:Hode:Obeaune:Minorous")])?);
                if l_answer.clone().loosely_equals(&runtime::local_get(&l_mons, &l_i.clone(), false)) {
                    l_dap = (l_dap.clone() + Val::from(1));
                }
            }
            l_i = (l_i.clone() + Val::from(1));
        }
        ctx.lines_as("Bazo", args!["Okay, let me give you the answers for the questions in the order I was thinking. Now, it might seem like I'm making them up right now, but I'm not. We gotta trust each other on this."])?;
        ctx.next()?;
        ctx.mes("[Bazo]")?;
        l_i = Val::from(1);
        'l3: loop {
            if !(l_i.clone().number()? <= 5) {
                break 'l3;
            }
            'b3: {
                if runtime::local_get(&l_mons, &l_i.clone(), false) == 1 {
                    ctx.mes("Poring")?;
                } else if runtime::local_get(&l_mons, &l_i.clone(), false) == 2 {
                    ctx.mes("Hode")?;
                } else if runtime::local_get(&l_mons, &l_i.clone(), false) == 3 {
                    ctx.mes("Obeaune")?;
                } else if runtime::local_get(&l_mons, &l_i.clone(), false) == 4 {
                    ctx.mes("Minorous")?;
                }
            }
            l_i = (l_i.clone() + Val::from(1));
        }
        if l_dap.clone().number()? > 0 {
            ctx.call(Function::DelItem, vec![l_itemreq.clone(), Val::from(30)])?;
            ctx.var("lv4_weapon").set((ctx.var("lv4_weapon").get()? + Val::from(3)))?;
        } else if l_dap.clone().number()? < 1 {
            ctx.call(Function::DelItem, vec![l_itemreq.clone(), Val::from(10)])?;
        }
        ctx.next()?;
        if l_dap.clone().number()? > 0 {
            ctx.lines_as("Bazo", args![((Val::from("You got the right answer ") + l_dap.clone()) + Val::from(" times! Incredible! As promised, I shall create a specialty Umbala item for you. Give me a little time to get ready, and then we'll get started."))])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if l_dap.clone().number()? < 1 {
            ctx.lines_as("Bazo", args!["Huh. You must not be able to", "read minds, or you've got really bad luck. Either way, we can't try this until we drive away the ill fortune around you."])?;
            ctx.next()?;
            ctx.lines_as(
                "Bazo",
                args![
                    "Hm, we need at least",
                    ((Val::from("10 ") + ctx.call(Function::GetItemName, vec![l_itemreq.clone()])?) + Val::from(" to try this")),
                    "again, so go ahead",
                    "and give me that."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Bazo",
                args![
                    "Well, if you want to try this mind reading game again, just come",
                    "back with the materials. I'll be waiting to gauge your luck."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if ((ctx.var("lv4_weapon").get()? == 2 || ctx.var("lv4_weapon").get()? == 3) || ctx.var("lv4_weapon").get()? == 4) {
        ctx.lines_as(
            "Bazo",
            args![
                "Hmm. Something doesn't",
                "feel right. We must be missing",
                "something. Would you check the",
                "materials you've brought again?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (((ctx.var("lv4_weapon").get()? == 1 && ctx.call(Function::CountItem, vec![Val::from(969)])?.number()? > 9)
        && ctx.call(Function::CountItem, vec![Val::from(999)])?.number()? > 49)
        && ctx.call(Function::CountItem, vec![Val::from(714)])?.number()? > 9)
    {
        ctx.lines_as(
            "Bazo",
            args![
                "Oh, you've brought all the basic materials. Now, let me check to see if you brought any of the rare ores I asked for..."
            ],
        )?;
        ctx.next()?;
        if ((ctx.call(Function::CountItem, vec![Val::from(7295)])?.number()? > 29
            || ctx.call(Function::CountItem, vec![Val::from(7294)])?.number()? > 29)
            || ctx.call(Function::CountItem, vec![Val::from(7291)])?.number()? > 29)
        {
            if ((ctx.call(Function::CountItem, vec![Val::from(7295)])?.number()? > 29
                && ctx.call(Function::CountItem, vec![Val::from(7294)])?.number()? > 29)
                && ctx.call(Function::CountItem, vec![Val::from(7291)])?.number()? > 29)
            {
                ctx.lines_as(
                    "Bazo",
                    args![
                        "Hahaha, I asked you to",
                        "bring one kind of ore, not all of them. So which one would you",
                        "like to use?"
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Citrin:Turquoise:Agate")])? {
                    1 => {
                        l_itemreq = Val::from(7295);
                    }
                    2 => {
                        l_itemreq = Val::from(7294);
                    }
                    3 => {
                        l_itemreq = Val::from(7291);
                    }
                    _ => {}
                }
            } else if (ctx.call(Function::CountItem, vec![Val::from(7295)])?.number()? > 29
                && ctx.call(Function::CountItem, vec![Val::from(7294)])?.number()? > 29)
            {
                ctx.lines(args![
                    "Hahaha, I asked you to",
                    "bring one kind of ore, not",
                    "two. So which one would",
                    "you like to use?"
                ])?;
                match runtime::select_values(ctx, &[Val::from("Citrin:Turquoise")])? {
                    1 => {
                        l_itemreq = Val::from(7295);
                    }
                    2 => {
                        l_itemreq = Val::from(7294);
                    }
                    _ => {}
                }
            } else if (ctx.call(Function::CountItem, vec![Val::from(7295)])?.number()? > 29
                && ctx.call(Function::CountItem, vec![Val::from(7291)])?.number()? > 29)
            {
                ctx.lines(args![
                    "Hahaha, I asked you to",
                    "bring one kind of ore, not",
                    "two. So which one would",
                    "you like to use?"
                ])?;
                match runtime::select_values(ctx, &[Val::from("Citrin:Agate")])? {
                    1 => {
                        l_itemreq = Val::from(7295);
                    }
                    2 => {
                        l_itemreq = Val::from(7291);
                    }
                    _ => {}
                }
            } else if (ctx.call(Function::CountItem, vec![Val::from(7294)])?.number()? > 29
                && ctx.call(Function::CountItem, vec![Val::from(7291)])?.number()? > 29)
            {
                ctx.lines(args![
                    "Hahaha, I asked you to",
                    "bring one kind of ore, not",
                    "two. So which one would",
                    "you like to use?"
                ])?;
                match runtime::select_values(ctx, &[Val::from("Turquoise:Agate")])? {
                    1 => {
                        l_itemreq = Val::from(7294);
                    }
                    2 => {
                        l_itemreq = Val::from(7291);
                    }
                    _ => {}
                }
            } else {
                if ctx.call(Function::CountItem, vec![Val::from(7295)])?.number()? > 29 {
                    l_itemreq = Val::from(7295);
                }
                if ctx.call(Function::CountItem, vec![Val::from(7294)])?.number()? > 29 {
                    l_itemreq = Val::from(7294);
                }
                if ctx.call(Function::CountItem, vec![Val::from(7291)])?.number()? > 29 {
                    l_itemreq = Val::from(7291);
                }
            }
            ctx.lines_as("Bazo", args![(ctx.call(Function::GetItemName, vec![l_itemreq.clone()])? + Val::from(", huh?")), "Alright then. Before we start, we must first test your luck. As you may have guessed, item crafting in Umbala is a very delicate process."])?;
            ctx.next()?;
            ctx.lines_as("Bazo", args!["But if we do this during a time when your luck is high, we'll have a good chance of succeeding in creating something great. That makes sense, right?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Bazo",
                args![
                    "Now, we'll test your luck by playing a simple mind reading",
                    "game. I'll think of a specific monster, and you'll have to",
                    "guess what it is from among",
                    "my favorites."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Bazo", args!["I'll give you five different chances, and if you can guess right once, we'll go ahead and craft the item. But if you get all of them wrong, we've got to drive away all the bad luck!"])?;
            ctx.next()?;
            ctx.lines_as("Bazo", args!["In order to drive away ill fortune, I'll be taking 10 of the special ore you've brought along. I know it sounds bad, but it's a lot better than losing 30 ores if we failed to create the item, right?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Bazo",
                args!["Okay, now I need", "some time to preprare...", "Talk to you later!"],
            )?;
            ctx.call(Function::DelItem, vec![Val::from(969), Val::from(10)])?;
            ctx.call(Function::DelItem, vec![Val::from(999), Val::from(50)])?;
            ctx.call(Function::DelItem, vec![Val::from(714), Val::from(10)])?;
            if l_itemreq.clone() == 7295 {
                ctx.var("lv4_weapon").set(Val::from(2))?;
            } else if l_itemreq.clone() == 7294 {
                ctx.var("lv4_weapon").set(Val::from(3))?;
            } else if l_itemreq.clone() == 7291 {
                ctx.var("lv4_weapon").set(Val::from(4))?;
            }
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Bazo",
                args![
                    "Please bring me 30 Citrin, Turquoise or Agate. You must",
                    "have forgotten to bring any of those, so please go back and get them so we can begin."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if ctx.var("lv4_weapon").get()? == 1 {
        ctx.lines_as("Bazo", args!["If you'd like me to make you one of Umbala's specialty items, I'll need you to bring me some stuff so I can craft it for you."])?;
        ctx.next()?;
        ctx.lines_as(
            "Bazo",
            args![
                "First, I'll need",
                "some basic materials.",
                "10 Gold,",
                "50 Steel and",
                "10 Emperium."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Bazo",
            args!["Then, I'll need some rare ore to enchant this product. The product's trait will depend on the ore that you use."],
        )?;
        ctx.next()?;
        ctx.lines_as("Bazo", args!["Please bring me 30 of either Citrin, Turquoise or Agate. Well then, I wish you luck in finding those things and I'll be waiting for you~"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as("Bazo", args!["Hello...", "Not from around", "here, are you?"])?;
    ctx.next()?;
    ctx.lines_as("Bazo", args!["My name is Bazo Heburiech. I'm pleased to make your acquaintance. As you can see, I'm also not from around here either. I feel as if... We were meant to meet."])?;
    ctx.next()?;
    ctx.lines_as("Bazo", args!["If you don't mind, let me tell you a little about this place. When I first got here, I could tell there was something different about", "this village."])?;
    ctx.next()?;
    ctx.lines_as(
        "Bazo",
        args![
            "For one thing, Umbala is close",
            "to Niflheim, realm of the dead. Because it's so near, some of the people here can actually draw evil power from that realm."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Bazo", args!["Don't worry, the locals here only use this power to produce specialty items. During my stay here, I too have learned some of those crafting methods."])?;
    ctx.next()?;
    ctx.lines_as(
        "Bazo",
        args!["If you like, I'd be willing to show you my skills. Now how does that sound?"],
    )?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("Sounds good~:No, thanks.")])? {
        1 => {
            if ctx.var("BaseLevel").get()?.number()? < 70 {
                ctx.lines_as("Bazo", args!["Hmm. For now, you should level", "up and get stronger first. Please understand that I mean well when I say that I don't think you're quite ready for my goods."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Bazo",
                    args![
                        "Like I said before, these",
                        "goods have some evil power. If you're not strong or experienced enough, you'd be driven insane."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Bazo", args!["Once you're ready,", "I'll gladly make you", "something great~"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Bazo",
                args!["Excellent! Let me tell you what materials I'll need. There's not that many, but you might want to write them down."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Bazo",
                args![
                    "First, I'll need",
                    "some basic materials.",
                    "10 Gold,",
                    "50 Steel and",
                    "10 Emperium."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Bazo",
                args!["Then, I'll need some rare ore to enchant this product. The product's trait will depend on the ore that you use."],
            )?;
            ctx.next()?;
            ctx.lines_as("Bazo", args!["Please bring me 30 of either Citrin, Turquoise or Agate. Well then, I wish you luck in finding those things and I'll be waiting for you~"])?;
            ctx.var("lv4_weapon").set(Val::from(1))?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        2 => {
            ctx.lines_as(
                "Bazo",
                args![
                    "Oh, okay.",
                    "That's fine with me.",
                    "But I'll be right here if you",
                    "ever change your mind."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn bazo_lv4(ctx: &Ctx) -> Script {
    bazo_lv4_body(ctx, Vec::new()).map(|_| ())
}

fn hibilaithan_lv4_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_answer = Val::from(0);
    let mut l_dap = Val::from(0);
    let mut l_i = Val::from(0);
    let mut l_itemreq = Val::from(0);
    let mut l_mons: Vec<Val> = Vec::new();
    if ctx.call(Function::CheckWeight, vec![Val::from(908), Val::from(300)])? == 0 {
        ctx.mes("^3355FFWait a second! Right now, you're carrying too many items with you. Please come back after putting some of your things into Kafra Storage.^000000")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("event_umbala").get()?.number()? < 3 {
        ctx.lines_as(
            "Hibilaithan",
            args![
                "Umba! Umbaba...umum! Baumba!",
                "Umumumbababaumumbabaumba!",
                "Umbaumbaumbaumbaumhah!",
                "Umumumumumbababababab!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("lv4_weapon").get()? == 0 {
        ctx.lines_as(
            "Hibilaithan",
            args![
                "Eh heh heh...",
                "At long last, this",
                "day has come. Finally,",
                "I've earned recognition as",
                "Umbala's greatest artisan!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Hibilaithan",
            args![
                "Yes yes, I'm ^802A2Athat^000000 Hibilaithan.",
                "And if you bring me the materials, I shall deign to craft one of my masterpieces for you. Bwahahaha~!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Hibilaithan",
            args![
                "Hm? What was that?",
                "Never heard of me?",
                "Please, enough jesting!",
                "I mean, even though I may be a maestro at my craft, there's no need to feel intimidated."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Hibilaithan",
            args![
                "Now be honest...",
                "Do you want to be",
                "the recipient of my",
                "inspired and oft imitated",
                "handiwork or not?"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Yes. Yes, I do.:Sorry, I don't need it!")])? {
            1 => {
                if ctx.var("BaseLevel").get()?.number()? < 70 {
                    ctx.lines_as(
                        "Hibilaithan",
                        args![
                            "Ooh...",
                            "Your mind is too weak to handle the power of my creations. I'm sorry, but you need more training."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Hibilaithan", args!["I know it must be unbearable, waiting to own one of my creations. But for your own sake, get stronger first and then come back to me."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Hibilaithan",
                    args![
                        "Hahahaha!",
                        "Of course you do!",
                        "That was a rhetorical question!",
                        "But I like your attitude. Now I'm going to tell you what you need",
                        "to bring me, so don't forget."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Hibilaithan",
                    args![
                        "Now, I'll need",
                        "10 Gold,",
                        "50 Steel and",
                        "10 Emperium",
                        "for the basic materials."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Hibilaithan", args!["I'll also need some rare ore to endow my creation with power. Muscovite, Biotite or Pyroxene. Just bring me 30 of one of those kinds of ores."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Hibilaithan",
                    args!["Okay, I wish you good luck! In the meantime, I shall relax here and wait for your return."],
                )?;
                ctx.var("lv4_weapon").set(Val::from(8))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Hibilaithan",
                    args![
                        "Hahahahaha~!",
                        "There's no need",
                        "to be modest!",
                        "Oh, you adventurers",
                        "can be so silly sometimes."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Hibilaithan", args!["Well, if you ever muster up the courage to ask me of this favor, come back anytime. Give me the chance to show you my genius!"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    if (ctx.var("lv4_weapon").get()?.number()? < 8 || ctx.var("lv4_weapon").get()?.number()? > 14) {
        ctx.lines_as(
            "Hibilaithan",
            args![
                "...",
                "Ah... I don't feel",
                "like doing ^820A2Aanything^000000",
                "right now. Come back later, alright?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ((ctx.var("lv4_weapon").get()? == 12 || ctx.var("lv4_weapon").get()? == 13) || ctx.var("lv4_weapon").get()? == 14) {
        ctx.lines_as(
            "Hibilaithan",
            args![
                "Ah, I sense that all is in readiness. Good, good,",
                "I can begin! Now, don't peek.",
                "I can't risk sharing my",
                "awesome secrets!"
            ],
        )?;
        ctx.next()?;
        ctx.mes("^3355FFHibilaithan picked up all the materials and turned his back to you. Although you couldn't get a clear look at what he was doing, his motions were deceptively simple and clumsy looking as you feel a strange energy gather around him.^000000")?;
        ctx.next()?;
        ctx.lines_as(
            "Hibilaithan",
            args![
                "Umm~ umm~ umm~",
                "Aww~ aww~ aww~",
                "Phew~ phew~ phew~",
                "Woo~ woo~ woo~",
                "Ho~ ho~ ho~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Hibilaithan",
            args![
                "Haha, it's done!",
                "^333333*Phew*^000000 That was pretty tough, but I'm sure you can't wait to see what we've made. Okay, let's take a look..."
            ],
        )?;
        ctx.next()?;
        ctx.mes("[Hibilaithan]")?;
        if ctx.var("lv4_weapon").get()? == 12 {
            ctx.mes("Ooh, this is a huge success! But of course, since it's my handiwork, that goes without saying. Hey, this is a weapon... Some sort of... Mailbreaker!")?;
            ctx.var("lv4_weapon").set(Val::from(0))?;
            ctx.call(Function::GetItem, vec![Val::from(1225), Val::from(1)])?;
        } else if ctx.var("lv4_weapon").get()? == 13 {
            ctx.mes("Ooh, this is a huge success! But of course, since it's my handiwork, that goes without saying. Hey, this is a weapon... Some sort of... Swordbreaker!")?;
            ctx.var("lv4_weapon").set(Val::from(0))?;
            ctx.call(Function::GetItem, vec![Val::from(1224), Val::from(1)])?;
        } else if ctx.var("lv4_weapon").get()? == 14 {
            ctx.mes("Ooh, this is a huge success! But of course, since it's my handiwork, that goes without saying. Hey, this is a weapon... Some sort of... Slaughter!")?;
            ctx.var("lv4_weapon").set(Val::from(0))?;
            ctx.call(Function::GetItem, vec![Val::from(1367), Val::from(1)])?;
        }
        ctx.next()?;
        ctx.lines_as("Hibilaithan", args!["Ah, yet another creation that's a testament to my awesome skills! Come back to me whenever you want me to make something truly great for you. See you around~"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (((ctx.var("lv4_weapon").get()? == 9 && ctx.call(Function::CountItem, vec![Val::from(7292)])?.number()? > 29)
        || (ctx.var("lv4_weapon").get()? == 10 && ctx.call(Function::CountItem, vec![Val::from(7297)])?.number()? > 29))
        || (ctx.var("lv4_weapon").get()? == 11 && ctx.call(Function::CountItem, vec![Val::from(7296)])?.number()? > 29))
    {
        if ctx.var("lv4_weapon").get()? == 9 {
            l_itemreq = Val::from(7292);
        } else if ctx.var("lv4_weapon").get()? == 10 {
            l_itemreq = Val::from(7297);
        } else if ctx.var("lv4_weapon").get()? == 11 {
            l_itemreq = Val::from(7296);
        }
        ctx.lines_as(
            "Hibilaithan",
            args![
                "Okay, let's get started!",
                "First, we're going to see how",
                "lucky you are today. We'll play this simple game where you guess",
                "which monster I'm thinking of."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Hibilaithan", args!["Don't worry, I'll tell you which monsters are my favorite, so this game isn't impossible. If you can guess just one of them right, we'll know your luck is good today!"])?;
        ctx.next()?;
        l_i = Val::from(1);
        'l2: loop {
            if !(l_i.clone().number()? <= 5) {
                break 'l2;
            }
            'b2: {
                let base = l_i.clone().number()?;
                runtime::local_set(
                    &mut l_mons,
                    &Val::from(base + 0),
                    ctx.call(Function::Rand, vec![Val::from(1), Val::from(4)])?,
                    false,
                );
                ctx.mes("[Hibilaithan]")?;
                if l_i.clone() == 1 {
                    ctx.lines(args![
                        "Alright...",
                        "My favorite monsters are",
                        "Zealotus, Alice, Munak and",
                        "Isis. Now... Guess which one",
                        "I'm thinking about right now!"
                    ])?;
                } else if l_i.clone() == 2 {
                    ctx.lines(args![
                        "Okay...",
                        "Now I'm thinking about a different monster. Or am I? Guess which one!"
                    ])?;
                } else if l_i.clone() == 3 {
                    ctx.lines(args!["Now...", "Which monster am", "I thinking about?"])?;
                } else if l_i.clone() == 4 {
                    ctx.lines(args!["Can you guess which", "monster is on my mind now?"])?;
                } else if l_i.clone() == 5 {
                    ctx.lines(args!["One more time!", "What monster", "am I thinking of?"])?;
                }
                ctx.next()?;
                l_answer = Val::from(runtime::select_values(ctx, &[Val::from("Zealotus:Alice:Munak:Isis")])?);
                if l_answer.clone().loosely_equals(&runtime::local_get(&l_mons, &l_i.clone(), false)) {
                    l_dap = (l_dap.clone() + Val::from(1));
                }
            }
            l_i = (l_i.clone() + Val::from(1));
        }
        ctx.lines_as(
            "Hibilaithan",
            args![
                "Alright, that's it!",
                "Now, this is the order",
                "of the monsters that",
                "I was thinking about..."
            ],
        )?;
        ctx.next()?;
        ctx.mes("[Hibilaithan]")?;
        l_i = Val::from(1);
        'l3: loop {
            if !(l_i.clone().number()? <= 5) {
                break 'l3;
            }
            'b3: {
                if runtime::local_get(&l_mons, &l_i.clone(), false) == 1 {
                    ctx.mes("Zealotus")?;
                } else if runtime::local_get(&l_mons, &l_i.clone(), false) == 2 {
                    ctx.mes("Alice")?;
                } else if runtime::local_get(&l_mons, &l_i.clone(), false) == 3 {
                    ctx.mes("Munak")?;
                } else if runtime::local_get(&l_mons, &l_i.clone(), false) == 4 {
                    ctx.mes("Isis")?;
                }
            }
            l_i = (l_i.clone() + Val::from(1));
        }
        if l_dap.clone().number()? > 0 {
            ctx.call(Function::DelItem, vec![l_itemreq.clone(), Val::from(30)])?;
            ctx.var("lv4_weapon").set((ctx.var("lv4_weapon").get()? + Val::from(3)))?;
        } else if l_dap.clone().number()? < 1 {
            ctx.call(Function::DelItem, vec![l_itemreq.clone(), Val::from(10)])?;
        }
        ctx.next()?;
        ctx.mes("[Hibilaithan]")?;
        if l_dap.clone().number()? > 0 {
            ctx.lines(args![
                "Amazing...",
                "You must have mental powers or something. You answered correctly",
                ((Val::from("") + l_dap.clone()) + Val::from(" times! Wow, you must have really good luck today!"))
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Hibilaithan",
                args![
                    "In that case, there's no reason",
                    "to delay. Give me a little time to prepare, and then we'll get started on making you something, okay?"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if l_dap.clone().number()? < 1 {
            ctx.lines(args![
                "Oooh...",
                "This isn't good.",
                "You got all of them ^660000wrong^000000.",
                ((Val::from("We better drive away all of your bad luck with 10 ")
                    + ctx.call(Function::GetItemName, vec![l_itemreq.clone()])?)
                    + Val::from("."))
            ])?;
            ctx.next()?;
            ctx.lines_as("Hibilaithan", args!["We can't get started until your luck is strong enough, so we'll need to play this game again. If you don't have enough ores, just get some more before coming back."])?;
            ctx.next()?;
            ctx.lines_as(
                "Hibilaithan",
                args!["I'm not going anywhere,", "so there's no rush. Take", "your time, I'll be waiting!"],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if ((ctx.var("lv4_weapon").get()? == 9 || ctx.var("lv4_weapon").get()? == 10) || ctx.var("lv4_weapon").get()? == 11) {
        ctx.lines_as(
            "Hibilaithan",
            args![
                "Hmm. Something doesn't",
                "feel right. We must be missing",
                "something. Would you check the",
                "materials you've brought again?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (((ctx.var("lv4_weapon").get()? == 8 && ctx.call(Function::CountItem, vec![Val::from(969)])?.number()? > 9)
        && ctx.call(Function::CountItem, vec![Val::from(999)])?.number()? > 49)
        && ctx.call(Function::CountItem, vec![Val::from(714)])?.number()? > 9)
    {
        ctx.lines_as("Hibilaithan", args!["Ooh, you're back earlier than I expected. I see that you have all the basic materials, but did you bring the most important thing...?"])?;
        ctx.next()?;
        if ((ctx.call(Function::CountItem, vec![Val::from(7292)])?.number()? > 29
            || ctx.call(Function::CountItem, vec![Val::from(7297)])?.number()? > 29)
            || ctx.call(Function::CountItem, vec![Val::from(7296)])?.number()? > 29)
        {
            if ((ctx.call(Function::CountItem, vec![Val::from(7292)])?.number()? > 29
                && ctx.call(Function::CountItem, vec![Val::from(7297)])?.number()? > 29)
                && ctx.call(Function::CountItem, vec![Val::from(7296)])?.number()? > 29)
            {
                ctx.lines_as("Hibilaithan", args!["Whoa, you brought all three kinds of those ores I asked for? Haha, we can only use one of them, so go ahead and choose."])?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Muscovite:Biotite:Pyroxene")])? {
                    1 => {
                        l_itemreq = Val::from(7292);
                    }
                    2 => {
                        l_itemreq = Val::from(7297);
                    }
                    3 => {
                        l_itemreq = Val::from(7296);
                    }
                    _ => {}
                }
            } else {
                if (ctx.call(Function::CountItem, vec![Val::from(7292)])?.number()? > 29
                    && ctx.call(Function::CountItem, vec![Val::from(7297)])?.number()? > 29)
                {
                    ctx.lines_as(
                        "Hibilaithan",
                        args!["Hahaha, you only needed to bring one kind of ore, not two. Now, which ore would you like to use?"],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Muscovite:Biotite")])? {
                        1 => {
                            l_itemreq = Val::from(7292);
                        }
                        2 => {
                            l_itemreq = Val::from(7297);
                        }
                        _ => {}
                    }
                } else {
                    if (ctx.call(Function::CountItem, vec![Val::from(7292)])?.number()? > 29
                        && ctx.call(Function::CountItem, vec![Val::from(7296)])?.number()? > 29)
                    {
                        ctx.lines_as(
                            "Hibilaithan",
                            args!["Hahaha, you only needed to bring one kind of ore, not two. Now, which ore would you like to use?"],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Muscovite:Pyroxene")])? {
                            1 => {
                                l_itemreq = Val::from(7292);
                            }
                            2 => {
                                l_itemreq = Val::from(7296);
                            }
                            _ => {}
                        }
                    } else if (ctx.call(Function::CountItem, vec![Val::from(7297)])?.number()? > 29
                        && ctx.call(Function::CountItem, vec![Val::from(7296)])?.number()? > 29)
                    {
                        ctx.lines_as(
                            "Hibilaithan",
                            args!["Hahaha, you only needed to bring one kind of ore, not two. Now, which ore would you like to use?"],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Biotite:Pyroxene")])? {
                            1 => {
                                l_itemreq = Val::from(7297);
                            }
                            2 => {
                                l_itemreq = Val::from(7296);
                            }
                            _ => {}
                        }
                    } else if ctx.call(Function::CountItem, vec![Val::from(7292)])?.number()? > 29 {
                        l_itemreq = Val::from(7292);
                    } else if ctx.call(Function::CountItem, vec![Val::from(7297)])?.number()? > 29 {
                        l_itemreq = Val::from(7297);
                    } else if ctx.call(Function::CountItem, vec![Val::from(7296)])?.number()? > 29 {
                        l_itemreq = Val::from(7296);
                    }
                }
            }
            ctx.lines_as(
                "Hibilaithan",
                args![
                    (ctx.call(Function::GetItemName, vec![l_itemreq.clone()])? + Val::from(", eh?")),
                    "Alright, before we start, we need to minimize as much of your negative fortune as we can."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Hibilaithan",
                args!["We're investing a lot of material and energy into this, so it'd be a pity if we fail, right?"],
            )?;
            ctx.next()?;
            ctx.lines_as("Hibilaithan", args!["Now, if I can craft you something on a day on which your fortune is strong, it should be okay. In order to measure your luck, we'll play a mind reading game."])?;
            ctx.next()?;
            ctx.lines_as("Hibilaithan", args!["It's simple. You simply guess the name of the monster I'm thinking about among four choices. If you guess correctly once, then we can get started!"])?;
            ctx.next()?;
            ctx.lines_as("Hibilaithan", args!["I'll be giving you five chances. However, if you get all five wrong, I need to take 10 of the special ore that you brought in order to drive away your misfortune."])?;
            ctx.next()?;
            ctx.lines_as(
                "Hibilaithan",
                args![
                    "Sure, that might sound bad,",
                    "but it's a small price to pay for insuring success in crafting your special item, right?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Hibilaithan",
                args![
                    "Okay, I'll need some time to get ready. Come back to me a little later so that I can judge your",
                    "luck today~"
                ],
            )?;
            ctx.call(Function::DelItem, vec![Val::from(969), Val::from(10)])?;
            ctx.call(Function::DelItem, vec![Val::from(999), Val::from(50)])?;
            ctx.call(Function::DelItem, vec![Val::from(714), Val::from(10)])?;
            if ctx.call(Function::CountItem, vec![Val::from(7292)])?.number()? > 29 {
                ctx.var("lv4_weapon").set(Val::from(9))?;
            } else if ctx.call(Function::CountItem, vec![Val::from(7297)])?.number()? > 29 {
                ctx.var("lv4_weapon").set(Val::from(10))?;
            } else if ctx.call(Function::CountItem, vec![Val::from(7296)])?.number()? > 29 {
                ctx.var("lv4_weapon").set(Val::from(11))?;
            }
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Hibilaithan",
                args![
                    "Hm...?",
                    "It looks like you didn't",
                    "bring any of that special",
                    "ore I asked for."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Hibilaithan",
                args![
                    "Please bring 30 of",
                    "either, Biotite, Muscovite",
                    "or Pyroxene since we need",
                    "one of those kinds of ores",
                    "for me to start crafting."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if ctx.var("lv4_weapon").get()? == 8 {
        ctx.lines_as(
            "Hibilaithan",
            args![
                "Now, I'll need",
                "10 Gold,",
                "50 Steel and",
                "10 Emperium",
                "for the basic materials."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Hibilaithan", args!["I'll also need some rare ore to endow my creation with power. Muscovite, Biotite or Pyroxene. Just bring me 30 of one of those kinds of ores."])?;
        ctx.next()?;
        ctx.lines_as(
            "Hibilaithan",
            args!["Okay, I wish you good luck! In the meantime, I shall relax here and wait for your return."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Hibilaithan",
        args![
            "Eh heh heh...",
            "At long last, this",
            "day has come. Finally,",
            "I've earned recognition as",
            "Umbala's greatest artisan!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Hibilaithan",
        args![
            "Yes yes, I'm ^802A2Athat^000000 Hibilaithan.",
            "And if you bring me the materials, I shall deign to craft one of my masterpieces for you. Bwahahaha~!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Hibilaithan",
        args![
            "Hm? What was that?",
            "Never heard of me?",
            "Please, enough jesting!",
            "I mean, even though I may be a maestro at my craft, there's no need to feel intimidated."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Hibilaithan",
        args![
            "Now be honest...",
            "Do you want to be",
            "the recipient of my",
            "inspired and oft imitated",
            "handiwork or not?"
        ],
    )?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("Yes. Yes, I do.:Sorry, I don't need it!")])? {
        1 => {
            if ctx.var("BaseLevel").get()?.number()? < 70 {
                ctx.lines_as(
                    "Hibilaithan",
                    args![
                        "Ooh...",
                        "Your mind is too weak to handle the power of my creations. I'm sorry, but you need more training."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Hibilaithan", args!["I know it must be unbearable, waiting to own one of my creations. But for your own sake, get stronger first and then come back to me."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Hibilaithan",
                args![
                    "Hahahaha!",
                    "Of course you do!",
                    "That was a rhetorical question!",
                    "But I like your attitude. Now I'm going to tell you what you need",
                    "to bring me, so don't forget."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Hibilaithan",
                args![
                    "Now, I'll need",
                    "10 Gold,",
                    "50 Steel and",
                    "10 Emperium",
                    "for the basic materials."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Hibilaithan", args!["I'll also need some rare ore to endow my creation with power. Muscovite, Biotite or Pyroxene. Just bring me 30 of one of those kinds of ores."])?;
            ctx.next()?;
            ctx.lines_as(
                "Hibilaithan",
                args!["Okay, I wish you good luck! In the meantime, I shall relax here and wait for your return."],
            )?;
            ctx.var("lv4_weapon").set(Val::from(8))?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        2 => {
            ctx.lines_as(
                "Hibilaithan",
                args![
                    "Hahahahaha~!",
                    "There's no need",
                    "to be modest!",
                    "Oh, you adventurers",
                    "can be so silly sometimes."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Hibilaithan", args!["Well, if you ever muster up the courage to ask me of this favor, come back anytime. Give me the chance to show you my genius!"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn hibilaithan_lv4(ctx: &Ctx) -> Script {
    hibilaithan_lv4_body(ctx, Vec::new()).map(|_| ())
}

fn tabezthan_lv4_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_answer = Val::from(0);
    let mut l_dap = Val::from(0);
    let mut l_i = Val::from(0);
    let mut l_itemreq = Val::from(0);
    let mut l_mons: Vec<Val> = Vec::new();
    if ctx.call(Function::CheckWeight, vec![Val::from(908), Val::from(300)])? == 0 {
        ctx.mes("^3355FFWait a second! Right now, you're carrying too many items with you. Please come back after putting some of your things into Kafra Storage.^000000")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("event_umbala").get()?.number()? < 3 {
        ctx.lines_as(
            "Tabezthan",
            args![
                "Umba! Umbaba...umum! Baumba!",
                "Umumumbababaumumbabaumba!",
                "Umbaumbaumbaumbaumhah!",
                "Umumumumumbababababab!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("lv4_weapon").get()? == 0 {
        ctx.lines_as(
            "Tabezthan",
            args![
                "Hmm...",
                "I feel something different from you. You are a stranger around here, are you not?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Tabezthan",
            args![
                "Allow me to introduce myself. I am Tabezthan, a veritable fountain of knowledge and a renown genius in Umbala. Ha ha ha!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Tabezthan",
            args!["I have two disciples: Hibilaithan, who is a fool, and Bazo, who is quite intelligent and shows potential."],
        )?;
        ctx.next()?;
        ctx.lines_as("Tabezthan", args!["Of course, both are very talented and skilled, but I worry about Hibilaithan. He makes many stupid mistakes but he is fairly shameless about his errors."])?;
        ctx.next()?;
        ctx.lines_as("Tabezthan", args!["I have taught them how to manipulate the ambient energy", "here in Umbala in order to create objects. I hear that there is a similar skill in the outside world known to you as alchemy."])?;
        ctx.next()?;
        ctx.lines_as(
            "Tabezthan",
            args![
                "In any case, do have any use",
                "for my skills? If so, please bring me the materials that I need so that I may be of service to you."
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Yes, please.:No, thank you.")])? {
            1 => {
                if ctx.var("BaseLevel").get()?.number()? < 70 {
                    ctx.lines_as("Tabezthan", args!["I regret to say that you are not yet capable of handling my crafts. Please train and acquire greater strength before returning to me. I wish you safety in your travels."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Tabezthan",
                    args![
                        "Good. I ask that you",
                        "remember what you need",
                        "to bring me so that I may craft something for you. I shall need..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Tabezthan",
                    args![
                        "10 Gold,",
                        "50 Steel",
                        "and 10 Emperium",
                        "for the basic materials.",
                        "I'll also need a rare ore to enchant my creation."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Tabezthan", args!["In addition, please bring me 30 of either Phlogopite, Peridot or Rose Quartz. Depending on the ore that you choose, my creation will possess a different trait."])?;
                ctx.next()?;
                ctx.lines_as("Tabezthan", args!["Please be aware that I do not know exactly what item will be produced. There are too many factors in alchemy and there is a limit to anyone's knowledge."])?;
                ctx.next()?;
                ctx.lines_as("Tabezthan", args!["In other words, we will have to trust to luck. However, we'll have time to talk about that later. For now, please go and gather the necessary materials. I shall be waiting here."])?;
                ctx.var("lv4_weapon").set(Val::from(15))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Tabezthan",
                    args![
                        "Oh... I see.",
                        "It is a little disappointing to hear that, but please return",
                        "if you believe that I can be",
                        "of service to you."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    if (ctx.var("lv4_weapon").get()?.number()? < 15 || ctx.var("lv4_weapon").get()?.number()? > 21) {
        ctx.lines_as("Tabezthan", args!["Hm...?", "You don't have any", "business with me, do you?"])?;
        ctx.next()?;
        ctx.lines_as("Tabezthan", args!["Please do not disregard me as merely an old fool. When a man gets older, he can see more clearly into the hearts of others. Please continue what you have been doing, and I wish you good fortune on your journeys."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ((ctx.var("lv4_weapon").get()? == 19 || ctx.var("lv4_weapon").get()? == 20) || ctx.var("lv4_weapon").get()? == 21) {
        ctx.lines_as(
            "Tabezthan",
            args!["Great, I'm ready to begin. Give me a minute to arrange these materials into a mystic circle."],
        )?;
        ctx.next()?;
        ctx.mes("^3355FFHe arranged the materials in a strange circle and began chanting. The air around Tabezthan grew ominously heavy and the materials seemed to surge with newfound energy.^000000")?;
        ctx.next()?;
        ctx.lines_as("Tabezthan", args!["Please understand that this power is not coming from me. I am merely using my skills to gather power from the realm of the dead and infusing them into these materials."])?;
        ctx.next()?;
        ctx.lines_as(
            "Tabezthan",
            args![
                "Let us pray for good results.",
                "All we can do now is leave the",
                "final result to fate."
            ],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFThe materials slowly melded",
            "into each other, combining into a completely new object. It is only",
            "a matter of time before we learn whether you and Tabezthan have succeeded or failed.^000000"
        ])?;
        ctx.next()?;
        ctx.mes("[Tabezthan]")?;
        if ctx.var("lv4_weapon").get()? == 19 {
            ctx.lines(args!["I don't believe it!", "This is such an incredibly rare weapon! Yes, I remember its name from what my late father told me. This is... Caesar's Sword!"])?;
            ctx.var("lv4_weapon").set(Val::from(0))?;
            ctx.call(Function::GetItem, vec![Val::from(1134), Val::from(1)])?;
        } else if ctx.var("lv4_weapon").get()? == 20 {
            ctx.lines(args![
                "I don't believe it!",
                "This is such an incredibly rare weapon! Yes, I remember its name from what my late father told me. This is... Tirfing!"
            ])?;
            ctx.var("lv4_weapon").set(Val::from(0))?;
            ctx.call(Function::GetItem, vec![Val::from(1139), Val::from(1)])?;
        } else if ctx.var("lv4_weapon").get()? == 21 {
            ctx.lines(args![
                "I don't believe it!",
                "This is such an incredibly rare weapon! Yes, I remember its name from what my late father told me. This is... Sabbath!"
            ])?;
            ctx.var("lv4_weapon").set(Val::from(0))?;
            ctx.call(Function::GetItem, vec![Val::from(1365), Val::from(1)])?;
        }
        ctx.next()?;
        ctx.lines_as(
            "Tabezthan",
            args![
                "This mighty weapon is yours to keep. Feel free to come back to me",
                "if you think that I can be of service to you once again.",
                "Farewell for now, brave adventurer."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (((ctx.var("lv4_weapon").get()? == 16 && ctx.call(Function::CountItem, vec![Val::from(7290)])?.is_true())
        || (ctx.var("lv4_weapon").get()? == 17 && ctx.call(Function::CountItem, vec![Val::from(7289)])?.is_true()))
        || (ctx.var("lv4_weapon").get()? == 18 && ctx.call(Function::CountItem, vec![Val::from(7293)])?.is_true()))
    {
        if ctx.var("lv4_weapon").get()? == 16 {
            l_itemreq = Val::from(7290);
        } else if ctx.var("lv4_weapon").get()? == 17 {
            l_itemreq = Val::from(7289);
        } else if ctx.var("lv4_weapon").get()? == 18 {
            l_itemreq = Val::from(7293);
        }
        ctx.lines_as("Tabezthan", args!["Shall we begin?", "I will tell you the names of 4 different monsters. You will be given five chances to predict which monster I am thinking about."])?;
        ctx.next()?;
        ctx.lines_as(
            "Tabezthan",
            args!["This mind reading game is an effective method of determining whether your fortune is good or bad. Get ready now..."],
        )?;
        ctx.next()?;
        l_i = Val::from(1);
        'l2: loop {
            if !(l_i.clone().number()? <= 5) {
                break 'l2;
            }
            'b2: {
                let base = l_i.clone().number()?;
                runtime::local_set(
                    &mut l_mons,
                    &Val::from(base + 0),
                    ctx.call(Function::Rand, vec![Val::from(1), Val::from(4)])?,
                    false,
                );
                ctx.mes("[Tabezthan]")?;
                if l_i.clone() == 1 {
                    ctx.lines(args![
                        "My favorite monsters are",
                        "Baphomet, Dark Lord, Bloody",
                        "Knight and Stormy Knight. Now,",
                        "try to guess which one I am",
                        "thinking about."
                    ])?;
                } else if l_i.clone() == 2 {
                    ctx.lines(args![
                        "Please try to determine",
                        "which monster I'm thinking",
                        "of right this moment."
                    ])?;
                } else if l_i.clone() == 3 {
                    ctx.lines(args!["Now, try to sense", "which monster I am", "visualizing in my mind."])?;
                } else if l_i.clone() == 4 {
                    ctx.lines(args![
                        "Once again, try",
                        "and guess what monster",
                        "I'm thinking about right now."
                    ])?;
                } else if l_i.clone() == 5 {
                    ctx.lines(args![
                        "Alright, this is",
                        "your last chance to",
                        "correctly guess which",
                        "monster I'm thinking of."
                    ])?;
                }
                ctx.next()?;
                l_answer = Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("Baphomet:Dark Lord:Bloody Knight:Stormy Knight")],
                )?);
                if l_answer.clone().loosely_equals(&runtime::local_get(&l_mons, &l_i.clone(), false)) {
                    l_dap = (l_dap.clone() + Val::from(1));
                }
            }
            l_i = (l_i.clone() + Val::from(1));
        }
        ctx.lines_as(
            "Tabezthan",
            args!["That's the end of the game. Now, this is the order of the monsters that I had in mind..."],
        )?;
        ctx.next()?;
        ctx.mes("[Tabezthan]")?;
        l_i = Val::from(1);
        'l3: loop {
            if !(l_i.clone().number()? <= 5) {
                break 'l3;
            }
            'b3: {
                if runtime::local_get(&l_mons, &l_i.clone(), false) == 1 {
                    ctx.mes("Baphomet")?;
                } else if runtime::local_get(&l_mons, &l_i.clone(), false) == 2 {
                    ctx.mes("Dark Lord")?;
                } else if runtime::local_get(&l_mons, &l_i.clone(), false) == 3 {
                    ctx.mes("Bloody Knight")?;
                } else if runtime::local_get(&l_mons, &l_i.clone(), false) == 4 {
                    ctx.mes("Stormy Knight")?;
                }
            }
            l_i = (l_i.clone() + Val::from(1));
        }
        if l_dap.clone().number()? > 0 {
            ctx.call(Function::DelItem, vec![l_itemreq.clone(), Val::from(30)])?;
            ctx.var("lv4_weapon").set((ctx.var("lv4_weapon").get()? + Val::from(3)))?;
        } else if l_dap.clone().number()? < 1 {
            ctx.call(Function::DelItem, vec![l_itemreq.clone(), Val::from(10)])?;
        }
        ctx.next()?;
        ctx.mes("[Tabezthan]")?;
        if l_dap.clone().number()? > 0 {
            ctx.lines(args![((Val::from("You have answered ") + l_dap.clone()) + Val::from(" times correctly. It appears that your luck is at a high point, and it is an ideal time for me to craft something for you. Give me a little time to prepare, and return to me."))])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if l_dap.clone().number()? < 1 {
            ctx.mes("Unfortunately, you weren't able to guess any of them correctly. It seems that your luck is charged with negative energy. We'll need to expel this bad luck with 10 of your Phologopite.")?;
            ctx.next()?;
            ctx.lines_as(
                "Tabezthan",
                args![
                    ((Val::from("Do not be disheartened. Losing 10 ") + ctx.call(Function::GetItemName, vec![l_itemreq.clone()])?)
                        + Val::from(" is much better than losing 30 on a failed crafting attempt."))
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Tabezthan", args!["Please come back with 30 Phologopite, and we shall try this mind reading game once again. I shall be waiting for you right here."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if ((ctx.var("lv4_weapon").get()? == 16 || ctx.var("lv4_weapon").get()? == 17) || ctx.var("lv4_weapon").get()? == 18) {
        ctx.lines_as(
            "Tabezthan",
            args![
                "Umm...",
                "You do not seem to",
                "have enough Rose Quartz.",
                "Please try to gather at",
                "least 30 of them and then",
                "return to me."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (((ctx.var("lv4_weapon").get()? == 15 && ctx.call(Function::CountItem, vec![Val::from(969)])?.number()? > 9)
        && ctx.call(Function::CountItem, vec![Val::from(999)])?.number()? > 49)
        && ctx.call(Function::CountItem, vec![Val::from(714)])?.number()? > 9)
    {
        ctx.lines_as("Tabezthan", args!["You've returned to me earlier than I've expected. I sense that you've brought all the basic materials, but did you bring enough special ore as well?"])?;
        ctx.next()?;
        if ((ctx.call(Function::CountItem, vec![Val::from(7290)])?.number()? > 29
            || ctx.call(Function::CountItem, vec![Val::from(7289)])?.number()? > 29)
            || ctx.call(Function::CountItem, vec![Val::from(7293)])?.number()? > 29)
        {
            if ((ctx.call(Function::CountItem, vec![Val::from(7290)])?.number()? > 29
                && ctx.call(Function::CountItem, vec![Val::from(7289)])?.number()? > 29)
                && ctx.call(Function::CountItem, vec![Val::from(7293)])?.number()? > 29)
            {
                ctx.lines_as(
                    "Tabezthan",
                    args!["Ah, you've brought all three. However, we can only use one kind of ore at a time, so please choose just one."],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Phlogopite:Peridot:Rose Quartz")])? {
                    1 => {
                        l_itemreq = Val::from(7290);
                    }
                    2 => {
                        l_itemreq = Val::from(7289);
                    }
                    3 => {
                        l_itemreq = Val::from(7293);
                    }
                    _ => {}
                }
            } else {
                if (ctx.call(Function::CountItem, vec![Val::from(7290)])?.number()? > 29
                    && ctx.call(Function::CountItem, vec![Val::from(7289)])?.number()? > 29)
                {
                    ctx.lines_as(
                        "Tabezthan",
                        args![
                            "Hahaha, you didn't need",
                            "to bring more than one kind",
                            "of ore. Now, which one would",
                            "you like to use?"
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Phlogopite:Peridot")])? {
                        1 => {
                            l_itemreq = Val::from(7290);
                        }
                        2 => {
                            l_itemreq = Val::from(7289);
                        }
                        _ => {}
                    }
                } else {
                    if (ctx.call(Function::CountItem, vec![Val::from(7290)])?.number()? > 29
                        && ctx.call(Function::CountItem, vec![Val::from(7293)])?.number()? > 29)
                    {
                        ctx.lines_as(
                            "Tabezthan",
                            args![
                                "Hahaha, you didn't need",
                                "to bring more than one kind",
                                "of ore. Now, which one would",
                                "you like to use?"
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Phlogopite:Rose Quartz")])? {
                            1 => {
                                l_itemreq = Val::from(7290);
                            }
                            2 => {
                                l_itemreq = Val::from(7293);
                            }
                            _ => {}
                        }
                    } else if (ctx.call(Function::CountItem, vec![Val::from(7289)])?.number()? > 29
                        && ctx.call(Function::CountItem, vec![Val::from(7293)])?.number()? > 29)
                    {
                        ctx.lines_as(
                            "Tabezthan",
                            args![
                                "Hahaha, you didn't need",
                                "to bring more than one kind",
                                "of ore. Now, which one would",
                                "you like to use?"
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Peridot:Rose Quartz")])? {
                            1 => {
                                l_itemreq = Val::from(7289);
                            }
                            2 => {
                                l_itemreq = Val::from(7293);
                            }
                            _ => {}
                        }
                    } else if ctx.call(Function::CountItem, vec![Val::from(7290)])?.number()? > 29 {
                        l_itemreq = Val::from(7290);
                    } else if ctx.call(Function::CountItem, vec![Val::from(7289)])?.number()? > 29 {
                        l_itemreq = Val::from(7289);
                    } else if ctx.call(Function::CountItem, vec![Val::from(7293)])?.number()? > 29 {
                        l_itemreq = Val::from(7293);
                    }
                }
            }
            ctx.lines_as(
                "Tabezthan",
                args![
                    ((Val::from("Good, ") + ctx.call(Function::GetItemName, vec![l_itemreq.clone()])?) + Val::from(".")),
                    "Now, before I can begin crafting,",
                    "I must first determine whether or not you have enough luck for me",
                    "to proceed."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Tabezthan",
                args![
                    "We will play a mind reading game",
                    "so that I can measure your luck and see if it is high enough to avoid the possibility of crafting failure."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Tabezthan", args!["You will have 5 chances to guess which one of my four favorite monsters that I am thinking of. If you can answer correctly even just once, I can begin crafting."])?;
            ctx.next()?;
            ctx.lines_as("Tabezthan", args!["However, if you answer incorrectly all five times, I must take 10 of your special ore in order to expel your misfortune."])?;
            ctx.next()?;
            ctx.lines_as(
                "Tabezthan",
                args!["Now, please give me a little time to finish my preparations. I shall speak to you later."],
            )?;
            ctx.call(Function::DelItem, vec![Val::from(999), Val::from(50)])?;
            ctx.call(Function::DelItem, vec![Val::from(969), Val::from(10)])?;
            ctx.call(Function::DelItem, vec![Val::from(714), Val::from(10)])?;
            if ctx.call(Function::CountItem, vec![Val::from(7290)])?.number()? > 29 {
                ctx.var("lv4_weapon").set(Val::from(16))?;
            } else if ctx.call(Function::CountItem, vec![Val::from(7289)])?.number()? > 29 {
                ctx.var("lv4_weapon").set(Val::from(17))?;
            } else if ctx.call(Function::CountItem, vec![Val::from(7293)])?.number()? > 29 {
                ctx.var("lv4_weapon").set(Val::from(18))?;
            }
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Tabezthan",
                args![
                    "Hmmm...",
                    "It doesn't look like it. Remember, I need 30 of either Phlogopite, Peridot or Rose Quartz before I can begin crafting."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if ctx.var("lv4_weapon").get()? == 15 {
        ctx.lines_as(
            "Tabezthan",
            args![
                "[Tabezthan]",
                "Bring me...",
                "10 Gold,",
                "50 Steel",
                "and 10 Emperium.",
                "I'll also need a rare ore to enchant my creation."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Tabezthan", args!["In addition, please bring me 30 of either Phlogopite, Peridot or Rose Quartz. Depending on the ore that you choose, my creation will possess a different trait."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Tabezthan",
        args![
            "Hmm...",
            "I feel something different from you. You are a stranger around here, are you not?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Tabezthan",
        args!["Allow me to introduce myself. I am Tabezthan, a veritable fountain of knowledge and a renown genius in Umbala. Ha ha ha!"],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Tabezthan",
        args!["I have two disciples: Hibilaithan, who is a fool, and Bazo, who is quite intelligent and shows potential."],
    )?;
    ctx.next()?;
    ctx.lines_as("Tabezthan", args!["Of course, both are very talented and skilled, but I worry about Hibilaithan. He makes many stupid mistakes but he is fairly shameless about his errors."])?;
    ctx.next()?;
    ctx.lines_as(
        "Tabezthan",
        args![
            "I have taught them how to manipulate the ambient energy",
            "here in Umbala in order to create objects. I hear that there is a similar skill in the outside world known to you as alchemy."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Tabezthan",
        args![
            "In any case, do have any use",
            "for my skills? If so, please bring me the materials that I need so that I may be of service to you."
        ],
    )?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("Yes, please.:No, thank you.")])? {
        1 => {
            if ctx.var("BaseLevel").get()?.number()? < 70 {
                ctx.lines_as("Tabezthan", args!["I regret to say that you are not yet capable of handling my crafts. Please train and acquire greater strength before returning to me. I wish you safety in your travels."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Tabezthan",
                args![
                    "Good. I ask that you",
                    "remember what you need",
                    "to bring me so that I may craft something for you. I shall need..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Tabezthan",
                args![
                    "10 Gold,",
                    "50 Steel",
                    "and 10 Emperium",
                    "for the basic materials.",
                    "I'll also need a rare ore to enchant my creation."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Tabezthan", args!["In addition, please bring me 30 of either Phlogopite, Peridot or Rose Quartz. Depending on the ore that you choose, my creation will possess a different trait."])?;
            ctx.next()?;
            ctx.lines_as("Tabezthan", args!["Please be aware that I do not know exactly what item will be produced. There are too many factors in alchemy and there is a limit to anyone's knowledge."])?;
            ctx.next()?;
            ctx.lines_as("Tabezthan", args!["In other words, we will have to trust to luck. However, we'll have time to talk about that later. For now, please go and gather the necessary materials. I shall be waiting here."])?;
            ctx.var("lv4_weapon").set(Val::from(15))?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        2 => {
            ctx.lines_as(
                "Tabezthan",
                args![
                    "Oh... I see.",
                    "It is a little disappointing to hear that, but please return",
                    "if you believe that I can be",
                    "of service to you."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn tabezthan_lv4(ctx: &Ctx) -> Script {
    tabezthan_lv4_body(ctx, Vec::new()).map(|_| ())
}

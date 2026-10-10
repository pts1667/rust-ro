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

pub fn martine_cmd(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Martine",
        args![
            "Gambling...? The games",
            "provided here in the Comodo",
            "Casino are a higher form of",
            "entertainment than gambling.",
            "Do you know what I mean?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Martine",
        args![
            "Granted, I did lose",
            "all of my zeny playing",
            "in this Casino, but I have",
            "no regrets. I'll simply earn",
            "more money, then blow it all",
            "again. Or I just might win big!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Martine",
        args![
            "Bwahahahaahah~!",
            "Yes, I can only lose so",
            "many times until I hit the",
            "jackpot! You see, you see?",
            "I'm playing the freakin' odds."
        ],
    )?;
    ctx.close()
}

pub fn scoursege_cmd(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Scoursege",
        args![
            "Damn it! Where did that",
            "guy go? He promised me that",
            "he'd easily double my money!",
            "Wait. Oh, wait. Oh... Oh no..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Scoursege",
        args![
            "Don't tell me that I just got",
            "conned out of my money!",
            "Oh no! Still, I better report",
            "this to the proper authorities,",
            "no matter how ashamed I feel..."
        ],
    )?;
    ctx.close()
}

pub fn roberto_cmd(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Roberto",
        args![
            "Heh heh heh...",
            "Whaaaat a gullible",
            "guy. I took his money",
            "so easily! I mean, I didn't",
            "even come up with that great",
            "of a lie, and he gave it to me!"
        ],
    )?;
    ctx.close()
}

pub fn deniroz_cmd(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Deniroz",
        args![
            "All I need is for this",
            "little steel bead to fall",
            "into the right hole. Then,",
            "I'll win the jackpot. Alright.",
            "Here goes. One last time..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Deniroz",
        args![
            "No! No, I was so close!",
            "Alright, next time I should",
            "be even closer, right? Yeah.",
            "Okay, this time will be the",
            "last time. Not again! Alright,",
            "j-just one more t-time..."
        ],
    )?;
    ctx.close()
}

pub fn shalone_cmd(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Shalone",
        args![
            "Oh, I'm sorry, sir,",
            "but it looks like you",
            "lost again. Maybe you",
            "should quit for now...",
            "You've been having quite",
            "a run of really bad luck..."
        ],
    )?;
    ctx.close()
}

pub fn stonae_cmd(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Stonae",
        args![
            "N-no...",
            "I lost again?!",
            "But I can't quit like",
            "this! I'm gonna keep",
            "going, and I'm gonna",
            "leave this place a winner!"
        ],
    )?;
    ctx.close()
}

pub fn g_j_cmd(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "G . J",
        args![
            "The more I think about it,",
            "it seems easier to become",
            "rich by working, saving, and",
            "making wise investments than",
            "to, you know... Rely on some",
            "kind of huge jackpot prize."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "G . J",
        args![
            "Gambling seems fun, but",
            "it seems smarter to make",
            "money in other ways. Sure,",
            "working hard is no fun, but",
            "there are ways to use your money to make more of it, right?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "G . J",
        args![
            "There's also the matter of",
            "being smart and responsible",
            "about your money--I mean, you're more likely to blow all your cash",
            "if you win it, right? Yeah, you",
            "gotta be wise about it all..."
        ],
    )?;
    ctx.close()
}

pub fn loyar_cmd(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Loyar",
        args![
            "Comodo Casino's interior",
            "design is so pleasing to the",
            "eyes, so clean and simple.",
            "The atmosphere here is perfect,",
            "and it makes me want to play ",
            "some more. Alright, let's go!"
        ],
    )?;
    ctx.next()?;
    let roll = ctx.call(Function::Rand, args![1, 3])?;
    if roll == 1 {
        ctx.lines_as(
            "Loyar",
            args![
                "Hmm... Maybe I better",
                "go home soon. I didn't",
                "spend all the money that",
                "I set aside for gambling",
                "quite yet, but it's not a good",
                "idea to stay out too long."
            ],
        )?;
        return ctx.close();
    }
    if roll == 2 {
        ctx.lines_as(
            "Loyar",
            args![
                "I have to admit, the",
                "atmosphere of this place",
                "is exciting and addictive.",
                "Even when you're tired, the",
                "energy of this place just",
                "gets into you, you know?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Loyar",
            args![
                "Although this kind of place",
                "may encourage people with",
                "serious gambling problems,",
                "it's much nicer to gamble",
                "here than in a place that's",
                "dirtier and more questionable."
            ],
        )?;
        return ctx.close();
    }
    if roll == 3 {
        ctx.lines_as(
            "Loyar",
            args![
                "Whoa whoa whoa...",
                "Why did that guy make",
                "that bet? What an amateur...",
                "Er, I guess you don't know",
                "too much about this game.",
                "As for me, I'm just a fan~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Loyar",
            args![
                "I'm a big fan of a lot",
                "of these games, but I'll",
                "admit that I'm an even bigger",
                "fan of winning! Still, I have",
                "enough sense to stay out of",
                "those high stakes games."
            ],
        )?;
        return ctx.close();
    }
    Ok(())
}

pub fn moo_cmd(ctx: &Ctx) -> Script {
    ctx.mes("[Moo]")?;
    if ctx.call(Function::Rand, args![1, 10])? == 1 {
        ctx.lines(args![
            "Those cheating punks!",
            "They'll never show their",
            "faces here again: otherwise",
            "they're gonna hafta get new",
            "ones! Oh--Sorry, I didn't",
            "see you there~ Hahahaha~"
        ])?;
        return ctx.close();
    }
    ctx.lines(args![
        "Greetings, I am Moo,",
        "manager of the Comodo",
        "Casino. We pride ourselves in",
        "serving all of our customers'",
        "needs, doing all we can so that your visit here is unforgettable."
    ])?;
    ctx.next()?;
    ctx.lines_as(
        "Moo",
        args![
            "All of our guests can enjoy",
            "our general gaming area, and",
            "we also provide a VIP area",
            "where high rollers can play",
            "exciting high stakes games."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Moo",
        args![
            "We always welcome all of",
            "your suggestions, and are",
            "always seeking to improve",
            "your experience here in",
            "the Comodo Casino."
        ],
    )?;
    ctx.close()
}

pub fn zyosegirl_cmd(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Zyosegirl",
        args![
            "People call me the",
            "Sea Lady because I'm",
            "always here working,",
            "gathering clams and other",
            "sea creatures to sell. It's",
            "a pretty good living, actually."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Zyosegirl",
        args![
            "It's nice to be able to work",
            "outdoors, but someday, I want",
            "to save enough money and move",
            "to the city. I'm still young, you know, and I've got dreams",
            "that I want to fulfill~"
        ],
    )?;
    ctx.close()
}

pub fn ziyaol_cmd(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Ziyaol",
        args![
            "Ahhh, it's nice being",
            "a fisherman. You just",
            "relax and let the fish",
            "come to you. Well, it takes",
            "some skill to catch as much",
            "fish as I do with no effort~"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Ziyaol",
        args![
            "I like the leisure involved",
            "in my job, but if it's not one",
            "thing, it's another. Yeah, that",
            "daughter of mine over there",
            "won't stop harping about ",
            "moving to the biiig city."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Ziyaol",
        args![
            "Why does she want to leave",
            "me so badly?! But if I don't",
            "let her go, she'll run away.",
            "What am I going to do with",
            "that girl? Well, I can't really",
            "stop her from dreaming..."
        ],
    )?;
    ctx.close()
}

pub fn daeguro_cmd(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Daeguro",
        args![
            "I love playing in",
            "the sand-- it's so soft",
            "and clean and pretty!",
            "But when I grow up,",
            "I wanna go to Alberta",
            "and see everything I can!"
        ],
    )?;
    ctx.close()
}

pub fn rahasu_cmd(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Rahasu",
        args![
            "Hey, I'm Rahasu.",
            "If you want to learn",
            "a little more about",
            "Paros Lighthouse, I'll",
            "be happy to tell you."
        ],
    )?;
    ctx.next()?;
    if ctx.menu(&["Paros Lighthouse?", "Cancel"])? == 0 {
        ctx.lines_as(
            "Rahasu",
            args![
                "For many years, this",
                "lighthouse guided many",
                "ships to shore. That was",
                "a long time ago: now this",
                "lighthouse sits quietly,",
                "unused, but never unloved."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Rahasu",
            args![
                "Although this place",
                "isn't the center of",
                "trade and commerce that",
                "it used to be, plenty of",
                "people still wander to this",
                "area. I wonder why, exactly..."
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as(
        "Rahasu",
        args![
            "Hey, before you leave,",
            "you really ought to check",
            "the view from the lighthouse.",
            "It's... It's breathtaking..."
        ],
    )?;
    ctx.close()
}

pub fn hallosu_cmd(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Hallosu",
        args![
            "Hello, this is one of the",
            "lighthouses that make up",
            "Paros Lighthouse. However,",
            "right now it's undergoing",
            "renovation, so it's not",
            "open to the public."
        ],
    )?;
    ctx.close()
}

pub fn zain_cmd(ctx: &Ctx) -> Script {
    let speaker = Val::from("[") + ctx.call(Function::StrNpcInfo, args![1])? + Val::from("]");
    ctx.lines(args![
        speaker.clone(),
        "Would you like to",
        "board a ship on the",
        "Reudelus route? You",
        "can travel on Reudelus",
        "to Alberta or Izlude."
    ])?;
    ctx.next()?;
    match ctx.menu(&["Alberta - 600 Zeny", "Izlude - 800 Zeny", "Cancel"])? {
        0 => {
            if ctx.player().zeny()? >= 600 {
                ctx.player().set_zeny(ctx.player().zeny()? - 600)?;
                ctx.warp("alberta", 192, 169)?;
                return ctx.end();
            }
        }
        1 => {
            if ctx.player().zeny()? >= 800 {
                ctx.player().set_zeny(ctx.player().zeny()? - 800)?;
                ctx.warp("izlude", 176, 182)?;
                return ctx.end();
            }
        }
        _ => {
            ctx.lines(args![
                speaker.clone(),
                "Travel by ship is",
                "still one of the safest and",
                "dependable methods of",
                "transportation. I invite you",
                "to try Reudelus travel soon~"
            ])?;
            return ctx.close();
        }
    }
    ctx.lines(args![
        speaker.clone(),
        "I'm sorry, but you",
        "don't have enough",
        "zeny for the boarding fare."
    ])?;
    ctx.close()
}

pub fn kafra_employee(ctx: &Ctx) -> Script {
    ctx.fx().cutin("kafra_07", 2)?;
    ctx.lines_as(
        "Kafra Misty",
        args![
            "Welcome to the",
            "Kafra Corporation.",
            "You know that our",
            "service is always",
            "on your side~"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Kafra Misty",
        args![
            "The Kafra Corporation",
            "Western Division promises",
            "the best quality service that",
            "emphasizes reliability, and",
            "total consumer satisfaction.",
            "Thank you for your patronage~"
        ],
    )?;
    ctx.close_window()?;
    ctx.fx().cutin("", 255)?;
    ctx.end()
}

pub fn serutero_cmd(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Serutero",
        args![
            "Hello, I'm Serutero,",
            "guardian of the roads that",
            "lead to Sandaruman Fortress.",
            "If you really want to go there,",
            "I'll permit you to continue, but you must beware of its dangers..."
        ],
    )?;
    ctx.next()?;
    match ctx.menu(&["I'm going there!", "Sandaruman Fortress?", "Cancel"])? {
        0 => {
            ctx.lines_as(
                "Serutero",
                args![
                    "So you're really going",
                    "to go to Sandaruman",
                    "Fortress. Alright then,",
                    "good luck, and be careful!"
                ],
            )?;
            ctx.close_window()?;
            ctx.warp("cmd_fild08", 331, 319)?;
            return ctx.end();
        }
        1 => {
            ctx.lines_as(
                "Serutero",
                args![
                    "Although Sandaruman",
                    "Fortress is infested with",
                    "monsters now, it used to be",
                    "a province where people lived.",
                    "However, they were always",
                    "invaded and pillaged..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Serutero",
                args![
                    "Sandaruman's inhabitants",
                    "eventually adapted to the",
                    "invasions, developing smoke",
                    "signals and fortifications to",
                    "withstand the ravages of war.",
                    "Then, Comodo was built..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Serutero",
                args![
                    "Comodo grew in power and",
                    "influence and eventually annexed Sandaruman. More and more people",
                    "moved from the fortress to Comodo until Sandaruman fortress was",
                    "essentially abandoned."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Serutero",
                args![
                    "There were a few people",
                    "remaining in Sandaruman,",
                    "but they revolted and some",
                    "fledging government came into",
                    "power there. The monsters took",
                    "the chance to take over..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Serutero",
                args![
                    "There's nothing around",
                    "Sandaruman now. Well, nothing",
                    "except maybe Paros Lighthouse,",
                    "which is southwest of here. That place might be of interest to",
                    "aspiring Rogues, I hear..."
                ],
            )?;
            return ctx.close();
        }
        _ => {
            ctx.lines_as(
                "Serutero",
                args![
                    "You know, if you're",
                    "tired of traveling, you",
                    "can rest in ^3355FFComodo^000000. That",
                    "place is a pretty popular",
                    "tourist attraction, especially",
                    "for you adventurer types."
                ],
            )?;
            return ctx.close();
        }
    }
}

use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn ledrion_payon_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_partyleader = Val::from(0);
    let mut l_partymembercount = Val::from(0);
    let mut l_present = Val::from(0);
    runtime::party_members(ctx, ctx.call(Function::GetCharacterId, vec![Val::from(1)])?, Val::from(0))?;
    l_partymembercount = ctx.var("$@partymembercount").get()?;
    l_partyleader = ctx.call(
        Function::IsPartyLeader,
        vec![ctx.call(Function::GetCharacterId, vec![Val::from(1)])?],
    )?;
    if ctx.call(Function::CheckWeight, vec![Val::from(907), Val::from(300)])? == 0 {
        ctx.lines(args![
            "^3355FFWait a minute! You're",
            "carrying too many items",
            "right now: store some of",
            "your extra things in Kafra",
            "Storage, and then come back.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (((((ctx.call(Function::CountItem, vec![Val::from(7731)])?.number()? > 0
        || ctx.call(Function::CountItem, vec![Val::from(7732)])?.number()? > 0)
        || ctx.call(Function::CountItem, vec![Val::from(7735)])?.number()? > 0)
        || ctx.call(Function::CountItem, vec![Val::from(7736)])?.number()? > 0)
        || ctx.call(Function::CountItem, vec![Val::from(7739)])?.number()? > 0)
        || ctx.call(Function::CountItem, vec![Val::from(7740)])?.number()? > 0)
    {
        ctx.lines_as(
            "Ledrion",
            args![
                "Hm? I'm afraid that you've",
                "misunderstood me earlier...",
                "You're not ready to see",
                "me until you bring enough",
                "of your group members with you."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (((ctx.call(Function::CountItem, vec![Val::from(7741)])?.number()? > 0 && l_partyleader.clone().loosely_equals(&Val::from(1)))
        && ctx.call(Function::GetCharacterId, vec![Val::from(2)])?.number()? > 0)
        && l_partymembercount.clone().number()? > 5)
    {
        ctx.lines_as(
            "Ledrion",
            args![
                "Ah, you've brought the",
                "last ticket from Rospii.",
                "Now I'm convinced that",
                "you and your group really",
                "work well together. Good",
                "job on passing the trials!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ledrion",
            args![
                "Your group must truly",
                "understand the value of",
                "teamwork. And thanks to",
                "you, I've won the bet! Er, but",
                "what's more important is you",
                "proving my faith in you guys."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ledrion",
            args![
                "I'm really proud of",
                "you guys for your great",
                "accomplishment. Here...",
                "You deserve a real reward!"
            ],
        )?;
        ctx.next()?;
        l_present = ctx.call(Function::Rand, vec![Val::from(1), Val::from(7)])?;
        ctx.call(Function::DelItem, vec![Val::from(7741), Val::from(1)])?;
        ctx.var("party_relay").set(Val::from(0))?;
        ctx.call(Function::GetItem, vec![Val::from(644), Val::from(6)])?;
        ctx.call(Function::GetItem, vec![Val::from(603), Val::from(3)])?;
        if l_present.clone() == 2 {
            ctx.call(Function::GetItem, vec![Val::from(1365), Val::from(1)])?;
        } else if l_present.clone() == 4 {
            ctx.call(Function::GetItem, vec![Val::from(1367), Val::from(1)])?;
        } else if l_present.clone() == 6 {
            ctx.call(Function::GetItem, vec![Val::from(1527), Val::from(1)])?;
        } else {
            ctx.call(Function::GetItem, vec![Val::from(617), Val::from(3)])?;
        }
        shared::quests_partyrelay::f_partyrelay_exp(ctx, vec![])?;
        ctx.lines_as(
            "Ledrion",
            args![
                "Well, I hope you like it!",
                "If you can't use it, then",
                "why don't you share it with",
                "someone in your guild? Feel",
                "free to come back if you want",
                "to try my trials again, okay?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ledrion",
            args![
                "Once again, I'd like to",
                "thank you. I'm not sure",
                "if we'll ever meet again,",
                "but I'll be praying for you as",
                "you go on your adventures."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.call(Function::CountItem, vec![Val::from(7741)])?.number()? > 0 {
        ctx.lines_as(
            "Ledrion",
            args![
                "Hm? Why aren't your",
                "comrades with you?",
                "You must bring your",
                "group members here to",
                "proceed with these trials."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("party_relay").get()? == 23 {
        ctx.lines_as(
            "Ledrion",
            args![
                "Aren't you supposed to",
                "ask one a Swordman or",
                "Mage Class member of ",
                "your group to bring a ticket",
                "over to Gatan right now?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (((ctx.call(Function::CountItem, vec![Val::from(7737)])?.number()? > 0 && l_partyleader.clone().loosely_equals(&Val::from(1)))
        && ctx.call(Function::GetCharacterId, vec![Val::from(2)])?.number()? > 0)
        && l_partymembercount.clone().number()? > 5)
    {
        ctx.lines_as(
            "Ledrion",
            args![
                "Great, you've brought",
                "me the eighth ticket from",
                "Lospii. Your group must",
                "work pretty well together,",
                "eh? It usually isn't easy to",
                "gather people like that..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ledrion",
            args![
                "Please keep up the good",
                "work until you finish all",
                "of the trials, okay? Here's",
                "a little reward for your",
                "effort thus far, and the",
                "nineth ticket for your guild."
            ],
        )?;
        ctx.call(Function::DelItem, vec![Val::from(7737), Val::from(1)])?;
        ctx.var("party_relay").set(Val::from(23))?;
        ctx.call(Function::GetItem, vec![Val::from(603), Val::from(3)])?;
        ctx.call(Function::GetItem, vec![Val::from(617), Val::from(1)])?;
        ctx.call(Function::GetItem, vec![Val::from(7738), Val::from(1)])?;
        shared::quests_partyrelay::f_partyrelay_exp(ctx, vec![])?;
        ctx.next()?;
        ctx.lines_as(
            "Ledrion",
            args![
                "Please give that ticket",
                "to someone in your guild",
                "that's a Swordman or Mage",
                "Class character, and tell",
                "him to deliver it to Gatan.",
                "Okay? I'll be seeing you."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.call(Function::CountItem, vec![Val::from(7737)])?.number()? > 0 {
        ctx.lines_as(
            "Ledrion",
            args![
                "Hm? Why aren't your",
                "comrades with you?",
                "You must bring your",
                "group members here to",
                "proceed with these trials."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("party_relay").get()? == 12 {
        ctx.lines_as(
            "Ledrion",
            args![
                "Hm? Aren't you supposed to",
                "ask a Mage Class character",
                "in your group to deliver",
                "a ticket to Gatan?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (((ctx.call(Function::CountItem, vec![Val::from(7733)])?.number()? > 0 && l_partyleader.clone().loosely_equals(&Val::from(1)))
        && ctx.call(Function::GetCharacterId, vec![Val::from(2)])?.number()? > 0)
        && l_partymembercount.clone().number()? > 5)
    {
        ctx.lines_as(
            "Ledrion",
            args![
                "I see that you've brought",
                "me the fourth ticket from",
                "Lospii. Good work, good work.",
                "Please hang in there, and",
                "finish all the trials we've",
                "set before you guys, alright?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ledrion",
            args![
                "Here's a little something",
                "to reward you for your",
                "efforts for now. Now, please",
                "give this ticket to a Mage",
                "Class character, and tell",
                "him to bring it to Gatan."
            ],
        )?;
        ctx.call(Function::DelItem, vec![Val::from(7733), Val::from(1)])?;
        ctx.var("party_relay").set(Val::from(12))?;
        ctx.call(Function::GetItem, vec![Val::from(644), Val::from(3)])?;
        ctx.call(Function::GetItem, vec![Val::from(603), Val::from(1)])?;
        ctx.call(Function::GetItem, vec![Val::from(7734), Val::from(1)])?;
        shared::quests_partyrelay::f_partyrelay_exp(ctx, vec![])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.call(Function::CountItem, vec![Val::from(7733)])?.number()? > 0 {
        ctx.lines_as(
            "Ledrion",
            args![
                "Hm? Why aren't your",
                "comrades with you?",
                "You must bring your",
                "group members here to",
                "proceed with these trials."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("party_relay").get()? == 2 {
        ctx.lines_as(
            "Ledrion",
            args![
                "You didn't finish what",
                "I asked you to do, did you?",
                "It's not time for you to",
                "come to me, not yet."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ((((ctx.var("BaseLevel").get()?.number()? > 39 && l_partyleader.clone().loosely_equals(&Val::from(1)))
        && ctx.call(Function::GetCharacterId, vec![Val::from(2)])?.number()? > 0)
        && l_partymembercount.clone().number()? > 5)
        && ctx.var("party_relay").get()? == 1)
    {
        ctx.lines_as(
            "Ledrion",
            args![
                "As I mentioned earlier,",
                "I'm challenging guild",
                "parties with missions",
                "that will require ",
                "cooperation between",
                "the group members."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ledrion",
            args![
                "These challenges are",
                "something of a relay",
                "race, where we'll ask",
                "for an item to be passed",
                "to a member of a specific",
                "Class to deliver for us."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ledrion",
            args![
                "Of course, we, the",
                "administrators of this",
                "test, might add our own",
                "little challenges here and",
                "there to see just how",
                "capable your group is."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ledrion",
            args![
                "Why don't you give it",
                "a shot? You won't lose",
                "anything by trying our",
                "test. I guarantee that",
                "you'll benefit from",
                "our little exercise."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ledrion",
            args![
                "Heh! Let's start with you!",
                "Here, take this ticket, and",
                "give it to a Swordman Class",
                "member of your guild, and",
                "ask him to bring it to Gatan.",
                "The relay has started~"
            ],
        )?;
        ctx.next()?;
        ctx.var("party_relay").set(Val::from(2))?;
        ctx.call(Function::GetItem, vec![Val::from(7730), Val::from(1)])?;
        ctx.lines_as(
            "Ledrion",
            args![
                "Now, since you're the",
                "one that started the relay,",
                "you'll have to come back",
                "to me later on. Don't",
                "worry, we'll let you know",
                "once the time is right."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (((l_partyleader.clone().loosely_equals(&Val::from(1)) && ctx.call(Function::GetCharacterId, vec![Val::from(2)])?.number()? > 0)
        && l_partymembercount.clone().number()? > 5)
        && ctx.var("party_relay").get()?.number()? > 0)
    {
        ctx.lines_as(
            "Ledrion",
            args![
                "As leader of your",
                "Party, you should",
                "always think of what",
                "would be best for all",
                "of your partners."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (l_partyleader.clone().loosely_equals(&Val::from(0)) && ctx.call(Function::GetCharacterId, vec![Val::from(2)])?.number()? > 0) {
        ctx.lines_as(
            "Ledrion",
            args![
                "I'm sorry, but there's",
                "nothing I can really offer",
                "you... Unless you formed",
                "a Party and became its",
                "leader, there's absolutely",
                "nothing I can do for you..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Ledrion",
        args![
            "Ah, pleased to make your",
            "acquaintance, adventurer.",
            "I am Ledrion, a man of great",
            "intelligence, mystery, and",
            "most importantly, wealth.",
            "Yes, I'm a philanthropist."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Ledrion",
        args![
            "I'm always working to make",
            "a meaningful contribution",
            "to all of Rune-Midgarts, which",
            "brings me to why I am here.",
            "There are many adventurers like you, but we have a problem."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Ledrion",
        args![
            "So many adventurers have",
            "chosen the ways of greed,",
            "and have forgotten how to",
            "work well together with their",
            "colleagues. Such selfishness",
            "sickens me to no end! Ugh!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Ledrion",
        args![
            "I may not be an adventurer,",
            "but I have the money to do",
            "something. I am hosting",
            "my own little challenge",
            "to adventurers to encourage",
            "camaraderie and teamwork."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Ledrion",
        args![
            "Please, I invite you to",
            "give it a try! Allow me to",
            "state some of my conditions.",
            "Firstly, you must be part of",
            "a group of 6 or more members,",
            "such as a Party or a Guild."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Ledrion",
        args![
            "Secondly, only the leader",
            "of the group can begin the",
            "first part of my challenge.",
            "If you are not the leader,",
            "let him know that he should",
            "speak to me for the challenge."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Ledrion",
        args![
            "Thirdly, you must have a",
            "Swordman Class, Mage Class,",
            "Acolyte Class, Archer Class,",
            "Thief Class, and Merchant",
            "Class character to complete",
            "the challenges you'll receive."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Ledrion",
        args![
            "Transcended characters",
            "are good too. Fourthly, all",
            "group members must be",
            "at least Base Level 40.",
            "Everything else, well,",
            "you'll learn along the way."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Ledrion",
        args![
            "Don't fret, I won't have you",
            "jumping through hoops for",
            "no reason at all. I'll provide",
            "plenty of rewards as an",
            "incentive for your group.",
            "Would you like to participate?"
        ],
    )?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("Sure!:No.")])? {
        1 => {
            if (((ctx.var("BaseLevel").get()?.number()? > 39 && l_partyleader.clone().loosely_equals(&Val::from(1)))
                && ctx.call(Function::GetCharacterId, vec![Val::from(2)])?.number()? > 0)
                && l_partymembercount.clone().number()? > 5)
            {
                ctx.lines_as(
                    "Ledrion",
                    args![
                        "Great! Let's see...",
                        "Well, you meet all the",
                        "requirements. It looks",
                        "like I'll win the bet! Er, let",
                        "me give you your first",
                        "set of instructions."
                    ],
                )?;
                ctx.next()?;
                ctx.var("party_relay").set(Val::from(1))?;
                ctx.lines_as(
                    "Ledrion",
                    args!["Just--^666666*Cough!*^000000", "Lemme clear my", "throat for a second..."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Ledrion",
                    args![
                        "That's great! Still,",
                        "you're not ready to tackle",
                        "my little challenge until",
                        "you fulfill all of my",
                        "conditions. Come back",
                        "once you do that, alright?"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
        2 => {
            ctx.lines_as(
                "Ledrion",
                args![
                    "Really? Trust me, this",
                    "little challenge will be",
                    "worthwhile for you, and it",
                    "would bring a greater sense",
                    "of solidarity and teamwork",
                    "to Rune-Midgarts. It's win-win!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn ledrion_payon(ctx: &Ctx) -> Script {
    ledrion_payon_body(ctx, Vec::new()).map(|_| ())
}

use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn knight_ss_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sign1 = Val::from(0);
    let mut l_sign2 = Val::from(0);
    let mut l_sign3 = Val::from(0);
    let mut l_sign4 = Val::from(0);
    let mut l_signid = Val::from(0);
    shared::quests_the_sign_quest::f_updatesignvars(ctx, vec![])?;
    ctx.mes("[Leibech]")?;
    if ctx.var("sign_q").get()?.number()? < 38 {
        ctx.lines(args![
            "I have a great",
            "interest in collecting",
            "unique and uncommon",
            "goods. You know, things",
            "that most people see just",
            "once in their lifetimes."
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Leibech",
            args![
                "I usually use the",
                "Alchesh Trading Company",
                "to help add to my collection.",
                "Their prices aren't the cheapest, but their service is very reliable."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("sign_q").get()? == 38 {
        if ctx.call(Function::CountItem, vec![Val::from(7049)])?.number()? < 1 {
            ctx.lines(args![
                "Excuse me...",
                "Are you from the Alchesh",
                "Trading Company? I've been",
                "waiting for my order and they"
            ])?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("No, I'm not.:They must be busy.:Yes, I have your order.")])? {
                1 => {
                    ctx.lines_as(
                        "Leibech",
                        args!["Oh, I'm sorry.", "My mistake. Well", "then, be safe in", "your travels."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines_as(
                        "Leibech",
                        args![
                            "I suppose you're",
                            "right. I mean, Juno",
                            "is pretty far from",
                            "Alberta. Maybe I'm",
                            "just overly excited",
                            "about my delivery..."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                3 => {
                    ctx.lines_as(
                        "Leibech",
                        args!["Really?!", "That's great!", "Do you have the", "item I ordered?", "Where is it?"],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("I put it somewhere.:Sorry, I lost it.")])? {
                        1 => {
                            ctx.lines_as(
                                "Leibech",
                                args![
                                    "Oh, alright. ",
                                    "So long as you didn't",
                                    "lose it, that's fine. I'll wait",
                                    "here, so would you bring it",
                                    "to me as soon as you can?"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as(
                                "Leibech",
                                args!["What...?", "Hey, man.", "That's not funny!", "You're joking, right?"],
                            )?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("No. I'm not joking.:Okay, you got me~")])? {
                                1 => {
                                    ctx.lines_as(
                                        "Leibech",
                                        args![
                                            "I don't believe it!",
                                            "I think I'm going to",
                                            "write a letter to your",
                                            "boss, Mister Bakerlan.",
                                            "What do you think ",
                                            "about that?"
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                2 => {
                                    ctx.lines_as(
                                        "Leibech",
                                        args![
                                            "Oh, thank goodness!",
                                            "I thought you might have",
                                            "really lost it! Say, would you",
                                            "bring it to me as soon as",
                                            "can? I'll wait for you here."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        } else {
            l_signid = ctx.call(
                Function::GetCharacterId,
                vec![Val::from(0), ctx.call(Function::StrCharInfo, vec![Val::from(0)])?],
            )?;
            l_sign3 = runtime::op(&l_signid.clone(), "&", &Val::from(65535))?;
            l_sign4 = runtime::op(&l_signid.clone(), ">>", &Val::from(16))?;
            l_sign1 = Val::from(254);
            l_sign2 = Val::from(0);
            if !(ctx
                .call(
                    Function::CountItem2,
                    vec![
                        Val::from(7049),
                        Val::from(1),
                        Val::from(0),
                        Val::from(0),
                        l_sign1.clone(),
                        l_sign2.clone(),
                        l_sign3.clone(),
                        l_sign4.clone(),
                    ],
                )?
                .is_true())
            {
                ctx.lines(args![
                    "Hm...?",
                    "There must be some",
                    "mistake. This isn't what",
                    "I ordered. Well, I can wait... "
                ])?;
                ctx.next()?;
                ctx.lines_as(
                    "Leibech",
                    args![
                        "Just come back and",
                        "bring me what I actually",
                        "ordered, alright? And this",
                        "time, don't make any mistakes.",
                        "Thanks, I appreciate it~"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines(args![
                    "Finally, it's here!",
                    "Yes, this is what I ordered.",
                    "Thanks for the delivery!"
                ])?;
                ctx.next()?;
                ctx.lines_as(
                    "Leibech",
                    args![
                        "Well, here's your",
                        "receipt. Please take",
                        "this to Mister Bakerlan",
                        "so he knows you did",
                        "a good job for me.",
                        "Thanks again!"
                    ],
                )?;
                ctx.call(
                    Function::DelItem2,
                    vec![
                        Val::from(7049),
                        Val::from(1),
                        Val::from(1),
                        Val::from(0),
                        Val::from(0),
                        l_sign1.clone(),
                        l_sign2.clone(),
                        l_sign3.clone(),
                        l_sign4.clone(),
                    ],
                )?;
                ctx.var("sign_q").set(Val::from(39))?;
                ctx.call(Function::GetItem, vec![Val::from(7181), Val::from(1)])?;
                {
                    if ctx.var("BaseLevel").get()?.number()? < 60 {
                        ctx.call(Function::GetExperience, vec![Val::from(1000), Val::from(0)])?;
                    } else if ctx.var("BaseLevel").get()?.number()? < 70 {
                        ctx.call(Function::GetExperience, vec![Val::from(2000), Val::from(0)])?;
                    } else if ctx.var("BaseLevel").get()?.number()? < 80 {
                        ctx.call(Function::GetExperience, vec![Val::from(4000), Val::from(0)])?;
                    } else if ctx.var("BaseLevel").get()?.number()? < 90 {
                        ctx.call(Function::GetExperience, vec![Val::from(7000), Val::from(0)])?;
                    } else {
                        ctx.call(Function::GetExperience, vec![Val::from(11000), Val::from(0)])?;
                    }
                }
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    } else {
        if ctx.var("sign_q").get()? == 97 {
            ctx.lines(args![
                "I have a great",
                "interest in collecting",
                "unique and uncommon",
                "goods. You know, things",
                "that most people see just",
                "once in their lifetimes."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Leibech",
                args![
                    "I usually use the",
                    "Alchesh Trading Company",
                    "to help add to my collection.",
                    "Their prices aren't the cheapest, but their service is very reliable."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("sign_q").get()? == 98 {
            ctx.lines(args![
                "I am interested in collecting uncommon stuffs.",
                "For that, I need to find something that I cannot see around here."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Leibech",
                args![
                    "Usually, I am using Alchesh Trading Company for that.",
                    "Although the price is little bit higher than I wish,",
                    "they are very reliable."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines(args![
                "Hey, thanks for",
                "delivering my order",
                "for me. It was a great",
                "addition to my collection!"
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Leibech",
                args![
                    "It's really tough to",
                    "find unique and oddball",
                    "items, but I find great pride",
                    "in expanding my collection~"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    Ok(Val::from(0))
}

pub fn knight_ss(ctx: &Ctx) -> Script {
    knight_ss_body(ctx, Vec::new()).map(|_| ())
}

fn lonely_looking_woman_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input_s = Val::from("");
    shared::quests_the_sign_quest::f_updatesignvars(ctx, vec![])?;
    ctx.mes("[Brenda Howard]")?;
    if ctx.var("sign_q").get()?.number()? < 54 {
        ctx.lines(args![
            "Hmm...",
            "What should I make for",
            "dinner today? Pickled cabbage?",
            "I learned how to make it a while ago, but I've never gotten the chance to make it yet..."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("sign_q").get()? == 54 {
        ctx.lines(args![
            "Hey, who are you anyhow?",
            "Don't you know it's rude to",
            "enter someone's house without",
            "being invited? Besides, I don't have the time to help strangers while I'm busy making dinner..."
        ])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["W-wait!", "I'm here to see..."],
        )?;
        ctx.next()?;
        let (input, status) = runtime::input_text(ctx, None, None)?;
        l_input_s = input;
        if l_input_s.clone() != "Engel Howard" {
            ctx.lines_as("Brenda Howard", args!["Huh?", "What are you", "talking about?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Brenda Howard",
                args!["Hmm...", "I wonder", "how my pickled", "cabbage will turn out?"],
            )?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_DELIGHT")?])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Brenda Howard",
                args![
                    "Ah... I see.",
                    "You're looking for my husband.",
                    "My husband, my daughter and",
                    "I just moved to Geffen from",
                    "Prontera a while ago."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Brenda Howard", args!["We moved to Geffen for the sake of my husband's business. Apparently, this town is closest to something he really seems to need. Whatever it is, it must be really important to his smithing work."])?;
            ctx.next()?;
            ctx.lines_as("Brenda Howard", args!["Oh, and if you're going to look", "for my husband, please talk to my daughter before you go. I think she has something that she wants to give to her father..."])?;
            ctx.var("sign_q").set(Val::from(55))?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else if ctx.var("sign_q").get()?.number()? < 57 {
        ctx.lines(args![
            "Ho-ho~!",
            "Crisp and delicious pickled",
            "cabbage~ My kid loves this stuff and finished it all by herself the last time I made it."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("sign_q").get()? == 57 {
        ctx.lines(args![
            "Ah...",
            "So you've seen my husband.",
            "How is he doing? I worry about",
            "whether or not he's taking care",
            "of himself enough..."
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Brenda Howard",
            args![
                "Oh, I can't help but worry!",
                "Thank you for letting me know",
                "how he's doing. Still, he should",
                "be doing a better job of keeping",
                "in touch with us. ^333333*Sigh*^000000"
            ],
        )?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("sign_q").get()?.number()? < 62 {
        if ctx.call(Function::CountItem, vec![Val::from(7278)])?.number()? > 0 {
            ctx.lines(args![
                "Thank you so much for bringing",
                "this to me. I'm so happy to hear that he's fine and doing well.",
                "Although he's not here often enough, it's good to know",
                "that he loves his job."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Brenda Howard",
                args![
                    "Even so, I should start letting",
                    "him know that I want him to spend",
                    "a little more time at home. Oh, and why don't you talk to Liana? She's been waiting to tell you something."
                ],
            )?;
            ctx.call(Function::DelItem, vec![Val::from(7278), Val::from(1)])?;
            ctx.var("sign_q").set((ctx.var("sign_q").get()? + Val::from(4)))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.mes("Liana's daddy needs to come home more often. ^333333*Sigh*^000000 Even though he's away for long periods of time, I suppose it's for the best...")?;
            ctx.close_window()?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
            return Err(Stop::End);
        }
    } else {
        if ctx.var("sign_q").get()?.number()? < 67 {
            ctx.lines(args![
                "So how have you been?",
                "So how have you been?",
                "As usual, Liana misses",
                "her daddy, but I suppose",
                "it can't be helped..."
            ])?;
            ctx.next()?;
            ctx.lines(args![
                "I may be no expert, but I'm",
                "certain that my husband is",
                "the best blacksmith in the world! Did you know Hollgrehenn and Aragham used to be his apprentices? Ho ho ho~"
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines(args![
                "Try to cherish your",
                "family, especially through the",
                "hard times. Even when you're angry with them, try to be more understanding. I know it's hard..."
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
}

pub fn lonely_looking_woman(ctx: &Ctx) -> Script {
    lonely_looking_woman_body(ctx, Vec::new()).map(|_| ())
}

fn cute_girl_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    shared::quests_the_sign_quest::f_updatesignvars(ctx, vec![])?;
    if ctx.call(Function::CheckWeight, vec![Val::from(908), Val::from(200)])? == 0 {
        ctx.lines(args![
            "^3355FFWait a second! Right now,",
            "you have too many items in your inventory. Please come back after you've made more available inventory space.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.mes("[Liana]")?;
    if ctx.var("sign_q").get()?.number()? < 54 {
        ctx.lines(args![
            "*Pout*",
            "When's my daddy",
            "coming home?!",
            "I... I miss him so much~",
            "*Cries*"
        ])?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("sign_q").get()? == 54 {
        ctx.lines(args![
            "Oh, my mom said she's gonna cook me something good today.",
            "I wonder what she's gonna make?",
            "I hope it's pickled cabbage again!"
        ])?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_SMILE")?])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("sign_q").get()? == 55 {
        ctx.lines(args![
            "Everyone says that daddy is the best blacksmith in the whole world! But now he lets his apprentices do",
            "the work so he can go around the world to find a rare ore~ Isn't that amazing?"
        ])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("What are you drawing by the way?:Apprentices?")])? {
            1 => {
                ctx.lines_as(
                    "Liana",
                    args![
                        "Oh, this?",
                        "It's a letter for my daddy.",
                        ((((Val::from("Mom said that some ")
                            + (if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                                Val::from("guy")
                            } else {
                                Val::from("lady")
                            }))
                            + Val::from(" is gonna try to find my dad, so I'm making this so "))
                            + (if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                                Val::from("")
                            } else {
                                Val::from("s")
                            }))
                            + Val::from("he can take it him~"))
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as("Liana", args!["Yeah, Uncle Hollegrehenn and Aragham come to visit sometimes and we all play. But, it's more fun to see my dad, even though he's not back yet. ^333333*Pout*^000000"])?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Don't you miss your dad?:He also misses you a lot.")])? {
                    1 => {
                        ctx.lines_as(
                            "Liana",
                            args![
                                "I miss him sooo much!",
                                "^333333*Cries*^000000 But Mom always",
                                "says that he's too busy.",
                                "^333333*Pouts*^000000"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as(
                            "Liana",
                            args![
                                "I hope so...",
                                "But I hate it when",
                                "he doesn't have enough",
                                "time to come home and",
                                "see me. Sooooooooo~",
                                "I wrote this letter!"
                            ],
                        )?;
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("....:I'll bring this to him.")])? {
                            1 => {
                                ctx.lines_as(
                                    "Brenda Howard",
                                    args![
                                        "Liana~",
                                        ((((Val::from("This nice ")
                                            + (if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                                                Val::from("man")
                                            } else {
                                                Val::from("lady")
                                            }))
                                            + Val::from(" is going to look for your dad. Why don't you ask "))
                                            + (if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                                                Val::from("him")
                                            } else {
                                                Val::from("her")
                                            }))
                                            + Val::from(" to take your letter to him for you?"))
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Liana", args!["You are...?", "Really, really?", "H-hooray!"])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Liana",
                                    args!["Okay then, don't", "forget to give this to", "my dad, okay? Promise?"],
                                )?;
                                ctx.call(Function::Emotion, vec![ctx.constant("ET_DELIGHT")?])?;
                                ctx.var("sign_q").set(Val::from(56))?;
                                ctx.call(Function::GetItem, vec![Val::from(7276), Val::from(1)])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    "Liana",
                                    args![
                                        "Mmmm...",
                                        "But I don't know you!",
                                        "Mom says I should never",
                                        "ask for anything from strangers."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    } else if ctx.var("sign_q").get()?.number()? < 62 {
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?.number()? < 6 {
            ctx.lines(args![
                "My dad never broke anything",
                "his customers gave him. And my uncle Hollgrehenn and Aragham wouldn't let anything like that",
                "happen, would they?"
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines(args![
                "Would you please bring my letter over to my dad? Heh heh, he'll be",
                "so happy to hear from me!"
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        if ctx.var("sign_q").get()?.number()? < 66 {
            ctx.lines(args![
                "Yay~!",
                "Thank you for bringing my letter to my dad. Here, here!",
                "You have to share my treasure with me, 'kay?"
            ])?;
            ctx.var("sign_q").set((ctx.var("sign_q").get()? + Val::from(4)))?;
            ctx.call(Function::GetItem, vec![Val::from(529), Val::from(10)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.mes("I never saw my dad fail to upgrade a weapon or armor. Mmm? But maybe if he had something really really old and rare and special...")?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    Ok(Val::from(0))
}

pub fn cute_girl(ctx: &Ctx) -> Script {
    cute_girl_body(ctx, Vec::new()).map(|_| ())
}

fn flaming_spirit_man_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_anvil = Val::from(0);
    let mut l_pass_s1 = Val::from(0);
    let mut l_stime_e = Val::from(0);
    let mut l_stime_e1 = Val::from(0);
    shared::quests_the_sign_quest::f_updatesignvars(ctx, vec![])?;
    if ctx.call(Function::CheckWeight, vec![Val::from(908), Val::from(200)])? == 0 {
        ctx.lines(args![
            "^3355FFWait a second! Right now,",
            "you have too many items in your inventory. Please come back after you've freed up more inventory space.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.mes("[Engel]")?;
    if (ctx.var("sign_q").get()?.number()? > 1 && ctx.var("sign_q").get()?.number()? < 54) {
        if ctx.call(Function::CountItem, vec![Val::from(1002)])?.number()? > 0 {
            ctx.lines(args![
                "Just as I suspected...",
                "I didn't bring enough of them.",
                "Hmm, this is serious. What",
                "am I going to do about this?"
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Engel",
                args![
                    "Oh great! I see that you're",
                    "carrying some Iron Ores with you. Would you be so kind as to lend me 1 Iron Ore? Please, I beg of you..."
                ],
            )?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Sure, why not?:Sorry, I can't.")])? {
                1 => {
                    ctx.lines_as(
                        "Engel",
                        args![
                            "Thank you so much!",
                            "If it weren't for your help,",
                            "I'd be in big trouble. I really appreciate you stepping in and volunteering your materials like this."
                        ],
                    )?;
                    ctx.call(Function::DelItem, vec![Val::from(1002), Val::from(1)])?;
                    ctx.call(Function::GetExperience, vec![Val::from(10), Val::from(0)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines_as(
                        "Engel",
                        args![
                            "I suppose I understand.",
                            "However, you're lucky that I don't kick you out of my forge right here and right now~"
                        ],
                    )?;
                    ctx.close_window()?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_HNG")?])?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        } else {
            ctx.lines(args!["I hope you understand that", "this forge isn't really open to the public. I'm doing some intensive training,so I'd appreciate it if you would just leave now."])?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_HNG")?])?;
            ctx.close_window()?;
            ctx.call(Function::Warp, vec![Val::from("mjo_dun02"), Val::from(372), Val::from(346)])?;
            return Err(Stop::End);
        }
    } else {
        if ctx.var("sign_q").get()?.number()? < 56 {
            ctx.lines(args!["I hope you understand that", "this forge isn't really open to the public. I'm doing some intensive training,so I'd appreciate it if you would just leave now."])?;
            ctx.close_window()?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_HNG")?])?;
            return Err(Stop::End);
        } else {
            if ctx.var("sign_q").get()? == 56 {
                if ctx.call(Function::CountItem, vec![Val::from(7276)])?.number()? > 0 {
                    ctx.lines(args![
                        "^333333*Sigh*^000000",
                        "It's been a long",
                        "time since I've seen",
                        "my family. I really should let them know how I'm doing sooner or later. Hmmm...."
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Engel",
                        args![
                            "...?",
                            "Were you looking for me?",
                            "I'm sorry, but I'm busy at the moment. Please don't disturb me while I try to get my work done."
                        ],
                    )?;
                    ctx.next()?;
                    'b2: {
                        let subject2 = Val::from(runtime::select_values(
                            ctx,
                            &[Val::from("I need your help.:Here is a letter for you.:I am leaving, leaving.")],
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
                                "Engel",
                                args![
                                    "Well, I'd like to help you,",
                                    "but now isn't a good for me.",
                                    "plenty of other good smiths out there who can help you with the work that you want done. Farewell."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        if !matched2 && subject2.loosely_equals(&Val::from(2)) {
                            matched2 = true;
                        }
                        if matched2 {
                            ctx.lines_as("Engel", args!["Oh, are you serious?", "Let me read it first."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Engel",
                                args!["Yes, yes.", "...Hahaha!", "It's so good to hear from", "my darling daughter."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Engel", args!["Thanks, I really appreciate that you've delivered this letter for me. Now, is there anything you'd like to ask of me? If it's not a big favor, I can probably help you."])?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("Please look at this.")])? {
                                1 => {
                                    ctx.lines_as("Engel", args!["Oh?", "Isn't this...?", "I see, I see!"])?;
                                    ctx.next()?;
                                    ctx.lines_as("Engel", args!["Long ago, my old master told me that there are these strange ores that have fallen from the sky. The most beautiful and mysterious is the one shines just like a star."])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Engel",
                                        args![
                                            "Most people may know it as Sobbing Starlight, but my master used to call it God's Tear Drop."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Engel", args!["However, this ore has been shattered into pieces. Judging from the smoothness of the edges, its inner power must have caused it to break apart. So you want me to put this back together, don't you?"])?;
                                    ctx.next()?;
                                    ctx.lines_as("Engel", args!["This will be a good challenge", "for me! However, I'll need some", "special tools and materials to repair this kind of ore. Now, I want you to bring me the following things..."])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Engel",
                                        args![
                                            "^FF00005 Mini Furnace^000000,",
                                            "^FF00002 Oridecon Hammer^000000,",
                                            "and ^ff00001 good quality Anvil^000000.",
                                            "Now keep in mind that ordinary anvils won't be good enough for",
                                            "this kind of work."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Engel", args!["Now, an Emperium Anvil would be ideal for this job. I used to have one, but a while back I ended up giving it to someone. Anyway, I'll wait here for you while you gather everything you need."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Engel", args!["Right. Let me guide you to a shortcut to the exit of these mines, just in case you don't know your way out of here. Good luck~"])?;
                                    ctx.call(Function::DelItem, vec![Val::from(7276), Val::from(1)])?;
                                    ctx.call(Function::DelItem, vec![Val::from(7177), Val::from(7)])?;
                                    ctx.var("sign_q").set(Val::from(57))?;
                                    ctx.close_window()?;
                                    ctx.call(Function::Warp, vec![Val::from("mjo_dun02"), Val::from(371), Val::from(344)])?;
                                    return Err(Stop::End);
                                }
                                _ => {}
                            }
                        }
                        if !matched2 && subject2.loosely_equals(&Val::from(3)) {
                            matched2 = true;
                        }
                        if matched2 {
                            ctx.lines_as("Engel", args!["Alright, then.", "Take care."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                } else {
                    ctx.lines(args![
                        "^333333*Sigh*^000000",
                        "It's been a long",
                        "time since I've seen",
                        "my family. I really should let them know how I'm doing sooner or later. Hmmm...."
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Engel",
                        args![
                            "...?",
                            "Do you have any business",
                            "me? I'm sorry, but right",
                            "now I'm very busy. Please leave",
                            "me alone to my research."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            } else {
                if ctx.var("sign_q").get()? == 57 {
                    ctx.mes("As I've told you before, I'm more likely to succeed in my work if I'm able to use a higher quality anvil. Let's me see what you've brought...")?;
                    ctx.next()?;
                    if ((ctx.call(Function::CountItem, vec![Val::from(612)])?.number()? > 4
                        && ctx.call(Function::CountItem, vec![Val::from(615)])?.number()? > 1)
                        && (((ctx.call(Function::CountItem, vec![Val::from(986)])?.number()? > 0
                            || ctx.call(Function::CountItem, vec![Val::from(987)])?.number()? > 0)
                            || ctx.call(Function::CountItem, vec![Val::from(988)])?.number()? > 0)
                            || ctx.call(Function::CountItem, vec![Val::from(989)])?.number()? > 0))
                    {
                        ctx.mes("[Engel]")?;
                        if ctx.call(Function::CountItem, vec![Val::from(986)])?.number()? > 0 {
                            ctx.lines(args![
                                "A...",
                                "Regular Anvil?",
                                "Alright, I guess I'll try it.",
                                "But I can't guarantee success."
                            ])?;
                            l_anvil = Val::from(0);
                        } else if ctx.call(Function::CountItem, vec![Val::from(987)])?.number()? > 0 {
                            ctx.lines(args![
                                "Ah...",
                                "An Oridecon Avil.",
                                "This isn't too bad,",
                                "but there's a good risk",
                                "that this might not work."
                            ])?;
                            l_anvil = Val::from(1);
                        } else if ctx.call(Function::CountItem, vec![Val::from(988)])?.number()? > 0 {
                            ctx.lines(args![
                                "Ah...",
                                "A Golden Anvil.",
                                "This just might be able to do the job. This might take a while, so I have something to ask of you."
                            ])?;
                            l_anvil = Val::from(2);
                        } else if ctx.call(Function::CountItem, vec![Val::from(989)])?.number()? > 0 {
                            ctx.lines(args!["Oh wow, an Emperium Anvil!", "And this looks like one I might have actually made. Great, I should be able to do this so long as I don't make any critical mistakes~"])?;
                            l_anvil = Val::from(3);
                        }
                        ctx.next()?;
                        ctx.lines_as("Engel", args!["While I'm working on this, would you deliver a letter to my family for me? I'm sorry, but please consider that I'm waiving the fee for restoring this Sobbing Starlight."])?;
                        ctx.next()?;
                        ctx.lines_as("Engel", args!["Well then...", "I wish you safety", "in your travels."])?;
                        ctx.call(Function::DelItem, vec![Val::from(612), Val::from(5)])?;
                        ctx.call(Function::DelItem, vec![Val::from(615), Val::from(2)])?;
                        ctx.call(Function::DelItem, vec![(Val::from(986) + l_anvil.clone()), Val::from(1)])?;
                        ctx.var("sign_q").set((Val::from(58) + l_anvil.clone()))?;
                        ctx.call(Function::GetItem, vec![Val::from(7278), Val::from(1)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.call(Function::Rand, vec![Val::from(1), Val::from(6)])? == 3 {
                        ctx.lines_as("Engel", args!["Oh, you don't have everything ready yet? Take your time, so long as you didn't forget what you needed to bring. You do remember, right?"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Engel",
                            args![
                                "^FF00005 Mini Furnace^000000,",
                                "^FF00002 Oridecon Hammer^000000,",
                                "and ^ff00001 good quality Anvil^000000.",
                                "That's all you need!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Engel", args!["Remember that I have a greater chance of repairing the Sobbing Starlight if I have access to a higher quality Anvil. Otherwise..."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as("Engel", args!["Still don't have everything ready? That's fine, just take your time. So long as you haven't forgotten all the things that you need. I mean, how can you forget when you have such an important ore on you?"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                } else {
                    if ctx.var("sign_q").get()?.number()? < 66 {
                        ctx.lines(args!["Are you back already?", "Sorry, but I haven't quite finished yet. I won't be done for a while, so why don't you deliver that letter to my family?"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if ctx.var("sign_q").get()?.number()? < 70 {
                            if ((((ctx.var("sign_q").get()? == 66
                                && ctx.call(Function::Rand, vec![Val::from(1), Val::from(100)])?.number()? < 38)
                                || (ctx.var("sign_q").get()? == 67
                                    && ctx.call(Function::Rand, vec![Val::from(1), Val::from(100)])?.number()? < 67))
                                || (ctx.var("sign_q").get()? == 68
                                    && ctx.call(Function::Rand, vec![Val::from(1), Val::from(100)])?.number()? < 91))
                                || ctx.var("sign_q").get()? == 69)
                            {
                                ctx.lines(args!["Are you back already?", "While you were gone, I managed to restore this Sobbing Starlight. Why don't you go ahead and take a look?"])?;
                                ctx.next()?;
                                ctx.var("sign_q").set(Val::from(71))?;
                                ctx.call(Function::GetItem, vec![Val::from(7178), Val::from(1)])?;
                                ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_YUFITELHIT")?])?;
                                ctx.next()?;
                                ctx.lines_as("Engel", args!["This was one of the most difficult jobs I've ever done. But look! It was beautiful when broken in fragments but now it's absolutely dazzling! This was really worth my effort."])?;
                                ctx.next()?;
                                ctx.lines_as("Engel", args!["In any case, I worked really hard to do a good job on this, so I hope you treasure this Sobbling Starlight, young adventurer."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                let subject4 = ctx.var("sign_q").get()?;
                                if subject4 == 66 || subject4 == 67 {
                                    ctx.lines(args![
                                        "You've returned?",
                                        "Well, I don't know how to put this, but once you hear it, don't take it the wrong way..."
                                    ])?;
                                    ctx.call(Function::Emotion, vec![ctx.constant("ET_PROFUSELY_SWEAT")?])?;
                                    ctx.next()?;
                                    ctx.lines_as("Engel", args!["^333333*Ahem*^000000", "Just as I've warned you, this attempt to restore the Sobbling Starlight failed because the anvil you gave me wasn't up to the job. I'm sorry about that."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Engel", args!["However, I only", "broke all of the tools", "and the ore pieces are still intact. Now, if you bring me the tools one more time, I can try this again."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Engel", args!["Since I'm a bit more familiar with this material, I won't take so much time. Once again, let me tell you which things you need to bring..."])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Engel",
                                        args![
                                            "^FF00005 Mini Furnace^000000,",
                                            "^FF00002 Oridecon Hammer^000000,",
                                            "and ^ff00001 good quality Anvil^000000.",
                                            "And this time, bring",
                                            "me a better anvil, huh?"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Engel", args!["Although it'd be perfect if I had an Emperium Anvil, I'm pretty sure it'll be alright if we used a Golden Anvil."])?;
                                    ctx.var("sign_q").set(Val::from(70))?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else if subject4 == 68 {
                                    ctx.lines(args![
                                        "Oh good, you're back.",
                                        "I've got some bad news,",
                                        "so please don't overreact~",
                                        "Um, are you ready?"
                                    ])?;
                                    ctx.call(Function::Emotion, vec![ctx.constant("ET_PROFUSELY_SWEAT")?])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Engel",
                                        args![
                                            "While I was trying to restore the ",
                                            "Sobbling Starlight, I was attacked ",
                                            "by monsters all of a sudden. If only",
                                            "I paid more attention to my",
                                            "surroundings..."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Engel", args!["Please understand that I get very deeply engrossed in my smithing work. If you don't mind, I'll try this again you'll bring all the tools just like the last time."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Engel", args!["^FF00005 Mini Furnace^000000 and", "^FF00002 Oridecon Hammer^000000.", "Don't forget the ^FF0000Golden Anvil^000000! But if you can get your hands on one, an ^FF0000Emperium Anvil^000000 would be better."])?;
                                    ctx.var("sign_q").set(Val::from(70))?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                            }
                        } else {
                            if ctx.var("sign_q").get()? == 70 {
                                if ((ctx.call(Function::CountItem, vec![Val::from(612)])?.number()? > 4
                                    && ctx.call(Function::CountItem, vec![Val::from(615)])?.number()? > 1)
                                    && (((ctx.call(Function::CountItem, vec![Val::from(986)])?.number()? > 0
                                        || ctx.call(Function::CountItem, vec![Val::from(987)])?.number()? > 0)
                                        || ctx.call(Function::CountItem, vec![Val::from(988)])?.number()? > 0)
                                        || ctx.call(Function::CountItem, vec![Val::from(989)])?.number()? > 0))
                                {
                                    if (ctx.call(Function::CountItem, vec![Val::from(986)])?.number()? > 0
                                        || ctx.call(Function::CountItem, vec![Val::from(987)])?.number()? > 0)
                                    {
                                        ctx.mes("Didn't I tell you to bring me a Golden Anvil or an Emperium Anvil? Anything less isn't good enough to do any restoration work on this Sobbing Starlight.")?;
                                        ctx.next()?;
                                        ctx.lines_as("Engel", args!["Next time you come", "here, bring a Golden or Emperium Anvil and make sure to leave all of your other anvils in Kafra Storage or something, okay?"])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if ctx.call(Function::CountItem, vec![Val::from(988)])?.number()? > 0 {
                                        ctx.lines(args!["Good, good.", "I see that you've brought a Golden Anvil. Now I can get started right away! Still, I'm a little about those monsters showing up again, so would you keep a lookout?"])?;
                                    } else if ctx.call(Function::CountItem, vec![Val::from(989)])?.number()? > 0 {
                                        ctx.mes("Excellent! You've brought an Emperium Anvil! Now, keep a lookout for monsters while I repair these ore fragments. We can't have those beasts ruin my work again, right? Thank you.")?;
                                    }
                                    ctx.next()?;
                                    ctx.lines(args![
                                        "^3355FF*Clink*",
                                        "*Clink Clink Clink*",
                                        "*Clink Clink Clink*",
                                        "*Cliiiiiiiiiiiiink*^000000"
                                    ])?;
                                    ctx.next()?;
                                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_REPAIRWEAPON")?])?;
                                    ctx.next()?;
                                    ctx.lines(args![
                                        "^3355FF*Clink*",
                                        "*Clink-Clink*",
                                        "*Clink-Clink-Clink*",
                                        "*Claaaaaaaaaaack*^000000"
                                    ])?;
                                    ctx.next()?;
                                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_REPAIRWEAPON")?])?;
                                    ctx.next()?;
                                    ctx.lines_as("Engel", args!["It's almost done.", "Just be a bit more patient..."])?;
                                    ctx.next()?;
                                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_REPAIRWEAPON")?])?;
                                    ctx.next()?;
                                    ctx.lines(args!["^3355FF*Tink Tink*^000000", " ", "...*Tonk*^000000"])?;
                                    ctx.next()?;
                                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_REPAIRWEAPON")?])?;
                                    ctx.next()?;
                                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_FLASHER")?])?;
                                    ctx.next()?;
                                    ctx.lines_as("Engel", args!["Finally... Completed.", "^333333*Phew!*^000000 That was one of the toughest jobs I've ever done, but this was well worth all of our efforts. Just look at the dazzling beauty of this Sobbing Starlight!"])?;
                                    ctx.next()?;
                                    ctx.lines_as("Engel", args!["We've both gone through a lot of trouble to make this, so I hope you treasure your Sobbing Starlight. Good luck on your adventures..."])?;
                                    if ctx.call(Function::CountItem, vec![Val::from(988)])?.number()? > 0 {
                                        ctx.call(Function::DelItem, vec![Val::from(988), Val::from(1)])?;
                                    } else if ctx.call(Function::CountItem, vec![Val::from(989)])?.number()? > 0 {
                                        ctx.call(Function::DelItem, vec![Val::from(989), Val::from(1)])?;
                                    }
                                    ctx.var("sign_q").set(Val::from(71))?;
                                    ctx.call(Function::GetItem, vec![Val::from(7178), Val::from(1)])?;
                                    ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_YUFITELHIT")?])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else if ctx.call(Function::Rand, vec![Val::from(1), Val::from(6)])? == 3 {
                                    ctx.mes("Oh, you don't have everything ready yet? Take your time, so long as you didn't forget what you needed to bring. You do remember, right?")?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Engel",
                                        args![
                                            "^FF00005 Mini Furnace^000000,",
                                            "^FF00002 Oridecon Hammer^000000,",
                                            "and ^ff00001 good quality Anvil^000000.",
                                            "That's all you need!"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Engel", args!["Remember that I have a greater chance of repairing the Sobbing Starlight if I have access to a higher quality Anvil. Otherwise..."])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    ctx.mes("Still don't have everything ready? That's fine, just take your time. So long as you haven't forgotten all the things that you need. I mean, how can you forget when you have such an important ore on you?")?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                            } else {
                                if ctx.var("sign_q").get()?.number()? < 139 {
                                    ctx.mes("How have you been doing lately? Feel free to ask me if you ever think that you need my expertise, alright?")?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    if ctx.var("sign_q").get()? == 139 {
                                        ctx.lines(args![
                                            "How have you been",
                                            "doing lately? You seem",
                                            "quite well. So is there anything",
                                            "I can help you with today?"
                                        ])?;
                                        ctx.next()?;
                                        let choice = runtime::select_values(ctx, &[Val::from("I need your help again.")])?;
                                        ctx.var("@menu").set(choice)?;
                                        ctx.lines_as(
                                            "Engel",
                                            args![
                                                "Haha, so what have",
                                                "you brought me this time?",
                                                "It must be fairly important if you've brought it all the way",
                                                "down here."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        let choice = runtime::select_values(ctx, &[Val::from("Please look at this.")])?;
                                        ctx.var("@menu").set(choice)?;
                                        ctx.lines_as(
                                            "Engel",
                                            args![
                                                "Isn't this...?!",
                                                "Amazing, just by looking at it,",
                                                "I can feel its power! You really",
                                                "are something! How do you come across all of this amazing stuff?"
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("Engel", args!["You know, I consider myself", "lucky to even see this kind of stuff. Most smiths might be able to see this kind of thing only once in their entire lives. And to be able to work on it..."])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Engel",
                                            args![
                                                "Hmmm...",
                                                "It's going to take quite",
                                                "a while to work on this. Unfortunately, I can't even",
                                                "give you a time estimate...",
                                                "But I'll do my best for you."
                                            ],
                                        )?;
                                        ctx.call(Function::DelItem, vec![Val::from(7314), Val::from(1)])?;
                                        ctx.var("sign_q").set(Val::from(140))?;
                                        l_stime_e = ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?;
                                        if l_stime_e.clone().number()? < 2 {
                                            ctx.var("sign_sq").set(Val::from(1))?;
                                        } else {
                                            if l_stime_e.clone().number()? < 4 {
                                                ctx.var("sign_sq").set(Val::from(2))?;
                                            } else {
                                                if l_stime_e.clone().number()? < 6 {
                                                    ctx.var("sign_sq").set(Val::from(3))?;
                                                } else {
                                                    if l_stime_e.clone().number()? < 8 {
                                                        ctx.var("sign_sq").set(Val::from(4))?;
                                                    } else {
                                                        if l_stime_e.clone().number()? < 10 {
                                                            ctx.var("sign_sq").set(Val::from(5))?;
                                                        } else {
                                                            if l_stime_e.clone().number()? < 12 {
                                                                ctx.var("sign_sq").set(Val::from(6))?;
                                                            } else {
                                                                if l_stime_e.clone().number()? < 14 {
                                                                    ctx.var("sign_sq").set(Val::from(7))?;
                                                                } else if l_stime_e.clone().number()? < 16 {
                                                                    ctx.var("sign_sq").set(Val::from(8))?;
                                                                } else if l_stime_e.clone().number()? < 18 {
                                                                    ctx.var("sign_sq").set(Val::from(9))?;
                                                                } else if l_stime_e.clone().number()? < 20 {
                                                                    ctx.var("sign_sq").set(Val::from(10))?;
                                                                } else if l_stime_e.clone().number()? < 22 {
                                                                    ctx.var("sign_sq").set(Val::from(11))?;
                                                                } else {
                                                                    ctx.var("sign_sq").set(Val::from(12))?;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else {
                                        if ctx.var("sign_q").get()? == 140 {
                                            l_stime_e1 = ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?;
                                            if l_stime_e1.clone().number()? < 2 {
                                                if ctx.var("sign_sq").get()? == 11 {
                                                    l_pass_s1 = Val::from(1);
                                                }
                                            } else {
                                                if l_stime_e1.clone().number()? < 4 {
                                                    if ctx.var("sign_sq").get()? == 12 {
                                                        l_pass_s1 = Val::from(1);
                                                    }
                                                } else {
                                                    if l_stime_e1.clone().number()? < 6 {
                                                        if ctx.var("sign_sq").get()? == 1 {
                                                            l_pass_s1 = Val::from(1);
                                                        }
                                                    } else {
                                                        if l_stime_e1.clone().number()? < 8 {
                                                            if ctx.var("sign_sq").get()? == 2 {
                                                                l_pass_s1 = Val::from(1);
                                                            }
                                                        } else {
                                                            if l_stime_e1.clone().number()? < 10 {
                                                                if ctx.var("sign_sq").get()? == 3 {
                                                                    l_pass_s1 = Val::from(1);
                                                                }
                                                            } else {
                                                                if l_stime_e1.clone().number()? < 12 {
                                                                    if ctx.var("sign_sq").get()? == 4 {
                                                                        l_pass_s1 = Val::from(1);
                                                                    }
                                                                } else {
                                                                    if l_stime_e1.clone().number()? < 14 {
                                                                        if ctx.var("sign_sq").get()? == 5 {
                                                                            l_pass_s1 = Val::from(1);
                                                                        }
                                                                    } else {
                                                                        if l_stime_e1.clone().number()? < 16 {
                                                                            if ctx.var("sign_sq").get()? == 6 {
                                                                                l_pass_s1 = Val::from(1);
                                                                            }
                                                                        } else if l_stime_e1.clone().number()? < 18 {
                                                                            if ctx.var("sign_sq").get()? == 7 {
                                                                                l_pass_s1 = Val::from(1);
                                                                            }
                                                                        } else if l_stime_e1.clone().number()? < 20 {
                                                                            if ctx.var("sign_sq").get()? == 8 {
                                                                                l_pass_s1 = Val::from(1);
                                                                            }
                                                                        } else if l_stime_e1.clone().number()? < 22 {
                                                                            if ctx.var("sign_sq").get()? == 9 {
                                                                                l_pass_s1 = Val::from(1);
                                                                            }
                                                                        } else if ctx.var("sign_sq").get()? == 10 {
                                                                            l_pass_s1 = Val::from(1);
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            if l_pass_s1.clone() == 1 {
                                                ctx.lines(args![
                                                    "It's so...",
                                                    "Beautiful! I don't know",
                                                    "if I'll ever see anything",
                                                    "like this again in my life..."
                                                ])?;
                                                ctx.next()?;
                                                ctx.lines_as("Engel", args!["Oh, you're back just in time.", "Although it was one of my most difficult jobs, I believe you'll be pleased with my work. Only a few smiths are privileged enough to work with this, even just once."])?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Engel",
                                                    args![
                                                        "This is now yours to keep.",
                                                        "Thank you for giving me the",
                                                        "honor of working on a worthy",
                                                        "smithing challenge."
                                                    ],
                                                )?;
                                                ctx.var("sign_q").set(Val::from(141))?;
                                                ctx.call(Function::GetItem, vec![Val::from(2644), Val::from(1)])?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            } else {
                                                ctx.mes("I haven't completed it yet, but please understand that I've got to be really careful when working with something so valuable. But don't worry, its inner power will be revealed when I'm finished.")?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                        } else {
                                            ctx.lines(args!["Sometimes, you can only", "improve yourself by training in solitude. But don't ever forget about the ones who really care about you. It's those people who make everything worth it."])?;
                                            ctx.close_window()?;
                                            ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
                                            return Err(Stop::End);
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
    Ok(Val::from(0))
}

pub fn flaming_spirit_man(ctx: &Ctx) -> Script {
    flaming_spirit_man_body(ctx, Vec::new()).map(|_| ())
}

fn annoyed_man_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input_s = Val::from("");
    shared::quests_the_sign_quest::f_updatesignvars(ctx, vec![])?;
    ctx.mes("[Dhota]")?;
    if ctx.var("sign_q").get()?.number()? < 72 {
        ctx.lines(args![
            "Hmmm...",
            "That can't be right...",
            "What could possibly be",
            "the answer I'm looking",
            "for? ^333333*Sigh...*^000000"
        ])?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_HNG")?])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("sign_q").get()? == 72 {
        if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_MAGE")?) {
            ctx.lines(args!["Wah~", "Why the hell did he even go there...!"])?;
        } else {
            ctx.lines(args![
                "You're not even a Mage,",
                "much less a Wizard, Sage, Warlock or a Sorcerer.",
                "Why bother climbing this tower?"
            ])?;
        }
        ctx.mes("What are you doing here?!")?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_HNG")?])?;
        ctx.next()?;
        let (input, status) = runtime::input_text(ctx, None, None)?;
        l_input_s = input;
        if (l_input_s.clone() == "Metz Brayde" || l_input_s.clone() == "Sobbing Starlight") {
            ctx.lines_as("Dhota", args!["Eh?"])?;
            if l_input_s.clone() == "Metz Brayde" {
                ctx.lines(args!["Did you just say you're", "here for Metz Brayde?"])?;
            }
            if l_input_s.clone() == "Sobbing Starlight" {
                ctx.mes("For that Sobbing Starlight, is that right?")?;
            }
            ctx.next()?;
            ctx.lines_as("Dhota", args!["..."])?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
            ctx.next()?;
            ctx.lines_as("Dhota", args!["...", "......"])?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
            ctx.next()?;
            ctx.lines_as("Dhota", args!["...", "......", "BWAAAAH~!"])?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_ANGER")?])?;
            ctx.next()?;
            ctx.lines_as(
                "Dhota",
                args![
                    "Right, he wants that guy",
                    "who's good at working with",
                    "gems and stuff. Eh, he's not",
                    "here right now. Went somewhere",
                    "near Comodo to investigate some",
                    "tribal people or whatever."
                ],
            )?;
            ctx.var("sign_q").set(Val::from(73))?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_HNG")?])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Dhota",
                args![
                    "Say what...?",
                    "I have no idea what",
                    "you're talking about!",
                    "If you've got nothing",
                    "to say, then leave me alone!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        ctx.lines(args![
            "Hmmm...",
            "That can't be right...",
            "What could possibly be",
            "the answer I'm looking",
            "for? ^333333*Sigh...*^000000"
        ])?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_HNG")?])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn annoyed_man(ctx: &Ctx) -> Script {
    annoyed_man_body(ctx, Vec::new()).map(|_| ())
}

fn native_s_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    shared::quests_the_sign_quest::f_updatesignvars(ctx, vec![])?;
    if ctx.var("event_umbala").get()?.number()? >= 3 {
        ctx.mes("[Laotan]")?;
        if ctx.var("sign_q").get()?.number()? < 73 {
            ctx.lines(args![
                "Oh, I wish I had",
                "a Mr. Smile mask!",
                "But where can I get",
                "one of those? Well, I'd",
                "be happy if I had some",
                "Meat to eat, I guess!"
            ])?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_DELIGHT")?])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("sign_q").get()? == 73 {
            ctx.lines(args!["Mmm...?", "A new guy in our village?", "I think I know him! Yea, I do!"])?;
            ctx.next()?;
            ctx.mes("[Laotan]")?;
            if ctx.call(Function::CountItem, vec![Val::from(2278)])?.number()? > 0 {
                ctx.lines(args![
                    "Oh my gosh!",
                    "You've got a Mr. Smile mask!",
                    "Would you give that to me?",
                    "Pretty please...?"
                ])?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Sure~:^FF0000No.^000000")])? {
                    1 => {
                        ctx.lines_as(
                            "Laotan",
                            args!["Hooooray!", "Thank you thank you", "thank you thank you", "soooooooooo much!"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Laotan", args!["Oh right~!", "The weird man in the funny clothes was in our village for a while, but ever since he went into that big tree, nobody's seen him!"])?;
                        ctx.call(Function::DelItem, vec![Val::from(2278), Val::from(1)])?;
                        ctx.var("sign_q").set(Val::from(74))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as("Laotan", args!["I...", "I was...", "I just...", "Waaaaaaahhhh!"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            } else {
                ctx.lines(args!["Waaaait.", "I knooow~!", "I know if you get", "me a Mr. Smile, okay?"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else {
            ctx.lines(args![
                "I love my Mr. Smile mask",
                "soooo much! I'm gonna show",
                "it to all my friends! Thank you",
                "so much again! You're really",
                "really nice, you know that?"
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        ctx.lines_as(
            "??????",
            args![
                "Chuuuba?",
                "Chu-chu-chu-chu-chaba?",
                "Oom oom oom daba. Blip blip?",
                "Sabaloo koombah Solo. Ho ho~"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn native_s(ctx: &Ctx) -> Script {
    native_s_body(ctx, Vec::new()).map(|_| ())
}

fn fastidious_looking_guy_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    shared::quests_the_sign_quest::f_updatesignvars(ctx, vec![])?;
    if ctx.call(Function::CheckWeight, vec![Val::from(908), Val::from(200)])? == 0 {
        ctx.lines(args![
            "^3355FFWait a second! Right now,",
            "you have too many items in your inventory. Please come back after you've freed up more inventory space.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("sign_q").get()?.number()? < 74 {
        ctx.lines(args![
            "^333333*Giggle*^000000",
            "So if I do this,",
            "then that and then...",
            "Ooh, these calculations",
            "are absolutely perfect!"
        ])?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_KIK")?])?;
        ctx.next()?;
        ctx.lines_as(
            "Cyon",
            args![
                "Wh-what?!",
                "Who are you?",
                "H-how did you get",
                "in here? I demand",
                "that you leave, right now!"
            ],
        )?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_FRET")?])?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("umbala"), Val::from(111), Val::from(121)])?;
        return Err(Stop::End);
    } else if ctx.var("sign_q").get()? == 74 {
        ctx.lines_as(
            "Cyon",
            args![
                "^333333*Giggle*^000000",
                "So if I do this,",
                "then that and then...",
                "Ooh, these calculations",
                "are absolutely perfect!"
            ],
        )?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_KIK")?])?;
        ctx.next()?;
        ctx.lines_as(
            "Cyon",
            args![
                "Wh-what?!",
                "Who are you?",
                "H-how did you get",
                "in here? I demand",
                "that you leave, right now!"
            ],
        )?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_FRET")?])?;
        ctx.next()?;
        match runtime::select_values(
            ctx,
            &[Val::from("I'm here for Metz Brayde.:My apologies.:Would you look at this?:...")],
        )? {
            1 => {
                ctx.lines_as(
                    "Cyon",
                    args![
                        "Huh? Brayde?",
                        "Do you expect me",
                        "to just trust you without",
                        "a single shred of proof that",
                        "he sent you? Get out now!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Cyon",
                    args![
                        "If you're so sorry,",
                        "then hurry up and get",
                        "out of here! Not just anyone",
                        "is allowed here! Leave!"
                    ],
                )?;
                ctx.call(Function::Emotion, vec![ctx.constant("ET_HNG")?])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            3 => {
                ctx.lines_as(
                    "Cyon",
                    args![
                        "What could be so",
                        "special about what",
                        "you've brought here?!",
                        "Fine, you've piqued my",
                        "scientific curiosity..."
                    ],
                )?;
                ctx.next()?;
                ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
                ctx.next()?;
                ctx.lines_as(
                    "Cyon",
                    args![
                        "Sweet lord...",
                        "This is 'God's Tear Drop!'",
                        "The Sobbling Starlight!",
                        "He finally made it...."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Cyon",
                    args![
                        "^333333*Giggle*^000000",
                        "Well, since he's kept his promise, I suppose that I should keep mine",
                        "as well. Now, within this Sobbing Starlight are these tiny letters... ^FFFFFFcobo^000000"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Cyon", args!["The text is so small that even Hunters skilled in Vulture's Eye are unable to read them. However, I've read that this Sobbing", "Starlight will respond", "to really old papers..."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Cyon",
                    args![
                        "Now, if we want to figure out",
                        "what's written in the Sobbing",
                        "Starlight, we'd need at least",
                        "10 ancient pieces of paper..."
                    ],
                )?;
                ctx.var("sign_q").set(Val::from(75))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            4 => {
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_VENOMDUST")?])?;
                ctx.lines_as(
                    "Cyon",
                    args![
                        "Not listening, eh?",
                        "Fine, if you don't",
                        "understand words, then",
                        "I'll have to use force!",
                        "Get out here right now!"
                    ],
                )?;
                ctx.close_window()?;
                ctx.call(Function::PercentHeal, vec![Val::from(-30), Val::from(0)])?;
                ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_MAGNUMBREAK")?])?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if ctx.var("sign_q").get()? == 75 {
        ctx.lines_as(
            "Cyon",
            args![
                "You've come back.",
                "I hope you were able",
                "to find some ancient",
                "or really aged paper..."
            ],
        )?;
        ctx.next()?;
        if ctx.call(Function::CountItem, vec![Val::from(1097)])?.number()? > 9 {
            ctx.lines_as(
                "Cyon",
                args![
                    "How did you find these",
                    "Worn Out Pages? This is great,",
                    "I'm sure the Sobbing Starlight",
                    "will respond to these!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Cyon",
                args![
                    "Yes. Yes...",
                    "This should be enough.",
                    "Now give them to me",
                    "and wait here while",
                    "I try a few things..."
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_SPHERE")?])?;
            ctx.next()?;
            ctx.lines_as("Cyon", args!["Just as I thought!", "The Sobbing Starlight is responding to these Worn Out Pages! Excellent! I've finally fulfilled my promise to Metz! Now he can leave", "me alone!"])?;
            ctx.call(Function::DelItem, vec![Val::from(1097), Val::from(10)])?;
            ctx.var("sign_q").set(Val::from(76))?;
            ctx.call(Function::GetItem, vec![Val::from(7275), Val::from(1)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.call(Function::CountItem, vec![Val::from(1097)])?.number()? > 0 {
            ctx.lines_as(
                "Cyon",
                args![
                    "How did you find these",
                    "^FF0000Worn Out Pages^000000? This is great,",
                    "I'm sure the Sobbing Starlight",
                    "will respond to these!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Cyon",
                args![
                    "Wait, we don't have enough",
                    "of them to make the Sobbing",
                    "Starlight respond. Find more",
                    "of these papers and bring",
                    "them all to me!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Cyon",
                args![
                    "You haven't found anything?",
                    "What kind of scientist are you?",
                    "Now go out and find some old and ancient papers so that we can learn what's written in the Sobbing Starlight! Go!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Cyon",
                args![
                    "...?",
                    "Why are you still here?",
                    "Leave and do what I told",
                    "you to do, alright?! Don't",
                    "make me get violent..."
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::PercentHeal, vec![Val::from(-30), Val::from(0)])?;
            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_MAGNUMBREAK")?])?;
            ctx.next()?;
            ctx.call(Function::Warp, vec![Val::from("umbala"), Val::from(111), Val::from(121)])?;
            return Err(Stop::End);
        }
    } else if ctx.var("sign_q").get()? == 76 {
        ctx.lines_as("Cyon", args!["My business with you is finished, so you go along on your way now. And tell Metz not to bother me anymore! I'm done doing favors!"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Cyon",
            args![
                "How dare you intrude",
                "on my property?! Now,",
                "get out now before I get",
                "violent! Didn't you hear me?",
                "GET OUT OF HERE!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn fastidious_looking_guy(ctx: &Ctx) -> Script {
    fastidious_looking_guy_body(ctx, Vec::new()).map(|_| ())
}

fn fastidious_old_man_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    shared::quests_the_sign_quest::f_updatesignvars(ctx, vec![])?;
    if ctx.call(Function::CheckWeight, vec![Val::from(908), Val::from(200)])? == 0 {
        ctx.lines(args![
            "^3355FFWait a second! Right now,",
            "you have too many items in your inventory. Please come back after you've freed up more inventory space.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.mes("[Frank]")?;
    if ctx.var("sign_q").get()?.number()? < 77 {
        ctx.lines(args![
            "My back~",
            "It's so sore!",
            "And my eyes are",
            "hurting worse and worse.",
            "^333333*Sigh*^000000 I'm getting old..."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("sign_q").get()? == 77 {
        ctx.lines(args![
            "My vision's getting",
            "blurrier and I get headaches",
            "when I read. I guess this old",
            "man's got no choice but to get",
            "some sort of seeing aid..."
        ])?;
        ctx.var("sign_q").set(Val::from(78))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("sign_q").get()? == 78 {
        if (ctx.call(Function::CountItem, vec![Val::from(2203)])?.number()? > 0
            && ctx.call(Function::CountItem, vec![Val::from(7275)])?.number()? > 0)
        {
            ctx.lines(args![
                "My vision's getting",
                "blurrier and I get headaches",
                "when I read. I guess this old",
                "man's got no choice but to get",
                "some sort of seeing aid..."
            ])?;
            ctx.next()?;
            'b1: {
                let subject1 = Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("Wait, I've got a pair of glasses...:Pass on by.")],
                )?);
                let mut matched1 = false;
                let no_case1 = !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2));
                if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                    matched1 = true;
                }
                if matched1 {
                    ctx.lines_as(
                        "Frank",
                        args![
                            "Oh, you've brought me",
                            "these Glasses? Thanks",
                            "so much, youngster. Now that",
                            "I can see better, what can",
                            "I do for you in return?"
                        ],
                    )?;
                    ctx.next()?;
                    'l2: loop {
                        if !(true) {
                            break 'l2;
                        }
                        'b2: {
                            match runtime::select_values(
                                ctx,
                                &[Val::from(
                                    "About Geffen's Hidden Power:About Sobbing Starlight:Interpret Ancient Document",
                                )],
                            )? {
                                1 => {
                                    ctx.lines_as("Frank", args!["Geffen's hidden power?", "I don't know much about it, but I'm sure that Geffen Tower is more than just a simple building. I'm sure it was created to restrain or contain some powerful force."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Frank", args!["It's even possible that the fountain in front of Geffen Tower also plays the same role, but I've got no hard evidence. Since I'm not too interested in Geffen, I haven't really investigated..."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Frank", args!["Still...", "There's a strong possibility that some enormous power is being sealed beneath the Geffen Tower."])?;
                                    ctx.next()?;
                                }
                                2 => {
                                    ctx.lines_as("Frank", args!["The Sobbing Starlight?", "To the experts, it's known as God's Tear Drop. Now, some believe it was created from the voice of God."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Frank", args!["Of course, it probably isn't made from God's voice or tear drops, but who knows? In any case, it's said that something's written in the Sobbing Starlight. What ever it is, it must be something important..."])?;
                                    ctx.next()?;
                                }
                                3 => {
                                    ctx.lines_as("Frank", args!["I don't believe it!", "This is... I see. You want me to translate this ancient language. Ah, you're rather fortunate. I'm probably the only person in the world who can translate this."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Frank", args!["Hmm, as far as I can tell, this is a sophisticated language spoken by the ancient gods. It's complex and confusing, but I'll do my best for you. Come back to me later, and I'll what I've learned."])?;
                                    ctx.call(Function::DelItem, vec![Val::from(2203), Val::from(1)])?;
                                    ctx.call(Function::DelItem, vec![Val::from(7275), Val::from(1)])?;
                                    ctx.var("sign_q").set(Val::from(79))?;
                                    ctx.close_window()?;
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
                    ctx.lines_as(
                        "Frank",
                        args![
                            "Oooh~",
                            "How will I ever",
                            "be able to continue",
                            "my life's work? I'm",
                            "helpless if I can't read..."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        } else {
            ctx.lines(args![
                "My vision's getting",
                "blurrier and I get headaches",
                "when I read. I guess this old",
                "man's got no choice but to get",
                "some sort of seeing aid..."
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        if ctx.var("sign_q").get()? == 79 {
            if ctx.call(Function::Rand, vec![Val::from(1), Val::from(50)])? == 9 {
                ctx.lines(args![
                    "I've finally completed the translation. It was a challenge deciphering the meaning of some",
                    "of these words. Even translated in our own language, this contents of this text are fairly ambiguous."
                ])?;
                ctx.next()?;
                ctx.lines_as("Frank", args!["Well, I've written down the best translation that I could for you. It's been a long time since I've had this kind of challenge. Thanks, youngster.", "Ha ha ha~"])?;
                ctx.var("sign_q").set(Val::from(80))?;
                ctx.call(Function::GetItem, vec![Val::from(7274), Val::from(1)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines(args!["I'm sorry, but I haven't finished translating this text. It's taking quite a long time since I don't have any reference material for this particular language. Why", "don't you come back later?"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else {
            ctx.lines(args!["There are many things out", "there that defy our understanding of the world and are beyond our imagination. Many fear the unknown, but the truly brave will always seek out the truth."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    Ok(Val::from(0))
}

pub fn fastidious_old_man(ctx: &Ctx) -> Script {
    fastidious_old_man_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ValkyrieWarpStep {
    Start,
    OnTouch,
}

fn valkyrie_warp_run(ctx: &Ctx, mut step: ValkyrieWarpStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ValkyrieWarpStep::Start => {
                shared::quests_the_sign_quest::f_updatesignvars(ctx, vec![])?;
                if (((ctx.var("sign_q").get()? == 80 && ctx.call(Function::CountItem, vec![Val::from(907)])?.number()? > 3)
                    && ctx.call(Function::CountItem, vec![Val::from(953)])?.number()? > 11)
                    && ctx.call(Function::CountItem, vec![Val::from(7013)])?.number()? > 364)
                {
                    ctx.lines(args![
                        "^6B8E23It seems that",
                        "something in the",
                        "vicinity is reacting",
                        "with the Sobbing Starlight",
                        "in your possession. Perhaps",
                        "you can find it nearby...^000000"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                step = ValkyrieWarpStep::OnTouch;
                continue 'machine;
            }
            ValkyrieWarpStep::OnTouch => {
                if (((ctx.var("sign_q").get()? == 80 && ctx.call(Function::CountItem, vec![Val::from(907)])?.number()? > 3)
                    && ctx.call(Function::CountItem, vec![Val::from(953)])?.number()? > 11)
                    && ctx.call(Function::CountItem, vec![Val::from(7013)])?.number()? > 364)
                {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["What the...?", "What's happening?!"],
                    )?;
                    ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_BEGINSPELL6")?])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFFor some reason, in",
                        "this particular spot, the",
                        "Sobbing Starlight is reacting",
                        "with the Resin, Stone Hearts",
                        "and Coral Reefs that you're",
                        "holding. All the objects are",
                        "violently resonating...^000000"
                    ])?;
                    ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_ENHANCE")?])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFAll of a sudden these",
                        "objects emit a bright flash",
                        "of light that envelops your",
                        "entire body, then you gently",
                        "fall into unconsciousness...^000000"
                    ])?;
                    ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_FLASHER")?])?;
                    ctx.next()?;
                    ctx.call(Function::DelItem, vec![Val::from(907), Val::from(4)])?;
                    ctx.call(Function::DelItem, vec![Val::from(953), Val::from(12)])?;
                    ctx.call(Function::DelItem, vec![Val::from(7013), Val::from(365)])?;
                    ctx.var("sign_q").set(Val::from(81))?;
                    ctx.call(Function::Warp, vec![Val::from("himinn"), Val::from(49), Val::from(10)])?;
                    return Err(Stop::End);
                } else if ctx.var("sign_q").get()? == 203 {
                    ctx.lines(args![
                        "^3355FFNothing seems to",
                        "happen when you",
                        "approach this area now.^000000"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("sign_q").get()?.number()? > 80 {
                    ctx.call(Function::Warp, vec![Val::from("himinn"), Val::from(49), Val::from(10)])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn valkyrie_warp(ctx: &Ctx) -> Script {
    valkyrie_warp_run(ctx, ValkyrieWarpStep::Start, Vec::new()).map(|_| ())
}

pub fn valkyrie_warp_ontouch(ctx: &Ctx) -> Script {
    valkyrie_warp_run(ctx, ValkyrieWarpStep::OnTouch, Vec::new()).map(|_| ())
}

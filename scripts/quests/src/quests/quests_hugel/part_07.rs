use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum KurupeStep {
    Start,
    OnTouch,
}

fn kurupe_run(ctx: &Ctx, mut step: KurupeStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_milkreward = Val::from(0);
    'machine: loop {
        match step {
            KurupeStep::Start => {
                if ctx.var("hg_milk").get()? == 1 {
                    ctx.lines_as(
                        "Kurupe",
                        args![
                            "Could Burupu be hiding",
                            "from me again? He's been",
                            "wanting to ditch work more",
                            "and more ever since he wanted",
                            "to become a Swordman. If only",
                            "I could hire someone else..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kurupe",
                        args![
                            "Unfortunately, he's the",
                            "only one I can find that",
                            "can actually milk my cow,",
                            "Booboo! If you can find",
                            "Burupu for me, I'll be sure",
                            "to pay you for your effort."
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Sure.:Sorry, I'm busy.")])? {
                        1 => {
                            ctx.lines_as(
                                "Kurupe",
                                args![
                                    "Thank you, you're a",
                                    "lifesaver! I need to get the",
                                    "milk delivered by tomorrow,",
                                    "so I really need Burupu back!",
                                    "You should be able to find him",
                                    "somewhere just outside of town."
                                ],
                            )?;
                            ctx.var("hg_milk").set(Val::from(2))?;
                            ctx.call(Function::SetQuest, vec![Val::from(12040)])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as(
                                "Kurupe",
                                args![
                                    "N-no...!",
                                    "If I can't find",
                                    "Burupu, then there's no",
                                    "way I'd be able to deliver",
                                    "the milk orders by tomorrow!",
                                    "Great, what am I gonna do?"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else {
                    if (ctx.var("hg_milk").get()?.number()? > 1 && ctx.var("hg_milk").get()?.number()? < 5) {
                        ctx.lines_as(
                            "Kurupe",
                            args![
                                "Burupu, I need you",
                                "to come to work! Maybe",
                                "you can find him practicing",
                                "with his sword right outside",
                                "of town. I hope you can find",
                                "that lazy guy for me..."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if ctx.var("hg_milk").get()? == 5 {
                            ctx.lines_as(
                                "Kurupe",
                                args![
                                    "Burupu is making demands?!",
                                    "I guess I don't have a choice.",
                                    "You wouldn't happen to know",
                                    "how much Rapiers cost, do you?",
                                    "Why does Burupu want to become",
                                    "a Swordman so much, huh?"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            if ctx.var("hg_milk").get()? == 6 {
                                ctx.lines_as(
                                    "Kurupe",
                                    args![
                                        "Oh, you're going to",
                                        "help me milk the cow?",
                                        "Great, great, thanks so",
                                        "much! Please milk Booboo",
                                        "right away, and get me",
                                        "some delicious milk~"
                                    ],
                                )?;
                                ctx.var("hg_milk").set(Val::from(7))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                if ctx.var("hg_milk").get()? == 7 {
                                    ctx.lines_as(
                                        "Kurupe",
                                        args![
                                            "Hmm, how about this?",
                                            "If you can actually milk",
                                            "Booboo the cow, then I'll",
                                            "give you some food. Does",
                                            "that sound fair? Anyway,",
                                            "please get to work soon~"
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    if ctx.var("hg_milk").get()? == 8 {
                                        ctx.lines_as(
                                            "Kurupe",
                                            args![
                                                "Great! Now that you've",
                                                "milked Booboo, I can go",
                                                "and make my deliveries",
                                                "tomorrow! Here's a little",
                                                "something to eat as my way",
                                                "of saying, ''Thanks a lot~''"
                                            ],
                                        )?;
                                        ctx.next()?;
                                        if ctx.call(Function::CheckWeight, vec![Val::from(12063), Val::from(3)])? != 1 {
                                            ctx.lines_as(
                                                "Kurupe",
                                                args![
                                                    "Wait a minute!",
                                                    "Currently you are carrying",
                                                    "too many items with you.",
                                                    "Please come back again",
                                                    "after you store some items into kafra storage."
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        ctx.var("hg_milk").set(Val::from(9))?;
                                        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_CONE")?])?;
                                        ctx.call(Function::GetExperience, vec![Val::from(100000), Val::from(0)])?;
                                        ctx.call(Function::GetItem, vec![Val::from(12063), Val::from(3)])?;
                                        ctx.call(Function::EraseQuest, vec![Val::from(12043)])?;
                                        ctx.lines_as(
                                            "Kurupe",
                                            args![
                                                "I always need",
                                                "someone to help me",
                                                "milk Booboo, especially",
                                                "when Burupu doesn't feel",
                                                "like doing it, so please come",
                                                "by and help me when you can~"
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else {
                                        if ctx.var("hg_milk").get()? == 9 {
                                            ctx.lines_as(
                                                "Kurupe",
                                                args![
                                                    "Oh, it's you again!",
                                                    "Did you want to help me",
                                                    "out by milking Booboo the",
                                                    "cow? I can always use a",
                                                    "dependable person like you."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            match runtime::select_values(
                                                ctx,
                                                &[Val::from(
                                                    "Sure, I'll milk Booboo.:What'll pay me this time?:Nah, just dropping by.",
                                                )],
                                            )? {
                                                1 => {
                                                    ctx.lines_as(
                                                        "Kurupe",
                                                        args![
                                                            "Great, great! If you want",
                                                            "to milk Booboo, first tell",
                                                            "Burupu that he doesn't need",
                                                            "to come in today, and then",
                                                            "come back to me. Then, I'll",
                                                            "let you milk Booboo the cow."
                                                        ],
                                                    )?;
                                                    ctx.var("hg_milk").set(Val::from(10))?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                }
                                                2 => {
                                                    ctx.lines_as(
                                                        "Kurupe",
                                                        args![
                                                            "Let's see...",
                                                            "Milk Booboo for me this",
                                                            "time, and I'll randomly",
                                                            "choose one of the following",
                                                            "sets of rewards to give you."
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines(args![
                                                        "1 set of 5 Milk,",
                                                        "1 set of 3 Honey,",
                                                        "1 set of 5 Orange Potions,",
                                                        "1 set of Yellow Potions, or",
                                                        "1 Bundle of Food. Hmmm...",
                                                        "That sounds fair, right?"
                                                    ])?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                }
                                                3 => {
                                                    ctx.lines_as(
                                                        "Kurupe",
                                                        args![
                                                            "Ahahahah, it's good",
                                                            "to see you! Now, don't",
                                                            "be a stranger, alright?",
                                                            "Burupu could learn a",
                                                            "couple things from you..."
                                                        ],
                                                    )?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                }
                                                _ => {}
                                            }
                                        } else {
                                            if ctx.var("hg_milk").get()? == 10 {
                                                ctx.lines_as(
                                                    "Kurupe",
                                                    args![
                                                        "For now, go talk",
                                                        "to Burupu and let him",
                                                        "know that he doesn't need",
                                                        "to come into work today~"
                                                    ],
                                                )?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            } else {
                                                if ctx.var("hg_milk").get()? == 11 {
                                                    ctx.lines_as(
                                                        "Kurupe",
                                                        args![
                                                            "Ah, you spoke to",
                                                            "Burupu already? Good,",
                                                            "now would you please",
                                                            "milk Booboo the cow?",
                                                            "Thanks once again~"
                                                        ],
                                                    )?;
                                                    ctx.var("hg_milk").set(Val::from(12))?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                } else if ctx.var("hg_milk").get()? == 12 {
                                                    ctx.lines_as(
                                                        "Kurupe",
                                                        args!["Please go and milk", "Booboo the cow as", "soon as you can, okay?"],
                                                    )?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                } else if ctx.var("hg_milk").get()? == 13 {
                                                    ctx.lines_as(
                                                        "Kurupe",
                                                        args![
                                                            "Ah, you've done a good",
                                                            "job of milking Booboo the",
                                                            "cow for me. Let's see, let's",
                                                            "see, what would be good to",
                                                            "give you as payment. Hmm..."
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    l_milkreward = ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?;
                                                    if l_milkreward.clone().number()? < 5 {
                                                        ctx.lines_as(
                                                            "Kurupe",
                                                            args![
                                                                "You know what?",
                                                                "Why don't you have",
                                                                "some Milk? It's only",
                                                                "fitting, after all. Besides,",
                                                                "it's really good for you!",
                                                                "Thanks again for your help~"
                                                            ],
                                                        )?;
                                                        if ctx.call(Function::CheckWeight, vec![Val::from(519), Val::from(5)])? != 1 {
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "Kurupe",
                                                                args![
                                                                    "Wait a minute!",
                                                                    "Currently you are carrying",
                                                                    "too many items with you.",
                                                                    "Please come back again",
                                                                    "after you store some items into kafra storage."
                                                                ],
                                                            )?;
                                                            ctx.close_window()?;
                                                            return Err(Stop::End);
                                                        }
                                                        ctx.var("hg_milk").set(Val::from(9))?;
                                                        ctx.call(Function::EraseQuest, vec![Val::from(12043)])?;
                                                        ctx.call(Function::GetItem, vec![Val::from(519), Val::from(5)])?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    } else if (l_milkreward.clone().number()? > 4 && l_milkreward.clone().number()? < 8) {
                                                        ctx.lines_as(
                                                            "Kurupe",
                                                            args![
                                                                "Ah, I've got it!",
                                                                "You like Orange Potions,",
                                                                "right? Come on, you looove",
                                                                "Orange Potions! Here, you can",
                                                                "have a bunch as my way of saying thanks for milking Booboo the cow~"
                                                            ],
                                                        )?;
                                                        if ctx.call(Function::CheckWeight, vec![Val::from(502), Val::from(5)])? != 1 {
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "Kurupe",
                                                                args![
                                                                    "Wait a minute!",
                                                                    "Currently you are carrying",
                                                                    "too many items with you.",
                                                                    "Please come back again",
                                                                    "after you store some items into kafra storage."
                                                                ],
                                                            )?;
                                                            ctx.close_window()?;
                                                            return Err(Stop::End);
                                                        }
                                                        ctx.var("hg_milk").set(Val::from(9))?;
                                                        ctx.call(Function::EraseQuest, vec![Val::from(12043)])?;
                                                        ctx.call(Function::GetItem, vec![Val::from(502), Val::from(5)])?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    } else if (l_milkreward.clone().number()? > 7 && l_milkreward.clone().number()? < 10) {
                                                        ctx.lines_as(
                                                            "Kurupe",
                                                            args![
                                                                "Oooh, I could give you",
                                                                "some Yellow Potions, Yes,",
                                                                "that's a good idea. Please",
                                                                "take these Yellow Potions",
                                                                "as a token of my gratitude",
                                                                "for milking old Booboo."
                                                            ],
                                                        )?;
                                                        if ctx.call(Function::CheckWeight, vec![Val::from(503), Val::from(5)])? != 1 {
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "Kurupe",
                                                                args![
                                                                    "Wait a minute!",
                                                                    "Currently you are carrying",
                                                                    "too many items with you.",
                                                                    "Please come back again",
                                                                    "after you store some items into kafra storage."
                                                                ],
                                                            )?;
                                                            ctx.close_window()?;
                                                            return Err(Stop::End);
                                                        }
                                                        ctx.var("hg_milk").set(Val::from(9))?;
                                                        ctx.call(Function::EraseQuest, vec![Val::from(12043)])?;
                                                        ctx.call(Function::GetItem, vec![Val::from(503), Val::from(5)])?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    } else {
                                                        l_milkreward = ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?;
                                                        if l_milkreward.clone().number()? < 7 {
                                                            ctx.lines_as(
                                                                "Kurupe",
                                                                args![
                                                                    "Hey, why don't you",
                                                                    "take some fresh Honey?",
                                                                    "Yes, it's sweet, delicious,",
                                                                    "it's everything you could",
                                                                    "ever want! Thanks for",
                                                                    "milking Booboo for me~"
                                                                ],
                                                            )?;
                                                            if ctx.call(Function::CheckWeight, vec![Val::from(518), Val::from(3)])? != 1 {
                                                                ctx.next()?;
                                                                ctx.lines_as(
                                                                    "Kurupe",
                                                                    args![
                                                                        "Wait a minute!",
                                                                        "Currently you are carrying",
                                                                        "too many items with you.",
                                                                        "Please come back again",
                                                                        "after you store some items into kafra storage."
                                                                    ],
                                                                )?;
                                                                ctx.close_window()?;
                                                                return Err(Stop::End);
                                                            }
                                                            ctx.var("hg_milk").set(Val::from(9))?;
                                                            ctx.call(Function::EraseQuest, vec![Val::from(12043)])?;
                                                            ctx.call(Function::GetItem, vec![Val::from(518), Val::from(3)])?;
                                                            ctx.close_window()?;
                                                            return Err(Stop::End);
                                                        }
                                                        ctx.lines_as(
                                                            "Kurupe",
                                                            args![
                                                                "Here, why don't you",
                                                                "have some of this really",
                                                                "delicious food? I don't",
                                                                "remember what I packed in",
                                                                "here, but I'm sure it tastes",
                                                                "good, and it's good for you~"
                                                            ],
                                                        )?;
                                                        if ctx.call(Function::CheckWeight, vec![Val::from(12111), Val::from(1)])? != 1 {
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "Kurupe",
                                                                args![
                                                                    "Wait a minute!",
                                                                    "Currently you are carrying",
                                                                    "too many items with you.",
                                                                    "Please come back again",
                                                                    "after you store some items into kafra storage."
                                                                ],
                                                            )?;
                                                            ctx.close_window()?;
                                                            return Err(Stop::End);
                                                        }
                                                        ctx.var("hg_milk").set(Val::from(9))?;
                                                        ctx.call(Function::EraseQuest, vec![Val::from(12043)])?;
                                                        ctx.call(Function::GetItem, vec![Val::from(12111), Val::from(1)])?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    }
                                                } else {
                                                    ctx.lines_as(
                                                        "Kurupe",
                                                        args![
                                                            "Aww, nuts...!",
                                                            "I've got so many",
                                                            "Milk deliveries to do",
                                                            "tomorrow, but what can",
                                                            "I do without any Milk?!"
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
                        }
                    }
                }
                step = KurupeStep::OnTouch;
                continue 'machine;
            }
            KurupeStep::OnTouch => {
                if ctx.var("BaseLevel").get()?.number()? > 49 && !(ctx.var("hg_milk").get()?.is_true()) {
                    ctx.lines_as(
                        "Kurupe",
                        args![
                            "Oh no, what am I gonna",
                            "do?! I need to deliver the",
                            "milk tomorrow, but Burupu",
                            "still hasn't arrived yet!"
                        ],
                    )?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
                    ctx.next()?;
                    ctx.lines_as("Kurupe", args!["When is he coming", "to work? Burupu,", "where are you?!"])?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_SWEAT")?])?;
                    ctx.var("hg_milk").set(Val::from(1))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Kurupe",
                    args![
                        "Why hasn't Burupu",
                        "arrived yet? If he",
                        "doesn't come soon,",
                        "how will I get the milk",
                        "delivered by tomorrow?"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn kurupe(ctx: &Ctx) -> Script {
    kurupe_run(ctx, KurupeStep::Start, Vec::new()).map(|_| ())
}

pub fn kurupe_ontouch(ctx: &Ctx) -> Script {
    kurupe_run(ctx, KurupeStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum BurupuStep {
    Start,
    OnTouch,
}

fn burupu_run(ctx: &Ctx, mut step: BurupuStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            BurupuStep::Start => {
                if ctx.var("hg_milk").get()? == 3 {
                    ctx.lines_as(
                        "Burupu",
                        args![
                            "What? Kurupe wants me",
                            "to come in to work again?",
                            "Man, does it look like I want",
                            "to milk Booboo all my life?",
                            "Forget that, I'm gonna be",
                            "a Swordman! Tally-hoooe!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Burupu",
                        args![
                            "Yeah, ever since that",
                            "Airship landed in our",
                            "town, and I saw all those",
                            "Swordmen, I knew it'd be",
                            "my destiny! Forget ranching, I'm gonna wield a frickin' sword!"
                        ],
                    )?;
                    ctx.var("hg_milk").set(Val::from(4))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("hg_milk").get()? == 4 {
                    ctx.lines_as(
                        "Burupu",
                        args![
                            "Hey, that equipment of",
                            "yours looks plenty expensive.",
                            "I think I'll need something",
                            "like that if I wanna become",
                            "a Swordman. Hey, I know",
                            "how we can help each other~"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Burupu",
                        args![
                            "I'll help you with milking",
                            "Booboo the cow if you can get",
                            "me some proper Swordman",
                            "equipment. Let's see... Why not",
                            "give me 1 Rapier with 2 Slots? That's not too much to ask, right?"
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Alright.:Whoa, that's too much!")])? {
                        1 => {
                            ctx.lines_as(
                                "Burupu",
                                args![
                                    "I'm glad you agree~",
                                    "Okay then, try to bring",
                                    "me a Rapier with 2 Slots",
                                    "as soon as you can! I can't",
                                    "wait to start training with it!"
                                ],
                            )?;
                            ctx.var("hg_milk").set(Val::from(5))?;
                            ctx.call(Function::ChangeQuest, vec![Val::from(12040), Val::from(12041)])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as(
                                "Burupu",
                                args![
                                    "You think so...?",
                                    "Well, it's the only",
                                    "thing I can think of",
                                    "that I really want..."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else if ctx.var("hg_milk").get()? == 5 {
                    if !(ctx.call(Function::CountItem, vec![Val::from(1110)])?.is_true()) {
                        ctx.lines_as(
                            "Burupu",
                            args![
                                "Hey, weren't you",
                                "supposed to bring me",
                                "a Rapier with 2 Slots?",
                                "If you want me to milk",
                                "Booboo the cow, then keep",
                                "your end of our bargain~"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        "Burupu",
                        args![
                            "Oh, wow...!",
                            "That's such a beautiful",
                            "Rapier! Look, it's got",
                            "2 Slots and everything!",
                            "I'm gonna start training",
                            "with it right now!"
                        ],
                    )?;
                    ctx.call(Function::DelItem, vec![Val::from(1110), Val::from(1)])?;
                    ctx.var("hg_milk").set(Val::from(6))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(12041), Val::from(12042)])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Burupu",
                        args![
                            "You know what?",
                            "I've changed my mind.",
                            "Screw milking that dumb cow!",
                            "Still, a Swordman is supposed",
                            "to keep his promises, huh?",
                            "Okay then, how about this?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Burupu",
                        args![
                            "I hid my secret instructions",
                            "for milking Booboo the cow",
                            "under the ground just northeast",
                            "of her. Read them carefully, and you should be able to milk her",
                            "yourself. You can do it, right?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Burupu",
                        args![
                            "Anyway, don't forget",
                            "to find and read those",
                            "instructions. Oh, and",
                            "thanks again for the Rapier~"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("hg_milk").get()? == 10 {
                    ctx.lines_as(
                        "Burupu",
                        args![
                            "Hmm? You wanna milk",
                            "Booboo today? Great,",
                            "that means I've got more",
                            "time to work on my fencing.",
                            "All right, now I can focus",
                            "completely on my training!"
                        ],
                    )?;
                    ctx.var("hg_milk").set(Val::from(11))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Burupu",
                        args![
                            "I'm gonna become the",
                            "best Swordman in the",
                            "world! But first, I gotta",
                            "learn how to use this thing!",
                            "Heeeeee-YAH! How's that? That was almost a Magnum Break, right?"
                        ],
                    )?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_PROFUSELY_SWEAT")?])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                step = BurupuStep::OnTouch;
                continue 'machine;
            }
            BurupuStep::OnTouch => {
                if ctx.var("hg_milk").get()? == 2 {
                    ctx.lines_as("Burupu", args!["Heeeyah!", "Yaaaaaaaah!", "Wh-whooooooosh!"])?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_PROFUSELY_SWEAT")?])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Burupu",
                        args![
                            "Crap! Why do I have",
                            "to have such a crappy",
                            "sword?! I mean, I'm getting",
                            "tired of making all my own",
                            "whooshing sounds and effects."
                        ],
                    )?;
                    ctx.var("hg_milk").set(Val::from(3))?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_FRET")?])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("hg_milk").get()? == 5 {
                    ctx.lines_as("Burupu", args!["Heeeyah!", "Yaaaaaaaah!", "Yeeeeeeeyoooop!"])?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_PROFUSELY_SWEAT")?])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn burupu(ctx: &Ctx) -> Script {
    burupu_run(ctx, BurupuStep::Start, Vec::new()).map(|_| ())
}

pub fn burupu_ontouch(ctx: &Ctx) -> Script {
    burupu_run(ctx, BurupuStep::OnTouch, Vec::new()).map(|_| ())
}

fn burupu_s_instructions_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("hg_milk").get()?.number()? < 6 {
        ctx.lines(args![
            "^3355FFYou've found a small^000000",
            "^3355FFnotebook on the ground.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines(args![
        "^804000This small notebook^000000",
        "^804000contains Burupu's detailed^000000",
        "^804000instructions for milking^000000",
        "^804000Booboo the cow. Although^000000",
        "^804000it is written by hand, it^000000",
        "^804000is very well organized.^000000"
    ])?;
    ctx.next()?;
    ctx.lines_as(
        "Table of Contents",
        args![
            " ",
            "Chapter 1: How to Milk Booboo",
            "Chapter 2: How to Treat Booboo",
            "Chapter 3: Before You Begin"
        ],
    )?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("Chapter 1:Chapter 2:Chapter 3")])? {
        1 => {
            ctx.lines_as(
                "Chapter 1",
                args![
                    "Booboo is a very emotionally",
                    "sensitive cow that expresses",
                    "herself through the power of",
                    "song. You must listen to her",
                    "song, determine how she feels,",
                    "and then comb her just right."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Chapter 1",
                args![
                    "Basically, depending on",
                    "how Booboo feels, you must",
                    "give her the number of brush",
                    "strokes that correspond to",
                    "her song. I'll explain more",
                    "about that in Chapter 2."
                ],
            )?;
        }
        2 => {
            ctx.lines_as(
                "Chapter 2 Contents",
                args![
                    " ",
                    "Part 1: How to Respond to",
                    "Booboo the Cow's Feelings",
                    "Part 2: Booboo's Songs"
                ],
            )?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Part 1:Part 2")])? {
                1 => {
                    ctx.lines_as(
                        "Chapter 2, Part 1",
                        args![
                            "Booboo the Cow usually",
                            "expresses five different",
                            "feelings through her songs:",
                            "these are joy, sadness, anger,",
                            "love, and neutral contentment."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Chapter 2, Part 1",
                        args![
                            "^800080She is so happy^000000!",
                            "When Booboo the Cow",
                            "feels joy, then you go",
                            "ahead and brush her",
                            "hair 3 times before",
                            "you can milk her.",
                            "^800080 3 times^000000."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Chapter 2, Part 1",
                        args![
                            "^800080She is as usual^000000.",
                            "When she feels so-so, you need to",
                            "comb her hair ^800080 5 times^000000."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Chapter 2, Part 1",
                        args![
                            "If Booboo the cow feels",
                            "sad, then you need to make",
                            "her feel more loved. How",
                            "do you do this? Just brush",
                            "her hair at least 10 times."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Chapter 2, Part 1",
                        args![
                            "Now, remember that when",
                            "Booboo the cow is angry,",
                            "you shouldn't touch her",
                            "at all. Just say, ''No!'' to",
                            "brushing her hair, okay?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Chapter 2, Part 1",
                        args![
                            "If you happen to see",
                            "Booboo the cow in love,",
                            "then just brush her hair",
                            "once. Just look for the",
                            "hearts, and you'll know",
                            "that she's in love, okay?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Chapter 2, Part 1",
                        args![
                            "Whenever Booboo the",
                            "cow is just feeling",
                            "neutral contentment,",
                            "then just brush her",
                            "5 times. That's all~"
                        ],
                    )?;
                }
                2 => {
                    ctx.lines_as(
                        "Chapter 2, Part 2",
                        args![
                            "Booboo the cow will",
                            "always sing this song.",
                            "Keep these sounds in",
                            "mind when she sings them..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^ff0000Brr~ Brrbrr~ Boobooboo~^000000",
                        "^0000ffBrrrrrr~ Booboo~ Boorrboo~^000000",
                        "^ff0000Booruboorubrr~ Rrrrboo~^000000",
                        "^0000ffRrrr~ Booboorrrr~ Boobooru~^000000",
                        "^ff0000Boobooboo~ Rururu~ Booruboorub~^000000",
                        "^0000ffBoo! B! Boo~^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Chapter 2, Part 2",
                        args![
                            "When you try to milk",
                            "Booboo, she will sing this",
                            "song with a slight difference.",
                            "Look for the difference in",
                            "Booboo's song, and then",
                            "hum the correct word to her."
                        ],
                    )?;
                }
                _ => {}
            }
        }
        3 => {
            ctx.lines_as(
                "Chapter 3",
                args![
                    "If you want to milk",
                    "Booboo the cow, you",
                    "must first get permission",
                    "from Kurupe. You will also",
                    "need 3 Concentration Potions",
                    "to insert into Booboo's comb."
                ],
            )?;
        }
        _ => {}
    }
    ctx.close_window()?;
    if ctx.call(Function::CheckQuest, vec![Val::from(12042)])?.number()? > -1 {
        ctx.call(Function::EraseQuest, vec![Val::from(12042)])?;
    }
    return Err(Stop::End);
}

pub fn burupu_s_instructions(ctx: &Ctx) -> Script {
    burupu_s_instructions_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum BoobooTheCowStep {
    Start,
    LMilkCow,
}

fn booboo_the_cow_run(ctx: &Ctx, mut step: BoobooTheCowStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_cowanswer_s = Val::from("");
    let mut l_cowbrush = Val::from(0);
    let mut l_cowsong_s = Val::from("");
    'machine: loop {
        match step {
            BoobooTheCowStep::Start => {
                if (ctx.var("hg_milk").get()? == 7 || ctx.var("hg_milk").get()? == 12) {
                    ctx.lines_as("Booboo", args!["Boop boop boo~", "Booboo Boop boo!"])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^804000It seems that",
                        "Booboo the cow",
                        "has something that",
                        "she wants to tell you.^000000"
                    ])?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Attempt to Milk Booboo:It's not the right time!")])? {
                        1 => {
                            if ctx.call(Function::CountItem, vec![Val::from(645)])?.number()? > 2 {
                                let subject2 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(5)])?;
                                if subject2 == 1 {
                                    booboo_the_cow_run(ctx, BoobooTheCowStep::LMilkCow, vec![Val::from("cow_01.wav"), Val::from(3)])?;
                                } else if subject2 == 2 {
                                    booboo_the_cow_run(ctx, BoobooTheCowStep::LMilkCow, vec![Val::from("cow_02.wav"), Val::from(5)])?;
                                } else if subject2 == 3 {
                                    booboo_the_cow_run(ctx, BoobooTheCowStep::LMilkCow, vec![Val::from("cow_03.wav"), Val::from(10)])?;
                                } else if subject2 == 4 {
                                    booboo_the_cow_run(ctx, BoobooTheCowStep::LMilkCow, vec![Val::from("cow_04.wav"), Val::from(0)])?;
                                } else if subject2 == 5 {
                                    booboo_the_cow_run(ctx, BoobooTheCowStep::LMilkCow, vec![Val::from("cow_05.wav"), Val::from(1)])?;
                                }
                                return Err(Stop::End);
                            }
                            ctx.lines(args![
                                "^3355FFTo use this comb to",
                                "brush Booboo the cow,",
                                "you will need to insert",
                                "3 Concentration Potions.",
                                "You can only brush Booboo",
                                "with this luxurious magic comb.^000000"
                            ])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines(args![
                                "^3355FFMaybe you should learn more",
                                "about Booboo the cow, and",
                                "get more information before",
                                "you can attempt to milk her.^000000"
                            ])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                }
                ctx.lines_as("Booboo", args!["Booo~Boooo!!", "Boobooboo Booo~~~", "Booboo Boop~"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            BoobooTheCowStep::LMilkCow => {
                ctx.lines(args![
                    "^3355FFThis giant magic comb was",
                    "specially ordered from Geffen's",
                    "Magic Academy. It is designed",
                    "to operate after inserting",
                    "3 Concentration Potions",
                    "into its special slots.^000000"
                ])?;
                ctx.next()?;
                ctx.lines(args![
                    "^3355FFOnce you place your",
                    "3 Concentration Potions",
                    "into the comb, it begins",
                    "to chime, and Booboo the",
                    "cow beings to express her^FFFFFF ^3355FF feelings in the center of Hugel.^000000"
                ])?;
                ctx.call(Function::DelItem, vec![Val::from(645), Val::from(3)])?;
                ctx.call(
                    Function::SoundEffect,
                    vec![
                        ((Val::from("") + runtime::arg(&args, 0, Val::from(0))) + Val::from("")),
                        Val::from(0),
                    ],
                )?;
                ctx.next()?;
                ctx.call(
                    Function::SoundEffect,
                    vec![
                        ((Val::from("") + runtime::arg(&args, 0, Val::from(0))) + Val::from("")),
                        Val::from(0),
                    ],
                )?;
                ctx.lines(args![
                    "^3355FFNow it's time for you",
                    "to brush Booboo the cow.",
                    "Depending on her mood,",
                    "you need to brush her a",
                    "certain number of times.^000000"
                ])?;
                ctx.next()?;
                let (input, status) = runtime::input_number(ctx, None, None)?;
                l_cowbrush = input;
                if !l_cowbrush.clone().loosely_equals(&runtime::arg(&args, 1, Val::from(0))) {
                    ctx.call(Function::SoundEffect, vec![Val::from("taming_fail.wav"), Val::from(0)])?;
                    ctx.lines(args![
                        "^3355FFAwwww...",
                        "Booboo the cow looks",
                        "so disappointed. She",
                        "looks like she wants",
                        "to close herself off",
                        "from the rest of the world...^000000"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.call(Function::SoundEffect, vec![Val::from("cow_06.wav"), Val::from(0)])?;
                ctx.lines(args![
                    "^3355FFBooboo the cow",
                    "seems fairly content,",
                    "and is singing a serenade.^000000"
                ])?;
                ctx.next()?;
                ctx.call(Function::SoundEffect, vec![Val::from("cow_06.wav"), Val::from(0)])?;
                ctx.mes("[Mrs. Booboo]")?;
                let subject3 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(6)])?;
                if subject3 == 1 {
                    l_cowsong_s = Val::from("Brrbrr");
                    ctx.lines(args![
                        "^ff0000Brr~ Brrboo~ Boobooboo~^000000",
                        "^0000ffBrrrrrr~ Booboo~ Boorrboo~^000000",
                        "^ff0000Booruboorubrr~ Rrrrboo~^000000",
                        "^0000ffRrrr~ Booboorrrr~ Boobooru~^000000",
                        "^ff0000Boobooboo~ Rururu~ Booruboorub~^000000",
                        "^0000ffBoo! B! Boo~~~^000000"
                    ])?;
                } else if subject3 == 2 {
                    l_cowsong_s = Val::from("Brrrrrr");
                    ctx.lines(args![
                        "^ff0000Brr~ Brrboo~ Boobooboo~^000000",
                        "^0000ffRrrururu~ Booboo~ Boorrboo~^000000",
                        "^ff0000Booruboorubrr~ Rrrrboo~^000000",
                        "^0000ffRrrr~ Booboorrrr~ Boobooru~^000000",
                        "^ff0000Boobooboo~ Rururu~ Booruboorub~^000000",
                        "^0000ffBoo! B! Boo~~~^000000"
                    ])?;
                } else if subject3 == 3 {
                    l_cowsong_s = Val::from("Rrrrboo");
                    ctx.lines(args![
                        "^ff0000Brr~ Brrbrr~ Boobooboo~^000000",
                        "^0000ffBrrrrrr~ Booboo~ Boorrboo~^000000",
                        "^ff0000Booruboorubrr~ Rurub~^000000",
                        "^0000ffRrrr~ Booboorrrr~ Boobooru~^000000",
                        "^ff0000Boobooboo~ Rururu~ Booruboorub~^000000",
                        "^0000ffBoo! B! Boo~^000000"
                    ])?;
                } else if subject3 == 4 {
                    l_cowsong_s = Val::from("Boobooru");
                    ctx.lines(args![
                        "^ff0000Brr~ Brrbrr~ Boobooboo~^000000",
                        "^0000ffBrrrrrr~ Booboo~ Boorrboo~^000000",
                        "^ff0000Booruboorubrr~ Rrrrboo~^000000",
                        "^0000ffRrrr~ Booboorrrr~ Bbrrrr~^000000",
                        "^ff0000Boobooboo~ Rururu~ Booruboorub~^000000",
                        "^0000ffBoo! B! Boo~^000000"
                    ])?;
                } else if subject3 == 5 {
                    l_cowsong_s = Val::from("Boobooboo");
                    ctx.lines(args![
                        "^ff0000Brr~ Brrbrr~ Boobooboo~^000000",
                        "^0000ffBrrrrrr~ Booboo~ Boorrboo~^000000",
                        "^ff0000Booruboorubrr~ Rrrrboo~^000000",
                        "^0000ffRrrr~ Booboorrrr~ Boobooru~^000000",
                        "^ff0000Bbb~ Rururu~ Booruboorub~^000000",
                        "^0000ffBoo! B! Boo~^000000"
                    ])?;
                } else if subject3 == 6 {
                    l_cowsong_s = Val::from("B");
                    ctx.lines(args![
                        "^ff0000Brr~ Brrbrr~ Boobooboo~^000000",
                        "^0000ffBrrrrrr~ Booboo~ Boorrboo~^000000",
                        "^ff0000Booruboorubrr~ Rrrrboo~^000000",
                        "^0000ffRrrr~ Booboorrrr~ Boobooru~^000000",
                        "^ff0000Boobooboo~ Rururu~ Booruboorub~^000000",
                        "^0000ffBoo! Boo! Boo~^000000"
                    ])?;
                }
                ctx.next()?;
                ctx.lines(args![
                    "^3355FFHurry, respond to Booboo's song! Look for the word that is slightly",
                    "different than Booboo's normal",
                    "song in Burupu's notes, and then tell her the correct world without",
                    "the tilde character (''~'').^000000"
                ])?;
                ctx.next()?;
                let (input, status) = runtime::input_text(ctx, None, None)?;
                l_cowanswer_s = input;
                if l_cowanswer_s.clone().loosely_equals(&l_cowsong_s.clone()) {
                    ctx.call(Function::SoundEffect, vec![Val::from("tming_success.wav"), Val::from(0)])?;
                    ctx.lines(args![
                        "^3355FFSuccess!",
                        "Booboo the cow",
                        "is quite happy, and",
                        "you were able to milk",
                        "her. Now, you should",
                        "report to Kurupe.^000000"
                    ])?;
                    if ctx.var("hg_milk").get()? == 12 {
                        ctx.var("hg_milk").set(Val::from(13))?;
                    } else {
                        ctx.var("hg_milk").set(Val::from(8))?;
                    }
                    ctx.call(Function::SetQuest, vec![Val::from(12043)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.call(Function::SoundEffect, vec![Val::from("taming_fail.wav"), Val::from(0)])?;
                ctx.lines(args![
                    "^3355FFUh oh...",
                    "Booboo the cow is starting",
                    "to snort violently! She seems",
                    "pretty angry with you, so you",
                    "better get away for now...^000000"
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn booboo_the_cow(ctx: &Ctx) -> Script {
    booboo_the_cow_run(ctx, BoobooTheCowStep::Start, Vec::new()).map(|_| ())
}

fn sign_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args!["**Recruitment Notice**", " "])?;
    if ctx.var("hg_odin").get()?.number()? < 60 {
        ctx.lines(args![
            "We are now hiring recruits",
            "for the Odin Shrine Expedition.",
            " ",
            "- Shrine Expedition Dept."
        ])?;
        if !(ctx.var("hg_odin").get()?.is_true()) {
            ctx.var("hg_odin").set(Val::from(1))?;
        }
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines(args![
        "Join our magician",
        "community, and make",
        "all your days a festival!",
        " ",
        "- Hugel Magician Community"
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn sign(ctx: &Ctx) -> Script {
    sign_body(ctx, Vec::new()).map(|_| ())
}

fn alex_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if !(ctx.var("hg_odin").get()?.is_true()) {
        ctx.lines_as(
            "Alex",
            args![
                "Whatever you're",
                "trying to sell me,",
                "I'm not interested!",
                "Now get out of here!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("hg_odin").get()? == 1 {
            ctx.call(Function::Cutin, vec![Val::from("hu_alex01.bmp"), Val::from(2)])?;
            ctx.lines_as("Alex", args!["Huh...?", "What do you want?"])?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Excuse me...:I saw the recruitment notice and...")])? {
                1 => {
                    ctx.lines_as(
                        "Alex",
                        args![
                            "Whatever you're",
                            "trying to sell me,",
                            "I'm not interested!",
                            "Now get out of here!"
                        ],
                    )?;
                    ctx.close_window()?;
                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines_as(
                        "Alex",
                        args!["Oh, so you saw the", "recruitment notice for the", "Odin Shrine Expedition, eh?"],
                    )?;
                    if ctx.var("BaseLevel").get()?.number()? > 59 {
                        ctx.lines(args![
                            "Hmmm... Alright, you look",
                            "like you're strong enough.",
                            "Yeah, I think you qualify."
                        ])?;
                        ctx.next()?;
                    } else {
                        ctx.call(Function::Cutin, vec![Val::from("hu_alex02.bmp"), Val::from(2)])?;
                        ctx.lines(args![
                            "Well, I dunno. I think that",
                            "kind of expedition would",
                            "eat you alive. No offense..."
                        ])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Alex",
                            args![
                                "Look, I'll reconsider",
                                "whether you qualify after",
                                "you get train a little more,",
                                "develop your skills, you know,",
                                "get stronger. For now, though,",
                                "I don't think we can use you."
                            ],
                        )?;
                        ctx.close_window()?;
                        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                        return Err(Stop::End);
                    }
                }
                _ => {}
            }
            ctx.call(Function::Cutin, vec![Val::from("hu_alex03.bmp"), Val::from(2)])?;
            ctx.lines_as(
                "Alex",
                args![
                    "I'm Alex Helmut, and I'm",
                    "in charge of the expedition.",
                    "See that guy over there? That's",
                    "my younger brother Julian who's",
                    "also working on the excavation."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Alex",
                args![
                    "Our main goal in this",
                    "excavation is to retrieve",
                    "some extremely valuable",
                    "item inside the shrine. However, it's too dangerous to go in there",
                    "if you're not strong enough."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Alex",
                args![
                    "Anyway, if you'd like to",
                    "help us, please take a look",
                    "around the shrine and see",
                    "if you'd want to work for us.",
                    "Please talk to the Boatman",
                    "to travel to the shrine, okay?"
                ],
            )?;
            ctx.var("hg_odin").set(Val::from(2))?;
            ctx.call(Function::SetQuest, vec![Val::from(11000)])?;
            ctx.close_window()?;
            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
            return Err(Stop::End);
        } else {
            if ctx.var("hg_odin").get()? == 2 {
                ctx.call(Function::Cutin, vec![Val::from("hu_alex03.bmp"), Val::from(2)])?;
                ctx.lines_as(
                    "Alex",
                    args![
                        "You really oughtta",
                        "check out the shrine",
                        "for yourself first. Just talk",
                        "to the Boatman so that",
                        "he take you over there."
                    ],
                )?;
                ctx.close_window()?;
                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                return Err(Stop::End);
            } else {
                if ctx.var("hg_odin").get()? == 3 {
                    ctx.call(Function::Cutin, vec![Val::from("hu_alex01.bmp"), Val::from(2)])?;
                    ctx.lines_as(
                        "Alex",
                        args![
                            "Oh, so you've gone",
                            "and seen the shrine",
                            "already. So will you help",
                            "us in our expedition?"
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Who are the people in the next room?:Well...:Yes, I do.")])? {
                        1 => {
                            ctx.call(Function::Cutin, vec![Val::from("hu_alex02.bmp"), Val::from(2)])?;
                            ctx.lines_as(
                                "Alex",
                                args![
                                    "Oh... Them.",
                                    "They're officially my",
                                    "co-workers, but quite",
                                    "frankly, they were forced",
                                    "upon me by the Rune-Midgarts",
                                    "Kingdom. It can't be helped..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Alex",
                                args![
                                    "Anyway, I have to accept",
                                    "them to help international",
                                    "relations or something silly",
                                    "like that. They say they're here to research religious relics,",
                                    "but I just don't trust them."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.call(Function::Cutin, vec![Val::from("hu_alex04.bmp"), Val::from(2)])?;
                            ctx.lines_as(
                                "Alex",
                                args![
                                    "In fact, I think it's",
                                    "obvious that they're here",
                                    "for something else, though",
                                    "I'm not sure what it may be.",
                                    "And that gray haired lady...",
                                    "She totally creeps me out!"
                                ],
                            )?;
                            ctx.close_window()?;
                            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as(
                                "Alex",
                                args![
                                    "Yeah, well, I know",
                                    "it might be a tough",
                                    "decision to make. Well,",
                                    "take your time. I understand",
                                    "that you have to weigh the",
                                    "risks and everything."
                                ],
                            )?;
                            ctx.close_window()?;
                            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                            return Err(Stop::End);
                        }
                        3 => {
                            ctx.lines_as(
                                "Alex",
                                args![
                                    "Great! In that case, your",
                                    "first assignment is to bring",
                                    "me ^3355FF5 Runes of the Darkness^000000",
                                    "from the shrine. Don't worry,",
                                    "I'll make sure that you're",
                                    "rewarded for your efforts."
                                ],
                            )?;
                            ctx.var("hg_odin").set(Val::from(4))?;
                            ctx.call(Function::ChangeQuest, vec![Val::from(11000), Val::from(11001)])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Alex",
                                args![
                                    "This is your chance",
                                    "to show me that your",
                                    "dependability and sense",
                                    "of responsibility. Then,",
                                    "we can move on to the",
                                    "more important stuff."
                                ],
                            )?;
                            ctx.close_window()?;
                            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else if ctx.var("hg_odin").get()? == 4 {
                    ctx.call(Function::Cutin, vec![Val::from("hu_alex02.bmp"), Val::from(2)])?;
                    ctx.lines_as(
                        "Alex",
                        args![
                            "What are you still",
                            "doing here? Shouldn't",
                            "you be headed to the",
                            "shrine already?"
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("What was I supposed to gather?:I'm leaving, I'm leaving!")])? {
                        1 => {
                            ctx.call(Function::Cutin, vec![Val::from("hu_alex03.bmp"), Val::from(2)])?;
                            ctx.lines_as(
                                "Alex",
                                args![
                                    "You've forgotten",
                                    "already? I asked you",
                                    "to bring me ^3355FF5 Runes",
                                    "of the Darkness^000000. Okay,",
                                    "now hurry up and get to it."
                                ],
                            )?;
                            ctx.close_window()?;
                            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as(
                                "Alex",
                                args![
                                    "That's fine...",
                                    "Just keep in mind that",
                                    "we're running behind",
                                    "schedule. If only that",
                                    "grey haired crone wasn't",
                                    "here... We'd be done by now!"
                                ],
                            )?;
                            ctx.close_window()?;
                            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else if ctx.var("hg_odin").get()? == 5 {
                    ctx.call(Function::Cutin, vec![Val::from("hu_alex01.bmp"), Val::from(2)])?;
                    ctx.lines_as(
                        "Alex",
                        args![
                            "Well, it's about",
                            "time you came back.",
                            "So did you bring me",
                            "^3355FF5 Runes of the Darkness^000000?"
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Er, not yet...:There you go!")])? {
                        1 => {
                            ctx.call(Function::Cutin, vec![Val::from("hu_alex02.bmp"), Val::from(2)])?;
                            ctx.lines_as(
                                "Alex",
                                args![
                                    "Wh-what?!",
                                    "You came here just to",
                                    "tell me that? Come back",
                                    "when you're done and",
                                    "don't waste my time!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.call(
                                Function::Emotion,
                                vec![
                                    ctx.constant("ET_DELIGHT")?,
                                    ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Julian")])?,
                                ],
                            )?;
                            ctx.lines_as(
                                "Julian",
                                args![
                                    "Hey, you know what,",
                                    "Alex? You can't push",
                                    "people around like that.",
                                    "Besides, this adventurer",
                                    "isn't a pushover, you know?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.call(Function::Cutin, vec![Val::from("hu_alex03.bmp"), Val::from(2)])?;
                            ctx.call(Function::Emotion, vec![ctx.constant("ET_FRET")?])?;
                            ctx.lines_as("Alex", args!["What was that...?", "Are you trying to", "make me mad, Julian?"])?;
                            ctx.next()?;
                            ctx.call(Function::Cutin, vec![Val::from("hu_alex02.bmp"), Val::from(2)])?;
                            ctx.lines_as(
                                "Julian",
                                args![
                                    "All I'm saying is that",
                                    "most adventurers know that",
                                    "you can't pay them what they're",
                                    "worth. That's why so many of",
                                    "them are going in and out",
                                    "of the other room, you know."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.call(Function::Cutin, vec![Val::from("hu_alex04.bmp"), Val::from(2)])?;
                            ctx.lines_as(
                                "Alex",
                                args![
                                    "That... grey... haired...!",
                                    "N-no... Calm down, Alex...",
                                    "Lose your head, you lose",
                                    "everything... D-don't stoop",
                                    "down to her level."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.call(Function::Cutin, vec![Val::from("hu_alex01.bmp"), Val::from(2)])?;
                            ctx.lines_as(
                                "Alex",
                                args![
                                    "Alright, I'm not in any",
                                    "position to make demands",
                                    "of you... But I would really",
                                    "appreciate it if you would",
                                    "bring me 5 Runes of the",
                                    "Darkness as soon as possible."
                                ],
                            )?;
                            ctx.close_window()?;
                            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                            return Err(Stop::End);
                        }
                        2 => {
                            if ctx.call(Function::CountItem, vec![Val::from(7511)])?.number()? > 4 {
                                ctx.call(Function::DelItem, vec![Val::from(7511), Val::from(5)])?;
                                ctx.var("hg_odin").set(Val::from(6))?;
                                ctx.call(Function::ChangeQuest, vec![Val::from(11001), Val::from(11002)])?;
                                ctx.call(Function::GetItem, vec![Val::from(603), Val::from(1)])?;
                                ctx.lines_as(
                                    "Alex",
                                    args![
                                        "Oh! Thank you",
                                        "so much, I'm sure",
                                        "these Runes will help",
                                        "us in our research. I know",
                                        "this isn't much, but please",
                                        "accept this Old Blue Box."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Julian", args!["Wait...", "Research?", "Since when did", "you do research?"])?;
                                ctx.next()?;
                                ctx.call(Function::Cutin, vec![Val::from("hu_alex03.bmp"), Val::from(2)])?;
                                ctx.lines_as("Alex", args!["Gosh, Julian,", "will you just", "shut up for a bit?"])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Julian",
                                    args![
                                        "Why? I'm the one who",
                                        "does all the grunt work",
                                        "and actual research.",
                                        "You think you're so",
                                        "big just ''supervising...''"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.call(Function::Cutin, vec![Val::from("hu_alex04.bmp"), Val::from(2)])?;
                                ctx.lines_as("Alex", args!["J-Julian!"])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Julian",
                                    args!["Right, right, we've", "got a visitor. Yeah,", "okay, sorry about that."],
                                )?;
                                ctx.next()?;
                                ctx.call(Function::Cutin, vec![Val::from("hu_alex02.bmp"), Val::from(2)])?;
                                ctx.lines_as(
                                    "Alex",
                                    args![
                                        "I'm sorry about all",
                                        "this. If you don't mind,",
                                        "we'll continue our research",
                                        "and hopefully finish our",
                                        "project soon. Thanks once",
                                        "again for all of your help."
                                    ],
                                )?;
                                ctx.close_window()?;
                                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                return Err(Stop::End);
                            }
                            ctx.call(Function::Cutin, vec![Val::from("hu_alex02.bmp"), Val::from(2)])?;
                            ctx.lines_as(
                                "Alex",
                                args![
                                    "Hm? Oh, this isn't",
                                    "enough Runes of the",
                                    "Darkness. I asked you",
                                    "to bring me 5 of them..."
                                ],
                            )?;
                            ctx.close_window()?;
                            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else if ctx.var("hg_odin").get()? == 6 {
                    ctx.call(Function::Cutin, vec![Val::from("hu_alex02.bmp"), Val::from(2)])?;
                    ctx.lines_as(
                        "Alex",
                        args![
                            "...Darn it! I think I might",
                            "have miscalculated. Maybe",
                            "I really do need more of",
                            "these. What am I gonna do?",
                            "And what is that grey haired",
                            "crone thinking? Hmmmm..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
                    ctx.call(Function::Cutin, vec![Val::from("hu_alex03.bmp"), Val::from(2)])?;
                    ctx.call(Function::Cutin, vec![Val::from("hu_alex01.bmp"), Val::from(2)])?;
                    ctx.lines_as(
                        "Alex",
                        args![
                            "O-Oh! I had no idea that",
                            "you were there! Y-you",
                            "didn't hear anything",
                            "s-strange, did you?",
                            "Hahahahahahaha!"
                        ],
                    )?;
                    ctx.close_window()?;
                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                    return Err(Stop::End);
                } else if (ctx.var("hg_odin").get()?.number()? > 11 && ctx.var("hg_odin").get()?.number()? < 17) {
                    ctx.call(Function::Cutin, vec![Val::from("hu_alex03.bmp"), Val::from(2)])?;
                    ctx.lines_as(
                        "Alex",
                        args![
                            "Excuse me?",
                            "The grey haired crone",
                            "offered you a part time",
                            "job, but won't pay you?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Alex",
                        args![
                            "She's really shameless!",
                            "We don't have much to pay",
                            "volunteers, but we should",
                            "at least give them something!",
                            "How can she do this?! Oh, er,",
                            "let me introduce myself first."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("hu_alex01.bmp"), Val::from(2)])?;
                    ctx.lines_as(
                        "Alex",
                        args![
                            "I'm Alex Helmut, and I'm",
                            "in charge of the expedition.",
                            "See that guy over there? That's",
                            "my younger brother Julian who's",
                            "also working on the excavation."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("hu_alex02.bmp"), Val::from(2)])?;
                    ctx.lines_as(
                        "Alex",
                        args![
                            "As supervisor of this",
                            "operation, I'm ashamed to",
                            "say that the excavation is",
                            "proceeding behind schedule.",
                            "If only that grey haired crone",
                            "in the next room weren't here!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Alex",
                        args![
                            "I can't work with",
                            "her at all! She's",
                            "manipulative, greedy,",
                            "selfish, and two faced!",
                            "There's got to be another",
                            "reason why she's here...!"
                        ],
                    )?;
                    ctx.close_window()?;
                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                    return Err(Stop::End);
                } else {
                    ctx.call(Function::Cutin, vec![Val::from("hu_alex02.bmp"), Val::from(2)])?;
                    ctx.lines_as(
                        "Alex",
                        args![
                            "Hm? Did you need",
                            "something? I'm sorry,",
                            "but I can't help you",
                            "right now. I'm pretty",
                            "busy, so I just want to",
                            "be left alone for a while."
                        ],
                    )?;
                    ctx.close_window()?;
                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                    return Err(Stop::End);
                }
            }
        }
    }
    Ok(Val::from(0))
}

pub fn alex(ctx: &Ctx) -> Script {
    alex_body(ctx, Vec::new()).map(|_| ())
}

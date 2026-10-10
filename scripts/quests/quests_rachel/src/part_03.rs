use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum Raevent1Step {
    Start,
    OnTouch,
}

fn raevent1_run(ctx: &Ctx, mut step: Raevent1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Raevent1Step::Start => {
                step = Raevent1Step::OnTouch;
                continue 'machine;
            }
            Raevent1Step::OnTouch => {
                if ctx.var("rach_vice").get()? == 23 {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Just looking at this",
                            "spring makes me think",
                            "of Bruspetti. What really",
                            "happened to her? I get",
                            "the feeling that she",
                            "caved in to despair."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "The water's not that",
                            "cold, or very deep...",
                            "And Katinshuell mentioned",
                            "that her body went limp...",
                            "But she was still looking",
                            "at him. It's haunting..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "I suppose she was torn...",
                            "She loved him, but couldn't",
                            "bear to live with his secret.",
                            "In the end, it's all so very",
                            "tragic. Katinshuell isn't",
                            "really a bad person..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "He was forced to",
                            "commit a heinous crime,",
                            "and kept making mistakes,",
                            "running from his guilt. Is",
                            "anyone accountable for this?",
                            "Who would be to blame?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.var("rach_vice").set(Val::from(24))?;
                    ctx.call(Function::CompleteQuest, vec![Val::from(8122)])?;
                    {
                        if ctx.var("BaseLevel").get()?.number()? > 90 {
                            ctx.call(Function::GetExperience, vec![Val::from(1300000), Val::from(0)])?;
                        } else if ctx.var("BaseLevel").get()?.number()? > 75 {
                            ctx.call(Function::GetExperience, vec![Val::from(850000), Val::from(0)])?;
                        } else {
                            ctx.call(Function::GetExperience, vec![Val::from(450000), Val::from(0)])?;
                        }
                    }
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Somehow, I wish that",
                            "Katinshuell had the",
                            "strength to face his",
                            "guilt, and then to",
                            "forgive himself."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("rach_vice").get()? == 22 {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["This must be where", "Bruspetti drowned..."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Grandma",
                        args![
                            "Oh! Excuse me,",
                            "young adventurer,",
                            "but you mustn't stand",
                            "there! It's very slippery.",
                            "What if you fall into the",
                            "spring? It's dangerous."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Oh... Er, thank you.", "Yes, I wouldn't want", "to get myself drowned."],
                    )?;
                    ctx.next()?;
                    ctx.var("rach_vice").set(Val::from(23))?;
                    ctx.lines_as(
                        "Grandma",
                        args![
                            "Drowned...? I just",
                            "wouldn't want you to",
                            "get your clothes wet.",
                            "The water isn't that deep...",
                            "Even if you can't swim, you",
                            "can climb out, you know."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["...What?"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("rach_vice").get()? == 8 {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "That old woman told",
                            "be to be careful not to",
                            "slip and fall into the water",
                            "around here, so I'd better",
                            "make sure I tread carefully."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("rach_vice").get()? == 7 {
                    ctx.lines_as(
                        "???",
                        args![
                            "Oh, be very careful!",
                            "You don't want to get",
                            "too close to the water.",
                            "What if you slip and fall?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Huh?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Grandma",
                        args![
                            "Oh, I'm sorry if",
                            "I startled you, but the",
                            "ground that you're standing",
                            "on is very slippery, you know."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Ah, I see."])?;
                    ctx.var("rach_vice").set(Val::from(8))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(8111), Val::from(8112)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn raevent1(ctx: &Ctx) -> Script {
    raevent1_run(ctx, Raevent1Step::Start, Vec::new()).map(|_| ())
}

pub fn raevent1_ontouch(ctx: &Ctx) -> Script {
    raevent1_run(ctx, Raevent1Step::OnTouch, Vec::new()).map(|_| ())
}

fn book_ra_in_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("rach_vice").get()? == 15 {
        ctx.lines(args![
            "^3355FFThis is where you",
            "found Bruspetti's diary.",
            "Perhaps you should take",
            "it with you the next time",
            "you talk to Katinshuell.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("rach_vice").get()? == 14 {
        ctx.var("rach_vice").set(Val::from(15))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(8118), Val::from(8119)])?;
        ctx.call(Function::GetItem, vec![Val::from(7571), Val::from(1)])?;
        ctx.lines(args![
            "^3355FFThis must be",
            "Bruspetti's diary!",
            "You now have permission",
            "to take it with you so that",
            "you can figure out if she and",
            "Katinshuell are connected...^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("rach_vice").get()? == 9 || ctx.var("rach_vice").get()? == 10) {
        ctx.lines(args![
            "^3355FFThis must be",
            "Bruspetti's diary!",
            "But... reading it",
            "would make you feel",
            "like a real creep.",
            "So don't touch it.^000000"
        ])?;
        ctx.var("rach_vice").set(Val::from(10))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(8113), Val::from(8114)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn book_ra_in(ctx: &Ctx) -> Script {
    book_ra_in_body(ctx, Vec::new()).map(|_| ())
}

fn sincere_follower_urstia_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::Cutin, vec![Val::from("ra_usti1"), Val::from(2)])?;
    if ctx.call(Function::CheckWeight, vec![Val::from(908), Val::from(200)])? == 0 {
        ctx.lines(args![
            "^3355FFWait a second!",
            "Right now, you're carrying",
            "too many things with you.",
            "Please come back after",
            "using the Kafra Service",
            "to store some of your items.^000000"
        ])?;
    }
    if ctx.var("ice_necklace_q").get()?.number()? < 1 {
        ctx.lines_as(
            "Urstialla",
            args![
                "Oh, are you an adventurer",
                "from the outside? Praise be",
                "to Freya! Her love and grace",
                "reaches all over the world,",
                "touching even the hearts of",
                "foreigners, leading them here!"
            ],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from("Freya? I'd like to know more.:That's crazy talk!")],
        )?) == 1
        {
            ctx.call(Function::Emotion, vec![ctx.constant("ET_OK")?])?;
            ctx.lines_as(
                "Urstialla",
                args![
                    "The day is coming when",
                    "Freya will resurrect and",
                    "lead all of her faithful to",
                    "Valhalla. Now she is in a",
                    "deep sleep, but even then,",
                    "she watches over all of us."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Urstialla",
                args![
                    "You see, Freya used up",
                    "all of her power fighting the",
                    "most fearsome of demons",
                    "in the Thousand Year War.",
                    "Now she rests and recovers",
                    "in a pure and sacred place."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Wait...", "How do you", "know that all of", "this happened?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Urstialla",
                args![
                    "These are truths that all",
                    "of Freya's worshippers know.",
                    "Freya delivers her messages to",
                    "us through her mortal vessel,",
                    "our beautiful pope that shines",
                    "with brilliant white light."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Urstialla",
                args![
                    "Unfortunately, even the most",
                    "faithful experience lapses in",
                    "judgment. It shames me to",
                    "admit that my son Egapeo",
                    "is... is guilty of sin."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Sin? What did he", "do, if you don't", "mind me asking?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Urstialla",
                args![
                    "I don't know, but Egapeo",
                    "has been sick for a while",
                    "now. I'm convinced that he",
                    "did something to anger Freya",
                    "Although he may deserve it,",
                    "I'm doing my best to help him."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["...Wait. What?!"])?;
            ctx.next()?;
            ctx.lines_as(
                "Urstialla",
                args![
                    "I know that if I pray hard",
                    "enough and please Freya,",
                    "she will forgive my son and",
                    "heal him of his illness.",
                    "But sometimes prayer isn't",
                    "enough... I need to pay tribute."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Urstialla",
                args![
                    "I managed to acquire a",
                    "necklace of incomparable",
                    "beauty from the dwarves.",
                    "However, the gems have dulled",
                    "with age and need to be shined",
                    "before I can offer them to Freya."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Urstialla",
                args![
                    "However, not just anyone can",
                    "shine this special necklace",
                    "forged by the Dwarves. I need",
                    "someone that knows arcane magic",
                    "to help me. I think there's only",
                    "one mage that can polish this..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Urstialla",
                args![
                    "Maheo, the renown mage with",
                    "access to the most powerful",
                    "magic spells in the world,",
                    "must be able to clean this",
                    "necklace's gems! However",
                    "There's no way I can reach him."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Urstialla",
                args![
                    "You see, Maheo left on a",
                    "journey to subjugate the",
                    "ice monsters in the ice cave",
                    "to the north, and he hasn't",
                    "returned yet. That place is too",
                    "dangerous for people like me."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Urstialla",
                args![
                    "I know that you adventurers",
                    "regularly travel through those",
                    "kinds of areas, so would you",
                    "please look for Maheo and ask",
                    "him to clean this necklace on",
                    "my behalf? Please, for my son..."
                ],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("But that's none of my business!:Alright, I'll do it.")],
            )?) == 1
            {
                ctx.lines_as(
                    "Urstialla",
                    args![
                        "I... I see.",
                        "I thought that Freya",
                        "had led you to me, but",
                        "maybe my prayers haven't",
                        "been answered yet. Perhaps",
                        "I need to pray more fervently?."
                    ],
                )?;
                ctx.close_window()?;
                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Urstialla",
                args![
                    "Thank you so much!",
                    "Please, take care of this",
                    "necklace and ask Maheo",
                    "to restore its luster. Then,",
                    "it'll be a fitting tribute to",
                    "our loving goddess Freya."
                ],
            )?;
            ctx.call(Function::SetQuest, vec![Val::from(2109)])?;
            ctx.call(Function::GetItem, vec![Val::from(7572), Val::from(1)])?;
            ctx.var("ice_necklace_q").set(Val::from(1))?;
            ctx.close_window()?;
            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
            return Err(Stop::End);
        }
        ctx.call(Function::Cutin, vec![Val::from("ra_usti2"), Val::from(2)])?;
        ctx.lines_as(
            "Urstialla",
            args![
                "...............................",
                "Are you telling me",
                "that you don't believe?",
                "Repent, and may your",
                "heart be opened to Freya!"
            ],
        )?;
    } else if (ctx.var("ice_necklace_q").get()?.number()? >= 1 && ctx.var("ice_necklace_q").get()?.number()? < 5) {
        ctx.lines_as(
            "Urstialla",
            args![
                "Please find Maheo the",
                "Mage and ask him to restore",
                "the beauty of the necklace",
                "I gave you. He should be",
                "fighting monsters in the",
                "ice cave to the north."
            ],
        )?;
    } else if ctx.var("ice_necklace_q").get()? == 5 {
        if ctx.call(Function::CountItem, vec![Val::from(7573)])?.number()? > 0 {
            ctx.lines_as(
                "Urstialla",
                args![
                    "Oh! My necklace! Thank you!",
                    "It's so beautiful! It will",
                    "make a wonderful tribute to",
                    "Freya! I am certain with this,",
                    "my son will get better!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Urstialla",
                args![
                    "Here, I know it's not much,",
                    "but please accept this as a",
                    "token of my appreication for",
                    " what you have done for me."
                ],
            )?;
            ctx.call(Function::DelItem, vec![Val::from(7573), Val::from(1)])?;
            ctx.call(Function::GetExperience, vec![Val::from(700000), Val::from(0)])?;
            ctx.var("ice_necklace_q").set(Val::from(6))?;
            ctx.call(Function::CompleteQuest, vec![Val::from(2113)])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Oh, let's just say it was", "a worthwhile experience", "for me."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Urstialla",
                args![
                    ".........",
                    "............",
                    "May Freya always protect and",
                    "guide you and forgive you for",
                    "that horrible joke."
                ],
            )?;
        } else {
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Oh, no! The necklace has disappeared!"],
            )?;
        }
    } else {
        ctx.lines_as(
            "Urstialla",
            args![
                "May Freya always",
                "protect and guide you",
                "with her everflowing",
                "grace and wisdom.."
            ],
        )?;
    }
    ctx.close_window()?;
    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
    return Err(Stop::End);
}

pub fn sincere_follower_urstia(ctx: &Ctx) -> Script {
    sincere_follower_urstia_body(ctx, Vec::new()).map(|_| ())
}

pub fn man_stuck_in_ice_cave(ctx: &Ctx) -> Script {
    man_stuck_in_ice_cave_run(ctx, ManStuckInIceCaveStep::Start, Vec::new()).map(|_| ())
}

pub fn man_stuck_in_ice_cave_ontouch(ctx: &Ctx) -> Script {
    man_stuck_in_ice_cave_run(ctx, ManStuckInIceCaveStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn man_stuck_in_ice_cave_ontouchnpc(ctx: &Ctx) -> Script {
    man_stuck_in_ice_cave_run(ctx, ManStuckInIceCaveStep::OnTouchNPC, Vec::new()).map(|_| ())
}

pub fn man_stuck_in_ice_cave_onmymobdead(ctx: &Ctx) -> Script {
    man_stuck_in_ice_cave_run(ctx, ManStuckInIceCaveStep::OnMyMobDead, Vec::new()).map(|_| ())
}

pub fn cave_vos(ctx: &Ctx) -> Script {
    cave_vos_run(ctx, CaveVosStep::Start, Vec::new()).map(|_| ())
}

pub fn cave_vos_oninit(ctx: &Ctx) -> Script {
    cave_vos_run(ctx, CaveVosStep::OnInit, Vec::new()).map(|_| ())
}

pub fn cave_vos_ontimer3600000(ctx: &Ctx) -> Script {
    cave_vos_run(ctx, CaveVosStep::OnTimer3600000, Vec::new()).map(|_| ())
}

pub fn cave_vos_ontimer7200000(ctx: &Ctx) -> Script {
    cave_vos_run(ctx, CaveVosStep::OnTimer7200000, Vec::new()).map(|_| ())
}

pub fn cave_vos_ontimer10800000(ctx: &Ctx) -> Script {
    cave_vos_run(ctx, CaveVosStep::OnTimer10800000, Vec::new()).map(|_| ())
}

fn hamion_aru_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(908), Val::from(200)])? == 0 {
        ctx.lines(args![
            "^3355FFWait a second!",
            "Right now, you're carrying",
            "too many things with you.",
            "Please come back after",
            "using the Kafra Service",
            "to store some of your items.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ice_necklace_q").get()?.number()? < 2 {
        ctx.lines_as(
            "Hamion",
            args![
                "Hm? Did you need",
                "something? If it's not too",
                "important, then I'd like to",
                "get back to reading my book."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("ice_necklace_q").get()? == 2 {
        ctx.lines_as(
            "Hamion",
            args![
                "Hm? Did you need",
                "something? If it's not too",
                "important, then I'd like to",
                "get back to reading my book."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Um, would you know",
                "a mage by the name",
                "of Maheo? I'm supposed",
                "to go look for his master",
                "around this area."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Hamion",
            args!["Well, you've found him", "because that would be me.", "What happened to Maheo?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Hamion",
            args![
                "He froze himself?!",
                "Hahahaha! Some genius!",
                "Oh, well, I suppose he still",
                "has plenty to learn about",
                "character, if not magic."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Hamion",
            args![
                "Let's see, let's see...",
                "This book ought to have",
                "the answer. Mmm... Ah!",
                "Page 42! I need to construct",
                "an artifact in order to break",
                "that mystical ice. Mmm."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Hamion",
            args![
                "Would you help",
                "me? I need to get",
                "^4D4DFF5 Rough Winds^000000,",
                "^4D4DFF1 Hammer^000000, and",
                "^4D4DFF1 Blank Scroll^000000"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Hamion",
            args![
                "You can get Blank Scrolls",
                "in the Sage Academy in Juno.",
                "Ah, and the Hammer is a weapon,",
                "not one of those smithing tools.",
                "I'd help you if I could, but...",
                "I'm just smart, not strong."
            ],
        )?;
        ctx.var("ice_necklace_q").set(Val::from(3))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(2110), Val::from(2111)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("ice_necklace_q").get()? == 3 {
        if ((ctx.call(Function::CountItem, vec![Val::from(996)])?.number()? < 5
            || ctx.call(Function::CountItem, vec![Val::from(1354)])?.number()? < 1)
            || ctx.call(Function::CountItem, vec![Val::from(7433)])?.number()? < 1)
        {
            ctx.lines_as(
                "Hamion",
                args![
                    "I need to create",
                    "a magical artifact",
                    "to break the ice that's",
                    "trapping Maheo. Would you",
                    "please help me? There's no",
                    "way I can get the items myself!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Hamion",
                args![
                    "I need to get",
                    "^4D4DFF5 Rough Winds^000000,",
                    "^4D4DFF1 Hammer^000000, and",
                    "^4D4DFF1 Blank Scroll^000000"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Hamion",
                args![
                    "You can get Blank Scrolls",
                    "in the Sage Academy in Juno.",
                    "Ah, and the Hammer is a weapon,",
                    "not one of those smithing tools.",
                    "I'd help you if I could, but...",
                    "I'm just smart, not strong."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Hamion",
            args![
                "Great, you have everything",
                "I need! But first, would you",
                "make sure that you only have",
                "1 Hammer in your inventory?",
                "If you have more than one,",
                "I might take the wrong Hammer."
            ],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from("Let me check.:Don't worry, I checked.")],
        )?) == 1
        {
            ctx.lines_as(
                "Hamion",
                args![
                    "Alright, it's always",
                    "better to be safe than",
                    "sorry! If only Maheo was",
                    "a little more careful, more",
                    "like you are, then maybe this",
                    "wouldn't have happened."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Hamion",
            args![
                "Okay, I'll take your",
                "word for it. Now, let's",
                "bring the artifact forging",
                "process. Hmmm... Let me",
                "review it here on page 45."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Hamion", args!["Here we go...!"])?;
        ctx.next()?;
        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_CONE")?])?;
        ctx.mes("^3355FF*Pzzzz*^000000")?;
        ctx.next()?;
        ctx.lines_as(
            "Hamion",
            args![
                "...Aaaand now it's done.",
                "Here, take this Wind Hammer",
                "and use it to free Maheo.",
                "This hammer will only work",
                "once, so make sure that you",
                "smash that ice properly!"
            ],
        )?;
        ctx.call(Function::DelItem, vec![Val::from(1354), Val::from(1)])?;
        ctx.call(Function::DelItem, vec![Val::from(996), Val::from(5)])?;
        ctx.call(Function::DelItem, vec![Val::from(7433), Val::from(1)])?;
        ctx.var("ice_necklace_q").set(Val::from(4))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(2111), Val::from(2112)])?;
        ctx.call(Function::GetItem, vec![Val::from(7569), Val::from(1)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("ice_necklace_q").get()?.number()? >= 4 {
        ctx.lines_as(
            "Hamion",
            args![
                "There are always",
                "singing birds and",
                "flitting butterflies",
                "all over this place",
                "It's so relaxing, and",
                "such a great place to read."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn hamion_aru(ctx: &Ctx) -> Script {
    hamion_aru_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum MohadianStep {
    Start,
    OnTouch,
}

fn mohadian_run(ctx: &Ctx, mut step: MohadianStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_hearts = Val::from(0);
    let mut l_totalprice = Val::from(0);
    'machine: loop {
        match step {
            MohadianStep::Start => {
                if ctx.var("ice_necklace_q").get()? == 6 {
                    ctx.lines_as(
                        "Mohadian",
                        args![
                            "I work at the bar around",
                            "here, and I've heard good",
                            "things about you. Some of",
                            "the customers've heard that",
                            "you helped Urstialla by going",
                            "into that dangerous Ice Cave."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mohadian",
                        args![
                            "Now, are you really capable",
                            "of regularly defeating the",
                            "snow monsters inside there?",
                            "If you are, I've got a bit of a",
                            "business proposition for you."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_AHA")?])?;
                    ctx.lines_as(
                        "Mohadian",
                        args![
                            "The weather around here",
                            "is naturally hot and humid,",
                            "so we need ice to make cold,",
                            "refreshing drinks at the pub.",
                            "And the Ice Cave is the best",
                            "place to get that ice, right?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mohadian",
                        args![
                            "If you bring me Ice Pieces",
                            "from the Ice Cave, I'll buy",
                            "them from you at 375 zeny",
                            "each. Think of it as kind of a",
                            "freelance job. I mean, if you",
                            "can bring me ice, I'll buy it!."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mohadian",
                        args![
                            "You can make some good",
                            "money, my customers can",
                            "enjoy ice cold drinks, and",
                            "my business will definitely",
                            "benefit. We'd kill three birds",
                            "with one stone! What do you say?"
                        ],
                    )?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("No, thanks.:Sure.")])?) == 1 {
                        ctx.lines_as(
                            "Mohadian",
                            args![
                                "Aww, how disappointing.",
                                "I was really sure that this",
                                "would be a great deal for",
                                "both of us. Well, if you're",
                                "willing to change your mind",
                                "then we can be partners!"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        "Mohadian",
                        args![
                            "Great! Now, the ice that",
                            "I need comes from Glacial",
                            "Hearts. You can obtain those",
                            "by hunting the snow monsters",
                            "in the Ice Cave. Remember, I'll",
                            "pay you 375 zeny for each one!"
                        ],
                    )?;
                    ctx.var("ice_necklace_q").set(Val::from(7))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("ice_necklace_q").get()? == 7 {
                    if ctx.call(Function::CountItem, vec![Val::from(7561)])?.number()? < 1 {
                        ctx.lines_as(
                            "Mohadian",
                            args![
                                "Bring me some Glacial",
                                "Hearts from the Ice Cave,",
                                "and I'll be sure to compensate",
                                "you with some zeny. This is a",
                                "really good deal for the two",
                                "of us when you think about it."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        "Mohadian",
                        args![
                            "Perfect, you brought me",
                            "some Glacial Hearts! I can",
                            "never get used to the beauty",
                            "and purity of these ice crystals."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mohadian",
                        args![
                            "Yes, I think I might",
                            "even be able to use these",
                            "to make Arunafeltz Glacial",
                            "Wine. Anyway, let me see",
                            "how many you've brought me."
                        ],
                    )?;
                    ctx.next()?;
                    l_hearts = ctx.call(Function::CountItem, vec![Val::from(7561)])?;
                    l_totalprice = (Val::from(375).try_mul(l_hearts.clone())?);
                    ctx.lines_as(
                        "Mohadian",
                        args![
                            (l_hearts.clone() + Val::from(" Glacial Hearts")),
                            "at 375 zeny each...",
                            "Looks like I owe you",
                            (l_totalprice.clone() + Val::from(" zeny. Here you are!")),
                            "It's always a pleasure",
                            "doing business with you~"
                        ],
                    )?;
                    ctx.call(Function::DelItem, vec![Val::from(7561), l_hearts.clone()])?;
                    ctx.var("Zeny").set((ctx.var("Zeny").get()? + l_totalprice.clone()))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                step = MohadianStep::OnTouch;
                continue 'machine;
            }
            MohadianStep::OnTouch => {
                if ctx.var("ice_necklace_q").get()? == 6 {
                    ctx.lines_as("Mohadian", args!["Welcome to--", "Er? Hello?", "Excuse me?"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn mohadian(ctx: &Ctx) -> Script {
    mohadian_run(ctx, MohadianStep::Start, Vec::new()).map(|_| ())
}

pub fn mohadian_ontouch(ctx: &Ctx) -> Script {
    mohadian_run(ctx, MohadianStep::OnTouch, Vec::new()).map(|_| ())
}

fn blazing_fire_ice1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args![
        "^3355FFThe flames in this fire",
        "barrier crackle with magic",
        "power. There's no way that",
        "you can put this fire out",
        "with conventional means.^000000"
    ])?;
    if ctx.var("ice_necklace_q").get()?.number()? > 4 && ctx.var("$@ktullanux_summon").get()?.number()? < 4 {
        if ctx.call(Function::CountItem, vec![Val::from(7574)])?.number()? > 0 {
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("Use Freezing Snow Powder.")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines(args![
                "^3355FFYou sprinkle the Freezing Snow",
                "Powder onto the flame. It",
                "flickers before extinguising.^000000"
            ])?;
            ctx.call(Function::DelItem, vec![Val::from(7574), Val::from(1)])?;
            ctx.call(Function::DisableNpc, vec![Val::from("Blazing Fire#ice1")])?;
            ctx.var("$@ktullanux_summon")
                .set((ctx.var("$@ktullanux_summon").get()? + Val::from(1)))?;
            if ctx.var("$@ktullanux_summon").get()? == 4 {
                ctx.call(Function::DoNpcEvent, vec![Val::from("ice_boss#broad::OnStart")])?;
            }
        } else if ctx.call(Function::CountItem, vec![Val::from(7562)])?.number()? > 0 {
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("Use Ice Scale.")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines(args![
                "^3355FFYou throw the Ice Scale into",
                "the flame, it crackles before",
                "extinguising.^000000"
            ])?;
            ctx.call(Function::DelItem, vec![Val::from(7562), Val::from(1)])?;
            ctx.call(Function::DisableNpc, vec![Val::from("Blazing Fire#ice1")])?;
            ctx.var("$@ktullanux_summon")
                .set((ctx.var("$@ktullanux_summon").get()? + Val::from(1)))?;
            if ctx.var("$@ktullanux_summon").get()? == 4 {
                ctx.call(Function::DoNpcEvent, vec![Val::from("ice_boss#broad::OnStart")])?;
            }
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn blazing_fire_ice1(ctx: &Ctx) -> Script {
    blazing_fire_ice1_body(ctx, Vec::new()).map(|_| ())
}

fn blazing_fire_ice2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args![
        "^3355FFThe flames in this fire",
        "barrier crackle with magic",
        "power. There's no way that",
        "you can put this fire out",
        "with conventional means.^000000"
    ])?;
    if ctx.var("ice_necklace_q").get()?.number()? > 4 && ctx.var("$@ktullanux_summon").get()?.number()? < 4 {
        if ctx.call(Function::CountItem, vec![Val::from(7574)])?.number()? > 0 {
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("Use Freezing Snow Powder.")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines(args![
                "^3355FFYou sprinkle the Freezing Snow",
                "Powder onto the flame. It",
                "flickers before extinguising.^000000"
            ])?;
            ctx.call(Function::DelItem, vec![Val::from(7574), Val::from(1)])?;
            ctx.call(Function::DisableNpc, vec![Val::from("Blazing Fire#ice2")])?;
            ctx.var("$@ktullanux_summon")
                .set((ctx.var("$@ktullanux_summon").get()? + Val::from(1)))?;
            if ctx.var("$@ktullanux_summon").get()? == 4 {
                ctx.call(Function::DoNpcEvent, vec![Val::from("ice_boss#broad::OnStart")])?;
            }
        } else if ctx.call(Function::CountItem, vec![Val::from(7562)])?.number()? > 0 {
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("Use Ice Scale.")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines(args![
                "^3355FFYou throw the Ice Scale into",
                "the flame, it crackles before",
                "extinguising.^000000"
            ])?;
            ctx.call(Function::DelItem, vec![Val::from(7562), Val::from(1)])?;
            ctx.call(Function::DisableNpc, vec![Val::from("Blazing Fire#ice2")])?;
            ctx.var("$@ktullanux_summon")
                .set((ctx.var("$@ktullanux_summon").get()? + Val::from(1)))?;
            if ctx.var("$@ktullanux_summon").get()? == 4 {
                ctx.call(Function::DoNpcEvent, vec![Val::from("ice_boss#broad::OnStart")])?;
            }
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn blazing_fire_ice2(ctx: &Ctx) -> Script {
    blazing_fire_ice2_body(ctx, Vec::new()).map(|_| ())
}

fn blazing_fire_ice3_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args![
        "^3355FFThe flames in this fire",
        "barrier crackle with magic",
        "power. There's no way that",
        "you can put this fire out",
        "with conventional means.^000000"
    ])?;
    if ctx.var("ice_necklace_q").get()?.number()? > 4 && ctx.var("$@ktullanux_summon").get()?.number()? < 4 {
        if ctx.call(Function::CountItem, vec![Val::from(7574)])?.number()? > 0 {
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("Use Freezing Snow Powder.")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines(args![
                "^3355FFYou sprinkle the Freezing Snow",
                "Powder onto the flame. It",
                "flickers before extinguising.^000000"
            ])?;
            ctx.call(Function::DelItem, vec![Val::from(7574), Val::from(1)])?;
            ctx.call(Function::DisableNpc, vec![Val::from("Blazing Fire#ice3")])?;
            ctx.var("$@ktullanux_summon")
                .set((ctx.var("$@ktullanux_summon").get()? + Val::from(1)))?;
            if ctx.var("$@ktullanux_summon").get()? == 4 {
                ctx.call(Function::DoNpcEvent, vec![Val::from("ice_boss#broad::OnStart")])?;
            }
        } else if ctx.call(Function::CountItem, vec![Val::from(7562)])?.number()? > 0 {
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("Use Ice Scale.")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines(args![
                "^3355FFYou throw the Ice Scale into",
                "the flame, it crackles before",
                "extinguising.^000000"
            ])?;
            ctx.call(Function::DelItem, vec![Val::from(7562), Val::from(1)])?;
            ctx.call(Function::DisableNpc, vec![Val::from("Blazing Fire#ice3")])?;
            ctx.var("$@ktullanux_summon")
                .set((ctx.var("$@ktullanux_summon").get()? + Val::from(1)))?;
            if ctx.var("$@ktullanux_summon").get()? == 4 {
                ctx.call(Function::DoNpcEvent, vec![Val::from("ice_boss#broad::OnStart")])?;
            }
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn blazing_fire_ice3(ctx: &Ctx) -> Script {
    blazing_fire_ice3_body(ctx, Vec::new()).map(|_| ())
}

fn blazing_fire_ice4_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args![
        "^3355FFThe flames in this fire",
        "barrier crackle with magic",
        "power. There's no way that",
        "you can put this fire out",
        "with conventional means.^000000"
    ])?;
    if ctx.var("ice_necklace_q").get()?.number()? > 4 && ctx.var("$@ktullanux_summon").get()?.number()? < 4 {
        if ctx.call(Function::CountItem, vec![Val::from(7574)])?.number()? > 0 {
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("Use Freezing Snow Powder.")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines(args![
                "^3355FFYou sprinkle the Freezing Snow",
                "Powder onto the flame. It",
                "flickers before extinguising.^000000"
            ])?;
            ctx.call(Function::DelItem, vec![Val::from(7574), Val::from(1)])?;
            ctx.call(Function::DisableNpc, vec![Val::from("Blazing Fire#ice4")])?;
            ctx.var("$@ktullanux_summon")
                .set((ctx.var("$@ktullanux_summon").get()? + Val::from(1)))?;
            if ctx.var("$@ktullanux_summon").get()? == 4 {
                ctx.call(Function::DoNpcEvent, vec![Val::from("ice_boss#broad::OnStart")])?;
            }
        } else if ctx.call(Function::CountItem, vec![Val::from(7562)])?.number()? > 0 {
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("Use Ice Scale.")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines(args![
                "^3355FFYou throw the Ice Scale into",
                "the flame, it crackles before",
                "extinguising.^000000"
            ])?;
            ctx.call(Function::DelItem, vec![Val::from(7562), Val::from(1)])?;
            ctx.call(Function::DisableNpc, vec![Val::from("Blazing Fire#ice4")])?;
            ctx.var("$@ktullanux_summon")
                .set((ctx.var("$@ktullanux_summon").get()? + Val::from(1)))?;
            if ctx.var("$@ktullanux_summon").get()? == 4 {
                ctx.call(Function::DoNpcEvent, vec![Val::from("ice_boss#broad::OnStart")])?;
            }
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn blazing_fire_ice4(ctx: &Ctx) -> Script {
    blazing_fire_ice4_body(ctx, Vec::new()).map(|_| ())
}

pub fn ice_boss_broad(ctx: &Ctx) -> Script {
    ice_boss_broad_run(ctx, IceBossBroadStep::Start, Vec::new()).map(|_| ())
}

pub fn ice_boss_broad_onstop(ctx: &Ctx) -> Script {
    ice_boss_broad_run(ctx, IceBossBroadStep::OnStop, Vec::new()).map(|_| ())
}

pub fn ice_boss_broad_onstart(ctx: &Ctx) -> Script {
    ice_boss_broad_run(ctx, IceBossBroadStep::OnStart, Vec::new()).map(|_| ())
}

pub fn ice_boss_broad_ontimer2000(ctx: &Ctx) -> Script {
    ice_boss_broad_run(ctx, IceBossBroadStep::OnTimer2000, Vec::new()).map(|_| ())
}

pub fn ice_boss_broad_ontimer8000(ctx: &Ctx) -> Script {
    ice_boss_broad_run(ctx, IceBossBroadStep::OnTimer8000, Vec::new()).map(|_| ())
}

pub fn ice_boss_broad_ontimer10000(ctx: &Ctx) -> Script {
    ice_boss_broad_run(ctx, IceBossBroadStep::OnTimer10000, Vec::new()).map(|_| ())
}

pub fn ice_boss_broad_ontimer13000(ctx: &Ctx) -> Script {
    ice_boss_broad_run(ctx, IceBossBroadStep::OnTimer13000, Vec::new()).map(|_| ())
}

pub fn ice_boss_broad_ontimer16000(ctx: &Ctx) -> Script {
    ice_boss_broad_run(ctx, IceBossBroadStep::OnTimer16000, Vec::new()).map(|_| ())
}

pub fn ice_boss_broad_ontimer19000(ctx: &Ctx) -> Script {
    ice_boss_broad_run(ctx, IceBossBroadStep::OnTimer19000, Vec::new()).map(|_| ())
}

pub fn ice_boss_broad_ontimer21000(ctx: &Ctx) -> Script {
    ice_boss_broad_run(ctx, IceBossBroadStep::OnTimer21000, Vec::new()).map(|_| ())
}

pub fn ice_boss_on(ctx: &Ctx) -> Script {
    ice_boss_on_run(ctx, IceBossOnStep::Start, Vec::new()).map(|_| ())
}

pub fn ice_boss_on_onstart(ctx: &Ctx) -> Script {
    ice_boss_on_run(ctx, IceBossOnStep::OnStart, Vec::new()).map(|_| ())
}

pub fn ice_boss_on_onstarttimer(ctx: &Ctx) -> Script {
    ice_boss_on_run(ctx, IceBossOnStep::OnStartTimer, Vec::new()).map(|_| ())
}

pub fn ice_boss_on_onstoptimer(ctx: &Ctx) -> Script {
    ice_boss_on_run(ctx, IceBossOnStep::OnStopTimer, Vec::new()).map(|_| ())
}

pub fn ice_boss_on_onmymobdead(ctx: &Ctx) -> Script {
    ice_boss_on_run(ctx, IceBossOnStep::OnMyMobDead, Vec::new()).map(|_| ())
}

pub fn ice_boss_on_ontimer7200000(ctx: &Ctx) -> Script {
    ice_boss_on_run(ctx, IceBossOnStep::OnTimer7200000, Vec::new()).map(|_| ())
}

fn ice_sec_run(ctx: &Ctx, mut step: IceSecStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            IceSecStep::Start => {
                step = IceSecStep::OnStart;
                continue 'machine;
            }
            IceSecStep::OnStart => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(Function::EnableNpc, vec![Val::from("#ice_4f_1")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("#ice_4f_2")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("#ice_4f_3")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("#ice_4f_4")])?;
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![ctx.constant("EF_MAPPILLAR2")?, ctx.constant("AREA")?, Val::from("#ice_4f_1")],
                )?;
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![ctx.constant("EF_MAPPILLAR2")?, ctx.constant("AREA")?, Val::from("#ice_4f_2")],
                )?;
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![ctx.constant("EF_MAPPILLAR2")?, ctx.constant("AREA")?, Val::from("#ice_4f_3")],
                )?;
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![ctx.constant("EF_MAPPILLAR2")?, ctx.constant("AREA")?, Val::from("#ice_4f_4")],
                )?;
                return Err(Stop::End);
            }
            IceSecStep::OnTimer60000 => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![Val::from("#ice_4f_1")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("#ice_4f_2")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("#ice_4f_3")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("#ice_4f_4")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn ice_sec(ctx: &Ctx) -> Script {
    ice_sec_run(ctx, IceSecStep::Start, Vec::new()).map(|_| ())
}

pub fn ice_sec_onstart(ctx: &Ctx) -> Script {
    ice_sec_run(ctx, IceSecStep::OnStart, Vec::new()).map(|_| ())
}

pub fn ice_sec_ontimer60000(ctx: &Ctx) -> Script {
    ice_sec_run(ctx, IceSecStep::OnTimer60000, Vec::new()).map(|_| ())
}

fn ice_4f_1_run(ctx: &Ctx, mut step: Ice4f1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Ice4f1Step::Start => {
                step = Ice4f1Step::OnTouch;
                continue 'machine;
            }
            Ice4f1Step::OnTouch => {
                ctx.call(Function::Warp, vec![Val::from("ice_dun04"), Val::from(33), Val::from(144)])?;
                return Err(Stop::End);
            }
            Ice4f1Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("#ice_4f_1")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn ice_4f_1(ctx: &Ctx) -> Script {
    ice_4f_1_run(ctx, Ice4f1Step::Start, Vec::new()).map(|_| ())
}

pub fn ice_4f_1_ontouch(ctx: &Ctx) -> Script {
    ice_4f_1_run(ctx, Ice4f1Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn ice_4f_1_oninit(ctx: &Ctx) -> Script {
    ice_4f_1_run(ctx, Ice4f1Step::OnInit, Vec::new()).map(|_| ())
}

fn ice_4f_2_run(ctx: &Ctx, mut step: Ice4f2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Ice4f2Step::Start => {
                step = Ice4f2Step::OnTouch;
                continue 'machine;
            }
            Ice4f2Step::OnTouch => {
                ctx.call(Function::Warp, vec![Val::from("ice_dun04"), Val::from(33), Val::from(144)])?;
                return Err(Stop::End);
            }
            Ice4f2Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("#ice_4f_2")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn ice_4f_2(ctx: &Ctx) -> Script {
    ice_4f_2_run(ctx, Ice4f2Step::Start, Vec::new()).map(|_| ())
}

pub fn ice_4f_2_ontouch(ctx: &Ctx) -> Script {
    ice_4f_2_run(ctx, Ice4f2Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn ice_4f_2_oninit(ctx: &Ctx) -> Script {
    ice_4f_2_run(ctx, Ice4f2Step::OnInit, Vec::new()).map(|_| ())
}

fn ice_4f_3_run(ctx: &Ctx, mut step: Ice4f3Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Ice4f3Step::Start => {
                step = Ice4f3Step::OnTouch;
                continue 'machine;
            }
            Ice4f3Step::OnTouch => {
                ctx.call(Function::Warp, vec![Val::from("ice_dun04"), Val::from(33), Val::from(144)])?;
                return Err(Stop::End);
            }
            Ice4f3Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("#ice_4f_3")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn ice_4f_3(ctx: &Ctx) -> Script {
    ice_4f_3_run(ctx, Ice4f3Step::Start, Vec::new()).map(|_| ())
}

pub fn ice_4f_3_ontouch(ctx: &Ctx) -> Script {
    ice_4f_3_run(ctx, Ice4f3Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn ice_4f_3_oninit(ctx: &Ctx) -> Script {
    ice_4f_3_run(ctx, Ice4f3Step::OnInit, Vec::new()).map(|_| ())
}

fn ice_4f_4_run(ctx: &Ctx, mut step: Ice4f4Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Ice4f4Step::Start => {
                step = Ice4f4Step::OnTouch;
                continue 'machine;
            }
            Ice4f4Step::OnTouch => {
                ctx.call(Function::Warp, vec![Val::from("ice_dun04"), Val::from(33), Val::from(144)])?;
                return Err(Stop::End);
            }
            Ice4f4Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("#ice_4f_4")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn ice_4f_4(ctx: &Ctx) -> Script {
    ice_4f_4_run(ctx, Ice4f4Step::Start, Vec::new()).map(|_| ())
}

pub fn ice_4f_4_ontouch(ctx: &Ctx) -> Script {
    ice_4f_4_run(ctx, Ice4f4Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn ice_4f_4_oninit(ctx: &Ctx) -> Script {
    ice_4f_4_run(ctx, Ice4f4Step::OnInit, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum TempleEntranceRaTemStep {
    Start,
    OnTouch,
}

fn temple_entrance_ra_tem_run(ctx: &Ctx, mut step: TempleEntranceRaTemStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            TempleEntranceRaTemStep::Start => {
                step = TempleEntranceRaTemStep::OnTouch;
                continue 'machine;
            }
            TempleEntranceRaTemStep::OnTouch => {
                if ctx.var("$rachel_donate").get()?.number()? >= 10000 {
                    if runtime::op(&ctx.var("misc_quest").get()?, "&", &Val::from(8192))?.is_true() {
                        ctx.call(Function::Warp, vec![Val::from("ra_temin"), Val::from(169), Val::from(23)])?;
                        return Err(Stop::End);
                    }
                    if ctx.var("ra_tem_q").get()?.number()? < 10 {
                        ctx.lines(args!["^3355FFThe temple's", "entrance is locked.^000000"])?;
                        ctx.next()?;
                        let choice = runtime::select_values(ctx, &[Val::from("Kick Door.:Smash Door with Weapon.")])?;
                        ctx.var("@menu").set(choice)?;
                        ctx.call(
                            Function::Emotion,
                            vec![
                                ctx.constant("ET_HUK")?,
                                ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Nemma#ra_temple")])?,
                            ],
                        )?;
                        ctx.lines_as("Priestess Nemma", args!["Please don't do that!"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                ctx.call(Function::Warp, vec![Val::from("ra_temin"), Val::from(169), Val::from(23)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn temple_entrance_ra_tem(ctx: &Ctx) -> Script {
    temple_entrance_ra_tem_run(ctx, TempleEntranceRaTemStep::Start, Vec::new()).map(|_| ())
}

pub fn temple_entrance_ra_tem_ontouch(ctx: &Ctx) -> Script {
    temple_entrance_ra_tem_run(ctx, TempleEntranceRaTemStep::OnTouch, Vec::new()).map(|_| ())
}

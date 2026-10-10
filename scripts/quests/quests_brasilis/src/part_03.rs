use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn shaman_nk_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_iara_q = Val::from(0);
    let mut l_iara_re = Val::from(0);
    let mut l_re_q = Val::from(0);
    l_iara_re = ctx.call(Function::CheckQuest, vec![Val::from(4135), ctx.constant("PLAYTIME")?])?;
    if (l_iara_re.clone() == 0 || l_iara_re.clone() == 1) {
        ctx.lines_as(
            "Anori",
            args![
                "I'm still preparing.",
                "I don't require your help at this time.",
                "Please come back later..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.call(Function::CheckQuest, vec![Val::from(4135)])?.number()? > 1 {
            ctx.call(Function::EraseQuest, vec![Val::from(4135)])?;
        }
        l_iara_q = ctx.call(Function::CheckQuest, vec![Val::from(4133)])?;
        if (l_iara_q.clone() == 0 || l_iara_q.clone() == 1) {
            ctx.lines_as(
                "Anori",
                args!["To block Iara ", "seducing the tribes", "we need a purifying potion..."],
            )?;
            ctx.next()?;
            ctx.lines_as("Anori", args!["Did you bring the materials", "to make the purifying potion?"])?;
            ctx.next()?;
            if ((ctx.call(Function::CountItem, vec![Val::from(950)])?.number()? > 19
                && ctx.call(Function::CountItem, vec![Val::from(7172)])?.number()? > 9)
                && ctx.call(Function::CountItem, vec![Val::from(1054)])?.number()? > 2)
            {
                ctx.lines_as(
                    "Anori",
                    args![
                        "Um... it seems to be okay.",
                        "I'll make you a potion which will",
                        "weaken Iara's power."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Anori",
                    args!["Let's see grind this...", "and mix in that...", "then add some magic..."],
                )?;
                ctx.next()?;
                ctx.call(Function::SetQuest, vec![Val::from(4135)])?;
                ctx.call(Function::EraseQuest, vec![Val::from(4133)])?;
                ctx.call(Function::SetQuest, vec![Val::from(4134)])?;
                ctx.call(Function::CompleteQuest, vec![Val::from(4134)])?;
                ctx.call(Function::DelItem, vec![Val::from(950), Val::from(20)])?;
                ctx.call(Function::DelItem, vec![Val::from(7172), Val::from(10)])?;
                ctx.call(Function::DelItem, vec![Val::from(1054), Val::from(3)])?;
                ctx.call(Function::GetItem, vec![Val::from(11517), Val::from(2)])?;
                ctx.lines_as(
                    "Anori",
                    args![
                        "Here, it's completed.",
                        "Take this.",
                        "It will make Iara stop",
                        "training at the cave",
                        "for a while."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Anori", args!["Please block the Iara threatening the security of the tribe."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Anori",
                    args![
                        "You haven't brought enough materials yet.",
                        "We cannot make the purification potion with only these."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else {
            if ctx.var("BaseLevel").get()?.number()? < 40 {
                ctx.lines_as(
                    "Anori",
                    args!["Ah... we need a strong adventurer.", "The tribe is facing a major threat."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            l_re_q = ctx.call(Function::CheckQuest, vec![Val::from(4134)])?;
            if l_re_q.clone() == 2 {
                ctx.lines_as(
                    "Anori",
                    args!["you are...", "the adventurer who came for the", "purification potion..."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Anori",
                    args![
                        "Maybe because of the purification potion...",
                        "After that, the Iara stopped seducing tribesmen but the effect didn't last long."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Anori",
                    args!["Could you get the same", "materials as before...", "I need your power."],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("No.:Okay, I'll do it.")])? {
                    1 => {
                        ctx.lines_as("Anori", args!["This, ah...", "There is no other way."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as(
                            "Anori",
                            args!["You are truly brave!", "I, on behalf of the tribe,", "offer you my thanks."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Anori",
                            args![
                                "Materials are the same as before.",
                                "If you just get^ff0000 20 Hearts of Mermaids,",
                                "10 Leopard Claws and",
                                "3 Ancient Lips^000000,",
                                "I will make you a potion that purifies evil spirits",
                                "by using a secret formula handed down to the tribe."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Anori",
                            args![
                                "The destiny of the tribe is up to you.",
                                "please get the materials quickly.",
                                "I will be preparing to make",
                                "the purification potion right here."
                            ],
                        )?;
                        ctx.call(Function::SetQuest, vec![Val::from(4133)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            } else {
                ctx.lines_as(
                    "Anori",
                    args!["There are some people I haven't seen before around here.", "It's a good sign..."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Anori",
                    args![
                        "Hey you...",
                        "Could you listen to my stories for a moment.",
                        "There's an emergency in our tribe."
                    ],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(ctx, &[Val::from("No.:Okay.")])?) == 1 {
                    ctx.lines_as(
                        "Anori",
                        args![
                            "You are a heartless person...",
                            "You don't seem the helpful type.",
                            "Just keep on going your way."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Anori",
                    args![
                        "Thank you, I met a kind person.",
                        "It's a secret of our tribe that",
                        "you cannot tell anyone."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Anori",
                    args![
                        "Lately young men from",
                        "the tribe are disappearing.",
                        "Our entire tribe is being threatened."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Anori",
                    args![
                        "It is likely because of a witch called Iara.",
                        "She is a water nymph seducing the hearts of young tribesmen at a cave behind the waterfall."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Anori",
                    args![
                        "How can I stop these young tribesmen?",
                        "But I discovered a way to make a purification potion to reverse the effects of the Iara's spells."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Anori",
                    args![
                        "This potion has been handed down from many generations in our tribe.",
                        "This purification potion possesses the power to cleanse evil spirits."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Anori",
                    args![
                        "If you could get the materials",
                        "I will make you",
                        "the purification potion.",
                        "Could you do that for me?"
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("No.:Yes, I can.")])? {
                    1 => {
                        ctx.lines_as(
                            "Anori",
                            args!["Hm...", "Well, then.", "If you change your mind you can come to me again."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Anori",
                            args![
                                "You shouldn't talk about",
                                "what you heard now to anyone",
                                "It's kind of embarrassing..."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as(
                            "Anori",
                            args![
                                "Oh! You are the savior",
                                "of our tribe indeed.",
                                "On behalf of the tribe, I offer you my thanks."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Anori",
                            args![
                                "Well, what we need is this.",
                                "It's all you can get from near here.",
                                "Note down well."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Anori",
                            args![
                                "^ff0000 20 Hearts of Mermaids",
                                "10 Leopard Claws",
                                "3 Ancient Lips^000000",
                                "are the only ones that are needed as the materials."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Anori", args!["When you get those, I will make you a potion that purifies evil spirits using a secret formula handed down to the tribe."])?;
                        ctx.call(Function::SetQuest, vec![Val::from(4133)])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Anori",
                            args![
                                "The destiny of the tribe is up to you.",
                                "I hope you move quickly.",
                                "Even at this moment, the village men are being seduced and slipping way..."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(Val::from(0))
}

pub fn shaman_nk(ctx: &Ctx) -> Script {
    shaman_nk_body(ctx, Vec::new()).map(|_| ())
}

fn iara_nk_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CountItem, vec![Val::from(11517)])?.number()? > 0 {
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Should I use a Purification Potion?"],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Yes.:No.")])? {
            1 => {
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_MAPPILLAR")?])?;
                ctx.lines_as(
                    "Iara",
                    args![
                        "Ah...this light is...",
                        "It's like getting cleansed of evil thoughts",
                        "from deep within my heart."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Iara",
                    args!["At last I can forget the curse that I placed on myself when I drowned in the water."],
                )?;
                ctx.next()?;
                ctx.lines_as("Iara", args!["Do you think I can be born again as a kind water nymph?"])?;
                ctx.next()?;
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_GHOST")?])?;
                ctx.lines_as(
                    "Iara",
                    args![
                        "Ah... Thank you for helping me recover my consciousness for a while.",
                        "But... I think that the curse has been with me too long.",
                        "Get away from me quickly."
                    ],
                )?;
                ctx.call(Function::DelItem, vec![Val::from(11517), Val::from(1)])?;
                ctx.call(
                    Function::StartStatus,
                    vec![ctx.constant("SC_INCCRI")?, Val::from(3600000), Val::from(7)],
                )?;
                ctx.call(Function::ConsumeItem, vec![Val::from(12043)])?;
                ctx.call(Function::ConsumeItem, vec![Val::from(12063)])?;
                ctx.call(Function::ConsumeItem, vec![Val::from(12058)])?;
                ctx.call(Function::ConsumeItem, vec![Val::from(12053)])?;
                ctx.call(Function::ConsumeItem, vec![Val::from(12048)])?;
                ctx.call(Function::ConsumeItem, vec![Val::from(12068)])?;
                ctx.next()?;
                ctx.lines_as("Iara", args!["Ahhh~..."])?;
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_DEVIL")?])?;
                ctx.next()?;
                ctx.lines_as(
                    "Iara",
                    args!["The curse is too strong for me to keep contained.", "Leave now while you are safe."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["(I guess I should ignore her.)"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else {
        ctx.lines_as(
            "Iara",
            args!["Aaaaaaaaaaaaaaaaaaaaaah.", "Eeeeeeeeeeeeeeeeeeeh.", "Oooooooooooooooooh."],
        )?;
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(2)])? == 1 {
            ctx.call(
                Function::StartStatus,
                vec![ctx.constant("SC_CURSE")?, Val::from(60000), Val::from(0)],
            )?;
        } else {
            ctx.call(
                Function::StartStatus,
                vec![ctx.constant("SC_CONFUSION")?, Val::from(60000), Val::from(0)],
            )?;
        }
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Ugh! What's this strange voice?"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    return Err(Stop::End);
}

pub fn iara_nk(ctx: &Ctx) -> Script {
    iara_nk_body(ctx, Vec::new()).map(|_| ())
}

fn iara_nk_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CountItem, vec![Val::from(11517)])?.number()? < 1 {
        ctx.lines_as(
            "Iara",
            args!["Aaaaaaaaaaaaaaaaaaaaaah.", "Eeeeeeeeeeeeeeeeeeeh.", "Oooooooooooooooooh."],
        )?;
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(2)])? == 1 {
            ctx.call(
                Function::StartStatus,
                vec![ctx.constant("SC_CURSE")?, Val::from(60000), Val::from(0)],
            )?;
        } else {
            ctx.call(
                Function::StartStatus,
                vec![ctx.constant("SC_CONFUSION")?, Val::from(60000), Val::from(0)],
            )?;
        }
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Ugh! What's this strange voice?"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    return Err(Stop::End);
}

pub fn iara_nk_ontouch(ctx: &Ctx) -> Script {
    iara_nk_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn native_warrior_nk_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Native Warrior",
        args!["Ah... the face I would never forget even in my dreams."],
    )?;
    ctx.next()?;
    ctx.lines_as("Native Warrior", args!["When will she come out of the waterfall again...?"])?;
    ctx.next()?;
    if (ctx.call(Function::CheckQuest, vec![Val::from(4133)])?.number()? >= 0
        || ctx.call(Function::CheckQuest, vec![Val::from(4134)])?.number()? >= 0)
    {
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["(Wh...what's this guy?)"],
        )?;
    } else {
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["(This guy will never", "get over Iara's curse...)"],
        )?;
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn native_warrior_nk(ctx: &Ctx) -> Script {
    native_warrior_nk_body(ctx, Vec::new()).map(|_| ())
}

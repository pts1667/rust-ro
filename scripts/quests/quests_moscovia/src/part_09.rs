use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn dirty_wall_rus39_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("rhea_rus_main").get()?.number()? >= 22 && ctx.var("rhea_rus_main").get()?.number()? < 26) {
        ctx.mes("- Something is engraved on the wall -")?;
        ctx.next()?;
        ctx.mes("- ^ff0000Her... careful.. Mermaid's song.. sleeping.. the eyes.. protect your.. mirror^000000 -")?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["...What is this talking about...?"],
        )?;
        if ctx.var("rhea_rus_main").get()? == 22 {
            ctx.var("rhea_rus_main").set(Val::from(23))?;
        }
        ctx.close_window()?;
        return Err(Stop::End);
    }
    return Err(Stop::End);
}

pub fn dirty_wall_rus39(ctx: &Ctx) -> Script {
    dirty_wall_rus39_body(ctx, Vec::new()).map(|_| ())
}

fn old_wooden_box_rus40_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("rhea_rus_main").get()? == 23 {
        ctx.mes("- Something is glimmering inside of the box -")?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Put hand inside the box")])?;
        ctx.var("@menu").set(choice)?;
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(7)])? == 4 {
            ctx.mes("- You take out something glimmering from the box -")?;
            ctx.next()?;
            ctx.mes("- ^0000ffYou find the handle of a broken key !!^000000 -")?;
            ctx.var("rhea_rus_main").set(Val::from(24))?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.mes("- You try to put your hand inside it but the gap is too small to do it -")?;
        ctx.next()?;
        ctx.mes("- You ruin the box a little -")?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("rhea_rus_main").get()?.number()? > 23 && ctx.var("rhea_rus_main").get()?.number()? < 26) {
        ctx.mes("- The old box that contained the handle of a broken key -")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    return Err(Stop::End);
}

pub fn old_wooden_box_rus40(ctx: &Ctx) -> Script {
    old_wooden_box_rus40_body(ctx, Vec::new()).map(|_| ())
}

fn opened_treasure_chest_41_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_russ_key02 = Val::from(0);
    if ctx.var("rhea_rus_main").get()? == 24 {
        ctx.mes("- You open the box, junk is in it -")?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["... Should I search this...?"],
        )?;
        ctx.next()?;
        ctx.mes("- You sigh and stretch your hand into the box -")?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["...!? What is this?!"],
        )?;
        ctx.next()?;
        l_russ_key02 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(7)])?;
        if l_russ_key02.clone() != 3 {
            let subject1 = l_russ_key02.clone();
            if subject1 == 1 {
                ctx.mes("- You find the cuticle of Kukre !! -")?;
            } else if subject1 == 2 {
                ctx.mes("- You find the egg of a Theif Bug !! -")?;
            } else if subject1 == 4 {
                ctx.mes("- You find something that seems to be a banana before !! -")?;
            } else if subject1 == 5 {
                ctx.mes("- You find the tentacles of a Jelly Fish !! -")?;
            } else if subject1 == 6 {
                ctx.mes("- You find pieces of cloth with must on it !! -")?;
            } else if subject1 == 7 {
                ctx.mes("- You find a bone !! -")?;
            }
            ctx.next()?;
            ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["W, what is this!?"])?;
            ctx.call(
                Function::Emotion,
                vec![
                    ctx.constant("ET_HUK")?,
                    Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.mes("- ^0000ffYou find the handle of a broken key in the junk !!^000000 -")?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Eh, this may be..."],
        )?;
        ctx.next()?;
        ctx.mes("- You adjust the piece of the broken key with the its handle and they make a sound and become a key !!-")?;
        ctx.var("rhea_rus_main").set(Val::from(25))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("rhea_rus_main").get()?.number()? > 24 && ctx.var("rhea_rus_main").get()?.number()? < 26) {
        ctx.mes("- The junk box containig the piece of the broken key -")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    return Err(Stop::End);
}

pub fn opened_treasure_chest_41(ctx: &Ctx) -> Script {
    opened_treasure_chest_41_body(ctx, Vec::new()).map(|_| ())
}

fn momotoro_publisher_rus42_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input_s = Val::from("");
    if (ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 3500 {
        ctx.lines_as(
            "Momotoro Publisher",
            args!["What on earth do you have in your bag?", "Are you training for something?"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("rhea_rus_main").get()?.number()? < 26 {
        ctx.lines_as("Momotoro Publisher", args!["Did you order a book?"])?;
        ctx.next()?;
        ctx.lines_as("Momotoro Publisher", args!["Someone ordered a book but did not pick it up."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("rhea_rus_main").get()? == 26 {
        ctx.lines_as("Momotoro Publisher", args!["Did you order a book?"])?;
        ctx.next()?;
        ctx.lines_as("Momotoro Publisher", args!["Someone ordered a book but did not pick it up."])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Ah, I.. a magic book..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Momotoro Publisher",
            args!["What? Ah, did you come here to pick up the book? Please, let me know your name?"],
        )?;
        ctx.next()?;
        let (input, status) = runtime::input_text(ctx, None, None)?;
        l_input_s = input;
        if l_input_s.clone() == "Baba Yaga" {
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![((Val::from("I am ") + l_input_s.clone()) + Val::from("."))],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Momotoro Publisher",
                args![((Val::from("") + l_input_s.clone()) + Val::from(" ... Ah, here it is."))],
            )?;
            ctx.next()?;
        } else {
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![((Val::from("I am ") + l_input_s.clone()) + Val::from(" "))],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Momotoro Publisher",
                args![((Val::from("") + l_input_s.clone()) + Val::from(" ... Let's see..."))],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Momotoro Publisher",
                args![
                    ((Val::from("Hmm? I am sorry but there is no book reserved for ") + l_input_s.clone())
                        + Val::from(". Can you check again please?"))
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as("Momotoro Publisher", args!["Are these the right ones? 'Magic to destroy time', 'The story of the house ghost around the world', 'Strange and weird world of Herbs", "Hmm, the price for all is 5,000 Zeny."])?;
        ctx.next()?;
        if ctx.var("Zeny").get()?.number()? > 4999 {
            ctx.lines_as("Momotoro Publisher", args!["Eh, yep this is 5,000 Zeny."])?;
            ctx.next()?;
        } else {
            ctx.lines_as(
                "Momotoro Publisher",
                args!["You don't have enough money?", "Check it again and come to me, please"],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Momotoro Publisher",
            args!["Thank you for buying our books. If you need more books, contact us please."],
        )?;
        ctx.call(Function::GetItem, vec![Val::from(7881), Val::from(1)])?;
        ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(5000))?))?;
        ctx.var("rhea_rus_main").set(Val::from(27))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("rhea_rus_main").get()?.number()? > 26 && ctx.var("rhea_rus_main").get()?.number()? < 31) {
        ctx.lines_as(
            "Momotoro Publisher",
            args!["Thank you for buying our books. If you need more books, contact us please."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as("Momotoro Publisher", args!["Did you order a book?"])?;
    ctx.next()?;
    ctx.lines_as("Momotoro Publisher", args!["Someone ordered a book but did not pick it up."])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn momotoro_publisher_rus42(ctx: &Ctx) -> Script {
    momotoro_publisher_rus42_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum HouseGhostJarRus43Step {
    Start,
    OnDisable,
    OnTimer180000,
    OnMyMobDead,
}

fn house_ghost_jar_rus43_run(ctx: &Ctx, mut step: HouseGhostJarRus43Step, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_card_s: Vec<Val> = Vec::new();
    let mut l_player_name_s = Val::from("");
    let mut l_rucard_game01 = Val::from(0);
    let mut l_rugame_turn01 = Val::from(0);
    let mut l_ruuser_score01 = Val::from(0);
    let mut l_s = Val::from(0);
    let mut l_win_string_s = Val::from("");
    'machine: loop {
        match step {
            HouseGhostJarRus43Step::Start => {
                if ctx.var("rhea_rus_main").get()? == 31 {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["... Is it in here..? Hey, hey..."],
                    )?;
                    ctx.next()?;
                    ctx.lines(args!["- You tap the jar -", "- with your hand -"])?;
                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_SIGHTRASHER")?])?;
                    ctx.call(
                        Function::Emotion,
                        vec![
                            ctx.constant("ET_HUK")?,
                            Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines(args!["- A voice laughs in the jar as -", "- it shakes from right to left -"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["You! No more games! Come out of there!"],
                    )?;
                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_HIT2")?])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "- You kick the jar -",
                        "- it rolls to the floor -",
                        "- as bugs come out!! -"
                    ])?;
                    ctx.var("rhea_rus_main").set(Val::from(32))?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("mosk_dun02"),
                            Val::from(58),
                            Val::from(220),
                            Val::from("Thief Bug Male"),
                            Val::from(1054),
                            Val::from(1),
                            Val::from("House Ghost Jar#rus43::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("mosk_dun02"),
                            Val::from(59),
                            Val::from(220),
                            Val::from("Ancient Worm"),
                            Val::from(1305),
                            Val::from(1),
                            Val::from("House Ghost Jar#rus43::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("mosk_dun02"),
                            Val::from(60),
                            Val::from(220),
                            Val::from("Thief Bug Male"),
                            Val::from(1054),
                            Val::from(1),
                            Val::from("House Ghost Jar#rus43::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("House Ghost Jar#rus43::OnDisable")])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("rhea_rus_main").get()? == 32 {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Ehh.. you, what are you doing!!"],
                    )?;
                    ctx.next()?;
                    ctx.lines(args![
                        "- As you plan to kick the jar -",
                        "- again, something scared -",
                        "- utters in a shaky voice -"
                    ])?;
                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_CURSEATTACK")?])?;
                    ctx.next()?;
                    ctx.lines_as("House Ghost", args!["No! Please stop it! I was wrong!"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "House Ghost",
                        args!["I have been bored. After dying, I've had nothing to do and no friends to play with.."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "House Ghost",
                        args!["When I was alive, I always stayed still, but now it's too boring..."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "House Ghost",
                        args!["I can't even drink my favorite milk... Being a ghost is too inconvenient."],
                    )?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("Can I play with you?:So what can I do for you?")],
                    )?) == 1
                    {
                        ctx.lines_as("House Ghost", args!["Ehhh, are you sure?"])?;
                    } else {
                        ctx.lines_as(
                            "House Ghost",
                            args!["Hmm, can you do something for me? It won't take much time."],
                        )?;
                    }
                    ctx.next()?;
                    ctx.lines_as(
                        "House Ghost",
                        args!["What about a card game? I was good at those games! Ah, where are my cards? Wait here! I'll find them!"],
                    )?;
                    ctx.var("rhea_rus_main").set(Val::from(33))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("rhea_rus_main").get()? == 33 {
                    l_player_name_s = ctx.call(Function::StrCharInfo, vec![Val::from(0)])?;
                    ctx.lines_as(
                        "House Ghost",
                        args![
                            "Hehe, what about a card game? I have an Angeling, Ghostring and a Poring Card.",
                            "All you have to do is just guess which card I pick up!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "House Ghost",
                        args!["Let's play best out of 5. So if you win 3 games, I promise to stay quiet!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(l_player_name_s.clone(), args!["Ok! It will be fun. Let's do it."])?;
                    ctx.next()?;
                    let base = Val::from(0).number()?;
                    runtime::local_set(&mut l_card_s, &Val::from(base + 0), Val::from("Poring"), true);
                    runtime::local_set(&mut l_card_s, &Val::from(base + 1), Val::from("Angeling"), true);
                    runtime::local_set(&mut l_card_s, &Val::from(base + 2), Val::from("Ghostring"), true);
                    'l1: loop {
                        if !(l_rugame_turn01.clone().number()? < 5 && l_ruuser_score01.clone().number()? < 3) {
                            break 'l1;
                        }
                        'b1: {
                            ctx.lines_as("House Ghost", args!["Ok, first let me shuffle these cards around.", "Ready!"])?;
                            ctx.call(Function::Emotion, vec![ctx.constant("ET_BLABLA")?])?;
                            ctx.next()?;
                            ctx.lines_as("House Ghost", args!["One!"])?;
                            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_HIT1")?])?;
                            ctx.next()?;
                            ctx.lines_as("House Ghost", args!["One! Two!"])?;
                            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_HIT2")?])?;
                            ctx.next()?;
                            ctx.lines_as("House Ghost", args!["One! Two! Three!"])?;
                            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_HIT3")?])?;
                            ctx.next()?;
                            ctx.lines_as("House Ghost", args!["Ok! I will pick one of them up!"])?;
                            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_STEAL")?])?;
                            ctx.next()?;
                            ctx.lines_as("House Ghost", args!["Ok! I will pick one of them up!", "What is this card?"])?;
                            ctx.call(Function::Cutin, vec![Val::from("sorry.bmp"), Val::from(4)])?;
                            ctx.next()?;
                            l_rucard_game01 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                            l_s = (Val::from(runtime::select_values(ctx, &[Val::from("Poring:Angeling:Ghostring")])?)
                                .try_sub(Val::from(1))?);
                            ctx.lines_as(
                                l_player_name_s.clone(),
                                args![
                                    ((Val::from("Hmmm, I think it is the ") + runtime::local_get(&l_card_s, &l_s.clone(), true))
                                        + Val::from(" Card!"))
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("House Ghost", args!["Ok, time to find out!", "One! Two! Three!!"])?;
                            ctx.next()?;
                            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                            let subject2 = l_rucard_game01.clone();
                            if subject2 == 1 {
                                ctx.call(Function::Cutin, vec![Val::from("����ī��"), Val::from(4)])?;
                                l_win_string_s = Val::from("You are good. Can you do it next time?");
                            } else if subject2 == 2 {
                                ctx.call(Function::Cutin, vec![Val::from("������ī��"), Val::from(4)])?;
                                l_win_string_s = Val::from("You are pretty good at this. Ok, but can you do it again?");
                            } else if subject2 == 3 {
                                ctx.call(Function::Cutin, vec![Val::from("����Ʈ��ī��"), Val::from(4)])?;
                                l_win_string_s = Val::from("You are pretty good at this. Ok, but can you do it again?");
                            }
                            ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
                            if l_rucard_game01.clone().loosely_equals(&(l_s.clone() + Val::from(1))) {
                                ctx.call(
                                    Function::Emotion,
                                    vec![
                                        ctx.constant("ET_AHA")?,
                                        Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("House Ghost", args!["You got it right.", l_win_string_s.clone()])?;
                                l_ruuser_score01 = (l_ruuser_score01.clone() + Val::from(1));
                            } else {
                                ctx.call(
                                    Function::Emotion,
                                    vec![
                                        ctx.constant("ET_HUK")?,
                                        Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("House Ghost", args!["Huuu...", "Better luck next time!"])?;
                            }
                            l_rugame_turn01 = (l_rugame_turn01.clone() + Val::from(1));
                            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                            ctx.next()?;
                        }
                    }
                    if l_ruuser_score01.clone() == 3 {
                        ctx.lines_as("House Ghost", args!["You are better than I thought."])?;
                        ctx.var("rhea_rus_main").set(Val::from(34))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        l_player_name_s.clone(),
                        args!["I was not thinking correctly! Let's do it again!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("House Ghost", args!["Huhu, ok. Let's do it again?"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("rhea_rus_main").get()? == 34 {
                    l_player_name_s = ctx.call(Function::StrCharInfo, vec![Val::from(0)])?;
                    ctx.lines_as(
                        "House Ghost",
                        args!["Ah, it was fun. It has been a long time since I've been able to play a game like that."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "House Ghost",
                        args!["As promised, I will be quiet now. Thank you for playing with me."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(l_player_name_s.clone(), args!["Oh and by the way, I have a present for you!"])?;
                    ctx.next()?;
                    if ctx.call(Function::CountItem, vec![Val::from(519)])?.is_true() {
                        ctx.lines_as(l_player_name_s.clone(), args!["You said you like milk. Here, take this."])?;
                        ctx.next()?;
                    } else {
                        ctx.lines_as(l_player_name_s.clone(), args!["You said you like milk. Here, take.. Eh?"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            l_player_name_s.clone(),
                            args!["W, where is this? Wait here. I will bring it soon!!"],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as("House Ghost", args!["Ah, wow!! Can I have this?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "House Ghost",
                        args![
                            "Thank you so much!!",
                            "I promise I wil be quiet now. I will be the acting guardian for this house!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("House Ghost", args!["I will never forget your kindness of entertaining me."])?;
                    ctx.var("rhea_rus_main").set(Val::from(45))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("rhea_rus_main").get()? == 45 {
                    ctx.lines_as(
                        "House Ghost",
                        args![
                            "Thank you so much!!",
                            "I promise I wil be quiet now. I will be the acting guardian for this house!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("House Ghost", args!["I will never forget your kindness of entertaining me."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
            HouseGhostJarRus43Step::OnDisable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![Val::from("House Ghost Jar#rus43")])?;
                return Err(Stop::End);
            }
            HouseGhostJarRus43Step::OnTimer180000 => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("mosk_dun02"), Val::from("House Ghost Jar#rus43::OnMyMobDead")],
                )?;
                step = HouseGhostJarRus43Step::OnMyMobDead;
                continue 'machine;
            }
            HouseGhostJarRus43Step::OnMyMobDead => {
                ctx.call(Function::EnableNpc, vec![Val::from("House Ghost Jar#rus43")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn house_ghost_jar_rus43(ctx: &Ctx) -> Script {
    house_ghost_jar_rus43_run(ctx, HouseGhostJarRus43Step::Start, Vec::new()).map(|_| ())
}

pub fn house_ghost_jar_rus43_ondisable(ctx: &Ctx) -> Script {
    house_ghost_jar_rus43_run(ctx, HouseGhostJarRus43Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn house_ghost_jar_rus43_ontimer180000(ctx: &Ctx) -> Script {
    house_ghost_jar_rus43_run(ctx, HouseGhostJarRus43Step::OnTimer180000, Vec::new()).map(|_| ())
}

pub fn house_ghost_jar_rus43_onmymobdead(ctx: &Ctx) -> Script {
    house_ghost_jar_rus43_run(ctx, HouseGhostJarRus43Step::OnMyMobDead, Vec::new()).map(|_| ())
}

fn broom_grandma_rus44_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("rhea_rus_main").get()?.number()? < 36 {
        ctx.lines_as("Broom Grandma", args!["A broom from Payon is the best!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Broom Grandma",
            args!["It is so good that even ghosts want only a Payon broom."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("rhea_rus_main").get()? == 36 {
        ctx.lines_as("Broom Grandma", args!["A broom from Payon is best!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Broom Grandma",
            args!["It is so good that even ghosts want only a Payon broom."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["I want to buy a broom."],
        )?;
        ctx.next()?;
        ctx.lines_as("Broom Grandma", args!["Oh, you are a guest. Yes, you need a broom?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Broom Grandma",
            args![
                "But, I have no broom. Ghosts stole all my newly made brooms. I was pleased because they were nice even for ghosts. But..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Broom Grandma",
            args!["My son went to their place to get back the brooms, but he failed since the ghosts tricked him."],
        )?;
        ctx.next()?;
        ctx.lines_as("Broom Grandma", args!["Ah.. What should I do..."])?;
        ctx.var("rhea_rus_main").set(Val::from(37))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("rhea_rus_main").get()?.number()? > 36 && ctx.var("rhea_rus_main").get()?.number()? < 41) {
        ctx.lines_as(
            "Broom Grandma",
            args!["There are ghosts inside the dungeon of Payon. There is a tree there where the ghosts is hiding."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Broom Grandma",
            args!["My son went to their place to get back the brooms, but he failed since the ghosts tricked him."],
        )?;
        ctx.next()?;
        ctx.lines_as("Broom Grandma", args!["Ah.. What should I do..."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("rhea_rus_main").get()? == 46 {
        ctx.lines_as(
            "Broom Grandma",
            args!["Ah, you have taken them back. You are a great adventurer."],
        )?;
        ctx.next()?;
        ctx.lines_as("Broom Grandma", args!["You can have that broom. I believe it is its destiny."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as("Broom Grandma", args!["A broom from Payon is the best!"])?;
    ctx.next()?;
    ctx.lines_as(
        "Broom Grandma",
        args!["It is so good that even ghosts want only a Payon broom."],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn broom_grandma_rus44(ctx: &Ctx) -> Script {
    broom_grandma_rus44_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum GhostTreeRus45Step {
    Start,
    SQuestion,
}

fn ghost_tree_rus45_run(ctx: &Ctx, mut step: GhostTreeRus45Step, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_emo: Vec<Val> = Vec::new();
    let mut l_input_s = Val::from("");
    let mut l_player_name_s = Val::from("");
    let mut l_rus_dice01 = Val::from(0);
    let mut l_rus_kafra01 = Val::from(0);
    let mut l_rustree_turn01 = Val::from(0);
    let mut l_rususer_score01 = Val::from(0);
    let mut l_tree_dice01 = Val::from(0);
    'machine: loop {
        match step {
            GhostTreeRus45Step::Start => {
                if ctx.var("rhea_rus_main").get()? == 37 {
                    ctx.lines_as(
                        "Ghost Tree",
                        args!["Errr, a human? What are you doing here? Do you want to play with me?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Return the broom."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ghost Tree",
                        args!["Kuhuhuhu, if you want it, you must give me the correct answers to my questions!"],
                    )?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("Ok, ask me!:No, just give it to me!")],
                    )?) == 2
                    {
                        ctx.lines_as(
                            "Ghost Tree",
                            args!["What!? You are rude to ask for what you want by doing nothing!"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Ghost Tree", args!["Get away! You must be punished!"])?;
                        ctx.call(
                            Function::StartStatus,
                            vec![ctx.constant("SC_CONFUSION")?, Val::from(60000), Val::from(0)],
                        )?;
                        ctx.close_window()?;
                        ctx.call(Function::Warp, vec![Val::from("pay_dun04"), Val::from(46), Val::from(43)])?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        "Ghost Tree",
                        args![
                            "Kuhuhu, you a cool kid?!",
                            "I like a person confident as talented. Let's begin the test!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ghost Tree",
                        args!["Eh.. Let's see. This will be a good start! Let's get started if you are ready!"],
                    )?;
                    ctx.var("rhea_rus_main").set(Val::from(38))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("rhea_rus_main").get()? == 38 {
                    ctx.lines_as(
                        "Ghost Tree",
                        args!["First question. Listen to me carefully and answer the question!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ghost Tree",
                        args!["Kafra sisters had a race. Listen to what they say and guess their ranking in the race. Understand?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Ghost Tree", args!["Ok, then, listen to them."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kafra Roxie",
                        args!["I am not good at running but Jasmine won over me and I won over Blossom."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Kafra Blossom", args!["I think that the smaller you are, the less the air disturbs you. Curly Sue was so fast that I couldn't follow her."])?;
                    ctx.next()?;
                    ctx.lines_as("Kafra Jasmine", args!["I won over Pavianne, but not Curly Sue. Huhu."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kafra Curly Sue",
                        args!["Wohuhu, it was difficult to race with the fast runners. Anyway, I am pleased to win over Roxie!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kafra Pavianne",
                        args!["...I wasn't the last one even though Roxie won over me..."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Ghost Tree", args!["Ok then, who was the first?!"])?;
                    ctx.next()?;
                    l_rus_kafra01 = ghost_tree_rus45_run(ctx, GhostTreeRus45Step::SQuestion, vec![Val::from("Curly Sue")])?;
                    ctx.lines_as("Ghost Tree", args!["Who was the second?"])?;
                    ctx.next()?;
                    l_rus_kafra01 =
                        (l_rus_kafra01.clone() + ghost_tree_rus45_run(ctx, GhostTreeRus45Step::SQuestion, vec![Val::from("Jasmine")])?);
                    ctx.lines_as("Ghost Tree", args!["Who was the third?"])?;
                    ctx.next()?;
                    l_rus_kafra01 =
                        (l_rus_kafra01.clone() + ghost_tree_rus45_run(ctx, GhostTreeRus45Step::SQuestion, vec![Val::from("Roxie")])?);
                    ctx.lines_as("Ghost Tree", args!["And then who was the forth?"])?;
                    ctx.next()?;
                    l_rus_kafra01 =
                        (l_rus_kafra01.clone() + ghost_tree_rus45_run(ctx, GhostTreeRus45Step::SQuestion, vec![Val::from("Pavianne")])?);
                    ctx.lines_as("Ghost Tree", args!["And who was the last?!"])?;
                    ctx.next()?;
                    l_rus_kafra01 =
                        (l_rus_kafra01.clone() + ghost_tree_rus45_run(ctx, GhostTreeRus45Step::SQuestion, vec![Val::from("Blossom")])?);
                    ctx.mes("[Ghost Tree]")?;
                    if l_rus_kafra01.clone().number()? > 4 {
                        ctx.mes("Ho, you are good!")?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ghost Tree",
                            args!["Ok, let's see if you are good next time.", "Gear yourself up!"],
                        )?;
                        ctx.var("rhea_rus_main").set(Val::from(39))?;
                    } else {
                        ctx.lines(args![
                            "Ah, you are wrong, wrong!",
                            "Do you really think that you can take the brooms back?"
                        ])?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["I was not thinking correctly! I will try it again!"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Ghost Tree", args!["Kuhuhu, ok. Let's do it again. Try your best."])?;
                    }
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("rhea_rus_main").get()? == 39 {
                    l_player_name_s = ctx.call(Function::StrCharInfo, vec![Val::from(0)])?;
                    ctx.lines_as("Ghost Tree", args!["The second question. Listen carefully and answer!"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ghost Tree",
                        args![
                            ((Val::from("One day, you, ") + l_player_name_s.clone())
                                + Val::from(", upgraded your 'Hat of the Sun God', your ancestral treasure."))
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(l_player_name_s.clone(), args!["Eh? I didn't..."])?;
                    ctx.next()?;
                    ctx.lines_as("Ghost Tree", args!["Shut up! Listen to me!", "While Aragam, Antonio, Hermanthorn, and Holgren, 4 blacksmiths, upgraded the 'Hat of the Sun God' in turn, it was broken!"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ghost Tree",
                        args!["The stauts of 'Hat of the Sun God' was +7. Here, you guess who broke it!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Aragam", args!["I heard that Holgren upraded it up to +7. It was overpace."])?;
                    ctx.next()?;
                    ctx.lines_as("Antonio", args!["...We upgraded it twice for each..."])?;
                    ctx.next()?;
                    ctx.lines_as("Hermanthorn", args!["Hogren worked after me."])?;
                    ctx.next()?;
                    ctx.lines_as("Holgren", args!["I did it after Aragam!"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ghost Tree",
                        args!["Have you heard all of them? Who broke the 'Hat of the Sun God'?!"],
                    )?;
                    ctx.next()?;
                    let (input, status) = runtime::input_text(ctx, None, None)?;
                    l_input_s = input;
                    ctx.lines_as(
                        l_player_name_s.clone(),
                        args![((Val::from("") + l_input_s.clone()) + Val::from(" !!"))],
                    )?;
                    ctx.next()?;
                    if l_input_s.clone() != "Antonio" {
                        ctx.lines_as(
                            "Ghost Tree",
                            args![
                                "You are wrong, wrong!",
                                "Look at this. In this way, it will take you a million years to take them back!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            l_player_name_s.clone(),
                            args!["I was not thinking correctly!!!", "Again, again!"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Ghost Tree", args!["Kuhuhu, Ok then. You should do better next time!"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        "Ghost Tree",
                        args![
                            "Good! Well done. The last one is to cast a dice with me! If you win over me, I will give you what you want!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Ghost Tree", args!["Gear yourself up and come to me again!"])?;
                    ctx.var("rhea_rus_main").set(Val::from(40))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("rhea_rus_main").get()? == 40 {
                    l_player_name_s = ctx.call(Function::StrCharInfo, vec![Val::from(0)])?;
                    ctx.lines_as(
                        "Ghost Tree",
                        args!["Kuhuhu, you did good until now. If you win over me, I will give the brooms as promisied."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Ghost Tree", args!["The game is simple. Your number must be more than mine."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ghost Tree",
                        args!["Cast 3 times if you win once, I will give you what you want."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(l_player_name_s.clone(), args!["Ok! I will do it!"])?;
                    ctx.next()?;
                    let base = Val::from(1).number()?;
                    runtime::local_set(&mut l_emo, &Val::from(base + 0), ctx.constant("ET_DICE1")?, false);
                    runtime::local_set(&mut l_emo, &Val::from(base + 1), ctx.constant("ET_DICE2")?, false);
                    runtime::local_set(&mut l_emo, &Val::from(base + 2), ctx.constant("ET_DICE3")?, false);
                    runtime::local_set(&mut l_emo, &Val::from(base + 3), ctx.constant("ET_DICE4")?, false);
                    runtime::local_set(&mut l_emo, &Val::from(base + 4), ctx.constant("ET_DICE5")?, false);
                    runtime::local_set(&mut l_emo, &Val::from(base + 5), ctx.constant("ET_DICE6")?, false);
                    'l1: loop {
                        if !(l_rustree_turn01.clone().number()? < 3 && l_rususer_score01.clone() == 0) {
                            break 'l1;
                        }
                        'b1: {
                            ctx.lines_as("Ghost Tree", args!["I cast first."])?;
                            ctx.next()?;
                            ctx.mes("- Ghost Tree casts a dice. The dice falls down, rotates and stops there-")?;
                            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_STUNATTACK")?])?;
                            ctx.next()?;
                            l_tree_dice01 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(6)])?;
                            ctx.lines_as(
                                "Ghost Tree",
                                args![((Val::from("I've got ^0000ffNumber ") + l_tree_dice01.clone()) + Val::from("^000000."))],
                            )?;
                            ctx.call(
                                Function::Emotion,
                                vec![runtime::local_get(&l_emo, &l_tree_dice01.clone(), false)],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(l_player_name_s.clone(), args!["Ok, it is my turn?"])?;
                            ctx.next()?;
                            ctx.mes("-You cast a dice. The dice falls down, rotates and stops there-")?;
                            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_STUNATTACK")?])?;
                            ctx.next()?;
                            l_rus_dice01 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(6)])?;
                            ctx.lines_as(l_player_name_s.clone(), args!["Let's see..."])?;
                            ctx.next()?;
                            ctx.call(
                                Function::Emotion,
                                vec![
                                    runtime::local_get(&l_emo, &l_rus_dice01.clone(), false),
                                    Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                                ],
                            )?;
                            ctx.lines(args![((Val::from("[") + l_player_name_s.clone()) + Val::from("]"))])?;
                            if l_rus_dice01.clone().loosely_equals(&l_tree_dice01.clone()) {
                                ctx.lines(args![
                                    ((Val::from("Let's see... Wow, I got it! I've got ^0000ffNumber ") + l_rus_dice01.clone())
                                        + Val::from("^000000!"))
                                ])?;
                                ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
                                ctx.next()?;
                                ctx.lines_as("Ghost Tree", args!["You seem lucky..."])?;
                                l_rususer_score01 = Val::from(1);
                            } else {
                                ctx.lines(args![
                                    ((Val::from("Let's see................... It is ^0000ff ") + l_rus_dice01.clone())
                                        + Val::from(" ^000000...")),
                                    "[Ghost Tree]",
                                    "Huuu..."
                                ])?;
                                ctx.call(Function::Emotion, vec![ctx.constant("ET_SMILE")?])?;
                            }
                            l_rustree_turn01 = (l_rustree_turn01.clone() + Val::from(1));
                            ctx.next()?;
                        }
                    }
                    if l_rususer_score01.clone().is_true() {
                        ctx.lines_as("Ghost Tree", args!["Ah. I lost, but it was fun."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ghost Tree",
                            args!["For the fun play, I will give the broom to you as promised."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ghost Tree",
                            args!["I can say that you are fun. If possible, play with me again. And take them."],
                        )?;
                        ctx.next()?;
                        ctx.mes("- ^0000ff You receive the best broom from Payon !!^000000 - ")?;
                        ctx.var("rhea_rus_main").set(Val::from(46))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        "Ghost Tree",
                        args!["Kuhuhu, nobody is worse than you. Remember that you have to win over me to get what you want"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(l_player_name_s.clone(), args!["I was not what I am! Let's do it again!"])?;
                    ctx.next()?;
                    ctx.lines_as("Ghost Tree", args!["Ok, let's play again. If prepared, try again?"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("rhea_rus_main").get()? == 46 {
                    ctx.lines_as(
                        "Ghost Tree",
                        args!["Kuhuhu, I can say that you are fun. If possible, play with me again."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
            GhostTreeRus45Step::SQuestion => {
                let (input, status) = runtime::input_text(ctx, None, None)?;
                l_input_s = input;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![((Val::from("") + l_input_s.clone()) + Val::from(" !!"))],
                )?;
                ctx.next()?;
                return Ok((if l_input_s.clone().loosely_equals(&runtime::arg(&args, 0, Val::from(0))) {
                    Val::from(1)
                } else {
                    Val::from(0)
                }));
            }
        }
    }
}

pub fn ghost_tree_rus45(ctx: &Ctx) -> Script {
    ghost_tree_rus45_run(ctx, GhostTreeRus45Step::Start, Vec::new()).map(|_| ())
}

fn koshei_globalvar_admin_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    shared::other_gm_npcs::f_gm_npc(ctx, vec![])?;
    ctx.lines_as("Koshei GlobalVar", args!["Please enter the password"])?;
    l_i = shared::other_gm_npcs::f_gm_npc(ctx, vec![Val::from("orchid"), Val::from(1)])?;
    ctx.next()?;
    if l_i.clone() == 0 {
        ctx.lines_as("Koshei GlobalVar", args!["Please input the password exactly."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Koshei GlobalVar",
            args![
                "I can tell you what the",
                "Koshei GlobalVar",
                "$@rus_req02",
                "on mosk_dun01",
                "is currently set to."
            ],
        )?;
        ctx.next()?;
        if ctx.var("$@rus_req02").get()? == 0 {
            ctx.lines_as("Koshei GlobalVar", args!["Currently the GlobalVar $@rus_req02 is set to 0"])?;
            ctx.next()?;
        } else if ctx.var("$@rus_req02").get()? == 1 {
            ctx.lines_as("Koshei GlobalVar", args!["Currently the GlobalVar $@rus_req02 is set to 1"])?;
            ctx.next()?;
        } else {
            ctx.lines_as("Koshei GlobalVar", args!["error"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as("Koshei GlobalVar", args!["What would you like to set the GlobalVar to?"])?;
        ctx.next()?;
        'b1: {
            let subject1 = Val::from(runtime::select_values(ctx, &[Val::from("0:1:Cancel")])?);
            let mut matched1 = false;
            let no_case1 = !subject1.loosely_equals(&Val::from(1))
                && !subject1.loosely_equals(&Val::from(2))
                && !subject1.loosely_equals(&Val::from(3));
            if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as("Koshei GlobalVar", args!["GlobalVar $@rus_req02 will now be set to '0'"])?;
                ctx.var("$@rus_req02").set(Val::from(0))?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Koshei#rus47::OnDisable")])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as("Koshei GlobalVar", args!["GlobalVar $@rus_req02 will now be set to '1'"])?;
                ctx.var("$@rus_req02").set(Val::from(1))?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Koshei#rus47::OnEnable")])?;
            }
            if !matched1 && subject1.loosely_equals(&Val::from(3)) {
                matched1 = true;
            }
            if matched1 {
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
    Ok(Val::from(0))
}

pub fn koshei_globalvar_admin(ctx: &Ctx) -> Script {
    koshei_globalvar_admin_body(ctx, Vec::new()).map(|_| ())
}

use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn friar_patrick_edq_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_doll = Val::from(0);
    let mut l_ins_bapho_check = Val::from(0);
    let mut l_party_id = Val::from(0);
    let mut l_selection = Val::from(0);
    ctx.call(Function::Cutin, vec![Val::from("ins_cata_pri_n"), Val::from(2)])?;
    ctx.lines_as(
        "Friar Patrick",
        args!["The peace of this world cannot last forever... The hands of Evil are reaching into the world again..."],
    )?;
    ctx.next()?;
    ctx.lines_as("Friar Patrick", args!["What brought you to this place?"])?;
    ctx.next()?;
    if ctx.call(Function::CountItem, vec![Val::from(6004)])?.number()? > 0 {
        l_doll = Val::from(1);
        l_selection = Val::from(runtime::select_values(
            ctx,
            &[Val::from(
                "What is this place?:I want to enter.:About the Cursed Baphomet Doll.:Cancel.",
            )],
        )?);
    } else {
        l_selection = Val::from(runtime::select_values(
            ctx,
            &[Val::from("What is this place?:I want to enter.:Cancel.")],
        )?);
    }
    'b1: {
        let subject1 = l_selection.clone();
        let mut matched1 = false;
        let no_case1 = !subject1.loosely_equals(&Val::from(1))
            && !subject1.loosely_equals(&Val::from(2))
            && !subject1.loosely_equals(&Val::from(3))
            && !subject1.loosely_equals(&Val::from(4));
        if !matched1 && subject1.loosely_equals(&Val::from(1)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Friar Patrick",
                args![
                    "Huh... Don't you know? This is St. Capitolina Monastery where the Brothers who wish to become monks train and pray."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Friar Patrick",
                args!["And this place is... What can I say... Yes. It's the grave of the Devil. Grave..."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Friar Patrick",
                args!["The very place where the great Devil who once demolished this world is sleeping."],
            )?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Tell me more.:Stop talking.")])? {
                1 => {
                    ctx.call(Function::Cutin, vec![Val::from("ins_cata_pri_n"), Val::from(2)])?;
                    ctx.lines_as(
                        "Friar Patrick",
                        args!["Baphomet... is the name of the Devil... I think you have heard of his name."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Friar Patrick",
                        args!["Numerous brave men and brothers have trained in this monastery..."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Friar Patrick",
                        args!["Under this gravestone in front of you... Baphomet is sealed."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Friar Patrick", args!["But... As we speak Satan Morocc is marshalling his powerful magic in order to affect all life on this continent."])?;
                    ctx.next()?;
                    ctx.lines_as("Friar Patrick", args!["Baphomet also... has awakened and is preparing for his revival, into this world, by weakening the power of the seal through the power of Satan Morocc..."])?;
                    ctx.next()?;
                    ctx.lines_as("Friar Patrick", args!["Now... I'm looking for someone brave enough to reseal Baphomet in its shrine... as we once did many years ago..."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Friar Patrick",
                        args!["Anyone who fights for good will know, deep inside, that evil is threatening to conquer this world..."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Friar Patrick",
                        args!["Train more and use your skills to protect the world from evil's looming presence..."],
                    )?;
                }
                2 => {
                    ctx.call(Function::Cutin, vec![Val::from("ins_cata_pri_n"), Val::from(2)])?;
                    ctx.lines_as(
                        "Friar Patrick",
                        args!["Anyone who fights for good will know, deep inside, that evil is threatening to conquer this world..."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Friar Patrick",
                        args!["Train more and use your skills to protect the world from evil's looming presence..."],
                    )?;
                }
                _ => {}
            }
            break 'b1;
        }
        if !matched1 && subject1.loosely_equals(&Val::from(2)) {
            matched1 = true;
        }
        if matched1 {
            if ctx.var("BaseLevel").get()?.number()? >= 75 {
                l_party_id = ctx.call(Function::GetCharacterId, vec![Val::from(1)])?;
                ctx.lines_as(
                    "Friar Patrick",
                    args!["Do you mean you'll go to the shrine and reseal Baphomet?"],
                )?;
                ctx.next()?;
                l_ins_bapho_check = ctx.call(Function::CheckQuest, vec![Val::from(3040), ctx.constant("PLAYTIME")?])?;
                if l_ins_bapho_check.clone() == -1 {
                    if (ctx
                        .call(Function::IsPartyLeader, vec![l_party_id.clone()])?
                        .loosely_equals(&Val::from(1))
                        && ctx
                            .call(
                                Function::InstanceCheckParty,
                                vec![l_party_id.clone(), Val::from(2), Val::from(75)],
                            )?
                            .is_true())
                    {
                        ctx.lines_as(
                            "Friar Patrick",
                            args![
                                ((Val::from("Party name is ") + ctx.call(Function::GetPartyName, vec![l_party_id.clone()])?)
                                    + Val::from("...")),
                                ((Val::from("Name of the leader is ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                    + Val::from("..."))
                            ],
                        )?;
                        if ctx.call(Function::InstanceCreate, vec![Val::from("Sealed Catacomb")])?.number()? < 0 {
                            ctx.mes("Umm... But it seems that there is a problem here... I'll check quickly. Please wait.")?;
                        } else {
                            ctx.mes("Okay... I'll adjust the shrine's seal so that you and your group can enter.")?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Friar Patrick",
                                args!["You will see a sign when the seal has broken. Please wait until the sign appears..."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Friar Patrick",
                                args!["When you see the sign, put your hands on the gravestone... Then you can move inside."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Friar Patrick", args!["One thing that you should remember is... Anyone who enters this shrine will be cursed by Baphomet and cannot enter or leave while they are cursed."])?;
                            ctx.next()?;
                            ctx.lines_as("Friar Patrick", args!["And one more thing... In this cursed area, some skills, which are protected by outer physical power are prohibited by the effect of the seal."])?;
                            ctx.next()?;
                            ctx.lines_as("Friar Patrick", args!["For example, the skills like ^0000FFSafety Wall and Assumptio^000000... So you'd better prepare before entering the shrine."])?;
                        }
                    } else {
                        ctx.lines_as("Friar Patrick", args!["Umm... I recognize your courage, but... I can't permit anyone to enter this place. I can only permit the leader of a party to enter first."])?;
                        ctx.next()?;
                        ctx.lines_as("Friar Patrick", args!["Once the party leader is permitted, the rest of the party can enter. This is a rule of this monastery, so please understand."])?;
                    }
                } else {
                    if (l_ins_bapho_check.clone() == 0 || l_ins_bapho_check.clone() == 1) {
                        ctx.lines_as("Friar Patrick", args!["It seems you have entered this shrine recently... You cannot reenter because Baphomet's Curse still remains. Baphomet's Curse disappears only after a certain amount of time has passed."])?;
                    } else if l_ins_bapho_check.clone() == 2 {
                        ctx.lines_as(
                            "Friar Patrick",
                            args!["Umm... It seems that Baphomet's Curse has weakened. I can remove it now."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Friar Patrick", args!["Haaaaaaap... Hocus Pocus Wingardium Abracadabra!!!!!"])?;
                        ctx.next()?;
                        ctx.call(Function::EraseQuest, vec![Val::from(3040)])?;
                        if ctx.call(Function::CheckQuest, vec![Val::from(3041)])?.number()? >= 0 {
                            ctx.call(Function::EraseQuest, vec![Val::from(3041)])?;
                        }
                        if ctx.call(Function::CheckQuest, vec![Val::from(3045)])?.number()? >= 0 {
                            ctx.call(Function::EraseQuest, vec![Val::from(3045)])?;
                        }
                        ctx.lines_as(
                            "Friar Patrick",
                            args!["Huu... It's over. Now that I've released Baphomet's Curse, you can enter again."],
                        )?;
                    }
                }
            } else {
                ctx.lines_as(
                    "Friar Patrick",
                    args!["Umm... You should train more to enter this dangerous place... You should reach at least Lv 75 to enter here."],
                )?;
                ctx.next()?;
                ctx.lines_as("Friar Patrick", args!["Please train more and come again."])?;
            }
            break 'b1;
        }
        if !matched1 && subject1.loosely_equals(&Val::from(3)) {
            matched1 = true;
        }
        if matched1 && l_doll.clone() == 1 {
            ctx.lines_as(
                "Friar Patrick",
                args!["That is... the villainous doll that you are holding... Let me see it."],
            )?;
            ctx.next()?;
            ctx.lines_as("Friar Patrick", args!["... ... ..."])?;
            ctx.next()?;
            ctx.lines_as(
                "Friar Patrick",
                args!["That's correct... I can feel Baphomet's evil inside... So, what will you do with the doll?"],
            )?;
            ctx.next()?;
            ctx.lines_as("Friar Patrick", args!["It is just a dangerous and useless thing if you do nothing with it... I'll introduce someone who can help you... Do you want to see him?"])?;
            ctx.next()?;
            ctx.lines_as("Friar Patrick", args!["Go to see ^0000FFRust Blackhand^000000 who is near the main building of the monastery... He will make this doll helpful to you."])?;
            ctx.call(Function::SetQuest, vec![Val::from(3042)])?;
            break 'b1;
        }
        if !matched1 && subject1.loosely_equals(&Val::from(4)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Friar Patrick",
                args!["Anyone who fights for good will know, deep inside, that evil is threatening to conquer this world..."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Friar Patrick",
                args!["Train more and use your skills to protect the world from evil's looming presence..."],
            )?;
            break 'b1;
        }
    }
    ctx.close_window()?;
    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
    return Err(Stop::End);
}

pub fn friar_patrick_edq(ctx: &Ctx) -> Script {
    friar_patrick_edq_body(ctx, Vec::new()).map(|_| ())
}

fn grave_of_baphomet_edq_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_ins_bapho_check = Val::from(0);
    let mut l_party_id = Val::from(0);
    if ctx.call(Function::CountItem, vec![Val::from(6002)])?.is_true() {
        ctx.call(
            Function::DelItem,
            vec![Val::from(6002), ctx.call(Function::CountItem, vec![Val::from(6002)])?],
        )?;
    }
    ctx.mes("This gravestone has a carving of a wicked devil with large horns. It arouses an ominous feeling.")?;
    ctx.next()?;
    if Val::from(runtime::select_values(ctx, &[Val::from("Touch the stone.:Step back.")])?) == 2 {
        ctx.close_window()?;
        return Err(Stop::End);
    }
    l_ins_bapho_check = ctx.call(Function::CheckQuest, vec![Val::from(3040), ctx.constant("PLAYTIME")?])?;
    if l_ins_bapho_check.clone() == -1 {
        'b1: {
            let subject1 = ctx.call(Function::InstanceEnter, vec![Val::from("Sealed Catacomb")])?;
            let mut matched1 = false;
            let no_case1 = !subject1.loosely_equals(&ctx.constant("IE_OTHER")?)
                && !subject1.loosely_equals(&ctx.constant("IE_NOINSTANCE")?)
                && !subject1.loosely_equals(&ctx.constant("IE_NOMEMBER")?)
                && !subject1.loosely_equals(&ctx.constant("IE_OK")?);
            if !matched1 && subject1.loosely_equals(&ctx.constant("IE_OTHER")?) {
                matched1 = true;
            }
            if !matched1 && subject1.loosely_equals(&ctx.constant("IE_NOINSTANCE")?) {
                matched1 = true;
            }
            if matched1 {
                ctx.mes("It's cold to the touch. It doesn't respond.")?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if !matched1 && subject1.loosely_equals(&ctx.constant("IE_NOMEMBER")?) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as(
                    "Friar Patrick",
                    args!["To enter this dangerous place, you can't go alone. Come again after you join a party."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if !matched1 && subject1.loosely_equals(&ctx.constant("IE_OK")?) {
                matched1 = true;
            }
            if matched1 {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("monk_test"),
                        ((((Val::from("[") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("] member of the ["))
                            + ctx.call(Function::GetPartyName, vec![l_party_id.clone()])?)
                            + Val::from("] party has entered the Sealed Shrine.")),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x00ff99"),
                    ],
                )?;
                ctx.call(Function::SetQuest, vec![Val::from(3040)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    } else if (l_ins_bapho_check.clone() == 0 || l_ins_bapho_check.clone() == 1) {
        ctx.lines_as(
            "Friar Patrick",
            args![
                "It seems you have entered this shrine recently... You cannot reenter because the curse of Baphomet still remains.",
                "The curse of Baphomet disappears after a certain amount of time after you entered."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if l_ins_bapho_check.clone() == 2 {
        ctx.lines_as(
            "Friar Patrick",
            args!["Umm... It seems the curse of Baphomet weakened. I'll clear the bad curse."],
        )?;
        ctx.next()?;
        ctx.lines_as("Friar Patrick", args!["Haaaaaaap... Wingardium Leviosa Expecto Patronum !!!!!"])?;
        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_HOLYHIT")?])?;
        ctx.call(Function::EraseQuest, vec![Val::from(3040)])?;
        if ctx.call(Function::CheckQuest, vec![Val::from(3041)])?.number()? >= 0 {
            ctx.call(Function::EraseQuest, vec![Val::from(3041)])?;
        }
        if ctx.call(Function::CheckQuest, vec![Val::from(3045)])?.number()? >= 0 {
            ctx.call(Function::EraseQuest, vec![Val::from(3045)])?;
        }
        ctx.next()?;
        ctx.lines_as(
            "Friar Patrick",
            args!["Huu... It's over. Now I released all of the curses on you. You can enter again."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn grave_of_baphomet_edq(ctx: &Ctx) -> Script {
    grave_of_baphomet_edq_body(ctx, Vec::new()).map(|_| ())
}

fn rust_blackhand_edq_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_new_maje = Val::from(0);
    l_new_maje = ctx.call(Function::CheckQuest, vec![Val::from(3043)])?;
    ctx.lines_as("Rust Blackhand", args!["Who are you? What do you want me for?"])?;
    ctx.next()?;
    ctx.lines_as("Rust Blackhand", args!["You're not a monk, then what do you want?"])?;
    ctx.next()?;
    if (l_new_maje.clone() == 0 || l_new_maje.clone() == 1) {
        ctx.lines_as("Rust Blackhand", args!["Did you bring all of the ingredients?"])?;
        ctx.next()?;
        if ((((ctx.call(Function::CountItem, vec![Val::from(6004)])?.number()? > 0
            && ctx.call(Function::CountItem, vec![Val::from(2256)])?.number()? > 0)
            && ctx.call(Function::CountItem, vec![Val::from(7799)])?.number()? > 29)
            && ctx.call(Function::CountItem, vec![Val::from(7798)])?.number()? > 49)
            && ctx.var("Zeny").get()?.number()? > 990000)
        {
            ctx.lines_as(
                "Rust Blackhand",
                args!["kkk... You prepared the ingredients well. Why don't you leave it there and wait?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Rust Blackhand",
                args!["Hmm... It's been such a long time since I last saw these wicked horns... kkk... Let me start..."],
            )?;
            ctx.next()?;
            ctx.mes("...")?;
            ctx.next()?;
            ctx.mes("... ...")?;
            ctx.next()?;
            ctx.mes("... ... ...")?;
            ctx.next()?;
            ctx.call(Function::DelItem, vec![Val::from(6004), Val::from(1)])?;
            ctx.call(Function::DelItem, vec![Val::from(2256), Val::from(1)])?;
            ctx.call(Function::DelItem, vec![Val::from(7799), Val::from(30)])?;
            ctx.call(Function::DelItem, vec![Val::from(7798), Val::from(50)])?;
            ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(990000))?))?;
            ctx.call(Function::CompleteQuest, vec![Val::from(3043)])?;
            ctx.call(Function::GetItem, vec![Val::from(5374), Val::from(1)])?;
            ctx.lines_as(
                "Rust Blackhand",
                args!["It's done. You may be excited, of course. I understand..."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Rust Blackhand",
                args!["I don't accept complaints or A/S requests, so use it with care. I must go..."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Rust Blackhand",
                args!["Huu... You don't understand what I said. You cannot make anything with these ingredients."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Rust Blackhand",
                args!["I'll tell you the ingredients one more time, so please gather them correctly."],
            )?;
            ctx.next()?;
            ctx.lines_as("Rust Blackhand", args!["^0000FFCursed Baphomet Doll, Magestic Goat, 30 Crystal of Darkness, 50 Fragment of Darkness^000000, and the most important, production cost is ^0000FF990000^000000 Zeny."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else if (l_new_maje.clone() == -1 && ctx.call(Function::CountItem, vec![Val::from(6004)])?.number()? > 0) {
        'b1: {
            let subject1 = Val::from(runtime::select_values(
                ctx,
                &[Val::from("About the Cursed Baphomet Doll:Stop talking.")],
            )?);
            let mut matched1 = false;
            let no_case1 = !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2));
            if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as(
                    "Rust Blackhand",
                    args!["What?... Hmmm... Did you get the doll? You're pretty good, unlike your appearance..."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Rust Blackhand",
                    args!["Let me see... Needless to say, Patric must have sent you here to deal with the doll, right?"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Rust Blackhand",
                    args!["Cool... I'll help you make the evil doll useful. What? What can I do?"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Rust Blackhand",
                    args!["I can make the strong and large horns of the wicked devil Baphomet for you. A helm that has his immense power."],
                )?;
                ctx.next()?;
                ctx.lines_as("Rust Blackhand", args!["It is called the ^4d4dffGigantic Magestic Goat^000000. You'll realize that the Magestic Goat you're familiar with is nothing in comparison."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Rust Blackhand",
                    args![
                        "The Cursed Baphomet Doll is the most important ingredient... I'll make you if you want. What would you like to do?"
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("I want to make one!:I don't need one.")])? {
                    1 => {
                        ctx.lines_as("Rust Blackhand", args!["kkk... Yes, wise men take their chances when the opportunity comes. I'll tell you the ingredients. Don't forget, and bring them all."])?;
                        ctx.next()?;
                        ctx.lines_as("Rust Blackhand", args!["^0000FFCursed Baphomet Doll, Magestic Goat, 30 Crystal of Darkness, 50 Fragment of Darkness^000000, and the most important, production cost is ^0000FF990000^000000 Zeny."])?;
                        ctx.next()?;
                        ctx.lines_as("Rust Blackhand", args!["You can get the Magestic Goat from the weak Baphomet in the Labyrinth Forest. Crystal of Darkness and Fragment of Darkness are from the Incarnation of Morocc."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Rust Blackhand",
                            args!["I'm sure that you can get the ingredients because you sealed the real Baphomet. Can't you? kkk..."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Rust Blackhand",
                            args![
                                "You'll never know how great this hat is until you get one. If you understood, go and get the ingredients."
                            ],
                        )?;
                        if ctx.call(Function::IsBeginQuest, vec![Val::from(3042)])?.is_true() {
                            ctx.call(Function::ChangeQuest, vec![Val::from(3042), Val::from(3043)])?;
                        } else {
                            ctx.call(Function::SetQuest, vec![Val::from(3043)])?;
                        }
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as(
                            "Rust Blackhand",
                            args!["Huh... Do you? Do whatever you want... Do you really want to let this opportunity go to waste?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Rust Blackhand",
                            args!["Tut, tut... I don't care if the wicked doll threatens your life all the time!"],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            }
            if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as("Rust Blackhand", args!["What a dull boy he is... huh..."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    } else if (l_new_maje.clone() == -1 && ctx.call(Function::CountItem, vec![Val::from(6004)])? == 0) {
        ctx.lines_as("Rust Blackhand", args!["If you don't have business with me, go away! As you see, I make equipment for the Brothers at the monastery, not for adventurers like you. Do you understand?"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (l_new_maje.clone() == 2 && ctx.call(Function::CountItem, vec![Val::from(6004)])?.number()? > 0) {
        'b3: {
            let subject3 = Val::from(runtime::select_values(
                ctx,
                &[Val::from("About the Cursed Baphomet Doll:Stop talking.")],
            )?);
            let mut matched3 = false;
            let no_case3 = !subject3.loosely_equals(&Val::from(1)) && !subject3.loosely_equals(&Val::from(2));
            if !matched3 && subject3.loosely_equals(&Val::from(1)) {
                matched3 = true;
            }
            if matched3 {
                ctx.lines_as("Rust Blackhand", args!["What?... You again? What do you want this time?"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Rust Blackhand",
                    args!["What? You got another doll from the wicked devil? Umm... You're much better than I thought..."],
                )?;
                ctx.next()?;
                ctx.lines_as("Rust Blackhand", args!["Alright... I'll help you again."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Rust Blackhand",
                    args!["If you want to make the doll into a ^4d4dffGigantic Magestic Goat^000000 again, I can make you another."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Rust Blackhand",
                    args!["I'll tell you the ingredients again. So, do you want to make?"],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("I want.:No, I don't want.")])? {
                    1 => {
                        ctx.lines_as("Rust Blackhand", args!["kkk... Yes, wise men take their chances when the opportunity comes. I'll tell you the ingredients. Don't forget, and bring them all."])?;
                        ctx.next()?;
                        ctx.lines_as("Rust Blackhand", args!["^0000FFCursed Baphomet Doll, Magestic Goat, 30 Crystal of Darkness, 50 Fragment of Darkness^000000, and the most important, production cost is ^0000FF990000^000000 Zeny."])?;
                        ctx.next()?;
                        ctx.lines_as("Rust Blackhand", args!["You can get the Magestic Goat from the weak Baphomet in the Labyrinth Forest. Crystal of Darkness and Fragment of Darkness are from the Incarnation of Morocc."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Rust Blackhand",
                            args!["I'm sure that you can get the ingredients because you sealed the real Baphomet. Can't you? kkk..."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Rust Blackhand",
                            args![
                                "You'll never know how great this hat is until you get one. If you understood, go and get the ingredients."
                            ],
                        )?;
                        if ctx.call(Function::IsBeginQuest, vec![Val::from(3042)])?.is_true() {
                            ctx.call(Function::ChangeQuest, vec![Val::from(3042), Val::from(3043)])?;
                        } else {
                            ctx.call(Function::EraseQuest, vec![Val::from(3043)])?;
                            ctx.call(Function::SetQuest, vec![Val::from(3043)])?;
                        }
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as(
                            "Rust Blackhand",
                            args!["Huh... Do you? Do whatever you want... Do you really want to let this opportunity go to waste?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Rust Blackhand",
                            args!["Tut, tut... I don't care if the wicked doll threatens your life all the time!"],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            }
            if !matched3 && subject3.loosely_equals(&Val::from(2)) {
                matched3 = true;
            }
            if matched3 {
                ctx.lines_as("Rust Blackhand", args!["What a dull boy he is... huh..."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    } else if (l_new_maje.clone() == 2 && ctx.call(Function::CountItem, vec![Val::from(6004)])? == 0) {
        ctx.lines_as(
            "Rust Blackhand",
            args!["Why are you hanging around here? If you don't want a ^4d4dffGigantic Magestic Goat^000000, go away."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn rust_blackhand_edq(ctx: &Ctx) -> Script {
    rust_blackhand_edq_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum InsBaphometLottoStep {
    Start,
    OnInstanceInit,
}

fn ins_baphomet_lotto_run(ctx: &Ctx, mut step: InsBaphometLottoStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    let mut l_true = Val::from(0);
    'machine: loop {
        match step {
            InsBaphometLottoStep::Start => {
                step = InsBaphometLottoStep::OnInstanceInit;
                continue 'machine;
            }
            InsBaphometLottoStep::OnInstanceInit => {
                l_true = ctx.call(Function::Rand, vec![Val::from(1), Val::from(13)])?;
                l_i = Val::from(1);
                'l1: loop {
                    if !(l_i.clone().number()? <= 13) {
                        break 'l1;
                    }
                    'b1: {
                        ctx.call(
                            Function::DisableNpc,
                            vec![ctx.call(
                                Function::InstanceNpcName,
                                vec![
                                    ((Val::from("Gravestone#1F_") + l_i.clone())
                                        + (if l_i.clone().loosely_equals(&l_true.clone()) {
                                            Val::from("T")
                                        } else {
                                            Val::from("F")
                                        })),
                                ],
                            )?],
                        )?;
                    }
                    l_i = (l_i.clone() + Val::from(1));
                }
                ctx.call(
                    Function::DisableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("ins_baphomet_lotto")])?],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn ins_baphomet_lotto(ctx: &Ctx) -> Script {
    ins_baphomet_lotto_run(ctx, InsBaphometLottoStep::Start, Vec::new()).map(|_| ())
}

pub fn ins_baphomet_lotto_oninstanceinit(ctx: &Ctx) -> Script {
    ins_baphomet_lotto_run(ctx, InsBaphometLottoStep::OnInstanceInit, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum InsBaphometLotto2Step {
    Start,
    OnEnable,
}

fn ins_baphomet_lotto2_run(ctx: &Ctx, mut step: InsBaphometLotto2Step, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    'machine: loop {
        match step {
            InsBaphometLotto2Step::Start => {
                step = InsBaphometLotto2Step::OnEnable;
                continue 'machine;
            }
            InsBaphometLotto2Step::OnEnable => {
                l_i = Val::from(1);
                'l1: loop {
                    if !(l_i.clone().number()? <= 12) {
                        break 'l1;
                    }
                    'b1: {
                        ctx.call(
                            Function::EnableNpc,
                            vec![ctx.call(Function::InstanceNpcName, vec![(Val::from("Bobbing Torch#") + l_i.clone())])?],
                        )?;
                    }
                    l_i = (l_i.clone() + Val::from(1));
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn ins_baphomet_lotto2(ctx: &Ctx) -> Script {
    ins_baphomet_lotto2_run(ctx, InsBaphometLotto2Step::Start, Vec::new()).map(|_| ())
}

pub fn ins_baphomet_lotto2_onenable(ctx: &Ctx) -> Script {
    ins_baphomet_lotto2_run(ctx, InsBaphometLotto2Step::OnEnable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum InsBaphometLotto3Step {
    Start,
    OnEnable,
    OnDisable,
    OnMyMobDead,
}

fn ins_baphomet_lotto3_run(ctx: &Ctx, mut step: InsBaphometLotto3Step, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_label_s = Val::from("");
    let mut l_map_s = Val::from("");
    'machine: loop {
        match step {
            InsBaphometLotto3Step::Start => {
                step = InsBaphometLotto3Step::OnEnable;
                continue 'machine;
            }
            InsBaphometLotto3Step::OnEnable => {
                l_label_s = (ctx.call(Function::InstanceNpcName, vec![Val::from("ins_baphomet_lotto3")])? + Val::from("::OnMyMobDead"));
                l_map_s = ctx.call(Function::InstanceMapName, vec![Val::from("1@cata")])?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1869),
                        Val::from(1),
                        l_label_s.clone(),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1291),
                        Val::from(1),
                        l_label_s.clone(),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1869),
                        Val::from(1),
                        l_label_s.clone(),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1291),
                        Val::from(1),
                        l_label_s.clone(),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1869),
                        Val::from(1),
                        l_label_s.clone(),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1291),
                        Val::from(1),
                        l_label_s.clone(),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1869),
                        Val::from(1),
                        l_label_s.clone(),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1132),
                        Val::from(1),
                        l_label_s.clone(),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1117),
                        Val::from(1),
                        l_label_s.clone(),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1132),
                        Val::from(1),
                        l_label_s.clone(),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1117),
                        Val::from(1),
                        l_label_s.clone(),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1132),
                        Val::from(1),
                        l_label_s.clone(),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1291),
                        Val::from(1),
                        l_label_s.clone(),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1117),
                        Val::from(1),
                        l_label_s.clone(),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1869),
                        Val::from(1),
                        l_label_s.clone(),
                    ],
                )?;
                return Err(Stop::End);
            }
            InsBaphometLotto3Step::OnDisable => {
                ctx.call(
                    Function::KillMonster,
                    vec![
                        ctx.call(Function::InstanceMapName, vec![Val::from("1@cata")])?,
                        (ctx.call(Function::InstanceNpcName, vec![Val::from("ins_baphomet_lotto3")])? + Val::from("::OnMyMobDead")),
                    ],
                )?;
                return Err(Stop::End);
            }
            InsBaphometLotto3Step::OnMyMobDead => {
                l_map_s = ctx.call(Function::InstanceMapName, vec![Val::from("1@cata")])?;
                if ctx
                    .call(
                        Function::MobCount,
                        vec![
                            l_map_s.clone(),
                            (ctx.call(Function::InstanceNpcName, vec![Val::from("ins_baphomet_lotto3")])? + Val::from("::OnMyMobDead")),
                        ],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            l_map_s.clone(),
                            Val::from("All apostles of Baphomet are dead!"),
                            ctx.constant("BC_MAP")?,
                            Val::from("0x00ff99"),
                        ],
                    )?;
                }
                ctx.call(Function::GetItem, vec![Val::from(6002), Val::from(1)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn ins_baphomet_lotto3(ctx: &Ctx) -> Script {
    ins_baphomet_lotto3_run(ctx, InsBaphometLotto3Step::Start, Vec::new()).map(|_| ())
}

pub fn ins_baphomet_lotto3_onenable(ctx: &Ctx) -> Script {
    ins_baphomet_lotto3_run(ctx, InsBaphometLotto3Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn ins_baphomet_lotto3_ondisable(ctx: &Ctx) -> Script {
    ins_baphomet_lotto3_run(ctx, InsBaphometLotto3Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn ins_baphomet_lotto3_onmymobdead(ctx: &Ctx) -> Script {
    ins_baphomet_lotto3_run(ctx, InsBaphometLotto3Step::OnMyMobDead, Vec::new()).map(|_| ())
}

fn gravestone_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_party_id = Val::from(0);
    l_party_id = ctx.call(Function::GetCharacterId, vec![Val::from(1)])?;
    if ctx.var("'ins_baphomet").get()? == 0 {
        ctx.mes("The gravestone is trembling...")?;
        ctx.next()?;
        ctx.mes("When touching the gravestone, I hear a voice.")?;
        ctx.next()?;
        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_CONE")?])?;
        ctx.lines_as(
            "Voice of the Gravestone",
            args!["I have waited and waited for a brave adventurer like you who will come back here again..."],
        )?;
        ctx.next()?;
        'l1: loop {
            if !(true) {
                break 'l1;
            }
            'b1: {
                match runtime::select_values(ctx, &[Val::from("Who are you?:Waited for me?:Cancel.")])? {
                    1 => {
                        ctx.lines_as(
                            "Voice of the Gravestone",
                            args!["I was one of the warriors to stop Baphomet like you. Now, I'm dead and only my soul remains..."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Voice of the Gravestone", args!["As you know... We don't have much time. You can't stop Baphomet by yourselves. The power of the devil has strengthened over the years."])?;
                        ctx.next()?;
                        ctx.lines_as("Voice of the Gravestone", args!["In the past, my companions and I sealed Baphomet at the altar located on the 2nd basement and blocked the entrance."])?;
                        ctx.next()?;
                        ctx.lines_as("Voice of the Gravestone", args!["I moved my soul's essence to my pendant, so that I could remain in this world. That's when I became this grave's guardian."])?;
                        ctx.next()?;
                    }
                    2 => {
                        ctx.lines_as("Voice of the Gravestone", args!["Baphomet's power is about to break the seal that we made. If you don't reactivate them... Baphomet's revival will only be a matter of time."])?;
                        ctx.next()?;
                        ctx.lines_as("Voice of the Gravestone", args!["To open the entrance, you must substantialize my soul. I'll open the entrance and reactivate the weakened seals after I am substantilized."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Voice of the Gravestone",
                            args!["To substantialize my soul, you should find my pendant. You can find my body near a grave here."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Voice of the Gravestone",
                            args![
                                "If your ^0000FFparty leader^000000 brings me the pendant, my soul can be substantialized. So, hurry up."
                            ],
                        )?;
                        ctx.var("'ins_baphomet").set(Val::from(1))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    3 => {
                        ctx.mes("I can feel the voice becoming faint.")?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            }
        }
    } else if ctx.var("'ins_baphomet").get()? == 1 {
        ctx.lines_as("Voice of the Gravestone", args!["To open the entrance, you must substantialize my soul. I'll open the entrance and reactivate the weakened seals after I am substantilized."])?;
        ctx.next()?;
        ctx.lines_as(
            "Voice of the Gravestone",
            args!["To substantialize my soul, you should find my pendant. You can find my body near a grave here."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Voice of the Gravestone",
            args!["If your ^0000FFparty leader^000000 brings me the pendant, my soul can be substantialized. So, hurry up."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("'ins_baphomet").get()? == 2
        && ctx
            .call(Function::IsPartyLeader, vec![l_party_id.clone()])?
            .loosely_equals(&Val::from(1)))
    {
        ctx.lines_as("Voice of the Gravestone", args!["Did you find the pendant?"])?;
        ctx.next()?;
        if ctx.call(Function::CountItem, vec![Val::from(6003)])?.number()? > 0 {
            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_MAPPILLAR")?])?;
            ctx.lines_as("Voice of the Gravestone", args!["Yes... This is... My pendant..."])?;
            ctx.next()?;
            ctx.call(Function::DelItem, vec![Val::from(6003), Val::from(1)])?;
            ctx.call(
                Function::EnableNpc,
                vec![ctx.call(Function::InstanceNpcName, vec![Val::from("Ancient Hero's Soul#1F")])?],
            )?;
            ctx.call(
                Function::DisableNpc,
                vec![ctx.call(Function::InstanceNpcName, vec![Val::from("Gravestone#")])?],
            )?;
            ctx.lines_as("Voice of the Gravestone", args!["Now I can substantialize my soul. I'll wait for you in front of the altar of fire located at the center of this grave. Let's meet there."])?;
            ctx.next()?;
            ctx.call(
                Function::MapAnnounce,
                vec![
                    ctx.call(Function::InstanceMapName, vec![Val::from("1@cata")])?,
                    Val::from("Ancient Hero's Soul : I'll wait for you in front of the altar of fire located at the center"),
                    ctx.constant("BC_MAP")?,
                    Val::from("0xFFFF00"),
                ],
            )?;
            ctx.mes("I can feel the voice becoming faint.")?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Voice of the Gravestone",
                args!["Are you still there? Bring back my pendant as soon as possible."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Voice of the Gravestone",
                args!["You can find my body near a grave here. Go and get my pendant there."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        ctx.lines_as(
            "Voice of the Gravestone",
            args!["I want to talk to ^0000FFa representative among your party^000000. Everyone else, wait here."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    return Err(Stop::End);
}

pub fn gravestone(ctx: &Ctx) -> Script {
    gravestone_body(ctx, Vec::new()).map(|_| ())
}

fn gravestone_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("'ins_baphomet").get()? == 0 {
        ctx.mes("'Krrrr... Krrrr...'")?;
        ctx.next()?;
        ctx.mes("I can feel something odd at the grave. It's like someone is calling out silently...")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    return Err(Stop::End);
}

pub fn gravestone_ontouch(ctx: &Ctx) -> Script {
    gravestone_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn gravestone_oninstanceinit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.var("'ins_baphomet").set(Val::from(0))?;
    return Err(Stop::End);
}

pub fn gravestone_oninstanceinit(ctx: &Ctx) -> Script {
    gravestone_oninstanceinit_body(ctx, Vec::new()).map(|_| ())
}

fn ancient_hero_s_soul_1f_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_exitloop = Val::from(0);
    let mut l_ins_baphomet_1f_1 = Val::from(0);
    let mut l_ins_baphomet_1f_2 = Val::from(0);
    let mut l_ins_baphomet_1f_3 = Val::from(0);
    let mut l_party_id = Val::from(0);
    l_party_id = ctx.call(Function::GetCharacterId, vec![Val::from(1)])?;
    ctx.call(Function::Cutin, vec![Val::from("ins_cata_champ_n"), Val::from(2)])?;
    if ctx.var("'ins_baphomet").get()? == 2 {
        ctx.lines_as(
            "Ancient Hero's Soul",
            args!["With your help, my soul can be substantialized. I want to talk more, but we do not have enough time..."],
        )?;
        ctx.next()?;
        ctx.lines_as("Ancient Hero's Soul", args!["I must be substantialized within the next hour. To reach the Main Altar underground, you must help me perform the ceremony for opening each seal."])?;
        ctx.next()?;
        ctx.lines_as(
            "Ancient Hero's Soul",
            args!["Now I'll tell you what should you do. First, collect ^0000FFEssence of Fire^000000 from the torches on the graves..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ancient Hero's Soul",
            args!["Second, all members who will enter the underground must carry a symbol, called the Token of Apostle."],
        )?;
        ctx.next()?;
        'l1: loop {
            if !(true) {
                break 'l1;
            }
            'b1: {
                ctx.call(Function::Cutin, vec![Val::from("ins_cata_champ_n"), Val::from(2)])?;
                match runtime::select_values(
                    ctx,
                    &[Val::from(
                        "What is Essence of Fire?:What is a Token of Apostle?:What should I do?:I am ready.",
                    )],
                )? {
                    1 => {
                        l_ins_baphomet_1f_1 = (l_ins_baphomet_1f_1.clone() + Val::from(1));
                        ctx.lines_as("Ancient Hero's Soul", args!["You can see the torches here and there. These torches are the essence of Thor, the god of Thunder. They are inherited from our predecessors to stop the darkness of Baphomet..."])?;
                        ctx.next()?;
                        ctx.lines_as("Ancient Hero's Soul", args!["To open the sealed underground gate, I must be purified by the ^0000FFEssence of Fire^000000 which has the power of Thor. Collect ^0000FF10 Essence of Fire^000000 from the torches."])?;
                        ctx.next()?;
                        ctx.lines_as("Ancient Hero's Soul", args!["One thing you must remember is... ^0000FFEssence of Fire^000000 can only be collected by the Inheritor of Faith."])?;
                        ctx.next()?;
                        ctx.lines_as("Ancient Hero's Soul", args!["I'll give the token of the Inheritor of Faith to the party leader. Only the party leader can collect the ^0000FFEssence of Fire^000000."])?;
                        ctx.next()?;
                    }
                    2 => {
                        l_ins_baphomet_1f_2 = (l_ins_baphomet_1f_2.clone() + Val::from(1));
                        ctx.lines_as("Ancient Hero's Soul", args!["At that time, it was impossible to get rid of Devil Baphomet by ourselves. After numerous heroes sacrificed their lives, we could barely seal him under this Abbey."])?;
                        ctx.next()?;
                        ctx.lines_as("Ancient Hero's Soul", args!["However, Baphomet never gave up. He continuously strengthened his power. And his power has brought new life to this shrine."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ancient Hero's Soul",
                            args!["Some of these monsters have magical powers. Baphomet calls them his 'Apostles'."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Ancient Hero's Soul", args!["The apostles are cloning themselves to fill the shrine with their evil energy. They are the monsters here in this catacomb."])?;
                        ctx.next()?;
                        ctx.lines_as("Ancient Hero's Soul", args!["Kill the Apostles. They can only be distinguished from their clones because they will possess a ^0000FFToken of Apostle^000000..."])?;
                        ctx.next()?;
                        ctx.lines_as("Ancient Hero's Soul", args!["If each party member possesses a ^0000FFToken of Apostle^000000, Baphomet will not be able to perceive your entrance to the Main Altar underground."])?;
                        ctx.next()?;
                        ctx.lines_as("Ancient Hero's Soul", args!["If Baphomet perceives you when you pass through the sealed gate, he may release his tremendous magical power. Then, this abbey will be demolished."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ancient Hero's Soul",
                            args!["Kill the apostles and get the ^0000FFToken of Apostle^000000."],
                        )?;
                        ctx.next()?;
                    }
                    3 => {
                        l_ins_baphomet_1f_3 = (l_ins_baphomet_1f_3.clone() + Val::from(1));
                        if ctx
                            .call(Function::IsPartyLeader, vec![l_party_id.clone()])?
                            .loosely_equals(&Val::from(1))
                        {
                            ctx.lines_as("Ancient Hero's Soul", args!["You look like the leader of this party. You need to go and get ^0000FF10 Essence of Fire^000000 from the torches."])?;
                            ctx.next()?;
                            ctx.lines_as("Ancient Hero's Soul", args!["Now I'll carve you the symbol which shows you're a inheritor of faith. Be aware that no one in your party but you can collect the Essence of Fire."])?;
                            ctx.next()?;
                            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_HOLYHIT")?])?;
                            ctx.lines_as(
                                "Ancient Hero's Soul",
                                args!["You should also carry a ^0000FFToken of Apostle^000000, so find one for yourself too."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Ancient Hero's Soul",
                                args!["When all of you are ready to go, let me know. I'll open the sealed gate when you're ready."],
                            )?;
                            ctx.next()?;
                        } else {
                            ctx.lines_as("Ancient Hero's Soul", args!["Make sure that your party leader has listened to my explanation of what you and your companions must do."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Ancient Hero's Soul",
                                args!["Kill the Apostles of Baphomet and find their ^0000FFToken of Apostle^000000."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Ancient Hero's Soul", args!["It might be better to kill all of them because there is no way to differentiate them from their clones."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Ancient Hero's Soul",
                                args![
                                    "Are you ready?",
                                    "Make sure that you have listened to all that I have to say before saying that you are ready."
                                ],
                            )?;
                            ctx.next()?;
                        }
                    }
                    4 => {
                        if ((l_ins_baphomet_1f_1.clone().number()? > 0 && l_ins_baphomet_1f_2.clone().number()? > 0)
                            && l_ins_baphomet_1f_3.clone().number()? > 0)
                        {
                            ctx.call(Function::Cutin, vec![Val::from("ins_cata_champ_n"), Val::from(2)])?;
                            ctx.lines_as(
                                "Ancient Hero's Soul",
                                args!["Are you ready to go? Then I'll open this sealed gate now."],
                            )?;
                            ctx.next()?;
                            l_exitloop = Val::from(1);
                        } else {
                            ctx.call(Function::Cutin, vec![Val::from("ins_cata_champ_a"), Val::from(2)])?;
                            ctx.lines_as(
                                "Ancient Hero's Soul",
                                args!["It may be difficult but I wish you luck braving the perils of this catacomb."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Ancient Hero's Soul", args!["Remember, you need to collect ^0000FF10 Essence of Fire^000000 from the torches and the ^0000FFToken of Apostle^000000 from the Apostles of Baphomet."])?;
                            ctx.next()?;
                        }
                    }
                    _ => {}
                }
                if l_exitloop.clone().is_true() {
                    break 'l1;
                }
            }
        }
        if ctx
            .call(Function::IsPartyLeader, vec![l_party_id.clone()])?
            .loosely_equals(&Val::from(1))
        {
            ctx.lines_as(
                "Ancient Hero's Soul",
                args![
                    "To remind you again, I must be substantialized within the next hour. So everyone, finish your work within that time!"
                ],
            )?;
            ctx.var("'ins_baphomet").set(Val::from(3))?;
            ctx.call(
                Function::DoNpcEvent,
                vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("ins_baphomet_1f_timer")])? + Val::from("::OnEnable"))],
            )?;
            ctx.call(
                Function::DoNpcEvent,
                vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("ins_baphomet_lotto2")])? + Val::from("::OnEnable"))],
            )?;
            ctx.call(
                Function::DoNpcEvent,
                vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("ins_baphomet_lotto3")])? + Val::from("::OnEnable"))],
            )?;
        } else {
            ctx.lines_as(
                "Ancient Hero's Soul",
                args!["To remind you again, I can be substantialized for the next hour. So everyone, finish your work within that time."],
            )?;
        }
        ctx.close_window()?;
        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
        return Err(Stop::End);
    } else {
        if (ctx.var("'ins_baphomet").get()? == 3
            && ctx
                .call(Function::IsPartyLeader, vec![l_party_id.clone()])?
                .loosely_equals(&Val::from(1)))
        {
            ctx.call(Function::Cutin, vec![Val::from("ins_cata_champ_n"), Val::from(2)])?;
            ctx.lines_as(
                "Ancient Hero's Soul",
                args!["Did you get 10 ^0000FFEssence of Fire^000000 and ^0000FFToken of Apostle^000000?"],
            )?;
            ctx.next()?;
            if (ctx.call(Function::CountItem, vec![Val::from(6001)])?.number()? > 9
                && ctx.call(Function::CountItem, vec![Val::from(6002)])?.number()? > 0)
            {
                ctx.call(
                    Function::DelItem,
                    vec![Val::from(6001), ctx.call(Function::CountItem, vec![Val::from(6001)])?],
                )?;
                ctx.var("'ins_baphomet").set(Val::from(4))?;
                ctx.lines_as(
                    "Ancient Hero's Soul",
                    args!["Okay. You've done your work. Now check your companions and tell me when everyone has finished their work."],
                )?;
            } else {
                ctx.lines_as(
                    "Ancient Hero's Soul",
                    args!["Not ready yet? You should prepare 10 ^0000FFEssence of Fire^000000 and ^0000FFToken of Apostle^000000."],
                )?;
            }
            ctx.close_window()?;
            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
            return Err(Stop::End);
        } else if ctx.var("'ins_baphomet").get()? == 3 {
            ctx.call(Function::Cutin, vec![Val::from("ins_cata_champ_n"), Val::from(2)])?;
            ctx.lines_as("Ancient Hero's Soul", args!["Do you have a ^0000FFToken of Apostle^000000?"])?;
            ctx.next()?;
            if ctx.call(Function::CountItem, vec![Val::from(6002)])?.number()? > 0 {
                ctx.var("'ins_baphomet").set(Val::from(4))?;
                ctx.lines_as("Ancient Hero's Soul", args!["Okay. You've done your work. Tell your representative to check your companions and come to me when everyone has finished their work."])?;
            } else {
                ctx.lines_as(
                    "Ancient Hero's Soul",
                    args!["Not ready yet? You should prepare ^0000FFToken of Apostle^000000."],
                )?;
            }
            ctx.close_window()?;
            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
            return Err(Stop::End);
        } else if (ctx.var("'ins_baphomet").get()? == 4
            && ctx
                .call(Function::IsPartyLeader, vec![l_party_id.clone()])?
                .loosely_equals(&Val::from(1)))
        {
            ctx.call(Function::Cutin, vec![Val::from("ins_cata_champ_n"), Val::from(2)])?;
            ctx.lines_as(
                "Ancient Hero's Soul",
                args!["Are you ready? I opened the sealed gate. To pass the gate, you should carry a ^0000FFToken of Apostle^000000."],
            )?;
            ctx.next()?;
            ctx.var("'ins_baphomet").set(Val::from(5))?;
            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_TELEPORTATION")?])?;
            ctx.call(
                Function::EnableNpc,
                vec![ctx.call(Function::InstanceNpcName, vec![Val::from("ins_bapho_to_2f")])?],
            )?;
            ctx.call(
                Function::DoNpcEvent,
                vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("ins_baphomet_1f_timer")])? + Val::from("::OnDisable"))],
            )?;
            ctx.lines_as(
                "Ancient Hero's Soul",
                args!["Now you can go to the main altar. It is located in the bottom right corner of this floor."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Ancient Hero's Soul",
                args!["Your real battle will begin... I'll follow you soon and find a way to help you."],
            )?;
            ctx.next()?;
            ctx.lines_as("Ancient Hero's Soul", args!["Go ahead, warriors."])?;
            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
            ctx.call(
                Function::MapAnnounce,
                vec![
                    ctx.call(Function::InstanceMapName, vec![Val::from("1@cata")])?,
                    Val::from("Ancient Hero's Soul : Now you can go to the Main Altar's gate. It is located in the Southeast"),
                    ctx.constant("BC_MAP")?,
                    Val::from("0xFFFF00"),
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("'ins_baphomet").get()? == 4 {
            ctx.call(Function::Cutin, vec![Val::from("ins_cata_champ_n"), Val::from(2)])?;
            ctx.lines_as(
                "Ancient Hero's Soul",
                args!["Are you ready? I opened the sealed gate. To pass the gate, you should carry a ^0000FFToken of Apostle^000000."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Ancient Hero's Soul",
                args!["I'll complete opening the sealed gate when your representative tells me that you're ready."],
            )?;
            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("'ins_baphomet").get()? == 5 {
            ctx.call(Function::Cutin, vec![Val::from("ins_cata_champ_n"), Val::from(2)])?;
            ctx.lines_as("Ancient Hero's Soul", args!["What are you doing? The entrance of the main altar is opened now, go and fight! The entrance is near the bottom right side of this floor."])?;
            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.call(Function::Cutin, vec![Val::from("ins_cata_champ_n"), Val::from(2)])?;
            ctx.lines_as("Ancient Hero's Soul", args!["I have nothing to say to you..."])?;
            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
}

pub fn ancient_hero_s_soul_1f(ctx: &Ctx) -> Script {
    ancient_hero_s_soul_1f_body(ctx, Vec::new()).map(|_| ())
}

fn ancient_hero_s_soul_1f_oninstanceinit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::DisableNpc,
        vec![ctx.call(Function::InstanceNpcName, vec![Val::from("Ancient Hero's Soul#1F")])?],
    )?;
    return Err(Stop::End);
}

pub fn ancient_hero_s_soul_1f_oninstanceinit(ctx: &Ctx) -> Script {
    ancient_hero_s_soul_1f_oninstanceinit_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum InsBaphoTo2fStep {
    Start,
    OnTouch,
    OnInstanceInit,
}

fn ins_bapho_to_2f_run(ctx: &Ctx, mut step: InsBaphoTo2fStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            InsBaphoTo2fStep::Start => {
                step = InsBaphoTo2fStep::OnTouch;
                continue 'machine;
            }
            InsBaphoTo2fStep::OnTouch => {
                if ctx.call(Function::CountItem, vec![Val::from(6002)])?.number()? > 0 {
                    ctx.call(
                        Function::DelItem,
                        vec![Val::from(6002), ctx.call(Function::CountItem, vec![Val::from(6002)])?],
                    )?;
                    ctx.var("'ins_baphomet").set(Val::from(5))?;
                    ctx.call(
                        Function::Warp,
                        vec![
                            ctx.call(Function::InstanceMapName, vec![Val::from("2@cata")])?,
                            Val::from(80),
                            Val::from(144),
                        ],
                    )?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Ancient Hero's Soul",
                        args!["Where is your Token of Apostle? I said you should carry the Token of Apostle to pass this gate."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            InsBaphoTo2fStep::OnInstanceInit => {
                ctx.call(
                    Function::DisableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("ins_bapho_to_2f")])?],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn ins_bapho_to_2f(ctx: &Ctx) -> Script {
    ins_bapho_to_2f_run(ctx, InsBaphoTo2fStep::Start, Vec::new()).map(|_| ())
}

pub fn ins_bapho_to_2f_ontouch(ctx: &Ctx) -> Script {
    ins_bapho_to_2f_run(ctx, InsBaphoTo2fStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn ins_bapho_to_2f_oninstanceinit(ctx: &Ctx) -> Script {
    ins_bapho_to_2f_run(ctx, InsBaphoTo2fStep::OnInstanceInit, Vec::new()).map(|_| ())
}

fn gravestone_ss1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("'ins_baphomet").get()? == 1 {
        ctx.call(Function::GetItem, vec![Val::from(6003), Val::from(1)])?;
        ctx.var("'ins_baphomet").set(Val::from(2))?;
        ctx.mes("A small object is shining under a leaning grave.")?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["I think this is the pendant..."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.mes("I can only feel gloom from this Gravestone.")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn gravestone_ss1(ctx: &Ctx) -> Script {
    gravestone_ss1_body(ctx, Vec::new()).map(|_| ())
}

fn gravestone_ss2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.mes("I can only feel gloom from this Gravestone.")?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn gravestone_ss2(ctx: &Ctx) -> Script {
    gravestone_ss2_body(ctx, Vec::new()).map(|_| ())
}

fn bobbing_torch_ss_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx
        .call(
            Function::IsPartyLeader,
            vec![ctx.call(Function::GetCharacterId, vec![Val::from(1)])?],
        )?
        .loosely_equals(&Val::from(1))
    {
        if (ctx.var("'ins_baphomet").get()? == 3 && ctx.call(Function::CountItem, vec![Val::from(6001)])?.number()? < 11) {
            ctx.mes("A huge torch appearing as if it can burn everything is bobbing up and down in front of me.")?;
            ctx.next()?;
            ctx.mes("The grand appearance and heat of the fire makes me step back... But I pluck up my courage and reach out to pick up the torch.")?;
            ctx.next()?;
            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_HOLYHIT")?])?;
            ctx.call(Function::GetItem, vec![Val::from(6001), Val::from(1)])?;
            ctx.mes("The symbol of inheritor shines. Then a small crystal falls into my hand from the torch.")?;
            ctx.call(Function::DisableNpc, vec![])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if (ctx.var("'ins_baphomet").get()? == 3 && ctx.call(Function::CountItem, vec![Val::from(6001)])?.number()? > 10) {
            ctx.mes("You have 10 Essence of Fire already, so you don't need to collect any more.")?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.mes("You don't need to collect Essence of Fire anymore.")?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        ctx.mes("A huge torch appearing as if it can burn everything is bobbing up and down in front of me..")?;
        ctx.next()?;
        ctx.lines_as(
            "Unknown Voice",
            args!["You are not a inheritor of faith. Do not desecrate the Essence of Fire with your disrespectful hands."],
        )?;
        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_FIRESPLASHHIT")?])?;
        ctx.call(Function::PercentHeal, vec![Val::from(-50), Val::from(0)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn bobbing_torch_ss(ctx: &Ctx) -> Script {
    bobbing_torch_ss_body(ctx, Vec::new()).map(|_| ())
}

fn bobbing_torch_ss_oninstanceinit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![])?;
    return Err(Stop::End);
}

pub fn bobbing_torch_ss_oninstanceinit(ctx: &Ctx) -> Script {
    bobbing_torch_ss_oninstanceinit_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum InsBaphomet1fTimerStep {
    Start,
    OnInstanceInit,
    OnEnable,
    OnDisable,
    OnTimer1800000,
    OnTimer2400000,
    OnTimer3000000,
    OnTimer3050000,
    OnTimer3100000,
    OnTimer3500000,
}

fn ins_baphomet_1f_timer_run(ctx: &Ctx, mut step: InsBaphomet1fTimerStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            InsBaphomet1fTimerStep::Start => {
                step = InsBaphomet1fTimerStep::OnInstanceInit;
                continue 'machine;
            }
            InsBaphomet1fTimerStep::OnInstanceInit => {
                ctx.call(
                    Function::DisableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("ins_baphomet_1f_timer")])?],
                )?;
                return Err(Stop::End);
            }
            InsBaphomet1fTimerStep::OnEnable => {
                ctx.call(
                    Function::EnableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("ins_baphomet_1f_timer")])?],
                )?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            InsBaphomet1fTimerStep::OnDisable => {
                ctx.call(
                    Function::DisableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("ins_baphomet_1f_timer")])?],
                )?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            InsBaphomet1fTimerStep::OnTimer1800000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::InstanceMapName, vec![Val::from("1@cata")])?,
                        Val::from("Ancient Hero's Soul : We don't have enough time! Hurry up!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            InsBaphomet1fTimerStep::OnTimer2400000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::InstanceMapName, vec![Val::from("1@cata")])?,
                        Val::from("Ancient Hero's Soul : My body is disappearing... Hurry up!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            InsBaphomet1fTimerStep::OnTimer3000000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::InstanceMapName, vec![Val::from("1@cata")])?,
                        Val::from("Ancient Hero's Soul : Everything is over... There is no other way but to wait for the next chance..."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            InsBaphomet1fTimerStep::OnTimer3050000 => {
                ctx.call(Function::MapAnnounce, vec![ctx.call(Function::InstanceMapName, vec![Val::from("1@cata")])?, Val::from("Ancient Hero's Soul : We failed... However... We still have a chance. I hope you will train yourselves until the time comes."), ctx.constant("BC_MAP")?, Val::from("0xFFFF00")])?;
                return Err(Stop::End);
            }
            InsBaphomet1fTimerStep::OnTimer3100000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::InstanceMapName, vec![Val::from("1@cata")])?,
                        Val::from("You've failed to open the seal of main altar."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            InsBaphomet1fTimerStep::OnTimer3500000 => {
                ctx.call(
                    Function::InstanceWarpAll,
                    vec![
                        Val::from("monk_test"),
                        Val::from(310),
                        Val::from(150),
                        ctx.call(Function::InstanceId, vec![])?,
                    ],
                )?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn ins_baphomet_1f_timer(ctx: &Ctx) -> Script {
    ins_baphomet_1f_timer_run(ctx, InsBaphomet1fTimerStep::Start, Vec::new()).map(|_| ())
}

pub fn ins_baphomet_1f_timer_oninstanceinit(ctx: &Ctx) -> Script {
    ins_baphomet_1f_timer_run(ctx, InsBaphomet1fTimerStep::OnInstanceInit, Vec::new()).map(|_| ())
}

pub fn ins_baphomet_1f_timer_onenable(ctx: &Ctx) -> Script {
    ins_baphomet_1f_timer_run(ctx, InsBaphomet1fTimerStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn ins_baphomet_1f_timer_ondisable(ctx: &Ctx) -> Script {
    ins_baphomet_1f_timer_run(ctx, InsBaphomet1fTimerStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn ins_baphomet_1f_timer_ontimer1800000(ctx: &Ctx) -> Script {
    ins_baphomet_1f_timer_run(ctx, InsBaphomet1fTimerStep::OnTimer1800000, Vec::new()).map(|_| ())
}

pub fn ins_baphomet_1f_timer_ontimer2400000(ctx: &Ctx) -> Script {
    ins_baphomet_1f_timer_run(ctx, InsBaphomet1fTimerStep::OnTimer2400000, Vec::new()).map(|_| ())
}

pub fn ins_baphomet_1f_timer_ontimer3000000(ctx: &Ctx) -> Script {
    ins_baphomet_1f_timer_run(ctx, InsBaphomet1fTimerStep::OnTimer3000000, Vec::new()).map(|_| ())
}

pub fn ins_baphomet_1f_timer_ontimer3050000(ctx: &Ctx) -> Script {
    ins_baphomet_1f_timer_run(ctx, InsBaphomet1fTimerStep::OnTimer3050000, Vec::new()).map(|_| ())
}

pub fn ins_baphomet_1f_timer_ontimer3100000(ctx: &Ctx) -> Script {
    ins_baphomet_1f_timer_run(ctx, InsBaphomet1fTimerStep::OnTimer3100000, Vec::new()).map(|_| ())
}

pub fn ins_baphomet_1f_timer_ontimer3500000(ctx: &Ctx) -> Script {
    ins_baphomet_1f_timer_run(ctx, InsBaphomet1fTimerStep::OnTimer3500000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Ins2fEnterStep {
    Start,
    OnTouch,
}

fn ins_2f_enter_run(ctx: &Ctx, mut step: Ins2fEnterStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Ins2fEnterStep::Start => {
                step = Ins2fEnterStep::OnTouch;
                continue 'machine;
            }
            Ins2fEnterStep::OnTouch => {
                ctx.call(
                    Function::DoNpcEvent,
                    vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("ins_2f_enter_broad")])? + Val::from("::OnEnable"))],
                )?;
                ctx.call(
                    Function::DisableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("ins_2f_enter")])?],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn ins_2f_enter(ctx: &Ctx) -> Script {
    ins_2f_enter_run(ctx, Ins2fEnterStep::Start, Vec::new()).map(|_| ())
}

pub fn ins_2f_enter_ontouch(ctx: &Ctx) -> Script {
    ins_2f_enter_run(ctx, Ins2fEnterStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Ins2fEnterBroadStep {
    Start,
    OnInstanceInit,
    OnDisable,
    OnEnable,
    OnTimer10000,
    OnTimer13000,
    OnTimer16000,
    OnTimer18000,
}

fn ins_2f_enter_broad_run(ctx: &Ctx, mut step: Ins2fEnterBroadStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Ins2fEnterBroadStep::Start => {
                step = Ins2fEnterBroadStep::OnInstanceInit;
                continue 'machine;
            }
            Ins2fEnterBroadStep::OnInstanceInit => {
                step = Ins2fEnterBroadStep::OnDisable;
                continue 'machine;
            }
            Ins2fEnterBroadStep::OnDisable => {
                ctx.call(
                    Function::DisableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("ins_2f_enter_broad")])?],
                )?;
                return Err(Stop::End);
            }
            Ins2fEnterBroadStep::OnEnable => {
                ctx.call(
                    Function::EnableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("ins_2f_enter_broad")])?],
                )?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Ins2fEnterBroadStep::OnTimer10000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::InstanceMapName, vec![Val::from("2@cata")])?,
                        Val::from("Baphomet : Humans... interfering again..."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xdb7093"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Ins2fEnterBroadStep::OnTimer13000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::InstanceMapName, vec![Val::from("2@cata")])?,
                        Val::from("Apostle of Baphomet : Humans! Humans have invaded our sanctum!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Ins2fEnterBroadStep::OnTimer16000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::InstanceMapName, vec![Val::from("2@cata")])?,
                        Val::from("Apostle of Baphomet : Kill the humans! Do not stop the revival of our Master!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Ins2fEnterBroadStep::OnTimer18000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::InstanceMapName, vec![Val::from("2@cata")])?,
                        Val::from("Apostle of Baphomet : Hurry up and release the seals of the altars! Our Master's return is upon us!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(
                    Function::DisableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("ins_2f_enter_broad")])?],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn ins_2f_enter_broad(ctx: &Ctx) -> Script {
    ins_2f_enter_broad_run(ctx, Ins2fEnterBroadStep::Start, Vec::new()).map(|_| ())
}

pub fn ins_2f_enter_broad_oninstanceinit(ctx: &Ctx) -> Script {
    ins_2f_enter_broad_run(ctx, Ins2fEnterBroadStep::OnInstanceInit, Vec::new()).map(|_| ())
}

pub fn ins_2f_enter_broad_ondisable(ctx: &Ctx) -> Script {
    ins_2f_enter_broad_run(ctx, Ins2fEnterBroadStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn ins_2f_enter_broad_onenable(ctx: &Ctx) -> Script {
    ins_2f_enter_broad_run(ctx, Ins2fEnterBroadStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn ins_2f_enter_broad_ontimer10000(ctx: &Ctx) -> Script {
    ins_2f_enter_broad_run(ctx, Ins2fEnterBroadStep::OnTimer10000, Vec::new()).map(|_| ())
}

pub fn ins_2f_enter_broad_ontimer13000(ctx: &Ctx) -> Script {
    ins_2f_enter_broad_run(ctx, Ins2fEnterBroadStep::OnTimer13000, Vec::new()).map(|_| ())
}

pub fn ins_2f_enter_broad_ontimer16000(ctx: &Ctx) -> Script {
    ins_2f_enter_broad_run(ctx, Ins2fEnterBroadStep::OnTimer16000, Vec::new()).map(|_| ())
}

pub fn ins_2f_enter_broad_ontimer18000(ctx: &Ctx) -> Script {
    ins_2f_enter_broad_run(ctx, Ins2fEnterBroadStep::OnTimer18000, Vec::new()).map(|_| ())
}

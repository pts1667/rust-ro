use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn brilliant_statue_tt3_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_inputstr_s = Val::from("");
    if (ctx.call(Function::CountItem, vec![Val::from(7423)])? == 0 && ctx.call(Function::CountItem, vec![Val::from(7428)])? == 0) {
        ctx.lines(args![
            "I can feel some magical power from this beautiful stone statue.",
            "There's a little crack between the wings."
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "Red light is shining on a crack in the left wing, and gold light on a crack in the right one.",
            "What will you do?"
        ])?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from("Stick the key into the crack...:Ignore it.")],
        )?) == 2
        {
            ctx.lines(args!["^3355FFYou decide to leave", "the statue alone.^000000"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        if (ctx.call(Function::CountItem, vec![Val::from(7421)])?.number()? > 0
            && ctx.call(Function::CountItem, vec![Val::from(7422)])?.number()? > 0)
        {
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["I'll try the left wing first.", "What key should I put in?"],
            )?;
            ctx.next()?;
            let (input, status) = runtime::input_text(ctx, None, None)?;
            l_inputstr_s = input;
            if (l_inputstr_s.clone() != "Red Key" && l_inputstr_s.clone() != "red key") {
                ctx.mes("It doesn't fit into the crack.")?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines(args![
                "The key fits and makes a click.",
                "You feel the magical power growing stronger..."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Now the right side..."],
            )?;
            ctx.next()?;
            let (input, status) = runtime::input_text(ctx, None, None)?;
            l_inputstr_s = input;
            if (l_inputstr_s.clone() != "Yellow Key" && l_inputstr_s.clone() != "yellow key") {
                ctx.mes("It doesn't fit into the crack.")?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines(args![
                "The key fits and makes a click.",
                "The beak of the bird suddenly opens, and a strong light comes out.",
                "There's a key-shaped object inside..."
            ])?;
            ctx.next()?;
            ctx.mes("^4d4dffThe powerful Blue Key appears.^000000")?;
            ctx.call(Function::GetItem, vec![Val::from(7423), Val::from(1)])?;
            if ctx.var("thana_tower").get()? != 6 {
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines(args![
                "^4d4dffOnce you hold the key, a shocking feeling passes through your head.",
                "You see an illusion of light...^000000"
            ])?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Concentrate on it.:Ignore it.")])? {
                1 => {
                    ctx.lines(args![
                        "This part isn't as clear as before.",
                        "You try to figure out what it's saying..."
                    ])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^b22222...this tower was built by a magical tribe, not by human beings.",
                        "This was quite interesting to me...",
                        "I started to investigate why they built up the tower..."
                    ])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^b22222I found out that this tower was a gate for the magic tribe",
                        "during a millennium war.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines(args![
                        ".........It's not clearly shown......",
                        "^b22222...Morocc has gone through this gate",
                        "from the magical world into the Midgard continent after the millennium war terminated..."
                    ])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^b22222The question is that when she come up here, the Satan appears at the same time...",
                        "The truth is..........^000000"
                    ])?;
                    ctx.next()?;
                    ctx.mes("The illusion shakes, then melts on the surface of the key as light.")?;
                    ctx.var("thana_tower").set(Val::from(7))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(7050), Val::from(7051)])?;
                    ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_COMBOATTACK1")?])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.mes("You decide to ignore it.")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        }
        ctx.mes("You try to fit something in the crack, but to no avail.")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.mes("You've acquired everything you need from this statue.")?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn brilliant_statue_tt3(ctx: &Ctx) -> Script {
    brilliant_statue_tt3_body(ctx, Vec::new()).map(|_| ())
}

fn brilliant_statue_tt3_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.call(Function::CountItem, vec![Val::from(7423)])? == 0 && ctx.call(Function::CountItem, vec![Val::from(7428)])? == 0) {
        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_LEVEL99_4")?])?;
    }
    return Err(Stop::End);
}

pub fn brilliant_statue_tt3_ontouch(ctx: &Ctx) -> Script {
    brilliant_statue_tt3_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn brilliant_statue_tt4_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_big_1 = Val::from(0);
    let mut l_big_2 = Val::from(0);
    let mut l_big_3 = Val::from(0);
    let mut l_small_1 = Val::from(0);
    let mut l_small_2 = Val::from(0);
    if (ctx.call(Function::CountItem, vec![Val::from(7424)])? == 0 && ctx.call(Function::CountItem, vec![Val::from(7429)])? == 0) {
        ctx.lines(args![
            "I can feel some magical power from this brilliant statue.",
            "This must be one of the seals."
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "You see a floating round object with a hole in it.",
            "There're red, yellow, and blue light in a row.",
            "What will you do?"
        ])?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Investigate.:Ignore it.")])?) == 2 {
            ctx.lines(args![
                "^3355FFYou don't see the need to",
                "investigate if nothing seems",
                "peculiar or out of place...^000000"
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        if ((ctx.call(Function::CountItem, vec![Val::from(7421)])?.number()? > 0
            && ctx.call(Function::CountItem, vec![Val::from(7422)])?.number()? > 0)
            && ctx.call(Function::CountItem, vec![Val::from(7423)])?.number()? > 0)
        {
            ctx.lines(args![
                "When you insert the key into the keyhole of the ornament",
                "and match them by corresponding color, the Stone Statue will snap open.",
                "You can see small wheels are moving inside."
            ])?;
            ctx.next()?;
            'b1: {
                let subject1 = Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("Investigate the Machine Chasis:Investigate the Wheels:Cancel")],
                )?);
                let mut matched1 = false;
                let no_case1 = !subject1.loosely_equals(&Val::from(1))
                    && !subject1.loosely_equals(&Val::from(2))
                    && !subject1.loosely_equals(&Val::from(3));
                if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                    matched1 = true;
                }
                if matched1 {
                    ctx.lines(args![
                        "^3355FFThe key in your inventory",
                        "does not seem to be affecting",
                        "the machine's chasis. The",
                        "screen mounted on the side",
                        "is still blank and deactivated.^000000"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                    matched1 = true;
                }
                if matched1 {
                    l_small_1 = Val::from(0);
                    l_small_2 = Val::from(0);
                    l_big_1 = Val::from(0);
                    l_big_2 = Val::from(0);
                    l_big_3 = Val::from(0);
                    ctx.lines(args![
                        "^3355FFYou touch the wheels and",
                        "find that they are actually",
                        "moving very slowly. You note",
                        "that there are 2 small wheels",
                        "and 3 larger wheels, totaling",
                        "5 wheels on this machine.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFThe big wheels are moving",
                        "vertically, up and down, as",
                        "they press against the smaller",
                        "wheels to make them rotate.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFJudging from the machine's",
                        "shaking and jittery noises, the",
                        "wheels may be misaligned.",
                        "You might be able to activate",
                        "the machine by properly ",
                        "aligning all the wheels.^000000"
                    ])?;
                    'l2: loop {
                        if !(true) {
                            break 'l2;
                        }
                        'b2: {
                            ctx.next()?;
                            ctx.lines(args!["^3355FFWhich wheel do", "you want to shift?^000000"])?;
                            match runtime::select_values(
                                ctx,
                                &[Val::from(
                                    "1st Small Wheel:2nd Small Wheel:1st Big Wheel:2nd Big Wheel:3rd Big Wheel:Check Current Wheel Configuration:Reset Wheels to Default Configuration",
                                )],
                            )? {
                                1 => 'b4: {
                                    match runtime::select_values(ctx, &[Val::from("Raise Wheel:Lower Wheel:Press Wheel")])? {
                                        1 => {
                                            ctx.mes("^EE0000*Choom*^000000")?;
                                            l_small_1 = Val::from(2);
                                            if ((((l_small_1.clone() == 2 && l_small_2.clone() == 2) && l_big_1.clone() == 2)
                                                && l_big_2.clone() == 2)
                                                && l_big_3.clone() == 2)
                                            {
                                                break 'b4;
                                            }
                                            break 'b2;
                                        }
                                        2 => {
                                            ctx.mes("^00B2EE*Sneeeep*^000000")?;
                                            l_small_1 = Val::from(1);
                                            if ((((l_small_1.clone() == 1 && l_small_2.clone() == 1) && l_big_1.clone() == 1)
                                                && l_big_2.clone() == 1)
                                                && l_big_3.clone() == 1)
                                            {
                                                break 'b4;
                                            }
                                            break 'b2;
                                        }
                                        3 => {
                                            ctx.mes("^5C246E*Mrreeem*^000000")?;
                                            l_small_1 = Val::from(3);
                                            if ((((l_small_1.clone() == 3 && l_small_2.clone() == 3) && l_big_1.clone() == 3)
                                                && l_big_2.clone() == 3)
                                                && l_big_3.clone() == 3)
                                            {
                                                break 'b4;
                                            }
                                            break 'b2;
                                        }
                                        _ => {}
                                    }
                                }
                                2 => 'b5: {
                                    match runtime::select_values(ctx, &[Val::from("Raise Wheel:Lower Wheel:Press Wheel")])? {
                                        1 => {
                                            ctx.mes("^5C246E*Mrreeem*^000000")?;
                                            l_small_2 = Val::from(3);
                                            if ((((l_small_1.clone() == 3 && l_small_2.clone() == 3) && l_big_1.clone() == 3)
                                                && l_big_2.clone() == 3)
                                                && l_big_3.clone() == 3)
                                            {
                                                break 'b5;
                                            }
                                            break 'b2;
                                        }
                                        2 => {
                                            ctx.mes("^EE0000*Choom*^000000")?;
                                            l_small_2 = Val::from(2);
                                            if ((((l_small_1.clone() == 2 && l_small_2.clone() == 2) && l_big_1.clone() == 2)
                                                && l_big_2.clone() == 2)
                                                && l_big_3.clone() == 2)
                                            {
                                                break 'b5;
                                            }
                                            break 'b2;
                                        }
                                        3 => {
                                            ctx.mes("^00B2EE*Sneeeep*^000000")?;
                                            l_small_2 = Val::from(1);
                                            if ((((l_small_1.clone() == 1 && l_small_2.clone() == 1) && l_big_1.clone() == 1)
                                                && l_big_2.clone() == 1)
                                                && l_big_3.clone() == 1)
                                            {
                                                break 'b5;
                                            }
                                            break 'b2;
                                        }
                                        _ => {}
                                    }
                                }
                                3 => 'b6: {
                                    match runtime::select_values(ctx, &[Val::from("Raise Wheel:Lower Wheel:Vertically Shift Wheel")])? {
                                        1 => {
                                            ctx.mes("^00B2EE*Sneeeep*^000000")?;
                                            l_big_1 = Val::from(1);
                                            if ((((l_small_1.clone() == 1 && l_small_2.clone() == 1) && l_big_1.clone() == 1)
                                                && l_big_2.clone() == 1)
                                                && l_big_3.clone() == 1)
                                            {
                                                break 'b6;
                                            }
                                            break 'b2;
                                        }
                                        2 => {
                                            ctx.mes("^5C246E*Mrreeem*^000000")?;
                                            l_big_1 = Val::from(3);
                                            if ((((l_small_1.clone() == 3 && l_small_2.clone() == 3) && l_big_1.clone() == 3)
                                                && l_big_2.clone() == 3)
                                                && l_big_3.clone() == 3)
                                            {
                                                break 'b6;
                                            }
                                            break 'b2;
                                        }
                                        3 => {
                                            ctx.mes("^EE0000*Choom*^000000")?;
                                            l_big_1 = Val::from(2);
                                            if ((((l_small_1.clone() == 2 && l_small_2.clone() == 2) && l_big_1.clone() == 2)
                                                && l_big_2.clone() == 2)
                                                && l_big_3.clone() == 2)
                                            {
                                                break 'b6;
                                            }
                                            break 'b2;
                                        }
                                        _ => {}
                                    }
                                }
                                4 => 'b7: {
                                    match runtime::select_values(ctx, &[Val::from("Raise Wheel:Lower Wheel:Vertically Shift Wheel")])? {
                                        1 => {
                                            ctx.mes("^EE0000*Choom*^000000")?;
                                            l_big_2 = Val::from(2);
                                            if ((((l_small_1.clone() == 2 && l_small_2.clone() == 2) && l_big_1.clone() == 2)
                                                && l_big_2.clone() == 2)
                                                && l_big_3.clone() == 2)
                                            {
                                                break 'b7;
                                            }
                                            break 'b2;
                                        }
                                        2 => {
                                            ctx.mes("^5C246E*Mrreeem*^000000")?;
                                            l_big_2 = Val::from(3);
                                            if ((((l_small_1.clone() == 3 && l_small_2.clone() == 3) && l_big_1.clone() == 3)
                                                && l_big_2.clone() == 3)
                                                && l_big_3.clone() == 3)
                                            {
                                                break 'b7;
                                            }
                                            break 'b2;
                                        }
                                        3 => {
                                            ctx.mes("^00B2EE*Sneeeep*^000000")?;
                                            l_big_2 = Val::from(1);
                                            if ((((l_small_1.clone() == 1 && l_small_2.clone() == 1) && l_big_1.clone() == 1)
                                                && l_big_2.clone() == 1)
                                                && l_big_3.clone() == 1)
                                            {
                                                break 'b7;
                                            }
                                            break 'b2;
                                        }
                                        _ => {}
                                    }
                                }
                                5 => 'b8: {
                                    match runtime::select_values(ctx, &[Val::from("Raise Wheel:Lower Wheel:Vertically Shift Wheel")])? {
                                        1 => {
                                            ctx.mes("^EE0000*Choom*^000000")?;
                                            l_big_3 = Val::from(2);
                                            if ((((l_small_1.clone() == 2 && l_small_2.clone() == 2) && l_big_1.clone() == 2)
                                                && l_big_2.clone() == 2)
                                                && l_big_3.clone() == 2)
                                            {
                                                break 'b8;
                                            }
                                            break 'b2;
                                        }
                                        2 => {
                                            ctx.mes("^00B2EE*Sneeeep*^000000")?;
                                            l_big_3 = Val::from(1);
                                            if ((((l_small_1.clone() == 1 && l_small_2.clone() == 1) && l_big_1.clone() == 1)
                                                && l_big_2.clone() == 1)
                                                && l_big_3.clone() == 1)
                                            {
                                                break 'b8;
                                            }
                                            break 'b2;
                                        }
                                        3 => {
                                            ctx.mes("^5C246E*Mrreeem*^000000")?;
                                            l_big_3 = Val::from(3);
                                            if ((((l_small_1.clone() == 3 && l_small_2.clone() == 3) && l_big_1.clone() == 3)
                                                && l_big_2.clone() == 3)
                                                && l_big_3.clone() == 3)
                                            {
                                                break 'b8;
                                            }
                                            break 'b2;
                                        }
                                        _ => {}
                                    }
                                }
                                6 => {
                                    if l_small_1.clone() == 0 {
                                        ctx.mes("1st Small Wheel: No Change")?;
                                    } else if l_small_1.clone() == 1 {
                                        ctx.mes("1st Small Wheel: Down")?;
                                    } else if l_small_1.clone() == 2 {
                                        ctx.mes("1st Small Wheel: Up")?;
                                    } else {
                                        ctx.mes("1st Small Wheel: Pressed")?;
                                    }
                                    if l_small_2.clone() == 0 {
                                        ctx.mes("2nd Small Wheel: No Change")?;
                                    } else if l_small_2.clone() == 1 {
                                        ctx.mes("2nd Small Wheel: Pressed")?;
                                    } else if l_small_2.clone() == 2 {
                                        ctx.mes("2nd Small Wheel: Down")?;
                                    } else {
                                        ctx.mes("2nd Small Wheel: Up")?;
                                    }
                                    if l_big_1.clone() == 0 {
                                        ctx.mes("1st Big Wheel: No Change")?;
                                    } else if l_big_1.clone() == 1 {
                                        ctx.mes("1st Big Wheel: Up")?;
                                    } else if l_big_1.clone() == 2 {
                                        ctx.mes("1st Big Wheel: Moved")?;
                                    } else {
                                        ctx.mes("1st Big Wheel: Down")?;
                                    }
                                    if l_big_2.clone() == 0 {
                                        ctx.mes("2nd Big Wheel: No Change")?;
                                    } else if l_big_2.clone() == 1 {
                                        ctx.mes("2nd Big Wheel: Moved")?;
                                    } else if l_big_2.clone() == 2 {
                                        ctx.mes("2nd Big Wheel: Up")?;
                                    } else {
                                        ctx.mes("2nd Big Wheel: Down")?;
                                    }
                                    if l_big_3.clone() == 0 {
                                        ctx.mes("3rd Big Wheel: No Change")?;
                                    } else if l_big_3.clone() == 1 {
                                        ctx.mes("3rd Big Wheel: Down")?;
                                    } else if l_big_3.clone() == 2 {
                                        ctx.mes("3rd Big Wheel: Up")?;
                                    } else {
                                        ctx.mes("3rd Big Wheel: Moved")?;
                                    }
                                    break 'b2;
                                }
                                7 => {
                                    l_small_1 = Val::from(0);
                                    l_small_2 = Val::from(0);
                                    l_big_1 = Val::from(0);
                                    l_big_2 = Val::from(0);
                                    l_big_3 = Val::from(0);
                                    break 'b2;
                                }
                                _ => {}
                            }
                            break 'l2;
                        }
                    }
                    ctx.lines(args![
                        "As you adjust the wheels,",
                        "they suddenly activate",
                        "with a firm click."
                    ])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "A part of the ornament in the stone statue starts to spin,",
                        "and a shining light beam climbs up the elegant statue.",
                        "A strong cursed power emerges and emits a dazzling green light."
                    ])?;
                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_DISPELL")?])?;
                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_POTION6")?])?;
                    ctx.next()?;
                    ctx.mes("^4d4dffAll of a sudden, the Green Key appears and you can feel great power from it.^000000")?;
                    ctx.call(Function::GetItem, vec![Val::from(7424), Val::from(1)])?;
                    if ctx.var("thana_tower").get()? != 7 {
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines(args![
                        "^4d4dffWhen you pick up the key,",
                        "your body trembles",
                        "with an unknown power",
                        "and you see a hallucination with some text.^000000"
                    ])?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Concentrate on it.:Ignore it.")])? {
                        1 => {
                            ctx.lines(args![
                                "^b22222...You found 4 keys",
                                "and finally released 4 spells...",
                                "This tower used to be",
                                "a gate to summon demons ages ago.",
                                "And the last visitor was",
                                "the infamous Satan Morocc."
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^b22222As I followed her trail",
                                "I realized that her spells",
                                "were scattered around the tower.",
                                "Then, somehow, it vanished into the ground."
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^b22222The great battle and",
                                "the protracted war...",
                                "But her purpose was...",
                                "that... sealing of the gate...",
                                "so... I tried to seal...",
                                "but it was incomplete...",
                                "the guard of the gate was...^000000"
                            ])?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["It is hard to read."],
                            )?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^b22222...So I changed the location",
                                "of the coupling spot...",
                                "I mean, push through to the tower",
                                "to tangle it...",
                                "Anyway, I wanted to respect",
                                "her loyalty and block the gate..."
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^b22222If I want to... then I have to",
                                "release the last spell...",
                                "He finally came to meet...",
                                "with his pieces...",
                                "...and then...^000000"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args!["The hallucination wobbles", "and fades into the key."])?;
                            ctx.var("thana_tower").set(Val::from(8))?;
                            ctx.call(Function::ChangeQuest, vec![Val::from(7051), Val::from(7052)])?;
                            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_BEGINSPELL6")?])?;
                            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_SPELLBREAKER")?])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.mes("It was too intense to see the hallucination, so you gave up reading.")?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                }
                if !matched1 && subject1.loosely_equals(&Val::from(3)) {
                    matched1 = true;
                }
                if matched1 {
                    ctx.mes("You decide to ignore it.")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        }
        ctx.mes("You don't have the right key for this keyhole.")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.mes("The spell is already released. You've acquired everything you need from this statue.")?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn brilliant_statue_tt4(ctx: &Ctx) -> Script {
    brilliant_statue_tt4_body(ctx, Vec::new()).map(|_| ())
}

fn brilliant_statue_tt4_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.call(Function::CountItem, vec![Val::from(7424)])? == 0 && ctx.call(Function::CountItem, vec![Val::from(7429)])? == 0) {
        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_LEVEL99_4")?])?;
    }
    return Err(Stop::End);
}

pub fn brilliant_statue_tt4_ontouch(ctx: &Ctx) -> Script {
    brilliant_statue_tt4_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn splendid_sword_tt5_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.call(Function::CountItem, vec![Val::from(7425)])? == 0 && ctx.call(Function::CountItem, vec![Val::from(7430)])? == 0) {
        ctx.lines(args![
            "An old, worn sword hangs above the splendid table.",
            "It emits a gloomy aura."
        ])?;
        ctx.next()?;
        if (((ctx.call(Function::CountItem, vec![Val::from(7421)])?.number()? > 0
            && ctx.call(Function::CountItem, vec![Val::from(7422)])?.number()? > 0)
            && ctx.call(Function::CountItem, vec![Val::from(7423)])?.number()? > 0)
            && ctx.call(Function::CountItem, vec![Val::from(7424)])?.number()? > 0)
        {
            ctx.lines(args![
                "As you approach, the keys in your pocket",
                "suddenly respond with a mysterious power."
            ])?;
            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_ABSORBSPIRITS")?])?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("Observe it.:Ignore it.")])?) == 2 {
                ctx.lines(args!["^3355FFYou decide to leave", "the sword alone.^000000"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines(args![
                "While you get closer and closer, the sword shakes with a loud noise.",
                "Your eyes keep getting drawn to the sword's hilt.",
                "Then you lose control of your hands, and they stretch forth to grasp the handle..."
            ])?;
            ctx.next()?;
            'b1: {
                let subject1 = Val::from(runtime::select_values(
                    ctx,
                    &[Val::from(
                        "Towards the blade of the sword:Towards the hilt of the sword:Towards the table",
                    )],
                )?);
                let mut matched1 = false;
                let no_case1 = !subject1.loosely_equals(&Val::from(1))
                    && !subject1.loosely_equals(&Val::from(2))
                    && !subject1.loosely_equals(&Val::from(3));
                if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                    matched1 = true;
                }
                if matched1 {
                    ctx.lines(args![
                        "By an unknown calling you decide to grab the blade of the sword.",
                        "Your hands get wounded and begin to bleed."
                    ])?;
                    ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_HIT1")?])?;
                    ctx.call(Function::PercentHeal, vec![Val::from(-20), Val::from(0)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                    matched1 = true;
                }
                if matched1 {
                    ctx.lines(args![
                        "By an unknown calling you decide to grab the hilt of the sword.",
                        "The mysterious power from the keys transfers to the sword, and it falls from the table."
                    ])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "At the same time, a little hole appears on the table's surface.",
                        "Now, you hold the sword and..."
                    ])?;
                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_EXIT2")?])?;
                    ctx.next()?;
                    match runtime::select_values(
                        ctx,
                        &[Val::from("Strike the table:Insert the sword into the hole:Bring the sword safely")],
                    )? {
                        1 => {
                            ctx.lines(args![
                                "You strike the table with the sword.",
                                "Numerous conflicting spells act upon it, and you can tell that you chose incorrectly.",
                                "The sword automatically returns to the table, as it was before."
                            ])?;
                            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_HIT1")?])?;
                            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_HIT1")?])?;
                            ctx.call(Function::PercentHeal, vec![Val::from(-20), Val::from(0)])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines(args![
                                "As you insert the sword, lightning flashes around it and black smoke rises from the hole.",
                                "Slowly the smoke clears, and you see an object with a certain shape..."
                            ])?;
                            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_CHANGEDARK")?])?;
                            ctx.next()?;
                            ctx.mes("^4d4dff All of a sudden, a Black Key appears from the smoke and you feel a great cursed power from it.^000000")?;
                            ctx.call(Function::GetItem, vec![Val::from(7425), Val::from(1)])?;
                            if ctx.var("thana_tower").get()? != 8 {
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.next()?;
                            ctx.lines(args![
                                "A drawer had snapped open under the table while you picked up the key and observed it.",
                                "You can see an old notebook inside the drawer."
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "The notebook definitely looks like it contains an important message.",
                                "Maybe this notebook is the one that Rekenber wants to find, even with all the sacrifices."
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^4d4dffYou decide to show the notebook and the keys to Burled.",
                                "You received Varmunt's Journal.^000000"
                            ])?;
                            ctx.var("thana_tower").set(Val::from(9))?;
                            ctx.call(Function::ChangeQuest, vec![Val::from(7052), Val::from(7053)])?;
                            ctx.call(Function::GetItem, vec![Val::from(11011), Val::from(1)])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        3 => {
                            ctx.lines(args![
                                "When you lift the sword, you feel a shock from numerous conflicting spells.",
                                "Right... stealing is the worst.",
                                "The sword automatically returns to the table, as it was before."
                            ])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                }
                if !matched1 && subject1.loosely_equals(&Val::from(3)) {
                    matched1 = true;
                }
                if matched1 {
                    ctx.mes("You stretch your hands to find something under the table, but there is nothing.")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        }
        ctx.lines(args![
            "You strongly feel a cursed, sealed power from here,",
            "but it is hard to figure out what is causing it.",
            "You cannot approach it."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines(args![
        "You have already released a seal using this sword.",
        "You've acquired everything you need from here."
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn splendid_sword_tt5(ctx: &Ctx) -> Script {
    splendid_sword_tt5_body(ctx, Vec::new()).map(|_| ())
}

fn splendid_sword_tt5_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.call(Function::CountItem, vec![Val::from(7425)])? == 0 && ctx.call(Function::CountItem, vec![Val::from(7430)])? == 0) {
        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_LEVEL99_4")?])?;
    }
    return Err(Stop::End);
}

pub fn splendid_sword_tt5_ontouch(ctx: &Ctx) -> Script {
    splendid_sword_tt5_ontouch_body(ctx, Vec::new()).map(|_| ())
}

pub fn charm_stone_admintt01(ctx: &Ctx) -> Script {
    charm_stone_admintt01_run(ctx, CharmStoneAdmintt01Step::Start, Vec::new()).map(|_| ())
}

pub fn charm_stone_admintt01_oninit(ctx: &Ctx) -> Script {
    charm_stone_admintt01_run(ctx, CharmStoneAdmintt01Step::OnInit, Vec::new()).map(|_| ())
}

pub fn charm_stone_admintt01_onenable(ctx: &Ctx) -> Script {
    charm_stone_admintt01_run(ctx, CharmStoneAdmintt01Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn charm_stone_admintt01_ontimer1000(ctx: &Ctx) -> Script {
    charm_stone_admintt01_run(ctx, CharmStoneAdmintt01Step::OnTimer1000, Vec::new()).map(|_| ())
}

pub fn charm_stone_admintt01_ontimer600000(ctx: &Ctx) -> Script {
    charm_stone_admintt01_run(ctx, CharmStoneAdmintt01Step::OnTimer600000, Vec::new()).map(|_| ())
}

pub fn charm_stone_admintt01_ontimer1200000(ctx: &Ctx) -> Script {
    charm_stone_admintt01_run(ctx, CharmStoneAdmintt01Step::OnTimer1200000, Vec::new()).map(|_| ())
}

pub fn charm_stone_admintt01_ontimer1800000(ctx: &Ctx) -> Script {
    charm_stone_admintt01_run(ctx, CharmStoneAdmintt01Step::OnTimer1800000, Vec::new()).map(|_| ())
}

pub fn charm_stone_admintt01_ontimer2400000(ctx: &Ctx) -> Script {
    charm_stone_admintt01_run(ctx, CharmStoneAdmintt01Step::OnTimer2400000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ShiningCrystalTtR1Step {
    Start,
    OnInit,
    OnDisable,
    OnEnable,
}

fn shining_crystal_tt_r1_run(ctx: &Ctx, mut step: ShiningCrystalTtR1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ShiningCrystalTtR1Step::Start => {
                if ctx.call(Function::CountItem, vec![Val::from(7421)])?.number()? > 0 {
                    ctx.mes("The Crystal ball is emitting mysterious power.")?;
                    ctx.next()?;
                    ctx.lines(args![
                        "Beside the crystal ball, you find faded patterns marked on the floor.",
                        "Looks like some kind of keyhole that needs to be inserted with something..."
                    ])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "The red key suddenly responds once you come close to the keyhole.",
                        "You insert the key in the keyhole and it turns out to be a small gem."
                    ])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "When you touch the gem which is dazzling red...",
                        "You feel isolated and depressed deep inside your heart..."
                    ])?;
                    ctx.next()?;
                    ctx.mes("^4d4dffYou have found a Red Charm Stone that is concentrated with cruel fate and a deep darkness.^000000")?;
                    ctx.call(Function::DelItem, vec![Val::from(7421), Val::from(1)])?;
                    ctx.call(Function::GetItem, vec![Val::from(7426), Val::from(1)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines(args![
                    "The Crystal ball is emitting mysterious power.",
                    "Something is definitely there inside... How do I get it to open?"
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ShiningCrystalTtR1Step::OnInit => {
                step = ShiningCrystalTtR1Step::OnDisable;
                continue 'machine;
            }
            ShiningCrystalTtR1Step::OnDisable => {
                ctx.call(Function::DisableNpc, vec![])?;
                return Err(Stop::End);
            }
            ShiningCrystalTtR1Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn shining_crystal_tt_r1(ctx: &Ctx) -> Script {
    shining_crystal_tt_r1_run(ctx, ShiningCrystalTtR1Step::Start, Vec::new()).map(|_| ())
}

pub fn shining_crystal_tt_r1_oninit(ctx: &Ctx) -> Script {
    shining_crystal_tt_r1_run(ctx, ShiningCrystalTtR1Step::OnInit, Vec::new()).map(|_| ())
}

pub fn shining_crystal_tt_r1_ondisable(ctx: &Ctx) -> Script {
    shining_crystal_tt_r1_run(ctx, ShiningCrystalTtR1Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn shining_crystal_tt_r1_onenable(ctx: &Ctx) -> Script {
    shining_crystal_tt_r1_run(ctx, ShiningCrystalTtR1Step::OnEnable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ShiningCrystalTtY1Step {
    Start,
    OnInit,
    OnDisable,
    OnEnable,
}

fn shining_crystal_tt_y1_run(ctx: &Ctx, mut step: ShiningCrystalTtY1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ShiningCrystalTtY1Step::Start => {
                if ctx.call(Function::CountItem, vec![Val::from(7422)])?.number()? > 0 {
                    ctx.mes("The Crystal ball is emitting mysterious power.")?;
                    ctx.next()?;
                    ctx.lines(args![
                        "Beside the crystal ball, you find faded patterns marked on the floor.",
                        "Looks like some kind of keyhole that needs to be inserted with something..."
                    ])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "The yellow key suddenly responds once you come close to the keyhole.",
                        "You insert the key in the keyhole and it turns out to be a small gem."
                    ])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "When you touch the gem which is dazzling yellow...",
                        "Your heart begins to throb as though you are suffering from a broken heart."
                    ])?;
                    ctx.next()?;
                    ctx.mes("^4d4dffYou have found a Yellow Charm Stone that is concentrated with grief and mysterious power.^000000")?;
                    ctx.call(Function::DelItem, vec![Val::from(7422), Val::from(1)])?;
                    ctx.call(Function::GetItem, vec![Val::from(7427), Val::from(1)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines(args![
                    "The Crystal ball is emitting mysterious power.",
                    "Something is definitely there inside... How do I get it to open?"
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ShiningCrystalTtY1Step::OnInit => {
                step = ShiningCrystalTtY1Step::OnDisable;
                continue 'machine;
            }
            ShiningCrystalTtY1Step::OnDisable => {
                ctx.call(Function::DisableNpc, vec![])?;
                return Err(Stop::End);
            }
            ShiningCrystalTtY1Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn shining_crystal_tt_y1(ctx: &Ctx) -> Script {
    shining_crystal_tt_y1_run(ctx, ShiningCrystalTtY1Step::Start, Vec::new()).map(|_| ())
}

pub fn shining_crystal_tt_y1_oninit(ctx: &Ctx) -> Script {
    shining_crystal_tt_y1_run(ctx, ShiningCrystalTtY1Step::OnInit, Vec::new()).map(|_| ())
}

pub fn shining_crystal_tt_y1_ondisable(ctx: &Ctx) -> Script {
    shining_crystal_tt_y1_run(ctx, ShiningCrystalTtY1Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn shining_crystal_tt_y1_onenable(ctx: &Ctx) -> Script {
    shining_crystal_tt_y1_run(ctx, ShiningCrystalTtY1Step::OnEnable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ShiningCrystalTtB1Step {
    Start,
    OnInit,
    OnDisable,
    OnEnable,
}

fn shining_crystal_tt_b1_run(ctx: &Ctx, mut step: ShiningCrystalTtB1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ShiningCrystalTtB1Step::Start => {
                if ctx.call(Function::CountItem, vec![Val::from(7423)])?.number()? > 0 {
                    ctx.mes("The Crystal ball is emitting mysterious power.")?;
                    ctx.next()?;
                    ctx.lines(args![
                        "Beside the crystal ball, you find faded patterns marked on the floor.",
                        "Looks like some kind of keyhole that needs to be inserted with something..."
                    ])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "The blue key suddenly responds once you come close to the keyhole.",
                        "You insert the key in the keyhole and it turns out to be a small gem."
                    ])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "When you touch the gem which is dazzling blue....",
                        "Someone's sobbing comes into your ears as you smell blood on the wind and suddenly you become angry."
                    ])?;
                    ctx.next()?;
                    ctx.mes("^4d4dffYou have found a Blue Charm Stone that is concentrated with grudge and mysterious power.^000000")?;
                    ctx.call(Function::DelItem, vec![Val::from(7423), Val::from(1)])?;
                    ctx.call(Function::GetItem, vec![Val::from(7428), Val::from(1)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines(args![
                    "The Crystal ball is emitting mysterious power.",
                    "Something is definitely there inside... How do I get it to open?"
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ShiningCrystalTtB1Step::OnInit => {
                step = ShiningCrystalTtB1Step::OnDisable;
                continue 'machine;
            }
            ShiningCrystalTtB1Step::OnDisable => {
                ctx.call(Function::DisableNpc, vec![])?;
                return Err(Stop::End);
            }
            ShiningCrystalTtB1Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn shining_crystal_tt_b1(ctx: &Ctx) -> Script {
    shining_crystal_tt_b1_run(ctx, ShiningCrystalTtB1Step::Start, Vec::new()).map(|_| ())
}

pub fn shining_crystal_tt_b1_oninit(ctx: &Ctx) -> Script {
    shining_crystal_tt_b1_run(ctx, ShiningCrystalTtB1Step::OnInit, Vec::new()).map(|_| ())
}

pub fn shining_crystal_tt_b1_ondisable(ctx: &Ctx) -> Script {
    shining_crystal_tt_b1_run(ctx, ShiningCrystalTtB1Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn shining_crystal_tt_b1_onenable(ctx: &Ctx) -> Script {
    shining_crystal_tt_b1_run(ctx, ShiningCrystalTtB1Step::OnEnable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ShiningCrystalTtG1Step {
    Start,
    OnInit,
    OnDisable,
    OnEnable,
}

fn shining_crystal_tt_g1_run(ctx: &Ctx, mut step: ShiningCrystalTtG1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ShiningCrystalTtG1Step::Start => {
                if ctx.call(Function::CountItem, vec![Val::from(7424)])?.number()? > 0 {
                    ctx.mes("The Crystal ball is emitting mysterious power.")?;
                    ctx.next()?;
                    ctx.lines(args![
                        "Beside the crystal ball, you find faded patterns marked on the floor.",
                        "Looks like some kind of keyhole that needs to be inserted with something..."
                    ])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "The green key suddenly responds once you come close to the keyhole.",
                        "You insert the key in the keyhole and it turns out to be a small gem."
                    ])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "When you touch the gem which is dazzling green...",
                        "Your head starts to ache and you feel worried and anxious about something."
                    ])?;
                    ctx.next()?;
                    ctx.mes("^4d4dffYou have found a Green Charm Stone that is concentrated with deep suffering.^000000")?;
                    ctx.call(Function::DelItem, vec![Val::from(7424), Val::from(1)])?;
                    ctx.call(Function::GetItem, vec![Val::from(7429), Val::from(1)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines(args![
                    "The Crystal ball is emitting mysterious power.",
                    "Something is definitely there inside... How do I get it to open?"
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ShiningCrystalTtG1Step::OnInit => {
                step = ShiningCrystalTtG1Step::OnDisable;
                continue 'machine;
            }
            ShiningCrystalTtG1Step::OnDisable => {
                ctx.call(Function::DisableNpc, vec![])?;
                return Err(Stop::End);
            }
            ShiningCrystalTtG1Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn shining_crystal_tt_g1(ctx: &Ctx) -> Script {
    shining_crystal_tt_g1_run(ctx, ShiningCrystalTtG1Step::Start, Vec::new()).map(|_| ())
}

pub fn shining_crystal_tt_g1_oninit(ctx: &Ctx) -> Script {
    shining_crystal_tt_g1_run(ctx, ShiningCrystalTtG1Step::OnInit, Vec::new()).map(|_| ())
}

pub fn shining_crystal_tt_g1_ondisable(ctx: &Ctx) -> Script {
    shining_crystal_tt_g1_run(ctx, ShiningCrystalTtG1Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn shining_crystal_tt_g1_onenable(ctx: &Ctx) -> Script {
    shining_crystal_tt_g1_run(ctx, ShiningCrystalTtG1Step::OnEnable, Vec::new()).map(|_| ())
}

fn gold_religious_statue_tt_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("$@thana_summon").get()? == 0 {
        if ctx.call(Function::CountItem, vec![Val::from(7427)])?.number()? > 0 {
            ctx.lines(args![
                "It's a statue giving off a golden light.",
                "The sword appears to be missing a gem.",
                "As I draw closer to the statue, the Yellow Charm Stone emits a bright light."
            ])?;
            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL5")?])?;
            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_BEGINSPELL5")?])?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Insert the Yellow Charm Stone.:Run away.")],
            )?) == 2
            {
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if ctx.var("$@thana_summon").get()? == 0 {
                ctx.mes("After inserting the Yellow Charm Stone into the sword, the statue begins to react.")?;
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL5")?])?;
                ctx.call(Function::DelItem, vec![Val::from(7427), Val::from(1)])?;
                ctx.var("$@thana_summon").set(Val::from(1))?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#tteffect01::OnEnable")])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("tha_t12"),
                        Val::from("The golden magic power has released part of the seal."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x00ff00"),
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.mes("Someone has already inserted a Charm Stone into this statue.")?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines(args![
            "The statue give off mysterious light.",
            "Strange power has blocked your access."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.mes("The statue has such a strong light that I can't see or touch it.")?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn gold_religious_statue_tt(ctx: &Ctx) -> Script {
    gold_religious_statue_tt_body(ctx, Vec::new()).map(|_| ())
}

fn green_wiseman_statue_tt_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("$@thana_summon").get()? == 1 {
        if ctx.call(Function::CountItem, vec![Val::from(7429)])?.number()? > 0 {
            ctx.lines(args![
                "A statue gives off green light.",
                "The wiseman's wand seems to be missing a gem.",
                "As I draw closer to the statue, the Green Charm Stone emits a bright light."
            ])?;
            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL4")?])?;
            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_BEGINSPELL4")?])?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Insert the Green Charm Stone.:Run away.")],
            )?) == 2
            {
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if ctx.var("$@thana_summon").get()? == 1 {
                ctx.mes("After inserting the Green Charm Stone into the wand, the statue begins to react.")?;
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL4")?])?;
                ctx.call(Function::DelItem, vec![Val::from(7429), Val::from(1)])?;
                ctx.var("$@thana_summon").set(Val::from(2))?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#tteffect02::OnEnable")])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("tha_t12"),
                        Val::from("The green magic power has released part of the seal."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x00ff00"),
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.mes("Someone has already inserted a Charm Stone into this statue.")?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines(args![
            "The statue give off mysterious light.",
            "Strange power has blocked your access."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.mes("The statue has such a strong light that I can't see or touch it.")?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn green_wiseman_statue_tt(ctx: &Ctx) -> Script {
    green_wiseman_statue_tt_body(ctx, Vec::new()).map(|_| ())
}

fn blue_angel_statue_tt_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("$@thana_summon").get()? == 2 {
        if ctx.call(Function::CountItem, vec![Val::from(7428)])?.number()? > 0 {
            ctx.lines(args![
                "An angel statue is covered with a blue light.",
                "A gem seems to be missing from the statue's belt.",
                "As I draw closer to the statue, the Blue Charm Stone emits a bright light."
            ])?;
            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL2")?])?;
            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_BEGINSPELL2")?])?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Insert the Blue Charm Stone.:Run away.")],
            )?) == 2
            {
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if ctx.var("$@thana_summon").get()? == 2 {
                ctx.mes("After inserting the Blue Charm Stone into the belt, the statue begins to react.")?;
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL2")?])?;
                ctx.call(Function::DelItem, vec![Val::from(7428), Val::from(1)])?;
                ctx.var("$@thana_summon").set(Val::from(3))?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#tteffect03::OnEnable")])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("tha_t12"),
                        Val::from("The blue magic power has released part of the seal."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x00ff00"),
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.mes("Someone has already inserted a Charm Stone into this statue.")?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines(args![
            "The statue give off mysterious light.",
            "Strange power has blocked you to access."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.mes("The statue has such a strong light that I can't see or touch it.")?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn blue_angel_statue_tt(ctx: &Ctx) -> Script {
    blue_angel_statue_tt_body(ctx, Vec::new()).map(|_| ())
}

fn bloody_knight_statue_tt_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("$@thana_summon").get()? == 3 {
        if ctx.call(Function::CountItem, vec![Val::from(7426)])?.number()? > 0 {
            ctx.lines(args![
                "A statue is shining like blood.",
                "A gem seems to be missing from the heart area of its armor.",
                "As I draw closer to the statue, the Red Charm Stone emits a bright light."
            ])?;
            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL3")?])?;
            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_BEGINSPELL3")?])?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Insert the Red Charm Stone.:Run away.")],
            )?) == 2
            {
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if ctx.var("$@thana_summon").get()? == 3 {
                ctx.mes("After inserting the Red Charm Stone into the armor, the statue begins to react.")?;
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL3")?])?;
                ctx.call(Function::DelItem, vec![Val::from(7426), Val::from(1)])?;
                ctx.var("$@thana_summon").set(Val::from(4))?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#tteffect04::OnEnable")])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("tha_t12"),
                        Val::from("The red magic power has released part of the seal."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x00ff00"),
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.mes("Someone has already inserted a Charm Stone into this statue.")?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines(args![
            "The statue give off mysterious light.",
            "Strange power has blocked you to access."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.mes("The statue has such a strong light that I can't see or touch it.")?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn bloody_knight_statue_tt(ctx: &Ctx) -> Script {
    bloody_knight_statue_tt_body(ctx, Vec::new()).map(|_| ())
}

fn dark_devil_statue_tt_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("$@thana_summon").get()? == 4 {
        if ctx.call(Function::CountItem, vec![Val::from(7430)])?.number()? > 0 {
            ctx.lines(args![
                "A devil statue emits dark light.",
                "The right eye seems to be missing a gem.",
                "As I draw closer to the statue, the Black Charm Stone emits a bright light."
            ])?;
            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL7")?])?;
            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_BEGINSPELL7")?])?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Insert the Black Charm Stone.:Run away.")],
            )?) == 2
            {
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if ctx.var("$@thana_summon").get()? == 4 {
                ctx.mes("After inserting the Red Charm Stone into the eye socket, the statue begins to react.")?;
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL7")?])?;
                ctx.call(Function::DelItem, vec![Val::from(7430), Val::from(1)])?;
                ctx.var("$@thana_summon").set(Val::from(5))?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#gateto_thanatos::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#tteffect01::OnStop")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#tteffect02::OnStop")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#tteffect03::OnStop")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#tteffect04::OnStop")])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("tha_t12"),
                        Val::from("The Black Charm Stone's magic power has released the final seal."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x00ff00"),
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.mes("Someone has already inserted a Charm Stone into this statue.")?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines(args![
            "The statue give off mysterious light.",
            "Strange power has blocked you to access."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.mes("The statue has such a strong light that I can't see or touch it.")?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn dark_devil_statue_tt(ctx: &Ctx) -> Script {
    dark_devil_statue_tt_body(ctx, Vec::new()).map(|_| ())
}

pub fn tteffect01(ctx: &Ctx) -> Script {
    tteffect01_run(ctx, Tteffect01Step::Start, Vec::new()).map(|_| ())
}

pub fn tteffect01_onenable(ctx: &Ctx) -> Script {
    tteffect01_run(ctx, Tteffect01Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn tteffect01_onstop(ctx: &Ctx) -> Script {
    tteffect01_run(ctx, Tteffect01Step::OnStop, Vec::new()).map(|_| ())
}

pub fn tteffect01_ontimer500(ctx: &Ctx) -> Script {
    tteffect01_run(ctx, Tteffect01Step::OnTimer500, Vec::new()).map(|_| ())
}

pub fn tteffect01_ontimer1000(ctx: &Ctx) -> Script {
    tteffect01_run(ctx, Tteffect01Step::OnTimer1000, Vec::new()).map(|_| ())
}

pub fn tteffect01_ontimer1500(ctx: &Ctx) -> Script {
    tteffect01_run(ctx, Tteffect01Step::OnTimer1500, Vec::new()).map(|_| ())
}

pub fn tteffect01_ontimer2000(ctx: &Ctx) -> Script {
    tteffect01_run(ctx, Tteffect01Step::OnTimer2000, Vec::new()).map(|_| ())
}

pub fn tteffect01_ontimer2500(ctx: &Ctx) -> Script {
    tteffect01_run(ctx, Tteffect01Step::OnTimer2500, Vec::new()).map(|_| ())
}

pub fn tteffect01_ontimer3000(ctx: &Ctx) -> Script {
    tteffect01_run(ctx, Tteffect01Step::OnTimer3000, Vec::new()).map(|_| ())
}

pub fn tteffect02(ctx: &Ctx) -> Script {
    tteffect02_run(ctx, Tteffect02Step::Start, Vec::new()).map(|_| ())
}

pub fn tteffect02_onenable(ctx: &Ctx) -> Script {
    tteffect02_run(ctx, Tteffect02Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn tteffect02_onstop(ctx: &Ctx) -> Script {
    tteffect02_run(ctx, Tteffect02Step::OnStop, Vec::new()).map(|_| ())
}

pub fn tteffect02_ontimer500(ctx: &Ctx) -> Script {
    tteffect02_run(ctx, Tteffect02Step::OnTimer500, Vec::new()).map(|_| ())
}

pub fn tteffect02_ontimer1000(ctx: &Ctx) -> Script {
    tteffect02_run(ctx, Tteffect02Step::OnTimer1000, Vec::new()).map(|_| ())
}

pub fn tteffect02_ontimer1500(ctx: &Ctx) -> Script {
    tteffect02_run(ctx, Tteffect02Step::OnTimer1500, Vec::new()).map(|_| ())
}

pub fn tteffect02_ontimer2000(ctx: &Ctx) -> Script {
    tteffect02_run(ctx, Tteffect02Step::OnTimer2000, Vec::new()).map(|_| ())
}

pub fn tteffect02_ontimer2500(ctx: &Ctx) -> Script {
    tteffect02_run(ctx, Tteffect02Step::OnTimer2500, Vec::new()).map(|_| ())
}

pub fn tteffect02_ontimer3000(ctx: &Ctx) -> Script {
    tteffect02_run(ctx, Tteffect02Step::OnTimer3000, Vec::new()).map(|_| ())
}

pub fn tteffect03(ctx: &Ctx) -> Script {
    tteffect03_run(ctx, Tteffect03Step::Start, Vec::new()).map(|_| ())
}

pub fn tteffect03_onenable(ctx: &Ctx) -> Script {
    tteffect03_run(ctx, Tteffect03Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn tteffect03_onstop(ctx: &Ctx) -> Script {
    tteffect03_run(ctx, Tteffect03Step::OnStop, Vec::new()).map(|_| ())
}

pub fn tteffect03_ontimer500(ctx: &Ctx) -> Script {
    tteffect03_run(ctx, Tteffect03Step::OnTimer500, Vec::new()).map(|_| ())
}

pub fn tteffect03_ontimer1000(ctx: &Ctx) -> Script {
    tteffect03_run(ctx, Tteffect03Step::OnTimer1000, Vec::new()).map(|_| ())
}

pub fn tteffect03_ontimer1500(ctx: &Ctx) -> Script {
    tteffect03_run(ctx, Tteffect03Step::OnTimer1500, Vec::new()).map(|_| ())
}

pub fn tteffect03_ontimer2000(ctx: &Ctx) -> Script {
    tteffect03_run(ctx, Tteffect03Step::OnTimer2000, Vec::new()).map(|_| ())
}

pub fn tteffect03_ontimer2500(ctx: &Ctx) -> Script {
    tteffect03_run(ctx, Tteffect03Step::OnTimer2500, Vec::new()).map(|_| ())
}

pub fn tteffect03_ontimer3000(ctx: &Ctx) -> Script {
    tteffect03_run(ctx, Tteffect03Step::OnTimer3000, Vec::new()).map(|_| ())
}

pub fn tteffect04(ctx: &Ctx) -> Script {
    tteffect04_run(ctx, Tteffect04Step::Start, Vec::new()).map(|_| ())
}

pub fn tteffect04_onenable(ctx: &Ctx) -> Script {
    tteffect04_run(ctx, Tteffect04Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn tteffect04_onstop(ctx: &Ctx) -> Script {
    tteffect04_run(ctx, Tteffect04Step::OnStop, Vec::new()).map(|_| ())
}

pub fn tteffect04_ontimer500(ctx: &Ctx) -> Script {
    tteffect04_run(ctx, Tteffect04Step::OnTimer500, Vec::new()).map(|_| ())
}

pub fn tteffect04_ontimer1000(ctx: &Ctx) -> Script {
    tteffect04_run(ctx, Tteffect04Step::OnTimer1000, Vec::new()).map(|_| ())
}

pub fn tteffect04_ontimer1500(ctx: &Ctx) -> Script {
    tteffect04_run(ctx, Tteffect04Step::OnTimer1500, Vec::new()).map(|_| ())
}

pub fn tteffect04_ontimer2000(ctx: &Ctx) -> Script {
    tteffect04_run(ctx, Tteffect04Step::OnTimer2000, Vec::new()).map(|_| ())
}

pub fn tteffect04_ontimer2500(ctx: &Ctx) -> Script {
    tteffect04_run(ctx, Tteffect04Step::OnTimer2500, Vec::new()).map(|_| ())
}

pub fn tteffect04_ontimer3000(ctx: &Ctx) -> Script {
    tteffect04_run(ctx, Tteffect04Step::OnTimer3000, Vec::new()).map(|_| ())
}

pub fn tteffect05(ctx: &Ctx) -> Script {
    tteffect05_run(ctx, Tteffect05Step::Start, Vec::new()).map(|_| ())
}

pub fn tteffect05_onenable(ctx: &Ctx) -> Script {
    tteffect05_run(ctx, Tteffect05Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn tteffect05_onstop(ctx: &Ctx) -> Script {
    tteffect05_run(ctx, Tteffect05Step::OnStop, Vec::new()).map(|_| ())
}

pub fn tteffect05_ontimer500(ctx: &Ctx) -> Script {
    tteffect05_run(ctx, Tteffect05Step::OnTimer500, Vec::new()).map(|_| ())
}

pub fn tteffect05_ontimer1000(ctx: &Ctx) -> Script {
    tteffect05_run(ctx, Tteffect05Step::OnTimer1000, Vec::new()).map(|_| ())
}

pub fn tteffect05_ontimer1500(ctx: &Ctx) -> Script {
    tteffect05_run(ctx, Tteffect05Step::OnTimer1500, Vec::new()).map(|_| ())
}

pub fn tteffect05_ontimer2000(ctx: &Ctx) -> Script {
    tteffect05_run(ctx, Tteffect05Step::OnTimer2000, Vec::new()).map(|_| ())
}

pub fn tteffect05_ontimer2500(ctx: &Ctx) -> Script {
    tteffect05_run(ctx, Tteffect05Step::OnTimer2500, Vec::new()).map(|_| ())
}

pub fn tteffect05_ontimer3000(ctx: &Ctx) -> Script {
    tteffect05_run(ctx, Tteffect05Step::OnTimer3000, Vec::new()).map(|_| ())
}

pub fn gateto_thanatos(ctx: &Ctx) -> Script {
    gateto_thanatos_run(ctx, GatetoThanatosStep::Start, Vec::new()).map(|_| ())
}

pub fn gateto_thanatos_oninit(ctx: &Ctx) -> Script {
    gateto_thanatos_run(ctx, GatetoThanatosStep::OnInit, Vec::new()).map(|_| ())
}

pub fn gateto_thanatos_onenable(ctx: &Ctx) -> Script {
    gateto_thanatos_run(ctx, GatetoThanatosStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn gateto_thanatos_onon2(ctx: &Ctx) -> Script {
    gateto_thanatos_run(ctx, GatetoThanatosStep::OnOn2, Vec::new()).map(|_| ())
}

pub fn gateto_thanatos_ontouch(ctx: &Ctx) -> Script {
    gateto_thanatos_run(ctx, GatetoThanatosStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn gateto_thanatos_ontimer6000(ctx: &Ctx) -> Script {
    gateto_thanatos_run(ctx, GatetoThanatosStep::OnTimer6000, Vec::new()).map(|_| ())
}

pub fn gateto_thanatos_ontimer1000(ctx: &Ctx) -> Script {
    gateto_thanatos_run(ctx, GatetoThanatosStep::OnTimer1000, Vec::new()).map(|_| ())
}

pub fn gateto_thanatos_ontimer5000(ctx: &Ctx) -> Script {
    gateto_thanatos_run(ctx, GatetoThanatosStep::OnTimer5000, Vec::new()).map(|_| ())
}

pub fn gateto_thanatos_ontimer3000(ctx: &Ctx) -> Script {
    gateto_thanatos_run(ctx, GatetoThanatosStep::OnTimer3000, Vec::new()).map(|_| ())
}

fn memory_seal_tt1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn memory_seal_tt1(ctx: &Ctx) -> Script {
    memory_seal_tt1_body(ctx, Vec::new()).map(|_| ())
}

fn memory_seal_tt1_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![ctx.call(Function::StrNpcInfo, vec![Val::from(0)])?])?;
    ctx.var(".hide").set(Val::from(0))?;
    return Err(Stop::End);
}

pub fn memory_seal_tt1_onenable(ctx: &Ctx) -> Script {
    memory_seal_tt1_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn memory_seal_tt1_ondisable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![ctx.call(Function::StrNpcInfo, vec![Val::from(0)])?])?;
    ctx.var(".hide").set(Val::from(0))?;
    return Err(Stop::End);
}

pub fn memory_seal_tt1_ondisable(ctx: &Ctx) -> Script {
    memory_seal_tt1_ondisable_body(ctx, Vec::new()).map(|_| ())
}

fn memory_seal_tt1_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_exact_tt_s = Val::from("");
    let mut l_i: Vec<Val> = Vec::new();
    let mut l_inputstr_s = Val::from("");
    let mut l_j_s = Val::from("");
    let mut l_seal = Val::from(0);
    let mut l_str_s = Val::from("");
    l_seal = runtime::atoi(&runtime::charat(
        &ctx.call(Function::StrNpcInfo, vec![Val::from(2)])?,
        &Val::from(2),
    )?);
    if runtime::op(
        &ctx.var(".hide").get()?,
        "&",
        &runtime::op(&Val::from(1), "<<", &l_seal.clone())?,
    )?
    .is_true()
    {
        return Err(Stop::End);
    }
    let subject1 = l_seal.clone();
    if subject1 == 1 {
        let base = Val::from(0).number()?;
        runtime::local_set(&mut l_i, &Val::from(base + 0), Val::from(7437), false);
        runtime::local_set(&mut l_i, &Val::from(base + 1), Val::from(1711), false);
        runtime::local_set(&mut l_i, &Val::from(base + 2), Val::from(217), false);
        runtime::local_set(&mut l_i, &Val::from(base + 3), Val::from(167), false);
        runtime::local_set(&mut l_i, &Val::from(base + 4), ctx.constant("EF_LANDPROTECTOR")?, false);
        l_j_s = Val::from("Misery");
    } else if subject1 == 2 {
        let base = Val::from(0).number()?;
        runtime::local_set(&mut l_i, &Val::from(base + 0), Val::from(7436), false);
        runtime::local_set(&mut l_i, &Val::from(base + 1), Val::from(1712), false);
        runtime::local_set(&mut l_i, &Val::from(base + 2), Val::from(202), false);
        runtime::local_set(&mut l_i, &Val::from(base + 3), Val::from(75), false);
        runtime::local_set(&mut l_i, &Val::from(base + 4), ctx.constant("EF_CRASHEARTH")?, false);
        l_j_s = Val::from("Agony");
    } else if subject1 == 3 {
        let base = Val::from(0).number()?;
        runtime::local_set(&mut l_i, &Val::from(base + 0), Val::from(7438), false);
        runtime::local_set(&mut l_i, &Val::from(base + 1), Val::from(1709), false);
        runtime::local_set(&mut l_i, &Val::from(base + 2), Val::from(80), false);
        runtime::local_set(&mut l_i, &Val::from(base + 3), Val::from(76), false);
        runtime::local_set(&mut l_i, &Val::from(base + 4), ctx.constant("EF_REPAIRWEAPON")?, false);
        l_j_s = Val::from("Hatred");
    } else if subject1 == 4 {
        let base = Val::from(0).number()?;
        runtime::local_set(&mut l_i, &Val::from(base + 0), Val::from(7439), false);
        runtime::local_set(&mut l_i, &Val::from(base + 1), Val::from(1710), false);
        runtime::local_set(&mut l_i, &Val::from(base + 2), Val::from(62), false);
        runtime::local_set(&mut l_i, &Val::from(base + 3), Val::from(171), false);
        runtime::local_set(&mut l_i, &Val::from(base + 4), ctx.constant("EF_REMOVETRAP")?, false);
        l_j_s = Val::from("Despair");
    }
    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_GUMGANG")?])?;
    if ctx.var("$@thana_summon2").get()?.number()? > 3 {
        ctx.lines(args![
            "^3355FFYou cannot approach",
            "the crest because it is",
            "generating intense heat.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines(args![
        "The seal seems to be dormant.",
        "There is a fragment missing from the crest of the Seal.",
        "Surely I saw a familiar fragment before..."
    ])?;
    ctx.next()?;
    let (input, status) = runtime::input_text(ctx, None, None)?;
    l_inputstr_s = input;
    l_exact_tt_s = (Val::from("Fragment of ") + l_j_s.clone());
    if (ctx
        .call(Function::CountItem, vec![runtime::local_get(&l_i, &Val::from(0), false)])?
        .number()?
        > 0
        && l_exact_tt_s.clone().loosely_equals(&l_inputstr_s.clone()))
    {
        ctx.lines(args![
            "^3355FFYou insert the",
            l_exact_tt_s.clone(),
            "into the crest, causing",
            "its glow to intensify.^000000"
        ])?;
        ctx.close_window()?;
        ctx.call(Function::DisableNpc, vec![ctx.call(Function::StrNpcInfo, vec![Val::from(0)])?])?;
        ctx.var(".hide").set(runtime::op(
            &ctx.var(".hide").get()?,
            "|",
            &runtime::op(&Val::from(1), "<<", &l_seal.clone())?,
        )?)?;
        ctx.call(
            Function::DelItem,
            vec![runtime::local_get(&l_i, &Val::from(0), false), Val::from(1)],
        )?;
        ctx.call(Function::NpcSpecialEffect, vec![runtime::local_get(&l_i, &Val::from(4), false)])?;
        ctx.call(
            Function::Monster,
            vec![
                Val::from("thana_boss"),
                runtime::local_get(&l_i, &Val::from(2), false),
                runtime::local_get(&l_i, &Val::from(3), false),
                l_j_s.clone(),
                runtime::local_get(&l_i, &Val::from(1), false),
                Val::from(1),
                (ctx.call(Function::StrNpcInfo, vec![Val::from(0)])? + Val::from("::OnMyMobDead")),
            ],
        )?;
        let subject2 = ctx.var("$@thana_summon2").get()?;
        if subject2 == 0 {
            l_str_s = ((Val::from("... who... released... the... Memory... of... ") + l_j_s.clone()) + Val::from("...?"));
        } else if subject2 == 1 {
            l_str_s = ((Val::from("... why... did you... release... the... Memory... of... ") + l_j_s.clone()) + Val::from("...?"));
        } else if subject2 == 2 {
            l_str_s = ((Val::from("... ugh... stop it... the Memory of ") + l_j_s.clone()) + Val::from("..."));
        } else {
            l_str_s = Val::from("... finally... you released the last piece of Memory...");
        }
        ctx.call(
            Function::MapAnnounce,
            vec![
                Val::from("thana_boss"),
                l_str_s.clone(),
                ctx.constant("BC_MAP")?,
                Val::from("0x7b68ee"),
            ],
        )?;
        return Err(Stop::End);
    }
    ctx.lines(args![
        "^3355FFThat action had no",
        "effect. You'll have to",
        "try something else.^000000"
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn memory_seal_tt1_ontouch(ctx: &Ctx) -> Script {
    memory_seal_tt1_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn memory_seal_tt1_onmymobdead_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.var("$@thana_summon2").set((ctx.var("$@thana_summon2").get()? + Val::from(1)))?;
    if ctx.var("$@thana_summon2").get()? == 4 {
        ctx.call(
            Function::MapWarp,
            vec![Val::from("thana_boss"), Val::from("thana_boss"), Val::from(141), Val::from(228)],
        )?;
    }
    return Err(Stop::End);
}

pub fn memory_seal_tt1_onmymobdead(ctx: &Ctx) -> Script {
    memory_seal_tt1_onmymobdead_body(ctx, Vec::new()).map(|_| ())
}

fn thanatos_seal_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn thanatos_seal(ctx: &Ctx) -> Script {
    thanatos_seal_body(ctx, Vec::new()).map(|_| ())
}

fn thanatos_seal_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("#thanatos_seal")])?;
    return Err(Stop::End);
}

pub fn thanatos_seal_onenable(ctx: &Ctx) -> Script {
    thanatos_seal_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn thanatos_seal_ondisable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("#thanatos_seal")])?;
    return Err(Stop::End);
}

pub fn thanatos_seal_ondisable(ctx: &Ctx) -> Script {
    thanatos_seal_ondisable_body(ctx, Vec::new()).map(|_| ())
}

fn thanatos_seal_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("$@thana_summon2").get()? == 4 {
        ctx.call(Function::InitNpcTimer, vec![])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("Memory Seal#tt5::OnEnable")])?;
        ctx.call(Function::DisableNpc, vec![Val::from("#thanatos_seal")])?;
    }
    return Err(Stop::End);
}

pub fn thanatos_seal_ontouch(ctx: &Ctx) -> Script {
    thanatos_seal_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn thanatos_seal_ontimer1000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("thana_boss"),
            Val::from("Thanatos : RAWWWWWWWWWWWWWWWWR!"),
            ctx.constant("BC_MAP")?,
            Val::from("0xff0000"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn thanatos_seal_ontimer1000(ctx: &Ctx) -> Script {
    thanatos_seal_ontimer1000_body(ctx, Vec::new()).map(|_| ())
}

fn thanatos_seal_ontimer4000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("thana_boss"),
            Val::from("Thanatos : ... crashed... broken... spread... and come back as tears..."),
            ctx.constant("BC_MAP")?,
            Val::from("0xff0000"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn thanatos_seal_ontimer4000(ctx: &Ctx) -> Script {
    thanatos_seal_ontimer4000_body(ctx, Vec::new()).map(|_| ())
}

fn thanatos_seal_ontimer7000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("thana_boss"),
            Val::from("Thanatos : I live again to fulfill my ambition!"),
            ctx.constant("BC_MAP")?,
            Val::from("0xff0000"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn thanatos_seal_ontimer7000(ctx: &Ctx) -> Script {
    thanatos_seal_ontimer7000_body(ctx, Vec::new()).map(|_| ())
}

fn thanatos_seal_ontimer10000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("thana_boss"),
            Val::from("Thanatos : ... gives off a strong bloody scent... soaked in sorrow..."),
            ctx.constant("BC_MAP")?,
            Val::from("0xff0000"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn thanatos_seal_ontimer10000(ctx: &Ctx) -> Script {
    thanatos_seal_ontimer10000_body(ctx, Vec::new()).map(|_| ())
}

fn thanatos_seal_ontimer13000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("thana_boss"),
            Val::from("Thanatos : I will remove all people who invade this continent again!"),
            ctx.constant("BC_MAP")?,
            Val::from("0xff0000"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn thanatos_seal_ontimer13000(ctx: &Ctx) -> Script {
    thanatos_seal_ontimer13000_body(ctx, Vec::new()).map(|_| ())
}

fn thanatos_seal_ontimer16000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("thana_boss"),
            Val::from("Thanatos : Come, be the first to fall before my might!"),
            ctx.constant("BC_MAP")?,
            Val::from("0xff0000"),
        ],
    )?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("Memory Seal#tt5::OnDisable")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("#sommon_thanatos::OnEnable")])?;
    ctx.var("$@thana_summon2").set(Val::from(5))?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn thanatos_seal_ontimer16000(ctx: &Ctx) -> Script {
    thanatos_seal_ontimer16000_body(ctx, Vec::new()).map(|_| ())
}

fn memory_seal_tt5_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn memory_seal_tt5(ctx: &Ctx) -> Script {
    memory_seal_tt5_body(ctx, Vec::new()).map(|_| ())
}

use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum EiFelleRepay01Step {
    Start,
    SReward,
    SBonusReward,
}

fn ei_felle_repay01_run(ctx: &Ctx, mut step: EiFelleRepay01Step, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_arg1 = Val::from(0);
    let mut l_input = Val::from(0);
    let mut l_j = Val::from(0);
    let mut l_m = Val::from(0);
    let mut l_medals = Val::from(0);
    let mut l_type_s: Vec<Val> = Vec::new();
    let mut l_weapon_id: Vec<Val> = Vec::new();
    let mut l_weapon_s: Vec<Val> = Vec::new();
    'machine: loop {
        match step {
            EiFelleRepay01Step::Start => {
                if !(ctx.var("ein_medal01").get()?.is_true()) {
                    ctx.lines_as(
                        "Ei'felle",
                        args![
                            "Curses! We need to deliver",
                            "what our customers ordered,",
                            "but we've been making nothing",
                            "but shipshod products! If we",
                            "only had that metal, we could",
                            "pump up our product quality!"
                        ],
                    )?;
                    ctx.next()?;
                    let choice = runtime::select_values(ctx, &[Val::from("What metal are you talking about?")])?;
                    ctx.var("@menu").set(choice)?;
                    ctx.lines_as(
                        "Ei'felle",
                        args![
                            "There's a small village",
                            "at the outskirts of the",
                            "Schwarzwald Republic that",
                            "gives these special medals",
                            "that are made of this metal",
                            "which we desperately need..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ei'felle",
                        args![
                            "We've sent one of our best",
                            "guildsmen to get some of those",
                            "medals for us, but he hasn't",
                            "reported back to us quite yet.",
                            "I'm getting pretty anxious..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ei'felle",
                        args![
                            "I mean, we need a whole",
                            "lot of that metal to fill out",
                            "our orders and finish our",
                            "manufacturing research,",
                            "but so far, none of us have",
                            "been able to get any medals..."
                        ],
                    )?;
                    ctx.next()?;
                    if ctx.call(Function::CountItem, vec![Val::from("Marvelous_Medal")])?.is_true() {
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![
                                "Oh! Are you talking",
                                "about the medals that",
                                "they give as rewards in",
                                "the Monster Race Arena?",
                                "I have some of those."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ei'felle",
                            args![
                                "Huh? Show them to me...",
                                "Yes! That's exactly what",
                                "we need! Would you please",
                                "donate your medals so that we",
                                "can finally make some quality",
                                "products for our customers?"
                            ],
                        )?;
                        ctx.next()?;
                        ei_felle_repay01_run(ctx, EiFelleRepay01Step::SReward, vec![])?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        "Ei'felle",
                        args![
                            "If you happen to obtain any",
                            "medals from the Monster",
                            "Race Arena in Hugel, then",
                            "please bring some of them",
                            "to me. I'll be sure to repay",
                            "you for your kindness..."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("ein_medal01").get()?.number()? < 500 {
                    ctx.lines_as(
                        "Ei'felle",
                        args![
                            "Oh, how have you been?",
                            "Thank you so much for",
                            "donating so many medals,",
                            "they've been helpful in my",
                            "research. Still, I need more",
                            "and more of them everyday..."
                        ],
                    )?;
                    ctx.next()?;
                    if ctx.call(Function::CountItem, vec![Val::from("Marvelous_Medal")])?.is_true() {
                        ctx.lines_as(
                            "Ei'felle",
                            args![
                                "The other Blacksmith",
                                "Guildsmen are doing their",
                                "best to collect Prize Medals",
                                "in Hugel, but they keep failing",
                                "to win them! If you have any",
                                "medals, then may I have some?"
                            ],
                        )?;
                        ctx.next()?;
                        ei_felle_repay01_run(ctx, EiFelleRepay01Step::SReward, vec![])?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        "Ei'felle",
                        args![
                            "If you happen to obtain any",
                            "medals from the Monster",
                            "Race Arena in Hugel, then",
                            "please bring some of them",
                            "to me. I'll be sure to repay",
                            "you for your kindness..."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if !(ctx.call(Function::CheckWeight, vec![Val::from("Knife"), Val::from(1)])?.is_true()) {
                    ctx.lines_as(
                        "Ei'felle",
                        args![
                            "Goodness, you're carrying",
                            "so many things with you!",
                            "You'd better put some of",
                            "it away in Kafra Storage",
                            "before you're overwhelmed",
                            "by the bulk of your items!"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if (ctx.var("ein_medal01").get()?.number()? > 499 && ctx.var("ein_medal01").get()?.number()? < 1500) {
                    if ctx.var("ein_medal01").get()?.number()? < 1000 {
                        ctx.lines_as(
                            "Ei'felle",
                            args![
                                "Ah, welcome back! I've finally",
                                "made a breakthrough in my",
                                "metal research! Look, I've",
                                "developed this Glittering",
                                "Jacket! It's lightweight and",
                                "very durable, you see?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ei'felle",
                            args![
                                "I wouldn't be able to have",
                                "completed this without your",
                                "help. Now, would you like to",
                                "receive this Glittering Jacket",
                                "as my way of repaying you?"
                            ],
                        )?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(
                            ctx,
                            &[Val::from("Accept:Wait for Further Development")],
                        )?) == 1
                        {
                            ei_felle_repay01_run(ctx, EiFelleRepay01Step::SBonusReward, vec![Val::from(500), Val::from(2319)])?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as(
                            "Ei'felle",
                            args![
                                "Ah, I see. You'd rather wait",
                                "until we develop something",
                                "more to your liking. In that",
                                "case, would you please donate",
                                "more medals to my research?",
                                "We're always low on them..."
                            ],
                        )?;
                        ctx.next()?;
                    } else if ctx.var("ein_medal01").get()?.number()? < 1500 {
                        ctx.lines_as(
                            "Ei'felle",
                            args![
                                "Oh, you're back!",
                                "Thanks to all the medals",
                                "that you've donated, I'm now",
                                "able to manufacture a set",
                                "of slotted armor imbued",
                                "with a property of your choice."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ei'felle",
                            args![
                                "If you'd like, I can repay you",
                                "now by creating a set of slotted elemental armor for you, or we",
                                "can wait for you to donate more",
                                "medals until I can develop",
                                "something else for you."
                            ],
                        )?;
                        ctx.next()?;
                        'b1: {
                            let subject1 = Val::from(runtime::select_values(
                                ctx,
                                &[Val::from("Accept Armor:Can I have something else?:Wait for Further Development")],
                            )?);
                            let mut matched1 = false;
                            let no_case1 = !subject1.loosely_equals(&Val::from(1))
                                && !subject1.loosely_equals(&Val::from(2))
                                && !subject1.loosely_equals(&Val::from(3));
                            if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                                matched1 = true;
                            }
                            if matched1 {
                                ctx.lines_as(
                                    "Ei'felle",
                                    args![
                                        "I can manufacture one set of",
                                        "slotted Armor imbued with the",
                                        "Fire, Earth, Wind, or Water",
                                        "property. Which property would",
                                        "you like your armor to have?"
                                    ],
                                )?;
                                ctx.next()?;
                                match runtime::select_values(
                                    ctx,
                                    &[Val::from("Fire Property:Earth Property:Wind Property:Water Property")],
                                )? {
                                    1 => {
                                        ei_felle_repay01_run(
                                            ctx,
                                            EiFelleRepay01Step::SBonusReward,
                                            vec![Val::from(1000), Val::from(2345)],
                                        )?;
                                        return Err(Stop::End);
                                    }
                                    2 => {
                                        ei_felle_repay01_run(
                                            ctx,
                                            EiFelleRepay01Step::SBonusReward,
                                            vec![Val::from(1000), Val::from(2351)],
                                        )?;
                                        return Err(Stop::End);
                                    }
                                    3 => {
                                        ei_felle_repay01_run(
                                            ctx,
                                            EiFelleRepay01Step::SBonusReward,
                                            vec![Val::from(1000), Val::from(2349)],
                                        )?;
                                        return Err(Stop::End);
                                    }
                                    4 => {
                                        ei_felle_repay01_run(
                                            ctx,
                                            EiFelleRepay01Step::SBonusReward,
                                            vec![Val::from(1000), Val::from(2347)],
                                        )?;
                                        return Err(Stop::End);
                                    }
                                    _ => {}
                                }
                            }
                            if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                                matched1 = true;
                            }
                            if matched1 {
                                ctx.lines_as(
                                    "Ei'felle",
                                    args![
                                        "Something else...?",
                                        "Oh, you must mean",
                                        "the Glittering Jacket that",
                                        "I developed earlier. After",
                                        "all, I have any other items",
                                        "to offer you for now..."
                                    ],
                                )?;
                                ctx.next()?;
                                ei_felle_repay01_run(ctx, EiFelleRepay01Step::SBonusReward, vec![Val::from(500), Val::from(2319)])?;
                                return Err(Stop::End);
                            }
                            if !matched1 && subject1.loosely_equals(&Val::from(3)) {
                                matched1 = true;
                            }
                            if matched1 {
                                ctx.lines_as(
                                    "Ei'felle",
                                    args![
                                        "Ah, I see. You'd rather wait",
                                        "until we develop something",
                                        "more to your liking. In that",
                                        "case, would you please donate",
                                        "more medals to my research?",
                                        "We're always low on them..."
                                    ],
                                )?;
                                ctx.next()?;
                                break 'b1;
                            }
                        }
                    }
                    if ctx.call(Function::CountItem, vec![Val::from("Marvelous_Medal")])?.is_true() {
                        ctx.lines_as(
                            "Ei'felle",
                            args![
                                "The other Blacksmith",
                                "Guildsmen are doing their",
                                "best to collect Prize Medals",
                                "in Hugel, but they keep failing",
                                "to win them! If you have any",
                                "medals, then may I have some?"
                            ],
                        )?;
                        ctx.next()?;
                        ei_felle_repay01_run(ctx, EiFelleRepay01Step::SReward, vec![])?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        "Ei'felle",
                        args![
                            "If you happen to obtain any",
                            "medals from the Monster",
                            "Race Arena in Hugel, then",
                            "please bring some of them",
                            "to me. I'll be sure to repay",
                            "you for your kindness..."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("ein_medal01").get()?.number()? > 1499 {
                    ctx.lines_as(
                        "Ei'felle",
                        args![
                            "Ah, you're back! I've",
                            "extracted all the metal",
                            "from the medals you've",
                            "given me, and I think I have",
                            "enough to create a Level 4",
                            "Weapon. Isn't that incredible?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ei'felle",
                        args![
                            "If you like, I can create",
                            "one of these weapons for you",
                            "as my way of repaying you for",
                            "your help. What do you think?"
                        ],
                    )?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("Yes, I want a Level 4 Weapon.:Can I have something else?")],
                    )?) == 2
                    {
                        ctx.lines_as(
                            "Ei'felle",
                            args![
                                "Something else?",
                                "Oh, alright then, would",
                                "you like to have a Glittering",
                                "Jacket, or a set of slotted",
                                "elemental Armor? Please go",
                                "ahead and make your choice~"
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(
                            ctx,
                            &[Val::from(
                                "Glittering Jacket:Fire Property Armor:Earth Property Armor:Wind Property Armor:Water Property Armor:Cancel",
                            )],
                        )? {
                            1 => {
                                ei_felle_repay01_run(ctx, EiFelleRepay01Step::SBonusReward, vec![Val::from(500), Val::from(2319)])?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ei_felle_repay01_run(ctx, EiFelleRepay01Step::SBonusReward, vec![Val::from(1000), Val::from(2345)])?;
                                return Err(Stop::End);
                            }
                            3 => {
                                ei_felle_repay01_run(ctx, EiFelleRepay01Step::SBonusReward, vec![Val::from(1000), Val::from(2351)])?;
                                return Err(Stop::End);
                            }
                            4 => {
                                ei_felle_repay01_run(ctx, EiFelleRepay01Step::SBonusReward, vec![Val::from(1000), Val::from(2349)])?;
                                return Err(Stop::End);
                            }
                            5 => {
                                ei_felle_repay01_run(ctx, EiFelleRepay01Step::SBonusReward, vec![Val::from(1000), Val::from(2347)])?;
                                return Err(Stop::End);
                            }
                            6 => {
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                    let base = Val::from(0).number()?;
                    runtime::local_set(&mut l_type_s, &Val::from(base + 0), Val::from("Dagger"), true);
                    runtime::local_set(&mut l_type_s, &Val::from(base + 1), Val::from("One Handed Sword"), true);
                    runtime::local_set(&mut l_type_s, &Val::from(base + 2), Val::from("Two Handed Sword"), true);
                    runtime::local_set(&mut l_type_s, &Val::from(base + 3), Val::from("Axe"), true);
                    runtime::local_set(&mut l_type_s, &Val::from(base + 4), Val::from("Mace"), true);
                    runtime::local_set(&mut l_type_s, &Val::from(base + 5), Val::from("Bow"), true);
                    runtime::local_set(&mut l_type_s, &Val::from(base + 6), Val::from("Staff"), true);
                    runtime::local_set(&mut l_type_s, &Val::from(base + 7), Val::from("Book"), true);
                    runtime::local_set(&mut l_type_s, &Val::from(base + 8), Val::from("Spear"), true);
                    runtime::local_set(&mut l_type_s, &Val::from(base + 9), Val::from("Katar"), true);
                    runtime::local_set(&mut l_type_s, &Val::from(base + 10), Val::from("Knuckle"), true);
                    runtime::local_set(&mut l_type_s, &Val::from(base + 11), Val::from("Whip"), true);
                    runtime::local_set(&mut l_type_s, &Val::from(base + 12), Val::from("Musical Instrument"), true);
                    l_m = (Val::from(runtime::select_values(ctx, &[runtime::implode(&l_type_s, &Val::from(":"))?])?)
                        .try_sub(Val::from(1))?);
                    ctx.lines_as(
                        "Ei'felle",
                        args![
                            "So you'd like to have a",
                            (runtime::local_get(&l_type_s, &l_m.clone(), true) + Val::from("? Please choose")),
                            "which Level 4 Weapon",
                            "that you want me to create."
                        ],
                    )?;
                    ctx.next()?;
                    'b4: {
                        let subject4 = l_m.clone();
                        let mut matched4 = false;
                        let no_case4 = !subject4.loosely_equals(&Val::from(0))
                            && !subject4.loosely_equals(&Val::from(1))
                            && !subject4.loosely_equals(&Val::from(2))
                            && !subject4.loosely_equals(&Val::from(3))
                            && !subject4.loosely_equals(&Val::from(4))
                            && !subject4.loosely_equals(&Val::from(5))
                            && !subject4.loosely_equals(&Val::from(6))
                            && !subject4.loosely_equals(&Val::from(7))
                            && !subject4.loosely_equals(&Val::from(8))
                            && !subject4.loosely_equals(&Val::from(9))
                            && !subject4.loosely_equals(&Val::from(10))
                            && !subject4.loosely_equals(&Val::from(11))
                            && !subject4.loosely_equals(&Val::from(12));
                        if !matched4 && subject4.loosely_equals(&Val::from(0)) {
                            matched4 = true;
                        }
                        if matched4 {
                            {
                                let assigned = Val::from(
                                    "Ginnungagap:Grimtooth:Dragon Killer:Mail Breaker:Bazerald:Sword Breaker:Ice Pick:Sucsamad:Kitchen Knife:Azoth:Exorciser:Assassin Dagger:Moonlight Dagger:Weeder Knife:Cursed Dagger:Dagger of Counter:Combat Knife:Fortune Sword",
                                );
                                runtime::local_set(&mut l_weapon_s, &Val::from(0), assigned, true);
                            }
                            let base = Val::from(0).number()?;
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 0), Val::from(13002), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 1), Val::from(1237), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 2), Val::from(13001), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 3), Val::from(1225), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 4), Val::from(1231), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 5), Val::from(1224), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 6), Val::from(1230), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 7), Val::from(1236), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 8), Val::from(1229), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 9), Val::from(1235), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 10), Val::from(1233), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 11), Val::from(1232), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 12), Val::from(1234), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 13), Val::from(1227), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 14), Val::from(1241), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 15), Val::from(1242), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 16), Val::from(1228), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 17), Val::from(1223), false);
                            break 'b4;
                        }
                        if !matched4 && subject4.loosely_equals(&Val::from(1)) {
                            matched4 = true;
                        }
                        if matched4 {
                            {
                                let assigned = Val::from(
                                    "Nagan:Immaterial Sword:Mysteltainn:Byeollungum:Star Dust Blade:Caesar's Sword:Ice Falchion:Excalibur:Edge:Cutlus:Solar Sword:Tirfing:Fireblend",
                                );
                                runtime::local_set(&mut l_weapon_s, &Val::from(0), assigned, true);
                            }
                            let base = Val::from(0).number()?;
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 0), Val::from(1130), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 1), Val::from(1141), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 2), Val::from(1138), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 3), Val::from(1140), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 4), Val::from(1148), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 5), Val::from(1134), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 6), Val::from(1131), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 7), Val::from(1137), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 8), Val::from(1132), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 9), Val::from(1135), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 10), Val::from(1136), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 11), Val::from(1139), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 12), Val::from(1133), false);
                            break 'b4;
                        }
                        if !matched4 && subject4.loosely_equals(&Val::from(2)) {
                            matched4 = true;
                        }
                        if matched4 {
                            {
                                let assigned =
                                    Val::from("Dragon Slayer:Masamune:Muramasa:Schweizersabel:Executioner:Zweihander:Katzbalger");
                                runtime::local_set(&mut l_weapon_s, &Val::from(0), assigned, true);
                            }
                            let base = Val::from(0).number()?;
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 0), Val::from(1166), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 1), Val::from(1165), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 2), Val::from(1164), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 3), Val::from(1167), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 4), Val::from(1169), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 5), Val::from(1168), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 6), Val::from(1170), false);
                            break 'b4;
                        }
                        if !matched4 && subject4.loosely_equals(&Val::from(3)) {
                            matched4 = true;
                        }
                        if matched4 {
                            {
                                let assigned =
                                    Val::from("Great Axe:Guillotine:Light Epsilon:Bloody Axe:Sabbath:Slaughter:Cleaver:Tomahawk");
                                runtime::local_set(&mut l_weapon_s, &Val::from(0), assigned, true);
                            }
                            let base = Val::from(0).number()?;
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 0), Val::from(1364), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 1), Val::from(1369), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 2), Val::from(1366), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 3), Val::from(1363), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 4), Val::from(1365), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 5), Val::from(1367), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 6), Val::from(1305), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 7), Val::from(1368), false);
                            break 'b4;
                        }
                        if !matched4 && subject4.loosely_equals(&Val::from(4)) {
                            matched4 = true;
                        }
                        if matched4 {
                            {
                                let assigned = Val::from("Golden Mace:Grand Cross:Long Mace:Spike:Slash:Quadrille");
                                runtime::local_set(&mut l_weapon_s, &Val::from(0), assigned, true);
                            }
                            let base = Val::from(0).number()?;
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 0), Val::from(1524), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 1), Val::from(1528), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 2), Val::from(1525), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 3), Val::from(1523), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 4), Val::from(1526), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 5), Val::from(1527), false);
                            break 'b4;
                        }
                        if !matched4 && subject4.loosely_equals(&Val::from(5)) {
                            matched4 = true;
                        }
                        if matched4 {
                            {
                                let assigned = Val::from("Roguemaster's Bow:Dragon Wing:Rudra's Bow:Ballista");
                                runtime::local_set(&mut l_weapon_s, &Val::from(0), assigned, true);
                            }
                            let base = Val::from(0).number()?;
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 0), Val::from(1719), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 1), Val::from(1724), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 2), Val::from(1720), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 3), Val::from(1722), false);
                            break 'b4;
                        }
                        if !matched4 && subject4.loosely_equals(&Val::from(6)) {
                            matched4 = true;
                        }
                        if matched4 {
                            {
                                let assigned = Val::from("Wing Staff:Wizardry Staff");
                                runtime::local_set(&mut l_weapon_s, &Val::from(0), assigned, true);
                            }
                            let base = Val::from(0).number()?;
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 0), Val::from(1616), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 1), Val::from(1473), false);
                        }
                        if !matched4 && subject4.loosely_equals(&Val::from(7)) {
                            matched4 = true;
                        }
                        if matched4 {
                            {
                                let assigned = Val::from("Legacy of Dragon:Book of the Apocalypse:Girl's Diary:Hardcover Book");
                                runtime::local_set(&mut l_weapon_s, &Val::from(0), assigned, true);
                            }
                            let base = Val::from(0).number()?;
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 0), Val::from(1559), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 1), Val::from(1557), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 2), Val::from(1558), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 3), Val::from(1561), false);
                            break 'b4;
                        }
                        if !matched4 && subject4.loosely_equals(&Val::from(8)) {
                            matched4 = true;
                        }
                        if matched4 {
                            {
                                let assigned = Val::from(
                                    "Gae Bolg:Gelerdria:Gungnir:Skewer:Longinus's Spear:Brionac:Bill Guisarme:Zephyrus:Crescent Scythe:Tjungkuletti:Hellfire",
                                );
                                runtime::local_set(&mut l_weapon_s, &Val::from(0), assigned, true);
                            }
                            let base = Val::from(0).number()?;
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 0), Val::from(1474), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 1), Val::from(1414), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 2), Val::from(1413), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 3), Val::from(1415), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 4), Val::from(1469), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 5), Val::from(1470), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 6), Val::from(1467), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 7), Val::from(1468), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 8), Val::from(1466), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 9), Val::from(1416), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 10), Val::from(1471), false);
                            break 'b4;
                        }
                        if !matched4 && subject4.loosely_equals(&Val::from(9)) {
                            matched4 = true;
                        }
                        if matched4 {
                            {
                                let assigned = Val::from("Infiltrator:Bloody Roar:Unholy Touch");
                                runtime::local_set(&mut l_weapon_s, &Val::from(0), assigned, true);
                            }
                            let base = Val::from(0).number()?;
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 0), Val::from(1261), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 1), Val::from(1265), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 2), Val::from(1263), false);
                            break 'b4;
                        }
                        if !matched4 && subject4.loosely_equals(&Val::from(10)) {
                            matched4 = true;
                        }
                        if matched4 {
                            {
                                let assigned = Val::from("Hatii Claw:Berserk:Kaiser Knuckle");
                                runtime::local_set(&mut l_weapon_s, &Val::from(0), assigned, true);
                            }
                            let base = Val::from(0).number()?;
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 0), Val::from(1815), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 1), Val::from(1814), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 2), Val::from(1813), false);
                            break 'b4;
                        }
                        if !matched4 && subject4.loosely_equals(&Val::from(11)) {
                            matched4 = true;
                        }
                        if matched4 {
                            {
                                let assigned = Val::from("Lariat:Rapture Rose:Blade Whip:Chemeti:Queen's Whip");
                                runtime::local_set(&mut l_weapon_s, &Val::from(0), assigned, true);
                            }
                            let base = Val::from(0).number()?;
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 0), Val::from(1962), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 1), Val::from(1963), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 2), Val::from(1969), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 3), Val::from(1964), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 4), Val::from(1970), false);
                            break 'b4;
                        }
                        if !matched4 && subject4.loosely_equals(&Val::from(12)) {
                            matched4 = true;
                        }
                        if matched4 {
                            {
                                let assigned = Val::from("Oriental Lute:Electric Guitar");
                                runtime::local_set(&mut l_weapon_s, &Val::from(0), assigned, true);
                            }
                            let base = Val::from(0).number()?;
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 0), Val::from(1918), false);
                            runtime::local_set(&mut l_weapon_id, &Val::from(base + 1), Val::from(1913), false);
                            break 'b4;
                        }
                    }
                    l_j = (Val::from(runtime::select_values(
                        ctx,
                        &[(runtime::local_get(&l_weapon_s, &Val::from(0), true) + Val::from(":Cancel"))],
                    )?)
                    .try_sub(Val::from(1))?);
                    l_weapon_s = runtime::explode(&runtime::local_get(&l_weapon_s, &Val::from(0), true), &Val::from(":"));
                    if l_j.clone().loosely_equals(&Val::from(l_weapon_s.len() as i32)) {
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ei_felle_repay01_run(
                        ctx,
                        EiFelleRepay01Step::SBonusReward,
                        vec![Val::from(1500), runtime::local_get(&l_weapon_id, &l_j.clone(), false)],
                    )?;
                    return Err(Stop::End);
                }
                step = EiFelleRepay01Step::SReward;
                continue 'machine;
            }
            EiFelleRepay01Step::SReward => {
                if Val::from(runtime::select_values(ctx, &[Val::from("Sure:No")])?) == 2 {
                    ctx.lines_as(
                        "Ei'felle",
                        args![
                            "Oh, alright...",
                            "Still, I really need",
                            "those medals, so if you",
                            "change your mind, please",
                            "come back as soon as you can."
                        ],
                    )?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Ei'felle",
                    args![
                        "Oh, thank you so much!",
                        "I can use the metal in those",
                        "medals to produce some high",
                        "quality products. Now, how",
                        "shall I repay you for giving me",
                        "some of your Prize Medals?"
                    ],
                )?;
                ctx.next()?;
                'b5: {
                    let subject5 = Val::from(runtime::select_values(
                        ctx,
                        &[Val::from(
                            "Base Experience:Job Experience:No, I'm just glad to help.:How's your research progressing?",
                        )],
                    )?);
                    let mut matched5 = false;
                    let no_case5 = !subject5.loosely_equals(&Val::from(1))
                        && !subject5.loosely_equals(&Val::from(2))
                        && !subject5.loosely_equals(&Val::from(3))
                        && !subject5.loosely_equals(&Val::from(4));
                    if !matched5 && subject5.loosely_equals(&Val::from(1)) {
                        matched5 = true;
                    }
                    if matched5 {
                        ctx.lines_as(
                            "Ei'felle",
                            args![
                                "You just want to improve",
                                "yourself, huh? Well, I dunno",
                                "if you want to get stronger,",
                                "smarter, or faster, but I can",
                                "coach you on some visualization if you like. Now, relax with me~",
                                "physical development."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ei'felle",
                            args![
                                "Focus... and believe.",
                                "Believe that you are",
                                "becoming what you want",
                                "to be! B-believe... with",
                                "all of your freakin' heart!"
                            ],
                        )?;
                        ctx.call(
                            Function::Emotion,
                            vec![
                                ctx.constant("ET_HUK")?,
                                Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                            ],
                        )?;
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
                        ctx.next()?;
                        ctx.lines_as("Ei'felle", args!["*Phew* How's that?", "Now do you see the", "power of hope?"])?;
                        ctx.call(Function::DelItem, vec![Val::from(7515), Val::from(1)])?;
                        {
                            if ctx.var("BaseLevel").get()?.number()? < 21 {
                                ctx.call(Function::GetExperience, vec![Val::from(150), Val::from(0)])?;
                            } else {
                                if (ctx.var("BaseLevel").get()?.number()? > 20 && ctx.var("BaseLevel").get()?.number()? < 31) {
                                    ctx.call(Function::GetExperience, vec![Val::from(300), Val::from(0)])?;
                                } else {
                                    if (ctx.var("BaseLevel").get()?.number()? > 30 && ctx.var("BaseLevel").get()?.number()? < 41) {
                                        ctx.call(Function::GetExperience, vec![Val::from(2000), Val::from(0)])?;
                                    } else if (ctx.var("BaseLevel").get()?.number()? > 40 && ctx.var("BaseLevel").get()?.number()? < 51) {
                                        ctx.call(Function::GetExperience, vec![Val::from(8000), Val::from(0)])?;
                                    } else if (ctx.var("BaseLevel").get()?.number()? > 50 && ctx.var("BaseLevel").get()?.number()? < 61) {
                                        ctx.call(Function::GetExperience, vec![Val::from(25000), Val::from(0)])?;
                                    } else if (ctx.var("BaseLevel").get()?.number()? > 60 && ctx.var("BaseLevel").get()?.number()? < 71) {
                                        ctx.call(Function::GetExperience, vec![Val::from(47000), Val::from(0)])?;
                                    } else if (ctx.var("BaseLevel").get()?.number()? > 70 && ctx.var("BaseLevel").get()?.number()? < 81) {
                                        ctx.call(Function::GetExperience, vec![Val::from(55000), Val::from(0)])?;
                                    } else {
                                        ctx.call(Function::GetExperience, vec![Val::from(65000), Val::from(0)])?;
                                    }
                                }
                            }
                        }
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    if !matched5 && subject5.loosely_equals(&Val::from(2)) {
                        matched5 = true;
                    }
                    if matched5 {
                        ctx.lines_as(
                            "Ei'felle",
                            args![
                                "You want to become more",
                                "competent in your job? Um...",
                                "Alright, we can do that. Just",
                                "meditate with me, and we'll go",
                                "do some imagery work together.",
                                "I'm real good at this you know."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ei'felle",
                            args![
                                "J-just... visualize",
                                "yourself... being...",
                                "t-totally... awesome!",
                                "You've gotta see it, and",
                                "you've gotta feel it in your",
                                "freakin' heart and mind!"
                            ],
                        )?;
                        ctx.call(
                            Function::Emotion,
                            vec![
                                ctx.constant("ET_HUK")?,
                                Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                            ],
                        )?;
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ei'felle",
                            args![
                                "Yeap, it doesn't matter",
                                "if it's real or imagined...",
                                "Your mind will work on",
                                "whatever you feed it.",
                                "Placebos and psychosomatic symptoms-- it all ties together."
                            ],
                        )?;
                        ctx.call(Function::DelItem, vec![Val::from(7515), Val::from(1)])?;
                        {
                            if ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_NOVICE")?) {
                                ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(30)])?;
                            } else {
                                if ((runtime::op(&ctx.var("Class").get()?, ">=", &ctx.constant("JOB_SWORDMAN")?)?.is_true()
                                    && runtime::op(&ctx.var("Class").get()?, "<=", &ctx.constant("JOB_THIEF")?)?.is_true())
                                    || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_TAEKWON")?))
                                {
                                    if ctx.var("JobLevel").get()?.number()? < 11 {
                                        ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(50)])?;
                                    } else if (ctx.var("JobLevel").get()?.number()? > 10 && ctx.var("JobLevel").get()?.number()? < 21) {
                                        ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(250)])?;
                                    } else if (ctx.var("JobLevel").get()?.number()? > 20 && ctx.var("JobLevel").get()?.number()? < 31) {
                                        ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(1500)])?;
                                    } else if (ctx.var("JobLevel").get()?.number()? > 30 && ctx.var("JobLevel").get()?.number()? < 41) {
                                        ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(7000)])?;
                                    } else {
                                        ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(20000)])?;
                                    }
                                } else {
                                    if (runtime::op(&ctx.var("Class").get()?, ">=", &ctx.constant("JOB_KNIGHT")?)?.is_true()
                                        && runtime::op(&ctx.var("Class").get()?, "<=", &ctx.constant("JOB_CRUSADER2")?)?.is_true())
                                    {
                                        if ctx.var("JobLevel").get()?.number()? < 11 {
                                            ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(80)])?;
                                        } else if (ctx.var("JobLevel").get()?.number()? > 10 && ctx.var("JobLevel").get()?.number()? < 21) {
                                            ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(2000)])?;
                                        } else if (ctx.var("JobLevel").get()?.number()? > 20 && ctx.var("JobLevel").get()?.number()? < 31) {
                                            ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(10000)])?;
                                        } else if (ctx.var("JobLevel").get()?.number()? > 30 && ctx.var("JobLevel").get()?.number()? < 41) {
                                            ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(25000)])?;
                                        } else {
                                            ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(38000)])?;
                                        }
                                    } else {
                                        if ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_NOVICE_HIGH")?) {
                                            ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(40)])?;
                                        } else {
                                            if (runtime::op(&ctx.var("Class").get()?, ">=", &ctx.constant("JOB_SWORDMAN_HIGH")?)?.is_true()
                                                && runtime::op(&ctx.var("Class").get()?, "<=", &ctx.constant("JOB_THIEF_HIGH")?)?.is_true())
                                            {
                                                if ctx.var("JobLevel").get()?.number()? < 11 {
                                                    ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(65)])?;
                                                } else if (ctx.var("JobLevel").get()?.number()? > 10
                                                    && ctx.var("JobLevel").get()?.number()? < 21)
                                                {
                                                    ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(300)])?;
                                                } else if (ctx.var("JobLevel").get()?.number()? > 20
                                                    && ctx.var("JobLevel").get()?.number()? < 31)
                                                {
                                                    ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(2500)])?;
                                                } else if (ctx.var("JobLevel").get()?.number()? > 30
                                                    && ctx.var("JobLevel").get()?.number()? < 41)
                                                {
                                                    ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(10000)])?;
                                                } else {
                                                    ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(25000)])?;
                                                }
                                            } else {
                                                if (runtime::op(&ctx.var("Class").get()?, ">=", &ctx.constant("JOB_LORD_KNIGHT")?)?
                                                    .is_true()
                                                    && runtime::op(&ctx.var("Class").get()?, "<=", &ctx.var("job_paladin_2").get()?)?
                                                        .is_true())
                                                {
                                                    if ctx.var("JobLevel").get()?.number()? < 11 {
                                                        ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(150)])?;
                                                    } else if (ctx.var("JobLevel").get()?.number()? > 10
                                                        && ctx.var("JobLevel").get()?.number()? < 21)
                                                    {
                                                        ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(2200)])?;
                                                    } else if (ctx.var("JobLevel").get()?.number()? > 20
                                                        && ctx.var("JobLevel").get()?.number()? < 31)
                                                    {
                                                        ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(13000)])?;
                                                    } else if (ctx.var("JobLevel").get()?.number()? > 30
                                                        && ctx.var("JobLevel").get()?.number()? < 41)
                                                    {
                                                        ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(27000)])?;
                                                    } else if (ctx.var("JobLevel").get()?.number()? > 40
                                                        && ctx.var("JobLevel").get()?.number()? < 51)
                                                    {
                                                        ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(38000)])?;
                                                    } else {
                                                        ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(40000)])?;
                                                    }
                                                } else {
                                                    if ctx.var("JobLevel").get()?.number()? < 11 {
                                                        ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(50)])?;
                                                    } else {
                                                        if (ctx.var("JobLevel").get()?.number()? > 10
                                                            && ctx.var("JobLevel").get()?.number()? < 21)
                                                        {
                                                            ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(250)])?;
                                                        } else if (ctx.var("JobLevel").get()?.number()? > 20
                                                            && ctx.var("JobLevel").get()?.number()? < 31)
                                                        {
                                                            ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(1500)])?;
                                                        } else if (ctx.var("JobLevel").get()?.number()? > 30
                                                            && ctx.var("JobLevel").get()?.number()? < 41)
                                                        {
                                                            ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(7000)])?;
                                                        } else if (ctx.var("JobLevel").get()?.number()? > 40
                                                            && ctx.var("JobLevel").get()?.number()? < 51)
                                                        {
                                                            ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(20000)])?;
                                                        } else if (ctx.var("JobLevel").get()?.number()? > 50
                                                            && ctx.var("JobLevel").get()?.number()? < 61)
                                                        {
                                                            ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(30000)])?;
                                                        } else {
                                                            ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(38000)])?;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    if !matched5 && subject5.loosely_equals(&Val::from(3)) {
                        matched5 = true;
                    }
                    if matched5 {
                        ctx.lines_as(
                            "Ei'felle",
                            args![
                                "What th--?! You sure",
                                "you don't want anything?",
                                "Hm, well, I think it's kind",
                                "of bad karma if I don't give",
                                "you anything in return, so...",
                                "Think of something. Quick."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![
                                "Er, just use the medals",
                                "to further your manufacturing",
                                "research, and then you can",
                                "pay me back if your develop",
                                "something new. It's, um, like",
                                "an investment in your work!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ei'felle",
                            args![
                                "Yeah... Yeah.",
                                "Okay, I see where",
                                "you're coming from.",
                                "That's pretty smart.",
                                "Okay, I'll repay you when",
                                "we develop something new!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ei'felle",
                            args![
                                "Anyway, I need as many",
                                "medals as I can get as",
                                "soon as I can. How many",
                                "medals do you think you",
                                "can give me right now?"
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Take them all.:How about this much?:No, I changed my mind.")])? {
                            1 => {
                                ctx.lines_as(
                                    "Ei'felle",
                                    args![
                                        "Th-thank you!",
                                        "Thank you so much!",
                                        "Your help will greatly",
                                        "advance my research,",
                                        "and I promise to repay",
                                        "you as soon as I can!"
                                    ],
                                )?;
                                l_medals = ctx.call(Function::CountItem, vec![Val::from("Marvelous_Medal")])?;
                                ctx.call(Function::DelItem, vec![Val::from(7515), l_medals.clone()])?;
                                ctx.var("ein_medal01").set((ctx.var("ein_medal01").get()? + l_medals.clone()))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    "Ei'felle",
                                    args![
                                        "Alright, please enter the",
                                        "number of medals that",
                                        "you're willing to give me.",
                                        "Please don't enter any",
                                        "number greater than 100."
                                    ],
                                )?;
                                ctx.next()?;
                                'l7: loop {
                                    if !(true) {
                                        break 'l7;
                                    }
                                    'b7: {
                                        let (input, status) = runtime::input_number(ctx, None, None)?;
                                        l_input = input;
                                        if !(l_input.clone().is_true()) {
                                            ctx.lines_as(
                                                "Ei'felle",
                                                args![
                                                    "Aw, so you've decided",
                                                    "to cancel? Well, it's your",
                                                    "choice, but I'm still so",
                                                    "disappointed. Please come",
                                                    "back if you change your mind..."
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        } else if (l_input.clone().number()? < 1 || l_input.clone().number()? > 100) {
                                            ctx.lines_as(
                                                "Ei'felle",
                                                args![
                                                    "Remember, you can only",
                                                    "enter a number from 1 to 100.",
                                                    "If you want to give me more",
                                                    "medals, then perhaps you",
                                                    "should just give them all to me~"
                                                ],
                                            )?;
                                            ctx.next()?;
                                        } else {
                                            break 'l7;
                                        }
                                    }
                                }
                                if runtime::op(
                                    &ctx.call(Function::CountItem, vec![Val::from("Marvelous_Medal")])?,
                                    "<",
                                    &l_input.clone(),
                                )?
                                .is_true()
                                {
                                    ctx.lines_as(
                                        "Ei'felle",
                                        args![
                                            "I'm sorry, but I don't",
                                            "think you have that many",
                                            "medals with you. Make sure",
                                            "that you offer me an amount of",
                                            "medals that you actually have."
                                        ],
                                    )?;
                                    ctx.call(Function::Emotion, vec![ctx.constant("ET_SMILE")?])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    ctx.lines_as(
                                        "Ei'felle",
                                        args![
                                            "Th-thank you!",
                                            "Thank you so much!",
                                            "Your help will greatly",
                                            "advance my research,",
                                            "and I promise to repay",
                                            "you as soon as I can!"
                                        ],
                                    )?;
                                    ctx.call(Function::DelItem, vec![Val::from(7515), l_input.clone()])?;
                                    ctx.var("ein_medal01").set((ctx.var("ein_medal01").get()? + l_input.clone()))?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                            }
                            3 => {
                                ctx.lines_as(
                                    "Ei'felle",
                                    args![
                                        "Oh, alright...",
                                        "Still, I really need",
                                        "those medals, so if you",
                                        "change your mind, please",
                                        "come back as soon as you can."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                    if !matched5 && subject5.loosely_equals(&Val::from(4)) {
                        matched5 = true;
                    }
                    if matched5 {
                        ctx.lines_as(
                            "Ei'felle",
                            args![
                                "Well, I've been able to",
                                "create a Glittering Jacket",
                                "using the medals that you've",
                                "brought to me. But I just know",
                                "I can make something better",
                                "if you'd bring me more!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ei'felle",
                            args![
                                "As of now, I have the metal",
                                ((Val::from("from ^FF0000") + ctx.var("ein_medal01").get()?) + Val::from("^000000 Prizes Medals that")),
                                "you've donated to me. The more",
                                "that you bring, the closer I can get to making a new breakthrough!"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                step = EiFelleRepay01Step::SBonusReward;
                continue 'machine;
            }
            EiFelleRepay01Step::SBonusReward => {
                ctx.mes("[Ei'felle]")?;
                l_arg1 = runtime::arg(&args, 1, Val::from(0));
                if l_arg1.clone() == 2319 {
                    ctx.lines(args![
                        "Do you really want",
                        ((Val::from("this ") + ctx.call(Function::GetItemName, vec![l_arg1.clone()])?) + Val::from("?")),
                        "You may want to forego this",
                        "reward in favor of getting",
                        "something better later..."
                    ])?;
                } else if (((l_arg1.clone() == 2345 || l_arg1.clone() == 2347) || l_arg1.clone() == 2349) || l_arg1.clone() == 2351) {
                    ctx.lines(args!["Are you sure that you want", "to accept this set of slotted"])?;
                    if l_arg1.clone() == 2345 {
                        ctx.mes("Fire property Armor? If you do,")?;
                    } else if l_arg1.clone() == 2351 {
                        ctx.mes("Earth property Armor? If you do,")?;
                    } else if l_arg1.clone() == 2349 {
                        ctx.mes("Wind property Armor? If you do,")?;
                    } else if l_arg1.clone() == 2347 {
                        ctx.mes("Water property Armor? If you do,")?;
                    }
                    ctx.lines(args![
                        "I'll need more medals from you",
                        "to make further advancements",
                        "in my manufacturing research."
                    ])?;
                } else {
                    ctx.lines(args![
                        "So you wish to have a",
                        (ctx.call(Function::GetItemName, vec![l_arg1.clone()])? + Val::from("? If you choose")),
                        "to have this Level 4 Weapon,",
                        "I'll need to melt many of the",
                        "medals that you've donated",
                        "to me. Shall we proceed?"
                    ])?;
                }
                ctx.next()?;
                if Val::from(runtime::select_values(ctx, &[Val::from("Decline:Accept")])?) == 1 {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "W-wait, I changed my",
                            "mind. Would it be fine",
                            "if I asked you to give",
                            "me a reward later?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.mes("[Ei'felle]")?;
                    if ctx
                        .call(Function::GetItemInfo, vec![l_arg1.clone(), ctx.constant("ITEMINFO_TYPE")?])?
                        .loosely_equals(&ctx.constant("IT_ARMOR")?)
                    {
                        ctx.lines(args![
                            "Of course, of course.",
                            "Remember, if you donate",
                            "more medals to me, then",
                            "I'll be able to create items",
                            "of higher quality for you~"
                        ])?;
                    } else {
                        ctx.lines(args![
                            "I don't think I can develop",
                            "anything better than these Level 4 Weapons, but after making",
                            "such a big investment, you should decide on what you want carefully."
                        ])?;
                    }
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.mes("[Ei'felle]")?;
                if l_arg1.clone() == 2319 {
                    ctx.lines(args![
                        "Here you are, I trust that",
                        ((Val::from("this ") + ctx.call(Function::GetItemName, vec![l_arg1.clone()])?) + Val::from(" will")),
                        "serve you well. Thank you",
                        "for your help, and I hope that",
                        "you'll continue to donate your",
                        "medals for my metal research~"
                    ])?;
                } else if (((l_arg1.clone() == 2345 || l_arg1.clone() == 2347) || l_arg1.clone() == 2349) || l_arg1.clone() == 2351) {
                    ctx.mes("Great choice! I'm sure")?;
                    if l_arg1.clone() == 2345 {
                        ctx.mes("that this set of slotted Fire")?;
                    } else if l_arg1.clone() == 2351 {
                        ctx.mes("that this set of slotted Earth")?;
                    } else if l_arg1.clone() == 2349 {
                        ctx.mes("that this set of slotted Wind")?;
                    } else if l_arg1.clone() == 2347 {
                        ctx.mes("that this set of slotted Water")?;
                    }
                    ctx.lines(args![
                        "property Armor will serve you",
                        "well. Thank you for your help,",
                        "and if you get more medals,",
                        "please donate them to me~"
                    ])?;
                } else {
                    ctx.lines(args![
                        "Once again, I'd like to",
                        "thank you for providing",
                        "me with all of those medals.",
                        "I imagine it must have been",
                        "difficult. In any case, I would",
                        "appreciate your continued help~"
                    ])?;
                }
                ctx.var("ein_medal01")
                    .set((ctx.var("ein_medal01").get()?.try_sub(runtime::arg(&args, 0, Val::from(0)))?))?;
                ctx.call(Function::GetItem, vec![l_arg1.clone(), Val::from(1)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn ei_felle_repay01(ctx: &Ctx) -> Script {
    ei_felle_repay01_run(ctx, EiFelleRepay01Step::Start, Vec::new()).map(|_| ())
}

fn wayne_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    let mut l_item_array: Vec<Val> = Vec::new();
    let mut l_items_s = Val::from("");
    let mut l_m = Val::from(0);
    let mut l_m2 = Val::from(0);
    let mut l_menu_s = Val::from("");
    let mut l_pm: Vec<Val> = Vec::new();
    let mut l_total_pm = Val::from(0);
    if !(ctx.call(Function::CheckWeight, vec![Val::from("Knife"), Val::from(1)])?.is_true()) {
        ctx.lines_as(
            "Wayne",
            args![
                "Hold on a second...",
                "If you want to exchange",
                "your Prize Medals for items,",
                "you'd better free up more space",
                "in your Inventory first. Why don't you use your Kafra Storage?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Wayne",
        args![
            "Hello, there! Ever wonder",
            "what you could do with all",
            "the Prize Medals you can win",
            "in Monster Race Arena? You",
            "can donate them in Einbroch",
            "to the Blacksmith Guild..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Wayne",
        args![
            "...Or you can exchange them",
            "for items, right here and right",
            "now, with me. As always, the",
            "choice is really up to you."
        ],
    )?;
    ctx.next()?;
    if Val::from(runtime::select_values(ctx, &[Val::from("Prize Medal Exchange:Cancel")])?) == 2 {
        ctx.lines_as(
            "Wayne",
            args![
                "Well, just keep in mind",
                "that you can always come",
                "to me to trade in your Prize",
                "Medals for consumable items.",
                "That guy in Einbroch? Not sure",
                "what he'd give you for them..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Wayne",
            args![
                "All I know is that he",
                "needs a whole lot of medals",
                "for the work that he's doing.",
                "Still, I hear that he just may",
                "make your donations worth",
                "all your effort, you know?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if !(ctx
        .call(Function::CheckWeight, vec![Val::from("Jellopy"), Val::from(550)])?
        .is_true())
    {
        ctx.lines_as(
            "Wayne",
            args![
                "Hold on a second...",
                "If you want to exchange",
                "your Prize Medals for items,",
                "you'd better free up more space",
                "in your Inventory first. Why don't you use your Kafra Storage?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Wayne",
            args![
                "Now, how many Prize Medals",
                "would you like to exchange?",
                "It doesn't take a genius to",
                "figure out that you can get",
                "more valuable items by trading",
                "more Prize Medals at a time."
            ],
        )?;
        ctx.next()?;
        let base = Val::from(0).number()?;
        runtime::local_set(&mut l_pm, &Val::from(base + 0), Val::from(1), false);
        runtime::local_set(&mut l_pm, &Val::from(base + 1), Val::from(3), false);
        runtime::local_set(&mut l_pm, &Val::from(base + 2), Val::from(7), false);
        runtime::local_set(&mut l_pm, &Val::from(base + 3), Val::from(8), false);
        runtime::local_set(&mut l_pm, &Val::from(base + 4), Val::from(16), false);
        runtime::local_set(&mut l_pm, &Val::from(base + 5), Val::from(25), false);
        runtime::local_set(&mut l_pm, &Val::from(base + 6), Val::from(42), false);
        runtime::local_set(&mut l_pm, &Val::from(base + 7), Val::from(59), false);
        l_total_pm = Val::from(l_pm.len() as i32);
        l_i = Val::from(0);
        'l1: loop {
            if !(runtime::op(&l_i.clone(), "<", &l_total_pm.clone())?.is_true()) {
                break 'l1;
            }
            'b1: {
                l_menu_s = ((l_menu_s.clone() + runtime::local_get(&l_pm, &l_i.clone(), false)) + Val::from(" Prize medal:"));
            }
            l_i = (l_i.clone() + Val::from(1));
        }
        l_m = (Val::from(runtime::select_values(ctx, &[l_menu_s.clone()])?).try_sub(Val::from(1))?);
        let subject2 = l_m.clone();
        if subject2 == 0 {
            l_items_s = Val::from("2 Hinale Leaflets:2 Aloe Leaflets:1 Mastela Fruit:5 Witch Starsands:4 Condensed Red Potions");
            let base = Val::from(0).number()?;
            runtime::local_set(&mut l_item_array, &Val::from(base + 0), Val::from(520), false);
            runtime::local_set(&mut l_item_array, &Val::from(base + 1), Val::from(2), false);
            runtime::local_set(&mut l_item_array, &Val::from(base + 2), Val::from(521), false);
            runtime::local_set(&mut l_item_array, &Val::from(base + 3), Val::from(2), false);
            runtime::local_set(&mut l_item_array, &Val::from(base + 4), Val::from(522), false);
            runtime::local_set(&mut l_item_array, &Val::from(base + 5), Val::from(1), false);
            runtime::local_set(&mut l_item_array, &Val::from(base + 6), Val::from(1061), false);
            runtime::local_set(&mut l_item_array, &Val::from(base + 7), Val::from(5), false);
            runtime::local_set(&mut l_item_array, &Val::from(base + 8), Val::from(545), false);
            runtime::local_set(&mut l_item_array, &Val::from(base + 9), Val::from(4), false);
        } else if subject2 == 1 {
            l_items_s = Val::from("1 Royal Jelly:6 Holy Waters");
            let base = Val::from(0).number()?;
            runtime::local_set(&mut l_item_array, &Val::from(base + 0), Val::from(526), false);
            runtime::local_set(&mut l_item_array, &Val::from(base + 1), Val::from(1), false);
            runtime::local_set(&mut l_item_array, &Val::from(base + 2), Val::from(523), false);
            runtime::local_set(&mut l_item_array, &Val::from(base + 3), Val::from(6), false);
        } else if subject2 == 2 {
            l_items_s = Val::from("1 Cookie Bag:1 First Aid Kit");
            let base = Val::from(0).number()?;
            runtime::local_set(&mut l_item_array, &Val::from(base + 0), Val::from(12130), false);
            runtime::local_set(&mut l_item_array, &Val::from(base + 1), Val::from(1), false);
            runtime::local_set(&mut l_item_array, &Val::from(base + 2), Val::from(12110), false);
            runtime::local_set(&mut l_item_array, &Val::from(base + 3), Val::from(1), false);
        } else if subject2 == 3 {
            l_items_s = Val::from("1 Gift Box");
            let base = Val::from(0).number()?;
            runtime::local_set(&mut l_item_array, &Val::from(base + 0), Val::from(644), false);
            runtime::local_set(&mut l_item_array, &Val::from(base + 1), Val::from(1), false);
        } else if subject2 == 4 {
            l_items_s = Val::from("1 Old Blue Box");
            let base = Val::from(0).number()?;
            runtime::local_set(&mut l_item_array, &Val::from(base + 0), Val::from(603), false);
            runtime::local_set(&mut l_item_array, &Val::from(base + 1), Val::from(1), false);
        } else if subject2 == 5 {
            l_items_s = Val::from("1 Taming Gift Set");
            let base = Val::from(0).number()?;
            runtime::local_set(&mut l_item_array, &Val::from(base + 0), Val::from(12105), false);
            runtime::local_set(&mut l_item_array, &Val::from(base + 1), Val::from(1), false);
        } else if subject2 == 6 {
            l_items_s = Val::from("1 Old Purple Box");
            let base = Val::from(0).number()?;
            runtime::local_set(&mut l_item_array, &Val::from(base + 0), Val::from(617), false);
            runtime::local_set(&mut l_item_array, &Val::from(base + 1), Val::from(1), false);
        } else if subject2 == 7 {
            l_items_s = Val::from("1 Poring Box");
            let base = Val::from(0).number()?;
            runtime::local_set(&mut l_item_array, &Val::from(base + 0), Val::from(12109), false);
            runtime::local_set(&mut l_item_array, &Val::from(base + 1), Val::from(1), false);
        }
        ctx.lines_as(
            "Wayne",
            args![
                "Now, please choose",
                "which of the following item",
                "sets that you'd like to receive",
                (((Val::from("in exchange for ") + runtime::local_get(&l_pm, &l_m.clone(), false)) + Val::from(" Prize Medal."))
                    + (if Val::from(l_item_array.len() as i32).number()? < 3 {
                        Val::from(" Well, we have only 1 set, but...")
                    } else {
                        Val::from("")
                    }))
            ],
        )?;
        ctx.next()?;
        l_m2 = ((Val::from(runtime::select_values(ctx, &[l_items_s.clone()])?).try_sub(Val::from(1))?).try_mul(Val::from(2))?);
        if runtime::op(
            &ctx.call(Function::CountItem, vec![Val::from("Marvelous_Medal")])?,
            "<",
            &runtime::local_get(&l_pm, &l_m.clone(), false),
        )?
        .is_true()
        {
            ctx.lines_as(
                "Wayne",
                args![
                    "Hey, you don't have",
                    "enough Prize Medals with",
                    "you. Go and get some more",
                    "if you want to exchange",
                    "them with me for anything."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Wayne",
            args![
                "There you go~",
                "Thanks for using my",
                "service, and I hope that",
                "you come visit me again",
                "soon. Enjoy the monster",
                "races, fair adventurer~"
            ],
        )?;
        ctx.call(
            Function::DelItem,
            vec![Val::from(7515), runtime::local_get(&l_pm, &l_m.clone(), false)],
        )?;
        ctx.call(
            Function::GetItem,
            vec![
                runtime::local_get(&l_item_array, &l_m2.clone(), false),
                runtime::local_get(&l_item_array, &(l_m2.clone() + Val::from(1)), false),
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn wayne(ctx: &Ctx) -> Script {
    wayne_body(ctx, Vec::new()).map(|_| ())
}

fn eocatt_decoy01_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Eocatt",
        args![
            "There's an old, humble",
            "village on the outskirts of",
            "the Schwarzwald Republic.",
            "It was just a tiny blip on the",
            "map until they opened up",
            "their Monster Race Arena!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Eocatt",
        args![
            "If you win wagers on the",
            "monster race games, you'll",
            "be rewarded with these Prize",
            "Medals that are made of some",
            "really rare metal. I hear this metal's in demand in Einbroch..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Eocatt",
        args![
            "Right, right...",
            "I remember now, the",
            "town was named Hugel.",
            "I'm sure there's other fun",
            "things to do there, but I'm sure that the Race Arena is a must!"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn eocatt_decoy01(ctx: &Ctx) -> Script {
    eocatt_decoy01_body(ctx, Vec::new()).map(|_| ())
}

fn mudie_dummy01_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Mudie",
        args![
            "The Monster Races",
            "are probably the biggest",
            "attraction here in Hugel.",
            "We don't have much else",
            "going on here, I'm afraid."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Mudie",
        args![
            "If you want to go visit",
            "the Monster Race Arena,",
            "just head towards the",
            "7 'o clock direction on",
            "your Mini-Map, and look for",
            "the hill surrounded by a fence."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Mudie",
        args![
            "You should find the arena",
            "somewhere around that area.",
            "Anyway, if you want to wager",
            "or just watch the races, just",
            "ask one of the Eckar brothers. I hope you enjoy our little town~"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn mudie_dummy01(ctx: &Ctx) -> Script {
    mudie_dummy01_body(ctx, Vec::new()).map(|_| ())
}

fn eccentric_scholar_double_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Eccentric Scholar",
        args![
            "Let's see now...",
            "Monster 1's average speed",
            "and luck, as affected by",
            "wind resistance, fatigue...",
            "What's the approximate",
            "probability of winning...?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Eccentric Scholar",
        args![
            "Crunch it into my",
            "algorithm... Carry the two...",
            "Wait, how many significant",
            "figures should I be using?",
            "Ah, right, 7, to account for x,",
            "a value representing--"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
        args!["Excuse me, but", "what are you doing?"],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Eccentric Scholar",
        args!["S-silence!", "I must complete", "my calculations!", "Now, where was I...?"],
    )?;
    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_CLAYMORE")?])?;
    ctx.call(Function::Emotion, vec![ctx.constant("ET_FRET")?])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn eccentric_scholar_double(ctx: &Ctx) -> Script {
    eccentric_scholar_double_body(ctx, Vec::new()).map(|_| ())
}

fn blacksmith_guildsman_dou_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if !(ctx.var("$@mon_time_2_2").get()?.is_true()) {
        ctx.lines_as(
            "Blacksmith Guildsman",
            args![
                "How many times must",
                "I wager on these races?!",
                "I haven't won even once!",
                "Oh, I must have the worst",
                "luck in wagering history!"
            ],
        )?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
        ctx.next()?;
        ctx.lines_as(
            "Blacksmith Guildsman",
            args![
                "I've been assigned by my",
                "guild to bring back some",
                "Prize Medals to Einbroch...",
                "They're apparently made",
                "with some rare metal, but...",
                "It's too hard for me to win~!"
            ],
        )?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_PROFUSELY_SWEAT")?])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Blacksmith Guildsman",
        args![
            "Run! Go go go!",
            "I need to win some",
            "medals! Otherwise, I'll",
            "be too ashamed to return",
            "home to Einbroch! F-faster!"
        ],
    )?;
    ctx.call(Function::Emotion, vec![ctx.constant("ET_FLAG")?])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn blacksmith_guildsman_dou(ctx: &Ctx) -> Script {
    blacksmith_guildsman_dou_body(ctx, Vec::new()).map(|_| ())
}

fn valiant_knight_double_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Valiant Knight",
        args![
            "Hey, have you been",
            "wagering on the races?",
            "If you've got a hot tip, then",
            "would you share it with me?",
            "I've won some wagers... But",
            "I really wanna win more!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Valiant Knight",
        args![
            "Hah hah! It's like I tell",
            "those Blacksmiths! If they",
            "don't wanna lose all the time,",
            "then they should just bet on the same monster. Me? I always",
            "bet on the black Deviruchi~"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Valiant Knight", args!["You too...!", "Always bet", "on Deviruchi!"])?;
    ctx.call(Function::Emotion, vec![ctx.constant("ET_BEST")?])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn valiant_knight_double(ctx: &Ctx) -> Script {
    valiant_knight_double_body(ctx, Vec::new()).map(|_| ())
}

fn drunkard_single_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Familiar Drunkard",
        args![
            "Grrr...! ^333333*Hiccup*^000000",
            "I just gotta win this",
            "next game! I hafta do it!",
            "Hey, you! Which number is",
            "your lucky number? Huh?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["I, er..."])?;
    ctx.next()?;
    ctx.lines_as(
        "Familiar Drunkard",
        args![
            "C'mon, I need your",
            "lucky number cuz I ran",
            "out of mine! Tell me!",
            "Tell me! ^333333*Hiccup~*^000000"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn drunkard_single(ctx: &Ctx) -> Script {
    drunkard_single_body(ctx, Vec::new()).map(|_| ())
}

fn blacksmith_guildsman_sin_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Blacksmith Guildsman",
        args![
            "How can this be so hard?",
            "Why can't I win at least",
            "one of these races? Argh!",
            "I can't go back until I get",
            "at least one Prize Medal!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Blacksmith Guildsman",
        args![
            "Yeah, I've been assigned",
            "by the Einbroch Factory to",
            "get some Prize Medals since",
            "they're made of this rare medal. But it looks like they picked",
            "the wrong guy for this job..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Blacksmith Guildsman",
        args![
            "I mean, I've been here",
            "forever and I haven't won",
            "anything yet! Hey, do me",
            "a favor and give any extra",
            "Prize Medals you might have",
            "to the Einbroch Factory, okay?"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn blacksmith_guildsman_sin(ctx: &Ctx) -> Script {
    blacksmith_guildsman_sin_body(ctx, Vec::new()).map(|_| ())
}

fn absent_minded_man_single_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Absent Minded Man",
        args![
            "Say, are you here to bet",
            "on the monster races? I've",
            "come all the way here, just",
            "because some strange man",
            "asked me to win medals. It's",
            "the only reason I'm in Hugel..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Absent Minded Man",
        args![
            "But I've made more than",
            "100 wagers, and haven't won",
            "any of them! I mean, if I bet",
            "on the same monster 6 times,",
            "I should win at least once,",
            "right? What's going on?!"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn absent_minded_man_single(ctx: &Ctx) -> Script {
    absent_minded_man_single_body(ctx, Vec::new()).map(|_| ())
}

fn monster_race_manager_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    let mut l_select = Val::from(0);
    shared::other_gm_npcs::f_gm_npc(ctx, vec![])?;
    ctx.lines_as(
        "Monster Race Manager",
        args![
            "I can activate and",
            "deactivate the Arena",
            "Entry NPCs for the Single",
            "and Dual Monster Races."
        ],
    )?;
    ctx.next()?;
    l_select = Val::from(runtime::select_values(
        ctx,
        &[Val::from(
            "Single Race Entry - ON:Dual Race Entry - ON:Single Race Entry - OFF:Dual Race Entry - OFF",
        )],
    )?);
    ctx.lines_as("Monster Race Manager", args!["Please enter", "the password."])?;
    ctx.next()?;
    l_i = shared::other_gm_npcs::f_gm_npc(ctx, vec![Val::from(1854), Val::from(0), Val::from(0), Val::from(2000)])?;
    if l_i.clone() == -2 {
        ctx.lines_as("Monster Race Manager", args!["Error."])?;
    } else if l_i.clone() == -1 {
        ctx.lines_as("Monster Race Manager", args!["Incorrect password."])?;
    } else if l_i.clone() == 1 {
        ctx.mes("[Monster Race Manager]")?;
        let subject1 = l_select.clone();
        if subject1 == 1 {
            ctx.lines(args!["The Single Monster", "Race Entry NPC is ON."])?;
            ctx.call(Function::DoNpcEvent, vec![Val::from("Eckar Ellebird#single::OnEnable")])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if subject1 == 2 {
            ctx.lines(args!["The Dual Monster", "Race Entry NPC is ON."])?;
            ctx.call(Function::DoNpcEvent, vec![Val::from("Eckar Erenes#double::OnEnable")])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if subject1 == 3 {
            ctx.lines(args!["The Single Monster", "Race Entry NPC is OFF."])?;
            ctx.call(Function::DoNpcEvent, vec![Val::from("Eckar Ellebird#single::OnDisable")])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if subject1 == 4 {
            ctx.lines(args!["The Dual Monster", "Race Entry NPC is OFF."])?;
            ctx.call(Function::DoNpcEvent, vec![Val::from("Eckar Erenes#double::OnDisable")])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn monster_race_manager(ctx: &Ctx) -> Script {
    monster_race_manager_body(ctx, Vec::new()).map(|_| ())
}

use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn chemicals_cube_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input_s = Val::from("");
    if ctx.var("lhz_secret03").get()?.number()? < 1 {
        ctx.lines(args![
            "^3355FFThere is a bottle",
            "containing slightly",
            "corrosive chemicals",
            "that is sitting on the ledge.^000000"
        ])?;
        ctx.next()?;
        let (input, status) = runtime::input_text(ctx, None, None)?;
        l_input_s = input;
        ctx.lines(args![
            "^3355FFWhatever you tried to",
            "pour the chemicals on",
            "wasn't affected at all.",
            "You should try pouring the",
            "chemicals on something else.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("lhz_secret03").get()? == 1 {
        ctx.lines(args![
            "^3355FFThere is a bottle",
            "containing slightly",
            "corrosive chemicals",
            "that is sitting on the ledge.^000000"
        ])?;
        ctx.next()?;
        let (input, status) = runtime::input_text(ctx, None, None)?;
        l_input_s = input;
        if l_input_s.clone() == "Rusty Key" {
            ctx.lines(args![
                "^3355FFPouring the chemicals",
                "on the Rusty Key removes",
                "the rust, making it usable",
                "again. Now that it is clean,",
                "the Rusty Key has become",
                "a sparkling ^000000Green Key^3355FF.^000000"
            ])?;
            ctx.var("lhz_secret03").set(Val::from(2))?;
        } else {
            ctx.lines(args![
                "^3355FFWhatever you tried to",
                "pour the chemicals on",
                "wasn't affected at all.",
                "You should try pouring the",
                "chemicals on something else.^000000"
            ])?;
        }
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines(args![
        "^3355FFThese are the chemicals",
        "that you used to clean the",
        "Rusty Key so that it was",
        "restored to its original glory,",
        "becoming the ^000000Green Key^3355FF.^000000"
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn chemicals_cube(ctx: &Ctx) -> Script {
    chemicals_cube_body(ctx, Vec::new()).map(|_| ())
}

fn cabinet_cube_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input_s = Val::from("");
    if ctx.var("lhz_secret03").get()?.number()? < 2 {
        ctx.lines(args![
            "^3355FFYou've found a",
            "cabinet that contains",
            "many drawers. Perhaps",
            "something useful is inside?"
        ])?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Open"), Val::from("Cancel")])?) == 1 {
            let (input, status) = runtime::input_text(ctx, None, None)?;
            l_input_s = input;
            ctx.lines(args![
                "^3355FFUnfortunately, the",
                "cabinet has been locked.",
                "You'll need the right key",
                "in order to open the drawers.^000000"
            ])?;
        }
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("lhz_secret03").get()? == 2 {
        ctx.lines(args![
            "^3355FFYou've found a",
            "cabinet that contains",
            "many drawers. Perhaps",
            "something useful is inside?"
        ])?;
        ctx.next()?;
        let (input, status) = runtime::input_text(ctx, None, None)?;
        l_input_s = input;
        if l_input_s.clone() == "Green Key" {
            ctx.lines(args![
                "^3355FFYou take the Green Key",
                "and finds that it fits into",
                "one of the drawer keyholes.",
                "You open the drawer and",
                "obtain a strange ^000000Polygon^3355FF.^000000"
            ])?;
            ctx.var("lhz_secret03").set(Val::from(3))?;
        } else {
            ctx.lines(args![
                "^3355FFWhatever you tried",
                "did not succeed in",
                "opening this cabinet.",
                "Think. Think of how",
                "locks are unlocked...",
                "Then you'll find the",
                "answer you seek.^000000"
            ])?;
        }
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines(args![
        "^3355FFThis is the locker",
        "where you used the",
        "Green Key to open one",
        "of the Drawers and obtained",
        "a ^000000Polygon^3355FF. You're pretty happy",
        "with your Polygon and don't",
        "need to open the other drawers.^000000"
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn cabinet_cube(ctx: &Ctx) -> Script {
    cabinet_cube_body(ctx, Vec::new()).map(|_| ())
}

fn experiment_tube_cube_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input_s = Val::from("");
    if ctx.var("lhz_secret01").get()?.number()? < 8 {
        ctx.lines(args![
            "^3355FFYou find a strange",
            "tube that seems to",
            "contain something.",
            "Underneath the tube is",
            "a thin plate with a keyhole",
            "and a card insertion slot.^000000"
        ])?;
        ctx.next()?;
        let (input, status) = runtime::input_text(ctx, None, None)?;
        l_input_s = input;
        ctx.lines(args![
            "^3355FFNothing happened.",
            "You'll probably need",
            "to find the right key for",
            "the keyhole and the correct",
            "card to insert into the slot.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("lhz_secret01").get()? == 8 {
        ctx.lines(args![
            "^3355FFYou find a strange",
            "tube that seems to",
            "contain something.",
            "Underneath the tube is",
            "a thin plate with a keyhole",
            "and a card insertion slot.^000000"
        ])?;
        ctx.next()?;
        let (input, status) = runtime::input_text(ctx, None, None)?;
        l_input_s = input;
        if l_input_s.clone() == "Black Key" {
            ctx.lines(args![
                "^3355FFYou insert the Black Key",
                "into the keyhole, causing",
                "the experiment tube to open",
                "and reveal an ^000000Oval^3355FF which you",
                "you choose to take with you.^000000"
            ])?;
            ctx.var("lhz_secret01").set(Val::from(9))?;
        } else {
            ctx.mes("^3355FFNothing happened...^000000")?;
        }
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("lhz_secret01").get()? == 9 {
        ctx.lines(args![
            "^3355FFThis is where you",
            "obtained the ^000000Oval^3355FF.",
            "As you look around",
            "the tube's location,",
            "you notice an artificial",
            "ground fissure which",
            "can probably open up...^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("lhz_sincube").get()? == 10 {
            ctx.lines(args![
                "^3355FFThere is a narrow,",
                "rectangular card slot",
                "in front of the tube.^000000"
            ])?;
            ctx.next()?;
            let (input, status) = runtime::input_text(ctx, None, None)?;
            l_input_s = input;
            if l_input_s.clone() == "Laboratory Permit" {
                if ctx.call(Function::CountItem, vec![Val::from(2657)])?.is_true() {
                    ctx.lines(args![
                        "^3355FFYou insert the",
                        "Laboratory Permit",
                        "into the slot and the",
                        "man made fissure in the",
                        "ground splits open, revealing",
                        "an underground staircase.^000000"
                    ])?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("Go downstairs"), Val::from("Cancel")])?) == 1 {
                        ctx.lines(args!["^3355FFYou walk down", "the long flight", "of winding stairs...^000000"])?;
                        ctx.close_window()?;
                        ctx.var("lhz_sincube").set(Val::from(0))?;
                        ctx.var("lhz_secret01").set(Val::from(0))?;
                        ctx.var("lhz_secret02").set(Val::from(0))?;
                        ctx.var("lhz_secret03").set(Val::from(0))?;
                        ctx.var("misc_quest")
                            .set(runtime::op(&ctx.var("misc_quest").get()?, "|", &Val::from(512))?)?;
                        ctx.call(Function::Warp, vec![Val::from("lhz_cube"), Val::from(177), Val::from(13)])?;
                        return Err(Stop::End);
                    }
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines(args!["^3355FF..............", "Nothing happened.^000000"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.mes("^3355FFNothing happened.^000000")?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines(args![
            "^3355FFThis is where you",
            "obtained the ^000000Oval^3355FF.",
            "As you look around",
            "the tube's location,",
            "you notice an articial",
            "ground fissure which",
            "can probably open up...^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn experiment_tube_cube(ctx: &Ctx) -> Script {
    experiment_tube_cube_body(ctx, Vec::new()).map(|_| ())
}

fn box_cube1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input_s = Val::from("");
    if ((ctx.var("lhz_secret01").get()?.number()? < 9 && ctx.var("lhz_secret02").get()?.number()? < 4)
        && ctx.var("lhz_secret03").get()?.number()? < 3)
    {
        ctx.lines(args![
            "^3355FFYou find a box with",
            "three distinctively",
            "shaped holes.^000000"
        ])?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Oval Hole:Cubic Hole:Polygon Hole")])?;
        ctx.var("@menu").set(choice)?;
        let (input, status) = runtime::input_text(ctx, None, None)?;
        l_input_s = input;
        ctx.mes("^3355FFNothing happened.^000000")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ((ctx.var("lhz_secret01").get()? != 10 || ctx.var("lhz_secret02").get()? != 5) || ctx.var("lhz_secret03").get()? != 4) {
        ctx.lines(args![
            "^3355FFYou find a box with",
            "three distinctively",
            "shaped holes.^000000"
        ])?;
        ctx.next()?;
        match runtime::select_values(
            ctx,
            &[Val::from("Oval Hole"), Val::from("Cube Hole"), Val::from("Polygon Hole")],
        )? {
            1 => {
                if ctx.var("lhz_secret01").get()? == 9 {
                    let (input, status) = runtime::input_text(ctx, None, None)?;
                    l_input_s = input;
                    if l_input_s.clone() == "Oval" {
                        ctx.lines(args![
                            "^3355FFYou insert the Oval",
                            "into the Oval shaped",
                            "hole where it fits perfectly.^000000"
                        ])?;
                        ctx.var("lhz_secret01").set(Val::from(10))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.mes("^3355FFNothing happened.^000000")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ctx.var("lhz_secret01").get()? == 10 {
                    ctx.lines(args![
                        "^3355FFThe Oval shaped hole",
                        "already has an Oval in it.",
                        "Besides, it's not you have",
                        "any Ovals to spare, anyway.^000000"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                let (input, status) = runtime::input_text(ctx, None, None)?;
                l_input_s = input;
                ctx.mes("^3355FFNothing happened.^000000")?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                if ctx.var("lhz_secret02").get()? == 4 {
                    let (input, status) = runtime::input_text(ctx, None, None)?;
                    l_input_s = input;
                    if l_input_s.clone() == "Cube" {
                        ctx.lines(args![
                            "^3355FFYou insert the Cube",
                            "into the Cubic hole",
                            "and it clicks into place.",
                            "Your formal Kindergarten",
                            "training is finally justified.^000000"
                        ])?;
                        ctx.var("lhz_secret02").set(Val::from(5))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.mes("^3355FFNothing happened.^000000")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ctx.var("lhz_secret02").get()? == 5 {
                    ctx.lines(args![
                        "^3355FFYou already placed",
                        "a Cube into the hole.",
                        "You could take it back",
                        "out and put it in again,",
                        "but that would just be",
                        "a total waste of time.^000000"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                let (input, status) = runtime::input_text(ctx, None, None)?;
                l_input_s = input;
                ctx.mes("^3355FFNothing happened.^000000")?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            3 => {
                if ctx.var("lhz_secret03").get()? == 3 {
                    let (input, status) = runtime::input_text(ctx, None, None)?;
                    l_input_s = input;
                    if l_input_s.clone() == "Polygon" {
                        ctx.lines(args![
                            "^3355FFFortunately, the",
                            "nondescript Polygon",
                            "that you have is exactly",
                            "the same shape as this",
                            "nondescript Polygonal hole.",
                            "The Polygon fits perfectly,",
                            "almost as if it were destiny.^000000"
                        ])?;
                        ctx.var("lhz_secret03").set(Val::from(4))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.mes("^3355FFNothing happened.^000000")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ctx.var("lhz_secret03").get()? == 4 {
                    ctx.lines(args![
                        "^3355FFThe Polygon is already",
                        "inserted into the hole.",
                        "Trust that this is as much",
                        "as this Polygon can do for you.^000000"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                let (input, status) = runtime::input_text(ctx, None, None)?;
                l_input_s = input;
                ctx.mes("^3355FFNothing happened.^000000")?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    if (((ctx.var("lhz_secret01").get()? == 10 && ctx.var("lhz_secret02").get()? == 5) && ctx.var("lhz_secret03").get()? == 4)
        && ctx.var("lhz_sincube").get()? != 10)
    {
        ctx.lines(args![
            "^3355FFYou find a box with",
            "three distinctively",
            "shaped holes that",
            "are now filled with the",
            "objects you've inserted.^000000"
        ])?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Open the box."), Val::from("Cancel")])?) == 1 {
            ctx.lines(args![
                "^3355FFYou open the box",
                "and find that there's",
                "a small card labeled",
                "''Laboratory Permit'' inside.",
                "You pocket this ^000000Laboratory",
                "Permit^3355FF, knowing that you",
                "will be needing it later."
            ])?;
            ctx.var("lhz_sincube").set(Val::from(10))?;
            ctx.call(Function::GetItem, vec![Val::from(2657), Val::from(1)])?;
        }
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines(args![
        "^3355FFYou find the open box",
        "which used to contain",
        "the ^000000Laboratory Permit^3355FF.",
        "This box is no longer",
        "useful to you now, but",
        "once upon a time, its",
        "mysteries were a challenge.^000000"
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn box_cube1(ctx: &Ctx) -> Script {
    box_cube1_body(ctx, Vec::new()).map(|_| ())
}

fn door_cube_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args!["^3355FFYou've come upon a", "door that leads outside.^000000"])?;
    ctx.next()?;
    if Val::from(runtime::select_values(ctx, &[Val::from("Exit"), Val::from("Cancel")])?) == 1 {
        ctx.call(Function::Warp, vec![Val::from("lighthalzen"), Val::from(310), Val::from(302)])?;
        return Err(Stop::End);
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn door_cube(ctx: &Ctx) -> Script {
    door_cube_body(ctx, Vec::new()).map(|_| ())
}

fn exit1_lt_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn exit1_lt(ctx: &Ctx) -> Script {
    exit1_lt_body(ctx, Vec::new()).map(|_| ())
}

fn exit1_lt_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if runtime::op(&ctx.var("misc_quest").get()?, "&", &Val::from(512))?.is_true() {
        ctx.call(Function::Warp, vec![Val::from("lhz_cube"), Val::from(231), Val::from(90)])?;
        return Err(Stop::End);
    }
    ctx.lines(args![
        "^3355FFYou found a passage that",
        "seems to lead somewhere,",
        "but you get the feeling that",
        "you shouldn't enter it for now.^000000"
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn exit1_lt_ontouch(ctx: &Ctx) -> Script {
    exit1_lt_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn suspicious_guy_lhz_01_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn suspicious_guy_lhz_01(ctx: &Ctx) -> Script {
    suspicious_guy_lhz_01_body(ctx, Vec::new()).map(|_| ())
}

fn suspicious_guy_lhz_01_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let subject1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
    if subject1 == 1 {
        ctx.mes("^3355FF*SHHHHHHUK!*^000000")?;
        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_STEAL")?])?;
        if ctx.var("Zeny").get()?.number()? >= 100 {
            ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(100))?))?;
        }
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Huh...?", "I sense something...", "No. It might just", "be my imagination."],
        )?;
        ctx.next()?;
        ctx.lines_as("?????", args!["^333333Heh heh heh heh...^000000"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if subject1 == 2 {
        ctx.mes("^3355FF*SHHHHHHUK!*^000000")?;
        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_STEAL")?])?;
        if ctx.var("Zeny").get()?.number()? >= 200 {
            ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(200))?))?;
        }
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Huh...?",
                "I could have sworn",
                "that these pockets full",
                "of zeny were heavier",
                "just a second ago..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("?????", args!["^333333Heh heh heh heh...^000000"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if subject1 == 3 {
        ctx.mes("^3355FF*SHHHHHHUK!*^000000")?;
        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_STEAL")?])?;
        if ctx.var("Zeny").get()?.number()? >= 10 {
            ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(10))?))?;
        }
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Huh...?",
                "What the?!",
                "That guy, did he just...?",
                "He did! Hey! That guy",
                "stole some of my money!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Suspicious Guy", args!["Oh crap!", "Gotta scram!", "Eat my dust, good guy!"])?;
        ctx.next()?;
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(4)])? == 1 {
            ctx.call(Function::EnableNpc, vec![Val::from("Suspicious Guy#lhz_03")])?;
            ctx.call(Function::DisableNpc, vec![Val::from("Suspicious Guy#lhz_01")])?;
        } else {
            ctx.call(Function::EnableNpc, vec![Val::from("Suspicious Guy#lhz_02")])?;
            ctx.call(Function::DisableNpc, vec![Val::from("Suspicious Guy#lhz_01")])?;
        }
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn suspicious_guy_lhz_01_ontouch(ctx: &Ctx) -> Script {
    suspicious_guy_lhz_01_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn suspicious_guy_lhz_02_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn suspicious_guy_lhz_02(ctx: &Ctx) -> Script {
    suspicious_guy_lhz_02_body(ctx, Vec::new()).map(|_| ())
}

fn suspicious_guy_lhz_02_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Suspicious Guy#lhz_02")])?;
    return Err(Stop::End);
}

pub fn suspicious_guy_lhz_02_oninit(ctx: &Ctx) -> Script {
    suspicious_guy_lhz_02_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn suspicious_guy_lhz_02_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let subject1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
    if subject1 == 1 {
        ctx.mes("^3355FF*SHHHHHHUK!*^000000")?;
        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_STEAL")?])?;
        if ctx.var("Zeny").get()?.number()? >= 100 {
            ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(100))?))?;
        }
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Huh...?", "I sense something...", "No. It might just", "be my imagination."],
        )?;
        ctx.next()?;
        ctx.lines_as("?????", args!["^333333Heh heh heh heh...^000000"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if subject1 == 2 {
        ctx.mes("^3355FF*SHHHHHHUK!*^000000")?;
        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_STEAL")?])?;
        if ctx.var("Zeny").get()?.number()? >= 200 {
            ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(200))?))?;
        }
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Huh...?",
                "I could have sworn",
                "that these pockets full",
                "of zeny were heavier",
                "just a second ago..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("?????", args!["^333333Heh heh heh heh...^000000"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if subject1 == 3 {
        ctx.mes("^3355FF*SHHHHHHUK!*^000000")?;
        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_STEAL")?])?;
        if ctx.var("Zeny").get()?.number()? >= 10 {
            ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(10))?))?;
        }
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Huh...?",
                "What the?!",
                "That guy, did he just...?",
                "He did! Hey! That guy",
                "stole some of my money!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Suspicious Guy", args!["Oh crap!", "Gotta scram!", "Eat my dust, good guy!"])?;
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(4)])? == 1 {
            ctx.call(Function::EnableNpc, vec![Val::from("Suspicious Guy#lhz_03")])?;
            ctx.call(Function::DisableNpc, vec![Val::from("Suspicious Guy#lhz_02")])?;
        } else {
            ctx.call(Function::EnableNpc, vec![Val::from("Suspicious Guy#lhz_01")])?;
            ctx.call(Function::DisableNpc, vec![Val::from("Suspicious Guy#lhz_02")])?;
        }
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn suspicious_guy_lhz_02_ontouch(ctx: &Ctx) -> Script {
    suspicious_guy_lhz_02_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn suspicious_guy_lhz_03_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn suspicious_guy_lhz_03(ctx: &Ctx) -> Script {
    suspicious_guy_lhz_03_body(ctx, Vec::new()).map(|_| ())
}

fn suspicious_guy_lhz_03_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Suspicious Guy#lhz_03")])?;
    return Err(Stop::End);
}

pub fn suspicious_guy_lhz_03_oninit(ctx: &Ctx) -> Script {
    suspicious_guy_lhz_03_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn suspicious_guy_lhz_03_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_number = Val::from(0);
    let mut l_price = Val::from(0);
    let subject1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(5)])?;
    if subject1 == 1 || subject1 == 2 {
        ctx.mes("^3355FF*SHHHHHHUK!*^000000")?;
        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_STEAL")?])?;
        if ctx.var("Zeny").get()?.number()? >= 100 {
            ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(100))?))?;
        }
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Huh...?", "I sense something...", "No. It might just", "be my imagination."],
        )?;
        ctx.next()?;
        ctx.lines_as("?????", args!["^333333Heh heh heh heh...^000000"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if subject1 == 3 || subject1 == 4 {
        ctx.mes("^3355FF*SHHHHHHUK!*^000000")?;
        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_STEAL")?])?;
        if ctx.var("Zeny").get()?.number()? >= 200 {
            ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(200))?))?;
        }
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Huh...?",
                "I could have sworn",
                "that these pockets full",
                "of zeny were heavier",
                "just a second ago..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("?????", args!["^333333Heh heh heh heh...^000000"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if subject1 == 5 {
        ctx.mes("^3355FF*SHHHHHHUK!*^000000")?;
        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_STEAL")?])?;
        if ctx.var("Zeny").get()?.number()? >= 10 {
            ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(10))?))?;
        }
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Huh...?",
                "What the?!",
                "That guy, did he just...?",
                "He did! Hey! That guy",
                "stole some of my money!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Suspicious Guy", args!["Oh crap!", "Gotta scram!", "Eat my dust, good gu--!"])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Oh no, you don't!", "I'm turning you in,", "you pickpocket!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Suspicious Guy",
            args!["No...! I'm sorry!", "I'll give you back", "your money, just ", "let me go! Crap!"],
        )?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Alright, fine,", "but you better quit", "this life of crime!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Suspicious Guy",
            args![
                "You're right, that was",
                "wrong of me. Thanks",
                "for letting me go. Since",
                "you did me a favor, I'll",
                "sell you some tonic that",
                "I use to run really fast."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Suspicious Guy",
            args![
                "I can only sell up to",
                "three of my secret tonic",
                "to you since that's all",
                "I have. Each one will",
                "cost 15,000 zeny. So",
                "what do you say?"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Give me 1.:Give me 2.:Give me 3.:No, I'm fine.")])? {
            1 => {
                l_number = Val::from(1);
                l_price = (Val::from(15000).try_mul(Val::from(1))?);
            }
            2 => {
                l_number = Val::from(2);
                l_price = (Val::from(15000).try_mul(Val::from(2))?);
            }
            3 => {
                l_number = Val::from(3);
                l_price = (Val::from(15000).try_mul(Val::from(3))?);
            }
            4 => {
                ctx.lines_as(
                    "Suspicious Guy",
                    args![
                        "Well...",
                        "You're just gonna",
                        "let me go, then?",
                        "That's awful generous.",
                        "Thanks, I appreciate it."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Suspicious Guy",
                    args!["Anyway, I'm gonna", "get back to work.", "Heh heh heh~"],
                )?;
            }
            _ => {}
        }
        if l_number.clone().is_true() {
            if runtime::op(&ctx.var("Zeny").get()?, "<", &l_price.clone())?.is_true() {
                ctx.lines_as(
                    "Suspicious Guy",
                    args![
                        "Er, since I'm quitting",
                        "pickpocketing, I need",
                        "to make cash legitimately",
                        "as a business person. I'm",
                        "real sorry pal, but I can't just",
                        "give these tonics away!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Suspicious Guy",
                    args!["Anyway, I'm gonna", "get back to work.", "Heh heh heh~"],
                )?;
            } else if (ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 400 {
                ctx.lines_as(
                    "Suspicious Guy",
                    args![
                        "Eh, I'm sorry, but you",
                        "don't have enough room",
                        "in your inventory to even",
                        "hold these tonics. Sorry",
                        "pal, but this deal's off."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Suspicious Guy",
                    args!["Anyway, I'm gonna", "get back to work.", "Heh heh heh~"],
                )?;
            } else {
                ctx.mes("[Suspicious Guy]")?;
                if l_number.clone() == 1 {
                    ctx.lines(args![
                        "Only one? Alright, you",
                        "might be a cheapskate,",
                        "but I do owe you. Just",
                        "take this and I hope we never",
                        "bump into each other again!"
                    ])?;
                } else if l_number.clone() == 2 {
                    ctx.lines(args![
                        "Two, huh? I can dig it.",
                        "Now take these and I hope",
                        "we never see each other",
                        "ever again! Ciao, baby~"
                    ])?;
                } else {
                    ctx.lines(args![
                        "You want all three?",
                        "Heh, you're much smarter",
                        "than I thought! Alright, take",
                        "these with my compliments,",
                        "but I hope we never bump",
                        "into each other ever again!"
                    ])?;
                }
                ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(l_price.clone())?))?;
                ctx.call(Function::GetItem, vec![Val::from(12016), l_number.clone()])?;
            }
        }
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(2)])? == 1 {
            ctx.call(Function::EnableNpc, vec![Val::from("Suspicious Guy#lhz_01")])?;
            ctx.call(Function::DisableNpc, vec![Val::from("Suspicious Guy#lhz_03")])?;
        } else {
            ctx.call(Function::EnableNpc, vec![Val::from("Suspicious Guy#lhz_02")])?;
            ctx.call(Function::DisableNpc, vec![Val::from("Suspicious Guy#lhz_03")])?;
        }
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn suspicious_guy_lhz_03_ontouch(ctx: &Ctx) -> Script {
    suspicious_guy_lhz_03_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn digotz_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("BaseLevel").get()?.number()? < 50 {
        ctx.lines_as(
            "Digotz",
            args![
                "Oh, an adventurer?",
                "Welcome to Uptown",
                "Lighthalzen. However,",
                "I'm afraid this area won't",
                "have much to offer you",
                "in the way of excitement."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Digotz",
            args![
                "Not to be rude or",
                "anything, but this town",
                "should be safe enough for",
                "you to explore. I mean, you",
                "just seem to be kind of new",
                "at this adventurer thing..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("friendship").get()?.number()? > 14 {
        ctx.call(Function::Cutin, vec![Val::from("lhz_diguts05.bmp"), Val::from(2)])?;
        ctx.lines(args![
            "^3355FFDigotz has passed",
            "away, but the look on",
            "his face seems very",
            "peaceful and content.^000000"
        ])?;
        ctx.close_window()?;
        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
        return Err(Stop::End);
    }
    if ctx.var("friendship").get()? == 14 {
        ctx.lines(args![
            "^3355FFDigotz is seriously",
            "injured from a wound",
            "by a knife that is still",
            "embedded in his belly.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Digotz...?", "Oh no, let me", "get you some help!"],
        )?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("lhz_diguts04.bmp"), Val::from(2)])?;
        ctx.lines_as(
            "Digotz",
            args![
                "H-hey... It's the",
                "adventurer... Man,",
                "that Maku. He always",
                "did bring me bad luck...",
                "It's too late for me and",
                "I don't have much time..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Digotz",
            args![
                "Those guards I told you",
                "about... The ones who don't",
                "want the poor and the rich to",
                "mingle? I... Guess they found",
                "I was gonna meet my old pal.",
                "I just wanted to see him..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Digotz",
            args![
                "This guy... In a black",
                "suit... He just... He just",
                "stabbed me! I... God. It's",
                "been so long since I've talked",
                "to him. We'll hang out and have",
                "fun, just like the good old days."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("lhz_diguts05.bmp"), Val::from(2)])?;
        ctx.lines_as(
            "Digotz",
            args![
                "I missed my buddies, but now...",
                "Now I can hear them calling me.",
                "Now we can all be together just",
                "like we all promised. Yeah...",
                "I was wrong. Life's too short",
                "to be angry with your frie--"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Digotz", args![".............."])?;
        ctx.next()?;
        ctx.lines_as("Digotz", args!["..............", "......................."])?;
        ctx.next()?;
        ctx.lines_as(
            "Digotz",
            args!["..............", ".......................", "..............................."],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFDigotz stopped breathing.",
            "You remove the Knife from",
            "his lifeless body as a final",
            "courtesy to a man who",
            "dearly loved his friends.^000000"
        ])?;
        ctx.next()?;
        ctx.var("friendship").set(Val::from(15))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(12005), Val::from(12006)])?;
        ctx.call(Function::GetItem, vec![Val::from(1201), Val::from(1)])?;
        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("friendship").get()? == 13 {
        ctx.call(Function::Cutin, vec![Val::from("lhz_diguts08.bmp"), Val::from(2)])?;
        ctx.lines_as(
            "Digotz",
            args![
                "Wh-whoa, I need to",
                "get ready! That Maku's",
                "gonna make fun of me if",
                "I look too rich and pampered.",
                "Damn! Where did I put all of",
                "my fashionable street clothes?"
            ],
        )?;
        ctx.close_window()?;
        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
        return Err(Stop::End);
    }
    if (ctx.var("friendship").get()? == 12 && ctx.call(Function::CountItem, vec![Val::from(7351)])?.number()? > 0) {
        ctx.call(Function::Cutin, vec![Val::from("lhz_diguts08.bmp"), Val::from(2)])?;
        ctx.lines_as(
            "Digotz",
            args![
                "Even if Benkaistein",
                "did come back, I don't",
                "think I could forgive Maku.",
                "In fact, you know what?",
                "I think I'd even be madder!"
            ],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from("Show Benkaistein's Journal.:Don't show Benkaistein's Journal.")],
        )?) == 1
        {
            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
            ctx.lines_as(
                "Digotz",
                args![
                    "Why am I so ticked off?",
                    "^3355FF*Sigh*^000000 You have something",
                    "to show me? Huh? Benkaistein",
                    "wanted me to read this diary",
                    "of his? Sure, why not? I do",
                    "owe him a lot over the years..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Benkaistein's Journal",
                args![
                    "^856363Today, me, Digotz and",
                    "Maku played this crazy flying",
                    "game. Basically, we make",
                    "these wings out of wood and",
                    "paper, jump off these hills",
                    "and try to fly. Dumb, I know.^000000"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Benkaistein's Journal",
                args![
                    "^856363Today it was my turn to",
                    "jump and flap my arms with",
                    "these fake, badly made wings.",
                    "It's not really a fun game when",
                    "I think about it. Boy, I hope",
                    "we don't do that again.^000000"
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::Cutin, vec![Val::from("lhz_diguts02.bmp"), Val::from(2)])?;
            ctx.lines_as(
                "Digotz",
                args![
                    "Oh yeah, I remember that!",
                    "Maku wore the wings most",
                    "of the time, but I still hold",
                    "the record for staying in the",
                    "air the longest! Yeah, I was",
                    "a regular Kid Pegasus~"
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
            ctx.lines_as(
                "Benkaistein's Journal",
                args![
                    "^856363Maku, Digotz and me went",
                    "outside of town. Of course,",
                    "we didn't tell anyone or else",
                    "we'd get in trouble. It was",
                    "a really exciting day. But",
                    "then, we ran into a monster!^000000"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Benkaistein's Journal",
                args![
                    "^856363I wanted to run away but Maku",
                    "and Digotz wanted to beat it so",
                    "that we could become heroes.",
                    "Of course, we got hurt pretty",
                    "bad and the monster got away.",
                    "Boy, mom was not happy...^000000"
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::Cutin, vec![Val::from("lhz_diguts03.bmp"), Val::from(2)])?;
            ctx.lines_as(
                "Digotz",
                args![
                    "Huh. I don't remember",
                    "that so well. But I know that",
                    "Benkaistein, me and Maku",
                    "weren't afraid of anything back",
                    "then. We must have been totally",
                    "nuts to fight a monster, though."
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
            ctx.lines_as(
                "Benkaistein's Journal",
                args![
                    "^856363Digotz's been sick for three",
                    "days now. It's just a normal",
                    "cold and Maku keeps saying",
                    "it's Digotz's fault he got sick.^FFFFFF ^856363 But he's always asking me to",
                    "go visit him and see if he's okay.^000000"
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::Cutin, vec![Val::from("lhz_diguts01.bmp"), Val::from(2)])?;
            ctx.lines_as(
                "Digotz",
                args![
                    "I think I remember being",
                    "pretty sick. Maku was worried?",
                    "I... I must have had a horrible",
                    "life threatening disease like,",
                    "um, Gonorrhitis. You know.",
                    "That might have been it."
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
            ctx.lines_as(
                "Benkaistein's Journal",
                args![
                    "^856363Mom and dad keep telling",
                    "me not to hang out with Maku",
                    "anymore. Their reason is really",
                    "dumb, and I don't care if he is",
                    "poor. He's one of the best guys",
                    "that I'll ever know.^000000"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Benkaistein's Journal",
                args![
                    "^856363Digotz's family is really",
                    "rich and they don't want him",
                    "to see Maku anymore either.",
                    "But Digotz doesn't care.",
                    "I know he likes Maku a lot.^000000"
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::Cutin, vec![Val::from("lhz_diguts07.bmp"), Val::from(2)])?;
            ctx.lines_as(
                "Digotz",
                args![
                    "Well, we were a lot",
                    "younger and closer back",
                    "then, so... ^333333*Ahem!*^000000 Why did",
                    "Benkaistein even write that?!"
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
            ctx.lines_as(
                "Benkaistein's Journal",
                args![
                    "^856363Today, the three of us",
                    "made an oath of brotherhood,",
                    "just like we read in the comic",
                    "book. We swore we'd always",
                    "be friends no matter what.",
                    "For always and for always.^000000"
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::Cutin, vec![Val::from("lhz_diguts07.bmp"), Val::from(2)])?;
            ctx.lines_as(
                "Digotz",
                args![
                    "I... I was forced to make",
                    "that oath! And people do",
                    "change, you know! I mean,",
                    "we were basically just kids,",
                    "it's not like that oath really",
                    "means anything now, does it?"
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
            ctx.var("friendship").set(Val::from(13))?;
            ctx.lines_as(
                "Digotz",
                args![
                    "That does it. I'm gonna",
                    "go see that Maku. I don't",
                    "miss him or anything, but",
                    "I gotta get him to cancel",
                    "that oath. And maybe I'll",
                    "beat up him a little bit."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Digotz",
            args![
                "I don't understand",
                "why I'm so angry!",
                "I'm starting to act",
                "more like Maku, though,",
                "don't get me wrong, it's",
                "not like I care about the guy."
            ],
        )?;
        ctx.close_window()?;
        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
        return Err(Stop::End);
    }
    if ctx.var("friendship").get()? == 7 {
        ctx.call(Function::Cutin, vec![Val::from("lhz_diguts03.bmp"), Val::from(2)])?;
        ctx.lines_as(
            "Digotz",
            args![
                "Even if Benkaistein came",
                "back from wherever he was",
                "studying, I don't think he'd be",
                "able to get Maku to apologize",
                "to me. That guy is just way",
                "too stubborn for his own good!"
            ],
        )?;
        ctx.close_window()?;
        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
        return Err(Stop::End);
    }
    if ctx.var("friendship").get()? == 6 {
        ctx.call(Function::Cutin, vec![Val::from("lhz_diguts01.bmp"), Val::from(2)])?;
        ctx.lines_as(
            "Digotz",
            args![
                "Oh, it's been a while.",
                "What are you doing back",
                "over here? And, um, did",
                "you deliver that message",
                "to Maku? Now when I think",
                "about it, I was kind of--"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "I delivered your message",
                "word for word, and Maku",
                "got angry, called you names",
                "and has been threatening to",
                "beat you up pretty badly."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("lhz_diguts08.bmp"), Val::from(2)])?;
        ctx.lines_as(
            "Digotz",
            args![
                "That no-good, dirty",
                "lying rotten scoundrel!",
                "If it weren't for those",
                "guards, I'd head over to",
                "the ghetto and beat Maku",
                "up myself! That stupid guy!"
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("lhz_diguts03.bmp"), Val::from(2)])?;
        ctx.lines_as(
            "Digotz",
            args![
                "During times like this,",
                "I really miss ^FF0000Benkaistein^000000.",
                "That guy would always have",
                "an answer for this kind of",
                "situation. Yeah, I think he's",
                "in some far off town, studying."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Digotz",
            args![
                "Supposedly he's in that",
                "place, whatever it's called,",
                "since there's a ton of books",
                "there that he can use. But",
                "yeah, Benkaistein would",
                "always be the mediator..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Digotz",
            args![
                "Even back then, when",
                "me, him and Maku used to",
                "hang out, Benkaistein would",
                "mediate if we got into some",
                "argument. Still, he couldn't",
                "do anything about Maku now..."
            ],
        )?;
        ctx.var("friendship").set(Val::from(7))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(12002), Val::from(12003)])?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
        ctx.lines_as(
            "Digotz",
            args![
                "I don't know why,",
                "but I'm so angry!",
                "Why am I stressing",
                "out so much over this?!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("friendship").get()? == 4 || ctx.var("friendship").get()? == 5) {
        ctx.call(Function::Cutin, vec![Val::from("lhz_diguts01.bmp"), Val::from(2)])?;
        ctx.lines_as(
            "Digotz",
            args![
                "Still checking out",
                "Uptown Lighthalzen?",
                "Not like I'd care, but if you",
                "do happen to see Maku,",
                "deliver this little message",
                "for me, sentence by sentence."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Digotz",
            args![
                "^FF0000Hopeless bastard!",
                "^FF0000You're still a stubborn jerk!",
                "^FF0000You owe me at least 3 lunches!",
                "^FF0000Not to mention an apology!",
                "^FF0000But who cares what you think?!",
                "I'm so goddamn happy without you!^000000"
            ],
        )?;
        ctx.close_window()?;
        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
        return Err(Stop::End);
    }
    if ctx.var("friendship").get()? == 3 {
        ctx.call(Function::Cutin, vec![Val::from("lhz_diguts01.bmp"), Val::from(2)])?;
        ctx.lines_as(
            "Digotz",
            args![
                "I know that the",
                "opulence of Uptown",
                "seems rather attractive,",
                "but trust me. This place",
                "is colorless. Now, have",
                "you visited the poor district?"
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Yes, I did already...")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Yes, I did already...", "And I met someone", "named Maku there."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Digotz",
            args![
                "Maku?! Oh, he must have",
                "mentioned something about",
                "me. But I don't care what he",
                "says, unless it's an apology",
                "for being a fully blown jerk.",
                "Ever since we were kids..."
            ],
        )?;
        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("lhz_diguts08.bmp"), Val::from(2)])?;
        ctx.lines_as(
            "Digotz",
            args![
                "Anyway, we used to be close,",
                "but that guy was never a true",
                "friend of mine! Like that one",
                "time he cheated to beat me at",
                "arm wrestling! Or when he never",
                "thanked me for buying us lunch!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Digotz",
            args![
                "Sure, he might have helped",
                "me a little in meeting my first",
                "girlfriend, but I'll never ever",
                "forgive him for fixing me up",
                "on the worst blind dates a",
                "man can possibly experience!"
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("lhz_diguts01.bmp"), Val::from(2)])?;
        ctx.lines_as(
            "Digotz",
            args![
                "Maku doesn't know a damn",
                "about friendship! Even if I did",
                "want to see him, there are these",
                "people who don't want the rich",
                "to ever meet with the poor."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Digotz",
            args![
                "If Maku's fine, that's",
                "good enough to hear for",
                "me! There's no need for me",
                "to go all the way over there",
                "and check up on him! I only",
                "have one regret though..."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("lhz_diguts07.bmp"), Val::from(2)])?;
        ctx.lines_as(
            "Digotz",
            args![
                "I only wish I had one",
                "last chance to see Maku...",
                "So that I could kick his sorry",
                "ass myself! Yeah, that's right!",
                "Arrogant bastard! But still,",
                "I'm not able to do that..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Digotz",
            args![
                "The security guards here",
                "will never allow the rich and",
                "poor to meet, fearing that",
                "the poor will disturb the peace",
                "and order of the city. It's a dumb rule made for dumb people."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Digotz",
            args![
                "Still, it's pretty scary that",
                "someone can get punished",
                "for violating such a stupid",
                "taboo, actually. Anyway, if",
                "you see Maku again, tell",
                "him this for me, got it?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Digotz",
            args![
                "^FF0000Hopeless bastard!",
                "^FF0000You're still a stubborn jerk!",
                "^FF0000You owe me at least 3 lunches!",
                "^FF0000Not to mention an apology!",
                "^FF0000But who cares what you think?!",
                "I'm so goddamn happy without you!^000000"
            ],
        )?;
        ctx.var("friendship").set(Val::from(4))?;
        ctx.close_window()?;
        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
        return Err(Stop::End);
    }
    if ctx.var("friendship").get()? == 2 {
        ctx.call(Function::Cutin, vec![Val::from("lhz_diguts01.bmp"), Val::from(2)])?;
        ctx.lines_as(
            "Digotz",
            args![
                "What are you still",
                "doing hanging around",
                "here? There's nothing",
                "interesting in Uptown",
                "for you to see, adventurer."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Digotz",
            args![
                "Gosh...!",
                "Just hearing about",
                "Maku makes me so feel",
                "so upset for some reason!"
            ],
        )?;
        ctx.close_window()?;
        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
        return Err(Stop::End);
    }
    if ctx.var("friendship").get()? == 1 {
        ctx.call(Function::Cutin, vec![Val::from("lhz_diguts02.bmp"), Val::from(2)])?;
        ctx.lines_as(
            "Digotz",
            args![
                "Oh, an adventurer?",
                "Welcome to Uptown",
                "Lighthalzen. However,",
                "I'm afraid this area won't",
                "have much to offer you",
                "in the way of excitement."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Digotz",
            args![
                "My name is Digotz,",
                "just another citizen",
                "of Upper Lighthalzen.",
                "I hope that you enjoy",
                "your stay in my hometown."
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Do you know someone named Maku?")])?;
        ctx.var("@menu").set(choice)?;
        ctx.call(Function::Cutin, vec![Val::from("lhz_diguts01.bmp"), Val::from(2)])?;
        ctx.lines_as(
            "Digotz",
            args![
                "Maku? Maku. Yes, he's my",
                "childhood friend. Or he was,",
                "anyway. Now he's just a jerk.",
                "In any case, we can't hang",
                "out, even if we wanted to,",
                "for several reasons."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Digotz",
            args![
                "Hey, why am I even",
                "talking about this? It's",
                "not like I'm bothered by",
                "the fact me and Maku aren't",
                "pals anymore. You know what?",
                "Just forget everything I said."
            ],
        )?;
        ctx.var("friendship").set(Val::from(2))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(12000), Val::from(12001)])?;
        ctx.close_window()?;
        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
        return Err(Stop::End);
    }
    ctx.call(Function::Cutin, vec![Val::from("lhz_diguts02.bmp"), Val::from(2)])?;
    ctx.lines_as(
        "Digotz",
        args![
            "Oh, an adventurer?",
            "Welcome to Uptown",
            "Lighthalzen. However,",
            "I'm afraid this area won't",
            "have much to offer you",
            "in the way of excitement."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Digotz",
        args![
            "Feel free to take",
            "a look around if you",
            "so wish. I'm actually",
            "glad to see somebody",
            "aside from the stuck up",
            "rich people who live here."
        ],
    )?;
    ctx.close_window()?;
    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
    return Err(Stop::End);
}

pub fn digotz(ctx: &Ctx) -> Script {
    digotz_body(ctx, Vec::new()).map(|_| ())
}

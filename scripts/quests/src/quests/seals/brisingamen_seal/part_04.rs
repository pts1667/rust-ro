use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum Berling1Step {
    Start,
    OnInit,
}

fn berling_1_run(ctx: &Ctx, mut step: Berling1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Berling1Step::Start => {
                if ctx.var("god_brising").get()?.number()? > 44 {
                    ctx.lines_as(
                        "Berling",
                        args!["You're the one that's awakened us? Hahaha, perhaps that's a sign we may see Freya again! Hahahaha!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Berling", args!["We are indebted to you for reviving us. Okay, I know what you want, and will help you obtain the power of the gods."])?;
                    ctx.close_window()?;
                    ctx.call(Function::DisableNpc, vec![Val::from("OnBerling#1")])?;
                    return Err(Stop::End);
                } else if ctx.var("god_brising").get()? == 43 {
                    ctx.lines_as(
                        "Berling",
                        args!["What the...?", "It's a human?!", "What do you want?", "Wh-who are you?"],
                    )?;
                    ctx.next()?;
                    'b1: {
                        let subject1 = Val::from(runtime::select_values(
                            ctx,
                            &[Val::from("Dvalin wants you to wake up!:Tell me who you are first!")],
                        )?);
                        let mut matched1 = false;
                        let no_case1 = !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2));
                        if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                            matched1 = true;
                        }
                        if matched1 {
                            ctx.lines_as(
                                "Berling",
                                args![
                                    "Dvalin?!",
                                    "You mean my brother!",
                                    "But he can only be revived if Alfrik is revived. What happened?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["Umm...", "Let me explain..."],
                            )?;
                            ctx.next()?;
                            ctx.lines(args!["...", "......", "........."])?;
                            ctx.next()?;
                            ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["...So I guess I need to wake up all four of you dwarves so that Brisingamen, which holds Freya's power, can be made again."])?;
                            ctx.next()?;
                            ctx.lines_as("Berling", args!["Oh! Say that name once again! Freya, the goddess who took my body and soul! Did you just say you want to make her necklace?"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Berling",
                                args![
                                    "Hahahaha!",
                                    "Stop joking around.",
                                    "The Brisingamen is",
                                    "a one of a kind",
                                    "masterpiece!"
                                ],
                            )?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("I don't mean the original one, but...")])? {
                                1 => {
                                    ctx.lines_as(
                                        "Berling",
                                        args![
                                            "Ah, now I see! You want a replica that has roughly the same power.",
                                            "A Brisingamen used by humans!",
                                            "Very clever!"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Berling", args!["I, on the other hand, do not desire power. I would be happy to own but a strand of my goddess' hair."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Berling", args!["Oh Freya...", "The more of her beauty exists in this world, the happier I'll be. I'm more than willing to help you recreate this memento of Freya."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Berling", args!["Now, please go and find the last one of my brothers! Poor Grer, perhaps he is still digging silver in that mine for her..."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Berling", args!["Go and find Grer. I will give you Freya's golden teardrop, but he might have revived on his own..."])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Berling",
                                        args![
                                            "If he doesn't trust you, sing this song. You must remember each",
                                            "and every word..."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Berling", args!["^4d4dffNo jewel in the world can compare.^000000"])?;
                                    ctx.next()?;
                                    ctx.lines_as("Berling", args!["^4d4dffOur masterpiece made from love.^000000"])?;
                                    ctx.next()?;
                                    ctx.lines_as("Berling", args!["^4d4dffShe wanted the dazzling necklace.^000000"])?;
                                    ctx.next()?;
                                    ctx.lines_as("Berling", args!["^4d4dffWe wanted the goddess of beauty.^000000"])?;
                                    ctx.next()?;
                                    ctx.lines_as("Berling", args!["^4d4dffOur happiest times were with her.^000000"])?;
                                    ctx.next()?;
                                    ctx.lines_as("Berling", args!["Memorize all five lines of this song exactly. Now take this golden teardrop and please seek out Grer."])?;
                                    ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_IMPOSITIO")?])?;
                                    ctx.var("god_brising").set(Val::from(44))?;
                                    ctx.close_window()?;
                                    ctx.call(Function::DisableNpc, vec![Val::from("Berling#1")])?;
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
                                "Berling",
                                args![
                                    "Where the hell",
                                    "did you come from?",
                                    "I, Berling, refuse to",
                                    "give my name to a rude,",
                                    "ill-bred human like yourself!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["Wait...", "Your name", "is Berling?"],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Berling", args!["Baaaaahhh!", "Quiet you!", "Mind your own", "business!"])?;
                            ctx.close_window()?;
                            ctx.call(Function::DisableNpc, vec![Val::from("Berling#1")])?;
                            return Err(Stop::End);
                        }
                    }
                } else if ctx.var("god_brising").get()? == 44 {
                    ctx.lines_as(
                        "Berling",
                        args!["Did you already forget the lyrics? Let me tell them to you again, so don't forget this time."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Berling", args!["^4d4dffNo jewel in the world can compare.^000000"])?;
                    ctx.next()?;
                    ctx.lines_as("Berling", args!["^4d4dffOur masterpiece made from love.^000000"])?;
                    ctx.next()?;
                    ctx.lines_as("Berling", args!["^4d4dffShe wanted the dazzling necklace.^000000"])?;
                    ctx.next()?;
                    ctx.lines_as("Berling", args!["^4d4dffWe wanted the goddess of beauty.^000000"])?;
                    ctx.next()?;
                    ctx.lines_as("Berling", args!["^4d4dffOur happiest times were with her.^000000"])?;
                    ctx.close_window()?;
                    ctx.call(Function::DisableNpc, vec![Val::from("Berling#1")])?;
                    return Err(Stop::End);
                }
                step = Berling1Step::OnInit;
                continue 'machine;
            }
            Berling1Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Berling#1")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn berling_1(ctx: &Ctx) -> Script {
    berling_1_run(ctx, Berling1Step::Start, Vec::new()).map(|_| ())
}

pub fn berling_1_oninit(ctx: &Ctx) -> Script {
    berling_1_run(ctx, Berling1Step::OnInit, Vec::new()).map(|_| ())
}

fn brisindwarf4_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if runtime::op(&ctx.var("$god2").get()?, "<", &ctx.var("$@god_check1").get()?)?.is_true() {
        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Bah, no way out."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if runtime::op(&ctx.var("$god3").get()?, ">=", &ctx.var("$@god_check2").get()?)?.is_true() {
        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Bah, no way out."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("god_brising").get()?.number()? > 45 && ctx.var("god_brising").get()?.number()? < 50) {
        ctx.lines_as("Grer", args!["What!"])?;
        ctx.call(Function::EnableNpc, vec![Val::from("Grer#1")])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("god_brising").get()? == 45 {
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Let's see.", "Um, that song.", "What was the first line...?"],
        )?;
        ctx.next()?;
        let (input, status) = runtime::input_text(ctx, None, None)?;
        ctx.var("@dwarfsong1$").set(input)?;
        if ctx.var("@dwarfsong1$").get()? == "No jewel in the world can compare." {
            ctx.var("@point").set((ctx.var("@point").get()? + Val::from(1)))?;
        }
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                ((Val::from(" ") + ctx.var("@dwarfsong1$").get()?) + Val::from("")),
                "Then...ummm..",
                "The second line?"
            ],
        )?;
        ctx.next()?;
        let (input, status) = runtime::input_text(ctx, None, None)?;
        ctx.var("@dwarfsong2$").set(input)?;
        if ctx.var("@dwarfsong2$").get()? == "Our masterpiece made from love." {
            ctx.var("@point").set((ctx.var("@point").get()? + Val::from(1)))?;
        }
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                ((Val::from(" ") + ctx.var("@dwarfsong2$").get()?) + Val::from("")),
                "Now, what was",
                "the third line...?"
            ],
        )?;
        ctx.next()?;
        let (input, status) = runtime::input_text(ctx, None, None)?;
        ctx.var("@dwarfsong3$").set(input)?;
        if ctx.var("@dwarfsong3$").get()? == "She wanted the dazzling necklace." {
            ctx.var("@point").set((ctx.var("@point").get()? + Val::from(1)))?;
        }
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                ((Val::from(" ") + ctx.var("@dwarfsong3$").get()?) + Val::from("")),
                "Now, the fourth",
                "line after that..."
            ],
        )?;
        ctx.next()?;
        let (input, status) = runtime::input_text(ctx, None, None)?;
        ctx.var("@dwarfsong4$").set(input)?;
        if ctx.var("@dwarfsong4$").get()? == "We wanted the goddess of beauty." {
            ctx.var("@point").set((ctx.var("@point").get()? + Val::from(1)))?;
        }
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                ((Val::from(" ") + ctx.var("@dwarfsong4$").get()?) + Val::from("")),
                "Alright, now",
                "for the last line..."
            ],
        )?;
        ctx.next()?;
        let (input, status) = runtime::input_text(ctx, None, None)?;
        ctx.var("@dwarfsong5$").set(input)?;
        if ctx.var("@dwarfsong5$").get()? == "Our happiest times were with her." {
            ctx.var("@point").set((ctx.var("@point").get()? + Val::from(1)))?;
        }
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                ((Val::from(" ") + ctx.var("@dwarfsong5$").get()?) + Val::from("")),
                "Alright, let's give it a try."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                ((Val::from("") + ctx.var("@dwarfsong1$").get()?) + Val::from("")),
                ((Val::from("") + ctx.var("@dwarfsong2$").get()?) + Val::from("")),
                ((Val::from("") + ctx.var("@dwarfsong3$").get()?) + Val::from("")),
                ((Val::from("") + ctx.var("@dwarfsong4$").get()?) + Val::from("")),
                ((Val::from("") + ctx.var("@dwarfsong5$").get()?) + Val::from(""))
            ],
        )?;
        ctx.next()?;
        if ctx.var("@point").get()?.number()? > 4 {
            ctx.call(Function::EnableNpc, vec![Val::from("Grer#1")])?;
            ctx.lines_as("Grer", args!["Wha--?", "Berling", "did send you!"])?;
            ctx.var("god_brising").set(Val::from(46))?;
            ctx.call(Function::StopNpcTimer, vec![])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as("Grer", args!["Bah!", "I knew it!", "I can't trust you!"])?;
            ctx.close_window()?;
            ctx.call(Function::Warp, vec![Val::from("mjo_dun02"), Val::from(118), Val::from(56)])?;
            return Err(Stop::End);
        }
    } else {
        ctx.mes("^3355FFYou found a hole filled with some clod and a lump of coal.^000000")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn brisindwarf4(ctx: &Ctx) -> Script {
    brisindwarf4_body(ctx, Vec::new()).map(|_| ())
}

fn brisindwarf4_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if runtime::op(&ctx.var("$god2").get()?, "<", &ctx.var("$@god_check1").get()?)?.is_true() {
        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Beh, no way out."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if runtime::op(&ctx.var("$god3").get()?, ">=", &ctx.var("$@god_check2").get()?)?.is_true() {
        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Beh, no way out."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("god_brising").get()? == 44 {
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_SURPRISE")?,
                Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
            ],
        )?;
        ctx.lines_as(
            "Male Voice",
            args![
                "Don't come any closer!",
                "How the hell do I feel",
                "Freya's presense on you?",
                "Tell me who you are!"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("I am sent by Berling.:Wah, you freaked me out!")])? {
            1 => {
                ctx.lines_as("Male Voice", args!["Lies! Lies!", "Prove it!", "Prove yourself!"])?;
                ctx.var("god_brising").set(Val::from(45))?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as("Male Voice", args!["Get out of here!"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if ctx.var("god_brising").get()? == 45 {
        ctx.lines_as(
            "Male Voice",
            args![
                "Prove it!",
                "Otherwise, leave me alone and let me mine silver. Silver...! Heh heh heh..."
            ],
        )?;
        ctx.call(Function::StopNpcTimer, vec![])?;
        ctx.call(Function::StartNpcTimer, vec![])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.call(Function::Warp, vec![Val::from("mjo_dun02"), Val::from(118), Val::from(56)])?;
    }
    return Err(Stop::End);
}

pub fn brisindwarf4_ontouch(ctx: &Ctx) -> Script {
    brisindwarf4_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn brisindwarf4_ontimer1000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("mjo_dun02"),
            Val::from("Grer: Hurry and show that I can trust you!"),
            Val::from(0),
            Val::from(7396315),
        ],
    )?;
    return Err(Stop::End);
}

pub fn brisindwarf4_ontimer1000(ctx: &Ctx) -> Script {
    brisindwarf4_ontimer1000_body(ctx, Vec::new()).map(|_| ())
}

fn brisindwarf4_ontimer240000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("mjo_dun02"),
            Val::from("Grer: Grrr, I won't wait much longer!"),
            Val::from(0),
            Val::from(7396315),
        ],
    )?;
    return Err(Stop::End);
}

pub fn brisindwarf4_ontimer240000(ctx: &Ctx) -> Script {
    brisindwarf4_ontimer240000_body(ctx, Vec::new()).map(|_| ())
}

fn brisindwarf4_ontimer298000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("mjo_dun02"),
            Val::from("Grer: Farewell, human!"),
            Val::from(0),
            Val::from(7396315),
        ],
    )?;
    return Err(Stop::End);
}

pub fn brisindwarf4_ontimer298000(ctx: &Ctx) -> Script {
    brisindwarf4_ontimer298000_body(ctx, Vec::new()).map(|_| ())
}

fn brisindwarf4_ontimer300000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::Warp, vec![Val::from("mjo_dun02"), Val::from(118), Val::from(56)])?;
    ctx.call(Function::DisableNpc, vec![Val::from("Grer#1")])?;
    return Err(Stop::End);
}

pub fn brisindwarf4_ontimer300000(ctx: &Ctx) -> Script {
    brisindwarf4_ontimer300000_body(ctx, Vec::new()).map(|_| ())
}

fn brisindwarf4_ontimer301000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn brisindwarf4_ontimer301000(ctx: &Ctx) -> Script {
    brisindwarf4_ontimer301000_body(ctx, Vec::new()).map(|_| ())
}

fn grer_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("god_brising").get()?.number()? > 47 {
        ctx.lines_as(
            "Grer",
            args![
                "As we promised,",
                "Brisingamen will appear in front of human's eyes. But until then, I shall make jewelry of silver. Mwahahaha~"
            ],
        )?;
        ctx.close_window()?;
        ctx.call(Function::DisableNpc, vec![Val::from("Grer#1")])?;
        return Err(Stop::End);
    } else if ctx.var("god_brising").get()? == 46 {
        ctx.lines_as(
            "Grer",
            args![
                "My name is Grer.",
                "I should have been",
                "the last one to be awakened. Now I see that I haven't mined as much silver as I may need."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Grer",
            args!["You might think gold would better suit a goddess, but no. Silver is what makes her golden hair more beautiful."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Grer",
            args!["Now that we're revived, there's a good chance that we may see her again. I don't really mind anything else."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Grer",
            args!["I want her spirit and scent to fill this world. I don't want Odin or Heimdall to know our plan."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Grer",
            args![
                "Now, this doesn't mean",
                "that I prefer humans over",
                "gods or giants, but if it's for Freya, I'll do anything."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Grer",
            args![
                "Anyway, now my brothers",
                "have been revived. For that, I give you my thanks. Someday, you will have the power to challenge the gods."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Grer",
            args![
                "Now, I need to",
                "get back to my work.",
                "You'll need a silver ornament for the necklace, so I'll continue to mine silver."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Grer",
            args!["I will be working", "hard to make the most", "beautiful ornament", "in the world."],
        )?;
        ctx.next()?;
        ctx.lines_as("Grer", args!["Okay, you", "may go back now!", "Farewell."])?;
        ctx.var("god_brising").set(Val::from(47))?;
        ctx.close_window()?;
        ctx.call(Function::DisableNpc, vec![Val::from("Grer#1")])?;
        return Err(Stop::End);
    } else if ctx.var("god_brising").get()? == 47 {
        ctx.lines_as(
            "Grer",
            args![
                "Heh heh...",
                "Why are you still",
                "here? Your job is done.",
                "Now, go back to where",
                "you came from!"
            ],
        )?;
        ctx.close_window()?;
        ctx.call(Function::DisableNpc, vec![Val::from("Grer#1")])?;
        return Err(Stop::End);
    } else {
        ctx.lines_as("Grer", args!["Who the", "hell are you?!"])?;
        ctx.close_window()?;
        ctx.call(Function::DisableNpc, vec![Val::from("Grer#1")])?;
        return Err(Stop::End);
    }
}

pub fn grer_1(ctx: &Ctx) -> Script {
    grer_1_body(ctx, Vec::new()).map(|_| ())
}

fn grer_1_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Grer#1")])?;
    return Err(Stop::End);
}

pub fn grer_1_oninit(ctx: &Ctx) -> Script {
    grer_1_oninit_body(ctx, Vec::new()).map(|_| ())
}

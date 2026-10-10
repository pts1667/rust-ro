use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum DoppelgangerPrstStep {
    Start,
    OnTouch,
}

fn doppelganger_prst_run(ctx: &Ctx, mut step: DoppelgangerPrstStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            DoppelgangerPrstStep::Start => {
                step = DoppelgangerPrstStep::OnTouch;
                continue 'machine;
            }
            DoppelgangerPrstStep::OnTouch => {
                if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_PRIEST")?) {
                    ctx.lines_as(
                        "Doppelganger",
                        args!["What are you doing here? You've already made your choice, there's no going back... Priest."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Doppelganger", args!["Besides, this is none of your business. Whether or not this Acolyte becomes a Priest isn't up to you. Now get out of here, before I get violent."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?) {
                    ctx.lines_as(
                        "Doppelganger",
                        args![
                            "Hold on there, Acolyte.",
                            "I'm not like Deviruchi,",
                            "so I won't mince",
                            "words with you."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Doppelganger", args!["Now, why would you want to become a Priest? It's such a worthless, thankless job. If you want, I'll give you the chance to become a Novice. Then you can become something much better!"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Doppelganger",
                        args!["Of course, I'll let you redistribute your stat points by your base level. Now, isn't that a sweet deal...?"],
                    )?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("Deal, Deal!:No deal... Doppelganger.")],
                    )?) == 1
                    {
                        ctx.lines_as(
                            "Doppelganger",
                            args!["Good choice~", "I shall return your", "job to a Novice", "as you wish."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Doppelganger", args!["Now go!!", "Never step into", "the light again!"])?;
                        ctx.close_window()?;
                        ctx.call(Function::Warp, vec![Val::from("gef_dun02"), Val::from(210), Val::from(177)])?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as("Doppelganger", args!["I don't think you understood what I just offered. Think about it again. I mean, this is your one and only chance to undo your life mistakes. I mean, becoming an Acolyte?"])?;
                    ctx.next()?;
                    ctx.lines_as("Doppelganger", args!["Just don't become a Priest. I won't ask you more than once. Then you can choose a better job... perhaps a Swordman like me."])?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("I don't want to be a Priest!:I'll never listen to you!")],
                    )?) == 1
                    {
                        ctx.lines_as(
                            "Doppelganger",
                            args!["Excellent choice. Now, never return to this place. I shall return your job to Novice as you wish."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Doppelganger", args!["Now go!!", "Never step into", "the light again!"])?;
                        ctx.close_window()?;
                        ctx.call(Function::Warp, vec![Val::from("gef_dun02"), Val::from(210), Val::from(177)])?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        "Doppelganger",
                        args!["Hmpf. I admire", "your determination.", "Okay, you can pass.", "For now."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Doppelganger",
                        args![
                            "But if by chance we meet again,",
                            "I assure you... You won't be happy at all to see me."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn doppelganger_prst(ctx: &Ctx) -> Script {
    doppelganger_prst_run(ctx, DoppelgangerPrstStep::Start, Vec::new()).map(|_| ())
}

pub fn doppelganger_prst_ontouch(ctx: &Ctx) -> Script {
    doppelganger_prst_run(ctx, DoppelgangerPrstStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum DarkLordPrstStep {
    Start,
    OnTouch,
}

fn dark_lord_prst_run(ctx: &Ctx, mut step: DarkLordPrstStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            DarkLordPrstStep::Start => {
                step = DarkLordPrstStep::OnTouch;
                continue 'machine;
            }
            DarkLordPrstStep::OnTouch => {
                if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_PRIEST")?) {
                    ctx.lines_as("Dark Lord", args!["^330033All is doom, darkness and despair! Those who love you will betray you, and all that will be left is grieving and fury!^000000"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Dark Lord",
                        args![
                            "^330033To choose to become a servant of God is to choose eternal pain!",
                            "I shall personally see to that, mortal.^000000"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?) {
                    ctx.lines_as(
                        "Dark Lord",
                        args!["^330033Halt, human.", "Who has granted", "you passage?^000000"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Dark Lord", args!["^330033You still wish to become a Priest?! Fool! Then, I shall not let you pass. Go back. Otherwise, you will not survive.^000000"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Dark Lord",
                        args![
                            "^330033It would be so easy for me to snap your fragile body in twain and grind your bones to dust.",
                            "Now, go back mortal!^000000"
                        ],
                    )?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("I'm so sorry. Spare me!:God will protect me.")],
                    )?) == 1
                    {
                        ctx.lines_as("Dark Lord", args!["^330033Don't ever come back!^000000"])?;
                        ctx.close_window()?;
                        ctx.call(Function::Warp, vec![Val::from("gl_church"), Val::from(145), Val::from(170)])?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as("Dark Lord", args!["^330033It is no use to feign strength and courage. You are completely helpless before me. Your skills are laughable, and your weapons are but mere toys compared to my power.^000000"])?;
                    ctx.next()?;
                    ctx.lines_as("Dark Lord", args!["^330033With just a wave of my hand, you will cease to exist. And no one will remember you. Tremble before the might of my infinite magic!^000000"])?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("I beg you, don't...!:Begone, vile fiend!")],
                    )?) == 1
                    {
                        ctx.lines_as("Dark Lord", args!["^330033Don't ever come back!^000000"])?;
                        ctx.close_window()?;
                        ctx.call(Function::Warp, vec![Val::from("gl_church"), Val::from(145), Val::from(170)])?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        "Dark Lord",
                        args![
                            "^330033Why...",
                            "Why don't you fear me?!",
                            "For a frail mortal, you",
                            "are quite annoying.^000000"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Dark Lord", args!["^330033When next we meet, I will shall escort you to a realm of suffering where you shall spend years immersed in excruciating pain.", "Mark my words...^000000"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn dark_lord_prst(ctx: &Ctx) -> Script {
    dark_lord_prst_run(ctx, DarkLordPrstStep::Start, Vec::new()).map(|_| ())
}

pub fn dark_lord_prst_ontouch(ctx: &Ctx) -> Script {
    dark_lord_prst_run(ctx, DarkLordPrstStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum BaphometPrstStep {
    Start,
    OnTouch,
}

fn baphomet_prst_run(ctx: &Ctx, mut step: BaphometPrstStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            BaphometPrstStep::Start => {
                step = BaphometPrstStep::OnTouch;
                continue 'machine;
            }
            BaphometPrstStep::OnTouch => {
                if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_PRIEST")?) {
                    ctx.lines_as("Baphomet", args!["I hate", "Priests..."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Baphomet",
                        args!["I don't have any business with you, servant of God. Just pass through."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?) {
                    ctx.lines_as("Baphomet", args!["Greetings."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Baphomet",
                        args![((Val::from("...") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("."))],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Baphomet", args!["Yes, human,", "I know who you are."])?;
                    ctx.next()?;
                    ctx.lines_as("Baphomet", args!["I also know that Deviruchi, Doppelganger and the Dark Lord have all failed to convince you to turn away from the Priesthood.", "Now, I stand before", "you to offer a deal."])?;
                    ctx.next()?;
                    ctx.lines_as("Baphomet", args!["I can grant you any treasure you desire and infinite power at your fingertips. Powerful weapons that humans have never before seen..."])?;
                    ctx.next()?;
                    ctx.lines_as("Baphomet", args!["Mountains of zeny that you cannot possibly hope to spend in a lifetime. Though, who's to say that your lifespan should be limited? Fame, power, immortality: It can all be yours."])?;
                    ctx.next()?;
                    ctx.lines_as("Baphomet", args!["I will be yours to summon at anytime. All other humans will dread making you their enemy. You will become the most powerful person in all of history!"])?;
                    ctx.next()?;
                    ctx.lines_as("Baphomet", args!["Cease this foolishness of pursuing the Priesthood. Make a contract with me. The entire world is yours for the taking."])?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("Deal.:No, Baphomet. You lose.")])?) == 1 {
                        ctx.lines_as(
                            "Baphomet",
                            args!["Then we shall form a contract. You won't ever regret this moment..."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Baphomet",
                            args!["Follow me.", "We will make the", "contract in my", "sanctum of darkness."],
                        )?;
                        ctx.close_window()?;
                        ctx.call(Function::Warp, vec![Val::from("glast_01"), Val::from(200), Val::from(203)])?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as("Baphomet", args!["Foolish human...", "You have made your choice. I will leave you alone for now, then. However, your training won't be as easy as you think."])?;
                    ctx.next()?;
                    ctx.lines_as("Baphomet", args!["I shall be preparing my troops for you. The day will come when I shall enjoy watching you writhe in agony as my fiends slowly devour you."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn baphomet_prst(ctx: &Ctx) -> Script {
    baphomet_prst_run(ctx, BaphometPrstStep::Start, Vec::new()).map(|_| ())
}

pub fn baphomet_prst_ontouch(ctx: &Ctx) -> Script {
    baphomet_prst_run(ctx, BaphometPrstStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Prst21Step {
    Start,
    OnTouch,
}

fn prst2_1_run(ctx: &Ctx, mut step: Prst21Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Prst21Step::Start => {
                step = Prst21Step::OnTouch;
                continue 'machine;
            }
            Prst21Step::OnTouch => {
                if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_PRIEST")?) {
                    ctx.call(Function::Warp, vec![Val::from("job_prist"), Val::from(98), Val::from(40)])?;
                } else if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?) {
                    ctx.call(Function::Warp, vec![Val::from("job_prist"), Val::from(98), Val::from(40)])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Mummy_Generator::OnEnable")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn prst2_1(ctx: &Ctx) -> Script {
    prst2_1_run(ctx, Prst21Step::Start, Vec::new()).map(|_| ())
}

pub fn prst2_1_ontouch(ctx: &Ctx) -> Script {
    prst2_1_run(ctx, Prst21Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn mummy_generator(ctx: &Ctx) -> Script {
    mummy_generator_run(ctx, MummyGeneratorStep::Start, Vec::new()).map(|_| ())
}

pub fn mummy_generator_oninit(ctx: &Ctx) -> Script {
    mummy_generator_run(ctx, MummyGeneratorStep::OnInit, Vec::new()).map(|_| ())
}

pub fn mummy_generator_onenable(ctx: &Ctx) -> Script {
    mummy_generator_run(ctx, MummyGeneratorStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn mummy_generator_onm1(ctx: &Ctx) -> Script {
    mummy_generator_run(ctx, MummyGeneratorStep::Onm1, Vec::new()).map(|_| ())
}

pub fn mummy_generator_onm2(ctx: &Ctx) -> Script {
    mummy_generator_run(ctx, MummyGeneratorStep::Onm2, Vec::new()).map(|_| ())
}

pub fn mummy_generator_onm3(ctx: &Ctx) -> Script {
    mummy_generator_run(ctx, MummyGeneratorStep::Onm3, Vec::new()).map(|_| ())
}

pub fn mummy_generator_ondisable(ctx: &Ctx) -> Script {
    mummy_generator_run(ctx, MummyGeneratorStep::OnDisable, Vec::new()).map(|_| ())
}

fn mummy1_1_run(ctx: &Ctx, mut step: Mummy11Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Mummy11Step::Start => {
                step = Mummy11Step::OnInit;
                continue 'machine;
            }
            Mummy11Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Mummy1_1")])?;
                return Err(Stop::End);
            }
            Mummy11Step::OnTouch => {
                if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?) {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Mummy_Generator::Onm1")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Mummy1_1::OnDisable")])?;
                }
                return Err(Stop::End);
            }
            Mummy11Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Mummy1_1")])?;
                return Err(Stop::End);
            }
            Mummy11Step::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("Mummy1_1")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn mummy1_1(ctx: &Ctx) -> Script {
    mummy1_1_run(ctx, Mummy11Step::Start, Vec::new()).map(|_| ())
}

pub fn mummy1_1_oninit(ctx: &Ctx) -> Script {
    mummy1_1_run(ctx, Mummy11Step::OnInit, Vec::new()).map(|_| ())
}

pub fn mummy1_1_ontouch(ctx: &Ctx) -> Script {
    mummy1_1_run(ctx, Mummy11Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn mummy1_1_onenable(ctx: &Ctx) -> Script {
    mummy1_1_run(ctx, Mummy11Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn mummy1_1_ondisable(ctx: &Ctx) -> Script {
    mummy1_1_run(ctx, Mummy11Step::OnDisable, Vec::new()).map(|_| ())
}

fn mummy2_1_run(ctx: &Ctx, mut step: Mummy21Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Mummy21Step::Start => {
                step = Mummy21Step::OnInit;
                continue 'machine;
            }
            Mummy21Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Mummy2_1")])?;
                return Err(Stop::End);
            }
            Mummy21Step::OnTouch => {
                if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?) {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Mummy_Generator::Onm2")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Mummy2_1::OnDisable")])?;
                }
                return Err(Stop::End);
            }
            Mummy21Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Mummy2_1")])?;
                return Err(Stop::End);
            }
            Mummy21Step::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("Mummy2_1")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn mummy2_1(ctx: &Ctx) -> Script {
    mummy2_1_run(ctx, Mummy21Step::Start, Vec::new()).map(|_| ())
}

pub fn mummy2_1_oninit(ctx: &Ctx) -> Script {
    mummy2_1_run(ctx, Mummy21Step::OnInit, Vec::new()).map(|_| ())
}

pub fn mummy2_1_ontouch(ctx: &Ctx) -> Script {
    mummy2_1_run(ctx, Mummy21Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn mummy2_1_onenable(ctx: &Ctx) -> Script {
    mummy2_1_run(ctx, Mummy21Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn mummy2_1_ondisable(ctx: &Ctx) -> Script {
    mummy2_1_run(ctx, Mummy21Step::OnDisable, Vec::new()).map(|_| ())
}

fn mummy3_1_run(ctx: &Ctx, mut step: Mummy31Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Mummy31Step::Start => {
                step = Mummy31Step::OnInit;
                continue 'machine;
            }
            Mummy31Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Mummy3_1")])?;
                return Err(Stop::End);
            }
            Mummy31Step::OnTouch => {
                if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?) {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Mummy_Generator::Onm3")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Mummy3_1::OnDisable")])?;
                }
                return Err(Stop::End);
            }
            Mummy31Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Mummy3_1")])?;
                return Err(Stop::End);
            }
            Mummy31Step::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("Mummy3_1")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn mummy3_1(ctx: &Ctx) -> Script {
    mummy3_1_run(ctx, Mummy31Step::Start, Vec::new()).map(|_| ())
}

pub fn mummy3_1_oninit(ctx: &Ctx) -> Script {
    mummy3_1_run(ctx, Mummy31Step::OnInit, Vec::new()).map(|_| ())
}

pub fn mummy3_1_ontouch(ctx: &Ctx) -> Script {
    mummy3_1_run(ctx, Mummy31Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn mummy3_1_onenable(ctx: &Ctx) -> Script {
    mummy3_1_run(ctx, Mummy31Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn mummy3_1_ondisable(ctx: &Ctx) -> Script {
    mummy3_1_run(ctx, Mummy31Step::OnDisable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Prst31Step {
    Start,
    OnTouch,
}

fn prst3_1_run(ctx: &Ctx, mut step: Prst31Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Prst31Step::Start => {
                step = Prst31Step::OnTouch;
                continue 'machine;
            }
            Prst31Step::OnTouch => {
                if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_PRIEST")?) {
                    ctx.call(Function::Warp, vec![Val::from("prt_church"), Val::from(15), Val::from(36)])?;
                    return Err(Stop::End);
                } else if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?) {
                    ctx.var("priest_q").set(Val::from(7))?;
                    if ctx.call(Function::CheckQuest, vec![Val::from(8012)])? != -1 {
                        ctx.call(Function::ChangeQuest, vec![Val::from(8012), Val::from(8013)])?;
                    }
                    ctx.call(Function::Warp, vec![Val::from("prt_church"), Val::from(16), Val::from(37)])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Mummy_Generator::OnDisable")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn prst3_1(ctx: &Ctx) -> Script {
    prst3_1_run(ctx, Prst31Step::Start, Vec::new()).map(|_| ())
}

pub fn prst3_1_ontouch(ctx: &Ctx) -> Script {
    prst3_1_run(ctx, Prst31Step::OnTouch, Vec::new()).map(|_| ())
}

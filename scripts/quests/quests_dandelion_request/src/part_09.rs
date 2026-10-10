use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum Maogate2Talk1Step {
    Start,
    OnInit,
    OnEnter,
    OnTouch,
}

fn maogate2_talk1_run(ctx: &Ctx, mut step: Maogate2Talk1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Maogate2Talk1Step::Start => {
                step = Maogate2Talk1Step::OnInit;
                continue 'machine;
            }
            Maogate2Talk1Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("#maogate2_talk1")])?;
                return Err(Stop::End);
            }
            Maogate2Talk1Step::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("#maogate2_talk1")])?;
                return Err(Stop::End);
            }
            Maogate2Talk1Step::OnTouch => {
                ctx.lines(args![
                    "^333333Holy... ritual... of...",
                    "blood... I am proud...",
                    "to b-be... the last...",
                    "sacrifice... Heh heh heh...^000000"
                ])?;
                ctx.next()?;
                ctx.lines(args![
                    "^3355FFThe maniacal laughter",
                    "from that strange voice",
                    "echoes in your head as you",
                    "wake up in some strange place.^000000"
                ])?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "Where am I...?",
                        "What is this place?",
                        "Arrgh, my head hurts,",
                        "but I've got to stop",
                        "Satan Morocc's revival..."
                    ],
                )?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("que_job03"),
                        Val::from("...Blood... is the currency... of the soul..."),
                        Val::from(1),
                        Val::from(8087790),
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "There's that voice",
                        "again! Kidd? Valdes?",
                        "Are any of you here?",
                        "Damn, I must be all alone.",
                        "I guess I have to stop this",
                        "weird ritual all by myself!"
                    ],
                )?;
                ctx.close_window()?;
                ctx.call(Function::DisableNpc, vec![Val::from("#maogate2_talk1")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn maogate2_talk1(ctx: &Ctx) -> Script {
    maogate2_talk1_run(ctx, Maogate2Talk1Step::Start, Vec::new()).map(|_| ())
}

pub fn maogate2_talk1_oninit(ctx: &Ctx) -> Script {
    maogate2_talk1_run(ctx, Maogate2Talk1Step::OnInit, Vec::new()).map(|_| ())
}

pub fn maogate2_talk1_onenter(ctx: &Ctx) -> Script {
    maogate2_talk1_run(ctx, Maogate2Talk1Step::OnEnter, Vec::new()).map(|_| ())
}

pub fn maogate2_talk1_ontouch(ctx: &Ctx) -> Script {
    maogate2_talk1_run(ctx, Maogate2Talk1Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Maogate2Talk2Step {
    Start,
    OnStop,
    OnEnter,
    OnTouch,
}

fn maogate2_talk2_run(ctx: &Ctx, mut step: Maogate2Talk2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Maogate2Talk2Step::Start => {
                step = Maogate2Talk2Step::OnStop;
                continue 'machine;
            }
            Maogate2Talk2Step::OnStop => {
                ctx.call(Function::DisableNpc, vec![Val::from("#maogate2_talk2")])?;
                return Err(Stop::End);
            }
            Maogate2Talk2Step::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("#maogate2_talk2")])?;
                return Err(Stop::End);
            }
            Maogate2Talk2Step::OnTouch => {
                ctx.lines(args!["...", "......", "........."])?;
                ctx.next()?;
                ctx.call(Function::SoundEffect, vec![Val::from("wander_man_move.wav"), Val::from(0)])?;
                ctx.call(
                    Function::StartStatus,
                    vec![ctx.constant("SC_BLIND")?, Val::from(60000), Val::from(0)],
                )?;
                ctx.lines_as("?????", args!["...Isn't the moon", "so beautiful tonight?"])?;
                ctx.next()?;
                ctx.lines_as(
                    "?????",
                    args!["...But I wonder...", "Why are there twice as many", "guards when the moon is full?"],
                )?;
                ctx.next()?;
                ctx.lines_as("???????", args!["The moon is so red...", "Yes, it's been a year..."])?;
                ctx.next()?;
                ctx.lines(args!["......", ".........", "............"])?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["What's going on?", "Who's there? Kidd?", "Valdes? Anybody?", "What was that?"],
                )?;
                ctx.close_window()?;
                ctx.call(Function::SoundEffect, vec![Val::from("wander_man_move.wav"), Val::from(0)])?;
                ctx.call(Function::EndStatus, vec![ctx.constant("SC_ALL")?])?;
                ctx.call(Function::DisableNpc, vec![Val::from("#maogate2_talk2")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn maogate2_talk2(ctx: &Ctx) -> Script {
    maogate2_talk2_run(ctx, Maogate2Talk2Step::Start, Vec::new()).map(|_| ())
}

pub fn maogate2_talk2_onstop(ctx: &Ctx) -> Script {
    maogate2_talk2_run(ctx, Maogate2Talk2Step::OnStop, Vec::new()).map(|_| ())
}

pub fn maogate2_talk2_onenter(ctx: &Ctx) -> Script {
    maogate2_talk2_run(ctx, Maogate2Talk2Step::OnEnter, Vec::new()).map(|_| ())
}

pub fn maogate2_talk2_ontouch(ctx: &Ctx) -> Script {
    maogate2_talk2_run(ctx, Maogate2Talk2Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Maogate2Talk3Step {
    Start,
    OnStop,
    OnEnter,
    OnTouch,
}

fn maogate2_talk3_run(ctx: &Ctx, mut step: Maogate2Talk3Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Maogate2Talk3Step::Start => {
                step = Maogate2Talk3Step::OnStop;
                continue 'machine;
            }
            Maogate2Talk3Step::OnStop => {
                ctx.call(Function::DisableNpc, vec![Val::from("#maogate2_talk3")])?;
                return Err(Stop::End);
            }
            Maogate2Talk3Step::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("#maogate2_talk3")])?;
                return Err(Stop::End);
            }
            Maogate2Talk3Step::OnTouch => {
                ctx.lines(args!["......", ".........", "............"])?;
                ctx.next()?;
                ctx.call(Function::SoundEffect, vec![Val::from("wander_man_move.wav"), Val::from(0)])?;
                ctx.call(
                    Function::StartStatus,
                    vec![ctx.constant("SC_BLIND")?, Val::from(60000), Val::from(0)],
                )?;
                ctx.lines_as(
                    "?????",
                    args![
                        "So, you telling me you",
                        "don't know how this town",
                        "got its name? Well, it's",
                        "kind of a scary tale..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "?????",
                    args![
                        "An evil demon invaded our",
                        "world, summoning troops of",
                        "his minions from the darkness.",
                        "However, he was defeated and",
                        "sealed beneath this castle",
                        "by a legendary hero."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "???????",
                    args![
                        "But the hatred of Satan",
                        "Morocc has not ebbed with",
                        "time. If you look at the full",
                        "moon from here, it is colored",
                        "red with Satan Morocc's rage..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines(args!["......", ".........", "............"])?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["...I keep hearing", "things! Where the hell", "is it all coming from?!"],
                )?;
                ctx.next()?;
                ctx.lines(args!["......", ".........", "............"])?;
                ctx.close_window()?;
                ctx.call(Function::SoundEffect, vec![Val::from("wander_man_move.wav"), Val::from(0)])?;
                ctx.call(Function::EndStatus, vec![ctx.constant("SC_ALL")?])?;
                ctx.call(Function::DisableNpc, vec![Val::from("#maogate2_talk3")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn maogate2_talk3(ctx: &Ctx) -> Script {
    maogate2_talk3_run(ctx, Maogate2Talk3Step::Start, Vec::new()).map(|_| ())
}

pub fn maogate2_talk3_onstop(ctx: &Ctx) -> Script {
    maogate2_talk3_run(ctx, Maogate2Talk3Step::OnStop, Vec::new()).map(|_| ())
}

pub fn maogate2_talk3_onenter(ctx: &Ctx) -> Script {
    maogate2_talk3_run(ctx, Maogate2Talk3Step::OnEnter, Vec::new()).map(|_| ())
}

pub fn maogate2_talk3_ontouch(ctx: &Ctx) -> Script {
    maogate2_talk3_run(ctx, Maogate2Talk3Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Maogate2Talk4Step {
    Start,
    OnStop,
    OnEnter,
    OnTouch,
}

fn maogate2_talk4_run(ctx: &Ctx, mut step: Maogate2Talk4Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Maogate2Talk4Step::Start => {
                step = Maogate2Talk4Step::OnStop;
                continue 'machine;
            }
            Maogate2Talk4Step::OnStop => {
                ctx.call(Function::DisableNpc, vec![Val::from("#maogate2_talk4")])?;
                return Err(Stop::End);
            }
            Maogate2Talk4Step::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("#maogate2_talk4")])?;
                return Err(Stop::End);
            }
            Maogate2Talk4Step::OnTouch => {
                ctx.lines(args!["......", ".........", "............"])?;
                ctx.next()?;
                ctx.call(Function::SoundEffect, vec![Val::from("wander_man_move.wav"), Val::from(0)])?;
                ctx.call(
                    Function::StartStatus,
                    vec![ctx.constant("SC_BLIND")?, Val::from(60000), Val::from(0)],
                )?;
                ctx.lines_as("?????", args!["Hmpf.", "It has begun."])?;
                ctx.next()?;
                ctx.lines_as(
                    "?????",
                    args!["That smoke...?", "What exactly is going", "on over there every", "full moon?"],
                )?;
                ctx.next()?;
                ctx.lines_as("???????", args!["Well...", "Who's to say?"])?;
                ctx.next()?;
                ctx.lines_as("????", args!["B-boss...!"])?;
                ctx.next()?;
                ctx.lines(args!["......", ".........", "............"])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("que_job03"),
                        Val::from("...Moooore... Give me... more blood..."),
                        Val::from(1),
                        Val::from(14524637),
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "These voices...",
                        "They're like past",
                        "memories of something",
                        "that's happened here",
                        "in Castle Morocc..."
                    ],
                )?;
                ctx.close_window()?;
                ctx.call(Function::SoundEffect, vec![Val::from("wander_man_move.wav"), Val::from(0)])?;
                ctx.call(Function::EndStatus, vec![ctx.constant("SC_ALL")?])?;
                ctx.call(Function::DisableNpc, vec![Val::from("#maogate2_talk4")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn maogate2_talk4(ctx: &Ctx) -> Script {
    maogate2_talk4_run(ctx, Maogate2Talk4Step::Start, Vec::new()).map(|_| ())
}

pub fn maogate2_talk4_onstop(ctx: &Ctx) -> Script {
    maogate2_talk4_run(ctx, Maogate2Talk4Step::OnStop, Vec::new()).map(|_| ())
}

pub fn maogate2_talk4_onenter(ctx: &Ctx) -> Script {
    maogate2_talk4_run(ctx, Maogate2Talk4Step::OnEnter, Vec::new()).map(|_| ())
}

pub fn maogate2_talk4_ontouch(ctx: &Ctx) -> Script {
    maogate2_talk4_run(ctx, Maogate2Talk4Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Maogate2Talk5Step {
    Start,
    OnStop,
    OnEnter,
    OnTouch,
}

fn maogate2_talk5_run(ctx: &Ctx, mut step: Maogate2Talk5Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Maogate2Talk5Step::Start => {
                step = Maogate2Talk5Step::OnStop;
                continue 'machine;
            }
            Maogate2Talk5Step::OnStop => {
                ctx.call(Function::DisableNpc, vec![Val::from("#maogate2_talk5")])?;
                return Err(Stop::End);
            }
            Maogate2Talk5Step::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("#maogate2_talk5")])?;
                return Err(Stop::End);
            }
            Maogate2Talk5Step::OnTouch => {
                ctx.lines(args!["......", ".........", "............"])?;
                ctx.next()?;
                ctx.call(Function::SoundEffect, vec![Val::from("wander_man_move.wav"), Val::from(0)])?;
                ctx.call(
                    Function::StartStatus,
                    vec![ctx.constant("SC_BLIND")?, Val::from(60000), Val::from(0)],
                )?;
                ctx.lines_as(
                    "?????",
                    args![
                        "Sir, our investigation has",
                        "turned something up. Every",
                        "15 days, a carriage secretly",
                        "transports about 20 to 30",
                        "children into the castle.",
                        "However, they never come out..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "?????",
                    args![
                        "Are you serious...?",
                        "What the hell could",
                        "they be doing in there?!",
                        "Come on, let's go..."
                    ],
                )?;
                ctx.close_window()?;
                ctx.call(Function::SoundEffect, vec![Val::from("wander_man_move.wav"), Val::from(0)])?;
                ctx.call(Function::EndStatus, vec![ctx.constant("SC_ALL")?])?;
                ctx.call(Function::DisableNpc, vec![Val::from("#maogate2_talk5")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn maogate2_talk5(ctx: &Ctx) -> Script {
    maogate2_talk5_run(ctx, Maogate2Talk5Step::Start, Vec::new()).map(|_| ())
}

pub fn maogate2_talk5_onstop(ctx: &Ctx) -> Script {
    maogate2_talk5_run(ctx, Maogate2Talk5Step::OnStop, Vec::new()).map(|_| ())
}

pub fn maogate2_talk5_onenter(ctx: &Ctx) -> Script {
    maogate2_talk5_run(ctx, Maogate2Talk5Step::OnEnter, Vec::new()).map(|_| ())
}

pub fn maogate2_talk5_ontouch(ctx: &Ctx) -> Script {
    maogate2_talk5_run(ctx, Maogate2Talk5Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Maogate2Talk6Step {
    Start,
    OnStop,
    OnEnter,
    OnTouch,
}

fn maogate2_talk6_run(ctx: &Ctx, mut step: Maogate2Talk6Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Maogate2Talk6Step::Start => {
                step = Maogate2Talk6Step::OnStop;
                continue 'machine;
            }
            Maogate2Talk6Step::OnStop => {
                ctx.call(Function::DisableNpc, vec![Val::from("#maogate2_talk6")])?;
                return Err(Stop::End);
            }
            Maogate2Talk6Step::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("#maogate2_talk6")])?;
                return Err(Stop::End);
            }
            Maogate2Talk6Step::OnTouch => {
                ctx.lines(args!["......", ".........", "............"])?;
                ctx.next()?;
                ctx.call(Function::SoundEffect, vec![Val::from("wander_man_move.wav"), Val::from(0)])?;
                ctx.call(
                    Function::StartStatus,
                    vec![ctx.constant("SC_BLIND")?, Val::from(60000), Val::from(0)],
                )?;
                ctx.lines_as("?????", args!["Wh-what's with", "all of this noise?"])?;
                ctx.next()?;
                ctx.lines_as(
                    "?????",
                    args![
                        "Sorry, m'lord.",
                        "It's nothing, but",
                        "we'll take care of",
                        "it as soon as possible.",
                        "Shut up, children. Silence!"
                    ],
                )?;
                ctx.next()?;
                ctx.mes("..............")?;
                ctx.next()?;
                ctx.lines(args!["......", ".........", "............"])?;
                ctx.next()?;
                ctx.lines_as(
                    "???????",
                    args![
                        "This place is so hot...",
                        "Where's my daddy? I don't...",
                        "I don't wanna go to Thanatos..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines(args!["......", ".........", "............"])?;
                ctx.next()?;
                ctx.lines_as(
                    "??????",
                    args!["Make sure that", "nothing interferes", "with our plan. Nothing..."],
                )?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("que_job03"),
                        Val::from("..Blood... is the currency...of the soul... Six more to go..."),
                        Val::from(1),
                        Val::from(8087790),
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "????",
                    args![
                        "I don't wanna",
                        "wear these chains!",
                        "G-get them off, please!",
                        "I'm sorry, I'm sorry, I'm s--"
                    ],
                )?;
                ctx.next()?;
                ctx.lines(args!["......", ".........", "............"])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("que_job03"),
                        Val::from("...Morocc Satan... needs... fresher blood..."),
                        Val::from(1),
                        Val::from(8087790),
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "I can't...",
                        "I can't endure",
                        "listening to this",
                        "much longer. Where",
                        "the hell is Raiyan Moore?!"
                    ],
                )?;
                ctx.close_window()?;
                ctx.call(Function::SoundEffect, vec![Val::from("wander_man_move.wav"), Val::from(0)])?;
                ctx.call(Function::EndStatus, vec![ctx.constant("SC_ALL")?])?;
                ctx.call(Function::DisableNpc, vec![Val::from("#maogate2_talk6")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn maogate2_talk6(ctx: &Ctx) -> Script {
    maogate2_talk6_run(ctx, Maogate2Talk6Step::Start, Vec::new()).map(|_| ())
}

pub fn maogate2_talk6_onstop(ctx: &Ctx) -> Script {
    maogate2_talk6_run(ctx, Maogate2Talk6Step::OnStop, Vec::new()).map(|_| ())
}

pub fn maogate2_talk6_onenter(ctx: &Ctx) -> Script {
    maogate2_talk6_run(ctx, Maogate2Talk6Step::OnEnter, Vec::new()).map(|_| ())
}

pub fn maogate2_talk6_ontouch(ctx: &Ctx) -> Script {
    maogate2_talk6_run(ctx, Maogate2Talk6Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Maogate2Talk7Step {
    Start,
    OnStop,
    OnEnter,
    OnTouch,
}

fn maogate2_talk7_run(ctx: &Ctx, mut step: Maogate2Talk7Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Maogate2Talk7Step::Start => {
                step = Maogate2Talk7Step::OnStop;
                continue 'machine;
            }
            Maogate2Talk7Step::OnStop => {
                ctx.call(Function::DisableNpc, vec![Val::from("#maogate2_talk7")])?;
                return Err(Stop::End);
            }
            Maogate2Talk7Step::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("#maogate2_talk7")])?;
                return Err(Stop::End);
            }
            Maogate2Talk7Step::OnTouch => {
                ctx.lines(args!["......", ".........", "............"])?;
                ctx.next()?;
                ctx.call(Function::SoundEffect, vec![Val::from("wander_man_move.wav"), Val::from(0)])?;
                ctx.call(
                    Function::StartStatus,
                    vec![ctx.constant("SC_BLIND")?, Val::from(60000), Val::from(0)],
                )?;
                ctx.lines_as(
                    "?????",
                    args!["Jeez, what's going", "on? Where the heck", "are all the guards?"],
                )?;
                ctx.next()?;
                ctx.lines(args!["......", ".........", "............"])?;
                ctx.next()?;
                ctx.lines_as(
                    "????",
                    args![
                        "What?! Why would",
                        "anyone do something",
                        "so stupid, so foolhardy,",
                        "as revive Satan Morocc?!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("????", args!["Why...?", "You can't possibly", "understand. Now die...!"])?;
                ctx.next()?;
                ctx.lines(args!["......", ".........", "............"])?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "Where is that voice",
                        "coming from? Wait...",
                        "That's... That's happening",
                        "right now! Please... Please",
                        "don't let me be too late!"
                    ],
                )?;
                ctx.close_window()?;
                ctx.call(Function::SoundEffect, vec![Val::from("wander_man_move.wav"), Val::from(0)])?;
                ctx.call(Function::EndStatus, vec![ctx.constant("SC_ALL")?])?;
                ctx.call(Function::DisableNpc, vec![Val::from("#maogate2_talk7")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn maogate2_talk7(ctx: &Ctx) -> Script {
    maogate2_talk7_run(ctx, Maogate2Talk7Step::Start, Vec::new()).map(|_| ())
}

pub fn maogate2_talk7_onstop(ctx: &Ctx) -> Script {
    maogate2_talk7_run(ctx, Maogate2Talk7Step::OnStop, Vec::new()).map(|_| ())
}

pub fn maogate2_talk7_onenter(ctx: &Ctx) -> Script {
    maogate2_talk7_run(ctx, Maogate2Talk7Step::OnEnter, Vec::new()).map(|_| ())
}

pub fn maogate2_talk7_ontouch(ctx: &Ctx) -> Script {
    maogate2_talk7_run(ctx, Maogate2Talk7Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Maogate2Talk8Step {
    Start,
    OnStop,
    OnEnter,
    OnStop2,
    OnTouch,
    OnTimer1000,
    OnTimer4000,
    OnTimer8000,
    OnTimer12000,
    OnTimer16000,
    OnTimer20000,
    OnTimer20100,
}

fn maogate2_talk8_run(ctx: &Ctx, mut step: Maogate2Talk8Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Maogate2Talk8Step::Start => {
                step = Maogate2Talk8Step::OnStop;
                continue 'machine;
            }
            Maogate2Talk8Step::OnStop => {
                ctx.call(Function::DisableNpc, vec![Val::from("#maogate2_talk8")])?;
                return Err(Stop::End);
            }
            Maogate2Talk8Step::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("#maogate2_talk8")])?;
                return Err(Stop::End);
            }
            Maogate2Talk8Step::OnStop2 => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Maogate2Talk8Step::OnTouch => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![Val::from("#maogate2_talk8")])?;
                return Err(Stop::End);
            }
            Maogate2Talk8Step::OnTimer1000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("que_job03"),
                        Val::from("...Blood... is the currency... of the soul..."),
                        Val::from(1),
                        Val::from(8087790),
                    ],
                )?;
                return Err(Stop::End);
            }
            Maogate2Talk8Step::OnTimer4000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("que_job03"),
                        Val::from("...Offer six souls and ...Receive seven in return..."),
                        Val::from(1),
                        Val::from(8087790),
                    ],
                )?;
                return Err(Stop::End);
            }
            Maogate2Talk8Step::OnTimer8000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("que_job03"),
                        Val::from("...Blood... is the currency... of the soul..."),
                        Val::from(1),
                        Val::from(8087790),
                    ],
                )?;
                return Err(Stop::End);
            }
            Maogate2Talk8Step::OnTimer12000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("que_job03"),
                        Val::from("...In payment, I give... the most honest... the holiest... dedication..."),
                        Val::from(1),
                        Val::from(8087790),
                    ],
                )?;
                return Err(Stop::End);
            }
            Maogate2Talk8Step::OnTimer16000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("que_job03"),
                        Val::from("...Please... Let the tornado of blood, the pile of bodies... rival the mountains!"),
                        Val::from(1),
                        Val::from(8087790),
                    ],
                )?;
                return Err(Stop::End);
            }
            Maogate2Talk8Step::OnTimer20000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("que_job03"),
                        Val::from("...Ha... Ha... Muhahahahaha!"),
                        Val::from(1),
                        Val::from(8087790),
                    ],
                )?;
                return Err(Stop::End);
            }
            Maogate2Talk8Step::OnTimer20100 => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn maogate2_talk8(ctx: &Ctx) -> Script {
    maogate2_talk8_run(ctx, Maogate2Talk8Step::Start, Vec::new()).map(|_| ())
}

pub fn maogate2_talk8_onstop(ctx: &Ctx) -> Script {
    maogate2_talk8_run(ctx, Maogate2Talk8Step::OnStop, Vec::new()).map(|_| ())
}

pub fn maogate2_talk8_onenter(ctx: &Ctx) -> Script {
    maogate2_talk8_run(ctx, Maogate2Talk8Step::OnEnter, Vec::new()).map(|_| ())
}

pub fn maogate2_talk8_onstop2(ctx: &Ctx) -> Script {
    maogate2_talk8_run(ctx, Maogate2Talk8Step::OnStop2, Vec::new()).map(|_| ())
}

pub fn maogate2_talk8_ontouch(ctx: &Ctx) -> Script {
    maogate2_talk8_run(ctx, Maogate2Talk8Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn maogate2_talk8_ontimer1000(ctx: &Ctx) -> Script {
    maogate2_talk8_run(ctx, Maogate2Talk8Step::OnTimer1000, Vec::new()).map(|_| ())
}

pub fn maogate2_talk8_ontimer4000(ctx: &Ctx) -> Script {
    maogate2_talk8_run(ctx, Maogate2Talk8Step::OnTimer4000, Vec::new()).map(|_| ())
}

pub fn maogate2_talk8_ontimer8000(ctx: &Ctx) -> Script {
    maogate2_talk8_run(ctx, Maogate2Talk8Step::OnTimer8000, Vec::new()).map(|_| ())
}

pub fn maogate2_talk8_ontimer12000(ctx: &Ctx) -> Script {
    maogate2_talk8_run(ctx, Maogate2Talk8Step::OnTimer12000, Vec::new()).map(|_| ())
}

pub fn maogate2_talk8_ontimer16000(ctx: &Ctx) -> Script {
    maogate2_talk8_run(ctx, Maogate2Talk8Step::OnTimer16000, Vec::new()).map(|_| ())
}

pub fn maogate2_talk8_ontimer20000(ctx: &Ctx) -> Script {
    maogate2_talk8_run(ctx, Maogate2Talk8Step::OnTimer20000, Vec::new()).map(|_| ())
}

pub fn maogate2_talk8_ontimer20100(ctx: &Ctx) -> Script {
    maogate2_talk8_run(ctx, Maogate2Talk8Step::OnTimer20100, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Maogate2BattleStep {
    Start,
    OnStop,
    OnEnter,
    OnStop2,
    OnTouch,
    OnTimer500,
    OnTimer5000,
    OnTimer6000,
    OnTimer6500,
    OnMyMobDead,
}

fn maogate2_battle_run(ctx: &Ctx, mut step: Maogate2BattleStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Maogate2BattleStep::Start => {
                step = Maogate2BattleStep::OnStop;
                continue 'machine;
            }
            Maogate2BattleStep::OnStop => {
                ctx.call(Function::DisableNpc, vec![Val::from("#maogate2_battle")])?;
                return Err(Stop::End);
            }
            Maogate2BattleStep::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("#maogate2_battle")])?;
                return Err(Stop::End);
            }
            Maogate2BattleStep::OnStop2 => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("que_job03"), Val::from("#maogate2_battle::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Maogate2BattleStep::OnTouch => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_KEK")?,
                        Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                    ],
                )?;
                ctx.call(Function::DisableNpc, vec![Val::from("#maogate2_battle")])?;
                return Err(Stop::End);
            }
            Maogate2BattleStep::OnTimer500 => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_job03"),
                        Val::from(71),
                        Val::from(85),
                        Val::from("Mummy"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#maogate2_battle::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_job03"),
                        Val::from(79),
                        Val::from(85),
                        Val::from("Mummy"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#maogate2_battle::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_job03"),
                        Val::from(71),
                        Val::from(80),
                        Val::from("Mummy"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#maogate2_battle::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_job03"),
                        Val::from(79),
                        Val::from(80),
                        Val::from("Mummy"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#maogate2_battle::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Maogate2BattleStep::OnTimer5000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("que_job03"), Val::from("Watch out!"), Val::from(1), Val::from(9498256)],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion Member#2_bt::OnEnter")])?;
                return Err(Stop::End);
            }
            Maogate2BattleStep::OnTimer6000 => {
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_LORD")?])?;
                return Err(Stop::End);
            }
            Maogate2BattleStep::OnTimer6500 => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("que_job03"), Val::from("#maogate2_battle::OnMyMobDead")],
                )?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Maogate2BattleStep::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn maogate2_battle(ctx: &Ctx) -> Script {
    maogate2_battle_run(ctx, Maogate2BattleStep::Start, Vec::new()).map(|_| ())
}

pub fn maogate2_battle_onstop(ctx: &Ctx) -> Script {
    maogate2_battle_run(ctx, Maogate2BattleStep::OnStop, Vec::new()).map(|_| ())
}

pub fn maogate2_battle_onenter(ctx: &Ctx) -> Script {
    maogate2_battle_run(ctx, Maogate2BattleStep::OnEnter, Vec::new()).map(|_| ())
}

pub fn maogate2_battle_onstop2(ctx: &Ctx) -> Script {
    maogate2_battle_run(ctx, Maogate2BattleStep::OnStop2, Vec::new()).map(|_| ())
}

pub fn maogate2_battle_ontouch(ctx: &Ctx) -> Script {
    maogate2_battle_run(ctx, Maogate2BattleStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn maogate2_battle_ontimer500(ctx: &Ctx) -> Script {
    maogate2_battle_run(ctx, Maogate2BattleStep::OnTimer500, Vec::new()).map(|_| ())
}

pub fn maogate2_battle_ontimer5000(ctx: &Ctx) -> Script {
    maogate2_battle_run(ctx, Maogate2BattleStep::OnTimer5000, Vec::new()).map(|_| ())
}

pub fn maogate2_battle_ontimer6000(ctx: &Ctx) -> Script {
    maogate2_battle_run(ctx, Maogate2BattleStep::OnTimer6000, Vec::new()).map(|_| ())
}

pub fn maogate2_battle_ontimer6500(ctx: &Ctx) -> Script {
    maogate2_battle_run(ctx, Maogate2BattleStep::OnTimer6500, Vec::new()).map(|_| ())
}

pub fn maogate2_battle_onmymobdead(ctx: &Ctx) -> Script {
    maogate2_battle_run(ctx, Maogate2BattleStep::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum DandelionMember2BtStep {
    Start,
    OnInit,
    OnEnter,
}

fn dandelion_member_2_bt_run(ctx: &Ctx, mut step: DandelionMember2BtStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            DandelionMember2BtStep::Start => {
                if (ctx.var("mao_request").get()? == 124 || ctx.var("mao_request").get()? == 125) {
                    ctx.lines_as(
                        "Dandelion Member",
                        args!["Hey, are you alright?", "That was a pretty huge", "explosion just now!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Mr. R...?",
                            "Oh. You're not him.",
                            "Thanks, I think you",
                            "just saved my life. Hey...",
                            "Have you seen a female",
                            "Assassin around here?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Dandelion Member",
                        args![
                            "No, I don't think",
                            "so, but there's another",
                            "Assassin, Kidd, who's just",
                            "in the area. Wait, were you the",
                            "one who was duped by Raiyan",
                            "Moore? He's your client, right?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Dandelion Member",
                        args![
                            "I'm a member of the",
                            "Dandelion Organization,",
                            "and we've been trying to find",
                            "him. It turns out that we were",
                            "right about Moore: he's really planning to revive Satan Morocc."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Huh? What are you",
                            "talking about? Wait,",
                            "Raiyan Moore? Do you",
                            "mean Mr. R. Moore?",
                            "No! Then this means..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Dandelion Member",
                        args![
                            "Right. He managed to",
                            "trick all of us. Now, if you'd",
                            "excuse me, I better go help",
                            "Kidd now that you're awake."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Lin's Voice", args!["...S-stop...", "...This is...", "This is all insane!"])?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "That's... That's Lin's",
                            "voice! I better walk",
                            "through this darkness",
                            "and try to find her as",
                            "soon as I can!"
                        ],
                    )?;
                    ctx.close_window()?;
                    ctx.var("mao_request").set(Val::from(125))?;
                    ctx.call(Function::DisableNpc, vec![Val::from("Dandelion Member#2_bt")])?;
                    return Err(Stop::End);
                }
                step = DandelionMember2BtStep::OnInit;
                continue 'machine;
            }
            DandelionMember2BtStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Dandelion Member#2_bt")])?;
                return Err(Stop::End);
            }
            DandelionMember2BtStep::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("Dandelion Member#2_bt")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn dandelion_member_2_bt(ctx: &Ctx) -> Script {
    dandelion_member_2_bt_run(ctx, DandelionMember2BtStep::Start, Vec::new()).map(|_| ())
}

pub fn dandelion_member_2_bt_oninit(ctx: &Ctx) -> Script {
    dandelion_member_2_bt_run(ctx, DandelionMember2BtStep::OnInit, Vec::new()).map(|_| ())
}

pub fn dandelion_member_2_bt_onenter(ctx: &Ctx) -> Script {
    dandelion_member_2_bt_run(ctx, DandelionMember2BtStep::OnEnter, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Maogate2SettingStep {
    Start,
    OnStop,
    OnEnter,
    OnTouch,
}

fn maogate2_setting_run(ctx: &Ctx, mut step: Maogate2SettingStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Maogate2SettingStep::Start => {
                step = Maogate2SettingStep::OnStop;
                continue 'machine;
            }
            Maogate2SettingStep::OnStop => {
                ctx.call(Function::DisableNpc, vec![Val::from("#maogate2_setting")])?;
                return Err(Stop::End);
            }
            Maogate2SettingStep::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("#maogate2_setting")])?;
                return Err(Stop::End);
            }
            Maogate2SettingStep::OnTouch => {
                if ctx.var("mao_request").get()? == 125 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate2_end::OnEnter")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Kidd#maogate2_1::OnEnter")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Lin#maogate2_1::OnEnter")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("R#maogate2::OnEnter")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion#maogate2_1::OnEnter")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion#maogate2_2::OnEnter")])?;
                    return Err(Stop::End);
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn maogate2_setting(ctx: &Ctx) -> Script {
    maogate2_setting_run(ctx, Maogate2SettingStep::Start, Vec::new()).map(|_| ())
}

pub fn maogate2_setting_onstop(ctx: &Ctx) -> Script {
    maogate2_setting_run(ctx, Maogate2SettingStep::OnStop, Vec::new()).map(|_| ())
}

pub fn maogate2_setting_onenter(ctx: &Ctx) -> Script {
    maogate2_setting_run(ctx, Maogate2SettingStep::OnEnter, Vec::new()).map(|_| ())
}

pub fn maogate2_setting_ontouch(ctx: &Ctx) -> Script {
    maogate2_setting_run(ctx, Maogate2SettingStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Maogate2EndStep {
    Start,
    OnInit,
    OnReset,
    OnStop,
    OnEnter,
    OnTouch,
    OnTimer500,
    OnTimer3000,
    OnTimer4000,
    OnTimer5000,
    OnTimer5500,
    OnTimer7000,
    OnTimer10000,
    OnMyMobDead,
}

fn maogate2_end_run(ctx: &Ctx, mut step: Maogate2EndStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Maogate2EndStep::Start => {
                step = Maogate2EndStep::OnInit;
                continue 'machine;
            }
            Maogate2EndStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("#maogate2_end")])?;
                return Err(Stop::End);
            }
            Maogate2EndStep::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("que_job03"), Val::from("#maogate2_end::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Maogate2EndStep::OnStop => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("que_job03"), Val::from("#maogate2_end::OnMyMobDead")],
                )?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Maogate2EndStep::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("#maogate2_end")])?;
                return Err(Stop::End);
            }
            Maogate2EndStep::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate2_setting::OnStop")])?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "Hey, what's going",
                        "on here? Kidd! Lin?",
                        "A-and who's that guy?",
                        "Could it possibly be..."
                    ],
                )?;
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_SURPRISE")?,
                        Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                    ],
                )?;
                ctx.next()?;
                ctx.call(Function::Cutin, vec![Val::from("mocseal_kid01.bmp"), Val::from(0)])?;
                ctx.lines_as(
                    "Kidd",
                    args![
                        ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("...")),
                        "Stay back and don't come",
                        "near me! Lin, why are you",
                        "doing this! Give up your",
                        "mission, you don't know",
                        "who your client really is!"
                    ],
                )?;
                ctx.next()?;
                ctx.call(Function::Cutin, vec![Val::from("mocseal_rin01.bmp"), Val::from(2)])?;
                ctx.lines_as(
                    "Lin",
                    args![
                        "What are you talking",
                        "about, Kidd? I just found",
                        "out that the people you're",
                        "working for, the Dandelion",
                        "Organization, have been",
                        "hunting Mr. Moore down..."
                    ],
                )?;
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_KEK")?,
                        ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Lin#maogate2_1")])?,
                    ],
                )?;
                ctx.next()?;
                ctx.call(Function::Cutin, vec![Val::from("mocseal_kid01.bmp"), Val::from(0)])?;
                ctx.lines_as(
                    "Kidd",
                    args![
                        "Yes, of course they're",
                        "hunting him down. Your client,",
                        "Mr. R. Moore, is none other",
                        "than Raiyan Moore-- and he",
                        "plans to revive Satan Morocc!"
                    ],
                )?;
                ctx.next()?;
                ctx.call(Function::Cutin, vec![Val::from("mocseal_rin01.bmp"), Val::from(2)])?;
                ctx.lines_as(
                    "Lin",
                    args![
                        "What?! But how can that",
                        "be? I've been with Moore",
                        "for a few days! If he's the one",
                        "who kidnapped the children,",
                        "then who is taking them hostage? "
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Lin",
                    args![
                        "No, you should tell",
                        "me everything you know",
                        "about your own client,",
                        "the Dandelion Organization..."
                    ],
                )?;
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_SCRATCH")?,
                        ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("R#maogate2")])?,
                    ],
                )?;
                ctx.next()?;
                ctx.call(Function::Cutin, vec![Val::from("mocseal_rin01.bmp"), Val::from(255)])?;
                ctx.call(Function::Cutin, vec![Val::from("mocseal_kid01.bmp"), Val::from(255)])?;
                ctx.lines_as(
                    "R. Moore",
                    args![
                        "Thanks for your",
                        "concern, Lin. But really...",
                        "It's unnecessary. Heh heh...",
                        "I don't need to worry about the Dandelion Organization any longer."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "R. Moore",
                    args![
                        "Prepare yourself",
                        "and be accepted into",
                        "this sacrifice... Lin...",
                        "Your blood will be used",
                        "to revive Morocc Satan!"
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("R#maogate2::OnSpell")])?;
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_SURPRISE")?,
                        ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Kidd#maogate2_1")])?,
                    ],
                )?;
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_SURPRISE")?,
                        Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Kidd", args!["Watch out!"])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Kidd#maogate2_1::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Lin#maogate2_1::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Kidd#maogate2_2::OnEnter")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Lin#maogate2_2::OnEnter")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate2_end2::OnSpell")])?;
                ctx.next()?;
                ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Kidd! Lin!"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Lin",
                    args![
                        "Wh-why...?",
                        "Mr. Moore, you said",
                        "that you were trying to",
                        "stop Satan Morocc's revival..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Kidd", args!["Can't you see...?", "He was lying! He", "tricked all of us!"])?;
                ctx.next()?;
                ctx.lines_as(
                    "R. Moore",
                    args![
                        "So...",
                        "You're resisting?",
                        "Fine, you leave me",
                        "no choice. Feel the",
                        "power of Morocc Satan!"
                    ],
                )?;
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_KIK")?,
                        ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("R#maogate2")])?,
                    ],
                )?;
                ctx.next()?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("R#maogate2::OnInit")])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_job03"),
                        Val::from(121),
                        Val::from(118),
                        Val::from("Mummy"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#maogate2_end::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_job03"),
                        Val::from(121),
                        Val::from(115),
                        Val::from("Mummy"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#maogate2_end::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_job03"),
                        Val::from(121),
                        Val::from(112),
                        Val::from("Mummy"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#maogate2_end::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_job03"),
                        Val::from(107),
                        Val::from(106),
                        Val::from("Mummy"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#maogate2_end::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_job03"),
                        Val::from(111),
                        Val::from(106),
                        Val::from("Mummy"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#maogate2_end::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_job03"),
                        Val::from(115),
                        Val::from(106),
                        Val::from("Mummy"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#maogate2_end::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_job03"),
                        Val::from(135),
                        Val::from(105),
                        Val::from("Mummy"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#maogate2_end::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_job03"),
                        Val::from(132),
                        Val::from(105),
                        Val::from("Mummy"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#maogate2_end::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_job03"),
                        Val::from(128),
                        Val::from(105),
                        Val::from("Mummy"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#maogate2_end::OnMyMobDead"),
                    ],
                )?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["Bastard, what", "are you doing?", "Holy crap, m-monsters!"],
                )?;
                ctx.next()?;
                ctx.lines_as("Lin", args!["Damn it!", "Let me go!", "I'm gonna chase", "Moore myself!"])?;
                ctx.var("mao_request").set(Val::from(126))?;
                ctx.var("$maoattack").set((ctx.var("$maoattack").get()? + Val::from(1)))?;
                ctx.close_window()?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Lin#maogate2_2::OnInit")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("#maogate2_end")])?;
                return Err(Stop::End);
            }
            Maogate2EndStep::OnTimer500 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("que_job03"),
                        Val::from("I'll back you up!"),
                        Val::from(1),
                        Val::from(9498256),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion#maogate2_3::OnEnter")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion#maogate2_4::OnEnter")])?;
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_GO")?,
                        ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Dandelion#maogate2_1")])?,
                    ],
                )?;
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_GO")?,
                        ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Dandelion#maogate2_2")])?,
                    ],
                )?;
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_GO")?,
                        ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Dandelion#maogate2_3")])?,
                    ],
                )?;
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_GO")?,
                        ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Dandelion#maogate2_4")])?,
                    ],
                )?;
                return Err(Stop::End);
            }
            Maogate2EndStep::OnTimer3000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion#maogate2_3::OnSpell")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion#maogate2_2::OnSpell")])?;
                return Err(Stop::End);
            }
            Maogate2EndStep::OnTimer4000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion#maogate2_4::OnSpell")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion#maogate2_2::OnSpell2")])?;
                return Err(Stop::End);
            }
            Maogate2EndStep::OnTimer5000 => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("que_job03"), Val::from("#maogate2_end::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Maogate2EndStep::OnTimer5500 => {
                ctx.var("$@maogate2mobs").set(Val::from(3))?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_job03"),
                        Val::from(128),
                        Val::from(105),
                        Val::from("Mummy"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("Kidd#maogate2_2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_job03"),
                        Val::from(115),
                        Val::from(106),
                        Val::from("Mummy"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("Kidd#maogate2_2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_job03"),
                        Val::from(121),
                        Val::from(112),
                        Val::from("Mummy"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("Kidd#maogate2_2::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Maogate2EndStep::OnTimer7000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("que_job03"),
                        Val::from("Dandelions, don't let Raiyan Moore escape!"),
                        Val::from(1),
                        Val::from(9498256),
                    ],
                )?;
                return Err(Stop::End);
            }
            Maogate2EndStep::OnTimer10000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("que_job03"),
                        Val::from("Kidd, we're leaving the rest to you...!"),
                        Val::from(1),
                        Val::from(9498256),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion#maogate2_4::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion#maogate2_3::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion#maogate2_2::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion#maogate2_1::OnInit")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Maogate2EndStep::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn maogate2_end(ctx: &Ctx) -> Script {
    maogate2_end_run(ctx, Maogate2EndStep::Start, Vec::new()).map(|_| ())
}

pub fn maogate2_end_oninit(ctx: &Ctx) -> Script {
    maogate2_end_run(ctx, Maogate2EndStep::OnInit, Vec::new()).map(|_| ())
}

pub fn maogate2_end_onreset(ctx: &Ctx) -> Script {
    maogate2_end_run(ctx, Maogate2EndStep::OnReset, Vec::new()).map(|_| ())
}

pub fn maogate2_end_onstop(ctx: &Ctx) -> Script {
    maogate2_end_run(ctx, Maogate2EndStep::OnStop, Vec::new()).map(|_| ())
}

pub fn maogate2_end_onenter(ctx: &Ctx) -> Script {
    maogate2_end_run(ctx, Maogate2EndStep::OnEnter, Vec::new()).map(|_| ())
}

pub fn maogate2_end_ontouch(ctx: &Ctx) -> Script {
    maogate2_end_run(ctx, Maogate2EndStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn maogate2_end_ontimer500(ctx: &Ctx) -> Script {
    maogate2_end_run(ctx, Maogate2EndStep::OnTimer500, Vec::new()).map(|_| ())
}

pub fn maogate2_end_ontimer3000(ctx: &Ctx) -> Script {
    maogate2_end_run(ctx, Maogate2EndStep::OnTimer3000, Vec::new()).map(|_| ())
}

pub fn maogate2_end_ontimer4000(ctx: &Ctx) -> Script {
    maogate2_end_run(ctx, Maogate2EndStep::OnTimer4000, Vec::new()).map(|_| ())
}

pub fn maogate2_end_ontimer5000(ctx: &Ctx) -> Script {
    maogate2_end_run(ctx, Maogate2EndStep::OnTimer5000, Vec::new()).map(|_| ())
}

pub fn maogate2_end_ontimer5500(ctx: &Ctx) -> Script {
    maogate2_end_run(ctx, Maogate2EndStep::OnTimer5500, Vec::new()).map(|_| ())
}

pub fn maogate2_end_ontimer7000(ctx: &Ctx) -> Script {
    maogate2_end_run(ctx, Maogate2EndStep::OnTimer7000, Vec::new()).map(|_| ())
}

pub fn maogate2_end_ontimer10000(ctx: &Ctx) -> Script {
    maogate2_end_run(ctx, Maogate2EndStep::OnTimer10000, Vec::new()).map(|_| ())
}

pub fn maogate2_end_onmymobdead(ctx: &Ctx) -> Script {
    maogate2_end_run(ctx, Maogate2EndStep::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum KiddMaogate21Step {
    Start,
    OnInit,
    OnEnter,
}

fn kidd_maogate2_1_run(ctx: &Ctx, mut step: KiddMaogate21Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            KiddMaogate21Step::Start => {
                step = KiddMaogate21Step::OnInit;
                continue 'machine;
            }
            KiddMaogate21Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Kidd#maogate2_1")])?;
                return Err(Stop::End);
            }
            KiddMaogate21Step::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("Kidd#maogate2_1")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn kidd_maogate2_1(ctx: &Ctx) -> Script {
    kidd_maogate2_1_run(ctx, KiddMaogate21Step::Start, Vec::new()).map(|_| ())
}

pub fn kidd_maogate2_1_oninit(ctx: &Ctx) -> Script {
    kidd_maogate2_1_run(ctx, KiddMaogate21Step::OnInit, Vec::new()).map(|_| ())
}

pub fn kidd_maogate2_1_onenter(ctx: &Ctx) -> Script {
    kidd_maogate2_1_run(ctx, KiddMaogate21Step::OnEnter, Vec::new()).map(|_| ())
}

fn kidd_maogate2_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn kidd_maogate2_2(ctx: &Ctx) -> Script {
    kidd_maogate2_2_body(ctx, Vec::new()).map(|_| ())
}

fn kidd_maogate2_2_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Kidd#maogate2_2")])?;
    return Err(Stop::End);
}

pub fn kidd_maogate2_2_oninit(ctx: &Ctx) -> Script {
    kidd_maogate2_2_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn kidd_maogate2_2_onenter_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("Kidd#maogate2_2")])?;
    return Err(Stop::End);
}

pub fn kidd_maogate2_2_onenter(ctx: &Ctx) -> Script {
    kidd_maogate2_2_onenter_body(ctx, Vec::new()).map(|_| ())
}

fn kidd_maogate2_2_onmymobdead_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.var("$@maogate2mobs")
        .set((ctx.var("$@maogate2mobs").get()?.try_sub(Val::from(1))?))?;
    if ctx.var("$@maogate2mobs").get()?.number()? < 1 {
        ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate2_end2::OnEnter")])?;
    }
    return Err(Stop::End);
}

pub fn kidd_maogate2_2_onmymobdead(ctx: &Ctx) -> Script {
    kidd_maogate2_2_onmymobdead_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum LinMaogate21Step {
    Start,
    OnInit,
    OnEnter,
}

fn lin_maogate2_1_run(ctx: &Ctx, mut step: LinMaogate21Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            LinMaogate21Step::Start => {
                step = LinMaogate21Step::OnInit;
                continue 'machine;
            }
            LinMaogate21Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Lin#maogate2_1")])?;
                return Err(Stop::End);
            }
            LinMaogate21Step::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("Lin#maogate2_1")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn lin_maogate2_1(ctx: &Ctx) -> Script {
    lin_maogate2_1_run(ctx, LinMaogate21Step::Start, Vec::new()).map(|_| ())
}

pub fn lin_maogate2_1_oninit(ctx: &Ctx) -> Script {
    lin_maogate2_1_run(ctx, LinMaogate21Step::OnInit, Vec::new()).map(|_| ())
}

pub fn lin_maogate2_1_onenter(ctx: &Ctx) -> Script {
    lin_maogate2_1_run(ctx, LinMaogate21Step::OnEnter, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum LinMaogate22Step {
    Start,
    OnInit,
    OnEnter,
}

fn lin_maogate2_2_run(ctx: &Ctx, mut step: LinMaogate22Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            LinMaogate22Step::Start => {
                step = LinMaogate22Step::OnInit;
                continue 'machine;
            }
            LinMaogate22Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Lin#maogate2_2")])?;
                return Err(Stop::End);
            }
            LinMaogate22Step::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("Lin#maogate2_2")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn lin_maogate2_2(ctx: &Ctx) -> Script {
    lin_maogate2_2_run(ctx, LinMaogate22Step::Start, Vec::new()).map(|_| ())
}

pub fn lin_maogate2_2_oninit(ctx: &Ctx) -> Script {
    lin_maogate2_2_run(ctx, LinMaogate22Step::OnInit, Vec::new()).map(|_| ())
}

pub fn lin_maogate2_2_onenter(ctx: &Ctx) -> Script {
    lin_maogate2_2_run(ctx, LinMaogate22Step::OnEnter, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum RMaogate2Step {
    Start,
    OnInit,
    OnEnter,
    OnSpell,
}

fn r_maogate2_run(ctx: &Ctx, mut step: RMaogate2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            RMaogate2Step::Start => {
                step = RMaogate2Step::OnInit;
                continue 'machine;
            }
            RMaogate2Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("R#maogate2")])?;
                return Err(Stop::End);
            }
            RMaogate2Step::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("R#maogate2")])?;
                return Err(Stop::End);
            }
            RMaogate2Step::OnSpell => {
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL2")?])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn r_maogate2(ctx: &Ctx) -> Script {
    r_maogate2_run(ctx, RMaogate2Step::Start, Vec::new()).map(|_| ())
}

pub fn r_maogate2_oninit(ctx: &Ctx) -> Script {
    r_maogate2_run(ctx, RMaogate2Step::OnInit, Vec::new()).map(|_| ())
}

pub fn r_maogate2_onenter(ctx: &Ctx) -> Script {
    r_maogate2_run(ctx, RMaogate2Step::OnEnter, Vec::new()).map(|_| ())
}

pub fn r_maogate2_onspell(ctx: &Ctx) -> Script {
    r_maogate2_run(ctx, RMaogate2Step::OnSpell, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum DandelionMaogate21Step {
    Start,
    OnInit,
    OnEnter,
}

fn dandelion_maogate2_1_run(ctx: &Ctx, mut step: DandelionMaogate21Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            DandelionMaogate21Step::Start => {
                step = DandelionMaogate21Step::OnInit;
                continue 'machine;
            }
            DandelionMaogate21Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Dandelion#maogate2_1")])?;
                return Err(Stop::End);
            }
            DandelionMaogate21Step::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("Dandelion#maogate2_1")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn dandelion_maogate2_1(ctx: &Ctx) -> Script {
    dandelion_maogate2_1_run(ctx, DandelionMaogate21Step::Start, Vec::new()).map(|_| ())
}

pub fn dandelion_maogate2_1_oninit(ctx: &Ctx) -> Script {
    dandelion_maogate2_1_run(ctx, DandelionMaogate21Step::OnInit, Vec::new()).map(|_| ())
}

pub fn dandelion_maogate2_1_onenter(ctx: &Ctx) -> Script {
    dandelion_maogate2_1_run(ctx, DandelionMaogate21Step::OnEnter, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum DandelionMaogate22Step {
    Start,
    OnInit,
    OnEnter,
    OnSpell,
    OnSpell2,
}

fn dandelion_maogate2_2_run(ctx: &Ctx, mut step: DandelionMaogate22Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            DandelionMaogate22Step::Start => {
                step = DandelionMaogate22Step::OnInit;
                continue 'machine;
            }
            DandelionMaogate22Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Dandelion#maogate2_2")])?;
                return Err(Stop::End);
            }
            DandelionMaogate22Step::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("Dandelion#maogate2_2")])?;
                return Err(Stop::End);
            }
            DandelionMaogate22Step::OnSpell => {
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL2")?])?;
                return Err(Stop::End);
            }
            DandelionMaogate22Step::OnSpell2 => {
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_LORD")?])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn dandelion_maogate2_2(ctx: &Ctx) -> Script {
    dandelion_maogate2_2_run(ctx, DandelionMaogate22Step::Start, Vec::new()).map(|_| ())
}

pub fn dandelion_maogate2_2_oninit(ctx: &Ctx) -> Script {
    dandelion_maogate2_2_run(ctx, DandelionMaogate22Step::OnInit, Vec::new()).map(|_| ())
}

pub fn dandelion_maogate2_2_onenter(ctx: &Ctx) -> Script {
    dandelion_maogate2_2_run(ctx, DandelionMaogate22Step::OnEnter, Vec::new()).map(|_| ())
}

pub fn dandelion_maogate2_2_onspell(ctx: &Ctx) -> Script {
    dandelion_maogate2_2_run(ctx, DandelionMaogate22Step::OnSpell, Vec::new()).map(|_| ())
}

pub fn dandelion_maogate2_2_onspell2(ctx: &Ctx) -> Script {
    dandelion_maogate2_2_run(ctx, DandelionMaogate22Step::OnSpell2, Vec::new()).map(|_| ())
}

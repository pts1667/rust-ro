use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum Maogate1Talk5Step {
    Start,
    OnStop,
    OnEnter,
    OnTouch,
}

fn maogate1_talk5_run(ctx: &Ctx, mut step: Maogate1Talk5Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Maogate1Talk5Step::Start => {
                step = Maogate1Talk5Step::OnStop;
                continue 'machine;
            }
            Maogate1Talk5Step::OnStop => {
                ctx.call(Function::DisableNpc, vec![Val::from("#maogate1_talk5")])?;
                return Err(Stop::End);
            }
            Maogate1Talk5Step::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("#maogate1_talk5")])?;
                return Err(Stop::End);
            }
            Maogate1Talk5Step::OnTouch => {
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
                ctx.call(Function::DisableNpc, vec![Val::from("#maogate1_talk5")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn maogate1_talk5(ctx: &Ctx) -> Script {
    maogate1_talk5_run(ctx, Maogate1Talk5Step::Start, Vec::new()).map(|_| ())
}

pub fn maogate1_talk5_onstop(ctx: &Ctx) -> Script {
    maogate1_talk5_run(ctx, Maogate1Talk5Step::OnStop, Vec::new()).map(|_| ())
}

pub fn maogate1_talk5_onenter(ctx: &Ctx) -> Script {
    maogate1_talk5_run(ctx, Maogate1Talk5Step::OnEnter, Vec::new()).map(|_| ())
}

pub fn maogate1_talk5_ontouch(ctx: &Ctx) -> Script {
    maogate1_talk5_run(ctx, Maogate1Talk5Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Maogate1Talk6Step {
    Start,
    OnStop,
    OnEnter,
    OnTouch,
}

fn maogate1_talk6_run(ctx: &Ctx, mut step: Maogate1Talk6Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Maogate1Talk6Step::Start => {
                step = Maogate1Talk6Step::OnStop;
                continue 'machine;
            }
            Maogate1Talk6Step::OnStop => {
                ctx.call(Function::DisableNpc, vec![Val::from("#maogate1_talk6")])?;
                return Err(Stop::End);
            }
            Maogate1Talk6Step::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("#maogate1_talk6")])?;
                return Err(Stop::End);
            }
            Maogate1Talk6Step::OnTouch => {
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
                        Val::from("que_job02"),
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
                        Val::from("que_job02"),
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
                ctx.call(Function::DisableNpc, vec![Val::from("#maogate1_talk6")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn maogate1_talk6(ctx: &Ctx) -> Script {
    maogate1_talk6_run(ctx, Maogate1Talk6Step::Start, Vec::new()).map(|_| ())
}

pub fn maogate1_talk6_onstop(ctx: &Ctx) -> Script {
    maogate1_talk6_run(ctx, Maogate1Talk6Step::OnStop, Vec::new()).map(|_| ())
}

pub fn maogate1_talk6_onenter(ctx: &Ctx) -> Script {
    maogate1_talk6_run(ctx, Maogate1Talk6Step::OnEnter, Vec::new()).map(|_| ())
}

pub fn maogate1_talk6_ontouch(ctx: &Ctx) -> Script {
    maogate1_talk6_run(ctx, Maogate1Talk6Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Maogate1Talk7Step {
    Start,
    OnStop,
    OnEnter,
    OnTouch,
}

fn maogate1_talk7_run(ctx: &Ctx, mut step: Maogate1Talk7Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Maogate1Talk7Step::Start => {
                step = Maogate1Talk7Step::OnStop;
                continue 'machine;
            }
            Maogate1Talk7Step::OnStop => {
                ctx.call(Function::DisableNpc, vec![Val::from("#maogate1_talk7")])?;
                return Err(Stop::End);
            }
            Maogate1Talk7Step::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("#maogate1_talk7")])?;
                return Err(Stop::End);
            }
            Maogate1Talk7Step::OnTouch => {
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
                ctx.call(Function::DisableNpc, vec![Val::from("#maogate1_talk7")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn maogate1_talk7(ctx: &Ctx) -> Script {
    maogate1_talk7_run(ctx, Maogate1Talk7Step::Start, Vec::new()).map(|_| ())
}

pub fn maogate1_talk7_onstop(ctx: &Ctx) -> Script {
    maogate1_talk7_run(ctx, Maogate1Talk7Step::OnStop, Vec::new()).map(|_| ())
}

pub fn maogate1_talk7_onenter(ctx: &Ctx) -> Script {
    maogate1_talk7_run(ctx, Maogate1Talk7Step::OnEnter, Vec::new()).map(|_| ())
}

pub fn maogate1_talk7_ontouch(ctx: &Ctx) -> Script {
    maogate1_talk7_run(ctx, Maogate1Talk7Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Maogate1Talk8Step {
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

fn maogate1_talk8_run(ctx: &Ctx, mut step: Maogate1Talk8Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Maogate1Talk8Step::Start => {
                step = Maogate1Talk8Step::OnStop;
                continue 'machine;
            }
            Maogate1Talk8Step::OnStop => {
                ctx.call(Function::DisableNpc, vec![Val::from("#maogate1_talk8")])?;
                return Err(Stop::End);
            }
            Maogate1Talk8Step::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("#maogate1_talk8")])?;
                return Err(Stop::End);
            }
            Maogate1Talk8Step::OnStop2 => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Maogate1Talk8Step::OnTouch => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![Val::from("#maogate1_talk8")])?;
                return Err(Stop::End);
            }
            Maogate1Talk8Step::OnTimer1000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("que_job02"),
                        Val::from("...Blood... is the currency... of the soul..."),
                        Val::from(1),
                        Val::from(8087790),
                    ],
                )?;
                return Err(Stop::End);
            }
            Maogate1Talk8Step::OnTimer4000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("que_job02"),
                        Val::from("...Offer six souls and ...Receive seven in return..."),
                        Val::from(1),
                        Val::from(8087790),
                    ],
                )?;
                return Err(Stop::End);
            }
            Maogate1Talk8Step::OnTimer8000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("que_job02"),
                        Val::from("...Blood... is the currency... of the soul..."),
                        Val::from(1),
                        Val::from(8087790),
                    ],
                )?;
                return Err(Stop::End);
            }
            Maogate1Talk8Step::OnTimer12000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("que_job02"),
                        Val::from("...In payment, I give... the most honest... the holiest... dedication..."),
                        Val::from(1),
                        Val::from(8087790),
                    ],
                )?;
                return Err(Stop::End);
            }
            Maogate1Talk8Step::OnTimer16000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("que_job02"),
                        Val::from("...Please... Let the tornado of blood, the pile of bodies... rival the mountains!"),
                        Val::from(1),
                        Val::from(8087790),
                    ],
                )?;
                return Err(Stop::End);
            }
            Maogate1Talk8Step::OnTimer20000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("que_job02"),
                        Val::from("...Ha... Ha... Muhahahahaha!"),
                        Val::from(1),
                        Val::from(8087790),
                    ],
                )?;
                return Err(Stop::End);
            }
            Maogate1Talk8Step::OnTimer20100 => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn maogate1_talk8(ctx: &Ctx) -> Script {
    maogate1_talk8_run(ctx, Maogate1Talk8Step::Start, Vec::new()).map(|_| ())
}

pub fn maogate1_talk8_onstop(ctx: &Ctx) -> Script {
    maogate1_talk8_run(ctx, Maogate1Talk8Step::OnStop, Vec::new()).map(|_| ())
}

pub fn maogate1_talk8_onenter(ctx: &Ctx) -> Script {
    maogate1_talk8_run(ctx, Maogate1Talk8Step::OnEnter, Vec::new()).map(|_| ())
}

pub fn maogate1_talk8_onstop2(ctx: &Ctx) -> Script {
    maogate1_talk8_run(ctx, Maogate1Talk8Step::OnStop2, Vec::new()).map(|_| ())
}

pub fn maogate1_talk8_ontouch(ctx: &Ctx) -> Script {
    maogate1_talk8_run(ctx, Maogate1Talk8Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn maogate1_talk8_ontimer1000(ctx: &Ctx) -> Script {
    maogate1_talk8_run(ctx, Maogate1Talk8Step::OnTimer1000, Vec::new()).map(|_| ())
}

pub fn maogate1_talk8_ontimer4000(ctx: &Ctx) -> Script {
    maogate1_talk8_run(ctx, Maogate1Talk8Step::OnTimer4000, Vec::new()).map(|_| ())
}

pub fn maogate1_talk8_ontimer8000(ctx: &Ctx) -> Script {
    maogate1_talk8_run(ctx, Maogate1Talk8Step::OnTimer8000, Vec::new()).map(|_| ())
}

pub fn maogate1_talk8_ontimer12000(ctx: &Ctx) -> Script {
    maogate1_talk8_run(ctx, Maogate1Talk8Step::OnTimer12000, Vec::new()).map(|_| ())
}

pub fn maogate1_talk8_ontimer16000(ctx: &Ctx) -> Script {
    maogate1_talk8_run(ctx, Maogate1Talk8Step::OnTimer16000, Vec::new()).map(|_| ())
}

pub fn maogate1_talk8_ontimer20000(ctx: &Ctx) -> Script {
    maogate1_talk8_run(ctx, Maogate1Talk8Step::OnTimer20000, Vec::new()).map(|_| ())
}

pub fn maogate1_talk8_ontimer20100(ctx: &Ctx) -> Script {
    maogate1_talk8_run(ctx, Maogate1Talk8Step::OnTimer20100, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Maogate1BattleStep {
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

fn maogate1_battle_run(ctx: &Ctx, mut step: Maogate1BattleStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Maogate1BattleStep::Start => {
                step = Maogate1BattleStep::OnStop;
                continue 'machine;
            }
            Maogate1BattleStep::OnStop => {
                ctx.call(Function::DisableNpc, vec![Val::from("#maogate1_battle")])?;
                return Err(Stop::End);
            }
            Maogate1BattleStep::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("#maogate1_battle")])?;
                return Err(Stop::End);
            }
            Maogate1BattleStep::OnStop2 => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("que_job02"), Val::from("#maogate1_battle::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Maogate1BattleStep::OnTouch => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_KEK")?,
                        Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                    ],
                )?;
                ctx.call(Function::DisableNpc, vec![Val::from("#maogate1_battle")])?;
                return Err(Stop::End);
            }
            Maogate1BattleStep::OnTimer500 => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_job02"),
                        Val::from(71),
                        Val::from(85),
                        Val::from("Mummy"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#maogate1_battle::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_job02"),
                        Val::from(79),
                        Val::from(85),
                        Val::from("Mummy"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#maogate1_battle::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_job02"),
                        Val::from(71),
                        Val::from(80),
                        Val::from("Mummy"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#maogate1_battle::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_job02"),
                        Val::from(79),
                        Val::from(80),
                        Val::from("Mummy"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#maogate1_battle::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Maogate1BattleStep::OnTimer5000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("que_job02"), Val::from("Watch out!"), Val::from(1), Val::from(9498256)],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion Member#1_bt::OnEnter")])?;
                return Err(Stop::End);
            }
            Maogate1BattleStep::OnTimer6000 => {
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_LORD")?])?;
                return Err(Stop::End);
            }
            Maogate1BattleStep::OnTimer6500 => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("que_job02"), Val::from("#maogate1_battle::OnMyMobDead")],
                )?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Maogate1BattleStep::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn maogate1_battle(ctx: &Ctx) -> Script {
    maogate1_battle_run(ctx, Maogate1BattleStep::Start, Vec::new()).map(|_| ())
}

pub fn maogate1_battle_onstop(ctx: &Ctx) -> Script {
    maogate1_battle_run(ctx, Maogate1BattleStep::OnStop, Vec::new()).map(|_| ())
}

pub fn maogate1_battle_onenter(ctx: &Ctx) -> Script {
    maogate1_battle_run(ctx, Maogate1BattleStep::OnEnter, Vec::new()).map(|_| ())
}

pub fn maogate1_battle_onstop2(ctx: &Ctx) -> Script {
    maogate1_battle_run(ctx, Maogate1BattleStep::OnStop2, Vec::new()).map(|_| ())
}

pub fn maogate1_battle_ontouch(ctx: &Ctx) -> Script {
    maogate1_battle_run(ctx, Maogate1BattleStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn maogate1_battle_ontimer500(ctx: &Ctx) -> Script {
    maogate1_battle_run(ctx, Maogate1BattleStep::OnTimer500, Vec::new()).map(|_| ())
}

pub fn maogate1_battle_ontimer5000(ctx: &Ctx) -> Script {
    maogate1_battle_run(ctx, Maogate1BattleStep::OnTimer5000, Vec::new()).map(|_| ())
}

pub fn maogate1_battle_ontimer6000(ctx: &Ctx) -> Script {
    maogate1_battle_run(ctx, Maogate1BattleStep::OnTimer6000, Vec::new()).map(|_| ())
}

pub fn maogate1_battle_ontimer6500(ctx: &Ctx) -> Script {
    maogate1_battle_run(ctx, Maogate1BattleStep::OnTimer6500, Vec::new()).map(|_| ())
}

pub fn maogate1_battle_onmymobdead(ctx: &Ctx) -> Script {
    maogate1_battle_run(ctx, Maogate1BattleStep::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum DandelionMember1BtStep {
    Start,
    OnInit,
    OnEnter,
}

fn dandelion_member_1_bt_run(ctx: &Ctx, mut step: DandelionMember1BtStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            DandelionMember1BtStep::Start => {
                if (ctx.var("mao_request").get()? == 26 || ctx.var("mao_request").get()? == 27) {
                    ctx.lines_as("Dandelion Member", args!["Hey... Are", "you alright?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Thanks...", "I think you just", "saved my life. What's", "going on? Are you alone?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Dandelion Member",
                        args![
                            "It took me a while, but",
                            "I gathered as many members",
                            "of Dandelion as I could. From",
                            "the looks of it, I wouldn't be",
                            "surprised if Satan Morocc was",
                            "almost unsealed by now..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Wait...", "What about Kidd?", "Where did he go?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Dandelion Member",
                        args![
                            "Kidd? He already",
                            "went in before any",
                            "of us. It sounds pretty",
                            "dangerous in there..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Kidd's Voice", args!["...I said--!", "...That's...", "Why I oughta--!"])?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Dandelion Member",
                        args![
                            "Yeah, that's him again.",
                            "Now that you're awake,",
                            "I better do something.",
                            "You stay here and gather",
                            "your strength while I go",
                            "in there and help him..."
                        ],
                    )?;
                    ctx.close_window()?;
                    ctx.call(
                        Function::Emotion,
                        vec![
                            ctx.constant("ET_SURPRISE")?,
                            Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                        ],
                    )?;
                    ctx.var("mao_request").set(Val::from(27))?;
                    ctx.call(Function::DisableNpc, vec![Val::from("Dandelion Member#1_bt")])?;
                    return Err(Stop::End);
                }
                step = DandelionMember1BtStep::OnInit;
                continue 'machine;
            }
            DandelionMember1BtStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Dandelion Member#1_bt")])?;
                return Err(Stop::End);
            }
            DandelionMember1BtStep::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("Dandelion Member#1_bt")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn dandelion_member_1_bt(ctx: &Ctx) -> Script {
    dandelion_member_1_bt_run(ctx, DandelionMember1BtStep::Start, Vec::new()).map(|_| ())
}

pub fn dandelion_member_1_bt_oninit(ctx: &Ctx) -> Script {
    dandelion_member_1_bt_run(ctx, DandelionMember1BtStep::OnInit, Vec::new()).map(|_| ())
}

pub fn dandelion_member_1_bt_onenter(ctx: &Ctx) -> Script {
    dandelion_member_1_bt_run(ctx, DandelionMember1BtStep::OnEnter, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Maogate1SettingStep {
    Start,
    OnStop,
    OnEnter,
    OnTouch,
}

fn maogate1_setting_run(ctx: &Ctx, mut step: Maogate1SettingStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Maogate1SettingStep::Start => {
                step = Maogate1SettingStep::OnStop;
                continue 'machine;
            }
            Maogate1SettingStep::OnStop => {
                ctx.call(Function::DisableNpc, vec![Val::from("#maogate1_setting")])?;
                return Err(Stop::End);
            }
            Maogate1SettingStep::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("#maogate1_setting")])?;
                return Err(Stop::End);
            }
            Maogate1SettingStep::OnTouch => {
                if ctx.var("mao_request").get()? == 27 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate1_end::OnEnter")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Kidd#maogate1_1::OnEnter")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Lin#maogate1_1::OnEnter")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("R#maogate1::OnEnter")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion#maogate1_1::OnEnter")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion#maogate1_2::OnEnter")])?;
                    return Err(Stop::End);
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn maogate1_setting(ctx: &Ctx) -> Script {
    maogate1_setting_run(ctx, Maogate1SettingStep::Start, Vec::new()).map(|_| ())
}

pub fn maogate1_setting_onstop(ctx: &Ctx) -> Script {
    maogate1_setting_run(ctx, Maogate1SettingStep::OnStop, Vec::new()).map(|_| ())
}

pub fn maogate1_setting_onenter(ctx: &Ctx) -> Script {
    maogate1_setting_run(ctx, Maogate1SettingStep::OnEnter, Vec::new()).map(|_| ())
}

pub fn maogate1_setting_ontouch(ctx: &Ctx) -> Script {
    maogate1_setting_run(ctx, Maogate1SettingStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Maogate1EndStep {
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

fn maogate1_end_run(ctx: &Ctx, mut step: Maogate1EndStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Maogate1EndStep::Start => {
                step = Maogate1EndStep::OnInit;
                continue 'machine;
            }
            Maogate1EndStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("#maogate1_end")])?;
                return Err(Stop::End);
            }
            Maogate1EndStep::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("que_job02"), Val::from("#maogate1_end::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Maogate1EndStep::OnStop => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("que_job02"), Val::from("#maogate1_end::OnMyMobDead")],
                )?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Maogate1EndStep::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("#maogate1_end")])?;
                return Err(Stop::End);
            }
            Maogate1EndStep::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate1_setting::OnStop")])?;
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
                        ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Lin#maogate1_1")])?,
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
                        ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("R#maogate1")])?,
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
                ctx.call(Function::DoNpcEvent, vec![Val::from("R#maogate1::OnSpell")])?;
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_SURPRISE")?,
                        ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Kidd#maogate1_1")])?,
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
                ctx.call(Function::DoNpcEvent, vec![Val::from("Kidd#maogate1_1::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Lin#maogate1_1::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Kidd#maogate1_2::OnEnter")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Lin#maogate1_2::OnEnter")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate1_end2::OnSpell")])?;
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
                        ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("R#maogate1")])?,
                    ],
                )?;
                ctx.next()?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("R#maogate1::OnInit")])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_job02"),
                        Val::from(121),
                        Val::from(118),
                        Val::from("Mummy"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#maogate1_end::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_job02"),
                        Val::from(121),
                        Val::from(115),
                        Val::from("Mummy"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#maogate1_end::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_job02"),
                        Val::from(121),
                        Val::from(112),
                        Val::from("Mummy"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#maogate1_end::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_job02"),
                        Val::from(107),
                        Val::from(106),
                        Val::from("Mummy"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#maogate1_end::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_job02"),
                        Val::from(111),
                        Val::from(106),
                        Val::from("Mummy"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#maogate1_end::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_job02"),
                        Val::from(115),
                        Val::from(106),
                        Val::from("Mummy"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#maogate1_end::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_job02"),
                        Val::from(135),
                        Val::from(105),
                        Val::from("Mummy"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#maogate1_end::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_job02"),
                        Val::from(132),
                        Val::from(105),
                        Val::from("Mummy"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#maogate1_end::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_job02"),
                        Val::from(128),
                        Val::from(105),
                        Val::from("Mummy"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#maogate1_end::OnMyMobDead"),
                    ],
                )?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["Bastard, what", "are you doing?", "Holy crap, m-monsters!"],
                )?;
                ctx.next()?;
                ctx.lines_as("Lin", args!["Damn it!", "Let me go!", "I'm gonna chase", "Moore myself!"])?;
                ctx.var("mao_request").set(Val::from(28))?;
                ctx.close_window()?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Lin#maogate1_2::OnInit")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("#maogate1_end")])?;
                return Err(Stop::End);
            }
            Maogate1EndStep::OnTimer500 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("que_job02"),
                        Val::from("I'll back you up!"),
                        Val::from(1),
                        Val::from(9498256),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion#maogate1_3::OnEnter")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion#maogate1_4::OnEnter")])?;
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_GO")?,
                        ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Dandelion#maogate1_1")])?,
                    ],
                )?;
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_GO")?,
                        ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Dandelion#maogate1_2")])?,
                    ],
                )?;
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_GO")?,
                        ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Dandelion#maogate1_3")])?,
                    ],
                )?;
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_GO")?,
                        ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Dandelion#maogate1_4")])?,
                    ],
                )?;
                return Err(Stop::End);
            }
            Maogate1EndStep::OnTimer3000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion#maogate1_3::OnSpell")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion#maogate1_2::OnSpell")])?;
                return Err(Stop::End);
            }
            Maogate1EndStep::OnTimer4000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion#maogate1_4::OnSpell")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion#maogate1_2::OnSpell2")])?;
                return Err(Stop::End);
            }
            Maogate1EndStep::OnTimer5000 => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("que_job02"), Val::from("#maogate1_end::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Maogate1EndStep::OnTimer5500 => {
                ctx.var("$@maogate1mobs").set(Val::from(3))?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_job02"),
                        Val::from(128),
                        Val::from(105),
                        Val::from("Mummy"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("Kidd#maogate1_2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_job02"),
                        Val::from(115),
                        Val::from(106),
                        Val::from("Mummy"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("Kidd#maogate1_2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_job02"),
                        Val::from(121),
                        Val::from(112),
                        Val::from("Mummy"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("Kidd#maogate1_2::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Maogate1EndStep::OnTimer7000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("que_job02"),
                        Val::from("Dandelions, don't let Raiyan Moore escape!"),
                        Val::from(1),
                        Val::from(9498256),
                    ],
                )?;
                return Err(Stop::End);
            }
            Maogate1EndStep::OnTimer10000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("que_job02"),
                        Val::from("Kidd, we're leaving the rest to you...!"),
                        Val::from(1),
                        Val::from(9498256),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion#maogate1_4::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion#maogate1_3::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion#maogate1_2::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion#maogate1_1::OnInit")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Maogate1EndStep::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn maogate1_end(ctx: &Ctx) -> Script {
    maogate1_end_run(ctx, Maogate1EndStep::Start, Vec::new()).map(|_| ())
}

pub fn maogate1_end_oninit(ctx: &Ctx) -> Script {
    maogate1_end_run(ctx, Maogate1EndStep::OnInit, Vec::new()).map(|_| ())
}

pub fn maogate1_end_onreset(ctx: &Ctx) -> Script {
    maogate1_end_run(ctx, Maogate1EndStep::OnReset, Vec::new()).map(|_| ())
}

pub fn maogate1_end_onstop(ctx: &Ctx) -> Script {
    maogate1_end_run(ctx, Maogate1EndStep::OnStop, Vec::new()).map(|_| ())
}

pub fn maogate1_end_onenter(ctx: &Ctx) -> Script {
    maogate1_end_run(ctx, Maogate1EndStep::OnEnter, Vec::new()).map(|_| ())
}

pub fn maogate1_end_ontouch(ctx: &Ctx) -> Script {
    maogate1_end_run(ctx, Maogate1EndStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn maogate1_end_ontimer500(ctx: &Ctx) -> Script {
    maogate1_end_run(ctx, Maogate1EndStep::OnTimer500, Vec::new()).map(|_| ())
}

pub fn maogate1_end_ontimer3000(ctx: &Ctx) -> Script {
    maogate1_end_run(ctx, Maogate1EndStep::OnTimer3000, Vec::new()).map(|_| ())
}

pub fn maogate1_end_ontimer4000(ctx: &Ctx) -> Script {
    maogate1_end_run(ctx, Maogate1EndStep::OnTimer4000, Vec::new()).map(|_| ())
}

pub fn maogate1_end_ontimer5000(ctx: &Ctx) -> Script {
    maogate1_end_run(ctx, Maogate1EndStep::OnTimer5000, Vec::new()).map(|_| ())
}

pub fn maogate1_end_ontimer5500(ctx: &Ctx) -> Script {
    maogate1_end_run(ctx, Maogate1EndStep::OnTimer5500, Vec::new()).map(|_| ())
}

pub fn maogate1_end_ontimer7000(ctx: &Ctx) -> Script {
    maogate1_end_run(ctx, Maogate1EndStep::OnTimer7000, Vec::new()).map(|_| ())
}

pub fn maogate1_end_ontimer10000(ctx: &Ctx) -> Script {
    maogate1_end_run(ctx, Maogate1EndStep::OnTimer10000, Vec::new()).map(|_| ())
}

pub fn maogate1_end_onmymobdead(ctx: &Ctx) -> Script {
    maogate1_end_run(ctx, Maogate1EndStep::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum KiddMaogate11Step {
    Start,
    OnInit,
    OnEnter,
}

fn kidd_maogate1_1_run(ctx: &Ctx, mut step: KiddMaogate11Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            KiddMaogate11Step::Start => {
                step = KiddMaogate11Step::OnInit;
                continue 'machine;
            }
            KiddMaogate11Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Kidd#maogate1_1")])?;
                return Err(Stop::End);
            }
            KiddMaogate11Step::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("Kidd#maogate1_1")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn kidd_maogate1_1(ctx: &Ctx) -> Script {
    kidd_maogate1_1_run(ctx, KiddMaogate11Step::Start, Vec::new()).map(|_| ())
}

pub fn kidd_maogate1_1_oninit(ctx: &Ctx) -> Script {
    kidd_maogate1_1_run(ctx, KiddMaogate11Step::OnInit, Vec::new()).map(|_| ())
}

pub fn kidd_maogate1_1_onenter(ctx: &Ctx) -> Script {
    kidd_maogate1_1_run(ctx, KiddMaogate11Step::OnEnter, Vec::new()).map(|_| ())
}

fn kidd_maogate1_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn kidd_maogate1_2(ctx: &Ctx) -> Script {
    kidd_maogate1_2_body(ctx, Vec::new()).map(|_| ())
}

fn kidd_maogate1_2_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Kidd#maogate1_2")])?;
    return Err(Stop::End);
}

pub fn kidd_maogate1_2_oninit(ctx: &Ctx) -> Script {
    kidd_maogate1_2_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn kidd_maogate1_2_onenter_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("Kidd#maogate1_2")])?;
    return Err(Stop::End);
}

pub fn kidd_maogate1_2_onenter(ctx: &Ctx) -> Script {
    kidd_maogate1_2_onenter_body(ctx, Vec::new()).map(|_| ())
}

fn kidd_maogate1_2_onmymobdead_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.var("$@maogate1mobs")
        .set((ctx.var("$@maogate1mobs").get()?.try_sub(Val::from(1))?))?;
    if ctx.var("$@maogate1mobs").get()?.number()? < 1 {
        ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate1_end2::OnEnter")])?;
    }
    return Err(Stop::End);
}

pub fn kidd_maogate1_2_onmymobdead(ctx: &Ctx) -> Script {
    kidd_maogate1_2_onmymobdead_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum LinMaogate11Step {
    Start,
    OnInit,
    OnEnter,
}

fn lin_maogate1_1_run(ctx: &Ctx, mut step: LinMaogate11Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            LinMaogate11Step::Start => {
                step = LinMaogate11Step::OnInit;
                continue 'machine;
            }
            LinMaogate11Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Lin#maogate1_1")])?;
                return Err(Stop::End);
            }
            LinMaogate11Step::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("Lin#maogate1_1")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn lin_maogate1_1(ctx: &Ctx) -> Script {
    lin_maogate1_1_run(ctx, LinMaogate11Step::Start, Vec::new()).map(|_| ())
}

pub fn lin_maogate1_1_oninit(ctx: &Ctx) -> Script {
    lin_maogate1_1_run(ctx, LinMaogate11Step::OnInit, Vec::new()).map(|_| ())
}

pub fn lin_maogate1_1_onenter(ctx: &Ctx) -> Script {
    lin_maogate1_1_run(ctx, LinMaogate11Step::OnEnter, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum LinMaogate12Step {
    Start,
    OnInit,
    OnEnter,
}

fn lin_maogate1_2_run(ctx: &Ctx, mut step: LinMaogate12Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            LinMaogate12Step::Start => {
                step = LinMaogate12Step::OnInit;
                continue 'machine;
            }
            LinMaogate12Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Lin#maogate1_2")])?;
                return Err(Stop::End);
            }
            LinMaogate12Step::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("Lin#maogate1_2")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn lin_maogate1_2(ctx: &Ctx) -> Script {
    lin_maogate1_2_run(ctx, LinMaogate12Step::Start, Vec::new()).map(|_| ())
}

pub fn lin_maogate1_2_oninit(ctx: &Ctx) -> Script {
    lin_maogate1_2_run(ctx, LinMaogate12Step::OnInit, Vec::new()).map(|_| ())
}

pub fn lin_maogate1_2_onenter(ctx: &Ctx) -> Script {
    lin_maogate1_2_run(ctx, LinMaogate12Step::OnEnter, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum RMaogate1Step {
    Start,
    OnInit,
    OnEnter,
    OnSpell,
}

fn r_maogate1_run(ctx: &Ctx, mut step: RMaogate1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            RMaogate1Step::Start => {
                step = RMaogate1Step::OnInit;
                continue 'machine;
            }
            RMaogate1Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("R#maogate1")])?;
                return Err(Stop::End);
            }
            RMaogate1Step::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("R#maogate1")])?;
                return Err(Stop::End);
            }
            RMaogate1Step::OnSpell => {
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL2")?])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn r_maogate1(ctx: &Ctx) -> Script {
    r_maogate1_run(ctx, RMaogate1Step::Start, Vec::new()).map(|_| ())
}

pub fn r_maogate1_oninit(ctx: &Ctx) -> Script {
    r_maogate1_run(ctx, RMaogate1Step::OnInit, Vec::new()).map(|_| ())
}

pub fn r_maogate1_onenter(ctx: &Ctx) -> Script {
    r_maogate1_run(ctx, RMaogate1Step::OnEnter, Vec::new()).map(|_| ())
}

pub fn r_maogate1_onspell(ctx: &Ctx) -> Script {
    r_maogate1_run(ctx, RMaogate1Step::OnSpell, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum DandelionMaogate11Step {
    Start,
    OnInit,
    OnEnter,
}

fn dandelion_maogate1_1_run(ctx: &Ctx, mut step: DandelionMaogate11Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            DandelionMaogate11Step::Start => {
                step = DandelionMaogate11Step::OnInit;
                continue 'machine;
            }
            DandelionMaogate11Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Dandelion#maogate1_1")])?;
                return Err(Stop::End);
            }
            DandelionMaogate11Step::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("Dandelion#maogate1_1")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn dandelion_maogate1_1(ctx: &Ctx) -> Script {
    dandelion_maogate1_1_run(ctx, DandelionMaogate11Step::Start, Vec::new()).map(|_| ())
}

pub fn dandelion_maogate1_1_oninit(ctx: &Ctx) -> Script {
    dandelion_maogate1_1_run(ctx, DandelionMaogate11Step::OnInit, Vec::new()).map(|_| ())
}

pub fn dandelion_maogate1_1_onenter(ctx: &Ctx) -> Script {
    dandelion_maogate1_1_run(ctx, DandelionMaogate11Step::OnEnter, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum DandelionMaogate12Step {
    Start,
    OnInit,
    OnEnter,
    OnSpell,
    OnSpell2,
}

fn dandelion_maogate1_2_run(ctx: &Ctx, mut step: DandelionMaogate12Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            DandelionMaogate12Step::Start => {
                step = DandelionMaogate12Step::OnInit;
                continue 'machine;
            }
            DandelionMaogate12Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Dandelion#maogate1_2")])?;
                return Err(Stop::End);
            }
            DandelionMaogate12Step::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("Dandelion#maogate1_2")])?;
                return Err(Stop::End);
            }
            DandelionMaogate12Step::OnSpell => {
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL2")?])?;
                return Err(Stop::End);
            }
            DandelionMaogate12Step::OnSpell2 => {
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_LORD")?])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn dandelion_maogate1_2(ctx: &Ctx) -> Script {
    dandelion_maogate1_2_run(ctx, DandelionMaogate12Step::Start, Vec::new()).map(|_| ())
}

pub fn dandelion_maogate1_2_oninit(ctx: &Ctx) -> Script {
    dandelion_maogate1_2_run(ctx, DandelionMaogate12Step::OnInit, Vec::new()).map(|_| ())
}

pub fn dandelion_maogate1_2_onenter(ctx: &Ctx) -> Script {
    dandelion_maogate1_2_run(ctx, DandelionMaogate12Step::OnEnter, Vec::new()).map(|_| ())
}

pub fn dandelion_maogate1_2_onspell(ctx: &Ctx) -> Script {
    dandelion_maogate1_2_run(ctx, DandelionMaogate12Step::OnSpell, Vec::new()).map(|_| ())
}

pub fn dandelion_maogate1_2_onspell2(ctx: &Ctx) -> Script {
    dandelion_maogate1_2_run(ctx, DandelionMaogate12Step::OnSpell2, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum DandelionMaogate13Step {
    Start,
    OnInit,
    OnEnter,
    OnSpell,
}

fn dandelion_maogate1_3_run(ctx: &Ctx, mut step: DandelionMaogate13Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            DandelionMaogate13Step::Start => {
                step = DandelionMaogate13Step::OnInit;
                continue 'machine;
            }
            DandelionMaogate13Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Dandelion#maogate1_3")])?;
                return Err(Stop::End);
            }
            DandelionMaogate13Step::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("Dandelion#maogate1_3")])?;
                return Err(Stop::End);
            }
            DandelionMaogate13Step::OnSpell => {
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL2")?])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn dandelion_maogate1_3(ctx: &Ctx) -> Script {
    dandelion_maogate1_3_run(ctx, DandelionMaogate13Step::Start, Vec::new()).map(|_| ())
}

pub fn dandelion_maogate1_3_oninit(ctx: &Ctx) -> Script {
    dandelion_maogate1_3_run(ctx, DandelionMaogate13Step::OnInit, Vec::new()).map(|_| ())
}

pub fn dandelion_maogate1_3_onenter(ctx: &Ctx) -> Script {
    dandelion_maogate1_3_run(ctx, DandelionMaogate13Step::OnEnter, Vec::new()).map(|_| ())
}

pub fn dandelion_maogate1_3_onspell(ctx: &Ctx) -> Script {
    dandelion_maogate1_3_run(ctx, DandelionMaogate13Step::OnSpell, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum DandelionMaogate14Step {
    Start,
    OnInit,
    OnEnter,
    OnSpell,
}

fn dandelion_maogate1_4_run(ctx: &Ctx, mut step: DandelionMaogate14Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            DandelionMaogate14Step::Start => {
                step = DandelionMaogate14Step::OnInit;
                continue 'machine;
            }
            DandelionMaogate14Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Dandelion#maogate1_4")])?;
                return Err(Stop::End);
            }
            DandelionMaogate14Step::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("Dandelion#maogate1_4")])?;
                return Err(Stop::End);
            }
            DandelionMaogate14Step::OnSpell => {
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_LORD")?])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn dandelion_maogate1_4(ctx: &Ctx) -> Script {
    dandelion_maogate1_4_run(ctx, DandelionMaogate14Step::Start, Vec::new()).map(|_| ())
}

pub fn dandelion_maogate1_4_oninit(ctx: &Ctx) -> Script {
    dandelion_maogate1_4_run(ctx, DandelionMaogate14Step::OnInit, Vec::new()).map(|_| ())
}

pub fn dandelion_maogate1_4_onenter(ctx: &Ctx) -> Script {
    dandelion_maogate1_4_run(ctx, DandelionMaogate14Step::OnEnter, Vec::new()).map(|_| ())
}

pub fn dandelion_maogate1_4_onspell(ctx: &Ctx) -> Script {
    dandelion_maogate1_4_run(ctx, DandelionMaogate14Step::OnSpell, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Maogate1End2Step {
    Start,
    OnInit,
    OnEnter,
    OnTouch,
}

fn maogate1_end2_run(ctx: &Ctx, mut step: Maogate1End2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Maogate1End2Step::Start => {
                step = Maogate1End2Step::OnInit;
                continue 'machine;
            }
            Maogate1End2Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("#maogate1_end2")])?;
                return Err(Stop::End);
            }
            Maogate1End2Step::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("#maogate1_end2")])?;
                return Err(Stop::End);
            }
            Maogate1End2Step::OnTouch => {
                if ctx.var("mao_request").get()? == 28 {
                    ctx.call(Function::Cutin, vec![Val::from("mocseal_kid01.bmp"), Val::from(2)])?;
                    ctx.lines_as(
                        "Kidd",
                        args![
                            ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("!")),
                            "Hey, you alright?",
                            "Come on, let's head",
                            "back, the other members",
                            "of the Assassin Guild",
                            "will take care of this..."
                        ],
                    )?;
                    ctx.var("mao_request").set(Val::from(29))?;
                    ctx.close_window()?;
                    ctx.call(
                        Function::MapWarp,
                        vec![Val::from("que_job02"), Val::from("que_job01"), Val::from(59), Val::from(49)],
                    )?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate1_talk1::OnInit")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate1_talk2::OnEnter")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate1_talk3::OnEnter")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate1_talk4::OnEnter")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate1_talk5::OnEnter")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate1_talk6::OnEnter")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate1_talk7::OnEnter")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Kidd#maogate1_1::OnInit")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Kidd#maogate1_2::OnInit")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Lin#maogate1_1::OnInit")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Lin#maogate1_2::OnInit")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("R#maogate1::OnInit")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate1_end::OnStop")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate1_end::OnInit")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate1_end2::OnInit")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion#maogate1_1::OnInit")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion#maogate1_2::OnInit")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion#maogate1_3::OnInit")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion#maogate1_4::OnInit")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate1_setting::OnEnter")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion Member#1_bt::OnInit")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate1_battle::OnStop")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate1_battle::OnEnter")])?;
                    ctx.call(Function::EnableNpc, vec![Val::from("#maogate1")])?;
                    ctx.var("$mao_gate1").set(Val::from(0))?;
                    return Err(Stop::End);
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn maogate1_end2(ctx: &Ctx) -> Script {
    maogate1_end2_run(ctx, Maogate1End2Step::Start, Vec::new()).map(|_| ())
}

pub fn maogate1_end2_oninit(ctx: &Ctx) -> Script {
    maogate1_end2_run(ctx, Maogate1End2Step::OnInit, Vec::new()).map(|_| ())
}

pub fn maogate1_end2_onenter(ctx: &Ctx) -> Script {
    maogate1_end2_run(ctx, Maogate1End2Step::OnEnter, Vec::new()).map(|_| ())
}

pub fn maogate1_end2_ontouch(ctx: &Ctx) -> Script {
    maogate1_end2_run(ctx, Maogate1End2Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Maogate2Step {
    Start,
    OnEnter,
    OnStop,
    OnTouch,
    OnTimer580000,
    OnTimer590000,
    OnTimer595000,
    OnTimer596000,
    OnTimer597000,
}

fn maogate2_run(ctx: &Ctx, mut step: Maogate2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Maogate2Step::Start => {
                step = Maogate2Step::OnEnter;
                continue 'machine;
            }
            Maogate2Step::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("#maogate2")])?;
                return Err(Stop::End);
            }
            Maogate2Step::OnStop => {
                ctx.call(Function::DisableNpc, vec![Val::from("#maogate2")])?;
                return Err(Stop::End);
            }
            Maogate2Step::OnTouch => {
                if (ctx.var("mao_request").get()? == 124 || ctx.var("mao_request").get()? == 125) {
                    ctx.call(Function::InitNpcTimer, vec![])?;
                    ctx.call(Function::DisableNpc, vec![Val::from("#maogate2")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate2_talk1::OnEnter")])?;
                } else {
                    ctx.lines(args!["^3355FFYou will now be", "teleported outside.^000000"])?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("morocc"), Val::from(100), Val::from(100)])?;
                    ctx.var("$mao_gate2").set(Val::from(0))?;
                }
                return Err(Stop::End);
            }
            Maogate2Step::OnTimer580000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("que_job02"),
                        Val::from("...Need... more blood... before... gate closes..."),
                        Val::from(1),
                        Val::from(14524637),
                    ],
                )?;
                return Err(Stop::End);
            }
            Maogate2Step::OnTimer590000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("que_job02"),
                        Val::from("...Grrrr... can't... fail this time... must revive..."),
                        Val::from(1),
                        Val::from(14524637),
                    ],
                )?;
                return Err(Stop::End);
            }
            Maogate2Step::OnTimer595000 => {
                ctx.call(
                    Function::MapWarp,
                    vec![Val::from("que_job02"), Val::from("morocc"), Val::from(160), Val::from(129)],
                )?;
                return Err(Stop::End);
            }
            Maogate2Step::OnTimer596000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate2_talk1::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate2_talk2::OnEnter")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate2_talk3::OnEnter")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate2_talk4::OnEnter")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate2_talk5::OnEnter")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate2_talk6::OnEnter")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate2_talk7::OnEnter")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Kidd#maogate2_1::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Kidd#maogate2_2::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Lin#maogate2_1::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Lin#maogate2_2::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("R#maogate2::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate2_end::OnStop")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate2_end::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate2_end2::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion#maogate2_1::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion#maogate2_2::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion#maogate2_3::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion#maogate2_4::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate2_setting::OnEnter")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion Member#2_bt::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate2_battle::OnStop2")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate2_battle::OnEnter")])?;
                return Err(Stop::End);
            }
            Maogate2Step::OnTimer597000 => {
                ctx.call(Function::EnableNpc, vec![Val::from("#maogate2")])?;
                ctx.var("$mao_gate2").set(Val::from(0))?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn maogate2(ctx: &Ctx) -> Script {
    maogate2_run(ctx, Maogate2Step::Start, Vec::new()).map(|_| ())
}

pub fn maogate2_onenter(ctx: &Ctx) -> Script {
    maogate2_run(ctx, Maogate2Step::OnEnter, Vec::new()).map(|_| ())
}

pub fn maogate2_onstop(ctx: &Ctx) -> Script {
    maogate2_run(ctx, Maogate2Step::OnStop, Vec::new()).map(|_| ())
}

pub fn maogate2_ontouch(ctx: &Ctx) -> Script {
    maogate2_run(ctx, Maogate2Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn maogate2_ontimer580000(ctx: &Ctx) -> Script {
    maogate2_run(ctx, Maogate2Step::OnTimer580000, Vec::new()).map(|_| ())
}

pub fn maogate2_ontimer590000(ctx: &Ctx) -> Script {
    maogate2_run(ctx, Maogate2Step::OnTimer590000, Vec::new()).map(|_| ())
}

pub fn maogate2_ontimer595000(ctx: &Ctx) -> Script {
    maogate2_run(ctx, Maogate2Step::OnTimer595000, Vec::new()).map(|_| ())
}

pub fn maogate2_ontimer596000(ctx: &Ctx) -> Script {
    maogate2_run(ctx, Maogate2Step::OnTimer596000, Vec::new()).map(|_| ())
}

pub fn maogate2_ontimer597000(ctx: &Ctx) -> Script {
    maogate2_run(ctx, Maogate2Step::OnTimer597000, Vec::new()).map(|_| ())
}

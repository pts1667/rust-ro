use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

pub fn brisinsummon_onlowenon(ctx: &Ctx) -> Script {
    brisinsummon_run(ctx, BrisinsummonStep::OnLowenOn, Vec::new()).map(|_| ())
}

pub fn brisinsummon_onlowenoff(ctx: &Ctx) -> Script {
    brisinsummon_run(ctx, BrisinsummonStep::OnLowenOff, Vec::new()).map(|_| ())
}

pub fn brisinsummon_onhermiteoff(ctx: &Ctx) -> Script {
    brisinsummon_run(ctx, BrisinsummonStep::OnHermiteOff, Vec::new()).map(|_| ())
}

pub fn brisinsummon_onlowen2off(ctx: &Ctx) -> Script {
    brisinsummon_run(ctx, BrisinsummonStep::OnLowen2Off, Vec::new()).map(|_| ())
}

pub fn brisinsummon_onsummon(ctx: &Ctx) -> Script {
    brisinsummon_run(ctx, BrisinsummonStep::OnSummon, Vec::new()).map(|_| ())
}

pub fn brisinsummon_onmobdeath(ctx: &Ctx) -> Script {
    brisinsummon_run(ctx, BrisinsummonStep::OnMobDeath, Vec::new()).map(|_| ())
}

pub fn brisinsummon_onreset(ctx: &Ctx) -> Script {
    brisinsummon_run(ctx, BrisinsummonStep::OnReset, Vec::new()).map(|_| ())
}

fn doppelganger1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn doppelganger1(ctx: &Ctx) -> Script {
    doppelganger1_body(ctx, Vec::new()).map(|_| ())
}

fn doppelganger2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn doppelganger2(ctx: &Ctx) -> Script {
    doppelganger2_body(ctx, Vec::new()).map(|_| ())
}

fn lowen_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn lowen(ctx: &Ctx) -> Script {
    lowen_body(ctx, Vec::new()).map(|_| ())
}

fn knight1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn knight1(ctx: &Ctx) -> Script {
    knight1_body(ctx, Vec::new()).map(|_| ())
}

fn knight2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn knight2(ctx: &Ctx) -> Script {
    knight2_body(ctx, Vec::new()).map(|_| ())
}

fn knight3_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn knight3(ctx: &Ctx) -> Script {
    knight3_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum LowenoneStep {
    Start,
    OnTouch,
}

fn lowenone_run(ctx: &Ctx, mut step: LowenoneStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            LowenoneStep::Start => {
                step = LowenoneStep::OnTouch;
                continue 'machine;
            }
            LowenoneStep::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("brisinsold2::OnSold2On")])?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["Where am I?", "Isn't this...?!"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "Ah, I see.",
                        "This must be the place where everything happened two years ago. Good, good..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Lowen Ellenen",
                    args![
                        "Shshh...",
                        "This is just a reflection of the past. Here, you'll see what happened at the time for yourself."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Lowen Ellenen",
                    args!["It wasn't any trouble", "getting down here. But...", "We've got problems now."],
                )?;
                ctx.next()?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("que_god02"),
                        Val::from("D...Doppelganger!!!"),
                        Val::from(0),
                        Val::from(10288896),
                    ],
                )?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["What...?!", "Doppelganger?!"],
                )?;
                ctx.next()?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#brisinsummon::OnDoppel1On")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#brisinsummon::OnKnight1On")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#brisinsummon::OnKnight2On")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#brisinsummon::OnKnight3On")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#brisinsummon::OnLowenOn")])?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["What's going on?", "Did you really fight with that creature? H-hey! Answer me!"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Lowen Ellenen",
                    args![
                        "Yes, this was the place...",
                        "The commander gave orders",
                        "and then... What happened?"
                    ],
                )?;
                ctx.next()?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("que_god02"),
                        Val::from("Everyone, stay calm! Get into your positions!"),
                        Val::from(0),
                        Val::from(10288896),
                    ],
                )?;
                ctx.lines(args![
                    "^4E2F2F[Crusader Commander]",
                    "Everyone, get into your positions! Front line, block its movement! Rear line, support the defense!^000000"
                ])?;
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_GO")?,
                        ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("#knight1")])?,
                    ],
                )?;
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_GO")?,
                        ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("#knight2")])?,
                    ],
                )?;
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_GO")?,
                        ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("#knight3")])?,
                    ],
                )?;
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_SURPRISE")?,
                        ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("#doppelganger1")])?,
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Lowen Ellenen", args!["N-no!", "I'm not running away!"])?;
                ctx.next()?;
                ctx.mes("^3355FFYou're filled with a great anxiety, and you experience for yourself the fear and pressure Lowen felt during those very moments.^000000")?;
                ctx.next()?;
                ctx.lines(args![
                    "^4E2F2F[Crusader Commander]",
                    "Go, Lowen!",
                    "Cast Grand Cross, now!^000000"
                ])?;
                ctx.next()?;
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![ctx.constant("EF_BEGINSPELL7")?, ctx.constant("AREA")?, Val::from("#lowen")],
                )?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("que_god02"),
                        Val::from("Mwahaha! Mortals are such fools..."),
                        Val::from(0),
                        Val::from(11053224),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#brisinsummon::OnDoppel1Off")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#brisinsummon::OnDoppel2On")])?;
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_SURPRISE")?,
                        ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("#knight1")])?,
                    ],
                )?;
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_SURPRISE")?,
                        ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("#knight2")])?,
                    ],
                )?;
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_SURPRISE")?,
                        ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("#knight3")])?,
                    ],
                )?;
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_SURPRISE")?,
                        ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("#lowen")])?,
                    ],
                )?;
                ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args![" ??!!!"])?;
                ctx.next()?;
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![ctx.constant("EF_BEGINSPELL7")?, ctx.constant("AREA")?, Val::from("#doppelganger2")],
                )?;
                ctx.lines_as("Male Voice", args!["Lowen!", "Noooooooooo!!!"])?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("que_god02"), Val::from(120), Val::from(52)])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#brisinsummon::OnDoppel2Off")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#brisinsummon::OnKnight1Off")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#brisinsummon::OnKnight2Off")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#brisinsummon::OnKnight3Off")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#brisinsummon::OnLowenOff")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn lowenone(ctx: &Ctx) -> Script {
    lowenone_run(ctx, LowenoneStep::Start, Vec::new()).map(|_| ())
}

pub fn lowenone_ontouch(ctx: &Ctx) -> Script {
    lowenone_run(ctx, LowenoneStep::OnTouch, Vec::new()).map(|_| ())
}

fn hermite_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn hermite(ctx: &Ctx) -> Script {
    hermite_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum MonologueStep {
    Start,
    OnTouch,
}

fn monologue_run(ctx: &Ctx, mut step: MonologueStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MonologueStep::Start => {
                step = MonologueStep::OnTouch;
                continue 'machine;
            }
            MonologueStep::OnTouch => {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["Where am I?", "Isn't that...", "What's Hermite", "doing here?!"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Hermite Charles",
                    args!["No, no, no!", "Lowen don't...!", "He's gonna kill you!"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Hermite Charles",
                    args![
                        "Grand Cross?!",
                        "Using that's",
                        "committing suicide!",
                        "Lowen, don't do it!",
                        "Don't do that, Lowen!"
                    ],
                )?;
                ctx.next()?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("que_god02"),
                        Val::from("Mwahaha! Mortals are such fools..."),
                        Val::from(0),
                        Val::from(11053224),
                    ],
                )?;
                ctx.lines_as("Hermite Charles", args!["Lowen!", "Noooooooooo!!!"])?;
                ctx.next()?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#brisinsummon::OnHermiteOff")])?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["Where did he go?", "Was... Was that", "from his mind?"],
                )?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("que_god02"),
                        Val::from("What happened?! L-Lowen's gone!!"),
                        Val::from(0),
                        Val::from(11053224),
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Lowen Ellenen", args!["...", "I see now.", "I know...", "What happened..."])?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("que_god02"), Val::from(18), Val::from(127)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn monologue(ctx: &Ctx) -> Script {
    monologue_run(ctx, MonologueStep::Start, Vec::new()).map(|_| ())
}

pub fn monologue_ontouch(ctx: &Ctx) -> Script {
    monologue_run(ctx, MonologueStep::OnTouch, Vec::new()).map(|_| ())
}

fn lowen_ellenen_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Lowen Ellenen",
        args![
            "I was a fool.",
            "Now I remember Hermite. He must have been the one who rescued me. But when I came to my senses, no one was there."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Lowen Ellenen",
        args!["I blamed myself for surviving alone. I should have died there with my friends. I felt so useless."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Lowen Ellenen",
        args!["Now I realize that there were many people who loved me. But at the time, I felt only guilt."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Lowen Ellenen",
        args!["I returned 3 days later to my commander. It was stupid, but at the moment I couldn't think of anything else."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Lowen Ellenen",
        args!["The higher ups and the politicians were looking for a scapegoat for the failure of the mission."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Lowen Ellenen",
        args!["So I was the one who was blamed. Then, they took away everything I had. My status, the holy power granted to me..."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Lowen Ellenen",
        args![
            "I thought I deserved that punishment. After all, why should",
            "I have survived when all my comrades died?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Lowen Ellenen",
        args![
            "It was then that I decided to avenge my friends, and salvage",
            "what was left of my sense of honor."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Lowen Ellenen",
        args![
            "I went back to that place.",
            "I fought and I fought, trying to make penance for what I had",
            "done wrong..."
        ],
    )?;
    ctx.next()?;
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("que_god02"),
            Val::from("If you wish, continue your battle. I promise you the winner will become an Einherjar..."),
            Val::from(0),
            Val::from(7396243),
        ],
    )?;
    ctx.lines_as("Lowen Ellenen", args!["That voice...", "Isn't that...?"])?;
    ctx.next()?;
    ctx.lines_as(
        "Lowen Ellenen",
        args!["Are you giving me another chance? But I no longer have a body to fight monsters anymore..."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
        args!["Don't worry,", "I'm with you!", "Let's do it!"],
    )?;
    ctx.close_window()?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("#brisinsummon::OnSummon")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("#brisinsummon::OnLowen2Off")])?;
    return Err(Stop::End);
}

pub fn lowen_ellenen_2(ctx: &Ctx) -> Script {
    lowen_ellenen_2_body(ctx, Vec::new()).map(|_| ())
}

fn valkyrie_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("god_brising").get()? == 31 {
        ctx.lines_as(
            "Valkyrie",
            args!["You have passed", "the test, Lowen.", "I don't have much", "time to speak."],
        )?;
        ctx.next()?;
        ctx.lines_as("Valkyrie", args!["Have you found the strength to release yourself from the guilt and the pain? I have been waiting for you. Will you accept my invitation to Valhalla?"])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Yes, I will follow you.:Um... I'm not Lowen.")])? {
            1 => {
                ctx.lines_as(
                    "Valkyrie",
                    args![
                        "Lowen Ellenen, misunderstood hero. I will now promote you",
                        "to Einherjar with",
                        "the will of God."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Valkyrie",
                    args!["Your soul and spirit will be reborn in Valhalla as a holy warrior preparing for the holy war."],
                )?;
                ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_BLESSING")?])?;
                ctx.next()?;
                ctx.lines_as(
                    "Valkyrie",
                    args![
                        "And...",
                        ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(",")),
                        "Your exploits are known to me. But I am curious as to what has brought you here?"
                    ],
                )?;
                ctx.next()?;
                ctx.mes("^3355FFYou explained what happened to the Valkyrie, starting from the Snow Crystal which never melts.^000000")?;
                ctx.next()?;
                ctx.lines_as(
                    "Valkyrie",
                    args![
                        "I understand.",
                        "The story begins a long, long time ago with Freya, the patron goddess of beauty and love."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Valkyrie", args!["Her teardrop became a beautiful jewel. The snow on which Freya's teardrops have fallen will always retain its crystalline beauty."])?;
                ctx.next()?;
                ctx.lines_as("Valkyrie", args!["You seek traces of the goddess? Perhaps this is a sign of the end of the thousand year peace. When the time comes, I hope humans will not abandon their faith."])?;
                ctx.next()?;
                ctx.lines_as("Valkyrie", args!["Perhaps, I was also sent to meet you. I wish that humans will use the power derived from the traces of the gods to protect justice."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Valkyrie",
                    args![
                        ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("...")),
                        "Listen. This is",
                        "what you must find."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Valkyrie",
                    args![
                        "A cold snowy path on",
                        "which the goddess tread.",
                        "A blue river the goddess crossed.",
                        "A narrow path the goddess passed.",
                        "Four dwarves that the goddess met..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Valkyrie",
                    args![
                        "^4d4dffThe beauty of the stars",
                        "Wanes in comparison.",
                        "We lost our hearts",
                        "To that golden hair",
                        "And those dazzling eyes.^000000"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Valkyrie",
                    args!["Remember this.", "I shall send you", "back to where you", "belong."],
                )?;
                ctx.var("god_brising").set(Val::from(34))?;
                ctx.next()?;
                ctx.lines_as("Valkyrie", args!["For Odin's honor..."])?;
                ctx.close_window()?;
                ctx.call(Function::DisableNpc, vec![Val::from("#doppelganger1")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("#doppelganger2")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("#knight1")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("#knight2")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("#knight3")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("#lowen")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Soldier#1_brising")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Soldier#2_brising")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("#hermite")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Lowen Ellenen#2")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Valkyrie#1")])?;
                ctx.call(Function::Warp, vec![Val::from("geffen"), Val::from(120), Val::from(101)])?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Valkyrie",
                    args!["Hahahaha...", "It has been", "a while since", "a mortal has", "made me laugh."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Valkyrie",
                    args![
                        ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(",")),
                        "I am speaking to the",
                        "spirit that rests within you, the girl to whom you lent your body so that she may learn the truth of the past."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Valkyrie",
                    args![
                        "Lowen Ellenen,",
                        "misunderstood hero.",
                        "I will now promote you to Einherjar with the will of God."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Valkyrie",
                    args![
                        "Your soul and",
                        "spirit will be reborn in Valhalla as a holy warrior preparing for the holy war."
                    ],
                )?;
                ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_BLESSING")?])?;
                ctx.next()?;
                ctx.lines_as(
                    "Valkyrie",
                    args![
                        "And...",
                        ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(",")),
                        "Your exploits are known to me. But I am curious as to what has brought you here?"
                    ],
                )?;
                ctx.next()?;
                ctx.mes("^3355FFYou explained what happened to the Valkyrie, starting from the Snow Crystal which never melts.^000000")?;
                ctx.next()?;
                ctx.lines_as(
                    "Valkyrie",
                    args![
                        "I understand.",
                        "The story begins a long, long time ago with Freya, the patron goddess of beauty and love."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Valkyrie", args!["Her teardrop became a beautiful jewel. The snow on which Freya's teardrops have fallen will always retain its crystalline beauty."])?;
                ctx.next()?;
                ctx.lines_as("Valkyrie", args!["You seek traces of the goddess? Perhaps this is a sign of the end of the thousand year peace. When the time comes, I hope humans will not abandon their faith."])?;
                ctx.next()?;
                ctx.lines_as("Valkyrie", args!["Perhaps, I was also sent to meet you. I wish that humans will use the power derived from the traces of the gods to protect justice."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Valkyrie",
                    args![
                        ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("...")),
                        "Listen. This is",
                        "what you must find."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Valkyrie",
                    args![
                        "A cold snowy path on",
                        "which the goddess tread.",
                        "A blue river the goddess crossed.",
                        "A narrow path the goddess passed."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Valkyrie",
                    args![
                        "^4d4dffThe beauty of the stars",
                        "Wanes in comparison.",
                        "We lost our hearts",
                        "To that golden hair",
                        "And those dazzling eyes.^000000"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Valkyrie",
                    args!["Remember this.", "I shall send you", "back to where you", "belong."],
                )?;
                ctx.var("god_brising").set(Val::from(34))?;
                ctx.next()?;
                ctx.lines_as("Valkyrie", args!["For Odin's honor..."])?;
                ctx.close_window()?;
                ctx.call(Function::DisableNpc, vec![Val::from("#doppelganger1")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("#doppelganger2")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("#knight1")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("#knight2")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("#knight3")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("#lowen")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Soldier#1_brising")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Soldier#2_brising")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("#hermite")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Lowen Ellenen#2")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Valkyrie#1")])?;
                ctx.call(Function::Warp, vec![Val::from("geffen"), Val::from(120), Val::from(101)])?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    Ok(Val::from(0))
}

pub fn valkyrie_1(ctx: &Ctx) -> Script {
    valkyrie_1_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Brisindwarf1Step {
    Start,
    OnTouch,
}

fn brisindwarf1_run(ctx: &Ctx, mut step: Brisindwarf1Step, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_point = Val::from(0);
    'machine: loop {
        match step {
            Brisindwarf1Step::Start => {
                if runtime::op(&ctx.var("$god2").get()?, "<", &ctx.var("$@god_check1").get()?)?.is_true() {
                    ctx.lines(args!["^3355FFA lot of snow", "is heavily piled", "here on the ground.^000000"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if runtime::op(&ctx.var("$god3").get()?, ">=", &ctx.var("$@god_check2").get()?)?.is_true() {
                    ctx.lines(args!["^3355FFA lot of snow", "is heavily piled", "here on the ground.^000000"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if (ctx.var("god_brising").get()?.number()? > 39 && ctx.var("god_brising").get()?.number()? < 50) {
                    ctx.lines(args!["^3355FFA lot of snow", "is heavily piled", "here on the ground.^000000"])?;
                    ctx.next()?;
                    ctx.mes("^3355FFHowever, this particular mound strikes you as curious for some reason.^000000")?;
                    ctx.next()?;
                    'b1: {
                        let subject1 = Val::from(runtime::select_values(
                            ctx,
                            &[Val::from(
                                "Sweep the snow away with your hand.:Poke the snow with your finger.:Cancel.",
                            )],
                        )?);
                        let mut matched1 = false;
                        let no_case1 = !subject1.loosely_equals(&Val::from(1))
                            && !subject1.loosely_equals(&Val::from(2))
                            && !subject1.loosely_equals(&Val::from(3))
                            && !subject1.loosely_equals(&Val::from(4));
                        if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                            matched1 = true;
                        }
                        if matched1 {
                            ctx.mes("^3355FFAs you sweep the snow with the palm of your hand, some of the snow melts, revealing a strange puzzle cube. It's multi-faceted, but you can read that each facet is inscribed with a lyric.")?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("Adjust the puzzle.:Quit.")])? {
                                1 => {
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args!["Okay, um...", "I guess I should", "choose a lyric..."],
                                    )?;
                                    ctx.next()?;
                                    match runtime::select_values(
                                        ctx,
                                        &[Val::from(
                                            "Wanes in comparison:To that beautiful hair:All of our hearts:The beauty of the stars:To those dazzling eyes",
                                        )],
                                    )? {
                                        1 => {
                                            ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["we gave..."])?;
                                        }
                                        2 => {
                                            ctx.lines_as(
                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                args!["To that beautiful hair"],
                                            )?;
                                        }
                                        3 => {
                                            ctx.lines_as(
                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                args!["All of our hearts in"],
                                            )?;
                                        }
                                        4 => {
                                            ctx.lines_as(
                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                args!["The beauty of the stars"],
                                            )?;
                                            l_point = (l_point.clone() + Val::from(10));
                                        }
                                        5 => {
                                            ctx.lines_as(
                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                args!["To those dazzling eyes"],
                                            )?;
                                        }
                                        _ => {}
                                    }
                                    match runtime::select_values(
                                        ctx,
                                        &[Val::from(
                                            "We gave:Wanes in comparison.:All of our hearts in:To the steps in:To those dazzling eyes",
                                        )],
                                    )? {
                                        1 => {
                                            ctx.mes("we gave..")?;
                                        }
                                        2 => {
                                            ctx.mes("Wanes in comparison.")?;
                                            l_point = (l_point.clone() + Val::from(10));
                                        }
                                        3 => {
                                            ctx.mes("All of our hearts in")?;
                                        }
                                        4 => {
                                            ctx.mes("To the steps in")?;
                                        }
                                        5 => {
                                            ctx.mes("To those dazzling eyes")?;
                                        }
                                        _ => {}
                                    }
                                    match runtime::select_values(
                                        ctx,
                                        &[Val::from(
                                            "We gave:To that beautiful hair:All of our hearts in:To the steps in:We lost our hearts",
                                        )],
                                    )? {
                                        1 => {
                                            ctx.mes("We gave")?;
                                        }
                                        2 => {
                                            ctx.mes("To that beautiful hair")?;
                                        }
                                        3 => {
                                            ctx.mes("All of our hearts in")?;
                                        }
                                        4 => {
                                            ctx.mes("To the steps in")?;
                                        }
                                        5 => {
                                            ctx.mes("We lost our hearts")?;
                                            l_point = (l_point.clone() + Val::from(10));
                                        }
                                        _ => {}
                                    }
                                    match runtime::select_values(
                                        ctx,
                                        &[Val::from(
                                            "To that golden hair:To that beautiful hair:All of our hearts in:To the steps in:To those dazzling eyes",
                                        )],
                                    )? {
                                        1 => {
                                            ctx.mes("To that golden hair")?;
                                            l_point = (l_point.clone() + Val::from(10));
                                        }
                                        2 => {
                                            ctx.mes("To that beautiful hair")?;
                                        }
                                        3 => {
                                            ctx.mes("All of our hearts in")?;
                                        }
                                        4 => {
                                            ctx.mes("To the steps in")?;
                                        }
                                        5 => {
                                            ctx.mes("To those dazzling eyes")?;
                                        }
                                        _ => {}
                                    }
                                    match runtime::select_values(
                                        ctx,
                                        &[Val::from(
                                            "We gave:To that beautiful hair:And those dazzling eyes.:To the steps in:To those dazzling eyes",
                                        )],
                                    )? {
                                        1 => {
                                            ctx.mes("We gave")?;
                                        }
                                        2 => {
                                            ctx.mes("To that beautiful hair")?;
                                        }
                                        3 => {
                                            ctx.mes("And those dazzling eyes.")?;
                                            l_point = (l_point.clone() + Val::from(10));
                                        }
                                        4 => {
                                            ctx.mes("To the steps in")?;
                                        }
                                        5 => {
                                            ctx.mes("To those dazzling eyes")?;
                                        }
                                        _ => {}
                                    }
                                    ctx.next()?;
                                    if l_point.clone().number()? > 40 {
                                        ctx.call(Function::EnableNpc, vec![Val::from("Alfrik#1")])?;
                                        ctx.lines_as("Old dwarf", args!["Who are you?!", "And why the hell", "did you wake me up?"])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                2 => {
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args![
                                            "Huh...?",
                                            "Nothing happened.",
                                            "Did I do something wrong, or is this just a completely ordinary puzzle cube?"
                                        ],
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
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "This...",
                                    "This better not",
                                    "be just some regular",
                                    "puzzle cube that someone",
                                    "just dropped in the snow!"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        if !matched1 && subject1.loosely_equals(&Val::from(3)) {
                            matched1 = true;
                        }
                        if matched1 {
                            ctx.mes("^3355FFYou use your finger to poke the pile of snow. However, you hurt your finger a little bit after hitting something really solid.^000000")?;
                            ctx.call(
                                Function::Emotion,
                                vec![
                                    ctx.constant("ET_ANGER")?,
                                    Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                                ],
                            )?;
                            ctx.call(Function::PercentHeal, vec![Val::from(-20), Val::from(0)])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        if !matched1 && subject1.loosely_equals(&Val::from(4)) {
                            matched1 = true;
                        }
                        if matched1 {
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                } else {
                    ctx.lines(args!["^3355FFA lot of snow", "is heavily piled", "here on the ground.^000000"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                step = Brisindwarf1Step::OnTouch;
                continue 'machine;
            }
            Brisindwarf1Step::OnTouch => {
                if ctx.var("god_brising").get()?.number()? > 39 {
                    ctx.call(
                        Function::Emotion,
                        vec![
                            ctx.constant("ET_SURPRISE")?,
                            Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                        ],
                    )?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn brisindwarf1(ctx: &Ctx) -> Script {
    brisindwarf1_run(ctx, Brisindwarf1Step::Start, Vec::new()).map(|_| ())
}

pub fn brisindwarf1_ontouch(ctx: &Ctx) -> Script {
    brisindwarf1_run(ctx, Brisindwarf1Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Alfrik1Step {
    Start,
    OnInit,
}

fn alfrik_1_run(ctx: &Ctx, mut step: Alfrik1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Alfrik1Step::Start => {
                if ctx.var("god_brising").get()? == 49 {
                    ctx.lines_as(
                        "Alfrik",
                        args![
                            "I've got nothing",
                            "more to say! Now",
                            "get out of here!",
                            "I don't want to",
                            "be found!"
                        ],
                    )?;
                    ctx.close_window()?;
                    ctx.call(Function::DisableNpc, vec![Val::from("Alfrik#1")])?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("god_brising").get()? == 48 {
                        ctx.lines_as("Alfrik", args!["What's up, human?", "All of us have awakened."])?;
                        ctx.next()?;
                        match runtime::select_values(
                            ctx,
                            &[Val::from("I just wanted to say hello.:What are Brisingamen's materials?")],
                        )? {
                            1 => {
                                ctx.lines_as("Alfrik", args!["Your gesture is appreciated, but I'd rather hide than socialize. I don't want to be found by the crows of Odin!"])?;
                                ctx.close_window()?;
                                ctx.call(Function::DisableNpc, vec![Val::from("Alfrik#1")])?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    "Alfrik",
                                    args!["Materials for Brisingamen? For the human imitation, the materials are pretty easy to gather..."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Alfrik", args!["The power of weapons used by gods, when they battled demons a thousand years ago, and everything detached from Brisingamen. That's basically what you need."])?;
                                ctx.next()?;
                                ctx.lines_as("Alfrik", args!["If you want to", "create a Brisingamen,", "you will need....."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Alfrik",
                                    args![
                                        "^4d4dffFreya's Jewel",
                                        "Silver Ornament",
                                        "Snow Crystal",
                                        "Ripple",
                                        "Drifting Air^000000"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Alfrik", args!["You might need some gems, like Saphire, Peal, Opal, Curse Ruby, and maybe even some Gold to decorate the necklace."])?;
                                ctx.next()?;
                                ctx.lines_as("Alfrik", args!["Although I give freely of my advice, we four Dwarves will no longer do smithing work unless it be Freya's command."])?;
                                ctx.next()?;
                                ctx.lines_as("Alfrik", args!["But I can promise you one thing. Since the four of us have awakened, humans will obtain the Brisingamen for sure."])?;
                                ctx.next()?;
                                ctx.lines_as("Alfrik", args!["Now...", "Leave me", "in peace."])?;
                                ctx.var("god_brising").set(Val::from(49))?;
                                ctx.close_window()?;
                                ctx.call(Function::DisableNpc, vec![Val::from("Alfrik#1")])?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    } else if ctx.var("god_brising").get()? == 40 {
                        ctx.lines_as(
                            "Alfrik",
                            args![
                                "^333333*Yawn...*^000000",
                                "It's been a long time since I've been outside! W-wait! You're not..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Alfrik", args!["Who sent you?!", "Loki? Heimdall?", "Or was it Freya?"])?;
                        ctx.next()?;
                        'b2: {
                            let subject2 = Val::from(runtime::select_values(
                                ctx,
                                &[Val::from("No one, it was an accident!:It was Valkyrie.")],
                            )?);
                            let mut matched2 = false;
                            let no_case2 = !subject2.loosely_equals(&Val::from(1)) && !subject2.loosely_equals(&Val::from(2));
                            if !matched2 && subject2.loosely_equals(&Val::from(1)) {
                                matched2 = true;
                            }
                            if matched2 {
                                ctx.lines_as(
                                    "Alfrik",
                                    args![
                                        "Don't even think of lying to me, human! There's no way you could have awoken me without knowing",
                                        "the password! Now, speak!"
                                    ],
                                )?;
                                ctx.next()?;
                                match runtime::select_values(ctx, &[Val::from("Valkyrie told me.:It was a coincidence!")])? {
                                    1 => {
                                        ctx.lines_as(
                                            "Alfrik",
                                            args![
                                                "Valkyrie...?",
                                                "Odin's warmonger?",
                                                "What could Odin want,",
                                                "I haven't done anything wrong!"
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.mes("^3355FFYou explained everything to Alfrik, who seems to worry about being targeted by Odin.^000000")?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Alfrik",
                                            args![
                                                "So that's it.",
                                                "Seeking out our",
                                                "masterpiece? You",
                                                "humans must desire",
                                                "the power of the gods.",
                                                "Interesting."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Alfrik",
                                            args![
                                                "The time for us to revive may actually have come! Alright then,",
                                                "I, Alfrik shall cooperate with you humans."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("Alfrik", args!["After all, getting on Odin's good side like this is much better than being tortured by Loki."])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Alfrik",
                                            args![
                                                "Now...",
                                                "Understand that",
                                                "there can only be",
                                                "one true Brisingamen.",
                                                "After all, it's our",
                                                "masterpiece."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("Alfrik", args!["However, that doesn't mean an imitation, with the same power as that godly item, can't be made. Isn't it tempting, the power of a god?"])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Alfrik",
                                            args![
                                                "First things first. If you want to make the necklace, you must",
                                                "awaken all four of us. Because of Loki's threat, we all hid ourselves in different places."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("Alfrik", args!["Go wake all my brothers!", "Let's see if the Brisingamen can be made once again! First, go and wake Dvalin before the gods and giants find out!"])?;
                                        ctx.var("god_brising").set(Val::from(41))?;
                                        ctx.close_window()?;
                                        ctx.call(Function::DisableNpc, vec![Val::from("Alfrik#1")])?;
                                        return Err(Stop::End);
                                    }
                                    2 => {
                                        ctx.lines_as(
                                            "Alfrik",
                                            args![
                                                "Coincidence?",
                                                "Impossible! How dare you lie! Don't ever come back, or the gods will find me!"
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        ctx.call(Function::DisableNpc, vec![Val::from("Alfrik#1")])?;
                                        return Err(Stop::End);
                                    }
                                    _ => {}
                                }
                            }
                            if !matched2 && subject2.loosely_equals(&Val::from(2)) {
                                matched2 = true;
                            }
                            if matched2 {
                                ctx.lines_as(
                                    "Alfrik",
                                    args![
                                        "Valkyrie...?",
                                        "Odin's warmonger?",
                                        "What could Odin want,",
                                        "I haven't done anything wrong!"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.mes(
                                    "^3355FFYou explained everything to Alfrik, who seems to worry about being targeted by Odin.^000000",
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Alfrik",
                                    args![
                                        "So that's it.",
                                        "Seeking out our",
                                        "masterpiece? You",
                                        "humans must desire",
                                        "the power of the gods.",
                                        "Interesting."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Alfrik",
                                    args![
                                        "The time for us to revive may actually have come! Alright then,",
                                        "I, Alfrik shall cooperate with you humans."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Alfrik",
                                    args!["After all, getting on Odin's good side like this is much better than being tortured by Loki."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Alfrik",
                                    args![
                                        "Now...",
                                        "Understand that",
                                        "there can only be",
                                        "one true Brisingamen.",
                                        "After all, it's our",
                                        "masterpiece."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Alfrik", args!["However, that doesn't mean an imitation, with the same power as that godly item, can't be made. Isn't it tempting, the power of a god?"])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Alfrik",
                                    args![
                                        "First things first. If you want to make the necklace, you must",
                                        "awaken all four of us. Because of Loki's threat, we all hid ourselves in different places."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Alfrik", args!["Go wake all my brothers!", "Let's see if the Brisingamen can be made once again! First, go and wake Dvalin before the gods and giants find out!"])?;
                                ctx.var("god_brising").set(Val::from(41))?;
                                ctx.close_window()?;
                                ctx.call(Function::DisableNpc, vec![Val::from("Alfrik#1")])?;
                                return Err(Stop::End);
                            }
                        }
                    } else if ctx.var("god_brising").get()? == 41 {
                        ctx.lines_as("Alfrik", args!["Why are you still here?", "I told you go wake Dvalin!"])?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Where is he?:Tell me more about Brisingamen.")])? {
                            1 => {
                                ctx.lines_as(
                                    "Alfrik",
                                    args!["Ah. Right.", "I forgot to tell", "you. Poor Dvalin.", "Now, where was it?"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Alfrik", args!["We were afraid that Odin and Heimdall would punish us because, well, in their eyes we disgraced the goddess."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Alfrik",
                                    args!["Oh course, they never would have been angered if it weren't for Loki's trickery."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Alfrik", args!["We loved Freya so much that we decided to hide ourselves beneath the path that she walked. To be as close to any trace of her as we could."])?;
                                ctx.next()?;
                                ctx.lines_as("Alfrik", args!["Poor Dvalin is sleeping", "under the path in the East. Once there, you'll feel the scent the goddess. I can't remember where the others are sleeping, but you cannot forget these words..."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Alfrik",
                                    args![
                                        "^4d4dffHer lovely scent",
                                        "Still lingers in the wind.",
                                        "We surrendered our hearts",
                                        "To those tender teardrops",
                                        "Those seductive red lips.^000000"
                                    ],
                                )?;
                                ctx.var("god_brising").set(Val::from(42))?;
                                ctx.close_window()?;
                                ctx.call(Function::DisableNpc, vec![Val::from("Alfrik#1")])?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as("Alfrik", args!["Brisingamen is our masterpiece, a necklace we forged for Freya. Even if we tried making it again, I doubt it would be as good."])?;
                                ctx.next()?;
                                ctx.lines_as("Alfrik", args!["It looked perfect on Freya. Very difficult to create jewelry that actually enhances the attractiveness of the goddess of beauty."])?;
                                ctx.next()?;
                                ctx.lines_as("Alfrik", args!["Perhaps our greatest reward was that beautiful smile of satisfaction she gave us when that we gave her that necklace. We loved her so."])?;
                                ctx.next()?;
                                ctx.lines_as("Alfrik", args!["Is it that wrong for dwarves to love a goddess? Perhaps we were punished because the gods, even Odin, loved her as well."])?;
                                ctx.next()?;
                                ctx.lines_as("Alfrik", args!["In any case, we've never seen the true Brisingamen again. But something very similar to it can be created..."])?;
                                ctx.next()?;
                                ctx.lines_as("Alfrik", args!["You'll need materials that have been graced by the goddess' presense. Traces of the goddess in the earth, water and the wind."])?;
                                ctx.next()?;
                                ctx.lines_as("Alfrik", args!["And if the Brisingamen is recreated, the power of a god will be in the hands of a human. That human could be you!"])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Alfrik",
                                    args![
                                        "Poor Dvalin is sleeping under the path in the East. Once there, you'll feel the scent the goddess."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Alfrik",
                                    args!["I can't remember where the others are sleeping, but you cannot forget these words..."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Alfrik",
                                    args![
                                        "^4d4dffHer lovely scent",
                                        "Still lingers in the wind.",
                                        "We surrendered our hearts",
                                        "To those tender teardrops",
                                        "Those seductive red lips.^000000"
                                    ],
                                )?;
                                ctx.var("god_brising").set(Val::from(42))?;
                                ctx.close_window()?;
                                ctx.call(Function::DisableNpc, vec![Val::from("Alfrik#1")])?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    } else if ctx.var("god_brising").get()? == 42 {
                        ctx.lines_as(
                            "Alfrik",
                            args![
                                "Will you please!",
                                "Get lost before Loki and Heimdall find out I've awakened! Now hurry, find Dvalin, and wake him up!"
                            ],
                        )?;
                        ctx.close_window()?;
                        ctx.call(Function::DisableNpc, vec![Val::from("Alfrik#1")])?;
                        return Err(Stop::End);
                    } else if (ctx.var("god_brising").get()?.number()? > 42 && ctx.var("god_brising").get()?.number()? < 48) {
                        ctx.lines_as("Alfrik", args!["Oh, finally all of us have awakened! Now, the power of the goddess can be retrieved from the earth, water and the wind!"])?;
                        ctx.next()?;
                        ctx.lines_as("Alfrik", args!["Hahahahaha!", "This should be", "interesting!"])?;
                        ctx.close_window()?;
                        ctx.call(Function::DisableNpc, vec![Val::from("Alfrik#1")])?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as("Alfrik", args!["Mmm...?", "I've got", "no business", "with you."])?;
                        ctx.close_window()?;
                        ctx.call(Function::DisableNpc, vec![Val::from("Alfrik#1")])?;
                        return Err(Stop::End);
                    }
                }
                step = Alfrik1Step::OnInit;
                continue 'machine;
            }
            Alfrik1Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Alfrik#1")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn alfrik_1(ctx: &Ctx) -> Script {
    alfrik_1_run(ctx, Alfrik1Step::Start, Vec::new()).map(|_| ())
}

pub fn alfrik_1_oninit(ctx: &Ctx) -> Script {
    alfrik_1_run(ctx, Alfrik1Step::OnInit, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Brisindwarf2Step {
    Start,
    OnTouch,
}

fn brisindwarf2_run(ctx: &Ctx, mut step: Brisindwarf2Step, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_point = Val::from(0);
    'machine: loop {
        match step {
            Brisindwarf2Step::Start => {
                if runtime::op(&ctx.var("$god2").get()?, "<", &ctx.var("$@god_check1").get()?)?.is_true() {
                    ctx.lines(args!["^3355FFIt's just an", "ordinary rock.^000000"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if runtime::op(&ctx.var("$god3").get()?, ">=", &ctx.var("$@god_check2").get()?)?.is_true() {
                    ctx.lines(args!["^3355FFIt's just an", "ordinary rock.^000000"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ctx.var("god_brising").get()?.number()? > 41 {
                    ctx.lines(args![
                        "^3355FFIt's just an ordinary rock underneath the shadow of",
                        "a tree. But on closer inspection, you notice that the moss on top of the rock looks perfectly flat.^000000"
                    ])?;
                    ctx.next()?;
                    match runtime::select_values(
                        ctx,
                        &[Val::from("Step on it.:Sweep the surface.:Scratch the surface.:Kick it.")],
                    )? {
                        1 => {
                            ctx.lines(args![
                                "^3355FFYou gingerly place your foot on the top of the stone. The entire rock suddenly swings downward",
                                "into the ground, and you fall helplessly...^000000"
                            ])?;
                            ctx.close_window()?;
                            ctx.call(Function::Warp, vec![Val::from("prt_fild02"), Val::from(165), Val::from(224)])?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.mes("^3355FFAs you sweep away the moss with your hand, you scratch your palm from the sharp edges of the rock.^000000")?;
                            ctx.call(
                                Function::Emotion,
                                vec![
                                    ctx.constant("ET_SURPRISE")?,
                                    Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["Damn...!", "I'm bleeding!", "But why didn't", "anything else", "happen?!"],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["This rock had", "better be one of", "those secret rocks!", "Or else...!"],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        3 => {
                            ctx.lines(args![
                                "^3355FFYou scratch away",
                                "at the moss, and feel that the surface of the rock has even recesses."
                            ])?;
                            ctx.next()?;
                            ctx.mes("^3355FFAfter peeling off the moss, a series of tiles, etched with faint words, are revealed on top of the rock. These tiles could be moved around, like in a puzzle.^000000")?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "Another puzzle?",
                                    "This was just like when I had to give Alfrik's password. Okay, let me see..."
                                ],
                            )?;
                            l_point = Val::from(0);
                            ctx.next()?;
                            match runtime::select_values(
                                ctx,
                                &[Val::from(
                                    "To the tear drops dripping on the way:Her lovely scent:We gave:To the seducing red lips:Our hearts in",
                                )],
                            )? {
                                1 => {
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args!["To the tear drops dripping on the way"],
                                    )?;
                                }
                                2 => {
                                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Her lovely scent"])?;
                                    l_point = (l_point.clone() + Val::from(10));
                                }
                                3 => {
                                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["We gave"])?;
                                }
                                4 => {
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args!["To the seducing red lips"],
                                    )?;
                                }
                                5 => {
                                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Our hearts in"])?;
                                }
                                _ => {}
                            }
                            'b3: {
                                let subject3 = Val::from(runtime::select_values(
                                    ctx,
                                    &[Val::from(
                                        "Still lingers in the wind.:To the scent drifted in the wind:We gave:To the seducing red lips:Our hearts in",
                                    )],
                                )?);
                                let mut matched3 = false;
                                let no_case3 = !subject3.loosely_equals(&Val::from(1))
                                    && !subject3.loosely_equals(&Val::from(2))
                                    && !subject3.loosely_equals(&Val::from(3))
                                    && !subject3.loosely_equals(&Val::from(4))
                                    && !subject3.loosely_equals(&Val::from(5));
                                if !matched3 && subject3.loosely_equals(&Val::from(1)) {
                                    matched3 = true;
                                }
                                if matched3 {
                                    ctx.mes("Still lingers in the wind.")?;
                                    l_point = (l_point.clone() + Val::from(10));
                                    break 'b3;
                                }
                                if !matched3 && subject3.loosely_equals(&Val::from(2)) {
                                    matched3 = true;
                                }
                                if matched3 {
                                    ctx.mes("To the scent drifted in the wind")?;
                                    break 'b3;
                                }
                                if !matched3 && subject3.loosely_equals(&Val::from(3)) {
                                    matched3 = true;
                                }
                                if matched3 {
                                    ctx.mes("We gave")?;
                                    break 'b3;
                                }
                                if !matched3 && subject3.loosely_equals(&Val::from(4)) {
                                    matched3 = true;
                                }
                                if matched3 {
                                    ctx.mes("To the seducing red lips")?;
                                }
                                if !matched3 && subject3.loosely_equals(&Val::from(5)) {
                                    matched3 = true;
                                }
                                if matched3 {
                                    ctx.mes("Our hearts in")?;
                                    break 'b3;
                                }
                            }
                            'b4: {
                                let subject4 = Val::from(runtime::select_values(
                                    ctx,
                                    &[Val::from(
                                        "To the tear drops dripping on the way:To the scent drifted in the wind:We gave:We surrendered our hearts:Our hearts in",
                                    )],
                                )?);
                                let mut matched4 = false;
                                let no_case4 = !subject4.loosely_equals(&Val::from(1))
                                    && !subject4.loosely_equals(&Val::from(2))
                                    && !subject4.loosely_equals(&Val::from(3))
                                    && !subject4.loosely_equals(&Val::from(4))
                                    && !subject4.loosely_equals(&Val::from(5));
                                if !matched4 && subject4.loosely_equals(&Val::from(1)) {
                                    matched4 = true;
                                }
                                if matched4 {
                                    ctx.mes("To the tear drops dripping on the way")?;
                                    break 'b4;
                                }
                                if !matched4 && subject4.loosely_equals(&Val::from(2)) {
                                    matched4 = true;
                                }
                                if matched4 {
                                    ctx.mes("To the scent drifted in the wind")?;
                                    break 'b4;
                                }
                                if !matched4 && subject4.loosely_equals(&Val::from(3)) {
                                    matched4 = true;
                                }
                                if matched4 {
                                    ctx.mes("We gave")?;
                                    break 'b4;
                                }
                                if !matched4 && subject4.loosely_equals(&Val::from(4)) {
                                    matched4 = true;
                                }
                                if matched4 {
                                    ctx.mes("We surrendered our hearts")?;
                                    l_point = (l_point.clone() + Val::from(10));
                                }
                                if !matched4 && subject4.loosely_equals(&Val::from(5)) {
                                    matched4 = true;
                                }
                                if matched4 {
                                    ctx.mes("Our hearts in")?;
                                    break 'b4;
                                }
                            }
                            match runtime::select_values(
                                ctx,
                                &[Val::from(
                                    "To the tear drops dripping on the way:To the scent drifted in the wind:To those tender teardrops:To the seducing red lips:Our hearts in",
                                )],
                            )? {
                                1 => {
                                    ctx.mes("To the tear drops dripping on the way")?;
                                }
                                2 => {
                                    ctx.mes("To the scent drifted in the wind")?;
                                }
                                3 => {
                                    ctx.mes("To those tender teardrops")?;
                                    l_point = (l_point.clone() + Val::from(10));
                                }
                                4 => {
                                    ctx.mes("To the seducing red lips")?;
                                }
                                5 => {
                                    ctx.mes("Our hearts in")?;
                                }
                                _ => {}
                            }
                            match runtime::select_values(
                                ctx,
                                &[Val::from(
                                    "To the tear drops dripping on the way:To the scent drifted in the wind:We gave:To the seducing red lips:Those seductive red lips",
                                )],
                            )? {
                                1 => {
                                    ctx.mes("To the tear drops dripping on the way")?;
                                }
                                2 => {
                                    ctx.mes("To the scent drifted in the wind")?;
                                }
                                3 => {
                                    ctx.mes("We gave")?;
                                }
                                4 => {
                                    ctx.mes("To the seducing red lips")?;
                                }
                                5 => {
                                    ctx.mes("Those seductive red lips")?;
                                    l_point = (l_point.clone() + Val::from(10));
                                }
                                _ => {}
                            }
                            ctx.next()?;
                            if l_point.clone().number()? > 40 {
                                ctx.call(Function::EnableNpc, vec![Val::from("Dvalin#1")])?;
                                ctx.lines_as("Dvalin", args!["...?", "Alfrik?", "Is that you?"])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        4 => {
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["^333333*Cough cough!*", "Aaaack!", "It's so dusty!"],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else {
                    ctx.lines(args!["^3355FFJust a normal", "rock, covered in moss.^000000"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                step = Brisindwarf2Step::OnTouch;
                continue 'machine;
            }
            Brisindwarf2Step::OnTouch => {
                if ctx.var("god_brising").get()?.number()? > 41 {
                    ctx.call(
                        Function::Emotion,
                        vec![
                            ctx.constant("ET_SURPRISE")?,
                            Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                        ],
                    )?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn brisindwarf2(ctx: &Ctx) -> Script {
    brisindwarf2_run(ctx, Brisindwarf2Step::Start, Vec::new()).map(|_| ())
}

pub fn brisindwarf2_ontouch(ctx: &Ctx) -> Script {
    brisindwarf2_run(ctx, Brisindwarf2Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Dvalin1Step {
    Start,
    OnInit,
}

fn dvalin_1_run(ctx: &Ctx, mut step: Dvalin1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Dvalin1Step::Start => {
                if ctx.var("god_brising").get()?.number()? > 43 {
                    ctx.lines_as("Dvalin", args!["A human?", "You woke me up?"])?;
                    ctx.next()?;
                    ctx.lines_as("Dvalin", args!["Hmm...", "And you'd be acting pretty pushy right about now if you were acting on behalf of a god. What could you want...?"])?;
                    ctx.next()?;
                    ctx.lines_as("Dvalin", args!["It makes sense that you want a replica of Brisingamen for its power. But remember, we originally made Brisingamen as a necklace fit for a goddess."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Dvalin",
                        args!["Some might say that we seduced Freya with the necklace, but we really loved her with all of our hearts."],
                    )?;
                    ctx.close_window()?;
                    ctx.call(Function::DisableNpc, vec![Val::from("Dvalin#1")])?;
                    return Err(Stop::End);
                } else if ctx.var("god_brising").get()? == 42 {
                    ctx.lines_as(
                        "Dvalin",
                        args![
                            "Wah, it's not Alfrik?",
                            "Who are you to wake Dvalin?",
                            "Tell me right now, or I'll...",
                            "I'll kick your ass!"
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Help!:Alfrik sent me to wake you up!")])? {
                        1 => {
                            ctx.lines_as(
                                "Dvalin",
                                args!["I don't know", "what the hell", "you're doing here,", "but leave!"],
                            )?;
                            ctx.close_window()?;
                            ctx.call(Function::DisableNpc, vec![Val::from("Dvalin#1")])?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as(
                                "Dvalin",
                                args!["Did you just say Alfrik sent you? How do you know my brother?!"],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Dvalin",
                                args!["Brisingamen?!", "Then, that means", "we can meet Freya again?!"],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["Umm...", "I'm afraid not.", "Or, at least, that's", "not why I'm here."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Dvalin",
                                args![
                                    "Okay...",
                                    "I understand.",
                                    "Sorry, I was just",
                                    "hoping you were a",
                                    "herald of Freya."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Dvalin",
                                args![
                                    "So, an imitation",
                                    "of Brisingamen...",
                                    "Even if it's not the original, I'm sure it can possess power comparable to the real thing. Yes..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Dvalin", args!["It's no use trying to predict the future, but I can't help but wonder what would mankind do with the power of the gods?"])?;
                            ctx.next()?;
                            ctx.lines_as("Dvalin", args!["If my brother Alfrik already approved of you, then I'll help you out. I'll tell you how to wake my brother Berling."])?;
                            ctx.next()?;
                            ctx.lines_as("Dvalin", args!["We hid ourselves near traces of Freya. The path where I am staying is one that my goddess has walked through."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Dvalin",
                                args![
                                    "The air that quietly drifts here holds her fragrant scent. You can use this wind to borrow her power."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Dvalin", args!["Aside from Freya, I have", "no special love for the gods or giants. But I am intrigued by you humans. Your race is one of both love and hatred, life and destruction."])?;
                            ctx.next()?;
                            ctx.lines_as("Dvalin", args!["Go now and seek Berling. He is sleeping near a river linked to Mount Mjolnir. Travel West to find the river were Freya's Teardrops have fallen."])?;
                            ctx.next()?;
                            ctx.lines_as("Dvalin", args!["If the phantom of water gives you a question, answer it. The answer is the punishment that Odin gave to our goddess."])?;
                            ctx.next()?;
                            ctx.lines_as("Dvalin", args!["Until thousands of Valkyries filled up Valhala, they had to repeatedly live, die, then be reborn the next day only to die again."])?;
                            ctx.next()?;
                            ctx.lines_as("Dvalin", args!["Think about", "who they were.", "That's all I can tell you."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Dvalin",
                                args!["Somehow, I know", "you are the one who", "can answer the question.", "Hahahaha!"],
                            )?;
                            ctx.var("god_brising").set(Val::from(43))?;
                            ctx.close_window()?;
                            ctx.call(Function::DisableNpc, vec![Val::from("Dvalin#1")])?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else if ctx.var("god_brising").get()? == 43 {
                    ctx.lines_as(
                        "Dvalin",
                        args!["Why do you keep calling me? I did everything I can do for you. I even gave you the key to the question."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Dvalin",
                        args!["From now on,", "it's all up to you.", "Now, go wake Berling..."],
                    )?;
                    ctx.close_window()?;
                    ctx.call(Function::DisableNpc, vec![Val::from("Dvalin#1")])?;
                    return Err(Stop::End);
                }
                step = Dvalin1Step::OnInit;
                continue 'machine;
            }
            Dvalin1Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Dvalin#1")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn dvalin_1(ctx: &Ctx) -> Script {
    dvalin_1_run(ctx, Dvalin1Step::Start, Vec::new()).map(|_| ())
}

pub fn dvalin_1_oninit(ctx: &Ctx) -> Script {
    dvalin_1_run(ctx, Dvalin1Step::OnInit, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Brisindwarf3Step {
    Start,
    OnTouch,
}

fn brisindwarf3_run(ctx: &Ctx, mut step: Brisindwarf3Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Brisindwarf3Step::Start => {
                if runtime::op(&ctx.var("$god2").get()?, "<", &ctx.var("$@god_check1").get()?)?.is_true() {
                    ctx.mes("^3355FFThe sight of this bubbling stream refreshes you just by looking at it.^000000")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if runtime::op(&ctx.var("$god3").get()?, ">=", &ctx.var("$@god_check2").get()?)?.is_true() {
                    ctx.mes("^3355FFThe sight of this bubbling stream refreshes you just by looking at it.^000000")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if (ctx.var("god_brising").get()?.number()? > 42 && ctx.var("god_brising").get()?.number()? < 50) {
                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BUBBLE")?])?;
                    ctx.lines_as("Echoing Voice", args!["Alfrik: confirmed.", "Dvalin: confirmed."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Echoing Voice",
                        args![
                            "Please answer me.",
                            "Odin punished beautiful",
                            "Freya, and made her curse",
                            "some humans with her power."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Echoing Voice", args!["Freya caused an eternal battle, where the combatants were all killed and brought back to life everyday. How many humans were cursed to participate?"])?;
                    ctx.next()?;
                    match runtime::select_values(
                        ctx,
                        &[
                            Val::from("20!"),
                            Val::from("How should I know?!"),
                            Val::from("40!"),
                            Val::from("42!"),
                        ],
                    )? {
                        1 => {
                            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BUBBLE")?])?;
                            ctx.lines_as("Echoing Voice", args!["Failed.", "Failed..."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BUBBLE")?])?;
                            ctx.lines_as("Echoing Voice", args!["Failed.", "Failed..."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        3 => {
                            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BUBBLE")?])?;
                            ctx.lines_as("Echoing Voice", args!["Failed.", "Failed..."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        4 => {
                            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BUBBLE")?])?;
                            ctx.lines_as(
                                "Echoing Voice",
                                args!["...Confirmed.", "Shutting down barrier.", "Initiating summoning."],
                            )?;
                            ctx.next()?;
                            ctx.call(Function::EnableNpc, vec![Val::from("Berling#1")])?;
                            ctx.lines_as(
                                "Berling",
                                args![
                                    "^333333*Gasp--!*^000000",
                                    "Thought I was",
                                    "gonna drown to death!",
                                    "Who woke me up?"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else {
                    ctx.mes("^3355FFThe sight of this bubbling stream refreshes you just by looking at it.^000000")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                step = Brisindwarf3Step::OnTouch;
                continue 'machine;
            }
            Brisindwarf3Step::OnTouch => {
                if ctx.var("god_brising").get()?.number()? > 42 {
                    ctx.call(
                        Function::Emotion,
                        vec![
                            ctx.constant("ET_SURPRISE")?,
                            Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                        ],
                    )?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn brisindwarf3(ctx: &Ctx) -> Script {
    brisindwarf3_run(ctx, Brisindwarf3Step::Start, Vec::new()).map(|_| ())
}

pub fn brisindwarf3_ontouch(ctx: &Ctx) -> Script {
    brisindwarf3_run(ctx, Brisindwarf3Step::OnTouch, Vec::new()).map(|_| ())
}

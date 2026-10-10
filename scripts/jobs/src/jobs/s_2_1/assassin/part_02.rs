use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn beholder_asntest_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.var(".mymobs").set(Val::from(6))?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(62),
            Val::from(161),
            Val::from("Job change target"),
            Val::from(1002),
            Val::from(1),
            Val::from("Beholder#ASNTEST::OnMyMobDead"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(85),
            Val::from(169),
            Val::from("Job change target"),
            Val::from(1063),
            Val::from(1),
            Val::from("Beholder#ASNTEST::OnMyMobDead"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(88),
            Val::from(152),
            Val::from("Job change target"),
            Val::from(1002),
            Val::from(1),
            Val::from("Beholder#ASNTEST::OnMyMobDead"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(90),
            Val::from(143),
            Val::from("Job change target"),
            Val::from(1113),
            Val::from(1),
            Val::from("Beholder#ASNTEST::OnMyMobDead"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(74),
            Val::from(167),
            Val::from("Job change target"),
            Val::from(1031),
            Val::from(1),
            Val::from("Beholder#ASNTEST::OnMyMobDead"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(77),
            Val::from(173),
            Val::from("Job change target"),
            Val::from(1002),
            Val::from(1),
            Val::from("Beholder#ASNTEST::OnMyMobDead"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(62),
            Val::from(161),
            Val::from("Job change creature"),
            Val::from(1063),
            Val::from(1),
            Val::from("Beholder#ASNTEST::OnMyMobDead2"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(85),
            Val::from(169),
            Val::from("Job change creature"),
            Val::from(1031),
            Val::from(1),
            Val::from("Beholder#ASNTEST::OnMyMobDead2"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(79),
            Val::from(174),
            Val::from("Job change creature"),
            Val::from(1113),
            Val::from(1),
            Val::from("Beholder#ASNTEST::OnMyMobDead2"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(85),
            Val::from(156),
            Val::from("Job change creature"),
            Val::from(1063),
            Val::from(1),
            Val::from("Beholder#ASNTEST::OnMyMobDead2"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(74),
            Val::from(171),
            Val::from("Job change monster"),
            Val::from(1002),
            Val::from(1),
            Val::from("Beholder#ASNTEST::OnMyMobDead2"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(68),
            Val::from(173),
            Val::from("Job change dummy"),
            Val::from(1113),
            Val::from(1),
            Val::from("Beholder#ASNTEST::OnMyMobDead2"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(65),
            Val::from(158),
            Val::from("Battle test target"),
            Val::from(1002),
            Val::from(1),
            Val::from("Beholder#ASNTEST::OnMyMobDead2"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(60),
            Val::from(158),
            Val::from("Warrior test target"),
            Val::from(1113),
            Val::from(1),
            Val::from("Beholder#ASNTEST::OnMyMobDead2"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(64),
            Val::from(169),
            Val::from("Job change targets"),
            Val::from(1002),
            Val::from(1),
            Val::from("Beholder#ASNTEST::OnMyMobDead2"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(71),
            Val::from(173),
            Val::from("Jobs change target"),
            Val::from(1063),
            Val::from(1),
            Val::from("Beholder#ASNTEST::OnMyMobDead2"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(77),
            Val::from(172),
            Val::from("Please don't hit me"),
            Val::from(1002),
            Val::from(1),
            Val::from("Beholder#ASNTEST::OnMyMobDead2"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(76),
            Val::from(172),
            Val::from("Job change sample"),
            Val::from(1063),
            Val::from(1),
            Val::from("Beholder#ASNTEST::OnMyMobDead2"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(75),
            Val::from(172),
            Val::from("Not me"),
            Val::from(1113),
            Val::from(1),
            Val::from("Beholder#ASNTEST::OnMyMobDead2"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(67),
            Val::from(167),
            Val::from("I got yours right here"),
            Val::from(1063),
            Val::from(1),
            Val::from("Beholder#ASNTEST::OnMyMobDead2"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(86),
            Val::from(170),
            Val::from("Job changes target"),
            Val::from(1031),
            Val::from(1),
            Val::from("Beholder#ASNTEST::OnMyMobDead2"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(86),
            Val::from(171),
            Val::from("Job quest target"),
            Val::from(1002),
            Val::from(1),
            Val::from("Beholder#ASNTEST::OnMyMobDead2"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(85),
            Val::from(170),
            Val::from("Job target change"),
            Val::from(1113),
            Val::from(1),
            Val::from("Beholder#ASNTEST::OnMyMobDead2"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(89),
            Val::from(171),
            Val::from("Hit me"),
            Val::from(1063),
            Val::from(1),
            Val::from("Beholder#ASNTEST::OnMyMobDead2"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(85),
            Val::from(170),
            Val::from("Battle Monster"),
            Val::from(1031),
            Val::from(1),
            Val::from("Beholder#ASNTEST::OnMyMobDead2"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(89),
            Val::from(156),
            Val::from("Bouncer"),
            Val::from(1002),
            Val::from(1),
            Val::from("Beholder#ASNTEST::OnMyMobDead2"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(89),
            Val::from(156),
            Val::from("Mungamorp"),
            Val::from(1113),
            Val::from(1),
            Val::from("Beholder#ASNTEST::OnMyMobDead2"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(89),
            Val::from(156),
            Val::from("Battle test target"),
            Val::from(1063),
            Val::from(1),
            Val::from("Beholder#ASNTEST::OnMyMobDead2"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(89),
            Val::from(156),
            Val::from("Dew of the Battle field"),
            Val::from(1113),
            Val::from(1),
            Val::from("Beholder#ASNTEST::OnMyMobDead2"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(89),
            Val::from(156),
            Val::from("Tear of test"),
            Val::from(1031),
            Val::from(1),
            Val::from("Beholder#ASNTEST::OnMyMobDead2"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(83),
            Val::from(169),
            Val::from("Evil Druid"),
            Val::from(1002),
            Val::from(1),
            Val::from("Beholder#ASNTEST::OnMyMobDead2"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(63),
            Val::from(158),
            Val::from("Doppelganger"),
            Val::from(1063),
            Val::from(1),
            Val::from("Beholder#ASNTEST::OnMyMobDead2"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(63),
            Val::from(157),
            Val::from("Job change dummy"),
            Val::from(1002),
            Val::from(1),
            Val::from("Beholder#ASNTEST::OnMyMobDead2"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(64),
            Val::from(159),
            Val::from("Job ready dummy"),
            Val::from(1002),
            Val::from(1),
            Val::from("Beholder#ASNTEST::OnMyMobDead2"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(63),
            Val::from(159),
            Val::from("Job change ready"),
            Val::from(1063),
            Val::from(1),
            Val::from("Beholder#ASNTEST::OnMyMobDead2"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(63),
            Val::from(159),
            Val::from("Archer test target"),
            Val::from(1002),
            Val::from(1),
            Val::from("Beholder#ASNTEST::OnMyMobDead2"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(63),
            Val::from(159),
            Val::from("Swordman test target"),
            Val::from(1002),
            Val::from(1),
            Val::from("Beholder#ASNTEST::OnMyMobDead2"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(83),
            Val::from(148),
            Val::from("Thief test target"),
            Val::from(1002),
            Val::from(1),
            Val::from("Beholder#ASNTEST::OnMyMobDead2"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(82),
            Val::from(148),
            Val::from("Acolyte test target"),
            Val::from(1002),
            Val::from(1),
            Val::from("Beholder#ASNTEST::OnMyMobDead2"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(84),
            Val::from(148),
            Val::from("Merchant test target"),
            Val::from(1002),
            Val::from(1),
            Val::from("Beholder#ASNTEST::OnMyMobDead2"),
        ],
    )?;
    ctx.call(Function::InitNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn beholder_asntest_onenable(ctx: &Ctx) -> Script {
    beholder_asntest_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn beholder_asntest_onreset_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::KillMonster,
        vec![Val::from("in_moc_16"), Val::from("Beholder#ASNTEST::OnMyMobDead")],
    )?;
    ctx.call(
        Function::KillMonster,
        vec![Val::from("in_moc_16"), Val::from("Beholder#ASNTEST::OnMyMobDead2")],
    )?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("Standby Room#ASNTEST::OnStart")])?;
    return Err(Stop::End);
}

pub fn beholder_asntest_onreset(ctx: &Ctx) -> Script {
    beholder_asntest_onreset_body(ctx, Vec::new()).map(|_| ())
}

fn beholder_asntest_onresetmob_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::KillMonster,
        vec![Val::from("in_moc_16"), Val::from("Beholder#ASNTEST::OnMyMobDead")],
    )?;
    ctx.call(
        Function::KillMonster,
        vec![Val::from("in_moc_16"), Val::from("Beholder#ASNTEST::OnMyMobDead2")],
    )?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn beholder_asntest_onresetmob(ctx: &Ctx) -> Script {
    beholder_asntest_onresetmob_body(ctx, Vec::new()).map(|_| ())
}

fn beholder_asntest_onmymobdead_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.var(".mymobs").set((ctx.var(".mymobs").get()?.try_sub(Val::from(1))?))?;
    if ctx.var(".mymobs").get()?.number()? < 1 {
        ctx.call(
            Function::MapAnnounce,
            vec![
                Val::from("in_moc_16"),
                Val::from("You seem to be doing quite well. Keep it up!"),
                ctx.constant("BC_MAP")?,
            ],
        )?;
        ctx.var("assin_q").set(Val::from(3))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(8003), Val::from(8004)])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("timestopper#1::OnEnable")])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("Keeper of the Door#ASN::OnEnable")])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("Beholder#ASNTEST::OnResetmob")])?;
        ctx.var(".disabletraps").set(Val::from(1))?;
        ctx.call(Function::StopNpcTimer, vec![])?;
    } else {
        ctx.call(
            Function::MapAnnounce,
            vec![
                Val::from("in_moc_16"),
                Val::from("Okay, you're doing good! Hang in there, you're almost there!"),
                ctx.constant("BC_MAP")?,
            ],
        )?;
    }
    return Err(Stop::End);
}

pub fn beholder_asntest_onmymobdead(ctx: &Ctx) -> Script {
    beholder_asntest_onmymobdead_body(ctx, Vec::new()).map(|_| ())
}

fn beholder_asntest_onmymobdead2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("in_moc_16"),
            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from("! You made a mistake! I'm bringing you back!")),
            ctx.constant("BC_MAP")?,
        ],
    )?;
    ctx.var("assin_q").set(Val::from(2))?;
    ctx.call(Function::Warp, vec![Val::from("in_moc_16"), Val::from(19), Val::from(161)])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("Beholder#ASNTEST::OnReset")])?;
    return Err(Stop::End);
}

pub fn beholder_asntest_onmymobdead2(ctx: &Ctx) -> Script {
    beholder_asntest_onmymobdead2_body(ctx, Vec::new()).map(|_| ())
}

fn beholder_asntest_ontimer1000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("in_moc_16"),
            Val::from(" Okay, let the test begin!"),
            ctx.constant("BC_MAP")?,
        ],
    )?;
    return Err(Stop::End);
}

pub fn beholder_asntest_ontimer1000(ctx: &Ctx) -> Script {
    beholder_asntest_ontimer1000_body(ctx, Vec::new()).map(|_| ())
}

fn beholder_asntest_ontimer2000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("in_moc_16"),
            Val::from("As you've been told before, find and only kill monsters named 'Job change target!'"),
            ctx.constant("BC_MAP")?,
        ],
    )?;
    return Err(Stop::End);
}

pub fn beholder_asntest_ontimer2000(ctx: &Ctx) -> Script {
    beholder_asntest_ontimer2000_body(ctx, Vec::new()).map(|_| ())
}

fn beholder_asntest_ontimer3000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("in_moc_16"),
            Val::from("The purpose of this test is to examine your ability to quickly distinguish enemies from other people!"),
            ctx.constant("BC_MAP")?,
        ],
    )?;
    return Err(Stop::End);
}

pub fn beholder_asntest_ontimer3000(ctx: &Ctx) -> Script {
    beholder_asntest_ontimer3000_body(ctx, Vec::new()).map(|_| ())
}

fn beholder_asntest_ontimer4000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("in_moc_16"),
            Val::from("You will have 3 minutes for the test! We will inform you of every minute passed."),
            ctx.constant("BC_MAP")?,
        ],
    )?;
    return Err(Stop::End);
}

pub fn beholder_asntest_ontimer4000(ctx: &Ctx) -> Script {
    beholder_asntest_ontimer4000_body(ctx, Vec::new()).map(|_| ())
}

fn beholder_asntest_ontimer5000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("in_moc_16"),
            Val::from("Ok, now you've got exactly 3 minutes. Move! Move!"),
            ctx.constant("BC_MAP")?,
        ],
    )?;
    return Err(Stop::End);
}

pub fn beholder_asntest_ontimer5000(ctx: &Ctx) -> Script {
    beholder_asntest_ontimer5000_body(ctx, Vec::new()).map(|_| ())
}

fn beholder_asntest_ontimer65000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("in_moc_16"),
            Val::from("2 minutes left. As I've told you, get the 'Job change target' monsters!"),
            ctx.constant("BC_MAP")?,
        ],
    )?;
    return Err(Stop::End);
}

pub fn beholder_asntest_ontimer65000(ctx: &Ctx) -> Script {
    beholder_asntest_ontimer65000_body(ctx, Vec::new()).map(|_| ())
}

fn beholder_asntest_ontimer125000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![Val::from("in_moc_16"), Val::from("1 minute left."), ctx.constant("BC_MAP")?],
    )?;
    return Err(Stop::End);
}

pub fn beholder_asntest_ontimer125000(ctx: &Ctx) -> Script {
    beholder_asntest_ontimer125000_body(ctx, Vec::new()).map(|_| ())
}

fn beholder_asntest_ontimer180000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![Val::from("in_moc_16"), Val::from("5 seconds left..."), ctx.constant("BC_MAP")?],
    )?;
    return Err(Stop::End);
}

pub fn beholder_asntest_ontimer180000(ctx: &Ctx) -> Script {
    beholder_asntest_ontimer180000_body(ctx, Vec::new()).map(|_| ())
}

fn beholder_asntest_ontimer181000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![Val::from("in_moc_16"), Val::from("4 seconds left..."), ctx.constant("BC_MAP")?],
    )?;
    return Err(Stop::End);
}

pub fn beholder_asntest_ontimer181000(ctx: &Ctx) -> Script {
    beholder_asntest_ontimer181000_body(ctx, Vec::new()).map(|_| ())
}

fn beholder_asntest_ontimer182000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![Val::from("in_moc_16"), Val::from("3 seconds left..."), ctx.constant("BC_MAP")?],
    )?;
    return Err(Stop::End);
}

pub fn beholder_asntest_ontimer182000(ctx: &Ctx) -> Script {
    beholder_asntest_ontimer182000_body(ctx, Vec::new()).map(|_| ())
}

fn beholder_asntest_ontimer183000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![Val::from("in_moc_16"), Val::from("2 seconds left..."), ctx.constant("BC_MAP")?],
    )?;
    return Err(Stop::End);
}

pub fn beholder_asntest_ontimer183000(ctx: &Ctx) -> Script {
    beholder_asntest_ontimer183000_body(ctx, Vec::new()).map(|_| ())
}

fn beholder_asntest_ontimer184000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![Val::from("in_moc_16"), Val::from("1 second left."), ctx.constant("BC_MAP")?],
    )?;
    return Err(Stop::End);
}

pub fn beholder_asntest_ontimer184000(ctx: &Ctx) -> Script {
    beholder_asntest_ontimer184000_body(ctx, Vec::new()).map(|_| ())
}

fn beholder_asntest_ontimer185000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![Val::from("in_moc_16"), Val::from("Time's up!"), ctx.constant("BC_MAP")?],
    )?;
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("in_moc_16"),
            Val::from("Well, good job... If you wanted to waste your time. You'll have to try again!"),
            ctx.constant("BC_MAP")?,
        ],
    )?;
    return Err(Stop::End);
}

pub fn beholder_asntest_ontimer185000(ctx: &Ctx) -> Script {
    beholder_asntest_ontimer185000_body(ctx, Vec::new()).map(|_| ())
}

fn beholder_asntest_ontimer186000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::AreaWarp,
        vec![
            Val::from("in_moc_16"),
            Val::from(60),
            Val::from(136),
            Val::from(93),
            Val::from(177),
            Val::from("in_moc_16"),
            Val::from(19),
            Val::from(161),
        ],
    )?;
    return Err(Stop::End);
}

pub fn beholder_asntest_ontimer186000(ctx: &Ctx) -> Script {
    beholder_asntest_ontimer186000_body(ctx, Vec::new()).map(|_| ())
}

fn beholder_asntest_ontimer187000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DoNpcEvent, vec![Val::from("Beholder#ASNTEST::OnReset")])?;
    return Err(Stop::End);
}

pub fn beholder_asntest_ontimer187000(ctx: &Ctx) -> Script {
    beholder_asntest_ontimer187000_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum S011Step {
    Start,
    OnTouch,
}

fn s_01_1_run(ctx: &Ctx, mut step: S011Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            S011Step::Start => {
                step = S011Step::OnTouch;
                continue 'machine;
            }
            S011Step::OnTouch => {
                if ctx
                    .call(
                        Function::GetVariableOfNpc,
                        vec![Val::from(".disabletraps"), Val::from("Beholder#ASNTEST"), Val::from(0)],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("in_moc_16"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(", you're trapped. You will be sent back.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                    ctx.var("assin_q").set(Val::from(2))?;
                    ctx.call(Function::Warp, vec![Val::from("in_moc_16"), Val::from(19), Val::from(161)])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Beholder#ASNTEST::OnResetmob")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Standby Room#ASNTEST::OnStart")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn s_01_1(ctx: &Ctx) -> Script {
    s_01_1_run(ctx, S011Step::Start, Vec::new()).map(|_| ())
}

pub fn s_01_1_ontouch(ctx: &Ctx) -> Script {
    s_01_1_run(ctx, S011Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn keeper_of_the_door_asn(ctx: &Ctx) -> Script {
    keeper_of_the_door_asn_run(ctx, KeeperOfTheDoorAsnStep::Start, Vec::new()).map(|_| ())
}

pub fn keeper_of_the_door_asn_oninit(ctx: &Ctx) -> Script {
    keeper_of_the_door_asn_run(ctx, KeeperOfTheDoorAsnStep::OnInit, Vec::new()).map(|_| ())
}

pub fn keeper_of_the_door_asn_ontouch(ctx: &Ctx) -> Script {
    keeper_of_the_door_asn_run(ctx, KeeperOfTheDoorAsnStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn keeper_of_the_door_asn_onenable(ctx: &Ctx) -> Script {
    keeper_of_the_door_asn_run(ctx, KeeperOfTheDoorAsnStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn keeper_of_the_door_asn_ondisable(ctx: &Ctx) -> Script {
    keeper_of_the_door_asn_run(ctx, KeeperOfTheDoorAsnStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn timestopper_1(ctx: &Ctx) -> Script {
    timestopper_1_run(ctx, Timestopper1Step::Start, Vec::new()).map(|_| ())
}

pub fn timestopper_1_onenable(ctx: &Ctx) -> Script {
    timestopper_1_run(ctx, Timestopper1Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn timestopper_1_ontimer187000(ctx: &Ctx) -> Script {
    timestopper_1_run(ctx, Timestopper1Step::OnTimer187000, Vec::new()).map(|_| ())
}

pub fn timestopper_1_ondisable(ctx: &Ctx) -> Script {
    timestopper_1_run(ctx, Timestopper1Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn timestopper_1_onmymobdead(ctx: &Ctx) -> Script {
    timestopper_1_run(ctx, Timestopper1Step::OnMyMobDead, Vec::new()).map(|_| ())
}

fn thomas_asntest_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn thomas_asntest(ctx: &Ctx) -> Script {
    thomas_asntest_body(ctx, Vec::new()).map(|_| ())
}

fn thomas_asntest_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("assin_q").get()? == 4 {
        ctx.lines_as("Thomas", args!["Damn...! You look like you're in a lot of pain. ^666666*Sigh*^000000 Give me a second, let me try to restore your HP and SP..."])?;
        ctx.call(Function::PercentHeal, vec![Val::from(100), Val::from(100)])?;
        ctx.next()?;
        ctx.lines_as(
            "Thomas",
            args![
                "It looks like you're having a tough time. You're either trying too hard, or not trying hard",
                "enough, kid."
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("I'm gonna try it again!:I... I quit!")])? {
            1 => {
                ctx.lines_as("Thomas", args!["Hmm. Well, okay.", "Good luck out there."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Thomas",
                    args![
                        "Huh...",
                        "Quit the test, eh? Well, I guess you don't wanna waste any more of our time."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Thomas", args!["Oh hey, don't forget to save your respawn point in town."])?;
                ctx.close_window()?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("in_moc_16"),
                        (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(" got scared and quit the test...Who's Next?!")),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                ctx.var("assin_q").set(Val::from(0))?;
                ctx.var("assin_q2").set(Val::from(0))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(8004), Val::from(8000)])?;
                ctx.call(
                    Function::SavePoint,
                    vec![Val::from("in_moc_16"), Val::from(18), Val::from(14), Val::from(1), Val::from(1)],
                )?;
                ctx.call(Function::Warp, vec![Val::from("in_moc_16"), Val::from(18), Val::from(14)])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Standby Room#ASNTEST::OnStart")])?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    ctx.lines_as(
        "Thomas",
        args!["Hey, I'm Thomas. I'm in charge of testing your use of the hiding skill. Think you're up to it?"],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Thomas",
        args!["Listen. In this test, you can't kill any monsters. Your goal is to reach 'Barcardi' at the opposite side of this room."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Thomas",
        args!["So basically, get to the other side of this room and meet 'Barcardi' without killing a single monster. Understand?"],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Thomas",
        args!["If you run away, get a nose bleed and pass out or something like that, I'll fail ya'. Enough talk. Let's see what you got."],
    )?;
    ctx.close_window()?;
    ctx.var("assin_q").set(Val::from(4))?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(81),
            Val::from(77),
            Val::from("Mummy"),
            Val::from(1041),
            Val::from(1),
            Val::from("timestopper#1::OnMyMobDead"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(82),
            Val::from(77),
            Val::from("Mummy"),
            Val::from(1041),
            Val::from(1),
            Val::from("timestopper#1::OnMyMobDead"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(83),
            Val::from(77),
            Val::from("Mummy"),
            Val::from(1041),
            Val::from(1),
            Val::from("timestopper#1::OnMyMobDead"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(84),
            Val::from(77),
            Val::from("Mummy"),
            Val::from(1041),
            Val::from(1),
            Val::from("timestopper#1::OnMyMobDead"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(85),
            Val::from(77),
            Val::from("Mummy"),
            Val::from(1041),
            Val::from(1),
            Val::from("timestopper#1::OnMyMobDead"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(86),
            Val::from(77),
            Val::from("Mummy"),
            Val::from(1041),
            Val::from(1),
            Val::from("timestopper#1::OnMyMobDead"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(88),
            Val::from(77),
            Val::from("Mummy"),
            Val::from(1041),
            Val::from(1),
            Val::from("timestopper#1::OnMyMobDead"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(89),
            Val::from(77),
            Val::from("Mummy"),
            Val::from(1041),
            Val::from(1),
            Val::from("timestopper#1::OnMyMobDead"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(90),
            Val::from(77),
            Val::from("Mummy"),
            Val::from(1041),
            Val::from(1),
            Val::from("timestopper#1::OnMyMobDead"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(77),
            Val::from(77),
            Val::from("Mummy"),
            Val::from(1041),
            Val::from(1),
            Val::from("timestopper#1::OnMyMobDead"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(78),
            Val::from(56),
            Val::from("Mummy"),
            Val::from(1041),
            Val::from(1),
            Val::from("timestopper#1::OnMyMobDead"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(79),
            Val::from(56),
            Val::from("Mummy"),
            Val::from(1041),
            Val::from(1),
            Val::from("timestopper#1::OnMyMobDead"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(80),
            Val::from(56),
            Val::from("Mummy"),
            Val::from(1041),
            Val::from(1),
            Val::from("timestopper#1::OnMyMobDead"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(81),
            Val::from(56),
            Val::from("Mummy"),
            Val::from(1041),
            Val::from(1),
            Val::from("timestopper#1::OnMyMobDead"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(91),
            Val::from(55),
            Val::from("Mummy"),
            Val::from(1041),
            Val::from(1),
            Val::from("timestopper#1::OnMyMobDead"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(92),
            Val::from(56),
            Val::from("Mummy"),
            Val::from(1041),
            Val::from(1),
            Val::from("timestopper#1::OnMyMobDead"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(93),
            Val::from(56),
            Val::from("Mummy"),
            Val::from(1041),
            Val::from(1),
            Val::from("timestopper#1::OnMyMobDead"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(94),
            Val::from(56),
            Val::from("Mummy"),
            Val::from(1041),
            Val::from(1),
            Val::from("timestopper#1::OnMyMobDead"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(95),
            Val::from(56),
            Val::from("Mummy"),
            Val::from(1041),
            Val::from(1),
            Val::from("timestopper#1::OnMyMobDead"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(96),
            Val::from(56),
            Val::from("Mummy"),
            Val::from(1041),
            Val::from(1),
            Val::from("timestopper#1::OnMyMobDead"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(97),
            Val::from(56),
            Val::from("Mummy"),
            Val::from(1041),
            Val::from(1),
            Val::from("timestopper#1::OnMyMobDead"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(76),
            Val::from(62),
            Val::from("Hydra"),
            Val::from(1068),
            Val::from(1),
            Val::from("timestopper#1::OnMyMobDead"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(79),
            Val::from(62),
            Val::from("Hydra"),
            Val::from(1068),
            Val::from(1),
            Val::from("timestopper#1::OnMyMobDead"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(79),
            Val::from(65),
            Val::from("Hydra"),
            Val::from(1068),
            Val::from(1),
            Val::from("timestopper#1::OnMyMobDead"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(76),
            Val::from(65),
            Val::from("Hydra"),
            Val::from(1068),
            Val::from(1),
            Val::from("timestopper#1::OnMyMobDead"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(96),
            Val::from(62),
            Val::from("Hydra"),
            Val::from(1068),
            Val::from(1),
            Val::from("timestopper#1::OnMyMobDead"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(96),
            Val::from(65),
            Val::from("Hydra"),
            Val::from(1068),
            Val::from(1),
            Val::from("timestopper#1::OnMyMobDead"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(99),
            Val::from(62),
            Val::from("Hydra"),
            Val::from(1068),
            Val::from(1),
            Val::from("timestopper#1::OnMyMobDead"),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("in_moc_16"),
            Val::from(99),
            Val::from(65),
            Val::from("Hydra"),
            Val::from(1068),
            Val::from(1),
            Val::from("timestopper#1::OnMyMobDead"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn thomas_asntest_ontouch(ctx: &Ctx) -> Script {
    thomas_asntest_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn thomas_asntest_ondisable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DoNpcEvent, vec![Val::from("Standby Room#ASNTEST::OnStart")])?;
    ctx.call(
        Function::KillMonster,
        vec![Val::from("in_moc_16"), Val::from("timestopper#1::OnMyMobDead")],
    )?;
    return Err(Stop::End);
}

pub fn thomas_asntest_ondisable(ctx: &Ctx) -> Script {
    thomas_asntest_ondisable_body(ctx, Vec::new()).map(|_| ())
}

fn barcardi_asn_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn barcardi_asn(ctx: &Ctx) -> Script {
    barcardi_asn_body(ctx, Vec::new()).map(|_| ())
}

fn barcardi_asn_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DoNpcEvent, vec![Val::from("timestopper#1::OnDisable")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("Thomas#ASNTEST::OnDisable")])?;
    ctx.lines_as(
        "Barcardi",
        args!["Oh! Congratulations!", "You may now proceed to our Guildmaster's room. Good luck!!"],
    )?;
    ctx.close_window()?;
    ctx.var("assin_q").set(Val::from(5))?;
    ctx.call(Function::ChangeQuest, vec![Val::from(8004), Val::from(8005)])?;
    ctx.call(Function::Warp, vec![Val::from("in_moc_16"), Val::from(181), Val::from(183)])?;
    return Err(Stop::End);
}

pub fn barcardi_asn_ontouch(ctx: &Ctx) -> Script {
    barcardi_asn_ontouch_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum MazeAssistantStep {
    Start,
    OnTouch,
}

fn maze_assistant_run(ctx: &Ctx, mut step: MazeAssistantStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MazeAssistantStep::Start => {
                step = MazeAssistantStep::OnTouch;
                continue 'machine;
            }
            MazeAssistantStep::OnTouch => {
                if (ctx.var("assin_q").get()? == 5 || ctx.var("assin_q").get()? == 6) {
                    ctx.call(Function::Warp, vec![Val::from("in_moc_16"), Val::from(181), Val::from(183)])?;
                    ctx.var("assin_q").set((ctx.var("assin_q").get()? + Val::from(1)))?;
                    if !(ctx.call(Function::IsBeginQuest, vec![Val::from(8006)])?.is_true()) {
                        ctx.call(Function::ChangeQuest, vec![Val::from(8005), Val::from(8006)])?;
                    }
                } else {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("in_moc_16"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(" has entered 'Guildmaster's room.'")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                    ctx.call(
                        Function::SavePoint,
                        vec![Val::from("in_moc_16"), Val::from(181), Val::from(183), Val::from(1), Val::from(1)],
                    )?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Guildmaster#ASN1::OnCast")])?;
                    ctx.call(Function::Warp, vec![Val::from("in_moc_16"), Val::from(167), Val::from(113)])?;
                    return Err(Stop::End);
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn maze_assistant(ctx: &Ctx) -> Script {
    maze_assistant_run(ctx, MazeAssistantStep::Start, Vec::new()).map(|_| ())
}

pub fn maze_assistant_ontouch(ctx: &Ctx) -> Script {
    maze_assistant_run(ctx, MazeAssistantStep::OnTouch, Vec::new()).map(|_| ())
}

fn guildmaster_asn1_run(ctx: &Ctx, mut step: GuildmasterAsn1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            GuildmasterAsn1Step::Start => {
                step = GuildmasterAsn1Step::OnTouch;
                continue 'machine;
            }
            GuildmasterAsn1Step::OnTouch => {
                ctx.call(
                    Function::SavePoint,
                    vec![Val::from("in_moc_16"), Val::from(167), Val::from(110), Val::from(1), Val::from(1)],
                )?;
                ctx.lines_as(
                    "Guildmaster",
                    args![
                        "Welcome. ",
                        "This place is called the 'Guildmaster's room,' the deepest place in the Assassin guild."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Guildmaster", args!["I'm going to give you a simple test. Please find your way through this maze and come to me. It is this maze that protects our guild from intruders."])?;
                ctx.next()?;
                ctx.lines_as("Guildmaster", args!["I look forward", "to meeting you", "at the end of maze."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            GuildmasterAsn1Step::OnCast => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("in_moc_16"),
                        Val::from("...Next volunteer, please come in."),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn guildmaster_asn1(ctx: &Ctx) -> Script {
    guildmaster_asn1_run(ctx, GuildmasterAsn1Step::Start, Vec::new()).map(|_| ())
}

pub fn guildmaster_asn1_ontouch(ctx: &Ctx) -> Script {
    guildmaster_asn1_run(ctx, GuildmasterAsn1Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn guildmaster_asn1_oncast(ctx: &Ctx) -> Script {
    guildmaster_asn1_run(ctx, GuildmasterAsn1Step::OnCast, Vec::new()).map(|_| ())
}

fn guildmaster_asn2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn guildmaster_asn2(ctx: &Ctx) -> Script {
    guildmaster_asn2_body(ctx, Vec::new()).map(|_| ())
}

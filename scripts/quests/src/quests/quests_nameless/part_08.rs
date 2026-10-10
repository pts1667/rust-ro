use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum Entrancecheck2Step {
    Start,
    OnTouch,
}

fn entrancecheck_2_run(ctx: &Ctx, mut step: Entrancecheck2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Entrancecheck2Step::Start => {
                step = Entrancecheck2Step::OnTouch;
                continue 'machine;
            }
            Entrancecheck2Step::OnTouch => {
                if ((ctx.var("zdan_edq").get()? == 15 || ctx.var("zdan_edq").get()? == 16) && ctx.var("$@monster_zgang").get()? == 0) {
                    ctx.var("$@monster_zgang").set(Val::from(1))?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("#zdan_broad::OnEnable")])?;
                } else if ((ctx.var("zdan_edq").get()? == 15 || ctx.var("zdan_edq").get()? == 16)
                    && ctx.var("$@monster_zgang").get()?.number()? > 0)
                {
                    return Err(Stop::End);
                } else if ctx.var("zdan_edq").get()? == 17 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("#ZGuard::OnDisable")])?;
                    ctx.call(Function::EnableNpc, vec![Val::from("Louis")])?;
                    ctx.call(Function::EnableNpc, vec![Val::from("Martha")])?;
                    ctx.call(Function::EnableNpc, vec![Val::from("Catfoii")])?;
                } else {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Where am I...?",
                            "Something has gone",
                            "terribly wrong, hasn't",
                            "it? Let me go baaaack~! "
                        ],
                    )?;
                    ctx.close_window()?;
                    ctx.var("$@monster_zgang").set(Val::from(0))?;
                    ctx.var("$@door2").set(Val::from(0))?;
                    ctx.call(Function::Warp, vec![Val::from("moc_fild17"), Val::from(209), Val::from(235)])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn entrancecheck_2(ctx: &Ctx) -> Script {
    entrancecheck_2_run(ctx, Entrancecheck2Step::Start, Vec::new()).map(|_| ())
}

pub fn entrancecheck_2_ontouch(ctx: &Ctx) -> Script {
    entrancecheck_2_run(ctx, Entrancecheck2Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn zdan_broad(ctx: &Ctx) -> Script {
    zdan_broad_run(ctx, ZdanBroadStep::Start, Vec::new()).map(|_| ())
}

pub fn zdan_broad_oninit(ctx: &Ctx) -> Script {
    zdan_broad_run(ctx, ZdanBroadStep::OnInit, Vec::new()).map(|_| ())
}

pub fn zdan_broad_onenable(ctx: &Ctx) -> Script {
    zdan_broad_run(ctx, ZdanBroadStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn zdan_broad_ondisable(ctx: &Ctx) -> Script {
    zdan_broad_run(ctx, ZdanBroadStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn zdan_broad_ontimer3000(ctx: &Ctx) -> Script {
    zdan_broad_run(ctx, ZdanBroadStep::OnTimer3000, Vec::new()).map(|_| ())
}

pub fn zdan_broad_ontimer5000(ctx: &Ctx) -> Script {
    zdan_broad_run(ctx, ZdanBroadStep::OnTimer5000, Vec::new()).map(|_| ())
}

pub fn zdan_broad_ontimer7000(ctx: &Ctx) -> Script {
    zdan_broad_run(ctx, ZdanBroadStep::OnTimer7000, Vec::new()).map(|_| ())
}

pub fn zdan_broad_ontimer9000(ctx: &Ctx) -> Script {
    zdan_broad_run(ctx, ZdanBroadStep::OnTimer9000, Vec::new()).map(|_| ())
}

pub fn zdan_broad_ontimer11000(ctx: &Ctx) -> Script {
    zdan_broad_run(ctx, ZdanBroadStep::OnTimer11000, Vec::new()).map(|_| ())
}

pub fn zdan_broad_ontimer13000(ctx: &Ctx) -> Script {
    zdan_broad_run(ctx, ZdanBroadStep::OnTimer13000, Vec::new()).map(|_| ())
}

pub fn zdan_broad_ontimer15000(ctx: &Ctx) -> Script {
    zdan_broad_run(ctx, ZdanBroadStep::OnTimer15000, Vec::new()).map(|_| ())
}

pub fn zdan_broad_ontimer18000(ctx: &Ctx) -> Script {
    zdan_broad_run(ctx, ZdanBroadStep::OnTimer18000, Vec::new()).map(|_| ())
}

pub fn zdan_broad_ontimer21000(ctx: &Ctx) -> Script {
    zdan_broad_run(ctx, ZdanBroadStep::OnTimer21000, Vec::new()).map(|_| ())
}

pub fn zdan_broad_ontimer300000(ctx: &Ctx) -> Script {
    zdan_broad_run(ctx, ZdanBroadStep::OnTimer300000, Vec::new()).map(|_| ())
}

pub fn zdan_broad_ontimer350000(ctx: &Ctx) -> Script {
    zdan_broad_run(ctx, ZdanBroadStep::OnTimer350000, Vec::new()).map(|_| ())
}

pub fn zguard(ctx: &Ctx) -> Script {
    zguard_run(ctx, ZguardStep::Start, Vec::new()).map(|_| ())
}

pub fn zguard_oninit(ctx: &Ctx) -> Script {
    zguard_run(ctx, ZguardStep::OnInit, Vec::new()).map(|_| ())
}

pub fn zguard_onenable(ctx: &Ctx) -> Script {
    zguard_run(ctx, ZguardStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn zguard_ondisable(ctx: &Ctx) -> Script {
    zguard_run(ctx, ZguardStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn zguard_onreset(ctx: &Ctx) -> Script {
    zguard_run(ctx, ZguardStep::OnReset, Vec::new()).map(|_| ())
}

pub fn zguard_onmymobdead(ctx: &Ctx) -> Script {
    zguard_run(ctx, ZguardStep::OnMyMobDead, Vec::new()).map(|_| ())
}

pub fn zguard_ontimer300000(ctx: &Ctx) -> Script {
    zguard_run(ctx, ZguardStep::OnTimer300000, Vec::new()).map(|_| ())
}

fn louis_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(907), Val::from(200)])? == 0 {
        ctx.lines_as(
            "Louis",
            args![
                "You mess with the",
                "Z Gang, you--wait.",
                "Let's just do this later.",
                "You know, when you're carrying",
                "fewer items with you. Then,",
                "we can rumble in peace."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("zdan_edq").get()? == 17 {
        ctx.lines_as(
            "Louis",
            args![
                "What happened?",
                "The big guns were",
                "supposed to come out!",
                "Why aren't we invisible",
                "anymore? What happened?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Catfoii",
            args![
                "You pushed the button",
                "to turn off the invisibility,",
                "meow! Did you really have",
                "to ask?! It's your fault!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Martha", args!["Argh! I should never", "have trusted that fool!"])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["So... Z Gang...", "And cat. It's time", "to turn you in."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Louis",
            args![
                "I... I can't accept",
                "our defeat! The Z Gang",
                "won't go down so easily!",
                "Come forth, my demon",
                "spawn minions of doom!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["I don't think", "they're coming."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Louis",
            args![
                "...............................",
                "We surrender. Argh, but",
                "we were so close to",
                "conquering the world!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Now if you'll just", "hand over the Book", "of Forbidden Mystery..."],
        )?;
        ctx.next()?;
        ctx.lines_as("Catfoii", args!["Louis, meow!", "Don't do it!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Martha",
            args!["Let's just surrender,", "Louis. It looks like", "the Z Gang is finished."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Anyway, the Rogue Guild",
                "is on its way to come get",
                "you guys. You won't be able",
                "to escape this time."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Louis",
            args![
                "Awwww, man!",
                "Fine, take the stupid",
                "ol' book. I couldn't",
                "use it that good anyway."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Louis",
            args![
                "Catfoii, would you",
                "lead this adventurer guy",
                "to the exit? I can't bear",
                "to look at the face that's",
                "caused my tragic downfall..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Catfoii", args!["Fine, meow.", "I understand.", "But how did we lose?!"])?;
        ctx.close_window()?;
        ctx.call(Function::GetItem, vec![Val::from(7724), Val::from(1)])?;
        ctx.call(Function::ChangeQuest, vec![Val::from(3133), Val::from(3134)])?;
        ctx.var("zdan_edq").set(Val::from(18))?;
        ctx.call(
            Function::MapWarp,
            vec![Val::from("z_agit"), Val::from("moc_fild17"), Val::from(209), Val::from(235)],
        )?;
        ctx.var("$@monster_zgang").set(Val::from(0))?;
        ctx.var("$@door2").set(Val::from(0))?;
        ctx.call(Function::DisableNpc, vec![Val::from("Louis")])?;
        ctx.call(Function::DisableNpc, vec![Val::from("Martha")])?;
        ctx.call(Function::DisableNpc, vec![Val::from("Catfoii")])?;
        ctx.call(Function::StopNpcTimer, vec![])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Where am I...?",
                "Something has gone",
                "terribly wrong, hasn't",
                "it? Let me go baaaack~! "
            ],
        )?;
        ctx.var("zdan_edq").set(Val::from(15))?;
        ctx.var("$@monster_zgang").set(Val::from(0))?;
        ctx.var("$@door2").set(Val::from(0))?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("moc_fild17"), Val::from(209), Val::from(235)])?;
        return Err(Stop::End);
    }
}

pub fn louis(ctx: &Ctx) -> Script {
    louis_body(ctx, Vec::new()).map(|_| ())
}

fn louis_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Louis")])?;
    return Err(Stop::End);
}

pub fn louis_oninit(ctx: &Ctx) -> Script {
    louis_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn martha_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(907), Val::from(200)])? == 0 {
        ctx.lines_as(
            "Martha",
            args![
                "Hey, you're carrying",
                "too much stuff. You'd",
                "better put some of your",
                "items away first in Kafra",
                "Storage or something."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("zdan_edq").get()? == 17 {
        ctx.lines_as("Martha", args!["Argh...! What'd ", "you do, Louis?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Louis",
            args![
                "What happened?",
                "The big guns were",
                "supposed to come out!",
                "Why aren't we invisible",
                "anymore? What happened?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Catfoii",
            args![
                "You pushed the button",
                "to turn off the invisibility,",
                "meow! Did you really have",
                "to ask?! It's your fault!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Martha", args!["Argh! I should never", "have trusted that fool!"])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["So... Z Gang...", "And cat. It's time", "to turn you in."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Louis",
            args![
                "I... I can't accept",
                "our defeat! The Z Gang",
                "won't go down so easily!",
                "Come forth, my demon",
                "spawn minions of doom!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["I don't think", "they're coming."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Louis",
            args![
                "...............................",
                "We surrender. Argh, but",
                "we were so close to",
                "conquering the world!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Now if you'll just", "hand over the Book", "of Forbidden Mystery..."],
        )?;
        ctx.next()?;
        ctx.lines_as("Catfoii", args!["Louis, meow!", "Don't do it!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Martha",
            args!["Let's just surrender,", "Louis. It looks like", "the Z Gang is finished."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Anyway, the Rogue Guild",
                "is on its way to come get",
                "you guys. You won't be able",
                "to escape this time."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Louis",
            args![
                "Awwww, man!",
                "Fine, take the stupid",
                "ol' book. I couldn't",
                "use it that good anyway."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Louis",
            args![
                "Catfoii, would you",
                "lead this adventurer guy",
                "to the exit? I can't bear",
                "to look at the face that's",
                "caused my tragic downfall..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Catfoii", args!["Fine, meow.", "I understand.", "But how did we lose?!"])?;
        ctx.close_window()?;
        ctx.call(Function::GetItem, vec![Val::from(7724), Val::from(1)])?;
        ctx.call(Function::ChangeQuest, vec![Val::from(3133), Val::from(3134)])?;
        ctx.var("zdan_edq").set(Val::from(18))?;
        ctx.call(
            Function::MapWarp,
            vec![Val::from("z_agit"), Val::from("moc_fild17"), Val::from(209), Val::from(235)],
        )?;
        ctx.var("$@monster_zgang").set(Val::from(0))?;
        ctx.var("$@door2").set(Val::from(0))?;
        ctx.call(Function::DisableNpc, vec![Val::from("Louis")])?;
        ctx.call(Function::DisableNpc, vec![Val::from("Martha")])?;
        ctx.call(Function::DisableNpc, vec![Val::from("Catfoii")])?;
        ctx.call(Function::DisableNpc, vec![Val::from("#zdan_broad")])?;
        ctx.call(Function::StopNpcTimer, vec![])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Where am I...?",
                "Something has gone",
                "terribly wrong, hasn't",
                "it? Let me go baaaack~! "
            ],
        )?;
        ctx.var("zdan_edq").set(Val::from(15))?;
        ctx.var("$@monster_zgang").set(Val::from(0))?;
        ctx.var("$@door2").set(Val::from(0))?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("moc_fild17"), Val::from(209), Val::from(235)])?;
        return Err(Stop::End);
    }
}

pub fn martha(ctx: &Ctx) -> Script {
    martha_body(ctx, Vec::new()).map(|_| ())
}

fn martha_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Martha")])?;
    return Err(Stop::End);
}

pub fn martha_oninit(ctx: &Ctx) -> Script {
    martha_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn catfoii_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(907), Val::from(200)])? == 0 {
        ctx.lines_as(
            "Catfoii",
            args![
                "You're carrying too",
                "many items: get rid of",
                "your extra stuff, put it in",
                "Kafra Storage, sell it, or",
                "whatever, before coming back."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("zdan_edq").get()? == 17 {
        ctx.lines_as(
            "Louis",
            args![
                "What happened?",
                "The big guns were",
                "supposed to come out!",
                "Why aren't we invisible",
                "anymore? What happened?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Catfoii",
            args![
                "You pushed the button",
                "to turn off the invisibility,",
                "meow! Did you really have",
                "to ask?! It's your fault!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Martha", args!["Argh! I should never", "have trusted that fool!"])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["So... Z Gang...", "And cat. It's time", "to turn you in."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Louis",
            args![
                "I... I can't accept",
                "our defeat! The Z Gang",
                "won't go down so easily!",
                "Come forth, my demon",
                "spawn minions of doom!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["I don't think", "they're coming."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Louis",
            args![
                "...............................",
                "We surrender. Argh, but",
                "we were so close to",
                "conquering the world!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Now if you'll just", "hand over the Book", "of Forbidden Mystery..."],
        )?;
        ctx.next()?;
        ctx.lines_as("Catfoii", args!["Louis, meow!", "Don't do it!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Martha",
            args!["Let's just surrender,", "Louis. It looks like", "the Z Gang is finished."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Anyway, the Rogue Guild",
                "is on its way to come get",
                "you guys. You won't be able",
                "to escape this time."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Louis",
            args![
                "Awwww, man!",
                "Fine, take the stupid",
                "ol' book. I couldn't",
                "use it that good anyway."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Louis",
            args![
                "Catfoii, would you",
                "lead this adventurer guy",
                "to the exit? I can't bear",
                "to look at the face that's",
                "caused my tragic downfall..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Catfoii", args!["Fine, meow.", "I understand.", "But how did we lose?!"])?;
        ctx.close_window()?;
        ctx.call(Function::GetItem, vec![Val::from(7724), Val::from(1)])?;
        ctx.call(Function::ChangeQuest, vec![Val::from(3133), Val::from(3134)])?;
        ctx.var("zdan_edq").set(Val::from(18))?;
        ctx.call(
            Function::MapWarp,
            vec![Val::from("z_agit"), Val::from("moc_fild17"), Val::from(209), Val::from(235)],
        )?;
        ctx.var("$@monster_zgang").set(Val::from(0))?;
        ctx.var("$@door2").set(Val::from(0))?;
        ctx.call(Function::DisableNpc, vec![Val::from("Louis")])?;
        ctx.call(Function::DisableNpc, vec![Val::from("Martha")])?;
        ctx.call(Function::DisableNpc, vec![Val::from("Catfoii")])?;
        ctx.call(Function::DisableNpc, vec![Val::from("#zdan_broad")])?;
        ctx.call(Function::StopNpcTimer, vec![])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Where am I...?",
                "Something has gone",
                "terribly wrong, hasn't",
                "it? Let me go baaaack~! "
            ],
        )?;
        ctx.var("zdan_edq").set(Val::from(15))?;
        ctx.var("$@monster_zgang").set(Val::from(0))?;
        ctx.var("$@door2").set(Val::from(0))?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("moc_fild17"), Val::from(209), Val::from(235)])?;
        return Err(Stop::End);
    }
}

pub fn catfoii(ctx: &Ctx) -> Script {
    catfoii_body(ctx, Vec::new()).map(|_| ())
}

fn catfoii_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Catfoii")])?;
    return Err(Stop::End);
}

pub fn catfoii_oninit(ctx: &Ctx) -> Script {
    catfoii_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn ragged_man_nd_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("treasure_nd").get()? == 0 {
        ctx.lines_as(
            "Ragged Man",
            args![
                "H-hello? Would you help me?",
                "You'd be saving this poor man's",
                "life if you would just do me",
                "this favor. Please..."
            ],
        )?;
        ctx.next()?;
        'b1: {
            let subject1 = Val::from(runtime::select_values(ctx, &[Val::from("How may I help you?:Ignore Him")])?);
            let mut matched1 = false;
            let no_case1 = !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2));
            if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["Okay, what can I do to help?"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Ragged Man",
                    args![
                        "Oh, thank you so much!",
                        "It feels so good to talk to",
                        "someone after being ignored",
                        "all this time! I might appear to",
                        "be a beggar, but I'm actually a",
                        "treasure hunter from Prontera."
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Umm... So?")])? {
                    1 => {
                        ctx.lines_as(
                            "Ragged Man",
                            args![
                                "I was so sure I could find",
                                "the treasure here in Morocc",
                                "when I first started... I did",
                                "all my research and prep work.",
                                "But after hiring teams of other",
                                "hunters and researchers..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ragged Man",
                            args![
                                "I've spent my entire fortune,",
                                "and I'm still not any closer",
                                "to finding that treasure.",
                                "I... I don't even have enough",
                                "money to go back home!"
                            ],
                        )?;
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ragged Man",
                            args!["Won't you spare some", "money, so that I can finally", "go back home to Prontera?"],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Why don't you just walk?:I can give you some money.")])? {
                            1 => {
                                ctx.lines_as(
                                    "Ragged Man",
                                    args![
                                        "What? You don't think that",
                                        "if I could walk to Prontera,",
                                        "I'd have done it already?!",
                                        "Why would I even bother to",
                                        "beg for money? Fine, I don't",
                                        "need your brand of sympathy."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Ragged Man",
                                    args![
                                        "You know, I had a hot tip",
                                        "that I was going to let you",
                                        "in on, but if you don't want",
                                        "to share, then neither do I!"
                                    ],
                                )?;
                                ctx.var("treasure_nd").set(Val::from(1))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    "Ragged Man",
                                    args![
                                        "Oh, thank you!",
                                        "Anything you can spare,",
                                        "I'll gladly appreciate!",
                                        "God bless you, bless you!"
                                    ],
                                )?;
                                ctx.next()?;
                                if ctx.var("Zeny").get()?.number()? < 1200 {
                                    ctx.lines_as(
                                        "Ragged Man",
                                        args![
                                            "Umm... Oh, dear...",
                                            "You're just as bad off",
                                            "as I am. I appreciate it,",
                                            "but I can't take your money."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Ragged Man",
                                        args![
                                            "It's a shame, really.",
                                            "If you had given me 1,200",
                                            "zeny, I would have tipped",
                                            "you off to something big..."
                                        ],
                                    )?;
                                    ctx.var("treasure_nd").set(Val::from(1))?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(1200))?))?;
                                    ctx.lines_as(
                                        "Ragged Man",
                                        args![
                                            "Let's see... I just need",
                                            "1,200 zeny. That's enough",
                                            "for me to go back home.",
                                            "Now, I can't let you just",
                                            "give me money and let you",
                                            "leave empty handed, can I?"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Ragged Man",
                                        args![
                                            "Remember that hidden",
                                            "treasure I was looking for?",
                                            "There's a rumor that it's",
                                            "location has finally been",
                                            "confirmed. Why don't you",
                                            "try finding it for yourself?"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Ragged Man",
                                        args![
                                            "I'd rather that it end up in",
                                            "the hands of someone like",
                                            "you than a rival treasure",
                                            "hunter. This hunter that",
                                            "stays north in town apparently confirmed the treasure's location."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Ragged Man",
                                        args![
                                            "Now, I hear that this guy",
                                            "hasn't returned home yet,",
                                            "and he still hasn't found",
                                            "the treasure. He's probably",
                                            "run into a pretty bad snag."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Ragged Man",
                                        args![
                                            "I doubt that he'll let you",
                                            "know the treasure's location",
                                            "easily, but who knows? Maybe",
                                            "he might slip, and accidentally",
                                            "give you some kind of clue."
                                        ],
                                    )?;
                                    ctx.var("treasure_nd").set(Val::from(2))?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as(
                    "Ragged Man",
                    args![
                        "Please! Please, wait!",
                        "You're an adventurer, aren't",
                        "you? I have some valuable",
                        "information to share if you'd",
                        "just spare me some... Hey!",
                        "Come back! No... I just..."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    } else if ctx.var("treasure_nd").get()? == 1 {
        ctx.lines_as(
            "Ragged Man",
            args![
                "Oh, I remember you.",
                "You must really want that",
                "treasure now, do you? Just",
                "give me 1,200 zeny for me",
                "to go back home, and I'll",
                "tell you everything I know."
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Give Money:Don't Give Money")])? {
            1 => {
                ctx.lines_as(
                    "Ragged Man",
                    args![
                        "Ah, good choice!",
                        "And thank you so much.",
                        "It'll be great to finally be",
                        "back home in Prontera."
                    ],
                )?;
                ctx.next()?;
                if ctx.var("Zeny").get()?.number()? < 1200 {
                    ctx.lines_as(
                        "Ragged Man",
                        args![
                            "What th--?! This isn't",
                            "enough money for me to go",
                            "back home! Well, just come",
                            "back later once you scrounge",
                            "up the funds. I'll be waiting."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(1200))?))?;
                    ctx.lines_as(
                        "Ragged Man",
                        args![
                            "Remember that hidden",
                            "treasure I was looking for?",
                            "There's a rumor that it's",
                            "location has finally been",
                            "confirmed. Why don't you",
                            "try finding it for yourself?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ragged Man",
                        args![
                            "I'd rather that it end up in",
                            "he hands of someone like",
                            "you than a rival treasure",
                            "hunter. This hunter that",
                            "stays north in town apparently confirmed the treasure's location."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ragged Man",
                        args![
                            "Now, I hear that this guy",
                            "hasn't returned home yet,",
                            "and he still hasn't found",
                            "the treasure. He's probably",
                            "run into a pretty big snag."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ragged Man",
                        args![
                            "I doubt that he'll let you",
                            "know the treasure's location",
                            "easily, but who knows? Maybe",
                            "he might slip, and accidentally",
                            "give you some kind of clue."
                        ],
                    )?;
                    ctx.var("treasure_nd").set(Val::from(2))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            2 => {
                ctx.lines_as(
                    "Ragged Man",
                    args![
                        "Fine, have it your way.",
                        "I admit that I'm a little",
                        "disappointed in you. Aren't",
                        "you adventurers supposed",
                        "to be heroes to the people?"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if ctx.var("treasure_nd").get()? == 2 {
        ctx.lines_as(
            "Ragged Man",
            args![
                "Oh, you're back?",
                "Ah, you must be having",
                "trouble finding that guy",
                "I was talking about. He's",
                "just in the northern part of this town. It can't be that hard."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("treasure_nd").get()?.number()? > 10 {
        ctx.lines_as(
            "Ragged Man",
            args![
                "You found the treasure?!",
                "It... It really exists...",
                "I'm sorry, it's just been",
                "my dream to... I feel like",
                "crying... Oh, God... I don't",
                "know what this feeling is..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ragged Man",
            args![
                "I mean... I feel so",
                "damned happy, but...",
                "I spent my entire fortune...",
                "Ruined my whole life just",
                "to see that treasure..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Ragged Man",
            args![
                "So how's the treasure",
                "hunt coming along? I hope",
                "you have better luck than",
                "I did. Still, I've got a pretty good feeling that you'll find it."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn ragged_man_nd(ctx: &Ctx) -> Script {
    ragged_man_nd_body(ctx, Vec::new()).map(|_| ())
}

fn man_zgang_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_joho1 = Val::from(0);
    let mut l_joho2 = Val::from(0);
    if ctx.var("treasure_nd").get()?.number()? < 2 {
        ctx.lines_as("Man", args!["What do you want?", "Just leave me alone."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("treasure_nd").get()? == 2 {
            ctx.lines_as("Man", args!["What do you want?", "Just leave me alone."])?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Tell me about the treasure.")])? {
                1 => {
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
                    ctx.lines_as(
                        "Man",
                        args![
                            "What? So you're the one",
                            "I was warned about! I've",
                            "been wondering when you'd",
                            "show up. Prepare to die!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Umm... Excuse me?",
                            "I've just heard that you",
                            "confirmed the location of",
                            "some amazing treasure,",
                            "and I just wanted to ask",
                            "you more about it. That's all!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Man",
                        args![
                            "What? If you're just here",
                            "for that, then how'd you",
                            "find out about me and",
                            "the treasure? Not just",
                            "everyone knows about it."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Well, I was talking to",
                            "this other treasure hunter",
                            "who's... Well, he's kind",
                            "of a beggar now..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Man",
                        args![
                            "Oh. That old man?",
                            "Well, I don't know if the",
                            "treasure he was looking",
                            "for and the one I've found",
                            "are the same one. But",
                            "yeah, I know that guy."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Man",
                        args![
                            "Alright. So I found this",
                            "treasure, but I have no way",
                            "of digging it up. Maybe we",
                            "can help each other. I need",
                            "money, and you want to know",
                            "more, right? Let's cut a deal."
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("No, thanks.:What'd you have in mind?")])? {
                        1 => {
                            ctx.lines_as(
                                "Man",
                                args![
                                    "No? Well...",
                                    "There's no reason for",
                                    "you to trust me now,",
                                    "I suppose. But what can",
                                    "you gain by walking away?"
                                ],
                            )?;
                            ctx.var("treasure_nd").set(Val::from(3))?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as(
                                "Man",
                                args![
                                    "Let me lay it out for you.",
                                    "I've been hired to find this",
                                    "treasure by some people:",
                                    "they said I can take whatever",
                                    "I find, but they specifically",
                                    "want one item for themselves."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Man",
                                args![
                                    "They pay me on delivery",
                                    "so I haven't seen a dime yet.",
                                    "Now, my agreement with them",
                                    "is conditional. I can't dig",
                                    "up the treasure so they'll",
                                    "understand if I say I failed."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Man",
                                args![
                                    "But if you dig it up, you can",
                                    "keep everything. Sound good?",
                                    "All I want is some stuff from",
                                    "you to make all my efforts up",
                                    "till now worth my while."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Man",
                                args![
                                    "In other words,",
                                    "if you bring me,",
                                    "^FF000020 Mementos^000000,",
                                    "^FF00002 Pearls^000000, and",
                                    "^FF00002 Zargons^000000, we can talk",
                                    "business further. Got it?"
                                ],
                            )?;
                            ctx.var("treasure_nd").set(Val::from(4))?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Man",
                                args![
                                    "Hey, I might be losing out",
                                    "by giving up my share of that",
                                    "treasure, but hey, a bird in",
                                    "the hand's always worth more."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        } else {
            if ctx.var("treasure_nd").get()? == 3 {
                ctx.lines_as(
                    "Man",
                    args![
                        "You again? Lemme guess...",
                        "You're reconsidering that",
                        "deal I had in mind, aren't you?"
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("No, not really.:I guess so.")])? {
                    1 => {
                        ctx.lines_as(
                            "Man",
                            args![
                                "You sure? No one would",
                                "be losing in this deal, you",
                                "know. Well, except the guys",
                                "that hired me to find that",
                                "treasure. It's pretty much",
                                "a win-win situation."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Man",
                            args![
                                "Way I figure it,",
                                "someone ought to get",
                                "that treasure. I can't",
                                "dig it up, but maybe",
                                "someone like you can..."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as(
                            "Man",
                            args![
                                "Let me lay it out for you.",
                                "I've been hired to find this",
                                "treasure by some people:",
                                "they said I can take whatever",
                                "I find, but they specifically",
                                "want one item for themselves."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Man",
                            args![
                                "They pay me on delivery",
                                "so I haven't seen a dime yet.",
                                "Now, my agreement with them",
                                "is conditional. I can't dig",
                                "up the treasure so they'll",
                                "understand if I say I failed."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Man",
                            args![
                                "But if you dig it up, you can",
                                "keep everything. Sound good?",
                                "All I want is some stuff from",
                                "you to make all my efforts up",
                                "till now worth my while."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Man",
                            args![
                                "In other words,",
                                "if you bring me,",
                                "^FF000020 Mementos^000000,",
                                "^FF00002 Pearls^000000, and",
                                "^FF00002 Zargons^000000, we can talk",
                                "business further. Got it?"
                            ],
                        )?;
                        ctx.var("treasure_nd").set(Val::from(4))?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Man",
                            args![
                                "Hey, I might be losing out",
                                "by giving up my share of that",
                                "treasure, but hey, a bird in",
                                "the hand's always worth more."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            } else {
                if ctx.var("treasure_nd").get()? == 4 {
                    ctx.lines_as("Man", args!["Back again, huh?", "So did you bring", "the stuff I asked?"])?;
                    ctx.next()?;
                    if ((ctx.call(Function::CountItem, vec![Val::from(722)])?.number()? > 1
                        && ctx.call(Function::CountItem, vec![Val::from(912)])?.number()? > 1)
                        && ctx.call(Function::CountItem, vec![Val::from(934)])?.number()? > 19)
                    {
                        ctx.lines_as(
                            "Man",
                            args![
                                "Nice. You did your part.",
                                "And now it's time that",
                                "I fulfilled my part of",
                                "this bargain. Listen up."
                            ],
                        )?;
                        ctx.call(Function::DelItem, vec![Val::from(722), Val::from(2)])?;
                        ctx.call(Function::DelItem, vec![Val::from(912), Val::from(2)])?;
                        ctx.call(Function::DelItem, vec![Val::from(934), Val::from(20)])?;
                        ctx.var("treasure_nd").set(Val::from(5))?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Man",
                            args![
                                "The treasure is buried",
                                "south of Morocc. Use the",
                                "south gate to leave town,",
                                "and then head to the next",
                                "field to the south. From",
                                "there, you're on your own."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Man",
                            args![
                                "I don't know exactly",
                                "where the treasure is",
                                "buried, but you should",
                                "be able to find it within",
                                "that general area. Good luck."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Man",
                            args![
                                "Huh, you must have",
                                "forgotten what I wanted.",
                                "I'm not really asking you",
                                "for extremely valuable stuff",
                                "here. Anyway, listen up."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Man",
                            args![
                                "Just bring me,",
                                "^FF000020 Mementos^000000,",
                                "^FF00002 Pearls^000000, and",
                                "^FF00002 Zargons^000000 so we can talk",
                                "business further. Got it?"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                } else if ctx.var("treasure_nd").get()? == 5 {
                    ctx.lines_as(
                        "Man",
                        args!["What are you still doing", "here? Shouldn't you be", "looking for the treasure?"],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("treasure_nd").get()? == 6 {
                    ctx.lines_as(
                        "Man",
                        args!["What are you still doing", "here? Shouldn't you be", "looking for the treasure?"],
                    )?;
                    ctx.next()?;
                    let choice = runtime::select_values(ctx, &[Val::from("I already found it, but...")])?;
                    ctx.var("@menu").set(choice)?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "I already found it, but...",
                            "It won't budge at all!",
                            "I think it's protected by",
                            "magic. Do you know",
                            "anything about that?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Man",
                        args![
                            "Wow, you actually found",
                            "it? Heh, you're better than",
                            "I thought. Sorry about that.",
                            "I didn't explain the magic",
                            "part since... Well, I didn't",
                            "think you'd get that far."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Man",
                        args![
                            "Anyway, I had problems",
                            "with that magic too. Some",
                            "guy was supposed to help me",
                            "with that, but he didn't come.",
                            "I've been so frustrated!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Man",
                        args![
                            "I mean, I have no clue",
                            "when it comes to magic, and",
                            "I don't understand how I'm",
                            "supposed to use this thing",
                            "my clients gave to me."
                        ],
                    )?;
                    ctx.next()?;
                    'l4: loop {
                        if !(true) {
                            break 'l4;
                        }
                        'b4: {
                            match runtime::select_values(ctx, &[Val::from("Who was going to help you?:Your clients gave you something?")])?
                            {
                                1 => {
                                    ctx.lines_as(
                                        "Man",
                                        args![
                                            "Oh, yeah. The guys that",
                                            "hired me said that if I waited",
                                            "at the treasure site, someone",
                                            "would eventually come to help",
                                            "me out on the magic end.",
                                            "I don't know who he is, though."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Man",
                                        args![
                                            "Oh, yeah. They mentioned",
                                            "something about him coming",
                                            "a bit from the west. Maybe",
                                            "he's over in Comodo?"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    l_joho1 = Val::from(1);
                                }
                                2 => {
                                    if ctx.var("treasure_nd").get()? == 7 {
                                        ctx.lines_as(
                                            "Man",
                                            args![
                                                "Oh, yeah. They gave me",
                                                "this weird document labeled",
                                                "''^FF0000[Open^000000.'' I don't know how",
                                                "to use it at all, though."
                                            ],
                                        )?;
                                        ctx.next()?;
                                    } else if ctx.var("treasure_nd").get()? == 8 {
                                        ctx.lines_as(
                                            "Man",
                                            args![
                                                "Oh, yeah. They gave me",
                                                "this weird document labeled",
                                                "''^FF0000[Unlock^000000.'' I don't know how",
                                                "to use it at all, though."
                                            ],
                                        )?;
                                        ctx.next()?;
                                    } else {
                                        ctx.lines_as(
                                            "Man",
                                            args![
                                                "Oh, yeah. Let's see here.",
                                                "Ah, here's the document",
                                                "that they gave me. It's kind",
                                                "of a bit torn, though."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(2)])? == 1 {
                                            ctx.lines(args!["^3355FFThe torn document", "is labeled ''^FF0000[Open^3355FF.''^000000"])?;
                                            ctx.next()?;
                                            ctx.var("treasure_nd").set(Val::from(7))?;
                                            l_joho2 = Val::from(1);
                                        } else {
                                            ctx.lines(args![
                                                "^3355FFThe torn document",
                                                "is labeled ''^FF0000[Unlock^3355FF.''^000000"
                                            ])?;
                                            ctx.next()?;
                                            ctx.var("treasure_nd").set(Val::from(8))?;
                                            l_joho2 = Val::from(1);
                                        }
                                    }
                                }
                                _ => {}
                            }
                            if (l_joho1.clone() == 1 && l_joho2.clone() == 1) {
                                break 'l4;
                            }
                        }
                    }
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Wait, how could you not",
                            "know how to use this?",
                            "Didn't you get a chance",
                            "to talk to your clients?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Man",
                        args![
                            "I haven't even met them.",
                            "I just get their instructions",
                            "through the mail. Everything",
                            "is through a medium with these",
                            "guys. I guess they've got their",
                            "eyes on me too, but whatever."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Man",
                        args![
                            "I guess they hired me because",
                            "they really want the treasure,",
                            "but don't want to tip anyone",
                            "off that they're looking for ",
                            "it, or that they have it."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Man",
                        args![
                            "Anyway, all I know is that",
                            "this piece of paper is supposed",
                            "to do something. But I guess",
                            "I'm not smart enough to figure",
                            "it out. Maybe you'll have",
                            "better luck with them."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Man",
                        args![
                            "You know, if you can actually",
                            "find that guy who was supposed",
                            "to help me but never showed up,",
                            "maybe he'll understand how to",
                            "use that document. Go ahead",
                            "and take 'em. Good luck, now."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if (ctx.var("treasure_nd").get()? == 7 || ctx.var("treasure_nd").get()? == 8) {
                    ctx.lines_as(
                        "Man",
                        args![
                            "I already told you everything",
                            "I know. You'd be better off",
                            "finding that guy over in",
                            "Comodo, or the rest of",
                            "that torn document."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if (ctx.var("treasure_nd").get()? == 9 || ctx.var("treasure_nd").get()? == 10) {
                    ctx.lines_as(
                        "Man",
                        args![
                            "Now that I think about it,",
                            "maybe the treasure can be",
                            "obtained by two people?",
                            "I mean, they planned for",
                            "me to go with that guy. Eh,",
                            "best not to think about it."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as("Man", args!["Heh! Looks like I'll be", "headed home soon~"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        }
    }
    Ok(Val::from(0))
}

pub fn man_zgang(ctx: &Ctx) -> Script {
    man_zgang_body(ctx, Vec::new()).map(|_| ())
}

fn man_in_hiding_nd_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("treasure_nd").get()?.number()? < 7 {
        ctx.lines_as("Man in Hiding", args!["Whoa, don't get so close!"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("treasure_nd").get()? == 7 || ctx.var("treasure_nd").get()? == 8) {
        if ctx.var("zdan_edq").get()?.number()? > 12 {
            ctx.lines_as("Man in Hiding", args!["Whoa, don't get so close!"])?;
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("Why are you hiding here?")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Why are you hiding here?", "Didn't the Z Gang hire you", "to dig up some treasure?"],
            )?;
            ctx.next()?;
            ctx.lines_as("Man in Hiding", args!["Huh? Z Gang? Treasure?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Man in Hiding",
                args!["Huh? Wait, tell me the", "truth. Did you actually", "meet the Z Gang?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "I not only met the Z Gang,",
                    "I also captured them. I've",
                    "come looking for you to",
                    "find out more about this",
                    "buried treasure."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Man in Hiding",
                args![
                    "I get it now. You must",
                    "have met that other guy",
                    "they hired to get the",
                    "treasure. Huh. He's got one",
                    "half of the spell to open it,",
                    "and I've got the other."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Man in Hiding",
                args![
                    "Well, you can have this,",
                    "the rest of the magic spell.",
                    "Let me warn you, though,",
                    "there's something weird",
                    "about the spell, and the",
                    "treasure itself."
                ],
            )?;
            ctx.next()?;
            if ctx.var("treasure_nd").get()? == 7 {
                ctx.lines(args![
                    "^3355FFYou received a torn",
                    "document that reads",
                    "''^FF0000Seseame]^000000.''"
                ])?;
                ctx.var("treasure_nd").set(Val::from(10))?;
                ctx.next()?;
            } else {
                ctx.lines(args![
                    "^3355FFYou received a torn",
                    "document that reads",
                    "''^FF0000Treasure]^000000.''"
                ])?;
                ctx.var("treasure_nd").set(Val::from(9))?;
                ctx.next()?;
            }
            ctx.lines_as(
                "Man in Hiding",
                args![
                    "I don't understand why they",
                    "went through the hassle of",
                    "splitting this spell in two",
                    "parts. There must be some",
                    "reason. Whatever it is, it",
                    "must be pretty important."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Man in Hiding",
                args![
                    "I guess maybe you need",
                    "two people to move the",
                    "treasure? Well, whatever.",
                    "Good luck finding that",
                    "treasure. Farewell now!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as("Man in Hiding", args!["Whoa, don't get so close!"])?;
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("Ask About Treasure")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["I've come looking", "for information", "about the treasure."],
            )?;
            ctx.next()?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
            ctx.lines_as(
                "Man in Hiding",
                args![
                    "You... You've come",
                    "looking for me?!",
                    "Ugh, I'm cornered!",
                    "I'm not going down",
                    "without a fight!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "What? No, it's not like",
                    "that. I met this treasure",
                    "hunter in Morocc that said",
                    "I could find you around here."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Man in Hiding",
                args!["Treasure hunter...?", "Oh, then you already", "know about the treasure..."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Man in Hiding",
                args![
                    "If I'm right, then you",
                    "need the other half of",
                    "the spell to open the",
                    "treasure. Here, you can",
                    "take it. I have no use",
                    "for it, anyway."
                ],
            )?;
            ctx.next()?;
            if ctx.var("treasure_nd").get()? == 7 {
                ctx.lines(args![
                    "^3355FFYou received a torn",
                    "document that reads",
                    "''^FF0000Sesame]^000000.''"
                ])?;
                ctx.var("treasure_nd").set(Val::from(10))?;
                ctx.next()?;
            } else {
                ctx.lines(args![
                    "^3355FFYou received a torn",
                    "document that reads",
                    "''^FF0000Treasure]^000000.''"
                ])?;
                ctx.var("treasure_nd").set(Val::from(9))?;
                ctx.next()?;
            }
            ctx.lines_as(
                "Man in Hiding",
                args![
                    "Now would you just get",
                    "out of here? If you couldn't",
                    "already tell, I'm hiding from",
                    "some people. I told you all",
                    "I know, so do me a favor,",
                    "and don't tell anyone I'm here!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        if ctx.var("treasure_nd").get()? == 9 {
            ctx.lines_as(
                "Man in Hiding",
                args![
                    "Aren't you going to",
                    "dig up that treasure?",
                    "You have both parts of",
                    "the spell that you need:",
                    "I already gave you the",
                    "''^FF0000Treasure]^000000'' part, right?"
                ],
            )?;
            ctx.next()?;
            if ctx.var("zdan_edq").get()?.number()? > 12 {
                ctx.lines_as(
                    "Man in Hiding",
                    args![
                        "Anyway, now that the",
                        "Z Gang is captured,",
                        "it might be safe enough",
                        "for me to go back home."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Man in Hiding",
                    args![
                        "Would you do me a favor,",
                        "and just get out of here",
                        "before someone finds",
                        "my hiding place?"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else if ctx.var("treasure_nd").get()? == 10 {
            ctx.lines_as(
                "Man in Hiding",
                args![
                    "Aren't you going to",
                    "dig up that treasure?",
                    "You have both parts of",
                    "the spell that you need:",
                    "I already gave you the",
                    "''^FF0000Sesame]^000000'' part, right?"
                ],
            )?;
            ctx.next()?;
            if ctx.var("zdan_edq").get()?.number()? > 12 {
                ctx.lines_as(
                    "Man in Hiding",
                    args![
                        "Anyway, now that the",
                        "Z Gang is captured,",
                        "it might be safe enough",
                        "for me to go back home."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Man in Hiding",
                    args![
                        "Would you do me a favor,",
                        "and just get out of here",
                        "before someone finds",
                        "my hiding place?"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else {
            ctx.lines_as(
                "Man in Hiding",
                args![
                    "Heh! Looks like it's",
                    "safe for me to come out",
                    "of hiding, and just go home!",
                    "But maybe I should stick",
                    "around here a little longer,",
                    "just to be on the safe side."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
}

pub fn man_in_hiding_nd(ctx: &Ctx) -> Script {
    man_in_hiding_nd_body(ctx, Vec::new()).map(|_| ())
}

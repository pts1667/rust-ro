use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn prince_eisen5_ondisable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Prince#eisen5")])?;
    return Err(Stop::End);
}

pub fn prince_eisen5_ondisable(ctx: &Ctx) -> Script {
    prince_eisen5_ondisable_body(ctx, Vec::new()).map(|_| ())
}

fn prince_eisen6_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Ernst", args!["Bro, tell me again. You just said something?"])?;
    ctx.next()?;
    ctx.lines_as("Ahrum", args!["Ok. Let me tell you. This Rune-Midgarts is rubbish. You and I struggle in the dump of trash, and we are all same with it, even if it cannot be recycled."])?;
    ctx.next()?;
    ctx.lines_as("Ernst", args!["Brother, what's wrong? Are you insane?"])?;
    ctx.next()?;
    ctx.lines_as(
        "Ahrum",
        args!["Insane? I am fully sane....J.m...I am just fed up with this disgusting reality, my lovely brother."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Ernst",
        args!["Reality? What reality? What did you perceive? You are not like as you are. I totally can't help you!"],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Ahrum",
        args![
            "I am as I usually am, Ernst. Just, reality is dirty, and I realized that I am trapped in the crap. I can't get out of here."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Ernst",
        args!["Liar! You always told me that you have possibilities in your book! You always behaved proudly!"],
    )?;
    ctx.next()?;
    ctx.lines_as("Ahrum", args!["Shut up."])?;
    ctx.next()?;
    ctx.lines_as(
        "Ernst",
        args![
            "It's dirty and not good. That's why we should correct it. We already know about this situation, but why do you act like this!!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Ahrum", args!["...I told you, don't make noise!"])?;
    ctx.next()?;
    ctx.lines_as(
        "Ernst",
        args!["No. Corruption is mutually agreed, and you brought me here, and because of your presence, I can depend on you!"],
    )?;
    ctx.next()?;
    ctx.lines_as("Ahrum", args!["Shut up Shut up!!!!!!!!!!!!"])?;
    ctx.next()?;
    ctx.lines_as("Ernst", args!["......!"])?;
    ctx.next()?;
    ctx.lines_as("Ahrum", args!["Meaningless actions cannot be welcomed! Being manipulated by dirt is disgusting. I was ignorant, I felt that I acted by my will, but now I realize I was manipulated!"])?;
    ctx.next()?;
    ctx.lines_as("Ahrum", args!["King?! What's that for?! That's nothing! Even if we make a new world, nothing is changed!! We are like in a play. A picture is someone's intention... only drawn!! Right?!"])?;
    ctx.next()?;
    ctx.lines_as("Ernst", args!["What are you talking about? I totally can't understand! Are you saying you are not in the status quo? Yes? What the heck are you talking about!"])?;
    ctx.next()?;
    ctx.lines_as("Ahrum", args!["Shut up! Without knowing anything, you don't know when you can be killed. You don't get me yet.... ah...! You need to not know this!"])?;
    ctx.next()?;
    ctx.lines_as(
        "Ernst",
        args!["Br...Brother, will you keep insisting on this? Our relationship is this short?"],
    )?;
    ctx.next()?;
    ctx.lines_as("Ahrum", args!["Shut up! If you are not contented, you can kill me as we promised! You... me... me...!! I don't wanna talk about it any more, I am going back to my room!"])?;
    ctx.next()?;
    ctx.lines_as("Ernst", args!["Ah. Hold on a moment Brother! Ahrum!"])?;
    ctx.next()?;
    ctx.lines_as("Ahrum", args!["I... don't know! King... you!"])?;
    ctx.next()?;
    ctx.lines(args![
        "-Ahrum blushes",
        "and leaves the room. He seems to be utterly fatigued.-"
    ])?;
    ctx.next()?;
    ctx.lines_as("Ernst", args!["Ahrum... Ahrum..."])?;
    ctx.call(Function::ChangeQuest, vec![Val::from(10023), Val::from(10024)])?;
    ctx.var("nkprince_eisen").set(Val::from(13))?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("Prince#eisen6::OnDisable")])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn prince_eisen6(ctx: &Ctx) -> Script {
    prince_eisen6_body(ctx, Vec::new()).map(|_| ())
}

fn prince_eisen6_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Prince#eisen6")])?;
    return Err(Stop::End);
}

pub fn prince_eisen6_oninit(ctx: &Ctx) -> Script {
    prince_eisen6_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn prince_eisen6_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("Prince#eisen6")])?;
    return Err(Stop::End);
}

pub fn prince_eisen6_onenable(ctx: &Ctx) -> Script {
    prince_eisen6_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn prince_eisen6_ondisable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Prince#eisen6")])?;
    return Err(Stop::End);
}

pub fn prince_eisen6_ondisable(ctx: &Ctx) -> Script {
    prince_eisen6_ondisable_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum EisenStep {
    Start,
    OnTouch,
}

fn eisen_run(ctx: &Ctx, mut step: EisenStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_armkoe = Val::from(0);
    'machine: loop {
        match step {
            EisenStep::Start => {
                step = EisenStep::OnTouch;
                continue 'machine;
            }
            EisenStep::OnTouch => {
                if (ctx.var("nkprince_eisen").get()? == 4 || ctx.var("nkprince_eisen").get()? == 5) {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Prince#another_ern::OnEnable")])?;
                    ctx.var("nkprince_eisen").set(Val::from(5))?;
                    ctx.call(Function::Warp, vec![Val::from("prt_castle"), Val::from(318), Val::from(368)])?;
                } else if (((ctx.var("nkprince_eisen").get()? == 9 || ctx.var("nkprince_eisen").get()? == 10)
                    || ctx.var("nkprince_eisen").get()? == 11)
                    || ctx.var("nkprince_eisen").get()? == 12)
                {
                    l_armkoe = ctx.call(Function::Rand, vec![Val::from(1), Val::from(4)])?;
                    if l_armkoe.clone() == 1 {
                        ctx.lines(args![
                            "-You can hear Ahrum's voice",
                            "before going into his room",
                            "It's coming from inside.-"
                        ])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ahrum",
                            args!["I cannot have that qualification. I was raised in such a dirty place. I was so ignorant!"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ahrum",
                            args!["What's the capacity of a king? And what is clean politics...? Ahhhhhhhhhhh!!"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["......"])?;
                        ctx.close_window()?;
                    } else if l_armkoe.clone() == 2 {
                        ctx.lines(args![
                            "-You can hear Ahrum's voice",
                            "before going into his room",
                            "It's coming from inside.-"
                        ])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ahrum",
                            args!["Proud family?... Famous...", "Birth?... King's family? ...What's all that about?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Ahrum", args!["To brother Ernst, from my family. It's sin. And I am a part of the sin. As a part of the family. Family's sin. Equal to my sin."])?;
                        ctx.next()?;
                        ctx.lines_as("Ahrum", args!["I cannot... cover it anymore... Ern... I hope even you..."])?;
                        ctx.next()?;
                        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["......"])?;
                        ctx.close_window()?;
                    } else if l_armkoe.clone() == 3 {
                        ctx.lines(args![
                            "-You can hear Ahrum's voice",
                            "before going into his room",
                            "It's coming from inside.-"
                        ])?;
                        ctx.next()?;
                        ctx.lines_as("Ahrum", args!["In the end, I am just being used... I make the most of my exertions and become the lead. That's all a part of the scenario..."])?;
                        ctx.next()?;
                        ctx.lines_as("Ahrum", args!["Damn it! Damn...!!", "I cannot accept it. Never. Never!"])?;
                        ctx.next()?;
                        ctx.lines_as("Ahrum", args!["My ancestor ^FF0000Schmidt^000000... What shall you do with this situation? No...were you just used like me... by others?"])?;
                        ctx.next()?;
                        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["......"])?;
                        ctx.close_window()?;
                    } else if l_armkoe.clone() == 4 {
                        ctx.lines(args![
                            "- I could hear Ahrum's voice ",
                            "before getting into his room",
                            "It was coming from inside.-"
                        ])?;
                        ctx.next()?;
                        ctx.lines_as("Ahrum", args!["No, it's no good... No... In this phase, I will be the king. I shouldn't be the king... Ern... You should not forget the meaning of a real king."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ahrum",
                            args!["For the real king's appearance... I am going tonight.... to darkness for you. ...Ahhhhhhhh!!!"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["......"])?;
                        ctx.close_window()?;
                    }
                    ctx.call(Function::Warp, vec![Val::from("prt_castle"), Val::from(318), Val::from(368)])?;
                } else if ctx.var("nkprince_eisen").get()? == 13 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Prince#another_ern::OnEnable")])?;
                    ctx.call(Function::Warp, vec![Val::from("prt_castle"), Val::from(318), Val::from(368)])?;
                } else if ctx.var("nkprince_eisen").get()? == 15 {
                    ctx.call(Function::Warp, vec![Val::from("prt_castle"), Val::from(318), Val::from(309)])?;
                } else {
                    ctx.call(Function::Warp, vec![Val::from("prt_castle"), Val::from(318), Val::from(368)])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn eisen(ctx: &Ctx) -> Script {
    eisen_run(ctx, EisenStep::Start, Vec::new()).map(|_| ())
}

pub fn eisen_ontouch(ctx: &Ctx) -> Script {
    eisen_run(ctx, EisenStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ErnStep {
    Start,
    OnTouch,
}

fn ern_run(ctx: &Ctx, mut step: ErnStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ErnStep::Start => {
                step = ErnStep::OnTouch;
                continue 'machine;
            }
            ErnStep::OnTouch => {
                if (ctx.call(Function::CheckQuest, vec![Val::from(10023)])? == 0
                    || ctx.call(Function::CheckQuest, vec![Val::from(10023)])? == 1)
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Prince#eisen6::OnEnable")])?;
                }
                ctx.call(Function::Warp, vec![Val::from("prt_castle"), Val::from(368), Val::from(308)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn ern(ctx: &Ctx) -> Script {
    ern_run(ctx, ErnStep::Start, Vec::new()).map(|_| ())
}

pub fn ern_ontouch(ctx: &Ctx) -> Script {
    ern_run(ctx, ErnStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ErichStep {
    Start,
    OnTouch,
}

fn erich_run(ctx: &Ctx, mut step: ErichStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ErichStep::Start => {
                step = ErichStep::OnTouch;
                continue 'machine;
            }
            ErichStep::OnTouch => {
                if (ctx.call(Function::CheckQuest, vec![Val::from(10020)])? == 0
                    || ctx.call(Function::CheckQuest, vec![Val::from(10020)])? == 1)
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Prince#eisen1::OnEnable")])?;
                }
                ctx.call(Function::Warp, vec![Val::from("prt_castle"), Val::from(274), Val::from(368)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn erich(ctx: &Ctx) -> Script {
    erich_run(ctx, ErichStep::Start, Vec::new()).map(|_| ())
}

pub fn erich_ontouch(ctx: &Ctx) -> Script {
    erich_run(ctx, ErichStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum HelmutStep {
    Start,
    OnTouch,
}

fn helmut_run(ctx: &Ctx, mut step: HelmutStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            HelmutStep::Start => {
                step = HelmutStep::OnTouch;
                continue 'machine;
            }
            HelmutStep::OnTouch => {
                if (ctx.call(Function::CheckQuest, vec![Val::from(10022)])? == 0
                    || ctx.call(Function::CheckQuest, vec![Val::from(10022)])? == 1)
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Prince#eisen3::OnEnable")])?;
                }
                ctx.call(Function::Warp, vec![Val::from("prt_castle"), Val::from(290), Val::from(208)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn helmut(ctx: &Ctx) -> Script {
    helmut_run(ctx, HelmutStep::Start, Vec::new()).map(|_| ())
}

pub fn helmut_ontouch(ctx: &Ctx) -> Script {
    helmut_run(ctx, HelmutStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum PoeStep {
    Start,
    OnTouch,
}

fn poe_run(ctx: &Ctx, mut step: PoeStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            PoeStep::Start => {
                step = PoeStep::OnTouch;
                continue 'machine;
            }
            PoeStep::OnTouch => {
                if (ctx.call(Function::CheckQuest, vec![Val::from(10018)])? == 0
                    || ctx.call(Function::CheckQuest, vec![Val::from(10018)])? == 1)
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Prince#eisen4::OnEnable")])?;
                }
                ctx.call(Function::Warp, vec![Val::from("prt_castle"), Val::from(390), Val::from(208)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn poe(ctx: &Ctx) -> Script {
    poe_run(ctx, PoeStep::Start, Vec::new()).map(|_| ())
}

pub fn poe_ontouch(ctx: &Ctx) -> Script {
    poe_run(ctx, PoeStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum PeterStep {
    Start,
    OnTouch,
}

fn peter_run(ctx: &Ctx, mut step: PeterStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            PeterStep::Start => {
                step = PeterStep::OnTouch;
                continue 'machine;
            }
            PeterStep::OnTouch => {
                if (ctx.call(Function::CheckQuest, vec![Val::from(10019)])? == 0
                    || ctx.call(Function::CheckQuest, vec![Val::from(10019)])? == 1)
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Prince#eisen5::OnEnable")])?;
                }
                ctx.call(Function::Warp, vec![Val::from("prt_castle"), Val::from(366), Val::from(368)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn peter(ctx: &Ctx) -> Script {
    peter_run(ctx, PeterStep::Start, Vec::new()).map(|_| ())
}

pub fn peter_ontouch(ctx: &Ctx) -> Script {
    peter_run(ctx, PeterStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum UrgenStep {
    Start,
    OnTouch,
}

fn urgen_run(ctx: &Ctx, mut step: UrgenStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            UrgenStep::Start => {
                step = UrgenStep::OnTouch;
                continue 'machine;
            }
            UrgenStep::OnTouch => {
                if (ctx.call(Function::CheckQuest, vec![Val::from(10021)])? == 0
                    || ctx.call(Function::CheckQuest, vec![Val::from(10021)])? == 1)
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Prince#eisen2::OnEnable")])?;
                }
                ctx.call(Function::Warp, vec![Val::from("prt_castle"), Val::from(340), Val::from(208)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn urgen(ctx: &Ctx) -> Script {
    urgen_run(ctx, UrgenStep::Start, Vec::new()).map(|_| ())
}

pub fn urgen_ontouch(ctx: &Ctx) -> Script {
    urgen_run(ctx, UrgenStep::OnTouch, Vec::new()).map(|_| ())
}

fn guard_princein_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("nk_prince").get()?.number()? > 4 {
        ctx.lines_as("Guard of a strange place", args!["You can go to the room where princes gather, through the hidden door. For security reasons, it is totally guarded and hidden."])?;
        ctx.next()?;
        ctx.lines_as("Guard of a strange place", args!["Let me open the door."])?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("prt_castle"), Val::from(220), Val::from(312)])?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Guard of a strange place",
            args!["I just came here to rest. For location information, you can ask it of others."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn guard_princein(ctx: &Ctx) -> Script {
    guard_princein_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ArmStep {
    Start,
    OnTouch,
}

fn arm_run(ctx: &Ctx, mut step: ArmStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ArmStep::Start => {
                step = ArmStep::OnTouch;
                continue 'machine;
            }
            ArmStep::OnTouch => {
                if ctx.var("nk_prince").get()? == 8 {
                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args![".............."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Milk is all spilt.", "Just forget about it."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Who is going to have the throne instead of him?"],
                    )?;
                    ctx.next()?;
                    ctx.var("nk_prince").set(Val::from(9))?;
                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Oh no..."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn arm(ctx: &Ctx) -> Script {
    arm_run(ctx, ArmStep::Start, Vec::new()).map(|_| ())
}

pub fn arm_ontouch(ctx: &Ctx) -> Script {
    arm_run(ctx, ArmStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Arm1Step {
    Start,
    OnTouch,
}

fn arm1_run(ctx: &Ctx, mut step: Arm1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Arm1Step::Start => {
                step = Arm1Step::OnTouch;
                continue 'machine;
            }
            Arm1Step::OnTouch => {
                if ctx.var("nkprince_eisen").get()?.number()? > 14 {
                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args![".............."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["What thoughts did he have, and how did he decide to become King in this room?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["What situation...", "pulled him back to an irreparable place?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["I can't ask him anymore, and there are no clues left. Just what's left is..."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ahrum",
                        args!["~~If you react wrongfully to this incident, my death will be worthless...kuk... Do you understand?~~"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args![".............."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "...Keeping silent for him.",
                            "Taking the burden that he left... all throughout my life."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["...Rest in peace."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn arm1(ctx: &Ctx) -> Script {
    arm1_run(ctx, Arm1Step::Start, Vec::new()).map(|_| ())
}

pub fn arm1_ontouch(ctx: &Ctx) -> Script {
    arm1_run(ctx, Arm1Step::OnTouch, Vec::new()).map(|_| ())
}

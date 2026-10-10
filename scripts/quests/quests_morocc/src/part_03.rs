use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum S3roomBarmuntStep {
    Start,
    OnTouch,
}

fn s_3room_barmunt_run(ctx: &Ctx, mut step: S3roomBarmuntStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            S3roomBarmuntStep::Start => {
                step = S3roomBarmuntStep::OnTouch;
                continue 'machine;
            }
            S3roomBarmuntStep::OnTouch => {
                if ctx.var("barmunt_crow").get()?.number()? < 4 {
                    ctx.call(Function::Warp, vec![Val::from("yuno_in04"), Val::from(100), Val::from(3)])?;
                } else {
                    ctx.mes("^660000You walked through the flame engulfed hallway and ended at a room that was about to collapse, just like everywhere else in this place.^000000")?;
                    ctx.next()?;
                    ctx.mes("^660000You could not clearly see what was inside the room because of all the smoke, but you knew there was something there.^000000")?;
                    ctx.next()?;
                    ctx.mes("^660000You squinted your eyes, and listened. You could hear human voices, but their silhouettes of whomever was talking were not human shaped.^000000")?;
                    ctx.next()?;
                    ctx.lines_as("Female", args!["Cough! Fire... Cough!", "Goddamn humans... Cough, cough!"])?;
                    ctx.next()?;
                    ctx.mes("^660000Her coughing grew worse as the roaring of the flames grew stronger.^000000")?;
                    ctx.next()?;
                    ctx.lines_as("Male", args!["They ran away, and left us behind to die!"])?;
                    ctx.next()?;
                    ctx.lines_as("Female", args!["...Are we going to die?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Male",
                        args![
                            "Die? ...Haha.",
                            "^3131FF'Eva'^000000, we won't die... We were never exactly alive to begin with."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.mes("- Crumbling -")?;
                    ctx.next()?;
                    ctx.lines_as("Male", args!["Argh!"])?;
                    ctx.next()?;
                    ctx.mes("^660000The only exit was blocked by steel bars; no one could get out of the room enveloped in flames. Then, a corner of a wall crumbled by the heat from the fire.^000000")?;
                    ctx.next()?;
                    ctx.mes("^660000The male silhouette quickly approached the wall, and started to dig at the crumbled corner.^000000")?;
                    ctx.next()?;
                    ctx.lines_as("Male", args!["...It's done!"])?;
                    ctx.next()?;
                    ctx.lines_as("Male", args!["Let's go, Eva!"])?;
                    ctx.next()?;
                    ctx.lines_as("Eva", args!["Wait, we have to take the children with us."])?;
                    ctx.next()?;
                    ctx.mes("^660000The female silhouette called Eva approached the test tubes standing in a row against a wall.^000000")?;
                    ctx.next()?;
                    ctx.lines_as("Male", args!["Eva!! We don't have time to save all of them! Come here!"])?;
                    if ctx.var("barmunt_crow").get()? == 4 {
                        ctx.var("barmunt_crow").set(Val::from(5))?;
                    }
                    ctx.call(
                        Function::StartStatus,
                        vec![ctx.constant("SC_BLIND")?, Val::from(600000), Val::from(0), Val::from(10000)],
                    )?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("que_ba"), Val::from(53), Val::from(232)])?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn s_3room_barmunt(ctx: &Ctx) -> Script {
    s_3room_barmunt_run(ctx, S3roomBarmuntStep::Start, Vec::new()).map(|_| ())
}

pub fn s_3room_barmunt_ontouch(ctx: &Ctx) -> Script {
    s_3room_barmunt_run(ctx, S3roomBarmuntStep::OnTouch, Vec::new()).map(|_| ())
}

fn grotesque_woman_eva1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_CLOAKING")?])?;
    return Err(Stop::End);
}

pub fn grotesque_woman_eva1(ctx: &Ctx) -> Script {
    grotesque_woman_eva1_body(ctx, Vec::new()).map(|_| ())
}

fn grotesque_man_zid1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_CLOAKING")?])?;
    return Err(Stop::End);
}

pub fn grotesque_man_zid1(ctx: &Ctx) -> Script {
    grotesque_man_zid1_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum GarasFYumeStep {
    Start,
    OnTouch,
}

fn garas_f_yume_run(ctx: &Ctx, mut step: GarasFYumeStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            GarasFYumeStep::Start => {
                step = GarasFYumeStep::OnTouch;
                continue 'machine;
            }
            GarasFYumeStep::OnTouch => {
                if ctx.var("barmunt_crow").get()? == 5 {
                    ctx.mes("^660000Suddenly a flash of light stung your eyes. A few seconds later, you opened your eyes and found that you were somewhere in Schwarzwald.^000000")?;
                    ctx.next()?;
                    ctx.mes("^660000And you aren't alone: you can now clearly see the man and woman that escaped the laboratory.^000000")?;
                    ctx.next()?;
                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Umm..."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Grotesque Man",
                        args!["Where should we go?", "No one will welcome us... We look like monsters!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Eva",
                        args!["...", "I know a cave that's hidden in the north.", "We can stay there for a while."],
                    )?;
                    ctx.next()?;
                    ctx.mes("^660000They seemed completely oblivious of your presence, and headed north.^000000")?;
                    ctx.next()?;
                    ctx.mes("^660000You were looking at their backs as they left, and then noticed that you were holding the book you were reading in your hands. Then....^000000")?;
                    ctx.var("barmunt_crow").set(Val::from(6))?;
                    ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_FLASHER")?])?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("yuno_in04"), Val::from(100), Val::from(3)])?;
                } else {
                    ctx.call(Function::Warp, vec![Val::from("yuno_in04"), Val::from(100), Val::from(3)])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn garas_f_yume(ctx: &Ctx) -> Script {
    garas_f_yume_run(ctx, GarasFYumeStep::Start, Vec::new()).map(|_| ())
}

pub fn garas_f_yume_ontouch(ctx: &Ctx) -> Script {
    garas_f_yume_run(ctx, GarasFYumeStep::OnTouch, Vec::new()).map(|_| ())
}

fn grotesque_woman_eva2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_CLOAKING")?])?;
    return Err(Stop::End);
}

pub fn grotesque_woman_eva2(ctx: &Ctx) -> Script {
    grotesque_woman_eva2_body(ctx, Vec::new()).map(|_| ())
}

fn grotesque_man_zid2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_CLOAKING")?])?;
    return Err(Stop::End);
}

pub fn grotesque_man_zid2(ctx: &Ctx) -> Script {
    grotesque_man_zid2_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum BarmutRoom1Step {
    Start,
    OnTouch,
}

fn barmut_room1_run(ctx: &Ctx, mut step: BarmutRoom1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            BarmutRoom1Step::Start => {
                step = BarmutRoom1Step::OnTouch;
                continue 'machine;
            }
            BarmutRoom1Step::OnTouch => {
                if ctx.var("barmunt_crow").get()? == 11 {
                    ctx.mes("^660000You wake up, and look around you. Somehow, you've now ended up in a huge mansion.^000000")?;
                    ctx.next()?;
                    ctx.mes("^660000You suddenly feel a sharp pain in your chest, as if someone were squeezing your heart.^000000")?;
                    ctx.next()?;
                    ctx.mes("^660000This mansion looks very elegant and expensive, but it is on fire, just like the laboratory in your first dream.^000000")?;
                    ctx.next()?;
                    ctx.mes("^660000You're having trouble thinking because of your chest pain and the fire, but you know that this mansion won't last long. You look around for an exit, and see a man standing downstairs in the center of flames.^000000")?;
                    ctx.next()?;
                    ctx.mes("^660000He didn't seem to care that he'd perish along with the mansion, and was talking slowly in a low, sad voice.^000000")?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mysterious Man",
                        args![
                            ".....................",
                            "After all, God has trifled with me...",
                            "This was his plan all long...",
                            "Even my best friends were his puppets..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mysterious Man",
                        args![
                            "I strongly resent my fate...",
                            "But I'm ready to accept the end... Of everything..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.mes("^660000He could not finish his words before flames swallowed him up. At the same time, you were...^000000")?;
                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_METEORSTORM")?])?;
                    ctx.var("barmunt_crow").set(Val::from(12))?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("yuno_in04"), Val::from(108), Val::from(57)])?;
                } else {
                    ctx.call(Function::Warp, vec![Val::from("yuno_in04"), Val::from(108), Val::from(57)])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn barmut_room1(ctx: &Ctx) -> Script {
    barmut_room1_run(ctx, BarmutRoom1Step::Start, Vec::new()).map(|_| ())
}

pub fn barmut_room1_ontouch(ctx: &Ctx) -> Script {
    barmut_room1_run(ctx, BarmutRoom1Step::OnTouch, Vec::new()).map(|_| ())
}

fn barmunt_fire_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn barmunt_fire(ctx: &Ctx) -> Script {
    barmunt_fire_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum BarmuntLivingStep {
    Start,
    OnTouch,
}

fn barmunt_living_run(ctx: &Ctx, mut step: BarmuntLivingStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            BarmuntLivingStep::Start => {
                step = BarmuntLivingStep::OnTouch;
                continue 'machine;
            }
            BarmuntLivingStep::OnTouch => {
                ctx.lines(args![
                    "..................",
                    "..................",
                    "..................",
                    ".................."
                ])?;
                ctx.next()?;
                ctx.lines_as(
                    "Mysterious Man",
                    args!["..................", "..................", "..................", "E...Eva, no!"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mysterious Man",
                    args!["Was it a dream?", "..................", "...Gosh, that was strange."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mysterious Man",
                    args![
                        "Mother was... She was a regular human.",
                        "Was I standing next to her?",
                        ".................."
                    ],
                )?;
                ctx.next()?;
                ctx.mes("^660000He mumbled to himself, and brushed back his sweaty hair with a trembling hand.^000000")?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "A dream again.",
                        "I guess he won't be able to see or hear me, just like the others."
                    ],
                )?;
                ctx.next()?;
                ctx.lines(args![
                    "^660000You put your guard down, and stare at him.",
                    "Suddenly his face distorted in anger as he looked right back at you.^000000"
                ])?;
                ctx.next()?;
                ctx.lines_as("Mysterious Man", args!["How the hell did you find me?!"])?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["...Wah! He can see me! He can see me!"],
                )?;
                ctx.next()?;
                ctx.call(Function::EnableNpc, vec![Val::from("#barmut_onna")])?;
                ctx.lines_as("Mysterious Woman", args!["You don't have to be so angry at me, sir."])?;
                ctx.next()?;
                ctx.lines(args![
                    "^660000As soon as you heard her voice,",
                    "you turned around and found a woman standing there.^000000"
                ])?;
                ctx.next()?;
                ctx.lines_as("Mysterious Woman", args!["It was almost done, but then you broke all of them -- even ^FF00FFBEEP^000000 and ^FF00FFBEEP^000000 -- and were hiding in here."])?;
                ctx.next()?;
                ctx.lines_as("Mysterious Man", args!["They weren't supposed to exist in this world!"])?;
                ctx.next()?;
                ctx.lines_as("Mysterious Woman", args!["It's his decision. Your fate is on his..."])?;
                ctx.next()?;
                ctx.lines_as("Mysterious Man", args!["Shut up!"])?;
                ctx.next()?;
                ctx.mes("^660000The raging man drew a sword, and pointed it at her. She then suddenly transformed into a crow and flew away. The echos of her laughter could be heard outside the window.^000000")?;
                ctx.call(Function::DisableNpc, vec![Val::from("#barmut_onna")])?;
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BAT")?])?;
                ctx.next()?;
                ctx.lines_as(
                    "Echoing Voice",
                    args!["You are bound to your fate, no matter how hard you try to escape..."],
                )?;
                ctx.call(Function::SoundEffect, vec![Val::from("loli_ruri_stand.wav"), Val::from(0)])?;
                ctx.next()?;
                ctx.lines(args![
                    "..................",
                    "..................",
                    "..................",
                    ".................."
                ])?;
                ctx.next()?;
                ctx.lines(args![
                    "..................",
                    "..................",
                    "..................",
                    ".................."
                ])?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("yuno_in04"), Val::from(165), Val::from(122)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn barmunt_living(ctx: &Ctx) -> Script {
    barmunt_living_run(ctx, BarmuntLivingStep::Start, Vec::new()).map(|_| ())
}

pub fn barmunt_living_ontouch(ctx: &Ctx) -> Script {
    barmunt_living_run(ctx, BarmuntLivingStep::OnTouch, Vec::new()).map(|_| ())
}

fn barmut_room2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn barmut_room2(ctx: &Ctx) -> Script {
    barmut_room2_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum BarmutOnnaStep {
    Start,
    OnInit,
}

fn barmut_onna_run(ctx: &Ctx, mut step: BarmutOnnaStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            BarmutOnnaStep::Start => {
                step = BarmutOnnaStep::OnInit;
                continue 'machine;
            }
            BarmutOnnaStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("#barmut_onna")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn barmut_onna(ctx: &Ctx) -> Script {
    barmut_onna_run(ctx, BarmutOnnaStep::Start, Vec::new()).map(|_| ())
}

pub fn barmut_onna_oninit(ctx: &Ctx) -> Script {
    barmut_onna_run(ctx, BarmutOnnaStep::OnInit, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum CaveSettlerG1Step {
    Start,
    OnTouch,
}

fn cave_settler_g1_run(ctx: &Ctx, mut step: CaveSettlerG1Step, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input_s = Val::from("");
    'machine: loop {
        match step {
            CaveSettlerG1Step::Start => {
                if ctx.var("barmunt_crow").get()? == 8 {
                    ctx.lines_as("Cave Settler", args!["No outsiders are allowed beyond this point."])?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Step back.:No, listen to me!")])? {
                        1 => {
                            ctx.lines_as("Cave Settler", args!["Hah, I knew you would be scared of me!"])?;
                            ctx.close_window()?;
                            ctx.call(Function::Warp, vec![Val::from("cave"), Val::from(81), Val::from(92)])?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "Ah... Ahchoo!",
                                    "'I'm not like other outsiders... Ahchoo!",
                                    "I have a goal to achieve in here... Ahchoo!"
                                ],
                            )?;
                            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_VENOMDUST")?])?;
                            ctx.next()?;
                            ctx.mes("^660000Although you a bit intimidated by this guard, and your sneezing definitely not helping, you continue.^000000")?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["I'm here to find somebody!"],
                            )?;
                            let (input, status) = runtime::input_text(ctx, None, None)?;
                            l_input_s = input;
                            ctx.lines(args![
                                ((Val::from("Her name is ^FF0000") + l_input_s.clone()) + Val::from("^000000!"))
                            ])?;
                            ctx.next()?;
                            if runtime::compare(&l_input_s.clone(), &Val::from("Eva")) == 0 {
                                ctx.lines_as(
                                    "Cave Settler",
                                    args!["Are you kidding me? If you're looking for a missing child, go to the Juno police station!"],
                                )?;
                                ctx.close_window()?;
                                ctx.call(Function::Warp, vec![Val::from("cave"), Val::from(81), Val::from(92)])?;
                                return Err(Stop::End);
                            } else {
                                ctx.lines_as("Cave Settler", args!["............!!!!"])?;
                                ctx.next()?;
                                ctx.lines_as("Cave Settler", args!["Did you just say Eva?", "Hmm...", "Wait here."])?;
                                ctx.next()?;
                                ctx.call(Function::Emotion, vec![ctx.constant("ET_COMEON")?])?;
                                ctx.lines_as("Cave Settler", args!["Hey, Jaeda!"])?;
                                ctx.call(Function::EnableNpc, vec![Val::from("Jaeda#garas1")])?;
                                ctx.next()?;
                                ctx.lines_as("Cave Settler", args!["Go upstairs, and tell Eva that she has a visitor."])?;
                                ctx.call(Function::DisableNpc, vec![Val::from("Jaeda#garas1")])?;
                                ctx.next()?;
                                ctx.mes("^660000Surprisingly, he seemed to know who Eva is. You still could not believe that she actually exists! It's all so very strange.^000000")?;
                                ctx.next()?;
                                ctx.mes("^660000You can feel the excitement and anticipation well within you. Maybe you're coming closer to learning what your dreams really mean.^000000")?;
                                ctx.call(Function::EnableNpc, vec![Val::from("Jaeda#garas1")])?;
                                ctx.next()?;
                                ctx.lines_as("Cave Settler", args!["Hmm... Yeah? I see."])?;
                                ctx.next()?;
                                ctx.call(Function::DisableNpc, vec![Val::from("Jaeda#garas1")])?;
                                ctx.lines_as("Cave Settler", args!["Hey, you can pass. Go upstairs, but you'd better think twice before trying anything funny. Do you understand me?"])?;
                                ctx.next()?;
                                ctx.lines(args![
                                    "^660000The stern-looking man examined you, and then stepped to the side, allowing you to pass.",
                                    "You clenched your fists in nervousness, and then started up the dark stairwell.^000000"
                                ])?;
                                ctx.var("barmunt_crow").set(Val::from(9))?;
                                ctx.close_window()?;
                                ctx.call(Function::Warp, vec![Val::from("cave"), Val::from(82), Val::from(105)])?;
                                return Err(Stop::End);
                            }
                        }
                        _ => {}
                    }
                } else if ctx.var("barmunt_crow").get()?.number()? < 8 {
                    ctx.lines_as("Cave Settler", args!["No outsiders are allowed beyond this point."])?;
                    ctx.next()?;
                    ctx.mes("^660000His voice is as stern and intimidating as his appearance.^000000")?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("cave"), Val::from(81), Val::from(92)])?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Cave Settler",
                        args![
                            "Do you still have business with Zid?",
                            "You'd better finish it quickly because we don't like outsiders running around our village."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                step = CaveSettlerG1Step::OnTouch;
                continue 'machine;
            }
            CaveSettlerG1Step::OnTouch => {
                if ctx.var("barmunt_crow").get()? == 8 {
                    ctx.lines_as("Cave Settler", args!["No outsiders are allowed beyond this point."])?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Step back.:No, listen to me!")])? {
                        1 => {
                            ctx.lines_as("Cave Settler", args!["Hah, I knew you would be scared of me!"])?;
                            ctx.close_window()?;
                            ctx.call(Function::Warp, vec![Val::from("cave"), Val::from(81), Val::from(92)])?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "Ah... Ahchoo!",
                                    "'I'm not like other outsiders... Ahchoo!",
                                    "I have a goal to achieve in here... Ahchoo!"
                                ],
                            )?;
                            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_VENOMDUST")?])?;
                            ctx.next()?;
                            ctx.mes("^660000Although you're a bit intimidated by this guard, and your sneezing definitely not helping, you continue.^000000")?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["I'm here to find somebody!"],
                            )?;
                            let (input, status) = runtime::input_text(ctx, None, None)?;
                            l_input_s = input;
                            ctx.lines(args![
                                ((Val::from("Her name is ^FF0000") + l_input_s.clone()) + Val::from("^000000!"))
                            ])?;
                            ctx.next()?;
                            if runtime::compare(&l_input_s.clone(), &Val::from("Eva")) == 0 {
                                ctx.lines_as(
                                    "Cave Settler",
                                    args!["Are you kidding me? If you're looking for a missing child, go to the Juno police station!"],
                                )?;
                                ctx.close_window()?;
                                ctx.call(Function::Warp, vec![Val::from("cave"), Val::from(81), Val::from(92)])?;
                                return Err(Stop::End);
                            } else {
                                ctx.lines_as("Cave Settler", args!["............!!!!"])?;
                                ctx.next()?;
                                ctx.lines_as("Cave Settler", args!["Did you just say Eva?", "Hmm...", "Wait here."])?;
                                ctx.next()?;
                                ctx.call(Function::Emotion, vec![ctx.constant("ET_COMEON")?])?;
                                ctx.lines_as("Cave Settler", args!["Hey, Jaeda!"])?;
                                ctx.call(Function::EnableNpc, vec![Val::from("Jaeda#garas1")])?;
                                ctx.next()?;
                                ctx.lines_as("Cave Settler", args!["Go upstairs, and tell Eva that she has a visitor."])?;
                                ctx.call(Function::DisableNpc, vec![Val::from("Jaeda#garas1")])?;
                                ctx.next()?;
                                ctx.mes("^660000Surprisingly, he seemed to know who Eva is. You still could not believe that she actually exists! It's all so very strange.^000000")?;
                                ctx.next()?;
                                ctx.mes("^660000You can feel the excitement and anticipation well within you. Maybe you're coming closer to learning what your dreams really mean.^000000")?;
                                ctx.call(Function::EnableNpc, vec![Val::from("Jaeda#garas1")])?;
                                ctx.next()?;
                                ctx.lines_as("Cave Settler", args!["Hmm... Yeah? I see."])?;
                                ctx.next()?;
                                ctx.call(Function::DisableNpc, vec![Val::from("Jaeda#garas1")])?;
                                ctx.lines_as("Cave Settler", args!["Hey, you can pass. Go upstairs, but you'd better think twice before trying anything funny. Do you understand me?"])?;
                                ctx.next()?;
                                ctx.lines(args![
                                    "^660000The stern-looking man examined you, and then stepped to the side, allowing you to pass.",
                                    "You clenched your fists in nervousness, and then started up the dark stairwell.^000000"
                                ])?;
                                ctx.var("barmunt_crow").set(Val::from(9))?;
                                ctx.close_window()?;
                                ctx.call(Function::Warp, vec![Val::from("cave"), Val::from(82), Val::from(105)])?;
                                return Err(Stop::End);
                            }
                        }
                        _ => {}
                    }
                } else if ctx.var("barmunt_crow").get()?.number()? < 8 {
                    ctx.lines_as("Cave Settler", args!["No outsiders are allowed beyond this point."])?;
                    ctx.next()?;
                    ctx.mes("^660000His voice is as stern and intimidating as his appearance.^000000")?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("cave"), Val::from(81), Val::from(92)])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn cave_settler_g1(ctx: &Ctx) -> Script {
    cave_settler_g1_run(ctx, CaveSettlerG1Step::Start, Vec::new()).map(|_| ())
}

pub fn cave_settler_g1_ontouch(ctx: &Ctx) -> Script {
    cave_settler_g1_run(ctx, CaveSettlerG1Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum JaedaGaras1Step {
    Start,
    OnInit,
}

fn jaeda_garas1_run(ctx: &Ctx, mut step: JaedaGaras1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            JaedaGaras1Step::Start => {
                step = JaedaGaras1Step::OnInit;
                continue 'machine;
            }
            JaedaGaras1Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Jaeda#garas1")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn jaeda_garas1(ctx: &Ctx) -> Script {
    jaeda_garas1_run(ctx, JaedaGaras1Step::Start, Vec::new()).map(|_| ())
}

pub fn jaeda_garas1_oninit(ctx: &Ctx) -> Script {
    jaeda_garas1_run(ctx, JaedaGaras1Step::OnInit, Vec::new()).map(|_| ())
}

fn monsterous_man_zid_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_exitloop = Val::from(0);
    if ctx.var("barmunt_crow").get()? == 9 {
        ctx.lines(args!["^660000You are looking at a man who appears as hideous as a monster.", "He was the man with Eva that you saw in your dream. Of course, he's older now, and his age is showing in his face and skin.^000000"])?;
        ctx.next()?;
        ctx.mes("(WHIZZ)")?;
        ctx.next()?;
        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Argh..."])?;
        ctx.next()?;
        ctx.lines(args![
            "^660000The pain in your chest suddenly hits you...",
            "And your lungs seem much weaker from all of the coughing and sneezing you've suffered lately.^000000"
        ])?;
        ctx.next()?;
        ctx.mes("^660000Rubbing your chest to ease the pain, you keep staring at him in surprise.^000000")?;
        ctx.next()?;
        ctx.mes("^660000Thankfully, he speaks to you first.^000000")?;
        ctx.next()?;
        ctx.lines_as("Monsterous Man", args!["Does my appearance bother you?"])?;
        ctx.next()?;
        ctx.lines_as("Monsterous Man", args!["So, you know Eva... Huh?"])?;
        ctx.next()?;
        ctx.mes("^660000He sounded weak, as if he did not have enough energy to talk.^000000")?;
        ctx.next()?;
        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Umm... Ahchoo!"])?;
        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_VENOMDUST")?])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Do you know a man by the name of Oliver Hilpert?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Monsterous Man",
            args!["Oliver? Well, I don't think so.", "What does he have to do with anything?"],
        )?;
        ctx.next()?;
        ctx.mes(
            "^660000You have given him Oliver's novel, <The Crow of the Fate>, explaining strange events that have happened to you.^000000",
        )?;
        ctx.next()?;
        ctx.mes("....................")?;
        ctx.next()?;
        ctx.lines(args!["....................", "...................."])?;
        ctx.next()?;
        ctx.lines(args!["....................", "....................", "...................."])?;
        ctx.next()?;
        ctx.mes("^660000He squinted his eyes in curiosity and looked through a few pages of the book quietly. Then he opened his mouth as he handed the book back to you.^000000")?;
        ctx.next()?;
        ctx.lines_as(
            "Monsterous Man",
            args!["I'm surprised that the story in this book is... It's similar to my life story. Very similar."],
        )?;
        ctx.next()?;
        'l1: loop {
            if !(true) {
                break 'l1;
            }
            'b1: {
                ctx.lines_as(
                    "Zid",
                    args!["My name is Zid. Tell me what you want from me. I want to know more about this book."],
                )?;
                ctx.next()?;
                match runtime::select_values(
                    ctx,
                    &[Val::from("About the Cave Village:About Himself:About Eva:End Conversation")],
                )? {
                    1 => {
                        ctx.lines_as("Zid", args!["Did you notice that our villagers look different?"])?;
                        ctx.next()?;
                        ctx.lines_as("Zid", args!["Human curiosity has led to great achievements and contributions, but there is a point where knowledge may be too much to handle.", "Everyone in this village is a runaway... We are victims of what happens when people play God, seek out forbidden knowledge, and toy with life."])?;
                        ctx.next()?;
                        ctx.lines_as("Zid", args!["We have no choice but hide here because they are too powerful for us to speak out against... And we know that isn't right."])?;
                        ctx.next()?;
                    }
                    2 => {
                        ctx.lines_as("Zid", args!["I was a normal man once, but poverty made me lose my pride, self-esteem, and identity.", "I was sick and tired of worrying about food everyday, and I volunteered to be a test subject in some experiment.", "I no longer had to worry about starving, but my life as a 'human' ended that day."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Zid",
                            args![
                                "At first, Eva and I were the only ones here, but others joined us, and we formed a small settlement.",
                                "Now I'm the oldest villager, and feel the heavy responsibility of protecting others."
                            ],
                        )?;
                        ctx.next()?;
                    }
                    3 => {
                        ctx.lines_as(
                            "Zid",
                            args![
                                "The laboratory in which Eva and I were being tested was not ordinary.",
                                "It was built to test and duplicate God's power of creation."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Zid",
                            args![
                                "They failed to achieve their objective, and caused a series of unfortunate events",
                                "by trying to manipulate materials that were difficult to even examine.",
                                "They believed their research would open a whole new world for mankind..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Zid", args!["Eva was one of them... Although she wasn't kind, she at least sympathized for me because I sold myself out of poverty."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Zid",
                            args![
                                "She had a man who loved her so much...",
                                "His name was Sefakiest, and he always supported her research."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Zid", args!["He... Killed himself in despair after Eva was turned into a mutant just like me... She wanted him to use her as his test subject.", "No one expected him to end his life so easily when he was the most enthusiastic researcher of all."])?;
                        ctx.next()?;
                        ctx.lines_as("Zid", args!["Then somebody set the laboratory on fire. I assume you've seen that in your dream. The fire completely burned down the building."])?;
                        ctx.next()?;
                        ctx.lines_as("Zid", args!["Eva and I were able to escape before we could be found, and then settled down here. After all, we can't just mingle with ordinary humans."])?;
                        ctx.next()?;
                        ctx.lines_as("Zid", args!["Eva managed to save one of the test subjects which actually grew up into a normal looking human, but he was actually perfect."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Zid",
                            args!["That boy ran away when he was about 10. I guess he hit puberty... You know boys and girls at that age."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Zid", args!["Eva couldn't forget him. She was waiting for his return inside this dark cave. For a while, he was sending letters without his address, but then they stopped coming. That's when Eva disappeared.", "I found ^3131FFa giant black feather^000000 in her room after she left."])?;
                        ctx.next()?;
                        ctx.lines_as("Zid", args!["I don't know why, but... I believe Eva no longer exists..."])?;
                        ctx.next()?;
                    }
                    4 => {
                        l_exitloop = Val::from(1);
                    }
                    _ => {}
                }
                if l_exitloop.clone().is_true() {
                    break 'l1;
                }
            }
        }
        ctx.lines_as("Zid", args!["Hmm...", "Maybe Oliver is Eva's boy..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Zid",
            args!["No, he's too young to be him because it happened a long time ago..."],
        )?;
        ctx.next()?;
        ctx.lines_as("Zid", args!["........................."])?;
        ctx.next()?;
        ctx.lines_as(
            "Zid",
            args!["I guess it's just an old memory... Why should I care? Why should anyone?"],
        )?;
        ctx.next()?;
        ctx.mes("^660000Zid stopped talking, and then lowered his head. After the pause, he opened his mouth again.^000000")?;
        ctx.next()?;
        ctx.lines_as(
            "Zid",
            args![
                "Now I'm too old to remember even Eva or the boy.",
                "My purpose in life is to protect the other villagers. They have lives ahead of them, living inside this dark cave."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Zid",
            args!["The book was interesting, but I'm sure that guy, Oliver, experienced the same thing that happened to you."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Zid",
            args!["Even if he has something to do with Eva, it's too late to do anything for her."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Zid",
            args!["Think about it: who can save us from this dark cave, even if Eva or that boy is still alive? The answer is nobody."],
        )?;
        ctx.next()?;
        ctx.lines_as("Zid", args!["................................."])?;
        ctx.next()?;
        ctx.lines_as(
            "Zid",
            args![
                "I'm so sorry for not being much help.",
                "You'd better wake up from the dream, and move on with your life.",
                "...Sadly everything is nothing but a dream..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Zid",
            args!["If you really feel sorry for us, please do not let others know where we are."],
        )?;
        ctx.next()?;
        ctx.mes("^660000Zid turned his head away from you. You're still not sure if all of this is really happening. It's all so surreal...^000000")?;
        ctx.next()?;
        ctx.mes("^660000Frustrated by the conversation with Zid, you have decided to go back to Morocc and forget everything.^000000")?;
        ctx.var("barmunt_crow").set(Val::from(10))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(2066), Val::from(2067)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("barmunt_crow").get()? == 10 {
        ctx.lines(args![
            "Zid seemed to have lost his interest in the book.",
            "You have decided to deliver the book to Benjamin of Morocc."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as("Monsterous Man", args!["Don't look at my face..."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn monsterous_man_zid(ctx: &Ctx) -> Script {
    monsterous_man_zid_body(ctx, Vec::new()).map(|_| ())
}

fn cave_settler_f_cave1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Cave Settler", args!["(Startled)"])?;
    ctx.next()?;
    ctx.lines_as("Cave Settler", args!["Mumble... Mumble..."])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn cave_settler_f_cave1(ctx: &Ctx) -> Script {
    cave_settler_f_cave1_body(ctx, Vec::new()).map(|_| ())
}

fn cave_settler_m_cave2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Cave Settler", args!["Heh...", "You want... This?"])?;
    ctx.next()?;
    ctx.mes("- She showed you Monster's Feed. -")?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn cave_settler_m_cave2(ctx: &Ctx) -> Script {
    cave_settler_m_cave2_body(ctx, Vec::new()).map(|_| ())
}

fn cave_settler_m_cave3_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Cave Settler",
        args!["Accessory!", "My pretty accessory!", "Waaah!", "My accessory is gone!"],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn cave_settler_m_cave3(ctx: &Ctx) -> Script {
    cave_settler_m_cave3_body(ctx, Vec::new()).map(|_| ())
}

fn cave_settler_f_cave4_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Cave Settler", args!["No one in this village welcomes you. Go back."])?;
    ctx.next()?;
    ctx.lines_as("Cave Settler", args!["Your eyes are so tempting... I wanna eat them! Heh heh!"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn cave_settler_f_cave4(ctx: &Ctx) -> Script {
    cave_settler_f_cave4_body(ctx, Vec::new()).map(|_| ())
}

fn cave_settler_f_cave6_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Cave Settler", args!["Hoho, did he really say that yesterday...?", "Wah!"])?;
    ctx.next()?;
    ctx.lines_as("Cave Settler", args!["It's a human..."])?;
    ctx.next()?;
    ctx.lines(args!["(Whisper)", "(Whisper)"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn cave_settler_f_cave6(ctx: &Ctx) -> Script {
    cave_settler_f_cave6_body(ctx, Vec::new()).map(|_| ())
}

fn cave_settler_f_cave7_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Cave Settler", args!["Hoho, did he really say that yesterday...?", "Wah!"])?;
    ctx.next()?;
    ctx.lines_as("Cave Settler", args!["It's a human..."])?;
    ctx.next()?;
    ctx.lines(args!["(Whisper)", "(Whisper)"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn cave_settler_f_cave7(ctx: &Ctx) -> Script {
    cave_settler_f_cave7_body(ctx, Vec::new()).map(|_| ())
}

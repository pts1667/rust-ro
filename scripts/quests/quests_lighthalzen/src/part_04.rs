use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn li_varmunt_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CountItem, vec![Val::from(7345)])?.number()? > 0 {
        if ctx.var("lhz_curse").get()? == 12 {
            ctx.lines_as("??", args!["Doctor Varmunt,", "you've finally agreed", "to join us. Welcome!"])?;
            ctx.next()?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_SWEAT")?])?;
            ctx.lines_as(
                "Varmunt",
                args![
                    "Well, I don't know if I agree",
                    "with this company's policies,",
                    "but the project you're offering",
                    "seems to be an opportunity that",
                    "comes once in a lifetime, so..."
                ],
            )?;
            ctx.next()?;
            ctx.call(
                Function::Emotion,
                vec![
                    ctx.constant("ET_BEST")?,
                    ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("#li_researcher")])?,
                ],
            )?;
            ctx.lines_as(
                "??",
                args![
                    "To be honest, this project",
                    "can only be a success with",
                    "your cooperation. We need",
                    "your genius and will provide",
                    "whatever you require."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Varmunt",
                args![
                    "I'm flattered.",
                    "And of course, I'll",
                    "do my best. It's just",
                    "that this deal sounds",
                    "too good to be true..."
                ],
            )?;
            ctx.next()?;
            ctx.mes("............")?;
            ctx.next()?;
            ctx.call(Function::SoundEffect, vec![Val::from("loli_ruri_stand.wav"), Val::from(0)])?;
            ctx.mes(".................")?;
            ctx.next()?;
            ctx.call(Function::SoundEffect, vec![Val::from("loli_ruri_stand.wav"), Val::from(0)])?;
            ctx.mes("...................")?;
            ctx.next()?;
            ctx.call(Function::SoundEffect, vec![Val::from("loli_ruri_stand.wav"), Val::from(0)])?;
            ctx.mes(".....................")?;
            ctx.next()?;
            ctx.mes(".......................")?;
            ctx.next()?;
            ctx.lines_as(
                "??",
                args![
                    "Amazing. You've accomplished",
                    "what most have thought to be",
                    "impossible ahead of schedule.",
                    "An imitation of Ymir's Heart!",
                    "This will surely spur Airship",
                    "and Guardian development~!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Varmunt",
                args![
                    "I still don't believe that",
                    "were able to do it. This is",
                    "a huge leap for science, even",
                    "if this imitation isn't as powerful as the real Ymir's Heart."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "??",
                args![
                    "Come, we must celebrate!",
                    "Let's go outside and have",
                    "a toast in your honor! Ha ha~"
                ],
            )?;
            ctx.next()?;
            ctx.mes("............")?;
            ctx.next()?;
            ctx.call(Function::SoundEffect, vec![Val::from("loli_ruri_stand.wav"), Val::from(0)])?;
            ctx.mes(".................")?;
            ctx.next()?;
            ctx.call(Function::SoundEffect, vec![Val::from("loli_ruri_stand.wav"), Val::from(0)])?;
            ctx.mes("....................")?;
            ctx.next()?;
            ctx.call(Function::SoundEffect, vec![Val::from("loli_ruri_stand.wav"), Val::from(0)])?;
            ctx.mes("......................")?;
            ctx.next()?;
            ctx.lines_as(
                "??",
                args![
                    "But why, Doctor Varmunt?",
                    "If you're unhappy with the",
                    "Rekenber Corporation for",
                    "any reason whatsoever..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Varmunt",
                args![
                    "Well, I don't really",
                    "have a reason to remain",
                    "now that we've accomplished",
                    "what I've agreed to do. It's time for me to return and work on",
                    "my personal research."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "??",
                args![
                    "Please, Doctor Varmunt,",
                    "reconsider! You may have",
                    "full use of our facilities to",
                    "conduct your research. I'm",
                    "willing to make you an offer."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Varmunt",
                args![
                    "No, I can't...",
                    "If I continue to work",
                    "here, I'm afraid I might",
                    "make a lot of people",
                    "unhappy. But, thank",
                    "you for everything."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("??", args!["Wait...", "Doctor Varmunt.", "You forgot your cane."])?;
            ctx.next()?;
            ctx.lines_as(
                "Varmunt",
                args![
                    "Since when have you",
                    "seen me use a cane?",
                    "And even if I did,",
                    "that one isn't mi--"
                ],
            )?;
            ctx.next()?;
            ctx.mes("...")?;
            ctx.next()?;
            ctx.lines(args!["...", "......"])?;
            ctx.next()?;
            ctx.lines(args!["...", "......", "........."])?;
            ctx.var("lhz_curse").set(Val::from(13))?;
            ctx.close_window()?;
            ctx.call(Function::Warp, vec![Val::from("lhz_in01"), Val::from(206), Val::from(129)])?;
            return Err(Stop::End);
        } else {
            ctx.call(Function::Warp, vec![Val::from("lhz_in01"), Val::from(206), Val::from(129)])?;
        }
    } else {
        ctx.call(Function::Warp, vec![Val::from("lhz_in01"), Val::from(206), Val::from(129)])?;
    }
    return Err(Stop::End);
}

pub fn li_varmunt(ctx: &Ctx) -> Script {
    li_varmunt_body(ctx, Vec::new()).map(|_| ())
}

fn li_researcher_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CountItem, vec![Val::from(7345)])?.number()? > 0 {
        if ctx.var("lhz_curse").get()? == 12 {
            ctx.lines(args!["Doctor Varmunt,", "you've finally agreed", "to join us. Welcome!"])?;
            ctx.next()?;
            ctx.call(
                Function::Emotion,
                vec![
                    ctx.constant("ET_SWEAT")?,
                    ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("#li_Varmunt")])?,
                ],
            )?;
            ctx.lines_as(
                "Varmunt",
                args![
                    "Well, I don't know if I agree",
                    "with this company's policies,",
                    "but the project you're offering",
                    "seems to be an opportunity that",
                    "comes once in a lifetime, so..."
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_BEST")?])?;
            ctx.lines_as(
                "??",
                args![
                    "To be honest, this project",
                    "can only be a success with",
                    "your cooperation. We need",
                    "your genius and will provide",
                    "whatever you require."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Varmunt",
                args![
                    "I'm flattered.",
                    "And of course, I'll",
                    "do my best. It's just",
                    "that this deal sounds",
                    "too good to be true..."
                ],
            )?;
            ctx.next()?;
            ctx.mes("............")?;
            ctx.next()?;
            ctx.call(Function::SoundEffect, vec![Val::from("loli_ruri_stand.wav"), Val::from(0)])?;
            ctx.mes(".................")?;
            ctx.next()?;
            ctx.call(Function::SoundEffect, vec![Val::from("loli_ruri_stand.wav"), Val::from(0)])?;
            ctx.mes("...................")?;
            ctx.next()?;
            ctx.call(Function::SoundEffect, vec![Val::from("loli_ruri_stand.wav"), Val::from(0)])?;
            ctx.mes(".....................")?;
            ctx.next()?;
            ctx.mes(".......................")?;
            ctx.next()?;
            ctx.lines_as(
                "??",
                args![
                    "Amazing. You've accomplished",
                    "what most have thought to be",
                    "impossible ahead of schedule.",
                    "An imitation of Ymir's Heart!",
                    "This will surely spur Airship",
                    "and Guardian development~!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Varmunt",
                args![
                    "I still don't believe that",
                    "were able to do it. This is",
                    "a huge leap for science, even",
                    "if this imitation isn't as powerful as the real Ymir's Heart."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "??",
                args![
                    "Come, we must celebrate!",
                    "Let's go outside and have",
                    "a toast in your honor! Ha ha~"
                ],
            )?;
            ctx.next()?;
            ctx.mes("............")?;
            ctx.next()?;
            ctx.call(Function::SoundEffect, vec![Val::from("loli_ruri_stand.wav"), Val::from(0)])?;
            ctx.mes(".................")?;
            ctx.next()?;
            ctx.call(Function::SoundEffect, vec![Val::from("loli_ruri_stand.wav"), Val::from(0)])?;
            ctx.mes("....................")?;
            ctx.next()?;
            ctx.call(Function::SoundEffect, vec![Val::from("loli_ruri_stand.wav"), Val::from(0)])?;
            ctx.mes("......................")?;
            ctx.next()?;
            ctx.lines_as(
                "??",
                args![
                    "But why, Doctor Varmunt?",
                    "If you're unhappy with the",
                    "Rekenber Corporation for",
                    "any reason whatsoever..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Varmunt",
                args![
                    "Well, I don't really",
                    "have a reason to remain",
                    "now that we've accomplished",
                    "what I've agreed to do. It's time for me to return and work on",
                    "my personal research."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "??",
                args![
                    "Please, Doctor Varmunt,",
                    "reconsider! You may have",
                    "full use of our facilities to",
                    "conduct your research. I'm",
                    "willing to make you an offer."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Varmunt",
                args![
                    "No, I can't...",
                    "If I continue to work",
                    "here, I'm afraid I might",
                    "make a lot of people",
                    "unhappy. But, thank",
                    "you for everything."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("??", args!["Wait...", "Doctor Varmunt.", "You forgot your cane."])?;
            ctx.next()?;
            ctx.lines_as(
                "Varmunt",
                args![
                    "Since when have you",
                    "seen me use a cane?",
                    "And even if I did,",
                    "that one isn't mi--"
                ],
            )?;
            ctx.next()?;
            ctx.mes("...")?;
            ctx.next()?;
            ctx.lines(args!["...", "......"])?;
            ctx.next()?;
            ctx.lines(args!["...", "......", "........."])?;
            ctx.var("lhz_curse").set(Val::from(13))?;
            ctx.close_window()?;
            ctx.call(Function::Warp, vec![Val::from("lhz_in01"), Val::from(206), Val::from(129)])?;
            return Err(Stop::End);
        } else {
            ctx.call(Function::Warp, vec![Val::from("lhz_in01"), Val::from(206), Val::from(129)])?;
        }
    } else {
        ctx.call(Function::Warp, vec![Val::from("lhz_in01"), Val::from(206), Val::from(129)])?;
    }
    return Err(Stop::End);
}

pub fn li_researcher(ctx: &Ctx) -> Script {
    li_researcher_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Kiz10Step {
    Start,
    OnTouch,
}

fn kiz10_run(ctx: &Ctx, mut step: Kiz10Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Kiz10Step::Start => {
                step = Kiz10Step::OnTouch;
                continue 'machine;
            }
            Kiz10Step::OnTouch => {
                if ctx.call(Function::CountItem, vec![Val::from(7345)])?.number()? > 0 {
                    if ctx.var("lhz_curse").get()? == 16 {
                        if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0 {
                            ctx.lines(args![
                                "^3355FFThere's something on",
                                "the floor, but you can't",
                                "really take a good look at",
                                "what it is right now. Perhaps",
                                "if you freed up more space",
                                "in your inventory..."
                            ])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines(args!["...", "......"])?;
                        ctx.next()?;
                        ctx.lines(args!["...", "......", "........."])?;
                        ctx.call(Function::SoundEffect, vec![Val::from("loli_ruri_stand.wav"), Val::from(0)])?;
                        ctx.call(
                            Function::StartStatus,
                            vec![ctx.constant("SC_CURSE")?, Val::from(1000), Val::from(0)],
                        )?;
                        ctx.close_window()?;
                        ctx.call(Function::Warp, vec![Val::from("lhz_que01"), Val::from(98), Val::from(59)])?;
                        return Err(Stop::End);
                    } else if ((ctx.var("lhz_curse").get()?.number()? > 0 && ctx.var("lhz_curse").get()?.number()? < 16)
                        && (ctx.var("lhz_curse").get()?.number()? > 16 && ctx.var("lhz_curse").get()?.number()? < 26))
                    {
                        ctx.call(
                            Function::StartStatus,
                            vec![ctx.constant("SC_CURSE")?, Val::from(1000), Val::from(0)],
                        )?;
                    }
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn kiz10(ctx: &Ctx) -> Script {
    kiz10_run(ctx, Kiz10Step::Start, Vec::new()).map(|_| ())
}

pub fn kiz10_ontouch(ctx: &Ctx) -> Script {
    kiz10_run(ctx, Kiz10Step::OnTouch, Vec::new()).map(|_| ())
}

fn li_man_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("lhz_curse").get()? == 16 {
        ctx.lines_as(
            "???",
            args!["It's over.", "I really want", "to quit. This is", "the end for me."],
        )?;
        ctx.next()?;
        ctx.lines_as("Peco Peco", args!["^3131FFThis is the end!", "This is the end!^000000"])?;
        ctx.next()?;
        ctx.lines_as(
            "???",
            args![
                "He was right to say that",
                "we'd be paid well, but going",
                "through this much torture isn't",
                "worth any sum of money in the",
                "world to me. *Sigh* Money..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "???",
            args![
                "She must be so worried",
                "about me by now. And her",
                "health is so bad. I'm such",
                "a fool for leaving her behind."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Peco Peco", args!["Such a fool!", "Such a fool!", "*Squaaawk~*"])?;
        ctx.next()?;
        ctx.lines_as(
            "???",
            args!["Damn it!", "Shut up, you stupid bird!", "Be quiet for just a minute!"],
        )?;
        ctx.next()?;
        ctx.lines_as("Peco Peco", args!["*Squaaaawk!*", "Death Penalty!", "Death Penalty!"])?;
        ctx.next()?;
        ctx.lines_as(
            "???",
            args![
                "Death penalty?",
                "Where'd you learn",
                "to say something weird",
                "like that? Huh. That's..."
            ],
        )?;
        ctx.next()?;
        ctx.mes("..............")?;
        ctx.next()?;
        ctx.mes(".................")?;
        ctx.next()?;
        ctx.mes("....................")?;
        ctx.next()?;
        ctx.mes("......................")?;
        ctx.next()?;
        ctx.lines_as(
            "???",
            args![
                "I'm the only one still",
                "in this room. Everyone",
                "else left and never came",
                "back. If they were... And",
                "I were to go out... Then...",
                "Maybe I better not leave."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "???",
            args![
                "If I could only give",
                "this pendant back to",
                "her, she wouldn't have",
                "to worry about me that",
                "much. But I might not",
                "be able to get back..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("???", args!["........"])?;
        ctx.next()?;
        ctx.lines_as(
            "?????",
            args![
                "Hey there, been",
                "waiting long? It's",
                "time for you to finish",
                "up your contract."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "???",
            args!["Finish up my...?", "N-no! I've decided!", "I'm not leaving this room!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "?????",
            args![
                "Hey hey, what the hell",
                "are you talking about?",
                "You came here to work,",
                "didn't you? And now it's",
                "time for you to collect, so..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "???",
            args![
                "N-no, l-let go of me!",
                "Please let me go!",
                "I want to see her again,",
                "please let me see her...!"
            ],
        )?;
        ctx.next()?;
        ctx.lines(args!["...", "......"])?;
        ctx.next()?;
        ctx.lines(args!["...", "......", "........."])?;
        ctx.next()?;
        ctx.mes("^3355FF*Clink*^000000")?;
        ctx.next()?;
        ctx.lines_as(
            "??",
            args![
                "Hm? Who the hell",
                "brought in this cheap",
                "jewery? Someone must",
                "have forgotten to throw",
                "away their trash..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "????",
            args![
                "Yeah, it's just a",
                "cheap trinket. That",
                "pendant isn't even",
                "worth picking up."
            ],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "^3131FFYou pick up an old",
            "pendant from the ground.",
            "No matter how hard you try,",
            "you can't open its clasp to see",
            "what this pendant contains.^000000"
        ])?;
        ctx.var("lhz_curse").set(Val::from(17))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(2090), Val::from(2091)])?;
        ctx.call(Function::GetItem, vec![Val::from(7341), Val::from(1)])?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("lhz_in01"), Val::from(278), Val::from(162)])?;
    } else {
        ctx.call(Function::Warp, vec![Val::from("lhz_in01"), Val::from(278), Val::from(162)])?;
    }
    return Err(Stop::End);
}

pub fn li_man(ctx: &Ctx) -> Script {
    li_man_body(ctx, Vec::new()).map(|_| ())
}

fn li_bird_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Peco Peco", args!["You're a fool!", "You're a fool!", "You're a fool!"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn li_bird(ctx: &Ctx) -> Script {
    li_bird_body(ctx, Vec::new()).map(|_| ())
}

fn li_bird_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Peco Peco", args!["You're a fool!", "You're a fool!", "You're a fool!"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn li_bird_ontouch(ctx: &Ctx) -> Script {
    li_bird_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn elder_lhz_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CountItem, vec![Val::from(7345)])?.number()? > 0 {
        if ((((ctx.var("lhz_curse").get()? == 1 && ctx.var("lhz_spi01").get()? == 1) && ctx.var("lhz_spi02").get()? == 1)
            && ctx.var("lhz_spi03").get()? == 1)
            && ctx.var("lhz_spi04").get()? == 1)
        {
            ctx.lines_as(
                "Elder",
                args![
                    "No wonder you look",
                    "so weary. Come, let",
                    "me help relieve you",
                    "of the burden that",
                    "you are carrying."
                ],
            )?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Um, burden...?:Crazy old woman!")])? {
                1 => {
                    ctx.lines_as(
                        "Elder",
                        args![
                            "Ah yes, they may not",
                            "be apparent to you, but",
                            "my eyes can clearly see",
                            "them. Yes. You're being",
                            "followed by those things."
                        ],
                    )?;
                    ctx.next()?;
                    let choice = runtime::select_values(ctx, &[Val::from("Things?")])?;
                    ctx.var("@menu").set(choice)?;
                    ctx.lines_as(
                        "Elder",
                        args![
                            "Yes. The best way I can",
                            "describe them is as evil",
                            "thoughts left in the world",
                            "when someone dies in",
                            "such a way that his grudge",
                            "survives to menace the living."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Elder",
                        args![
                            "Sometimes, these lingering",
                            "thoughts are created when",
                            "someone is broken hearted",
                            "or clings to this plane for the",
                            "sake of a loved one. Yes, those",
                            "thoughts are following you."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Elder",
                        args![
                            "I don't know why they",
                            "are following you, but I'm",
                            "certain they're there. Have",
                            "you been experiencing chills",
                            "down your spine, cold sweats,",
                            "maybe even hearing voices?"
                        ],
                    )?;
                    ctx.call(Function::SoundEffect, vec![Val::from("loli_ruri_stand.wav"), Val::from(0)])?;
                    ctx.call(
                        Function::StartStatus,
                        vec![ctx.constant("SC_CURSE")?, Val::from(5000), Val::from(0)],
                    )?;
                    ctx.next()?;
                    let choice = runtime::select_values(ctx, &[Val::from("Yes! How do I get rid of them?")])?;
                    ctx.var("@menu").set(choice)?;
                    ctx.lines_as(
                        "Elder",
                        args![
                            "Well, usually these lingering",
                            "thoughts have some sort of",
                            "physical anchor, an object",
                            "that has feelings attached",
                            "to it, something important",
                            "to its late owner."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Elder",
                        args![
                            "The rage they're directing",
                            "at you seems to be growing",
                            "and I can see the angry spirits",
                            "pulling at the hems of your",
                            "clothes. Tell me, have you",
                            "wronged anyone recently?!"
                        ],
                    )?;
                    ctx.next()?;
                    let choice = runtime::select_values(
                        ctx,
                        &[Val::from("I don't... think so.:I can't remember every bad thing I've done!")],
                    )?;
                    ctx.var("@menu").set(choice)?;
                    ctx.lines_as(
                        "Elder",
                        args![
                            "Well, you better do",
                            "something soon, before",
                            "the evil taints your mind",
                            "and drives you to insanity!",
                            "Now, I need to know for sure",
                            "if you've been hearing voices."
                        ],
                    )?;
                    ctx.next()?;
                    let choice = runtime::select_values(ctx, &[Val::from("Y-yes, I have.")])?;
                    ctx.var("@menu").set(choice)?;
                    ctx.lines_as(
                        "Elder",
                        args![
                            "There's still hope.",
                            "The spirits are trying",
                            "to reach you for now, but",
                            "if you wait too long, you may",
                            "become a victim of their wrath.",
                            "Hurry, there is much to do!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Elder",
                        args![
                            "You must appease these",
                            "forces by finding out what",
                            "happened to them in life.",
                            "Now, I don't possess great",
                            "power, but I can encourage",
                            "the spirits to guide you."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Elder",
                        args![
                            "I can only have the spirits",
                            "reveal the places they wish",
                            "for you to search only once.",
                            "You must remember the",
                            "locations that I am about",
                            "to show you. Get ready..."
                        ],
                    )?;
                    ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_SIGHT")?])?;
                    ctx.next()?;
                    ctx.lines_as("Elder", args!["Yaaaaappp ---!"])?;
                    ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_SIGHTRASHER")?])?;
                    ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_VOLCANO")?])?;
                    ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_MAPPILLAR")?])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Elder",
                        args![
                            "There! The locations",
                            "you must search should",
                            "be clear to you now! Don't",
                            "forget these placemarks!"
                        ],
                    )?;
                    ctx.call(
                        Function::ViewPoint,
                        vec![Val::from(1), Val::from(104), Val::from(282), Val::from(1), Val::from(10092339)],
                    )?;
                    ctx.call(
                        Function::ViewPoint,
                        vec![Val::from(1), Val::from(105), Val::from(282), Val::from(2), Val::from(10092339)],
                    )?;
                    ctx.call(
                        Function::ViewPoint,
                        vec![Val::from(1), Val::from(104), Val::from(281), Val::from(3), Val::from(10092339)],
                    )?;
                    ctx.call(
                        Function::ViewPoint,
                        vec![Val::from(1), Val::from(105), Val::from(281), Val::from(4), Val::from(10092339)],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Elder",
                        args![
                            "Good luck, youngster.",
                            "I hope you can appease",
                            "the wrath of these spirits...",
                            "But as long as you let them guide you, you ought to be safe."
                        ],
                    )?;
                    ctx.var("lhz_curse").set(Val::from(6))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(2086), Val::from(2087)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines_as(
                        "Elder",
                        args![
                            "Crazy...? Hm, you must",
                            "not quite understand what's",
                            "happening to you. Please do",
                            "not hesitate to come back to",
                            "me when you realize that you",
                            "need my help, youngster."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        } else if (ctx.var("lhz_curse").get()?.number()? > 5 && ctx.var("lhz_curse").get()?.number()? < 17) {
            ctx.lines_as(
                "Elder",
                args![
                    "I'm sorry, but there's",
                    "nothing more I can do for",
                    "you right now. But if you",
                    "find anything related to the",
                    "spirits that torment you,",
                    "please let me know."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("lhz_curse").get()? == 17 {
            if ctx.call(Function::CountItem, vec![Val::from(7341)])?.number()? > 0 {
                ctx.lines_as(
                    "Elder",
                    args![
                        "Greetings, adventurer.",
                        "How goes your search for",
                        "the remains of the spirits",
                        "that still cling to this plane?"
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Show him the Pendant.:Cancel")])? {
                    1 => {
                        ctx.lines_as(
                            "Elder",
                            args![
                                "Oh my... There are some",
                                "incredibly powerful emotions",
                                "clinging to this pendant. If we",
                                "don't do anything about this,",
                                "you'll be cursed very soon.",
                                "This is what you must do."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Elder",
                            args![
                                "Hurry and bring",
                                "^3131FF5 Holy Water^000000 and",
                                "^3131FF1 Bouquet^000000. The Holy Water",
                                "will purify this Pendant and",
                                "the Bouquet will comfort",
                                "the spirit of its owner."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Elder",
                            args![
                                "You don't have much",
                                "time, so return to me",
                                "as soon as possible!",
                                "It won't be long until",
                                "the spirits are consumed",
                                "by their supernatural rage..."
                            ],
                        )?;
                        ctx.var("lhz_curse").set(Val::from(18))?;
                        ctx.call(Function::ChangeQuest, vec![Val::from(2091), Val::from(2092)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            } else {
                ctx.lines_as(
                    "Elder",
                    args![
                        "Greetings, adventurer.",
                        "How goes your search for",
                        "the remains of the spirits",
                        "that still cling to this plane?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Elder",
                    args![
                        "Wait...",
                        "Why do I sense",
                        "that you've found",
                        "something, but have",
                        "not brought it with you?",
                        "You must retrace your steps!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else {
            if ctx.var("lhz_curse").get()? == 18 {
                if ((ctx.call(Function::CountItem, vec![Val::from(523)])?.number()? > 4
                    && ctx.call(Function::CountItem, vec![Val::from(744)])?.number()? > 0)
                    && ctx.call(Function::CountItem, vec![Val::from(7341)])?.number()? > 0)
                {
                    ctx.lines_as(
                        "Elder",
                        args![
                            "Good, good.",
                            "All is in readiness.",
                            "Please be silent as",
                            "I focus my spirit for the",
                            "great task before me."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Elder", args!["...", "......", ".........", "Hooooooo..."])?;
                    ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_BENEDICTIO")?])?;
                    ctx.next()?;
                    ctx.lines_as("Elder", args!["Yaaaaapp ---!"])?;
                    ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_ASPERSIO")?])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Elder",
                        args![
                            "Whew! I've managed",
                            "to nullify this curse for you.",
                            "That still doesn't change the",
                            "fact that what happened to this",
                            "pendant's owner was tragic..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Elder",
                        args![
                            "Why don't you bring",
                            "this pendant to the place",
                            "where it really belongs?",
                            "I'm sure that would bring",
                            "great comfort to its owner."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFSuddenly, the clasp on",
                        "the pendant pops open,",
                        "revealing a picture of a happy",
                        "couple. Somehow, the girl in",
                        "the picture, sitting uncomfortably^FFFFFF^3355FF in an old chair, looks familiar...^000000"
                    ])?;
                    ctx.var("lhz_curse").set(Val::from(19))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(2092), Val::from(2093)])?;
                    ctx.call(Function::DelItem, vec![Val::from(523), Val::from(5)])?;
                    ctx.call(Function::DelItem, vec![Val::from(744), Val::from(1)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Elder",
                        args![
                            "Hurry and bring",
                            "^3131FF5 Holy Water^000000 and",
                            "^3131FF1 Bouquet^000000 in order",
                            "for me to nullify this",
                            "curse. Do not forget to",
                            "bring the Pendant as well."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            } else {
                ctx.lines_as(
                    "Elder",
                    args![
                        "Predators are always",
                        "on the lookout for easy",
                        "prey. Be careful, youngster!",
                        "You look innocent enough",
                        "to become a victim in the city."
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("About Lighthalzen:About the Slum:????")])? {
                    1 => {
                        ctx.lines_as(
                            "Elder",
                            args![
                                "Lighthalzen might seem",
                                "like a splendid city at first,",
                                "but you'll quickly learn that",
                                "the poor are segregated from",
                                "the rich and treated as less",
                                "than second class citizens."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Elder",
                            args![
                                "At first, separation between",
                                "the rich and poor districts was",
                                "subtly enforced. They built the",
                                "railroad right between the two",
                                "districts to make it easier for",
                                "the rich to ignore the poor."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Elder",
                            args![
                                "But now they even have",
                                "guards to make sure that",
                                "the poor can't bother the",
                                "rich. I'm pretty sure that",
                                "this segregation won't be",
                                "ending anytime soon..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Elder",
                            args![
                                "Now, I've heard that the",
                                "Rekenber Corporation is",
                                "actually providing jobs for",
                                "people in the slums. Beggars",
                                "can't be choosers, so I'm sure",
                                "these jobs aren't that great."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as(
                            "Elder",
                            args![
                                "To live in the slum is to",
                                "be familiar with poverty,",
                                "disease, condemnation",
                                "and contempt. But we're all",
                                "still people, you know, so let",
                                "go of any of your misgivings."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Elder",
                            args![
                                "We're struggling just",
                                "to survive here. At the",
                                "very least, please respect",
                                "that. It's a fact that the",
                                "people in the rich district",
                                "seem to keep forgetting."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    3 => {
                        ctx.lines_as(
                            "Elder",
                            args![
                                "Well, if you need any",
                                "help around here or have",
                                "any questions, come back",
                                "and ask me. I get the feeling",
                                "that we'll probably meet again."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            }
        }
    } else {
        if ctx.var("lhz_curse").get()?.number()? < 1 {
            ctx.lines_as(
                "Elder",
                args![
                    "Predators are always",
                    "on the lookout for easy",
                    "prey. Be careful, youngster!",
                    "You look innocent enough",
                    "to become a victim in the city."
                ],
            )?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("About Lighthalzen:About the Slum:????")])? {
                1 => {
                    ctx.lines_as(
                        "Elder",
                        args![
                            "Lighthalzen might seem",
                            "like a splendid city at first,",
                            "but you'll quickly learn that",
                            "the poor are segregated from",
                            "the rich and treated as less",
                            "than second class citizens."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Elder",
                        args![
                            "At first, separation between",
                            "the rich and poor districts was",
                            "subtly enforced. They built the",
                            "railroad right between the two",
                            "districts to make it easier for",
                            "the rich to ignore the poor."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Elder",
                        args![
                            "But now they even have",
                            "guards to make sure that",
                            "the poor can't bother the",
                            "rich. I'm pretty sure that",
                            "this segregation won't be",
                            "ending anytime soon..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Elder",
                        args![
                            "Now, I've heard that the",
                            "Rekenber Corporation is",
                            "actually providing jobs for",
                            "people in the slums. Beggars",
                            "can't be choosers, so I'm sure",
                            "these jobs aren't that great."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines_as(
                        "Elder",
                        args![
                            "To live in the slum is to",
                            "be familiar with poverty,",
                            "disease, condemnation",
                            "and contempt. But we're all",
                            "still people, you know, so let",
                            "go of any of your misgivings."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Elder",
                        args![
                            "We're struggling just",
                            "to survive here. At the",
                            "very least, please respect",
                            "that. It's a fact that the",
                            "people in the rich district",
                            "seem to keep forgetting."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                3 => {
                    ctx.lines_as(
                        "Elder",
                        args![
                            "Well, if you need any",
                            "help around here or have",
                            "any questions, come back",
                            "and ask me. I get the feeling",
                            "that we'll probably meet again."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        } else {
            ctx.lines_as(
                "Elder",
                args![
                    "Predators are always",
                    "on the lookout for easy",
                    "prey. Be careful, youngster!",
                    "You look innocent enough",
                    "to become a victim in the city."
                ],
            )?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("About Lighthalzen:About the Slum:????")])? {
                1 => {
                    ctx.lines_as(
                        "Elder",
                        args![
                            "Lighthalzen might seem",
                            "like a splendid city at first,",
                            "but you'll quickly learn that",
                            "the poor are segregated from",
                            "the rich and treated as less",
                            "than second class citizens."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Elder",
                        args![
                            "At first, separation between",
                            "the rich and poor districts was",
                            "subtly enforced. They built the",
                            "railroad right between the two",
                            "districts to make it easier for",
                            "the rich to ignore the poor."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Elder",
                        args![
                            "But now they even have",
                            "guards to make sure that",
                            "the poor can't bother the",
                            "rich. I'm pretty sure that",
                            "this segregation won't be",
                            "ending anytime soon..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Elder",
                        args![
                            "Now, I've heard that the",
                            "Rekenber Corporation is",
                            "actually providing jobs for",
                            "people in the slums. Beggars",
                            "can't be choosers, so I'm sure",
                            "these jobs aren't that great."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines_as(
                        "Elder",
                        args![
                            "To live in the slum is to",
                            "be familiar with poverty,",
                            "disease, condemnation",
                            "and contempt. But we're all",
                            "still people, you know, so let",
                            "go of any of your misgivings."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Elder",
                        args![
                            "We're struggling just",
                            "to survive here. At the",
                            "very least, please respect",
                            "that. It's a fact that the",
                            "people in the rich district",
                            "seem to keep forgetting."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                3 => {
                    ctx.lines_as(
                        "Elder",
                        args![
                            "Well, if you need any",
                            "help around here or have",
                            "any questions, come back",
                            "and ask me. I get the feeling",
                            "that we'll probably meet again."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        }
    }
    Ok(Val::from(0))
}

pub fn elder_lhz(ctx: &Ctx) -> Script {
    elder_lhz_body(ctx, Vec::new()).map(|_| ())
}

fn crippled_girl_li_tre_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("lhz_curse").get()? == 23 {
        if ctx.call(Function::CountItem, vec![Val::from(7341)])?.number()? > 0 {
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Hey there, are", "you feeling alright?"],
            )?;
            ctx.next()?;
            ctx.lines_as("Crippled Girl", args!["Oh, thanks,", "I'm fine. But...", "Have we met before?"])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "Yeah, we did.",
                    "You should know who",
                    "I am by now. Hey, you",
                    "didn't forget, did you?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Lady",
                args![
                    "Oh, how do I put this?",
                    "The fever she had for the",
                    "last few days. She's gotten",
                    "better, but she's forgotten",
                    "everything that's happened",
                    "in the past few weeks..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "Oh... Oh. I'm so",
                    "sorry. But maybe this",
                    "is for the best? Here,",
                    "I think you should keep",
                    "this pendant, though."
                ],
            )?;
            ctx.next()?;
            ctx.lines(args!["^3131ffYou place the", "old pendant", "into her hands.^000000"])?;
            ctx.next()?;
            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_FORESTLIGHT4")?])?;
            ctx.mes("...............")?;
            ctx.next()?;
            ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["...", "......"])?;
            ctx.next()?;
            ctx.lines(args![
                ((Val::from("[Possessed ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("]")),
                "^3131FFSetsu, please remember",
                "that I'd never do anything",
                "to hurt you. Forgive me for",
                "leaving you behind. I hope",
                "that one day we'll meet",
                "again, little princess.^000000"
            ])?;
            ctx.next()?;
            ctx.lines(args![
                ((Val::from("[Possessed ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("]")),
                "^3131FFUntil then, I want",
                "you to be happy, okay?",
                "Your brother's always",
                "gonna be looking out for",
                "you, one way or another...^000000"
            ])?;
            ctx.next()?;
            ctx.lines_as("Setsu", args!["My brother's...", "That's my brother's", "voice! Brother!"])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Eh? What? Whoa...", "What came over me?", "The last thing I reme--"],
            )?;
            ctx.next()?;
            ctx.lines_as("Setsu", args!["...", "......"])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Well...", "This is awkward."],
            )?;
            ctx.call(Function::DelItem, vec![Val::from(7341), Val::from(1)])?;
            ctx.var("lhz_curse").set(Val::from(24))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Hey there, are", "you feeling alright?"],
            )?;
            ctx.next()?;
            ctx.lines_as("Crippled Girl", args!["Oh, thanks,", "I'm fine. But...", "Have we met before?"])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "Yeah, we did.",
                    "You should know who",
                    "I am by now. Hey, you",
                    "didn't forget, did you?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Lady",
                args![
                    "Oh, how do I put this?",
                    "The fever she had for the",
                    "last few days. She's gotten",
                    "better, but she's forgotten",
                    "everything that's happened",
                    "in the past few weeks..."
                ],
            )?;
            ctx.next()?;
            ctx.lines(args!["^3131ffThis would be a good", "opportunity to give her the...^000000"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        if ctx.var("lhz_curse").get()? == 24 {
            ctx.lines_as("Setsu", args!["...", "......"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("lhz_curse").get()? == 25 {
            ctx.lines_as(
                "Setsu",
                args!["Excuse me, I can't still", "walk but I don't cry", "anymore. I'm doing my best!"],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as("Crippled Girl", args!["...", "......", "*Sigh...*"])?;
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("Hello, how are you?")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as(
                "Crippled Girl",
                args![
                    "Oh, I'm fine,",
                    "thanks for asking.",
                    "I'm just waiting for",
                    "somebody, that's all."
                ],
            )?;
            if ctx.var("lhz_curse").get()? == 19 {
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Show her the Pendant.:Okay, have a good day.")])? {
                    1 => {}
                    2 => {
                        ctx.lines_as("Crippled Girl", args!["Okay,", "bye-bye..."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["Um, would you have", "any idea who might", "have owned this pendant?"],
                )?;
                ctx.next()?;
                if ctx.call(Function::CountItem, vec![Val::from(7341)])?.number()? < 1 {
                    ctx.lines_as("Crippled Girl", args!["What are you talking about?"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
                ctx.lines_as(
                    "Crippled Girl",
                    args![
                        "Oh, that's mine!",
                        "I gave it to my big brother",
                        "before he went away on some",
                        "sort of business trip. You must",
                        "be his friend, is that right?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["Actually, um,", "you know what...?"],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Brutal Truth:Break it to her gently")])? {
                    1 => {
                        ctx.lines(args!["...", "......"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![
                                "...So basically, your",
                                "brother's @lhz_ghost, oh right,",
                                "did I mention he was dead?",
                                "Anyway, so he's not alive",
                                "anymore, but his spirit or",
                                "whatever is still around and--"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Crippled Girl", args!["...", "......"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Crippled Girl",
                            args![
                                "...Dead?",
                                "No! I don't believe you!",
                                "He can't die! He was the",
                                "bravest and the sweetest",
                                "and the-- Leave me alone!",
                                "Oh god, get the hell away!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["...", "......", "Sheesh.", "Don't kill the", "messenger."],
                        )?;
                        ctx.var("lhz_curse").set(Val::from(20))?;
                        ctx.close_window()?;
                        ctx.call(Function::Warp, vec![Val::from("lighthalzen"), Val::from(322), Val::from(323)])?;
                        return Err(Stop::End);
                    }
                    2 => {
                        if ctx.call(Function::CountItem, vec![Val::from(7341)])?.number()? > 0 {
                            ctx.lines_as("Crippled Girl", args!["Did you meet my", "big brother...?"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Crippled Girl",
                                args![
                                    "My brother is a strong",
                                    "person, and I should be",
                                    "happy of that, because",
                                    "nothing could happen",
                                    "to him."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Um... yes. So..."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "Sure, your big brother must be happy.",
                                    "He'll come back sometime.",
                                    "You should sleep to be healthy",
                                    "once your brother returns."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Crippled Girl",
                                args!["Yes! I must be okay", "and be able to walk when my", "brother come."],
                            )?;
                            ctx.next()?;
                            ctx.lines(args!["^3131ffYou place the", "old pendant", "into her hands.^000000"])?;
                            ctx.next()?;
                            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_FORESTLIGHT4")?])?;
                            ctx.mes("......")?;
                            ctx.next()?;
                            ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["..............."])?;
                            ctx.next()?;
                            ctx.lines(args![
                                ((Val::from("[Possessed ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("]")),
                                "^3131FFSetsu, please remember",
                                "that I'd never do anything",
                                "to hurt you. Forgive me for",
                                "leaving you behind. I hope",
                                "that one day we'll meet",
                                "again, little princess.^000000"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                ((Val::from("[Possessed ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("]")),
                                "^3131FFUntil then, I want",
                                "you to be happy, okay?",
                                "Your brother's always",
                                "gonna be looking out for",
                                "you, one way or another...^000000"
                            ])?;
                            ctx.next()?;
                            ctx.lines_as("Setsu", args!["My brother's...", "That's my brother's", "voice! Brother!"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["Eh? What? Whoa...", "What came over me?", "The last thing I reme--"],
                            )?;
                            ctx.next()?;
                            ctx.lines(args!["^FF0000Tears fall from the", "girl's face, then you...^000000"])?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("Distract her:Comfort her")])? {
                                1 => {
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args![
                                            "Oh, all of this was a play...",
                                            "Y-yes a play, I was practising.",
                                            "Ha ha~ Is it okay, isn't it?"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Setsu",
                                        args![
                                            "Oh, you surprised me!",
                                            "Even though I didn't think",
                                            "it was really my brother, you know?"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Excuse me..."])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Setsu",
                                        args![
                                            "I'll do my best!",
                                            "I must be okay when my brother return.",
                                            "He'll be very proud of me.",
                                            "Heh heh~"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args!["Yes, I'm sure of that."],
                                    )?;
                                    ctx.call(Function::DelItem, vec![Val::from(7341), Val::from(1)])?;
                                    ctx.var("lhz_curse").set(Val::from(25))?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                2 => {
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args![
                                            "Crying have no use!",
                                            "You're a very pretty",
                                            "girl to be ruining your",
                                            "face with useless tears."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Setsu", args!["............"])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args!["You should do your best", "to be healthy for the time", "your big brother return."],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Setsu", args!["Yes..."])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args!["Listen, when your brother", "return you'll celebrate in", "a beautiful place."],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Setsu", args!["Really?", "It's a promise then! Heh heh~"])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args![
                                            "I hope it'll be soon!",
                                            "When I return I want you to be",
                                            "able to walk. See you soon, then!"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Setsu", args!["I'll try hard!"])?;
                                    ctx.call(Function::DelItem, vec![Val::from(7341), Val::from(1)])?;
                                    ctx.var("lhz_curse").set(Val::from(25))?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                _ => {}
                            }
                        } else {
                            ctx.lines_as("Crippled Girl", args!["Did you meet my", "big brother...?"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Crippled Girl",
                                args![
                                    "My brother is a strong",
                                    "person, and I should be",
                                    "happy of that, because",
                                    "nothing could happen",
                                    "to him."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Um... yes. So..."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "Sure, your big brother must be happy.",
                                    "He'll come back sometime.",
                                    "You should sleep to be healthy",
                                    "once your brother returns."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Crippled Girl",
                                args!["Yes! I must be okay", "and be able to walk when my", "brother come."],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                    _ => {}
                }
            }
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
}

pub fn crippled_girl_li_tre(ctx: &Ctx) -> Script {
    crippled_girl_li_tre_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum LiDoorStep {
    Start,
    OnTouch,
}

fn li_door_run(ctx: &Ctx, mut step: LiDoorStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            LiDoorStep::Start => {
                step = LiDoorStep::OnTouch;
                continue 'machine;
            }
            LiDoorStep::OnTouch => {
                if (ctx.var("lhz_curse").get()?.number()? > 19 && ctx.var("lhz_curse").get()?.number()? < 23) {
                    ctx.mes("^3355FFThe door is locked.^000000")?;
                    ctx.var("lhz_curse").set((ctx.var("lhz_curse").get()? + Val::from(1)))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("lhz_curse").get()?.number()? > 23 {
                    ctx.mes("^3355FFThe door is locked.^000000")?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Citizen",
                        args![
                            "I'm sorry, but another",
                            "epidemic is starting to",
                            "spread around the slums.",
                            "We're not going outside and we're keeping our children safe!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Citizen",
                        args![
                            "Not to be unfriendly,",
                            "but you should be careful",
                            "too. The living conditions",
                            "of this area aren't exactly",
                            "sanitary, you know?"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.call(Function::Warp, vec![Val::from("lhz_in03"), Val::from(15), Val::from(162)])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn li_door(ctx: &Ctx) -> Script {
    li_door_run(ctx, LiDoorStep::Start, Vec::new()).map(|_| ())
}

pub fn li_door_ontouch(ctx: &Ctx) -> Script {
    li_door_run(ctx, LiDoorStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum LiBotherStep {
    Start,
    OnTouch,
}

fn li_bother_run(ctx: &Ctx, mut step: LiBotherStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            LiBotherStep::Start => {
                step = LiBotherStep::OnTouch;
                continue 'machine;
            }
            LiBotherStep::OnTouch => {
                if (ctx.var("lhz_curse").get()? == 24 || ctx.var("lhz_curse").get()? == 25) {
                    ctx.mes(".............")?;
                    ctx.next()?;
                    ctx.mes("*Shhhzzzzzzz!*")?;
                    ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_MAPPILLAR")?])?;
                    ctx.next()?;
                    ctx.lines_as("????", args!["..........."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "????",
                        args!["^FF0000...I'm sorry...", "......I appreciate", "that you.............^000000"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "H-huh?!",
                            "What was that?",
                            "That can't be the",
                            "wind, I must be",
                            "hearing things again..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Wait. I thought I was",
                            "rid of those thoughts",
                            "or spirits, whatever was",
                            "haunting me before. Maybe",
                            "they still want me to do",
                            "something for them. Hmmm..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "I get the feeling that",
                            "all of their suffering",
                            "is tied to the ^FF0000Rekenber",
                            "Corporation^000000 and that",
                            "^FF0000Regenschirm Laboratory^000000."
                        ],
                    )?;
                    ctx.var("lhz_curse").set(Val::from(26))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(2093), Val::from(2094)])?;
                    {
                        if ctx.var("BaseLevel").get()?.number()? < 70 {
                            ctx.call(Function::GetExperience, vec![Val::from(800000), Val::from(300000)])?;
                        } else if (ctx.var("BaseLevel").get()?.number()? > 69 && ctx.var("BaseLevel").get()?.number()? < 80) {
                            ctx.call(Function::GetExperience, vec![Val::from(1000000), Val::from(500000)])?;
                        } else if (ctx.var("BaseLevel").get()?.number()? > 79 && ctx.var("BaseLevel").get()?.number()? < 90) {
                            ctx.call(Function::GetExperience, vec![Val::from(1500000), Val::from(800000)])?;
                        } else {
                            ctx.call(Function::GetExperience, vec![Val::from(2000000), Val::from(1000000)])?;
                        }
                    }
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn li_bother(ctx: &Ctx) -> Script {
    li_bother_run(ctx, LiBotherStep::Start, Vec::new()).map(|_| ())
}

pub fn li_bother_ontouch(ctx: &Ctx) -> Script {
    li_bother_run(ctx, LiBotherStep::OnTouch, Vec::new()).map(|_| ())
}

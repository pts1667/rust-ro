use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn botanist_ep13_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("ep13_ryu").get()?.number()? < 100 && ctx.var("ep13_start").get()?.number()? < 100) {
        ctx.lines_as(
            "Botanist",
            args!["This new land!", "Undiscovered life!", "Everything about this world excites me."],
        )?;
        ctx.next()?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_BIGTHROB")?])?;
        ctx.lines_as("Botanist", args!["Oh, okay... Ah-hah!", "This is how it goes..."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("ep13_animal").get()?.number()? < 4 {
        ctx.lines_as(
            "Botanist",
            args!["This new land!", "Undiscovered life!", "Everything about this world excites me."],
        )?;
        ctx.next()?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_BIGTHROB")?])?;
        ctx.lines_as("Botanist", args!["Oh, okay... Ah-hah!", "This is how it goes..."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("ep13_animal").get()? == 4 {
        ctx.lines(args![
            "This new land!",
            "Undiscovered life!",
            "Everything about this world excites me."
        ])?;
        ctx.next()?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_BIGTHROB")?])?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_THINK")?,
                Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
            ],
        )?;
        ctx.lines_as("Botanist", args!["Oh, okay... Ah-hah!", "This is how it goes..."])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Express your displeasure.:Agree with him.")])? {
            1 => {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["Your excitement agitates me."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Botanist",
                    args![
                        "Err? Ahahahaha!",
                        "Oh, come on! I'm just being happy.",
                        "By the way, is there anything I can help you with?"
                    ],
                )?;
                ctx.next()?;
            }
            2 => {
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_BIGTHROB")?,
                        Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                    ],
                )?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["I agree. There's so much to see around here!"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Botanist",
                    args![
                        "Ahahaha!",
                        "Nice to meet you, my friend.",
                        "So, is there anything I can help you with?"
                    ],
                )?;
                ctx.next()?;
            }
            _ => {}
        }
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Your brother wanted me to send this to you. Maybe it'll help with your study."],
        )?;
        ctx.next()?;
        ctx.mes("- You have given the Nepenthes Specimen to the botanist. -")?;
        ctx.next()?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_STARE")?])?;
        ctx.lines_as("Botanist", args!["Oh...? Isn't this?!"])?;
        ctx.next()?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_HUK")?,
                Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
            ],
        )?;
        ctx.lines_as("Botanist", args!["...Well, I've got a ton of samples of this specimen. Sorry."])?;
        ctx.next()?;
        ctx.mes("- He points at a pile of specimens in the tent. -")?;
        ctx.next()?;
        ctx.lines_as("Botanist", args!["Still, I'm impressed. I always thought my brother could never do anything on his own. But look, he was able to collect a Nepenthes specimen."])?;
        ctx.next()?;
        ctx.lines_as(
            "Botanist",
            args![
                "Then again, I had this strong feeling that someone has helped him.",
                "That someone was probably you, right?~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Botanist",
            args!["Either way, I don't care.", "I'm just happy that he cares about me..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Why do you think that?", "Can't you just thank him for trying to help you?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Botanist",
            args![
                "Well, you must forget that he and I are twins.",
                "We are spiritually connected to each other."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Botanist",
            args!["Usually twins share the same feelings and ideas, I can see what he does or thinks if I try hard."],
        )?;
        ctx.next()?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_OHNO")?])?;
        ctx.lines_as("Botanist", args!["Let me see what Rumis is doing right now... Oh, he's picking his nose with his left hand and eating a slice of bread with his right.", "God, how disgusting!"])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Go check Rumis if it's true.:Do not trust his word.")])? {
            1 => {
                ctx.call(Function::Emotion, vec![ctx.constant("ET_SMILE")?])?;
                ctx.lines_as(
                    "Botanist",
                    args![
                        "Hahaha!",
                        "Muhahaha!",
                        "Let me guess, you want to go check Rumis to see if I'm telling the truth. Am I wrong?",
                        "Hahahaha! Sorry, but I lied. I mean, remote vision? Because we're twins?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Botanist",
                    args![
                        "God, you're so funny. Hahaha~",
                        "You please me, just like the new creatures blooming on the World Tree.~"
                    ],
                )?;
                ctx.next()?;
            }
            2 => {
                ctx.call(Function::Emotion, vec![ctx.constant("ET_OHNO")?])?;
                ctx.lines_as(
                    "Botanist",
                    args![
                        "Oh,",
                        "won't you cut me some slack? I was just joking to melt the ice.",
                        "This world is too barren and tough to live without good humor and jokes, you know?"
                    ],
                )?;
                ctx.next()?;
            }
            _ => {}
        }
        ctx.call(Function::Emotion, vec![ctx.constant("ET_HNG")?])?;
        ctx.lines_as(
            "Botanist",
            args![
                "Anyways, as I said, I've got a lot of Nepenthes specimens.",
                "But I'll gratefully take this pretentious gift of his."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Botanist",
            args!["This may be an unexpected question, but... Are you interested in meeting dangerous girls playing on a field?"],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Dangerous girls?")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as("Botanist", args!["Yes, femme fatales!", "I've seen girls that are so beautiful.", "They're too dangerous to get close, but that's what make them so irresistibly attractive. Do you understand what I'm saying?"])?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("It's... Hard to say.")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Botanist",
            args![
                "Have you explored the area over the bridge at the right side of the united expedition camp?",
                "Then you know the area is so cold that it'll instantly freeze your heart."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Botanist",
            args![
                "You should go across the leftward bridge, you'll be surprised to see what's ahead of you.",
                "It's yet to be known how such a thing can happen."
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("What are you talking about?")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Botanist",
            args![
                "Well...",
                "I suppose I don't have the words to really describe such an amazing scene."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Botanist",
            args!["I'll send a message to the garrison for you. Why don't you go see it with your own eyes? You won't be disappointed!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Botanist",
            args![
                "Of course, I'm offering such an extraordinary service because there's something I want in return. Ahahaha!",
                "...Truth be told, I'm having some trouble. And I need someone to help me."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Botanist", args!["For now, you should go across the leftward bridge. Speak to a guard over there, and then just go ahead.", "A group of beautiful girls will welcome you. I hope you'll enjoy having a conversation with them. Maybe you'll get to understand the mystery of nature."])?;
        ctx.next()?;
        ctx.lines_as("Botanist", args!["Yes, I only need... ^3131FF30^000000 of them."])?;
        ctx.var("ep13_animal").set(Val::from(5))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(2149), Val::from(2150)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("ep13_animal").get()? == 5 {
        if ctx.call(Function::CheckQuest, vec![Val::from(2150), ctx.constant("HUNTING")?])? == 2 {
            ctx.lines_as("Botanist", args!["Welcome back! How was it?", "It was amazing, wasn't it?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Botanist",
                args![
                    "On the right side of this united expedition camp,",
                    "we have a land surrounded by roaring blizzards."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Botanist",
                args![
                    "And on the other side,",
                    "we have a peaceful green land",
                    "where we can enjoy Mother Nature's warm embrace."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Botanist",
                args![
                    "Two completely different worlds coexist",
                    "within a short distance of each other.",
                    "Where else can you see such amazing contrasts at a glance?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Botanist", args!["I'm curious to know all secrets about this world."])?;
            ctx.next()?;
            ctx.lines_as(
                "Botanist",
                args!["I'm so excited that I can't hide it!", "*Pant Pant*", "*Pant Pant*"],
            )?;
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("What a mysterious world this is!")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as(
                "Botanist",
                args!["Yes, it is...", "Oh, right!", "Actually I'm here to study this strange phenomenon."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Botanist",
                args![
                    "To understand the environmental conditions,",
                    "including temperature and humidity,",
                    "5.....and check the families and growth of plants,",
                    "I've installed several special environmental meters",
                    "in this area."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Botanist",
                args![
                    "Unfortunately, some evil monsters",
                    "keep destroying the precious meters",
                    "with their ^3131FFHeinous Hoops^000000!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Botanist",
                args![
                    "The culprits are called Cornus.",
                    "I'd like to ask you to teach them not to destroy my devices again."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Botanist",
                args!["How'd you like to hunt ^3131FF10 of them^000000 to set an example? Good luck!"],
            )?;
            ctx.var("ep13_animal").set(Val::from(6))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(2150), Val::from(2151)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as("Botanist", args!["For now, you should go across the leftward bridge. Speak to a guard over there, and then just go ahead.", "A group of beautiful girls will welcome you. I hope you'll enjoy having a conversation with them. Maybe you'll get to understand the mystery of nature."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        if ctx.var("ep13_animal").get()? == 6 {
            if ctx.call(Function::CheckQuest, vec![Val::from(2151), ctx.constant("HUNTING")?])? == 2 {
                ctx.lines_as(
                    "Botanist",
                    args!["Hopefully the Cornuses have learned their lesson, and won't touch my special environmental meters anymore."],
                )?;
                ctx.next()?;
                ctx.lines_as("Botanist", args!["Oh Karyl, while you were away, I received an interesting report from one of my research machines that remained intact."])?;
                ctx.next()?;
                ctx.lines_as("Botanist", args!["According to the report, The original weather condition of Ash Vacuum is cold and dry, just like the land on the right side."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Botanist",
                    args!["That means, somebody has artificially cultivated the leftward land."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Botanist",
                    args![
                        "Do you remember seeing strange mushroom shaped buildings everywhere in the fields?",
                        "I suspect it to be an environmental purifier that converts barren land into lush, green fields."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Botanist",
                    args![
                        "Let me show you.",
                        "Please take this rotting plant stem,",
                        "I'll meet you at the environmental purifier standing right next to the leftward bridge on the other side."
                    ],
                )?;
                ctx.var("ep13_animal").set(Val::from(7))?;
                ctx.call(Function::GetItem, vec![Val::from(6035), Val::from(1)])?;
                ctx.call(Function::ChangeQuest, vec![Val::from(2151), Val::from(2152)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Botanist",
                    args![
                        "Cornuses are breaking my special environmental meters!",
                        "Please show them the power of Midgardians!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else {
            if (ctx.var("ep13_animal").get()?.number()? > 6 && ctx.var("ep13_animal").get()?.number()? < 10) {
                ctx.lines_as(
                    "Botanist",
                    args!["I'll meet you at the environmental purifier standing right next to the leftward bridge on the other side."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Botanist",
                    args![
                        "I submitted a report to upper management.",
                        "I haven't received any response, but I'm sure they'll be as excited as I am once they read the report. Woohoo!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn botanist_ep13(ctx: &Ctx) -> Script {
    botanist_ep13_body(ctx, Vec::new()).map(|_| ())
}

fn camp_guard_man1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Camp Guard",
        args![
            "Stop!",
            "You're about to enter an area that has not been fully explored.",
            "Only personnel--researchers and explorers--authorized by United Midgard and the garrison are allowed to enter the danger zone."
        ],
    )?;
    if (ctx.var("ep13_ryu").get()?.number()? < 100 && ctx.var("ep13_start").get()?.number()? < 100) {
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("I want to enter the next area.:I want to stay.")])? {
        1 => {
            ctx.lines_as(
                "Camp Guard",
                args![
                    "Please be careful out there.",
                    "If you encounter any threats or strange phenomenon, then please don't hesitate to report to us at the garrison."
                ],
            )?;
            ctx.close_window()?;
            let subject2 = runtime::atoi(&runtime::charat(
                &ctx.call(Function::StrNpcInfo, vec![Val::from(2)])?,
                &Val::from(3),
            )?);
            if subject2 == 1 {
                if ctx.var("ep13_animal").get()? == 1 {
                    ctx.var("ep13_animal").set(Val::from(2))?;
                }
                ctx.call(Function::Warp, vec![Val::from("man_fild01"), Val::from(36), Val::from(235)])?;
            } else if subject2 == 2 {
                ctx.call(Function::Warp, vec![Val::from("spl_fild02"), Val::from(379), Val::from(143)])?;
            } else if subject2 == 3 {
                ctx.call(Function::Warp, vec![Val::from("spl_fild02"), Val::from(380), Val::from(217)])?;
            }
            return Err(Stop::End);
        }
        2 => {
            ctx.lines_as("Camp Guard", args!["No, you can't. Please return to the expedition camp."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn camp_guard_man1(ctx: &Ctx) -> Script {
    camp_guard_man1_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum EnvClearStep {
    Start,
    OnTouch,
}

fn env_clear_run(ctx: &Ctx, mut step: EnvClearStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            EnvClearStep::Start => {
                step = EnvClearStep::OnTouch;
                continue 'machine;
            }
            EnvClearStep::OnTouch => {
                if ctx.var("ep13_animal").get()? == 7 {
                    ctx.call(Function::EnableNpc, vec![Val::from("Botanist#ep13_1")])?;
                    ctx.lines_as("Botanist", args!["This is it."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Botanist",
                        args![
                            "Look.",
                            "You can tell the pileus is creating drops of light which drip to the ground."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Botanist", args!["Please try to plant the rotting stem in the ground."])?;
                    ctx.var("ep13_animal").set(Val::from(8))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("ep13_animal").get()? == 8 {
                    ctx.call(Function::EnableNpc, vec![Val::from("Botanist#ep13_1")])?;
                    ctx.lines_as("Botanist", args!["Please try to plant the rotting stem in the ground."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("ep13_animal").get()? == 9 {
                    ctx.call(Function::EnableNpc, vec![Val::from("Botanist#ep13_1")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn env_clear(ctx: &Ctx) -> Script {
    env_clear_run(ctx, EnvClearStep::Start, Vec::new()).map(|_| ())
}

pub fn env_clear_ontouch(ctx: &Ctx) -> Script {
    env_clear_run(ctx, EnvClearStep::OnTouch, Vec::new()).map(|_| ())
}

fn botanist_ep13_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ep13_animal").get()? == 8 {
        ctx.lines_as("Botanist", args!["Please try to plant the rotting stem in the ground."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("ep13_animal").get()? == 9 {
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["It's sprouted! That shoot looks healthy!"],
        )?;
        ctx.next()?;
        ctx.lines_as("Botanist", args!["See?", "This giant mushroom, otherwise known as an Environmental Purifier, not only brings dying plants back to life, but also stimulates their growth."])?;
        ctx.next()?;
        ctx.lines_as(
            "Botanist",
            args!["Isn't it amazing?!", "This finding will surely stir up the academic world."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Botanist",
            args![
                "Now, I should get back to work. I need to write a report based on my study results.",
                "I can't wait to see how the expedition management will react to my report. Hahaha~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Botanist",
            args![
                "Oh, right... Rumis!",
                "Can you please tell him that I've received his Nepenthes specimen safely?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Botanist",
            args!["And don't forget to tell him that I've produced great research results. That'll make him cry like a baby. Hahahaha!"],
        )?;
        ctx.var("ep13_animal").set(Val::from(10))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(2152), Val::from(2153)])?;
        ctx.call(Function::DisableNpc, vec![Val::from("Botanist#ep13_1")])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    return Err(Stop::End);
}

pub fn botanist_ep13_1(ctx: &Ctx) -> Script {
    botanist_ep13_1_body(ctx, Vec::new()).map(|_| ())
}

fn botanist_ep13_1_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Botanist#ep13_1")])?;
    return Err(Stop::End);
}

pub fn botanist_ep13_1_oninit(ctx: &Ctx) -> Script {
    botanist_ep13_1_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn dirt_ep13_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ep13_animal").get()? == 8 {
        ctx.mes("- You dug a hole into the ground, planted the rotting stem, and then watered it. -")?;
        ctx.next()?;
        ctx.lines(args!["... ... ... ...", "... ... ... ..."])?;
        ctx.next()?;
        ctx.lines(args!["... ... ... ...", "... ... ... ...", "... ... ... ..."])?;
        ctx.next()?;
        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_LIGHTSPHERE")?])?;
        ctx.lines(args!["- Pzzzz -", "- Ssshuhhhh -"])?;
        ctx.next()?;
        ctx.mes("- Something is growing out of the dirt. -")?;
        ctx.var("ep13_animal").set(Val::from(9))?;
        ctx.call(Function::DelItem, vec![Val::from(6035), Val::from(1)])?;
        ctx.call(Function::GetItem, vec![Val::from(7193), Val::from(1)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    return Err(Stop::End);
}

pub fn dirt_ep13(ctx: &Ctx) -> Script {
    dirt_ep13_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum EvtLumisStep {
    Start,
    OnTouch,
}

fn evt_lumis_run(ctx: &Ctx, mut step: EvtLumisStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            EvtLumisStep::Start => {
                step = EvtLumisStep::OnTouch;
                continue 'machine;
            }
            EvtLumisStep::OnTouch => {
                if ctx.var("ep13_animal").get()? == 11 {
                    ctx.lines_as(
                        "Rumis Block",
                        args!["This is it! Come to the small tree in the southeast direction!"],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn evt_lumis(ctx: &Ctx) -> Script {
    evt_lumis_run(ctx, EvtLumisStep::Start, Vec::new()).map(|_| ())
}

pub fn evt_lumis_ontouch(ctx: &Ctx) -> Script {
    evt_lumis_run(ctx, EvtLumisStep::OnTouch, Vec::new()).map(|_| ())
}

fn frozen_tree_evt_lumis_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ep13_animal").get()? == 11 {
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Knock, knock. Are you here?"],
        )?;
        ctx.next()?;
        ctx.mes("- Something is making loud noise near the top of the tree. -")?;
        ctx.next()?;
        ctx.lines_as(
            "Rumis Block",
            args![
                "...Oh, yes. Yes!",
                "Monsters were looking at me, so I climbed up this tree without even thinking!",
                "How can I get down now?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Rumis Block",
            args![
                "Oh wait, I can see everything so clearly from here.",
                "I guess staying here will be much safer and easier for studying monsters."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Rumis Block",
            args![
                "...I'm looking at Hillsrion, the creature that people are talking about.",
                "It's covered with soft fur that reminds me of soft ice flakes floating in the air.",
                "...Oh, sorry.. I was just talking to myself."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Rumis Block",
            args![
                ((Val::from("I'll be watching the Hillsrions from this tree. ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                    + Val::from(", can you please go collect ^3131FFHillsrion Horns^000000 for me?"))
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Rumis Block",
            args![
                "Their horns might possess a special power. And I want to have them for further research.",
                "...5 horns will be enough."
            ],
        )?;
        ctx.var("ep13_animal").set(Val::from(12))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(2154), Val::from(2155)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("ep13_animal").get()? == 12 {
        if ctx.call(Function::CountItem, vec![Val::from(6032)])?.number()? < 5 {
            ctx.lines_as(
                "Rumis Block",
                args!["I'd like to study Hillsrion's Horns. Please bring 5 of them for me, okay."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as("Rumis Block", args!["...*Shiver*..."])?;
            ctx.next()?;
            ctx.lines_as("Rumis Block", args!["...*Shiver*...", "...*Shiver*..."])?;
            ctx.next()?;
            ctx.lines_as("Rumis Block", args!["...*Shiver*...", "...*Shiver*...", "...*Shiver*..."])?;
            ctx.next()?;
            ctx.lines_as("Rumis Block", args!["Ah! You're back!", "Phew..."])?;
            ctx.next()?;
            ctx.lines_as(
                "Rumis Block",
                args![
                    "Are you okay? Their fangs look so sharp and shiny...",
                    "Oh right, you're an expert when it comes to this kind of job. Hahaha, I don't have to worry about you anymore."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Rumis Block",
                args!["Have you brought Hillsrion's Horns?", "Those are the horns in your hands?"],
            )?;
            ctx.next()?;
            ctx.lines_as("Rumis Block", args!["Oh..."])?;
            ctx.next()?;
            ctx.lines_as("Rumis Block", args!["Ho..."])?;
            ctx.next()?;
            ctx.lines_as(
                "Rumis Block",
                args![
                    "Well, they appear to be ordinary horns with a spiral pattern.",
                    "Monsters with such horns are a dime a dozen in Midgard."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Rumis Block",
                args!["I, however, still have no idea how Hillsrions use their horns."],
            )?;
            ctx.next()?;
            ctx.lines_as("Rumis Block", args!["If they have bad vision, they could use the horns as feelers, or... I know you don't want to hear me ramble on with my conjecture. Sorry."])?;
            ctx.next()?;
            ctx.lines_as("Rumis Block", args!["We can call it a day for today. Thanks to you, I now feel confident enough to at least come to this tree by myself next time."])?;
            ctx.next()?;
            ctx.lines_as(
                "Rumis Block",
                args!["Let's go back to the camp. Shall we? I have something to discuss with you."],
            )?;
            ctx.var("ep13_animal").set(Val::from(13))?;
            ctx.call(Function::DelItem, vec![Val::from(6032), Val::from(5)])?;
            ctx.call(Function::ChangeQuest, vec![Val::from(2155), Val::from(2156)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    return Err(Stop::End);
}

pub fn frozen_tree_evt_lumis(ctx: &Ctx) -> Script {
    frozen_tree_evt_lumis_body(ctx, Vec::new()).map(|_| ())
}

fn small_fairy_spl_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("BaseLevel").get()?.number()? > 69 && ctx.call(Function::CheckQuest, vec![Val::from(2158)])? == -1 {
        ctx.lines(args![
            "You find a little creature flying in the bushes.",
            "It has tiny wings on the back...",
            "It's a fairy!"
        ])?;
        ctx.next()?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_FRET")?])?;
        ctx.lines_as(
            "Small Fairy",
            args!["RLGHLRXLA TKANTLFDMS", "WJACK TNAHRDNJSDMFH", "WLSGHKWND !!"],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "The fairy notices you, and looks very surprised.",
            "It is saying something, but you don't understand."
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "You should report this to the united expedition quickly.",
            "Perhaps the guard captain of the expedition is the right person to report to."
        ])?;
        ctx.call(Function::SetQuest, vec![Val::from(2158)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.call(Function::Emotion, vec![ctx.constant("ET_FRET")?])?;
    if (!(ctx.call(Function::IsEquipped, vec![Val::from(2782)])?.is_true()) && ctx.var("ep13_2_rhea").get()?.number()? < 100) {
        ctx.lines_as(
            "Small Fairy",
            args!["RLGHLRXLA TKANTLFDMS", "WJACK TNAHRDNJSDMFH", "WLSGHKWND !!"],
        )?;
        ctx.next()?;
        ctx.mes("The surprised fairy is saying something to you, but you cannot understand fairy language.")?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as("Small Fairy", args!["Who are you?! Are you looking for the Sapha!?"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn small_fairy_spl(ctx: &Ctx) -> Script {
    small_fairy_spl_body(ctx, Vec::new()).map(|_| ())
}

fn tree_giant_man_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("BaseLevel").get()?.number()? > 69 && ctx.call(Function::CheckQuest, vec![Val::from(2159)])? == -1 {
        ctx.lines(args![
            "You have found something moving between dry branches.",
            "It appears to be a tree at first glance, but it turns out to be a giant that is half tree and half man."
        ])?;
        ctx.next()?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
        ctx.lines_as(
            "Tree Giant",
            args!["TJDTMFJDNS CJFDI", "TKADLFDMF QKATOS", "EKDTLSDML DLFMADMS.."],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "The giant notices you, and looks very surprised.",
            "It is saying something, but you don't understand."
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "You should report this to the united expedition quickly.",
            "Perhaps the guard captain of the expedition is the right person to report to."
        ])?;
        ctx.call(Function::SetQuest, vec![Val::from(2159)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
    if (!(ctx.call(Function::IsEquipped, vec![Val::from(2782)])?.is_true()) && ctx.var("ep13_2_rhea").get()?.number()? < 100) {
        ctx.lines_as(
            "Tree Giant",
            args!["TJDTMFJDNS CJFDI", "TKADLFDMF QKATOS", "EKDTLSDML DLFMADMS.."],
        )?;
        ctx.next()?;
        ctx.mes("The surprised giant is saying something to you, but you cannot understand.")?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Tree Giant",
            args!["Where are the Laphine reinforcements? I might have a problem."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn tree_giant_man(ctx: &Ctx) -> Script {
    tree_giant_man_body(ctx, Vec::new()).map(|_| ())
}

fn camp_guard_captain_man1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Captain",
        args![
            "Good day. I'm here to protect the",
            "peace and safety of explorers",
            "working for United Midgard.",
            "How may I help you?"
        ],
    )?;
    ctx.next()?;
    if (ctx.call(Function::CheckQuest, vec![Val::from(2158)])? == 2 && ctx.call(Function::CheckQuest, vec![Val::from(2159)])? == 2) {
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Oh, nothing.", "Sorry to bother you."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.call(Function::CheckQuest, vec![Val::from(2158)])? == 1 {
        ctx.mes("- You report your encounter with a small fairy in the Splandid area to the guard captain. -")?;
        ctx.next()?;
        ctx.lines_as(
            "Captain",
            args![
                "That sounds very important.",
                "It's too early to say this, but we might have to dispatch an investigation group to the area."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Captain",
            args![
                "I'll discuss this more with the expedition management.",
                "Thank you for your valuable information."
            ],
        )?;
        ctx.call(Function::CompleteQuest, vec![Val::from(2158)])?;
        ctx.call(Function::GetExperience, vec![Val::from(70000), Val::from(0)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.call(Function::CheckQuest, vec![Val::from(2158)])? == -1 {
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Oh, nothing.", "Sorry to bother you."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.call(Function::CheckQuest, vec![Val::from(2159)])? == 1 {
        ctx.mes("- You report your encounter with a tree giant in the Manuk area to the guard captain. -")?;
        ctx.next()?;
        ctx.lines_as(
            "Captain",
            args![
                "That sounds very important.",
                "It's too early to say this, but we might have to dispatch an investigation group to the area."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Captain",
            args![
                "I'll discuss this more with the expedition management.",
                "Thank you for your valuable information."
            ],
        )?;
        ctx.call(Function::CompleteQuest, vec![Val::from(2159)])?;
        ctx.call(Function::GetExperience, vec![Val::from(70000), Val::from(0)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Oh, nothing.", "Sorry to bother you."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn camp_guard_captain_man1(ctx: &Ctx) -> Script {
    camp_guard_captain_man1_body(ctx, Vec::new()).map(|_| ())
}

pub fn research_official_ep131(ctx: &Ctx) -> Script {
    research_official_ep131_run(ctx, ResearchOfficialEp131Step::Start, Vec::new()).map(|_| ())
}

pub fn research_official_ep131_ontouch(ctx: &Ctx) -> Script {
    research_official_ep131_run(ctx, ResearchOfficialEp131Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn research_official_ep131_oninit(ctx: &Ctx) -> Script {
    research_official_ep131_run(ctx, ResearchOfficialEp131Step::OnInit, Vec::new()).map(|_| ())
}

pub fn research_official_ep131_onenable(ctx: &Ctx) -> Script {
    research_official_ep131_run(ctx, ResearchOfficialEp131Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn research_official_ep131_ondisable(ctx: &Ctx) -> Script {
    research_official_ep131_run(ctx, ResearchOfficialEp131Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn research_official_ep131_onmeet(ctx: &Ctx) -> Script {
    research_official_ep131_run(ctx, ResearchOfficialEp131Step::OnMeet, Vec::new()).map(|_| ())
}

pub fn research_official_ep131_oncall(ctx: &Ctx) -> Script {
    research_official_ep131_run(ctx, ResearchOfficialEp131Step::OnCall, Vec::new()).map(|_| ())
}

pub fn research_official_ep131_onmymobdead(ctx: &Ctx) -> Script {
    research_official_ep131_run(ctx, ResearchOfficialEp131Step::OnMyMobDead, Vec::new()).map(|_| ())
}

pub fn research_official_ep131_ontimer300000(ctx: &Ctx) -> Script {
    research_official_ep131_run(ctx, ResearchOfficialEp131Step::OnTimer300000, Vec::new()).map(|_| ())
}

fn ryosen_ep131_rhea01_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0
        || (ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 3500)
    {
        ctx.lines_as(
            "Ryosen",
            args!["How come you've got so much to carry?", "Are you perhaps on training or something?"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("ep13_ryu").get()?.number()? > 99 || ctx.var("ep13_start").get()?.number()? > 99) {
        if ctx.var("ep13_1_rhea").get()?.number()? < 1 {
            ctx.lines_as("Ryosen", args!["Please, keep your hands off my stuff!"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("ep13_1_rhea").get()? == 1 {
                ctx.lines_as("Ryosen", args!["Please, keep your hands off my stuff!"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Ryosen",
                    args!["Ugh! My, my... What am I gonna do with this!? Files, files, files!!!", "Ahhhhhkk!"],
                )?;
                ctx.next()?;
                ctx.lines_as("Ryosen", args!["What time is it now? Ugh... This is such a mess!!"])?;
                ctx.next()?;
                ctx.lines_as("Ryosen", args!["What are they thinking!!? I don't get it. How come I have to work, while those guys from other countries are just praying for a breakthrough, every single day!"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Ryosen",
                    args![
                        "This is so unfair!! My stress is growing geometrically every hour!",
                        "What a waste of time! Why do they have to pray! Why?! Why?! Why don't they work instead of pray!!",
                        "Ugh...!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Ryosen", args!["Nothing's appreciated! Nothing!", "They're just so useless!"])?;
                ctx.next()?;
                ctx.lines(args!["- He took a deep breath -", "- trying to calm down. -"])?;
                ctx.next()?;
                ctx.lines_as("Ryosen", args!["Anyway, who are you? This place is restricted."])?;
                ctx.next()?;
                ctx.lines_as("Ryosen", args!["Ah! You must be the one who came to help us track down that Satan Morocc, the bastard who made the giant hole and then was gone like the wind."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Ryosen",
                    args![
                        "Ugh. I should watch my mouth.",
                        "Sorry. You're on our side. I should just be polite, but it's just... I'm so stressed out."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Ryosen", args!["How about having some tea here?", "It's Rafflesia tea from Schwarzwald. Don't even expect taste, it's awful... But what can I do? It's the only thing I've got..."])?;
                ctx.var("ep13_1_rhea").set(Val::from(2))?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("ep13_1_rhea").get()? == 2 {
                    ctx.lines_as("Ryosen", args!["You must have heard about this, but this is the place where Satan Morocc landed through the crack of dimension."])?;
                    ctx.next()?;
                    ctx.lines_as("Ryosen", args!["As you see, this seems like a totally different world. It's nothing like the continent of Midgard, though we haven't found any concrete evidence. We decided to call this place Ash-Vacuum."])?;
                    ctx.next()?;
                    ctx.lines_as("Ryosen", args!["We're doing everything we can, to track down Satan Morocc, but all those researchers from different countries couldn't help but wonder what this place really is."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ryosen",
                        args!["Well, so we're all interested in one thing. However, the real problem lies on our structure of the group."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Ryosen", args!["Three different countries united to take down the comon enemy, Satan Morocc, byt they eventually started to gather and keep the information to themselves."])?;
                    ctx.next()?;
                    ctx.lines_as("Ryosen", args!["So... frankly speaking...", "Tracking Satan Morocc is not or major concern anymore. Our expedition was formed in such hase that so may things can cause problems in this group."])?;
                    ctx.next()?;
                    ctx.lines_as("Ryosen", args!["People don't believe other members of the group except for the ones from their own countries. They're all so sensitive that every little thing hits a nerve."])?;
                    ctx.next()?;
                    ctx.lines_as("Ryosen", args!["But what can we do? After all, we were gathered to achieve the same goal, and we researchers don't want to cause any trouble among countries. It's really hard for us to live with our complaints."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ryosen",
                        args!["I mean... the situation here is... just like the taste of this tea.", "Just awful!"],
                    )?;
                    ctx.var("ep13_1_rhea").set(Val::from(3))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("ep13_1_rhea").get()? == 3 {
                        ctx.lines_as(
                            "Ryosen",
                            args!["Oh, I spent too much time talking.", "I should be working now!"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Ryosen", args!["Look at this pile of document files... We're supposed to share all the information within these documents..."])?;
                        ctx.next()?;
                        ctx.lines_as("Ryosen", args!["Oh, well... Maybe the same thing's happening at other offices. People from other countries might have hidden some important information behind my back."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ryosen",
                            args!["Whatever... Could you deliver this document to the Schwarzwald and Arunafeltz researchers?"],
                        )?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(ctx, &[Val::from("Why me?:Alright.")])?) == 1 {
                            ctx.lines_as("Ryosen", args!["... Eh? Didn't you come to help us?"])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as(
                            "Ryosen",
                            args![
                                "Whew~ Very good.",
                                "No one in our research group wants to visit the offices of the other countries!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Ryosen", args!["Ok, now, please deliver the file to Hue and Hansenne. Hue's in charge of Schwarzwald's documents and Hansenne's in chare of Arunafeltz's."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ryosen",
                            args!["The file contains the brief introduction and the schedule of the meesting."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Ryosen", args!["For your information, you should go see Hue at the Schwarzwald's office first. Otherwise, he'll be whining about it."])?;
                        ctx.next()?;
                        ctx.lines_as("Ryosen", args!["You must check if they read and understand correctly after delivering it. Don't forget to get them to sign at the bottom of the page, just in case."])?;
                        ctx.next()?;
                        ctx.lines_as("Ryosen", args!["Go on now.", "Just so you know, today's Desert Day. Midgard is sending some desserts through the crack of dimension, regularly."])?;
                        ctx.next()?;
                        ctx.lines_as("Ryosen", args!["Midgard's desserts are definitely the world's best."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ryosen",
                            args!["The only thing we enjoy here is that dessert we receive, once a week."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ryosen",
                            args!["Schwarzwald's Pineapple Jubilee is just awful. It tastes something like rusty steel."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Ryosen", args!["Arunafeltz's Desert Sandwich is nothing better than that. I think that sandwich's made of some old meat, otherwise, we wouldn't have been food poisoned so often!"])?;
                        ctx.next()?;
                        ctx.lines_as("Ryosen", args!["But, Rune-Midgarts' Strawberry Cake! It's the best."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ryosen",
                            args![
                                "Nice and soft spongecake covered with sweet-tasting whipped cream, lots of fresh strawberries on top!",
                                "Rune-Midgarts' top chef, Charles Orleans makes it himself!!!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ryosen",
                            args!["Hurry, you must hurry up, or you won't be able to taste this wonderful cake!"],
                        )?;
                        ctx.next()?;
                        ctx.lines(args![
                            "- You got the Meeting -",
                            "- Invitation from Ryosen, -",
                            "- which briefly explains -",
                            "- the agenda and schedule -",
                            "- of the whole meeting! -"
                        ])?;
                        ctx.var("ep13_1_rhea").set(Val::from(4))?;
                        ctx.call(Function::GetItem, vec![Val::from(6036), Val::from(1)])?;
                        ctx.call(Function::ChangeQuest, vec![Val::from(8196), Val::from(8197)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if (ctx.var("ep13_1_rhea").get()?.number()? > 3 && ctx.var("ep13_1_rhea").get()?.number()? < 8) {
                            ctx.lines_as("Ryosen", args!["Hurry up and deliver the document to Hue and Hansenne!"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Ryosen",
                                args!["Don't forget! Schwarzwald's Hue must get it first, otherwise, he'll nag us like crazy."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Ryosen",
                                args!["And you must get the document signed at the bottom before you bring it back."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Ryosen",
                                args!["Hurry, you must hurry up or you won't be able to taste this wonderfull cake!"],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            if ctx.var("ep13_1_rhea").get()? == 8 {
                                if ctx.call(Function::CountItem, vec![Val::from(6036)])?.number()? > 0 {
                                    ctx.lines_as("Ryosen", args!["Ah, welcome back!"])?;
                                    ctx.next()?;
                                    ctx.lines_as("Ryosen", args!["Must've been hard to get them all signed. Well done!"])?;
                                    ctx.next()?;
                                    ctx.lines_as("Ryosen", args!["Hue's so egotistic and stubborn. He thinks he's some kind of royale... and Hansenne... Whew~nobody ever gets what he's just talking about.", "He's just ridiculous."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Ryosen", args!["Thank you for dealing with those guys. I've got to go through the file now. Please give me the Invitation."])?;
                                    ctx.call(Function::DelItem, vec![Val::from(6036), Val::from(1)])?;
                                    ctx.var("ep13_1_rhea").set(Val::from(9))?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    ctx.lines_as(
                                        "Ryosen",
                                        args!["Please get Hue and Hansenne to sign the Inviation, then come back."],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Ryosen",
                                        args!["Hurry, you must hurry up or you won't be able to taste this wonderfull cake!"],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                            } else {
                                if ctx.var("ep13_1_rhea").get()? == 9 {
                                    ctx.lines_as("Ryosen", args!["Huhuhuhu, what a sight...!"])?;
                                    ctx.next()?;
                                    ctx.lines_as("Ryosen", args!["This autograph here looks like Hue's. See, the letters are all so strongly pressed down with his anger. This one here should be Hansenne's, scrawled without thinking."])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Ryosen",
                                        args!["How was it? Did you have a chance t find out what the problem is among us researchers?"],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Ryosen", args!["This isn't all because of the different personalities. Lots of other members have similar problems. Howerver, the problem among us three: Hue, Hansenne and I... is just more visible than others."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Ryosen", args!["So, here we are. We are finally ordered to have a meeting and supposed to find the way to solve our problem."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Ryosen", args!["But, do you know what really would be the best solution for us? It's to avoid meeting face to face."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Ryosen", args!["It wasn't our will to have this meeting, and I never imagined they'd sign this thing... Well, I just feel like I've been punched in the head or something."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Ryosen", args!["Well, it's already spilled milk. I need to go to that meeting whether I like it or not. Could you please send this to the Official of the United Research?"])?;
                                    ctx.next()?;
                                    ctx.lines_as("Ryosen", args!["In the meantime, I should prepare myself for the meeting."])?;
                                    ctx.var("ep13_1_rhea").set(Val::from(10))?;
                                    ctx.call(Function::GetItem, vec![Val::from(6036), Val::from(1)])?;
                                    ctx.call(Function::ChangeQuest, vec![Val::from(8197), Val::from(8198)])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    if ctx.var("ep13_1_rhea").get()? == 10 {
                                        ctx.lines_as("Ryosen", args!["It wasn't our will to have this meeting, and I never imagined they'd sign this thing... Well, I just feel like I've been punched in the head or something."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Ryosen", args!["Well, it's already spilled milk. I need to go to that meeting whether I like it or not. Could you please send this to the Official of the United Research?"])?;
                                        ctx.next()?;
                                        ctx.lines_as("Ryosen", args!["In the meantime, I should prepare myself for the meeting."])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else {
                                        if ctx.var("ep13_1_rhea").get()? == 11 {
                                            ctx.lines_as("Ryosen", args!["Hi! Did you give that Invitation to the Official?"])?;
                                            ctx.next()?;
                                            ctx.lines_as("Ryosen", args!["Oh, goo to hear that it's delivered safely.", "Anyway, what brings you here again?", "Would you like some tea? We haven't got any dessert yet. I only have some tea to offer."])?;
                                            ctx.next()?;
                                            let choice = runtime::select_values(ctx, &[Val::from("Can I help you with anything?")])?;
                                            ctx.var("@menu").set(choice)?;
                                            ctx.lines_as("Ryosen", args!["Help?", "Well, there are tons of things waiting for help!"])?;
                                            ctx.next()?;
                                            ctx.lines_as("Ryosen", args!["Nice Timing.", "I've requested something from the Arunafeltz office but haven't gotten anything yet. Could you get it from Hansenne?"])?;
                                            ctx.next()?;
                                            ctx.lines_as("Ryosen", args!["I need that for the meeting, but he never sends it to me. He must've forgotten about it, again! You'll help me with this, right!?"])?;
                                            ctx.var("ep13_1_rhea").set(Val::from(12))?;
                                            ctx.call(Function::ChangeQuest, vec![Val::from(8199), Val::from(8200)])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        } else {
                                            if ctx.var("ep13_1_rhea").get()? == 12 {
                                                ctx.lines_as("Ryosen", args!["I've requested something from the Arunafeltz office but haven't gotten anything yet. Could you get it from Hansenne?"])?;
                                                ctx.next()?;
                                                ctx.lines_as("Ryosen", args!["I need that for the meeting, but he never sends it to me. He must've forgotten about it, again! You'll help me with this, right!?"])?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            } else {
                                                if ctx.var("ep13_1_rhea").get()? == 13 {
                                                    ctx.lines_as("Ryosen", args!["Huh? Are you serious???? Dropped it on a bridge while playing treasure hunting???"])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Ryosen", args!["Hansenne! What and itiot! I knew it! His brain must've been melted with hot air!", "Aarrrrr!!!!! Can't believe this!!"])?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Ryosen",
                                                        args![
                                                            "What the heck should I do??? The meesting's soon!!!",
                                                            "Oh my! I really, prefer to work alone. Oh, I hate those guys!",
                                                            "I hate foreigners."
                                                        ],
                                                    )?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                } else {
                                                    if ctx.var("ep13_1_rhea").get()? == 14 {
                                                        if ctx.call(Function::CountItem, vec![Val::from(6037)])?.number()? > 0 {
                                                            ctx.lines_as("Ryosen", args![".....Whew... completely soaked, so loosely binded...torn here and there... I can't even read this thing!"])?;
                                                            ctx.next()?;
                                                            ctx.lines_as("Ryosen", args!["What is he expecting from me with this!!!!"])?;
                                                            ctx.next()?;
                                                            ctx.lines_as("Ryosen", args!["Aarrrrr!!!!! This is a nightmare!"])?;
                                                            ctx.close_window()?;
                                                            return Err(Stop::End);
                                                        } else {
                                                            ctx.lines_as("Ryosen", args!["Huh? Are you serious???? Dropped it on a bridge while playing treasure hunting???"])?;
                                                            ctx.next()?;
                                                            ctx.lines_as("Ryosen", args!["Hansenne! What and idiot! I knew it! His brain must've been melted with hot air!", "Aarrrrr!!!!! Can't believe this!!"])?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "Ryosen",
                                                                args![
                                                                    "What the heck should I do??? The meesting's soon!!!",
                                                                    "Oh my! I really, prefer to work alone. Oh, I hate those guys!",
                                                                    "I hate foreigners."
                                                                ],
                                                            )?;
                                                            ctx.close_window()?;
                                                            return Err(Stop::End);
                                                        }
                                                    } else {
                                                        if ctx.var("ep13_1_rhea").get()? == 18 {
                                                            if ctx.call(Function::CountItem, vec![Val::from(6038)])?.number()? > 0 {
                                                                ctx.lines_as("Ryosen", args!["...Oh, this is..."])?;
                                                                ctx.next()?;
                                                                ctx.lines_as("Ryosen", args!["This is the document I requested.", "I thought the document was lost at some bridge and ruined completely, but then..."])?;
                                                                ctx.next()?;
                                                                ctx.lines_as("Ryosen", args!["Wow... this document is amazingly neat. Looks like Hue's the one who made this... He's really picky, if you know what I mean... Anyway, it's pretty impressive..."])?;
                                                                ctx.next()?;
                                                                ctx.lines_as("Ryosen", args!["I'm so relieved I got this before the meeting. I've got to go now.", "I'm running late."])?;
                                                                ctx.next()?;
                                                                ctx.lines_as("Ryosen", args!["But first, I have to find another document for the meeting. Could you please ask the Official to wait for me a bit?"])?;
                                                                ctx.next()?;
                                                                ctx.lines_as("Ryosen", args!["Please, the meeting's gonna begin soon."])?;
                                                                ctx.call(Function::DelItem, vec![Val::from(6038), Val::from(1)])?;
                                                                ctx.var("ep13_1_rhea").set(Val::from(19))?;
                                                                ctx.call(Function::ChangeQuest, vec![Val::from(8204), Val::from(8205)])?;
                                                                ctx.close_window()?;
                                                                return Err(Stop::End);
                                                            } else {
                                                                ctx.lines_as(
                                                                    "Ryosen",
                                                                    args!["How could I go to the meeting without the research report??!"],
                                                                )?;
                                                                ctx.next()?;
                                                                ctx.lines_as(
                                                                    "Ryosen",
                                                                    args![
                                                                        "Oh my! I really, prefer to work alone. Oh, I hate those guys!",
                                                                        "I hate foreigners."
                                                                    ],
                                                                )?;
                                                                ctx.close_window()?;
                                                                return Err(Stop::End);
                                                            }
                                                        } else {
                                                            if ctx.var("ep13_1_rhea").get()? == 19 {
                                                                ctx.lines_as(
                                                                    "Ryosen",
                                                                    args!["Could you please ask the Official to wait for me a bit?"],
                                                                )?;
                                                                ctx.next()?;
                                                                ctx.lines_as(
                                                                    "Ryosen",
                                                                    args![
                                                                        "I have to find another document for the meeting. Please, hurry."
                                                                    ],
                                                                )?;
                                                                ctx.close_window()?;
                                                                return Err(Stop::End);
                                                            } else {
                                                                if ctx.var("ep13_1_rhea").get()? == 20 {
                                                                    ctx.lines_as("Ryosen", args!["Ah..... That... in that meeting room... Is.. everything alright now?"])?;
                                                                    ctx.close_window()?;
                                                                    return Err(Stop::End);
                                                                } else if (ctx.var("ep13_1_rhea").get()?.number()? > 20
                                                                    && ctx.var("ep13_1_rhea").get()?.number()? < 25)
                                                                {
                                                                    ctx.lines_as(
                                                                        "Ryosen",
                                                                        args!["Oh, I need to be left alone for a while."],
                                                                    )?;
                                                                    ctx.close_window()?;
                                                                    return Err(Stop::End);
                                                                } else if ctx.var("ep13_1_rhea").get()? == 25 {
                                                                    ctx.lines_as("Ryosen", args!["Krrrrrrrr! Ugh...!"])?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                                        args!["Uhhhh..."],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        "Ryosen",
                                                                        args![
                                                                            "I know, I know! I went too far!",
                                                                            "I was just so shocked and disappointed at the same time.",
                                                                            "After all, it was just a cake..."
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        "Ryosen",
                                                                        args![
                                                                            "I wasn't very mature.",
                                                                            "Oh, what have I done?",
                                                                            "I should go an apologize."
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                                        args!["Wait! Hue wanted me to bring this to you."],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    if ctx.call(Function::CountItem, vec![Val::from(6038)])?.number()? > 0 {
                                                                        ctx.lines_as(
                                                                            "Ryosen",
                                                                            args![
                                                                                "A report... ?",
                                                                                "Ahhh... the one I left in the meeting room..."
                                                                            ],
                                                                        )?;
                                                                        ctx.next()?;
                                                                        ctx.lines(args![
                                                                            "- Ryosen takes the report -",
                                                                            "- and gives a glance -",
                                                                            "- at the memo stuck on it. -"
                                                                        ])?;
                                                                        ctx.next()?;
                                                                    } else {
                                                                        ctx.lines_as("Ryosen", args!["... ?"])?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as("Ryosen", args!["Oh. What are you talking about?"])?;
                                                                        ctx.close_window()?;
                                                                        return Err(Stop::End);
                                                                    }
                                                                    ctx.lines_as("Ryosen", args!["Ah. I'm so embarrassed."])?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        "Ryosen",
                                                                        args![
                                                                            "Oh, I didn't expect to get an apology. This is such a shame."
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as("Ryosen", args!["Now that I think of it. I didn't even say 'thank you' to him for restoring my documents..."])?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as("Ryosen", args!["Ok. I should setup another meeting ASAP. That way, we'd have a chance to get to know each other."])?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as("Ryosen", args!["So... Could you do me another favor?", "Could you go see the Official in my stead and set up a meeting?"])?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as("Ryosen", args!["Oh! And... Here's Rune-Midgarts' fresh strawberry cake. This is for you. I'm so sorry I haven't even offered you a piece. I was so distracted with stress, I think."])?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as("Ryosen", args!["Thank you so much for everything you've done. And... pleade don't forget to do me this last favor, going to the Official and setting up a meeting."])?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        "Ryosen",
                                                                        args!["I... think I should prepare for the meeting again."],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as("Ryosen", args!["Again, thank you so much."])?;
                                                                    ctx.call(Function::DelItem, vec![Val::from(6038), Val::from(1)])?;
                                                                    ctx.var("ep13_1_rhea").set(Val::from(26))?;
                                                                    ctx.call(Function::GetItem, vec![Val::from(12319), Val::from(1)])?;
                                                                    ctx.call(
                                                                        Function::ChangeQuest,
                                                                        vec![Val::from(8209), Val::from(8210)],
                                                                    )?;
                                                                    ctx.close_window()?;
                                                                    return Err(Stop::End);
                                                                } else if (ctx.var("ep13_1_rhea").get()?.number()? > 25
                                                                    && ctx.var("ep13_1_rhea").get()?.number()? < 100)
                                                                {
                                                                    ctx.lines_as("Ryosen", args!["Ok. I should setup another meeting ASAP. That way, we'd have a chance to get to know each other."])?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as("Ryosen", args!["Could you go see the Official in my stead and set up a meeting?", "I... think I should prepare for the meeting again."])?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as("Ryosen", args!["Thank you so much."])?;
                                                                    ctx.close_window()?;
                                                                    return Err(Stop::End);
                                                                } else if ctx.var("ep13_1_rhea").get()?.number()? > 99 {
                                                                    ctx.lines_as("Ryosen", args!["Ah, hello adventurer!"])?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        "Ryosen",
                                                                        args![
                                                                            "I don't know how to thank you.",
                                                                            "You can't imagine how greatful I feel!"
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as("Ryosen", args!["Oh, but then... Have you ever seen a ^0000ffMystic Horn^000000? I'd love to get one.", "I heard you can get this thing from a monster named, 'Cornus' found in the fields."])?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as("Ryosen", args!["Or... do you have any ^0000ffMystic Horns^000000 with you? Of course, I'm not saying that I want it for free. If you'd give me ^0000ff2 Mystic Horns^000000, I'd give you my Rune-Midgarts' Strawberry Cake."])?;
                                                                    ctx.next()?;
                                                                    match runtime::select_values(
                                                                        ctx,
                                                                        &[Val::from("I haven't seen such a thing.:Oh, ok, I'll give you!")],
                                                                    )? {
                                                                        1 => {
                                                                            ctx.lines_as("Ryosen", args!["Mmmmmmm~"])?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as(
                                                                                "Ryosen",
                                                                                args![
                                                                                    "If by any chance you find one, please come back to me."
                                                                                ],
                                                                            )?;
                                                                            ctx.close_window()?;
                                                                            return Err(Stop::End);
                                                                        }
                                                                        2 => {
                                                                            if ctx
                                                                                .call(Function::CountItem, vec![Val::from(12319)])?
                                                                                .number()?
                                                                                > 4
                                                                            {
                                                                                ctx.lines_as("Ryosen", args!["Huh?"])?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as(
                                                                                    "Ryosen",
                                                                                    args!["You have Rune-Midgarts' Strawberry Cake?"],
                                                                                )?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as("Ryosen", args!["How come you haven't had it yet?", "You should quickly finish that cake or else, somebody would steal it from you."])?;
                                                                                ctx.close_window()?;
                                                                                return Err(Stop::End);
                                                                            }
                                                                        }
                                                                        _ => {}
                                                                    }
                                                                    ctx.lines_as("Ryosen", args!["I don't need it!"])?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as("Ryosen", args!["... Haha, I'm joking..."])?;
                                                                    ctx.next()?;
                                                                    if ctx.call(Function::CountItem, vec![Val::from(6023)])?.number()? > 1 {
                                                                        ctx.lines_as("Ryosen", args!["Are you really, honestly, positively sure you want to exchange them for my cake?"])?;
                                                                        ctx.next()?;
                                                                        if Val::from(runtime::select_values(
                                                                            ctx,
                                                                            &[Val::from("No way.:Yeah... I'll exchange.")],
                                                                        )?) == 1
                                                                        {
                                                                            ctx.lines_as("Ryosen", args!["Ahhh..."])?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as(
                                                                                "Ryosen",
                                                                                args!["Come back any time if you change your mind."],
                                                                            )?;
                                                                            ctx.close_window()?;
                                                                            return Err(Stop::End);
                                                                        }
                                                                        ctx.lines_as(
                                                                            "Ryosen",
                                                                            args!["I don't need it!", "... Haha, I'm joking..."],
                                                                        )?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as("Ryosen", args!["I heard you could extract some magical energy form Mystic Horns and use it. Anyway, thank you!"])?;
                                                                        ctx.call(Function::DelItem, vec![Val::from(6023), Val::from(2)])?;
                                                                        ctx.call(Function::GetItem, vec![Val::from(12319), Val::from(1)])?;
                                                                        ctx.close_window()?;
                                                                        return Err(Stop::End);
                                                                    } else {
                                                                        ctx.lines_as(
                                                                            "Ryosen",
                                                                            args!["You lie! You don't even have any!!"],
                                                                        )?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as(
                                                                            "Ryosen",
                                                                            args!["Please, if you find any Mystic Horns, come back to me!"],
                                                                        )?;
                                                                        ctx.close_window()?;
                                                                        return Err(Stop::End);
                                                                    }
                                                                } else {
                                                                    ctx.lines_as("Ryosen", args!["Please, don't touch anything!"])?;
                                                                    ctx.close_window()?;
                                                                    return Err(Stop::End);
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        ctx.lines_as("Ryosen", args!["You know somethig... Something comes out from this thing..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Ryosen",
            args!["If anyone tried to talk to you in the middle of the night, you should just walk away... Don't ever look back!!"],
        )?;
        ctx.next()?;
        ctx.lines_as("Ryosen", args!["And don't stare at the crack of dimension... or you'd be dragged to somewhere mysterious and never be able to come back...huhuhuhu..."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn ryosen_ep131_rhea01(ctx: &Ctx) -> Script {
    ryosen_ep131_rhea01_body(ctx, Vec::new()).map(|_| ())
}

use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn kiehl_original_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(908), Val::from(200)])? == 0 {
        ctx.lines(args![
            "^3355FFJust a second...",
            "You're carrying too",
            "many items with you",
            "right now, so you'll",
            "need to free up more",
            "Inventory space first...^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.call(Function::Cutin, vec![Val::from("kh_kiel01"), Val::from(2)])?;
    if (ctx.var("kielhyrequest").get()?.number()? < 94 || ctx.var("kielhyrequest").get()?.number()? >= 106) {
        ctx.lines_as("Kiehl", args!["......", ".........", "............"])?;
        ctx.close_window()?;
        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
        return Err(Stop::End);
    }
    if (ctx.var("kielhyrequest").get()? == 94
        && ctx
            .call(
                Function::GetVariableOfNpc,
                vec![Val::from(".khkilled"), Val::from("KiehlRoom"), Val::from(0)],
            )?
            .number()?
            < 5)
    {
        ctx.lines_as(
            "Kiehl",
            args![
                "I'm surprised you made",
                "it this far, adventurer~",
                "I bid you welcome to my",
                "humble room. I assume that",
                "you've come for the Condensed",
                "Memory Scroll... my mind."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kiehl",
            args![
                "You can understand",
                "why I can't let you have it,",
                "so if you really want the",
                "Condensed Memory Scroll,",
                "then show me what you've got!"
            ],
        )?;
        ctx.close_window()?;
        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
        ctx.call(
            Function::SetVariableOfNpc,
            vec![Val::from(".khkilled"), Val::from("KiehlRoom"), Val::from(0), Val::from(0)],
        )?;
        ctx.call(
            Function::Monster,
            vec![
                Val::from("kh_kiehl02"),
                Val::from(50),
                Val::from(52),
                Val::from("Aliot"),
                Val::from(1740),
                Val::from(1),
                Val::from("KiehlRoom::OnKiehlMobDead"),
            ],
        )?;
        ctx.call(
            Function::Monster,
            vec![
                Val::from("kh_kiehl02"),
                Val::from(50),
                Val::from(52),
                Val::from("Alicel"),
                Val::from(1739),
                Val::from(1),
                Val::from("KiehlRoom::OnKiehlMobDead"),
            ],
        )?;
        ctx.call(
            Function::Monster,
            vec![
                Val::from("kh_kiehl02"),
                Val::from(50),
                Val::from(52),
                Val::from("Constant"),
                Val::from(1745),
                Val::from(1),
                Val::from("KiehlRoom::OnKiehlMobDead"),
            ],
        )?;
        ctx.call(
            Function::Monster,
            vec![
                Val::from("kh_kiehl02"),
                Val::from(50),
                Val::from(52),
                Val::from("Aliot"),
                Val::from(1740),
                Val::from(1),
                Val::from("KiehlRoom::OnKiehlMobDead"),
            ],
        )?;
        ctx.call(
            Function::Monster,
            vec![
                Val::from("kh_kiehl02"),
                Val::from(50),
                Val::from(52),
                Val::from("Alicel"),
                Val::from(1739),
                Val::from(1),
                Val::from("KiehlRoom::OnKiehlMobDead"),
            ],
        )?;
        ctx.call(
            Function::Monster,
            vec![
                Val::from("kh_kiehl02"),
                Val::from(50),
                Val::from(52),
                Val::from("Constant"),
                Val::from(1745),
                Val::from(1),
                Val::from("KiehlRoom::OnKiehlMobDead"),
            ],
        )?;
        ctx.call(
            Function::Monster,
            vec![
                Val::from("kh_kiehl02"),
                Val::from(50),
                Val::from(52),
                Val::from("Aliot"),
                Val::from(1740),
                Val::from(1),
                Val::from("KiehlRoom::OnKiehlMobDead"),
            ],
        )?;
        ctx.call(Function::DisableNpc, vec![Val::from("Kiehl#Original")])?;
        return Err(Stop::End);
    } else if (ctx.var("kielhyrequest").get()? == 94
        && ctx
            .call(
                Function::GetVariableOfNpc,
                vec![Val::from(".khkilled"), Val::from("KiehlRoom"), Val::from(0)],
            )?
            .number()?
            >= 5)
    {
        ctx.call(Function::Cutin, vec![Val::from("kh_kiel03"), Val::from(2)])?;
        ctx.lines_as(
            "Kiehl",
            args![
                "Hmpf! You're pretty good.",
                "Father must have spent",
                "a lot of money to hire",
                "you. So has he sent",
                "you to kill me?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Kiel Hyre sent me to ask",
                "you to stop turning all of",
                "the Third Generation robots",
                "into killing machines! How",
                "can do something like that",
                "to other robots like you?"
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("kh_kiel01"), Val::from(2)])?;
        ctx.lines_as(
            "Kiehl",
            args![
                "Why not? It's said that man",
                "was made in the image of God.",
                "Well, robots were made in the",
                "image of man. You humans kill",
                "each other as much as you",
                "like, as far as I can tell."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kiehl",
            args![
                "It might not be ethical",
                "for me to provide weapons",
                "to humans that need them...",
                "But that's what they are.",
                "Weapons. It's more humane",
                "for robots to fight than humans."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kiehl",
            args![
                "Robots don't naturally",
                "feel pain or emotions...",
                "Not unless they're specially",
                "programmed. Sorry, but I don't",
                "plans to stop what I'm doing."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("kh_kiel04"), Val::from(2)])?;
        ctx.lines_as(
            "Kiehl",
            args![
                "I am sorry, but I don't have a plan to stop what I am doing.",
                "Aside from that, my father showed me a great example of",
                "how far a human could go for his own selfinishness by destroying a family.",
                "And therefore I don't think that he could create better robots than mine."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Humankind may not be",
                "perfect, but think about",
                "who you're working with!",
                "Rekenber is the epitome of",
                "human evil! How can you",
                "support them like this?"
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("kh_kiel02"), Val::from(2)])?;
        ctx.lines_as(
            "Kiehl",
            args![
                "I've had a long relationship",
                "with Rekenber. I'm fully aware",
                "of their capabilities. Do you",
                "remember the first room you",
                "passed on your way here, the",
                "one with all the toys?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kiehl",
            args![
                "I bet you didn't know that there",
                "were 5 Second Generation",
                "robots. Me, and my four other",
                "brothers and sisters. Father",
                "built that room so that all",
                "five of us could live together."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("kh_kiel03"), Val::from(2)])?;
        ctx.lines_as(
            "Kiehl",
            args![
                "I was the only one to",
                "survive. I returned to",
                "Father and even got a",
                "name. But yes, I know",
                "how bad Rekenber really is..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kiehl",
            args![
                "I don't... I don't want",
                "to talk about this any more.",
                "You've made me... Just leave.",
                "I think I will let you live."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
        ctx.lines_as(
            "Mitchell",
            args![
                "Freeze!",
                "Kiehl Hyre, you're",
                "under arrest for creating",
                "and trading illegal weapons!"
            ],
        )?;
        ctx.var("kielhyrequest").set(Val::from(100))?;
        ctx.call(Function::EnableNpc, vec![Val::from("Mitchell#KiehlRoom")])?;
        ctx.call(Function::EnableNpc, vec![Val::from("Agent#KHAgent1")])?;
        ctx.call(Function::EnableNpc, vec![Val::from("Agent#KHAgent2")])?;
        ctx.call(Function::EnableNpc, vec![Val::from("Agent#KHAgent3")])?;
        ctx.call(Function::EnableNpc, vec![Val::from("Agent#KHAgent4")])?;
        ctx.next()?;
    }
    if (ctx.var("kielhyrequest").get()? == 100
        && ctx
            .call(
                Function::GetVariableOfNpc,
                vec![Val::from(".khkilledboss"), Val::from("KiehlRoom"), Val::from(0)],
            )?
            .number()?
            < 1)
    {
        ctx.call(
            Function::SetVariableOfNpc,
            vec![Val::from(".khkilledboss"), Val::from("KiehlRoom"), Val::from(0), Val::from(0)],
        )?;
        ctx.call(Function::Cutin, vec![Val::from("kh_kiel01"), Val::from(2)])?;
        ctx.lines_as(
            "Kiehl",
            args![
                "Ah, Schwarzwald Republic",
                "agents. Heh. I haven't had",
                "this many guests before.",
                "Well, I guess this means",
                "we'll have to fight after",
                "all, you and I. *Sigh* Pity."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kiehl",
            args![
                "First, in all fairness,",
                "let me take care of these",
                "nuisances. They're just",
                "mindlessly doing their",
                "jobs--sort of like robots-- but",
                "don't worry, they won't be hurt."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Mitchell", args!["Nooooo!"])?;
        ctx.call(
            Function::NpcSpecialEffect,
            vec![ctx.constant("PF_FOGWALL")?, ctx.constant("AREA")?, Val::from("Mitchell#KiehlRoom")],
        )?;
        ctx.call(
            Function::NpcSpecialEffect,
            vec![ctx.constant("PF_FOGWALL")?, ctx.constant("AREA")?, Val::from("Agent#KHAgent1")],
        )?;
        ctx.call(
            Function::NpcSpecialEffect,
            vec![ctx.constant("PF_FOGWALL")?, ctx.constant("AREA")?, Val::from("Agent#KHAgent2")],
        )?;
        ctx.call(
            Function::NpcSpecialEffect,
            vec![ctx.constant("PF_FOGWALL")?, ctx.constant("AREA")?, Val::from("Agent#KHAgent3")],
        )?;
        ctx.call(
            Function::NpcSpecialEffect,
            vec![ctx.constant("PF_FOGWALL")?, ctx.constant("AREA")?, Val::from("Agent#KHAgent4")],
        )?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("kh_kiel03"), Val::from(2)])?;
        ctx.lines_as(
            "Kiehl",
            args![
                "And now, you and I can",
                "have a proper duel, human.",
                "I'm interested in seeing just",
                "how strong you really are~"
            ],
        )?;
        ctx.close_window()?;
        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
        ctx.call(
            Function::Monster,
            vec![
                Val::from("kh_kiehl02"),
                Val::from(50),
                Val::from(52),
                Val::from("Kiehl"),
                Val::from(1733),
                Val::from(1),
                Val::from("KiehlRoom::OnKiehlDead"),
            ],
        )?;
        ctx.call(Function::DisableNpc, vec![Val::from("Kiehl#Original")])?;
        return Err(Stop::End);
    } else if (ctx.var("kielhyrequest").get()? == 100
        && ctx.call(
            Function::GetVariableOfNpc,
            vec![Val::from(".khkilledboss"), Val::from("KiehlRoom"), Val::from(0)],
        )? == 1)
    {
        ctx.call(Function::Cutin, vec![Val::from("kh_kiel02"), Val::from(2)])?;
        ctx.lines_as(
            "Kiehl",
            args![
                "D-damn...!",
                "Well played, adventurer.",
                "Well played. I should have",
                "known that Father would send",
                "the very best after me. Still,",
                "you've failed to truly defeat me."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("kh_kiel03"), Val::from(2)])?;
        ctx.lines_as(
            "Kiehl",
            args![
                "I still have a few",
                "trump cards left",
                "I think... I'll take you",
                "to hell with me... Well,",
                "if robots can go there~"
            ],
        )?;
        ctx.next()?;
        ctx.call(
            Function::MapAnnounce,
            vec![
                Val::from("kh_kiehl02"),
                Val::from("*Jeeeezzzgggg~ Geezzz~ Grrrr~ Clank~*"),
                ctx.constant("BC_MAP")?,
                Val::from("0xFF0000"),
            ],
        )?;
        ctx.lines_as(
            "Mitchell",
            args!["No...! We're locked", "in the room! We're...", "We're trapped in here!"],
        )?;
        ctx.next()?;
        ctx.lines_as("Kiehl", args!["Yay~ Let's burn", "everything down~"])?;
        ctx.next()?;
        ctx.lines_as(
            "Mitchell",
            args![
                ((Val::from("Quick, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(",")),
                "use Kiel Hyre's power",
                "device, the one that's",
                "supposed to mess with",
                "Kiehl's power supply!",
                "Hurry, use it right now!"
            ],
        )?;
        ctx.next()?;
        ctx.call(
            Function::MapAnnounce,
            vec![
                Val::from("kh_kiehl02"),
                Val::from("*Gzzzz~ Gzzzz~*"),
                ctx.constant("BC_MAP")?,
                Val::from("0xFF0000"),
            ],
        )?;
        ctx.lines_as(
            "Kiehl",
            args![
                "Wh-what? I c-can't",
                "move! This day is just",
                "full of surprises. Oh, well.",
                "I guess it's time for me to",
                "use my other trump card."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Mitchell", args!["What...?", "How many trump", "cards do you have?"])?;
        ctx.next()?;
        ctx.lines_as(
            "????",
            args!["I'm so disappointed...", "I can't believe none", "of you thought of this."],
        )?;
        ctx.next()?;
        ctx.lines_as("Mitchell", args!["Who are you...?", "Show yourself!"])?;
        ctx.next()?;
        ctx.lines_as(
            "????",
            args![
                "Please.",
                "Don't insult me.",
                "You know this voice.",
                "It's been talking to",
                "you this entire time~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Mitchell", args!["Impossible...", "How can there", "be two of you...?!"])?;
        ctx.next()?;
        ctx.call(Function::EnableNpc, vec![Val::from("Kiehl#Copy")])?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("kh_kiel01"), Val::from(0)])?;
        ctx.lines_as(
            "Kiehl",
            args![
                "Hahahahaha!",
                "I'm a robot!",
                "I can make extra",
                "bodies, switch brains",
                "with them. It's awfully",
                "convenient, let me tell you."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("kh_kiel03"), Val::from(2)])?;
        ctx.lines_as(
            "Kiehl",
            args![
                "Anyway, I don't mean to show",
                "off, but I suppose I better",
                "reveal to you my final trump",
                "card. First of all, I know all",
                "about you, Ms. Mitchell Layla~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Mitchell", args!["What? How do you", "know my name?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Kiehl",
            args!["Well, I have a few", "spies of my own...", "I'll allow him to explain..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Masked Man",
            args!["Mitchell...", "I'm sorry that", "you had to get", "involved in all this..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Mitchell",
            args![
                "Wolkeus? Wolkeus Kaiser?!",
                "You're the spy?! But you risked",
                "your life to save our president!",
                "No! Oh, God! How can this be",
                "happening?! Everything's just...",
                "This is all crazy! All of it!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Wolkeus",
            args![
                "This is just the result",
                "of elaborate plans that",
                "were made years ago. I didn't",
                "expect you to be this surprised,",
                "Mitchell. It's the way the game",
                "is played. You know that."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Mitchell", args!["Mister President...", "I failed you... I'm sorry..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Kiehl",
            args![
                "Well, Kaiser, she took",
                "it pretty badly, but at least",
                "you're being gentlemanly",
                "about it. Well, I'd like for",
                "all of us to get better",
                "acquainted, but..."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("kh_kiel01"), Val::from(2)])?;
        ctx.lines_as(
            "Kiehl",
            args![
                "We'd better say our",
                "farewells here. This",
                "place will be gone in",
                "five minutes. Ah, and",
                "Ms. Layla, you're coming",
                "with us. We have questions~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kiehl",
            args![
                "I'm curious as to what",
                "the president's plans are.",
                "Mister Kaiser, if you'll",
                "escort Ms. Layla, please..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Wolkeus Kaiser",
            args!["...............................", "Sorry, Mitchell.", "I gotta do it."],
        )?;
        ctx.next()?;
        ctx.lines_as("Mitchell", args!["No, get away! Let me go!", "Let go of me, Wolkeus!"])?;
        ctx.call(Function::DisableNpc, vec![Val::from("Mitchell#KiehlRoom")])?;
        ctx.call(Function::DisableNpc, vec![Val::from("Agent#KHAgent1")])?;
        ctx.call(Function::DisableNpc, vec![Val::from("Agent#KHAgent2")])?;
        ctx.call(Function::DisableNpc, vec![Val::from("Agent#KHAgent3")])?;
        ctx.call(Function::DisableNpc, vec![Val::from("Agent#KHAgent4")])?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("kh_kiel02"), Val::from(2)])?;
        ctx.lines_as(
            "Kiehl",
            args![
                "Great, we're done with",
                "that ugly business. Now,",
                "where was I? Ah, right.",
                "Yes. I'm sorry. We don't",
                "have any more time to play."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kiehl",
            args![
                "Here, adventurer.",
                "I'm aware that my father",
                "sent you here to get this.",
                "Consider it my final gift",
                "to him. I'm surprised he left",
                "this ring inside me, though..."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("kh_kiel04"), Val::from(2)])?;
        ctx.lines_as(
            "Kiehl",
            args![
                "I imagine that it must",
                "be precious to him. But",
                "I wonder why he placed",
                "it inside me? Well, anyway,",
                "I have a message I'd like",
                "for you to deliver to him."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("kh_kiel01"), Val::from(2)])?;
        ctx.lines_as(
            "Kiehl",
            args![
                "First... I guess we",
                "should get rid of this",
                "old thing. It was a good",
                "body, and it's served me",
                "well for 23 years. I'll miss",
                "it. Rest well, old Kiehl."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_DEVIL")?])?;
        ctx.next()?;
        ctx.lines_as(
            "Kiehl",
            args![
                "Now, this was the body",
                "that my father made.",
                "Please tell him that",
                "this means that we're",
                "no longer related to",
                "each other at all."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kiehl",
            args![
                "The body I'm using right now?",
                "I made it myself with the most",
                "advanced technology. Consider",
                "it... a Fourth Generation robot",
                "body if you will. Father will",
                "understand what I mean."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kiehl",
            args![
                "Anyway, please tell him",
                "that, and get his old ring",
                "out of my old robot body,",
                "and then give it to him. For",
                "now, let's get out of here: we",
                "just have 3 minutes to evacuate."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kiehl",
            args![
                "You're a worthy opponent,",
                "and a human I respect.",
                "I don't know if we'll meet",
                "again, but who knows?",
                "Anyway, I'll open up the",
                "exit for you. Farewell~"
            ],
        )?;
        ctx.call(Function::DelItem, vec![Val::from(7504), Val::from(1)])?;
        ctx.var("kielhyrequest").set(Val::from(104))?;
        ctx.call(Function::DisableNpc, vec![Val::from("Kiehl#Copy")])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("Kiehl_Room_Exit::OnEnable")])?;
        ctx.call(Function::EnableNpc, vec![Val::from("Kiehl_Room_Exit")])?;
        ctx.call(Function::InitNpcTimer, vec![])?;
        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("kielhyrequest").get()? == 104 {
        ctx.call(Function::Cutin, vec![Val::from("kh_kiel02"), Val::from(2)])?;
        ctx.lines(args![
            "^3355FFYou retrieve the",
            "ring from the heart of",
            "Kiehl's old robotic body.^000000"
        ])?;
        ctx.call(Function::GetItem, vec![Val::from(7508), Val::from(1)])?;
        ctx.var("kielhyrequest").set(Val::from(106))?;
        ctx.close_window()?;
        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
        return Err(Stop::End);
    } else {
        ctx.call(Function::Cutin, vec![Val::from("kh_kiel02"), Val::from(2)])?;
        ctx.lines(args![
            "^3355FFKiehl's old",
            "robotic body",
            "stands alone,",
            "lifeless and silent.^000000"
        ])?;
        ctx.close_window()?;
        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
        return Err(Stop::End);
    }
}

pub fn kiehl_original(ctx: &Ctx) -> Script {
    kiehl_original_body(ctx, Vec::new()).map(|_| ())
}

fn kiehl_original_ontimer180000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("kh_kiehl02"),
            Val::from("Beeeeeeeeeeeeep~"),
            ctx.constant("BC_NPC")?,
            Val::from(16711680),
        ],
    )?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("KiehlRoom::OnReset")])?;
    return Err(Stop::End);
}

pub fn kiehl_original_ontimer180000(ctx: &Ctx) -> Script {
    kiehl_original_ontimer180000_body(ctx, Vec::new()).map(|_| ())
}

fn kiehl_original_ontimer179000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("kh_kiehl02"),
            Val::from("1 second remaining until detonation"),
            ctx.constant("BC_NPC")?,
            Val::from(16711680),
        ],
    )?;
    return Err(Stop::End);
}

pub fn kiehl_original_ontimer179000(ctx: &Ctx) -> Script {
    kiehl_original_ontimer179000_body(ctx, Vec::new()).map(|_| ())
}

fn kiehl_original_ontimer178000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("kh_kiehl02"),
            Val::from("2 seconds remaining until detonation"),
            ctx.constant("BC_NPC")?,
            Val::from(16711680),
        ],
    )?;
    return Err(Stop::End);
}

pub fn kiehl_original_ontimer178000(ctx: &Ctx) -> Script {
    kiehl_original_ontimer178000_body(ctx, Vec::new()).map(|_| ())
}

fn kiehl_original_ontimer177000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("kh_kiehl02"),
            Val::from("3 seconds remaining until detonation"),
            ctx.constant("BC_NPC")?,
            Val::from(16711680),
        ],
    )?;
    return Err(Stop::End);
}

pub fn kiehl_original_ontimer177000(ctx: &Ctx) -> Script {
    kiehl_original_ontimer177000_body(ctx, Vec::new()).map(|_| ())
}

fn kiehl_original_ontimer176000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("kh_kiehl02"),
            Val::from("4 seconds remaining until detonation"),
            ctx.constant("BC_NPC")?,
            Val::from(16711680),
        ],
    )?;
    return Err(Stop::End);
}

pub fn kiehl_original_ontimer176000(ctx: &Ctx) -> Script {
    kiehl_original_ontimer176000_body(ctx, Vec::new()).map(|_| ())
}

fn kiehl_original_ontimer175000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("kh_kiehl02"),
            Val::from("5 seconds remaining until detonation"),
            ctx.constant("BC_NPC")?,
            Val::from(16711680),
        ],
    )?;
    return Err(Stop::End);
}

pub fn kiehl_original_ontimer175000(ctx: &Ctx) -> Script {
    kiehl_original_ontimer175000_body(ctx, Vec::new()).map(|_| ())
}

fn kiehl_original_ontimer170000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("kh_kiehl02"),
            Val::from("10 seconds remaining until detonation"),
            ctx.constant("BC_NPC")?,
            Val::from(16711680),
        ],
    )?;
    return Err(Stop::End);
}

pub fn kiehl_original_ontimer170000(ctx: &Ctx) -> Script {
    kiehl_original_ontimer170000_body(ctx, Vec::new()).map(|_| ())
}

fn kiehl_original_ontimer160000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("kh_kiehl02"),
            Val::from("20 seconds remaining until detonation"),
            ctx.constant("BC_NPC")?,
            Val::from(16711680),
        ],
    )?;
    return Err(Stop::End);
}

pub fn kiehl_original_ontimer160000(ctx: &Ctx) -> Script {
    kiehl_original_ontimer160000_body(ctx, Vec::new()).map(|_| ())
}

fn kiehl_original_ontimer150000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("kh_kiehl02"),
            Val::from("30 seconds remaining until detonation"),
            ctx.constant("BC_NPC")?,
            Val::from(16711680),
        ],
    )?;
    return Err(Stop::End);
}

pub fn kiehl_original_ontimer150000(ctx: &Ctx) -> Script {
    kiehl_original_ontimer150000_body(ctx, Vec::new()).map(|_| ())
}

fn kiehl_original_ontimer120000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("kh_kiehl02"),
            Val::from("1 minute remaining until detonation."),
            ctx.constant("BC_NPC")?,
            Val::from(16711680),
        ],
    )?;
    return Err(Stop::End);
}

pub fn kiehl_original_ontimer120000(ctx: &Ctx) -> Script {
    kiehl_original_ontimer120000_body(ctx, Vec::new()).map(|_| ())
}

fn kiehl_original_ontimer60000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("kh_kiehl02"),
            Val::from("2 minutes remaining until detonation."),
            ctx.constant("BC_NPC")?,
            Val::from(16711680),
        ],
    )?;
    return Err(Stop::End);
}

pub fn kiehl_original_ontimer60000(ctx: &Ctx) -> Script {
    kiehl_original_ontimer60000_body(ctx, Vec::new()).map(|_| ())
}

fn kiehl_original_ontimer1000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("kh_kiehl02"),
            Val::from("3 minutes remaining until detonation."),
            ctx.constant("BC_NPC")?,
            Val::from(16711680),
        ],
    )?;
    return Err(Stop::End);
}

pub fn kiehl_original_ontimer1000(ctx: &Ctx) -> Script {
    kiehl_original_ontimer1000_body(ctx, Vec::new()).map(|_| ())
}

fn kiehlroom_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn kiehlroom(ctx: &Ctx) -> Script {
    kiehlroom_body(ctx, Vec::new()).map(|_| ())
}

fn kiehlroom_onkiehlmobdead_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.var(".khkilled").set((ctx.var(".khkilled").get()? + Val::from(1)))?;
    if ctx.var(".khkilled").get()? == 5 {
        ctx.call(Function::EnableNpc, vec![Val::from("Kiehl#Original")])?;
    }
    return Err(Stop::End);
}

pub fn kiehlroom_onkiehlmobdead(ctx: &Ctx) -> Script {
    kiehlroom_onkiehlmobdead_body(ctx, Vec::new()).map(|_| ())
}

fn kiehlroom_onkiehldead_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.var(".khkilledboss").set(Val::from(1))?;
    ctx.call(Function::EnableNpc, vec![Val::from("Kiehl#Original")])?;
    return Err(Stop::End);
}

pub fn kiehlroom_onkiehldead(ctx: &Ctx) -> Script {
    kiehlroom_onkiehldead_body(ctx, Vec::new()).map(|_| ())
}

fn kiehlroom_onreset_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DoNpcEvent, vec![Val::from("Kiehl_Room_Trap::OnGlobalTimerOff")])?;
    if ctx.call(Function::GetMapUsers, vec![Val::from("kh_kiehl02")])?.number()? > 0 {
        ctx.call(
            Function::MapWarp,
            vec![Val::from("kh_kiehl02"), Val::from("lighthalzen"), Val::from(192), Val::from(200)],
        )?;
    }
    ctx.call(Function::KillMonster, vec![Val::from("kh_kiehl02"), Val::from("All")])?;
    ctx.call(Function::DisableNpc, vec![Val::from("Kiehl_Room_Exit")])?;
    ctx.call(Function::DisableNpc, vec![Val::from("Mitchell#KiehlRoom")])?;
    ctx.call(Function::DisableNpc, vec![Val::from("Agent#KHAgent1")])?;
    ctx.call(Function::DisableNpc, vec![Val::from("Agent#KHAgent2")])?;
    ctx.call(Function::DisableNpc, vec![Val::from("Agent#KHAgent3")])?;
    ctx.call(Function::DisableNpc, vec![Val::from("Agent#KHAgent4")])?;
    ctx.call(Function::DisableNpc, vec![Val::from("Kiehl#Copy")])?;
    ctx.call(Function::EnableNpc, vec![Val::from("Kiehl#Original")])?;
    ctx.var(".khkilledboss").set(Val::from(0))?;
    ctx.var(".khkilled").set(Val::from(0))?;
    ctx.call(
        Function::SetVariableOfNpc,
        vec![Val::from(".khtrapsprung"), Val::from("Kiehl_Room_Trap"), Val::from(0), Val::from(0)],
    )?;
    ctx.var("$@khquestbusy").set(Val::from(0))?;
    return Err(Stop::End);
}

pub fn kiehlroom_onreset(ctx: &Ctx) -> Script {
    kiehlroom_onreset_body(ctx, Vec::new()).map(|_| ())
}

fn mitchell_kiehlroom_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn mitchell_kiehlroom(ctx: &Ctx) -> Script {
    mitchell_kiehlroom_body(ctx, Vec::new()).map(|_| ())
}

fn mitchell_kiehlroom_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![ctx.call(Function::StrNpcInfo, vec![Val::from(0)])?])?;
    return Err(Stop::End);
}

pub fn mitchell_kiehlroom_oninit(ctx: &Ctx) -> Script {
    mitchell_kiehlroom_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn kiehl_copy_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn kiehl_copy(ctx: &Ctx) -> Script {
    kiehl_copy_body(ctx, Vec::new()).map(|_| ())
}

fn kiehl_copy_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Kiehl#Copy")])?;
    return Err(Stop::End);
}

pub fn kiehl_copy_oninit(ctx: &Ctx) -> Script {
    kiehl_copy_oninit_body(ctx, Vec::new()).map(|_| ())
}

pub fn kiehl_room_exit(ctx: &Ctx) -> Script {
    kiehl_room_exit_run(ctx, KiehlRoomExitStep::Start, Vec::new()).map(|_| ())
}

pub fn kiehl_room_exit_ontouch(ctx: &Ctx) -> Script {
    kiehl_room_exit_run(ctx, KiehlRoomExitStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn kiehl_room_exit_onenable(ctx: &Ctx) -> Script {
    kiehl_room_exit_run(ctx, KiehlRoomExitStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn kiehl_room_exit_oninit(ctx: &Ctx) -> Script {
    kiehl_room_exit_run(ctx, KiehlRoomExitStep::OnInit, Vec::new()).map(|_| ())
}

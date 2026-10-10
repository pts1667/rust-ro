#![allow(
    unused_imports,
    unused_labels,
    unused_assignments,
    unused_mut,
    unused_parens,
    unused_variables,
    unreachable_code
)]

use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, constants, runtime};

pub fn arpesto(ctx: &Ctx) -> Script {
    if ctx.var("BaseJob").get()? == constants::JOB_HUNTER {
        if ctx.var("qskill_hunter").get()? == 100 {
            if ctx.call(Function::GetSkillLv, args!["HT_PHANTASMIC"])?.is_true() {
                ctx.lines_as(
                    "Arpesto",
                    args![
                        "Hm, I'm sorry if I made",
                        "a big scene when you grazed",
                        "me with that attack accidentally. It's just that I was training so",
                        "diligently and blood sugar was incredibly low and the fatigue..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Reidin Corse",
                    args!["Hah! Admit it,", "old man! You were", "just scared! Me, on", "the other hand..."],
                )?;
                ctx.call(
                    Function::Emotion,
                    args![constants::ET_KIK, ctx.call(Function::GetNpcId, args![0, "Reidin Corse#tu"])?],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Arpesto",
                    args!["You little", "whippersnapper!", "I don't know what", "you're talking about!"],
                )?;
                return ctx.close();
            }
            ctx.lines_as(
                "Arpesto",
                args![
                    "I recognize you",
                    "from somewhere,",
                    "I think. Reidin, do you",
                    "remember this guy?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Reidin Corse",
                args![
                    "Oh come on, we see",
                    "hundreds of Archers and",
                    "Hunters and Snipers and...",
                    "I can't remember them all.",
                    "But I guess this guy's up",
                    "to snuff. Trust me on this!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Arpesto",
                args![
                    "Alright, alright.",
                    "If Reidin thinks you're",
                    "worthy, then you're probably",
                    "are. In that case, I'd like to",
                    "offer to teach you my secret",
                    "skill, ''Phantasmic Arrow.''"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Reidin Corse",
                args![
                    "Wait, wait. This guy",
                    "only needs a refresher.",
                    "Yeah, I'm pretty sure you",
                    "were here to learn this",
                    "skill before. Yeah, I've",
                    "got a gut feeling about it."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Arpesto",
                args![
                    "Damn. Only a refresher?",
                    "That means you'll learn",
                    "this free of charge. Alright...",
                    "You should be able to use the",
                    "Phantasmic Arrow skill now.",
                    "Travel safely now, you hear?"
                ],
            )?;
            ctx.call(Function::Skill, args!["HT_PHANTASMIC", 1, constants::SKILL_PERM])?;
            return ctx.close();
        } else if ctx.var("qskill_hunter").get()? == 1 {
            if ctx.items().count(724)? > 4 && ctx.items().count(7115)? > 4 && ctx.items().count(537)? > 29 {
                ctx.lines_as(
                    "Arpesto",
                    args![
                        "Hm? You're back?",
                        "Oh, did you bring",
                        "everything? Ah, you did.",
                        "Good! Now I shall reveal",
                        "my super secret skill...",
                        "The 1st Arpesto Form!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Reidin Corse",
                    args![
                        "Wait! Wait...",
                        "Didn't you call it",
                        "Arpesto's 3rd Form",
                        "or something at first?"
                    ],
                )?;
                ctx.call(
                    Function::Emotion,
                    args![constants::ET_SWEAT, ctx.call(Function::GetNpcId, args![0, "Reidin Corse#tu"])?],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Arpesto",
                    args![
                        "Er, yes, Arpesto's",
                        "3rd Form, that's right.",
                        "It's just there's just so",
                        "many of them, that... ",
                        "Anyway, let me teach you",
                        "the ''Emergency Arrow'' skill."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Reidin Corse",
                    args![
                        "Whoa, hold on!",
                        "''Emergency Arrow?!''",
                        "That's the old name you",
                        "gave that skill, right?",
                        "Didn't you change it?"
                    ],
                )?;
                ctx.npc().emotion(constants::ET_HUK)?;
                ctx.next()?;
                ctx.npc().emotion(constants::ET_HUK)?;
                ctx.lines_as("Arpesto", args!["What the hell", "are you talkin--"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Arpesto",
                    args![
                        "Oh, you're right.",
                        "We agreed that name",
                        "was too cheesy. What",
                        "did we call it now? Um...",
                        "''Phantasmic Arrow?''"
                    ],
                )?;
                ctx.npc().emotion(constants::ET_THINK)?;
                ctx.next()?;
                ctx.lines_as("Arpesto", args!["...", "......"])?;
                ctx.npc().emotion(constants::ET_CRY)?;
                ctx.next()?;
                ctx.lines_as("Reidin Corse", args!["Umm....", "Ummm........"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Reidin Corse",
                    args![
                        "Yeah, that was the one!",
                        "''^3131FFPhantasmic Arrow^000000'' sounds",
                        "soooo much cooler than that",
                        "other name you came up with!"
                    ],
                )?;
                ctx.call(
                    Function::Emotion,
                    args![constants::ET_AHA, ctx.call(Function::GetNpcId, args![0, "Reidin Corse#tu"])?],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Arpesto",
                    args![
                        "You can be pretty handy",
                        "sometimes, Reidin. Now,",
                        "back to the matter at hand.",
                        "I shall teach you my awesome",
                        "skill, Phantasmic Arrow. Now...",
                        "Ready your mind and body!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Arpesto", args!["Phan-!", "Ta-!", "Ssssss-!", "Mic-!"])?;
                ctx.next()?;
                ctx.lines_as("Arpesto", args!["ARRRRRRRROOOOOOW~!"])?;
                ctx.npc().special_effect(constants::EF_FLASHER)?;
                ctx.fx().special_effect(constants::EF_FLASHER)?;
                ctx.next()?;
                ctx.lines_as(
                    "Arpesto",
                    args![
                        "^333333*Pant Pant*^000000",
                        "Alright, I used a special",
                        "technique to teach you the",
                        "skill-- directly into your brain. You should be able to use it",
                        "now. Why don't give it a try?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines(args!["^3355FF*Swwwwwww!*", "*Bang!*^000000"])?;
                ctx.npc().special_effect(constants::EF_HIT2)?;
                ctx.npc().emotion(constants::ET_HUK)?;
                ctx.call(
                    Function::Emotion,
                    args![constants::ET_HUK, ctx.call(Function::GetCharacterId, args![0])?.is_true()],
                )?;
                ctx.next()?;
                ctx.lines_as("Arpesto", args!["^333333*Pant Pant*", "*Cough Cough*", "*Cough Cough*^000000"])?;
                ctx.next()?;
                ctx.lines_as("Arpesto", args!["WHHHHHHHY MEEEEE?!"])?;
                ctx.call(
                    Function::Emotion,
                    args![constants::ET_HUK, ctx.call(Function::GetNpcId, args![0, "Reidin Corse#tu"])?],
                )?;
                ctx.next()?;
                ctx.mes("^3355FF*Thump*^000000")?;
                ctx.next()?;
                ctx.lines_as(
                    "Reidin Corse",
                    args![
                        "Ar-Arpesto, no!",
                        "...........................",
                        "Oh. Oh, come on! The arrow",
                        "just glanced you! You'll be",
                        "alright. Quit being a baby."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Reidin Corse",
                    args![
                        "Eh, don't worry about",
                        "Arpesto. He's just being",
                        "a crazy old man. I'll take",
                        "care of him, so just make",
                        "sure you make good use",
                        "of that new skill. Take care~"
                    ],
                )?;
                ctx.items().take(724, 5)?;
                ctx.items().take(7115, 5)?;
                ctx.items().take(537, 30)?;
                ctx.var("qskill_hunter").set(Val::from(100))?;
                ctx.call(Function::Skill, args!["HT_PHANTASMIC", 1, constants::SKILL_PERM])?;
                return ctx.close();
            }
            ctx.lines_as(
                "Arpesto",
                args![
                    "Hurry up and bring",
                    "^3131FF5 Cursed Rubies^000000,",
                    "^3131FF30 Pet Foods^000000 and",
                    "^3131FF5 Harpy Feathers^000000.",
                    "Then, I can teach you",
                    "the Emergency Arrow skill."
                ],
            )?;
            return ctx.close();
        } else {
            if ctx.call(Function::GetSkillLv, args!["HT_PHANTASMIC"])?.is_true() {
                ctx.lines_as(
                    "Arpesto",
                    args![
                        "Ah, very nice.",
                        "Um, as you demonstrated",
                        "earlier, my teaching technique",
                        "was perfect. But next time, use",
                        "the skill on foes that deserve",
                        "to be beaten, alright? Good."
                    ],
                )?;
                ctx.var("qskill_hunter").set(Val::from(100))?;
                return ctx.close();
            }
            if ctx.var("BaseJob").get()? == constants::JOB_HUNTER && ctx.player().job_level()? < 40 {
                ctx.lines_as(
                    "Arpesto",
                    args![
                        "Did the master send you",
                        "to me? Hmm... You're still",
                        "pretty green from the looks",
                        "of it. Yeah, I don't think there's anything I can teach quite yet."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Arpesto",
                    args![
                        "Train a little more and get",
                        "some more experience in",
                        "your job. When you become",
                        "more adept at hunting, you'll",
                        "be able to grasp what I've been",
                        "teaching Hunters and Snipers."
                    ],
                )?;
                return ctx.close();
            }
            ctx.lines_as(
                "Arpesto",
                args![
                    "Has the master sent you?",
                    "Ah, you definitely have the",
                    "keen, vulture like eyes of a",
                    "true Hunter. You should be",
                    "capable of learning my secret",
                    "skill, the 3rd Arpesto Form..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Arpesto",
                args![
                    "It's a handy skill that",
                    "I developed while battling",
                    "Medusas and running out",
                    "of arrows. I simply call it,",
                    "''^3131FFEmergency Arrow^000000.'' Would you",
                    "be interested in learning it?"
                ],
            )?;
            ctx.next()?;
            match ctx.menu(&["Yes, please!", "Ummm..."])? {
                0 => {
                    ctx.lines_as(
                        "Arpesto",
                        args![
                            "Great, great. Of course,",
                            "you can't resist this offer",
                            "if you recognize this skill's",
                            "value. However, I'd like to",
                            "ask for a little tuition in return for me teaching this to you."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Arpesto",
                        args![
                            "Although I feel guilt at",
                            "asking for payment from",
                            "a fellow Hunter, creating",
                            "this skill required much",
                            "sacrifice and unimaginable",
                            "bloodshed on my part..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Reidin Corse",
                        args![
                            "Hey! You liar!",
                            "I was there when you",
                            "accidently made up that",
                            "skill when you were bored",
                            "and just messing around",
                            "with one of the Bows!"
                        ],
                    )?;
                    ctx.call(
                        Function::Emotion,
                        args![constants::ET_KIK, ctx.call(Function::GetNpcId, args![0, "Reidin Corse#tu"])?],
                    )?;
                    ctx.next()?;
                    ctx.npc().emotion(constants::ET_HUK)?;
                    ctx.lines_as(
                        "Arpesto",
                        args![
                            "^333333*Ahem*^000000 Please Reidin,",
                            "don't entertain unfounded",
                            "rumors. It was only after",
                            "countless battles with Eddga",
                            "that I managed to invent and",
                            "perfect Emergency Arrow."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Reidin Corse",
                        args![
                            "^333333*Tsk tsk*^000000 Huh.",
                            "I thought you said",
                            "you developed it while",
                            "you were fighting Medusas."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Arpesto",
                        args![
                            "A-anyway, please bring",
                            "me the following items",
                            "as tuition in exchange for",
                            "me teaching you this skill."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Arpesto",
                        args![
                            "Please bring me",
                            "^3131FF5 Cursed Rubies^000000,",
                            "^3131FF30 Pet Foods^000000 and",
                            "^3131FF5 Harpy Feathers^000000.",
                            "Then I can teach",
                            "you this skill."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Reidin Corse",
                        args!["30 Pet Foods?", "What the heck do", "you need all that for?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Arpesto", args!["I...", "I'm going to", "use it to feed", "my pet Falcon."])?;
                    ctx.next()?;
                    ctx.lines_as("Reidin Corse", args!["...", "......", ".........", "......Riiiight."])?;
                    ctx.call(
                        Function::Emotion,
                        args![constants::ET_THINK, ctx.call(Function::GetNpcId, args![0, "Reidin Corse#tu"])?],
                    )?;
                    ctx.var("qskill_hunter").set(Val::from(1))?;
                    return ctx.close();
                }
                1 => {
                    ctx.lines_as(
                        "Arpesto",
                        args![
                            "Alright, it's your",
                            "decision. However, I'm",
                            "sure that you may have",
                            "great need of my skill",
                            "sooner or later, so return",
                            "to me if you change your mind."
                        ],
                    )?;
                    return ctx.close();
                }
                _ => {}
            }
        }
    } else {
        ctx.lines_as("Arpesto", args!["The sky still looks clear, but recently the winds that have been blowing through the land seem to carry with them an air of misfortune."])?;
        ctx.next()?;
        ctx.call(
            Function::Emotion,
            args![constants::ET_HNG, ctx.call(Function::GetNpcId, args![0, "Reidin Corse#tu"])?],
        )?;
        ctx.lines_as(
            "Reidin Corse",
            args!["So...", "Danger's coming?", "Heh! Chill, gramps~", "I got it covered!"],
        )?;
        ctx.next()?;
        ctx.npc().emotion(constants::ET_PROFUSELY_SWEAT)?;
        ctx.lines_as("Arpesto", args!["Ho ho...", "Energetic, but", "cocky. Kids nowadays..."])?;
        return ctx.close();
    }
    Ok(())
}

#![allow(
    unused_imports,
    unused_labels,
    unused_assignments,
    unused_mut,
    unused_parens,
    unused_variables,
    unreachable_code
)]

use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, constants};

pub fn apprentice_monk_qsk_mo(ctx: &Ctx) -> Script {
    if ctx.var("BaseJob").get()? == constants::JOB_MONK {
        if ctx.var("monk_sk").get()? == 7 {
            ctx.lines_as(
                "Monk",
                args![
                    "To take the time to",
                    "comtemplate on your most",
                    "important goal, your highest",
                    "priority, is never a waste.",
                    "Never confuse your means",
                    "to the end you wish to achieve."
                ],
            )?;
            return ctx.close();
        } else if ctx.var("monk_sk").get()? == 6 && ctx.call(Function::GetSkillLv, args!["MO_KITRANSLATION"])? == 0 {
            ctx.lines_as(
                "Monk",
                args![
                    "You've forgotten the",
                    "basics behind performing",
                    "Spiritual Bestowment?",
                    "How can this be...?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Monk",
                args![
                    "Ah, now I understand.",
                    "You've transcended the",
                    "limitations that used to",
                    "hold you back, and have",
                    "become a Champion. I'm",
                    "sorry for underestimating you."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Monk",
                args![
                    "Very well. It will be an honor",
                    "for me to help you relearn the",
                    "Spiritual Bestowment skill. As",
                    "a side effect, you will probably relearn Excruciating Palm as well."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Monk",
                args![
                    "Now be still and relax",
                    "as I perform my special",
                    "accupressure treatment that",
                    "will commit these skills to",
                    "your nervous system and ",
                    "motor memory..."
                ],
            )?;
            ctx.next()?;
            ctx.lines(args![
                "^3355FF*Tap-tap-tap-tap-tap-tap-tap-tap- tap-tap-tap-tap-tap-tap-tap-tap-",
                "tap-tap-tap-tap-tap-tap-tap-tap- tap-tap-tap-tap-tap-tap-tap-tap-",
                "tap-tap-tap-tap-tap-tap-tap-tap- tap-tap-tap-tap-tap-tap-tap-tap-",
                "tap-tap-tap-tap-tap-tap* *POKE*^000000"
            ])?;
            ctx.var("monk_sk").set(Val::from(7))?;
            ctx.call(Function::Skill, args!["MO_KITRANSLATION", 1, constants::SKILL_PERM])?;
            ctx.call(Function::Skill, args!["MO_BALKYOUNG", 1, constants::SKILL_PERM])?;
            ctx.next()?;
            ctx.lines_as(
                "Monk",
                args![
                    "It is done. Please",
                    "continue to use these",
                    "special skills to promote",
                    "peace and harmony in this",
                    "world. Farewell, my friend."
                ],
            )?;
            return ctx.close();
        } else if ctx.var("monk_sk").get()?.number()? > 2 {
            ctx.lines_as(
                "Monk",
                args![
                    "To take the time to",
                    "comtemplate on your most",
                    "important goal, your highest",
                    "priority, is never a waste.",
                    "Never confuse your means",
                    "to the end you wish to achieve."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Monk",
                args![
                    "Ah, I have a favor to ask.",
                    "Please do not tell others that",
                    "I have taught you Spiritual",
                    "Bestowment. I don't wish to",
                    "spend my time teaching too",
                    "many people this skill..."
                ],
            )?;
            return ctx.close();
        } else if ctx.var("monk_sk").get()? == 2 {
            if ctx.var("Weight").get()? == 0 {
                ctx.lines_as(
                    "Monk",
                    args![
                        "Ah. Well done. I see",
                        "that you managed to learn",
                        "the most important thing,",
                        "selflessness, on your own.",
                        "Now I trust that you won't",
                        "abuse what I have to teach you."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Monk",
                    args![
                        "My test may seem too strict,",
                        "but it is impossible to learn",
                        "Spiritual Bestowment without the right discipline. As with all",
                        "things, readiness and preparation precede all forms of realization."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Monk",
                    args![
                        "Now just relax for a moment.",
                        "I will use accupressure to",
                        "train your nervous system to",
                        "physically memorize the energy",
                        "flow and movements required",
                        "for Spiritual Bestowment."
                    ],
                )?;
                ctx.next()?;
                ctx.lines(args!["^3355FF*Tap... tap*", "*Tap... Tap*", "*Tap tap tap*^000000"])?;
                ctx.next()?;
                ctx.lines(args!["^3355FF*Tap tap tap*", "*Tap tap tap*", "*Tap... Tap tap*^000000"])?;
                ctx.next()?;
                ctx.lines(args![
                    "^3355FF*Tap-tap-tap-tap-tap-tap-tap-tap- tap-tap-tap-tap-tap-tap-tap-tap-",
                    "tap-tap-tap-tap-tap-tap-tap-tap- tap-tap-tap-tap-tap-tap-tap-tap-",
                    "tap-tap-tap-tap-tap-tap-tap-tap- tap-tap-tap-tap-tap-tap-tap-tap-",
                    "tap-tap-tap-tap-tap-tap* *POKE*^000000"
                ])?;
                ctx.next()?;
                ctx.lines_as(
                    "Monk",
                    args![
                        "It is done. You are now",
                        "capable of using Spiritual",
                        "Bestowment. Ah, yes. I've never",
                        "really named this skill, but that is what it is usually called by",
                        "other Monks who have seen it."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Monk",
                    args![
                        "Remember that this skill has a",
                        "long Cast Time and Skill Delay,",
                        "so use it very carefully. This",
                        "skill will let you give a Spirit Sphere to one of your Party",
                        "Members by consuming 40 SP."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Monk",
                    args![
                        "You will better understand",
                        "the use of this skill if you",
                        "practice. Keep in mind that",
                        "fighting may be necessary at",
                        "times, but it is a means to an",
                        "end, and not an end in itself."
                    ],
                )?;
                ctx.var("monk_sk").set(Val::from(3))?;
                ctx.call(Function::Skill, args!["MO_KITRANSLATION", 1, constants::SKILL_PERM])?;
                ctx.next()?;
                ctx.lines_as(
                    "Monk",
                    args![
                        "Our skills should not be",
                        "used to shed blood. If you",
                        "can transcend conflict, you",
                        "will know that fighting has",
                        "no worth, no value. Peace",
                        "be with you, my friend."
                    ],
                )?;
                return ctx.close();
            } else {
                ctx.lines_as(
                    "Monk",
                    args![
                        "Have you learned the",
                        "meaning of selflessness?",
                        "When you abandon greed",
                        "and the desires of the world,",
                        "you will be ready for my teaching."
                    ],
                )?;
                return ctx.close();
            }
        } else if ctx.var("monk_sk").get()? == 1 {
            ctx.lines_as(
                "Monk",
                args![
                    "Fighting is meaningless.",
                    "Transcend conflict and",
                    "difference through the",
                    "realization that all are",
                    "one. Discipline your mind",
                    "and the truth will be revealed."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Monk",
                args![
                    "Hm? To what purpose have",
                    "you come to visit me? Ah, so",
                    "you wish to learn the skill that I have developed. My colleague",
                    "was right: more and more people have been wanting to learn this."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Monk",
                args![
                    "Do not misunderstand me.",
                    "It is true that power and",
                    "justice, wisdom and action",
                    "must be balanced. However,",
                    "I have been visited by too many power hungry, bloodthirsty people."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Monk",
                args![
                    "In all fairness, I shall",
                    "give you a chance. Prove",
                    "to me that you would use",
                    "the skills I teach you for",
                    "peace and justice. Prove that",
                    "you are free from selfishness."
                ],
            )?;
            ctx.var("monk_sk").set(Val::from(2))?;
            return ctx.close();
        }
    }
    ctx.lines_as(
        "Monk",
        args![
            "Fighting is meaningless.",
            "Transcend conflict and",
            "difference through the",
            "realization that all are",
            "one. Discipline your mind",
            "and the truth will be revealed."
        ],
    )?;
    ctx.close()
}

pub fn monk_qsk_mo(ctx: &Ctx) -> Script {
    if ctx.var("BaseJob").get()? == constants::JOB_MONK {
        if ctx.var("monk_sk").get()? == 6 {
            ctx.lines_as(
                "Monk",
                args![
                    "How did you convince",
                    "that monk to teach you",
                    "Spiritual Bestowment? It'd",
                    "be nice to popularize both",
                    "of our specialty skills, but the world may not be ready yet."
                ],
            )?;
            return ctx.close();
        } else if ctx.var("monk_sk").get()? == 5 {
            ctx.lines_as(
                "Monk",
                args![
                    "To begin, Excruciating Palm",
                    "is performed by taking your",
                    "inner strength and making it",
                    "into outer strength. Then, you",
                    "focus all of that into your palm. You'll explode if you do it wrong."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Monk",
                args![
                    "This skill truly causes",
                    "excruciating pain, consuming",
                    "20 SP and even 10 HP. It will",
                    "knock back monsters with the",
                    "chance of stunning them, but only the targeted monster is damaged."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Monk",
                args![
                    "If you use this skill on",
                    "other players, it will cause",
                    "a certain amount of damage,",
                    "but it won't knock back other",
                    "players near the original target. Does that make sense?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Monk",
                args![
                    "Anyway, keep practicing",
                    "these principles and you'll",
                    "master Excruciating Palm.",
                    "You should definitely be able",
                    "to do it since you've learned",
                    "Spiritual Endowment."
                ],
            )?;
            ctx.var("monk_sk").set(Val::from(6))?;
            ctx.call(Function::Skill, args!["MO_BALKYOUNG", 1, constants::SKILL_PERM])?;
            ctx.next()?;
            ctx.lines_as(
                "Monk",
                args![
                    "Well, we've completed",
                    "the lesson. I hope you",
                    "use your new skills to",
                    "fight for justice and",
                    "combat evil. Goodbye",
                    "for now, my friend."
                ],
            )?;
            return ctx.close();
        } else if ctx.var("monk_sk").get()? == 4 && ctx.items().count(905)? > 19 && ctx.items().count(711)? > 2 {
            ctx.lines_as(
                "Monk",
                args![
                    "What's that...?",
                    "You've learned the",
                    "Spiritual Bestowment",
                    "skill? I assume that you",
                    "wish to learn Excruciating",
                    "Palm next. Am I correct?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Monk",
                args![
                    "Hm. Well, you are strong",
                    "enough to handily defeat a",
                    "good number of Mandragoras.",
                    "All right. You seem to be ready. Come back in a little while so",
                    "that I can finish preparations."
                ],
            )?;
            ctx.items().take(905, 20)?;
            ctx.items().take(711, 3)?;
            ctx.var("monk_sk").set(Val::from(5))?;
            return ctx.close();
        } else if ctx.var("monk_sk").get()? == 4 {
            ctx.lines_as(
                "Monk",
                args![
                    "What's that...?",
                    "You've learned the",
                    "Spiritual Bestowment",
                    "skill? I assume that you",
                    "wish to learn Excruciating",
                    "Palm next. Am I correct?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Monk",
                args![
                    "As I mentioned to you",
                    "before, you must subjugate",
                    "Mandragoras to prove that",
                    "you are qualified, in terms",
                    "of strength and compassion,",
                    "to learn Excruciating Palm."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Monk",
                args![
                    "Fight to protect those",
                    "innocents terrorized by",
                    "the aggressive Mandragoras.",
                    "If you bring me ^FF000020 Stems^000000 and",
                    "^FF00003 Shoots^000000, I will be satisfied with this proof of your ability."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Monk", args!["Do you understand now?"])?;
            return ctx.close();
        } else if ctx.var("monk_sk").get()? == 3 {
            ctx.lines_as(
                "Monk",
                args![
                    "Have you come to learn",
                    "Excruciating Palm? If only",
                    "that were possible. There",
                    "are two obstacles we face",
                    "in fulfilling your request."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Monk",
                args![
                    "Firstly, you must learn",
                    "the Spiritual Bestowment",
                    "skill beforehand. It will be",
                    "difficult to get that monk",
                    "to teach it to you, if not",
                    "outright impossible."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Monk",
                args![
                    "Secondly, you will need",
                    "to pass my little test of",
                    "strength, so that I can be",
                    "use that your body can endure",
                    "the awesome power involved",
                    "in using Excruciating Palm."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Monk",
                args![
                    "Mm. I know that Mandragoras",
                    "have been menacing travelers,",
                    "so if you can prove to me that",
                    "you've been hunting them, you",
                    "will prove worthy of learning",
                    "Excruciating Palm."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Monk",
                args![
                    "If you can somehow convince",
                    "that monk to teach you how to",
                    "perform Spiritual Bestowment,",
                    "I shall then ask you to bring",
                    "^FF000020 Stems^000000 and ^FF00003 Shoots^000000 from",
                    "hunting Mandragoras."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Monk",
                args![
                    "Of course, completing",
                    "my test is meaningless",
                    "if you do not first learn the",
                    "Spiritual Bestowment skill.",
                    "I wish you luck with that..."
                ],
            )?;
            ctx.var("monk_sk").set(Val::from(4))?;
            return ctx.close();
        } else if ctx.var("monk_sk").get()? == 1 {
            ctx.lines_as(
                "Monk",
                args![
                    "There is a pacifist monk",
                    "living in seclusion that",
                    "can teach the Spiritual",
                    "Bestowment skill. However,",
                    "he is stubborn, and has refused many who wish to learn from him."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Monk",
                args![
                    "It would be good if",
                    "you can convince him",
                    "to teach it to you, as it is",
                    "impossible to learn how",
                    "to perform this skill alone.",
                    "It's really quite a pity..."
                ],
            )?;
            return ctx.close();
        } else if ctx.var("monk_sk").get()? == 0 {
            ctx.lines_as(
                "Monk",
                args![
                    "Power without justice will",
                    "not last. Justice without",
                    "power cannot be upheld.",
                    "Power does not equate to",
                    "justice, but true justice can",
                    "be the means to power."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Monk",
                args![
                    "Always remember this truth.",
                    "Justice and power, compassion",
                    "and strength, wisdom and action. These are all complements that",
                    "go hand in hand and must always",
                    "be harmoniousy balanced."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Monk",
                args![
                    "So it is with some skills.",
                    "I have developed a skill",
                    "that is the pure manifestation",
                    "of power, Excruciating Palm!",
                    "However, it is useless without",
                    "its complement to balance it."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Monk",
                args![
                    "The complement to my skill",
                    "is a skill known as Spiritual",
                    "Bestowment, which can only",
                    "be taught by one Monk. However,",
                    "he refuses to teach it to others, believing they are unworthy."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Monk",
                args![
                    "Although I've let him know",
                    "that he should popularize his",
                    "teaching of that skill, I must respect his decision. Still, you",
                    "may try talking to him if you wish to try to learn some new skills."
                ],
            )?;
            ctx.var("monk_sk").set(Val::from(1))?;
            ctx.next()?;
            ctx.lines_as(
                "Monk",
                args![
                    "To find him, simply seek",
                    "out the monk who always",
                    "preaches the empty meaning",
                    "of fighting. That will be him."
                ],
            )?;
            return ctx.close();
        }
    }
    ctx.lines_as(
        "Monk",
        args![
            "Power without justice will",
            "not last. Justice without",
            "power cannot be upheld.",
            "Power does not equate to",
            "justice, but true justice can",
            "be the means to power."
        ],
    )?;
    ctx.close()
}

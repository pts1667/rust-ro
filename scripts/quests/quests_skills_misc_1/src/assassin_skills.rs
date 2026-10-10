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

pub fn assassin_realman(ctx: &Ctx) -> Script {
    if ctx.var("BaseJob").get()? == constants::JOB_ASSASSIN && ctx.var("assn_sk2").get()? == 1 {
        if ctx.call(Function::GetSkillLv, args!["AS_VENOMKNIFE"])? == 0 {
            ctx.lines_as(
                "Killtin",
                args![
                    "Ah yes, that's why you",
                    "look so familiar. You're",
                    "of those to whom I've taught",
                    "the ^990099Venom Knife^000000 skill. So, what",
                    "brings you to me this time?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Killtin",
                args![
                    "What's that...?!",
                    "You want me to teach",
                    "it to you once again?",
                    "It's a shame you've forgotten,",
                    "but I suppose it can't be helped. Alright, alright, I'll teach you."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Killtin",
                args![
                    "First, you need to equip",
                    "a Knife class weapon, and",
                    "then cast Envenom on your",
                    "knife. Throwing the blade?",
                    "That's all in the wrist. Now,",
                    "watch me closely and take note."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Killtin",
                args![
                    "You see? Having",
                    "good form is essential",
                    "to performing flawless",
                    "technique. Always basics",
                    "before the specifics. Now,",
                    "why don't you give it a try?"
                ],
            )?;
            ctx.call(Function::NpcSpecialEffect, args![constants::EF_INVENOM])?;
            ctx.next()?;
            ctx.lines_as(
                "Killtin",
                args![
                    "Good... Very good...",
                    "Perfect form. Yes...",
                    "Hmm. Are you sure that",
                    "you forgot how to do this",
                    "skill? I suppose that all you",
                    "needed was a quick refresher."
                ],
            )?;
            ctx.call(Function::SpecialEffect, args![constants::EF_INVENOM])?;
            ctx.next()?;
            ctx.lines_as(
                "Killtin",
                args![
                    "Alright, I think it's",
                    "safe to say that you've",
                    "mastered the Venom Knife",
                    "skill. Leave me now, and",
                    "always fight for the honor",
                    "of the Assassin Guild!"
                ],
            )?;
            ctx.call(Function::Skill, args!["AS_VENOMKNIFE", 1, constants::SKILL_PERM])?;
            return ctx.close();
        } else {
            ctx.lines_as(
                "Killtin",
                args![
                    "So how has that",
                    "^990099Venom Knife^000000 skill",
                    "been working for you?",
                    "Be careful, and make sure",
                    "that your victims always",
                    "deserve what you give them!"
                ],
            )?;
            return ctx.close();
        }
    } else if ctx.var("assn_sk2").get()? == 1 && ctx.var("assn_sk").get()? == 7 {
        ctx.lines_as(
            "Killtin",
            args![
                "So you've learned all of",
                "the specialized Assassin",
                "skills, eh? Don't let yourself",
                "become overconfident. Strive",
                "for even greater strength for",
                "the Assassin Guild's honor."
            ],
        )?;
        return ctx.close();
    } else if ctx.var("assn_sk2").get()? == 1 {
        ctx.lines_as(
            "Killtin",
            args![
                "So how has that",
                "^990099Venom Knife^000000 skill",
                "been working for you?",
                "Be careful, and make sure",
                "that your victims always",
                "deserve what you give them!"
            ],
        )?;
        return ctx.close();
    } else if ctx.var("BaseJob").get()? == constants::JOB_ASSASSIN {
        ctx.lines_as(
            "Killtin",
            args![
                "Hm? Ah, you're definitely",
                "a member of the Assassin",
                "Guild. Great, you've come",
                "just at the right time."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Killtin",
            args![
                "Our guildmaster recently",
                "succeeded in developing two",
                "new skills for Assassins. I've",
                "been charged with the task of",
                "teaching these new skills to",
                "all the members of our guild."
            ],
        )?;
        ctx.next()?;
        ctx.var("@menu").set(runtime::select_values(ctx, &[Val::from("New skills?")])?)?;
        ctx.lines_as(
            "Killtin",
            args![
                Val::from("That's right, ") + ctx.player().name()? + Val::from("."),
                "The first skill specifically",
                "enhances the Sonic Blow",
                "skill, and the second skill",
                "is a long range attack that's",
                "named ''^990099Venom Knife^000000.''"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Killtin",
            args![
                "If you have any questions,",
                "feel free to ask me about",
                "any of these new skills. It's",
                "my job to teach you as much",
                "as I can about them."
            ],
        )?;
        ctx.next()?;
        loop {
            match ctx.menu(&[
                "^0000FFSonic Blow Enhancement^000000",
                "^990099Venom Knife^000000",
                "Continue Conversation",
            ])? {
                0 => {
                    ctx.lines_as(
                        "Killtin",
                        args![
                            "If you've been an Assassin",
                            "for a while, then you must",
                            "be familiar with the Sonic",
                            "Blow skill, which inflicts 8",
                            "powerful strikes at an enemy",
                            "in one blindingly fast attack."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Killtin",
                        args![
                            "However, due to the speed",
                            "involved in that skill, Sonic",
                            "Blow isn't as accurate as it",
                            "can be. After years of testing",
                            "and research, our guildmaster",
                            "developed a way to fix this."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Killtin",
                        args![
                            "He created a new skill",
                            "named ''Sonic Acceleration''",
                            "that Assassins can cast on",
                            "themselves in order to quickly",
                            "detect and accurately strike",
                            "the target's fatal points."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Killtin",
                        args![
                            "In effect, Sonic Acceleration",
                            "roughly doubles the damage",
                            "that you can inflict with the",
                            "Sonic Blow. If you use Sonic",
                            "Blow pretty often, then this",
                            "skill will be pretty useful."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Killtin",
                        args![
                            "I suggest that you learn",
                            "the Venom Knife skill from",
                            "me first. Then, you can talk",
                            "to Esmille, the beautiful",
                            "Assassin Cross right next to me, to learn Sonic Acceleration."
                        ],
                    )?;
                    ctx.next()?;
                }
                1 => {
                    ctx.lines_as(
                        "Killtin",
                        args![
                            "As you may well know, our",
                            "job isn't really known for its",
                            "long range attacks. Sure, we",
                            "can use Bows, and we've got",
                            "a few long distance skills, but",
                            "their uses are kind of limited."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Killtin",
                        args![
                            "This Venom Knife skill was",
                            "developed with this weakness",
                            "in long range attacking in mind. Basically, we use the Envenom",
                            "skill on a knife and throw it at a distant enemy to poison them."
                        ],
                    )?;
                    ctx.next()?;
                }
                2 => {
                    ctx.lines_as(
                        "Killtin",
                        args![
                            "Now, if you like, I can",
                            "teach you the ^009900Venom Knife^000000",
                            "skill right now. It won't take",
                            "that much time, so what do",
                            "you say? You ready to learn?"
                        ],
                    )?;
                    ctx.next()?;
                    loop {
                        if ctx.menu(&["Learn Venom Knife", "I d-don't wanna learn!"])? == 0 {
                            ctx.lines_as(
                                "Killtin",
                                args![
                                    "First, you need to equip",
                                    "a Knife class weapon, and",
                                    "then cast Envenom on your",
                                    "knife. Throwing the blade?",
                                    "That's all in the wrist. Now,",
                                    "watch me closely and take note."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Killtin",
                                args![
                                    "You see? Having",
                                    "good form is essential",
                                    "to performing flawless",
                                    "technique. Always basics",
                                    "before the specifics. Now,",
                                    "why don't you give it a try?"
                                ],
                            )?;
                            ctx.call(Function::NpcSpecialEffect, args![constants::EF_INVENOM])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Killtin",
                                args![
                                    "Hey, that's pretty good.",
                                    "You're catching on really",
                                    "quick. Heh heh, but still,",
                                    "I guess I can take a little",
                                    "bit of credit for my excellent",
                                    "instruction. Ah, very nice."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.call(Function::SpecialEffect, args![constants::EF_INVENOM])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Killtin",
                                args![
                                    "Alright, you may need",
                                    "to practice a bit more",
                                    "of this skill, but for the most",
                                    "part, you can use Venom",
                                    "Knife pretty easily in battle."
                                ],
                            )?;
                            ctx.var("assn_sk2").set(1)?;
                            ctx.var("assn_sk").set(1)?;
                            ctx.call(Function::Skill, args!["AS_VENOMKNIFE", 1, constants::SKILL_PERM])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Killtin",
                                args![
                                    "Well, that's all I can",
                                    "teach you. Use this skill",
                                    "expertly, and bring woe to",
                                    "your enemies for the honor",
                                    "of the Assassin Guild!"
                                ],
                            )?;
                            return ctx.close();
                        }
                        ctx.lines_as(
                            "Killtin",
                            args![
                                "You can't refuse an",
                                "order from our guildmaster...",
                                "Like it or not, this skill will",
                                "make you a better Assassin.",
                                "Trust me on this and just agree",
                                "to learn the skill, will you?"
                            ],
                        )?;
                        ctx.next()?;
                    }
                }
                _ => {}
            }
        }
    } else if ctx.var("BaseJob").get()? == constants::JOB_THIEF {
        ctx.lines_as(
            "Killtin",
            args![
                "A Thief...? Huh.",
                "That's a respectable",
                "job. But listen, if you",
                "want me to be able to",
                "teach you anything, you'll",
                "need to get stronger first."
            ],
        )?;
        return ctx.close();
    } else {
        ctx.lines_as("Assassin", args!["...............................", "Just keep moving."])?;
        ctx.close()
    }
}

pub fn assassin_realgirl(ctx: &Ctx) -> Script {
    if ctx.var("BaseJob").get()? == constants::JOB_ASSASSIN && ctx.var("assn_sk").get()? == 7 {
        if ctx.call(Function::GetSkillLv, args!["AS_SONICACCEL"])? == 0 {
            ctx.lines_as(
                "Esmille",
                args![
                    "Mm? Ah, you've transcended",
                    "and become an Assassin Cross",
                    "as well. I understand the trouble that you must have gone through",
                    "to be reborn with new strength."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Esmille",
                args![
                    "You probably need to learn",
                    "the Sonic Acceleration skill",
                    "again after having lost some",
                    "of your memories. I truly",
                    "sympathize, and am willing",
                    "to teach it to you again."
                ],
            )?;
            ctx.next()?;
            if ctx.call(Function::GetSkillLv, args!["AS_SONICBLOW"])? == 0 {
                ctx.lines_as(
                    "Esmille",
                    args![
                        "First, go and learn the",
                        "Sonic Blow skill. The skill",
                        "I will teach you is completely",
                        "useless unless you learn how",
                        "to perform a Sonic Blow. I shall be waiting right here till then."
                    ],
                )?;
                return ctx.close();
            }
            ctx.lines_as(
                "Esmille",
                args![
                    "Now, right before you",
                    "perform Sonic Blow, make",
                    "sure your feet are positioned",
                    "like this. Then, as smoothly",
                    "and quickly as possible, shift",
                    "your weight over to this side."
                ],
            )?;
            ctx.call(Function::NpcSpecialEffect, args![constants::EF_SONICBLOW])?;
            ctx.next()?;
            ctx.lines_as(
                "Esmille",
                args![
                    "Watch carefully, this",
                    "is the most important",
                    "part. See where my hands",
                    "are and the angle of my",
                    "arms? This is the form that",
                    "you've got to memorize."
                ],
            )?;
            ctx.call(Function::NpcSpecialEffect, args![constants::EF_SONICBLOW])?;
            ctx.next()?;
            ctx.lines_as(
                "Esmille",
                args![
                    "Alright, that's all",
                    "you need to know. Now,",
                    "please try it so I can give",
                    "you feedback on your form."
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::SpecialEffect, args![constants::EF_SONICBLOW])?;
            ctx.next()?;
            ctx.lines_as(
                "Esmille",
                args![
                    "Hmm, you're shifting",
                    "your weight kind of",
                    "unsteadily. It might",
                    "help if your center of",
                    "gravity was like this..."
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::SpecialEffect, args![constants::EF_SONICBLOW])?;
            ctx.next()?;
            ctx.lines_as(
                "Esmille",
                args![
                    "Ah, you're so very",
                    "close to perfection.",
                    "Focus more on smoothly",
                    "transitioning from your",
                    "stance to executed action."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Esmille",
                args![
                    "Yes, that's it...!",
                    "Very well executed.",
                    Val::from("Good work, ") + ctx.player().name()? + Val::from(".")
                ],
            )?;
            ctx.call(Function::Skill, args!["AS_SONICACCEL", 1, constants::SKILL_PERM])?;
            ctx.var("assn_sk").set(7)?;
            ctx.next()?;
            ctx.lines_as(
                "Esmille",
                args![
                    "Do you understand now?",
                    "You should have no problem",
                    "remembering this skill now.",
                    "I can teach you nothing more,",
                    "so all I can do now is wish",
                    "you luck on your journeys."
                ],
            )?;
            return ctx.close();
        } else {
            ctx.lines_as(
                "Esmille",
                args![
                    "I trust that using",
                    "Sonic Acceleration in",
                    "battle has given you an",
                    "edge over the enemy. Bring",
                    "swift defeat to your foes for",
                    "the Assassin Guild's honor."
                ],
            )?;
            return ctx.close();
        }
    } else if ctx.var("assn_sk").get()? == 7 {
        ctx.lines_as(
            "Esmille",
            args![
                "I trust that using",
                "Sonic Acceleration in",
                "battle has given you an",
                "edge over the enemy. Bring",
                "swift defeat to your foes for",
                "the Assassin Guild's honor."
            ],
        )?;
        return ctx.close();
    } else if ctx.var("assn_sk").get()? == 6 {
        ctx.lines_as(
            "Esmille",
            args![
                "Please focus on the",
                "training... If we continue to",
                "be interrupted, you'll never",
                "be able to learn anything.",
                "Now, please listen closely."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Esmille",
            args![
                "Now, right before you",
                "perform Sonic Blow, make",
                "sure your feet are positioned",
                "like this. Then, as smoothly",
                "and quickly as possible, shift",
                "your weight over to this side."
            ],
        )?;
        ctx.call(Function::NpcSpecialEffect, args![constants::EF_SONICBLOW])?;
        ctx.next()?;
        ctx.lines_as(
            "Esmille",
            args![
                "Watch carefully, this",
                "is the most important",
                "part. See where my hands",
                "are and the angle of my",
                "arms? This is the form that",
                "you've got to memorize."
            ],
        )?;
        ctx.call(Function::NpcSpecialEffect, args![constants::EF_SONICBLOW])?;
        ctx.next()?;
        ctx.lines_as(
            "Esmille",
            args![
                "Alright, that's all",
                "you need to know. Now,",
                "please try it so I can give",
                "you feedback on your form."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::SpecialEffect, args![constants::EF_SONICBLOW])?;
        ctx.next()?;
        ctx.lines_as(
            "Esmille",
            args![
                "Hmm, you're shifting",
                "your weight kind of",
                "unsteadily. It might",
                "help if your center of",
                "gravity was like this..."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::SpecialEffect, args![constants::EF_SONICBLOW])?;
        ctx.next()?;
        ctx.lines_as(
            "Esmille",
            args![
                "That's a little better.",
                "Hmmm. Try to think of",
                "the enemy's weak point",
                "and follow through with",
                "the stabbing motion."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::SpecialEffect, args![constants::EF_SONICBLOW])?;
        ctx.next()?;
        ctx.lines_as(
            "Esmille",
            args![
                "Ah, you're so very",
                "close to perfection.",
                "Focus more on smoothly",
                "transitioning from your",
                "stance to executed action."
            ],
        )?;
        ctx.call(Function::SpecialEffect, args![constants::EF_SONICBLOW])?;
        ctx.next()?;
        ctx.lines_as(
            "Esmille",
            args![
                "Yes, that's it...!",
                "Very well executed.",
                Val::from("Good work, ") + ctx.player().name()? + Val::from(".")
            ],
        )?;
        ctx.call(Function::Skill, args!["AS_SONICACCEL", 1, constants::SKILL_PERM])?;
        ctx.var("assn_sk").set(7)?;
        ctx.next()?;
        ctx.lines_as(
            "Esmille",
            args![
                "Do you understand now?",
                "You should have no problem",
                "remembering this skill now.",
                "I can teach you nothing more,",
                "so all I can do now is wish",
                "you luck on your journeys."
            ],
        )?;
        return ctx.close();
    } else if ctx.var("assn_sk").get()? == 5 {
        ctx.lines_as(
            "Esmille",
            args![
                "So how is your little",
                "mission coming along?",
                "I trust that you've completed",
                "that task I assigned for you."
            ],
        )?;
        ctx.next()?;
        if (ctx.var("assn_sk").get()? == 5 && ctx.items().count(726)? > 0)
            || (ctx.var("assn_sk").get()? == 5 && ctx.items().count(723)? > 0)
            || (ctx.var("assn_sk").get()? == 5 && ctx.items().count(720)? > 0)
        {
            ctx.var("@menu")
                .set(runtime::select_values(ctx, &[Val::from("How's this for treasure?")])?)?;
            ctx.lines_as(
                "Esmille",
                args![
                    "Oh, that jewel...!",
                    "It's so captivating~",
                    "I haven't seen anything",
                    "so beautiful in such a long",
                    "time. You've done very well..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Esmille",
                args![
                    "That jewel is yours",
                    "to keep. In truth, I don't",
                    "really need any treasure, just some proof of your qualification.",
                    "It looks like you're ready for me to teach you Sonic Acceleration."
                ],
            )?;
            ctx.var("assn_sk").set(6)?;
            ctx.next()?;
            ctx.lines_as(
                "Esmille",
                args![
                    "Now, right before you",
                    "perform Sonic Blow, make",
                    "sure your feet are positioned",
                    "like this. Then, as smoothly",
                    "and quickly as possible, shift",
                    "your weight over to this side."
                ],
            )?;
            ctx.call(Function::NpcSpecialEffect, args![constants::EF_SONICBLOW])?;
            ctx.next()?;
            ctx.lines_as(
                "Esmille",
                args![
                    "Watch carefully, this",
                    "is the most important",
                    "part. See where my hands",
                    "are and the angle of my",
                    "arms? This is the form that",
                    "you've got to memorize."
                ],
            )?;
            ctx.call(Function::NpcSpecialEffect, args![constants::EF_SONICBLOW])?;
            ctx.next()?;
            ctx.lines_as(
                "Esmille",
                args![
                    "Alright, that's all",
                    "you need to know. Now,",
                    "please try it so I can give",
                    "you feedback on your form."
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::SpecialEffect, args![constants::EF_SONICBLOW])?;
            ctx.next()?;
            ctx.lines_as(
                "Esmille",
                args![
                    "Hmm, you're shifting",
                    "your weight kind of",
                    "unsteadily. It might",
                    "help if your center of",
                    "gravity was like this..."
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::SpecialEffect, args![constants::EF_SONICBLOW])?;
            ctx.next()?;
            ctx.lines_as(
                "Esmille",
                args![
                    "That's a little better.",
                    "Hmmm. Try to think of",
                    "the enemy's weak point",
                    "and follow through with",
                    "the stabbing motion."
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::SpecialEffect, args![constants::EF_SONICBLOW])?;
            ctx.next()?;
            ctx.lines_as(
                "Esmille",
                args![
                    "Yes, that's it...!",
                    "Very well executed.",
                    Val::from("Good work, ") + ctx.player().name()? + Val::from(".")
                ],
            )?;
            ctx.call(Function::Skill, args!["AS_SONICACCEL", 1, constants::SKILL_PERM])?;
            ctx.var("assn_sk").set(7)?;
            ctx.next()?;
            ctx.lines_as(
                "Esmille",
                args![
                    "Do you understand now?",
                    "You should have no problem",
                    "remembering this skill now.",
                    "I can teach you nothing more,",
                    "so all I can do now is wish",
                    "you luck on your journeys."
                ],
            )?;
            return ctx.close();
        } else {
            ctx.lines_as("Esmille", args!["Hmmm..."])?;
            return ctx.close();
        }
    } else if ctx.var("assn_sk").get()? == 2 || ctx.var("assn_sk").get()? == 3 || ctx.var("assn_sk").get()? == 4 {
        ctx.lines_as(
            "Esmille",
            args![
                "So how is your little",
                "mission coming along?",
                "If you've forgotten the",
                "location I've asked you to",
                "search for treasure, then",
                "I can briefly remind you."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Esmille", args!["Find something valuable"])?;
        if ctx.var("assn_sk").get()? == 2 {
            ctx.lines(args!["for me by searching the", "^FF0000Coffins^000000 in the Sphinx."])?;
        } else if ctx.var("assn_sk").get()? == 3 {
            ctx.lines(args!["for me by searching the", "^FF0000Stone Statues^000000 in the Sphinx."])?;
        } else if ctx.var("assn_sk").get()? == 4 {
            ctx.lines(args![
                "in the ^FF0000flooded crypt in the",
                "bottom floor^000000 of the Pyramids."
            ])?;
        }
        ctx.lines(args![
            "Only the strong can explore",
            "that area, so doing this will",
            "prove your competency to me."
        ])?;
        return ctx.close();
    } else if ctx.var("assn_sk").get()? == 1 {
        ctx.lines_as(
            "Esmille",
            args![
                "Ah. Hello, comrade.",
                "Have you heard about",
                "the latest news from",
                "the Assassin Guild?",
                "Ah, you've spoken to",
                "Killtin. Good, good..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Esmille",
            args![
                "Now, I've been charged",
                "with the responsibility of",
                "teaching the Sonic Acceleration skill to all interested Assassins.",
                "I can tell you more about it if",
                "Killtin didn't fully explain."
            ],
        )?;
        ctx.next()?;
        loop {
            match ctx.menu(&["Please tell me more...", "I want to learn Sonic Acceleration!"])? {
                0 => {
                    ctx.lines_as(
                        "Esmille",
                        args![
                            "Sonic Acceleration is",
                            "a ^FF0000support skill used in",
                            "conjunction with Sonic Blow^000000.",
                            "Assassins can only cast this",
                            "skill on themselves for their",
                            "own personal benefit."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Esmille",
                        args![
                            "If you're familiar with",
                            "Sonic Blow, you'll know",
                            "that it's difficult to inflict",
                            "fatal damage with that skill.",
                            "It's far too fast to be able",
                            "to attack very accurately..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Esmille",
                        args![
                            "However, by learning",
                            "Sonic Acceleration, you",
                            "can overcome this accuracy",
                            "drawback and fulfill the full",
                            "damage potential of the",
                            "Sonic Blow skill."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Esmille",
                        args![
                            "This skill is truly great.",
                            "And our guildmaster,",
                            "the one who invented this",
                            "skill, is also... He's a man",
                            "amongst men, I must say."
                        ],
                    )?;
                    ctx.call(Function::Emotion, args![constants::ET_THROB])?;
                    ctx.next()?;
                }
                1 => {
                    ctx.lines_as(
                        "Esmille",
                        args![
                            "Ah, I'm glad to see",
                            "that you're so enthusiastic",
                            "about learning this skill.",
                            "But first, there we need to",
                            "take care of the prerequisites..."
                        ],
                    )?;
                    ctx.next()?;
                    if ctx.call(Function::GetSkillLv, args!["AS_SONICBLOW"])? == 0 {
                        ctx.lines_as(
                            "Esmille",
                            args![
                                "First, go and learn the",
                                "Sonic Blow skill. The skill",
                                "I will teach you is completely",
                                "useless unless you learn how",
                                "to perform a Sonic Blow. I shall be waiting right here till then."
                            ],
                        )?;
                        return ctx.close();
                    }
                    ctx.lines_as("Esmille", args!["Your task will be to bring"])?;
                    match ctx.call(Function::Rand, args![1, 3])?.number()? {
                        1 => {
                            ctx.lines(args![
                                "treasure from the Sphinx.",
                                "Search the ^FF0000Coffins^000000 that",
                                "are there for precious",
                                "valuables. Consider this",
                                "a test of your strength."
                            ])?;
                            ctx.var("assn_sk").set(2)?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Esmille",
                                args![
                                    "Whether you can complete",
                                    "this task will determine",
                                    "if you are worthy enough",
                                    "for me to teach you the",
                                    "Sonic Acceleration skill.",
                                    "Best of luck, and please hurry."
                                ],
                            )?;
                            return ctx.close();
                        }
                        2 => {
                            ctx.lines(args![
                                "treasure from the Sphinx.",
                                "Search the ^FF0000Stone Statues^000000",
                                "there for precious valuables.",
                                "Consider this excursion as",
                                "a test of your strength."
                            ])?;
                            ctx.var("assn_sk").set(3)?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Esmille",
                                args![
                                    "Whether you can complete",
                                    "this task will determine",
                                    "if you are worthy enough",
                                    "for me to teach you the",
                                    "Sonic Acceleration skill.",
                                    "Best of luck, and please hurry."
                                ],
                            )?;
                            return ctx.close();
                        }
                        3 => {
                            ctx.lines(args![
                                "treasure from the Pyramids.",
                                "Search the ^FF0000flooded crypt at",
                                "the bottom floor for precious",
                                "valuables^000000. Consider this as",
                                "a test of your strength."
                            ])?;
                            ctx.var("assn_sk").set(4)?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Esmille",
                                args![
                                    "Whether you can complete",
                                    "this task will determine",
                                    "if you are worthy enough",
                                    "for me to teach you the",
                                    "Sonic Acceleration skill.",
                                    "Best of luck, and please hurry."
                                ],
                            )?;
                            return ctx.close();
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    } else if ctx.var("BaseJob").get()? == constants::JOB_ASSASSIN {
        ctx.lines_as(
            "Assassin",
            args![
                "Ah. Hello, comrade.",
                "Have you heard about",
                "the latest news from",
                "the Assassin Guild?"
            ],
        )?;
        ctx.call(Function::Emotion, args![constants::ET_SMILE])?;
        ctx.next()?;
        ctx.var("@menu")
            .set(runtime::select_values(ctx, &[Val::from("News from the Assassin Guild?")])?)?;
        ctx.lines_as(
            "Assassin",
            args![
                "Hm. You must not have",
                "heard it, then. If you want",
                "to know more about it, you",
                "should speak to Killtin, who",
                "is right next to me. He will",
                "explain everything clearly."
            ],
        )?;
        return ctx.close();
    } else {
        ctx.lines_as(
            "Assassin",
            args![
                "Hm. Do you know any",
                "Assassins? Tell them",
                "to come here if they",
                "haven't already."
            ],
        )?;
        ctx.close()
    }
}

#[derive(Clone, Copy, Debug)]
enum OldCoffinQskAsStep {
    Start,
    OnTouch,
}

fn old_coffin_qsk_as_run(ctx: &Ctx, mut step: OldCoffinQskAsStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            OldCoffinQskAsStep::Start => {
                step = OldCoffinQskAsStep::OnTouch;
                continue 'machine;
            }
            OldCoffinQskAsStep::OnTouch => {
                if ctx.var("assn_sk").get()? == 2 {
                    ctx.lines(args![
                        "^3355FFIt's an ancient coffin",
                        "with a broken lid that",
                        "is slightly ajar. You",
                        "momentarily catch a glint",
                        "of something shining inside.^000000"
                    ])?;
                    ctx.call(Function::NpcSpecialEffect, args![constants::EF_CONE])?;
                    ctx.next()?;
                    loop {
                        match ctx.menu(&[
                            "Put your hand inside",
                            "Inspect the coffin's opening",
                            "Lift the lid",
                            "Turn the coffin upside down",
                            "Ignore it",
                        ])? {
                            0 => {
                                ctx.lines(args![
                                    "^3355FFYou carefully put your",
                                    "hand inside the coffin",
                                    "and try to retrieve the",
                                    "shining object that",
                                    "you had glimpsed.",
                                    "...............................^000000"
                                ])?;
                                ctx.next()?;
                                ctx.lines(args![
                                    "^3355FFSomething inside",
                                    "the coffin bit your",
                                    "hand really hard!^000000"
                                ])?;
                                ctx.call(Function::StartStatus, args![ctx.constant("SC_POISON")?, 30000, 0])?;
                                ctx.call(Function::StartStatus, args![ctx.constant("SC_BLEEDING")?, 10000, 0])?;
                                ctx.call(
                                    Function::Emotion,
                                    args![constants::ET_HUK, ctx.call(Function::GetCharacterId, args![0])?.is_true()],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            1 => {
                                ctx.lines(args![
                                    "^3355FFYou try to peek",
                                    "inside the coffin",
                                    "through the gap",
                                    "between the coffin's",
                                    "edge and the lid.",
                                    "...............................^000000"
                                ])?;
                                ctx.next()?;
                                ctx.lines(args![
                                    "^3355FFYou're barely able to",
                                    "perceive that something",
                                    "is squirming inside the",
                                    "coffin, but it's far too dark",
                                    "to see anything else.^000000"
                                ])?;
                                ctx.call(Function::StartStatus, args![ctx.constant("SC_BLIND")?, 30000, 0])?;
                                ctx.next()?;
                            }
                            2 => {
                                ctx.lines(args![
                                    "^3355FFYou don't have the",
                                    "strength to move",
                                    "something as heavy",
                                    "as this coffin's lid...^000000"
                                ])?;
                                ctx.call(Function::StartStatus, args![ctx.constant("SC_CURSE")?, 30000, 0])?;
                                ctx.next()?;
                            }
                            3 => {
                                ctx.lines(args![
                                    "^3355FFYou don't have the",
                                    "strength to turn this",
                                    "coffin upside down.^000000"
                                ])?;
                                ctx.next()?;
                            }
                            4 => {
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn old_coffin_qsk_as(ctx: &Ctx) -> Script {
    old_coffin_qsk_as_run(ctx, OldCoffinQskAsStep::Start, Vec::new()).map(|_| ())
}

pub fn old_coffin_qsk_as_ontouch(ctx: &Ctx) -> Script {
    old_coffin_qsk_as_run(ctx, OldCoffinQskAsStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum OldCoffinQskAs2Step {
    Start,
    OnTouch,
}

fn old_coffin_qsk_as2_run(ctx: &Ctx, mut step: OldCoffinQskAs2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            OldCoffinQskAs2Step::Start => {
                step = OldCoffinQskAs2Step::OnTouch;
                continue 'machine;
            }
            OldCoffinQskAs2Step::OnTouch => {
                if ctx.var("assn_sk").get()? == 2 {
                    ctx.lines(args![
                        "^3355FFIt's an ancient coffin",
                        "with a broken lid that",
                        "is slightly ajar. You",
                        "momentarily catch a glint",
                        "of something shining inside.^000000"
                    ])?;
                    ctx.call(Function::NpcSpecialEffect, args![constants::EF_CONE])?;
                    ctx.next()?;
                    loop {
                        match ctx.menu(&[
                            "Put your hand inside",
                            "Inspect the coffin's opening",
                            "Lift the lid",
                            "Turn the coffin upside down",
                            "Ignore it",
                        ])? {
                            0 => {
                                ctx.lines(args![
                                    "^3355FFYou carefully put your",
                                    "hand inside the coffin",
                                    "and try to retrieve the",
                                    "shining object that",
                                    "you had glimpsed.",
                                    "...............................^000000"
                                ])?;
                                ctx.next()?;
                                if ctx.call(Function::Rand, args![1, 3])? != 3 {
                                    ctx.lines(args![
                                        "^3355FFYou carefully put your",
                                        "hand inside the coffin",
                                        "and try to retrieve the",
                                        "shining object that",
                                        "you had glimpsed.",
                                        "...............................^000000"
                                    ])?;
                                    ctx.next()?;
                                    ctx.lines(args![
                                        "^3355FFSomething inside",
                                        "the coffin bit your",
                                        "hand really hard!^000000"
                                    ])?;
                                    ctx.call(Function::StartStatus, args![ctx.constant("SC_POISON")?, 30000, 0])?;
                                    ctx.call(Function::StartStatus, args![ctx.constant("SC_BLEEDING")?, 10000, 0])?;
                                    ctx.call(
                                        Function::Emotion,
                                        args![constants::ET_HUK, ctx.call(Function::GetCharacterId, args![0])?.is_true()],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    ctx.lines(args![
                                        "^3355FFYour fingers manage",
                                        "to find a solid object",
                                        "that you pull out of the",
                                        "coffin. You have obtained",
                                        "a Sapphire for Esmille.^000000"
                                    ])?;
                                    ctx.var("assn_sk").set(5)?;
                                    ctx.items().give(726, 1)?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                            }
                            1 => {
                                ctx.lines(args![
                                    "^3355FFYou try to peek",
                                    "inside the coffin",
                                    "through the gap",
                                    "between the coffin's",
                                    "edge and the lid.",
                                    "...............................^000000"
                                ])?;
                                ctx.next()?;
                                ctx.lines(args![
                                    "^3355FFYou're barely able to",
                                    "perceive that something",
                                    "is squirming inside the",
                                    "coffin, but it's far too dark",
                                    "to see anything else.^000000"
                                ])?;
                                ctx.call(Function::StartStatus, args![ctx.constant("SC_BLIND")?, 30000, 0])?;
                                ctx.next()?;
                            }
                            2 => {
                                ctx.lines(args![
                                    "^3355FFYou don't have the",
                                    "strength to move",
                                    "something as heavy",
                                    "as this coffin's lid...^000000"
                                ])?;
                                ctx.call(Function::StartStatus, args![ctx.constant("SC_CURSE")?, 30000, 0])?;
                                ctx.next()?;
                            }
                            3 => {
                                ctx.lines(args![
                                    "^3355FFYou don't have the",
                                    "strength to turn this",
                                    "coffin upside down.^000000"
                                ])?;
                                ctx.next()?;
                            }
                            4 => {
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn old_coffin_qsk_as2(ctx: &Ctx) -> Script {
    old_coffin_qsk_as2_run(ctx, OldCoffinQskAs2Step::Start, Vec::new()).map(|_| ())
}

pub fn old_coffin_qsk_as2_ontouch(ctx: &Ctx) -> Script {
    old_coffin_qsk_as2_run(ctx, OldCoffinQskAs2Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum StoneStatueQskAsStep {
    Start,
    OnTouch,
}

fn stone_statue_qsk_as_run(ctx: &Ctx, mut step: StoneStatueQskAsStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            StoneStatueQskAsStep::Start => {
                step = StoneStatueQskAsStep::OnTouch;
                continue 'machine;
            }
            StoneStatueQskAsStep::OnTouch => {
                if ctx.var("assn_sk").get()? == 3 {
                    ctx.call(Function::NpcSpecialEffect, args![constants::EF_CONE])?;
                    ctx.lines(args![
                        "^3355FFThis ancient stone statue",
                        "is covered with cracks and",
                        "looks close to falling apart.",
                        "The glimmer of a shining object",
                        "peers out from beneath one of",
                        "the feet. The ground appears",
                        "soft enough to dig through...^000000"
                    ])?;
                    ctx.call(Function::NpcSpecialEffect, args![constants::EF_CONE])?;
                    ctx.next()?;
                    if ctx.menu(&["Dig to retrieve the shining object", "Ignore it"])? == 0 {
                        ctx.lines(args!["^3355FFAs your fingers dig into", "the soft ground, it emits^000000"])?;
                        if ctx.call(Function::Rand, args![1, 3])? != 3 {
                            ctx.lines(args![
                                "^3355FFa yellow gas that clouds",
                                "your senses and briefly",
                                "knocks you unconscious.^000000"
                            ])?;
                            ctx.call(Function::StartStatus, args![ctx.constant("SC_SLEEP")?, 30000, 0])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines(args![
                            "^3355FFa yellow gas. However, you",
                            "hold your breath in and expel",
                            "all gas in your lungs in time",
                            "to escape its effects. You've",
                            "retrieved a Ruby for Esmille.^000000"
                        ])?;
                        ctx.var("assn_sk").set(5)?;
                        ctx.items().give(723, 1)?;
                    }
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn stone_statue_qsk_as(ctx: &Ctx) -> Script {
    stone_statue_qsk_as_run(ctx, StoneStatueQskAsStep::Start, Vec::new()).map(|_| ())
}

pub fn stone_statue_qsk_as_ontouch(ctx: &Ctx) -> Script {
    stone_statue_qsk_as_run(ctx, StoneStatueQskAsStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum StoneStatueQskAs2Step {
    Start,
    OnTouch,
}

fn stone_statue_qsk_as2_run(ctx: &Ctx, mut step: StoneStatueQskAs2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            StoneStatueQskAs2Step::Start => {
                step = StoneStatueQskAs2Step::OnTouch;
                continue 'machine;
            }
            StoneStatueQskAs2Step::OnTouch => {
                if ctx.var("assn_sk").get()? == 3 {
                    ctx.call(Function::NpcSpecialEffect, args![constants::EF_CONE])?;
                    ctx.lines(args![
                        "^3355FFThis ancient stone statue",
                        "is covered with cracks and",
                        "looks close to falling apart.",
                        "The glimmer of a shining object",
                        "peers out from beneath one of",
                        "the feet. The ground appears",
                        "soft enough to dig through...^000000"
                    ])?;
                    ctx.call(Function::NpcSpecialEffect, args![constants::EF_CONE])?;
                    ctx.next()?;
                    if ctx.menu(&["Dig to retrieve the shining object", "Ignore it"])? == 0 {
                        ctx.lines(args!["^3355FFAs your fingers dig into", "the soft ground, it emits^000000"])?;
                        if ctx.call(Function::Rand, args![1, 3])? != 3 {
                            ctx.lines(args![
                                "^3355FFa yellow gas that clouds",
                                "your senses and briefly",
                                "knocks you unconscious.^000000"
                            ])?;
                            ctx.call(Function::StartStatus, args![ctx.constant("SC_SLEEP")?, 30000, 0])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines(args![
                            "^3355FFa yellow gas. However, you",
                            "hold your breath in and expel",
                            "all gas in your lungs in time",
                            "to escape its effects. Sadly,",
                            "all you found was broken glass.^000000"
                        ])?;
                        ctx.call(
                            Function::Emotion,
                            args![constants::ET_HUK, ctx.call(Function::GetCharacterId, args![0])?.is_true()],
                        )?;
                    }
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn stone_statue_qsk_as2(ctx: &Ctx) -> Script {
    stone_statue_qsk_as2_run(ctx, StoneStatueQskAs2Step::Start, Vec::new()).map(|_| ())
}

pub fn stone_statue_qsk_as2_ontouch(ctx: &Ctx) -> Script {
    stone_statue_qsk_as2_run(ctx, StoneStatueQskAs2Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum CryptStep {
    Start,
    OnTouch,
}

fn crypt_run(ctx: &Ctx, mut step: CryptStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            CryptStep::Start => {
                step = CryptStep::OnTouch;
                continue 'machine;
            }
            CryptStep::OnTouch => {
                if ctx.var("assn_sk").get()? == 4 {
                    ctx.call(Function::NpcSpecialEffect, args![constants::EF_CONE])?;
                    ctx.lines(args![
                        "^3355FFThere's something",
                        "glimmering beneath",
                        "the surface of the water...^000000"
                    ])?;
                    ctx.next()?;
                    if ctx.menu(&["Pick it up", "Ignore it"])? == 0 {
                        ctx.lines(args![
                            "^3355FFAs soon as you dip your",
                            "hand into the water, the",
                            "water's freezing chill shoots",
                            "up through your arm. You ",
                            "better hurry before you freeze!^000000"
                        ])?;
                        ctx.next()?;
                        if ctx.call(Function::Rand, args![1, 3])? != 3 {
                            ctx.lines(args![
                                "^3355FFIt's too late!",
                                "Your body has just",
                                "been frozen solid.^000000"
                            ])?;
                            ctx.call(Function::StartStatus, args![ctx.constant("SC_FREEZE")?, 10000, 0])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines(args![
                            "^3355FFYou quickly pick up",
                            "the glimmering object",
                            "before the water can",
                            "freeze you. You obtained",
                            "an Aquamarine for Esmille.^000000"
                        ])?;
                        ctx.var("assn_sk").set(5)?;
                        ctx.items().give(720, 1)?;
                    }
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn crypt(ctx: &Ctx) -> Script {
    crypt_run(ctx, CryptStep::Start, Vec::new()).map(|_| ())
}

pub fn crypt_ontouch(ctx: &Ctx) -> Script {
    crypt_run(ctx, CryptStep::OnTouch, Vec::new()).map(|_| ())
}

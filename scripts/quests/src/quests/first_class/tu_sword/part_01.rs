use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn shurank_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_chk_bash = Val::from(0);
    let mut l_chk_endure = Val::from(0);
    let mut l_chk_hanson = Val::from(0);
    let mut l_chk_hp = Val::from(0);
    let mut l_chk_magnum = Val::from(0);
    let mut l_chk_provoke = Val::from(0);
    let mut l_chk_yangson = Val::from(0);
    ctx.mes("[Shurank]")?;
    if (ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_NOVICE")?)
        || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_BABY")?))
    {
        ctx.lines(args!["Still wondering", "what to do with", "your future, eh?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Shurank",
            args![
                "If I may be so bold,",
                "I suggest that you follow",
                "the path of strength.",
                "Become... A Swordman."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Shurank",
            args!["If you wish to stake your life on the way of the sword, speak to the Swordman Association representative to the left."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Shurank",
            args![
                "Whatever you do,",
                "I hope that you live out your life without any regrets. And if you do happen to become a Swordman,",
                "come back to me..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_SWORDMAN")?)
        && runtime::op(
            &ctx.call(Function::EaClass, vec![])?,
            "&",
            &runtime::op(
                &runtime::op(&ctx.constant("EAJL_2")?, "|", &ctx.constant("EAJL_UPPER")?)?,
                "|",
                &ctx.constant("EAJL_THIRD")?,
            )?,
        )?
        .is_true())
    {
        ctx.lines(args!["From your raiment,", "I see that you are"])?;
        if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
            ctx.mes("a man of the sword.")?;
        } else {
            ctx.mes("a woman of the sword.")?;
        }
        ctx.lines(args!["Your eyes tell me that", "you've seen many a battle."])?;
        ctx.next()?;
        ctx.lines_as("Shurank", args!["There's nothing that I can actually offer to teach you. But if you know any new Swordmen that show potential, please direct them to me."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if !ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_SWORDMAN")?) {
        ctx.lines(args![
            "Greetings adventurer.",
            "If you happen to know",
            "any new Swordmen,",
            "please direct them to me."
        ])?;
        ctx.next()?;
        ctx.lines_as("Shurank", args!["My specialty happens to be teaching the fundamentals of the Swordman job to green recruits that have just made the job change."])?;
        ctx.next()?;
        ctx.lines_as(
            "Shurank",
            args![
                "Although it would be great if everyone can wield a sword,",
                "I'm afraid there's nothing I can effectively teach to you. The methods of your job are...",
                "Foreign to me."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    'b1: {
        let subject1 = ctx.var("tu_swordman").get()?;
        let mut matched1 = false;
        let no_case1 = !subject1.loosely_equals(&Val::from(23))
            && !subject1.loosely_equals(&Val::from(22))
            && !subject1.loosely_equals(&Val::from(21))
            && !subject1.loosely_equals(&Val::from(13))
            && !subject1.loosely_equals(&Val::from(12))
            && !subject1.loosely_equals(&Val::from(11))
            && !subject1.loosely_equals(&Val::from(10))
            && !subject1.loosely_equals(&Val::from(9))
            && !subject1.loosely_equals(&Val::from(8))
            && !subject1.loosely_equals(&Val::from(7))
            && !subject1.loosely_equals(&Val::from(6))
            && !subject1.loosely_equals(&Val::from(5))
            && !subject1.loosely_equals(&Val::from(4))
            && !subject1.loosely_equals(&Val::from(3))
            && !subject1.loosely_equals(&Val::from(2))
            && !subject1.loosely_equals(&Val::from(1))
            && !subject1.loosely_equals(&Val::from(0));
        if !matched1 && subject1.loosely_equals(&Val::from(23)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines(args![
                "Greetings, my friend.",
                "There is nothing more",
                "that I can teach you..."
            ])?;
            ctx.next()?;
            ctx.lines_as("Shurank", args!["To improve your skills,", "you must explore this vast world and experience all sorts of battle situations. And always commit yourself to your training."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        if !matched1 && subject1.loosely_equals(&Val::from(22)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines(args![
                "This will be my",
                "final lecture to you.",
                "Please listen carefully",
                "as I tell you all about",
                "Magnum Break."
            ])?;
            ctx.next()?;
            ctx.lines_as("Shurank", args!["Magnum Break, mastered at Skill Level 10, requires ^5D478BLevel 5 Bash^000000. When used, this skill inflicts splash damage to enemies surrounding the caster."])?;
            ctx.next()?;
            ctx.lines_as("Shurank", args!["Enemies struck by Magnum Break", "will be pushed back by a Fire property attack. Afterwards, your weapon will temporarily inflict 20% more damage with Fire property attacks."])?;
            ctx.next()?;
            ctx.lines_as(
                "Shurank",
                args![
                    "Magnum Break is ideal for use",
                    "when surrounded by foes, but also keep in mind that it also inflicts a little damage on its caster."
                ],
            )?;
            ctx.next()?;
            l_chk_magnum = ctx.call(Function::GetSkillLv, vec![Val::from("SM_MAGNUM")])?;
            if l_chk_magnum.clone().number()? > 5 {
                ctx.lines_as("Shurank", args!["I see that you've been training in the use of Magnum Break. Although it is an awesome skill, it's not for every Swordman. Still, there's no harm in learning it."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Shurank",
                    args![
                        "Here is a humble reward for",
                        "your great efforts. But do not let your talents allow you to grow arrogant!"
                    ],
                )?;
                ctx.var("tu_swordman").set(Val::from(23))?;
                ctx.call(Function::CompleteQuest, vec![Val::from(8228)])?;
                ctx.call(Function::GetExperience, vec![Val::from(1860), Val::from(0)])?;
                ctx.call(Function::GetItem, vec![Val::from(1113), Val::from(1)])?;
                ctx.next()?;
                ctx.lines_as("Shurank", args!["That is all I have to teach you about Swordman skills. Hopefully, I've been able to clear up anything you haven't understood about your job and skills."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Shurank",
                    args![
                        "As for the matter concerning",
                        "that murderer, it's no longer in our hands. At this time, there's nothing that we can effectively",
                        "do about it."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Shurank",
                    args![
                        "However, I'd like",
                        "to ask that you keep the",
                        "information you've learned",
                        "to yourself. This matter is considered highly classified..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Shurank", args!["In any case, there's no need to worry about it anymore. But if we ever need assistance, I'll remember to ask you for your help."])?;
                ctx.next()?;
                ctx.lines_as("Shurank", args!["You've done very well. Continue training and if you reach ^5D478BJob Level 40^000000, you can be promoted to a Knight or a Crusader."])?;
                ctx.next()?;
                ctx.lines_as("Shurank", args!["Well...", "We Swordmen typically aren't prone to being sentimental, but I'm glad to have met you. Do your best and become a splendid Swordman."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as("Shurank", args!["That is all I have to teach you about Swordman skills. Hopefully, I've been able to clear up anything you haven't understood about your job and skills."])?;
            ctx.next()?;
            ctx.lines_as(
                "Shurank",
                args![
                    "As for the matter concerning",
                    "that murderer, it's no longer in our hands. At this time, there's nothing that we can effectively do about it."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Shurank",
                args![
                    "However, I'd like",
                    "to ask that you keep the",
                    "information you've learned",
                    "to yourself. This matter is considered highly classified..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Shurank", args!["In any case, there's no need to worry about it anymore. But if we ever need assistance, I'll remember to ask you for your help."])?;
            ctx.next()?;
            ctx.lines_as("Shurank", args!["You've done very well. Continue training and if you reach ^5D478BJob Level 40^000000, you can be promoted to a Knight or a Crusader."])?;
            ctx.next()?;
            ctx.lines_as("Shurank", args!["Well...", "We Swordmen typically aren't prone to being sentimental, but I'm glad to have met you. Do your best and become a splendid Swordman."])?;
            ctx.next()?;
            ctx.lines_as(
                "Shurank",
                args![
                    "Before you go, please take this.",
                    "I hope that you make good use of it as you train to become stronger and stronger. Good luck out there."
                ],
            )?;
            ctx.var("tu_swordman").set(Val::from(23))?;
            ctx.call(Function::CompleteQuest, vec![Val::from(8228)])?;
            ctx.call(Function::GetItem, vec![Val::from(1113), Val::from(1)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        if !matched1 && subject1.loosely_equals(&Val::from(21)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines(args![
                "Ah, you've returned. I've just received a communique from",
                "Dequ'ee. It seems you've been working very hard and that you've made the most of this experience."
            ])?;
            ctx.next()?;
            ctx.lines_as("Shurank", args!["For now, I believe you'll benefit from learning some more about Swordman skills. First, let me tell you more about the Sword Masteries."])?;
            ctx.next()?;
            ctx.lines_as(
                "Shurank",
                args![
                    "There are two different kinds:",
                    "^5D478BOne Handed Sword Mastery^000000 and",
                    "^5D478BTwo Handed Sword Mastery^000000. Both",
                    "are Passive Skills that increase your Attack Power as these skills are leveled up."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Shurank", args!["One Handed Sword Mastery", "increases the damage of Daggers and One Handed Swords while Two Handed Sword Mastery increases the damage of Two Handed Swords."])?;
            ctx.next()?;
            l_chk_hanson = ctx.call(Function::GetSkillLv, vec![Val::from("SM_SWORD")])?;
            l_chk_yangson = ctx.call(Function::GetSkillLv, vec![Val::from("SM_TWOHAND")])?;
            if (l_chk_hanson.clone().number()? > 9 || l_chk_yangson.clone().number()? > 9) {
                ctx.lines_as(
                    "Shurank",
                    args![
                        "Ah, from the way you handle your Sword, I see that you have mastered a Sword Mastery. I'm thoroughly impressed."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Shurank",
                    args!["Let me give you", "this small reward", "in recognition of", "your accomplishment."],
                )?;
                ctx.var("tu_swordman").set(Val::from(22))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(8227), Val::from(8228)])?;
                ctx.call(Function::GetExperience, vec![Val::from(1860), Val::from(0)])?;
                ctx.next()?;
                ctx.lines_as(
                    "Shurank",
                    args![
                        "Next time, I will tell you more about the Magnum Break skill.",
                        "I'll be here when you're ready to learn more."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Shurank",
                args![
                    "Next time, I will tell you more about the Magnum Break skill.",
                    "I'll be here when you're ready to learn more."
                ],
            )?;
            ctx.var("tu_swordman").set(Val::from(22))?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        if !matched1 && subject1.loosely_equals(&Val::from(13)) {
            matched1 = true;
        }
        if matched1 {
            ctx.mes("You should leave as soon as possible to meet Dequ'ee in Geffen. You do remember where to find him, don't you?")?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        if !matched1 && subject1.loosely_equals(&Val::from(12)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines(args!["So how has your", "training for the Endure", "skill coming along?"])?;
            ctx.next()?;
            l_chk_endure = ctx.call(Function::GetSkillLv, vec![Val::from("SM_ENDURE")])?;
            if l_chk_endure.clone().number()? > 1 {
                ctx.lines_as(
                    "Shurank",
                    args!["From that gleen of toughness upon your skin, I see now that I was foolish to ask. Well done~"],
                )?;
                ctx.var("tu_swordman").set(Val::from(13))?;
                ctx.call(Function::EraseQuest, vec![Val::from(8221)])?;
                ctx.call(Function::SetQuest, vec![Val::from(8222)])?;
                ctx.call(Function::GetExperience, vec![Val::from(1260), Val::from(0)])?;
                ctx.next()?;
                ctx.lines_as(
                    "Shurank",
                    args![
                        "Ah, once again, I have",
                        "another task for you to perform. Please visit Dequ'ee in Geffen since it seems he has something",
                        "to ask of you."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Shurank",
                    args!["Hurry and meet with Dequ'ee and return to me when you complete whatever it is that he wishes for you to do."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Shurank",
                    args![
                        "Hmm...",
                        "You're not quite",
                        "there yet. In the",
                        "meantime, I encourage",
                        "you to double your efforts!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
        if !matched1 && subject1.loosely_equals(&Val::from(11)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines(args!["So have you", "learned how to use", "the Endure skill yet?"])?;
            ctx.next()?;
            l_chk_endure = ctx.call(Function::GetSkillLv, vec![Val::from("SM_ENDURE")])?;
            if (l_chk_endure.clone().number()? > 0 && l_chk_endure.clone().number()? < 2) {
                ctx.lines_as(
                    "Shurank",
                    args![
                        "I see that you",
                        "know how to use the",
                        "Endure skill. Still, it wouldn't hurt if I review the basic details regarding its use..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Shurank", args!["As you've noticed, receiving any damage causes you to reel in pain, stunning you for an instant. Thus, rapidly attacking enemies are dangerous since it's difficult to counter or escape them."])?;
                ctx.next()?;
                ctx.lines_as("Shunrank", args!["Activating Endure enables you to shrug off attacks. You'll receive damage, but enemy attacks will not impede you as much. This means you can counter enemy attacks or escape if you need to."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Shurank",
                    args![
                        "Of course, you can't use Endure forever. However, the more you increase the level of the Endure",
                        "skill, the longer Endure's effect will last. Also, remember that Endure also has a skill delay."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Shurank",
                    args![
                        "Since you've learned the Endure skill already, I don't think I need to explain it any further. You've done well."
                    ],
                )?;
                ctx.call(Function::EraseQuest, vec![Val::from(8221)])?;
                ctx.call(Function::SetQuest, vec![Val::from(8222)])?;
                ctx.var("tu_swordman").set(Val::from(13))?;
                ctx.next()?;
                ctx.lines_as(
                    "Shurank",
                    args![
                        "Ah, once again, I have",
                        "another task for you to perform. Please visit Dequ'ee in Geffen since it seems he has something",
                        "to ask of you."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Shurank",
                    args!["Hurry and meet with Dequ'ee and return to me when you complete whatever it is that he wishes for you to do."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if l_chk_endure.clone().number()? > 1 {
                ctx.lines_as(
                    "Shurank",
                    args!["From that gleen of toughness upon your skin, I see now that I was foolish to ask. Well done~"],
                )?;
                ctx.var("tu_swordman").set(Val::from(13))?;
                ctx.call(Function::EraseQuest, vec![Val::from(8221)])?;
                ctx.call(Function::SetQuest, vec![Val::from(8222)])?;
                ctx.call(Function::GetExperience, vec![Val::from(1260), Val::from(0)])?;
                ctx.next()?;
                ctx.lines_as(
                    "Shurank",
                    args![
                        "Ah, once again, I have",
                        "another task for you to perform. Please visit Dequ'ee in Geffen since it seems he has something",
                        "to ask of you."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Shurank",
                    args!["Hurry and meet with Dequ'ee and return to me when you complete whatever it is that he wishes for you to do."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Shurank",
                    args![
                        "No...?",
                        "I recommend that you learn it as soon as you can. Endure is an invaluable skill for any Swordman to have!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
        if !matched1 && subject1.loosely_equals(&Val::from(10)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines(args![
                "The time has come",
                "for me to tell you",
                "all I know about",
                "the Endure skill."
            ])?;
            ctx.next()?;
            l_chk_endure = ctx.call(Function::GetSkillLv, vec![Val::from("SM_ENDURE")])?;
            if l_chk_endure.clone() == 0 {
                ctx.lines_as("Shurank", args!["As you've noticed, receiving any damage causes you to reel in pain, stunning you for an instant. Thus, rapidly attacking enemies are dangerous since it's difficult to counter or escape them."])?;
                ctx.next()?;
                ctx.lines_as("Shunrank", args!["Activating Endure enables you to shrug off attacks. You'll receive damage, but enemy attacks will not impede you as much. This means you can counter enemy attacks or escape if you need to."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Shurank",
                    args![
                        "Of course, you can't use Endure forever. However, the more you increase the level of the Endure",
                        "skill, the longer Endure's effect will last. Also, remember that Endure also has a skill delay."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Shurank",
                    args![
                        "Alright. Now I want you to train in the use of the Endure skill. You can become a great Swordman",
                        "if you can endure attacks",
                        "from your enemies!"
                    ],
                )?;
                ctx.var("tu_swordman").set(Val::from(11))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(8220), Val::from(8221)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if (l_chk_endure.clone().number()? > 0 && l_chk_endure.clone().number()? < 2) {
                ctx.lines_as(
                    "Shurank",
                    args![
                        "I see that you",
                        "know how to use the",
                        "Endure skill. Still, it wouldn't hurt if I review the basic details regarding its use..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Shurank", args!["As you've noticed, receiving any damage causes you to reel in pain, stunning you for an instant. Thus, rapidly attacking enemies are dangerous since it's difficult to counter or escape them."])?;
                ctx.next()?;
                ctx.lines_as("Shurank", args!["Activating Endure enables you to shrug off attacks. You'll receive damage, but enemy attacks will not impede you as much. This means you can counter enemy attacks or escape if you need to."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Shurank",
                    args![
                        "Of course, you can't use Endure forever. However, the more you increase the level of the Endure",
                        "skill, the longer Endure's effect will last. Also, remember that Endure also has a skill delay."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Shurank",
                    args![
                        "Since you've learned the Endure skill already, I don't think I need to explain it any further. You've done well."
                    ],
                )?;
                ctx.var("tu_swordman").set(Val::from(13))?;
                ctx.call(Function::EraseQuest, vec![Val::from(8221)])?;
                ctx.call(Function::SetQuest, vec![Val::from(8222)])?;
                ctx.next()?;
                ctx.lines_as(
                    "Shurank",
                    args![
                        "Ah, once again, I have",
                        "another task for you to perform. Please visit Dequ'ee in Geffen since it seems he has something",
                        "to ask of you."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Shurank",
                    args!["Hurry and meet with Dequ'ee and return to me when you complete whatever it is that he wishes for you to do."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if l_chk_endure.clone().number()? > 1 {
                ctx.lines_as("Shurank", args!["Hm. But judging from the gleen of toughness on your skin, I suppose teaching you about Endure would be a waste of your time. Well done!"])?;
                ctx.var("tu_swordman").set(Val::from(13))?;
                ctx.call(Function::EraseQuest, vec![Val::from(8221)])?;
                ctx.call(Function::SetQuest, vec![Val::from(8222)])?;
                ctx.call(Function::GetExperience, vec![Val::from(1260), Val::from(0)])?;
                ctx.next()?;
                ctx.lines_as(
                    "Shurank",
                    args![
                        "Ah, once again, I have",
                        "another task for you to perform. Please visit Dequ'ee in Geffen since it seems he has something",
                        "to ask of you."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Shurank",
                    args!["Hurry and meet with Dequ'ee and return to me when you complete whatever it is that he wishes for you to do."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
        if !matched1 && subject1.loosely_equals(&Val::from(9)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines(args![
                "So have you been",
                "learning more about",
                "the use of the Provoke skill?"
            ])?;
            ctx.next()?;
            l_chk_provoke = ctx.call(Function::GetSkillLv, vec![Val::from("SM_PROVOKE")])?;
            if (l_chk_provoke.clone().number()? > 4 && l_chk_provoke.clone().number()? < 10) {
                ctx.lines_as("Shurank", args!["Ah, I see that you have a fairly good understanding of the Provoke skill. Let me reward you in this small way for your efforts."])?;
                ctx.var("tu_swordman").set(Val::from(10))?;
                ctx.call(Function::GetExperience, vec![Val::from(1120), Val::from(0)])?;
                ctx.next()?;
                ctx.lines_as("Shurank", args!["Next time we speak, I will tell you what I know about the ^5D478BEndure^000000 skill. When you're ready to learn, come back to me."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if l_chk_provoke.clone() == 10 {
                ctx.lines_as(
                    "Shurank",
                    args![
                        "Y-you've mastered",
                        "Provoke? Impressive!",
                        "I see that you've made",
                        "up your mind to become",
                        "an outstanding Swordman.",
                        "Here is a small reward..."
                    ],
                )?;
                ctx.var("tu_swordman").set(Val::from(10))?;
                ctx.call(Function::GetExperience, vec![Val::from(1260), Val::from(0)])?;
                ctx.next()?;
                ctx.lines_as("Shurank", args!["Next time we speak, I will tell you what I know about the ^5D478BEndure^000000 skill. When you're ready to learn, come back to me."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Shurank",
                    args![
                        "Hmm, I acknowledge",
                        "your progress, but you're not quite there yet. You've got to become more skilled in Provoke before",
                        "I can continue my lectures."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
        if !matched1 && subject1.loosely_equals(&Val::from(8)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines(args![
                "Ah, you've returned.",
                "So how has your training",
                "in the use of the Provoke",
                "skill been progressing?"
            ])?;
            ctx.next()?;
            l_chk_provoke = ctx.call(Function::GetSkillLv, vec![Val::from("SM_PROVOKE")])?;
            if (l_chk_provoke.clone().number()? > 0 && l_chk_provoke.clone().number()? < 5) {
                ctx.lines_as("Shurank", args!["Ah, now I can see that you", "know how to use it. That's a vast improvement over not being able to use it at all. Let me refesh you on Provoke's details."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Shurank",
                    args![
                        "In order to effectively use Provoke, you must understand",
                        "how it works: Provoke infuriates your opponents, making them",
                        "want to attack you."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Shurank", args!["When provoked, an enemy will have greater attack strength, making it more dangerous, but will also have reduced defense, making your enemy easier to defeat."])?;
                ctx.next()?;
                ctx.lines_as("Shurank", args!["Ideally, you may wish to use Provoke on enemies when they are weakened enough so that you can finish them off more quickly. But that's up to you."])?;
                ctx.next()?;
                ctx.lines_as("Shurank", args!["Also, the higher the level of Provoke that is used, the higher the attack increase and defense decrease of your target."])?;
                ctx.next()?;
                ctx.lines_as("Shurank", args!["As an Active Skill, Provoke will consume a small amount of SP, just like Bash. So be careful not to run out of SP when using this skill."])?;
                ctx.next()?;
                ctx.lines_as("Shurank", args!["Alright, come back to me when you become sufficiently skilled in the use of Provoke. In the meantime, keep training."])?;
                ctx.var("tu_swordman").set(Val::from(9))?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if (l_chk_provoke.clone().number()? > 4 && l_chk_provoke.clone().number()? < 10) {
                ctx.lines_as("Shurank", args!["Ah, I see that you have a fairly good understanding of the Provoke skill. Let me reward you in this small way for your efforts."])?;
                ctx.var("tu_swordman").set(Val::from(10))?;
                ctx.call(Function::GetExperience, vec![Val::from(1120), Val::from(0)])?;
                ctx.next()?;
                ctx.lines_as("Shurank", args!["Next time we speak, I will tell you what I know about the ^5D478BEndure^000000 skill. When you're ready to learn, come back to me."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if l_chk_provoke.clone() == 10 {
                ctx.lines_as(
                    "Shurank",
                    args![
                        "Y-you've mastered",
                        "Provoke? Impressive!",
                        "I see that you've made",
                        "up your mind to become",
                        "an outstanding Swordman.",
                        "Here is a small reward..."
                    ],
                )?;
                ctx.var("tu_swordman").set(Val::from(10))?;
                ctx.call(Function::GetExperience, vec![Val::from(1260), Val::from(0)])?;
                ctx.next()?;
                ctx.mes("Next time we speak, I will tell you what I know about the ^5D478BEndure^000000 skill. When you're ready to learn, come back to me.")?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as("Shurank", args!["Still haven't learned Provoke, eh? As a Swordman, I believe that it's important that you at least be able to use Provoke!"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
        if !matched1 && subject1.loosely_equals(&Val::from(7)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines(args![
                "Ah, you've returned. For a new Swordman, you are doing quite",
                "well. So what is the message",
                "from Dequ'ee?"
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Shurank",
                args![
                    "I see...",
                    "Well, now that we taken care",
                    "of that, let me continue your instruction on Swordman skills. This time, we'll cover Provoke."
                ],
            )?;
            ctx.next()?;
            l_chk_provoke = ctx.call(Function::GetSkillLv, vec![Val::from("SM_PROVOKE")])?;
            if l_chk_provoke.clone() == 0 {
                ctx.lines_as(
                    "Shurank",
                    args![
                        "You haven't",
                        "learned Provoke yet?",
                        "Hm. All the more reason why",
                        "I should explain it to you then."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Shurank",
                    args![
                        "In order to effectively use Provoke, you must understand",
                        "how it works: Provoke infuriates your opponents, making them",
                        "want to attack you."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Shurank", args!["When provoked, an enemy will have greater attack strength, making it more dangerous, but will also have reduced defense, making your enemy easier to defeat."])?;
                ctx.next()?;
                ctx.lines_as("Shurank", args!["Ideally, you may wish to use Provoke on enemies when they are weakened enough so that you can finish them off more quickly. But that's up to you."])?;
                ctx.next()?;
                ctx.lines_as("Shurank", args!["Also, the higher the level of Provoke that is used, the higher the attack increase and defense decrease of your target."])?;
                ctx.next()?;
                ctx.lines_as("Shurank", args!["As an Active Skill, Provoke will consume a small amount of SP, just like Bash. So be careful not to run out of SP when using this skill."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Shurank",
                    args![
                        "Now I ask that you train yourself in the use of Provoke. Come back",
                        "to me when you have a sufficient understanding of the use of the Provoke skill."
                    ],
                )?;
                ctx.var("tu_swordman").set(Val::from(8))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(8217), Val::from(8218)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if (l_chk_provoke.clone().number()? > 0 && l_chk_provoke.clone().number()? < 5) {
                ctx.lines_as("Shurank", args!["Hm. You seem to have a basic understanding of Provoke, but let me give you a few details so that you can better understand the use of that skill."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Shurank",
                    args![
                        "In order to effectively use Provoke, you must understand",
                        "how it works: Provoke infuriates your opponents, making them",
                        "want to attack you."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Shurank", args!["When provoked, an enemy will have greater attack strength, making it more dangerous, but will also have reduced defense, making your enemy easier to defeat."])?;
                ctx.next()?;
                ctx.lines_as("Shurank", args!["Ideally, you may wish to use Provoke on enemies when they are weakened enough so that you can finish them off more quickly. But that's up to you."])?;
                ctx.next()?;
                ctx.lines_as("Shurank", args!["Also, the higher the level of Provoke that is used, the higher the attack increase and defense decrease of your target."])?;
                ctx.next()?;
                ctx.lines_as("Shurank", args!["As an Active Skill, Provoke will consume a small amount of SP, just like Bash. So be careful not to run out of SP when using this skill."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Shurank",
                    args![
                        "Now I ask that you train yourself in the use of Provoke. Come back",
                        "to me when you have a sufficient understanding of the use of the Provoke skill."
                    ],
                )?;
                ctx.var("tu_swordman").set(Val::from(9))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(8218), Val::from(8219)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if (l_chk_provoke.clone().number()? > 4 && l_chk_provoke.clone().number()? < 10) {
                ctx.lines_as("Shurank", args!["Ah, I see that you have a fairly good understanding of the Provoke skill. Let me reward you in this small way for your efforts."])?;
                ctx.var("tu_swordman").set(Val::from(10))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(8219), Val::from(8220)])?;
                ctx.call(Function::GetExperience, vec![Val::from(1120), Val::from(0)])?;
                ctx.next()?;
                ctx.lines_as("Shurank", args!["Next time we speak, I will tell you what I know about the ^5D478BEndure^000000 skill. When you're ready to learn, come back to me."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if l_chk_provoke.clone() == 10 {
                ctx.lines_as(
                    "Shurank",
                    args![
                        "Y-you've mastered",
                        "Provoke? Impressive!",
                        "I see that you've made",
                        "up your mind to become",
                        "an outstanding Swordman.",
                        "Here is a small reward..."
                    ],
                )?;
                ctx.var("tu_swordman").set(Val::from(10))?;
                ctx.call(Function::CompleteQuest, vec![Val::from(8218)])?;
                ctx.call(Function::CompleteQuest, vec![Val::from(8219)])?;
                ctx.call(Function::CompleteQuest, vec![Val::from(8220)])?;
                ctx.call(Function::SetQuest, vec![Val::from(8221)])?;
                ctx.call(Function::GetExperience, vec![Val::from(1260), Val::from(0)])?;
                ctx.next()?;
                ctx.lines_as("Shurank", args!["Next time we speak, I will tell you what I know about the ^5D478BEndure^000000 skill. When you're ready to learn, come back to me."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
        if !matched1 && subject1.loosely_equals(&Val::from(6)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines(args!["I wish for you to find a Knight named ^5D478BDequ'ee^000000 in ^5D478BGeffen^000000. Listen carefully, this is the message", "I want you to give him."])?;
            ctx.next()?;
            ctx.lines_as(
                "Shurank",
                args![
                    "^0000FFWhat happened",
                    "to the murderer?^000000",
                    "^0000FFDid you find out who he is? If you did, what are we supposed to do now?^000000"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Shurank",
                args![
                    "If you use those exact words, Dequ'ee will understand what",
                    "you're talking about. Remember his response and let me know his answer."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Shurank", args!["It shouldn't be too hard fighting the monsters on the way to Geffen, but if you encounter any that are too strong, there's no shame in running away."])?;
            ctx.next()?;
            ctx.lines_as(
                "Shurank",
                args![
                    "But now that I think about it, it may take you a while to travel to Geffen. I shall send you there myself! Good luck!"
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::Warp, vec![Val::from("gef_fild07"), Val::from(35), Val::from(192)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        if !matched1 && subject1.loosely_equals(&Val::from(5)) {
            matched1 = true;
        }
        if matched1 {
            ctx.mes("Ah, you've returned. I know I said that I would teach you what I know about the Provoke skill, but...")?;
            ctx.next()?;
            ctx.lines_as("Shurank", args!["I actually have an important task that I'd like for you to perform. It's not very difficult: All you must do is deliver a message."])?;
            ctx.next()?;
            ctx.lines_as("Shurank", args!["I wish for you to find a Knight named ^5D478BDequ'ee^000000 in ^5D478BGeffen^000000. Listen carefully, this is the message", "I want you to give him."])?;
            ctx.next()?;
            ctx.lines_as(
                "Shurank",
                args![
                    "^0000FFWhat happened",
                    "to the murderer?^000000",
                    "^0000FFDid you find out who he is? If you did, what are we supposed to do now?^000000"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Shurank",
                args![
                    "If you use those exact words, Dequ'ee will understand what",
                    "you're talking about. Remember his response and let me know his answer."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Shurank", args!["It shouldn't be too hard fighting the monsters on the way to Geffen, but if you encounter any that are too strong, there's no shame in running away."])?;
            ctx.next()?;
            ctx.lines_as(
                "Shurank",
                args![
                    "Although there's a Warp service,",
                    "I recommend traveling by foot. You cannot grow strong without testing your strength and practicing your skills!"
                ],
            )?;
            ctx.var("tu_swordman").set(Val::from(6))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(8215), Val::from(8216)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        if !matched1 && subject1.loosely_equals(&Val::from(4)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines(args![
                "Since you've mastered Bash,",
                "I feel that you're ready to learn more about Swordman skills."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Shurank",
                args![
                    "Now, you must be familiar with the ^5D478BIncrease HP Recovery Skill^000000. When",
                    "it comes to damage, you must be",
                    "able to both take it and dish it out."
                ],
            )?;
            ctx.next()?;
            l_chk_hp = ctx.call(Function::GetSkillLv, vec![Val::from("SM_RECOVERY")])?;
            if l_chk_hp.clone() == 0 {
                ctx.lines_as(
                    "Shurank",
                    args![
                        "But judging from that",
                        "scrawny frame of yours,",
                        "its looks like you haven't learned the Increase HP Recovery Skill at all...!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Shurank",
                    args![
                        "The Increase HP Recovery Skill",
                        "is a passive skill that is always in effect and does not consume",
                        "any of your SP."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Shurank", args!["It enables you to regenerate your health twice as fast. This might not seem like a big deal, but it will reduce the time you need to rest from battles."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Shurank",
                    args![
                        "I strongly suggest that you",
                        "learn this skill! Of course, there are other Swordman skills that are just as important, though..."
                    ],
                )?;
                ctx.var("tu_swordman").set(Val::from(5))?;
                ctx.next()?;
            } else if l_chk_hp.clone() == 10 {
                ctx.lines_as(
                    "Shurank",
                    args![
                        "Ah! That healthy glow!",
                        "I see that you've mastered this skill already. Let me give you a little reward for your hard training!"
                    ],
                )?;
                ctx.var("tu_swordman").set(Val::from(5))?;
                ctx.call(Function::GetExperience, vec![Val::from(1120), Val::from(0)])?;
                ctx.next()?;
            } else {
                ctx.lines_as("Shurank", args!["Ah, and I see that you've already learned a little bit about it. But let me briefly tell you more about Increase HP Recovery."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Shurank",
                    args![
                        "The Increase HP Recovery Skill",
                        "is a passive skill that is always in effect and does not consume",
                        "any of your SP."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Shurank", args!["It enables you to regenerate your health twice as fast. This might not seem like a big deal, but it will reduce the time you need to rest from battles."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Shurank",
                    args![
                        "Increase HP Recovery is a fairly important skill, but whether or not you want to master that is really up to you."
                    ],
                )?;
                ctx.var("tu_swordman").set(Val::from(5))?;
                ctx.next()?;
            }
            ctx.call(Function::SetQuest, vec![Val::from(8215)])?;
            ctx.lines_as("Shurank", args!["By now, I believe that you know enough about the Increase HP Recovery skill. Next time, I shall teach you what I know about the Provoke skill."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        if !matched1 && subject1.loosely_equals(&Val::from(3)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines(args![
                "If you want to be",
                "able to use the full",
                "power of the Bash skill,",
                "you better have mastered it",
                "if you want me to acknowledge",
                "you as a fellow Swordman."
            ])?;
            ctx.next()?;
            l_chk_bash = ctx.call(Function::GetSkillLv, vec![Val::from("SM_BASH")])?;
            if l_chk_bash.clone() == 10 {
                ctx.lines_as(
                    "Shurank",
                    args![
                        "Those calluses...!",
                        "Only a master of the Bash skill has those kinds of hands. Great work, Swordman. I'm very impressed."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Shurank", args!["Here's a small prize in recognition of your efforts thus far. But don't relax yet! You still have a long way to go before grasping", "all of the basics."])?;
                ctx.var("tu_swordman").set(Val::from(4))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(8213), Val::from(8214)])?;
                ctx.call(Function::GetExperience, vec![Val::from(970), Val::from(0)])?;
                ctx.call(Function::GetItem, vec![Val::from(2503), Val::from(1)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as("Shurank", args!["Hmm, you might be able to", "use Bash quite well, but you are not a master of it yet. I encourage you to master Bash in order to unleash its true potential!"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
        if !matched1 && subject1.loosely_equals(&Val::from(2)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines(args![
                "I hope you've been",
                "training yourself in using",
                "Bash. If you want to grow as a Swordman, you will have great",
                "need to master this skill."
            ])?;
            ctx.next()?;
            l_chk_bash = ctx.call(Function::GetSkillLv, vec![Val::from("SM_BASH")])?;
            if (l_chk_bash.clone().number()? > 4 && l_chk_bash.clone().number()? < 10) {
                ctx.lines_as(
                    "Shurank",
                    args!["Hm. It doesn't look like you've gained mastery of Bash, but it seems like you're trying hard."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Shurank",
                    args!["Let me review the use of the Bash skill to you. If you've heard this before, it'll be a good refresher."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Shurank",
                    args![
                        "Bash is an active skill, mastered at Level 10, which allows you to smash your target. In essense,",
                        "it's a highly damaging attack!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Shurank",
                    args![
                        "Since it's an active skill, Bash consumes SP each time it is",
                        "used. If you're not careful, you'll be out of SP in no time...!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Shurank",
                    args![
                        "Here's a little reward for all of your hard work. I hope you realize the importance of using Bash as",
                        "you grow stronger."
                    ],
                )?;
                ctx.var("tu_swordman").set(Val::from(3))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(8212), Val::from(8213)])?;
                ctx.call(Function::GetExperience, vec![Val::from(830), Val::from(0)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if l_chk_bash.clone() == 10 {
                ctx.lines_as(
                    "Shurank",
                    args![
                        "Those calluses...!",
                        "Only a master of the Bash skill has those kinds of hands. Great work, Swordman. I'm very impressed."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Shurank", args!["Here's a small prize in recognition of your efforts thus far. But don't relax yet! You still have a long way to go before grasping", "all of the basics."])?;
                ctx.var("tu_swordman").set(Val::from(4))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(8212), Val::from(8214)])?;
                ctx.call(Function::GetExperience, vec![Val::from(970), Val::from(0)])?;
                ctx.call(Function::GetItem, vec![Val::from(2503), Val::from(1)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as("Shurank", args!["Hmm, you might be able to", "use Bash quite well, but you are not a master of it yet. I encourage you to master Bash in order to unleash its true potential!"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
        if !matched1 && subject1.loosely_equals(&Val::from(1)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines(args![
                "Have you been learning",
                "the art of using Bash? I fervently believe that the Bash skill is the essense of Swordmanship!"
            ])?;
            ctx.next()?;
            l_chk_bash = ctx.call(Function::GetSkillLv, vec![Val::from("SM_BASH")])?;
            if (l_chk_bash.clone().number()? > 0 && l_chk_bash.clone().number()? < 5) {
                ctx.lines_as(
                    "Shurank",
                    args![
                        "I see that you understand",
                        "a little bit about the Bash skill. But still, it's not enough. Let me explain in detail..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Shurank",
                    args![
                        "Bash is an active skill, mastered at Level 10, which allows you to smash your target. In essense,",
                        "it's a highly damaging attack!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Shurank",
                    args![
                        "Since it's an active skill, Bash consumes SP each time it is",
                        "used. If you're not careful, you'll be out of SP in no time...!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Shurank", args!["Well, that's all I can tell you about Bash for now. As for its subtle nuances, you'll have to experience them for yourself", "in battle."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Shurank",
                    args![
                        "Still, for your training efforts, let me give you a humble reward. But remember, we still have more",
                        "of the fundamentals to cover!"
                    ],
                )?;
                ctx.var("tu_swordman").set(Val::from(2))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(8211), Val::from(8212)])?;
                ctx.call(Function::GetExperience, vec![Val::from(580), Val::from(0)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if (l_chk_bash.clone().number()? > 4 && l_chk_bash.clone().number()? < 10) {
                ctx.lines_as(
                    "Shurank",
                    args![
                        "Hmm...",
                        "Still haven't mastered Bash, eh? Well, in any case, I think you've made some progress."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Shurank",
                    args!["Let me explain the use of the Bash skill to you. If you've heard this before, it'll be a good refresher."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Shurank",
                    args![
                        "I see that you understand",
                        "a little bit about the Bash skill. But still, it's not enough. Let me explain in detail..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Shurank",
                    args![
                        "Bash is an active skill, mastered at Level 10, which allows you to smash your target. In essense,",
                        "it's a highly damaging attack!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Shurank",
                    args![
                        "Since it's an active skill, Bash consumes SP each time it is",
                        "used. If you're not careful, you'll be out of SP in no time...!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Shurank", args!["Well, that's all I can tell you about Bash for now. As for its subtle nuances, you'll have to experience them for yourself", "in battle."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Shurank",
                    args!["For your efforts and patience, I offer you this small reward. Please take it and grow even stronger..."],
                )?;
                ctx.var("tu_swordman").set(Val::from(3))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(8211), Val::from(8213)])?;
                ctx.call(Function::GetExperience, vec![Val::from(830), Val::from(0)])?;
                ctx.next()?;
                ctx.lines_as("Shurank", args!["I believe that every true Swordman should master the Bash skill. If you ever do master the skill as a Swordman, come back to me."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if l_chk_bash.clone() == 10 {
                ctx.lines_as(
                    "Shurank",
                    args![
                        "Those calluses...!",
                        "Only a master of the Bash skill has those kinds of hands. Great work, Swordman. I'm very impressed."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Shurank", args!["Here's a small prize in recognition of your efforts thus far. But don't relax yet! You still have a long way to go before grasping", "all of the basics."])?;
                ctx.var("tu_swordman").set(Val::from(4))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(8211), Val::from(8214)])?;
                ctx.call(Function::GetExperience, vec![Val::from(970), Val::from(0)])?;
                ctx.call(Function::GetItem, vec![Val::from(2503), Val::from(1)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Shurank",
                    args![
                        "You still can't even use Bash...? What's the use of a sword if not to smash things?! Come back to me",
                        "once you've learned how to use",
                        "that skill!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
        if !matched1 && subject1.loosely_equals(&Val::from(0)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines(args![
                "Ah, a comrade in arms.",
                "Allow me to introduce myself. I am Shurank Chainlier, a Knight in the service of the Prontera Chivalry."
            ])?;
            ctx.next()?;
            ctx.lines_as("Shurank", args!["It concerns me deeply that the Swordmen of today may not truly understand the way of the sword. And to think that they may become Knights later..."])?;
            ctx.next()?;
            ctx.lines_as(
                "Shurank",
                args![
                    "Have you been training",
                    "to be an expert Swordman?",
                    "If so, you must learn the",
                    "fundamentals, the first of",
                    "which is the ^5D478BBash^000000 skill."
                ],
            )?;
            ctx.next()?;
            l_chk_bash = ctx.call(Function::GetSkillLv, vec![Val::from("SM_BASH")])?;
            if l_chk_bash.clone() == 0 {
                ctx.lines_as("Shurank", args!["^333333*Gasp*^000000", "Those soft, delicate hands! You've never learned how to use Bash, haven't you?! How do you expect to lead battles without knowing Bash?"])?;
                ctx.next()?;
                ctx.lines_as("Shurank", args!["Now, if you wish to become a true Swordman, you must learn the Bash skill. There's no question about it: You've got to know these basics! Take this, and learn to Bash!"])?;
                ctx.var("tu_swordman").set(Val::from(1))?;
                ctx.call(Function::GetExperience, vec![Val::from(490), Val::from(0)])?;
                ctx.call(Function::SetQuest, vec![Val::from(8211)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if (l_chk_bash.clone().number()? > 0 && l_chk_bash.clone().number()? < 5) {
                ctx.lines_as(
                    "Shurank",
                    args![
                        "I see that you understand",
                        "a little bit about the Bash skill. But still, it's not enough. Let me explain in detail..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Shurank",
                    args![
                        "Bash is an active skill, mastered at Level 10, which allows you to smash your target. In essense,",
                        "it's a highly damaging attack!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Shurank",
                    args![
                        "Since it's an active skill, Bash consumes SP each time it is",
                        "used. If you're not careful, you'll be out of SP in no time...!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Shurank", args!["Well, that's all I can tell you about Bash for now. As for its subtle nuances, you'll have to experience them for yourself", "in battle."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Shurank",
                    args![
                        "Still, for your training efforts, let me give you a humble reward. But remember, we still have more",
                        "of the fundamentals to cover!"
                    ],
                )?;
                ctx.var("tu_swordman").set(Val::from(2))?;
                ctx.call(Function::SetQuest, vec![Val::from(8212)])?;
                ctx.call(Function::GetExperience, vec![Val::from(580), Val::from(0)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if (l_chk_bash.clone().number()? > 4 && l_chk_bash.clone().number()? < 10) {
                ctx.lines_as(
                    "Shurank",
                    args!["I see that you've gained some proficiency with the Bash skill. But still, it's not enough. Let me explain..."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Shurank",
                    args![
                        "Bash is an active skill, mastered at Level 10, which allows you to smash your target. In essense,",
                        "it's a highly damaging attack!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Shurank",
                    args![
                        "Since it's an active skill, Bash consumes SP each time it is",
                        "used. If you're not careful, you'll be out of SP in no time...!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Shurank", args!["Well, that's all I can tell you about Bash for now. As for its subtle nuances, you'll have to experience them for yourself", "in battle."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Shurank",
                    args![
                        "Still, for your training efforts, let me give you a humble reward. But remember, we still have more",
                        "of the fundamentals to cover!"
                    ],
                )?;
                ctx.var("tu_swordman").set(Val::from(3))?;
                ctx.call(Function::SetQuest, vec![Val::from(8213)])?;
                ctx.call(Function::GetExperience, vec![Val::from(830), Val::from(0)])?;
                ctx.next()?;
                ctx.lines_as(
                    "Shurank",
                    args!["But if you wish to become an expert Swordman in my eyes, you must master Bash!"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if l_chk_bash.clone() == 10 {
                ctx.lines_as(
                    "Shurank",
                    args![
                        "Those calluses...!",
                        "Only a master of the Bash skill has those kinds of hands. Great work, Swordman. I'm very impressed."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Shurank", args!["Here's a small prize in recognition of your efforts thus far. But don't relax yet! You still have a long way to go before grasping", "all of the basics."])?;
                ctx.var("tu_swordman").set(Val::from(4))?;
                ctx.call(Function::SetQuest, vec![Val::from(8214)])?;
                ctx.call(Function::GetExperience, vec![Val::from(970), Val::from(0)])?;
                ctx.call(Function::GetItem, vec![Val::from(2503), Val::from(1)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
    ctx.lines(args!["...", "......"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn shurank(ctx: &Ctx) -> Script {
    shurank_body(ctx, Vec::new()).map(|_| ())
}

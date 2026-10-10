use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn guildsman_asn_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("Upper").get()? == 1 {
        ctx.lines_as(
            "Ferocious-looking guy",
            args!["Hm? You....?", "I sense that you're different than most people..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ferocious-looking guy",
            args!["I've never met anyone as intimidating as you! For some reason, I don't like you. I think you should leave!"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("SkillPoint").get()?.is_true() {
        ctx.lines_as(
            "Ferocious-looking guy",
            args!["You can't change your job if you have any unused skill points from the 1st job. You better go and use those up first."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("assin_q").get()? == 4 {
        ctx.lines_as(
            "Ferocious-looking guy",
            args!["Oh, stop making that face. Can you really be in that much pain?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ferocious-looking guy",
            args!["Wah wah wah, you're hurting, I can see that. Look, I'll restore HP and SP. Happy?"],
        )?;
        ctx.call(Function::PercentHeal, vec![Val::from(100), Val::from(100)])?;
        ctx.next()?;
        ctx.lines_as(
            "Ferocious-looking guy",
            args![
                "Is it that hard to stay alive?",
                "Why don't you try harder next time? You can't force yourself too hard to become an Assassin..."
            ],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from("I will become an Assassin no matter what!:Oh man, I gotta take a break.")],
        )?) == 1
        {
            ctx.lines_as("Ferocious-looking guy", args!["Oh...", "Well then,", "go for it!"])?;
            ctx.close_window()?;
            ctx.var("assin_q").set(Val::from(0))?;
            ctx.call(Function::Warp, vec![Val::from("in_moc_16"), Val::from(19), Val::from(76)])?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Ferocious-looking guy",
            args!["Take a break? Oh alright, have it your way. When you feel like you're ready to become an Assassin, come back."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ferocious-looking guy",
            args!["You'll have to walk if you want to get back to town. Oh, and don't forget to save your spawn point, alright?"],
        )?;
        ctx.close_window()?;
        ctx.var("assin_q").set(Val::from(0))?;
        ctx.var("assin_q2").set(Val::from(0))?;
        if ctx.var("assin_q3").get()?.number()? < 3 {
            ctx.var("assin_q3").set(Val::from(0))?;
        }
        ctx.call(
            Function::SavePoint,
            vec![Val::from("in_moc_16"), Val::from(18), Val::from(14), Val::from(1), Val::from(1)],
        )?;
        ctx.call(Function::Warp, vec![Val::from("in_moc_16"), Val::from(18), Val::from(14)])?;
        return Err(Stop::End);
    }
    if ((ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_THIEF")?)
        && ctx.call(Function::CountItem, vec![Val::from(1008)])? == 0)
        && ctx.var("assin_q").get()?.number()? > 7)
    {
        ctx.lines_as("Assassin Expert 'Huey'", args!["Hey, what happened...? How come you didn't bring the ^006699Necklace of Oblivion^000000? You're supposed to carry that with you, so where is it?"])?;
        ctx.next()?;
        ctx.lines_as("Assassin Expert 'Huey'", args!["You get better get that ^006699Necklace of Oblivion^000000 again before the guildmaster finds out! Hurry, and do your best to get it!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Assassin Expert 'Huey'",
            args!["When you finally succeed in getting it, bring it to me! ^666666*Sigh...*^000000"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ((ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_THIEF")?)
        && ctx.call(Function::CountItem, vec![Val::from(1008)])?.number()? > 0)
        && ctx.var("assin_q").get()?.number()? > 7)
    {
        ctx.lines_as("Assassin Expert 'Huey'", args!["Well well well, you got it. Congratulations! But since it's been clearly scratched, I can't accept it. You'll never become an Assassin!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Assassin Expert 'Huey'",
            args!["Hahahah~! I'm just joking, don't take it seriously. But I do need to check this necklace with the guildmaster first."],
        )?;
        ctx.next()?;
        ctx.lines_as("Assassin Expert 'Huey'", args!["..."])?;
        ctx.next()?;
        ctx.lines_as("Assassin Expert 'Huey'", args!["...", "......"])?;
        ctx.next()?;
        ctx.lines_as("Assassin Expert 'Huey'", args!["Alright!", "You've been approved!"])?;
        ctx.next()?;
        ctx.call(Function::DelItem, vec![Val::from(1008), Val::from(1)])?;
        ctx.call(Function::ChangeQuest, vec![Val::from(8007), Val::from(8008)])?;
        ctx.call(Function::CompleteQuest, vec![Val::from(8008)])?;
        shared::other_global_functions::job_change(ctx, vec![ctx.constant("JOB_ASSASSIN")?])?;
        shared::other_global_functions::f_clearjobvar(ctx, vec![])?;
        ctx.lines_as("Assassin Expert 'Huey'", args!["Now! Do your best to be a great Assassin! Travel with faith and kill with dignity. Come by anytime and pay us a visit. Once again, congratulations."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ((ctx.call(Function::CountItem, vec![Val::from(1008)])?.number()? > 0
            && ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_THIEF")?))
            && ctx.var("assin_q").get()?.number()? < 7)
        {
            ctx.lines_as("Ferocious-looking guy", args!["Eh?", "What do you want?"])?;
            ctx.next()?;
            ctx.lines_as("Ferocious-looking guy", args!["I see you're carrying a ^006699Necklace of Oblivion^000000... You want to become an Assassin, don't you? Let me check it..."])?;
            ctx.next()?;
            ctx.lines_as("Ferocious-looking guy", args!["..."])?;
            ctx.next()?;
            ctx.lines_as("Ferocious-looking guy", args!["...", "......"])?;
            ctx.next()?;
            ctx.lines_as(
                "Ferocious-looking guy",
                args!["Wait a second...", "Why you no good BASTARD! THIS IS A FAKE!"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Ferocious-looking guy",
                args!["How dare you think of trying to trick me with a fake! Are you stupid or what!? I should kill you..."],
            )?;
            ctx.close_window()?;
            ctx.call(Function::Warp, vec![Val::from("moc_fild16"), Val::from(206), Val::from(229)])?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Ferocious-looking guy",
                args![
                    "What brings you here?",
                    "I don't think I like the way you're looking at me... Punk."
                ],
            )?;
            ctx.next()?;
            if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_NOVICE")?) {
                ctx.lines_as(
                    "Ferocious-looking guy",
                    args!["Hey Newbie. You should really get out of here as soon as you can. I can't guarantee your safety."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_SWORDMAN")?) {
                    ctx.lines_as(
                        "Ferocious-looking guy",
                        args![
                            "What brings a man of the sword to this place? Why don't you try smashing stuff somewhere else, ya lunkhead."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_MAGE")?) {
                        ctx.lines_as("Ferocious-looking guy", args!["Now what would a magic user be doing here?"])?;
                        ctx.next()?;
                        ctx.lines_as("Ferocious-looking guy", args!["There's a library in Prontera and Juno where you're welcome, so why don't you make like a magic trick and disappear?"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_ARCHER")?) {
                            ctx.lines_as("Ferocious-looking guy", args!["Well well well.", "Look at that purdy bow."])?;
                            ctx.next()?;
                            ctx.lines_as("Ferocious-looking guy", args!["There aren't many Bowmen with the gall to even come close to this place. Well, what do you think you're doin' here?!"])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?) {
                                ctx.lines_as("Ferocious-looking guy", args!["I thought something smelled funny. What's a servant of God doing in this place? You don't belong here."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_MERCHANT")?) {
                                ctx.lines_as("Ferocious-looking guy", args!["We don't like greedy people around these parts. You better sell your stuff somewhere else, Moneybags."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ROGUE")?) {
                                ctx.lines_as("Ferocious-looking guy", args!["You look like you don't have a care in the world. Well, I hope you enjoy your rest while you stay here. It's okay, since the Rogue and Assassin Guilds have always gotten along pretty well."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Ferocious-looking guy",
                                    args!["By the way...", "Have you ever seen", "a girl named Markie?"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Ferocious-looking guy", args!["Markie...", "We promised that we'd be together forever. ^666666*Sigh...*^000000 I don't even think she remembers that promise anymore. Then again, we were pretty young back then..."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN")?) {
                                ctx.lines_as(
                                    "Assassin Expert 'Huey'",
                                    args![
                                        "Hey, I remember you~",
                                        "Wasn't your name, umm, I remember 'cause it sounded funny to me..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Assassin Expert 'Huey'",
                                    args![
                                        ((((Val::from(":+:") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                            + Val::from(":+:, right? No wait, just "))
                                            + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                            + Val::from(". Yeah, how's it goin'?"))
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Assassin Expert 'Huey'", args!["Unfortunately, I don't have any requests for you at this time from the guild. Just keep focusing on your training. Till then, see ya."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_THIEF")?)
                                && ctx.var("JobLevel").get()?.number()? > 39)
                            {
                                if ctx.var("SkillPoint").get()?.is_true() {
                                    ctx.lines_as("Ferocious-looking guy", args!["You can't change your job if you still have unused skill points from First Job. You better use up those skill points first."])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    ctx.lines_as("Ferocious-looking guy", args!["Hmm...", "A Thief...?"])?;
                                    ctx.next()?;
                                    ctx.lines_as("Ferocious-looking guy", args!["And a well-experienced Thief since I can't seem to find my wallet. We do need people like you, you know."])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Ferocious-looking guy",
                                        args!["So how about taking the next step and becoming an Assassin?"],
                                    )?;
                                    ctx.next()?;
                                    match runtime::select_values(
                                        ctx,
                                        &[Val::from(
                                            "Yes. I've picked my last pocket.:What's the requirements?:Maybe later, I need to steal some things first.",
                                        )],
                                    )? {
                                        1 => {
                                            ctx.lines_as(
                                                "Ferocious-looking guy",
                                                args!["It's been a while since I've received a guest. I'm sending", "you to the office."],
                                            )?;
                                            ctx.close_window()?;
                                            ctx.var("assin_q").set(Val::from(0))?;
                                            if ctx.call(Function::CheckQuest, vec![Val::from(8000)])? != -1 {
                                                ctx.call(Function::ChangeQuest, vec![Val::from(8000), Val::from(8001)])?;
                                            } else {
                                                ctx.call(Function::SetQuest, vec![Val::from(8001)])?;
                                            }
                                            ctx.call(Function::Warp, vec![Val::from("in_moc_16"), Val::from(19), Val::from(76)])?;
                                            return Err(Stop::End);
                                        }
                                        2 => {
                                            ctx.lines_as("Ferocious-looking guy", args!["Requirements? Well, first you need to be a Thief. Second, you need to be at least Thief job level 40."])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Ferocious-looking guy",
                                                args![
                                                    "And third, you need to pass a test to become an Assassin. You got",
                                                    "all that? If you're sure of your ability as a Thief, you won't have to worry."
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        3 => {
                                            ctx.lines_as(
                                                "Ferocious-looking guy",
                                                args!["Hmm...", "Alright then.", "But come back when", "you think you're ready."],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        _ => {}
                                    }
                                }
                            } else {
                                ctx.lines_as("Ferocious-looking guy", args!["Huh. You're not qualified to become an Assassin yet. There are requirements you need to meet first, you know."])?;
                                ctx.next()?;
                                ctx.lines_as("Ferocious-looking guy", args!["Well, keep training. You need to be at least job level 40, got it? But if you're above job level 40, that will probably be even better."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(Val::from(0))
}

pub fn guildsman_asn(ctx: &Ctx) -> Script {
    guildsman_asn_body(ctx, Vec::new()).map(|_| ())
}

fn guildsman_asn2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Assassin 'Khai'", args!["Umm?!"])?;
    ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
    ctx.next()?;
    ctx.lines_as("Assassin 'Khai'", args!["Come closer. I prefer to talk to people face to face. It really irritates me if I have to raise my voice, just so you can hear me.", "I feel irritated when somebody talks to me behind my back."])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn guildsman_asn2(ctx: &Ctx) -> Script {
    guildsman_asn2_body(ctx, Vec::new()).map(|_| ())
}

fn guildsman_asn2_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("assin_q2").get()? == 4 {
        ctx.lines_as("Assassin 'Khai'", args!["Ehhh?", "Didn't you just", "pass me a minute ago?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Assassin 'Khai'",
            args!["Eh...?!", "You failed?", "Even on the", "writing test?", "Bwahahahahaha!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Assassin 'Khai'",
            args!["Well...", "It's been a long time since", "I've met such a big failure."],
        )?;
        ctx.next()?;
        ctx.lines_as("Assassin 'Khai'", args!["HAH!", "Hahahahah~!", "Oh, you're killing me...."])?;
        ctx.next()?;
        ctx.lines_as(
            "Assassin 'Khai'",
            args!["Sorry for laughing, but this is hilarious! Hahaha~ So do you want me to give you some hints?"],
        )?;
        ctx.next()?;
        match runtime::select_values(
            ctx,
            &[Val::from(
                "I beg you, give me hints.:Don't laugh at me! Now, give me hints!:...Shut up, I don't need your help!",
            )],
        )? {
            1 => {
                ctx.lines_as(
                    "Assassin 'Khai'",
                    args![
                        "Haaahahahaha!!!",
                        "Well well, aren't we honest. You're not even an Assassin yet, but you're killing me, I tell you, killing me!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("The Anonymous One", args!["Ho ho ho..."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Assassin 'Khai'",
                    args!["Did you hear that Anonymous one?! 'I beg you, give me hints.' Hahahah!"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "The Anonymous One",
                    args!["Yes.", "This one is quite hilarious", "in a pathetic sort of way."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Assassin 'Khai'",
                    args!["Hahahahahahah!", "Soooooo, you wanted", "some hints, right?"],
                )?;
                ctx.next()?;
                ctx.lines_as("Assassin 'Khai'", args!["..."])?;
                ctx.next()?;
                ctx.lines_as("Assassin 'Khai'", args!["...", "......"])?;
                ctx.next()?;
                ctx.lines_as("Assassin 'Khai'", args!["...", "......", "........."])?;
                ctx.next()?;
                ctx.lines_as("Assassin 'Khai'", args!["...", "......", ".........", "............"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Assassin 'Khai'",
                    args!["...", "......", ".........", "............", "..............."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Assassin 'Khai'",
                    args![
                        "Nah.",
                        "I changed my mind!",
                        "I'm not gonna give you any hints after all. Hee hee hee~"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.mes("[Assassin 'Khai']")?;
                if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                    ctx.mes("Huh. You must have a lot of self confidence to be a Thief nowadays.")?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Assassin 'Khai'",
                        args!["Yeah yeah, I understand. Everyone messes up from time to time. Sorry for laughing at your mistakes."],
                    )?;
                } else {
                    ctx.mes("Hmm. I like your attitude. You should keep your pride as a Thief. Sorry for laughing at your mistakes. I think you'll do better next time.")?;
                }
                ctx.next()?;
                ctx.lines_as(
                    "Assassin 'Khai'",
                    args!["I'm not allowed to give you hints, I can tell you more about being an Assassin..."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Assassin 'Khai'",
                    args!["Above all else, we value our dignity. We're Assassins, after all and people will need us."],
                )?;
                ctx.next()?;
                ctx.lines_as("Assassin 'Khai'", args!["If people are close to you in some way, they might not understand what I'm saying. We're born to be loners due to our nature."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Assassin 'Khai'",
                    args![
                        "Imagine if a lover or a friend saw the blood on your hands. There's a chance that they might not stay with you."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Assassin 'Khai'", args!["Sometimes it gets lonely but it's not that bad. At least I can do what I want to do, you know, and do things my way."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Assassin 'Khai'",
                    args![
                        "Well, that's all I can tell you for now. Does being an Assassin",
                        "seem depressing to you?"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            3 => {
                ctx.lines_as("Assassin 'Khai'", args!["...Hm."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Assassin 'Khai'",
                    args![
                        "Right, that's the spirit. Don't ever let anyone else look down",
                        "on you. We're Assassins..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Assassin 'Khai'", args!["I apologize for laughing at you earlier. I want you to remember to keep that sense of pride and dignity as an Assassin."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Assassin 'Khai'",
                    args![
                        "Along with keeping your pride,",
                        "I want that you respect the blood that may stain your Katar or Dagger."
                    ],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(ctx, &[Val::from("...Got you.:...I'm confused.")])?) == 1 {
                    ctx.lines_as(
                        "Assassin 'Khai'",
                        args!["Yeah, I can trust you now. Let me give you some important tips."],
                    )?;
                    ctx.next()?;
                    let subject2 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                    if subject2 == 1 {
                        ctx.lines_as("Assassin 'Khai'", args!["First of all, Grimtooth is ...A skill specifically for the Katar. Therefore, it doesn't require any skills related to Dagger weapons."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Assassin 'Khai'",
                            args!["Double attack ...Haven't you tried it? It allows you to attack an enemy twice at a time."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Assassin 'Khai'",
                            args!["Red Blood is an elemental stone, Blue Gemstone doesn't have to do the Assassin job at all!"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Assassin 'Khai'", args!["Have you ever seen Mages hunt Elder willow using the Cold Bolt skill? Water overpowers Fire. Water puts Fire under control, and Wind puts water under control."])?;
                        ctx.next()?;
                        ctx.lines_as("Assassin 'Khai'", args!["As long as you stick close to the shadows, by walls and things like that, Cloaking will hide you from sight perfectly! Unless some bastard uses a certain detecting skill, you know."])?;
                        ctx.next()?;
                    } else if subject2 == 2 {
                        ctx.lines_as(
                            "Assassin 'Khai'",
                            args!["'Sharpened Legbone of Ghoul' possesses the Undead property."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Assassin 'Khai'", args!["What kind of weapon have you used so far? Damascus? Gladius? Stiletto? Or Main Gauche? What is that you're carrying now?"])?;
                        ctx.next()?;
                        ctx.lines_as("Assassin 'Khai'", args!["It's possible to get a slotted Katar from Desert Wolf. Well, just keep that in mind. You will need this information someday."])?;
                        ctx.next()?;
                        ctx.lines_as("Assassin 'Khai'", args!["You can gain a slotted Jur from a buddy living in a dark and damp place under the ground. Well, I have no idea why that dude has that weapon... Maybe he needs it to dig a hole?"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Assassin 'Khai'",
                            args!["And...", "I've always wanted a frog as a pet. But it's impossible!"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Assassin 'Khai'", args!["As far as I know, a Goblin carrying a hammer possesses the Earth property. Keep in mind that Fire overcomes the Earth property."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Assassin 'Khai'",
                            args!["You know elemental weapons? The names of Blacksmiths are engraved on them usually..."],
                        )?;
                        ctx.next()?;
                    } else if subject2 == 3 {
                        ctx.lines_as("Assassin 'Khai'", args!["Sell an Elder Willow Card to a Mage as soon as you can. They are mad about the card for some reason. Doesn't it increase the INT of a character? Hmmm..."])?;
                        ctx.next()?;
                        ctx.lines_as("Assassin 'Khai'", args!["For us, Dodge and Attack is more important than defense. Don't ever think about wearing a helm. It's heavy, uncomfortable and will even block your sight."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Assassin 'Khai'",
                            args!["'Increase Dodge' allows you to have +3% flee rate per skill lvl."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Assassin 'Khai'",
                            args!["As I have told you repeatedly: Katar class weapons (Jamadhar/Jur/Katar etc) are two-handed!"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Assassin 'Khai'", args!["City of desert... I miss my hometown, Morocc. I haven't been there for a long time. I feel like I became a Thief a few days ago. Time flies so fast..."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Assassin 'Khai'",
                            args!["Heh. I remember my Thief quest. I was so damn nervous when I broke into the farm to get Mushrooms..."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Assassin 'Khai'",
                            args!["Insects detect hiding/cloaking skills. Their feelers never fail to find targets."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Assassin 'Khai'",
                            args!["I've heard that the Baphomet Jr. Card adds +3 points to Agility and +1 point on Critical Attack..."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Assassin 'Khai'", args!["Yeah, we Assassins specialize in training Agility. We can gain a bonus of 10 Agility points even before mastering job level. The problem is it won't go up anymore after that, you know."])?;
                        ctx.next()?;
                    }
                    ctx.lines_as("Assassin 'Khai'", args!["^666666*Phew*^000000 That's all I can tell you, though that's a lot of hints. I don't doubt that I told you almost everything."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Assassin 'Khai'",
                        args!["Well then, go ask to take the test again with 'The Anonymous.'"],
                    )?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("in_moc_16"), Val::from(19), Val::from(144)])?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Assassin 'Khai'",
                    args![
                        "^666666*Sigh...*^000000",
                        "How can you not understand the concept of dignity? You just showed some to me just now!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Assassin 'Khai'",
                    args!["Oh, I get it. It wasn't pride you were showing, you were just being a jerk!"],
                )?;
                ctx.next()?;
                ctx.lines_as("Assassin 'Khai'", args!["Grrrrr...", "WARP PORTAL!"])?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("c_tower4"), Val::from(64), Val::from(76)])?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else {
        ctx.lines_as(
            "Assassin 'Khai'",
            args![
                "Oh, you must be an Assassin trainee. You are here to become",
                "an Assassin, aren't you?"
            ],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Yes, I am. :...No, I'm not.")])?) == 1 {
            ctx.lines_as(
                "Assassin 'Khai'",
                args!["Okay, good. Let's fill out the application form. Please sign your name and include your job level."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Assassin 'Khai'",
                args![
                    "Let's see.",
                    "Your name is",
                    ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("...")),
                    ((Val::from("Job level ") + ctx.var("JobLevel").get()?) + Val::from("..."))
                ],
            )?;
            ctx.next()?;
            if ctx.var("JobLevel").get()?.number()? > 48 {
                ctx.lines_as(
                    "Assassin 'Khai'",
                    args![
                        ((Val::from("Wait, Job level ") + ctx.var("JobLevel").get()?)
                            + Val::from("?! I can see you've been training pretty hard! My bosses will like this~"))
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Assassin 'Khai'", args!["Did you finish the form? Alright, go ahead and give it to me. Give me a second and I'll transport you to the Test Hall."])?;
                ctx.next()?;
                ctx.lines_as("Assassin 'Khai'", args!["Alright then,", "best of luck to you!"])?;
                ctx.close_window()?;
                if ctx.var("assin_q3").get()?.number()? < 3 {
                    ctx.var("assin_q3").set(Val::from(1))?;
                }
                ctx.var("assin_q").set(Val::from(1))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(8001), Val::from(8002)])?;
                ctx.call(Function::Warp, vec![Val::from("in_moc_16"), Val::from(19), Val::from(144)])?;
                return Err(Stop::End);
            } else if ctx.var("JobLevel").get()?.number()? < 49 {
                ctx.lines_as(
                    "Assassin 'Khai'",
                    args!["Well, you passed", "the requirements.", "Not bad at all."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Assassin 'Khai'",
                    args![
                        "Go ahead and give",
                        "me the form when you're",
                        "done filling it out.",
                        "Alright, thanks."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Assassin 'Khai'",
                    args!["I'll transport you", "to the Test Hall.", "Best of luck~"],
                )?;
                ctx.close_window()?;
                if ctx.var("assin_q3").get()?.number()? < 3 {
                    ctx.var("assin_q3").set(Val::from(2))?;
                }
                ctx.var("assin_q").set(Val::from(1))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(8001), Val::from(8002)])?;
                ctx.call(Function::Warp, vec![Val::from("in_moc_16"), Val::from(19), Val::from(144)])?;
                return Err(Stop::End);
            } else {
                ctx.lines_as("Assassin 'Khai'", args!["Who the", "hell are you?", "...Guards!"])?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("moc_fild16"), Val::from(206), Val::from(229)])?;
                return Err(Stop::End);
            }
        } else {
            ctx.lines_as(
                "Assassin 'Khai'",
                args![
                    "Huh...?",
                    "What, are you trying to trick me or something? Don't you wanna be an Assassin?"
                ],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("No.:Yes, I want to be an Assassin.")])?) == 1 {
                ctx.lines_as("Assassin 'Khai'", args!["Eh, get outta here.", "Stop wastin' my time..."])?;
                ctx.close_window()?;
                ctx.var("assin_q").set(Val::from(0))?;
                ctx.var("assin_q2").set(Val::from(0))?;
                ctx.call(Function::EraseQuest, vec![Val::from(8001)])?;
                ctx.call(Function::Warp, vec![Val::from("moc_fild16"), Val::from(206), Val::from(229)])?;
                return Err(Stop::End);
            }
            ctx.lines_as("Assassin 'Khai'", args!["...", "What the hell?", "Okay, then."])?;
            ctx.next()?;
            ctx.lines_as(
                "Assassin 'Khai'",
                args!["Fill out the application form with your name and job level."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Assassin 'Khai'",
                args![
                    ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("?")),
                    "That's your name?",
                    "It sounds funny.",
                    ((Val::from("Let's see... Job Level ") + ctx.var("JobLevel").get()?) + Val::from("..."))
                ],
            )?;
            ctx.next()?;
            if ctx.var("JobLevel").get()?.number()? > 48 {
                ctx.lines_as(
                    "Assassin 'Khai'",
                    args![
                        ((Val::from("Ho? Job Level ") + ctx.var("JobLevel").get()?)
                            + Val::from("?! You must have been training really hard. The bosses will like that for sure..."))
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Assassin 'Khai'",
                    args!["Are you done filling out the form? Alright, give it to me so I can send you to the Test Hall. Good luck~"],
                )?;
                ctx.next()?;
                if ctx.var("assin_q3").get()?.number()? < 3 {
                    ctx.var("assin_q3").set(Val::from(1))?;
                }
                ctx.var("assin_q").set(Val::from(1))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(8001), Val::from(8002)])?;
                ctx.call(Function::Warp, vec![Val::from("in_moc_16"), Val::from(19), Val::from(144)])?;
                return Err(Stop::End);
            } else if ctx.var("JobLevel").get()?.number()? < 49 {
                ctx.lines_as(
                    "Assassin 'Khai'",
                    args!["Not bad. You fulfilled our requirements. Not bad at all. Now are you done filling out the form?"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Assassin 'Khai'",
                    args![
                        "Then give me the form so that I can send you to the Test Hall, alright?",
                        "Good luck..."
                    ],
                )?;
                ctx.next()?;
                if ctx.var("assin_q3").get()?.number()? < 3 {
                    ctx.var("assin_q3").set(Val::from(2))?;
                }
                ctx.var("assin_q").set(Val::from(1))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(8001), Val::from(8002)])?;
                ctx.call(Function::Warp, vec![Val::from("in_moc_16"), Val::from(19), Val::from(144)])?;
                return Err(Stop::End);
            } else {
                ctx.lines_as("Assassin 'Khai'", args!["How the hell did", "you get in here?", "Get out!"])?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("moc_fild16"), Val::from(206), Val::from(229)])?;
                return Err(Stop::End);
            }
        }
    }
    Ok(Val::from(0))
}

pub fn guildsman_asn2_ontouch(ctx: &Ctx) -> Script {
    guildsman_asn2_ontouch_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum NamelessOneStep {
    Start,
    OnTouch,
}

fn nameless_one_run(ctx: &Ctx, mut step: NamelessOneStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_assassin_t = Val::from(0);
    'machine: loop {
        match step {
            NamelessOneStep::Start => {
                step = NamelessOneStep::OnTouch;
                continue 'machine;
            }
            NamelessOneStep::OnTouch => {
                if ctx.var("assin_q2").get()?.number()? < 5 {
                    if ctx.var("assin_q2").get()?.number()? < 3 {
                        ctx.lines_as(
                            "The Anonymous One",
                            args!["Welcome, guest.", "Mwahaha, it's useless", "to try to find or see me..."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "The Anonymous One",
                            args![
                                "I am perfectly hidden!",
                                "To become undetectable can only be done by the greatest Assassins!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "The Anonymous One",
                            args!["Aren't you scared that you can't see me? I could kill you at any time and it would be so easy..."],
                        )?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(
                            ctx,
                            &[Val::from("I think I crapped my pants!:You're all talk. I challenge you!")],
                        )?) == 1
                        {
                            ctx.lines_as("The Anonymous One", args!["Now I see that", "you're nothing", "but a wimp."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "The Anonymous One",
                                args!["Bwahahahahahah!", "Stop cowering in fear!", "It's making me laugh!"],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as("The Anonymous One", args!["So...", "You wish for", "a challenge?", "From me?!"])?;
                        ctx.next()?;
                        ctx.lines_as("The Anonymous One", args!["A river of blood follows my every footstep. I am nameless, for the sting of my blades is all anyone needs to know."])?;
                        ctx.next()?;
                        ctx.lines_as("The Anonymous One", args!["I am here to test your knowledge, as well as your capacity for heartlessness. Those are both necessary to become an Assassin."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "The Anonymous One",
                            args![
                                "For your challenge, you must",
                                "answer my questions correctly. Very difficult questions that only an Assassin can answer."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "The Anonymous One",
                            args![
                                "Although I am heartless,",
                                "I am not necessarily cruel. Before we proceed, is there anything you wish to know?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.var("assin_q2").set(Val::from(0))?;
                        'l1: loop {
                            if !(ctx.var("assin_q2").get()?.number()? < 3) {
                                break 'l1;
                            }
                            'b1: {
                                match runtime::select_values(ctx, &[Val::from("...Skills?:...Stats?:Hmpf, I know it all.")])? {
                                    1 => {
                                        ctx.lines_as("The Anonymous One", args!["Skills...?", "Although skills can have circumstantial applications, I will tell you about the basic concepts."])?;
                                        ctx.next()?;
                                        ctx.lines_as("The Anonymous One", args!["First, ^3355FFKatar Mastery^000000. This skill increases the damage of Katar class weapons. The higher the skill level, the more damage is increased."])?;
                                        ctx.next()?;
                                        ctx.lines_as("The Anonymous One", args!["^3355FFLeft Hand Mastery^000000 and ^3355FFRight Hand Mastery^000000. Assassins can equip different weapons in each hand when using Dagger class weapons."])?;
                                        ctx.next()?;
                                        ctx.lines_as("The Anonymous One", args!["But it is obviously more difficult to handle 2 weapons at a time than using just one. The Left and Right Hand Mastery skills increase the damage when using two Daggers."])?;
                                        ctx.next()?;
                                        ctx.lines_as("The Anonymous One", args!["However, if you don't want to use two Daggers, you won't need this skill. You will see how 'Left Hand Mastery' works as soon as you reach 'Right Hand Mastery' Level 2."])?;
                                        ctx.next()?;
                                        ctx.lines_as("The Anonymous One", args!["^3355FFSonic Blow^000000 allows you to strike an enemy 8 times at once. This skill only works with Katar weapons because of the speed it requires."])?;
                                        ctx.next()?;
                                        ctx.lines_as("The Anonymous One", args!["Of course, the damage is affected by STR and weapon damage. You'll understand how this skill works when you reach Level 4 Katar Mastery."])?;
                                        ctx.next()?;
                                        ctx.lines_as("The Anonymous One", args!["^3355FFGrimtooth^000000 allows you to attack enemies while hiding under the ground. As you master it, you'll be able to attack foes from a distance."])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "The Anonymous One",
                                            args!["Since it's a ranged attack, it can be very useful when you're surrounded by enemies."],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("The Anonymous One", args!["Because you're required to perfectly hide yourself to use this skill, you must first learn Level 2 Cloaking before you can learn Grimtooth."])?;
                                        ctx.next()?;
                                        ctx.lines_as("The Anonymous One", args!["To learn ^3355FFCloaking^000000, you must learn Level 2 Hiding. Then you will be able to move while hiding if you are close to a wall."])?;
                                        ctx.next()?;
                                        ctx.lines_as("The Anonymous One", args!["The ^3355FFEnchant Poison^000000 skill allows you to enchant poison on the weapon you're using. This will temporarily give the weapon the Poison property."])?;
                                        ctx.next()?;
                                        ctx.lines_as("The Anonymous One", args!["This will also make your attacks poison the enemy by chance. You can also use this skill to enchant the weapons of your party members..."])?;
                                        ctx.next()?;
                                        ctx.lines_as("The Anonymous One", args!["^3355FFPoison React^000000 shields the user from attacks with the Poison property, and can be used on other people as well. However, you must learn Level 3 Enchant Poison first."])?;
                                        ctx.next()?;
                                        ctx.lines_as("The Anonymous One", args!["^3355FFVenom Dust^000000 consumes a Red Gemstone to contaminate an area with poison. The duration of contamination increases with the level of this skill."])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "The Anonymous One",
                                            args!["You can learn the Venom Dust skill after you learn Level 5 Enchant Poison."],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("The Anonymous One", args!["^3355FFVenom Splasher^000000 is a skill that, after it is used on a target, will cause it to explode when its HP is less than a certain amount after three seconds."])?;
                                        ctx.next()?;
                                        ctx.lines_as("The Anonymous One", args!["When the target explodes, the enemies in the vicinity are also damaged. This is an essential skill for Assassins. It requires Level 5 Poison React and Level 5 Venom Dust."])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "The Anonymous One",
                                            args!["Now...", "That's all I have to tell you", "about Assassin skills."],
                                        )?;
                                        ctx.var("assin_q2").set(Val::from(1))?;
                                        ctx.next()?;
                                    }
                                    2 => {
                                        ctx.lines_as(
                                            "The Anonymous One",
                                            args!["Hmm, Stats...", "For Assassins, Agility, or AGI, is the most important stat."],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("The Anonymous One", args!["For the sake of assassination, STR is probably the second most important stat. But that is only my recommendation."])?;
                                        ctx.next()?;
                                        ctx.lines_as("The Anonymous One", args!["I cannot give you better advice than that in regards to Stats. You should research and see which stats suit you, and decide what kind of Assassin you want to be."])?;
                                        ctx.var("assin_q2").set(Val::from(2))?;
                                        ctx.next()?;
                                    }
                                    3 => {
                                        if ctx.var("assin_q2").get()? == 0 {
                                            ctx.lines_as(
                                                "The Anonymous One",
                                                args!["Know everything do you?!", "I'll be the judge of that!"],
                                            )?;
                                            ctx.next()?;
                                        }
                                        ctx.var("assin_q2").set(Val::from(3))?;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        ctx.lines_as(
                            "The Anonymous One",
                            args!["Hmpf. It is now time to test your knowledge. You are not allowed to miss more than one question."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("The Anonymous One", args!["In other words, if you want to pass this test, you must give me 9 correct answers out of 10 questions. I won't let you know which answer you got wrong..."])?;
                        ctx.next()?;
                        ctx.lines_as("The Anonymous One", args!["Are you ready?", "Prepare yourself!"])?;
                    } else if ctx.var("assin_q2").get()?.number()? < 5 {
                        ctx.lines_as(
                            "The Anonymous One",
                            args!["Having problems", "passing a simple test?", "You should have", "known better."],
                        )?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(
                            ctx,
                            &[Val::from("Help me, how do I pass?:I challenge you again!")],
                        )?) == 1
                        {
                            ctx.lines_as("The Anonymous One", args!["Well, that's a damn good question. But you're banished from the Assassin Guild, so it's no concern of mine..."])?;
                            ctx.close_window()?;
                            ctx.call(Function::Warp, vec![Val::from("moc_fild16"), Val::from(206), Val::from(151)])?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as(
                            "The Anonymous One",
                            args![
                                "So I see...",
                                "Now go, but do not fear. I will be by your side as you learn the outcome of your choice..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("The Anonymous One", args!["Now, we shall test you once more! Keep in mind, you must answer 9 questions out of 10 correctly. Remember I am doing you a favor..."])?;
                        ctx.next()?;
                        ctx.lines_as("The Anonymous One", args!["You must answer 9 questions out of 10 correctly. If you miss more than one question, you can never become an Assassin."])?;
                        ctx.next()?;
                        ctx.lines_as("The Anonymous One", args!["Okay,", "are you ready?", "Good luck."])?;
                    }
                    ctx.next()?;
                    let subject3 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                    if subject3 == 1 {
                        ctx.lines_as(
                            "The Anonymous One",
                            args!["1. Choose skill that is not required to learn Grimtooth."],
                        )?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(
                            ctx,
                            &[Val::from(
                                "Cloaking level 2:Sonic Blow level 5:Katar Mastery level 4:Right hand Mastery level 2",
                            )],
                        )?) == 4
                        {
                            l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                        }
                        ctx.lines_as("The Anonymous One", args!["2. What property does Enchant Poison possess?"])?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(ctx, &[Val::from("Poison:Earth:Fire:Wind")])?) == 1 {
                            l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                        }
                        ctx.lines_as("The Anonymous One", args!["3. How does Level 4 Right Hand Mastery work?"])?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(
                            ctx,
                            &[Val::from(
                                "Recover 80% of damage decrease:Recover 90% of damage decrease:Increase 90% of damage:Increase 108% of damage",
                            )],
                        )?) == 2
                        {
                            l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                        }
                        ctx.lines_as("The Anonymous One", args!["4. What is the item required for using Venom Dust?"])?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(
                            ctx,
                            &[Val::from("Red Blood:Blue Gemstone:Yellow Gemstone:Red Gemstone")],
                        )?) == 4
                        {
                            l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                        }
                        ctx.lines_as(
                            "The Anonymous One",
                            args!["5. Which skill can you learn when you reach Level 5 Enchant Poison?"],
                        )?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(
                            ctx,
                            &[Val::from("Envenom:Sonic Blow:Venom Splasher:Venom Dust")],
                        )?) == 4
                        {
                            l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                        }
                        ctx.lines_as(
                            "The Anonymous One",
                            args!["6. Among the following skills, which allows you to walk while invisible?"],
                        )?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(
                            ctx,
                            &[Val::from("Hiding:Back Slide:Cloaking:Sand Attack")],
                        )?) == 3
                        {
                            l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                        }
                        ctx.lines_as(
                            "The Anonymous One",
                            args!["7. Choose the condition that is unrelated to Venom Splasher."],
                        )?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(
                            ctx,
                            &[Val::from("Poisoned target.:Red Gemstone.:Remaing HP of Target.")],
                        )?) == 2
                        {
                            l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                        }
                        ctx.lines_as(
                            "The Anonymous One",
                            args!["8. Which monster is weak to a weapon with Vadon card (adds 20% damage on Fire property monster)?"],
                        )?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(
                            ctx,
                            &[Val::from("Steel Chonchon:Deviruchi:Elder Willow:Baphomet")],
                        )?) == 3
                        {
                            l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                        }
                        ctx.lines_as("The Anonymous One", args!["9. How much SP does", "Double Attack need?"])?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(
                            ctx,
                            &[Val::from(
                                "15:It's a passive skill, so SP use is 0.:It's passive skill, so SP use is 10.:54",
                            )],
                        )?) == 2
                        {
                            l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                        }
                        ctx.lines_as(
                            "The Anonymous One",
                            args!["10. What is the best elemental Main Gauche weapon for hunting in Izlude dungeon?"],
                        )?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(
                            ctx,
                            &[Val::from("Wind Main Gauche:Ice Main Gauche:Earth Main Gauche:Fire Main Gauche")],
                        )?) == 1
                        {
                            l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                        }
                    } else if subject3 == 2 {
                        ctx.lines_as("The Anonymous One", args!["1. Which monster", "drops a slotted Katar?"])?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(
                            ctx,
                            &[Val::from("Thief Bug:Peco Peco:Desert Wolf:Hammer Cobolt")],
                        )?) == 3
                        {
                            l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                        }
                        ctx.lines_as("The Anonymous One", args!["2. Which monster", "drops a slotted Jur?"])?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(ctx, &[Val::from("Martin:Desert Wolf:Marionette:Myst")])?) == 1 {
                            l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                        }
                        ctx.lines_as(
                            "The Anonymous One",
                            args!["3. Which class is allowed to craft elemental weapons?"],
                        )?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(ctx, &[Val::from("Merchant:Blacksmith:Thief:Priest")])?) == 2 {
                            l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                        }
                        ctx.lines_as(
                            "The Anonymous One",
                            args!["4. Choose the weapon which is not in the Katar class."],
                        )?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(ctx, &[Val::from("Jamadhar:Jur:Katar:Gladius")])?) == 4 {
                            l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                        }
                        ctx.lines_as(
                            "The Anonymous One",
                            args!["5. What property do Izlude dungeon monsters posses?"],
                        )?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(ctx, &[Val::from("Water:Fire:Wind:Earth")])?) == 1 {
                            l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                        }
                        ctx.lines_as("The Anonymous One", args!["6. Which monster", "cannot be a Cute Pet?"])?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(
                            ctx,
                            &[Val::from("Poporing:Roda Frog:Smokie:Poison Spore")],
                        )?) == 2
                        {
                            l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                        }
                        ctx.lines_as(
                            "The Anonymous One",
                            args!["7. Choose a monster that Fire property Daggers work the best on."],
                        )?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(
                            ctx,
                            &[Val::from("Dagger Goblin:Mace Goblin:Morning Star Goblin:Hammer Goblin")],
                        )?) == 4
                        {
                            l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                        }
                        ctx.lines_as(
                            "The Anonymous One",
                            args!["8. Choose the non-elemental Katar from the following:"],
                        )?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(
                            ctx,
                            &[Val::from(
                                "Katar of Raging Blaze:Katar of Dusty Thornbush:Sharpened Legbone of Ghoul:Infiltrator",
                            )],
                        )?) == 4
                        {
                            l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                        }
                        ctx.lines_as("The Anonymous One", args!["9. Which is the uncommon monster?"])?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(ctx, &[Val::from("Poring:Mastering:Ghostring:Spore")])?) == 3 {
                            l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                        }
                        ctx.lines_as("The Anonymous One", args!["10. Choose the monster", "that is not Undead."])?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(ctx, &[Val::from("Drake:Megalodon:Spore:Khalitzburg")])?) == 3 {
                            l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                        }
                    } else if subject3 == 3 {
                        ctx.lines_as("The Anonymous One", args!["1. Choose the correct amount of the maximum dodge rate increase from the 'Increase Dodge' skill when at level 10."])?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(ctx, &[Val::from("30:40:160:20")])?) == 1 {
                            l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                        }
                        ctx.lines_as(
                            "The Anonymous One",
                            args!["2. Choose a monster which detects hiding/cloaking Thieves and Assassins."],
                        )?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(
                            ctx,
                            &[Val::from("Worm Tail:Andre:Mummy:Soldier Skeleton")],
                        )?) == 2
                        {
                            l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                        }
                        ctx.lines_as(
                            "The Anonymous One",
                            args!["3. Choose a group of weapons that cannot be used by an Assassin at once."],
                        )?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(
                            ctx,
                            &[Val::from(
                                "Main Gaughe + Gladius:Stiletto + Main Gauche:Katar + Maingauche:Hammer + Stiletto",
                            )],
                        )?) == 3
                        {
                            l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                        }
                        ctx.lines_as(
                            "The Anonymous One",
                            args!["4. Choose the town where Thieves can change their jobs."],
                        )?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(ctx, &[Val::from("Prontera:Lutie:Alberta:Morocc")])?) == 4 {
                            l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                        }
                        ctx.lines_as(
                            "The Anonymous One",
                            args!["5. Choose a card that does not affect the AGI stat."],
                        )?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(
                            ctx,
                            &[Val::from("Baphomet Jr. card:Whisper Card:Female Thiefbug card:Male Thiefbug card")],
                        )?) == 2
                        {
                            l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                        }
                        ctx.lines_as(
                            "The Anonymous One",
                            args!["6. Choose the correct specialty of the Assassin class."],
                        )?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(
                            ctx,
                            &[Val::from(
                                "Excellent singing talent:Excellent reading talent:Excellent dancing talent:Excellent dodge ability",
                            )],
                        )?) == 4
                        {
                            l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                        }
                        ctx.lines_as(
                            "The Anonymous One",
                            args!["7. Choose the maximum AGI bonus an Assassin can get at job level 50."],
                        )?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(ctx, &[Val::from("7:8:9:10")])?) == 4 {
                            l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                        }
                        ctx.lines_as("The Anonymous One", args!["8. Choose the item that an Assassin cannot equip."])?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(ctx, &[Val::from("Dagger:Helm:Boots:Brooch")])?) == 2 {
                            l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                        }
                        ctx.lines_as("The Anonymous One", args!["9. Choose the job change item for Thief."])?;
                        ctx.next()?;
                        match runtime::select_values(
                            ctx,
                            &[Val::from(
                                "Orange Gooey Mushroom:Red Gooey Mushroom:Orange Net Mushroom:Orange Hair Mushroom",
                            )],
                        )? {
                            1 | 3 => {
                                l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                            }
                            _ => {}
                        }
                        ctx.lines_as(
                            "The Anonymous One",
                            args!["10. Choose a card that would typically benefit an Assassin the least."],
                        )?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(
                            ctx,
                            &[Val::from("Whisper card:Elder Willow card:Soldier Skeleton card:Cobold card")],
                        )?) == 2
                        {
                            l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                        }
                    }
                    if ctx.var("assin_q2").get()? == 3 {
                        ctx.next()?;
                        ctx.lines_as(
                            "The Anonymous One",
                            args!["Hmpf.", "Somehow, you", "have shown me", "great effort."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "The Anonymous One",
                            args![
                                "Let's see...",
                                "You scored",
                                ((Val::from("") + l_assassin_t.clone()) + Val::from(" percent..."))
                            ],
                        )?;
                        if l_assassin_t.clone().number()? > 80 {
                            ctx.var("assin_q2").set(Val::from(5))?;
                            ctx.call(Function::ChangeQuest, vec![Val::from(8002), Val::from(8003)])?;
                            ctx.lines(args!["Well done.", "You pass."])?;
                            ctx.next()?;
                            ctx.lines_as("The Anonymous One", args!["However, another test awaits you. When you go inside the next area, you will receive your instructions..."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            ctx.var("assin_q2").set(Val::from(4))?;
                            ctx.mes("That means you fail!")?;
                            ctx.next()?;
                            ctx.lines_as(
                                "The Anonymous One",
                                args![
                                    "How could you expect to be an Assassin with this score? Keep training and come back when you're ready."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "The Anonymous One",
                                args!["I would ask 'Khai,' the one who processed your application, for advice."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("The Anonymous One", args!["You may also use this code: ^880000iro.ragnarokonline.com^000000. Somehow, those words are linked to a vast body of otherworldly knowledge..."])?;
                            ctx.close_window()?;
                            ctx.call(Function::Warp, vec![Val::from("in_moc_16"), Val::from(19), Val::from(76)])?;
                            return Err(Stop::End);
                        }
                    } else if ctx.var("assin_q2").get()? == 4 {
                        ctx.next()?;
                        ctx.lines_as("The Anonymous One", args!["You showed", "great effort..."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "The Anonymous One",
                            args![
                                "Let's see...",
                                "You scored",
                                ((Val::from("") + l_assassin_t.clone()) + Val::from(" points..."))
                            ],
                        )?;
                        if l_assassin_t.clone().number()? > 80 {
                            ctx.var("assin_q2").set(Val::from(5))?;
                            ctx.call(Function::ChangeQuest, vec![Val::from(8002), Val::from(8003)])?;
                            ctx.next()?;
                            ctx.lines_as("The Anonymous One", args!["You didn't fail this time! But you're not done just yet. You have another test ahead of you. Once you proceed, you will be informed about your next trial."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            ctx.var("assin_q2").set(Val::from(4))?;
                            ctx.mes("You failed!")?;
                            ctx.next()?;
                            ctx.lines_as(
                                "The Anonymous One",
                                args!["You're too underqualified. How can you even think about becoming an Assassin?!"],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("The Anonymous One", args!["I'm surprised that you were even able to become a Thief. Go away, and come back only when you know what the hell you're doing."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "The Anonymous One",
                                args!["Hmpf, if you really don't have a clue, I will give you a little advice."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "The Anonymous One",
                                args!["Go ask 'Khai,' the guy who takes care of your test application, maybe he will help you."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("The Anonymous One", args!["You may also wish to take advantage of the ancient code, ^3355FFiro.ragnarokonline.com^000000. Supposedly, those words are linked to a vast body of otherworldly knowledge..."])?;
                            ctx.close_window()?;
                            ctx.call(Function::Warp, vec![Val::from("in_moc_16"), Val::from(19), Val::from(76)])?;
                            return Err(Stop::End);
                        }
                    }
                } else {
                    ctx.lines_as("The Anonymous One", args!["...I will keep watching you."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn nameless_one(ctx: &Ctx) -> Script {
    nameless_one_run(ctx, NamelessOneStep::Start, Vec::new()).map(|_| ())
}

pub fn nameless_one_ontouch(ctx: &Ctx) -> Script {
    nameless_one_run(ctx, NamelessOneStep::OnTouch, Vec::new()).map(|_| ())
}

fn standby_room_asntest_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn standby_room_asntest(ctx: &Ctx) -> Script {
    standby_room_asntest_body(ctx, Vec::new()).map(|_| ())
}

fn standby_room_asntest_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Standby Room#ASNTEST")])?;
    ctx.call(
        Function::WaitingRoom,
        vec![
            Val::from("Standby Room"),
            Val::from(10),
            Val::from("Standby Room#ASNTEST::OnStartArena"),
            Val::from(1),
        ],
    )?;
    ctx.call(Function::EnableWaitingRoomEvent, vec![])?;
    return Err(Stop::End);
}

pub fn standby_room_asntest_oninit(ctx: &Ctx) -> Script {
    standby_room_asntest_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn standby_room_asntest_onstartarena_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::WarpWaitingPc,
        vec![Val::from("in_moc_16"), Val::from(66), Val::from(151)],
    )?;
    ctx.call(
        Function::AttachRid,
        vec![ctx.var("$@warpwaitingpc").get_at(runtime::index(&Val::from(0))?)?],
    )?;
    if ctx.var("assin_q2").get()?.number()? < 5 {
        ctx.call(
            Function::Warp,
            vec![
                Val::from("in_moc_16"),
                Val::from(20),
                Val::from(145),
                ctx.call(Function::GetCharacterId, vec![Val::from(0)])?,
            ],
        )?;
        return Err(Stop::End);
    }
    ctx.call(Function::DoNpcEvent, vec![Val::from("Beholder#ASNTEST::OnEnable")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("Keeper of the Door#ASN::OnDisable")])?;
    ctx.call(
        Function::SetVariableOfNpc,
        vec![
            Val::from(".disabletraps"),
            Val::from("Beholder#ASNTEST"),
            Val::from(0),
            Val::from(0),
        ],
    )?;
    ctx.call(Function::DisableWaitingRoomEvent, vec![])?;
    return Err(Stop::End);
}

pub fn standby_room_asntest_onstartarena(ctx: &Ctx) -> Script {
    standby_room_asntest_onstartarena_body(ctx, Vec::new()).map(|_| ())
}

fn standby_room_asntest_onstart_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableWaitingRoomEvent, vec![])?;
    return Err(Stop::End);
}

pub fn standby_room_asntest_onstart(ctx: &Ctx) -> Script {
    standby_room_asntest_onstart_body(ctx, Vec::new()).map(|_| ())
}

fn test_guide_asn_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn test_guide_asn(ctx: &Ctx) -> Script {
    test_guide_asn_body(ctx, Vec::new()).map(|_| ())
}

fn test_guide_asn_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("assin_q2").get()?.number()? < 5 {
        ctx.lines_as(
            "Barcardi",
            args!["You can't take the next trial without passing the written test first. You better speak to the Anonymous One..."],
        )?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("in_moc_16"), Val::from(19), Val::from(76)])?;
        return Err(Stop::End);
    }
    if (ctx.var("assin_q").get()? == 1 && ctx.var("assin_q2").get()? == 5) {
        ctx.lines_as(
            "Barcardi",
            args![
                ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("...")),
                "You passed the test..?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Barcardi", args!["To be honest, I want to grant you the job change without any other condition. Too many pathetic people don't even have the basic knowledge to be Assassins..."])?;
        ctx.next()?;
        ctx.lines_as("Barcardi", args!["We must keep our dignity as Assassins and be truly great! Regrettably, there are too many idiots that don't have any pride."])?;
        ctx.next()?;
        ctx.lines_as("Barcardi", args!["All Assassins must respect the enemies they slay, the blood that they spill, and above all, maintain their sense of dignity!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Barcardi",
            args!["Alright. This next trial will test your ability to quickly find your target."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Barcardi",
            args![
                "If you're going to be an Assassin, we need to determine whether or not you can distinguish friend from foe in an instant."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Barcardi",
            args!["The main goal of this test is to find and kill as many monsters named ^008800Job change target^000000 as possible."],
        )?;
        ctx.next()?;
        ctx.lines_as("Barcardi", args!["You must kill at least", "6 ^008800Job change target^000000 monsters. They're intermingled among similar looking monsters, so you need to be careful..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Barcardi",
            args![
                "If you fail, you'll have to restart this test. Go to the room above",
                "me to be transported to the Test Hall."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Barcardi", args!["Only one person is allowed to take the test at a time, so if anyone is taking the test, you'll have to wait until that person finishes."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as("Barcardi", args!["Hey, don't be too hard", "on yourself. Cheer up!"])?;
        ctx.next()?;
        ctx.lines_as("Barcardi", args!["Hmm, if you're exhausted, I'm willing to bring you back. Of course, if you leave, you'll have to take the job test over again. So what do you want to do?"])?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from("Continue!:Quit the job change test for now.")],
        )?) == 1
        {
            ctx.lines_as(
                "Barcardi",
                args![
                    "Good choice!",
                    "Remember, you",
                    "must find and kill",
                    "6 ^008800Job change target^000000 monsters!",
                    "Good luck!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as("Barcardi", args!["Alright...", "I guess you", "could use a break..."])?;
        ctx.close_window()?;
        ctx.var("assin_q").set(Val::from(0))?;
        ctx.var("assin_q2").set(Val::from(0))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(8003), Val::from(8000)])?;
        ctx.call(Function::Warp, vec![Val::from("in_moc_16"), Val::from(19), Val::from(13)])?;
        return Err(Stop::End);
    }
}

pub fn test_guide_asn_ontouch(ctx: &Ctx) -> Script {
    test_guide_asn_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn beholder_asntest_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn beholder_asntest(ctx: &Ctx) -> Script {
    beholder_asntest_body(ctx, Vec::new()).map(|_| ())
}

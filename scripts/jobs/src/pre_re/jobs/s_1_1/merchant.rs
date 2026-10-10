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

#[derive(Clone, Copy, Debug)]
enum MerchantMerStep {
    Start,
    SGiveSerial,
}

fn merchant_mer_run(ctx: &Ctx, mut step: MerchantMerStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MerchantMerStep::Start => {
                if ctx.var("Upper").get()? == 1 {
                    if ctx.var("Class").get()? == constants::JOB_NOVICE_HIGH
                        && (ctx.var("advjob").get()? == constants::JOB_WHITESMITH || ctx.var("advjob").get()? == constants::JOB_CREATOR)
                    {
                        ctx.lines_as(
                            "Chief Mahnsoo",
                            args![
                                "Long time no see!",
                                "Hey, you didn't quit",
                                "your business, did you?",
                                "What happened?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Chief Mahnsoo",
                            args!["Whoa...", "You've actually been to Valhalla?! Wow, you've come a long way..."],
                        )?;
                        ctx.next()?;
                        if !shared::other_global_functions::f_canchangejob(ctx, vec![])?.is_true() {
                            ctx.lines_as("Chief Mahnsoo", args!["Hmmm...", "It seems that you're not ready to become a Merchant again. Go finish learning the Basic Novice Skills first."])?;
                            ctx.next()?;
                            ctx.lines_as("Chief Mahnsoo", args!["Don't worry, we'll always have a Merchant position open for you. Just come back when you're ready, okay?"])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as("Chief Mahnsoo", args!["I guess it's destiny that we meet like this once more. Alright. Once again, let me change you into a Merchant!"])?;
                        ctx.next()?;
                        ctx.call(Function::Skill, args!["NV_TRICKDEAD", 0, constants::SKILL_PERM])?;
                        ctx.call(Function::JobChange, args![constants::JOB_MERCHANT_HIGH])?;
                        ctx.call(Function::Skill, args!["MC_CARTREVOLUTION", 1, constants::SKILL_PERM])?;
                        ctx.call(Function::Skill, args!["MC_CHANGECART", 1, constants::SKILL_PERM])?;
                        ctx.call(Function::Skill, args!["MC_LOUD", 1, constants::SKILL_PERM])?;
                        ctx.lines_as(
                            "Chief Mahnsoo",
                            args!["Ah~ How nostalgic. Just like old times! Alright, do your best!"],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Chief Mahnsoo",
                            args![
                                "^333333*Sigh*^000000",
                                "I'm so bored...",
                                "When will I hear from my lovely Blossom?"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                if ctx.var("BaseJob").get()? == constants::JOB_MERCHANT {
                    ctx.lines_as("Chief Mahnsoo", args!["Hello there!", "How do you like", "being a Merchant?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Chief Mahnsoo",
                        args!["Having a way with", "money certainly", "has its perks,", "does it not?"],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("BaseJob").get()? != constants::JOB_MERCHANT && ctx.var("BaseJob").get()? != constants::JOB_NOVICE {
                    ctx.lines_as(
                        "Chief Mahnsoo",
                        args!["We Merchants hate people who are two faced. It's bad for business."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Chief Mahnsoo", args!["People who always try to take advantage of other people by selling things at a ridiculous price just so they can make money that they'll waste are the worst."])?;
                    ctx.next()?;
                    ctx.lines_as("Chief Mahnsoo", args!["Well, in any case, we only accept Novices for job changes to the Merchant class. But I appreciate your interest in what we do."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("job_merchant_q").get()? == 9 {
                    ctx.lines_as("Chief Mahnsoo", args!["Hello there,", ctx.player().name()? + "."])?;
                    ctx.var("job_merchant_q").set(Val::from(0))?;
                    ctx.var("job_merchant_q2").set(Val::from(0))?;
                    ctx.var("quest_alb_01").set(Val::from(0))?;
                    ctx.mes("Unfortunately, you failed to earn your Merchant License this time.")?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Chief Mahnsoo",
                        args!["I'll erase your records, so come back anytime when you want to reapply."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("job_merchant_q").get()? == 8 || ctx.var("job_merchant_q").get()? == 7 {
                    ctx.lines_as(
                        "Chief Mahnsoo",
                        args![
                            "Hello there,",
                            ctx.player().name()? + ".",
                            "I'm pleased to tell you",
                            "that I have good news!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Chief Mahnsoo",
                        args![
                            "The Merchant Guild accepted your application. You've proven that you are fully qualified to become a Merchant."
                        ],
                    )?;
                    if ctx.var("job_merchant_q").get()? == 7 {
                        ctx.next()?;
                        ctx.lines_as(
                            "Chief Mahnsoo",
                            args!["The only thing to take care of is your Membership Fee.", "Are you ready?"],
                        )?;
                        ctx.next()?;
                        match ctx.menu(&["Pay the rest of the 500 Zeny", "Quit"])? {
                            0 => {
                                ctx.mes("[Chief Mahnsoo]")?;
                                if ctx.player().zeny()? < 500 {
                                    ctx.lines(args![
                                        "Hmmm...",
                                        "I suppose you currently don't have enough zeny to pay the rest of your Membership fee right now."
                                    ])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Chief Mahnsoo",
                                        args!["Please return when you have earned the 500 zeny that you need to become a Merchant."],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                ctx.player().set_zeny(ctx.player().zeny()? - 500)?;
                                ctx.lines(args!["Ah yes...!", "Now your", "membership", "is paid in full."])?;
                            }
                            _ => {
                                ctx.lines_as(
                                    "Chief Mahnsoo",
                                    args![
                                        "I suppose you need some time to gather some zeny to pay your membership fee. Please come",
                                        "back as soon as you're ready."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                    }
                    ctx.next()?;
                    ctx.lines_as("Chief Mahnsoo", args!["Congratulations!"])?;
                    shared::other_global_functions::job_change(ctx, args![constants::JOB_MERCHANT])?;
                    shared::other_global_functions::f_clearjobvar(ctx, vec![])?;
                    for quest_id in 1009_i32..=1012 {
                        if ctx.call(Function::IsBeginQuest, args![quest_id])? == 1 {
                            ctx.call(Function::CompleteQuest, args![quest_id])?;
                        }
                    }
                    ctx.mes("I'm very pleased that you are joining the Merchant Guild and hope that you will play an active part in Rune-Midgarts' economy.")?;
                    ctx.next()?;
                    if ctx.var("quest_alb_01").get()? == 1 {
                        ctx.lines_as(
                            "Chief Mahnsoo",
                            args![
                                "*Ahem* Aaaaand let me give you a little bit of money for delivering that message to Blossom for me.",
                                "I hope you'll help me again next time~"
                            ],
                        )?;
                        ctx.player().set_zeny(ctx.player().zeny()? + 200)?;
                        ctx.var("quest_alb_01").set(Val::from(2))?;
                    } else {
                        ctx.lines_as("Chief Mahnsoo", args!["The message you were supposed to deliver as per my request? You've forgotten about that? Oh well. Good work!"])?;
                    }
                    ctx.next()?;
                    ctx.lines_as(
                        "Chief Mahnsoo",
                        args!["Our goal is to control 20 % of the world's income! We're going to need young, eager people like you!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Chief Mahnsoo",
                        args![
                            "But overall, we'll also be happy just to make loads of money.",
                            "But we all know that~"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("job_merchant_q2").get()?.is_true()
                    && ctx.var("job_merchant_q").get()?.number()? > 0
                    && ctx.var("job_merchant_q").get()?.number()? < 7
                {
                    ctx.mes("[Chief Mahnsoo]")?;
                    if ctx.var("job_merchant_q2").get()? == 1 || ctx.var("job_merchant_q2").get()? == 2 {
                        ctx.mes("First, get the delivery package from the storehouse, and then take it to the former Swordman's Association in Prontera.")?;
                        ctx.next()?;
                        ctx.lines_as("Chief Mahnsoo", args!["When you get there, give the package to the Kafra Employee stationed near there. Her name is Blossom. Did you get all that?"])?;
                        ctx.next()?;
                        if ctx.var("job_merchant_q2").get()? == 1 {
                            ctx.lines_as(
                                "Chief Mahnsoo",
                                args!["Remember, the Serial Number of the package is ^3355FF2485741^000000."],
                            )?;
                        } else {
                            ctx.lines_as(
                                "Chief Mahnsoo",
                                args!["Remember, the Serial Number of the package is ^3355FF2328137^000000."],
                            )?;
                        }
                    } else if ctx.var("job_merchant_q2").get()? == 3 || ctx.var("job_merchant_q2").get()? == 4 {
                        ctx.mes("First, get the delivery package from the storehouse, and then take it to the Mage Guild in Geffen.")?;
                        ctx.next()?;
                        if ctx.var("job_merchant_q2").get()? == 3 {
                            ctx.lines_as("Chief Mahnsoo", args!["When you get there, give the package to the Mage Guildsman in charge. Remember, the packages Serial Number is ^3355FF2989396^000000."])?;
                        } else {
                            ctx.lines_as("Chief Mahnsoo", args!["When you get there, give the package to the Mage Guildsman in charge. Remember, the packages Serial Number is ^3355FF2191737^000000."])?;
                        }
                    } else if ctx.var("job_merchant_q2").get()? == 5 || ctx.var("job_merchant_q2").get()? == 6 {
                        ctx.mes("First, get the delivery package from the storehouse, and then take it to Morocc.")?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Chief Mahnsoo",
                            args!["You'll have to find Java Dullihan, the Dyemaker, so that you can deliver the product he ordered."],
                        )?;
                        ctx.next()?;
                        if ctx.var("job_merchant_q2").get()? == 5 {
                            ctx.lines_as("Chief Mahnsoo", args!["But he's a little forgetful, so give it to one of his students. Remember, the package's Serial Number is ^3355FF3012685^000000."])?;
                        } else {
                            ctx.lines_as("Chief Mahnsoo", args!["But he's a little forgetful, give it to one of his students. Remember, the package's Serial Number is ^3355FF3487372^000000."])?;
                        }
                    } else if ctx.var("job_merchant_q2").get()? == 7 || ctx.var("job_merchant_q2").get()? == 8 {
                        ctx.mes("First, get the package from the storehouse, and then give it to the Kafra Employee stationed on Byalan Island. Her name is Blossom.")?;
                        ctx.next()?;
                        if ctx.var("job_merchant_q2").get()? == 7 {
                            ctx.lines_as(
                                "Chief Mahnsoo",
                                args!["Remember, the package's Serial Number is ^3355FF3318702^000000."],
                            )?;
                        } else {
                            ctx.lines_as(
                                "Chief Mahnsoo",
                                args!["Remember, the package's Serial Number is ^3355FF3543625^000000."],
                            )?;
                        }
                    }
                    if ctx.var("job_merchant_q2").get()? == 7 || ctx.var("job_merchant_q2").get()? == 8 {
                        ctx.next()?;
                        ctx.lines_as(
                            "Chief Mahnsoo",
                            args!["Aaaannnnd...", "Don't forget to deliver that message for me~"],
                        )?;
                    }
                    ctx.next()?;
                    ctx.lines_as(
                        "Chief Mahnsoo",
                        args![
                            "Don't forget your destination and the package's Serial Number.",
                            "You'll need to tell them",
                            "to the storekeeper."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Chief Mahnsoo",
                        args![
                            "The storehouse is in the room",
                            "to my right. There, you can talk",
                            "to the storekeeper, and he'll",
                            "help you out."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Chief Mahnsoo",
                        args![
                            "After you make the delivery, return to the storehouse and give the receipt to the storekeeper.",
                            "Then, come back",
                            "and see me."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Chief Mahnsoo",
                        args!["Is that clear?", "Alright, that's", "the spirit.", "Take care!"],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Chief Mahnsoo",
                        args![
                            "So, what brings you to",
                            "the Merchant Association?",
                            "Is there anything",
                            "I can help you with?"
                        ],
                    )?;
                    ctx.next()?;
                    match ctx.menu(&[
                        "I want to be a Merchant.",
                        "Tell me about Merchants.",
                        "Tell me the requirements.",
                        "Nope.",
                    ])? {
                        0 => {
                            ctx.lines_as("Chief Mahnsoo", args!["Do you want to", "be a Merchant?", "Well..."])?;
                            ctx.next()?;
                            if !shared::other_global_functions::f_canchangejob(ctx, vec![])?.is_true() {
                                ctx.lines_as("Chief Mahnsoo", args!["First, you have to be a Novice with Job Level 10. Once you do that, make sure you learn all of the Basic Skills."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Chief Mahnsoo",
                                    args![
                                        "We're not just",
                                        "simple money makers!",
                                        "We pride ourselves on having standards and only accepting qualified applicants!"
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.lines_as(
                                "Chief Mahnsoo",
                                args!["Alright, you'll need to fill out this application and prepare 1,000 Zeny for your Membership Fee."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Chief Mahnsoo",
                                args![
                                    "Oh...!",
                                    "If you don't have all the money,",
                                    "I can just take 500 Zeny now.",
                                    "You can pay the rest after you",
                                    "pass the test and earn your",
                                    "Merchant Guild License."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Chief Mahnsoo", args!["So what do you think?", "Are you ready to join now?"])?;
                            ctx.next()?;
                            if ctx.menu(&["Yes, I will.", "Ummm, maybe later..."])? == 0 {
                                ctx.lines_as(
                                    "Chief Mahnsoo",
                                    args!["Let me check if you", "filled out everything", "on your application form..."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Chief Mahnsoo",
                                    args!["Hmm... ", ctx.player().name()? + "...", "That's a nice name."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Chief Mahnsoo",
                                    args![
                                        "This application will",
                                        "only be registered once",
                                        "the Membership Fee is paid.",
                                        "How do you wish to",
                                        "handle the fee?"
                                    ],
                                )?;
                                ctx.next()?;
                                match ctx.menu(&["Pay all 1,000 Zeny now!", "Two payments of 500 Zeny.", "Quit"])? {
                                    0 => {
                                        ctx.mes("[Chief Mahnsoo]")?;
                                        if ctx.player().zeny()? >= 1000 {
                                            ctx.var("job_merchant_q").set(Val::from(2))?;
                                            ctx.player().set_zeny(ctx.player().zeny()? - 1000)?;
                                            ctx.lines(args!["Alright~", "That's 1,000 zeny.", "Excellent, excellent."])?;
                                        } else {
                                            ctx.mes("It seems don't have enough zeny to pay all of the fee right now. Why don't you just pay 500 zeny now? Think about it.")?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                    }
                                    1 => {
                                        ctx.mes("[Chief Mahnsoo]")?;
                                        if ctx.player().zeny()? >= 500 {
                                            ctx.var("job_merchant_q").set(Val::from(1))?;
                                            ctx.player().set_zeny(ctx.player().zeny()? - 500)?;
                                            ctx.lines(args!["Let's see...", "That's 500 Zeny. Although I don't think splitting payment is a good idea for any Merchant, it's alright since you're still learning."])?;
                                        } else {
                                            ctx.lines(args!["Hmm...", "It seems you don't have the funds to pay half of the membership fee. Please come back once you collect the zeny that you need."])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                    }
                                    _ => {
                                        ctx.lines_as(
                                            "Chief Mahnsoo",
                                            args!["Feel free to return anytime", "when you are ready, alright?"],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                }
                            } else {
                                ctx.lines_as(
                                    "Chief Mahnsoo",
                                    args![
                                        "You don't have enough zeny now? That's no problem. Take your time and come back when you're",
                                        "ready, okay?"
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.next()?;
                            ctx.lines_as(
                                "Chief Mahnsoo",
                                args!["Alright, you're now on the list of applicants. Ah, before I get started let me say just one thing."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Chief Mahnsoo",
                                args![
                                    "There are some dumb and greedy people out there who do not know what it means to be a Merchant.",
                                    "I hope you won't turn out to be like them, will you?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Chief Mahnsoo",
                                args!["Now, let me", "explain what you", "need to do for the", "Merchant License Test."],
                            )?;
                            ctx.next()?;
                            ctx.mes("[Chief Mahnsoo]")?;
                            match ctx.rand_range(1, 4)? {
                                1 => {
                                    ctx.mes("First, get the delivery package from the storehouse, then go to the former Swordman's Association in Prontera.")?;
                                    ctx.next()?;
                                    ctx.lines_as("Chief Mahnsoo", args!["When you get there, visit the Kafra Employee stationed there. Her name is Blossom. Did you get", "all of that?"])?;
                                    merchant_mer_run(ctx, MerchantMerStep::SGiveSerial, args![2485741, 1, 2328137, 2, 1009])?;
                                }
                                2 => {
                                    ctx.mes("First, get the delivery package from the storehouse, and then go to the Mage Guild in Geffen. When you get there, visit the Mage Guildsman in charge.")?;
                                    merchant_mer_run(ctx, MerchantMerStep::SGiveSerial, args![2989396, 3, 2191737, 4, 1010])?;
                                }
                                3 => {
                                    ctx.mes("First, get the delivery package from the storehouse, and then go to Morocc. There you must find Java Dullihan, the dyemaker.")?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Chief Mahnsoo",
                                        args!["He's a bit forgetful, so you should probably give the package to one of his students."],
                                    )?;
                                    merchant_mer_run(ctx, MerchantMerStep::SGiveSerial, args![3012685, 5, 3487372, 6, 1011])?;
                                }
                                _ => {
                                    ctx.mes("First, get the delivery package from the storehouse, and then give it to the Kafra Employee stationed on Byalan Island.")?;
                                    merchant_mer_run(ctx, MerchantMerStep::SGiveSerial, args![3318702, 7, 3543625, 8, 1012])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Chief Mahnsoo",
                                        args!["Ummmm...", "And I also have", "a bit of a personal", "request for you."],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Chief Mahnsoo",
                                        args!["Would you please give her this message when you deliver the package? Please~"],
                                    )?;
                                    ctx.items().give(1072, 1)?;
                                }
                            }
                            ctx.next()?;
                            ctx.lines_as("Chief Mahnsoo", args!["Don't forget your destination and the package's Serial Number. You will need to tell those to the storekeeper in the storehouse to the right of me."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Chief Mahnsoo",
                                args!["After the delivery, give the receipt to the storekeeper, and then come back and see me."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Chief Mahnsoo",
                                args!["Is that clear?", "Alright, that's", "the spirit.", "Take care!"],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        1 => {
                            ctx.lines_as(
                                "Chief Mahnsoo",
                                args![
                                    "Merchant?",
                                    "Well, we basically sell goods to make money. That is the way",
                                    "of the Merchant."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Chief Mahnsoo", args!["I guess we may not be the best at fighting, and we don't have many special attacks. We've got no healing skills..."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Chief Mahnsoo",
                                args![
                                    "But we can buy goods at lower prices from NPC shops and sell them at a higher price to other people~"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Chief Mahnsoo",
                                args![
                                    "Our ultimate attack skill is 'Mammonite.' The strength of Mammonite comes from the anger",
                                    "when we're forced to throw away perfectly good zeny."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Chief Mahnsoo",
                                args![
                                    "Throwing away zeny like that",
                                    "causes a deadly rage to well up in the heart of any Merchant!",
                                    "Just thinking about it",
                                    "makes my blood boil!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Chief Mahnsoo",
                                args![
                                    "Anyway, we can use most",
                                    "weapons except Bows, Rods, and Two-Handed Swords. But we can always sell those."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Chief Mahnsoo",
                                args!["Yes...", "We Merchants generally", "have money on our minds..."],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as(
                                "Chief Mahnsoo",
                                args!["There are three conditions that must be fulfilled before you can become a Merchant."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Chief Mahnsoo",
                                args!["First, You have to be a Novice with Job Level 10, and have learned all of the Basic Skills."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Chief Mahnsoo", args!["Second, You have to pay a 1,000 Zeny Membership Fee. I believe any Merchant candidate should be able to earn 1,000 Zeny with ease."])?;
                            ctx.next()?;
                            ctx.lines_as("Chief Mahnsoo", args!["Third, there is a License Test to test your physical strength and sense of direction. You will deliver a package to a specific person in a specific location."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                }
                step = MerchantMerStep::SGiveSerial;
                continue 'machine;
            }
            MerchantMerStep::SGiveSerial => {
                ctx.next()?;
                ctx.lines_as("Chief Mahnsoo", args!["Remember...", "The package's", "Serial Number is"])?;
                if ctx.call(Function::Rand, args![2])?.is_true() {
                    ctx.lines(args![
                        Val::from("^3355FF") + runtime::arg(&args, 0, Val::from(0)) + Val::from("^000000.")
                    ])?;
                    ctx.var("job_merchant_q2").set(runtime::arg(&args, 1, Val::from(0)))?;
                } else {
                    ctx.lines(args![
                        Val::from("^3355FF") + runtime::arg(&args, 2, Val::from(0)) + Val::from("^000000.")
                    ])?;
                    ctx.var("job_merchant_q2").set(runtime::arg(&args, 3, Val::from(0)))?;
                }
                ctx.call(Function::SetQuest, vec![runtime::arg(&args, 4, Val::from(0))])?;
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn merchant_mer(ctx: &Ctx) -> Script {
    merchant_mer_run(ctx, MerchantMerStep::Start, Vec::new()).map(|_| ())
}

pub fn merchant_guildsman_mer(ctx: &Ctx) -> Script {
    let mut l_input = Val::from(0);
    if ctx.var("BaseJob").get()? == constants::JOB_MERCHANT {
        ctx.lines_as("Union Staff Kay", args!["Heya pal.", "How ya doin'?"])?;
        return ctx.close();
    } else if ctx.var("BaseJob").get()? != constants::JOB_MERCHANT && ctx.var("BaseJob").get()? != constants::JOB_NOVICE {
        ctx.lines_as(
            "Union Staff Kay",
            args![
                "Hey you. We don't have any open positions for part time work. If you wanna earn some zeny, you'll hafta look elsewhere."
            ],
        )?;
        return ctx.close();
    } else if ctx.var("job_merchant_q").get()? == 9 {
        ctx.lines_as(
            "Union Staff Kay",
            args![
                "Hey you. Yeah, you.",
                "If you wanna restart the test, go visit Mahnsoo in the other room. Then we can talk."
            ],
        )?;
        return ctx.close();
    } else if ctx.var("job_merchant_q").get()? == 8 || ctx.var("job_merchant_q").get()? == 7 {
        ctx.lines_as(
            "Union Staff Kay",
            args!["Alright! Everything looks perfect! I'll report your success to the guildmaster. Now go talk to Chief Mahnsoo, yeah?"],
        )?;
        return ctx.close();
    } else if ctx.var("job_merchant_q").get()? == 6 || ctx.var("job_merchant_q").get()? == 5 {
        ctx.lines(args![
            " [Union Staff Kay] ",
            format!(
                "Oh, yeah? Okay, lemme check. Your name is {}? Alright, your destination was...",
                ctx.player().name()?
            )
        ])?;
        ctx.next()?;
        ctx.mes("[Union Staff Kay]")?;
        if ctx.var("job_merchant_q2").get()? == 1 || ctx.var("job_merchant_q2").get()? == 2 {
            ctx.mes("Wow! You met the Kafra babe in Prontera?! Lucky you~ ...Receipt?")?;
        } else if ctx.var("job_merchant_q2").get()? == 3 || ctx.var("job_merchant_q2").get()? == 4 {
            ctx.mes("Geffen Magic Academy. Okay, receipt?")?;
        } else if ctx.var("job_merchant_q2").get()? == 5 || ctx.var("job_merchant_q2").get()? == 6 {
            ctx.mes("The dyemaker in Morocc. Not bad. Receipt?")?;
        } else if ctx.var("job_merchant_q2").get()? == 7 || ctx.var("job_merchant_q2").get()? == 8 {
            ctx.mes("Oh hohohoho~! The Kafra Babe on Byalan Island?! Awesome! Anyway, did you bring the receipt?")?;
        }
        if ctx.var("job_merchant_q2").get()? == 1 && ctx.call(Function::CountItem, args![1073])? != 0 {
            ctx.items().take(1073, 1)?;
        } else if ctx.var("job_merchant_q2").get()? == 2 && ctx.call(Function::CountItem, args![1074])? != 0 {
            ctx.items().take(1074, 1)?;
        } else if ctx.var("job_merchant_q2").get()? == 3 && ctx.call(Function::CountItem, args![1075])? != 0 {
            ctx.items().take(1075, 1)?;
        } else if ctx.var("job_merchant_q2").get()? == 4 && ctx.call(Function::CountItem, args![1076])? != 0 {
            ctx.items().take(1076, 1)?;
        } else if ctx.var("job_merchant_q2").get()? == 5 && ctx.call(Function::CountItem, args![1077])? != 0 {
            ctx.items().take(1077, 1)?;
        } else if ctx.var("job_merchant_q2").get()? == 6 && ctx.call(Function::CountItem, args![1078])? != 0 {
            ctx.items().take(1078, 1)?;
        } else if ctx.var("job_merchant_q2").get()? == 7 && ctx.call(Function::CountItem, args![1079])? != 0 {
            ctx.items().take(1079, 1)?;
        } else if ctx.var("job_merchant_q2").get()? == 8 && ctx.call(Function::CountItem, args![1080])? != 0 {
            ctx.items().take(1080, 1)?;
        } else {
            ctx.next()?;
            ctx.var("job_merchant_q").set(Val::from(9))?;
            ctx.lines(args![
                " [Union Staff Kay] ",
                "Wait a sec.",
                "Where's the receipt?",
                "What happened?"
            ])?;
            ctx.next()?;
            ctx.lines(args![" [Union Staff Kay] ", "If you don't have the receipt, you fail the test! You better talk to Mahnsoo if you wanna retake it, alright? Pay attention next time!"])?;
            return ctx.close();
        }
        ctx.next()?;
        ctx.lines(args![
            " [Union Staff Kay] ",
            "...Great! Everything's perfect! I'll report your success to the Guildmaster. You should talk to Chief Mahnsoo now, alright?"
        ])?;
        ctx.close_window()?;
        if ctx.var("job_merchant_q").get()? == 6 {
            ctx.var("job_merchant_q").set(Val::from(8))?;
        } else if ctx.var("job_merchant_q").get()? == 5 {
            ctx.var("job_merchant_q").set(Val::from(7))?;
        }
        return ctx.end();
    } else if ctx.var("job_merchant_q").get()? == 4
        || (ctx.var("job_merchant_q").get()? == 3
            && ctx.call(Function::CountItem, args![1081])? == 0
            && ctx.call(Function::CountItem, args![1082])? == 0
            && ctx.call(Function::CountItem, args![1091])? == 0)
    {
        ctx.lines(args![
            " [Union Staff Kay] ",
            "Huh?",
            "You're back?",
            "So how did",
            "the delivery go?"
        ])?;
        ctx.next()?;
        if ctx.menu(&["*Sob* I lost the package.", "Fine."])? == 0 {
            ctx.var("job_merchant_q").set(Val::from(9))?;
            ctx.lines_as(
                "Union Staff Kay",
                args!["Are you kidding me? You'll fail the test if you lose the package!"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Union Staff Kay",
                args!["Awwww man. Well, if you wanna restart the test, talk to Mahnsoo, okay? You're lucky you're getting another chance!"],
            )?;
            return ctx.close();
        }
        ctx.lines_as("Union Staff Kay", args!["Huh...", "Okay..."])?;
        return ctx.close();
    } else if ctx.var("job_merchant_q").get()? == 4
        || (ctx.var("job_merchant_q").get()? == 3 && ctx.call(Function::CountItem, args![1081])? != 0)
        || ctx.call(Function::CountItem, args![1082])? != 0
        || ctx.call(Function::CountItem, args![1091])? != 0
    {
        ctx.lines_as(
            "Union Staff Kay",
            args!["Hey, what are you still doing here? Shouldn't you be on your way already?"],
        )?;
        ctx.next()?;
        if ctx.menu(&["I need a new package.", "Oh, yeah. You're right!"])? == 0 {
            if ctx.call(Function::CountItem, args![1081])? == 0
                && ctx.call(Function::CountItem, args![1082])? == 0
                && ctx.call(Function::CountItem, args![1083])? == 0
                && ctx.call(Function::CountItem, args![1091])? == 0
            {
                ctx.lines_as(
                    "Union Staff Kay",
                    args!["Wha--?", "So where did", "the package go?", "Where is it?!"],
                )?;
                ctx.next()?;
                if ctx.menu(&["*Sob* I lost it!", "I have it right here."])? == 0 {
                    ctx.var("job_merchant_q").set(Val::from(9))?;
                    ctx.lines_as("Union Staff Kay", args!["You...", "Lost it?!", "You failed the test!"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Union Staff Kay",
                        args!["*Sigh* If you want to restart the test, go visit Mahnsoo in the other room, alright?"],
                    )?;
                    return ctx.close();
                }
                ctx.lines_as(
                    "Union Staff Kay",
                    args!["Huh.", "I thought", "you lost it.", "You don't", "need a new one."],
                )?;
                return ctx.close();
            }
            ctx.lines_as(
                "Union Staff Kay",
                args!["*Sigh* Man, you're starting to become a pain in the ass. Hold on, lemme cancel your record..."],
            )?;
            if ctx.call(Function::CountItem, args![1081])? != 0 {
                ctx.items().take(1081, 1)?;
            } else if ctx.call(Function::CountItem, args![1082])? != 0 {
                ctx.items().take(1082, 1)?;
            } else if ctx.call(Function::CountItem, args![1091])? != 0 {
                ctx.items().take(1091, 1)?;
            }
            if ctx.var("job_merchant_q").get()? == 4 {
                ctx.var("job_merchant_q").set(Val::from(2))?;
            } else if ctx.var("job_merchant_q").get()? == 3 {
                ctx.var("job_merchant_q").set(Val::from(1))?;
            }
            ctx.next()?;
            ctx.lines_as(
                "Union Staff Kay",
                args!["I need some time to get everything in order, so come back later."],
            )?;
            return ctx.close();
        }
        ctx.lines_as("Union Staff Kay", args!["What a bummer..."])?;
        return ctx.close();
    } else if ctx.var("job_merchant_q").get()? == 0 || ctx.var("job_merchant_q").get()? == 1 || ctx.var("job_merchant_q").get()? == 2 {
        ctx.var("where_village").set(Val::from(0))?;
        ctx.lines_as("Union Staff Kay", args!["Hey there.", "what brings", "you here?"])?;
        ctx.next()?;
        match ctx.menu(&["My Merchant License test.", "I'm looking for part time work.", "Nothing."])? {
            0 => {
                ctx.lines_as(
                    "Union Staff Kay",
                    args!["I see.", "Alright.", "So what's", "your name?", ctx.player().name()? + "...?"],
                )?;
                ctx.next()?;
                if ctx.var("job_merchant_q").get()? == 0 {
                    ctx.lines_as(
                        "Union Staff Kay",
                        args!["Huh. Your name's not on my list. Did you apply for the job change quest or what?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Union Staff Kay",
                        args![
                            "You gotta apply first by talking to Chief Mahnsoo in the center",
                            "of this building, okay?"
                        ],
                    )?;
                    return ctx.close();
                }
                ctx.lines_as(
                    "Union Staff Kay",
                    args!["Alright, there you go. Lemme give you the package. Now, choose the destination of the delivery."],
                )?;
                ctx.next()?;
                let l_where_village = match ctx.menu(&["Prontera.", "Geffen.", "Morocc.", "Byalan Island."])? {
                    0 => 1,
                    1 => 2,
                    2 => 3,
                    _ => 4,
                };
                ctx.lines_as(
                    "Union Staff Kay",
                    args!["Okay, now you need to give me the package's Serial Number. If you wanna cancel, just enter '0', alright?"],
                )?;
                ctx.next()?;
                loop {
                    loop {
                        let (input, _) = runtime::input_number(ctx, None, None)?;
                        l_input = input;
                        if l_input == 0 {
                            ctx.lines_as("Union Staff Kay", args!["Are you sure that you wanna cancel?"])?;
                            if ctx.menu(&["Yes.", "Let me try again."])? == 0 {
                                ctx.mes("Alright, we'll cancel for now.")?;
                                return ctx.close();
                            }
                            ctx.next()?;
                        } else if l_input.number()? < 1000000 || l_input.number()? > 5000000 {
                            ctx.lines_as(
                                "Union Staff Kay",
                                args!["Hey hey. That number's not valid! Enter a value from 1000000 to 5000000. got it?"],
                            )?;
                            ctx.next()?;
                        } else {
                            break;
                        }
                    }
                    ctx.mes("[Union Staff Kay]")?;
                    if l_where_village == 1 {
                        ctx.lines(args![
                            Val::from("Destination is Prontera. The Serial Number is ")
                                + l_input.clone()
                                + Val::from(". Are you positive?")
                        ])?;
                    } else if l_where_village == 2 {
                        ctx.lines(args![
                            Val::from("Destination is Geffen. Phew! That's really far! The Serial Number is ")
                                + l_input.clone()
                                + Val::from(". Are you positive?")
                        ])?;
                    } else if l_where_village == 3 {
                        ctx.lines(args![
                            Val::from("Destination is Morocc. That's pretty far away! The Serial Number is ")
                                + l_input.clone()
                                + Val::from(". Are you positive?")
                        ])?;
                    } else {
                        ctx.lines(args![
                            Val::from("Lucky you! Your destination is Byalan Island. The Serial Number is ")
                                + l_input.clone()
                                + Val::from(". Are you positive?")
                        ])?;
                    }
                    ctx.next()?;
                    if ctx.menu(&["Positive.", "Whoops! Wrong number!"])? == 0 {
                        break;
                    }
                }
                if l_where_village == 1 {
                    if ctx.var("job_merchant_q2").get()? == 1 && l_input == 2485741 {
                        ctx.items().give(1081, 1)?;
                    } else if ctx.var("job_merchant_q2").get()? == 2 && l_input == 2328137 {
                        ctx.items().give(1082, 1)?;
                    } else {
                        ctx.items().give(1091, 1)?;
                    }
                } else if l_where_village == 2 {
                    if ctx.var("job_merchant_q2").get()? == 3 && l_input == 2989396 {
                        ctx.items().give(1081, 1)?;
                    } else if ctx.var("job_merchant_q2").get()? == 4 && l_input == 2191737 {
                        ctx.items().give(1082, 1)?;
                    } else {
                        ctx.items().give(1091, 1)?;
                    }
                } else if l_where_village == 3 {
                    if ctx.var("job_merchant_q2").get()? == 5 && l_input == 3012685 {
                        ctx.items().give(1081, 1)?;
                    } else if ctx.var("job_merchant_q2").get()? == 6 && l_input == 3487372 {
                        ctx.items().give(1082, 1)?;
                    } else {
                        ctx.items().give(1091, 1)?;
                    }
                } else if ctx.var("job_merchant_q2").get()? == 7 && l_input == 3318702 {
                    ctx.items().give(1081, 1)?;
                } else if ctx.var("job_merchant_q2").get()? == 8 && l_input == 3543625 {
                    ctx.items().give(1082, 1)?;
                } else {
                    ctx.items().give(1091, 1)?;
                }
                if ctx.var("job_merchant_q").get()? == 2 {
                    ctx.var("job_merchant_q").set(Val::from(4))?;
                } else if ctx.var("job_merchant_q").get()? == 1 {
                    ctx.var("job_merchant_q").set(Val::from(3))?;
                }
                ctx.lines_as("Union Staff Kay", args!["Alright. Take this package and guard it with your life until it's safely delivered to the customer. Don't lose this thing, got it?"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Union Staff Kay",
                    args![
                        "Well then, I wish you luck. Remember, you gotta bring me",
                        "a receipt once you finish the delivery, okay?"
                    ],
                )?;
                return ctx.close();
            }
            1 => {
                ctx.lines_as(
                    "Union Staff Kay",
                    args!["Part time job? Sorry pal, no jobs yet. The Paymaster's department can never balance our budget..."],
                )?;
                return ctx.close();
            }
            _ => {
                ctx.lines_as(
                    "Union Staff Kay",
                    args!["Nothing, eh?", "I guess you enjoy", "bothering people for", "no reason then, yeah?"],
                )?;
                return ctx.close();
            }
        }
    }
    Ok(())
}

pub fn student_mer(ctx: &Ctx) -> Script {
    if ctx.var("job_merchant_q").get()? == 4 || ctx.var("job_merchant_q").get()? == 3 {
        ctx.lines_as(
            "Dyer's Student",
            args!["You're from", "the Merchant Guild?", "Yes! You've come to", "the right place."],
        )?;
        ctx.next()?;
        ctx.mes("[Dyer's Student]")?;
        if ctx.call(Function::CountItem, args![1081])? == 1
            || ctx.call(Function::CountItem, args![1082])? == 1
            || ctx.call(Function::CountItem, args![1091])? == 1
        {
            ctx.lines(args!["Okay~", "Please set the", "package down", "over there."])?;
        } else {
            ctx.lines(args!["But...", "Where's the", "package I ordered?", "That's strange..."])?;
            return ctx.close();
        }
        ctx.next()?;
        ctx.lines_as(
            "Dyer's Student",
            args!["Let me check the Serial Number of the package so I can give you the receipt, okay?"],
        )?;
        ctx.next()?;
        ctx.mes("[Dyer's Student]")?;
        if ctx.var("job_merchant_q2").get()? == 5 && ctx.call(Function::CountItem, args![1081])? != 0 {
            ctx.lines(args!["3012685...", "That's right.", "Here's your", "receipt."])?;
            ctx.items().take(1081, 1)?;
            ctx.items().give(1077, 1)?;
        } else if ctx.var("job_merchant_q2").get()? == 6 && ctx.call(Function::CountItem, args![1082])? != 0 {
            ctx.lines(args!["3487372...", "That's right.", "Here's your", "receipt."])?;
            ctx.items().take(1082, 1)?;
            ctx.items().give(1078, 1)?;
        } else {
            ctx.mes("Excuse me, but...")?;
            if ctx.var("job_merchant_q2").get()? == 5 {
                ctx.mes("I don't think this is the package we ordered. The Serial Number should be 3012685. See?")?;
            } else if ctx.var("job_merchant_q2").get()? == 6 {
                ctx.mes("I don't think this is the package we ordered. The Serial Number should be 3487372. See?")?;
            } else {
                ctx.mes("I don't think this is the package we ordered. The Serial Number should be 3012685 or 3487372. Well, one of those two...")?;
            }
            return ctx.close();
        }
        if ctx.var("job_merchant_q").get()? == 4 {
            ctx.var("job_merchant_q").set(Val::from(6))?;
        } else if ctx.var("job_merchant_q").get()? == 3 {
            ctx.var("job_merchant_q").set(Val::from(5))?;
        }
        ctx.next()?;
        ctx.lines_as("Dyer's Student", args!["Thanks a lot!", "See you again", "sometime!"])?;
        return ctx.close();
    } else if (ctx.var("job_merchant_q").get()? == 6 || (ctx.var("job_merchant_q").get()? == 5 && ctx.var("job_merchant_q2").get()? == 6))
        || ctx.var("job_merchant_q2").get()? == 5
    {
        ctx.lines_as(
            "Dyer's Student",
            args!["Oh...", "You're gonna", "go back? Okay", "then, take care!"],
        )?;
        return ctx.close();
    } else {
        ctx.lines_as(
            "Dyer's Student",
            args!["Mr. Java Dullihan is the one and only, the best dye maker on the Midgard continent."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Dyer's Student",
            args!["Aaaand I'm proud to say that I'm his student! Someday, I'll be able to make really beautiful dyes too!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Dyer's Student",
            args!["Of course, I'm still learning the basics right now, but someday..."],
        )?;
        return ctx.close();
    }
}

pub fn guild_staff_mer(ctx: &Ctx) -> Script {
    if ctx.var("job_merchant_q").get()? == 4 || ctx.var("job_merchant_q").get()? == 3 {
        ctx.lines_as(
            "Guild Staff",
            args!["Ah, you must be with the Merchant Guild. Finally, my package has arrived! Alright...!"],
        )?;
        ctx.next()?;
        ctx.mes("[Guild Staff]")?;
        if ctx.call(Function::CountItem, args![1081])? == 1
            || ctx.call(Function::CountItem, args![1082])? == 1
            || ctx.call(Function::CountItem, args![1091])? == 1
        {
            ctx.lines(args![
                "You must be very tired",
                "from having to travel",
                "in this kind",
                "of weather..."
            ])?;
        } else {
            ctx.lines(args!["Wait...", "Where's the", "package?"])?;
            return ctx.close();
        }
        ctx.next()?;
        ctx.lines_as("Guild Staff", args!["Alright, let me", "check the Serial Number..."])?;
        if ctx.var("job_merchant_q2").get()? == 3 && ctx.call(Function::CountItem, args![1081])? != 0 {
            ctx.mes("2989396. Yes, this is what we ordered. Here is your receipt.")?;
            ctx.items().take(1081, 1)?;
            ctx.items().give(1075, 1)?;
        } else if ctx.var("job_merchant_q2").get()? == 4 && ctx.call(Function::CountItem, args![1082])? != 0 {
            ctx.mes("2191737. Yes, this is what we ordered. Here is your receipt.")?;
            ctx.items().take(1082, 1)?;
            ctx.items().give(1076, 1)?;
        } else {
            ctx.mes("Uh oh, this is the wrong number. This isn't what we ordered...")?;
            ctx.next()?;
            ctx.mes("[Guild Staff]")?;
            if ctx.var("job_merchant_q2").get()? == 3 {
                ctx.lines(args!["The Serial Number", "should be 2989396."])?;
            } else if ctx.var("job_merchant_q2").get()? == 4 {
                ctx.lines(args!["The Serial Number", "should be 2191737."])?;
            } else {
                ctx.lines(args![
                    "The Serial Number",
                    "should be 2989396",
                    "or 2191737, one of",
                    "those two."
                ])?;
            }
            ctx.lines(args!["Look here!", "Don't you see", "something", "is wrong?"])?;
            return ctx.close();
        }
        if ctx.var("job_merchant_q").get()? == 4 {
            ctx.var("job_merchant_q").set(Val::from(6))?;
        } else if ctx.var("job_merchant_q").get()? == 3 {
            ctx.var("job_merchant_q").set(Val::from(5))?;
        }
        ctx.next()?;
        ctx.lines_as("Guild Staff", args!["Heh heh~", "Thank you!", "Bye bye!"])?;
        return ctx.close();
    } else if (ctx.var("job_merchant_q").get()? == 6 || (ctx.var("job_merchant_q").get()? == 5 && ctx.var("job_merchant_q2").get()? == 4))
        || ctx.var("job_merchant_q2").get()? == 3
    {
        ctx.lines_as("Guild Staff", args!["Hello,", "Merchant Guildsman~", "I give you my thanks."])?;
        return ctx.close();
    } else {
        ctx.lines_as(
            "Guild Staff",
            args!["My package should have arrived by now. Huh. I guess the Merchant Guild might be running a little late..."],
        )?;
        return ctx.close();
    }
}

pub fn kafra_employee_mer(ctx: &Ctx) -> Script {
    ctx.fx().cutin("kafra_02", 2)?;
    if (ctx.var("job_merchant_q").get()? == 6 || ctx.var("job_merchant_q").get()? == 5)
        && (ctx.var("job_merchant_q2").get()? == 2 || ctx.var("job_merchant_q2").get()? == 1)
    {
        ctx.lines_as(
            "Kafra Employee",
            args!["Oh! Thank you for", "traveling such a long", "way to come over here~"],
        )?;
        ctx.close_window()?;
        ctx.fx().cutin("", 255)?;
        return ctx.end();
    } else if ctx.var("job_merchant_q").get()? == 4 || ctx.var("job_merchant_q").get()? == 3 {
        ctx.lines_as(
            "Kafra Employee",
            args![
                "A delivery from",
                "the Merchant Guild?",
                "Oh, yes, please set",
                "it down right over there..."
            ],
        )?;
        if ctx.call(Function::CountItem, args![1081])? == 1
            || ctx.call(Function::CountItem, args![1082])? == 1
            || ctx.call(Function::CountItem, args![1091])? == 1
        {
            ctx.lines(args!["You must be really tired", "after carrying it for so long!"])?;
        } else {
            ctx.lines(args!["W-wait. Didn't you bring it?", "Where's the package?"])?;
            ctx.close_window()?;
            ctx.fx().cutin("", 255)?;
            return ctx.end();
        }
        ctx.next()?;
        ctx.lines_as("Kafra Employee", args!["Now, let me check", "the serial number..."])?;
        if ctx.var("job_merchant_q2").get()? == 1 && ctx.call(Function::CountItem, args![1081])? != 0 {
            ctx.lines(args![
                "2485741. Right, this is",
                "the one we ordered. Oh,",
                "and don't forget this receipt!"
            ])?;
            ctx.next()?;
            ctx.items().take(1081, 1)?;
            ctx.items().give(1073, 1)?;
        } else if ctx.var("job_merchant_q2").get()? == 2 && ctx.call(Function::CountItem, args![1082])? != 0 {
            ctx.lines(args![
                "2328137. Right, this is",
                "the one we ordered. Oh,",
                "and don't forget this receipt!"
            ])?;
            ctx.next()?;
            ctx.items().take(1082, 1)?;
            ctx.items().give(1074, 1)?;
        } else {
            ctx.lines(args!["Mmmm? Hold on. This is", "the wrong package. What we"])?;
            if ctx.var("job_merchant_q2").get()? == 1 {
                ctx.mes("ordered had the serial number 2485741. I'm sure it's not this.")?;
            } else if ctx.var("job_merchant_q2").get()? == 2 {
                ctx.mes("ordered had the serial number 2328137. I'm sure it's not this.")?;
            } else {
                ctx.mes("ordered had the serial number 2485741 or 2328137.")?;
            }
            ctx.next()?;
            ctx.lines_as(
                "Kafra Employee",
                args![
                    "I'm afraid there",
                    "must be some kind",
                    "of mistake. Perhaps",
                    "you should go back to",
                    "the Merchant Guild to",
                    "clear up this situation?"
                ],
            )?;
            ctx.close_window()?;
            ctx.fx().cutin("", 255)?;
            return ctx.end();
        }
        if ctx.var("job_merchant_q").get()? == 4 {
            ctx.var("job_merchant_q").set(Val::from(6))?;
        } else if ctx.var("job_merchant_q").get()? == 3 {
            ctx.var("job_merchant_q").set(Val::from(5))?;
        }
        ctx.lines_as(
            "Kafra Employee",
            args!["Thanks again", "for going through", "all of that trouble~"],
        )?;
        ctx.close_window()?;
        ctx.fx().cutin("", 255)?;
        return ctx.end();
    } else {
        ctx.lines_as(
            "Kafra Employee",
            args![
                "Welcome to the",
                "Kafra Corportation,",
                "where the service is",
                "always on your side~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kafra Employee",
            args![
                "As you can see, the",
                "Swordman Assocation",
                "has moved to Izlude, a",
                "satellite city of Prontera.",
                "Currently, we offer a Teleport",
                "Service to Izlude for 600 zeny."
            ],
        )?;
        ctx.next()?;
        if ctx.menu(&["Use", "Cancel"])? == 0 {
            if ctx.player().zeny()? < 600 {
                ctx.lines_as(
                    "Kafra Employee",
                    args!["I'm sorry, but you", "don't have enough zeny", "for this Teleport Service."],
                )?;
                ctx.close_window()?;
                ctx.fx().cutin("", 255)?;
                return ctx.end();
            }
            ctx.player().set_zeny(ctx.player().zeny()? - 600)?;
            ctx.var("resrvpts").set(ctx.var("resrvpts").get()? + Val::from(37))?;
            ctx.fx().cutin("", 255)?;
            ctx.warp("izlude", 94, 103)?;
            return ctx.end();
        }
        ctx.close_window()?;
        ctx.fx().cutin("", 255)?;
        return ctx.end();
    }
}

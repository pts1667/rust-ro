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

pub fn guarnien(ctx: &Ctx) -> Script {
    ctx.mes("[Guarnien]")?;
    if ctx.player().class()? != constants::JOB_MERCHANT {
        ctx.lines(args![
            "Hello stranger.",
            "I wish I could",
            "offer you help",
            "on your adventures..."
        ])?;
        ctx.next()?;
        ctx.lines_as("Guarnien", args!["However, what I have to teach would only benefit Merchants. Trading and money making are my areas of expertise. If you have any Merchant friends, send them to me."])?;
        if ctx.player().class()? == constants::JOB_MERCHANT_HIGH {
            ctx.next()?;
            ctx.lines_as("Guarnien", args!["Technically, yes, you're a Merchant, but I know an expert when I see one. You've been through the Merchant life before, I can tell!"])?;
        }
        return ctx.close();
    }
    match ctx.var("tu_merchant").get()?.number()? {
        17 => {
            ctx.lines(args![
                "We can't even begin",
                "to fathom the number",
                "of markets and people",
                "out there in this wide,",
                "wide world of ours."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Guarnien",
                args![
                    "All of them are possible customers and business partners. Of course, it'll be up to you to meet them.",
                    "If you can be sincerely kind to everyone that you meet, you'll",
                    "have no problem."
                ],
            )?;
            return ctx.close();
        }
        15 | 16 => {
            let l_chk_vend = ctx.call(Function::GetSkillLv, args!["MC_VENDING"])?;
            if ctx.var("tu_merchant").get()? == 15 {
                ctx.lines(args![
                    "I believe it's time",
                    "for my final lecture.",
                    "I'll discuss the definitive",
                    "skill of the Merchant: ^871F78Vending^000000. Please listen carefully."
                ])?;
                ctx.next()?;
                ctx.lines_as("Guarnien", args!["The Vending skill is an active skill which allows you to open your own shop where you stand. Vending allows you to do real business with other people."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Guarnien",
                    args![
                        "However, you can only vend items that are stored in your PushCart,",
                        "so you'll need one of those. The maximum price you can set for",
                        "each item is 99,990,000 Zeny."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Guarnien",
                    args!["Basically, you've got to learn the Push Cart skill first before you can learn the Vending skill."],
                )?;
                ctx.next()?;
                ctx.lines_as("Guarnien", args!["The higher the level of the Vending skill, the more items you can vend at one time. Vending is mastered at Skill Level 10."])?;
                ctx.next()?;
                ctx.lines_as("Guarnien", args!["Level 1 Vending allows you", "to vend a total of 3 items at one time. Each time you level up the Vending skill, the total number of items you can vend will", "increase by one."])?;
                ctx.next()?;
                ctx.mes("[Guarnien]")?;
                if l_chk_vend.number()? > 3 && l_chk_vend.number()? < 10 {
                    ctx.mes("But I see that you must already know that, huh? Well, seeing as you're so clever, I think you've earned this little prize~")?;
                    ctx.call(Function::GetExperience, args![186, 60])?;
                } else if l_chk_vend == 10 {
                    ctx.mes("But it looks like I just put my foot in my mouth. I'm sorry, I didn't notice that you already mastered the Vending skill...")?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Guarnien",
                        args![
                            "You're such an",
                            "excellent student!",
                            "Please, take this",
                            "little reward for",
                            "all your efforts~"
                        ],
                    )?;
                    ctx.call(Function::GetExperience, args![199, 69])?;
                } else {
                    ctx.mes("For your final assignment, I will ask you to learn the Vending skill up to Skill Level 4. Of course, once you do, I'll give you a little reward. I'll be waiting, so do your best~")?;
                    ctx.var("tu_merchant").set(Val::from(16))?;
                    ctx.quests().erase(8239)?;
                    ctx.quests().start(8240)?;
                    return ctx.close();
                }
                ctx.quests().complete(8239)?;
            } else {
                ctx.lines(args!["So...", "How much have", "you learned", "about Vending?"])?;
                ctx.next()?;
                ctx.mes("[Guarnien]")?;
                if l_chk_vend.number()? > 3 && l_chk_vend.number()? < 10 {
                    ctx.lines(args![
                        "Excellent!",
                        "You've taught yourself well. Here, you've earned this little reward~"
                    ])?;
                    ctx.call(Function::GetExperience, args![186, 43])?;
                } else if l_chk_vend == 10 {
                    ctx.mes("Oh, I see that you're very serious about being a Merchant. I'm proud that you've managed to master this skill! Here, you deserve this reward!")?;
                    ctx.call(Function::GetExperience, args![199, 69])?;
                } else {
                    ctx.lines(args![
                        "Hmm...",
                        "Haven't learned Level 4 Vending yet, huh? Just keep in mind that being able to vend things is one",
                        "of the defining characteristics of being a Merchant!"
                    ])?;
                    return ctx.close();
                }
                ctx.quests().complete(8240)?;
            }
            ctx.var("tu_merchant").set(Val::from(17))?;
            ctx.next()?;
            ctx.lines_as("Guarnien", args!["Ah, now you have mastered the fundamentals of the Merchant class. Just keep practicing the basic principles, like researching the market for item prices, and you should be fine."])?;
            ctx.next()?;
            ctx.lines_as("Guarnien", args!["Although everything we do is for money, never forget customer needs and always sell your products at a suitably fair price. Otherwise, no one may buy your goods!"])?;
            ctx.next()?;
            ctx.lines_as(
                "Guarnien",
                args!["Well, there's nothing more I can teach you for now. Go out into the world and make your mark as a Merchant."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Guarnien",
                args![
                    "And feel free to",
                    "visit me whenever",
                    "the mood strikes you.",
                    "Good luck to you~"
                ],
            )?;
            return ctx.close();
        }
        13 | 14 => {
            let mut l_chk_cart = ctx.call(Function::GetSkillLv, args!["MC_PUSHCART"])?;
            if ctx.var("tu_merchant").get()? == 13 {
                ctx.lines(args!["Now to talk about", "^871F78Push Cart^000000. First off, you need to know Level 5 Increase Weight Limit before you can even learn the Push Cart skill."])?;
                ctx.next()?;
                ctx.lines_as("Guarnien", args!["Push Cart can be mastered at", "Skill Level 10. This skill allows you to carry a Push Cart with you. You can carry a maximum of 100 different kinds of items that weigh less than 8,000."])?;
                ctx.next()?;
                ctx.lines_as("Guarnien", args!["As you can see, this skill is an advanced form of the Increase Weight Limit skill. You can procure a PushCart from most Kafra Ladies for a certain fee."])?;
                ctx.next()?;
                ctx.lines_as("Guarnien", args!["Once you have rented a Cart, you can hold on to it so long as you don't take it off. The disadvantage to using a PushCart is that your movement speed is reduced to", "almost half! Horrible, isn't it?"])?;
                ctx.next()?;
                ctx.lines_as("Guarnien", args!["Fortunately, when you increase", "the level of the Push Cart skill, your movement speed will slowly be restored. With Push Cart Level 10, a PushCart won't slow you", "down at all!"])?;
                ctx.next()?;
                ctx.quests().erase(8237)?;
                l_chk_cart = ctx.call(Function::GetSkillLv, args!["MC_PUSHCART"])?;
                ctx.mes("[Guarnien]")?;
                if l_chk_cart.number()? > 3 && l_chk_cart.number()? < 10 {
                    ctx.lines(args!["Ah, and I see that you've been focusing on learning Push Cart. Although you might still have a problem with your movement", "speed, it shouldn't be a big deal.", "Well done~"])?;
                    ctx.call(Function::GetExperience, args![162, 52])?;
                } else if l_chk_cart == 10 {
                    ctx.lines(args![
                        "Wonderful! You've already",
                        "mastered the Push Cart skill.",
                        "You truly deserve a little bit of a reward~"
                    ])?;
                    ctx.call(Function::GetExperience, args![186, 60])?;
                } else {
                    ctx.lines(args![
                        "For now, why don't",
                        "you learn the Push Cart",
                        "skill up to Level 4? Once",
                        "you do that, I can go on to",
                        "my next lecture."
                    ])?;
                    ctx.var("tu_merchant").set(Val::from(14))?;
                    ctx.quests().start(8238)?;
                    return ctx.close();
                }
                ctx.quests().start(8239)?;
            } else {
                ctx.lines(args!["So, have you", "learned Push Cart up", "to Level 4 like I asked?"])?;
                ctx.next()?;
                ctx.mes("[Guarnien]")?;
                if l_chk_cart.number()? > 3 && l_chk_cart.number()? < 10 {
                    ctx.lines(args![
                        "Well done~",
                        "You've been a very",
                        "cooperative student.",
                        "You deserve a little",
                        "reward for your effort~"
                    ])?;
                    ctx.call(Function::GetExperience, args![162, 52])?;
                } else if l_chk_cart == 10 {
                    ctx.lines(args![
                        "Excellent!",
                        "You've actually went above and beyond mastered the Push Cart skill. Great work!"
                    ])?;
                    ctx.call(Function::GetExperience, args![186, 60])?;
                } else {
                    ctx.lines(args![
                        "Still not there, huh?",
                        "Well, remember that you need",
                        "Level 5 Increase Weight Limit before you can learn the Push",
                        "Cart skill. I hope that helps."
                    ])?;
                    return ctx.close();
                }
                ctx.quests().erase(8238)?;
                ctx.quests().start(8239)?;
            }
            ctx.var("tu_merchant").set(Val::from(15))?;
            ctx.next()?;
            ctx.lines_as("Guarnien", args!["Next time, I'll talk about the skill that sets Merchants apart from all the other classes: ^871F78Vending^000000. So don't miss it!"])?;
            return ctx.close();
        }
        11 | 12 => {
            let mut l_chk_over = ctx.call(Function::GetSkillLv, args!["MC_OVERCHARGE"])?;
            if ctx.var("tu_merchant").get()? == 11 {
                ctx.lines(args![
                    "Ah, ready to",
                    "learn already,",
                    "are you? Alright,",
                    "let me tell you",
                    "about ^871F78Over Charge^000000."
                ])?;
                ctx.next()?;
                ctx.lines_as("Guarnien", args!["Like Discount, Over Charge is a Passive skill that can be mastered at Skill Level 10. Therefore, it doesn't require SP to use it."])?;
                ctx.next()?;
                ctx.lines_as("Guarnien", args!["Over Charge allows you to sell items to NPC shops for more Zeny. The higher your Over Charge Skill Level, the more Zeny you'll receive for items sold to NPCs."])?;
                ctx.next()?;
                ctx.mes("[Guarnien]")?;
                ctx.quests().erase(8235)?;
                l_chk_over = ctx.call(Function::GetSkillLv, args!["MC_OVERCHARGE"])?;
                if l_chk_over.number()? > 3 && l_chk_over.number()? < 10 {
                    ctx.mes("Ah, you've already learned Level 4 Over Charge. Perfect! Let me give you a little reward~")?;
                    ctx.call(Function::GetExperience, args![126, 27])?;
                } else if l_chk_over == 10 {
                    ctx.mes("Amazing! You've already mastered the Over Charge skill! You must have wanted this reward badly~")?;
                    ctx.call(Function::GetExperience, args![142, 33])?;
                } else {
                    ctx.mes("Personally, I think you should learn this skill for sure! For your next assignment, learn Over Charge up to Level 4. It shouldn't be too difficult to do.")?;
                    ctx.var("tu_merchant").set(Val::from(12))?;
                    ctx.quests().start(8236)?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Guarnien",
                        args!["Once you do that,", "we can talk about", "more advanced stuff.", "Do your best~"],
                    )?;
                    return ctx.close();
                }
                ctx.quests().start(8237)?;
            } else {
                ctx.lines(args![
                    "So...",
                    "How is it going",
                    "with learning that",
                    "^871F78Over Charge^000000 skill?"
                ])?;
                ctx.next()?;
                ctx.mes("[Guarnien]")?;
                if l_chk_over.number()? > 3 && l_chk_over.number()? < 10 {
                    ctx.lines(args![
                        "Nice work!",
                        "I can tell that",
                        "you've done just",
                        "as I asked. Please,",
                        "take this little reward~"
                    ])?;
                    ctx.call(Function::GetExperience, args![126, 27])?;
                } else if l_chk_over == 10 {
                    ctx.lines(args![
                        "Whoa...",
                        "You actually",
                        "mastered Over Charge?",
                        "Most impressive! You deserve a small reward for your work!"
                    ])?;
                    ctx.call(Function::GetExperience, args![142, 33])?;
                } else {
                    ctx.lines(args![
                        "Mm? You haven't",
                        "learned Over Charge",
                        "up to Skill Level 4 yet?",
                        "There's no rush, but I won't be able to lecture on anything until you finish this little task."
                    ])?;
                    return ctx.close();
                }
                ctx.quests().erase(8236)?;
                ctx.quests().start(8237)?;
            }
            ctx.var("tu_merchant").set(Val::from(13))?;
            ctx.next()?;
            ctx.lines_as(
                "Guarnien",
                args![
                    "Next time, I'll discuss the",
                    "Push Cart skill. Be ready for this lecture, as Push Cart is one of the more important skills for a Merchant."
                ],
            )?;
            return ctx.close();
        }
        10 => {
            ctx.lines(args![
                "Let me see those",
                "Red Potions. Now,",
                "how much did you",
                "pay for these...?"
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Guarnien",
                args![
                    "Ah...!",
                    "It looks like...",
                    "You were scammed!",
                    "D-don't worry, though,",
                    "I won't tell anybody."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Guarnien",
                args!["Oh well, it happens even to the best of us. Just think of this as an invaluable learning experience."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Guarnien",
                args!["Now, why don't you go to Prontera and try to buy 10 Red Potions from an NPC for the cheapest price again?"],
            )?;
            ctx.var("tu_merchant").set(ctx.call(Function::Rand, args![6, 8])?)?;
            return ctx.close();
        }
        9 => {
            ctx.lines(args![
                "Let me see those",
                "Red Potions. Ah, it seems you researched the market and bought the cheapest ones! Great work!"
            ])?;
            ctx.var("tu_merchant").set(Val::from(11))?;
            ctx.quests().change(8234, 8235)?;
            ctx.call(Function::GetExperience, args![112, 22])?;
            ctx.next()?;
            ctx.lines_as(
                "Guarnien",
                args![
                    "Alright, next time, we'll talk about the skill that you can learn after knowing how to Discount:",
                    "^871F78Over Charge^000000."
                ],
            )?;
            return ctx.close();
        }
        6 | 7 | 8 => {
            ctx.lines(args![
                "Hmm...",
                "You'll find some NPCs",
                "selling 10 Red Potions",
                "in Prontera. Your task",
                "is to buy the potions",
                "for the cheapest price."
            ])?;
            ctx.next()?;
            ctx.lines_as("Guarnien", args!["First, you'll probably want to research the market price for Red Potions. And always, be careful of scammers and cheats!"])?;
            return ctx.close();
        }
        5 => {
            ctx.lines(args![
                "Alright...",
                "I've given it some thought and I've come up with a little challenge for you. Bring me... 10 Red Potions!"
            ])?;
            ctx.next()?;
            ctx.lines_as("Guarnien", args!["It's not too hard, but the stipulation I'm adding is that you've got to find the merchant selling Red Potions for the cheapest price."])?;
            ctx.next()?;
            ctx.lines_as("Guarnien", args!["So first, you'll need to research the market price for Red Potions. Also, be careful of scammers! If the price is too good to be true, it usually is."])?;
            ctx.next()?;
            ctx.lines_as(
                "Guarnien",
                args!["Ha ha ha, let me make your goal a bit less confusing. You will need to buy 10 Red Potions from an NPC in Prontera."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Guarnien",
                args![
                    "Prontera's a bit far from here, isn't it? I'll just use this Kafra Warp-- Mahnsoo has a bunch for",
                    "some reason --to send you there right now. Get ready...!"
                ],
            )?;
            ctx.next()?;
            ctx.var("tu_merchant").set(ctx.call(Function::Rand, args![6, 8])?)?;
            ctx.quests().change(8233, 8234)?;
            ctx.warp("prontera", 155, 46)?;
            return ctx.end();
        }
        3 | 4 => {
            let l_chk_disc = ctx.call(Function::GetSkillLv, args!["MC_DISCOUNT"])?;
            if ctx.var("tu_merchant").get()? == 3 {
                ctx.lines(args![
                    "Ah, you're back!",
                    "Now, I was going",
                    "to tell you about",
                    "the Discount skill, right?"
                ])?;
                ctx.next()?;
                ctx.lines_as("Guarnien", args!["The ^871F78Discount^000000 skill allows you to buy items at lower prices from NPC shops. It is a ^871F78Passive Skill^000000 which is always in effect and does not consume SP."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Guarnien",
                    args![
                        "The Discount skill is",
                        "mastered at ^871F78Level 10^000000. The higher the skill level, the less you have to pay for items at NPC shops."
                    ],
                )?;
                ctx.next()?;
                ctx.mes("[Guarnien]")?;
                if l_chk_disc.number()? > 3 && l_chk_disc.number()? < 10 {
                    ctx.lines(args![
                        "Oooh, good work.",
                        "I see that you've",
                        "raised your Discount",
                        "to Level 4 already.",
                        "Here, take this",
                        "small reward~"
                    ])?;
                    ctx.call(Function::GetExperience, args![83, 11])?;
                } else if l_chk_disc == 10 {
                    ctx.lines(args![
                        "Incredible!",
                        "You've actually mastered the Discount skill! You're such a great student. You deserve a bit of a reward!"
                    ])?;
                    ctx.call(Function::GetExperience, args![97, 18])?;
                } else {
                    ctx.mes("Alright! Now, in order to become closer towards becoming a true Merchant, I want you to learn the Discount skill up to Level 4.")?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Guarnien",
                        args![
                            "It'd be nicer if you could",
                            "master it, but you might be more interested in devoting your time to other skills."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Guarnien", args!["Still, you can't ignore the fact that bargaining is an essential skill for a Merchant! Come back to me when you're ready~"])?;
                    ctx.var("tu_merchant").set(Val::from(4))?;
                    ctx.quests().erase(8231)?;
                    ctx.quests().start(8232)?;
                    return ctx.close();
                }
                ctx.quests().erase(8231)?;
                ctx.quests().start(8233)?;
            } else {
                ctx.mes("So, how is it going with learning the Discount skill? Like I always say, if you can't make a bargain, you can't be a Merchant!")?;
                ctx.next()?;
                ctx.mes("[Guarnien]")?;
                if l_chk_disc.number()? > 3 && l_chk_disc.number()? < 10 {
                    ctx.lines(args![
                        "Ah, you've learned",
                        "how to use the Discount skill",
                        "well enough. Good, here's a little reward for your hard work~"
                    ])?;
                    ctx.call(Function::GetExperience, args![83, 11])?;
                } else if l_chk_disc == 10 {
                    ctx.lines(args![
                        "Incredible!",
                        "You've actually mastered the Discount skill! You're such a great student. You deserve a bit of a reward!"
                    ])?;
                    ctx.call(Function::GetExperience, args![97, 18])?;
                } else {
                    ctx.lines(args![
                        "Not yet, huh?",
                        "Well, you should be able to reach that goal soon. After all, it's one of the basics of being a Merchant!"
                    ])?;
                    return ctx.close();
                }
                ctx.quests().erase(8232)?;
                ctx.quests().start(8233)?;
            }
            ctx.var("tu_merchant").set(Val::from(5))?;
            ctx.next()?;
            ctx.lines_as("Guarnien", args!["Alright, my next lesson will hopefully offer you a bit more of a challenge. Come back when you think you're ready, alright?"])?;
            return ctx.close();
        }
        2 => {
            ctx.mes("Ah, you've come back to learn more. Let's see, what was I going to tell you about... Right, Mammonite!")?;
            ctx.next()?;
            ctx.lines_as(
                "Guarnien",
                args!["^871F78Mammonite^000000 is a skill that lets you greatly damage targets with Zeny. Money really is power!"],
            )?;
            ctx.next()?;
            ctx.lines_as("Guarnien", args!["The upside is that it inflicts great damage and only uses a little SP. However, the downside is that it will also consume your Zeny!"])?;
            ctx.next()?;
            ctx.lines_as("Guarnien", args!["So if you use this skill too often, you might end up bankrupt! So be really careful. Also, as the Skill Level for Mammonite increases, so does the attack strength and Zeny consumption."])?;
            ctx.next()?;
            ctx.mes("[Guarnien]")?;
            let l_chk_mam = ctx.call(Function::GetSkillLv, args!["MC_MAMMONITE"])?;
            if l_chk_mam.number()? > 3 && l_chk_mam.number()? < 10 {
                ctx.mes(
                    "Ah, I see that you've already tried this skill. How much money have you wasted using Mammonite? Not too much, I hope.",
                )?;
                ctx.next()?;
                ctx.lines_as("Guarnien", args!["Well, since you're still green, I feel awfully sorry if you've wasted Zeny. Why don't you take this as compensation?"])?;
                ctx.var("tu_merchant").set(Val::from(3))?;
                ctx.quests().erase(8230)?;
                ctx.quests().start(8231)?;
                ctx.call(Function::GetExperience, args![70, 12])?;
                ctx.next()?;
                ctx.lines_as("Guarnien", args!["Okay. Now I think we're ready for me to discuss the ^871F78Discount^000000 skill. Come back when you'd like me to tell you more about it, okay?"])?;
                return ctx.close();
            } else {
                ctx.mes("Okay. Now I think we're ready for me to discuss the ^871F78Discount^000000 skill. Come back when you'd like me to tell you more about it, okay?")?;
                ctx.var("tu_merchant").set(Val::from(3))?;
                ctx.quests().erase(8230)?;
                ctx.quests().start(8231)?;
                return ctx.close();
            }
        }
        1 => {
            ctx.lines(args![
                "So...",
                "Did you learn the",
                "Increase Weight Limit",
                "skill up to Level 4",
                "like I asked?"
            ])?;
            ctx.next()?;
            ctx.mes("[Guarnien]")?;
            let l_chk_soji = ctx.call(Function::GetSkillLv, args!["MC_INCCARRY"])?;
            if l_chk_soji.number()? > 3 && l_chk_soji.number()? < 10 {
                ctx.mes("Ah. I can tell that you have. Not bad! By now you should be able to learn the ^871F78Discount^000000 skill, but I'll teach you about that later.")?;
                ctx.next()?;
                ctx.lines_as(
                    "Guarnien",
                    args!["There are a few other basic skills which you can learn right away that I'll talk about first. Let's not rush~"],
                )?;
                ctx.next()?;
                ctx.lines_as("Guarnien", args!["First, there's the ^871F78Item Appraisal^000000 skill. This ability, once you've learned it, allows you to ^871F78identify unknown items^000000."])?;
                ctx.next()?;
                ctx.lines_as("Guarnien", args!["This skill works exactly like a ^871F78Magnifier^000000, so you won't have to spend any Zeny on them. Also, this skill is mastered at ^871F78Level 1^000000."])?;
                ctx.next()?;
                if ctx.call(Function::GetSkillLv, args!["MC_IDENTIFY"])?.is_true() {
                    ctx.lines_as("Guarnien", args!["I see in your eyes that you've already learned to appraise items. Haha, although having that knowledge is reward in itself, let me give you something extra!"])?;
                    ctx.var("tu_merchant").set(Val::from(2))?;
                    ctx.quests().erase(8229)?;
                    ctx.quests().start(8230)?;
                    ctx.call(Function::GetExperience, args![58, 11])?;
                    ctx.items().give(1351, 1)?;
                    ctx.next()?;
                    ctx.lines_as("Guarnien", args!["Alright, next time", "I'll talk about the Mammonite skill. For now, let me take a little bit of a break from all of this arduous lecture. ^666666*Whew!*^000000"])?;
                    return ctx.close();
                }
            } else {
                ctx.lines(args![
                    "No? That's fine.",
                    "But I can't teach you",
                    "much more if you can't grasp these simple basics, so hurry and learn those skills, okay?"
                ])?;
                return ctx.close();
            }
            ctx.lines(args!["Alright, next time", "I'll talk about the Mammonite skill. For now, let me take a little bit of a break from all of this arduous lecture. ^666666*Whew!*^000000"])?;
            ctx.var("tu_merchant").set(Val::from(2))?;
            ctx.quests().erase(8229)?;
            ctx.quests().start(8230)?;
            return ctx.close();
        }
        _ => {}
    }
    ctx.mes("You've just started out as a Merchant, haven't you? How would you like to learn the fundamentals of business from an experienced colleague?")?;
    ctx.next()?;
    if ctx.menu(&["Sure!", "I'll make it on my own!"])? == 0 {
        if ctx.call(Function::GetSkillLv, args!["MC_INCCARRY"])?.number()? < 4 {
            ctx.var("tu_merchant").set(Val::from(1))?;
            ctx.quests().start(8229)?;
            ctx.lines_as(
                "Guarnien",
                args![
                    shared::other_global_functions::f_sexmes(ctx, args!["Atta girl~!", "Atta boy~!"])?,
                    "But first things first!",
                    "You better learn the",
                    "^871F78Increase Weight Limit^000000 skill!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Guarnien", args!["You can't really be a Merchant if you can't even carry around goods to sell. As you increase the level of this skill, you can carry around more items."])?;
            ctx.next()?;
            ctx.lines_as(
                "Guarnien",
                args![
                    "Why don't you upgrade your",
                    "Increase Weight Limit skill up to Level 4? After that, we can talk more about becoming a true Merchant."
                ],
            )?;
            return ctx.close();
        } else {
            ctx.lines_as("Guarnien", args!["Great...!", "Ah, and I see that you've learned the ^871F78Increase Weight Limit^000000 skill to at least Level 4. You're a real go-getter, aren't you?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Guarnien",
                args![
                    "Since your Increase Weight",
                    "Limit skill level is high enough, you must be able to see a new skill in your Skill Window: ^871F78Discount^000000!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Guarnien", args!["But we'll discuss the Discount skill later. There are a few other basic skills which you can learn right away that I'll talk about first. Let's not rush~"])?;
            ctx.next()?;
            ctx.lines_as("Guarnien", args!["First, there's the ^871F78Item Appraisal^000000 skill. This ability, once you've learned it, allows you to ^871F78identify unknown items^000000."])?;
            ctx.next()?;
            ctx.lines_as("Guarnien", args!["This skill works exactly like a ^871F78Magnifier^000000, so you won't have to spend any Zeny on them. Also, this skill is mastered at ^871F78Level 1^000000."])?;
            ctx.next()?;
            ctx.mes("[Guarnien]")?;
            if ctx.call(Function::GetSkillLv, args!["MC_IDENTIFY"])?.is_true() {
                ctx.mes("I see in your eyes that you've already learned to appraise items. Haha, although having that knowledge is reward in itself, let me give you something extra!")?;
                ctx.var("tu_merchant").set(Val::from(2))?;
                ctx.quests().erase(8229)?;
                ctx.quests().start(8230)?;
                ctx.call(Function::GetExperience, args![58, 11])?;
                ctx.items().give(1351, 1)?;
                ctx.next()?;
                ctx.lines_as("Guarnien", args!["Alright, next time", "I'll talk about the Mammonite skill. For now, let me take a little bit of a break from all of this arduous lecture. ^666666*Whew!*^000000"])?;
                return ctx.close();
            }
            ctx.lines(args![
                "Alright, next time",
                "I'll talk about the Mammonite skill. For now, let me take a little bit of a break from all of this arduous study."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Guarnien",
                args![
                    "Oh...",
                    "And I'll tell you",
                    "more about the Discount",
                    "skill later. I always manage",
                    "to forget about that!"
                ],
            )?;
            ctx.var("tu_merchant").set(Val::from(2))?;
            ctx.quests().erase(8229)?;
            ctx.quests().start(8230)?;
            return ctx.close();
        }
    } else {
        ctx.lines_as(
            "Guarnien",
            args![
                "On your own?",
                "Hahaha, that's the spirit!",
                "But still, the advice I'm giving is free. It couldn't hurt to give what I say a little bit of thought."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Guarnien", args!["Well, if you ever change your mind, stop by and talk to me anytime. And as a Merchant, you should know that free advice is the best!"])?;
        return ctx.close();
    }
}

#[derive(Clone, Copy, Debug)]
enum SagleStep {
    Start,
    LPotions,
}

fn sagle_run(ctx: &Ctx, step: SagleStep, args: Vec<Val>) -> Script {
    'machine: loop {
        match step {
            SagleStep::Start => {
                ctx.mes("[Sagle]")?;
                if ctx.var("tu_merchant").get()? == 6 {
                    sagle_run(ctx, SagleStep::LPotions, args![390, 9])?;
                } else if ctx.var("tu_merchant").get()? == 7 {
                    sagle_run(ctx, SagleStep::LPotions, args![410, 10])?;
                } else if ctx.var("tu_merchant").get()? == 8 {
                    sagle_run(ctx, SagleStep::LPotions, args![420, 10])?;
                }
                ctx.lines(args!["It's on the tip", "of my tongue, but", "I can't quite remember..."])?;
                ctx.next()?;
                ctx.lines_as("Sagle", args!["You know, the name of the company that has all those beautiful ladies working for it. Ka... Ka-something. Anyway, I hear they're concerned about their growing competition. "])?;
                ctx.next()?;
                ctx.lines_as(
                    "Sagle",
                    args![
                        "It's none of my business, but it's good to know what's going on in",
                        "the world, particularly if it'll affect the markets."
                    ],
                )?;
                return ctx.close();
            }
            SagleStep::LPotions => {
                let l_cost = runtime::arg(&args, 0, Val::from(0));
                ctx.lines(args![
                    "Hello, hello~",
                    "Why don't you buy",
                    "some Red Potions?",
                    "They're essential for travel, and these Red Potions are the best for newer adventurers~"
                ])?;
                ctx.next()?;
                ctx.lines_as(
                    "Sagle",
                    args![
                        ((Val::from("I will sell you 10 Red Potions for ^871F78") + l_cost.clone())
                            + Val::from(" zeny^000000. You'd better get them now while they're still here!"))
                    ],
                )?;
                ctx.next()?;
                match ctx.menu(&["Buy", "Cancel"])? {
                    0 => {
                        ctx.lines_as(
                            "Sagle",
                            args![
                                "Excellent!",
                                "You certainly have an eye for bargains. Here you go, 10 Red Potions fresh from the... Potioner..."
                            ],
                        )?;
                        if runtime::op(&ctx.var("Zeny").get()?, "<", &l_cost)?.is_true() {
                            ctx.next()?;
                            ctx.lines_as(
                                "Sagle",
                                args![
                                    "Whoa, hold on!",
                                    "You can't afford",
                                    "these potions...",
                                    "Please come back",
                                    "when you have enough",
                                    "Zeny, alright?"
                                ],
                            )?;
                            return ctx.close();
                        }
                        if ctx.var("MaxWeight").get()?.number()? - ctx.var("Weight").get()?.number()? < 71 {
                            ctx.next()?;
                            ctx.lines_as("Sagle", args!["Whoa, hold on!", "There's no way you carry all of this! Why don't you put some of your stuff in Kafra Storage before coming back?"])?;
                        } else {
                            ctx.var("Zeny").set(ctx.var("Zeny").get()?.try_sub(l_cost.clone())?)?;
                            ctx.var("tu_merchant").set(runtime::arg(&args, 1, Val::from(0)))?;
                            ctx.items().give(501, 10)?;
                        }
                        return ctx.close();
                    }
                    1 => {
                        ctx.lines_as(
                            "Sagle",
                            args!["Alright...", "But don't", "complain if", "these potions", "are sold out!"],
                        )?;
                        return ctx.close();
                    }
                    _ => {}
                }
                return Ok(());
            }
        }
    }
}

pub fn sagle(ctx: &Ctx) -> Script {
    sagle_run(ctx, SagleStep::Start, Vec::new())
}

#[derive(Clone, Copy, Debug)]
enum KellionStep {
    Start,
    LPotions,
}

fn kellion_run(ctx: &Ctx, step: KellionStep, args: Vec<Val>) -> Script {
    'machine: loop {
        match step {
            KellionStep::Start => {
                ctx.mes("[Kellion]")?;
                if ctx.var("tu_merchant").get()? == 6 {
                    kellion_run(ctx, KellionStep::LPotions, args![400, 10])?;
                } else if ctx.var("tu_merchant").get()? == 7 {
                    kellion_run(ctx, KellionStep::LPotions, args![390, 9])?;
                } else if ctx.var("tu_merchant").get()? == 8 {
                    kellion_run(ctx, KellionStep::LPotions, args![340, 10])?;
                }
                ctx.lines(args!["Recently, I hear that something", "has happened to the royal family."])?;
                ctx.next()?;
                ctx.lines_as("Kellion", args!["It's probably not even a reliable rumor, but I've heard that members of the royal family have been dying from some unknown cause."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Kellion",
                    args![
                        "Huh.",
                        "Now that's some morbid news!",
                        "Well, hopefully, there's no truth to it whatsoever. None at all..."
                    ],
                )?;
                return ctx.close();
            }
            KellionStep::LPotions => {
                let l_cost = runtime::arg(&args, 0, Val::from(0));
                ctx.lines(args![
                    "I'm selling sets of",
                    ((Val::from("10 Red Potions for ") + l_cost.clone()) + Val::from(" Zeny.")),
                    "Would you like some?"
                ])?;
                ctx.next()?;
                match ctx.menu(&["Buy", "Cancel"])? {
                    0 => {
                        ctx.lines_as("Kellion", args!["Good choice~", "Thank you for", "using my shop."])?;
                        if runtime::op(&ctx.var("Zeny").get()?, "<", &l_cost)?.is_true() {
                            ctx.next()?;
                            ctx.lines_as(
                                "Kellion",
                                args![
                                    "Ooops!",
                                    "I'm sorry, but you don't have enough Zeny. I've got to stay in business somehow, you know?"
                                ],
                            )?;
                            return ctx.close();
                        }
                        if ctx.var("MaxWeight").get()?.number()? - ctx.var("Weight").get()?.number()? < 71 {
                            ctx.next()?;
                            ctx.lines_as(
                                "Kellion",
                                args![
                                    "...Huh?",
                                    "You better put some of your stuff in Kafra Storage, you can't carry much else!"
                                ],
                            )?;
                        } else {
                            ctx.var("Zeny").set(ctx.var("Zeny").get()?.try_sub(l_cost.clone())?)?;
                            ctx.var("tu_merchant").set(runtime::arg(&args, 1, Val::from(0)))?;
                            ctx.items().give(501, 10)?;
                        }
                        return ctx.close();
                    }
                    1 => {
                        ctx.lines_as(
                            "Kellion",
                            args!["Well, I'm sorry", "to hear that. But", "I guess I'll see", "you later."],
                        )?;
                        return ctx.close();
                    }
                    _ => {}
                }
                return Ok(());
            }
        }
    }
}

pub fn kellion(ctx: &Ctx) -> Script {
    kellion_run(ctx, KellionStep::Start, Vec::new())
}

#[derive(Clone, Copy, Debug)]
enum AigieStep {
    Start,
    LPotions,
}

fn aigie_run(ctx: &Ctx, step: AigieStep, args: Vec<Val>) -> Script {
    'machine: loop {
        match step {
            AigieStep::Start => {
                ctx.mes("[Aigie]")?;
                if ctx.var("tu_merchant").get()? == 6 {
                    aigie_run(ctx, AigieStep::LPotions, args![340, 10])?;
                } else if ctx.var("tu_merchant").get()? == 7 {
                    aigie_run(ctx, AigieStep::LPotions, args![420, 10])?;
                } else if ctx.var("tu_merchant").get()? == 8 {
                    aigie_run(ctx, AigieStep::LPotions, args![390, 9])?;
                }
                ctx.mes("It's true that money isn't everything. I'm sure other things are important to have in order to be happy.")?;
                ctx.next()?;
                ctx.lines_as("Aigie", args!["However, money can be much more dependable than some people I know. You can rely on money more than anything else in this world."])?;
                return ctx.close();
            }
            AigieStep::LPotions => {
                let l_cost = runtime::arg(&args, 0, Val::from(0));
                ctx.lines(args![
                    "Welcome to my shop.",
                    "I'm just a young girl who's a victim of circumstance, forced to sell potions on the street for dirt cheap prices."
                ])?;
                ctx.next()?;
                ctx.lines_as(
                    "Aigie",
                    args![
                        "Would you like",
                        "to buy 10 Red Potions",
                        ((Val::from("for ") + l_cost.clone()) + Val::from(" Zeny, kind adventurer?"))
                    ],
                )?;
                ctx.next()?;
                match ctx.menu(&["Buy", "Cancel"])? {
                    0 => {
                        ctx.lines_as(
                            "Aigie",
                            args!["Thank you so much.", "Now I can finally", "afford food again..."],
                        )?;
                        if runtime::op(&ctx.var("Zeny").get()?, "<", &l_cost)?.is_true() {
                            ctx.next()?;
                            ctx.lines_as(
                                "Aigie",
                                args![
                                    "Wait, wait!",
                                    "I am sorry, but you don't have enough money. Would you please get some more Zeny before coming back?"
                                ],
                            )?;
                            return ctx.close();
                        }
                        if ctx.var("MaxWeight").get()?.number()? - ctx.var("Weight").get()?.number()? < 71 {
                            ctx.next()?;
                            ctx.lines_as(
                                "Aigie",
                                args![
                                    "Wait, wait!",
                                    "You can't possibly carry any more items. Why don't you put your things in Kafra Storage first?"
                                ],
                            )?;
                        } else {
                            ctx.var("Zeny").set(ctx.var("Zeny").get()?.try_sub(l_cost.clone())?)?;
                            ctx.var("tu_merchant").set(runtime::arg(&args, 1, Val::from(0)))?;
                            ctx.items().give(501, 10)?;
                        }
                        return ctx.close();
                    }
                    1 => {
                        ctx.lines_as(
                            "Aigie",
                            args![
                                "I understand.",
                                "But I hope you",
                                "see that I'm offering",
                                "you a really good price.",
                                "^666666*Sniff Sniff*^000000"
                            ],
                        )?;
                        return ctx.close();
                    }
                    _ => {}
                }
                return Ok(());
            }
        }
    }
}

pub fn aigie(ctx: &Ctx) -> Script {
    aigie_run(ctx, AigieStep::Start, Vec::new())
}

#[derive(Clone, Copy, Debug)]
enum JayonStep {
    Start,
    LPotions,
}

fn jayon_run(ctx: &Ctx, step: JayonStep, args: Vec<Val>) -> Script {
    'machine: loop {
        match step {
            JayonStep::Start => {
                ctx.mes("[Jayon]")?;
                if ctx.var("tu_merchant").get()? == 6 {
                    jayon_run(ctx, JayonStep::LPotions, args![410, 10])?;
                } else if ctx.var("tu_merchant").get()? == 7 {
                    jayon_run(ctx, JayonStep::LPotions, args![400, 10])?;
                } else if ctx.var("tu_merchant").get()? == 8 {
                    jayon_run(ctx, JayonStep::LPotions, args![410, 10])?;
                }
                ctx.lines(args![
                    "The Schwarzwald Republic...",
                    "It's a really interesting country. There's Juno and a few other places worth looking around. "
                ])?;
                ctx.next()?;
                ctx.lines_as(
                    "Jayon",
                    args![
                        "They may not be the best places",
                        "for sight seeing, but I'm sure they'll offer something of interest to you adventurers."
                    ],
                )?;
                return ctx.close();
            }
            JayonStep::LPotions => {
                let l_cost = runtime::arg(&args, 0, Val::from(0));
                ctx.lines(args![((Val::from("If you've checked out the market for Red Potions, you know that my price is the best. I'm offering 10 Red Potions for only ") + l_cost.clone()) + Val::from(" Zeny!"))])?;
                ctx.next()?;
                match ctx.menu(&["Buy", "Cancel"])? {
                    0 => {
                        ctx.lines_as(
                            "Jayon",
                            args![
                                "Heh heh...!",
                                "It looks like",
                                "you know your",
                                "stuff. You're a",
                                "Merchant yourself,",
                                "aren't you?"
                            ],
                        )?;
                        if runtime::op(&ctx.var("Zeny").get()?, "<", &l_cost)?.is_true() {
                            ctx.next()?;
                            ctx.lines_as(
                                "Jayon",
                                args![
                                    "But sorry buddy.",
                                    "Rules are rules.",
                                    "You gotta meet my price",
                                    "if you want these potions."
                                ],
                            )?;
                            return ctx.close();
                        }
                        if ctx.var("MaxWeight").get()?.number()? - ctx.var("Weight").get()?.number()? < 71 {
                            ctx.next()?;
                            ctx.lines_as(
                                "Jayon",
                                args![
                                    "But sorry buddy. I can't let you carry more than you can handle.",
                                    "You ought to free up some of your inventory space when you get the chance."
                                ],
                            )?;
                        } else {
                            ctx.var("Zeny").set(ctx.var("Zeny").get()?.try_sub(l_cost.clone())?)?;
                            ctx.var("tu_merchant").set(runtime::arg(&args, 1, Val::from(0)))?;
                            ctx.items().give(501, 10)?;
                        }
                        return ctx.close();
                    }
                    1 => {
                        ctx.lines_as(
                            "Jayon",
                            args![
                                "Just looking",
                                "around, eh?",
                                "I understand.",
                                "But you better take",
                                "advantage of a real",
                                "deal when you see one!"
                            ],
                        )?;
                        return ctx.close();
                    }
                    _ => {}
                }
                return Ok(());
            }
        }
    }
}

pub fn jayon(ctx: &Ctx) -> Script {
    jayon_run(ctx, JayonStep::Start, Vec::new())
}

#[derive(Clone, Copy, Debug)]
enum MaosStep {
    Start,
    LPotions,
}

fn maos_run(ctx: &Ctx, step: MaosStep, args: Vec<Val>) -> Script {
    'machine: loop {
        match step {
            MaosStep::Start => {
                ctx.mes("[Maos]")?;
                if ctx.var("tu_merchant").get()? == 6 {
                    maos_run(ctx, MaosStep::LPotions, args![400, 10])?;
                } else if ctx.var("tu_merchant").get()? == 7 {
                    maos_run(ctx, MaosStep::LPotions, args![340, 10])?;
                } else if ctx.var("tu_merchant").get()? == 8 {
                    maos_run(ctx, MaosStep::LPotions, args![420, 10])?;
                }
                ctx.mes("I see too many people struggling to scrape every penny and put it all into their savings...")?;
                ctx.next()?;
                ctx.lines_as("Maos", args!["Now, don't get me wrong. It's a good idea to invest in the future. But money exists to be spent! Enjoy every moment, that's what I say~"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Maos",
                    args![
                        "Besides...",
                        "You can't take",
                        "your money with you",
                        "when your time's up,",
                        "you know?"
                    ],
                )?;
                return ctx.close();
            }
            MaosStep::LPotions => {
                let l_cost = runtime::arg(&args, 0, Val::from(0));
                ctx.lines(args![
                    "Don't say anything.",
                    "I know what you want.",
                    "10 Red Potions, right?",
                    "I'll sell them to you",
                    ((Val::from("for the low price of ") + l_cost.clone()) + Val::from(" Zeny~"))
                ])?;
                ctx.next()?;
                match ctx.menu(&["Buy", "Cancel"])? {
                    0 => {
                        ctx.lines_as(
                            "Maos",
                            args![
                                "Excellent choice!",
                                "Good products at affordable prices! That's my ethic as a merchant."
                            ],
                        )?;
                        if runtime::op(&ctx.var("Zeny").get()?, "<", &l_cost)?.is_true() {
                            ctx.next()?;
                            ctx.lines_as("Maos", args!["Ooh, but you don't even have enough Zeny to purchase these potions. I'm sorry, but I can't sell these just to lose money, you know?"])?;
                            return ctx.close();
                        }
                        if ctx.var("MaxWeight").get()?.number()? - ctx.var("Weight").get()?.number()? < 71 {
                            ctx.next()?;
                            ctx.lines_as(
                                "Maos",
                                args!["Ooh, but it doesn't look like you've got enough inventory space to carry any more items."],
                            )?;
                        } else {
                            ctx.var("Zeny").set(ctx.var("Zeny").get()?.try_sub(l_cost.clone())?)?;
                            ctx.var("tu_merchant").set(runtime::arg(&args, 1, Val::from(0)))?;
                            ctx.items().give(501, 10)?;
                        }
                        return ctx.close();
                    }
                    1 => {
                        ctx.lines_as(
                            "Maos",
                            args!["Well, I can't force you to buy these, but I'm telling you that you're passing up a real bargain!"],
                        )?;
                        return ctx.close();
                    }
                    _ => {}
                }
                return Ok(());
            }
        }
    }
}

pub fn maos(ctx: &Ctx) -> Script {
    maos_run(ctx, MaosStep::Start, Vec::new())
}

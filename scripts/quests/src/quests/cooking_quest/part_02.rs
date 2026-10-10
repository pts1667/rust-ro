use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum MadeleineChuCookStep {
    Start,
    SSellSets,
}

fn madeleine_chu_cook_run(ctx: &Ctx, mut step: MadeleineChuCookStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_item_cost = Val::from(0);
    let mut l_item_id = Val::from(0);
    let mut l_item_weight = Val::from(0);
    let mut l_sell = Val::from(0);
    let mut l_talk_j = Val::from(0);
    let mut l_total_cost = Val::from(0);
    let mut l_total_weight = Val::from(0);
    'machine: loop {
        match step {
            MadeleineChuCookStep::Start => {
                if (ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 2000 {
                    ctx.lines_as(
                        "Madeleine Chu",
                        args![
                            "I'm sorry, but right now",
                            "you're carrying too many",
                            "items. You should put your",
                            "extra things in Kafra Storage,",
                            "and then talk to me again, okay? "
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ctx.var("cooking_q").get()? == 0 {
                    ctx.lines_as(
                        "Madeleine Chu",
                        args![
                            "Oh, hello~",
                            "I'm Madeleine Chu,",
                            "chef apprentice to",
                            "Sir Charles. May I help",
                            "you with anything today?"
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(
                        ctx,
                        &[Val::from(
                            "What do you do as a chef?:Which foods can you make?:I want to learn cooking too!",
                        )],
                    )? {
                        1 => {
                            ctx.lines_as(
                                "Madeleine Chu",
                                args![
                                    "What do I do as a chef?",
                                    "Well, I'm just an apprentice now, so I'm still learning how to cook.",
                                    "But someday, I want to become",
                                    "a great chef and have everybody",
                                    "recognize my culinary talents~"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Madeleine Chu",
                                args![
                                    "There's a lot of grueling",
                                    "work that goes into cooking,",
                                    "as well as a lot of finesse.",
                                    "I have to control fire better",
                                    "tham a firefighter and craft",
                                    "my dishes like an artist."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Madeleine Chu",
                                args![
                                    "Well, I'm exaggerating",
                                    "a little bit, but cooking",
                                    "at a certain level is much",
                                    "more difficult than it appears."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as(
                                "Madeleine Chu",
                                args![
                                    "Well, I only know the basic",
                                    "recipes for now. Sir Charles",
                                    "says that even the best chef",
                                    "knows how to bring out the",
                                    "flavors of even common foods."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Madeleine Chu",
                                args![
                                    "He says that I need to develop",
                                    "my culinary skills until I can",
                                    "learn more advanced recipes.",
                                    "Someday, I'll advance and then",
                                    "I'll know enough to create my",
                                    "own unique, delicious dishes!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Madeleine Chu",
                                args![
                                    "Sir Charles may be harsh to",
                                    "his students, but his skills",
                                    "are unequaled. I tried one of",
                                    "his desserts once, and it was",
                                    "the most heavenly experience.",
                                    "I swear I saw winged hearts~!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Madeleine Chu",
                                args![
                                    "Sadly, I'm a still long way",
                                    "from learning how to make ",
                                    "his specialty, Handmade",
                                    "Chocolates. Before that, I need",
                                    "to master these strange recipes",
                                    "that he keeps teaching me..."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        3 => {
                            ctx.lines_as(
                                "Madeleine Chu",
                                args![
                                    "If you want to learn cooking,",
                                    "why don't you ask Sir Charles?",
                                    "He's fairly harsh to his students, but he does it out of tough love.",
                                    "He demands nothing less than",
                                    "absolute perfection, you know."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Madeleine Chu",
                                args![
                                    "If you manage to get",
                                    "Sir Charles to teach you",
                                    "a recipe, you should practice",
                                    "it over and over again to hone",
                                    "your skills. Then, you'll find",
                                    "yourself improving at cooking."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Madeleine Chu",
                                args![
                                    "Now, Sir Charles will only",
                                    "teach students that are truly",
                                    "committed to cooking. You",
                                    "might want to show your",
                                    "dedication with the proper",
                                    "attire... like a Chef Hat."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Madeleine Chu",
                                args![
                                    "But yes, Sir Charles",
                                    "has very little patience for",
                                    "beginners, meaning that",
                                    "you'll have to be patient",
                                    "with his teaching methods..."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else {
                    if (ctx.var("cooking_q").get()?.number()? > 0 && ctx.var("cooking_q").get()?.number()? < 7) {
                        ctx.lines_as(
                            "Madeleine Chu",
                            args![
                                "Hello, is there any",
                                "way I can help you today?",
                                "Oh, if you're studying cooking",
                                "under Sir Charles, I can remind",
                                "you of the ingredients you need",
                                "if you've forgotten them~"
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(
                            ctx,
                            &[Val::from(
                                "Fried Grasshopper Legs:Grape Juice Herbal Tea:Honey Grape Juice:Frog Egg and Squid Ink Soup:Steamed Crab Nippers:Fried Monkey Tails",
                            )],
                        )? {
                            1 => {
                                ctx.lines_as(
                                    "Madeleine Chu",
                                    args![
                                        "Oh! You'll need",
                                        "^4D4DFF5 Grasshopper Legs^000000,",
                                        "^4D4DFF1 Cooking Oil^000000, and",
                                        "^4D4DFF1 Old Frying Pan^000000 to make",
                                        "fried Grasshopper Legs."
                                    ],
                                )?;
                            }
                            2 => {
                                ctx.lines_as(
                                    "Madeleine Chu",
                                    args![
                                        "Oh! You'll need",
                                        "^4D4DFF3 Grapes^000000, and",
                                        "^4D4DFF2 Red Potions^000000 for",
                                        "Grape Juice Herbal Tea."
                                    ],
                                )?;
                            }
                            3 => {
                                ctx.lines_as(
                                    "Madeleine Chu",
                                    args![
                                        "Oh! You'll need",
                                        "^4D4DFF1 Honey^000000,",
                                        "^4D4DFF2 Grapes^000000, and",
                                        "^4D4DFF1 Red Potion^000000."
                                    ],
                                )?;
                            }
                            4 => {
                                ctx.lines_as(
                                    "Madeleine Chu",
                                    args![
                                        "Oh! You'll need",
                                        "^4D4DFF1 Bag of Grain^000000,",
                                        "^4D4DFF10 Spawns^000000, and",
                                        "^4D4DFF1 Squid Ink^000000 for Frog",
                                        "Egg and Squid Ink soup."
                                    ],
                                )?;
                            }
                            5 => {
                                ctx.lines_as(
                                    "Madeleine Chu",
                                    args![
                                        "Oh! You'll need",
                                        "^4D4DFF10 Green Herbs^000000,",
                                        "^4D4DFF10 Nippers^000000, and",
                                        "^4D4DFF1 Yellow Potion^000000 for",
                                        "Steamed Crab Nippers."
                                    ],
                                )?;
                            }
                            6 => {
                                ctx.lines_as(
                                    "Madeleine Chu",
                                    args![
                                        "Oh! You'll need",
                                        "^4D4DFF1 Frying Pan^000000,",
                                        "^4D4DFF5 Yoyo Tails^000000, and",
                                        "^4D4DFF1 Cooking Oil^000000 for",
                                        "Fried Monkey Tails."
                                    ],
                                )?;
                            }
                            _ => {}
                        }
                        ctx.next()?;
                        ctx.lines_as(
                            "Madeleine Chu",
                            args![
                                "I know that Sir Charles",
                                "is stubborn and won't tell",
                                "you the ingredients again",
                                "if you forget. Anyway, I hope",
                                "you collect them and complete",
                                "the recipe as soon as you can~"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if ctx.var("cooking_q").get()? == 7 {
                            ctx.lines_as(
                                "Madeleine Chu",
                                args![
                                    "Sir Charles taught you",
                                    "a recipe? That's great!",
                                    "I hope you remember that the",
                                    "quality of your dishes mostly",
                                    "relies on your skills, so always remember to keep practicing."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Madeleine Chu",
                                args![
                                    "Ah, you know what might",
                                    "help you improve your",
                                    "culinary skills? Why don't",
                                    "you borrow this cookbook",
                                    "and try some of its recipes?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Madeleine Chu",
                                args![
                                    "Before you cook, make sure",
                                    "that you have enough of the",
                                    "ingredients. Oh, and keep the",
                                    "cookbook nearby while you are",
                                    "cooking. You'll probably need",
                                    "to refer to it pretty often..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Madeleine Chu",
                                args![
                                    "You might fail to make",
                                    "edible food during your",
                                    "first attempts, but you'll",
                                    "improve as you practice. ",
                                    "Please take this cookbook",
                                    "with the basic Level 1 recipes."
                                ],
                            )?;
                            ctx.var("cooking_q").set(Val::from(8))?;
                            ctx.call(Function::GetItem, vec![Val::from(7472), Val::from(1)])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Madeleine Chu",
                                args![
                                    "Once you learn all the recipes,",
                                    "feel free to come back to me for more, okay? Also, you'll need",
                                    "these cooking kits to practice.",
                                    "You can have these for free, and you can buy more from me later~"
                                ],
                            )?;
                            ctx.call(Function::GetItem, vec![Val::from(12125), Val::from(10)])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.var("cooking_q").get()? == 8 {
                            ctx.lines_as(
                                "Madeleine Chu",
                                args![
                                    "So how has your cooking",
                                    "been coming along? You'll",
                                    "need to practice to develop",
                                    "your culinary skills. Now,",
                                    "can I help you with anything?"
                                ],
                            )?;
                            ctx.next()?;
                            'b3: {
                                let subject3 = Val::from(runtime::select_values(
                                    ctx,
                                    &[Val::from(
                                        "I need some Cooking Kits.:Will you try the food I cooked?:How does the food I cooked look?",
                                    )],
                                )?);
                                let mut matched3 = false;
                                let no_case3 = !subject3.loosely_equals(&Val::from(1))
                                    && !subject3.loosely_equals(&Val::from(2))
                                    && !subject3.loosely_equals(&Val::from(3));
                                if !matched3 && subject3.loosely_equals(&Val::from(1)) {
                                    matched3 = true;
                                }
                                if matched3 {
                                    ctx.lines_as("Madeleine Chu", args!["Sure, which kind", "of Cooking Kits", "did you need?"])?;
                                    ctx.next()?;
                                    match runtime::select_values(
                                        ctx,
                                        &[Val::from("Outdoor Cooking Kit - 500z:Home Cooking Kit - 1,000z:Quit")],
                                    )? {
                                        1 => {
                                            madeleine_chu_cook_run(ctx, MadeleineChuCookStep::SSellSets, vec![Val::from(12125)])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        2 => {
                                            ctx.lines_as(
                                                "Madeleine Chu",
                                                args![
                                                    "Oh, I'm sorry, but you",
                                                    "don't have enough skills",
                                                    "to use a Home Cooking Kit.",
                                                    "Please practice some more",
                                                    "with the Outdoor Cooking",
                                                    "Kits first, alright?"
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        3 => {
                                            ctx.lines_as(
                                                "Madeleine Chu",
                                                args![
                                                    "Please come back and",
                                                    "let me know if you need",
                                                    "to purchase any Cooking",
                                                    "Kits, alright? See you later~"
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        _ => {}
                                    }
                                }
                                if !matched3 && subject3.loosely_equals(&Val::from(2)) {
                                    matched3 = true;
                                }
                                if matched3 {
                                    if (((((ctx.call(Function::CountItem, vec![Val::from(12041)])?.number()? > 0
                                        && ctx.call(Function::CountItem, vec![Val::from(12046)])?.number()? > 0)
                                        && ctx.call(Function::CountItem, vec![Val::from(12061)])?.number()? > 0)
                                        && ctx.call(Function::CountItem, vec![Val::from(12056)])?.number()? > 0)
                                        && ctx.call(Function::CountItem, vec![Val::from(12051)])?.number()? > 0)
                                        && ctx.call(Function::CountItem, vec![Val::from(12066)])?.number()? > 0)
                                    {
                                        ctx.lines_as(
                                            "Madeleine Chu",
                                            args![
                                                "Oh, you've made a sample",
                                                "of every recipe detailed in",
                                                "that basic cookbook, did you?",
                                                "That must have been very good training for your culinary skills."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Madeleine Chu",
                                            args![
                                                "I'd love to taste your",
                                                "food and give my opinion,",
                                                "but do you mind if I ask",
                                                "you a favor first? I have",
                                                "a friend in Payon who used",
                                                "to study cooking in Prontera."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Madeleine Chu",
                                            args![
                                                "However, he became frustrated",
                                                "with the culinary classes and",
                                                "moved back to Prontera. Would",
                                                "you mind asking him to taste",
                                                "them? Here, I'll wrap your",
                                                "food in this handy cloth..."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.call(Function::DelItem, vec![Val::from(12041), Val::from(1)])?;
                                        ctx.call(Function::DelItem, vec![Val::from(12046), Val::from(1)])?;
                                        ctx.call(Function::DelItem, vec![Val::from(12061), Val::from(1)])?;
                                        ctx.call(Function::DelItem, vec![Val::from(12056), Val::from(1)])?;
                                        ctx.call(Function::DelItem, vec![Val::from(12051), Val::from(1)])?;
                                        ctx.call(Function::DelItem, vec![Val::from(12066), Val::from(1)])?;
                                        ctx.var("cooking_q").set(Val::from(9))?;
                                        ctx.call(Function::GetItem, vec![Val::from(12111), Val::from(1)])?;
                                        ctx.lines_as(
                                            "Madeleine Chu",
                                            args![
                                                "There you go, it's ready",
                                                "to be delivered. Now, make",
                                                "sure not to open this before",
                                                "giving it to my old friend,",
                                                "Chulsoo. You can find him",
                                                "somewhere around Payon..."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Madeleine Chu",
                                            args![
                                                "You should be able to",
                                                "find Chulsoo around the",
                                                "water mill or the pub in",
                                                "Payon. Oh, and don't",
                                                "forget to tell him that",
                                                "I sent you, okay? Thanks~"
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    ctx.lines_as(
                                        "Madeleine Chu",
                                        args![
                                            "Hmm... I think it'd",
                                            "be better if you tried",
                                            "to make every recipe in",
                                            "that basic cookbook I gave",
                                            "you first. That way, I can more",
                                            "accurately judge your skills."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Madeleine Chu",
                                        args![
                                            "It's not bad to focus",
                                            "on just one recipe, but",
                                            "as a beginner, you need",
                                            "to cover all of the basics.",
                                            "Please read the cookbook that I gave you very carefully, okay?"
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                if !matched3 && subject3.loosely_equals(&Val::from(3)) {
                                    matched3 = true;
                                }
                                if matched3 {
                                    if (((((ctx.call(Function::CountItem, vec![Val::from(12041)])?.number()? > 0
                                        || ctx.call(Function::CountItem, vec![Val::from(12046)])?.number()? > 0)
                                        || ctx.call(Function::CountItem, vec![Val::from(12061)])?.number()? > 0)
                                        || ctx.call(Function::CountItem, vec![Val::from(12056)])?.number()? > 0)
                                        || ctx.call(Function::CountItem, vec![Val::from(12051)])?.number()? > 0)
                                        || ctx.call(Function::CountItem, vec![Val::from(12066)])?.number()? > 0)
                                    {
                                        ctx.lines_as(
                                            "Madeleine Chu",
                                            args![
                                                "Oh, I see that you've",
                                                "tried some recipes in that",
                                                "basic cookbook that I gave",
                                                "you. Everything you made looks",
                                                "delicious. All that's left now",
                                                "is for someone to taste it..."
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    ctx.lines_as(
                                        "Madeleine Chu",
                                        args![
                                            "Well... I don't know...",
                                            "I think you really should try",
                                            "to make every recipe listed",
                                            "in that basic cookbook that",
                                            "I gave to you first. Then, you",
                                            "can present your dishes~"
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                            }
                        } else if ctx.var("cooking_q").get()? == 9 {
                            if ctx.call(Function::CountItem, vec![Val::from(12111)])?.number()? > 0 {
                                ctx.lines_as(
                                    "Madeleine Chu",
                                    args![
                                        "Please find my friend",
                                        "Chulsoo in Payon and",
                                        "give him the Bundle of",
                                        "Food so that he can taste",
                                        "the dishes you've made."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                ctx.lines_as(
                                    "Madeleine Chu",
                                    args![
                                        "Ah, hello~ oh, will you give me a second?",
                                        "Right now, I am frying something so, I need to focus on this work for a while.",
                                        "Hahahaha."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Madeleine Chu",
                                    args![
                                        "Oh, so have you met",
                                        "my friend Chulsoo?",
                                        "You brought him the",
                                        "Bundle of Food, right?",
                                        "I'd be disappointed if",
                                        "you lost it or sold it..."
                                    ],
                                )?;
                                ctx.next()?;
                                if Val::from(runtime::select_values(ctx, &[Val::from("I did!:I lost the Bundle of Food!")])?) == 1 {
                                    ctx.lines_as(
                                        "Madeleine Chu",
                                        args![
                                            "Hahaha, I suppose you",
                                            "did. But even if you didn't",
                                            "yet, make sure that you do",
                                            "it soon, alright? See you~"
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                if ctx.call(Function::CountItem, vec![Val::from(7472)])?.number()? > 0 {
                                    ctx.lines_as(
                                        "Madeleine Chu",
                                        args![
                                            "You lost it? Oh, that's",
                                            "not good. How can you ",
                                            "disrespect the culinary",
                                            "arts in that way? I'm so",
                                            "very ashamed of you..."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.call(Function::DelItem, vec![Val::from(7472), Val::from(1)])?;
                                    ctx.var("cooking_q").set(Val::from(0))?;
                                    ctx.lines_as(
                                        "Madeleine Chu",
                                        args![
                                            "First of all, I'd like",
                                            "you to return my cookbook.",
                                            "I want you to reflect on what",
                                            "you've done, and then learn",
                                            "cooking skills from Sir Charles, starting from the very beginning."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                ctx.lines_as(
                                    "Madeleine Chu",
                                    args![
                                        "Oh, you must be joking~",
                                        "I'm sure you must have",
                                        "hidden it somewhere.",
                                        "Anyway, please deliver that",
                                        "Bundle of Food to Chulsoo."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        } else {
                            if ctx.var("cooking_q").get()? == 10 {
                                ctx.lines_as(
                                    "Madeleine Chu",
                                    args![
                                        "I just received a",
                                        "message from Chulsoo",
                                        "thanking me for having",
                                        "you send him that food.",
                                        "I'm guessing that he",
                                        "really liked it a lot."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Madeleine Chu",
                                    args![
                                        "I think you're ready to use",
                                        "higher grade cooking tools now.",
                                        "But never forget that your own",
                                        "skills are the most important",
                                        "factor in quality cuisine."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.var("cooking_q").set(Val::from(11))?;
                                ctx.call(Function::GetItem, vec![Val::from(12126), Val::from(10)])?;
                                ctx.lines_as(
                                    "Madeleine Chu",
                                    args![
                                        "Please try these Indoor",
                                        "Cooking Kits to help you",
                                        "create more delicate dishes.",
                                        "When you run out, feel free to",
                                        "purchase more from me, okay?"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Madeleine Chu",
                                    args![
                                        "Also, if you want to",
                                        "learn some new recipes,",
                                        "why don't you talk to Sir",
                                        "Charles again? Okay then,",
                                        "good luck, and I'll see you later~ "
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if ctx.var("cooking_q").get()? == 11 {
                                ctx.lines_as(
                                    "Madeleine Chu",
                                    args![
                                        "How are you? I hope",
                                        "that you've been honing",
                                        "your cooking skills since",
                                        "the last time we've met.",
                                        "Now, can I help you with",
                                        "anything in particular?"
                                    ],
                                )?;
                                ctx.next()?;
                                'b5: {
                                    let subject5 = Val::from(runtime::select_values(
                                        ctx,
                                        &[Val::from("I need some Cooking Kits.:How is Sir Charles?:Um, who's that kid?")],
                                    )?);
                                    let mut matched5 = false;
                                    let no_case5 = !subject5.loosely_equals(&Val::from(1))
                                        && !subject5.loosely_equals(&Val::from(2))
                                        && !subject5.loosely_equals(&Val::from(3));
                                    if !matched5 && subject5.loosely_equals(&Val::from(1)) {
                                        matched5 = true;
                                    }
                                    if matched5 {
                                        ctx.lines_as("Madeleine Chu", args!["Sure, which kind", "of Cooking Kits", "did you need?"])?;
                                        ctx.next()?;
                                        match runtime::select_values(
                                            ctx,
                                            &[Val::from(
                                                "Outdoor Cooking Kit - 500z:Home Cooking Kit - 1,000z:Show me a different kit.:Quit",
                                            )],
                                        )? {
                                            1 => {
                                                madeleine_chu_cook_run(ctx, MadeleineChuCookStep::SSellSets, vec![Val::from(12125)])?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                            2 => {
                                                madeleine_chu_cook_run(ctx, MadeleineChuCookStep::SSellSets, vec![Val::from(12126)])?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                            3 => {
                                                ctx.lines_as(
                                                    "Madeleine Chu",
                                                    args![
                                                        "Well, I only have two",
                                                        "types of cooking kits,",
                                                        "although there is a superior",
                                                        "Professional Cooking Kit that",
                                                        "real experts, like Sir Charles,",
                                                        "use. Amazing, isn't it?"
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Madeleine Chu",
                                                    args![
                                                        "You're still a beginner, so",
                                                        "my kits will serve you well. ",
                                                        "You know, there's a rumor about a cooking kit that can perfectly",
                                                        "make any recipe, so long as all of the ingredients are provided."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Madeleine Chu",
                                                    args![
                                                        "Of course, it's only",
                                                        "a rumor, probably just",
                                                        "the result of someone's",
                                                        "weird imagination. I still",
                                                        "believe skill is the most",
                                                        "important ingredient~"
                                                    ],
                                                )?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                            4 => {
                                                ctx.lines_as(
                                                    "Madeleine Chu",
                                                    args![
                                                        "Please come back and",
                                                        "let me know if you need",
                                                        "to purchase any Cooking",
                                                        "Kits, alright? See you later~"
                                                    ],
                                                )?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                            _ => {}
                                        }
                                    }
                                    if !matched5 && subject5.loosely_equals(&Val::from(2)) {
                                        matched5 = true;
                                    }
                                    if matched5 {
                                        ctx.lines_as(
                                            "Madeleine Chu",
                                            args![
                                                "Hm? Sir Charles is",
                                                "fine, but lately he's been",
                                                "getting a little upset at",
                                                "even small things. Still,",
                                                "I guess it's understandable."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        l_talk_j = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                                        if l_talk_j.clone() == 1 {
                                            ctx.lines_as(
                                                "Madeleine Chu",
                                                args![
                                                    "You know, when I first",
                                                    "met him, I assumed he was",
                                                    "only good at cooking sweets",
                                                    "like chocolates and caramels.",
                                                    "However, he is highly skilled",
                                                    "at cooking almost everything!"
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Madeleine Chu",
                                                args![
                                                    "I suppose he's been focusing",
                                                    "on foods other than desserts",
                                                    "ever since our king disappeared. I wonder if King Tristram III's",
                                                    "disappearance is related to Sir",
                                                    "Charles's change in mood?"
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        } else if l_talk_j.clone() == 2 {
                                            ctx.lines_as(
                                                "Madeleine Chu",
                                                args![
                                                    "I mean, Sir Charles seems",
                                                    "to be the type that has trouble",
                                                    "opening up to other people.",
                                                    "That may explain why he's much",
                                                    "nicer to women than to men.",
                                                    "Doesn't that make sense?"
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        ctx.lines_as(
                                            "Madeleine Chu",
                                            args![
                                                "Maybe it's because he's",
                                                "been experimenting with",
                                                "a new recipe lately. I think he",
                                                "mentioned something about",
                                                "wanting to treat some woman",
                                                "to the finest food ever made."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Madeleine Chu",
                                            args![
                                                "I've never seen Sir Charles",
                                                "so excited before. That woman",
                                                "must be very lucky: she has the",
                                                "chance to eat his cooking every",
                                                "day if she wanted! I'm almost",
                                                "jealous of her, you know that?"
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    if !matched5 && subject5.loosely_equals(&Val::from(3)) {
                                        matched5 = true;
                                    }
                                    if matched5 {
                                        ctx.lines_as(
                                            "Madeleine Chu",
                                            args![
                                                "Oh, you mean the",
                                                "child with the cat?",
                                                "I'm not sure, but I think",
                                                "I overheard that she might",
                                                "be the younger sister of",
                                                "Madam Wickebine."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Madeleine Chu",
                                            args![
                                                "I wonder why Sir Charles",
                                                "gives Madam Wickebine such",
                                                "special treatment. Whenever",
                                                "I ask him about it, he gets so",
                                                "upset and doesn't say anything!"
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.call(Function::Cutin, vec![Val::from("orleans_6"), Val::from(0)])?;
                                        ctx.lines_as(
                                            "Charles Orleans",
                                            args![
                                                "Mince alors!",
                                                "I just felt a chill down my",
                                                "spine... Could someone",
                                                "be talking about me?"
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                        return Err(Stop::End);
                                    }
                                }
                            }
                        }
                    }
                }
                ctx.lines_as("Madeleine Chu", args!["Error occurred."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            MadeleineChuCookStep::SSellSets => {
                l_item_id = runtime::arg(&args, 0, Val::from(0));
                l_item_cost = ctx.call(Function::GetItemInfo, vec![l_item_id.clone(), ctx.constant("ITEMINFO_BUY")?])?;
                l_item_weight = ctx.call(Function::GetItemInfo, vec![l_item_id.clone(), ctx.constant("ITEMINFO_WEIGHT")?])?;
                ctx.lines_as(
                    "Madeleine Chu",
                    args![
                        (Val::from("How many ")
                            + (if l_item_id.clone() == 12125 {
                                Val::from("Outdoor")
                            } else {
                                Val::from("Indoor")
                            })),
                        "Cooking Kits would",
                        "you like to buy? If you",
                        "want to cancel, please",
                        "enter the number 0."
                    ],
                )?;
                ctx.next()?;
                'l7: loop {
                    if !(true) {
                        break 'l7;
                    }
                    'b7: {
                        let (input, status) = runtime::input_number(ctx, None, None)?;
                        l_sell = input;
                        if l_sell.clone() == 0 {
                            ctx.lines_as(
                                "Madeleine Chu",
                                args![
                                    "You've changed your",
                                    "mind? Well, if you need",
                                    (Val::from("to buy ") + ctx.call(Function::GetItemName, vec![l_item_id.clone()])?),
                                    "Kits later, just come back",
                                    "to me at anytime, alright?"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if l_sell.clone().number()? > 100 {
                            ctx.lines_as(
                                "Madeleine Chu",
                                args![
                                    "Oh, I'm sorry, but",
                                    "I don't sell more than",
                                    (Val::from("100 ") + ctx.call(Function::GetItemName, vec![l_item_id.clone()])?),
                                    "at a time, just to be safe."
                                ],
                            )?;
                            ctx.next()?;
                        } else {
                            break 'l7;
                        }
                    }
                }
                l_total_cost = (l_sell.clone().try_mul(l_item_cost.clone())?);
                l_total_weight = (l_sell.clone().try_mul(l_item_weight.clone())?);
                if runtime::op(&ctx.var("Zeny").get()?, "<", &l_total_cost.clone())?.is_true() {
                    ctx.lines_as(
                        "Madeleine Chu",
                        args![
                            "Oh, I'm sorry, but you",
                            "can't afford this many",
                            ctx.call(Function::GetItemName, vec![l_item_id.clone()])?,
                            "Please check your zeny",
                            "before purchasing my kits~"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if !(ctx.call(Function::CheckWeight, vec![l_item_id.clone(), l_sell.clone()])?.is_true()) {
                    ctx.lines_as(
                        "Madeleine Chu",
                        args![
                            "I'm sorry, but you don't",
                            "have enough room in your",
                            "Inventory for this many",
                            (ctx.call(Function::GetItemName, vec![l_item_id.clone()])? + Val::from("..."))
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(l_total_cost.clone())?))?;
                ctx.call(Function::GetItem, vec![l_item_id.clone(), l_sell.clone()])?;
                ctx.lines_as(
                    "Madeleine Chu",
                    args!["Here you are~", "Best of luck with", "your culinary training!"],
                )?;
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn madeleine_chu_cook(ctx: &Ctx) -> Script {
    madeleine_chu_cook_run(ctx, MadeleineChuCookStep::Start, Vec::new()).map(|_| ())
}

fn child_with_cat_cook_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_nyu = Val::from(0);
    l_nyu = ctx.call(Function::Rand, vec![Val::from(1), Val::from(2)])?;
    if l_nyu.clone() == 1 {
        ctx.call(Function::Cutin, vec![Val::from("nyuang_1"), Val::from(2)])?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_QUESTION")?])?;
        ctx.lines_as("Child with Cat", args!["...Nya?", "(...Meow?)"])?;
        ctx.close_window()?;
        ctx.call(Function::Cutin, vec![Val::from("nyuang_1"), Val::from(255)])?;
        return Err(Stop::End);
    }
    ctx.call(Function::Cutin, vec![Val::from("nyuang_3"), Val::from(2)])?;
    ctx.call(Function::Emotion, vec![ctx.constant("ET_DELIGHT")?])?;
    ctx.lines_as("Child with Cat", args!["Nyahahahaha,", "nyahahahaha~", "(Meow~, meow~)"])?;
    ctx.close_window()?;
    ctx.call(Function::Cutin, vec![Val::from("nyuang_1"), Val::from(255)])?;
    return Err(Stop::End);
}

pub fn child_with_cat_cook(ctx: &Ctx) -> Script {
    child_with_cat_cook_body(ctx, Vec::new()).map(|_| ())
}

fn wickebine_cook_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Wickebine#cook")])?;
    return Err(Stop::End);
}

pub fn wickebine_cook(ctx: &Ctx) -> Script {
    wickebine_cook_body(ctx, Vec::new()).map(|_| ())
}

fn wickebine_cook_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Wickebine#cook")])?;
    return Err(Stop::End);
}

pub fn wickebine_cook_oninit(ctx: &Ctx) -> Script {
    wickebine_cook_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn wickebine_cook_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("Wickebine#cook")])?;
    return Err(Stop::End);
}

pub fn wickebine_cook_onenable(ctx: &Ctx) -> Script {
    wickebine_cook_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn wickebine_cook_ondisable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Wickebine#cook")])?;
    return Err(Stop::End);
}

pub fn wickebine_cook_ondisable(ctx: &Ctx) -> Script {
    wickebine_cook_ondisable_body(ctx, Vec::new()).map(|_| ())
}

fn servant_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if !(ctx.call(Function::CheckWeight, vec![Val::from(555), Val::from(1)])?.is_true()) {
        ctx.lines_as(
            "Chulsoo",
            args![
                "Hold on, you're carrying",
                "too many items with you.",
                "Why don't you put some of",
                "your stuff in Kafra Storage",
                "before coming back to me?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("cooking_q").get()? == 10 {
        ctx.lines_as(
            "Chulsoo",
            args![
                "When you get the chance,",
                "please give Madeline my",
                "thanks. I'll visit Prontera",
                "soon to see her, as well as",
                "make amends with Sir Charles."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("cooking_q").get()? == 9 {
        ctx.lines_as(
            "Chulsoo",
            args![
                "How would you like",
                "to buy a Rice Cake?",
                "It's only 200 zeny, but",
                "it's oh-so-delicious~"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(
            ctx,
            &[Val::from("Sure, I'll buy one!:No, thanks.:Actually, Madeleine sent me...")],
        )? {
            1 => {
                if ctx.var("Zeny").get()?.number()? < 200 {
                    ctx.lines_as(
                        "Chulsoo",
                        args![
                            "Oh, I'm sorry, but",
                            "you don't have enough",
                            "money to buy a Rice Cake...",
                            "Still, it should be easy to",
                            "raise 200 zeny, right?"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(200))?))?;
                ctx.call(Function::GetItem, vec![Val::from(555), Val::from(1)])?;
                ctx.lines_as("Chulsoo", args!["Thank you very", "much! I hope you", "enjoy your Rice Cake~"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Chulsoo",
                    args![
                        "Are you sure about",
                        "that? You won't get",
                        "the chance to have a",
                        "Rice Cake this delicious",
                        "anywhere else. Oh well,",
                        "that means more for me~"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            3 => {
                if ctx.call(Function::CountItem, vec![Val::from(12111)])?.number()? > 0 {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Actually, Madeleine",
                            "sent me here to find",
                            "you. She said that you'd",
                            "be willing to taste test",
                            "the food in this bundle..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Chulsoo",
                        args![
                            "Madeleine? You mean",
                            "Madeleine Chu? Oh, I haven't",
                            "heard from her in such a long",
                            "time! Great, let me see the",
                            "bundle that she sent me. Ah, everything here looks appetizing!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Chulsoo",
                        args![
                            "Oh, wait. She even",
                            "included a message",
                            "inside this bundle.",
                            "Let's see, here..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^333333Dear Chulsoo,",
                        " It's been a long time.",
                        "I know you left Prontera on",
                        "bad terms with Sir Charles,",
                        "but please understand that",
                        "he was only trying his best to",
                        "help improve your cooking.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^333333 Sir Charles always wished",
                        "that you'd expand your repetoire, and that you'd make these kinds",
                        "of foods someday. The person",
                        "that delivered this food also",
                        "cooked it. Please try it...^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^333333 Hopefully, you'll be",
                        "able to understand Sir ",
                        "Charles a little better ",
                        "after tasting this food.",
                        " ",
                        " Your friend, Madeleine^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Chulsoo",
                        args![
                            "Now I get it...",
                            "These are the recipes",
                            "that Sir Charles tried",
                            "to teach me. But I refused",
                            "to learn them because I had",
                            "thought they were too gross..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Chulsoo",
                        args![
                            "......",
                            ".........",
                            "It's so delicious... Are",
                            "you sure you're just a",
                            "beginner? No... This must",
                            "be what I've been missing..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Chulsoo",
                        args![
                            "It's what my master",
                            "always tried to teach me,",
                            "but I was too impatient to",
                            "properly learn it. The greatest",
                            "ingredient of them all... ^D02090heart^000000. After all this time, I understand."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Chulsoo",
                        args![
                            "Thank you for bringing",
                            "this food to me. I will enjoy",
                            "it thoroughly, and reflect upon",
                            "what my old teacher was trying",
                            "to tell me. In return, please have one of my humble Rice Cakes."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::DelItem, vec![Val::from(12111), Val::from(1)])?;
                    ctx.var("cooking_q").set(Val::from(10))?;
                    ctx.call(Function::GetItem, vec![Val::from(555), Val::from(1)])?;
                    ctx.lines_as(
                        "Chulsoo",
                        args![
                            "I better visit Prontera",
                            "again soon. It's been a long",
                            "time since I've seen Madeleine.",
                            "More importantly, I think that",
                            "I should apologize to Sir Charles. "
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "Actually, Madeleine",
                        "sent me here to find",
                        "you. She said that you'd",
                        "be willing to taste test",
                        "the food in this bundle..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["Wait, wait...", "I don't have it!", "Where did I put", "that Bundle of Food?"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if ctx.var("cooking_q").get()? == 8 {
        ctx.lines_as(
            "Chulsoo",
            args![
                "Lately, it seems that",
                "no one wants to buy my",
                "Rice Cakes. It's been like",
                "that ever since I left Prontera... "
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Chulsoo",
            args![
                "You see, I used to study",
                "in that city as one of Sir",
                "Charles's apprentices. It",
                "was only a few months, but",
                "I was very excited to get the",
                "chance to learn under him."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Chulsoo",
            args![
                "At least, I was excited",
                "at first. Sir Charles really",
                "frustrated me: he would",
                "only teach me to make these",
                "really gross sounding recipes! Like Grasshopper Legs and-- ugh!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Chulsoo",
            args![
                "He kept insisting that",
                "I was forgetting the most",
                "important ingredient, and that",
                "it was possible to make things",
                "like Fried Monkey Tails delicious. But I can't believe that nonsense!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Chulsoo",
            args![
                "In the end, I ran away.",
                "For some reason, I feel",
                "a little ashamed and regret",
                "what I did. Still, I don't see",
                "what Sir Charles meant..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Chulsoo",
            args![
                "How would you like",
                "to buy a Rice Cake?",
                "It's only 200 zeny, but",
                "it's oh-so-delicious~"
            ],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Sure, I'll buy one!:No, thanks.")])?) == 1 {
            if ctx.var("Zeny").get()?.number()? < 200 {
                ctx.lines_as(
                    "Chulsoo",
                    args![
                        "Oh, I'm sorry, but",
                        "you don't have enough",
                        "money to buy a Rice Cake...",
                        "Still, it should be easy to",
                        "raise 200 zeny, right?"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(200))?))?;
            ctx.call(Function::GetItem, vec![Val::from(555), Val::from(1)])?;
            ctx.lines_as("Chulsoo", args!["Thank you very", "much! I hope you", "enjoy your Rice Cake~"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Chulsoo",
            args![
                "Are you sure about",
                "that? You won't get",
                "the chance to have a",
                "Rice Cake this delicious",
                "anywhere else. Oh well,",
                "that means more for me~"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn servant(ctx: &Ctx) -> Script {
    servant_body(ctx, Vec::new()).map(|_| ())
}

use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn larissa_mos_01_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_di: Vec<Val> = Vec::new();
    let mut l_i = Val::from(0);
    let mut l_n: Vec<Val> = Vec::new();
    if (ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 2000 {
        ctx.lines(args![
            "- Please stop here !! -",
            "- You're carrying too many items -",
            "- Please try again -",
            "- after using the kafra service -"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("mos_swan").get()? == 100 {
        ctx.lines_as(
            "Larissa",
            args![
                "Oh, you're the adventurer that Madame told me about.",
                "Did you find her son?",
                "I'm so glad. He's a good kid."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Larissa",
            args!["Here you are. This special pancake is for you.", "It will be very delicious."],
        )?;
        ctx.var("mos_swan").set(Val::from(101))?;
        ctx.call(Function::GetItem, vec![Val::from(592), Val::from(5)])?;
        ctx.call(Function::GetItem, vec![Val::from(593), Val::from(5)])?;
        ctx.next()?;
        ctx.lines_as(
            "Larissa",
            args![
                "Whenever you'd like to eat these pancake, you come on by.",
                "I'll bake you the most delicious one."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("mos_swan").get()?.number()? > 100 {
        ctx.lines_as(
            "Larissa",
            args!["Did the pancake taste good?", "Would you like another pancake?"],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Yes:No")])?) == 1 {
            if (((((ctx.call(Function::CountItem, vec![Val::from(7031)])?.is_true()
                && ctx.call(Function::CountItem, vec![Val::from(519)])?.number()? > 1)
                && ctx.call(Function::CountItem, vec![Val::from(504)])?.number()? > 1)
                && ctx.call(Function::CountItem, vec![Val::from(548)])?.is_true())
                && ctx.call(Function::CountItem, vec![Val::from(1019)])?.is_true())
                && ctx.call(Function::CountItem, vec![Val::from(518)])?.is_true())
            {
                ctx.lines_as(
                    "Larissa",
                    args![
                        "Let me check if you have all the ingredients.",
                        "A frying pan, milk... potion that softens the dough, cheese..honey..and firewood.",
                        "Perfect!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Larissa",
                    args![
                        "Let us begin by kneading the dough.",
                        "I'll add sugar and baking powder.",
                        "It can be mixed well with this sieve."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Larissa", args!["What should I do with the wheat flour?"])?;
                ctx.next()?;
                match runtime::select_values(
                    ctx,
                    &[Val::from("Turn it down.:Pass it through a sieve.:Press it with your fist.")],
                )? {
                    1 => {
                        ctx.lines_as(
                            "Larissa",
                            args![
                                "Oh, my! What are you doing now?",
                                "I'm going to do that from the beginning.",
                                "Look, pass it through a sieve."
                            ],
                        )?;
                    }
                    2 => {
                        ctx.lines_as("Larissa", args!["There you go. Just do it slowly while tapping the sides, sifting the sugar and baking powder through the sieve together."])?;
                    }
                    3 => {
                        ctx.lines_as(
                            "Larissa",
                            args!["If we don't have the sieve, we can use our fist but we've got one. So we might as well use it right?"],
                        )?;
                    }
                    _ => {}
                }
                ctx.next()?;
                ctx.lines_as(
                    "Larissa",
                    args![
                        "That way, it will mix well.",
                        "And then I'm adding an egg, some milk and a white potion.",
                        "What next?"
                    ],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("Stir it up with great speed.:Do the same as she said.")],
                )?) == 1
                {
                    ctx.lines_as(
                        "Larissa",
                        args!["No, you don't have to do that way. It won't rise properly if you stir it too fast."],
                    )?;
                } else {
                    ctx.lines_as("Larissa", args!["You're very good at this!"])?;
                }
                ctx.next()?;
                ctx.lines_as(
                    "Larissa",
                    args![
                        "It'll rise while baking if you stir it properly",
                        "Ok, let me bake now. Let's pour it on our pan and spread it around."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Larissa",
                    args![
                        "Cook in low temperature. It'll rise and have bubbles on it.",
                        "When you see the bubbles, you can turn it over."
                    ],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("Cook in high temperature.:Can I turn it over now?")],
                )?) == 1
                {
                    ctx.lines_as(
                        "Larissa",
                        args!["No, you don't do that.", "You'll burn it up.", "Wait for a while."],
                    )?;
                } else {
                    ctx.lines_as("Larissa", args!["Hmm let's see. It's good.", "You were good at it."])?;
                }
                ctx.next()?;
                ctx.lines_as(
                    "Larissa",
                    args![
                        "Ok, turn it over. Oh it looks delicious",
                        "What should I top it with? First I put some cheese but the rest is up to you..."
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Caviar:Mushrooms:Honey:Strawberry jam:Sour cream")])? {
                    1 => {
                        ctx.lines_as(
                            "Larissa",
                            args![
                                "Ok. you want caviar. Fortunately, I still have a little left.",
                                "Hoohoo, this'll be luxurious..",
                                "Here we go. It's all done!"
                            ],
                        )?;
                        let base = Val::from(0).number()?;
                        runtime::local_set(&mut l_n, &Val::from(base + 0), Val::from(591), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 1), Val::from(1), false);
                    }
                    2 => {
                        ctx.lines_as(
                            "Larissa",
                            args!["Ok. you want mushrooms.. It has an earthly flavor.", "Here we go. It's all done!"],
                        )?;
                        let base = Val::from(0).number()?;
                        runtime::local_set(&mut l_n, &Val::from(base + 0), Val::from(595), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 1), Val::from(1), false);
                    }
                    3 => {
                        ctx.lines_as(
                            "Larissa",
                            args!["Ok, honey syrup goes perfectly with pancakes.", "Here we go. It's all done!"],
                        )?;
                        let base = Val::from(0).number()?;
                        runtime::local_set(&mut l_n, &Val::from(base + 0), Val::from(593), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 1), Val::from(1), false);
                    }
                    4 => {
                        ctx.lines_as(
                            "Larissa",
                            args![
                                "Who could get sick of pancakes with strawberry jam?",
                                "Hoohoo it tastes sweet too.",
                                "Here we go. It's all done!"
                            ],
                        )?;
                        let base = Val::from(0).number()?;
                        runtime::local_set(&mut l_n, &Val::from(base + 0), Val::from(592), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 1), Val::from(1), false);
                    }
                    5 => {
                        ctx.lines_as(
                            "Larissa",
                            args![
                                "You want sour cream.. you will feel as if you fly in the sky.",
                                "It can help reduce stress.",
                                "Here we go. It's all done!"
                            ],
                        )?;
                        let base = Val::from(0).number()?;
                        runtime::local_set(&mut l_n, &Val::from(base + 0), Val::from(594), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 1), Val::from(1), false);
                    }
                    _ => {}
                }
                let base = Val::from(0).number()?;
                runtime::local_set(&mut l_di, &Val::from(base + 0), Val::from(7031), false);
                runtime::local_set(&mut l_di, &Val::from(base + 1), Val::from(1), false);
                runtime::local_set(&mut l_di, &Val::from(base + 2), Val::from(519), false);
                runtime::local_set(&mut l_di, &Val::from(base + 3), Val::from(2), false);
                runtime::local_set(&mut l_di, &Val::from(base + 4), Val::from(504), false);
                runtime::local_set(&mut l_di, &Val::from(base + 5), Val::from(2), false);
                runtime::local_set(&mut l_di, &Val::from(base + 6), Val::from(548), false);
                runtime::local_set(&mut l_di, &Val::from(base + 7), Val::from(1), false);
                runtime::local_set(&mut l_di, &Val::from(base + 8), Val::from(1019), false);
                runtime::local_set(&mut l_di, &Val::from(base + 9), Val::from(1), false);
                runtime::local_set(&mut l_di, &Val::from(base + 10), Val::from(518), false);
                runtime::local_set(&mut l_di, &Val::from(base + 11), Val::from(1), false);
                l_i = Val::from(0);
                'l3: loop {
                    if !(runtime::op(&l_i.clone(), "<", &Val::from(l_di.len() as i32))?.is_true()) {
                        break 'l3;
                    }
                    'b3: {
                        ctx.call(
                            Function::DelItem,
                            vec![
                                runtime::local_get(&l_di, &l_i.clone(), false),
                                runtime::local_get(&l_di, &(l_i.clone() + Val::from(1)), false),
                            ],
                        )?;
                    }
                    l_i = (l_i.clone() + Val::from(2));
                }
                ctx.call(
                    Function::GetItem,
                    vec![
                        runtime::local_get(&l_n, &Val::from(0), false),
                        runtime::local_get(&l_n, &Val::from(1), false),
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Larissa",
                args![
                    "That's good. I have ingredients and several toppings but...",
                    "It's still not enough. You need to get all of the things that we don't have now."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Larissa",
                args![
                    "I need ^4d4dff1 old frying pan , 2 bottles of milk , 2 white potion, 1 cheese, 1 Trunk, 1 Honey ^000000.",
                    "We can bake our pancake when we get all of those."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as("Larissa", args!["Ok we can bake it next time.", "I'm always available."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Larissa",
        args![
            "Young lady is a practical joker and her brother is tenderhearted.",
            "By the way, I haven't seen him for quite some time."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn larissa_mos_01(ctx: &Ctx) -> Script {
    larissa_mos_01_body(ctx, Vec::new()).map(|_| ())
}

fn acorn_dealer_mos_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input = Val::from(0);
    let mut l_price = Val::from(0);
    ctx.lines_as(
        "Acorn Dealer",
        args![
            "We have very fresh acorns. Everyone will like them!",
            "You can buy one acorn for 100zeny!"
        ],
    )?;
    ctx.next()?;
    if Val::from(runtime::select_values(ctx, &[Val::from("Buy one.:What can I use them for?")])?) == 2 {
        ctx.lines_as(
            "Acorn Dealer",
            args![
                "Well uh...",
                "You can grind them to make",
                "something to eat and you can feed squirrels."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Acorn Dealer",
            args![
                "Someone can decorate their house",
                "with them but I don't know how...",
                "they've got to be highly talented.",
                "Haha."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Acorn Dealer",
        args!["I'll bet you that they are very fresh!", "How many acorns do you need?"],
    )?;
    ctx.next()?;
    'l1: loop {
        if !(l_input.clone() == 0 || l_input.clone().number()? > 500) {
            break 'l1;
        }
        'b1: {
            let (input, status) = runtime::input_number(ctx, None, None)?;
            l_input = input;
            if l_input.clone().number()? <= 0 {
                ctx.lines_as("Acorn Dealer", args!["Do you want to cancel this trade?"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if l_input.clone().number()? > 500 {
                ctx.lines_as("Acorn Dealer", args!["You can't buy more than 500."])?;
                ctx.next()?;
            }
        }
    }
    if !(ctx.call(Function::CheckWeight, vec![Val::from(1026), l_input.clone()])?.is_true()) {
        ctx.lines_as(
            "Acorn Dealer",
            args![
                "Hello, I think you can't get acorns",
                "now. You're carrying too many",
                "items!",
                "Please use Kafra service. I'll be",
                "right here."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    l_price = (l_input.clone().try_mul(Val::from(100))?);
    if runtime::op(&ctx.var("Zeny").get()?, "<", &l_price.clone())?.is_true() {
        ctx.lines_as(
            "Acorn Dealer",
            args!["Hello? You've turned pale! Are you ok??", "Do you have enough money?"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Acorn Dealer",
        args!["Oh, thank you...", "What do you think of them? They're fresh, aren't they?"],
    )?;
    ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(l_price.clone())?))?;
    ctx.call(Function::GetItem, vec![Val::from(1026), l_input.clone()])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn acorn_dealer_mos(ctx: &Ctx) -> Script {
    acorn_dealer_mos_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum BigSquirrelMosStep {
    Start,
    OnTouch,
}

fn big_squirrel_mos_run(ctx: &Ctx, mut step: BigSquirrelMosStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_cyworld = Val::from(0);
    let mut l_stonez1 = Val::from(0);
    let mut l_stworld = Val::from(0);
    'machine: loop {
        match step {
            BigSquirrelMosStep::Start => {
                step = BigSquirrelMosStep::OnTouch;
                continue 'machine;
            }
            BigSquirrelMosStep::OnTouch => {
                if (ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 2000 {
                    ctx.lines(args![
                        "- Please stop here!! -",
                        "- You're carrying too many items -",
                        "- Please try again -",
                        "- after using the kafra service -"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.mes("- It's an extraordinary big squirrel. -")?;
                ctx.next()?;
                ctx.mes("- When the animal comes across you, it starts to sniffle and purse up its lips. -")?;
                ctx.next()?;
                if ctx.call(Function::CountItem, vec![Val::from(1026)])?.number()? > 19 {
                    match runtime::select_values(ctx, &[Val::from("Give acorns to it.:Ignore it.:Ask about the squirrel.")])? {
                        1 => {
                            ctx.mes("- You take one of acorns out and hold it out to the squirrel. -")?;
                            ctx.next()?;
                            ctx.mes("- It cocked it's ears up and begins to nibble the acorn quickly. -")?;
                            ctx.next()?;
                            ctx.lines(args![
                                "- It makes a crunching sound -",
                                "- It makes a crunching sound -",
                                "- It makes a crunching sound -"
                            ])?;
                            ctx.next()?;
                            ctx.mes("- After eating the acorn up, the squirrel dances around wildly. Suddenly it curls its body and throws something up with a spit-spit sound -")?;
                            ctx.call(Function::DelItem, vec![Val::from(1026), Val::from(20)])?;
                            l_cyworld = ctx.call(Function::Rand, vec![Val::from(1), Val::from(1000)])?;
                            l_stworld = ctx.call(Function::Rand, vec![Val::from(1), Val::from(150)])?;
                            if ((l_cyworld.clone().try_rem(Val::from(100))?) == 0 && l_cyworld.clone().number()? < 1000) {
                                if l_stworld.clone().number()? < 90 {
                                    if l_stworld.clone().number()? < 10 {
                                        ctx.call(Function::GetItem, vec![Val::from(718), Val::from(1)])?;
                                    } else {
                                        if l_stworld.clone().number()? < 20 {
                                            ctx.call(Function::GetItem, vec![Val::from(719), Val::from(1)])?;
                                        } else {
                                            if l_stworld.clone().number()? < 30 {
                                                ctx.call(Function::GetItem, vec![Val::from(720), Val::from(1)])?;
                                            } else {
                                                if l_stworld.clone().number()? < 40 {
                                                    ctx.call(Function::GetItem, vec![Val::from(721), Val::from(1)])?;
                                                } else if l_stworld.clone().number()? < 50 {
                                                    ctx.call(Function::GetItem, vec![Val::from(722), Val::from(1)])?;
                                                } else if l_stworld.clone().number()? < 60 {
                                                    ctx.call(Function::GetItem, vec![Val::from(723), Val::from(1)])?;
                                                } else if l_stworld.clone().number()? < 70 {
                                                    ctx.call(Function::GetItem, vec![Val::from(725), Val::from(1)])?;
                                                } else if l_stworld.clone().number()? < 80 {
                                                    ctx.call(Function::GetItem, vec![Val::from(728), Val::from(1)])?;
                                                } else {
                                                    ctx.call(Function::GetItem, vec![Val::from(729), Val::from(1)])?;
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    if l_stworld.clone().number()? < 100 {
                                        if ((l_cyworld.clone() == 100 || l_cyworld.clone() == 400) || l_cyworld.clone() == 700) {
                                            ctx.call(Function::GetItem, vec![Val::from(730), Val::from(1)])?;
                                        } else if ((l_cyworld.clone() == 200 || l_cyworld.clone() == 500) || l_cyworld.clone() == 800) {
                                            ctx.call(Function::GetItem, vec![Val::from(731), Val::from(1)])?;
                                        } else {
                                            ctx.call(Function::GetItem, vec![Val::from(732), Val::from(1)])?;
                                        }
                                    } else {
                                        let subject2 = l_cyworld.clone();
                                        if subject2 == 200 {
                                            l_stonez1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?;
                                            if l_stonez1.clone().number()? < 6 {
                                                ctx.call(Function::GetItem, vec![Val::from(7290), Val::from(1)])?;
                                            } else if l_stonez1.clone().number()? < 8 {
                                                ctx.call(Function::GetItem, vec![Val::from(7297), Val::from(1)])?;
                                            } else {
                                                ctx.call(Function::GetItem, vec![Val::from(7292), Val::from(1)])?;
                                            }
                                        } else if subject2 == 300 {
                                            if ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?.number()? < 6 {
                                                ctx.call(Function::GetItem, vec![Val::from(7291), Val::from(1)])?;
                                            } else {
                                                ctx.call(Function::GetItem, vec![Val::from(7294), Val::from(1)])?;
                                            }
                                        } else if subject2 == 100 {
                                            ctx.call(Function::GetItem, vec![Val::from(7289), Val::from(1)])?;
                                        } else if subject2 == 400 {
                                            ctx.call(Function::GetItem, vec![Val::from(7295), Val::from(1)])?;
                                        } else if subject2 == 500 {
                                            ctx.call(Function::GetItem, vec![Val::from(7293), Val::from(1)])?;
                                        } else if subject2 == 600 {
                                            ctx.call(Function::GetItem, vec![Val::from(7292), Val::from(1)])?;
                                        } else if subject2 == 700 {
                                            ctx.call(Function::GetItem, vec![Val::from(7290), Val::from(1)])?;
                                        } else if subject2 == 800 {
                                            ctx.call(Function::GetItem, vec![Val::from(7296), Val::from(1)])?;
                                        } else if subject2 == 900 {
                                            ctx.call(Function::GetItem, vec![Val::from(7297), Val::from(1)])?;
                                        }
                                    }
                                }
                            } else {
                                if l_cyworld.clone() == 1000 {
                                    ctx.call(Function::GetItem, vec![Val::from(727), Val::from(1)])?;
                                } else if l_cyworld.clone().number()? < 500 {
                                    ctx.call(Function::GetItem, vec![Val::from(733), Val::from(1)])?;
                                } else {
                                    ctx.call(Function::GetItem, vec![Val::from(724), Val::from(1)])?;
                                }
                            }
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["It's so cute."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        3 => {
                            ctx.lines_as(
                                "Aged Man",
                                args![
                                    "Is it bigger than an ordinary one?",
                                    "I found it one day and was barely able to save it..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Aged Man",
                                args!["I felt that heaven had sent me a friend so that my life wouldn't be so lonely."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Aged Man", args!["But I didn't know that it eats so much, it won't give me any attention if I give anything ^3131FFbelow 20 acorns^000000.", "That's why it's bigger than ordinary ones hahaha."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                }
                ctx.mes("- The squirrel looks at the acorn which you held for a while but it turned its head with indifference. -")?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn big_squirrel_mos(ctx: &Ctx) -> Script {
    big_squirrel_mos_run(ctx, BigSquirrelMosStep::Start, Vec::new()).map(|_| ())
}

pub fn big_squirrel_mos_ontouch(ctx: &Ctx) -> Script {
    big_squirrel_mos_run(ctx, BigSquirrelMosStep::OnTouch, Vec::new()).map(|_| ())
}

fn mos_sq_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn mos_sq(ctx: &Ctx) -> Script {
    mos_sq_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum BabaYagaTheHorribleStep {
    Start,
    SBY1,
    OnTouch,
}

fn baba_yaga_the_horrible_run(ctx: &Ctx, mut step: BabaYagaTheHorribleStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    let mut l_item: Vec<Val> = Vec::new();
    let mut l_plus1 = Val::from(0);
    let mut l_size = Val::from(0);
    'machine: loop {
        match step {
            BabaYagaTheHorribleStep::Start => {
                if (!(ctx.var("mos_nowinter").get()?.is_true()) || ctx.var("mos_nowinter").get()? == 1) {
                    ctx.lines_as(
                        "Baba Yaga, the Horrible",
                        args![
                            "Hohoho",
                            "I need to kick your ass more.",
                            "Come here,",
                            "to play a game.",
                            "Heeeeeeeee."
                        ],
                    )?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("mosk_dun02"), Val::from(53), Val::from(217)])?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("mos_nowinter").get()? == 2 {
                        ctx.lines_as(
                            "Baba Yaga, the Horrible",
                            args![
                                "Let me live, leave me alone, and don't come near me.",
                                "Stop, I am hungry and scared."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.mes("-She is blabbing something weird.-")?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Baba Yaga, the Horrible",
                            args!["Yes, if you let me live, I will compensate you", "with a present, how about that?"],
                        )?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(ctx, &[Val::from("A present?:I don't need.")])?) == 1 {
                            ctx.lines_as(
                                "Baba Yaga, the Horrible",
                                args![
                                    "Yes, if you let me live",
                                    "I will give you the Yaga Pestles",
                                    "Aren't you gathering them?"
                                ],
                            )?;
                            ctx.next()?;
                            let choice = runtime::select_values(ctx, &[Val::from("I don't need them anymore.")])?;
                            ctx.var("@menu").set(choice)?;
                        }
                        ctx.lines_as(
                            "Baba Yaga, the Horrible",
                            args!["Kaaaaaaaaaak!", "Please, let me live,", "I will do anything you want."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Baba Yaga, the Horrible",
                            args![
                                "Don't you need any cream for wounds",
                                "or indigestion?",
                                "They are a bit dirty,",
                                "but very useful."
                            ],
                        )?;
                        ctx.next()?;
                        let choice = runtime::select_values(ctx, &[Val::from("Well...")])?;
                        ctx.var("@menu").set(choice)?;
                        ctx.lines_as(
                            "Baba Yaga, the Horrible",
                            args![
                                "How about this?",
                                "I bewitch this country so that",
                                "the winter will never come here again."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Baba Yaga, the Horrible",
                            args![
                                "I guess that people will like it and",
                                "I'm sure that the Csar",
                                "will award you for it."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Baba Yaga, the Horrible",
                            args!["Ah? You seem", "interested... ya?", "What do you think about that?"],
                        )?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(
                            ctx,
                            &[Val::from("Good, but I think you're lying.:I am not interested.")],
                        )?) == 1
                        {
                            ctx.lines_as(
                                "Baba Yaga, the Horrible",
                                args![
                                    "Sure, you can trust me.",
                                    "It is so complicated that I cannot",
                                    "perform it on my own",
                                    "I am not a liar."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Baba Yaga, the Horrible", args!["But, the problem is..."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Baba Yaga, the Horrible",
                                args!["I am not able to get the materials necessary", "because I don't feel good."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Baba Yaga, the Horrible",
                                args![
                                    "So,",
                                    "if you give me a hand,",
                                    "I can use the magic that",
                                    "stops winter from returning. Will you help me?"
                                ],
                            )?;
                            ctx.next()?;
                            if Val::from(runtime::select_values(ctx, &[Val::from("Ok, I will.:No. I don't like it.")])?) == 1 {
                                baba_yaga_the_horrible_run(ctx, BabaYagaTheHorribleStep::SBY1, vec![])?;
                            }
                            ctx.lines_as(
                                "Baba Yaga, the Horrible",
                                args!["Heeeek,", "Just, just I need a little help.. It is really little."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Baba Yaga, the Horrible", args!["Please, don't kill me..."])?;
                            ctx.next()?;
                            ctx.mes("'Hmmm, what should I do...'")?;
                            ctx.var("mos_nowinter").set(Val::from(4))?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as("Baba Yaga, the Horrible", args!["Akkk... What on earth did you do to me..."])?;
                        ctx.next()?;
                        ctx.mes("Hmm... What do you want...")?;
                        ctx.var("mos_nowinter").set(Val::from(3))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if ctx.var("mos_nowinter").get()? == 3 {
                            ctx.lines_as("Baba Yaga, the Horrible", args!["Akkkk!", "You came back.."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Baba Yaga, the Horrible",
                                args!["If you tell the Csar that", "winter won't come back again", "he will reward you."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Baba Yaga, the Horrible",
                                args!["Isn't that better for you", "than hurting me?"],
                            )?;
                            ctx.next()?;
                            if Val::from(runtime::select_values(
                                ctx,
                                &[Val::from("Ok, tell me the story.:Let me think...")],
                            )?) == 1
                            {
                                ctx.lines_as(
                                    "Baba Yaga, the Horrible",
                                    args![
                                        "Certainly, trust me.",
                                        "It is so complicated that I cannot",
                                        "perform it on my own",
                                        "I am not a liar."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Baba Yaga, the Horrible", args!["The problem is..."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Baba Yaga, the Horrible",
                                    args!["I am not able to get the materials necessary", "because I don't feel good."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Baba Yaga, the Horrible",
                                    args![
                                        "So,",
                                        "if you give me a hand.",
                                        "I can use the magic that",
                                        "stops winter from returning. Will you help me?"
                                    ],
                                )?;
                                ctx.next()?;
                                if Val::from(runtime::select_values(ctx, &[Val::from("Ok, I will.:No. I don't like it.")])?) == 1 {
                                    baba_yaga_the_horrible_run(ctx, BabaYagaTheHorribleStep::SBY1, vec![])?;
                                }
                                ctx.lines_as(
                                    "Baba Yaga, the Horrible",
                                    args!["Heeeeek,", "Just, just I need a little help.. It is really little."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Baba Yaga, the Horrible", args!["Please, don't kill me..."])?;
                                ctx.next()?;
                                ctx.mes("'Hmm... what should I do...'")?;
                                ctx.var("mos_nowinter").set(Val::from(4))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.lines_as(
                                "Baba Yaga, the Horrible",
                                args!["You think more...", "What do you want from me..."],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            if ctx.var("mos_nowinter").get()? == 4 {
                                ctx.lines_as(
                                    "Baba Yaga, the Horrible",
                                    args!["You just need to help little,", "really little..."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Baba Yaga, the Horrible", args!["Well, did you change your mind?"])?;
                                ctx.next()?;
                                if Val::from(runtime::select_values(
                                    ctx,
                                    &[Val::from("Ok, let's try.:Let me think more...")],
                                )?) == 1
                                {
                                    baba_yaga_the_horrible_run(ctx, BabaYagaTheHorribleStep::SBY1, vec![])?;
                                }
                                ctx.lines_as(
                                    "Baba Yaga, the Horrible",
                                    args!["You think more...", "What do you want from me..."],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                if ctx.var("mos_nowinter").get()? == 5 {
                                    ctx.lines_as("Baba Yaga, the Horrible", args!["Let me see...where the season spell..."])?;
                                    ctx.next()?;
                                    ctx.mes("rummaging...")?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Baba Yaga, the Horrible",
                                        args!["Ah, here it is... I found it!", "Hoook~ Hoook~"],
                                    )?;
                                    ctx.next()?;
                                    ctx.mes("-What a dusty old book!-")?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Baba Yaga, the Horrible",
                                        args![
                                            "Well, open your ears.",
                                            "It is so rare around here that",
                                            "it is not easy for you to remember at once."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Baba Yaga, the Horrible",
                                        args!["Above all,", "I need 20 Grasshopper's Legs..."],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Baba Yaga, the Horrible", args!["Cough, cough,", "Kaaak, Kaaaaak"])?;
                                    ctx.next()?;
                                    ctx.lines_as("Baba Yaga, the Horrible", args!["Sniff, sniff.", "5 Spawn..."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Baba Yaga, the Horrible", args!["eh... and...", "20 Wings Of Red Bat..."])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Baba Yaga, the Horrible",
                                        args!["Let me see...", "a glue, no, to cast a spell", "10 Sticky Mucus..."],
                                    )?;
                                    ctx.next()?;
                                    if Val::from(runtime::select_values(
                                        ctx,
                                        &[Val::from("...Are you sure?:A glue? You want me to kill you?!")],
                                    )?) == 1
                                    {
                                        ctx.lines_as(
                                            "Baba Yaga, the Horrible",
                                            args!["Yes, yes, I am certain. Why don't you trust me..."],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Baba Yaga, the Horrible",
                                            args!["Cough, cough,", "I am coughing now, so do not disturb me,", "you write them down."],
                                        )?;
                                        l_plus1 = Val::from(1);
                                    } else {
                                        ctx.lines_as("Baba Yaga, the Horrible", args!["No, It just..."])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Baba Yaga, the Horrible",
                                            args![
                                                "Ehhh, terrible.",
                                                "Hey, relax your hand and",
                                                "keep writing.",
                                                "Forget the Sticky Mucus..."
                                            ],
                                        )?;
                                    }
                                    ctx.next()?;
                                    ctx.lines_as("Baba Yaga, the Horrible", args!["Where were we...", "Cough, cough, cough!"])?;
                                    ctx.next()?;
                                    ctx.lines_as("Baba Yaga, the Horrible", args!["Ah, yes. Next elements are", "essential."])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Baba Yaga, the Horrible",
                                        args![
                                            "10 Witched Starsand",
                                            "10 Fine Grits...",
                                            "Hooook, hooook~",
                                            "Ekkkk, what a dusty book!"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Baba Yaga, the Horrible",
                                        args![
                                            "Next is for",
                                            "something hot.",
                                            "1 Detonator",
                                            "5 Red Blood,",
                                            "and 10 Burning Hearts."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Baba Yaga, the Horrible",
                                        args!["Ah, and I need", "a 1, 2, or 3 carat", "^ff0000Diamond^000000."],
                                    )?;
                                    ctx.next()?;
                                    ctx.mes("[Baba Yaga, the Horrible]")?;
                                    if l_plus1.clone() == 1 {
                                        ctx.lines(args!["Ok, I am repeating the items.", "Eh, they are..."])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Baba Yaga, the Horrible",
                                            args![
                                                "^ff000020 Grasshopper's Legs,",
                                                "5 Spawn,",
                                                "20 Wing Of Red Bat,",
                                                "10 Sticky Mucus,",
                                                "10 Witched Starsand^000000..."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Baba Yaga, the Horrible",
                                            args![
                                                "^ff000010 Fine Grit,",
                                                "1 Detonator,",
                                                "5 Red Blood,",
                                                "10 Burning Heart^000000 and,",
                                                "a 1, 2, or 3 carat",
                                                "^ff00001 Diamond^000000."
                                            ],
                                        )?;
                                        ctx.var("mos_nowinter").set(Val::from(6))?;
                                        ctx.call(Function::ChangeQuest, vec![Val::from(18070), Val::from(18071)])?;
                                    } else {
                                        ctx.lines(args!["Ok, I am repeating the items.", "Eh, they are..."])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Baba Yaga, the Horrible",
                                            args![
                                                "^ff000020 Grasshopper's Legs,",
                                                "5 Spawn,",
                                                "20 Wing Of Red Bat,",
                                                "10 Witched Starsand^000000..."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Baba Yaga, the Horrible",
                                            args![
                                                "^ff000010 Fine Grit,",
                                                "1 Detonator,",
                                                "5 Red Blood,",
                                                "10 Burning Heart^000000 and,",
                                                "a 1, 2, or 3 carat",
                                                "^ff0000Diamond^000000."
                                            ],
                                        )?;
                                        ctx.var("mos_nowinter").set(Val::from(7))?;
                                        ctx.call(Function::ChangeQuest, vec![Val::from(18070), Val::from(18072)])?;
                                    }
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Baba Yaga, the Horrible",
                                        args!["I am preparing to make it,", "you should get me them quickly. Cough, cough!"],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    if (ctx.var("mos_nowinter").get()? == 6 || ctx.var("mos_nowinter").get()? == 7) {
                                        ctx.lines_as("Baba Yaga, the Horrible", args!["Ehhh... You came back earlier."])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Baba Yaga, the Horrible",
                                            args!["I'm almost prepared..", "Did you get", "all of the items?"],
                                        )?;
                                        ctx.next()?;
                                        let base = Val::from(0).number()?;
                                        runtime::local_set(&mut l_item, &Val::from(base + 0), Val::from(940), false);
                                        runtime::local_set(&mut l_item, &Val::from(base + 1), Val::from(20), false);
                                        runtime::local_set(&mut l_item, &Val::from(base + 2), Val::from(908), false);
                                        runtime::local_set(&mut l_item, &Val::from(base + 3), Val::from(5), false);
                                        runtime::local_set(&mut l_item, &Val::from(base + 4), Val::from(7006), false);
                                        runtime::local_set(&mut l_item, &Val::from(base + 5), Val::from(20), false);
                                        runtime::local_set(&mut l_item, &Val::from(base + 6), Val::from(1061), false);
                                        runtime::local_set(&mut l_item, &Val::from(base + 7), Val::from(10), false);
                                        runtime::local_set(&mut l_item, &Val::from(base + 8), Val::from(7041), false);
                                        runtime::local_set(&mut l_item, &Val::from(base + 9), Val::from(10), false);
                                        runtime::local_set(&mut l_item, &Val::from(base + 10), Val::from(1051), false);
                                        runtime::local_set(&mut l_item, &Val::from(base + 11), Val::from(1), false);
                                        runtime::local_set(&mut l_item, &Val::from(base + 12), Val::from(990), false);
                                        runtime::local_set(&mut l_item, &Val::from(base + 13), Val::from(5), false);
                                        runtime::local_set(&mut l_item, &Val::from(base + 14), Val::from(7097), false);
                                        runtime::local_set(&mut l_item, &Val::from(base + 15), Val::from(10), false);
                                        l_size = Val::from(l_item.len() as i32);
                                        if ctx.var("mos_nowinter").get()? == 6 {
                                            let base = l_size.clone().number()?;
                                            runtime::local_set(&mut l_item, &Val::from(base + 0), Val::from(938), false);
                                            runtime::local_set(&mut l_item, &Val::from(base + 1), Val::from(10), false);
                                            l_size = (l_size.clone() + Val::from(2));
                                        }
                                        l_i = Val::from(0);
                                        'l1: loop {
                                            if !(runtime::op(&l_i.clone(), "<", &l_size.clone())?.is_true()) {
                                                break 'l1;
                                            }
                                            'b1: {
                                                if runtime::op(
                                                    &ctx.call(Function::CountItem, vec![runtime::local_get(&l_item, &l_i.clone(), false)])?,
                                                    "<",
                                                    &runtime::local_get(&l_item, &(l_i.clone() + Val::from(1)), false),
                                                )?
                                                .is_true()
                                                {
                                                    break 'l1;
                                                }
                                            }
                                            l_i = (l_i.clone() + Val::from(2));
                                        }
                                        if (runtime::op(&l_i.clone(), ">=", &l_size.clone())?.is_true()
                                            && ((ctx.call(Function::CountItem, vec![Val::from(730)])?.is_true()
                                                || ctx.call(Function::CountItem, vec![Val::from(731)])?.is_true())
                                                || ctx.call(Function::CountItem, vec![Val::from(732)])?.is_true()))
                                        {
                                            l_i = Val::from(0);
                                            'l2: loop {
                                                if !(runtime::op(&l_i.clone(), "<", &l_size.clone())?.is_true()) {
                                                    break 'l2;
                                                }
                                                'b2: {
                                                    ctx.call(
                                                        Function::DelItem,
                                                        vec![
                                                            runtime::local_get(&l_item, &l_i.clone(), false),
                                                            runtime::local_get(&l_item, &(l_i.clone() + Val::from(1)), false),
                                                        ],
                                                    )?;
                                                }
                                                l_i = (l_i.clone() + Val::from(2));
                                            }
                                            if ctx.call(Function::CountItem, vec![Val::from(730)])?.is_true() {
                                                ctx.call(Function::DelItem, vec![Val::from(730), Val::from(1)])?;
                                            } else if ctx.call(Function::CountItem, vec![Val::from(731)])?.is_true() {
                                                ctx.call(Function::DelItem, vec![Val::from(731), Val::from(1)])?;
                                            } else if ctx.call(Function::CountItem, vec![Val::from(732)])?.is_true() {
                                                ctx.call(Function::DelItem, vec![Val::from(732), Val::from(1)])?;
                                            }
                                            if ctx.var("mos_nowinter").get()? == 6 {
                                                ctx.call(Function::ChangeQuest, vec![Val::from(18071), Val::from(18073)])?;
                                            } else {
                                                ctx.call(Function::ChangeQuest, vec![Val::from(18072), Val::from(18073)])?;
                                            }
                                            ctx.var("mos_nowinter").set(Val::from(8))?;
                                            ctx.lines_as(
                                                "Baba Yaga, the Horrible",
                                                args![
                                                    "Right, you got them all.",
                                                    "With this amount.. For some time...",
                                                    "No, to make the Secret Medicine,",
                                                    "These are enough, cough."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as("Baba Yaga, the Horrible", args!["Let me see, we have all", "that we need..."])?;
                                            ctx.next()?;
                                            ctx.lines_as("Baba Yaga, the Horrible", args!["Ehhhh...", "Huk!"])?;
                                            ctx.next()?;
                                            ctx.lines_as("Baba Yaga, the Horrible", args!["Cough, cough, cough!", "Eh... I mean..."])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Baba Yaga, the Horrible",
                                                args![
                                                    "Have you heard the story of",
                                                    "a dragon sleeping for a long time?",
                                                    "Next thing that I need is",
                                                    "something from that dragon, kkkk."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Baba Yaga, the Horrible",
                                                args![
                                                    "You can find a funny-shaped bottle",
                                                    "in the dragon's lair.",
                                                    "It's a gourd bottle that can",
                                                    "contain people's speech.",
                                                    "I must have this."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Baba Yaga, the Horrible",
                                                args![
                                                    "I don't know how strong you are,",
                                                    "but I would like to recommend that",
                                                    "you avoid fighting that dragon, Kaaaaak~"
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Baba Yaga, the Horrible",
                                                args![
                                                    "Then, release me.",
                                                    "I want to do something,",
                                                    "...but, my life is the priority. Yes, it is... Cough, cough!"
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        ctx.lines_as("Baba Yaga, the Horrible", args!["I think", "I need more, cough, cough!"])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Baba Yaga, the Horrible",
                                            args![
                                                "I am not able to do",
                                                "anything without them.",
                                                "I am repeating the items in the list,",
                                                "you should get them all, kaaaaak!"
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Baba Yaga, the Horrible",
                                            args!["^ff000020 Grasshopper's Legs,", "5 Spawn,", "20 Wings Of Red Bat"],
                                        )?;
                                        if ctx.var("mos_nowinter").get()? == 6 {
                                            ctx.mes("10 Sticky Mucus,")?;
                                        }
                                        ctx.mes("10 Witch Starsand^000000...")?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Baba Yaga, the Horrible",
                                            args![
                                                "^ff000010 Fine Grit,",
                                                "1 Detonator,",
                                                "5 Red Blood",
                                                "10 Burning Hearts^000000 and",
                                                "a 1, 2, or 3 carat",
                                                "^ff0000Diamond^000000."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Baba Yaga, the Horrible",
                                            args!["Cough, cough.", "Hu, I talked so much", "that it hurts me, Kaaaak~"],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else {
                                        if ctx.var("mos_nowinter").get()? == 8 {
                                            ctx.lines_as(
                                                "Baba Yaga, the Horrible",
                                                args![
                                                    "It is very, very hard to return alive",
                                                    "from a lair of a dragon",
                                                    "without strength and courage."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Baba Yaga, the Horrible",
                                                args![
                                                    "Unless you want to be killed,",
                                                    "you'd better forget",
                                                    "the magic gourd bottle, cough!"
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        } else {
                                            if ctx.var("mos_nowinter").get()? == 9 {
                                                if ctx.call(Function::CountItem, vec![Val::from(7761)])?.is_true() {
                                                    ctx.lines_as(
                                                        "Baba Yaga, the Horrible",
                                                        args!["Akkkk?", "Did you really bring it", "from the lair of the dragon...?"],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Baba Yaga, the Horrible",
                                                        args![
                                                            "You are superbly great.",
                                                            "I don't know whether",
                                                            "you killed the dragon or",
                                                            "you just stole it from the dragon,",
                                                            "either way I don't care."
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Baba Yaga, the Horrible",
                                                        args![
                                                            "It is great that",
                                                            "you've returned alive",
                                                            "from the lair of the dragon, cough!"
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Baba Yaga, the Horrible",
                                                        args![
                                                            "Here, I told you before that",
                                                            "this gourd bottle can",
                                                            "contain people's speech"
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Baba Yaga, the Horrible",
                                                        args![
                                                            "The next thing that we have to do",
                                                            "is to receive sincere speeches from",
                                                            "people with this bottle."
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Baba Yaga, the Horrible",
                                                        args![
                                                            "The important thing is that",
                                                            "you cannot force them to speak.",
                                                            "For example, you cannot tell them",
                                                            "to say 'I don't want winter to come.'"
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Baba Yaga, the Horrible",
                                                        args![
                                                            "You should have them",
                                                            "speak naturally.",
                                                            "Ah, one more thing that",
                                                            "you have to take care to remember."
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Baba Yaga, the Horrible",
                                                        args![
                                                            "You need to have the",
                                                            "voices of three people.",
                                                            "From a child, a young person and a",
                                                            "middle aged person."
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Baba Yaga, the Horrible",
                                                        args![
                                                            "The magic is not just witchcraft",
                                                            "This type of magic is effective",
                                                            "only by embodying the desires of people."
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Baba Yaga, the Horrible", args!["Take care of yourself, cough, cough."])?;
                                                    ctx.var("mos_nowinter").set(Val::from(10))?;
                                                    ctx.call(Function::ChangeQuest, vec![Val::from(18074), Val::from(18075)])?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                }
                                                ctx.lines_as(
                                                    "Baba Yaga, the Horrible",
                                                    args![
                                                        "It is very, very hard to return alive",
                                                        "from a lair of a dragon",
                                                        "without strength and courage."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Baba Yaga, the Horrible",
                                                    args![
                                                        "Unless you want to be killed",
                                                        "you'd better forget",
                                                        "the magic gourd bottle, cough!"
                                                    ],
                                                )?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            } else {
                                                if ctx.var("mos_nowinter").get()? == 10 {
                                                    ctx.lines_as(
                                                        "Baba Yaga, the Horrible",
                                                        args![
                                                            "To make the magic,",
                                                            "I need you to contain",
                                                            "the wish phrase 'I don't want winter to come.'",
                                                            "from the three people, kkk."
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Baba Yaga, the Horrible",
                                                        args![
                                                            "But, one thing that you need to remember",
                                                            "is, do not force them to say it!"
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Baba Yaga, the Horrible",
                                                        args![
                                                            "Next,",
                                                            "you should contain the word from",
                                                            "a child, a young man and a middle aged man."
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Baba Yaga, the Horrible",
                                                        args!["Well, take care of yourself.", "Cough, cough, cough!"],
                                                    )?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                } else {
                                                    if (ctx.var("mos_nowinter").get()?.number()? > 10
                                                        && ctx.var("mos_nowinter").get()?.number()? < 14)
                                                    {
                                                        ctx.lines_as(
                                                            "Baba Yaga, the Horrible",
                                                            args!["My place has been revealed, cough, cough.", "because of you!"],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Baba Yaga, the Horrible",
                                                            args![
                                                                "What did you do outside?",
                                                                "The soldiers of the Csar are",
                                                                "surrounding here so that",
                                                                "I cannot go out."
                                                            ],
                                                        )?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    } else {
                                                        if ctx.var("mos_nowinter").get()? == 14 {
                                                            ctx.lines_as(
                                                                "Baba Yaga, the Horrible",
                                                                args!["Did you do", "what I told", "you to do?"],
                                                            )?;
                                                            ctx.next()?;
                                                            if !(ctx.call(Function::CountItem, vec![Val::from(7761)])?.is_true()) {
                                                                ctx.lines_as(
                                                                    "Baba Yaga, the Horrible",
                                                                    args![
                                                                        "Where is the Gourd Bottle?",
                                                                        "I can't do anything without it.",
                                                                        "Cough, cough!"
                                                                    ],
                                                                )?;
                                                                ctx.close_window()?;
                                                                return Err(Stop::End);
                                                            }
                                                            ctx.call(Function::DelItem, vec![Val::from(7761), Val::from(1)])?;
                                                            ctx.lines_as(
                                                                "Baba Yaga, the Horrible",
                                                                args![
                                                                    "Anyway, give me the bottle and",
                                                                    "think about",
                                                                    "what to do, cough!"
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "Baba Yaga, the Horrible",
                                                                args![
                                                                    "This bottle will be",
                                                                    "used later and now,",
                                                                    "the last step remains",
                                                                    "to complete the work.",
                                                                    "Pass me the book."
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "Baba Yaga, the Horrible",
                                                                args![
                                                                    "My abilities are deteriorating",
                                                                    "and magic should only be",
                                                                    "done by a single person.",
                                                                    "So, you must compound the materials yourself,",
                                                                    "kaaaaak!"
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "Baba Yaga, the Horrible",
                                                                args![
                                                                    "I know it is hard to do,",
                                                                    "but it is the magic that",
                                                                    "embodies and reveals",
                                                                    "people's desires.",
                                                                    "It is useless unless you put your",
                                                                    "full effort into it, cough!"
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "Baba Yaga, the Horrible",
                                                                args![
                                                                    "Ok, you see that book??",
                                                                    "The letters on the book ",
                                                                    "are illegible,",
                                                                    "But you'll be able to read it 'cuz",
                                                                    "I added comments under the lines for you."
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "Baba Yaga, the Horrible",
                                                                args![
                                                                    "Read carefully and just follow what",
                                                                    "it tells you.",
                                                                    "The magic is complete.",
                                                                    "Do your best,",
                                                                    "Cough!"
                                                                ],
                                                            )?;
                                                            ctx.var("mos_nowinter").set(Val::from(15))?;
                                                            ctx.close_window()?;
                                                            return Err(Stop::End);
                                                        } else if (ctx.var("mos_nowinter").get()? == 15
                                                            || ctx.var("mos_nowinter").get()? == 16)
                                                        {
                                                            ctx.lines_as(
                                                                "Baba Yaga, the Horrible",
                                                                args!["Magic does not come from", "something mysterious."],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "Baba Yaga, the Horrible",
                                                                args![
                                                                    "It is your effort",
                                                                    "that makes magic.",
                                                                    "There is a saying,",
                                                                    "if you really want it,",
                                                                    "it will happen, cough!"
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "Baba Yaga, the Horrible",
                                                                args![
                                                                    "Here, according to the book",
                                                                    "put the materials in the pot and",
                                                                    "boil them for some time."
                                                                ],
                                                            )?;
                                                            ctx.close_window()?;
                                                            return Err(Stop::End);
                                                        } else if ctx.var("mos_nowinter").get()? == 17 {
                                                            ctx.lines_as(
                                                                "Baba Yaga, the Horrible",
                                                                args!["Ehmmm...", "Don't you think that", "something is wrong?"],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "Baba Yaga, the Horrible",
                                                                args!["Well, try it again.", "We still have enough materials."],
                                                            )?;
                                                            ctx.var("mos_nowinter").set(Val::from(16))?;
                                                            ctx.close_window()?;
                                                            return Err(Stop::End);
                                                        } else if ctx.var("mos_nowinter").get()? == 18 {
                                                            ctx.lines_as("Baba Yaga, the Horrible", args!["Let me see, hmm!"])?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "Baba Yaga, the Horrible",
                                                                args![
                                                                    "Hmm, this might be your first time, but",
                                                                    "this is good.",
                                                                    "You may be talented, kkkk."
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "Baba Yaga, the Horrible",
                                                                args![
                                                                    "Now then, we are in the final step.",
                                                                    "I'll put the medicine in the bottle,",
                                                                    "you break it by dropping it",
                                                                    "in the center of the town."
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "Baba Yaga, the Horrible",
                                                                args![
                                                                    "If you do this, what the villagers want",
                                                                    "will be happening,",
                                                                    "forever."
                                                                ],
                                                            )?;
                                                            ctx.call(Function::GetItem, vec![Val::from(7765), Val::from(1)])?;
                                                            ctx.var("mos_nowinter").set(Val::from(19))?;
                                                            ctx.call(Function::ChangeQuest, vec![Val::from(18077), Val::from(18078)])?;
                                                            ctx.close_window()?;
                                                            return Err(Stop::End);
                                                        } else if ctx.var("mos_nowinter").get()? == 19 {
                                                            ctx.lines_as(
                                                                "Baba Yaga, the Horrible",
                                                                args![
                                                                    "You just drop the bottle",
                                                                    "in the middle of the village",
                                                                    "to break it."
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "Baba Yaga, the Horrible",
                                                                args![
                                                                    "And then,",
                                                                    "It doesn't matter to me",
                                                                    "if you tell the whole town or",
                                                                    "report to the Csar to be praised."
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "Baba Yaga, the Horrible",
                                                                args!["But, don't come back to me, cough, cough."],
                                                            )?;
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
                ctx.lines_as(
                    "Baba Yaga, the Horrible",
                    args!["Hey,", "I told you", "to never come back, cough."],
                )?;
                ctx.next()?;
                ctx.lines_as("Baba Yaga, the Horrible", args!["Stop disturbing me."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            BabaYagaTheHorribleStep::SBY1 => {
                ctx.lines_as("Baba Yaga, the Horrible", args!["Ah, do you accept?", "Thank you."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Baba Yaga, the Horrible",
                    args![
                        "Well, I thank you for your kindness.",
                        "Let's cooperate and",
                        "make my Secret Medicine."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Baba Yaga, the Horrible", args!["Let me see... Ah..."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Baba Yaga, the Horrible",
                    args![
                        "It is not used often.",
                        "I don't remember",
                        "where it is.",
                        "Could you please come back later?"
                    ],
                )?;
                ctx.var("mos_nowinter").set(Val::from(5))?;
                ctx.call(Function::SetQuest, vec![Val::from(18070)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            BabaYagaTheHorribleStep::OnTouch => {
                if (ctx.var("mos_nowinter").get()? == 0 || ctx.var("mos_nowinter").get()? == 1) {
                    ctx.mes("[Baba Yaga, the Horrible]")?;
                    if (ctx.call(Function::CountItem, vec![Val::from(7762)])?.number()? > 39 && ctx.var("BaseLevel").get()?.number()? > 59)
                    {
                        ctx.var("mos_nowinter").set(Val::from(2))?;
                        ctx.lines(args!["Who the hell are you.", "You want me to kick your ass!"])?;
                        ctx.next()?;
                        ctx.lines_as("Baba Yaga, the Horrible", args!["No. They are..."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Baba Yaga, the Horrible",
                            args!["You have our Yaga Pestles!", "You have so many of them..."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Baba Yaga, the Horrible", args!["Who, who are you?"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    if ctx.var("mos_nowinter").get()? == 0 {
                        ctx.var("mos_nowinter").set(Val::from(1))?;
                        if (ctx.call(Function::CountItem, vec![Val::from(7762)])?.number()? > 39
                            && ctx.var("BaseLevel").get()?.number()? < 60)
                        {
                            ctx.lines(args![
                                "Hohoho~",
                                "You cannot beat my friends!",
                                "I don't know where you got",
                                "those Yaga Pestles, but",
                                "give them back to me!"
                            ])?;
                            ctx.call(
                                Function::DelItem,
                                vec![Val::from(7762), ctx.call(Function::Rand, vec![Val::from(1), Val::from(20)])?],
                            )?;
                        } else {
                            ctx.lines(args!["Who the hell are you?", "You want me to kick your ass!"])?;
                        }
                    } else if (ctx.call(Function::CountItem, vec![Val::from(7762)])?.number()? > 39
                        && ctx.var("BaseLevel").get()?.number()? < 60)
                    {
                        ctx.lines(args!["You, chicken,", "I need to kick your ass more."])?;
                        ctx.next()?;
                        ctx.mes("[Baba Yaga, the Horrible]")?;
                        if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                            ctx.lines(args!["I will bewitch", "and enslave you.", "Come here, Heeee!"])?;
                        } else {
                            ctx.lines(args![
                                "Recently, I have no appetite.",
                                "But you will be",
                                "a good appetizer.",
                                "Come here, Heeeeeeeee."
                            ])?;
                        }
                    } else {
                        ctx.lines(args![
                            "I need to kick your ass more.",
                            "Come here",
                            "to play a game",
                            "Heeeeeeeeeeee"
                        ])?;
                    }
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("mosk_dun02"), Val::from(53), Val::from(217)])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn baba_yaga_the_horrible(ctx: &Ctx) -> Script {
    baba_yaga_the_horrible_run(ctx, BabaYagaTheHorribleStep::Start, Vec::new()).map(|_| ())
}

pub fn baba_yaga_the_horrible_ontouch(ctx: &Ctx) -> Script {
    baba_yaga_the_horrible_run(ctx, BabaYagaTheHorribleStep::OnTouch, Vec::new()).map(|_| ())
}

fn book_russia_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("mos_nowinter").get()? == 15 {
        ctx.lines(args![
            "-Beside the pot,",
            "There is an unusually large book.",
            "The letters are in disarray and",
            "some of them, I can't understand.",
            "The comments by Baba Yaga",
            "help me read some of them.-"
        ])?;
        ctx.next()?;
        ctx.mes("-Ok, time to read it.-")?;
        ctx.next()?;
        ctx.lines(args![
            "...Therefore, this spell is",
            "particularly good among our mighty ones.",
            "Usually, the magic is considered",
            "as a contract with evil.",
            "But, the following is not the power",
            "of Hell but of Nature and the spirit of humans.",
            "I already told you that this spell",
            "has 2 features as follows.",
            "First, it changes natural phenomenon.",
            "Unlike the idea from foolish scholars,",
            "this world is composed of invisible and precise orders.",
            "The things that people call",
            "miracles are made by stimulating",
            "them or simply altering their arrangement.",
            "Most of the spells for weather",
            "changing are included in them and",
            "they require a high level of comprehension.",
            "The orders of this world are not simply aligned.",
            "It is almost impossible to observe",
            "the smallest and detailed part of",
            "them and completely understand their structure",
            "However, it is not that difficult",
            "to repeat the spells that are already made.",
            "This thick spell book is for that purpose."
        ])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["That? That's just the preface?!"],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "...The second feature is",
            "This spell book represents the desire of mankind.",
            "Their spirit is so great,",
            "sometimes flames in their hearts",
            "are embodied and become reality.",
            "Now, practice one of the spells for weather.",
            "You have to go to a damp and deserted place,",
            "and have a pot of melting materials",
            "along with the spirit of the caster.",
            "The next spells should be conducted",
            "through the exact process.",
            "If you realize that something is",
            "wrong, you'd MUST stop immediately.",
            "To prevent the ordinary from",
            "abusing the magic,",
            "the narratives of the materials are metaphors,",
            "but, whoever understands this can",
            "read everything in it."
        ])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "It just accounts for the spell.",
                "What a long preface.",
                "I'll mark this spot.",
                "so I can easily find the mixing",
                "process next time..."
            ],
        )?;
        ctx.var("mos_nowinter").set(Val::from(16))?;
        ctx.next()?;
        ctx.lines(args![
            "^0000ffSeasonal Magic^000000:^ff0000Eternal Summer^000000",
            "First of all, you need to put and",
            "mix the first 3 things in a pre-heated pot.",
            "The extended arms of those who stalk at night,",
            "The rotten mixture of plants and animals,",
            "and the limbs of one going forward,",
            "who wants to run backward.",
            "This first stage should be complete.",
            "Just after this stage, the mixture",
            "will barely show any alteration.",
            "After this, you must add the liquid",
            "that aids all life and stir 20 times.",
            "At this point you will certainly",
            "see its color and smell changing.",
            "Remember that if you realize that",
            "something is wrong, you must stop and start over.",
            "Following these precise steps are",
            "most important when working on magic.",
            "Next, 2 things must be put in the pot.",
            "First, objects that are seen when",
            "looking at the night sky.",
            "Second the pretty mass of earth,",
            "slipping through even when watered.",
            "You should check if it's smell has changed.",
            "Then stir it well 15 times.",
            "It's color will change.",
            "The final ingredients will be those",
            "that contain heat.",
            "You must follow the order.",
            "Stones that have the attribute of sun,",
            "brains of a marine sphere,",
            "the organ of passion,",
            "and a stone that cannot be cut.",
            "Once the final ingredient is placed in the pot,",
            "It will be over.",
            "The solution must be sprinkled at",
            "the place where you want to affect,",
            "the changing of the seasons."
        ])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["I'll mark this page so that I", "know where to read next time."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("mos_nowinter").get()? == 16 {
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from("Read from the preface.:Read from the marked page.")],
        )?) == 1
        {
            ctx.lines(args![
                "...Therefore, this spell is",
                "particularly good among our mighty ones.",
                "Usually, the magic is considered",
                "as a contract with evil.",
                "But, the following is not the power",
                "of Hell but of Nature and the spirit of humans.",
                "I already told you that this spell",
                "has 2 features as follows.",
                "First, it changes natural phenomenon.",
                "Unlike the idea from foolish scholars,",
                "this world is composed of invisible and precise orders.",
                "The things that people call",
                "miracles are made by stimulating",
                "them or simply altering their arrangement.",
                "Most of the spells for weather",
                "changing are included in them and",
                "they require a high level of comprehension.",
                "The orders of this world are not simply aligned.",
                "It is almost impossible to observe",
                "the smallest and detailed part of",
                "them and completely understand their structure",
                "However, it is not that difficult",
                "to repeat the spells that are already made.",
                "This thick spell book is for that purpose."
            ])?;
            ctx.next()?;
            ctx.lines(args![
                "...The second feature is",
                "This spell book represents the desire of mankind.",
                "Their spirit is so great,",
                "sometimes flames in their hearts",
                "are embodied and become reality.",
                "Now, practice one of the spells for weather.",
                "You have to go to a damp and deserted place,",
                "and have a pot of melting materials",
                "along with the spirit of the caster.",
                "The next spells should be conducted",
                "through the exact process.",
                "If you realize that something is",
                "wrong, you'd MUST stop immediately.",
                "To prevent the ordinary from",
                "abusing the magic,",
                "the narratives of the materials are metaphors,",
                "but, whoever understands this can",
                "read everything in it."
            ])?;
            ctx.next()?;
            ctx.lines(args![
                "^0000ffSeasonal Magic^000000:^ff0000Eternal Summer^000000",
                "First of all, you need to put and",
                "mix the first 3 things in a pre-heated pot.",
                "The extended arms of those who stalk at night,",
                "The rotten mixture of plants and animals,",
                "and the limbs of one going forward,",
                "who wants to run backward.",
                "This first stage should be complete.",
                "Just after this stage, the mixture",
                "will barely show any alteration.",
                "After this, you must add the liquid",
                "that aids all life and stir 20 times.",
                "At this point you will certainly",
                "see its color and smell changing.",
                "Remember that if you realize that",
                "something is wrong, you must stop and start over.",
                "Following these precise steps are",
                "most important when working on magic.",
                "Next, 2 things must be put in the pot.",
                "First, objects that are seen when",
                "looking at the night sky.",
                "Second the pretty mass of earth,",
                "slipping through even when watered.",
                "You should check if it's smell has changed.",
                "Then stir it well 15 times.",
                "It's color will change.",
                "The final ingredients will be those",
                "that contain heat.",
                "You must follow the order.",
                "Stones that have the attribute of sun,",
                "brains of a marine sphere,",
                "the organ of passion,",
                "and a stone that cannot be cut.",
                "Once the final ingredient is placed in the pot,",
                "It will be over.",
                "The solution must be sprinkled at",
                "the place where you want to affect,",
                "the changing of the seasons."
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines(args![
            "^0000ffSeasonal Magic^000000:^ff0000Eternal Summer^000000",
            "First of all, you need to put and",
            "mix the first 3 things in a pre-heated pot.",
            "The extended arms of those who stalk at night,",
            "The rotten mixture of plants and animals,",
            "and the limbs of one going forward,",
            "who wants to run backward.",
            "This first stage should be complete.",
            "Just after this stage, the mixture",
            "will barely show any alteration.",
            "After this, you must add the liquid",
            "that aids all life and stir 20 times.",
            "At this point you will certainly",
            "see its color and smell changing.",
            "Remember that if you realize that",
            "something is wrong, you must stop and start over.",
            "Following these precise steps are",
            "most important when working on magic.",
            "Next, 2 things must be put in the pot.",
            "First, objects that are seen when",
            "looking at the night sky.",
            "Second the pretty mass of earth,",
            "slipping through even when watered.",
            "You should check if it's smell has changed.",
            "Then stir it well 15 times.",
            "It's color will change.",
            "The final ingredients will be those",
            "that contain heat.",
            "You must follow the order.",
            "Stones that have the attribute of sun,",
            "brains of a marine sphere,",
            "the organ of passion,",
            "and a stone that cannot be cut.",
            "Once the final ingredient is placed in the pot,",
            "It will be over.",
            "The solution must be sprinkled at",
            "the place where you want to affect,",
            "the changing of the seasons."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("mos_nowinter").get()?.number()? > 14 {
        ctx.lines(args![
            "-This book is the magic book that",
            "helped me make the medicine, but,",
            "I never want to read it again.-"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.mes("-It's an unusually large book laid down, open.-")?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn book_russia(ctx: &Ctx) -> Script {
    book_russia_body(ctx, Vec::new()).map(|_| ())
}

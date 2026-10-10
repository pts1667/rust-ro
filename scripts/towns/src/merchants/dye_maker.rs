#![allow(
    unused_imports,
    unused_labels,
    unused_assignments,
    unused_mut,
    unused_parens,
    unused_variables,
    unreachable_code
)]

use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum JavaDullihanStep {
    Start,
    ShowRecipe,
}

fn set_recipe(items: &mut Vec<Val>, counts: &mut Vec<Val>, recipe: &[(i32, i32)]) {
    for (i, &(item, count)) in recipe.iter().enumerate() {
        runtime::local_set(items, &Val::from(i as i32), Val::from(item), false);
        runtime::local_set(counts, &Val::from(i as i32), Val::from(count), false);
    }
}

fn java_dullihan_run(ctx: &Ctx, mut step: JavaDullihanStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_cost: i32 = 0;
    let mut l_count: Vec<Val> = Vec::new();
    let mut l_dyestuff: i32 = 0;
    let mut l_item: Vec<Val> = Vec::new();
    'machine: loop {
        match step {
            JavaDullihanStep::Start => {
                if ctx.var("MaxWeight").get()?.number()? - ctx.var("Weight").get()?.number()? < 200
                    || ctx.call(Function::CheckWeight, args![1201, 1])? == 0
                {
                    ctx.lines(args![
                        "- Wait a moment! -",
                        "- Currently you're carrying -",
                        "- too many items with you. -",
                        "- Please come back later -",
                        "- after you put some items into kafra storage. -"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Dye Maker Java Dullihan",
                    args!["Wow...", "Such a nice day. Days like this are perfect to make dyes."],
                )?;
                ctx.next()?;
                'b1: {
                    let subject1 = ctx.menu(&["Talk", "Make Dyestuffs", "Cancel"])?;
                    let mut matched1 = false;
                    if !matched1 && subject1 == 0 {
                        matched1 = true;
                    }
                    if matched1 {
                        ctx.lines_as("Dye Maker Java Dullihan", args!["Erm, I don't really have much to say to you. But, if you would like me to tell you about my life, I can do that. It's a little long and boring, but would you like to listen?"])?;
                        ctx.next()?;
                        if ctx.menu(&["Listen", "Don't Listen"])? == 0 {
                            ctx.lines_as("Dye Maker Java Dullihan", args!["As long as I can remember, my father has been making dyes. He used to spend countless hours making dyes of different colors. Even when my mother passed away, he never stopped."])?;
                            ctx.next()?;
                            ctx.lines_as("Dye Maker Java Dullihan", args!["At one point in my life, I became rebellious and ran away from home. I didn't want to be stuck to the family business and wanted to try other things."])?;
                            ctx.next()?;
                            ctx.lines_as("Dye Maker Java Dullihan", args!["Anyways...", "I eventually wound up back home to carry on the family tradition, making dyes for 15 years already. I guess it was really in my blood."])?;
                            ctx.next()?;
                            ctx.lines_as("Dye Maker Java Dullihan", args!["Something I realized these days is that now I can understand my father. Why my father devoted everything to making dyes..."])?;
                            ctx.next()?;
                            ctx.lines_as("Dye Maker Java Dullihan", args!["In the middle of all that tedious and hard work, he probably felt the magic of those colors passing on so many dreams for other people."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Dye Maker Java Dullihan",
                                args!["It was probably for those dreams that he tried so hard to make dyes for his customers..."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Dye Maker Java Dullihan", args!["Heh... How do I know this? Well, that's the way I feel right now. Since a couple years ago, I've been able to hear what the colors were saying..."])?;
                            ctx.next()?;
                            ctx.lines_as("Dye Maker Java Dullian", args!["Just watch. Someday, I'm going to succeed in making the color my father wanted to, but never got the chance to make."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as(
                            "Dye Maker Java Dullihan",
                            args![
                                "Ahahahaha...",
                                "Well, I guess no one would want to listen to a measly dyemaker's story anyway. Hahaha...."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    if !matched1 && subject1 == 1 {
                        matched1 = true;
                    }
                    if matched1 {
                        ctx.lines_as("Dye Maker Java Dullihan", args!["Great! If you want it, I'll make it for you. I promise I'll make you the color you want. But what color dye would you like to make? The fee is different depending on the color."])?;
                        ctx.next()?;
                        ctx.lines_as("Dye Maker Java Dullihan", args!["Why, do you ask? Just remember that there are colors that are easy to make, and those that require more effort and work. Please don't think the fee is too expensive."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Dye Maker Java Dullihan",
                            args!["The color of the dye is special, made with all my heart and soul."],
                        )?;
                        ctx.next()?;
                        match ctx.menu(&[
                            "Scarlet Dyestuffs",
                            "Lemon Dyestuffs",
                            "Cobaltblue Dyestuffs",
                            "Darkgreen Dyestuffs",
                            "Orange Dyestuffs",
                            "Violet Dyestuffs",
                            "White Dyestuffs",
                            "Black Dyestuffs",
                            "Cancel",
                        ])? {
                            0 => {
                                java_dullihan_run(ctx, JavaDullihanStep::ShowRecipe, args![1, 1])?;
                            }
                            1 => {
                                java_dullihan_run(ctx, JavaDullihanStep::ShowRecipe, args![2, 1])?;
                            }
                            2 => {
                                java_dullihan_run(ctx, JavaDullihanStep::ShowRecipe, args![3, 2])?;
                            }
                            3 => {
                                java_dullihan_run(ctx, JavaDullihanStep::ShowRecipe, args![4, 3])?;
                            }
                            4 => {
                                java_dullihan_run(ctx, JavaDullihanStep::ShowRecipe, args![5, 4])?;
                            }
                            5 => {
                                java_dullihan_run(ctx, JavaDullihanStep::ShowRecipe, args![6, 4])?;
                            }
                            6 => {
                                java_dullihan_run(ctx, JavaDullihanStep::ShowRecipe, args![7, 4])?;
                            }
                            7 => {
                                java_dullihan_run(ctx, JavaDullihanStep::ShowRecipe, args![8, 4])?;
                            }
                            8 => {
                                ctx.lines_as(
                                    "Dye Maker Java Dullihan",
                                    args!["Eeeehhhh!! What's this? You change your mind now!? So disappointing..."],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                    if !matched1 && subject1 == 2 {
                        matched1 = true;
                    }
                    if matched1 {
                        ctx.lines_as("Dye Maker Java Dullihan", args!["I'm not bragging or anything. But I have the skills to make dyestuff. If you ever need dyestuff, please come to me. I'll make them for you at a reasonable price."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                step = JavaDullihanStep::ShowRecipe;
                continue 'machine;
            }
            JavaDullihanStep::ShowRecipe => {
                ctx.mes("[Dye Maker Java Dullihan]")?;
                match runtime::arg(&args, 0, Val::from(0)).number()? {
                    1 => {
                        ctx.mes("Mmm... I need 30 Red Herbs, 1 Counteragent, and 1 Empty Bottle to make Red Dyestuffs. The fee is only 3000 zeny to make it.")?;
                        set_recipe(&mut l_item, &mut l_count, &[(507, 30), (973, 1), (713, 1)]);
                        l_cost = 3000;
                        l_dyestuff = 975;
                    }
                    2 => {
                        ctx.mes("Mmm... I need 30 Yellow Herbs, 1 Couneragent, and 1 Empty Bottle to make Lemon Dyestuffs. The fee is only 3000 zeny to make it.")?;
                        set_recipe(&mut l_item, &mut l_count, &[(508, 30), (973, 1), (713, 1)]);
                        l_cost = 3000;
                        l_dyestuff = 976;
                    }
                    3 => {
                        ctx.mes("Mmm... I need 20 Blue Herbs, 1 Counteragent, and 1 Empty Bottle to make Cobaltblue Dyestuff. It's hard to use the Blue Herb, so the fee is going to be 3500 zeny.")?;
                        set_recipe(&mut l_item, &mut l_count, &[(510, 20), (973, 1), (713, 1)]);
                        l_cost = 3500;
                        l_dyestuff = 978;
                    }
                    4 => {
                        ctx.mes("Mmm... I need 5 Blue Herbs, 20 Green Herbs, 20 Yellow Herbs, 1 Counteragent, 1 Mixture, and 1 Empty Bottle to make Darkgreen Dyestuffs. Don't get all of the materials confused. The fee is only 5000 zeny.")?;
                        set_recipe(
                            &mut l_item,
                            &mut l_count,
                            &[(510, 5), (511, 20), (508, 20), (974, 1), (973, 1), (713, 1)],
                        );
                        l_cost = 5000;
                        l_dyestuff = 979;
                    }
                    5 => {
                        ctx.mes("Mmm... I need 20 Red Herbs, 20 Yellow Herbs, 1 Counteragent, 1 Mixture, and 1 Empty Bottle to make Orange Dyestuff. The fee is going to be 5000 zeny.")?;
                        set_recipe(&mut l_item, &mut l_count, &[(507, 20), (508, 20), (974, 1), (973, 1), (713, 1)]);
                        l_cost = 5000;
                        l_dyestuff = 980;
                    }
                    6 => {
                        ctx.mes("Mmm... I need 10 Blue Herbs, 30 Red Herbs, 1 Counteragent, 1 Mixture, and 1 Empty Bottle to make Violet Dyestuffs. The fee will be 5000 zeny.")?;
                        set_recipe(&mut l_item, &mut l_count, &[(510, 10), (507, 30), (974, 1), (973, 1), (713, 1)]);
                        l_cost = 5000;
                        l_dyestuff = 981;
                    }
                    7 => {
                        ctx.mes("Mmm... I need 30 White Herbs, 1 Counteragent, and 1 Empty bottle to make White Dyestuffs. The fee will be 3000 zeny.")?;
                        set_recipe(&mut l_item, &mut l_count, &[(509, 30), (973, 1), (713, 1)]);
                        l_cost = 3000;
                        l_dyestuff = 982;
                    }
                    8 => {
                        ctx.mes("Mmm... I need 30 of each Red, Yellow, and Green Herb, 5 Blue Herbs, 1 Counteragent, 1 Mixture, and 1 Empty Bottle. The process takes longer and more effort than the others, so it is going to be 7000 zeny.")?;
                        set_recipe(
                            &mut l_item,
                            &mut l_count,
                            &[(507, 30), (508, 30), (511, 30), (510, 5), (974, 1), (973, 1), (713, 1)],
                        );
                        l_cost = 7000;
                        l_dyestuff = 983;
                    }
                    _ => {}
                }
                ctx.next()?;
                ctx.mes("[Dye Maker Java Dullihan]")?;
                match runtime::arg(&args, 1, Val::from(0)).number()? {
                    1 => {
                        ctx.mes("Ah! I think you would have everything ready. Would you like to start the process?")?;
                    }
                    2 => {
                        ctx.mes("Okay! I believe you would have everything ready. Would you like to start the process?")?;
                    }
                    3 => {
                        ctx.mes("Okay! I believe you would have everything ready. Shall we begin the process?")?;
                    }
                    4 => {
                        ctx.mes("Okay! I believe you would have everything prepared. Would you like to start the process?")?;
                    }
                    _ => {}
                }
                ctx.next()?;
                if ctx.menu(&["Make Dyestuffs", "Cancel"])? == 0 {
                    let size = l_item.len() as i32;
                    let mut i = 0;
                    while i < size {
                        let have = ctx
                            .call(Function::CountItem, args![runtime::local_get(&l_item, &Val::from(i), false)])?
                            .number()?;
                        if have < runtime::local_get(&l_count, &Val::from(i), false).number()? {
                            break;
                        }
                        i += 1;
                    }
                    if i < size || ctx.player().zeny()? < l_cost {
                        ctx.lines_as("Dye Maker Java Dullihan", args!["Hmmm. Not enough...", "I don't think I'll be able to make the color you want with those materials. Why don't you go get some more materials...?"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    for i in 0..size {
                        ctx.call(
                            Function::DelItem,
                            args![
                                runtime::local_get(&l_item, &Val::from(i), false),
                                runtime::local_get(&l_count, &Val::from(i), false)
                            ],
                        )?;
                    }
                    ctx.player().set_zeny(ctx.player().zeny()? - l_cost)?;
                    ctx.call(Function::GetItem, args![l_dyestuff, 1])?;
                    ctx.lines_as("Dye Maker Java Dullihan", args!["Hmm... It came out pretty well. A very rich color. Of course I'll be trying harder to make a more charming color..."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Dye Maker Java Dullihan",
                        args!["Well, then. Stop by whenever you need more dyes."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Dye Maker Java Dullihan",
                    args!["Eeeehhhh!! What's this?", "You change your mind now!? So disappointing..."],
                )?;
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn java_dullihan(ctx: &Ctx) -> Script {
    java_dullihan_run(ctx, JavaDullihanStep::Start, Vec::new()).map(|_| ())
}

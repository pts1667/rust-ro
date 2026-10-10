use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn hue_ep131_rhea02_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_shu_agree00 = Val::from(0);
    if (ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0
        || (ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 3500)
    {
        ctx.lines_as(
            "Hue",
            args!["How come you've got so much to carry?", "Are you perhaps on training or something?"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("ep13_ryu").get()?.number()? > 99 || ctx.var("ep13_start").get()?.number()? > 99) {
        if ctx.var("ep13_1_rhea").get()?.number()? < 4 {
            ctx.lines_as("Hue", args!["What is it you want?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Hue",
                args![
                    "Don't walk around this place without thinking.",
                    "After all, it'll be you who'd be in trouble."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("ep13_1_rhea").get()? == 4 {
                if ctx.call(Function::CountItem, vec![Val::from(6036)])?.number()? > 0 {
                    ctx.lines_as("Hue", args!["What is it you want?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Hue",
                        args!["You can't just come and go around as you please here. This is a restricted area."],
                    )?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("Sorry...:I came to show you this.")])?) == 1 {
                        ctx.lines_as("Hue", args!["You're just helpless, aren't you?"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as("Hue", args!["What's that?"])?;
                    ctx.next()?;
                    ctx.lines_as("Hue", args!["...An invitation to the meeting... ?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Hue",
                        args![
                            "Haha. You expect me to accept this?",
                            "As always, scheduling things as they want, report to us to accept it..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Hue",
                        args!["I can't think of any other words to describe them, except for this word: RUDE!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Hue", args!["Never ever asking for our opinion and always delivering things through some strangers... By the way, no offense..."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Hue",
                        args!["THey don't ever take the official route! Rune-Midgartish way of reporting! Very unprofessional!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Hue",
                        args!["Anyway, what is this meeting about?", "Give it to me. I should read it at least."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Hue",
                        args!["You should have some Sand chips while I'm reading this. Don't expect any taste, it's from Arunafeltz."],
                    )?;
                    ctx.call(Function::DelItem, vec![Val::from(6036), Val::from(1)])?;
                    ctx.var("ep13_1_rhea").set(Val::from(5))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as("Hue", args!["What are you doing here?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Hue",
                        args!["You can't go around this place as you want. This is a restricted area."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            } else {
                if ctx.var("ep13_1_rhea").get()? == 5 {
                    l_shu_agree00 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(4)])?;
                    if l_shu_agree00.clone() == 2 {
                        ctx.lines_as(
                            "Hue",
                            args![
                                "...Meeting Schedule, on the Xth day, from hh:mm to hh:mm...",
                                "Location... Rune-Midgarts' camp...",
                                "Ha! I knew it. Rune-Midgarts' camp again!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Hue",
                            args!["Objectives... to report on the researching progress of tracking Satan Morocc and the Ash-Vacuum!"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Hue", args!["Having some time to get to know each other??? Haaaaa!"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Hue",
                            args!["Well, thank you for entertaining me. You should tell Ryosen that."],
                        )?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(
                            ctx,
                            &[Val::from("Ok, I will.:Please sign the invitation.")],
                        )?) == 1
                        {
                            ctx.lines_as(
                                "Hue",
                                args!["Well, you should also tell him that I think he's very talented in settin up comedy. Hahaha."],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as("Hue", args!["... Sign this thing???"])?;
                        ctx.next()?;
                        ctx.lines_as("Hue", args!["You mean this meeting's for real???"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Hue",
                            args!["Oh my!", "I have never seen an official invitation to be this stupid-looking!"],
                        )?;
                        ctx.next()?;
                        ctx.mes("- Hue got so furious and pressed the pen so hard that the ball point got pushed in. Then he threw the document after signing it. -")?;
                        ctx.next()?;
                        ctx.lines_as("Hue", args!["Well, at least I respect that Ryosen hasn't forgotten the reporting order. So, you're off to Arunafeltz now?"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Hue",
                            args![
                                "You'll experience some hard time talking to Hansenne... It'll be like testing your patience.",
                                "Good luck!"
                            ],
                        )?;
                        ctx.var("ep13_1_rhea").set(Val::from(6))?;
                        ctx.call(Function::GetItem, vec![Val::from(6036), Val::from(1)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if l_shu_agree00.clone() == 3 {
                        ctx.lines_as(
                            "Hue",
                            args!["Come on, what's the hurry?", "Wait a few more minutes, I didn't finish reading it!"],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Hue",
                            args!["You should have some Sand chips while I'm reading this. Don't expect any taste, it's from Arunafeltz."],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                } else {
                    if ctx.var("ep13_1_rhea").get()? == 6 {
                        ctx.lines_as("Hue", args!["I'm not sure if everyone from Arunafeltz is like him, but, it'll be like testing your patience to get Hansenne to read this thing.", "You can trust me on that one. Good luck."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if ctx.var("ep13_1_rhea").get()? == 15 {
                            if ctx.call(Function::CountItem, vec![Val::from(6037)])?.number()? > 0 {
                                ctx.lines_as("Hue", args!["What is this stupid looking pile of documents?"])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Hue",
                                    args!["Is this the file Ryosen requested from Hansenne??? The one which is all ruined...?"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Hue",
                                    args!["Ha! Tsk,tsk! Is this some kind of joke?! Who'd manage the files like this? I'd never know!"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Hue",
                                    args!["Whew... I can't even stand looking at it! Oh just, give it to me!"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Hue", args!["Too bad I'm not in Schwarzwald. I could use our document restoring machine back at home... Ugh! Now I gotta do everything manually."])?;
                                ctx.next()?;
                                ctx.lines_as("Hue", args!["Hmmm. It's completely soaked. I Should dry this thing first and iron these crumples. Then, I should restore the texts."])?;
                                ctx.next()?;
                                ctx.lines_as("Hue", args!["I need some materials. You gotta help me on this. I need ^0000ff1 Fan, 1 Old Frying Pan, 1 Flame Stone, 1 Chinese Ink^000000."])?;
                                ctx.next()?;
                                ctx.lines_as("Hue", args!["Hey, hey... don't look at me like that. I know you're not delighted to do these things but think about it, what about me? Why should I restore this garbage?"])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Hue",
                                    args!["Listen, sometimes, we've got to do things we don't like in order to get things accomplished."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Hue",
                                    args!["Just don't gorget that the file will be restored as fast as you prepare those things for me."],
                                )?;
                                ctx.var("ep13_1_rhea").set(Val::from(16))?;
                                ctx.call(Function::ChangeQuest, vec![Val::from(8202), Val::from(8203)])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                ctx.lines_as("Hue", args!["I wonder if the preparation for the meeting's going ok."])?;
                                ctx.next()?;
                                ctx.lines_as("Hue", args!["I heard Ryosen say that the file he requested from Hansenne is all ruined. I hope I dont get involved in that trouble."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        } else {
                            if ctx.var("ep13_1_rhea").get()? == 16 {
                                if ((((ctx.call(Function::CountItem, vec![Val::from(6037)])?.number()? > 0
                                    && ctx.call(Function::CountItem, vec![Val::from(7262)])?.number()? > 0)
                                    && ctx.call(Function::CountItem, vec![Val::from(7031)])?.number()? > 0)
                                    && ctx.call(Function::CountItem, vec![Val::from(7521)])?.number()? > 0)
                                    && ctx.call(Function::CountItem, vec![Val::from(1024)])?.number()? > 0)
                                {
                                    ctx.lines_as("Hue", args!["Impressive. That was fast enough."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Hue", args!["Give them to me.", "I have no time to waste."])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Hue",
                                        args!["Could you please sit there and wait for me while I restore this file?"],
                                    )?;
                                    ctx.call(Function::DelItem, vec![Val::from(6037), Val::from(1)])?;
                                    ctx.call(Function::DelItem, vec![Val::from(7262), Val::from(1)])?;
                                    ctx.call(Function::DelItem, vec![Val::from(7031), Val::from(1)])?;
                                    ctx.call(Function::DelItem, vec![Val::from(7521), Val::from(1)])?;
                                    ctx.call(Function::DelItem, vec![Val::from(1024), Val::from(1)])?;
                                    ctx.var("ep13_1_rhea").set(Val::from(17))?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    ctx.lines_as("Hue", args!["Hmmm. It's completely soaked. I Should dry this thing first and iron these crumples. Then, I should restore the texts."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Hue", args!["I need some materials. You gotta help me on this. I need ^0000ff1 Fan, 1 Old Frying Pan, 1 Flame Stone, 1 Chinese Ink^000000."])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Hue",
                                        args![
                                            "Just don't gorget that the file will be restored as fast as you prepare those things for me."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                            } else {
                                if ctx.var("ep13_1_rhea").get()? == 17 {
                                    if ctx.call(Function::Rand, vec![Val::from(1), Val::from(4)])? == 2 {
                                        ctx.lines_as("Hue", args!["There!"])?;
                                        ctx.next()?;
                                        ctx.lines_as("Hue", args!["The binding was too loose, so I re-arranged and bound the file all over again. This is how the true official document should look like. I really think they should learn how to arrange files."])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Hue",
                                            args!["Please, deliver this file before it's too late. The meeting's gonna begin soon."],
                                        )?;
                                        ctx.var("ep13_1_rhea").set(Val::from(18))?;
                                        ctx.call(Function::GetItem, vec![Val::from(6038), Val::from(1)])?;
                                        ctx.call(Function::ChangeQuest, vec![Val::from(8203), Val::from(8204)])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else {
                                        ctx.lines_as("Hue", args!["Hey, stop pushing me."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Hue", args!["I have to be extra careful with this kind of work, ok? Can't you sit quietly over there? Take a nap or something."])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                } else {
                                    if ctx.var("ep13_1_rhea").get()? == 18 {
                                        ctx.lines_as("Hue", args!["The binding was too loose, so I re-arranged and bound the file all over again. This is how the true official document should look like. I really think they should learn how to arrange files."])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Hue",
                                            args!["Please, deliver this file before it's too late. The meeting's gonna begin soon."],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else {
                                        if ctx.var("ep13_1_rhea").get()? == 19 {
                                            ctx.lines_as("Hue", args!["Well, finally... It's time for the meeting."])?;
                                            ctx.next()?;
                                            ctx.lines_as("Hue", args!["Like it or not... I should go in."])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        } else {
                                            if ctx.var("ep13_1_rhea").get()? == 20 {
                                                ctx.lines_as("Hue", args![".................."])?;
                                                ctx.next()?;
                                                ctx.lines_as("Hue", args![".....Ittt....Is... it still there... ?"])?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            } else if (ctx.var("ep13_1_rhea").get()?.number()? > 20
                                                && ctx.var("ep13_1_rhea").get()?.number()? < 24)
                                            {
                                                ctx.mes("- Looks like he's thinking about something... very seriously. -")?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            } else if ctx.var("ep13_1_rhea").get()? == 24 {
                                                ctx.lines_as("Hue", args!["I just decided I can't stay like this."])?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Hue",
                                                    args![
                                                        "I couldn't confess the truth when Hansenne said he ruined the cake instead of me."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as("Hue", args!["I was so bewildered that I lost the chance to say anything... but... I was such a coward to admit that I was the one who caused the trouble."])?;
                                                ctx.next()?;
                                                ctx.lines_as("Hue", args!["I must apologize.", "But then..."])?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Hue",
                                                    args!["Oh, I'm so embarrassed to see him face to face. Or.. how about..."],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as("Hue", args!["Here, this is the Rune-Midgarts' report I took with me when everything went messy in the meeting room. Could you please give this to Ryosen?"])?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Hue",
                                                    args![
                                                        "I'm sorry to ask you this but...",
                                                        "Could you? For this, I'll give you some Schwarzwald's Pineapple Jubilee."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as("Hue", args!["Please give this report to Ryosen.", "Thank you."])?;
                                                ctx.var("ep13_1_rhea").set(Val::from(25))?;
                                                ctx.call(Function::GetItem, vec![Val::from(6038), Val::from(1)])?;
                                                ctx.call(Function::GetItem, vec![Val::from(12320), Val::from(1)])?;
                                                ctx.call(Function::ChangeQuest, vec![Val::from(8208), Val::from(8209)])?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            } else if (ctx.var("ep13_1_rhea").get()?.number()? > 24
                                                && ctx.var("ep13_1_rhea").get()?.number()? < 100)
                                            {
                                                ctx.lines_as(
                                                    "Hue",
                                                    args!["Oh, I'm so embarrassed to see him face to face. Or.. how about..."],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as("Hue", args!["Here, this is the Rune-Midgarts' report I took with me when everything went messy in the meeting room. Could you please give this to Ryosen?"])?;
                                                ctx.next()?;
                                                ctx.lines_as("Hue", args!["I'm sorry to ask you this but...", "Could you?"])?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            } else if ctx.var("ep13_1_rhea").get()?.number()? > 99 {
                                                ctx.lines_as("Hue", args!["Ah, hello!"])?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Hue",
                                                    args![
                                                        "If it weren't for you, we wouldn't be able to get along at all.",
                                                        "I don't know how to thank you!"
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as("Hue", args!["Have you ever seen something called, ^0000ffFur^000000 ?", "I hear you can get it from the monsters called 'Tatacho' and 'Hillsrion' in the fields."])?;
                                                ctx.next()?;
                                                ctx.lines_as("Hue", args!["Could you bring ^0000ffFur^000000 if you get some? I'm not asking it for free.", "I'll give you my Pineapple Jubilee from Schwarzwald, if you bring me ^0000ff2 Furs^000000."])?;
                                                ctx.next()?;
                                                match runtime::select_values(
                                                    ctx,
                                                    &[Val::from("I don't have any, right now.:Oh, I'll give you Fur.")],
                                                )? {
                                                    1 => {
                                                        ctx.lines_as("Hue", args!["Alright, you don't have any."])?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Hue", args!["Please give me some when you get some."])?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    }
                                                    2 => {
                                                        if ctx.call(Function::CountItem, vec![Val::from(12320)])?.number()? > 4 {
                                                            ctx.lines_as("Hue", args!["Ah, oh my."])?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "Hue",
                                                                args!["You already have the Pineapple Jubilee from Schwarzwald!"],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines_as("Hue", args!["I think you should drin that thing fast. Otherwise, all the ice will melt and it'll taste like some kinda medicine!"])?;
                                                            ctx.close_window()?;
                                                            return Err(Stop::End);
                                                        }
                                                    }
                                                    _ => {}
                                                }
                                                ctx.lines_as("Hue", args!["Ah, are you sure you want to exchange?"])?;
                                                ctx.next()?;
                                                if ctx.call(Function::CountItem, vec![Val::from(6020)])?.number()? > 1 {
                                                    ctx.lines_as("Hue", args!["Oh, so it's the thing called, Fur??"])?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Hue",
                                                        args!["Are you sure you want to exchange this for my Pineapple Jubilee?"],
                                                    )?;
                                                    ctx.next()?;
                                                    if Val::from(runtime::select_values(ctx, &[Val::from("No way.:Sure.")])?) == 1 {
                                                        ctx.lines_as("Hue", args!["Ahhhhh..."])?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Hue", args!["Come back any time if you change your mind."])?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    }
                                                    ctx.lines_as("Hue", args!["Thank you so much."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Hue", args!["I always needed this because the weather here is colder then I expected. I'm sure this will keep me warm."])?;
                                                    ctx.call(Function::DelItem, vec![Val::from(6020), Val::from(2)])?;
                                                    ctx.call(Function::GetItem, vec![Val::from(12320), Val::from(1)])?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                } else {
                                                    ctx.lines_as("Hue", args!["Well, I don't think you have any Fur."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Hue", args!["Please come back if you find any Fur."])?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                }
                                            } else {
                                                ctx.lines_as("Hue", args!["Don't walk around this area. It's restricted."])?;
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
    } else {
        ctx.lines_as("Hue", args!["What are you doing here?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Hue",
            args![
                "Don't walk around this place without thinking.",
                "After all, it'll be you who'd be in trouble."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn hue_ep131_rhea02(ctx: &Ctx) -> Script {
    hue_ep131_rhea02_body(ctx, Vec::new()).map(|_| ())
}

fn hansenne_ep131_rhea03_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_shu_agree00 = Val::from(0);
    if (ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0
        || (ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 3500)
    {
        ctx.lines_as(
            "Hansenne",
            args!["How come you've got so much to carry?", "Are you perhaps on training or something?"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("ep13_ryu").get()?.number()? > 99 || ctx.var("ep13_start").get()?.number()? > 99) {
        if ctx.var("ep13_1_rhea").get()?.number()? < 6 {
            ctx.lines_as("Hansenne", args!["Who's there?"])?;
            ctx.next()?;
            ctx.lines_as("Hansenne", args!["Who's here?"])?;
            ctx.next()?;
            ctx.lines_as("Hansenne", args!["Haha, hahaha, hahahahahahahahaha."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("ep13_1_rhea").get()? == 6 {
                if ctx.call(Function::CountItem, vec![Val::from(6036)])?.number()? > 0 {
                    ctx.lines_as("Hansenne", args!["Hmmm? What brings you here?"])?;
                    ctx.next()?;
                    ctx.lines_as("Hansenne", args!["Of course, your feet brought you here!?!"])?;
                    ctx.next()?;
                    ctx.lines_as("Hansenne", args!["Haha, hahaha, hahahahahahahahaha."])?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("......:Come and look at this.")])? {
                        1 => {
                            ctx.lines_as("Hansenne", args!["What are you looking at?", "Something I said?"])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {}
                        _ => {}
                    }
                    ctx.lines_as("Hansenne", args!["What's that?"])?;
                    ctx.next()?;
                    ctx.lines_as("Hansenne", args!["An invitation to the meeting?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Hansenne",
                        args!["Hmm, Ryosen from Rune-Midgarts made it and... Hue from Schwarzwald agreed to sign it too?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Hansenne", args!["Hmmm, that's not funny at all."])?;
                    ctx.next()?;
                    ctx.lines_as("Hansenne", args!["They're so boring. They always report to me after making all the decisions. No sense of humor... so stubborn and strict... They're totally not my style."])?;
                    ctx.next()?;
                    ctx.lines_as("Hansenne", args!["Anyway, give it to me.", "I should have a look at least."])?;
                    ctx.next()?;
                    ctx.lines_as("Hansenne", args!["You should take a rest, while I'm reading this. Oh, have some Poring Candy from Rune-Midgarts. It's terribly sour and sometimes salty. Funny taste, huh?"])?;
                    ctx.call(Function::DelItem, vec![Val::from(6036), Val::from(1)])?;
                    ctx.var("ep13_1_rhea").set(Val::from(7))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as("Hansenne", args!["Hmmm? Are you from Rune-Midgarts?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Hansenne",
                        args!["Then where's the person from Rude Midgard? Hahahahah, do you get my joke?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Hansenne", args!["Haha, hahaha, hahahahahahahahaha."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            } else {
                if ctx.var("ep13_1_rhea").get()? == 7 {
                    l_shu_agree00 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(5)])?;
                    if l_shu_agree00.clone() == 1 {
                        ctx.lines_as("Hansenne", args!["Mmmmmm~"])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "- Hansenne picks up the file -",
                            "- and suddenly gets serious -",
                            "- while turning the pages... -"
                        ])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if l_shu_agree00.clone() == 2 {
                        ctx.lines_as("Hansenne", args!["Mmmmmm~"])?;
                        ctx.next()?;
                        ctx.mes("- Hansenne drops some kind of liquid on the file document, and carefully looks at it. -")?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if l_shu_agree00.clone() == 3 {
                        ctx.lines_as(
                            "Hansenne",
                            args![
                                "Meeting Schedule... on the Umpth day, from then-o'clock to later-o'clock",
                                "Haha, hahaha, hahahahahahahahaha.",
                                "Meeting Schedule, shmeating schedule... Beating Schedule!!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Hansenne",
                            args![
                                "Location... Rune-Midgarts' camp!",
                                "Rude-Midgard's camp!?!?",
                                "Are they camping in that place??",
                                "Haha, hahaha, hahahahahahahahaha."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Hansenne",
                            args!["Objectives... to report on the researching progress of tracking Satan Morocc and of the Ash-Vacuum!"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Hansenne",
                            args![
                                "Ash-Vacuum!",
                                "Ash-Vacuum... what is that, a homemaker's device???",
                                "Haha, hahaha, hahahahahahahahaha."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Hansenne",
                            args![
                                "...and a very useless one which vacuums only ash!",
                                "Haha, hahaha, hahahahahahahahaha."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Hansenne",
                            args![
                                "Ehhhhh? And what's this? Having some time to get to know each other??? ...Pffthahahahaha...",
                                "Aaahahahahaha."
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("What's so funny?!:Hey, just sign this thing.")])? {
                            1 => {
                                ctx.lines_as(
                                    "Hansenne",
                                    args![
                                        "Haha...get to know each other...",
                                        "Pffftahahaha.",
                                        "Haha, hahaha, hahahahahahahahaha."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {}
                            _ => {}
                        }
                        ctx.lines_as(
                            "Hansenne",
                            args![
                                "Phew! Excuse me!",
                                "I can't help it! I can't stand this! What the heck!!! What do we have to know about each other???",
                                "Hahaha!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines(args![
                            "- Hansenne signs the paper -",
                            "- with a shaking hand -",
                            "- still laughing out loud. -"
                        ])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Hansenne",
                            args![
                                "Puhhh-haha, this is unbelievable.",
                                "Ryosen's avery funny guy. Well, he wouldn't know how funny he is...",
                                "Haha, you should tell him that.",
                                "Haha, hahaha, hahahahahahahahaha."
                            ],
                        )?;
                        ctx.var("ep13_1_rhea").set(Val::from(8))?;
                        ctx.call(Function::GetItem, vec![Val::from(6036), Val::from(1)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if l_shu_agree00.clone() == 4 {
                        ctx.lines_as("Hansenne", args!["Ahhh, wait. I didn't finish reading it."])?;
                        ctx.next()?;
                        ctx.lines_as("Hansenne", args!["I didn't finish reading it."])?;
                        ctx.next()?;
                        ctx.lines_as("Hansenne", args!["Haha, hahaha, hahahahahahahahaha."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as("Hansenne", args!["Hush! Please don't talk! We must pray now!"])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "- He takes a bow silently, -",
                            "- toward the sky, -",
                            "- with his eyes firmly closed. -"
                        ])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                } else {
                    if ctx.var("ep13_1_rhea").get()? == 8 {
                        ctx.lines_as(
                            "Hansenne",
                            args![
                                "I got it, I got it. I should go to the meeting. Ryosen's really a funny guy, if you know what I mean.",
                                "Please, thank him for entertaining me. Haha, hahaha,",
                                "hahahahahahahahaha."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if ctx.var("ep13_1_rhea").get()? == 12 {
                            ctx.lines_as("Hansenne", args!["Hmmm? You again?"])?;
                            ctx.next()?;
                            ctx.lines_as("Hansenne", args!["What are you doing here?"])?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("I missed you.:I came to pick up the requested document.")])? {
                                1 => {
                                    ctx.lines_as("Hansenne", args!["You what?"])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Hansenne",
                                        args!["You missed me? Don't expect a kiss!", "Haha, hahaha, hahahahahahahahaha."],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                2 => {}
                                _ => {}
                            }
                            ctx.lines_as("Hansenne", args!["A requested document? Did Ryosen send you?"])?;
                            ctx.next()?;
                            ctx.lines_as("Hansenne", args!["Hmmmmm.", "Wait for me here. Now, where did I put it...?"])?;
                            ctx.next()?;
                            ctx.lines_as("Hansenne", args!["Hmmmmm~?"])?;
                            ctx.next()?;
                            ctx.lines_as("Hansenne", args!["Ahhhh!", "Hmmmmm, right, right!"])?;
                            ctx.next()?;
                            ctx.lines_as("Hansenne", args!["It's gone!"])?;
                            ctx.next()?;
                            ctx.lines_as("Hansenne", args!["I lost it. I think I might have dropped it into the river or something when I played treasure-hunting on a bridge!"])?;
                            ctx.next()?;
                            ctx.lines_as("Hansenne", args!["But you know, the funny thing is... I heard my parents picked me up on a bridge when I was a baby, and now I lost the document on a bridge! What a coincidence! Haha, hahaha, hahahahahahahahaha."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Hansenne",
                                args![
                                    "It'll take time to make it all over. Although... Hmm... Should I go fishing and catch that thing~?",
                                    "Those pussycats on the South-West can teach us fishing, you know."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Hansenne",
                                args!["Maybe we can catch that document with a fishing rod if we're lucky!", "Wow!"],
                            )?;
                            ctx.var("ep13_1_rhea").set(Val::from(13))?;
                            ctx.call(Function::ChangeQuest, vec![Val::from(8200), Val::from(8201)])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            if ctx.var("ep13_1_rhea").get()? == 13 {
                                ctx.lines_as("Hansenne", args!["I lost it. I think I might have dropped it into the river or something when I played treasure-hunting on a bridge!"])?;
                                ctx.next()?;
                                ctx.lines_as("Hansenne", args!["But you know, the funny thing is... I heard my parents picked me up on a bridge when I was a baby, and now I lost the document on a bridge! What a coincidence! Haha, hahaha, hahahahahahahahaha."])?;
                                ctx.next()?;
                                ctx.lines_as("Hansenne", args!["It'll take time to make it all over. Although... Hmm... Should I go fishing and catch that thing~?", "Those pussycats on the South-West can teach us fishing, you know."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                if ctx.var("ep13_1_rhea").get()? == 14 {
                                    if ctx.call(Function::CountItem, vec![Val::from(6037)])?.number()? > 0 {
                                        ctx.lines_as("Hansenne", args!["Cool, you found it!"])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Hansenne",
                                            args!["Mmmm, I'm sure this is the one I made, just by looking at this overall shape."],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Hansenne",
                                            args!["Hmmmmm, but then... I'm afraid Ryosen would explode when he sees this! Boom!!!"],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("Hansenne", args!["Schwarzwald people have some talent in restoring things like this... Why don't you ask Hue to restore this?"])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Hansenne",
                                            args![
                                                "He'd scream so hard and the windows might break. But he'll do it.",
                                                "Haha, hahaha, hahahahahahahahaha."
                                            ],
                                        )?;
                                        ctx.var("ep13_1_rhea").set(Val::from(15))?;
                                        ctx.call(Function::ChangeQuest, vec![Val::from(8201), Val::from(8202)])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else {
                                        ctx.lines_as("Hansenne", args!["I lost it. I think I might have dropped it into the river or something when I played treasure-hunting on a bridge!"])?;
                                        ctx.next()?;
                                        ctx.lines_as("Hansenne", args!["But you know, the funny thing is... I heard my parents picked me up on a bridge when I was a baby, and now I lost the document on a bridge! What a coincidence! Haha, hahaha, hahahahahahahahaha."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Hansenne", args!["It'll take time to make it all over. Although... Hmm... Should I go fishing and catch that thing~?", "Those pussycats on the South-West can teach us fishing, you know."])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                } else {
                                    if ctx.var("ep13_1_rhea").get()? == 15 {
                                        ctx.lines_as(
                                            "Hansenne",
                                            args![
                                                "Hmmmmm, it's all screwed up. I'm afraid Ryosen would explode when he sees this! Boom!!!"
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("Hansenne", args!["Schwarzwald people have some talent in restoring things like this... Why don't you ask Hue to restore this?"])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Hansenne",
                                            args![
                                                "He'd scream so hard and the windows might break. But he'll do it.",
                                                "Haha, hahaha, hahahahahahahahaha."
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else {
                                        if ctx.var("ep13_1_rhea").get()? == 19 {
                                            ctx.lines_as("Hansenne", args!["Hmmmmm~ It's about time."])?;
                                            ctx.next()?;
                                            ctx.lines_as("Hansenne", args!["I don't wanna get in trouble. I should hurry up."])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        } else {
                                            if ctx.var("ep13_1_rhea").get()? == 20 {
                                                ctx.lines_as("Hansenne", args!["Haha, hahaha...."])?;
                                                ctx.next()?;
                                                ctx.lines_as("Hansenne", args!["That friend of yours never gets my jokes... My sense of humor is... just... out of his league!! Haha, hahaha, hahahahahahahahaha."])?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            } else if (ctx.var("ep13_1_rhea").get()? == 21 || ctx.var("ep13_1_rhea").get()? == 22) {
                                                ctx.lines_as("Hansenne", args!["Huh? You again?"])?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                    args!["Why did you do that?"],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as("Hansenne", args!["Hehhhhh??? What are you talking about? If you wanna blame me for anything, tell me the reason first~", "Haha, hahaha, hahahahahahahahaha."])?;
                                                ctx.next()?;
                                                ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["I know you're not the one who ruined the cake. I saw it. It was Hue! I saw him clearly when he pressed his hand on the cake when the Thief Bug came out."])?;
                                                ctx.next()?;
                                                ctx.lines_as("Hansenne", args!["Well... Who cares who did it?", "It was just a mistake. Nothing's gonna change even if we find out who did it. The cake is still ruined, isn't it?"])?;
                                                ctx.next()?;
                                                ctx.lines_as("Hansenne", args!["Hmmmmm... or... I don't know. Maybe I'm thanlful that he restored my garbage looking file... ?"])?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                    args!["But... You were blamed for it.", "Aren't you feeling bad? It's unfair!"],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as("Hansenne", args!["Well, not really.", "Ryosen could be very rhough when he gets freaked out, but he will forget about all that soon..."])?;
                                                ctx.next()?;
                                                ctx.lines_as("Hansenne", args!["I understand. He was upset."])?;
                                                ctx.next()?;
                                                ctx.lines_as("Hansenne", args!["You see, he brought that cake for all of us in there. He wouldn't normally do that, but I guess he wanted to be nice. But then, poof~ the cake was ruined."])?;
                                                ctx.next()?;
                                                ctx.lines_as("Hansenne", args!["He must've been so upset that we all couldn't share that cake. Isn't it kinda cute?", "Haha, hahaha, hahahahahahahahaha."])?;
                                                ctx.var("ep13_1_rhea").set(Val::from(23))?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            } else if ctx.var("ep13_1_rhea").get()? == 23 {
                                                ctx.lines_as("Hansenne", args!["Well, it's nothing big. Don't you think?"])?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Hansenne",
                                                    args!["He'll be ok soon. He'll get to thinking that it wasn't actually a big deal."],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as("Hansenne", args!["But the thing is that... the relationship between Ryosen and Hue isn't looking good at all.", "Why don't you help them get along with each other?"])?;
                                                ctx.next()?;
                                                ctx.lines_as("Hansenne", args!["Sorry for asking these things of you. And... here's Arunageltz's Desert Sandwich. It's nothing special but... tastes pretty good."])?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Hansenne",
                                                    args!["Thank you for doing me a favor.", "Haha, hahaha, hahahahahahahahaha."],
                                                )?;
                                                ctx.var("ep13_1_rhea").set(Val::from(24))?;
                                                ctx.call(Function::GetItem, vec![Val::from(12321), Val::from(1)])?;
                                                ctx.call(Function::ChangeQuest, vec![Val::from(8207), Val::from(8208)])?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            } else if (ctx.var("ep13_1_rhea").get()?.number()? > 23
                                                && ctx.var("ep13_1_rhea").get()?.number()? < 100)
                                            {
                                                ctx.lines_as(
                                                    "Hansenne",
                                                    args![
                                                        "The relationship between Ryosen and Hue isn't looking good at all.",
                                                        "Why don't you help them get along with each other?"
                                                    ],
                                                )?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            } else if ctx.var("ep13_1_rhea").get()?.number()? > 99 {
                                                ctx.lines_as("Hansenne", args!["Ah, it's you adventurer."])?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Hansenne",
                                                    args!["I heard about you from the Official. Thank you so much for helping us."],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as("Hansenne", args!["Anyways.. I wonder if you have seen a ^0000ffPeaked Hat^000000. I heard the monster, 'Tatacho' has that hat."])?;
                                                ctx.next()?;
                                                ctx.lines_as("Hansenne", args!["Can you give me a ^0000ffPeaked Hat^000000 if you have any? I'm not asking for free.", "For ^0000ff2 Peaked Hats^000000, I'll give you my special dessert!"])?;
                                                ctx.next()?;
                                                match runtime::select_values(ctx, &[Val::from("No way.:Sure.")])? {
                                                    1 => {
                                                        ctx.lines_as("Hansenne", args!["No way?"])?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Hansenne",
                                                            args!["I'd say, yes way?", "Haha, hahaha, hahahahahahahahaha."],
                                                        )?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    }
                                                    2 => {
                                                        if ctx.call(Function::CountItem, vec![Val::from(12321)])?.number()? > 4 {
                                                            ctx.lines_as("Hansenne", args!["... Huh?"])?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "Hansenne",
                                                                args![
                                                                    "Oh, it loos like you already have the Arunafeltz's Desert Sandwich."
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines_as("Hansenne", args!["You should eat that up before it gets rotten. Maybe it's gone bad already.", "Haha, hahaha, hahahahahahahahaha."])?;
                                                            ctx.close_window()?;
                                                            return Err(Stop::End);
                                                        }
                                                    }
                                                    _ => {}
                                                }
                                                ctx.lines_as("Hansenne", args!["Ah!!! You have it?"])?;
                                                ctx.next()?;
                                                if ctx.call(Function::CountItem, vec![Val::from(6021)])?.number()? > 1 {
                                                    ctx.lines_as("Hansenne", args!["Oh, you really have those!"])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Hansenne", args!["Are you sure you want to give these to me?"])?;
                                                    ctx.next()?;
                                                    match runtime::select_values(ctx, &[Val::from("No way.:Sure.")])? {
                                                        1 => {
                                                            ctx.lines_as(
                                                                "Hansenne",
                                                                args![
                                                                    "No way. Yes way. This way. That way.",
                                                                    "Haha, hahaha, hahahahahahahahaha."
                                                                ],
                                                            )?;
                                                            ctx.close_window()?;
                                                            return Err(Stop::End);
                                                        }
                                                        2 => {}
                                                        _ => {}
                                                    }
                                                    ctx.lines_as(
                                                        "Hansenne",
                                                        args![
                                                            "Ohhhhh, thank you!",
                                                            "Ehhmmm, thank you!",
                                                            "Geeeee, thank you!",
                                                            "Haha, hahaha, hahahahahahahahaha."
                                                        ],
                                                    )?;
                                                    ctx.call(Function::DelItem, vec![Val::from(6021), Val::from(2)])?;
                                                    ctx.call(Function::GetItem, vec![Val::from(12321), Val::from(1)])?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                } else {
                                                    ctx.lines_as("Hansenne", args!["You haven't got any."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Hansenne",
                                                        args!["How about getting some?", "Haha, hahaha, hahahahahahahahaha."],
                                                    )?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                }
                                            } else {
                                                ctx.lines_as("Hansenne", args!["Sasquatch Hydra!!!"])?;
                                                ctx.next()?;
                                                ctx.lines_as("Hansenne", args!["Haha, hahaha, hahahahahahahahaha."])?;
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
    } else {
        ctx.lines_as("Hansenne", args!["Sasquatch Hydra!!!"])?;
        ctx.next()?;
        ctx.lines_as("Hansenne", args!["Haha, hahaha, hahahahahahahahaha."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn hansenne_ep131_rhea03(ctx: &Ctx) -> Script {
    hansenne_ep131_rhea03_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum RyosenEp131Rhea05Step {
    Start,
    OnEnable,
    OnDisable,
    OnInit,
    OnTimer300000,
}

fn ryosen_ep131_rhea05_run(ctx: &Ctx, mut step: RyosenEp131Rhea05Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            RyosenEp131Rhea05Step::Start => {
                return Err(Stop::End);
            }
            RyosenEp131Rhea05Step::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Ryosen#ep131_rhea05")])?;
                return Err(Stop::End);
            }
            RyosenEp131Rhea05Step::OnDisable => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                step = RyosenEp131Rhea05Step::OnInit;
                continue 'machine;
            }
            RyosenEp131Rhea05Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Ryosen#ep131_rhea05")])?;
                return Err(Stop::End);
            }
            RyosenEp131Rhea05Step::OnTimer300000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Ryosen#ep131_rhea05::OnDisable")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn ryosen_ep131_rhea05(ctx: &Ctx) -> Script {
    ryosen_ep131_rhea05_run(ctx, RyosenEp131Rhea05Step::Start, Vec::new()).map(|_| ())
}

pub fn ryosen_ep131_rhea05_onenable(ctx: &Ctx) -> Script {
    ryosen_ep131_rhea05_run(ctx, RyosenEp131Rhea05Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn ryosen_ep131_rhea05_ondisable(ctx: &Ctx) -> Script {
    ryosen_ep131_rhea05_run(ctx, RyosenEp131Rhea05Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn ryosen_ep131_rhea05_oninit(ctx: &Ctx) -> Script {
    ryosen_ep131_rhea05_run(ctx, RyosenEp131Rhea05Step::OnInit, Vec::new()).map(|_| ())
}

pub fn ryosen_ep131_rhea05_ontimer300000(ctx: &Ctx) -> Script {
    ryosen_ep131_rhea05_run(ctx, RyosenEp131Rhea05Step::OnTimer300000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum HueEp131Rhea06Step {
    Start,
    OnEnable,
    OnDisable,
    OnInit,
    OnTimer300000,
}

fn hue_ep131_rhea06_run(ctx: &Ctx, mut step: HueEp131Rhea06Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            HueEp131Rhea06Step::Start => {
                return Err(Stop::End);
            }
            HueEp131Rhea06Step::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Hue#ep131_rhea06")])?;
                return Err(Stop::End);
            }
            HueEp131Rhea06Step::OnDisable => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                step = HueEp131Rhea06Step::OnInit;
                continue 'machine;
            }
            HueEp131Rhea06Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Hue#ep131_rhea06")])?;
                return Err(Stop::End);
            }
            HueEp131Rhea06Step::OnTimer300000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Hue#ep131_rhea06::OnDisable")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn hue_ep131_rhea06(ctx: &Ctx) -> Script {
    hue_ep131_rhea06_run(ctx, HueEp131Rhea06Step::Start, Vec::new()).map(|_| ())
}

pub fn hue_ep131_rhea06_onenable(ctx: &Ctx) -> Script {
    hue_ep131_rhea06_run(ctx, HueEp131Rhea06Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn hue_ep131_rhea06_ondisable(ctx: &Ctx) -> Script {
    hue_ep131_rhea06_run(ctx, HueEp131Rhea06Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn hue_ep131_rhea06_oninit(ctx: &Ctx) -> Script {
    hue_ep131_rhea06_run(ctx, HueEp131Rhea06Step::OnInit, Vec::new()).map(|_| ())
}

pub fn hue_ep131_rhea06_ontimer300000(ctx: &Ctx) -> Script {
    hue_ep131_rhea06_run(ctx, HueEp131Rhea06Step::OnTimer300000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum HansenneEp131Rhea07Step {
    Start,
    OnEnable,
    OnDisable,
    OnInit,
    OnTimer300000,
}

fn hansenne_ep131_rhea07_run(ctx: &Ctx, mut step: HansenneEp131Rhea07Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            HansenneEp131Rhea07Step::Start => {
                return Err(Stop::End);
            }
            HansenneEp131Rhea07Step::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Hansenne#ep131_rhea07")])?;
                return Err(Stop::End);
            }
            HansenneEp131Rhea07Step::OnDisable => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                step = HansenneEp131Rhea07Step::OnInit;
                continue 'machine;
            }
            HansenneEp131Rhea07Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Hansenne#ep131_rhea07")])?;
                return Err(Stop::End);
            }
            HansenneEp131Rhea07Step::OnTimer300000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Hue#ep131_rhea06::OnDisable")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn hansenne_ep131_rhea07(ctx: &Ctx) -> Script {
    hansenne_ep131_rhea07_run(ctx, HansenneEp131Rhea07Step::Start, Vec::new()).map(|_| ())
}

pub fn hansenne_ep131_rhea07_onenable(ctx: &Ctx) -> Script {
    hansenne_ep131_rhea07_run(ctx, HansenneEp131Rhea07Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn hansenne_ep131_rhea07_ondisable(ctx: &Ctx) -> Script {
    hansenne_ep131_rhea07_run(ctx, HansenneEp131Rhea07Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn hansenne_ep131_rhea07_oninit(ctx: &Ctx) -> Script {
    hansenne_ep131_rhea07_run(ctx, HansenneEp131Rhea07Step::OnInit, Vec::new()).map(|_| ())
}

pub fn hansenne_ep131_rhea07_ontimer300000(ctx: &Ctx) -> Script {
    hansenne_ep131_rhea07_run(ctx, HansenneEp131Rhea07Step::OnTimer300000, Vec::new()).map(|_| ())
}

pub fn timer_alba01(ctx: &Ctx) -> Script {
    timer_alba01_run(ctx, TimerAlba01Step::Start, Vec::new()).map(|_| ())
}

pub fn timer_alba01_oninit(ctx: &Ctx) -> Script {
    timer_alba01_run(ctx, TimerAlba01Step::OnInit, Vec::new()).map(|_| ())
}

pub fn timer_alba01_onenable(ctx: &Ctx) -> Script {
    timer_alba01_run(ctx, TimerAlba01Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn timer_alba01_onstop(ctx: &Ctx) -> Script {
    timer_alba01_run(ctx, TimerAlba01Step::OnStop, Vec::new()).map(|_| ())
}

pub fn timer_alba01_ontimer1000(ctx: &Ctx) -> Script {
    timer_alba01_run(ctx, TimerAlba01Step::OnTimer1000, Vec::new()).map(|_| ())
}

pub fn timer_alba01_ontimer180000(ctx: &Ctx) -> Script {
    timer_alba01_run(ctx, TimerAlba01Step::OnTimer180000, Vec::new()).map(|_| ())
}

pub fn timer_alba01_ontimer360000(ctx: &Ctx) -> Script {
    timer_alba01_run(ctx, TimerAlba01Step::OnTimer360000, Vec::new()).map(|_| ())
}

pub fn timer_alba01_ontimer600000(ctx: &Ctx) -> Script {
    timer_alba01_run(ctx, TimerAlba01Step::OnTimer600000, Vec::new()).map(|_| ())
}

pub fn timer_alba01_ontimer7800000(ctx: &Ctx) -> Script {
    timer_alba01_run(ctx, TimerAlba01Step::OnTimer7800000, Vec::new()).map(|_| ())
}

fn breeder_taab_ep13_alba_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_alba_check = Val::from(0);
    if (ctx.var("ep13_ryu").get()?.number()? > 99 || ctx.var("ep13_start").get()?.number()? > 99) {
        if ctx.var("ep13_alba").get()?.number()? < 1 {
            ctx.lines_as("Taab", args!["How may I help you?"])?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("What is this place?:I'm here for a job.:No, thanks.")])? {
                1 => {
                    ctx.lines_as(
                        "Taab",
                        args![
                            "This is where we keep creatures",
                            "captured here in Ash Vacuum.",
                            "We study them to understand",
                            "their characteristics and habits."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Taab",
                        args![
                            "Of course, scholars and",
                            "specialists perform the studies.",
                            "I'm here to tame and breed",
                            "dangerous monsters."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Taab",
                        args![
                            "...I hope I can carry them",
                            "around as my pets.",
                            "Hillstions are so cute.",
                            "You can't find such an",
                            "animal on the mainland,",
                            "you know?"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    if ctx.var("$@parttimeon").get()? == 0 {
                        ctx.lines_as(
                            "Taab",
                            args![
                                "I'm sorry, but I don't need any assistance right now.",
                                "I'll make an official anouncement if I need help.",
                                "Please come back then."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("$@parttimeslots").get()?.number()? < 5 {
                        ctx.var("$@parttimeslots").set((ctx.var("$@parttimeslots").get()? + Val::from(1)))?;
                        ctx.lines_as(
                            "Taab",
                            args![
                                "Welcome.",
                                "Your job is simple:",
                                "help me out by gathering",
                                "the animals' feed or by",
                                "cleaning their cages.",
                                "These are the available jobs."
                            ],
                        )?;
                        ctx.next()?;
                        if ctx.var("$@parttimeslots").get()? == 1 {
                            ctx.var("ep13_alba").set(Val::from(1))?;
                            ctx.call(Function::SetQuest, vec![Val::from(7042)])?;
                            ctx.lines_as(
                                "Taab",
                                args![
                                    "Please bring me 50 Fresh Fish.",
                                    "They're feed for Tatachoes.",
                                    "I'm fresh out, and I'll need to restock very soon."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Taab",
                                args![
                                    "You can find them from Tatachoes in the fields.",
                                    "Strange, isn't it?",
                                    "I don't know where they've caught the fish."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Taab",
                                args![
                                    "Thank you in advance.",
                                    "Please bring me feed for Tatachoes: ^4d4dff50 Fresh Fish^000000."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.var("$@parttimeslots").get()? == 2 {
                            ctx.var("ep13_alba").set(Val::from(2))?;
                            ctx.call(Function::SetQuest, vec![Val::from(7043)])?;
                            ctx.lines_as(
                                "Taab",
                                args![
                                    "I've just run out of",
                                    "feed for Cornuses.",
                                    "Can you please bring me",
                                    "^4d4dff30 Great Leaves and 30 Brown Roots^000000?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Taab",
                                args!["You can find them from", "Pinguiculas in the fields.", "Thank you in advance."],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.var("$@parttimeslots").get()? == 3 {
                            ctx.var("ep13_alba").set(Val::from(3))?;
                            ctx.call(Function::SetQuest, vec![Val::from(7044)])?;
                            ctx.lines_as(
                                "Taab",
                                args![
                                    "I've just run out of",
                                    "feed for Hillsrions.",
                                    "Scholars think these",
                                    "guys are members",
                                    "of the cat family.",
                                    "I've been trying to feed",
                                    "them various things."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Taab",
                                args![
                                    "This time... I'd like to",
                                    "try Monster's Feed.",
                                    "Can you please bring me",
                                    "^4d4dff20 Monster's Feeds and 30 Pet Foods^000000?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Taab",
                                args![
                                    "Thank you in advance.",
                                    "Don't forget the",
                                    "20 Monster's Feeds",
                                    "and 30 Pet Foods.",
                                    "I hope the Hallsrions",
                                    "will like them."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.var("$@parttimeslots").get()? == 4 {
                            ctx.var("ep13_alba").set(Val::from(4))?;
                            ctx.call(Function::SetQuest, vec![Val::from(7045)])?;
                            ctx.lines_as(
                                "Taab",
                                args![
                                    "This is perfect because",
                                    "I was going to try some",
                                    "new feed for Hillsrions.",
                                    "I tried Monster's Feed,",
                                    "but I don't know if they",
                                    "liked it or not."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Taab",
                                args![
                                    "I want to try Meat this time.",
                                    "Can you please bring me",
                                    "^4d4dff50 Meat^000000 and ^4d4dff30 Pet Foods^000000?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Taab",
                                args![
                                    "Thank you in advance.",
                                    "Don't forget the",
                                    "^4d4dff50 Meat^000000 and ^4d4dff30 Pet Foods^000000",
                                    "I really hope they'll like the Meat."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            ctx.var("ep13_alba").set(Val::from(5))?;
                            ctx.call(Function::SetQuest, vec![Val::from(7046)])?;
                            ctx.lines_as(
                                "Taab",
                                args![
                                    "Oh, I always wanted to put something warm on the floor for my creatures.",
                                    "I was thinking of using fur.",
                                    "I can also use fur to cover the cage during rainy days, you know?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Taab",
                                args![
                                    "Can you please bring me ^4d4dff30 scraps of fur^000000?",
                                    "It sounds easy, doesn't it?",
                                    "I think the fur of Tatachoes and Hillsrions will be perfect."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Taab",
                                args![
                                    "Of course, I'm not going to use them for the cages with the Tatachoes and Hillsrions.",
                                    "Those furs will be for the Cornuses."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Taab", args!["It seems Cornuses love being warm.", "The weather in this area is so strange that I'm having a hard time optimizing the temperature for each kind of creature."])?;
                            ctx.next()?;
                            ctx.lines_as("Taab", args!["Thank you in advance.", "Don't forget the 30 scraps of fur."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    } else {
                        ctx.lines_as(
                            "Taab",
                            args![
                                "Oh, I'm sorry, but no jobs are available right now.",
                                "Some other part-timers finished all the work."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Taab",
                            args![
                                "I'm sorry for the trouble I must have caused you to come here. Haha...",
                                "I'll see you next time."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                3 => {
                    ctx.lines_as(
                        "Taab",
                        args![
                            "Please stay far away from the cages.",
                            "Sometimes, the creatures try to escape their cages."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        } else if ctx.var("ep13_alba").get()? == 1 {
            if ctx.call(Function::CountItem, vec![Val::from(579)])?.number()? > 49 {
                ctx.lines_as(
                    "Taab",
                    args![
                        "Oh, thanks!",
                        "You brought them their food!",
                        "Just in time too: they look",
                        "like they're ready to chow down.",
                        "The Tatachoes will love these."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Taab",
                    args![
                        "I really want to give you",
                        "something in return...",
                        "But I have nothing",
                        "material to give you.",
                        "How about a spiritual reward?"
                    ],
                )?;
                ctx.next()?;
                ctx.call(Function::DelItem, vec![Val::from(579), Val::from(50)])?;
                ctx.var("ep13_alba").set(Val::from(6))?;
                {
                    ctx.call(Function::GetExperience, vec![Val::from(80000), Val::from(30000)])?;
                    ctx.lines(args!["^4d4dff You have received 80,000 EXP", "and 30,000 JEXP.^000000."])?;
                }
                ctx.call(Function::EraseQuest, vec![Val::from(7042)])?;
                ctx.call(Function::SetQuest, vec![Val::from(7047)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Taab",
                    args![
                        "Please bring me 50 Fresh Fish for Tatachoes.",
                        "You should hurry up because they get impatient when they're hungry."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else {
            if ctx.var("ep13_alba").get()? == 2 {
                if (ctx.call(Function::CountItem, vec![Val::from(7198)])?.number()? > 29
                    && ctx.call(Function::CountItem, vec![Val::from(7188)])?.number()? > 29)
                {
                    ctx.lines_as(
                        "Taab",
                        args![
                            "Oh, thanks!",
                            "You brought them their food!",
                            "Just in time too: they look",
                            "like they're ready to chow down.",
                            "The Cornus will love these."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Taab",
                        args![
                            "I really want to give you",
                            "something in return...",
                            "But I have nothing",
                            "material to give you.",
                            "How about a spiritual reward?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::DelItem, vec![Val::from(7198), Val::from(30)])?;
                    ctx.call(Function::DelItem, vec![Val::from(7188), Val::from(30)])?;
                    ctx.var("ep13_alba").set(Val::from(6))?;
                    {
                        ctx.call(Function::GetExperience, vec![Val::from(90000), Val::from(40000)])?;
                        ctx.lines(args!["^4d4dff You have received 90,000 EXP", "and 40,000 JEXP.^000000."])?;
                    }
                    ctx.call(Function::EraseQuest, vec![Val::from(7043)])?;
                    ctx.call(Function::SetQuest, vec![Val::from(7047)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Taab",
                        args![
                            "Don't forget the 30 Great",
                            "Leaves and 30 Brown Roots.",
                            "I need to stock as much",
                            "of them as possible",
                            "because the Cornuses",
                            "get hungry quite often."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            } else {
                if ctx.var("ep13_alba").get()? == 3 {
                    if (ctx.call(Function::CountItem, vec![Val::from(528)])?.number()? > 19
                        && ctx.call(Function::CountItem, vec![Val::from(537)])?.number()? > 29)
                    {
                        ctx.lines_as(
                            "Taab",
                            args![
                                "Oh, thanks!",
                                "You brought them their",
                                "food! Just in time too,",
                                "they look like they're",
                                "ready to chow down.",
                                "The Hillsrions will love these."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Taab",
                            args![
                                "I really want to give you",
                                "something in return...",
                                "But I have nothing",
                                "material to give you.",
                                "How about a spiritual reward?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::DelItem, vec![Val::from(528), Val::from(20)])?;
                        ctx.call(Function::DelItem, vec![Val::from(537), Val::from(30)])?;
                        ctx.var("ep13_alba").set(Val::from(6))?;
                        {
                            ctx.call(Function::GetExperience, vec![Val::from(80000), Val::from(30000)])?;
                            ctx.lines(args!["^4d4dff You have received 80,000 EXP", "and 30,000 JEXP.^000000."])?;
                        }
                        ctx.call(Function::EraseQuest, vec![Val::from(7044)])?;
                        ctx.call(Function::SetQuest, vec![Val::from(7047)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Taab",
                            args![
                                "Don't forget the",
                                "20 Monster's Feeds",
                                "and 30 Pet Foods.",
                                "They're for the Hillsrions."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                } else {
                    if ctx.var("ep13_alba").get()? == 4 {
                        if (ctx.call(Function::CountItem, vec![Val::from(517)])?.number()? > 49
                            && ctx.call(Function::CountItem, vec![Val::from(537)])?.number()? > 29)
                        {
                            ctx.lines(args![
                                "Oh, thanks!",
                                "You brought them their",
                                "food! Just in time too,",
                                "they look like they're",
                                "ready to chow down.",
                                "The Hillsrions will love these."
                            ])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Taab",
                                args![
                                    "I really want to give you",
                                    "something in return...",
                                    "But I have nothing",
                                    "material to give you.",
                                    "How about a spiritual reward?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.call(Function::DelItem, vec![Val::from(517), Val::from(50)])?;
                            ctx.call(Function::DelItem, vec![Val::from(537), Val::from(30)])?;
                            ctx.var("ep13_alba").set(Val::from(6))?;
                            {
                                ctx.call(Function::GetExperience, vec![Val::from(80000), Val::from(30000)])?;
                                ctx.lines(args!["^4d4dff You have received 80,000 EXP", "and 30,000 JEXP.^000000."])?;
                            }
                            ctx.call(Function::EraseQuest, vec![Val::from(7045)])?;
                            ctx.call(Function::SetQuest, vec![Val::from(7047)])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            ctx.lines_as(
                                "Taab",
                                args!["Don't forget the", "50 Meat and", "30 Pet Foods.", "They're for the Hillsrions."],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    } else {
                        if ctx.var("ep13_alba").get()? == 5 {
                            if ctx.call(Function::CountItem, vec![Val::from(6020)])?.number()? > 29 {
                                ctx.lines_as("Taab", args!["Oh, thanks!", "You brought me the Furs!", "Just in time too!"])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Taab",
                                    args![
                                        "I really want to give you",
                                        "something in return...",
                                        "But I have nothing",
                                        "material to give you.",
                                        "How about a spiritual reward?"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.call(Function::DelItem, vec![Val::from(6020), Val::from(30)])?;
                                ctx.var("ep13_alba").set(Val::from(6))?;
                                {
                                    ctx.call(Function::GetExperience, vec![Val::from(80000), Val::from(30000)])?;
                                    ctx.lines(args!["^4d4dff You have received 80,000 EXP", "and 30,000 JEXP.^000000."])?;
                                }
                                ctx.call(Function::EraseQuest, vec![Val::from(7046)])?;
                                ctx.call(Function::SetQuest, vec![Val::from(7047)])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                ctx.lines_as("Taab", args!["Don't forget the", "30 Furs. They're for the Cornus' cage."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        } else if ctx.var("ep13_alba").get()? == 6 {
                            l_alba_check = ctx.call(Function::CheckQuest, vec![Val::from(7047), ctx.constant("PLAYTIME")?])?;
                            if l_alba_check.clone() == -1 {
                                ctx.lines_as(
                                    "Taab",
                                    args![
                                        "Thank you for",
                                        "helping me last time.",
                                        "We have a constant",
                                        "flow of part-time work.",
                                        "I hope you'll come by",
                                        "to help me again."
                                    ],
                                )?;
                                ctx.call(Function::EraseQuest, vec![Val::from(7047)])?;
                                ctx.var("ep13_alba").set(Val::from(0))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if (l_alba_check.clone() == 0 || l_alba_check.clone() == 1) {
                                ctx.lines_as(
                                    "Taab",
                                    args![
                                        "I've got enough feed",
                                        "and supplies to last a while.",
                                        "Thank you for your",
                                        "help last time."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Taab",
                                    args![
                                        "I don't think I need",
                                        "any assistance for now...",
                                        ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(",")),
                                        "why don't you go rest?"
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if l_alba_check.clone() == 2 {
                                ctx.lines_as(
                                    "Taab",
                                    args![
                                        "Thank you for",
                                        "helping me last time.",
                                        "We have a constant",
                                        "flow of part-time work.",
                                        "I hope you'll come by",
                                        "to help me again."
                                    ],
                                )?;
                                ctx.call(Function::EraseQuest, vec![Val::from(7047)])?;
                                ctx.var("ep13_alba").set(Val::from(0))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                    }
                }
            }
        }
    } else {
        ctx.lines_as(
            "Taab",
            args![
                "Please step aside! It's dangerous",
                "to get too close to those",
                "creatures. You don't look like a",
                "member of the expedition.",
                "I guess you're not allowed",
                "to be here. Please leave."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn breeder_taab_ep13_alba(ctx: &Ctx) -> Script {
    breeder_taab_ep13_alba_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum HillsrionAlba01Step {
    Start,
    OnEnable,
    OnDisable,
    OnTouch,
}

fn hillsrion_alba01_run(ctx: &Ctx, mut step: HillsrionAlba01Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            HillsrionAlba01Step::Start => {
                step = HillsrionAlba01Step::OnEnable;
                continue 'machine;
            }
            HillsrionAlba01Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Hillsrion#alba01")])?;
                return Err(Stop::End);
            }
            HillsrionAlba01Step::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("Hillsrion#alba01")])?;
                return Err(Stop::End);
            }
            HillsrionAlba01Step::OnTouch => {
                ctx.lines(args![
                    "It is hissing in a low voice.",
                    "Sometimes it purrs, too.",
                    "It must be in a happy mood."
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn hillsrion_alba01(ctx: &Ctx) -> Script {
    hillsrion_alba01_run(ctx, HillsrionAlba01Step::Start, Vec::new()).map(|_| ())
}

pub fn hillsrion_alba01_onenable(ctx: &Ctx) -> Script {
    hillsrion_alba01_run(ctx, HillsrionAlba01Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn hillsrion_alba01_ondisable(ctx: &Ctx) -> Script {
    hillsrion_alba01_run(ctx, HillsrionAlba01Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn hillsrion_alba01_ontouch(ctx: &Ctx) -> Script {
    hillsrion_alba01_run(ctx, HillsrionAlba01Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum TatachoAlba02Step {
    Start,
    OnEnable,
    OnDisable,
    OnTouch,
}

fn tatacho_alba02_run(ctx: &Ctx, mut step: TatachoAlba02Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            TatachoAlba02Step::Start => {
                step = TatachoAlba02Step::OnEnable;
                continue 'machine;
            }
            TatachoAlba02Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Tatacho#alba02")])?;
                return Err(Stop::End);
            }
            TatachoAlba02Step::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("Tatacho#alba02")])?;
                return Err(Stop::End);
            }
            TatachoAlba02Step::OnTouch => {
                ctx.lines_as(
                    "Taab",
                    args![
                        "Oh, please don't disturb",
                        "it's sleep. It hates that.",
                        "By the way, doesn't it",
                        "remind you of something?",
                        "I mean, like maybe",
                        "a vagrant or a hobo?"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn tatacho_alba02(ctx: &Ctx) -> Script {
    tatacho_alba02_run(ctx, TatachoAlba02Step::Start, Vec::new()).map(|_| ())
}

pub fn tatacho_alba02_onenable(ctx: &Ctx) -> Script {
    tatacho_alba02_run(ctx, TatachoAlba02Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn tatacho_alba02_ondisable(ctx: &Ctx) -> Script {
    tatacho_alba02_run(ctx, TatachoAlba02Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn tatacho_alba02_ontouch(ctx: &Ctx) -> Script {
    tatacho_alba02_run(ctx, TatachoAlba02Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum CornusAlba03Step {
    Start,
    OnEnable,
    OnDisable,
    OnTouch,
}

fn cornus_alba03_run(ctx: &Ctx, mut step: CornusAlba03Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            CornusAlba03Step::Start => {
                step = CornusAlba03Step::OnEnable;
                continue 'machine;
            }
            CornusAlba03Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Cornus#alba03")])?;
                return Err(Stop::End);
            }
            CornusAlba03Step::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("Cornus#alba03")])?;
                return Err(Stop::End);
            }
            CornusAlba03Step::OnTouch => {
                ctx.lines_as(
                    "Taab",
                    args![
                        "Oh, please don't get",
                        "too close to it or try to",
                        "feed something strange.",
                        "It's pretty sensitive..."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn cornus_alba03(ctx: &Ctx) -> Script {
    cornus_alba03_run(ctx, CornusAlba03Step::Start, Vec::new()).map(|_| ())
}

pub fn cornus_alba03_onenable(ctx: &Ctx) -> Script {
    cornus_alba03_run(ctx, CornusAlba03Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn cornus_alba03_ondisable(ctx: &Ctx) -> Script {
    cornus_alba03_run(ctx, CornusAlba03Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn cornus_alba03_ontouch(ctx: &Ctx) -> Script {
    cornus_alba03_run(ctx, CornusAlba03Step::OnTouch, Vec::new()).map(|_| ())
}

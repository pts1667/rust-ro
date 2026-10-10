use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn brasilis_girl_bra_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_card: Vec<Val> = Vec::new();
    if !(ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])?.is_true()) {
        ctx.lines(args![
            "- wait a second!! -",
            "- you have too many items -",
            "- so you can't get any more items. -",
            "- make your body lighter -",
            "- then try again. -"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("brazil_regia").get()? == 2 {
        ctx.lines_as(
            "Distant Sound",
            args!["Jasira!!!", "Where are you going again?!!", "come back~, please!!"],
        )?;
        ctx.next()?;
        ctx.lines_as("Brasilis Girl", args!["Mom, I have to go out!!"])?;
        ctx.next()?;
        ctx.lines_as("Distant Sound", args!["No way~!! You shouldn't!!"])?;
        ctx.next()?;
        ctx.lines_as("Brasilis Girl", args!["Gosh.. today also failed."])?;
        ctx.next()?;
        ctx.lines_as(
            "Brasilis Girl",
            args![
                "......",
                "What's up? Why are you looking at me?",
                "I don't want to be a showgirl!! Get out!!"
            ],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Nothing, sorry.:What's wrong?")])?) == 1 {
            ctx.lines_as("Brasilis Girl", args!["I am so sad!!!"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as("Brasilis Girl", args!["It's not your business.", "You are just an outsider!"])?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("How rude!:Just trying to help.")])?) == 1 {
            ctx.lines_as("Brasilis Girl", args!["What's it matter to you that I'm rude??!!"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["I know that I'm just passing by but I might be able to help you. What do you think?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["This kind of meeting could be more than just a coincidence."],
        )?;
        ctx.next()?;
        ctx.lines_as("Brasilis Girl", args!["......................"])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Hmm can you tell me your name?"],
        )?;
        ctx.next()?;
        ctx.lines_as("Brasilis Girl", args!["ja...", "Jasira.", "My name is Jasira."])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Nice name~.", "Jasira what's going on?"],
        )?;
        ctx.next()?;
        ctx.lines_as("Jasira", args!["............."])?;
        ctx.next()?;
        ctx.lines_as("Jasira", args!["I have to meet 'Jasi' but I can't go out...."])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["who is Jasi?", "Your.... lover?"],
        )?;
        ctx.next()?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
        ctx.lines_as("Jasira", args!["l...o...v...e...lover??!!", "No way~"])?;
        ctx.next()?;
        ctx.lines_as("Jasira", args!["If he is my lover, it would be great... but..."])?;
        ctx.next()?;
        ctx.lines_as("Jasira", args!["Jasi is......", "the great moon."])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["The moon?", "Maybe... are you talking about the moon from the story?"],
        )?;
        ctx.next()?;
        ctx.lines_as("Jasira", args!["Yeah!", "Dear Jasi is from the moon from the sky!"])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Why are you thinking like that?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Jasira",
            args!["Cuz' Jasi is really gorgeous and the most important thing is he is taking care of the water lily in Brasilis."],
        )?;
        ctx.next()?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_HUK")?,
                Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
            ],
        )?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Brasilis water lily??!!", "Isn't it the uniqe flower?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Jasira",
            args![
                "Right. It's a really mysterious flower and difficult to find.",
                "But around Jasi there are lots of water lilies.",
                "That's why I believe Jasi is the moon."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Where is Jasi?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Jasira",
            args![
                "He is deep inside the Jungle.",
                "As you can see I am so weak so, I've been staying home. But once, I was strong enough to leave this village."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Jasira",
            args![
                "I just wandered the jungle and fell down somewhere and that's where I saw him.",
                "He was so nice. He helped heal me and guided me back home.",
                "That was really really great time."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Jasira",
            args![
                "Since I came back home, my parents punished me.",
                "I can understand why they are worrying but i missed Jasi a lot!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Why don't you meet him after recovering your strength?"],
        )?;
        ctx.next()?;
        ctx.lines_as("Jasira", args![".................", "I wanna see him right now..."])?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Help Jasira.:Ignore her.")])?) == 2 {
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Sorry I can't help you. Cheer up!"],
            )?;
            ctx.next()?;
            ctx.lines_as("Jasira", args!["Crying........"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Jasira I came here to find the Brasilis water lily.",
                "Don't you think fate has brought us together?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["If you tell me how to find Jasi, I can help you."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Jasira",
            args!["Really? But I don't know exactly how to get there. I was just wandering around when I met him."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Can't you remember anything?", "If you know something you've gotta tell me."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Jasira",
            args!["Let's see... I was wandering around a waterfall then fell down into the water then I was sucked into somewhere."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Good, that's better than nothing! I will look for a similar place."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Jasira",
            args!["I gave you your information, so can you do me a favor?", "It's really simple..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Jasira",
            args![
                "I'd like to give a delicious fruit.",
                "The place where Jas seemed cozy but I didn't see any food around... So if he sees a yummy fruit he will be happy!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Jasira",
            args![
                "Give him 10 Banana and tell him that I really miss him.",
                "Sorry for ignoring you before. Please, only you can help me!"
            ],
        )?;
        ctx.var("brazil_regia").set(Val::from(3))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(2201), Val::from(2202)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if (ctx.var("brazil_regia").get()? == 3 || ctx.var("brazil_regia").get()? == 4) {
            ctx.lines_as(
                "Jasira",
                args![
                    "If you meet Jasi, give him 10 Bananas.",
                    "Let's see... I was wandering around a waterfall then fell down into the water then I was sucked into somewhere."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Jasira",
                args![
                    "If you can't find the way, go up to the waterfall and ask the kids in Brasilis village.",
                    "I heard one of the children went to a strange place before. He might've gone to the same place as me!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("brazil_regia").get()? == 5 {
            ctx.lines_as(
                "Jasira",
                args!["Did you meet Jasi?", "Did you talk about me?", "You didn't? Uh? Stupid! Gosh~!"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Hey girl~ you've got a short temper.", "I did see him and I talked about you!"],
            )?;
            ctx.next()?;
            ctx.lines_as("Jasira", args!["Did you?", "What did he say?", "Does he remember me?"])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "That you have a really good heart~",
                    "I told him that you will try to meet him when your condition gets better."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Jasira",
                args!["Yeahhhhh!!", "Thank you! You are more reliable than I thought you would be."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "Anyway, I'm looking for a fruit that's brown and has a hard shell.",
                    "It has juice inside and can be used as a cup to drink out of."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Jasira",
                args!["Duh! You mean a coconut right?!", "They're everywhere here in Brasilis."],
            )?;
            ctx.next()?;
            ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Thanks Jasira!"])?;
            ctx.var("brazil_regia").set(Val::from(6))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(2204), Val::from(2205)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if (ctx.var("brazil_regia").get()? == 6 || ctx.var("brazil_regia").get()? == 7) {
            ctx.lines_as(
                "Jasira",
                args![
                    "I should take care of my strength by myself!",
                    "I can't just lie in my bed forever. Don't you agree?"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("brazil_regia").get()? == 8 {
            ctx.lines_as("Jasira", args!["Uh? Why have you come back?"])?;
            ctx.next()?;
            ctx.mes("- You tell her what Jasi told you to tell her -")?;
            ctx.next()?;
            ctx.lines_as(
                "Jasira",
                args!["Oh... really?", "Did he say that?", "Gosh! Gosh!!!", "Kkkkkaaaaa - !!"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Thanks to you, I was able to get a flower.", "Thanks a lot!!"],
            )?;
            ctx.next()?;
            ctx.lines_as("Jasira", args!["Wooow. It's so beautiful."])?;
            ctx.next()?;
            ctx.lines_as("Jasira", args!["Ah... can I see it for a second?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Jasira",
                args![
                    "Surprise~!!",
                    "I've been working on this hat while you were gone and now it's complete with the water lily flower!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Jasira", args!["I know, I know! I'm the best..."])?;
            ctx.call(Function::DelItem, vec![Val::from(7553), Val::from(1)])?;
            ctx.var("brazil_regia").set(Val::from(9))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(2206), Val::from(2207)])?;
            let base = Val::from(0).number()?;
            runtime::local_set(&mut l_card, &Val::from(base + 0), Val::from(4195), false);
            runtime::local_set(&mut l_card, &Val::from(base + 1), Val::from(4177), false);
            runtime::local_set(&mut l_card, &Val::from(base + 2), Val::from(4188), false);
            ctx.call(
                Function::GetItem2,
                vec![
                    Val::from(5302),
                    Val::from(1),
                    Val::from(1),
                    Val::from(0),
                    Val::from(0),
                    runtime::local_get(&l_card, &ctx.call(Function::Rand, vec![Val::from(3)])?, false),
                    Val::from(0),
                    Val::from(0),
                    Val::from(0),
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("brazil_regia").get()?.number()? > 8 {
            ctx.lines_as(
                "Jasira",
                args![
                    "I just need to get a little bit stronger!",
                    "I can't just lie in bed forever. My Jasi is waiting for me~"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Distant Sound",
                args!["Jasira!!!", "Where are you going again?!!", "Come back~, please!!"],
            )?;
            ctx.next()?;
            ctx.lines_as("Brasilis Girl", args!["Please mom~!", "Please let me go!"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    return Err(Stop::End);
}

pub fn brasilis_girl_bra(ctx: &Ctx) -> Script {
    brasilis_girl_bra_body(ctx, Vec::new()).map(|_| ())
}

fn brasilis_girl_bra_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("brazil_regia").get()? == 2 {
        ctx.call(Function::Emotion, vec![ctx.constant("ET_FRET")?])?;
    }
    return Err(Stop::End);
}

pub fn brasilis_girl_bra_ontouch(ctx: &Ctx) -> Script {
    brasilis_girl_bra_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn recluse_bra_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("brazil_regia").get()? == 3 {
        ctx.lines_as("Recluse", args!["Oh, I haven't seen another person in such a long time."])?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Keep going.:Are you the moon?")])?) == 1 {
            ctx.lines_as("Recluse", args!["You don't have specific business with me."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Recluse",
            args![
                "Moon?",
                "My name is Jasi.",
                "My family has worked to take care of the water lily from generation to generation."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Jasi", args!["Basically the Brasilis water lily is too shy to appear in front of people so, they only bloom in rare places. I guess they like it here, though, that's why I've been staying here for such a long time."])?;
        ctx.next()?;
        ctx.lines_as(
            "Jasi",
            args!["My family has taken care of the water lily calmly to prevent harm from people's hand or monsters."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Do you remember a girl named Jasi."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Jasi",
            args![
                "Ja...si..........",
                "Ah!! a hurry scurry girl. ",
                "Gosh.. I was in trouble due to that girl."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Trouble?"])?;
        ctx.next()?;
        ctx.lines_as("Jasi", args!["One day a young lady appeared with lots of scars so I helped her. Then suddenly she tried to pick up the water lily and asked me to accept her as my wife or make her into a water lily what a nutcase!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Jasi",
            args![
                "I was barely able to calm down and send her to the village.",
                "My life is that water lily so I didn't want anything embarrassing to happen."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "This is a gift from Jasira to say sorry for that time.",
                "She is really sad that can't come here by herself due to private difficulties."
            ],
        )?;
        ctx.next()?;
        if ctx.call(Function::CountItem, vec![Val::from(513)])?.number()? < 10 {
            ctx.lines_as("Jasi", args!["What are you saying?"])?;
            ctx.next()?;
            ctx.mes("- Oh yeah, I forgot to bring 10 Bananas -")?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Jasi",
            args!["Ah! Bananas! Wow it's been a long time. She's pretty considerate isn't she?"],
        )?;
        ctx.next()?;
        ctx.lines_as("Jasi", args!["Anyway is that all the business you have with me?"])?;
        ctx.next()?;
        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Umm honestly I was wondering to find water lily and met you by coincidence. Jasira told me her sad story so that's what led me here."])?;
        ctx.next()?;
        ctx.lines_as(
            "Jasi",
            args![
                "I got it.",
                "As you can see, there are lots of Brasilis water lily around here.",
                "If you make sure that you won't destroy them you can appreciate them as you wish."
            ],
        )?;
        ctx.var("brazil_regia").set(Val::from(4))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(2202), Val::from(2203)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("brazil_regia").get()? == 4 {
        ctx.lines_as("Jasi", args!["Did you enjoy the water lily?"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("brazil_regia").get()? == 5 {
        ctx.lines_as("Jasi", args!["I forgot what the name of that fruit was..."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("brazil_regia").get()? == 6 {
        if ctx.call(Function::CountItem, vec![Val::from(11515)])?.number()? < 5 {
            ctx.lines_as("Jasi", args!["I forgot what the name of that fruit was..."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as("Jasi", args!["Did you find the fruit?", "Oh right this is....?"])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["It's called a 'coconut'."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Jasi",
                args![
                    "Ahah! COCONUT!!",
                    "Now I remember thank you very much. I can't remember the last time I had this fruit."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Jasi",
                args!["I guess I should keep my promise.", "You can take one Water lily."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Jasi",
                args![
                    "I hope the Brasilis water lily will understand me.",
                    "You better grab the flower while you have a chance~"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Jasi",
                args![
                    "Oh, can you tell that girl Jasira something for me?",
                    "Tell her that I am not the moon from the story, but I want to become the moon to shine only for her."
                ],
            )?;
            ctx.call(Function::DelItem, vec![Val::from(11515), Val::from(5)])?;
            ctx.var("brazil_regia").set(Val::from(7))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(2205), Val::from(2206)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        ctx.lines_as("Jasi", args!["The flowers blooming from the Water lily today is wonderful."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    return Err(Stop::End);
}

pub fn recluse_bra(ctx: &Ctx) -> Script {
    recluse_bra_body(ctx, Vec::new()).map(|_| ())
}

fn recluse_bra_ontouchnpc_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::Warp, vec![Val::from("this"), Val::from(67), Val::from(215)])?;
    return Err(Stop::End);
}

pub fn recluse_bra_ontouchnpc(ctx: &Ctx) -> Script {
    recluse_bra_ontouchnpc_body(ctx, Vec::new()).map(|_| ())
}

fn water_lily_bra_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("brazil_regia").get()? == 4 {
        ctx.mes(
            "An unusual Water lily is blooming here. You can't stop staring at it, knowing that few people have seen this flower bloom.",
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Pick up the flower.:Keep gazing.")])?) == 2 {
            ctx.mes("- You can't avoid staring at it's beauty. -")?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as("Jasi", args!["Uh! What are you doing??!!"])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["There is a person who really needs this flower, can I just take one of 'em?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Jasi",
            args![
                "As I said earlier, I am the guardian of this water lily.",
                "I can't just stand by here and watch you pluck even a single flower from it."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Hey man~ I brought these delicious fruits for you... try it! They are really well matured and fresh bananas."],
        )?;
        ctx.next()?;
        if ctx.call(Function::CountItem, vec![Val::from(513)])?.number()? < 10 {
            ctx.lines_as("Jasi", args!["What are you saying?"])?;
            ctx.next()?;
            ctx.mes("- Oh yeah, I forgot to bring 10 Bananas -")?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as("Jasi", args!["Hmm... It's been so long since I've had this fruit."])?;
        ctx.next()?;
        ctx.lines_as("Jasi", args!["I will just try one. That's all."])?;
        ctx.next()?;
        ctx.lines(args!["- munch -", "- mumble mumble mumble -"])?;
        ctx.call(
            Function::NpcSpecialEffect,
            vec![ctx.constant("EF_POTION7")?, ctx.constant("AREA")?, Val::from("Recluse#bra")],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Jasi",
            args!["Uh, this taste... is!", "I remember my mom baking these into a tasty bread!"],
        )?;
        ctx.next()?;
        ctx.lines_as("Jasi", args!["It makes me miss my childhood."])?;
        ctx.next()?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_HUK")?,
                ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Recluse#bra")])?,
            ],
        )?;
        ctx.lines_as(
            "Jasi",
            args![
                "Hoho!!!!",
                "I've been here for as long as I can remember...",
                "I don't have enough time to even do simple things like eat delicious fruit."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Jasi",
            args!["It was a really delicious banana.", "But rules are rules!", "I must do my duty."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Please! I just need one flower~ What can I do to convince you?"],
        )?;
        ctx.next()?;
        ctx.lines_as("Jasi", args!["Rules are rules, what do you want from me?"])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Didn't that banana remind you of your childhood? What can I get for you?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Jasi",
            args![
                "Now that you mention it, there is one fruit that I really miss.",
                "It was my favorite when I was young but I don't remember what it was called."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Jasi", args!["It's brown and has a hard shell around it. It has juice inside and you can use it as a cup when you're done eating the fruit.", "Do you know what it is?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Jasi",
            args!["If you bring 5 of those things, I will reconsider your suggestion."],
        )?;
        ctx.call(Function::DelItem, vec![Val::from(513), Val::from(10)])?;
        ctx.var("brazil_regia").set(Val::from(5))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(2203), Val::from(2204)])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Ok so I have to bring 5 fruits with hard shells.", "Hmm what is it?"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("brazil_regia").get()? == 5 {
        ctx.lines_as(
            "Jasi",
            args!["It was my favorite when I was young but I don't remember what it was called."],
        )?;
        ctx.next()?;
        ctx.lines_as("Jasi", args!["It's brown and has a hard shell around it. It has juice inside and you can use it as a cup when you're done eating the fruit.", "Do you know what it is?"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("brazil_regia").get()? == 7 {
        if !(ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])?.is_true()) {
            ctx.lines(args![
                "- wait a second!! -",
                "- you have too many items -",
                "- so you can't get any more items. -",
                "- make your body lighter -",
                "- then try again. -"
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.mes("- You take a beautiful water lily carefully in your hands. -")?;
        ctx.var("brazil_regia").set(Val::from(8))?;
        ctx.call(Function::GetItem, vec![Val::from(7553), Val::from(1)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn water_lily_bra(ctx: &Ctx) -> Script {
    water_lily_bra_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum PedroBraStep {
    Start,
    OnTalk,
    HoistEnd1,
}

fn pedro_bra_run(ctx: &Ctx, mut step: PedroBraStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            PedroBraStep::Start => {
                if ctx.var("brazil_ghost").get()? == 0 {
                    step = PedroBraStep::OnTalk;
                    continue 'machine;
                } else if ctx.var("brazil_ghost").get()? == 1 {
                    ctx.lines_as("Pedro", args!["Do you wanna hear the magic words again?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Pedro",
                        args![
                            "'^3131FFMother the door won't open!^000000'",
                            "'^FF0000Turn the key as many times as there are colors in the rainbow.^000000'",
                            "'^3131FFMother the water is flooding!^000000'",
                            "'^FF0000If the moon disappears 3 times, don't worry.^000000'",
                            "'^3131FFMother the drought has started!^000000'",
                            "'^FF0000Don't worry, the waterfall will help it.^000000'",
                            "'^3131FFMother where are my friends?^000000'",
                            "'^FF0000Your 7 friends are sleeping. now it's time to wake them.^000000'",
                            "'^3131FFWhere are you mom?^000000'"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as("Pedro", args!["I wonder what I need to do to have a statue made of me?"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                step = PedroBraStep::HoistEnd1;
                continue 'machine;
            }
            PedroBraStep::OnTalk => {
                ctx.lines_as("Pedro", args!["Wow it's really a great statue!"])?;
                ctx.next()?;
                ctx.lines_as("Mariana", args!["It is, isn't it?", "This statue is called Verass Monument."])?;
                ctx.next()?;
                ctx.lines_as("Mariana", args!["A long time ago there was a really brave adventurer named Verass, thanks to his dedicated exploration, Brasilis was able to develop into this great city."])?;
                ctx.next()?;
                ctx.lines_as("Pedro", args!["Awesome!!", "i wanna become a real man like Verass."])?;
                ctx.next()?;
                ctx.lines_as("Mariana", args!["Pedro, you can become whatever you want."])?;
                ctx.next()?;
                ctx.lines_as("Pedro", args!["Mariana is so smart, isn't she? hehe."])?;
                ctx.next()?;
                ctx.lines_as("Fabio", args!["Ooooh! You love her don't you!"])?;
                ctx.next()?;
                ctx.lines_as("Daniel", args!["Wooooaaaa Pedro and Mari sitting in a tree!"])?;
                ctx.next()?;
                ctx.lines_as("Fabio", args!["Woooo k-i-s-s-i-n-g~!!!"])?;
                ctx.next()?;
                ctx.lines_as("Daniel", args!["Nya nya nya!"])?;
                ctx.next()?;
                ctx.lines_as("Fabio", args!["Hahahahaha."])?;
                ctx.next()?;
                ctx.lines_as("Pedro", args!["Stop acting like babies!"])?;
                ctx.next()?;
                ctx.lines_as("Mariana", args!["Boys~!"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Daniel",
                    args!["Yah yah...", "Hey guys, did you hear that something happened a few days ago?"],
                )?;
                ctx.next()?;
                ctx.lines_as("Mariana", args!["Oh yeah~ I heard that something really scary happened."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Fabio",
                    args!["Uh, yeah that's why Mariana got scared of going ot the bathroom for 3 days and everything was stinky. Ewwww~"],
                )?;
                ctx.next()?;
                ctx.lines_as("Daniel", args!["Ha ha ha! Smelly Mari!"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Mariana",
                    args![
                        "I hate you~!",
                        "Stop spreading rumors about me. I'm not scared of the bathroom.",
                        "Pedro, do you think that I stink?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Pedro", args!["Uh? Uh?", "N......no... no way.", "Hey guys~ be nice to her~"])?;
                ctx.next()?;
                ctx.lines_as("Fabio", args!["kkkickkkkkkkkick"])?;
                ctx.next()?;
                ctx.lines_as("Daniel", args!["kkkickkkkkkkk"])?;
                ctx.next()?;
                if Val::from(runtime::select_values(ctx, &[Val::from("Walk by.:Ask about the gossip.")])?) == 1 {
                    ctx.lines_as("Fabio", args!["Mariana~ smells~ Nya nya~"])?;
                    ctx.next()?;
                    ctx.lines_as("Daniel", args!["Oh man you stink too~! Nya nya~"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as("Fabio", args!["Haven't you heard?", "The ghost story in the art museum."])?;
                ctx.next()?;
                ctx.lines_as("Daniel", args!["Ooohhhh! Scary~~~!"])?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("Can you tell me more?")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as(
                    "Fabio",
                    args![
                        "A coupla days ago we went to the art museum for a picnic at school.",
                        "You know nothing special, just a ordinary field trip."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Fabio",
                    args!["Museums are boring so me and some friends snuck away from the group~!"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Fabio",
                    args!["That's when we heard a scream echoing through the whole museum."],
                )?;
                ctx.next()?;
                ctx.lines_as("Daniel", args!["kkakkakkaaaah!!", "kkieeeeeeh!", "kehkeh.."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Mariana",
                    args![
                        "I heard the scream too...",
                        "You boys are always making noises where you're not supposed to."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Pedro",
                    args!["What else are we supposed to do? If we don't do it someone else will."],
                )?;
                ctx.next()?;
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_FRET")?,
                        ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Mariana#bra")])?,
                    ],
                )?;
                ctx.lines_as("Mariana", args!["Argh~ Boys are so frustrating sometimes."])?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("So then what happened?")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as(
                    "Fabio",
                    args!["Daniel and me guessed something weird was goin' on so we ran to where we thought the screams were comin' from."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Fabio",
                    args![
                        "They were coming from the bathroom.",
                        "Some kids got so scared that they started screaming too and closing their eyes. It got pretty bad."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Daniel",
                    args!["I think you pissed or pooped your pants. It smelled freakin' gross."],
                )?;
                ctx.next()?;
                ctx.lines_as("Fabio", args!["Nah uh~ Your mom pissed her pants~ Nyah!"])?;
                ctx.next()?;
                ctx.lines_as("Daniel", args!["Nah uh~ You~ pissed your pants~"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Fabio",
                    args!["Anyway, so yeah, anyway that's how the rumor of the ghost in the museum started."],
                )?;
                ctx.next()?;
                ctx.lines_as("Daniel", args!["Liar, there's no such thing as ghosts~"])?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("So was it a ghost?")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as(
                    "Fabio",
                    args![
                        "How should I know?",
                        "No one could say they saw one and no one wanted to get in trouble from the teachers."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Pedro",
                    args!["I heard if you say special magic words that the ghost will come out."],
                )?;
                ctx.next()?;
                ctx.lines_as("Daniel", args!["Quit butting into our conversation Pedro."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Fabio",
                    args!["Yah, what are you talking about, Pedro?", "So did you see the ghost?"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Pedro",
                    args![
                        "N... no. I'm scared of ghosts.",
                        "But my friends said they saw one and they're not liars."
                    ],
                )?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("Did anyone tell you the magic words?")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as(
                    "Pedro",
                    args!["I heard it in a kind of song.", "the special magic words are..."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Pedro",
                    args![
                        "'^3131FFMother the door won't open!^000000'",
                        "'^FF0000Turn the key as many times as there are colors in the rainbow.^000000'",
                        "'^3131FFMother the water is flooding!^000000'",
                        "'^FF0000If the moon disappears 3 times, don't worry.^000000'",
                        "'^3131FFMother the drought has started!^000000'",
                        "'^FF0000Don't worry, the waterfall will help it.^000000'",
                        "'^3131FFMother where are my friends?^000000'",
                        "'^FF0000Your 7 friends are sleeping. now it's time to wake them.^000000'",
                        "'^3131FFWhere are you mom?^000000'"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Mariana", args!["Umm it seems like a riddle."])?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("Wanna help me find this ghost?")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as("Pedro", args!["You're on your own pal~."])?;
                ctx.next()?;
                ctx.lines_as("Mariana", args!["I don't like scary things!"])?;
                ctx.next()?;
                ctx.lines_as("Fabio", args!["Pfft, I can't believe you're gonna believe that story."])?;
                ctx.next()?;
                ctx.lines_as("Daniel", args!["I'll do whatever Fabio does, as always!"])?;
                ctx.next()?;
                ctx.lines_as("Fabio", args!["Maybe you're just scared..."])?;
                ctx.var("brazil_ghost").set(Val::from(1))?;
                ctx.call(Function::SetQuest, vec![Val::from(2208)])?;
                ctx.close_window()?;
                return Err(Stop::End);
                step = PedroBraStep::HoistEnd1;
                continue 'machine;
            }
            PedroBraStep::HoistEnd1 => {
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn pedro_bra(ctx: &Ctx) -> Script {
    pedro_bra_run(ctx, PedroBraStep::Start, Vec::new()).map(|_| ())
}

pub fn pedro_bra_ontalk(ctx: &Ctx) -> Script {
    pedro_bra_run(ctx, PedroBraStep::OnTalk, Vec::new()).map(|_| ())
}

fn mariana_bra_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("brazil_ghost").get()? == 0 {
        ctx.call(Function::DoEvent, vec![Val::from("Pedro#bra::OnTalk")])?;
        return Err(Stop::End);
    } else if ctx.var("brazil_ghost").get()? == 1 {
        ctx.lines_as(
            "Mariana",
            args![
                "Can you guys stop talking about the ghosts?",
                "I've already got goosebumps all over."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as("Mariana", args!["Why do Fabio and Daniel always bother us?"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn mariana_bra(ctx: &Ctx) -> Script {
    mariana_bra_body(ctx, Vec::new()).map(|_| ())
}

fn fabio_bra_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("brazil_ghost").get()? == 0 {
        ctx.call(Function::DoEvent, vec![Val::from("Pedro#bra::OnTalk")])?;
        return Err(Stop::End);
    } else if ctx.var("brazil_ghost").get()? == 1 {
        ctx.lines_as("Fabio", args!["You still wasting your time with that ghost story?"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as("Fabio", args!["Mariana, wanna see something cool?"])?;
        ctx.next()?;
        ctx.lines_as("Mariana", args!["kkkkkkkaaaaaacck!! Bugs!! Get 'em away!"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn fabio_bra(ctx: &Ctx) -> Script {
    fabio_bra_body(ctx, Vec::new()).map(|_| ())
}

fn daniel_bra_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("brazil_ghost").get()? == 0 {
        ctx.call(Function::DoEvent, vec![Val::from("Pedro#bra::OnTalk")])?;
        return Err(Stop::End);
    } else if ctx.var("brazil_ghost").get()? == 1 {
        ctx.lines_as("Daniel", args!["Nyah nyah nyah~"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as("Daniel", args!["Keke Here~ I found more bugs~"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn daniel_bra(ctx: &Ctx) -> Script {
    daniel_bra_body(ctx, Vec::new()).map(|_| ())
}

fn door_bra_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_braspell_s = Val::from("");
    let mut l_chkspell = Val::from(0);
    let mut l_input = Val::from(0);
    let mut l_input_s = Val::from("");
    if ctx.var("brazil_ghost").get()?.number()? > 0 {
        ctx.mes("- A key is inserted in the locked door.-")?;
        ctx.next()?;
        'b1: {
            let subject1 = Val::from(runtime::select_values(ctx, &[Val::from("Turn the key.:Ignore it.")])?);
            let mut matched1 = false;
            let no_case1 = !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2));
            if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                matched1 = true;
            }
            if matched1 {
                ctx.mes("You start saying the first line of the magic words.")?;
                let (input, status) = runtime::input_text(ctx, None, None)?;
                l_input_s = input;
                ctx.next()?;
                ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args![l_input_s.clone()])?;
                ctx.next()?;
                l_braspell_s = Val::from("Mother the door won't open!");
                l_chkspell = runtime::compare(&l_braspell_s.clone(), &l_input_s.clone());
                if !(l_chkspell.clone().is_true()) {
                    ctx.mes("Seems like you said something wrong.")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("brazil_ghost").get()? == 2 {
                    ctx.lines_as(
                        "Sobbing Voice",
                        args!["'^FF0000Turn the key as many times as there are colors in the rainbow.^000000'"],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(
                        ctx,
                        &[Val::from("Open the door:Knock on the door.:Turn the key.:Take the key out.")],
                    )? {
                        1 => {
                            ctx.lines(args!["The door is locked.", "So nothing happens."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.mes("How many times should I try to knock?")?;
                            let (input, status) = runtime::input_number(ctx, Some(0), Some(999))?;
                            l_input = input;
                            ctx.next()?;
                            ctx.lines(args![
                                ((Val::from("You knocked on the door ") + l_input.clone()) + Val::from(" times."))
                            ])?;
                            ctx.next()?;
                            ctx.mes("But, nothing happens.")?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        3 => {
                            ctx.mes("How many times should I turn the key?")?;
                            let (input, status) = runtime::input_number(ctx, Some(0), Some(999))?;
                            l_input = input;
                            ctx.next()?;
                            if l_input.clone() == 7 {
                                ctx.mes("You turn the key 7 times.")?;
                                ctx.next()?;
                                ctx.lines(args!["Click! Click! Click!", "Click! Click! Click!", "Click...!"])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Distant Sound",
                                    args!["^FF0000kkkkhee- hihihihi!!!^000000", "You hear water flushing."],
                                )?;
                                ctx.next()?;
                                ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_VENOMDUST")?])?;
                                ctx.mes("Faint laughing can be heard off in the direction of the toilet.")?;
                                ctx.var("brazil_ghost").set(Val::from(3))?;
                                ctx.call(Function::ChangeQuest, vec![Val::from(2208), Val::from(60351)])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                ctx.lines(args![
                                    ((Val::from("You turned over the key ") + l_input.clone()) + Val::from(" times."))
                                ])?;
                                ctx.next()?;
                                ctx.mes("But nothing doesn't happened.")?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                        4 => {
                            ctx.mes("How many times should I insert the key into the door?")?;
                            let (input, status) = runtime::input_number(ctx, Some(0), Some(999))?;
                            l_input = input;
                            ctx.next()?;
                            ctx.lines(args![
                                ((Val::from("You inserted the key ") + l_input.clone()) + Val::from(" times."))
                            ])?;
                            ctx.next()?;
                            ctx.mes("But nothing happened.")?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else {
                    ctx.mes("Mother the door won't open!")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                matched1 = true;
            }
            if matched1 {
                ctx.mes("You do nothing.")?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    } else {
        ctx.mes("- A key is inserted in the locked door.-")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn door_bra(ctx: &Ctx) -> Script {
    door_bra_body(ctx, Vec::new()).map(|_| ())
}

fn toilet_bra_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_braspell_s = Val::from("");
    let mut l_chkspell = Val::from(0);
    let mut l_input = Val::from(0);
    let mut l_input_s = Val::from("");
    if ctx.var("brazil_ghost").get()?.number()? > 0 {
        ctx.mes("- Looks like an ordinary toilet -")?;
        ctx.next()?;
        if ctx.var("brazil_ghost").get()?.number()? > 6 {
            match runtime::select_values(ctx, &[Val::from("Flush the toilet.:Doing nothing.")])? {
                1 => {
                    ctx.mes("After flushing the toilet, you suddenly feel dizzy and are suddenly swept away somewhere.")?;
                    ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_WATERFALL_SMALL_T2_90")?])?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("bra_in01"), Val::from(206), Val::from(102)])?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.mes("The water in the toilet looks gross.")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        }
        'b2: {
            let subject2 = Val::from(runtime::select_values(ctx, &[Val::from("Use the toilet:Ignore.")])?);
            let mut matched2 = false;
            let no_case2 = !subject2.loosely_equals(&Val::from(1)) && !subject2.loosely_equals(&Val::from(2));
            if !matched2 && subject2.loosely_equals(&Val::from(1)) {
                matched2 = true;
            }
            if matched2 {
                ctx.mes("- What was the second line to that spell now? -")?;
                let (input, status) = runtime::input_text(ctx, None, None)?;
                l_input_s = input;
                ctx.next()?;
                ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args![l_input_s.clone()])?;
                ctx.next()?;
                l_braspell_s = Val::from("Mother the water is flooding!");
                l_chkspell = runtime::compare(&l_braspell_s.clone(), &l_input_s.clone());
                if !(l_chkspell.clone().is_true()) {
                    ctx.mes("Seems like you said something wrong.")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("brazil_ghost").get()? == 3 {
                    ctx.lines_as(
                        "Sobbing Voice",
                        args!["^FF0000If the moon disappears 3 times... don't worry.....^000000"],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Flush the toilet.:Close the lid.")])? {
                        1 => {
                            ctx.mes("How many times should I flush?")?;
                            let (input, status) = runtime::input_number(ctx, Some(0), Some(999))?;
                            l_input = input;
                            ctx.next()?;
                            if l_input.clone() == 3 {
                                ctx.mes("You flush the toilet 3 times.")?;
                                ctx.next()?;
                                ctx.lines(args!["qwaaaaaaaaa!", "kwaaaaaaaaaa!", "kwaaaaaaaaaaaaaaaaaaa!"])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Distant Sound",
                                    args![
                                        "^FF0000kkkkhee- hihihihi!!!^000000",
                                        "Suddenly the sink sounds like water is flowing freely from it."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_VENOMDUST")?])?;
                                ctx.mes("Faint laughing can be heard off in the direction of the faucet.")?;
                                ctx.var("brazil_ghost").set(Val::from(4))?;
                                ctx.call(Function::ChangeQuest, vec![Val::from(60351), Val::from(60352)])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                ctx.lines(args![
                                    ((Val::from("You flush the toilet ") + l_input.clone()) + Val::from(" times."))
                                ])?;
                                ctx.next()?;
                                ctx.mes("But nothing happens.")?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                        2 => {
                            ctx.lines(args!["You close the lid of the toilet.", "Nothing seems to be happening."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else {
                    ctx.mes("Nothing happens.")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            if !matched2 && subject2.loosely_equals(&Val::from(2)) {
                matched2 = true;
            }
            if matched2 {
                ctx.mes("You do nothing.")?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    } else {
        ctx.mes("- Looks like an ordinary toilet -")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn toilet_bra(ctx: &Ctx) -> Script {
    toilet_bra_body(ctx, Vec::new()).map(|_| ())
}

fn faucet_bra_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_braspell_s = Val::from("");
    let mut l_chkspell = Val::from(0);
    let mut l_input = Val::from(0);
    let mut l_input_s = Val::from("");
    if ctx.var("brazil_ghost").get()?.number()? > 0 {
        ctx.mes("- It seems like an ordinary faucet -")?;
        ctx.next()?;
        'b1: {
            let subject1 = Val::from(runtime::select_values(ctx, &[Val::from("Examine it.:Ignore.")])?);
            let mut matched1 = false;
            let no_case1 = !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2));
            if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                matched1 = true;
            }
            if matched1 {
                ctx.mes("- What was the next line to that spell now? -")?;
                let (input, status) = runtime::input_text(ctx, None, None)?;
                l_input_s = input;
                ctx.next()?;
                ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args![l_input_s.clone()])?;
                ctx.next()?;
                l_braspell_s = Val::from("Mother the drought has started!");
                l_chkspell = runtime::compare(&l_braspell_s.clone(), &l_input_s.clone());
                if !(l_chkspell.clone().is_true()) {
                    ctx.mes("Seems like you said something wrong.")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("brazil_ghost").get()? == 4 {
                    ctx.lines_as(
                        "Sobbing Voice",
                        args!["^FF0000Don't worry... the waterfall will help it....^000000"],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Tap on the faucet.:Turn on the water.")])? {
                        1 => {
                            ctx.mes("How many times will you tap the faucet?")?;
                            let (input, status) = runtime::input_number(ctx, Some(0), Some(999))?;
                            l_input = input;
                            ctx.next()?;
                            ctx.lines(args![
                                ((Val::from("You tap the faucet ") + l_input.clone()) + Val::from(" times."))
                            ])?;
                            ctx.next()?;
                            ctx.mes("But nothing happens.")?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.mes("How many times should I turn the water on?")?;
                            let (input, status) = runtime::input_number(ctx, Some(0), Some(999))?;
                            l_input = input;
                            ctx.next()?;
                            if l_input.clone() == 1 {
                                ctx.mes("You turn the faucet on once.")?;
                                ctx.next()?;
                                ctx.mes("swwwaaaaaaa-")?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Distant Sound",
                                    args!["^FF0000kkkkhee- hihihihi!!!^000000", "You see the carpet move."],
                                )?;
                                ctx.next()?;
                                ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_VENOMDUST")?])?;
                                ctx.mes("Faint laughing can be heard off in the direction of the carpet.")?;
                                ctx.var("brazil_ghost").set(Val::from(5))?;
                                ctx.call(Function::ChangeQuest, vec![Val::from(60352), Val::from(60353)])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                ctx.lines(args![
                                    ((Val::from("You turn the faucet on ") + l_input.clone()) + Val::from(" times."))
                                ])?;
                                ctx.next()?;
                                ctx.mes("But nothing happens.")?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                        _ => {}
                    }
                } else {
                    ctx.mes("Nothing happens.")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                matched1 = true;
            }
            if matched1 {
                ctx.mes("You do nothing.")?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    } else {
        ctx.mes("- It seems like an ordinary faucet -")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn faucet_bra(ctx: &Ctx) -> Script {
    faucet_bra_body(ctx, Vec::new()).map(|_| ())
}

fn carpet_bra_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_braspell_s = Val::from("");
    let mut l_chkspell = Val::from(0);
    let mut l_input = Val::from(0);
    let mut l_input_s = Val::from("");
    if ctx.var("brazil_ghost").get()?.number()? > 0 {
        ctx.mes("- A carpet with an intricate pattern on it -")?;
        ctx.next()?;
        'b1: {
            let subject1 = Val::from(runtime::select_values(ctx, &[Val::from("Examine it.:Ignore.")])?);
            let mut matched1 = false;
            let no_case1 = !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2));
            if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                matched1 = true;
            }
            if matched1 {
                ctx.mes("- What was the next line to that spell now? -")?;
                let (input, status) = runtime::input_text(ctx, None, None)?;
                l_input_s = input;
                ctx.next()?;
                ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args![l_input_s.clone()])?;
                ctx.next()?;
                l_braspell_s = Val::from("Mother where are my friends?");
                l_chkspell = runtime::compare(&l_braspell_s.clone(), &l_input_s.clone());
                if !(l_chkspell.clone().is_true()) {
                    ctx.mes("Seems like you said something wrong.")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("brazil_ghost").get()? == 5 {
                    ctx.lines_as(
                        "Sobbing Voice",
                        args!["^FF0000your 7 friends....are...sleeping... now it...'s time ....to wake them........^000000"],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Jump on the carpet.:Lie on the carpet.:Shake the carpet.")])? {
                        1 => {
                            ctx.mes("How many times should I jump?")?;
                            let (input, status) = runtime::input_number(ctx, Some(0), Some(999))?;
                            l_input = input;
                            ctx.next()?;
                            ctx.lines(args![
                                ((Val::from("You jump on the carpet ") + l_input.clone()) + Val::from(" times."))
                            ])?;
                            ctx.next()?;
                            ctx.mes("But nothing happens.")?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.mes("How many times should I lie on the carpet?")?;
                            let (input, status) = runtime::input_number(ctx, Some(0), Some(999))?;
                            l_input = input;
                            ctx.next()?;
                            ctx.lines(args![
                                ((Val::from("You lie on the carpet ") + l_input.clone()) + Val::from(" times."))
                            ])?;
                            ctx.next()?;
                            ctx.mes("But nothing happens.")?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        3 => {
                            ctx.mes("How many times should I shake the carpet?")?;
                            let (input, status) = runtime::input_number(ctx, Some(0), Some(999))?;
                            l_input = input;
                            ctx.next()?;
                            if l_input.clone() == 7 {
                                ctx.mes("You shake the carpet 7 times.")?;
                                ctx.next()?;
                                ctx.mes("- fly~ fly~ fly~ fly~ fly~ fly~ fly~ -")?;
                                ctx.next()?;
                                ctx.lines_as("Distant Sound", args!["^FF0000kkkkhee- hihihihi!!!^000000"])?;
                                ctx.next()?;
                                ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_VENOMDUST")?])?;
                                ctx.mes("Faint laughing can be heard off in the direction of the mirror.")?;
                                ctx.var("brazil_ghost").set(Val::from(6))?;
                                ctx.call(Function::ChangeQuest, vec![Val::from(60353), Val::from(60354)])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                ctx.lines(args![
                                    ((Val::from("You shake the carpet ") + l_input.clone()) + Val::from(" times."))
                                ])?;
                                ctx.next()?;
                                ctx.mes("But nothing happens.")?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                        _ => {}
                    }
                } else {
                    ctx.mes("Nothing happens.")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                matched1 = true;
            }
            if matched1 {
                ctx.mes("You do nothing.")?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    } else {
        ctx.mes("- A carpet with an intricate pattern on it -")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn carpet_bra(ctx: &Ctx) -> Script {
    carpet_bra_body(ctx, Vec::new()).map(|_| ())
}

fn mirror_bra_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_braspell_s = Val::from("");
    let mut l_chkspell = Val::from(0);
    let mut l_cpudice = Val::from(0);
    let mut l_input_s = Val::from("");
    let mut l_pcdice = Val::from(0);
    if ctx.var("brazil_ghost").get()?.number()? > 0 {
        ctx.mes("- You can see a clean mirror without any marks or dust -")?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Examine it.:Ignore.")])? {
            1 => {
                ctx.mes("- What was the next line to that spell now? -")?;
                let (input, status) = runtime::input_text(ctx, None, None)?;
                l_input_s = input;
                ctx.next()?;
                ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args![l_input_s.clone()])?;
                ctx.next()?;
                l_braspell_s = Val::from("Where are you mom?");
                l_chkspell = runtime::compare(&l_braspell_s.clone(), &l_input_s.clone());
                if !(l_chkspell.clone().is_true()) {
                    ctx.mes("Seems like you said something wrong.")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("brazil_ghost").get()? == 6 {
                    ctx.lines_as("Distant Sound", args!["^FF0000kihe! hit! hit! hit! hit!^000000"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Distant Sound",
                        args!["^FF0000kihe! hit! hit! hit! hit!^000000", "^FF0000kihe! hit! hit! hit! hit!^000000"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Distant Sound", args!["Behind you..."])?;
                    ctx.call(Function::EnableNpc, vec![Val::from("Ghost#bra")])?;
                    ctx.next()?;
                    ctx.call(
                        Function::Emotion,
                        vec![
                            ctx.constant("ET_HUK")?,
                            Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                        ],
                    )?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["The stories about the ghost are true~!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Ghost", args!["^FF0000my baby....^000000"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ghost",
                        args!["^FF0000I can't see.... my eye....^000000", "^FF0000What's going on....?^000000"],
                    )?;
                    ctx.next()?;
                    ctx.mes("- You take a deep breath and then look at the Ghost and notice it has an eye patch -")?;
                    ctx.next()?;
                    ctx.lines_as("Ghost", args!["^FF0000My eyes are so tight... can you take this off?^000000"])?;
                    ctx.next()?;
                    ctx.mes("You step carefully towards the ghost.")?;
                    ctx.next()?;
                    ctx.mes("His face was covered with dust making strange contortions with it's face.")?;
                    ctx.next()?;
                    ctx.lines_as("Ghost", args!["^FF0000Come on help mom.....^000000"])?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Take the eye bandage off.:Run away~.")])? {
                        1 => {
                            'l3: loop {
                                if !(true) {
                                    break 'l3;
                                }
                                'b3: {
                                    l_cpudice = ctx.call(Function::Rand, vec![Val::from(1), Val::from(6)])?;
                                    l_pcdice = ctx.call(Function::Rand, vec![Val::from(1), Val::from(6)])?;
                                    if !l_cpudice.clone().loosely_equals(&l_pcdice.clone()) {
                                        ctx.call(
                                            Function::Emotion,
                                            vec![
                                                (ctx.constant("ET_OTL")? + l_cpudice.clone()),
                                                ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Ghost#bra")])?,
                                            ],
                                        )?;
                                        ctx.call(
                                            Function::Emotion,
                                            vec![
                                                (ctx.constant("ET_OTL")? + l_pcdice.clone()),
                                                Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                                            ],
                                        )?;
                                        break 'l3;
                                    }
                                }
                            }
                            if runtime::op(&l_cpudice.clone(), ">", &l_pcdice.clone())?.is_true() {
                                ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_DEVIL")?])?;
                                ctx.lines_as("Ghost", args!["^FF0000Go away!^000000"])?;
                                ctx.var("brazil_ghost").set(Val::from(1))?;
                                ctx.call(Function::ChangeQuest, vec![Val::from(60354), Val::from(2208)])?;
                                ctx.call(Function::PercentHeal, vec![Val::from(-50), Val::from(-50)])?;
                                ctx.close_window()?;
                                ctx.call(Function::DisableNpc, vec![Val::from("Ghost#bra")])?;
                                ctx.call(Function::Warp, vec![Val::from("bra_in01"), Val::from(12), Val::from(183)])?;
                                return Err(Stop::End);
                            } else {
                                ctx.call(
                                    Function::Emotion,
                                    vec![
                                        ctx.constant("ET_STARE")?,
                                        Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                                    ],
                                )?;
                                ctx.lines_as("Ghost", args!["^FF0000Ahh!^000000", "The Ghost disappeared into the toilet."])?;
                                ctx.var("brazil_ghost").set(Val::from(7))?;
                                ctx.call(Function::ChangeQuest, vec![Val::from(60354), Val::from(60355)])?;
                                ctx.close_window()?;
                                ctx.call(Function::DisableNpc, vec![Val::from("Ghost#bra")])?;
                                ctx.call(Function::Warp, vec![Val::from("bra_in01"), Val::from(206), Val::from(100)])?;
                                return Err(Stop::End);
                            }
                        }
                        2 => {
                            ctx.mes("You run away from the ghost.")?;
                            ctx.close_window()?;
                            ctx.var("brazil_ghost").set(Val::from(1))?;
                            ctx.call(Function::ChangeQuest, vec![Val::from(60354), Val::from(2208)])?;
                            ctx.call(Function::Warp, vec![Val::from("bra_in01"), Val::from(12), Val::from(183)])?;
                            ctx.call(Function::DisableNpc, vec![Val::from("Ghost#bra")])?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.mes("Nothing happens.")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            2 => {
                ctx.mes("You do nothing.")?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else {
        ctx.mes("- You can see a clean mirror without any marks or dust -")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn mirror_bra(ctx: &Ctx) -> Script {
    mirror_bra_body(ctx, Vec::new()).map(|_| ())
}

fn ghost_bra_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn ghost_bra(ctx: &Ctx) -> Script {
    ghost_bra_body(ctx, Vec::new()).map(|_| ())
}

fn ghost_bra_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Ghost#bra")])?;
    return Err(Stop::End);
}

pub fn ghost_bra_oninit(ctx: &Ctx) -> Script {
    ghost_bra_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn curator_bra_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("BaseLevel").get()?.number()? < 40 {
        ctx.lines_as("Curator", args!["I'm sorry but this area is under construction right now."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("brazil_ghost").get()?.number()? > 0 && ctx.var("brazil_ghost").get()?.number()? < 7) {
        if ctx.call(Function::CountItem, vec![Val::from(11515)])?.number()? > 0 {
            ctx.lines_as("Curator", args!["What can I do for you?"])?;
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("I need to use the bathroom.")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as(
                "Curator",
                args![
                    "Sorry we are remodeling inside right now so, it's closed.",
                    "Please use the other one."
                ],
            )?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("But I forgot something inside.:Give up.")])? {
                1 => {
                    ctx.lines_as(
                        "Curator",
                        args![
                            "That's tooooo bad.",
                            "But my manager ordered me to stop anyone from entering this bathroom so, I should follow his orders."
                        ],
                    )?;
                    ctx.next()?;
                    let choice = runtime::select_values(ctx, &[Val::from("It's such a hot day!")])?;
                    ctx.var("@menu").set(choice)?;
                    ctx.lines_as(
                        "Curator",
                        args![
                            "It's always hot in Brasilis but today is ridiculously hot.",
                            "Maybe I need to drink some coconut juice to cool down."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.mes("You give a coconut to the Curator.")?;
                    ctx.next()?;
                    ctx.lines_as("Curator", args!["Oh really can I have it?", "Thanks a lot!"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Curator",
                        args!["Pay it forward right?", "Ok I'll let you through this one time."],
                    )?;
                    ctx.next()?;
                    ctx.mes("The curator looks around calmly then opens the door.")?;
                    ctx.call(Function::DelItem, vec![Val::from(11515), Val::from(1)])?;
                    ctx.var("brazil_ghost").set(Val::from(2))?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("bra_in01"), Val::from(138), Val::from(176)])?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.mes("You give up trying to enter.")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        } else {
            ctx.lines_as("Curator", args!["What can I do for you?"])?;
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("I need to use the bathroom.")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as(
                "Curator",
                args![
                    "Sorry we are remodeling inside right now so, it's closed.",
                    "Please use the other one."
                ],
            )?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("But I forgot something inside.:Give up.")])? {
                1 => {
                    ctx.lines_as(
                        "Curator",
                        args![
                            "That's tooooo bad.",
                            "But my manager ordered me to stop anyone from entering this bathroom so, I should follow his orders."
                        ],
                    )?;
                    ctx.next()?;
                    let choice = runtime::select_values(ctx, &[Val::from("It's such a hot day!")])?;
                    ctx.var("@menu").set(choice)?;
                    ctx.lines_as(
                        "Curator",
                        args![
                            "It's always hot in Brasilis but today is ridiculously hot.",
                            "Maybe I need to drink some coconut juice to cool down."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.mes("You give up trying to enter.")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        }
    } else {
        if ctx.var("brazil_ghost").get()?.number()? > 6 {
            ctx.lines_as(
                "Curator",
                args!["Hey thanks for the Coconut earlier it really helped me cool down."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as("Curator", args!["Is it just me? Or is it hotter than it's ever been today!"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    Ok(Val::from(0))
}

pub fn curator_bra(ctx: &Ctx) -> Script {
    curator_bra_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum InbathroomBraStep {
    Start,
    OnTouch,
}

fn inbathroom_bra_run(ctx: &Ctx, mut step: InbathroomBraStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            InbathroomBraStep::Start => {
                step = InbathroomBraStep::OnTouch;
                continue 'machine;
            }
            InbathroomBraStep::OnTouch => {
                if ctx.var("brazil_ghost").get()?.number()? > 6 {
                    ctx.call(Function::Warp, vec![Val::from("bra_in01"), Val::from(138), Val::from(176)])?;
                } else {
                    ctx.mes("The entrance has been blocked.")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn inbathroom_bra(ctx: &Ctx) -> Script {
    inbathroom_bra_run(ctx, InbathroomBraStep::Start, Vec::new()).map(|_| ())
}

pub fn inbathroom_bra_ontouch(ctx: &Ctx) -> Script {
    inbathroom_bra_run(ctx, InbathroomBraStep::OnTouch, Vec::new()).map(|_| ())
}

fn open_manhole_todunbra_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("brazil_ghost").get()? == 7 {
        ctx.call(Function::EnableNpc, vec![Val::from("Ghost#bra_end")])?;
        ctx.lines_as(
            "Ghost",
            args!["I am a ghost who died while wandering the jungle many years ago."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ghost",
            args!["I found a pipeline in the jungle and followed the voice of a man to this very spot."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ghost",
            args!["That's also where I hurt one of my eyes while walking around in the dark."],
        )?;
        ctx.next()?;
        ctx.lines_as("Ghost", args!["I wandered these sewers for days until I found the end of this line connected to the toilet in the museum. I shouted forever begging for help but no one answered my calls."])?;
        ctx.next()?;
        ctx.lines_as("Ghost", args!["You finally answered my call but it's already way too late. Thank you for at least checking. No one else has bothered to this day."])?;
        ctx.next()?;
        ctx.lines_as(
            "Ghost",
            args![
                "There are many dangerous creatures at the end of this sewer.",
                "You seem brave though. I bet you wouldn't worry about the monsters there anyways."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Ghost", args!["I guess now I can finally rest in peace.", "Thank you friend."])?;
        ctx.var("brazil_ghost").set(Val::from(8))?;
        ctx.call(Function::CompleteQuest, vec![Val::from(60355)])?;
        if (ctx.constant("VIP_SCRIPT")?.is_true() && ctx.call(Function::VipStatus, vec![ctx.constant("VIP_STATUS_ACTIVE")?])?.is_true()) {
            ctx.call(Function::GetExperience, vec![Val::from(135000), Val::from(0)])?;
        } else {
            ctx.call(Function::GetExperience, vec![Val::from(90000), Val::from(0)])?;
        }
        ctx.call(Function::DisableNpc, vec![Val::from("Ghost#bra_end")])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.call(Function::Warp, vec![Val::from("bra_dun01"), Val::from(87), Val::from(47)])?;
    return Err(Stop::End);
}

pub fn open_manhole_todunbra(ctx: &Ctx) -> Script {
    open_manhole_todunbra_body(ctx, Vec::new()).map(|_| ())
}

fn pipe_bra_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::Warp, vec![Val::from("bra_in01"), Val::from(206), Val::from(185)])?;
    return Err(Stop::End);
}

pub fn pipe_bra(ctx: &Ctx) -> Script {
    pipe_bra_body(ctx, Vec::new()).map(|_| ())
}

fn pipe_brafild_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("brazil_ghost").get()?.number()? > 6 {
        ctx.mes("You see a rusty pipe. It seems to be linked to somewhere beneath the jungle.")?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Examine it:Ignore.")])? {
            1 => {
                ctx.mes("You swim through a gap in the pipe and are swept by a sudden rush of water.")?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("bra_in01"), Val::from(206), Val::from(182)])?;
                return Err(Stop::End);
            }
            2 => {
                ctx.mes("It might be dangerous, I better not act rashly.")?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else {
        ctx.mes("You see a rusty pipe. It seems to be linked to somewhere beneath the jungle.")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn pipe_brafild(ctx: &Ctx) -> Script {
    pipe_brafild_body(ctx, Vec::new()).map(|_| ())
}

fn ghost_bra_end_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn ghost_bra_end(ctx: &Ctx) -> Script {
    ghost_bra_end_body(ctx, Vec::new()).map(|_| ())
}

fn ghost_bra_end_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Ghost#bra_end")])?;
    return Err(Stop::End);
}

pub fn ghost_bra_end_oninit(ctx: &Ctx) -> Script {
    ghost_bra_end_oninit_body(ctx, Vec::new()).map(|_| ())
}

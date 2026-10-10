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

pub fn s_2013questreset2_xmas(ctx: &Ctx) -> Script {
    let mut l_i = Val::from(0);
    let mut l_quests: Vec<Val> = Vec::new();
    if shared::other_gm_npcs::f_gm_npc(ctx, vec![Val::from(1854), Val::from(0)])? == 1 {
        let base = Val::from(0).number()?;
        runtime::local_set(&mut l_quests, &Val::from(base + 0), Val::from(15055), false);
        runtime::local_set(&mut l_quests, &Val::from(base + 1), Val::from(15056), false);
        runtime::local_set(&mut l_quests, &Val::from(base + 2), Val::from(15057), false);
        runtime::local_set(&mut l_quests, &Val::from(base + 3), Val::from(15059), false);
        runtime::local_set(&mut l_quests, &Val::from(base + 4), Val::from(15060), false);
        runtime::local_set(&mut l_quests, &Val::from(base + 5), Val::from(15061), false);
        runtime::local_set(&mut l_quests, &Val::from(base + 6), Val::from(15062), false);
        runtime::local_set(&mut l_quests, &Val::from(base + 7), Val::from(15063), false);
        runtime::local_set(&mut l_quests, &Val::from(base + 8), Val::from(15064), false);
        l_i = Val::from(0);
        'l1: loop {
            if !(runtime::op(&l_i.clone(), "<=", &Val::from(l_quests.len() as i32))?.is_true()) {
                break 'l1;
            }
            'b1: {
                if ctx.call(Function::IsBeginQuest, args![l_i.clone()])?.is_true() {
                    ctx.call(Function::EraseQuest, args![l_i.clone()])?;
                }
            }
            l_i = (l_i.clone() + Val::from(1));
        }
    }
    return ctx.end();
}

pub fn union_commander_cliff(ctx: &Ctx) -> Script {
    if ctx.var("BaseLevel").get()?.number()? < 40 {
        ctx.lines_as(
            "Union Commander Cliff",
            args!["No words for noob!! Level 40 below cannot join Singles Union Army!"],
        )?;
        return ctx.close();
    }
    if (ctx.call(Function::CheckWeight, args![1201, 1])? == 0
        || (ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 1)
    {
        ctx.mes("^ff0000You are overweight. Check your inventory and speak to me later.^000000")?;
        return ctx.close();
    }
    let l_playtime = ctx.call(Function::CheckQuest, args![15059, constants::PLAYTIME])?;
    if (l_playtime == 0 || l_playtime == 1) {
        ctx.mes("- You can repeat this quest after 24 hours.")?;
        return ctx.close();
    } else if ctx.call(Function::CheckQuest, args![15059, constants::PLAYTIME])?.number()? > 1 {
        ctx.quests().erase(15059)?;
        ctx.var("xmas2013_01").set(Val::from(0))?;
    }
    let l_que_allmem = ((((ctx.call(Function::IsBeginQuest, args![15060])? + ctx.call(Function::IsBeginQuest, args![15061])?)
        + ctx.call(Function::IsBeginQuest, args![15062])?)
        + ctx.call(Function::IsBeginQuest, args![15063])?)
        + ctx.call(Function::IsBeginQuest, args![15064])?);
    if ctx.call(Function::IsBeginQuest, args![15057])? == 1 {
        if ((ctx.items().count(7909)? < 10 || ctx.items().count(7910)? < 10) || ctx.items().count(6682)? < 10) {
            ctx.lines_as(
                "Union Commander Cliff",
                args!["We need more materials to hold a Christmas party for the Singles Union Army!"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Union Commander Cliff",
                args!["Bring ^ff000010 Stolen Cookie, 10 Stolen Candy, and 10 Bag Of Selling Goods^000000 from those damn raccoons!"],
            )?;
            return ctx.close();
        }
        ctx.lines_as("Union Commander Cliff", args!["Did you kick the ass of the Raccoon Hooray team? Oh! You've brought all the items we need. We will be able to feel some Christmas mood with it!"])?;
        ctx.next()?;
        ctx.lines_as("Union Commander Cliff", args!["I will take ALL the items you brought. For that, I'll give you an exclusive reward only for Singles Union Army. You-must-open-it-ALONE!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Union Commander Cliff",
            args!["Seize the holiday!", "Glory to the Singles Union Army! hihihi HAHAHAHA!!"],
        )?;
        ctx.call(Function::DelItem, args![7909, ctx.call(Function::CountItem, args![7909])?])?;
        ctx.call(Function::DelItem, args![7910, ctx.call(Function::CountItem, args![7910])?])?;
        ctx.call(Function::DelItem, args![6682, ctx.call(Function::CountItem, args![6682])?])?;
        ctx.quests().erase(15057)?;
        ctx.quests().start(15059)?;
        ctx.items().give(22685, 1)?;
        ctx.call(Function::SpecialEffect, args![constants::EF_MAGICALATTHIT])?;
        ctx.call(Function::SpecialEffect, args![constants::EF_POTION2])?;
        ctx.call(Function::SpecialEffect, args![constants::EF_ANGEL2])?;
        return ctx.close();
    } else if (ctx.call(Function::IsBeginQuest, args![15056])? == 1 && ctx.var("xmas2013_01").get()?.number()? < 5) {
        ctx.lines_as(
            "Union Commander Cliff",
            args![
                ((Val::from("You have a total of ") + ctx.var("xmas2013_01").get()?) + Val::from(" couple breaking points.")),
                "Break up at least 5 couples and come back to me!"
            ],
        )?;
        return ctx.close();
    } else if (ctx.call(Function::IsBeginQuest, args![15056])? == 1 && ctx.var("xmas2013_01").get()?.number()? > 4) {
        ctx.lines_as(
            "Union Commander Cliff",
            args!["Yes! You did it!! More and more are joining the Singles Union Army!! hihihi HAHAHA-HAK! Victory is ours!!"],
        )?;
        ctx.quests().erase(15056)?;
        ctx.quests().start(15057)?;
        ctx.items().give(22686, 5)?;
        ctx.next()?;
        ctx.lines_as("Union Commander Cliff", args!["BUT!!! While we're busy breaking up couples, those damn Raccoon Hooray brats stole most of our items. Without it, we cannot get the feel of Christmas mood."])?;
        ctx.next()?;
        ctx.lines_as(
            "Union Commander Cliff",
            args!["It's time to punish them and reclaim the items for the Singles Union Army!!"],
        )?;
        ctx.next()?;
        ctx.lines_as("Union Commander Cliff", args!["^ff0000>10 Stolen Cookie, 10 Stolen Candy and 10 Bag Of Selling Goods^000000 should be enough for the party. Find the raccoons on field maps and dungeons!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Union Commander Cliff",
            args!["It doesn't matter if you brought more, I will take them ALL!!"],
        )?;
        return ctx.close();
    } else if ctx.call(Function::IsBeginQuest, args![15055])? == 0 {
        ctx.lines_as("Union Commander Cliff", args!["Hey you!! Yes! You right there! You!!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Union Commander Cliff",
            args!["Let me ask you bluntly.", "You must be single. I've got a feeling. Right?!??"],
        )?;
        ctx.next()?;
        if ctx.menu(&["- ...maybe...?", "WHAT! I'm in a relationship!!"])? == 1 {
            ctx.lines_as(
                "Union Commander Cliff",
                args![
                    "What? ...in a relationship?!!",
                    "...Not single, but a couple...",
                    "......",
                    "Damn you! Taste the wrath of the Singles Union Army!!!"
                ],
            )?;
            ctx.close_window()?;
            ctx.call(Function::SpecialEffect, args![constants::EF_MAGICALATTHIT])?;
            ctx.call(Function::SpecialEffect, args![constants::EF_POTION2])?;
            ctx.call(Function::SpecialEffect, args![constants::EF_CRASHEARTH])?;
            ctx.call(Function::SpecialEffect, args![constants::EF_COIN])?;
            return ctx.end();
        }
        ctx.lines_as(
            "Union Commander Cliff",
            args![
                "hihihi HAHAHA-HAK!",
                "I knew it! First glance I can tell!",
                "Don't underestimate me, I've been single all my life! hihihi HAHAHA-HAK!",
                "My name is Union Commander Cliff!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Union Commander Cliff", args!["Anyway, this December, the day is coming...."])?;
        ctx.next()?;
        ctx.lines_as("Union Commander Cliff", args!["Christmas songs everywhere, bright colored lights, pouring white snow, and couples seeing each other with smile on their faces...couples...COUPPPLEEES...!!!!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Union Commander Cliff",
            args!["In the beginning was a celebration of the birth of our Lord! Then, why...how...when did it became a day for couples??!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Union Commander Cliff",
            args![
                "I can't stand this....",
                "I will not forgive them for the happiness they bring only for themselves!!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Union Commander Cliff",
            args!["Christmas for couples?", "No way! This Christmas will be a day for singles!!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Union Commander Cliff",
            args!["However, I will need members to stand against them. Its name would be, Singles Union Army!!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Union Commander Cliff",
            args!["What do you think? Would you join my Singles Union Army and make Christmas day for singles?"],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Count me in!:I'll help.:I can't say no, can I?")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Union Commander Cliff",
            args!["Yeah! That's it! but we need more members to stand against couples."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Union Commander Cliff",
            args![
                "Here is your first mission!",
                "Go find someone in Lutie and make them join us. 5 singles should be enough, now gogogo~!",
                "hihihi HAHAHA-HAK!"
            ],
        )?;
        ctx.close_window()?;
        ctx.quests().start(15055)?;
        return ctx.end();
    } else if (ctx.call(Function::IsBeginQuest, args![15055])? == 1 && l_que_allmem.number()? < 10) {
        ctx.lines_as(
            "Union Commander Cliff",
            args!["No time to waste!", "Go find 5 singles in Lutie and make them join!"],
        )?;
        return ctx.close();
    } else if (ctx.call(Function::IsBeginQuest, args![15055])? == 1 && l_que_allmem.number()? > 9) {
        ctx.lines_as(
            "Union Commander Cliff",
            args!["Good job! I've met them all! They are worthy to join our cause hihihi HAHAHA-HAK!!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Union Commander Cliff",
            args!["While you were gathering more Singles Union Army members, I've made badges.."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Union Commander Cliff",
            args!["It might not be useful but this kind of small token can make our bonds solid and strong!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Union Commander Cliff",
            args!["Well, take this! Now you are the proud member of Singles Union Army!!"],
        )?;
        ctx.var("xmas2013_01").set(Val::from(0))?;
        ctx.quests().complete(15055)?;
        ctx.items().give(6821, 1)?;
        ctx.next()?;
        ctx.lines_as(
            "Union Commander Cliff",
            args!["Now it is time to show our power to couples as we've got enough members!!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Union Commander Cliff",
            args![
                "On to the second mission!",
                "Go to Lutie and make couples separate. Doesn't matter how you do it, just them break up!!",
                "hihihi HAHAHA-HAK!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Union Commander Cliff", args!["Well there is one more thing...", "When you disturb couple, if they broke up, you will gain 2 point, in case of just simple argument will grant you 1 point.", "But if they become more into each others, your score will be minus 1 point. So when you got 5 points and come here for report, it means mission accomplished."])?;
        ctx.next()?;
        ctx.lines_as(
            "Union Commander Cliff",
            args!["Surely you won't have minus score so don't worry."],
        )?;
        ctx.quests().start(15056)?;
        return ctx.close();
    } else if ctx.call(Function::IsBeginQuest, args![15055])? == 2 {
        ctx.lines_as(
            "Union Commander Cliff",
            args!["Now it is time to show our power to couples as we've got enough troops!!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Union Commander Cliff",
            args![
                "Let's go for second mission!",
                "Go to Lutie and make couples separated. Whatever they are just make them break up!! "
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Union Commander Cliff", args!["Well there is one more thing...", "When you disturb couple, if they broke up, you will gain 2 point, in case of just simple argument will grant you 1 point. ", "But if they become more into each others, your score will be minus 1 point. So when you got 5 points and come here for report, it means mission accomplished."])?;
        ctx.next()?;
        ctx.lines_as(
            "Union Commander Cliff",
            args!["Surely you won't have minus score so don't worry."],
        )?;
        ctx.quests().start(15056)?;
        return ctx.close();
    }
    return ctx.end();
}

pub fn lonely_kwami_xmas(ctx: &Ctx) -> Script {
    if ctx.call(Function::IsBeginQuest, args![15055])? == 0 {
        ctx.lines_as(
            "Kwami",
            args!["Haa.....breaking up right before Christmas....I'm alone...a single! ...AM I??!!"],
        )?;
        return ctx.close();
    } else if (ctx.call(Function::IsBeginQuest, args![15055])? == 1 && ctx.call(Function::IsBeginQuest, args![15060])? == 0) {
        ctx.lines_as(
            "Kwami",
            args!["Haa.....breaking up right before Christmas....I'm alone...a single! ...AM I??!!"],
        )?;
        ctx.next()?;
        ctx.lines_as("Kwami", args!["Who are you!?", "What's your business with me?"])?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("- Singles Union Army has come to you. -")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as("Kwami", args!["err? Singles Union Army? What is that?"])?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("- Explain about the club -")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as("Kwami", args!["...yeah. You're right!"])?;
        ctx.next()?;
        ctx.lines_as("Kwami", args!["As I'm single! I'd never let couples enjoy this Christmas!!"])?;
        ctx.next()?;
        ctx.lines_as("Kwami", args!["I will join to the Singles Union Army!!"])?;
        ctx.next()?;
        ctx.mes("- Kwami has become a member of Singles Union Army. -")?;
        ctx.quests().start(15060)?;
        ctx.quests().complete(15060)?;
        return ctx.close();
    } else {
        ctx.lines_as(
            "Singles Union Kwami",
            args!["First of all, I've got to have revenge on my ex...No?"],
        )?;
        return ctx.close();
    }
}

pub fn lonely_willer_xmas(ctx: &Ctx) -> Script {
    if ctx.call(Function::IsBeginQuest, args![15055])? == 0 {
        ctx.lines_as("Willer", args!["Hoooooo... it is so boring to play alone...."])?;
        return ctx.close();
    } else if (ctx.call(Function::IsBeginQuest, args![15055])? == 1 && ctx.call(Function::IsBeginQuest, args![15061])? == 0) {
        ctx.lines_as("Willer", args!["Hoooooo... it is so boring to play alone...."])?;
        ctx.next()?;
        ctx.lines_as(
            "Willer",
            args!["Isn't there anything exciting?", "...hey, would you play with me??"],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("- Singles Union Army has come to you. -")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as("Willer", args!["Singles Union what? Why come to me?"])?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("- Explain about the club -")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Willer",
            args!["Sooo...", "Break up couples and make Christmas party only for singles?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Willer",
            args!["That must be fun! I don't understand exactly but I will join, I'm in!!"],
        )?;
        ctx.next()?;
        ctx.mes("- Willer has become a member of Singles Union Army. -")?;
        ctx.quests().start(15061)?;
        ctx.quests().complete(15061)?;
        return ctx.close();
    } else {
        ctx.lines_as("Singles Union Willer", args!["Break up couples...this should be fun!!"])?;
        return ctx.close();
    }
}

pub fn lonely_rinka_xmas(ctx: &Ctx) -> Script {
    if ctx.call(Function::IsBeginQuest, args![15055])? == 0 {
        ctx.lines_as("Rinka", args!["Ewww! darn... what kind of friend would only boast about her boyfriend, disgusting! Does she really think that I can't have one?!?"])?;
        return ctx.close();
    } else if (ctx.call(Function::IsBeginQuest, args![15055])? == 1 && ctx.call(Function::IsBeginQuest, args![15062])? == 0) {
        ctx.lines_as("Rinka", args!["Ewww! Darn... what kind of friend would only boast about her boyfriend, disgusting! Does she really think that I can't have one?!?"])?;
        ctx.next()?;
        ctx.lines_as("Rinka", args!["- sobbing -"])?;
        ctx.next()?;
        ctx.lines_as(
            "Rinka",
            args!["I can't... in fact, there is no one... I have no one!!!!!!!!!!!!!!"],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("- Singles Union Army has come to you. -")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Rinka",
            args!["EEeepp! Wow!!", "You surprised me. What are you talking about, what is that?!"],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("- Explain about the club -")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Rinka",
            args![
                "Hoho...there's something like that?",
                "Breaking up couples...",
                "Okay! Let me join!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Rinka",
            args![
                "Is there room for another member?",
                "...soon another member will join!",
                "Wait for it~ my friend!! ..."
            ],
        )?;
        ctx.next()?;
        ctx.mes("- Rinka has become a member of Singles Union Army. -")?;
        ctx.quests().start(15062)?;
        ctx.quests().complete(15062)?;
        return ctx.close();
    } else {
        ctx.lines_as("Singles Union Rinka", args!["Wait for it~ my friend!! You will... join soon!!"])?;
        return ctx.close();
    }
}

pub fn lonely_jee_xmas(ctx: &Ctx) -> Script {
    if ctx.call(Function::IsBeginQuest, args![15055])? == 0 {
        ctx.lines_as("Jee", args!["How beautiful to be single..."])?;
        return ctx.close();
    } else if (ctx.call(Function::IsBeginQuest, args![15055])? == 1 && ctx.call(Function::IsBeginQuest, args![15063])? == 0) {
        ctx.lines_as("Jee", args!["How beautiful to be single..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Jee",
            args!["There is no one to bother you. No one to take care of. No more extra work from others. More time to spend it alone."],
        )?;
        ctx.next()?;
        ctx.lines_as("Jee", args!["Why are people giving up this advantage and want to be couples??"])?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("- Singles Union Army has come to you. -")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Jee",
            args!["Yeah, I've heard of it recently. I knew that you would come to me."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Jee",
            args!["No more words necessary. I will spread the advantage of being single!"],
        )?;
        ctx.next()?;
        ctx.mes("- Jee has become a member of Singles Union Army. -")?;
        ctx.quests().start(15063)?;
        ctx.quests().complete(15063)?;
        return ctx.close();
    } else {
        ctx.lines_as("Singles Union Jee", args!["I will spread the advantage of being single!"])?;
        return ctx.close();
    }
}

pub fn lonely_marty_xmas(ctx: &Ctx) -> Script {
    if ctx.call(Function::IsBeginQuest, args![15055])? == 0 {
        ctx.lines_as("Marty", args!["umm... Zzz ... nyam-nyam..."])?;
        return ctx.close();
    } else if (ctx.call(Function::IsBeginQuest, args![15055])? == 1 && ctx.call(Function::IsBeginQuest, args![15064])? == 0) {
        ctx.lines_as("Marty", args!["umm... Zzz ... nyamnyam..."])?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("- Singles Union Army has come to you. -")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as("Marty", args!["nyam.... nyamnyam... Grrr... Zzz"])?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("- Explain about the club -")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as("Marty", args!["Huum... nyaaa.... Zzz"])?;
        ctx.next()?;
        ctx.mes("- ...you may consider him to agree to join. -")?;
        ctx.next()?;
        ctx.mes("- Marty has become a member of Singles Union Army. -")?;
        ctx.quests().start(15064)?;
        ctx.quests().complete(15064)?;
        return ctx.close();
    } else {
        ctx.lines_as("Singles Union Marty", args!["nyam.... nyamnyam... Grrrr... Zzz"])?;
        return ctx.close();
    }
}

pub fn drop_machine_xmas(ctx: &Ctx) -> Script {
    if ctx.call(Function::IsBeginQuest, args![15056])? == 0 {
        ctx.lines_as("Drop Machine", args!["Dingding dong Ding!~"])?;
        ctx.call(Function::Emotion, args![constants::ET_THROB])?;
        ctx.next()?;
        ctx.lines_as("Slot Machine", args!["!~"])?;
        ctx.call(
            Function::Emotion,
            args![constants::ET_THROB, ctx.call(Function::GetNpcId, args![0, "Slot Machine#xmas"])?],
        )?;
        return ctx.close();
    } else if (ctx.call(Function::IsBeginQuest, args![15056])? == 1 && ctx.var("xmas2013_01").get()?.number()? < 5) {
        ctx.lines_as("Drop Machine", args!["Dingdingding ding!~"])?;
        ctx.call(Function::Emotion, args![constants::ET_THROB])?;
        ctx.next()?;
        ctx.lines_as("Slot Machine", args!["Charrrrr Sharrrrrrrr Ding!~"])?;
        ctx.call(
            Function::Emotion,
            args![constants::ET_THROB, ctx.call(Function::GetNpcId, args![0, "Slot Machine#xmas"])?],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "- Oh no, another couple!",
            "- Even though they're just machines...",
            "- I cannot bear to see this!"
        ])?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("- Run the machine to full power. -")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Drop Machine",
            args!["Ding ding ding dong ding ding ding dong Ding ding ding dong ding ding ding dong"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Slot Machine",
            args!["Krrrrrrrrrr Dingding dong Krrrrrr Dingding Krrrrrrrrrrr ding Krrrrrrrrr"],
        )?;
        ctx.next()?;
        let roll = ctx.rand_range(1, 10)?;
        if roll < 3 {
            ctx.lines(args![
                "- I...",
                "- I think I broke them both!",
                "- Successfully broke up a couple!",
                "- 2 Points for a great job!"
            ])?;
            ctx.var("xmas2013_01").set((ctx.var("xmas2013_01").get()? + Val::from(2)))?;
        } else if roll > 7 {
            ctx.lines(args![
                "- I ran the machine for a while",
                "- But the sound of the machine",
                "- seems to grow louder.",
                "- Failed to break them, minus 1 point!"
            ])?;
            if ctx.var("xmas2013_01").get()?.number()? > 0 {
                ctx.var("xmas2013_01").set((ctx.var("xmas2013_01").get()?.try_sub(Val::from(1))?))?;
            }
        } else {
            ctx.lines(args![
                "- After running the machine for a while",
                "- I think I finally broke one of them!",
                "- Here's 1 point for a good job!"
            ])?;
            ctx.var("xmas2013_01").set((ctx.var("xmas2013_01").get()? + Val::from(1)))?;
        }
        ctx.lines(args![
            ((Val::from("- Current couple breaking point is ") + ctx.var("xmas2013_01").get()?) + Val::from("."))
        ])?;
        ctx.set_npc_visible("Drop Machine#xmas", false)?;
        ctx.set_npc_visible("Slot Machine#xmas", false)?;
        ctx.call(Function::InitNpcTimer, args![])?;
        return ctx.close();
    } else if (ctx.call(Function::IsBeginQuest, args![15056])? == 1 && ctx.var("xmas2013_01").get()?.number()? > 4) {
        ctx.lines(args![
            "- Couple breaking point",
            "- is now more than 5.",
            "- I should go back to Union Commander Cliff."
        ])?;
        return ctx.close();
    }
    return ctx.end();
}

pub fn drop_machine_xmas_ontimer100000(ctx: &Ctx) -> Script {
    ctx.set_npc_visible("Drop Machine#xmas", true)?;
    ctx.set_npc_visible("Slot Machine#xmas", true)?;
    ctx.call(Function::StopNpcTimer, args![])?;
    return ctx.end();
}

pub fn slot_machine_xmas(ctx: &Ctx) -> Script {
    return ctx.end();
}

pub fn frightened_man_xmas(ctx: &Ctx) -> Script {
    if ctx.call(Function::IsBeginQuest, args![15056])? == 0 {
        ctx.lines_as("Frightened Man", args!["It's my fault, I'm sorry. Please don't be mad."])?;
        ctx.call(Function::Emotion, args![constants::ET_SORRY])?;
        ctx.next()?;
        ctx.lines_as(
            "Angry Woman",
            args!["You're sorry? Do you know what you did? Do you have any idea why I'm mad?"],
        )?;
        ctx.call(
            Function::Emotion,
            args![constants::ET_FRET, ctx.call(Function::GetNpcId, args![0, "Angry Woman#xmas"])?],
        )?;
        return ctx.close();
    } else if (ctx.call(Function::IsBeginQuest, args![15056])? == 1 && ctx.var("xmas2013_01").get()?.number()? < 5) {
        ctx.lines_as("Frightened Man", args!["It's my bad, I'm sorry. Please don't be mad"])?;
        ctx.call(Function::Emotion, args![constants::ET_SORRY])?;
        ctx.next()?;
        ctx.lines_as(
            "Angry Woman",
            args!["You're sorry? Do you know what you did? Do you have any idea why I'm mad?"],
        )?;
        ctx.call(
            Function::Emotion,
            args![constants::ET_FRET, ctx.call(Function::GetNpcId, args![0, "Angry Woman#xmas"])?],
        )?;
        ctx.next()?;
        ctx.lines_as("Frightened Man", args!["Well, I just feel like everything's my fault"])?;
        ctx.next()?;
        ctx.lines_as(
            "Angry Woman",
            args!["You're always like this!", "You never know what you should apologize for!!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Frightened Man",
            args!["Ah... What should I do?", "Hey, what do you think I should say to my girlfriend?"],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("- Tell her 'We should stop seeing each other'!")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Frightened Man",
            args!["Maybe we.. should stop seeing each other?", "Oh no, I just said it out loud!"],
        )?;
        ctx.next()?;
        let roll = ctx.rand_range(1, 10)?;
        if roll < 3 {
            ctx.lines_as("Angry Woman", args!["What? What did you just say? We should stop seeing each other? How could you say that to me?! You're the one who's done everything wrong! Okay, you know what? Let's just break up! I'm sick of all this!"])?;
            ctx.var("xmas2013_01").set((ctx.var("xmas2013_01").get()? + Val::from(2)))?;
            ctx.set_npc_visible("Frightened Man#xmas", false)?;
            ctx.set_npc_visible("Angry Woman#xmas", false)?;
            ctx.call(Function::InitNpcTimer, args![])?;
            ctx.next()?;
            ctx.lines_as("Frightened Man", args!["No, that's not what I... okay, that's it! I'm sick of this too, sick of you whining all the time! Let's just end this today!!"])?;
            ctx.next()?;
            ctx.lines(args!["- Successfully broke up a couple!!", "- 2 Points!"])?;
        } else if roll > 7 {
            ctx.lines_as(
                "Angry Woman",
                args!["What? ...What did you just say? We should stop seeing each other?"],
            )?;
            if ctx.var("xmas2013_01").get()?.number()? > 0 {
                ctx.var("xmas2013_01").set((ctx.var("xmas2013_01").get()?.try_sub(Val::from(1))?))?;
            }
            ctx.set_npc_visible("Frightened Man#xmas", false)?;
            ctx.set_npc_visible("Angry Woman#xmas", false)?;
            ctx.call(Function::InitNpcTimer, args![])?;
            ctx.next()?;
            ctx.lines_as(
                "Angry Woman",
                args!["I can't believe you can be such a hot, tough guy! It's like I'm falling in love all over again!"],
            )?;
            ctx.next()?;
            ctx.lines_as("Angry Woman", args!["Honey, can you say that again, more in a rough way?"])?;
            ctx.next()?;
            ctx.lines_as("Frightened Man", args!["Huh? sure... Let's... let's just end this!!!"])?;
            ctx.next()?;
            ctx.lines_as("Angry Woman", args!["Hahahaha! You're awesome! Awesome! Say it again! Say it!"])?;
            ctx.next()?;
            ctx.lines_as("Frightened Man", args!["Let's just break up! I'm leaving you!!"])?;
            ctx.next()?;
            ctx.lines_as("Angry Woman", args!["Hahahahahahaha! Yay! You're the best!"])?;
            ctx.next()?;
            ctx.lines(args!["- ...These people aren't normal.", "- minus 1 point for the failure!"])?;
        } else {
            ctx.var("xmas2013_01").set((ctx.var("xmas2013_01").get()? + Val::from(1)))?;
            ctx.set_npc_visible("Frightened Man#xmas", false)?;
            ctx.set_npc_visible("Angry Woman#xmas", false)?;
            ctx.call(Function::InitNpcTimer, args![])?;
            ctx.lines_as(
                "Angry Woman",
                args!["What? ...What did you just say? We should stop seeing each other?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Angry Woman",
                args!["...I think we're both too upset now. Let's just talk about it later..."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Frightened Man",
                args!["Hmm?... ah, yes, you're right... Goodbye, get home safely..."],
            )?;
            ctx.next()?;
            ctx.mes("- Here's 1 point for ruining their date!")?;
        }
        ctx.lines(args![
            ((Val::from("- Current couple breaking point is ") + ctx.var("xmas2013_01").get()?) + Val::from("."))
        ])?;
        return ctx.close();
    } else if (ctx.call(Function::IsBeginQuest, args![15056])? == 1 && ctx.var("xmas2013_01").get()?.number()? > 4) {
        ctx.lines(args![
            "- Couple breaking point",
            "- is now more than 5.",
            "- I should go back to Union Commander Cliff."
        ])?;
        return ctx.close();
    }
    return ctx.end();
}

pub fn frightened_man_xmas_ontimer100000(ctx: &Ctx) -> Script {
    ctx.set_npc_visible("Frightened Man#xmas", true)?;
    ctx.set_npc_visible("Angry Woman#xmas", true)?;
    ctx.call(Function::StopNpcTimer, args![])?;
    return ctx.end();
}

pub fn angry_woman_xmas(ctx: &Ctx) -> Script {
    return ctx.end();
}

pub fn singles_union_kwami(ctx: &Ctx) -> Script {
    if ctx.call(Function::IsBeginQuest, args![15056])? == 0 {
        ctx.lines_as(
            "Singles Union Kwami",
            args!["Rinka. It's a huge mystery how such a pretty girl like you ended up in this Singles Union Army."],
        )?;
        ctx.call(Function::Emotion, args![constants::ET_THROB])?;
        ctx.next()?;
        ctx.lines_as(
            "Singles Union Rinka",
            args!["You think I'm pretty? hoho... It's mystery for me too, that a handsome man like you joined the Singles Union Army."],
        )?;
        ctx.call(
            Function::Emotion,
            args![constants::ET_THROB, ctx.call(Function::GetNpcId, args![0, "Singles Union Rinka"])?],
        )?;
        return ctx.close();
    } else if (ctx.call(Function::IsBeginQuest, args![15056])? == 1 && ctx.var("xmas2013_01").get()?.number()? < 5) {
        ctx.lines_as(
            "Singles Union Kwami",
            args!["Rinka. It's a huge mystery how such a pretty girl like you ended up in this Singles Union Army."],
        )?;
        ctx.call(Function::Emotion, args![constants::ET_THROB])?;
        ctx.next()?;
        ctx.lines_as(
            "Singles Union Rinka",
            args!["You think I'm pretty? hoho... It's mystery for me too, that a handsome man like you joined the Singles Union Army."],
        )?;
        ctx.call(
            Function::Emotion,
            args![constants::ET_THROB, ctx.call(Function::GetNpcId, args![0, "Singles Union Rinka"])?],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Singles Union Kwami",
            args!["Well, isn't it a shame we should spend Christmas alone? What do you say, Rinka, you and me..."],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(
            ctx,
            &[Val::from("- To be a couple is to be a betrayer! You betray us, you die!!!")],
        )?;
        ctx.var("@menu").set(choice)?;
        ctx.var("xmas2013_01").set((ctx.var("xmas2013_01").get()? + Val::from(1)))?;
        ctx.set_npc_visible("Singles Union Kwami", false)?;
        ctx.set_npc_visible("Singles Union Rinka", false)?;
        ctx.call(Function::InitNpcTimer, args![])?;
        ctx.lines_as(
            "Singles Union Rinka",
            args![
                "Argh!!!!! Who is this?!",
                "Isn't this the one who dragged us to the Singles Union Army?!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Singles Union Kwami",
            args![
                "Were you.. eavesdropping?!",
                "I just wanted to say we should find more members for the Club! Hahahahaha!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Singles Union Kwami",
            args!["I'll see you later, Rinka. I am going out to hunt down more couples. Don't tell the boss!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Singles Union Rinka",
            args!["Hmm, I guess I should get going to. See you later Kwami, goodbye to you too~"],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "- Successfully stopped a couple from forming!",
            "- 1 Point for the good job!",
            ((Val::from("- Current couple breaking point is ") + ctx.var("xmas2013_01").get()?) + Val::from("."))
        ])?;
        return ctx.close();
    } else if (ctx.call(Function::IsBeginQuest, args![15056])? == 1 && ctx.var("xmas2013_01").get()?.number()? > 4) {
        ctx.lines(args![
            "- Couple breaking point",
            "- is now more than 5.",
            "- I should go back to Union Commander Cliff."
        ])?;
        return ctx.close();
    }
    return ctx.end();
}

pub fn singles_union_kwami_ontimer300000(ctx: &Ctx) -> Script {
    ctx.set_npc_visible("Singles Union Kwami", true)?;
    ctx.set_npc_visible("Singles Union Rinka", true)?;
    ctx.call(Function::StopNpcTimer, args![])?;
    return ctx.end();
}

pub fn singles_union_rinka(ctx: &Ctx) -> Script {
    return ctx.end();
}

pub fn poor_alchemist_xmas(ctx: &Ctx) -> Script {
    if ctx.call(Function::IsBeginQuest, args![15056])? == 0 {
        ctx.lines_as("Poor Alchemist", args!["Oh, hello."])?;
        ctx.call(Function::Emotion, args![constants::ET_PROFUSELY_SWEAT])?;
        ctx.next()?;
        ctx.lines_as("Florist", args!["Hello, Mr. Alchemist.", "Are you working today, too?"])?;
        ctx.call(
            Function::Emotion,
            args![constants::ET_DELIGHT, ctx.call(Function::GetNpcId, args![0, "Florist#xmas"])?],
        )?;
        return ctx.close();
    } else if (ctx.call(Function::IsBeginQuest, args![15056])? == 1 && ctx.var("xmas2013_01").get()?.number()? < 5) {
        ctx.lines_as("Poor Alchemist", args!["Oh, hello."])?;
        ctx.call(Function::Emotion, args![constants::ET_PROFUSELY_SWEAT])?;
        ctx.next()?;
        ctx.lines_as("Florist", args!["Hello, Mr. Alchemist.", "Are you working today, too?"])?;
        ctx.call(
            Function::Emotion,
            args![constants::ET_DELIGHT, ctx.call(Function::GetNpcId, args![0, "Florist#xmas"])?],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Poor Alchemist",
            args![
                "Actually, I have something to tell you.",
                "It's very important so please listen carefully."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Florist", args!["Oh, sure, please speak freely."])?;
        ctx.next()?;
        ctx.lines_as("Poor Alchemist", args!["Biologically speaking, limbic system in the cerebrum enables us to be sociable, to communicate with people, to entertain etc."])?;
        ctx.next()?;
        ctx.lines_as("Poor Alchemist", args!["So when you see a beautiful woman, the information such as her appearance, her scent, the tone of her voice etc. goes to the limbic area."])?;
        ctx.next()?;
        ctx.lines_as(
            "Poor Alchemist",
            args!["The limbic system then evaluates the information, to see if this lady is friendly, agressive, or charming, etc."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Poor Alchemist",
            args!["When the evaluation is done, the limbic system immediately begins to secrete the chemicals."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Poor Alchemist",
            args!["So if you're attracted to this lady, a hormone calls Dopamine is secreted and it makes you feel slightly happy."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Poor Alchemist",
            args!["When that stage ends, you get the Adrenaline pumping, then you become more direct and enthusiastic."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Poor Alchemist",
            args!["I'm in that stage now towards you! In other word, I am very...!!"],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("- Very hungry, right?!")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as("Florist", args!["Huh? Is that right?", "Are you hungry now, Mr. Alchemist?"])?;
        ctx.next()?;
        ctx.lines_as("Poor Alchemist", args!["Ah.. yes.. I think I am...", "...very hungry.."])?;
        ctx.next()?;
        let roll = ctx.rand_range(1, 10)?;
        if roll < 3 {
            ctx.lines_as(
                "Florist",
                args!["There's a nice restaurant near here. Why don't you go there and have a meal?"],
            )?;
            ctx.var("xmas2013_01").set((ctx.var("xmas2013_01").get()? + Val::from(2)))?;
            ctx.set_npc_visible("Poor Alchemist#xmas", false)?;
            ctx.set_npc_visible("Florist#xmas", false)?;
            ctx.call(Function::InitNpcTimer, args![])?;
            ctx.next()?;
            ctx.lines_as("Poor Alchemist", args!["...Sure, thank you. Have a good day..."])?;
            ctx.next()?;
            ctx.lines(args![
                "- Successfully stopped a couple from forming!",
                "- 2 points for the great job!"
            ])?;
        } else if roll > 7 {
            ctx.lines_as("Florist", args!["I see. I understand what you mean."])?;
            if ctx.var("xmas2013_01").get()?.number()? > 0 {
                ctx.var("xmas2013_01").set((ctx.var("xmas2013_01").get()?.try_sub(Val::from(1))?))?;
            }
            ctx.set_npc_visible("Poor Alchemist#xmas", false)?;
            ctx.set_npc_visible("Florist#xmas", false)?;
            ctx.call(Function::InitNpcTimer, args![])?;
            ctx.next()?;
            ctx.lines_as("Florist", args!["Can I cook some nice meal for you, then?"])?;
            ctx.next()?;
            ctx.lines_as("Florist", args!["...For the rest of my life, if you want..."])?;
            ctx.next()?;
            ctx.lines_as("Poor Alchemist", args!["Okay...", "......", "Wait, what?????"])?;
            ctx.next()?;
            ctx.lines_as("Poor Alchemist", args!["......", "...... Yes! I want!!!!!!!!"])?;
            ctx.next()?;
            ctx.lines(args!["- Failed to break up a couple", "- minus 1 point for the failure!"])?;
        } else {
            ctx.var("xmas2013_01").set((ctx.var("xmas2013_01").get()? + Val::from(1)))?;
            ctx.set_npc_visible("Poor Alchemist#xmas", false)?;
            ctx.set_npc_visible("Florist#xmas", false)?;
            ctx.call(Function::InitNpcTimer, args![])?;
            ctx.lines_as("Florist", args!["There's some food in the store, please come in."])?;
            ctx.next()?;
            ctx.lines_as("Poor Alchemist", args!["Ah... yes, thank you."])?;
            ctx.next()?;
            ctx.mes("- You did interrupt the couple in a way. Here's 1 point for you!")?;
        }
        ctx.lines(args![
            ((Val::from("- Current couple breaking point is ") + ctx.var("xmas2013_01").get()?) + Val::from("."))
        ])?;
        return ctx.close();
    } else if (ctx.call(Function::IsBeginQuest, args![15056])? == 1 && ctx.var("xmas2013_01").get()?.number()? > 4) {
        ctx.lines(args![
            "- Couple breaking point",
            "- is now more than 5.",
            "- I should go back to Union Commander Cliff."
        ])?;
        return ctx.close();
    }
    return ctx.end();
}

pub fn poor_alchemist_xmas_ontimer100000(ctx: &Ctx) -> Script {
    ctx.set_npc_visible("Poor Alchemist#xmas", true)?;
    ctx.set_npc_visible("Florist#xmas", true)?;
    ctx.call(Function::StopNpcTimer, args![])?;
    return ctx.end();
}

pub fn florist_xmas(ctx: &Ctx) -> Script {
    return ctx.end();
}

pub fn raffini_boy_xmas(ctx: &Ctx) -> Script {
    if ctx.call(Function::IsBeginQuest, args![15056])? == 0 {
        ctx.lines_as("Raffini Boy", args!["This is Lutie village..."])?;
        ctx.call(Function::Emotion, args![constants::ET_STARE_ABOUT])?;
        ctx.next()?;
        ctx.lines_as(
            "Raffini Girl",
            args!["So much beautiful white snow flakes.", "But it's too cold here..."],
        )?;
        ctx.call(
            Function::Emotion,
            args![constants::ET_CRY, ctx.call(Function::GetNpcId, args![0, "Raffini Girl#xmas"])?],
        )?;
        return ctx.close();
    } else if (ctx.call(Function::IsBeginQuest, args![15056])? == 1 && ctx.var("xmas2013_01").get()?.number()? < 5) {
        ctx.lines_as("Raffini Boy", args!["This is Lutie village..."])?;
        ctx.call(Function::Emotion, args![constants::ET_STARE_ABOUT])?;
        ctx.next()?;
        ctx.lines_as(
            "Raffini Girl",
            args!["So much beautiful white snow flakes.", "But it's too cold here..."],
        )?;
        ctx.call(
            Function::Emotion,
            args![constants::ET_CRY, ctx.call(Function::GetNpcId, args![0, "Raffini Girl#xmas"])?],
        )?;
        ctx.next()?;
        ctx.lines_as("Raffini Boy", args!["But it's you who wanted to see the snow this Christmas."])?;
        ctx.next()?;
        ctx.lines_as("Raffini Girl", args!["Oh but I didn't know how much cold it is...oops..."])?;
        ctx.next()?;
        ctx.lines_as("Raffini Boy", args!["Well, ok I see...Just come closer to me."])?;
        ctx.next()?;
        ctx.lines_as("Raffini Girl", args!["What? I cannot hear you. What were you saying?"])?;
        ctx.next()?;
        ctx.lines_as("Raffini Boy", args!["...Come closer to me. To my side."])?;
        ctx.next()?;
        ctx.lines_as("Raffini Girl", args!["Ohh ok..."])?;
        ctx.next()?;
        ctx.lines(args!["- Hmm...there is no space at all", "- for me to interrupt..."])?;
        ctx.next()?;
        ctx.set_npc_visible("Raffini Boy#xmas", false)?;
        ctx.set_npc_visible("Raffini Girl#xmas", false)?;
        ctx.call(Function::InitNpcTimer, args![])?;
        let roll = ctx.rand_range(1, 10)?;
        if roll < 6 {
            ctx.var("xmas2013_01").set((ctx.var("xmas2013_01").get()? + Val::from(1)))?;
            ctx.lines_as(
                "Raffini Girl",
                args!["....But it's still too cold! I don't want to be here any more."],
            )?;
            ctx.next()?;
            ctx.lines_as("Raffini Girl", args!["I want to go back to Eclage!"])?;
            ctx.next()?;
            ctx.lines_as(
                "Raffini Boy",
                args!["Oh? Hey...we finally just got here, but you will go back right now?"],
            )?;
            ctx.next()?;
            ctx.lines_as("Raffini Boy", args!["Hey! Let's go together!!"])?;
            ctx.next()?;
            ctx.lines(args![
                "- No sweat at all, but everything is going alright.",
                "- Get 1 score point!"
            ])?;
        } else {
            if ctx.var("xmas2013_01").get()?.number()? > 0 {
                ctx.var("xmas2013_01").set((ctx.var("xmas2013_01").get()?.try_sub(Val::from(1))?))?;
            }
            ctx.lines_as("Raffini Girl", args!["Oh! It's much warmer than before...hehe"])?;
            ctx.next()?;
            ctx.lines_as("Raffini Boy", args!["D..don't be too close to me!"])?;
            ctx.next()?;
            ctx.lines_as("Raffini Girl", args!["Why not? Who cares. Let's go to grab something yummy."])?;
            ctx.next()?;
            ctx.lines_as("Raffini Boy", args!["You're too close too me! I, I can feel your..um..hey!"])?;
            ctx.next()?;
            ctx.lines(args![
                "- Oops, I was attacked by that couple even without doing anything.",
                "- Failed, got -1 score point!"
            ])?;
        }
        ctx.lines(args![
            ((Val::from("- Current couple breaking point is ") + ctx.var("xmas2013_01").get()?) + Val::from("."))
        ])?;
        return ctx.close();
    } else if (ctx.call(Function::IsBeginQuest, args![15056])? == 1 && ctx.var("xmas2013_01").get()?.number()? > 4) {
        ctx.lines(args![
            "- Couple breaking point",
            "- is now more than 5.",
            "- I should go back to Union Commander Cliff."
        ])?;
        return ctx.close();
    }
    return ctx.end();
}

pub fn raffini_boy_xmas_ontimer100000(ctx: &Ctx) -> Script {
    ctx.set_npc_visible("Raffini Boy#xmas", true)?;
    ctx.set_npc_visible("Raffini Girl#xmas", true)?;
    ctx.call(Function::StopNpcTimer, args![])?;
    return ctx.end();
}

pub fn raffini_girl_xmas(ctx: &Ctx) -> Script {
    return ctx.end();
}

pub fn angeling_xmas(ctx: &Ctx) -> Script {
    if ctx.call(Function::IsBeginQuest, args![15056])? == 0 {
        ctx.lines_as("Angeling", args!["Kkuing~"])?;
        ctx.call(Function::Emotion, args![constants::ET_THROB])?;
        ctx.next()?;
        ctx.lines_as("Arc Angeling", args!["Kkuing Kkuing~"])?;
        ctx.call(
            Function::Emotion,
            args![constants::ET_THROB, ctx.call(Function::GetNpcId, args![0, "Arc Angeling#xmas"])?],
        )?;
        return ctx.close();
    } else if (ctx.call(Function::IsBeginQuest, args![15056])? == 1 && ctx.var("xmas2013_01").get()?.number()? < 5) {
        ctx.lines_as("Angeling", args!["Kkuing~"])?;
        ctx.call(Function::Emotion, args![constants::ET_THROB])?;
        ctx.next()?;
        ctx.lines_as("Arc Angeling", args!["Kkuing Kkuing~"])?;
        ctx.call(
            Function::Emotion,
            args![constants::ET_THROB, ctx.call(Function::GetNpcId, args![0, "Arc Angeling#xmas"])?],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "- I cannot bother them though.",
            "- Can't even talk to them,",
            "- oh well."
        ])?;
        ctx.next()?;
        ctx.lines(args!["- Just hunt them?", "- ......No way."])?;
        return ctx.close();
    } else if (ctx.call(Function::IsBeginQuest, args![15056])? == 1 && ctx.var("xmas2013_01").get()?.number()? > 4) {
        ctx.lines(args![
            "- Couple breaking point",
            "- is now more than 5.",
            "- I should go back to Union Commander Cliff."
        ])?;
        return ctx.close();
    }
    return ctx.end();
}

pub fn arc_angeling_xmas(ctx: &Ctx) -> Script {
    return ctx.end();
}

pub fn prenetan_xmas(ctx: &Ctx) -> Script {
    if ctx.call(Function::IsBeginQuest, args![15056])? == 0 {
        ctx.lines_as("Prenetan", args!["Umba~ Umba Umba!", "Umba Umumumum!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Prenetan",
            args!["Finally got you, huh? You are cheating on me until now and even made me chase after you? I'll kill you boy!"],
        )?;
        ctx.call(Function::Emotion, args![constants::ET_FRET])?;
        ctx.next()?;
        ctx.lines_as("Umpoucoriotan", args!["Ooohh... my wife is too violent. Ohh...it hurts!"])?;
        ctx.call(
            Function::Emotion,
            args![constants::ET_CRY, ctx.call(Function::GetNpcId, args![0, "Umpoucoriotan#xmas"])?],
        )?;
        return ctx.close();
    } else if (ctx.call(Function::IsBeginQuest, args![15056])? == 1 && ctx.var("xmas2013_01").get()?.number()? < 5) {
        ctx.lines_as("Prenetan", args!["Umba~ Umba umba!", "Umba umumumum!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Prenetan",
            args!["Finally got you, huh? You are cheating on me until now and even made me chase after you? I'll kill you boy!"],
        )?;
        ctx.call(Function::Emotion, args![constants::ET_FRET])?;
        ctx.next()?;
        ctx.lines_as("Umpoucoriotan", args!["Ooohh... my wife is too violent. Ohh...it hurts!"])?;
        ctx.call(
            Function::Emotion,
            args![constants::ET_CRY, ctx.call(Function::GetNpcId, args![0, "Umpoucoriotan#xmas"])?],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Umpoucoriotan",
            args![
                "Wenathan, berzthan, Chabimathan",
                "Oooh... The girls of Umbala are here... They're waiting for me now....Arg! Don't touch me!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Prenetan",
            args![
                "How stupid you are!",
                "Those girls never wanted to see you and that's just a lie that they would come here!!"
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("- No. They are all here!")])?;
        ctx.var("@menu").set(choice)?;
        ctx.set_npc_visible("Prenetan#xmas", false)?;
        ctx.set_npc_visible("Umpoucoriotan#xmas", false)?;
        ctx.call(Function::InitNpcTimer, args![])?;
        let roll = ctx.rand_range(1, 10)?;
        if roll < 6 {
            ctx.var("xmas2013_01").set((ctx.var("xmas2013_01").get()? + Val::from(1)))?;
            ctx.lines_as("Umpoucoriotan", args!["Umm...yeah I know..Is it true those girls are here?"])?;
            ctx.next()?;
            ctx.lines_as("Umpoucoriotan", args!["...Oh I got a stomachache... Did I have something wrong... Darling, just a second. I'll be right back soon to go to the restroom! Just wait for a while here!"])?;
            ctx.next()?;
            ctx.lines_as("Prenetan", args!["Hey you idiot! Stop right there! Stop! ...STOP!!!"])?;
            ctx.next()?;
            ctx.mes("- Well, things are quite easy. Got 1 score point!")?;
        } else {
            ctx.var("xmas2013_01").set((ctx.var("xmas2013_01").get()? + Val::from(1)))?;
            ctx.lines_as("Prenetan", args!["Who's there? You want to be killed, too?"])?;
            ctx.next()?;
            ctx.mes("- Oops")?;
            ctx.next()?;
            ctx.lines_as(
                "Prenetan",
                args!["You still don't understand anything. Come here, I'm taking you back to Umbala!"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Umpoucoriotan",
                args!["ARG! ....Arrrrrrrrg!! It hurts! Noooo!!! They're waiting for me!!!"],
            )?;
            ctx.next()?;
            ctx.mes("- Well, he just made it harder for himself. Got 1 score point!")?;
        }
        ctx.lines(args![
            ((Val::from("- Current couple breaking point is ") + ctx.var("xmas2013_01").get()?) + Val::from("."))
        ])?;
        return ctx.close();
    } else if (ctx.call(Function::IsBeginQuest, args![15056])? == 1 && ctx.var("xmas2013_01").get()?.number()? > 4) {
        ctx.lines(args![
            "- Couple breaking point",
            "- is now more than 5.",
            "- I should go back to Union Commander Cliff."
        ])?;
        return ctx.close();
    }
    return ctx.end();
}

pub fn prenetan_xmas_ontimer100000(ctx: &Ctx) -> Script {
    ctx.set_npc_visible("Prenetan#xmas", true)?;
    ctx.set_npc_visible("Umpoucoriotan#xmas", true)?;
    ctx.call(Function::StopNpcTimer, args![])?;
    return ctx.end();
}

pub fn umpoucoriotan_xmas(ctx: &Ctx) -> Script {
    return ctx.end();
}

pub fn dark_lord_xmas(ctx: &Ctx) -> Script {
    if ctx.call(Function::IsBeginQuest, args![15056])? == 0 {
        ctx.lines_as(
            "Dark Lord",
            args!["Whahahahaha the world will turn into darkness soon and everyone will kneel down before me!!"],
        )?;
        ctx.call(Function::Emotion, args![constants::ET_SURPRISE])?;
        ctx.next()?;
        ctx.lines_as("Succubus", args!["My master. All worlds will follow your will."])?;
        ctx.call(
            Function::Emotion,
            args![constants::ET_THROB, ctx.call(Function::GetNpcId, args![0, "Succubus#xmas"])?],
        )?;
        return ctx.close();
    } else if (ctx.call(Function::IsBeginQuest, args![15056])? == 1 && ctx.var("xmas2013_01").get()?.number()? < 5) {
        ctx.lines_as(
            "Dark Lord",
            args!["Whahahahaha the world will turn into darkness soon and everyone will kneel down before me!!"],
        )?;
        ctx.call(Function::Emotion, args![constants::ET_SURPRISE])?;
        ctx.next()?;
        ctx.lines_as("Succubus", args!["My master. All worlds will follow your will."])?;
        ctx.call(
            Function::Emotion,
            args![constants::ET_THROB, ctx.call(Function::GetNpcId, args![0, "Succubus#xmas"])?],
        )?;
        ctx.next()?;
        ctx.lines_as("Dark Lord", args!["You've been loyal to me even though I've been giving difficult orders. You deserve a reward. What do you want for now? I'll definitely make it for you. I can even give you more power!"])?;
        ctx.next()?;
        ctx.lines_as("Succubus", args!["......", "Could you really give me anything that I want...?"])?;
        ctx.next()?;
        ctx.lines_as("Succubus", args!["What I really need is...", "......", "Only you, my master."])?;
        ctx.next()?;
        ctx.lines_as("Dark Lord", args!["......", "?!!!!!"])?;
        ctx.next()?;
        ctx.lines_as("Succubus", args!["I've been in love with you...", "for a very long time..."])?;
        ctx.next()?;
        ctx.lines_as("Dark Lord", args!["......"])?;
        ctx.next()?;
        ctx.lines_as("Dark Lord", args!["It's not that I didn't know about that."])?;
        ctx.next()?;
        ctx.lines_as("Dark Lord", args!["I also..."])?;
        ctx.next()?;
        ctx.lines_as("Dark Lord", args!["...But we're demons. Demons choosing a path towards love?! Ridiculous, the dark world will never forgive us! We will be hiding for the rest of our lives!"])?;
        ctx.next()?;
        ctx.lines_as("Succubus", args!["It doesn't matter as long as I am with you, my master..."])?;
        ctx.next()?;
        let choice = runtime::select_values(
            ctx,
            &[Val::from(
                "- I'm sorry to bother you... but weren't you discussing about conquering the world?",
            )],
        )?;
        ctx.var("@menu").set(choice)?;
        ctx.set_npc_visible("Dark Lord#xmas", false)?;
        ctx.set_npc_visible("Succubus#xmas", false)?;
        ctx.call(Function::InitNpcTimer, args![])?;
        let roll = ctx.rand_range(1, 10)?;
        if roll < 5 {
            ctx.var("xmas2013_01").set((ctx.var("xmas2013_01").get()? + Val::from(2)))?;
            ctx.lines_as(
                "Dark Lord",
                args!["Ye..yes. That's right, succubus! We've already had such a high goal!"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Dark Lord",
                args!["I appreciated the thought but know your place! Just be my subordinate as you already are."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Succubus",
                args!["Oh yes...my master, please forgive me. I'll try my best to make your the world yours."],
            )?;
            ctx.next()?;
            ctx.lines_as("Dark Lord", args!["Yes! There is nothing better than turning this world into darkness. Everyone will kneel down before me. Hahahahahahahahah!!!"])?;
            ctx.next()?;
            ctx.lines(args![
                "- Even though the world will",
                "- be engulfed in darkness soon...",
                "- I got a big success to break up a couple!",
                "- 2 score points!"
            ])?;
        } else {
            if ctx.var("xmas2013_01").get()?.number()? > 0 {
                ctx.var("xmas2013_01").set((ctx.var("xmas2013_01").get()?.try_sub(Val::from(1))?))?;
            }
            ctx.lines_as(
                "Dark Lord",
                args![
                    "Conquering the world... It's useless compared to what we've feeling right now.",
                    "Dear Succubus, can you really overcome any kind of agony if you are with me?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Succubus", args!["...Master!"])?;
            ctx.next()?;
            ctx.lines_as(
                "Succubus",
                args!["Of course, I'm prepared and will keep going... I'll follow you forever even if I shall be in any kind of agony."],
            )?;
            ctx.next()?;
            ctx.lines_as("Succubus", args!["I'll be yours forever!"])?;
            ctx.next()?;
            ctx.lines_as(
                "Dark Lord",
                args!["I see...I can feel your heart now. Ok, let's go together... no matter how hard it will be!!"],
            )?;
            ctx.next()?;
            ctx.lines(args![
                "- I saved this world from darkess.",
                "- though a couple was formed!",
                "- very sad...",
                "- Failed, -1 score point!"
            ])?;
        }
        ctx.lines(args![
            ((Val::from("- Current couple breaking point is ") + ctx.var("xmas2013_01").get()?) + Val::from("."))
        ])?;
        return ctx.close();
    } else if (ctx.call(Function::IsBeginQuest, args![15056])? == 1 && ctx.var("xmas2013_01").get()?.number()? > 4) {
        ctx.lines(args![
            "- Couple breaking point",
            "- is now more than 5.",
            "- I should go back to Union Commander Cliff."
        ])?;
        return ctx.close();
    }
    return ctx.end();
}

pub fn dark_lord_xmas_ontimer100000(ctx: &Ctx) -> Script {
    ctx.set_npc_visible("Dark Lord#xmas", true)?;
    ctx.set_npc_visible("Succubus#xmas", true)?;
    ctx.call(Function::StopNpcTimer, args![])?;
    return ctx.end();
}

pub fn succubus_xmas(ctx: &Ctx) -> Script {
    return ctx.end();
}

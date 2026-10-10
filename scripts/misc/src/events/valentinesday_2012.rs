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

const LOVE_FLOWER: i32 = 7864;

/// The sweets Pinkamenia sells: (item id, Love Flowers each one costs).
const SWEETS: [(i32, i32); 11] = [
    (538, 1),
    (558, 2),
    (539, 5),
    (573, 10),
    (559, 10),
    (560, 10),
    (12062, 15),
    (596, 15),
    (597, 15),
    (12414, 20),
    (12319, 20),
];

pub fn pinkamenia(ctx: &Ctx) -> Script {
    let speaker = "^0000FF[Pinkamenia]^000000";
    if ctx.var("#v_que12").get()? == 3 {
        ctx.lines(args![
            speaker,
            Val::from("Hello ") + ctx.player().name()? + Val::from(", do"),
            "you want to buy some items?",
            "You'll have to give me Love",
            "Flowers for them, of course!"
        ])?;
        ctx.next()?;
        if ctx.menu(&["Yes, sure!", "No, never mind."])? == 1 {
            return ctx.close();
        }
        ctx.next()?;
        ctx.lines(args![
            speaker,
            "Select an item.",
            "The amount of Love Flowers you'll need is in brackets."
        ])?;
        let mut menu = Val::from("");
        for (item, cost) in SWEETS {
            menu = menu
                + Val::from("^00AA00[")
                + Val::from(cost)
                + Val::from("]^000000 ")
                + ctx.call(Function::GetItemName, args![item])?
                + Val::from(":");
        }
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[menu])? - 1;
        let (item, cost) = SWEETS[choice as usize];
        ctx.lines(args![speaker])?;
        if ctx.items().count(LOVE_FLOWER)? < cost {
            ctx.mes("You'll need more Love Flowers if you want that!")?;
            return ctx.close();
        }
        ctx.lines(args![
            Val::from("Are you sure you want to trade ^00aa00")
                + Val::from(cost)
                + Val::from("x Love Flower ^000000 for ^0055FF")
                + ctx.call(Function::GetItemName, args![item])?
                + Val::from("^000000?")
        ])?;
        if ctx.menu(&["No, I've changed my mind.", "Yes, trade!"])? == 0 {
            return ctx.close();
        }
        ctx.call(Function::DelItem, args![LOVE_FLOWER, cost])?;
        ctx.call(Function::GetItem, args![item, 1])?;
        ctx.mes("Have fun with your item!")?;
        ctx.close()
    } else if ctx.var("#v_que12").get()? == 2 {
        ctx.lines(args![speaker])?;
        if ctx.items().count(LOVE_FLOWER)? < 15 {
            ctx.mes("You have to bring me 15 Love Flowers!")?;
            return ctx.close();
        }
        ctx.lines(args!["Thank you so much for", "getting our Flowers back!"])?;
        ctx.items().take(LOVE_FLOWER, 15)?;
        ctx.next()?;
        ctx.lines(args![speaker, "Take this as a little", "'thank you'."])?;
        ctx.call(Function::GetExperience, args![500000, 400000])?;
        ctx.items().give(617, 1)?;
        ctx.items().give(12319, 2)?;
        ctx.next()?;
        ctx.lines(args![
            speaker,
            "If you get more Love",
            "Flowers, you can exchange",
            "them for some sweets here.",
            "See you soon!"
        ])?;
        ctx.var("#v_que12").set(3)?;
        ctx.close()
    } else if ctx.var("#v_que12").get()? == 1 {
        ctx.lines(args![
            speaker,
            "Please bring a +8 Cake Hat",
            "to the Baker Extraordinaire",
            "standing next to me!"
        ])?;
        ctx.close()
    } else if ctx.player().base_level()? >= 45 {
        ctx.lines(args![
            speaker,
            Val::from("Hello ") + ctx.player().name()? + Val::from(", it's"),
            "Valentine's Day and we",
            "love to deliver sweet",
            "chocolates."
        ])?;
        ctx.next()?;
        ctx.lines(args![
            speaker,
            "Our problem is that we",
            "need a strong person",
            "like you who could help",
            "us, but first you'll have",
            "to bring a +8 Cake Hat",
            "to the Baker Extraordinaire, who",
            "is standing right next to me!"
        ])?;
        ctx.var("#v_que12").set(1)?;
        ctx.close()
    } else {
        ctx.lines(args![speaker, Val::from("Hello ") + ctx.player().name()? + Val::from("!")])?;
        ctx.close()
    }
}

pub fn pinkamenia_oninit(ctx: &Ctx) -> Script {
    ctx.call(
        Function::SetItemScript,
        args![
            5105,
            "{ bonus bDex,1; bonus bMaxSP,80; bonus3 bAddMonsterDropItem,7864,RC_DemiHuman,500; }",
            0
        ],
    )?;
    ctx.end()
}

pub fn baker_extraordinaire(ctx: &Ctx) -> Script {
    let speaker = "^0000FF[Baker Extraordinaire]^000000";
    if ctx.var("#v_que12").get()? == 3 {
        ctx.lines(args![speaker, "Exchange your Love Flowers with Pinkamenia!"])?;
        ctx.close()
    } else if ctx.var("#v_que12").get()? == 2 {
        ctx.lines(args![speaker, "Bring 15 Love Flowers to Pinkamenia!"])?;
        ctx.close()
    } else if ctx.var("#v_que12").get()? == 1 {
        ctx.lines(args![speaker])?;
        if ctx.call(Function::GetEquipId, args![ctx.constant("EQI_HEAD_TOP")?])? == 5024
            && ctx
                .call(Function::GetEquipRefineryCnt, args![ctx.constant("EQI_HEAD_TOP")?])?
                .number()?
                >= 8
        {
            ctx.lines(args![
                "Ah, so Pinkamenia told you",
                "to bring me the +8 Cake",
                "Hat. Now I'm going",
                "to exchange your +8 Cake",
                "Hat for another Cake Hat."
            ])?;
            ctx.next()?;
            ctx.lines(args![speaker])?;
            ctx.items().take(5024, 1)?;
            ctx.items().give(5105, 1)?;
            ctx.call(Function::Equip, args![5105])?;
            ctx.lines(args![
                "Now, if you wear the",
                "new Cake Hat, there",
                "is a chance that",
                "Demi-Human Monsters will",
                "drop a Love Flower!"
            ])?;
            ctx.next()?;
            ctx.lines(args![
                speaker,
                "The mobs stole our Flowers,",
                "which we need to create ",
                "our chocolates. Now get",
                "15 Love Flowers and bring",
                "them to Pinkamenia!"
            ])?;
            ctx.var("#v_que12").set(2)?;
            ctx.close()
        } else {
            ctx.lines(args![
                "Sorry, but where is your",
                "+8 Cake Hat? Bring",
                "it to me, and remember",
                "to have it equipped!"
            ])?;
            ctx.close()
        }
    } else {
        ctx.lines(args![speaker, Val::from("Hello, ") + ctx.player().name()? + Val::from("!")])?;
        ctx.close()
    }
}

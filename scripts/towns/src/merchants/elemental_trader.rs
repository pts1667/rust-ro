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

pub fn alchemist_ama(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Laspuchin Gregory",
        args!["KeekeekeeKeheheh.", "This is amazing!", "The results are extraordinary!!"],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Laspuchin Gregory",
        args![
            "Using my skills in this distant",
            "land was unexpected...",
            "Keheheh... The lord of palace",
            "was quite accomodating."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Laspuchin Gregory",
        args![
            "Oops, I better be careful...",
            "If the guild finds out my",
            "location, stupid Myster will",
            "get mad at me. Kehehehkeh..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Laspuchin Gregory",
        args![
            "What? Wanna say something?",
            "If you are here for tourism,",
            "enjoy your day off, then go back to your hometown.",
            "Keekeekee... Or else,",
            "I will let you taste my acid bottle...!"
        ],
    )?;
    ctx.next()?;
    if ctx.menu(&["Please, excuse me", "Do you need help?"])? == 0 {
        ctx.lines_as(
            "Laspuchin Gregory",
            args![
                "Keheheh... Did you hear",
                "what I said? It would be",
                "better to forget...",
                "Keekeekeekeekee..."
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as(
        "Laspuchin Gregory",
        args![
            "Help, eh?...",
            "Now that I think about it...",
            "I need some items right now...",
            "Keeheeheekeehee..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Laspuchin Gregory",
        args![
            "Sir Laspuchin needs some",
            "enchant stones for an experiment.",
            "I will use it efficiently...",
            "Do you have them now?"
        ],
    )?;
    ctx.next()?;
    if ctx.menu(&["Nope", "I got some"])? == 0 {
        ctx.lines_as(
            "Laspuchin Gregory",
            args![
                "Then, find 8 stones of one kind.",
                "If you bring 8 of one kind of",
                "enchant stones, I will change it",
                "to a better one..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Laspuchin Gregory",
            args![
                "Then, find 8 stones of one kind.",
                "not the round gemstones but",
                "enchant stones.",
                "If you bring the wrong stones,",
                "I will throw a flame bottle!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Laspuchin Gregory",
            args!["Bring me stones, slave~!!", "And keep your promise! Kehehehkehkeh..."],
        )?;
        return ctx.close();
    }
    ctx.lines_as("Laspuchin Gregory", args!["Uh-huh, what did you bring?"])?;
    ctx.next()?;
    let mut l_items: Vec<Val> = Vec::new();
    runtime::local_set(&mut l_items, &Val::from(1), Val::from(995), false);
    runtime::local_set(&mut l_items, &Val::from(2), Val::from(997), false);
    runtime::local_set(&mut l_items, &Val::from(3), Val::from(994), false);
    runtime::local_set(&mut l_items, &Val::from(4), Val::from(996), false);
    let l_i = runtime::local_get(
        &l_items,
        &Val::from(runtime::select_values(
            ctx,
            &[Val::from("Mystic Frozen:Great Nature:Flame Heart:Rough Wind")],
        )?),
        false,
    );
    ctx.mes("[Laspuchin Gregory]")?;
    if ctx.call(Function::CountItem, args![l_i.clone()])?.number()? > 7 {
        ctx.lines(args![
            "I will take 8 of them and",
            "give you an enchant stone.",
            "How's that sound?",
            "Keheheh....."
        ])?;
        ctx.next()?;
        ctx.lines_as("Laspuchin Gregory", args!["HeeHee, what do you want? Choose one!"])?;
        ctx.next()?;
        let mut l_menu_s = Val::from("");
        for j in 1..5 {
            let item = runtime::local_get(&l_items, &Val::from(j), false);
            if !item.loosely_equals(&l_i) {
                l_menu_s = l_menu_s + ctx.call(Function::GetItemName, args![item])?;
            }
            l_menu_s = l_menu_s + Val::from(":");
        }
        let choice = runtime::select_values(ctx, &[l_menu_s + Val::from("Cancel the trade")])?;
        ctx.mes("[Laspuchin Gregory]")?;
        if choice == 5 {
            ctx.lines(args![
                "Oh, well.",
                "Don't tell anyone about my location...",
                "Keheheh...after all, you did promise~"
            ])?;
            return ctx.close();
        }
        if choice == 4 {
            if ctx.call(Function::CountItem, args![l_i.clone()])?.number()? > 11 {
                ctx.call(Function::DelItem, args![l_i.clone(), 12])?;
                ctx.items().give(996, 1)?;
                ctx.lines(args![
                    "Kehhehheh, You know something?",
                    "I don't have many of those",
                    "so I'm going to take ^0000FF12^000000 of yours, okay?",
                    "Of course, it is okay.",
                    "It is an honor to help Laspuchin!"
                ])?;
                return ctx.close();
            }
            ctx.lines(args![
                "Keheheh, You know something.",
                "I don't have many of these so",
                "I can't just get 8 of yours.",
                "If you want to change them to Rough Wind,",
                "bring me 4 more stones. 8 + 4 = 12...",
                "Requires ^0000FF12^000000 stones."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Laspuchin Gregory",
                args!["Bring me stones, slave~!!", "And keep your promise! Kehheheheheh..."],
            )?;
            return ctx.close();
        }
        ctx.call(Function::DelItem, args![l_i.clone(), 8])?;
        ctx.call(
            Function::GetItem,
            args![runtime::local_get(&l_items, &Val::from(choice), false), 1],
        )?;
        ctx.lines(args![
            "Keheheh! You've chosen a good one!",
            "Use it well...",
            "Bring me other stones if you have them!"
        ])?;
        return ctx.close();
    }
    if ctx.call(Function::CountItem, args![l_i.clone()])?.is_true() {
        ctx.lines(args![
            "You can't help Laspuchin with just a couple",
            "of enchant stones... I said 8!",
            "Bring me just 8 stones!",
            "Keheheh....."
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Laspuchin Gregory",
            args!["Bring me stones, slave~!!", "And keep your promise! Keheheheheh..."],
        )?;
        return ctx.close();
    }
    ctx.lines(args![
        "Keheheh~ Check your pockets",
        "before you tell me",
        "how foolish you are....."
    ])?;
    ctx.next()?;
    ctx.lines_as(
        "Laspuchin Gregory",
        args![
            "Bring me stones! You said you are going to help me!",
            "Keep your promises! Kehehehkehkeh..."
        ],
    )?;
    ctx.close()
}

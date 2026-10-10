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

pub fn nemuk_ein(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Nemuk",
        args![
            "You seem to be an",
            "outsider, so let me",
            "ask you something.",
            "What do you think ",
            "of Einbech?"
        ],
    )?;
    ctx.next()?;
    match ctx.menu(&["It's fine.", "It looks tough to live here."])? {
        0 => {
            ctx.lines_as(
                "Nemuk",
                args![
                    "Huh...?",
                    "I'm not sure what",
                    "you've seen, but I'm",
                    "surprised to hear you",
                    "say something like that."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Nemuk",
                args![
                    "It's been ten years since",
                    "I've started to think about",
                    "moving out. However, I'm still",
                    "debating it. Now, if I were rich, I'd leave in no time, but it's hard getting the money to move out."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Nemuk",
                args![
                    "^333333*Sigh...*^000000",
                    "Maybe if I had been",
                    "an adventurer when I was",
                    "younger, I wouldn't have",
                    "these problems today..."
                ],
            )?;
            return ctx.close();
        }
        1 => {
            ctx.lines_as(
                "Nemuk",
                args!["I thought so.", "Well, I apologize if", "I put you on the spot."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Nemuk",
                args![
                    "Everyone here has been",
                    "having a tough time just",
                    "living day to day for as long",
                    "as I can remember. It's like",
                    "things never seem to get any",
                    "better, no matter what we do."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Nemuk",
                args![
                    "I really want to leave,",
                    "but it's just an empty",
                    "wish. My body is trapped",
                    "here while my heart longs",
                    "for a much better life. ^333333*Sigh*^000000",
                    "Is it hopeless? What can I do?"
                ],
            )?;
            return ctx.close();
        }
        _ => {}
    }
    Ok(())
}

pub fn young_man_air2(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Heinz",
        args!["Wow...", "An adventurer from", "Rune-Midgarts, eh?", "What brings you here?"],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Heinz",
        args![
            "Einbech doesn't offer much",
            "in terms of sight-seeing, but",
            "have you come to see the mine?",
            "Right now, it's swarming with",
            "monsters and we can't dig any",
            "ores because it's so dangerous."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Heinz", args!["Now, if some adventurers were", "generous enough to hunt down", "those evil creatures, we'd be able to mine again and they could earn some extra zeny. It's like killing two birds with one stone. Hahaha!"])?;
    ctx.next()?;
    ctx.lines_as(
        "Heinz",
        args![
            "Oh wait... I'm sorry.",
            "I don't know what's wrong",
            "with me, asking complete",
            "strangers to do favors for",
            "me. It's completely rude!",
            "I mean, who would do that?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Heinz",
        args![
            "But... I'm beyond caring",
            "about my pride. For the sake",
            "of all that is good and holy, I'm begging you, please kill those foul and evil creatures. Please~!"
        ],
    )?;
    ctx.close()
}

pub fn mogan_ein(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Mogan",
        args![
            "Recently, there were a few",
            "cave-ins where many miners",
            "were injured. It was discussed",
            "in the Town Council and in my",
            "opinion, I think the miners dug",
            "too deep and disturbed... ^FF0000it^000000."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Mogan",
        args![
            "Yes, they awoke Ungoliant,",
            "the master of the caves that",
            "has existed since ancient time.",
            "I don't know how many more will",
            "be victimized by Ungoliant in the",
            "future. There's no telling..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Mogan",
        args![
            "Adventurer, be careful",
            "if you travel inside the",
            "mines, lest your footsteps",
            "disturb Ungoliant's slumber."
        ],
    )?;
    ctx.close()
}

pub fn hander_ein(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Hander",
        args![
            "Those Einbroch bastards!",
            "Living off the resources we",
            "dig up while we keep working",
            "for them like suckers! Damn!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Hander",
        args![
            "Everyday, we risk our",
            "freakin' lives just so we",
            "can make a living! Why don't",
            "the elders do something about",
            "this, like raise our ore prices?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Hander",
        args![
            "The work schedule's",
            "unreasonable, Cavitar's",
            "wife was attacked by a mine",
            "creature, the hospital's too",
            "far away and we don't have",
            "any food to eat! Why...?!"
        ],
    )?;
    ctx.close()
}

pub fn gushenmu_ein(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Gushenmu",
        args![
            "I've lived here a long time",
            "and, believe it or not, things",
            "weren't as tough in the past",
            "as they are right now."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Gushenmu",
        args![
            "For lots of different reasons,",
            "the work is more dangerous",
            "and we're running real low on",
            "manpower. And the factories in",
            "Einbroch make so much smog,",
            "we can't even see sunlight here."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Gushenmu",
        args![
            "The sad reality of mining",
            "life right now is that we",
            "wake up, go to work, and at",
            "the end of the day, some of us",
            "are injured while a few others never come to work the next day."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Gushenmu",
        args![
            "And as Einbech and Einbroch",
            "have grown, I hear more and",
            "more rumors that unfamiliar",
            "monsters are beginning to",
            "swarm outside of town. This",
            "is really Einbech's worst time..."
        ],
    )?;
    ctx.close()
}

pub fn train_station_staff_ein3(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Staff",
        args![
            "Welcome to",
            "the Train Station.",
            "The fare to take the",
            "train to Einbroch is",
            "200 zeny. Would",
            "you like to ride?"
        ],
    )?;
    ctx.next()?;
    match ctx.menu(&["Yes.", "No."])? {
        0 => {
            if ctx.player().zeny()? > 199 {
                ctx.lines_as("Staff", args!["Thank you and", "we hope you enjoy", "the ride. All aboard!"])?;
                ctx.close_window()?;
                ctx.player().set_zeny(ctx.player().zeny()? - 200)?;
                ctx.warp("einbroch", 226, 276)?;
                return ctx.end();
            } else {
                ctx.lines_as(
                    "Staff",
                    args!["I'm sorry,", "but you don't", "have enough zeny", "to pay the train fare."],
                )?;
                return ctx.close();
            }
        }
        1 => {
            ctx.lines_as("Staff", args!["Please enjoy", "your stay here", "in Einbech."])?;
            return ctx.close();
        }
        _ => {}
    }
    Ok(())
}

pub fn train_station_manager_ei(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Train Station Manager",
        args![
            "This train station",
            "is strictly for trains",
            "running from Einbech",
            "to Einbroch. Please speak",
            "to the staff in the 11 'o clock direction if you'd like to board."
        ],
    )?;
    ctx.close()
}

pub fn tollaf_ein(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Tollaf",
        args![
            "Ah...!",
            "This is killing me!",
            "I don't have the money",
            "to move, but I don't wanna",
            "live in this town anymore!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Tollaf",
        args![
            "People everywhere else",
            "live so much better than we",
            "do, especially those snobs in",
            "Einbroch! Einbech must be the",
            "worst town Schwarzwald Republic. No, it's the worst in the world!"
        ],
    )?;
    ctx.close()
}

pub fn raust_ein(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Raust",
        args![
            "I don't get it!",
            "Einbroch gets bigger",
            "and fancier and our",
            "town gets dirtier and",
            "nastier. What the hell?!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Raust",
        args![
            "Not only do the people",
            "here look more ragged, we're",
            "more tired and older looking",
            "even! It's dirty, it's crowded,",
            "everything in this city is total crap! What, you want a list?!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Raust", args!["The food, literally, is", "garbage! The jobs here have", "to be violations of human rights. There's barely any women here and the ones we do have are all stank anyway! Are you convinced yet?!"])?;
    ctx.next()?;
    ctx.lines_as(
        "Raust",
        args![
            "Why is everything",
            "that's good over in",
            "Einbroch?! I hate this!",
            "^333333*Grumble*^000000"
        ],
    )?;
    ctx.close()
}

pub fn mjunia_ein(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Mjunia",
        args![
            "It's hard being a woman",
            "in this town. By being born",
            "here, it's like fate just decided to be especially cruel to me."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Mjunia",
        args![
            "My skin and hands are",
            "rough from all the work",
            "I have to do. But worst of",
            "all... I... I... I've developed",
            "bigger muscles than most",
            "guys! Waaaaaah~!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Mjunia",
        args![
            "I wish I could find",
            "a nice guy from Einbroch",
            "and get married so I can",
            "get away from this town.",
            "But it doesn't look like",
            "that will happen..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Mjunia",
        args![
            "And I'd never marry",
            "anyone from Einbech!",
            "I'd rather die cold and",
            "alone than cold and married",
            "to some Einbech hooligan."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Mjunia",
        args![
            "Look at these",
            "muscles. What do",
            "you think? Am I pretty?",
            "^333333*Sniff*^000000 I gave up trying",
            "to be feminine years ago.",
            "I have to work so hard..."
        ],
    )?;
    ctx.close()
}

pub fn ekuri_ein(ctx: &Ctx) -> Script {
    ctx.lines_as("Ekuri", args!["Yo-heave-ho!", "Yo-heave-ho~!"])?;
    ctx.next()?;
    ctx.lines_as(
        "Ekuri",
        args![
            "What am I doing here?",
            "Heck, I'm scared to death",
            "of entering the mine! But",
            "I can make a living here at",
            "the entrance by gathering",
            "scrap metal! Smart, huh?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Ekuri",
        args![
            "Sometimes, I get lucky",
            "and score an entire ore!",
            "Sure, I'm a coward, but",
            "at least I'm alive. Well,",
            "for the time being."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Ekuri",
        args![
            "Now you know what",
            "I'm doing here. So why",
            "don't you leave me to",
            "my work? Heave-ho!",
            "Ores, come to me!"
        ],
    )?;
    ctx.close()
}

pub fn bulletin_board_einbech11(ctx: &Ctx) -> Script {
    ctx.lines(args![" ", " Mine Dungeon Entrance ", " "])?;
    ctx.close()
}

pub fn bulletin_board_einbech22(ctx: &Ctx) -> Script {
    ctx.lines(args![" ", " Train Station ", " "])?;
    ctx.close()
}

pub fn bulletin_board_einbech33(ctx: &Ctx) -> Script {
    ctx.lines(args![" ", " Freight Train Station ", " "])?;
    ctx.close()
}

pub fn bulletin_board_einbech44(ctx: &Ctx) -> Script {
    ctx.lines(args![" ", " Tool Shop ", " "])?;
    ctx.close()
}

pub fn bulletin_board_einbech55(ctx: &Ctx) -> Script {
    ctx.lines(args![" ", " Tavern ", " "])?;
    ctx.close()
}

pub fn bulletin_board_einbech01(ctx: &Ctx) -> Script {
    ctx.mes("Welcome to 'Einbech'.")?;
    ctx.next()?;
    ctx.lines(args!["East - Tavern, Tool Shop", "North - Train Station, Mine Dungeon"])?;
    ctx.close()
}

pub fn bulletin_board_einbech03(ctx: &Ctx) -> Script {
    ctx.lines(args![
        "Northwest - Train Station",
        "South - Tavern",
        "North - Tool Shop, Mine Dungeon"
    ])?;
    ctx.close()
}

pub fn tavern_lady_ein(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Tavern Lady",
        args!["Most Einbech men are", "crude and primitive male", "chauvinists! They disgust me!"],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Tavern Lady",
        args![
            "I mean, there's nothing",
            "good about them! They're",
            "wild, violent, simple minded",
            "and ignorant. They settle all",
            "their arguments with brawn",
            "and they're so... close minded!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Tavern Lady",
        args![
            "How can they not know",
            "that women want gentle,",
            "sensitive men with whom",
            "they can share their feelings",
            "and drink chamomile tea over",
            "freshly knit doilies?"
        ],
    )?;
    ctx.close()
}

pub fn ryan_danger_air_einbech(ctx: &Ctx) -> Script {
    ctx.lines_as("R.D. Kim", args!["Oooh..."])?;
    ctx.next()?;
    ctx.lines_as("R.D. Kim", args!["Oooh...", "Momma."])?;
    ctx.next()?;
    ctx.lines_as("R.D. Kim", args!["Oooh...", "Momma.", "You are so..."])?;
    ctx.next()?;
    ctx.lines_as("R.D. Kim", args!["Oooh...", "Momma.", "You are so...", "^FF0000Hot^000000!"])?;
    ctx.next()?;
    ctx.lines_as(
        "R.D. Kim",
        args![
            "Why don't you take off",
            "those heavy, uncomfortable",
            "clothes? I'll buy you whatever",
            "you want, it's on me! C'mon~"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
        args![
            "N-no...!",
            "I-I-I-I...",
            "^666666(This is the",
            "shadiest guy",
            "I've ever seen!)^000000"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "R.D. Kim",
        args![
            "Hm? No...?",
            "Absolutely no?",
            "Are you sure?",
            "Alright, alright.",
            "I'm sorry, I apologize.",
            "I was totally out of line."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("R.D. Kim", args!["...", "Or am I?", "Bwahahahaha!"])?;
    ctx.next()?;
    ctx.lines_as(
        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
        args!["(Th-this guy", "must be drunk out", "of his freakin' mind!)"],
    )?;
    ctx.close()
}

pub fn drunken_man_einbech(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Drunken Man",
        args![
            "...^333333*Hiccup*^000000...",
            "^333333*Hiccup*^000000...",
            "^333333*Yawn*^000000.....",
            ".................",
            "..^333333*Hiccup*^000000.....",
            "^333333*Hiccup*^000000.."
        ],
    )?;
    ctx.close()
}

pub fn shena_ein(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Shena",
        args!["I think it's weird!", "How do you youngsters", "not learn all of this?"],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Luda",
        args![
            "Well, I'm sure the",
            "generation gap has",
            "something to do with it,",
            "but I'm surprised that elder",
            "people would know so much~"
        ],
    )?;
    ctx.next()?;
    'b1: {
        let subject1 = Val::from(runtime::select_values(
            ctx,
            &[Val::from("What are you guys talking about?:Pass on by")],
        )?);
        let mut matched1 = false;
        let no_case1 = !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2));
        if !matched1 && subject1.loosely_equals(&Val::from(1)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Shena",
                args!["Oh? Well, well!", "Aren't you the most", "adorable little girl?", "Hello, dearie~"],
            )?;
            ctx.next()?;
            if ctx.var("Sex").get()? == constants::SEX_MALE {
                let choice = runtime::select_values(ctx, &[Val::from("Excuse me, but I'm actually a guy.")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as(
                    "Shena",
                    args![
                        "Oh, oh are you now?",
                        "Perhaps my eyes are",
                        "getting bad in my old",
                        "age. Getting harder to",
                        "tell the difference nowadays..."
                    ],
                )?;
                ctx.next()?;
            }
            ctx.lines_as(
                "Shena",
                args![
                    "Anyway, me and Luda",
                    "were just having a little",
                    "chat about all the monsters",
                    "near Einbroch. Apparently,",
                    "you youngsters don't know as",
                    "much about them as you should."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Shena",
                args![
                    "If adventuring is your",
                    "business, you should",
                    "know what you're up against.",
                    "Did you have any questions",
                    "about the monsters around",
                    "here, young lady?"
                ],
            )?;
            ctx.next()?;
            if ctx.var("Sex").get()? == constants::SEX_MALE {
                let choice = runtime::select_values(ctx, &[Val::from("I told you, I'm a dude...!")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as(
                    "Shena",
                    args![
                        "Hmm. ''Dude.'' I think",
                        "I've heard that before.",
                        "Ho ho~ You'll have to ",
                        "forgive this old biddy. ",
                        "I don't quite have a grasp",
                        "on all the words you kids use."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Shena",
                    args!["So dearie,", "which monster", "would you like", "to hear about?"],
                )?;
                ctx.next()?;
            }
            'l2: loop {
                if !(true) {
                    break 'l2;
                }
                'b2: {
                    match ctx.menu(&["Metalling", "Mineral", "Pit Man", "Old Stove", "Quit"])? {
                        0 => {
                            ctx.lines_as(
                                "Shena",
                                args![
                                    "Well, the Metallings",
                                    "were created during",
                                    "the time when the gods",
                                    "ruled over this world."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Shena",
                                args![
                                    "I'm not sure if you knew",
                                    "this or not, but according",
                                    "to myth, Porings and Drops",
                                    "were created from Odin's",
                                    "saliva. You might not want",
                                    "to know about Poporing..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Shena",
                                args![
                                    "Metallings, on the",
                                    "other hand, were made",
                                    "from the blood of living",
                                    "machines that I believe",
                                    "were called ''Gigantes.''"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Shena",
                                args![
                                    "Metalling is still like",
                                    "those other hopping",
                                    "blobs of gelatin in that",
                                    "they'll swallow whatever",
                                    "might be lying on the ground."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Shena",
                                args![
                                    "If you defeat a Metalling,",
                                    "it could drop Large Jellopy,",
                                    "Iron Ore or even Iron. That",
                                    "might be good to know, right?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Shena", args!["Is there", "anything else", "you'd like to", "ask, dearie?"])?;
                            ctx.next()?;
                        }
                        1 => {
                            ctx.lines_as(
                                "Shena",
                                args![
                                    "Did you know that",
                                    "stalactites and cave",
                                    "crystals grow for thousands",
                                    "and thousands of years?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Shena",
                                args![
                                    "Now, if something's been",
                                    "growing for thousands of",
                                    "years, it would make sense",
                                    "if it were actually alive. Now,",
                                    "Mineral monsters are actually living stalactites."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Shena",
                                args![
                                    "It's rumored that they",
                                    "are grown in a dark cave",
                                    "in which something inside",
                                    "has some sort of malicious",
                                    "influence over them."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Shena", args!["Minerals can defend themselves,", "but they might drop Crystal Piece, Topaz or Emvertacon if you defeat one. There's also a slim chance that they may drop a rare jewel, but I'm not quite sure."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Shena",
                                args![
                                    "Are there any",
                                    "other monsters",
                                    "around here that",
                                    "you'd want to learn",
                                    "more about?"
                                ],
                            )?;
                            ctx.next()?;
                        }
                        2 => {
                            ctx.lines_as(
                                "Shena",
                                args![
                                    "Pit Men are the ghosts",
                                    "of dead miners that haunt",
                                    "old and rusted mine cars.",
                                    "For some reason, they can't",
                                    "leave this world so they just",
                                    "wander around the mines."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Shena",
                                args![
                                    "If you can defeat",
                                    "them, they'll drop",
                                    "Old Pick, Lantern, Iron,",
                                    "Steel, Coal, Flashlight",
                                    "and Old Iron Plate."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Shena",
                                args!["Did you want", "to ask me about", "any other of the", "local monsters?"],
                            )?;
                            ctx.next()?;
                        }
                        3 => {
                            ctx.lines_as("Shena", args!["It's just an evil,", "man-eating stove."])?;
                            ctx.next()?;
                            match ctx.menu(&["...That's it?", "H-horrifying!"])? {
                                0 => {
                                    ctx.lines_as(
                                        "Shena",
                                        args![
                                            "Now, you know the importance",
                                            "of recycling and preserving our",
                                            "natural resources, right? Now,",
                                            "it would do my heart good if you were to recycle the scrap iron",
                                            "from those Old Stove monsters."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Shena", args!["Old Stoves will usually", "drop Battered Pot, Burnt Tree,", "Iron, Iron Ore and Old Iron Plate. But once in a while they might drop interesting items like Rusty Iron or even Dead Branch."])?;
                                    ctx.next()?;
                                }
                                1 => {
                                    ctx.lines_as(
                                        "Shena",
                                        args![
                                            "Yes. God's creation,",
                                            "that creature is cruel",
                                            "and merciless, perhaps",
                                            "a symbol of purest evil",
                                            "if I ever saw one."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Shena",
                                        args![
                                            "Unlike newer appliances,",
                                            "Old Stoves were hand made by",
                                            "master craftsmen that, I guess,",
                                            "developed their own souls. They",
                                            "used to be benevolent machines, content to provide loving warmth."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Shena",
                                        args![
                                            "For years they would serve",
                                            "their owners with loyalty. But",
                                            "as technology advanced and",
                                            "they became obsolete, they were",
                                            "discarded like pieces of trash. This twisted their hearts to ^FF0000evil^000000."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Shena", args!["Old Stoves will usually", "drop Battered Pot, Burnt Tree,", "Iron, Iron Ore and Old Iron Plate. But once in a while they might drop interesting items like Rusty Iron or even Dead Branch."])?;
                                    ctx.next()?;
                                }
                                _ => {}
                            }
                            ctx.lines_as(
                                "Shena",
                                args!["So, is there", "anything else", "you'd like me to", "share with you?"],
                            )?;
                            ctx.next()?;
                        }
                        4 => {
                            ctx.lines_as("Shena", args!["Alright then.", "Have a good", "day, young lady."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Luda",
                                args![
                                    "I'm sorry about my",
                                    "mother! She can be",
                                    "overly friendly, I suppose.",
                                    "But if you're bored, please",
                                    "feel free to visit. Be safe",
                                    "on your travels, adventurer~"
                                ],
                            )?;
                            return ctx.close();
                        }
                        _ => {}
                    }
                }
            }
        }
        if !matched1 && subject1.loosely_equals(&Val::from(2)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Shena",
                args![
                    "Luda...",
                    "You don't live",
                    "to be as old as",
                    "I am and not learn",
                    "a little something",
                    "about this world of ours~"
                ],
            )?;
            return ctx.close();
        }
    }
    Ok(())
}

pub fn jung_ein(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Jung",
        args![
            "I'm one of the few",
            "people who's lived",
            "in both Einbech and",
            "Einbroch for a long time.",
            "So I guess I'm one of the",
            "best guides of this area."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Jung",
        args![
            "Say, if you're thinking of",
            "entering the Mine Dungeon,",
            "I can tell you all I know about",
            "the monsters in that place so",
            "that you'll be better prepared."
        ],
    )?;
    ctx.next()?;
    'b1: {
        let subject1 = Val::from(runtime::select_values(ctx, &[Val::from("Sure, why not?:No, thanks.")])?);
        let mut matched1 = false;
        let no_case1 = !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2));
        if !matched1 && subject1.loosely_equals(&Val::from(1)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Jung",
                args![
                    "Let's see. Ah, the monsters that are unique to the Mine Dungeon",
                    "are Noxious, Venomous, Pollcellio and Obsidian. Which one do you",
                    "want to know more about?"
                ],
            )?;
            ctx.next()?;
            match ctx.menu(&["Noxious and Venomous", "Pollcellio", "Obsidian"])? {
                0 => {
                    ctx.lines_as(
                        "Jung",
                        args![
                            "You know, no one seems",
                            "to know where Noxious and",
                            "Venomous have come from.",
                            "It's like they appeared out of",
                            "nowhere when Einbroch",
                            "started to industrialize."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Jung",
                        args![
                            "Now that I think about it,",
                            "I don't think they're naturally created monsters. They have",
                            "this fixed look of despair and",
                            "suffering and tend to act like they want their enemies to kill them."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Jung",
                        args![
                            "Still, you'd better be careful!",
                            "careful! Noxious and Venomous",
                            "are stealthy monsters that can",
                            "glide quietly through the air",
                            "and attack you before",
                            "you even notice..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Jung",
                        args![
                            "You should know that",
                            "Noxious is Ghost property",
                            "and Venomous is Poison.",
                            "Both are medium sized,",
                            "formless monsters."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Jung",
                        args![
                            "Both of them drop Apple,",
                            "Dust Pollutant, Toxic Gas,",
                            "Poisonous Powder, Bacillus,",
                            "Mold Powder and Anodyne."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Jung",
                        args![
                            "That's all for now.",
                            "Feel free to ask me",
                            "if you have any questions",
                            "about monsters in the Mine",
                            "Dungeon. Be safe, adventurer."
                        ],
                    )?;
                    return ctx.close();
                }
                1 => {
                    ctx.lines_as(
                        "Jung",
                        args![
                            "Pollcellio is an insect that",
                            "lives in caves and drinks water",
                            "dripped from stalactites. It's",
                            "different from Ungoliant since",
                            "it likes to be near different",
                            "kinds of minerals and ores."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Jung",
                        args![
                            "Pollcellio drops Jubilee,",
                            "Insect Antenna, Single Cell,",
                            "Moss of Morning Dew, Neon",
                            "Liquid and a few other things",
                            "I can't quite remember."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Jung",
                        args![
                            "Lastly, Pollcellio is an",
                            "Earth property monster.",
                            "That's all I know about it.",
                            "But if you want to know more",
                            "about some other monster in the",
                            "Mine Dungeon, feel free to ask."
                        ],
                    )?;
                    return ctx.close();
                }
                2 => {
                    ctx.lines_as(
                        "Jung",
                        args![
                            "Do you know about the",
                            "belief that underground",
                            "minerals that contain huge",
                            "amounts of energy actually",
                            "have souls? Obsidian is",
                            "one of these living rocks."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Jung", args!["Supposedly, just a piece of an Obsidian in a Jung Processor has enough energy to light up the night sky. Unfortunately, it's impossible to capture one alive and hunting them isn't so easy."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Jung",
                        args![
                            "Obsidian is a small,",
                            "shapeless monster that",
                            "drops Clear Jewel, Piece of",
                            "Black Crystal, Coal, Elunium,",
                            "Iron and Steel."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Jung",
                        args![
                            "That's all for Obsidian.",
                            "If you have any questions",
                            "about other monsters living",
                            "in the Mine Dungeon, feel",
                            "free to ask me."
                        ],
                    )?;
                    return ctx.close();
                }
                _ => {}
            }
        }
        if !matched1 && subject1.loosely_equals(&Val::from(2)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Jung",
                args![
                    "I understand if you're",
                    "kind of in a hurry. Still,",
                    "if you're pretty new around",
                    "here, you should learn as",
                    "much as you can before",
                    "entering any dungeons."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Jung", args!["Alright then,", "be safe on your", "adventures, alright?"])?;
            return ctx.close();
        }
    }
    Ok(())
}

pub fn franz_ein(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Franz",
        args!["So bored...", "Starving for...", "Conversation.", "S-somebody..."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Franz",
        args![
            "Hey, a traveler!",
            "Are you planning to explore",
            "the Mine Dungeon or the fields",
            "around here? Let's chat for a bit and maybe you'll learn something."
        ],
    )?;
    ctx.next()?;
    'b1: {
        let subject1 = Val::from(runtime::select_values(ctx, &[Val::from("Okay, fine.:No, thanks.")])?);
        let mut matched1 = false;
        let no_case1 = !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2));
        if !matched1 && subject1.loosely_equals(&Val::from(1)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Franz",
                args![
                    "Ooh, have you heard",
                    "about the creature in the",
                    "Mine Dungeon or what's",
                    "happened in town recently?",
                    "Which would you like to",
                    "know more about?"
                ],
            )?;
            ctx.next()?;
            match ctx.menu(&["Creature of Mine Dungeon", "Town Incident"])? {
                0 => {
                    ctx.lines_as(
                        "Franz",
                        args![
                            "The creature I'm talking about is Ungoliant, which also called the Master of the Caves around here.",
                            "It's said to live deep in the caves where it guards peculiar ores and minerals with strange powers."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Franz",
                        args![
                            "At first I thought it was",
                            "just an old fairy tale, but it",
                            "actually started appearing",
                            "again about ten years ago",
                            "when the tunnel cave-ins",
                            "started to happen."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Franz",
                        args![
                            "As sightings of Ungoliant",
                            "increased, more and more",
                            "tunnel cave-ins occurred.",
                            "I guess the miners have",
                            "inadvertently intruded",
                            "into its territory."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Franz",
                        args![
                            "According to legend,",
                            "ancient giants snuck into",
                            "a mine to steal coal from",
                            "humans. But they made too",
                            "much noise while they were",
                            "digging and awoke Ungoliant."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Franz",
                        args![
                            "When the miners went to work",
                            "the next morning, they found the bloodied bodies of those giants.",
                            "After that, people have feared",
                            "the threat that Ungoliant poses",
                            "to anyone entering the mines."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Franz",
                        args![
                            "Now, an adventurer that",
                            "managed to kill an Ungoliant",
                            "has told me that it drops Ant's",
                            "Jaw, Colorful Shell, Very Hard",
                            "Shell, Long Leg, Neon Liquid",
                            "and Zilcon."
                        ],
                    )?;
                    return ctx.close();
                }
                1 => {
                    ctx.lines_as(
                        "Franz",
                        args![
                            "In Einbroch, there was",
                            "a short lived teddy bear",
                            "fad. However, a series of",
                            "mysterious accidents and",
                            "murders where entire families",
                            "were killed also occurred."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Franz",
                        args![
                            "It turns out that every family",
                            "that had been murdered had",
                            "bought one of these teddy bears. There were even rumors that these teddy bears were coming to life."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Franz",
                        args![
                            "After an investigation, the",
                            "authorities learned that all the merchants who sold these bears",
                            "had purchased them from the",
                            "same wholesaler, an outsider",
                            "no one knew anything about."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Franz",
                        args![
                            "Since the teddy bears were",
                            "clearly not made to be mere,",
                            "harmless toys, troops were",
                            "sent to secure all the teddy",
                            "bears and dispose of them",
                            "outside of town."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Franz",
                        args![
                            "But as soon as the teddy",
                            "bears were set outside of",
                            "town, they sprang to life and",
                            "started rioting! This is clear",
                            "proof that these bears are",
                            "controlled by some evil force. "
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Franz",
                        args![
                            "Now those aggressive teddy",
                            "bears are scattered all over",
                            "the place and the government",
                            "has classified them as monsters. Kill with extreme prejudice!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Franz",
                        args![
                            "According to adventurers",
                            "who have caught these bears,",
                            "they're small, neutral monsters",
                            "which drop Honey, Screw, Well-baked Cookie and Oridecon Hammer."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Franz",
                        args![
                            "That's all I know",
                            "about it. Watch out",
                            "for those bears if you",
                            "go exploring, okay? They",
                            "may be cute, but they're",
                            "known to be extremely vicious!"
                        ],
                    )?;
                    return ctx.close();
                }
                _ => {}
            }
        }
        if !matched1 && subject1.loosely_equals(&Val::from(2)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Franz",
                args![
                    "Oh, okay.",
                    "You're busy and you have",
                    "things to do, I understand.",
                    "You probably have to head",
                    "off somewhere right away.",
                    "Right. Got it."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Franz",
                args!["I...", "I've got stuff", "I should be working", "on. Yes. So very busy."],
            )?;
            return ctx.close();
        }
    }
    Ok(())
}

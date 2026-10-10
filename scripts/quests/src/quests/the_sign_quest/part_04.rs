use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn aaron_sign_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_npc_p = Val::from(0);
    let mut l_user_p = Val::from(0);
    shared::quests_the_sign_quest::f_updatesignvars(ctx, vec![])?;
    ctx.mes("[Aaron]")?;
    if ctx.var("sign_q").get()? == 21 {
        ctx.lines(args!["Alright, before", "we begin, let me", "tell you the rules..."])?;
        ctx.next()?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_PROFUSELY_SWEAT")?])?;
        ctx.lines_as(
            "Aaron",
            args![
                "I'm going to give you",
                "ten multiple choice questions.",
                "Since this is an impromptu",
                "exhibition bout, I'm going to",
                "use questions that I use",
                "in teaching my class."
            ],
        )?;
        ctx.next()?;
        ctx.var("sign_q").set(Val::from(22))?;
        ctx.lines_as(
            "Aaron",
            args![
                "Alright...",
                "I'm almost finished",
                "compiling the questions.",
                "We're ready to go when",
                "you're ready to begin!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("sign_q").get()? == 22 {
        let subject1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
        if subject1 == 1 {
            ctx.lines(args!["1. Which NPC is", "not relevant to the", "Blacksmith Job Quest?"])?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Baisulist:Wickebine:Barcardi:Krongast")],
            )?) == 3
            {
                l_user_p = (l_user_p.clone() + Val::from(1));
            }
        } else if subject1 == 2 {
            ctx.lines(args!["1. Which item is not", "relevant to the creation", "of a Counteragent?"])?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Karvodailnirol:Detrimindexta:Alcohol")],
            )?) != 1
            {
                l_user_p = (l_user_p.clone() + Val::from(1));
            }
        } else if subject1 == 3 {
            ctx.lines(args![
                "1. Choose the",
                "monster that is",
                "a different size",
                "than the others."
            ])?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Giant Whisper:Marine:Cornutus:Kobold Archer")],
            )?) == 2
            {
                l_user_p = (l_user_p.clone() + Val::from(1));
            }
        }
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?.number()? < 9 {
            ctx.call(Function::DoNpcEvent, vec![Val::from("Jesqurienne#sign::OnHo")])?;
            ctx.lines_as("Jesqurienne", args!["Heh heh...", "The first question", "is always too easy!"])?;
            l_npc_p = (l_npc_p.clone() + Val::from(1));
        } else {
            ctx.call(Function::DoNpcEvent, vec![Val::from("Jesqurienne#sign::OnOmg")])?;
            ctx.lines_as("Jesqurienne", args!["Huh...?", "How can I not know", "the answer to this?!"])?;
        }
        ctx.next()?;
        let subject2 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
        if subject2 == 1 {
            ctx.lines_as(
                "Aaron",
                args![
                    "2. Choose the skill",
                    "related to the Priest's",
                    "B.S. Sacramenti from the",
                    "ones displayed in the list."
                ],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Lex Divina:Gloria:Recovery:Sanctuary")],
            )?) == 2
            {
                l_user_p = (l_user_p.clone() + Val::from(1));
            }
        } else if subject2 == 2 {
            ctx.lines_as(
                "Aaron",
                args![
                    "2. Choose the material that",
                    "is not related to the creation",
                    "of a Condensed White Potion."
                ],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Medicine Bowl:Witch Starsand:Empty Bottle:Empty Potion Bottle")],
            )?) == 3
            {
                l_user_p = (l_user_p.clone() + Val::from(1));
            }
        } else if subject2 == 3 {
            ctx.lines_as(
                "Aaron",
                args!["2. Choose the item that is", "necessary for a Blacksmith", "to create a Gladius."],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("Sapphire:Zircon:Topaz:Cursed Ruby")])?) == 1 {
                l_user_p = (l_user_p.clone() + Val::from(1));
            }
        }
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?.number()? < 8 {
            ctx.call(Function::DoNpcEvent, vec![Val::from("Jesqurienne#sign::OnHo")])?;
            ctx.lines_as(
                "Jesqurienne",
                args!["Ho ho ho ho~!", "I know the answer.", "Any fool should know", "this. But do you?"],
            )?;
            l_npc_p = (l_npc_p.clone() + Val::from(1));
        } else {
            ctx.call(Function::DoNpcEvent, vec![Val::from("Jesqurienne#sign::OnOmg")])?;
            ctx.lines_as(
                "Jesqurienne",
                args![
                    "Oh...",
                    "I knew the answer",
                    "to this just yesterday!",
                    "Why can't I think of it now?!"
                ],
            )?;
        }
        ctx.next()?;
        let subject3 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
        if subject3 == 1 {
            ctx.lines_as(
                "Aaron",
                args!["3. Choose the", "property that is", "unrelated to the", "Mage's Bolt type skills."],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("Water:Earth:Fire:Wind")])?) == 2 {
                l_user_p = (l_user_p.clone() + Val::from(1));
            }
        } else if subject3 == 2 {
            ctx.lines_as("Aaron", args!["3. What is the", "Bunny Band's DEF", "and its added ability?"])?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("1 / LUK +2:1 / LUK +5:2 / LUK +2:2 / LUK +5")],
            )?) == 3
            {
                l_user_p = (l_user_p.clone() + Val::from(1));
            }
        } else if subject3 == 3 {
            ctx.lines_as(
                "Aaron",
                args![
                    "3. Choose the prefix or",
                    "suffix that is incorrectly",
                    "matched with its Monster",
                    "Card name."
                ],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Hornet Card - Martyr:Requiem - Chaos:Wormtail - Clever:Golem - Immortal")],
            )?) == 4
            {
                l_user_p = (l_user_p.clone() + Val::from(1));
            }
        }
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?.number()? < 8 {
            ctx.call(Function::DoNpcEvent, vec![Val::from("Jesqurienne#sign::OnHo")])?;
            ctx.lines_as("Jesqurienne", args!["Ah...", "That's another", "point for me!"])?;
            l_npc_p = (l_npc_p.clone() + Val::from(1));
        } else {
            ctx.call(Function::DoNpcEvent, vec![Val::from("Jesqurienne#sign::OnOmg")])?;
            ctx.lines_as(
                "Jesqurienne",
                args!["z...z...Z", "W-what? Oh!", "Well, I won't need", "that point anyway..."],
            )?;
        }
        ctx.next()?;
        ctx.lines_as(
            "Jesqurienne",
            args!["Let's check our", "scores, shall we?", "................."],
        )?;
        ctx.next()?;
        if runtime::op(&l_npc_p.clone(), ">", &l_user_p.clone())?.is_true() {
            ctx.lines_as(
                "Jesqurienne",
                args![
                    "Heh heh...",
                    "It looks like",
                    "I'm beating you!",
                    "But you shouldn't",
                    "be surprised..."
                ],
            )?;
        } else if l_npc_p.clone().loosely_equals(&l_user_p.clone()) {
            ctx.lines_as("Jesqurienne", args!["Still, you're", "smarter than", "I thought you'd be..."])?;
        } else {
            ctx.lines_as(
                "Jesqurienne",
                args![
                    "W-wait a minute!",
                    "H-how could you",
                    "have more points",
                    "than me?! I better",
                    "get serious!"
                ],
            )?;
        }
        ctx.next()?;
        ctx.lines_as("Aaron", args!["^333333*Ahem!*^000000", "Question", "number four!"])?;
        ctx.next()?;
        let subject4 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(2)])?;
        if subject4 == 1 {
            ctx.lines_as(
                "Aaron",
                args!["4. Choose the correct", "name of the ruler of the", "Rune-Midgarts Kingdom."],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Tristan lll:Tristram lll:Tristar lll:Trust lll")],
            )?) == 2
            {
                l_user_p = (l_user_p.clone() + Val::from(1));
            }
        } else if subject4 == 2 {
            ctx.lines_as(
                "Aaron",
                args!["4. Choose the monster", "that is a different type", "than the others."],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("Penomena:Hatii:Pest:Explosion")])?) == 1 {
                l_user_p = (l_user_p.clone() + Val::from(1));
            }
        }
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?.number()? < 8 {
            ctx.call(Function::DoNpcEvent, vec![Val::from("Jesqurienne#sign::OnHo")])?;
            ctx.lines_as("Jesqurienne", args!["That counts as", "a question? You're", "being too easy!"])?;
            l_npc_p = (l_npc_p.clone() + Val::from(1));
        } else {
            ctx.call(Function::DoNpcEvent, vec![Val::from("Jesqurienne#sign::OnOmg")])?;
            ctx.lines_as(
                "Jesqurienne",
                args!["I know the", "answer! W-why", "can't I think of", "it right now?!"],
            )?;
        }
        ctx.next()?;
        if runtime::op(&l_npc_p.clone(), ">", &l_user_p.clone())?.is_true() {
            ctx.lines_as(
                "Jesqurienne",
                args![
                    "Heh heh~ It looks",
                    "like I have the lead.",
                    "But that's only natural for",
                    "someone as brilliant as me~"
                ],
            )?;
        } else if l_npc_p.clone().loosely_equals(&l_user_p.clone()) {
            ctx.lines_as(
                "Jesqurienne",
                args![
                    "Up till now",
                    "you've managed",
                    "to keep up with",
                    "me. Huh. So you're",
                    "not a total idiot..."
                ],
            )?;
        } else {
            ctx.lines_as(
                "Jesqurienne",
                args![
                    "Heh heh~ It looks",
                    "like I have the lead.",
                    "But that's only natural for",
                    "someone as brilliant a--",
                    "Wait. How do you have",
                    "more points than me?!"
                ],
            )?;
        }
        ctx.next()?;
        let subject5 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
        if subject5 == 1 {
            ctx.lines_as(
                "Aaron",
                args!["5. Choose the monster", "which does not drop the", "'Yggdrasil Leaf' item."],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Marduk:Baphomet Jr.:Angeling:Wanderer")],
            )?) == 1
            {
                l_user_p = (l_user_p.clone() + Val::from(1));
            }
        } else if subject5 == 2 {
            ctx.lines_as("Aaron", args!["5. Choose the job class", "that cannot equip Silk Robe."])?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("Swordman:Merchant:Hunter:Mage")])?) == 3 {
                l_user_p = (l_user_p.clone() + Val::from(1));
            }
        } else if subject5 == 3 {
            ctx.lines_as(
                "Aaron",
                args!["5. Choose the level", "requirement for entering", "the PvP Room."],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("30:31:32:33")])?) == 2 {
                l_user_p = (l_user_p.clone() + Val::from(1));
            }
        }
        ctx.var("zis_5").set(ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?)?;
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?.number()? < 9 {
            ctx.call(Function::DoNpcEvent, vec![Val::from("Jesqurienne#sign::OnHo")])?;
            ctx.lines_as(
                "Jesqurienne",
                args![
                    "I didn't have",
                    "to think very hard",
                    "to know the answer.",
                    "Are you still thinking?",
                    "I wouldn't think you'd know!"
                ],
            )?;
            l_npc_p = (l_npc_p.clone() + Val::from(1));
        } else {
            ctx.call(Function::DoNpcEvent, vec![Val::from("Jesqurienne#sign::OnOmg")])?;
            ctx.lines_as(
                "Jesqurienne",
                args!["I...", "Hold on,", "I know this one...", "(Oh craaaaaap!)"],
            )?;
        }
        ctx.next()?;
        if runtime::op(&l_npc_p.clone(), ">", &l_user_p.clone())?.is_true() {
            ctx.lines_as(
                "Jesqurienne",
                args![
                    "Oh? What a surprise.",
                    "I'm in the lead. I know",
                    "the suspense must be",
                    "killing you, but I'm pretty",
                    "sure who the winner will",
                    "be. Me me me meee~"
                ],
            )?;
        } else if l_npc_p.clone().loosely_equals(&l_user_p.clone()) {
            ctx.lines_as(
                "Jesqurienne",
                args![
                    "Are you copying",
                    "my answers? Because",
                    "there's no way we can",
                    "be tied right now..."
                ],
            )?;
        } else {
            ctx.lines_as(
                "Jesqurienne",
                args!["I...", "I must be more", "drunk than I thought", "if I'm losing right now..."],
            )?;
        }
        ctx.next()?;
        let subject6 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
        if subject6 == 1 {
            ctx.lines_as("Aaron", args!["6. What is the", "correct weight", "for 1 Empty Bottle?"])?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("1:2:3")])?) == 2 {
                l_user_p = (l_user_p.clone() + Val::from(1));
            }
        } else if subject6 == 2 {
            ctx.lines_as("Aaron", args!["6. Choose the", "correct DEF for", "the Indian Filet item."])?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("0:1:2:3")])?) == 4 {
                l_user_p = (l_user_p.clone() + Val::from(1));
            }
        } else if subject6 == 3 {
            ctx.lines_as("Aaron", args!["6. What is the", "city closest to", "Turtle Island?"])?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("Al De Baran:Alberta:Comodo:Izlude")])?) == 2 {
                l_user_p = (l_user_p.clone() + Val::from(1));
            }
        }
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?.number()? < 9 {
            ctx.call(Function::DoNpcEvent, vec![Val::from("Jesqurienne#sign::OnHo")])?;
            ctx.lines_as(
                "Jesqurienne",
                args!["^333333*Sigh...*^000000", "Simple questions,", "simple answers.", "Did you get it?"],
            )?;
            l_npc_p = (l_npc_p.clone() + Val::from(1));
        } else {
            ctx.call(Function::DoNpcEvent, vec![Val::from("Jesqurienne#sign::OnOmg")])?;
            ctx.lines_as(
                "Jesqurienne",
                args!["W-wait...!", "I know the", "answer to this", "one! Just let me", "th-think...!"],
            )?;
        }
        ctx.next()?;
        if runtime::op(&l_npc_p.clone(), ">", &l_user_p.clone())?.is_true() {
            ctx.lines_as(
                "Jesqurienne",
                args![
                    "Oh, look at that.",
                    "I have more points",
                    "than you. I know, it",
                    "must be frustrating",
                    "trying to keep up."
                ],
            )?;
        } else if l_npc_p.clone().loosely_equals(&l_user_p.clone()) {
            ctx.lines_as(
                "Jesqurienne",
                args![
                    "Mmm...?",
                    "How are you cheating?!",
                    "You having the same score",
                    "as me must be impossible!"
                ],
            )?;
        } else {
            ctx.lines_as(
                "Jesqurienne",
                args![
                    "I don't understand.",
                    "You have a higher sco--",
                    "I don't-- How can yo--",
                    "Why is this happening?"
                ],
            )?;
        }
        ctx.next()?;
        let subject7 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
        if subject7 == 1 {
            ctx.lines_as(
                "Aaron",
                args![
                    "7. Fifty-one multiplied",
                    "by fifteen, divided by three,",
                    "plus five is equal to...?"
                ],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("250:255:260:265")])?) == 3 {
                l_user_p = (l_user_p.clone() + Val::from(1));
            }
        } else if subject7 == 2 {
            ctx.lines_as(
                "Aaron",
                args!["7. Four thousand five hundred sixty divided by four, divided by two, plus three is equal to...?"],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("573:574:575:576")])?) == 1 {
                l_user_p = (l_user_p.clone() + Val::from(1));
            }
        } else if subject7 == 3 {
            ctx.lines_as(
                "Aaron",
                args![
                    "7. Three thousand one hundred",
                    "two added to five hundred, plus four, divided by six equals..."
                ],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("600:601:602")])?) == 2 {
                l_user_p = (l_user_p.clone() + Val::from(1));
            }
        }
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?.number()? < 9 {
            ctx.call(Function::DoNpcEvent, vec![Val::from("Jesqurienne#sign::OnHo")])?;
            ctx.lines_as(
                "Jesqurienne",
                args!["Math?", "My mind is", "a veritable", "calculator!", "Ho ho ho~!"],
            )?;
            l_npc_p = (l_npc_p.clone() + Val::from(1));
        } else {
            ctx.call(Function::DoNpcEvent, vec![Val::from("Jesqurienne#sign::OnOmg")])?;
            ctx.lines_as(
                "Jesqurienne",
                args![
                    "What the--?!",
                    "Why is all the math",
                    "written out in words",
                    "instead of the traditional",
                    "numerics and symbols?!",
                    "A-answer me--!!"
                ],
            )?;
        }
        ctx.next()?;
        if runtime::op(&l_npc_p.clone(), ">", &l_user_p.clone())?.is_true() {
            ctx.lines_as(
                "Jesqurienne",
                args![
                    "Ah. Did I tell you",
                    "I was in the lead",
                    "right now? Well, let",
                    "me remind you, in case",
                    "I forget. Ho ho ho ho~!"
                ],
            )?;
        } else if l_npc_p.clone().loosely_equals(&l_user_p.clone()) {
            ctx.lines_as(
                "Jesqurienne",
                args![
                    "That's it.",
                    "I don't understand",
                    "how your score is",
                    "equal to mine. Are",
                    "you getting bonus",
                    "points somehow?!"
                ],
            )?;
        } else {
            ctx.lines_as(
                "Jesqurienne",
                args![
                    "There's no other",
                    "explanation! You've",
                    "got to be using some",
                    "sort of crib sheet! But",
                    "where are you hiding it?"
                ],
            )?;
        }
        ctx.next()?;
        let subject8 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(2)])?;
        if subject8 == 1 {
            ctx.lines_as(
                "Aaron",
                args![
                    "8. Choose the building",
                    "that is the closest to the",
                    "Item Upgrade Place in Juno."
                ],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Tavern:Monster Library:Tool Shop:Weapon Shop")],
            )?) == 4
            {
                l_user_p = (l_user_p.clone() + Val::from(1));
            }
        } else if subject8 == 2 {
            ctx.lines_as(
                "Aaron",
                args!["8. Choose the Hunter's", "Trap skill which does not", "inflict Property Damage."],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Claymore Trap:Freezing Trap:Shockwave Trap:Land Mine")],
            )?) == 3
            {
                l_user_p = (l_user_p.clone() + Val::from(1));
            }
        }
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?.number()? < 9 {
            ctx.call(Function::DoNpcEvent, vec![Val::from("Jesqurienne#sign::OnHo")])?;
            ctx.lines_as("Jesqurienne", args!["Hah hah~!", "That was a", "piece of cake!"])?;
            l_npc_p = (l_npc_p.clone() + Val::from(1));
        } else {
            ctx.call(Function::DoNpcEvent, vec![Val::from("Jesqurienne#sign::OnOmg")])?;
            ctx.lines_as("Jesqurienne", args!["Grrr...", "Why don't", "I know this one?!"])?;
        }
        ctx.next()?;
        if runtime::op(&l_npc_p.clone(), ">", &l_user_p.clone())?.is_true() {
            ctx.lines_as(
                "Jesqurienne",
                args![
                    "I know, you're wondering",
                    "how such an intelligent",
                    "person can exist. Well,",
                    "I guess my intellect sort",
                    "of balances the stupidity",
                    "of the rest of the world..."
                ],
            )?;
        } else if l_npc_p.clone().loosely_equals(&l_user_p.clone()) {
            ctx.lines_as(
                "Jesqurienne",
                args![
                    "How can our scores",
                    "be equal? No... No...",
                    "Think, Jesqurienne...",
                    "Time travel isn't possible.",
                    "But why would you time travel",
                    "for the answers to a quiz?"
                ],
            )?;
        } else {
            ctx.lines_as(
                "Jesqurienne",
                args![
                    "I... I don't",
                    "get it. H-how are",
                    "you w-winning? M-my",
                    "whole perception of",
                    "reality is starting t-to..."
                ],
            )?;
        }
        ctx.next()?;
        let subject9 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(2)])?;
        if subject9 == 1 {
            ctx.lines_as(
                "Aaron",
                args!["9. Which monster would", "receive the most damage", "from a Fire Property Dagger?"],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Dagger Goblin:Mace Goblin:Morning Star Goblin:Hammer Goblin")],
            )?) == 4
            {
                l_user_p = (l_user_p.clone() + Val::from(1));
            }
        } else if subject9 == 2 {
            ctx.lines_as(
                "Aaron",
                args![
                    "9. Choose the monster",
                    "on which the Mage skill,",
                    "''Stone Curse,'' is ineffective. ^FFFFFFaaaaaa aaaaaaa aaaaaa aaaaaaa aaaaaaaaa^000000"
                ],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Elder Willow:Evil Druid:Magnolia:Marc")],
            )?) == 2
            {
                l_user_p = (l_user_p.clone() + Val::from(1));
            }
        }
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?.number()? < 9 {
            ctx.call(Function::DoNpcEvent, vec![Val::from("Jesqurienne#sign::OnHo")])?;
            ctx.lines_as(
                "Jesqurienne",
                args![
                    "Ahhh...",
                    "I'm just breezing",
                    "through this Quiz Challenge!",
                    "Aren't you having a hard time?"
                ],
            )?;
            l_npc_p = (l_npc_p.clone() + Val::from(1));
        } else {
            ctx.call(Function::DoNpcEvent, vec![Val::from("Jesqurienne#sign::OnOmg")])?;
            ctx.lines_as(
                "Jesqurienne",
                args!["Huh...?", "I don't understand", "why I don't know this..."],
            )?;
        }
        ctx.next()?;
        if runtime::op(&l_npc_p.clone(), ">", &l_user_p.clone())?.is_true() {
            ctx.lines_as(
                "Jesqurienne",
                args![
                    "Bwahahaha!",
                    "Only one question",
                    "left! You have to get",
                    "this one right or I'll",
                    "call you stupid forever!"
                ],
            )?;
        } else if l_npc_p.clone().loosely_equals(&l_user_p.clone()) {
            ctx.lines_as(
                "Jesqurienne",
                args![
                    "We're tied?!",
                    "Oh no... I better",
                    "get this last question",
                    "right. (And I hope that",
                    "you get it wrong!)"
                ],
            )?;
        } else {
            ctx.lines_as(
                "Jesqurienne",
                args!["I'm losing?!", "And we're at the", "last question already?!"],
            )?;
        }
        ctx.next()?;
        let subject10 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(2)])?;
        if subject10 == 1 {
            ctx.lines_as(
                "Aaron",
                args!["10. Choose the NPC", "that looks different", "than all the others."],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from(
                    "Prontera Hollgrehenn:Prontera Doll Merchant:Izlude Meat Merchant:Morocc Meat Merchant",
                )],
            )?) == 3
            {
                l_user_p = (l_user_p.clone() + Val::from(1));
            }
        } else if subject10 == 2 {
            ctx.lines_as(
                "Aaron",
                args!["10. Choose the item", "that cannot be equipped", "by Novice class characters."],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Funeral Hat:Shackles:Wooden Mail:Pantie")],
            )?) == 3
            {
                l_user_p = (l_user_p.clone() + Val::from(1));
            }
        }
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?.number()? < 9 {
            ctx.call(Function::DoNpcEvent, vec![Val::from("Jesqurienne#sign::OnHo")])?;
            ctx.lines_as(
                "Jesqurienne",
                args!["As usual, I got", "the last question", "absolutely right~"],
            )?;
            l_npc_p = (l_npc_p.clone() + Val::from(1));
        } else {
            ctx.call(Function::DoNpcEvent, vec![Val::from("Jesqurienne#sign::OnOmg")])?;
            ctx.lines_as(
                "Jesqurienne",
                args!["Huh...", "Wh-what was", "the question again?", "Nooo, I missed it!"],
            )?;
        }
        ctx.next()?;
        ctx.lines_as(
            "Aaron",
            args![
                "Okay, the quiz",
                "is over! Now, I'll",
                "reveal the results",
                "of the competitors..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Aaron",
            args![
                "Umm...",
                ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(":")),
                ((Val::from("") + l_user_p.clone()) + Val::from(" points!"))
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Aaron",
            args!["Jesqurienne...", ((Val::from("") + l_npc_p.clone()) + Val::from(" points!"))],
        )?;
        ctx.next()?;
        if runtime::op(&l_npc_p.clone(), ">", &l_user_p.clone())?.is_true() {
            ctx.var("sign_q").set(Val::from(23))?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_KIK")?])?;
            ctx.lines_as("Jesqurienne", args!["Bwahahaha!", "I win! I knew it!", "Ho ho ho ho ho ho~"])?;
        } else if l_npc_p.clone().loosely_equals(&l_user_p.clone()) {
            ctx.var("sign_q").set(Val::from(24))?;
            ctx.lines_as(
                "Jesqurienne",
                args![
                    "Tied?! ",
                    "Alright, I'm impressed.",
                    "I'll admit, you're much",
                    "smarter than I thought..."
                ],
            )?;
        } else {
            ctx.var("sign_q").set(Val::from(24))?;
            ctx.lines_as("Jesqurienne", args!["No...!", "I can't...", "Believe this..."])?;
        }
        ctx.next()?;
        ctx.lines_as(
            "Aaron",
            args![
                "Thank you",
                "for playing!",
                ".......................",
                "...........................",
                "Alright, back to drinking!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
        ctx.lines(args![
            "Goodness...!",
            "Either these beer",
            "goggles are on too tight,",
            "or you're the most gorgeous",
            "woman I've seen in my life!"
        ])?;
        ctx.next()?;
        if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
            let choice = runtime::select_values(ctx, &[Val::from("Um, I'm a dude...")])?;
            ctx.var("@menu").set(choice)?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
            ctx.lines_as(
                "Aaron",
                args![
                    "N-no...!",
                    "M-maybe I should",
                    "take it easy on the",
                    "alcohol. I do have classes",
                    "to teach tomorrow anyway..."
                ],
            )?;
        } else {
            let choice = runtime::select_values(ctx, &[Val::from("Why, thank you~")])?;
            ctx.var("@menu").set(choice)?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_BEST")?])?;
            ctx.lines_as(
                "Aaron",
                args![
                    "Ah, yes. Your",
                    "sense of style, your",
                    "magnificent body. Everything",
                    "about you is beautiful! Not just that, but you're also intelligent!"
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_BIGTHROB")?])?;
            ctx.lines_as(
                "Aaron",
                args![
                    "I should know!",
                    "Not only do I teach",
                    "classes, but I'm also",
                    "a Quiz Challenge moderater",
                    "master in my spare time~",
                    "You're truly a jewel..."
                ],
            )?;
        }
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn aaron_sign(ctx: &Ctx) -> Script {
    aaron_sign_body(ctx, Vec::new()).map(|_| ())
}

fn aaron_sign_onsmile_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::Emotion, vec![ctx.constant("ET_SMILE")?])?;
    return Err(Stop::End);
}

pub fn aaron_sign_onsmile(ctx: &Ctx) -> Script {
    aaron_sign_onsmile_body(ctx, Vec::new()).map(|_| ())
}

fn strange_guy_sign_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_diaris_t = Val::from(0);
    shared::quests_the_sign_quest::f_updatesignvars(ctx, vec![])?;
    ctx.mes("[Dearles]")?;
    if ctx.var("sign_q").get()?.number()? < 25 {
        ctx.lines(args!["No...! Damn it,", "not again! When's", "my lucky streak", "gonna start?"])?;
        ctx.next()?;
        ctx.lines_as("Dearles", args!["Hey, you've got that look", "like you want something from"])?;
        if ctx.var("Zeny").get()?.number()? < 10000 {
            ctx.mes("me. Hah! You are poor!")?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_HNG")?])?;
            ctx.lines(args![
                "It also looks like you're low on cash~! Well, this is what guys like me to do to shameless",
                "moochers like you!"
            ])?;
            ctx.call(Function::PercentHeal, vec![Val::from(-10), Val::from(0)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines(args![
                "me. It also looks like you got zeny to spare! I think I'll help myself and borrow some of your cash!",
                "^FF0000Yoink!^000000"
            ])?;
            ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(10000))?))?;
            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_COIN")?])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        if ctx.var("sign_q").get()? == 25 {
            ctx.lines(args!["No...! Damn it,", "not again! When's", "my lucky streak", "gonna start?"])?;
            ctx.next()?;
            'b1: {
                let subject1 = Val::from(runtime::select_values(ctx, &[Val::from("Excuse me...:Just pass by.")])?);
                let mut matched1 = false;
                let no_case1 = !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2));
                if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                    matched1 = true;
                }
                if matched1 {
                    ctx.lines_as(
                        "Dearles",
                        args![
                            "What is it?!",
                            "I'm not in the mood",
                            "for chatting with complete",
                            "and total strangers, so",
                            "get to the point~!"
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Oh, nothing.:I'm here for Metz...?")])? {
                        1 => {
                            ctx.lines_as(
                                "Dearles",
                                args![
                                    "Wha...? You bother",
                                    "me and make lose this",
                                    "game for nothing? For that,",
                                    "I'll freakin' beat you to near",
                                    "freakin' death! Bam bam bam!"
                                ],
                            )?;
                            ctx.close_window()?;
                            ctx.call(Function::PercentHeal, vec![Val::from(-100), Val::from(0)])?;
                            ctx.call(Function::SoundEffect, vec![Val::from("effect\\sign_noise.wav"), Val::from(1)])?;
                            ctx.call(Function::PercentHeal, vec![Val::from(-99), Val::from(0)])?;
                            ctx.call(Function::Warp, vec![Val::from("comodo"), Val::from(122), Val::from(100)])?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as("Dearles", args!["Oh. You're here for", "the Sobbing Starlight, huh? If you wanna take my test, there's *ahem* a fee of 30,000 zeny. So take it or leave it, capish?"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Dearles",
                                args![
                                    "I'll have nothing",
                                    "to do with you if you",
                                    "can't pay up. Just think",
                                    "of the fee as a preliminary",
                                    "for my test. You know, to",
                                    "weed out the riffraff."
                                ],
                            )?;
                            ctx.var("sign_q").set(Val::from(26))?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                }
                if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                    matched1 = true;
                }
                if matched1 {
                    ctx.lines_as(
                        "Dearles",
                        args![
                            "Crap...!",
                            "I'm almost broke!",
                            "But I gotta win all",
                            "my cash back! How can",
                            "I raise more money fast",
                            "without actually working..."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        } else {
            if ctx.var("sign_q").get()? == 26 {
                ctx.lines(args!["Eh heh heh~", "Soooooo, did you", "bring the money?"])?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Here you are...:Not yet...")])? {
                    1 => {
                        ctx.lines_as("Dearles", args!["Ummm......."])?;
                        if ctx.var("Zeny").get()?.number()? < 30000 {
                            ctx.lines(args![
                                "When I say 30,000 zeny,",
                                "I mean 30,000 zeny! I'll let",
                                "you off on account of you looking like you're not smart enough to know how to count. But pull this",
                                "on me again and I'll...!"
                            ])?;
                        } else {
                            ctx.lines(args![
                                "Heh heh~",
                                "That's thirty grand,",
                                "alright! Okay, come back",
                                "to me tomorrow night and",
                                "we'll start your test..."
                            ])?;
                            ctx.var("sign_q").set(Val::from(27))?;
                            ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(30000))?))?;
                        }
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as(
                            "Dearles",
                            args![
                                "Huh...?",
                                "Alright, but you",
                                "better hurry. I can",
                                "change my mind at",
                                "any time, you know!"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            } else {
                if ctx.var("sign_q").get()? == 27 {
                    if (ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? > 18
                        && ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? < 23)
                    {
                        ctx.lines(args![
                            "Nice, you're here",
                            "just in time. Well,",
                            "all that matters is that",
                            "you come during the night."
                        ])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Dearles",
                            args![
                                "You're wondering how",
                                "a guy like me is pals with",
                                "Metz, but that's none of your",
                                "business. I guess I gotta keep",
                                "this promise to him, so when",
                                "you're ready, say the word."
                            ],
                        )?;
                        ctx.var("sign_q").set(Val::from(28))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines(args![
                            "Hey... When I say",
                            "''tomorrow night,'' I mean,",
                            "come and talk to me at nighttime! Don't you know anything about shady dealings?! Pfft! Adventurers..."
                        ])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                } else {
                    if ctx.var("sign_q").get()? == 28 {
                        ctx.lines(args![
                            "Alright! I want you to",
                            "bring me a bunch of items!",
                            "Yeah, I know, you're a real",
                            "pro at this, but let me set",
                            "you straight before you blow",
                            "this off as a piece of cake..."
                        ])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Dearles",
                            args![
                                "I'm not gonna give you an",
                                "exact list of items I want. Oh",
                                "no, what you gotta do is bring",
                                "me artsy crafts. This test is",
                                "gonna judge your appreciation",
                                "for... Craftsmanship~"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Dearles",
                            args![
                                "Since I'm seeing whether",
                                "or not you got good taste,",
                                "variety is the important thing",
                                "here. So don't bring freakin'",
                                "a hundred of the same object.",
                                "Just one of each kind'll do."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Dearles",
                            args!["What are you", "standin' around", "waiting for? Bring", "those collection items!"],
                        )?;
                        ctx.var("sign_q").set(Val::from(29))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("sign_q").get()?.number()? < 35 {
                        'b4: {
                            let subject4 = ctx.var("sign_q").get()?;
                            let mut matched4 = false;
                            let no_case4 = !subject4.loosely_equals(&Val::from(29))
                                && !subject4.loosely_equals(&Val::from(30))
                                && !subject4.loosely_equals(&Val::from(31))
                                && !subject4.loosely_equals(&Val::from(32))
                                && !subject4.loosely_equals(&Val::from(33))
                                && !subject4.loosely_equals(&Val::from(34));
                            if !matched4 && subject4.loosely_equals(&Val::from(29)) {
                                matched4 = true;
                            }
                            if matched4 {
                                ctx.lines(args!["Ah, so is this", "the stuff you brought?", "Lemme take a look-see..."])?;
                                if ctx.call(Function::CountItem, vec![Val::from(734)])?.is_true() {
                                    l_diaris_t = (l_diaris_t.clone() + Val::from(1));
                                }
                                if ctx.call(Function::CountItem, vec![Val::from(735)])?.is_true() {
                                    l_diaris_t = (l_diaris_t.clone() + Val::from(1));
                                }
                                if ctx.call(Function::CountItem, vec![Val::from(736)])?.is_true() {
                                    l_diaris_t = (l_diaris_t.clone() + Val::from(1));
                                }
                                if ctx.call(Function::CountItem, vec![Val::from(7149)])?.is_true() {
                                    l_diaris_t = (l_diaris_t.clone() + Val::from(1));
                                }
                                if ctx.call(Function::CountItem, vec![Val::from(747)])?.is_true() {
                                    l_diaris_t = (l_diaris_t.clone() + Val::from(2));
                                }
                                if ctx.call(Function::CountItem, vec![Val::from(749)])?.is_true() {
                                    l_diaris_t = (l_diaris_t.clone() + Val::from(4));
                                }
                                if ctx.call(Function::CountItem, vec![Val::from(740)])?.is_true() {
                                    l_diaris_t = (l_diaris_t.clone() + Val::from(1));
                                }
                                if ctx.call(Function::CountItem, vec![Val::from(741)])?.is_true() {
                                    l_diaris_t = (l_diaris_t.clone() + Val::from(1));
                                }
                                if ctx.call(Function::CountItem, vec![Val::from(742)])?.is_true() {
                                    l_diaris_t = (l_diaris_t.clone() + Val::from(2));
                                }
                                if ctx.call(Function::CountItem, vec![Val::from(743)])?.is_true() {
                                    l_diaris_t = (l_diaris_t.clone() + Val::from(3));
                                }
                                if ctx.call(Function::CountItem, vec![Val::from(752)])?.is_true() {
                                    l_diaris_t = (l_diaris_t.clone() + Val::from(3));
                                }
                                if ctx.call(Function::CountItem, vec![Val::from(753)])?.is_true() {
                                    l_diaris_t = (l_diaris_t.clone() + Val::from(4));
                                }
                                if ctx.call(Function::CountItem, vec![Val::from(754)])?.is_true() {
                                    l_diaris_t = (l_diaris_t.clone() + Val::from(4));
                                }
                                if ctx.call(Function::CountItem, vec![Val::from(750)])?.is_true() {
                                    l_diaris_t = (l_diaris_t.clone() + Val::from(7));
                                }
                                if ctx.call(Function::CountItem, vec![Val::from(751)])?.is_true() {
                                    l_diaris_t = (l_diaris_t.clone() + Val::from(7));
                                }
                                ctx.next()?;
                                if l_diaris_t.clone().number()? > 10 {
                                    ctx.lines_as(
                                        "Dearles",
                                        args![
                                            "Nice, nice~",
                                            "These'll sell for--",
                                            "Er, I'm glad to say",
                                            "that you pass for now."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Dearles",
                                        args![
                                            "I think you're ready",
                                            "for the next step. Now,",
                                            "I'm gonna send you to",
                                            "Lamadin for Part Two",
                                            "of my test. Yeah, I know..."
                                        ],
                                    )?;
                                    if ctx.call(Function::CountItem, vec![Val::from(734)])?.is_true() {
                                        ctx.call(Function::DelItem, vec![Val::from(734), Val::from(1)])?;
                                    }
                                    if ctx.call(Function::CountItem, vec![Val::from(735)])?.is_true() {
                                        ctx.call(Function::DelItem, vec![Val::from(735), Val::from(1)])?;
                                    }
                                    if ctx.call(Function::CountItem, vec![Val::from(736)])?.is_true() {
                                        ctx.call(Function::DelItem, vec![Val::from(736), Val::from(1)])?;
                                    }
                                    if ctx.call(Function::CountItem, vec![Val::from(7149)])?.is_true() {
                                        ctx.call(Function::DelItem, vec![Val::from(7149), Val::from(1)])?;
                                    }
                                    if ctx.call(Function::CountItem, vec![Val::from(747)])?.is_true() {
                                        ctx.call(Function::DelItem, vec![Val::from(747), Val::from(1)])?;
                                    }
                                    if ctx.call(Function::CountItem, vec![Val::from(749)])?.is_true() {
                                        ctx.call(Function::DelItem, vec![Val::from(749), Val::from(1)])?;
                                    }
                                    if ctx.call(Function::CountItem, vec![Val::from(740)])?.is_true() {
                                        ctx.call(Function::DelItem, vec![Val::from(740), Val::from(1)])?;
                                    }
                                    if ctx.call(Function::CountItem, vec![Val::from(741)])?.is_true() {
                                        ctx.call(Function::DelItem, vec![Val::from(741), Val::from(1)])?;
                                    }
                                    if ctx.call(Function::CountItem, vec![Val::from(742)])?.is_true() {
                                        ctx.call(Function::DelItem, vec![Val::from(742), Val::from(1)])?;
                                    }
                                    if ctx.call(Function::CountItem, vec![Val::from(743)])?.is_true() {
                                        ctx.call(Function::DelItem, vec![Val::from(743), Val::from(1)])?;
                                    }
                                    if ctx.call(Function::CountItem, vec![Val::from(752)])?.is_true() {
                                        ctx.call(Function::DelItem, vec![Val::from(752), Val::from(1)])?;
                                    }
                                    if ctx.call(Function::CountItem, vec![Val::from(753)])?.is_true() {
                                        ctx.call(Function::DelItem, vec![Val::from(753), Val::from(1)])?;
                                    }
                                    if ctx.call(Function::CountItem, vec![Val::from(754)])?.is_true() {
                                        ctx.call(Function::DelItem, vec![Val::from(754), Val::from(1)])?;
                                    }
                                    if ctx.call(Function::CountItem, vec![Val::from(750)])?.is_true() {
                                        ctx.call(Function::DelItem, vec![Val::from(750), Val::from(1)])?;
                                    }
                                    if ctx.call(Function::CountItem, vec![Val::from(751)])?.is_true() {
                                        ctx.call(Function::DelItem, vec![Val::from(751), Val::from(1)])?;
                                    }
                                    ctx.var("sign_q").set(Val::from(30))?;
                                    {
                                        if ctx.var("BaseLevel").get()?.number()? < 60 {
                                            ctx.call(Function::GetExperience, vec![Val::from(3000), Val::from(0)])?;
                                        } else if ctx.var("BaseLevel").get()?.number()? < 70 {
                                            ctx.call(Function::GetExperience, vec![Val::from(5000), Val::from(0)])?;
                                        } else if ctx.var("BaseLevel").get()?.number()? < 80 {
                                            ctx.call(Function::GetExperience, vec![Val::from(8000), Val::from(0)])?;
                                        } else if ctx.var("BaseLevel").get()?.number()? < 90 {
                                            ctx.call(Function::GetExperience, vec![Val::from(10000), Val::from(0)])?;
                                        } else {
                                            ctx.call(Function::GetExperience, vec![Val::from(13000), Val::from(0)])?;
                                        }
                                    }
                                    ctx.next()?;
                                    ctx.call(Function::Warp, vec![Val::from("cmd_in01"), Val::from(33), Val::from(29)])?;
                                    return Err(Stop::End);
                                } else if (l_diaris_t.clone().number()? > 0 && l_diaris_t.clone().number()? < 11) {
                                    ctx.lines_as(
                                        "Dearles",
                                        args![
                                            "Hmm... Some of this",
                                            "stuff looks good, but",
                                            "there isn't enough variety",
                                            "here that proves your eye for",
                                            "craftsmanship. Come back",
                                            "with more stuff, alright?"
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    ctx.lines_as(
                                        "Dearles",
                                        args![
                                            "Hmm...",
                                            "All the stuff you're",
                                            "carrying? Worthless junk!",
                                            "Go and find stuff that was",
                                            "skillfully crafted, okay?"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Dearles", args!["And by crafts I don't mean", "potions, weapons, equipment,", "none of that. Just think of, well, stuff you'd show off in your rooom and you'll be on the right track. Remember, variety!"])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                            }
                            if !matched4 && subject4.loosely_equals(&Val::from(30)) {
                                matched4 = true;
                            }
                            if !matched4 && subject4.loosely_equals(&Val::from(31)) {
                                matched4 = true;
                            }
                            if matched4 {
                                ctx.lines(args![
                                    "Hey, you gotta",
                                    "finish Part Two of",
                                    "my test. I'm gonna send",
                                    "you to Lamadin now..."
                                ])?;
                                ctx.close_window()?;
                                ctx.call(Function::Warp, vec![Val::from("cmd_in01"), Val::from(33), Val::from(29)])?;
                                return Err(Stop::End);
                            }
                            if !matched4 && subject4.loosely_equals(&Val::from(32)) {
                                matched4 = true;
                            }
                            if matched4 {
                                ctx.lines(args![
                                    "You failed Part Two,",
                                    "the rhythm portion of",
                                    "my freakin' test?! If you",
                                    "wanna try again, you're",
                                    "welcome to another chance..."
                                ])?;
                                ctx.next()?;
                                match runtime::select_values(ctx, &[Val::from("Yes.:Maybe later.")])? {
                                    1 => {
                                        ctx.lines_as(
                                            "Dearles",
                                            args![
                                                "Okay...",
                                                "Just keep in mind",
                                                "that Part Two was ",
                                                "supposed to be",
                                                "the easy part..."
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        ctx.call(Function::Warp, vec![Val::from("cmd_in01"), Val::from(33), Val::from(29)])?;
                                        return Err(Stop::End);
                                    }
                                    2 => {
                                        ctx.lines_as(
                                            "Dearles",
                                            args![
                                                "What...?",
                                                "Alright, but you",
                                                "really shouldn't give",
                                                "up. I mean, Part Two is",
                                                "supposed to be ridiculously",
                                                "easy. I made it that way..."
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    _ => {}
                                }
                            }
                            if !matched4 && subject4.loosely_equals(&Val::from(33)) {
                                matched4 = true;
                            }
                            if matched4 {
                                ctx.lines(args![
                                    "Alright, Lamadin",
                                    "tells me you passed.",
                                    "Not the best performance,",
                                    "but it's good enough."
                                ])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dearles",
                                    args![
                                        "Alright, take this chunk",
                                        "of Sobbing Starlight and",
                                        "go find someone named,",
                                        "um, ''Bakerlan'' in Alberta."
                                    ],
                                )?;
                                ctx.var("sign_q").set(Val::from(35))?;
                                ctx.call(Function::GetItem, vec![Val::from(7177), Val::from(1)])?;
                                {
                                    if ctx.var("BaseLevel").get()?.number()? < 60 {
                                        ctx.call(Function::GetExperience, vec![Val::from(3000), Val::from(0)])?;
                                    } else if ctx.var("BaseLevel").get()?.number()? < 70 {
                                        ctx.call(Function::GetExperience, vec![Val::from(7000), Val::from(0)])?;
                                    } else if ctx.var("BaseLevel").get()?.number()? < 80 {
                                        ctx.call(Function::GetExperience, vec![Val::from(19000), Val::from(0)])?;
                                    } else if ctx.var("BaseLevel").get()?.number()? < 90 {
                                        ctx.call(Function::GetExperience, vec![Val::from(12000), Val::from(0)])?;
                                    } else {
                                        ctx.call(Function::GetExperience, vec![Val::from(17000), Val::from(0)])?;
                                    }
                                }
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dearles",
                                    args![
                                        "Well, that's it.",
                                        "There's no reason",
                                        "for you to ever see",
                                        "me again. But yeah,",
                                        "good luck with that",
                                        "Sobbing Starlight business."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            if !matched4 && subject4.loosely_equals(&Val::from(34)) {
                                matched4 = true;
                            }
                            if matched4 {
                                ctx.lines(args![
                                    "Lamadin tells me",
                                    "you passed. Since you",
                                    "did so good, I'm gonna",
                                    "give you ^333333some^000000 of your money",
                                    "back. Not all, but most of it.",
                                    "Isn't that reward enough?"
                                ])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dearles",
                                    args![
                                        "Alright, take this chunk",
                                        "of Sobbing Starlight and",
                                        "go find someone named,",
                                        "um, ''Bakerlan'' in Alberta."
                                    ],
                                )?;
                                ctx.var("sign_q").set(Val::from(35))?;
                                ctx.call(Function::GetItem, vec![Val::from(7177), Val::from(1)])?;
                                ctx.var("Zeny").set((ctx.var("Zeny").get()? + Val::from(20000)))?;
                                {
                                    if ctx.var("BaseLevel").get()?.number()? < 60 {
                                        ctx.call(Function::GetExperience, vec![Val::from(3000), Val::from(0)])?;
                                    } else if ctx.var("BaseLevel").get()?.number()? < 70 {
                                        ctx.call(Function::GetExperience, vec![Val::from(7000), Val::from(0)])?;
                                    } else if ctx.var("BaseLevel").get()?.number()? < 80 {
                                        ctx.call(Function::GetExperience, vec![Val::from(10000), Val::from(0)])?;
                                    } else if ctx.var("BaseLevel").get()?.number()? < 90 {
                                        ctx.call(Function::GetExperience, vec![Val::from(14000), Val::from(0)])?;
                                    } else {
                                        ctx.call(Function::GetExperience, vec![Val::from(19000), Val::from(0)])?;
                                    }
                                }
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dearles",
                                    args![
                                        "Well, that's it.",
                                        "There's no reason",
                                        "for you to ever see",
                                        "me again. But yeah,",
                                        "good luck with that",
                                        "Sobbing Starlight business."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                    } else if ctx.var("sign_q").get()? == 97 {
                        ctx.lines(args!["No...! Damn it,", "not again! When's", "my lucky streak", "gonna start?"])?;
                        ctx.next()?;
                        ctx.lines_as("Dearles", args!["Hey, you've got that look", "like you want something from"])?;
                        if ctx.var("Zeny").get()?.number()? < 10000 {
                            ctx.lines(args![
                                "me. It also looks like you're low on cash~! Well, this is what guys like me to do to shameless",
                                "moochers like you!"
                            ])?;
                            ctx.call(Function::Emotion, vec![ctx.constant("ET_HNG")?])?;
                            ctx.call(Function::PercentHeal, vec![Val::from(-10), Val::from(0)])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            ctx.lines(args![
                                "me. It also looks like you got zeny to spare! I think I'll help myself and borrow some of your cash!",
                                "^FF0000Yoink!^000000",
                                "Giggle giggle.."
                            ])?;
                            ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(10000))?))?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    } else if ctx.var("sign_q").get()? == 98 {
                        ctx.lines(args!["Whaaaaaaaaaat are", "you doing back here?"])?;
                        if ctx.var("Zeny").get()?.number()? > 9999 {
                            ctx.lines(args![
                                "Fine, since we know each",
                                "other, I'm borrowing some",
                                "cash. You know, for old",
                                "time's sake. ^FF0000Yoink!^000000"
                            ])?;
                            ctx.call(Function::Emotion, vec![ctx.constant("ET_KIK")?])?;
                            ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(10000))?))?;
                        } else {
                            ctx.lines(args![
                                "I got nothin' for you and",
                                "you obviously have nothing",
                                "for me! Now lemme gamble!"
                            ])?;
                            ctx.call(Function::Emotion, vec![ctx.constant("ET_HNG")?])?;
                        }
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines(args![
                            "Hey, why are you",
                            "still talkin' to me?",
                            "You finished my test.",
                            "It's over. Not get outta",
                            "here before I jack some",
                            "more of your cash!"
                        ])?;
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_KIK")?])?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Okay.:I want to take Part Two again.")])? {
                            1 => {
                                ctx.lines_as(
                                    "Dearles",
                                    args!["Fine, fine.", "Geez, why do", "these guys always", "come and bother me?"],
                                )?;
                                ctx.call(Function::Emotion, vec![ctx.constant("ET_HNG")?])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    "Dearles",
                                    args!["Wha...?", "Part Two", "was really", "that fun? Fine,", "knock yourself out."],
                                )?;
                                ctx.next()?;
                                ctx.call(Function::Warp, vec![Val::from("cmd_in01"), Val::from(33), Val::from(29)])?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
    Ok(Val::from(0))
}

pub fn strange_guy_sign(ctx: &Ctx) -> Script {
    strange_guy_sign_body(ctx, Vec::new()).map(|_| ())
}

fn examiner_sign_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    shared::quests_the_sign_quest::f_updatesignvars(ctx, vec![])?;
    ctx.mes("[Lamadin]")?;
    if ctx.var("sign_q").get()? == 30 {
        ctx.lines(args![
            "Welcome to",
            "Part Two of Dearles'",
            "exam where your sense",
            "of rhythm will be tested~"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Lamadin",
            args![
                "Please click on the",
                "Chat Room to enter the",
                "Standby Room. When it's",
                "your turn, the test will begin!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lamadin",
            args![
                "The top left pole",
                "means 'Up,' and there",
                "are a total of four marks",
                "designated as Upper, Lower",
                "Left and Right. Hitting each",
                "mark will produce a sound."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lamadin",
            args![
                "During the test,",
                "you must hit the marks",
                "according to the given",
                "instructions in order to",
                "play music. Nifty, huh?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lamadin",
            args![
                "I'm in charge of grading",
                "your performance and determining your qualification. Please do your best and come back to me when",
                "you finish the test. Good luck~",
                "Please do your best."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("sign_q").get()? == 31 {
        ctx.var("sign_q").set(Val::from(30))?;
        ctx.lines(args![
            "Please click on the",
            "Chat Room to enter the",
            "Standby Room. When it's",
            "your turn, the test will begin!"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Lamadin",
            args![
                "The top left pole",
                "means 'Up,' and there",
                "are a total of four marks",
                "designated as Upper, Lower",
                "Left and Right. Hitting each",
                "mark will produce a sound."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lamadin",
            args![
                "During the test,",
                "you must hit the marks",
                "according to the given",
                "instructions in order to",
                "play music. Nifty, huh?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lamadin",
            args![
                "I'm in charge of grading",
                "your performance and determining your qualification. Please do your best and come back to me when",
                "you finish the test. Good luck~",
                "Please do your best."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("sign_q").get()? == 32 {
        ctx.lines(args![
            "Let's see...",
            ((Val::from("You're ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("?")),
            "Oh, what a shame!",
            "You failed this time.",
            "But don't you worry..."
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Lamadin",
            args!["You're welcome to", "try again whenever", "you want, alright?", "Thank you~"],
        )?;
        ctx.var("sign_q").set(Val::from(30))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("sign_q").get()? == 33 {
        ctx.lines(args![
            "Let's see...",
            ((Val::from("You're ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("?")),
            "Congratulations!",
            "You passed the test!"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Lamadin",
            args![
                "I'll send a message to",
                "Mister Dearles right away,",
                "so please go speak to him",
                "again. Once again, great job~ "
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("sign_q").get()? == 34 {
        ctx.lines(args![
            "Let's see...",
            ((Val::from("You're ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("?")),
            "Congratulations!",
            "You got a perfect",
            "score on this test!"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Lamadin",
            args![
                "I'll send a message to",
                "Mister Dearles right away,",
                "so please go speak to him",
                "again. He may even give",
                "you a reward since you",
                "did such a great job~"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines(args![
            "H-how did you",
            "find your way here?",
            "This is private property,",
            "owned by Mister Dearles.",
            "Please leave immediately!"
        ])?;
        ctx.next()?;
        ctx.call(Function::Warp, vec![Val::from("comodo"), Val::from(187), Val::from(164)])?;
        return Err(Stop::End);
    }
}

pub fn examiner_sign(ctx: &Ctx) -> Script {
    examiner_sign_body(ctx, Vec::new()).map(|_| ())
}

fn emergency_exit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Exit", args!["Would you like to go out?"])?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("Yes.:No.")])? {
        1 => {
            ctx.lines_as("Exit", args!["Farewell."])?;
            ctx.next()?;
            ctx.call(Function::Warp, vec![Val::from("comodo"), Val::from(187), Val::from(163)])?;
            return Err(Stop::End);
        }
        2 => {
            ctx.lines_as("Exit", args![".........."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn emergency_exit(ctx: &Ctx) -> Script {
    emergency_exit_body(ctx, Vec::new()).map(|_| ())
}

fn standby_room_sign_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn standby_room_sign(ctx: &Ctx) -> Script {
    standby_room_sign_body(ctx, Vec::new()).map(|_| ())
}

fn standby_room_sign_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::WaitingRoom,
        vec![
            Val::from("DANCE~ DANCE~"),
            Val::from(20),
            Val::from("Standby Room#sign::OnStartArena"),
            Val::from(1),
            Val::from(0),
            Val::from(50),
        ],
    )?;
    ctx.call(Function::EnableWaitingRoomEvent, vec![])?;
    return Err(Stop::End);
}

pub fn standby_room_sign_oninit(ctx: &Ctx) -> Script {
    standby_room_sign_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn standby_room_sign_onstartarena_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance_timer::OnButton_Off")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("Examiner#sd::OnDisable")])?;
    ctx.call(
        Function::WarpWaitingPc,
        vec![Val::from("cmd_in01"), Val::from(16), Val::from(15), Val::from(1)],
    )?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance_timer::OnEnable")])?;
    ctx.call(Function::DisableWaitingRoomEvent, vec![])?;
    return Err(Stop::End);
}

pub fn standby_room_sign_onstartarena(ctx: &Ctx) -> Script {
    standby_room_sign_onstartarena_body(ctx, Vec::new()).map(|_| ())
}

fn standby_room_sign_onreset_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableWaitingRoomEvent, vec![])?;
    return Err(Stop::End);
}

pub fn standby_room_sign_onreset(ctx: &Ctx) -> Script {
    standby_room_sign_onreset_body(ctx, Vec::new()).map(|_| ())
}

pub fn s_dance_timer(ctx: &Ctx) -> Script {
    s_dance_timer_run(ctx, SDanceTimerStep::Start, Vec::new()).map(|_| ())
}

pub fn s_dance_timer_onenable(ctx: &Ctx) -> Script {
    s_dance_timer_run(ctx, SDanceTimerStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn s_dance_timer_ondisable(ctx: &Ctx) -> Script {
    s_dance_timer_run(ctx, SDanceTimerStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn s_dance_timer_onbutton_off(ctx: &Ctx) -> Script {
    s_dance_timer_run(ctx, SDanceTimerStep::OnButtonOff, Vec::new()).map(|_| ())
}

pub fn s_dance_timer_ondisableall(ctx: &Ctx) -> Script {
    s_dance_timer_run(ctx, SDanceTimerStep::OnDisableAll, Vec::new()).map(|_| ())
}

pub fn s_dance_timer_ontimer2000(ctx: &Ctx) -> Script {
    s_dance_timer_run(ctx, SDanceTimerStep::OnTimer2000, Vec::new()).map(|_| ())
}

pub fn s_dance_timer_ontimer4000(ctx: &Ctx) -> Script {
    s_dance_timer_run(ctx, SDanceTimerStep::OnTimer4000, Vec::new()).map(|_| ())
}

pub fn s_dance_timer_ontimer6000(ctx: &Ctx) -> Script {
    s_dance_timer_run(ctx, SDanceTimerStep::OnTimer6000, Vec::new()).map(|_| ())
}

pub fn s_dance_timer_ontimer8000(ctx: &Ctx) -> Script {
    s_dance_timer_run(ctx, SDanceTimerStep::OnTimer8000, Vec::new()).map(|_| ())
}

pub fn s_dance_timer_ontimer11000(ctx: &Ctx) -> Script {
    s_dance_timer_run(ctx, SDanceTimerStep::OnTimer11000, Vec::new()).map(|_| ())
}

pub fn s_dance_timer_ontimer13000(ctx: &Ctx) -> Script {
    s_dance_timer_run(ctx, SDanceTimerStep::OnTimer13000, Vec::new()).map(|_| ())
}

pub fn s_dance_timer_ontimer18000(ctx: &Ctx) -> Script {
    s_dance_timer_run(ctx, SDanceTimerStep::OnTimer18000, Vec::new()).map(|_| ())
}

pub fn s_dance_timer_ontimer21000(ctx: &Ctx) -> Script {
    s_dance_timer_run(ctx, SDanceTimerStep::OnTimer21000, Vec::new()).map(|_| ())
}

pub fn s_dance_timer_ontimer24000(ctx: &Ctx) -> Script {
    s_dance_timer_run(ctx, SDanceTimerStep::OnTimer24000, Vec::new()).map(|_| ())
}

pub fn s_dance_timer_ontimer30000(ctx: &Ctx) -> Script {
    s_dance_timer_run(ctx, SDanceTimerStep::OnTimer30000, Vec::new()).map(|_| ())
}

pub fn s_dance_timer_ontimer34000(ctx: &Ctx) -> Script {
    s_dance_timer_run(ctx, SDanceTimerStep::OnTimer34000, Vec::new()).map(|_| ())
}

pub fn s_dance_timer_ontimer38000(ctx: &Ctx) -> Script {
    s_dance_timer_run(ctx, SDanceTimerStep::OnTimer38000, Vec::new()).map(|_| ())
}

pub fn s_dance_timer_ontimer44000(ctx: &Ctx) -> Script {
    s_dance_timer_run(ctx, SDanceTimerStep::OnTimer44000, Vec::new()).map(|_| ())
}

pub fn s_dance_up(ctx: &Ctx) -> Script {
    s_dance_up_run(ctx, SDanceUpStep::Start, Vec::new()).map(|_| ())
}

pub fn s_dance_up_oninit(ctx: &Ctx) -> Script {
    s_dance_up_run(ctx, SDanceUpStep::OnInit, Vec::new()).map(|_| ())
}

pub fn s_dance_up_onenable(ctx: &Ctx) -> Script {
    s_dance_up_run(ctx, SDanceUpStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn s_dance_up_ondisable(ctx: &Ctx) -> Script {
    s_dance_up_run(ctx, SDanceUpStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn s_dance_up_onup(ctx: &Ctx) -> Script {
    s_dance_up_run(ctx, SDanceUpStep::OnUp, Vec::new()).map(|_| ())
}

pub fn s_dance_up_onreset(ctx: &Ctx) -> Script {
    s_dance_up_run(ctx, SDanceUpStep::OnReset, Vec::new()).map(|_| ())
}

pub fn s_dance_up_ontouch(ctx: &Ctx) -> Script {
    s_dance_up_run(ctx, SDanceUpStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn s_dance_down(ctx: &Ctx) -> Script {
    s_dance_down_run(ctx, SDanceDownStep::Start, Vec::new()).map(|_| ())
}

pub fn s_dance_down_oninit(ctx: &Ctx) -> Script {
    s_dance_down_run(ctx, SDanceDownStep::OnInit, Vec::new()).map(|_| ())
}

pub fn s_dance_down_onenable(ctx: &Ctx) -> Script {
    s_dance_down_run(ctx, SDanceDownStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn s_dance_down_ondisable(ctx: &Ctx) -> Script {
    s_dance_down_run(ctx, SDanceDownStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn s_dance_down_onup(ctx: &Ctx) -> Script {
    s_dance_down_run(ctx, SDanceDownStep::OnUp, Vec::new()).map(|_| ())
}

pub fn s_dance_down_onreset(ctx: &Ctx) -> Script {
    s_dance_down_run(ctx, SDanceDownStep::OnReset, Vec::new()).map(|_| ())
}

pub fn s_dance_down_ontouch(ctx: &Ctx) -> Script {
    s_dance_down_run(ctx, SDanceDownStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn s_dance_left(ctx: &Ctx) -> Script {
    s_dance_left_run(ctx, SDanceLeftStep::Start, Vec::new()).map(|_| ())
}

pub fn s_dance_left_oninit(ctx: &Ctx) -> Script {
    s_dance_left_run(ctx, SDanceLeftStep::OnInit, Vec::new()).map(|_| ())
}

pub fn s_dance_left_onenable(ctx: &Ctx) -> Script {
    s_dance_left_run(ctx, SDanceLeftStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn s_dance_left_ondisable(ctx: &Ctx) -> Script {
    s_dance_left_run(ctx, SDanceLeftStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn s_dance_left_onup(ctx: &Ctx) -> Script {
    s_dance_left_run(ctx, SDanceLeftStep::OnUp, Vec::new()).map(|_| ())
}

pub fn s_dance_left_onreset(ctx: &Ctx) -> Script {
    s_dance_left_run(ctx, SDanceLeftStep::OnReset, Vec::new()).map(|_| ())
}

pub fn s_dance_left_ontouch(ctx: &Ctx) -> Script {
    s_dance_left_run(ctx, SDanceLeftStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn s_dance_right(ctx: &Ctx) -> Script {
    s_dance_right_run(ctx, SDanceRightStep::Start, Vec::new()).map(|_| ())
}

pub fn s_dance_right_oninit(ctx: &Ctx) -> Script {
    s_dance_right_run(ctx, SDanceRightStep::OnInit, Vec::new()).map(|_| ())
}

pub fn s_dance_right_onenable(ctx: &Ctx) -> Script {
    s_dance_right_run(ctx, SDanceRightStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn s_dance_right_ondisable(ctx: &Ctx) -> Script {
    s_dance_right_run(ctx, SDanceRightStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn s_dance_right_onup(ctx: &Ctx) -> Script {
    s_dance_right_run(ctx, SDanceRightStep::OnUp, Vec::new()).map(|_| ())
}

pub fn s_dance_right_onreset(ctx: &Ctx) -> Script {
    s_dance_right_run(ctx, SDanceRightStep::OnReset, Vec::new()).map(|_| ())
}

pub fn s_dance_right_ontouch(ctx: &Ctx) -> Script {
    s_dance_right_run(ctx, SDanceRightStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn s_dance_cen(ctx: &Ctx) -> Script {
    s_dance_cen_run(ctx, SDanceCenStep::Start, Vec::new()).map(|_| ())
}

pub fn s_dance_cen_oninit(ctx: &Ctx) -> Script {
    s_dance_cen_run(ctx, SDanceCenStep::OnInit, Vec::new()).map(|_| ())
}

pub fn s_dance_cen_onenable(ctx: &Ctx) -> Script {
    s_dance_cen_run(ctx, SDanceCenStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn s_dance_cen_ondisable(ctx: &Ctx) -> Script {
    s_dance_cen_run(ctx, SDanceCenStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn s_dance_cen_onup(ctx: &Ctx) -> Script {
    s_dance_cen_run(ctx, SDanceCenStep::OnUp, Vec::new()).map(|_| ())
}

pub fn s_dance_cen_onreset(ctx: &Ctx) -> Script {
    s_dance_cen_run(ctx, SDanceCenStep::OnReset, Vec::new()).map(|_| ())
}

pub fn s_dance_cen_ontouch(ctx: &Ctx) -> Script {
    s_dance_cen_run(ctx, SDanceCenStep::OnTouch, Vec::new()).map(|_| ())
}
